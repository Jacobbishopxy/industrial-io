//! Domain

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
    async fn show_catalogue(&self) -> TGResult<Vec<Category>> {
        self.repository.show_catalogue().await
    }

    // TODO: a graph is a new data structure containing companies and relationships
    async fn save_graph(&self, graph: ()) -> TGResult<()> {
        todo!()
    }
}
