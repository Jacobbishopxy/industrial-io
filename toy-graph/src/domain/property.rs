//! Property
//!
//! A generic property for company entities.

use crud::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{VertexOption, ID};

#[derive(Serialize, Deserialize, Debug, Clone, Default, GrantCRUD)]
pub struct Property {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub label: Option<String>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl Property {
    pub fn new<T: Into<String>>(
        name: T,
        label: Option<T>,
        data: Option<JsonValue>,
        option: Option<VertexOption>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            label: label.map(Into::into),
            data,
            option: option.unwrap_or_default(),
        }
    }
}
