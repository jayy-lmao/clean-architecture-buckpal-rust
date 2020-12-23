use crate::account::domain::*;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait GetAccountBalanceQuery {
    fn getAccountBalance(&self, accountId: AccountId) -> Result<Money>;
}
