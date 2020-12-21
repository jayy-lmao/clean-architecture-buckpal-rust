use crate::account::adapter::entities::*;
use crate::account::adapter::repositories::{AccountRepository, ActivityRepository};
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::domain::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;

pub mod ActivityMapper {
    use super::*;
    pub fn mapEntityToDomain(activity_entity: ActivityEntity) -> Activity {
        Activity {
            fromAccount: AccountId::new(activity_entity.sourceAccountId),
            toAccount: AccountId::new(activity_entity.targetAccountId),
            timestamp: activity_entity.timestamp,
            money: Money::new(activity_entity.amount),
        }
    }
}

pub mod AccountMapper {
    use super::*;
    pub fn mapEntityToDomain(
        account_entity: AccountEntity,
        activities: Vec<ActivityEntity>,
        withdraw_balance: f32,
        deposit_balance: f32,
    ) -> Account {
        Account::withId(
            AccountId::new(account_entity.id),
            Money::new(deposit_balance - withdraw_balance),
            activities
                .iter()
                .map(ActivityMapper::mapEntityToDomain)
                .collect(),
        )
    }
}

pub struct AccountPersistenceAdapter {
    accountRepository: AccountRepository,
    activityRepository: ActivityRepository,
}

#[async_trait]
impl LoadAccountPort for AccountPersistenceAdapter {
    async fn loadAccount(
        &self,
        accountId: AccountId,
        baselineDate: NaiveDateTime,
    ) -> Result<Account, dyn std::error::Error> {
        let account_entity = match self.accountRepository.findById(accountId.to_i64()).await {
            Ok(account) => account,
            Err(_) => return std::error::Error::from("yikes"),
        };

        let activities = self
            .activityRepository
            .findByOwnerSince(accountId.to_i64(), baselineDate)
            .await
            .unwrap_or(Vec::new());

        let withdraw_balance = self
            .activityRepository
            .getWithdrawalBalance(accountId.to_i64(), baselineDate);

        let deposit_balance = self
            .activityRepository
            .getDepositBalance(accountId.to_i64(), baselineDate);

        return Some(AccountMapper::mapEntityToDomain(
            account_entity,
            activities,
            withdraw_balance,
            deposit_balance,
        ));
    }
}
