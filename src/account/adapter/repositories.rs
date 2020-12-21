use super::entities::AccountEntity;
use super::entities::ActivityEntity;
use super::entities::BalanceEntity;

use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use sqlx;
use sqlx::sqlite::SqlitePool;
use std::env;
use std::result::Result;

lazy_static! {
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
    pub static ref DB_POOL: SqlitePool =
        SqlitePool::connect_lazy("DATABASE_URL").expect("Could not connect to database");
}

pub struct DbError(String);
pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    fn new() -> Self {
        Self {
            pool: DB_POOL.clone(),
        }
    }

    pub async fn findById(&self, id: i64) -> Result<AccountEntity, sqlx::Error> {
        let account = sqlx::query_as!(AccountEntity, r#"SELECT * FROM accounts WHERE id = $1"#, id)
            .fetch_one(&self.pool)
            .await;

        account
    }
}

pub struct ActivityRepository {
    pool: SqlitePool,
}

impl ActivityRepository {
    fn new() -> Self {
        Self {
            pool: DB_POOL.clone(),
        }
    }

    async fn findById(&self, id: i64) -> Result<Vec<ActivityEntity>, sqlx::Error> {
        let activities = sqlx::query_as!(
            ActivityEntity,
            r#"SELECT * FROM activities WHERE id = $1"#,
            id
        )
        .fetch_all(&self.pool)
        .await;

        activities
    }
    async fn findByOwnerSince(
        &self,
        ownerAccountId: i64,
        since: NaiveDateTime,
    ) -> Result<Vec<ActivityEntity>, sqlx::Error> {
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
        .await;

        activities
    }
    async fn getDepositBalance(
        &self,
        accountId: i64,
        until: NaiveDateTime,
    ) -> Result<f32, sqlx::Error> {
        /*
         *     let (is_connected,): (bool,) = sqlx::query_as("SELECT true")
        .fetch_one(&req.state().clone().db_pool)
        .await?;
         * */
        let balance = sqlx::query_as::<_, BalanceEntity>(
            "
           SELECT sum(amount) FROM activities
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

        Ok(balance.totalAmount)
    }

    async fn getWithdrawalBalance(
        &self,
        accountId: i64,
        until: NaiveDateTime,
    ) -> Result<f32, sqlx::Error> {
        /*
         *     let (is_connected,): (bool,) = sqlx::query_as("SELECT true")
        .fetch_one(&req.state().clone().db_pool)
        .await?;
         * */
        let balance = sqlx::query_as::<_, BalanceEntity>(
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

        Ok(balance.totalAmount)
    }
}
