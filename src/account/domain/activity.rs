use crate::account::domain::*;
use chrono::{DateTime, Local};

pub struct Activity {
    pub fromAccount: AccountId,
    pub toAccount: AccountId,
    pub timestamp: DateTime<Local>,
    pub money: Money,
}
