/// TODO: What exactly is an attribute?
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl Attribute {
    /// Intrepret raw bytes, according to the Tock Format (TODO: Source??)
    pub fn parse_raw(bytes: Vec<u8>) -> Attribute {
        todo!()
    }
}
