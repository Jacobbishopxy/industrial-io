//! Relationship
//!
//! A relationship is a connection between two entities.

use crud::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{EdgeOption, Weight, ID};

#[derive(Serialize, Deserialize, Debug, Clone, Default, GrantCRUD)]
pub struct Relationship {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub source: ID,
    pub target: ID,
    pub weight: Option<Weight>,
    pub data: Option<JsonValue>,
    pub option: EdgeOption,
}

impl Relationship {
    pub fn new(
        source: ID,
        target: ID,
        weight: Option<Weight>,
        data: Option<JsonValue>,
        option: Option<EdgeOption>,
    ) -> Self {
        Self {
            id: None,
            source,
            target,
            weight,
            data,
            option: option.unwrap_or_default(),
        }
    }
}
