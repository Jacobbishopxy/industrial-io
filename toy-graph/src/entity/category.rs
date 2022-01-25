//! Category
//!
//! Category is a collection of graphs' metadata.

use bson::{to_document, Document};
use serde::{Deserialize, Serialize};

use crud_derive::*;

use super::{IDMutator, ID};
use crate::TGResult;

/// Category
///
/// name: collection of a graph
#[derive(Serialize, Deserialize, Debug, Clone, CRUD)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryDto<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

impl<'a> CategoryDto<'a> {
    pub fn new(name: &'a str, description: Option<&'a str>) -> Self {
        CategoryDto { name, description }
    }

    pub fn to_catalog(self) -> Category {
        Category {
            id: None,
            name: self.name.to_string(),
            description: self.description.map(str::to_string),
        }
    }
}

impl<'a> From<CategoryDto<'a>> for Document {
    fn from(v: CategoryDto<'a>) -> Self {
        to_document(&v).unwrap()
    }
}

impl<'a> TryFrom<CategoryDto<'a>> for Category {
    type Error = anyhow::Error;

    fn try_from(value: CategoryDto<'a>) -> Result<Self, Self::Error> {
        Ok(value.to_catalog())
    }
}
