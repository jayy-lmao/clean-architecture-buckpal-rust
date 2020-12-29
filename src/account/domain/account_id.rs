#[derive(Debug, Clone, Copy)]
pub struct AccountId(i64);

impl AccountId {
    pub fn new(id: i64) -> Self {
        Self(id.to_owned())
    }
    pub fn to_i64(&self) -> i64 {
        self.0
    }
}

impl PartialEq for AccountId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
