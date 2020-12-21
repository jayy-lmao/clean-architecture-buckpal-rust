use crate::account::domain::*;
use chrono::Utc;

pub struct Account {
    pub id: AccountId,
    baselineBalance: Money,
    pub activityWindow: ActivityWindow,
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
            timestamp: Utc::now().naive_utc(),
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
            timestamp: Utc::now().naive_utc(),
            money,
        };

        self.activityWindow.addActivity(deposit);

        true
    }
    // pub fn withoutId(baselineBalance: Money, activityWindow: ActivityWindow) -> Account {
    //     return Account::new(accountId, baselineBalance, activityWindow)
    // }
    pub fn withId(
        accountId: AccountId,
        baselineBalance: Money,
        activityWindow: ActivityWindow,
    ) -> Account {
        return Account {
            id: accountId,
            baselineBalance,
            activityWindow,
        };
    }
}
