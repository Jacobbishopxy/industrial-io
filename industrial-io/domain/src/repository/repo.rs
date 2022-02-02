//! Repository
//!
//! Trait

use async_trait::async_trait;

use crate::entities::*;
use crate::TGResult;

#[async_trait]
pub trait Repository: Send + Sync {
    // ===========================================================================
    // category
    // ===========================================================================

    async fn get_all_category(&self) -> TGResult<Vec<Category>>;

    async fn get_category(&self, id: ID) -> TGResult<Option<Category>>;

    async fn save_category(&self, category: Category) -> TGResult<Category>;

    async fn delete_category(&self, id: ID) -> TGResult<Option<Category>>;

    /// `View` is a collection who contains all the industrial data.
    /// All the `View`s name must be unique.
    async fn get_view(&self, name: &str) -> TGResult<Option<View>>;

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, id: ID) -> TGResult<Option<Company>>;

    async fn save_company(&self, company: Company) -> TGResult<Company>;

    async fn delete_company(&self, id: ID) -> TGResult<Option<Company>>;

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, id: ID) -> TGResult<Option<Property>>;

    async fn save_property(&self, property: Property) -> TGResult<Property>;

    async fn delete_property(&self, id: ID) -> TGResult<Option<Property>>;

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, id: ID) -> TGResult<Option<Relationship>>;

    async fn save_relationship(&self, relationship: Relationship) -> TGResult<Relationship>;

    async fn delete_relationship(&self, id: ID) -> TGResult<Option<Relationship>>;
}
