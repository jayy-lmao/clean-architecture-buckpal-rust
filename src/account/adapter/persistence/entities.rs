use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct AccountEntity {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ActivityEntity {
    pub id: i64,
    pub timestamp: NaiveDateTime,
    pub owner_account_id: i64,
    pub source_account_id: i64,
    pub target_account_id: i64,
    pub amount: f32,
}
