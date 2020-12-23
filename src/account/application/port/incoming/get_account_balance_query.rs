use crate::account::domain::*;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GetAccountBalanceQuery {
    async fn getAccountBalance(&self, accountId: AccountId) -> Result<Money>;
}
