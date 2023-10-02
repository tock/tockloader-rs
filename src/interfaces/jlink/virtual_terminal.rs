use crate::errors::TockloaderError;
use crate::interfaces::traits::VirtualTerminal;
use crate::interfaces::JLinkInterface;
use async_trait::async_trait;

#[async_trait]
impl VirtualTerminal for JLinkInterface {
    async fn run_terminal(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
