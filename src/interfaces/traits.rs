use crate::errors::TockloaderError;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}

#[async_trait]
#[enum_dispatch]
pub trait VirtualTerminal {
    async fn run_terminal(&mut self) -> Result<(), TockloaderError>;
}
