use crate::account::adapter::entities::AccountEntity;
use crate::account::adapter::entities::ActivityEntity;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepositoryTrait {
    fn find_by_id(&self, id: i64) -> Option<AccountEntity>;
}
pub trait ActivityRepositoryTrait {
    fn find_by_id(&self, id: i64) -> Option<ActivityEntity>;
}
