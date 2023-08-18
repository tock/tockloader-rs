use core::fmt;

#[derive(Debug)]
pub enum TockloaderError {}

impl fmt::Display for TockloaderError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
