use super::SendMoneyCommand;
use async_trait::async_trait;

#[async_trait]
pub trait SendMoneyUseCase {
    async fn send_money(&self, command: SendMoneyCommand) -> anyhow::Result<bool>;
}
