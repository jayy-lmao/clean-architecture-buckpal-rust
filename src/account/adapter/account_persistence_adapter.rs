use std::unimplemented;

use crate::account::adapter::entities::*;
use crate::account::adapter::repositories::{AccountRepository, ActivityRepository};
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::application::port::outgoing::update_account_state_port::UpdateAccountStatePort;
use crate::account::domain::*;
use anyhow;
use async_trait::async_trait;
use chrono::NaiveDateTime;

pub mod ActivityMapper {
    use super::*;
    pub fn mapEntityToDomain(activity_entity: &ActivityEntity) -> Activity {
        Activity {
            fromAccount: AccountId::new(activity_entity.sourceAccountId),
            toAccount: AccountId::new(activity_entity.targetAccountId),
            timestamp: activity_entity.timestamp,
            money: Money::new(activity_entity.amount),
        }
    }
    pub fn mapDomainToEntity(activity: &Activity, ownerAccountId: i64) -> ActivityEntity {
        ActivityEntity {
            id: -1, // this should be ignored by insert
            ownerAccountId: ownerAccountId,
            sourceAccountId: activity.fromAccount.to_i64(),
            targetAccountId: activity.toAccount.to_i64(),
            timestamp: activity.timestamp,
            amount: activity.money.to_f32(),
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
            ActivityWindow::new(
                activities
                    .iter()
                    .map(ActivityMapper::mapEntityToDomain)
                    .collect(),
            ),
        )
    }
}

pub struct AccountPersistenceAdapter {
    accountRepository: AccountRepository,
    activityRepository: ActivityRepository,
}

impl AccountPersistenceAdapter {
    pub fn default() -> Self {
        Self {
            accountRepository: AccountRepository::new(),
            activityRepository: ActivityRepository::new(),
        }
    }
}
#[async_trait]
impl UpdateAccountStatePort for AccountPersistenceAdapter {
    async fn updateAccountState(
        &self,
        account: Account,
        timestamp: NaiveDateTime,
    ) -> anyhow::Result<()> {
        dbg!("Updating Account State");
        let latest_recorded_transaction_date = self
            .activityRepository
            .findLatestByOwner(account.id.to_i64())
            .await?;

        let activities: Vec<ActivityEntity> = account
            .activityWindow
            .activities
            .iter()
            .filter(|&a| match &latest_recorded_transaction_date {
                None => true,
                Some(latest_recorded_transaction_date) => {
                    a.timestamp > latest_recorded_transaction_date.timestamp
                }
            })
            .map(|a| ActivityMapper::mapDomainToEntity(a, account.id.to_i64()))
            .collect();

        self.activityRepository
            .insertActivities(dbg!(activities))
            .await?;
        Ok(())
    }
}

#[async_trait]
impl LoadAccountPort for AccountPersistenceAdapter {
    async fn loadAccount(
        &self,
        accountId: AccountId,
        baselineDate: NaiveDateTime,
    ) -> anyhow::Result<Account> {
        let account_entity = self.accountRepository.findById(accountId.to_i64()).await?;

        let activities = self
            .activityRepository
            .findByOwnerSince(accountId.to_i64(), baselineDate)
            .await?;

        let withdraw_balance: f32 = self
            .activityRepository
            .getWithdrawalBalance(accountId.to_i64(), baselineDate)
            .await?;

        let deposit_balance: f32 = self
            .activityRepository
            .getDepositBalance(accountId.to_i64(), baselineDate)
            .await?;

        return Ok(AccountMapper::mapEntityToDomain(
            account_entity,
            activities,
            withdraw_balance,
            deposit_balance,
        ));
    }
}
