#[derive(Clone, Copy)]
pub struct AccountId(usize);

impl AccountId {
    pub fn new(id: usize) -> Self {
        Self(id.to_owned())
    }
}

impl PartialEq for AccountId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
