use crate::account::domain::*;

pub struct ActivityWindow {
    activities: Vec<Activity>,
}

impl ActivityWindow {
    pub fn new(
    activities: Vec<Activity>,
        ) -> Self {
        Self {
            activities,
        }
    }
    pub fn addActivity(&mut self, a: Activity) {
        self.activities.push(a);
    }
    pub fn calculateBalance(&self, id: AccountId) -> Money {
        self.activities
            .iter()
            .map(|a| match (a.toAccount == id, a.fromAccount == id) {
                (true, _) => a.money,
                (_, true) => a.money.clone().negate(),
                _ => Money::new(0.),
            })
            .fold(Money::new(0.), |acc, x| Money::add(acc, x))
    }
}
