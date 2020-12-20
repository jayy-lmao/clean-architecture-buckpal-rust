use crate::account::domain::*;
use chrono::Local;

pub struct Account {
    id: AccountId,
    baselineBalance: Money,
    activityWindow: ActivityWindow,
}

impl Account {
    pub fn calculateBalance(&self) -> Money {
        Money::add(
            self.baselineBalance,
            self.activityWindow.calculateBalance(self.id),
        )
    }

    pub fn withdraw(&mut self, money: Money, targetAccountId: AccountId) -> bool {
        if !self.mayWithdraw(money) {
            return false;
        };

        let withdrawal = Activity {
            fromAccount: self.id,
            toAccount: targetAccountId,
            timestamp: Local::now(),
            money,
        };

        self.activityWindow.addActivity(withdrawal);

        return true;
    }

    pub fn mayWithdraw(&self, money: Money) -> bool {
        Money::add(self.calculateBalance(), money.clone().negate()).isPositive()
    }

    pub fn deposit(&mut self, money: Money, sourceAccountId: AccountId) -> bool {
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
