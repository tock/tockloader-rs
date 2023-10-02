use bytes::{Buf, BufMut, BytesMut};
use tokio::task::JoinHandle;

use crate::errors::TockloaderError;
use crate::interfaces::traits::VirtualTerminal;
use crate::interfaces::SerialInterface;
use async_trait::async_trait;
use console::Term;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::io::Write;
use std::sync::Arc;
use std::{io, str};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
struct TerminalCodec;

#[async_trait]
impl VirtualTerminal for SerialInterface {
    // Run the virtual terminal to interact with the tock console.
    async fn run_terminal(&mut self) -> Result<(), TockloaderError> {
        if self.stream.is_none() {
            // Note: I'm using panic here because "unreachable!" doesn't feel appropriate
            // This code could very well be reached if, let's say, a function
            // unexpectedly has the side-effect of making the stream "None", even though
            // in a release build the program should never panic from this.
            panic!("Stream is not initialized!")
        }

        let (writer, reader) = TerminalCodec.framed(self.stream.take().unwrap()).split();

        let reader_arc = Arc::new(tokio::sync::Mutex::new(reader));
        let read_handle: JoinHandle<Result<(), TockloaderError>> = tokio::spawn({
            let reader_arc = Arc::clone(&reader_arc);
            async move {
                // Q: I don't get why the decoder returns Result<Option<String>, ...> but
                // line_result is actually Result<String, ...>.
                // A: Because the decoded uses Ok(None) as an indicator that it needs to wait for
                // more bytes, where we will always have a result (even if it happens to be an
                // empty string).
                // TODO: What does it mean if .next() return None?
                while let Some(line_result) = reader_arc.lock().await.next().await {
                    print!("{}", line_result?);

                    // We need to flush the buffer because the "tock$" prompt does not have a newline.
                    io::stdout().flush().unwrap();
                }

                Ok(())
            }
        });

        let writer_arc = Arc::new(tokio::sync::Mutex::new(writer));
        let write_handle: JoinHandle<Result<(), TockloaderError>> = tokio::spawn({
            let writer_arc = Arc::downgrade(&writer_arc);
            async move {
                loop {
                    if let Some(buffer) = get_key().await? {
                        if let Some(writer) = writer_arc.upgrade() {
                            writer.lock().await.send(buffer).await?
                        } else {
                            return Ok(());
                        }
                    }
                }
            }
        });

        let result = tokio::select! {
            join_result = read_handle => {
                join_result?
            }
            join_result = write_handle => {
                join_result?
            }
        };

        // Arc::into_innter will always return Mutex<SplitSink/SplitStream> because the previous select statement
        // will always make sure all the closures are either finished or cancelled and the arc is cloned nowhere else.
        // We can move out of the mutex for similar reasons, no one else will need this reader/writer.
        let writer = Arc::into_inner(writer_arc).unwrap().into_inner();
        let reader = Arc::into_inner(reader_arc).unwrap().into_inner();
        // Reader and Writer are from the same call, so reunite() shouldn't fail,
        // and we can remove the "Framed" around the strean with .into_inner()
        self.stream = Some(reader.reunite(writer).unwrap().into_inner());

        return result;
    }
}

async fn get_key() -> Result<Option<String>, TockloaderError> {
    let console_result = tokio::task::spawn_blocking(move || Term::stdout().read_key()).await?;

    let key = console_result?;

    Ok(match key {
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
        console::Key::Del => Some("\x7f".into()),
        console::Key::Shift => None,
        console::Key::Insert => None,
        console::Key::PageUp => None,
        console::Key::PageDown => None,
        console::Key::Char(c) => Some(c.into()),
        _ => todo!(),
    })
}

impl Decoder for TerminalCodec {
    type Item = String;
    type Error = TockloaderError;

    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if source.is_empty() {
            return Ok(None);
        }

        // There may be incomplete utf-8 sequences, so interpret as much as we can.
        // We aren't expecting to get non-utf8 bytes. Otherwise, the decoder would get stuck!
        match str::from_utf8(source) {
            Ok(result_str) => {
                // Release immutable reference to source
                let result = result_str.to_string();

                source.clear();
                Ok(Some(result))
            }
            Err(error) => {
                let index = error.valid_up_to();

                if index == 0 {
                    // Q: Returning Some("") makes it so no other bytes are read in. I have no idea why.
                    // If you find a reason why, please edit this comment.
                    // A: By looking at the documentaion of the 'decode' method, Ok(None) signals
                    // that we need to read more bytes. Otherwise, returning Some("") would call
                    // 'decode' again until Ok(None) is returned.
                    return Ok(None);
                }

                let result = str::from_utf8(&source[..index])
                    .expect("UTF-8 string failed after verifying with 'valid_up_to()'")
                    .to_string();
                source.advance(index);

                Ok(Some(result))
            }
        }
    }
}

impl Encoder<String> for TerminalCodec {
    type Error = TockloaderError;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(item.as_bytes());
        Ok(())
    }
}
