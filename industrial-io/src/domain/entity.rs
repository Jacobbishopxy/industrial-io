//! Entity
//!
//! 1. Catalog
//! 1. Company
//! 1. Property
//! 1. Relationship
//! 1. View

use std::str::FromStr;

use anyhow::Context;
use crud::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{EdgeOption, VertexOption, Weight, ID};
use crate::{TGError, TGResult};

/// Industry
///
/// Represents the industry of a company.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Industry {
    MacroStrategy,
    CyclicalIndustry,
    TechnologyMediaTelecom,
    Internet,
    ConsumerIndustry,
    PublicServiceAndMonopoly,
    FinanceAndRealEstate,
    AdvancedManufacturing,
    AgricultureAndAquaculture,
    HealthAndMedicine,
    PowerSystemAndEnergy,
    Electronics,
}

impl FromStr for Industry {
    type Err = TGError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "MacroStrategy" => Ok(Industry::MacroStrategy),
            "CyclicalIndustry" => Ok(Industry::CyclicalIndustry),
            "TechnologyMediaTelecom" => Ok(Industry::TechnologyMediaTelecom),
            "Internet" => Ok(Industry::Internet),
            "ConsumerIndustry" => Ok(Industry::ConsumerIndustry),
            "PublicServiceAndMonopoly" => Ok(Industry::PublicServiceAndMonopoly),
            "FinanceAndRealEstate" => Ok(Industry::FinanceAndRealEstate),
            "AdvancedManufacturing" => Ok(Industry::AdvancedManufacturing),
            "AgricultureAndAquaculture" => Ok(Industry::AgricultureAndAquaculture),
            "HealthAndMedicine" => Ok(Industry::HealthAndMedicine),
            "PowerSystemAndEnergy" => Ok(Industry::PowerSystemAndEnergy),
            "Electronics" => Ok(Industry::Electronics),
            _ => Err(TGError::Parse(value.to_string())),
        }
    }
}

/// Category
///
/// Catalog of all graphs.
/// A category is a collection of one specific graph.
#[derive(Serialize, Deserialize, Debug, Clone, Default, GrantCRUD)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn new<T: Into<String>>(name: T, description: Option<T>) -> Self {
        Self {
            id: None,
            name: name.into(),
            description: description.map(Into::into),
        }
    }
}

/// Entity Type
///
/// The type of an entity.
/// An industrial graph, who contains different types of entities,
/// will be saved in a single collection. Hence, entity type turns
/// out to be the only unique identifier of an entity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityType {
    Company,
    Property,
    Relationship,
}

/// Company
///
/// (A company is a node in the graph.)
/// Companies are the basic entities in Industrial-IO.
/// They can be chained together by relationships.
#[derive(Serialize, Deserialize, Debug, Clone, GrantCRUD)]
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

/// Property
///
/// (A property is a node in the graph.)
/// Used for storing company's affiliated data, such as resources,
/// other collections' info and etc.
#[derive(Serialize, Deserialize, Debug, Clone, GrantCRUD)]
pub struct Property {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub etype: EntityType,
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
            etype: EntityType::Property,
            name: name.into(),
            label: label.map(Into::into),
            data,
            option: option.unwrap_or_default(),
        }
    }
}

/// Relationship
///
/// (A relationship is an edge in the graph.)
/// Used for connecting companies or properties.
#[derive(Serialize, Deserialize, Debug, Clone, GrantCRUD)]
pub struct Relationship {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub etype: EntityType,
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
            etype: EntityType::Relationship,
            source,
            target,
            weight,
            data,
            option: option.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test_category {
    use super::*;

    const URI: &str = "mongodb://root:secret@localhost:27017";

    #[tokio::test]
    async fn test_category() {
        let category = Category::new("test", None);
        assert_eq!(category.name, "test");
        assert_eq!(category.description, None);

        let client = MongoClient::new(URI, "test", "dev").await.unwrap();

        let res = client.create(category).await;
        assert!(res.is_ok());
    }
}
