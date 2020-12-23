use crate::account::domain::*;
use async_trait::async_trait;

#[async_trait]
pub trait LoadAccountQuery {
    async fn loadAccount(&self, id: i64) -> Account;
}
