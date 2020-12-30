use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::application::port::outgoing::update_account_state_port::UpdateAccountStatePort;
use crate::account::domain::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::*;
use repositories::*;

use super::{entities, repositories};

pub mod activity_mapper {
    use super::*;
    pub fn map_entity_to_domain(activity_entity: &ActivityEntity) -> Activity {
        Activity {
            from_account: AccountId::new(activity_entity.source_account_id),
            to_account: AccountId::new(activity_entity.target_account_id),
            timestamp: activity_entity.timestamp,
            money: Money::new(activity_entity.amount),
        }
    }
    pub fn map_domain_to_entity(activity: &Activity, owner_account_id: i64) -> ActivityEntity {
        ActivityEntity {
            id: -1, // this should be ignored by insert
            owner_account_id,
            source_account_id: activity.from_account.to_i64(),
            target_account_id: activity.to_account.to_i64(),
            timestamp: activity.timestamp,
            amount: activity.money.to_f32(),
        }
    }
}

pub mod account_mapper {
    use super::*;
    pub fn map_entity_to_domain(
        account_entity: AccountEntity,
        activities: Vec<ActivityEntity>,
        withdraw_balance: f32,
        deposit_balance: f32,
    ) -> Account {
        Account::with_id(
            AccountId::new(account_entity.id),
            Money::new(deposit_balance - withdraw_balance),
            ActivityWindow::new(
                activities
                    .iter()
                    .map(activity_mapper::map_entity_to_domain)
                    .collect(),
            ),
        )
    }
}

pub struct AccountPersistenceAdapter {
    account_repository: AccountRepository,
    activity_repository: ActivityRepository,
}

impl AccountPersistenceAdapter {
    pub fn default() -> Self {
        Self {
            account_repository: AccountRepository::new(),
            activity_repository: ActivityRepository::new(),
        }
    }
}
#[async_trait]
impl UpdateAccountStatePort for AccountPersistenceAdapter {
    async fn update_account_state(
        &self,
        account: Account,
        _timestamp: NaiveDateTime,
    ) -> anyhow::Result<()> {
        let latest_recorded_transaction_date = self
            .activity_repository
            .find_latest_by_owner(account.id.to_i64())
            .await?;

        let activities: Vec<ActivityEntity> = account
            .activity_window
            .activities
            .iter()
            .filter(|&a| match &latest_recorded_transaction_date {
                None => true,
                Some(latest_recorded_transaction_date) => {
                    a.timestamp > latest_recorded_transaction_date.timestamp
                }
            })
            .map(|a| activity_mapper::map_domain_to_entity(a, account.id.to_i64()))
            .collect();

        self.activity_repository
            .insert_activities(activities)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl LoadAccountPort for AccountPersistenceAdapter {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: NaiveDateTime,
    ) -> anyhow::Result<Account> {
        let account_entity = self
            .account_repository
            .find_by_id(account_id.to_i64())
            .await?;

        let activities = self
            .activity_repository
            .find_by_owner_since(account_id.to_i64(), baseline_date)
            .await?;

        let withdraw_balance: f32 = self
            .activity_repository
            .get_withdrawal_balance(account_id.to_i64(), baseline_date)
            .await?;

        let deposit_balance: f32 = self
            .activity_repository
            .get_deposit_balance(account_id.to_i64(), baseline_date)
            .await?;

        Ok(account_mapper::map_entity_to_domain(
            account_entity,
            activities,
            withdraw_balance,
            deposit_balance,
        ))
    }
}
