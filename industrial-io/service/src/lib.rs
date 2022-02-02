//! Service
//!
//! Implementation of Domain's `Repository`

pub mod provider;

use async_trait::async_trait;
// use crud::*;
use domain::entities::{Category, Company, Property, Relationship, View, ID};
use domain::{Repository, TGResult};
use provider::Provider;

#[async_trait]
impl Repository for Provider {
    async fn get_all_category(&self) -> TGResult<Vec<Category>> {
        unimplemented!()
    }

    async fn get_category(&self, id: ID) -> TGResult<Category> {
        unimplemented!()
    }

    async fn save_category(&self, category: Category) -> TGResult<Category> {
        unimplemented!()
    }

    async fn delete_category(&self, id: ID) -> TGResult<()> {
        unimplemented!()
    }

    async fn get_view(&self, name: &str) -> TGResult<View> {
        unimplemented!()
    }

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, id: ID) -> TGResult<Company> {
        unimplemented!()
    }

    async fn save_company(&self, company: Company) -> TGResult<Company> {
        unimplemented!()
    }

    async fn delete_company(&self, id: ID) -> TGResult<()> {
        unimplemented!()
    }

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, id: ID) -> TGResult<Property> {
        unimplemented!()
    }

    async fn save_property(&self, property: Property) -> TGResult<Property> {
        unimplemented!()
    }

    async fn delete_property(&self, id: ID) -> TGResult<()> {
        unimplemented!()
    }

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, id: ID) -> TGResult<Relationship> {
        unimplemented!()
    }

    async fn save_relationship(&self, relationship: Relationship) -> TGResult<Relationship> {
        unimplemented!()
    }

    async fn delete_relationship(&self, id: ID) -> TGResult<()> {
        unimplemented!()
    }
}
