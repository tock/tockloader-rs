use crate::errors::TockloaderError;
use crate::interfaces::traits::VirtualTerminal;
use crate::interfaces::OpenOCDInterface;
use async_trait::async_trait;

#[async_trait]
impl VirtualTerminal for OpenOCDInterface {
    async fn run_terminal(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
