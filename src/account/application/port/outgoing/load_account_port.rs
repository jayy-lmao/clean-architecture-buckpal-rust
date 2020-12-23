use crate::account::domain::*;
use chrono::NaiveDateTime;
use async_trait::async_trait;
use anyhow;

#[async_trait]
pub trait LoadAccountPort {
    async fn loadAccount(&self, accountId: AccountId, timeStamp: NaiveDateTime) -> anyhow::Result<Account>;
}
