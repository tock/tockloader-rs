use futures::stream::{SplitSink, SplitStream, StreamExt};
use futures::SinkExt;
use std::io::Write;
use std::{io, str};
use tokio_util::codec::{Decoder, Encoder, Framed};

use bytes::{BufMut, BytesMut};
use console::Term;
use tokio_serial::SerialPortBuilderExt;
use tokio_serial::SerialStream;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        // Read everything you can, and interpret it as a string.
        // TODO: Note that this can fail if we try to decode in the middle of a multi-byte UTF-8 Character.
        // We could wait for more output, or use this <https://doc.rust-lang.org/stable/core/str/struct.Utf8Error.html#method.valid_up_to>
        let result = match str::from_utf8(src) {
            Ok(s) => {
                let output = s.replace('\n', "\r\n");
                Ok(Some(output))
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
        };
        src.clear();
        result
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
    let a = tokio::spawn(async move {
        read_from_serial(reader).await;
    });
    tokio::spawn(async move {
        write_to_serial(writer).await;
    });

    a.await.unwrap();
}

pub async fn read_from_serial(mut reader: SplitStream<Framed<SerialStream, LineCodec>>) {
    // TODO: What if there is another instance of tockloader open? Check the python implementation

    // TODO: Spawn this into its own task, so that we may read and write at the same time.
    // TODO: Can we hijack CTRL+C so that we can exit cleanly?
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        print!("{}", line);
        // We need to flush the buffer because the "tock>" prompt does not have a newline.
        io::stdout().flush().unwrap();
    }
}

pub async fn write_to_serial(
    mut writer: SplitSink<Framed<SerialStream, LineCodec>, std::string::String>,
) {
    let term = Term::stdout();

    loop {
        let key = match term.read_key() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Read error: {e}");
                break;
            }
        };

        let buf: Option<String> = match key {
            console::Key::Unknown => todo!(),
            console::Key::UnknownEscSeq(_) => todo!(),
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
            console::Key::Del => Some("\x7f".into()),
            console::Key::Shift => None,
            console::Key::Insert => None,
            console::Key::PageUp => None,
            console::Key::PageDown => None,
            console::Key::Char(c) => Some(c.into()),
            _ => todo!(),
        };

        if let Some(c) = buf {
            println!("Sending {}", c);
            writer
                .send(c.into())
                .await
                .expect("Could not send message.");
        }
    }
    // loop {
    //     let mut buffer = String::new();
    //     let _ = io::stdin().read_line(&mut buffer);
    //     writer.send(buffer).await.expect("AAAaaaa");
    //     writer.flush().await.expect("BBBBBBbb");
    // }
}
