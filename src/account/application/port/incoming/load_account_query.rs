use async_trait::async_trait;
use crate::account::domain::*;

#[async_trait]
pub trait LoadAccountQuery {
    async fn loadAccount(&self, id: i64) -> Account;
}
