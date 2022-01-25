//! Repository
//!
//! Trait

use async_trait::async_trait;

use crate::domain::*;
use crate::TGResult;

#[async_trait]
pub trait Repository: Send + Sync {
    // ===========================================================================
    // main
    // ===========================================================================

    async fn show_catalogue(&self) -> TGResult<Vec<Category>>;

    async fn get_metadata(&self, id: ID) -> TGResult<Category>;

    // async fn get_contents(&self, id: ID) -> TGResult<View>;

    // ===========================================================================
    // category
    // ===========================================================================

    async fn get_category(&self, id: ID) -> TGResult<Category>;

    async fn save_category(&self, category: Category) -> TGResult<Category>;

    async fn delete_category(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, id: ID) -> TGResult<Company>;

    async fn save_company(&self, company: Company) -> TGResult<Company>;

    async fn delete_company(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, id: ID) -> TGResult<Property>;

    async fn save_property(&self, property: Property) -> TGResult<Property>;

    async fn delete_property(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, id: ID) -> TGResult<Relationship>;

    async fn save_relationship(&self, relationship: Relationship) -> TGResult<Relationship>;

    async fn delete_relationship(&self, id: ID) -> TGResult<()>;
}
