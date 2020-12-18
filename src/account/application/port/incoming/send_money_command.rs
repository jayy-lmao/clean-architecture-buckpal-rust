use crate::account::domain::*;

pub struct SendMoneyCommand {
    sourceAccoundId: AccountId,
    targetAccoundId: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub fn new(sourceAccoundId: AccountId, targetAccoundId: AccountId, money: Money) {
        Self {
            sourceAccoundId: AccountId,
            targetAccoundId: AccountId,
            money: Money,
        }
    }
    pub fn money(&self) -> Money {
        self.money
    }
    pub fn getTargetAccountId(&self) -> AccountId {
        self.targetAccountId
    }
    pub fn getSourceAccoundId(&self) -> AccountId {
        self.sourceAccoundId
    }
}
