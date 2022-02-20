//! Repository
//!
//! Trait

use async_trait::async_trait;
use crud::{MongoCRUD, MongoClientFactory};

use crate::entities::*;
use crate::TGResult;

#[async_trait]
pub trait Repository: Send + Sync + MongoClientFactory {
    // ===========================================================================
    // category
    // ===========================================================================

    async fn get_all_category(&self) -> TGResult<Vec<Category>> {
        self.client().read_all().await
    }

    async fn get_category(&self, id: ID) -> TGResult<Option<Category>> {
        self.client().read(id).await
    }

    async fn save_category(&self, category: Category) -> TGResult<Category> {
        self.client().create(category).await
    }

    async fn delete_category(&self, id: ID) -> TGResult<Option<Category>> {
        self.client().delete(id).await
    }

    /// `View` is a collection who contains all the industrial data.
    /// All the `View`s name must be unique.
    async fn get_view(&self, _name: &str) -> TGResult<Option<View>> {
        // TODO: implement
        unimplemented!()
    }

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, id: ID) -> TGResult<Option<Company>> {
        self.client().read(id).await
    }

    async fn save_company(&self, company: Company) -> TGResult<Company> {
        self.client().create(company).await
    }

    async fn delete_company(&self, id: ID) -> TGResult<Option<Company>> {
        self.client().delete(id).await
    }

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, id: ID) -> TGResult<Option<Property>> {
        self.client().read(id).await
    }

    async fn save_property(&self, property: Property) -> TGResult<Property> {
        self.client().create(property).await
    }

    async fn delete_property(&self, id: ID) -> TGResult<Option<Property>> {
        self.client().delete(id).await
    }

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, id: ID) -> TGResult<Option<Relationship>> {
        self.client().read(id).await
    }

    async fn save_relationship(&self, relationship: Relationship) -> TGResult<Relationship> {
        self.client().create(relationship).await
    }

    async fn delete_relationship(&self, id: ID) -> TGResult<Option<Relationship>> {
        self.client().delete(id).await
    }
}
