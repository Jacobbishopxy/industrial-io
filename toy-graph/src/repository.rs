//! Repository
//!
//! Trait

use async_trait::async_trait;

use crate::entity::*;
use crate::TGResult;

#[async_trait]
pub trait Repository: Send + Sync {
    // TODO: methods defined as unit process
    async fn show_catalogue(&self) -> TGResult<Vec<Category>>;

    async fn get_category(&self, id: ID) -> TGResult<Category>;

    async fn new_category<'a>(&self, dto: CategoryDto<'a>) -> TGResult<Category>;
}
