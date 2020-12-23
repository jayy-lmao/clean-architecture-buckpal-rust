use super::SendMoneyCommand;
use anyhow;
use async_trait::async_trait;

#[async_trait]
pub trait SendMoneyUseCase {
    async fn sendMoney(&self, command: SendMoneyCommand) -> anyhow::Result<bool>;
}
