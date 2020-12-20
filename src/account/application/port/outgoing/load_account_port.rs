use crate::account::domain::*;
use chrono::prelude::{DateTime, Local};

pub trait LoadAccountPort {
    fn loadAccount(&self, accountId: AccountId, timeStamp: DateTime<Local>) -> Account;
}
