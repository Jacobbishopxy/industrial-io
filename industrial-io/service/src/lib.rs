//! Service
//!
//! Implementation of Domain's `Repository`

pub mod provider;

use async_trait::async_trait;
use crud::*;
use domain::entities::{Category, Company, Property, Relationship, View, ID};
use domain::{Repository, TGResult};
use provider::Provider;

#[async_trait]
impl Repository for Provider {
    async fn get_all_category(&self) -> TGResult<Vec<Category>> {
        self.persistence_client.read_all().await
    }

    async fn get_category(&self, id: ID) -> TGResult<Option<Category>> {
        self.persistence_client.read(id).await
    }

    async fn save_category(&self, category: Category) -> TGResult<Category> {
        self.persistence_client.create(category).await
    }

    async fn delete_category(&self, id: ID) -> TGResult<Option<Category>> {
        self.persistence_client.delete(id).await
    }

    async fn get_view(&self, _name: &str) -> TGResult<Option<View>> {
        unimplemented!()
    }

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, _id: ID) -> TGResult<Option<Company>> {
        unimplemented!()
    }

    async fn save_company(&self, _company: Company) -> TGResult<Company> {
        unimplemented!()
    }

    async fn delete_company(&self, _id: ID) -> TGResult<Option<Company>> {
        unimplemented!()
    }

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, _id: ID) -> TGResult<Option<Property>> {
        unimplemented!()
    }

    async fn save_property(&self, _property: Property) -> TGResult<Property> {
        unimplemented!()
    }

    async fn delete_property(&self, _id: ID) -> TGResult<Option<Property>> {
        unimplemented!()
    }

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, _id: ID) -> TGResult<Option<Relationship>> {
        unimplemented!()
    }

    async fn save_relationship(&self, _relationship: Relationship) -> TGResult<Relationship> {
        unimplemented!()
    }

    async fn delete_relationship(&self, _id: ID) -> TGResult<Option<Relationship>> {
        unimplemented!()
    }
}
