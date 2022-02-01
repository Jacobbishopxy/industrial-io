//! Category

use crud::*;
use serde::{Deserialize, Serialize};

use super::ID;

/// Catalog of all graphs.
/// A category is a collection of one specific graph.
#[derive(Serialize, Deserialize, Debug, Clone, Default, GrantCRUD)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn new<T: Into<String>>(name: T, description: Option<T>) -> Self {
        Self {
            id: None,
            name: name.into(),
            description: description.map(Into::into),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
