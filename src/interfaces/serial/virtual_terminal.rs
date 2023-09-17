use bytes::{Buf, BufMut, BytesMut};
use tokio::task::JoinHandle;

use crate::errors::TockloaderError;
use crate::interfaces::traits::VirtualTerminal;
use crate::interfaces::SerialInterface;
use async_trait::async_trait;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use futures::SinkExt;
use std::str;
use tokio_util::codec::{Decoder, Encoder, Framed};

struct TerminalCodec;

#[async_trait]
impl VirtualTerminal for SerialInterface {
    // Run the virtual terminal to interact with the tock console.
    // Warning!
    //     Before returning, the connection is closed. You must re-open the connection for any
    //     further operations.
    async fn run_terminal(&mut self) -> Result<(), TockloaderError> {
        if self.stream.is_none() {
            unreachable!("Stream is not initialized!")
        }

        let (mut writer, mut reader) = TerminalCodec
            .framed(self.stream.take().expect("SerialStream wasn't initialized"))
            .split();

        let read_handle: JoinHandle<Result<(), TockloaderError>> = tokio::spawn(async move {
            // TODO: I don't get why the decoder returns Result<Option<String>, ...> but
            // line_result is actually Result<String, ...>.
            // What does it mean if .next() return None?
            while let Some(line_result) = reader.next().await {
                print!("{}", line_result?);
            }

            Ok(())
        });

        let write_handle = tokio::spawn(async move {
            // if write_to_serial(writer).await.is_err() {
            //     eprintln!("Connection closed due to error.");
            // }
            todo!()
        });

        tokio::select! {
            join_result = read_handle => {
                match join_result {
                    Ok(read_handle_result) => read_handle_result,
                    Err(join_err) => Err(TockloaderError::JoinError(join_err)),
                }
            }
            _ = write_handle => {
                unreachable!("Write handle shouldn't return, yet it did.")
            }
        }
    }
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
                    // Returning Some("") makes it so no other bytes are read in. I have no idea why.
                    // If you find a reason why, please edit this comment.
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
