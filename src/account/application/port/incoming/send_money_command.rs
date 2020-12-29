use crate::account::domain::*;

#[derive(Debug)]
pub struct SendMoneyCommand {
    sourceAccountId: AccountId,
    targetAccountId: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub fn new(sourceAccoundId: AccountId, targetAccoundId: AccountId, money: Money) -> Self {
        Self {
            sourceAccountId: sourceAccoundId,
            targetAccountId: targetAccoundId,
            money,
        }
    }
    pub fn money(&self) -> Money {
        self.money
    }
    pub fn getTargetAccountId(&self) -> AccountId {
        self.targetAccountId
    }
    pub fn getSourceAccoundId(&self) -> AccountId {
        self.sourceAccountId
    }
}
