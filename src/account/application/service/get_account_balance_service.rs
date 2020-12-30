use crate::account::application::port::incoming::GetAccountBalanceQuery;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::domain::*;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct GetAccountBalanceService {
    pub load_account_port: Arc<dyn LoadAccountPort + Sync + Send>,
}

#[async_trait]
impl GetAccountBalanceQuery for GetAccountBalanceService {
    async fn get_account_balance(&self, account_id: AccountId) -> anyhow::Result<Money> {
        Ok(self
            .load_account_port
            .load_account(account_id, Utc::now().naive_utc())
            .await?
            .calculate_balance())
    }
}
