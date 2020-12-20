use crate::account::domain::*;

pub trait GetAccountBalanceQuery {
    fn getAccountBalance(&self, accountId: AccountId) -> Money;
}
