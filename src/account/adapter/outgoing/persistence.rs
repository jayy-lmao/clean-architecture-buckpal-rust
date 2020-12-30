use crate::account::adapter::persistence::entities::AccountEntity;
use crate::account::adapter::persistence::entities::ActivityEntity;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepositoryTrait {
    fn find_by_id(&self, id: i64) -> Option<AccountEntity>;
}
pub trait ActivityRepositoryTrait {
    fn find_by_id(&self, id: i64) -> Option<ActivityEntity>;
}
