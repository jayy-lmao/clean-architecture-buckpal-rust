use crate::account::application::port::incoming::GetAccountBalanceQuery;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::domain::*;
use chrono::prelude::*;
use std::sync::Arc;

pub struct GetAccountBalanceService {
    loadAccountPort: Arc<dyn LoadAccountPort + Sync + Send>,
}

impl GetAccountBalanceQuery for GetAccountBalanceService {
    fn getAccountBalance(&self, accountId: AccountId) -> Money {
        self.loadAccountPort
            .loadAccount(accountId, Local::now())
            .calculateBalance()
    }
}
