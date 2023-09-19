use crate::errors::TockloaderError;
use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct BinaryCodec;

impl Decoder for BinaryCodec {
    type Item = Vec<u8>;
    type Error = TockloaderError;

    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if source.is_empty() {
            return Ok(None);
        }

        Ok(Some(Vec::from(&source[..])))
    }
}

impl Encoder<Vec<u8>> for BinaryCodec {
    type Error = TockloaderError;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(&item[..]);
        Ok(())
    }
}

impl Encoder<&[u8]> for BinaryCodec {
    type Error = TockloaderError;

    fn encode(&mut self, item: &[u8], dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(item);
        Ok(())
    }
}

impl<const N: usize> Encoder<[u8; N]> for BinaryCodec {
    type Error = TockloaderError;

    fn encode(&mut self, item: [u8; N], dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(&item[..]);
        Ok(())
    }
}
