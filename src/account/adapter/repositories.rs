use super::entities::AccountEntity;
use super::entities::ActivityEntity;
use super::entities::BalanceEntity;

use anyhow;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use sqlx;
use sqlx::sqlite::SqlitePool;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
    pub static ref DB_POOL: SqlitePool =
        SqlitePool::connect_lazy(DATABASE_URL.as_str()).expect("Could not connect to database");
}

pub struct DbError(String);
pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new() -> Self {
        println!("DBURL: {}", DATABASE_URL.as_str());
        Self {
            pool: DB_POOL.clone(),
        }
    }

    pub async fn findById(&self, id: i64) -> anyhow::Result<AccountEntity> {
        let account = sqlx::query_as!(AccountEntity, r#"SELECT * FROM accounts WHERE id = $1"#, id)
            .fetch_one(&self.pool)
            .await?;
        Ok(account)
    }
}

pub struct ActivityRepository {
    pool: SqlitePool,
}

impl ActivityRepository {
    pub fn new() -> Self {
        Self {
            pool: DB_POOL.clone(),
        }
    }

    pub async fn findById(&self, id: i64) -> anyhow::Result<Vec<ActivityEntity>> {
        let activities = sqlx::query_as!(
            ActivityEntity,
            r#"SELECT * FROM activities WHERE id = $1"#,
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(activities)
    }

    pub async fn insertActivities(&self, activities: Vec<ActivityEntity>) -> anyhow::Result<()> {
        let values: String = activities
            .into_iter()
            .map(|a| {
                format!(
                    "({}, \"{}\", {}, {}, {})",
                    a.amount, a.timestamp, a.ownerAccountId, a.sourceAccountId, a.targetAccountId
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!(
            "INSERT INTO activities (
            amount,
            timestamp,
            ownerAccountId,
            sourceAccountId,
            targetAccountId
        ) values {};",
            values
        );
        dbg!(&query);

        let res = sqlx::query(&query[..]).execute(&self.pool).await;
        dbg!(res);
        Ok(())
    }

    pub async fn findLatestByOwner(
        &self,
        ownerAccountId: i64,
    ) -> anyhow::Result<Option<ActivityEntity>> {
        let activity = sqlx::query_as!(
            ActivityEntity,
            r#"SELECT * FROM activities 
            WHERE ownerAccountId = $1
            ORDER BY timestamp DESC
            LIMIT 1"#,
            ownerAccountId,
        )
        .fetch_one(&self.pool)
        .await;

        match activity {
            Ok(activity) => Ok(Some(activity)),
            Err(error) => {
                if let error = sqlx::Error::RowNotFound {
                    return Ok(None);
                }
                return Err(anyhow::Error::new(error));
            }
        }
    }

    pub async fn findByOwnerSince(
        &self,
        ownerAccountId: i64,
        since: NaiveDateTime,
    ) -> anyhow::Result<Vec<ActivityEntity>> {
        let activities = sqlx::query_as!(
            ActivityEntity,
            r#"SELECT * FROM activities 
            WHERE ownerAccountId = $1
            AND timestamp >= $2
            "#,
            ownerAccountId,
            since,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(activities)
    }
    pub async fn getDepositBalance(
        &self,
        accountId: i64,
        until: NaiveDateTime,
    ) -> anyhow::Result<f32> {
        /*
         *     let (is_connected,): (bool,) = sqlx::query_as("SELECT true")
        .fetch_one(&req.state().clone().db_pool)
        .await?;
         * */
        let balance: (f32,) = sqlx::query_as(
            "
           SELECT sum(amount) as totalAmount FROM activities
           WHERE targetAccountId = $1
           AND ownerAccountId = $2
           and timestamp < $3
           ",
        )
        .bind(accountId)
        .bind(accountId)
        .bind(until)
        .fetch_one(&self.pool)
        .await?;
        Ok(balance.0)
    }

    pub async fn getWithdrawalBalance(
        &self,
        accountId: i64,
        until: NaiveDateTime,
    ) -> anyhow::Result<f32> {
        /*
         *     let (is_connected,): (bool,) = sqlx::query_as("SELECT true")
        .fetch_one(&req.state().clone().db_pool)
        .await?;
         * */
        let balance: (f32,) = sqlx::query_as(
            "
           SELECT sum(amount) FROM activities
           WHERE sourceAccountId = $1
           AND ownerAccountId = $2
           and timestamp < $3
           ",
        )
        .bind(accountId)
        .bind(accountId)
        .bind(until)
        .fetch_one(&self.pool)
        .await?;

        Ok(balance.0)
    }
}
