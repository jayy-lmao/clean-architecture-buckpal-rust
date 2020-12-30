use crate::account::domain::*;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GetAccountBalanceQuery {
    async fn get_account_balance(&self, account_id: AccountId) -> Result<Money>;
}
