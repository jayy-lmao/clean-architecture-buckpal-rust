pub struct AccountId(usize);

impl AccountId {
    fn new (id: usize)->Self{
        Self(id.to_owned())
    }
}
