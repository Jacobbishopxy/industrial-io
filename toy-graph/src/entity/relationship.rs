//! Relationship
//!
//! A relationship is a connection between two entities.

use bson::{to_document, Document};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{EdgeOption, ID};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relationship {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub source: ID,
    pub target: ID,
    pub weight: Option<f64>,
    pub data: Option<JsonValue>,
    pub option: EdgeOption,
}

impl From<&Relationship> for Document {
    fn from(v: &Relationship) -> Self {
        to_document(v).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelationshipDto {
    pub source: ID,
    pub target: ID,
    pub weight: Option<f64>,
    pub data: Option<JsonValue>,
    pub option: EdgeOption,
}

impl RelationshipDto {
    pub fn new(
        source: ID,
        target: ID,
        weight: Option<f64>,
        data: Option<JsonValue>,
        option: EdgeOption,
    ) -> Self {
        RelationshipDto {
            source,
            target,
            weight,
            data,
            option,
        }
    }

    pub fn to_relationship(self) -> Relationship {
        Relationship {
            id: None,
            source: self.source,
            target: self.target,
            weight: self.weight,
            data: self.data,
            option: self.option,
        }
    }
}
