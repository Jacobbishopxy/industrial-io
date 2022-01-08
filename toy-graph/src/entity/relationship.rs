//! Relationship
//!
//! A relationship is a connection between two entities.

use bson::oid::ObjectId;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::EdgeOption;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relationship {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub source: ObjectId,
    pub target: ObjectId,
    pub weight: Option<f64>,
    pub data: Option<JsonValue>,
    pub option: EdgeOption,
}
