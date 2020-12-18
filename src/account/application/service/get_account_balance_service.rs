use chrono::prelude::*;
use crate::account::domain::*;

pub struct GetAccountBalanceService {
    loadAccountPort: LoadAccountPort,
}

impl GetAccountBalanceQuery for GetAccountBalanceService {
    fn getAccountBalance(accountId: AccountId) -> Money {
        loadAccountPort
            .loadAccount(accountId, Local::now())
            .calculateBalance();
    }
}
