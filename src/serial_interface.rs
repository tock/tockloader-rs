use futures::stream::{SplitSink, SplitStream, StreamExt};
use futures::SinkExt;
use std::error::Error;
use std::io::Write;
use std::{io, str};

use tokio_util::codec::{Decoder, Encoder, Framed};

use bytes::{Buf, BufMut, BytesMut};
use console::Term;
use tokio_serial::SerialPortBuilderExt;
use tokio_serial::SerialStream;

pub struct LineCodec;

impl LineCodec {
    fn clean_input(input: &str) -> String {
        // Use consistent line endings for all OS versions.
        // More so, in the newest version of the kernel a "backspace" is echoed out as
        //
        // <backspace><space><backspace><null><backspace><space><backspace>
        //
        // The <backspace> character only moves the cursor back, and does not delete. What the
        // space does is overwrite the previous character with a seemingly empty one (space) and
        // then moves the cursor back.
        //
        // In previous versiouns only these three characters were printed, but now also
        // an null (or "End of file" byte) is also transmitted and promptly deleted.
        // The issues appear when we can't actually delete null bytes, the actual result
        // being two (normal) characters being deleted at once, sometimes overflowing and
        // starting to (visually) delete the tock prompt ("tock$ ") that preceds all lines.
        //
        // Python's minterm  dealt with this issue by converting the null byte into
        // Unicode code point 0x2400. This is a specific "end of file" 3-byte long character
        // which can be deleted.
        input.replace('\n', "\r\n").replace('\x00', "\u{2400}")
    }
}

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if source.is_empty() {
            return Ok(None);
        }

        // Read everything you can, and interpret it as a string.
        match str::from_utf8(source) {
            Ok(utf8_string) => {
                let output = LineCodec::clean_input(utf8_string);
                source.clear();
                Ok(Some(output))
            }
            Err(error) => {
                let index = error.valid_up_to();

                if index == 0 {
                    // Returning Some("") makes it so no other bytes are read in. I have no idea why.
                    // If you find a reason why, please edit this comment.
                    return Ok(None);
                }

                match str::from_utf8(&source[..index]) {
                    Ok(utf8_string) => {
                        let output = LineCodec::clean_input(utf8_string);
                        source.advance(index);
                        Ok(Some(output))
                    }
                    Err(_) => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Couldn't parse input as UTF8. Last valid index: {}. Buffer: {:?}",
                            index, source
                        ),
                    )),
                }
            }
        }
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        _dst.put(_item.as_bytes());
        Ok(())
    }
}

pub fn open_first_available_port(baud_rate: u32) -> tokio_serial::Result<SerialStream> {
    let ports = tokio_serial::available_ports()?;

    for p in ports {
        // For whatever reason, the returend ports are listed under the "/sys/class/tty/<port>" directory.
        // While it does identify the right port, the result doesn't actually point to the device.
        // As such, we'll try to use the name of the port with the "/dev/" path

        let port_path: String = if p.port_name.contains("tty") {
            match p.port_name.split('/').last() {
                Some(port_name) => format!("/dev/{}", port_name),
                None => p.port_name,
            }
        } else {
            p.port_name
        };

        match open_port(port_path.clone(), baud_rate) {
            Ok(stream) => return Ok(stream),
            Err(_) => println!("Failed to open port {}.", port_path),
        }
    }

    Err(tokio_serial::Error::new(
        tokio_serial::ErrorKind::Io(io::ErrorKind::NotConnected),
        "Couldn't open any of the available ports.",
    ))
}

pub fn open_port(path: String, baud_rate: u32) -> tokio_serial::Result<SerialStream> {
    // Is it async? It can't be awaited...
    // TODO: What if we don't know the port? We need to copy over the implemenntation from the python version
    tokio_serial::new(path, baud_rate).open_native_async()
}

pub async fn run_terminal(stream: SerialStream) {
    let (writer, reader) = LineCodec.framed(stream).split();
    let read_handle = tokio::spawn(async move {
        if read_from_serial(reader).await.is_err() {
            eprintln!("Connection closed due to error.");
            Err(())
        } else {
            Ok(())
        }
    });
    let write_handle = tokio::spawn(async move {
        if write_to_serial(writer).await.is_err() {
            eprintln!("Connection closed due to error.");
        }
    });

    tokio::select! {
        result = read_handle => {
            // The write handle cannot be aborrted because of the blocking task spawned in it.
            // As such, I think we are pretty much safe to forcefully exit at this point.
            match result {
                Ok(_) => std::process::exit(0),
                Err(_) => std::process::exit(1),
            }
        }
        _ = write_handle => {}
    }
}

pub async fn read_from_serial(
    mut reader: SplitStream<Framed<SerialStream, LineCodec>>,
) -> Result<(), Box<dyn Error>> {
    // TODO: What if there is another instance of tockloader open? Check the python implementation

    while let Some(line_result) = reader.next().await {
        let line = match line_result {
            Ok(it) => it,
            Err(err) => {
                eprint!("Failed to read string. Error: {:?}", err);
                return Err(Box::new(err));
            }
        };
        print!("{}", line);

        // We need to flush the buffer because the "tock>" prompt does not have a newline.
        io::stdout().flush().unwrap();
    }

    Ok(())
}

pub async fn write_to_serial(
    mut writer: SplitSink<Framed<SerialStream, LineCodec>, std::string::String>,
) -> Result<(), Box<dyn Error>> {
    loop {
        let console_input = tokio::task::spawn_blocking(move || Term::stdout().read_key()).await?;

        let key = console_input?;

        let send_buffer: Option<String> = match key {
            console::Key::Unknown => None,
            console::Key::UnknownEscSeq(_) => None,
            console::Key::ArrowLeft => Some("\u{1B}[D".into()),
            console::Key::ArrowRight => Some("\u{1B}[C".into()),
            console::Key::ArrowUp => Some("\u{1B}[A".into()),
            console::Key::ArrowDown => Some("\u{1B}[B".into()),
            console::Key::Enter => Some("\n".into()),
            console::Key::Escape => None,
            console::Key::Backspace => Some("\x08".into()),
            console::Key::Home => Some("\u{1B}[H".into()),
            console::Key::End => Some("\u{1B}[F".into()),
            console::Key::Tab => Some("\t".into()),
            console::Key::BackTab => Some("\t".into()),
            console::Key::Alt => None,
            // In latest version of kernel (2023.08.25), the "del" ascii code (\x7F)
            // is handled exactly as backspace (\x08). Proper del is this:
            console::Key::Del => Some("\u{1B}[3~".into()),
            console::Key::Shift => None,
            console::Key::Insert => None,
            console::Key::PageUp => None,
            console::Key::PageDown => None,
            console::Key::Char(c) => Some(c.into()),
            _ => todo!(),
        };

        if let Some(buffer) = send_buffer {
            if let Err(err) = writer.send(buffer.clone()).await {
                eprintln!(
                    "Error writing to serial. Buffer {}. Error: {:?} ",
                    buffer, err
                );
                return Err(Box::new(err));
            }
        }
    }

    // TODO: handle CTRL+C
    // Ok(())
}
