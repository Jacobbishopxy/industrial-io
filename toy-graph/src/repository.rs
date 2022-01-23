//! Repository
//!
//! Trait

use async_trait::async_trait;
use bson::Bson;
use serde::Serialize;

use crate::entity::*;
use crate::infra::{MongoClient, MongoClientFactory};
use crate::{TGError, TGResult};

pub trait IDMutator {
    fn mutate_id(&mut self, id: Bson) -> TGResult<()>;
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
        res.mutate_id(insert.inserted_id)?;
        Ok(res)
    }
}

impl IDMutator for Company {
    fn mutate_id(&mut self, id: Bson) -> TGResult<()> {
        let id = id.as_object_id().ok_or(TGError::InvalidID)?;
        self.id = Some(id);
        Ok(())
    }
}

#[async_trait]
impl<'a> CRUD<CompanyDto<'a>, Company> for MongoClient {}

impl IDMutator for Category {
    fn mutate_id(&mut self, id: Bson) -> TGResult<()> {
        let id = id.as_object_id().ok_or(TGError::InvalidID)?;
        self.id = Some(id);
        Ok(())
    }
}

#[async_trait]
impl<'a> CRUD<CategoryDto<'a>, Category> for MongoClient {}

#[cfg(test)]
mod test_crud {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let uri = "mongodb://root:secret@localhost:27017";
        let client = MongoClient::new(uri, "test").await.unwrap();

        let dto = CompanyDto::new("test", "MacroStrategy", None, None, VertexOption::default());

        client.create("graph", "company", dto).await.unwrap();

        let dto = CategoryDto::new("test", Some("233"));

        client.create("graph", "category", dto).await.unwrap();
    }
}

#[async_trait]
pub trait Repository: Send + Sync {
    // ===========================================================================
    // main
    // ===========================================================================

    async fn show_catalogue(&self) -> TGResult<Vec<Category>>;

    async fn get_metadata(&self, id: ID) -> TGResult<Category>;

    async fn get_contents(&self, id: ID) -> TGResult<View>;

    // ===========================================================================
    // category
    // ===========================================================================

    async fn get_category(&self, id: ID) -> TGResult<Category>;

    async fn create_category<'a>(&self, dto: CategoryDto<'a>) -> TGResult<Category>;

    async fn update_category(&self, category: Category) -> TGResult<Category>;

    async fn delete_category(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // company
    // ===========================================================================

    async fn get_company(&self, id: ID) -> TGResult<Company>;

    async fn create_company<'a>(&self, dto: CompanyDto<'a>) -> TGResult<Company>;

    async fn update_company(&self, company: Company) -> TGResult<Company>;

    async fn delete_company(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // property
    // ===========================================================================

    async fn get_property(&self, id: ID) -> TGResult<Property>;

    async fn create_property<'a>(&self, dto: PropertyDto<'a>) -> TGResult<Property>;

    async fn update_property(&self, property: Property) -> TGResult<Property>;

    async fn delete_property(&self, id: ID) -> TGResult<()>;

    // ===========================================================================
    // relationship
    // ===========================================================================

    async fn get_relationship(&self, id: ID) -> TGResult<Relationship>;

    async fn create_relationship(&self, dto: RelationshipDto) -> TGResult<Relationship>;

    async fn update_relationship(&self, relationship: Relationship) -> TGResult<Relationship>;

    async fn delete_relationship(&self, id: ID) -> TGResult<()>;
}
