//! Company
//!
//! (A company is a node in the graph.)
//! Companies are the basic entities in Industrial-IO.
//! They can be chained together by relationships.

use anyhow::Context;
use crud::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{EntityType, Industry, VertexOption, ID};
use crate::TGResult;

#[derive(Serialize, Deserialize, Debug, Clone, CRUD)]
pub struct Company {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub etype: EntityType,
    pub name: String,
    pub category: Industry,
    pub group: Option<String>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl Company {
    pub fn new<T, I>(
        name: T,
        category: I,
        group: Option<T>,
        data: Option<JsonValue>,
        option: Option<VertexOption>,
    ) -> TGResult<Self>
    where
        T: Into<String>,
        I: AsRef<str>,
    {
        let company = Self {
            id: None,
            etype: EntityType::Company,
            name: name.into(),
            category: category.as_ref().parse().context("CompanyDto -> Company")?,
            group: group.map(Into::into),
            data,
            option: option.unwrap_or_default(),
        };

        Ok(company)
    }
}
