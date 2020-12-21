use crate::account::domain::*;
use chrono::NaiveDateTime;
use async_trait::async_trait;

#[async_trait]
pub trait LoadAccountPort {
    async fn loadAccount(&self, accountId: AccountId, timeStamp: NaiveDateTime) -> Result<Account, dyn std::error::Error>;
}
