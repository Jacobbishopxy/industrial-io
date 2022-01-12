//! Catalog
//!
//! Catalog is a collection of graphs' metadata.

use bson::oid::ObjectId;
use pyo3::prelude::*;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Catalog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CatalogDto<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

impl<'a> CatalogDto<'a> {
    pub fn new(name: &'a str, description: Option<&'a str>) -> Self {
        CatalogDto { name, description }
    }

    pub fn to_catalog(self) -> Catalog {
        Catalog {
            id: None,
            name: self.name.to_string(),
            description: self.description.map(str::to_string),
        }
    }
}
