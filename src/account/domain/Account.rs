use crate::account::domain::*;
use chrono::Utc;

#[derive(Debug)]
pub struct Account {
    pub id: AccountId,
    baseline_balance: Money,
    pub activity_window: ActivityWindow,
}

impl Account {
    pub fn calculate_balance(&self) -> Money {
        Money::add(
            self.baseline_balance,
            self.activity_window.calculate_balance(self.id),
        )
    }

    pub fn withdraw(&mut self, money: Money, target_account_id: AccountId) -> bool {
        if !self.may_withdraw(money) {
            return false;
        };

        let withdrawal = Activity {
            from_account: self.id,
            to_account: target_account_id,
            timestamp: Utc::now().naive_utc(),
            money,
        };

        self.activity_window.add_activity(withdrawal);

        true
    }

    pub fn may_withdraw(&self, money: Money) -> bool {
        Money::add(self.calculate_balance(), money.clone().negate()).is_positive()
    }

    pub fn deposit(&mut self, money: Money, source_account_id: AccountId) -> bool {
        let deposit = Activity {
            from_account: source_account_id,
            to_account: self.id,
            timestamp: Utc::now().naive_utc(),
            money,
        };

        self.activity_window.add_activity(deposit);

        true
    }
    // pub fn withoutId(baselineBalance: Money, activityWindow: ActivityWindow) -> Account {
    //     return Account::new(accountId, baselineBalance, activityWindow)
    // }
    pub fn with_id(
        account_id: AccountId,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Account {
        Account {
            id: account_id,
            baseline_balance,
            activity_window,
        }
    }
}
