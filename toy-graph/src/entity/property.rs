//! Property
//!
//! A generic property for company entities.

use mongodb::bson::{to_document, Document};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{VertexOption, ID};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub label: Option<String>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl From<&Property> for Document {
    fn from(v: &Property) -> Self {
        to_document(v).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyDto<'a> {
    pub name: &'a str,
    pub label: Option<&'a str>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl<'a> PropertyDto<'a> {
    pub fn new(
        name: &'a str,
        label: Option<&'a str>,
        data: Option<JsonValue>,
        option: VertexOption,
    ) -> Self {
        PropertyDto {
            name,
            label,
            data,
            option,
        }
    }

    pub fn to_property(self) -> Property {
        Property {
            id: None,
            name: self.name.to_string(),
            label: self.label.map(str::to_string),
            data: self.data,
            option: self.option,
        }
    }
}
