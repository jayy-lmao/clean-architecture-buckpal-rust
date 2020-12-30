use crate::account::domain::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[async_trait]
pub trait UpdateAccountStatePort {
    async fn update_account_state(
        &self,
        account: Account,
        timestamp: NaiveDateTime,
    ) -> anyhow::Result<()>;
}
