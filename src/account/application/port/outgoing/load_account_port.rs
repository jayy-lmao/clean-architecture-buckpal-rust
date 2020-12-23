use crate::account::domain::*;
use anyhow;
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[async_trait]
pub trait LoadAccountPort {
    async fn loadAccount(
        &self,
        accountId: AccountId,
        timeStamp: NaiveDateTime,
    ) -> anyhow::Result<Account>;
}
