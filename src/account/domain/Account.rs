use chrono::prelude::*;

pub struct Account {
    id: AccountId,
    baselineBalance: Money,
    activityWindow: ActivityWindow,
}

pub impl Account {
    pub fn calculateBalance(&self) -> Money {
        Money.add(
            self.baselineBalance,
            self.activityWindow.calculateBalance(self.id),
        )
    }

    pub fn withdraw(&self, money: Money, targetAccountId: AccountId) -> bool {
        if (!self.mayWithdraw(money)) {
            false
        }

        let withdrawal = Activity {
            fromAccount: self.id,
            toAccount: targetAccountId,
            timestamp: Local::now(),
            money,
        };

        this.activityWindow.addActivity(withdrawal);

        true
    }

    pub fn mayWithdraw(&self, money: Money) -> bool {
        Money
            .add(self.calculateBalance(), money.negate())
            .isPositive()
    }

    pub fn deposit(money: Money, sourceAccountId: AcountId)->bool{
        let deposit = Activity {
            fromAccount: self.id,
            toAccount: sourceAccountId,
            timestamp: Local::now(),
            money,
        };
        self.activityWindow.addActivity(deposit);

        true
    }
}
