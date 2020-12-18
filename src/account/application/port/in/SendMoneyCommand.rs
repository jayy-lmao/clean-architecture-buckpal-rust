pub struct SendMoneyCommand {
    sourceAccoundId: AccountId,
    targetAccoundId: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub fn new(sourceAccoundId: AccountId, targetAccoundId: AccountId, money: Money) {
        requireGreaterThan(money, 0);
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
