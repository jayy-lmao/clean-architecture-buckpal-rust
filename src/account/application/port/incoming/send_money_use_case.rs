use super::SendMoneyCommand;
use async_trait::async_trait;
use anyhow;

#[async_trait]
pub trait SendMoneyUseCase {
    async fn sendMoney(&self, command: SendMoneyCommand) -> anyhow::Result<bool>;
}
