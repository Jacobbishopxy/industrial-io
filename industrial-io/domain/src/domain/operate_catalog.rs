//! Show Catalog
//!
//! Bushiness logic of catalog.

use crate::entities::{Category, View, ID};
use crate::repository::Repository;
use crate::TGResult;

pub struct OperateCatalog<T: Repository> {
    repo: T,
}

impl<T: Repository> OperateCatalog<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn show_catalog(&self) -> TGResult<Vec<Category>> {
        self.repo.get_all_category().await
    }

    pub async fn get_view_metadata(&self, id: ID) -> TGResult<Category> {
        self.repo.get_category(id).await
    }

    pub async fn save_view_metadata(&self, category: Category) -> TGResult<Category> {
        self.repo.save_category(category).await
    }

    pub async fn delete_view_metadata(&self, id: ID) -> TGResult<()> {
        self.repo.delete_category(id).await
    }

    pub async fn get_view_by_name(&self, name: &str) -> TGResult<View> {
        self.repo.get_view(name).await
    }

    pub async fn get_view_by_category_id(&self, id: ID) -> TGResult<View> {
        let category = self.repo.get_category(id).await?;
        let view_name = category.name();

        self.repo.get_view(view_name).await
    }
}
