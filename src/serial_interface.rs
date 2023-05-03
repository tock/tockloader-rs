use futures::stream::StreamExt;
use std::io::Write;
use std::{io, str};
use tokio_util::codec::Decoder;

use bytes::BytesMut;
use tokio_serial::SerialPortBuilderExt;

use tokio_serial::SerialStream;

struct LineCodec;

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
            Ok(s) => Ok(Some(s.to_string())),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
        };
        src.clear();
        result
    }
}

pub fn open_port(path: String, baud_rate: u32) -> tokio_serial::Result<SerialStream> {
    // Is it async? It can't be awaited...
    // TODO: What if we don't know the port? We need to copy over the implemenntation from the python version
    tokio_serial::new(path, baud_rate).open_native_async()
}

pub async fn run_terminal(stream: SerialStream) {
    // TODO: What if there is another instance of tockloader open? Check the python implementation

    let mut reader = LineCodec.framed(stream);
    // TODO: Spawn this into its own task, so that we may read and write at the same time.
    // TODO: Can we hijack CTRL+C so that we can exit cleanly?
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        print!("{}", line);
        // We need to flush the buffer because the "tock>" prompt does not have a newline.
        io::stdout().flush().unwrap();
    }
}
