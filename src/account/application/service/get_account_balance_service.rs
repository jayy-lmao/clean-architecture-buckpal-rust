use chrono::prelude::*;

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
