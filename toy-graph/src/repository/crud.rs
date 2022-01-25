//! CRUD
//!
//! ActiveRecord-like CRUD operations.

use async_trait::async_trait;
use bson::oid::ObjectId;
use serde::Serialize;

use crate::entity::*;
use crate::infra::{MongoClient, MongoClientFactory};
use crate::{TGError, TGResult};

impl IDMutator for Relationship {
    fn mutate_id(&mut self, oid: ObjectId) -> TGResult<()> {
        self.id = Some(oid);
        Ok(())
    }
}

#[async_trait]
pub trait CRUD<DTO, RES>: MongoClientFactory
where
    DTO: Send + Sync + Serialize + Clone,
    RES: TryFrom<DTO, Error = anyhow::Error> + IDMutator,
{
    async fn create<'a>(&'a self, db: &str, collection: &str, t: DTO) -> TGResult<RES>
    where
        DTO: 'a,
    {
        let insert = self
            .collection::<DTO>(db, collection)
            .insert_one(t.clone(), None)
            .await?;
        let mut res = RES::try_from(t)?;
        let oid = insert
            .inserted_id
            .as_object_id()
            .ok_or(TGError::InvalidID)?;
        res.mutate_id(oid)?;
        Ok(res)
    }
}

#[async_trait]
impl<'a> CRUD<CompanyDto<'a>, Company> for MongoClient {}

#[async_trait]
impl<'a> CRUD<CategoryDto<'a>, Category> for MongoClient {}

#[async_trait]
impl<'a> CRUD<PropertyDto<'a>, Property> for MongoClient {}

#[async_trait]
impl CRUD<RelationshipDto, Relationship> for MongoClient {}

#[cfg(test)]
mod test_crud {
    use super::*;

    const DB_NAME: &str = "test";
    const COLL_NAME: &str = "dev";

    #[tokio::test]
    async fn test_create() {
        let uri = "mongodb://root:secret@localhost:27017";
        let client = MongoClient::new(uri, "test").await.unwrap();

        // create category
        let dto = CategoryDto::new("test", Some("233"));
        let res = client.create(DB_NAME, COLL_NAME, dto).await;
        assert!(res.is_ok());

        // create company
        let dto = CompanyDto::new("test", "MacroStrategy", None, None, VertexOption::default());
        let res = client.create(DB_NAME, COLL_NAME, dto).await;
        assert!(res.is_ok());

        // create property
        let dto = PropertyDto::new("test", None, None, VertexOption::default());
        let res = client.create(DB_NAME, COLL_NAME, dto).await;
        assert!(res.is_ok());
    }
}
