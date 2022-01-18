//! Category
//!
//! Category is a collection of graphs' metadata.

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::ID;

/// Category
///
/// name: collection of a graph
#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
