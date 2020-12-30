use crate::account::domain::*;
use async_trait::async_trait;

#[async_trait]
pub trait LoadAccountQuery {
    async fn load_account(&self, id: i64) -> Account;
}
