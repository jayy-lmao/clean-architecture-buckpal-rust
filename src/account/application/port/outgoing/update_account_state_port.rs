use crate::account::domain::*;
use anyhow;
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[async_trait]
pub trait UpdateAccountStatePort {
    async fn updateAccountState(
        &self,
        account: Account,
        timeStamp: NaiveDateTime,
    ) -> anyhow::Result<Account>;
}
