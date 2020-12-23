use chrono::NaiveDateTime;
use sqlx;

#[derive(sqlx::FromRow)]
pub struct AccountEntity {
    pub id: i64,
}

#[derive(sqlx::FromRow)]
pub struct ActivityEntity {
    pub id: i64,
    pub timestamp: NaiveDateTime,
    pub ownerAccountId: i64,
    pub sourceAccountId: i64,
    pub targetAccountId: i64,
    pub amount: f32,
}
#[derive(sqlx::FromRow)]
pub struct BalanceEntity {
    pub totalAmount: f32,
}
