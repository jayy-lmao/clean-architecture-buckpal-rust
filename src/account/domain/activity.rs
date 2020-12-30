use crate::account::domain::*;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Activity {
    pub from_account: AccountId,
    pub to_account: AccountId,
    pub timestamp: NaiveDateTime,
    pub money: Money,
}
