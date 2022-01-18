//! Domain
//!
//! Domain implement all business logic, and use all methods provided by Repository.

use async_trait::async_trait;

use crate::repository::Repository;
use crate::{entity::*, TGResult};

pub struct ToyGraph<T>
where
    T: Repository,
{
    repository: T,
}

impl<T> ToyGraph<T>
where
    T: Repository,
{
    /// show all catalogues
    async fn show_catalogue(&self) -> TGResult<Vec<Category>> {
        self.repository.show_catalogue().await
    }

    /// save a view, which is a graph of companies and relationships
    async fn save_view(&self, view: View) -> TGResult<()> {
        todo!()
    }
}
