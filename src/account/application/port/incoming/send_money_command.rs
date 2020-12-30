use crate::account::domain::*;

#[derive(Debug)]
pub struct SendMoneyCommand {
    source_account_id: AccountId,
    target_account_id: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub fn new(source_account_id: AccountId, target_account_id: AccountId, money: Money) -> Self {
        Self {
            source_account_id,
            target_account_id,
            money,
        }
    }
    pub fn money(&self) -> Money {
        self.money
    }
    pub fn get_target_account_id(&self) -> AccountId {
        self.target_account_id
    }
    pub fn get_source_account_id(&self) -> AccountId {
        self.source_account_id
    }
}
