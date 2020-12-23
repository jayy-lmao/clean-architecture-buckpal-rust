use crate::account::application::port::incoming::GetAccountBalanceQuery;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::domain::*;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use anyhow;

pub struct GetAccountBalanceService {
    loadAccountPort: Arc<dyn LoadAccountPort + Sync + Send>,
}

#[async_trait]
impl GetAccountBalanceQuery for GetAccountBalanceService {
    async fn getAccountBalance(
        &self,
        accountId: AccountId,
    ) -> anyhow::Result<Money> {
        Ok(self
            .loadAccountPort
            .loadAccount(accountId, Utc::now().naive_utc())
            .await?
            .calculateBalance())
    }
}
