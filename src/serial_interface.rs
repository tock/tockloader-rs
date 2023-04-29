use futures::stream::StreamExt;
use std::io::Write;
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;
use tokio_serial::SerialPortBuilderExt;


use tokio_serial::{SerialStream};

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // let newline = src.as_ref().iter().position(|b| *b == b'\n');
        // if let Some(n) = newline {
        //     let line = src.split_to(n + 1);
        //     return match str::from_utf8(line.as_ref()) {
        //         Ok(s) => Ok(Some(s.to_string())),
        //         Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
        //     };
        // }
        // Ok(None)
        if src.is_empty() {
            return Ok(None);
        }
        let result = match str::from_utf8(&src) {
            Ok(s) => Ok(Some(s.to_string())),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
        };
        src.clear();
        return result;
    }
}

pub fn open_port(path: String, baud_rate:u32) -> tokio_serial::Result<SerialStream> {
    // Is it async? It can't be awaited...
    tokio_serial::new(path, baud_rate).open_native_async()
}

pub async fn run_terminal(stream: SerialStream) {
    let mut reader = LineCodec.framed(stream);
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        print!("{}", line);
        io::stdout().flush().unwrap();
    }
}