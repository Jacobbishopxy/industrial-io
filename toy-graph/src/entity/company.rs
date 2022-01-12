//! Vertex
//!
//!

use anyhow::Context;
use mongodb::bson::{to_document, Document};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use crate::TGResult;

use super::{Industry, VertexOption, ID};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub category: Industry,
    pub group: Option<String>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl From<&Company> for Document {
    fn from(v: &Company) -> Self {
        to_document(v).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyDto<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub group: Option<&'a str>,
    pub data: Option<JsonValue>,
    pub option: VertexOption,
}

impl<'a> CompanyDto<'a> {
    pub fn new(
        name: &'a str,
        category: &'a str,
        group: Option<&'a str>,
        data: Option<JsonValue>,
        option: VertexOption,
    ) -> Self {
        CompanyDto {
            name,
            category,
            group,
            data,
            option,
        }
    }

    pub fn to_company(self) -> TGResult<Company> {
        let company = Company {
            id: None,
            name: self.name.to_string(),
            category: self.category.parse().context("CompanyDto -> Company")?,
            group: self.group.map(str::to_string),
            data: self.data,
            option: self.option,
        };

        Ok(company)
    }
}

#[test]
fn test_company_dto_conversion() {
    // invalid category
    let dto = CompanyDto::new(
        "name",
        "invalid_category",
        None,
        None,
        VertexOption::default(),
    );
    let company = dto.to_company();
    assert!(company.is_err());

    // valid category
    let dto = CompanyDto::new(
        "name",
        "MacroStrategy",
        Some("group-1"),
        None,
        VertexOption::default(),
    );
    let company = dto.to_company();
    assert!(company.is_ok());
}
