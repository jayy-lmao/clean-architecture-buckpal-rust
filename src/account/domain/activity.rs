use crate::account::domain::*;
use chrono::NaiveDateTime;

pub struct Activity {
    pub fromAccount: AccountId,
    pub toAccount: AccountId,
    pub timestamp: NaiveDateTime,
    pub money: Money,
}
