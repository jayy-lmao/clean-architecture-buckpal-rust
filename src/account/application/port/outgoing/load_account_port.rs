use crate::account::domain::*;
use anyhow;
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[async_trait]
pub trait LoadAccountPort {
    async fn load_account(
        &self,
        account_id: AccountId,
        timestamp: NaiveDateTime,
    ) -> anyhow::Result<Account>;
}
