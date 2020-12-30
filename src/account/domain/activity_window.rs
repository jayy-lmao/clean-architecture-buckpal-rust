use crate::account::domain::*;

#[derive(Debug)]
pub struct ActivityWindow {
    pub activities: Vec<Activity>,
}

impl ActivityWindow {
    pub fn new(activities: Vec<Activity>) -> Self {
        Self { activities }
    }
    pub fn add_activity(&mut self, a: Activity) {
        self.activities.push(a);
    }
    pub fn calculate_balance(&self, id: AccountId) -> Money {
        self.activities
            .iter()
            .map(|a| match (a.to_account == id, a.from_account == id) {
                (true, _) => a.money,
                (_, true) => a.money.clone().negate(),
                _ => Money::new(0.),
            })
            .fold(Money::new(0.), Money::add)
    }
}
