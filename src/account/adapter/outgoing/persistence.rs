use crate::account::adapter::entities::AccountEntity;
use crate::account::adapter::entities::ActivityEntity;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepositoryTrait {
    fn findById(&self, id: i64) -> Option<AccountEntity>;
}
pub trait ActivityRepositoryTrait {
    fn findById(&self, id: i64) -> Option<ActivityEntity>;
}
