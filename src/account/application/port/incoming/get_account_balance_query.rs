use crate::account::domain::*;
use async_trait::async_trait;

#[async_trait]
pub trait GetAccountBalanceQuery {
    fn getAccountBalance(&self, accountId: AccountId) -> Result<Money, dyn std::error::Error>;
}
