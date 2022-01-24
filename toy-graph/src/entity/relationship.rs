//! Relationship
//!
//! A relationship is a connection between two entities.

use bson::{to_document, Document};
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

use super::{EdgeOption, Weight, ID};
use crate::{TGError, TGResult};

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
    pub joint: Option<(ID, ID)>,
    pub weight: Option<Weight>,
    pub data: Option<JsonValue>,
    pub option: EdgeOption,
}

impl RelationshipDto {
    pub fn new(
        joint: Option<(ID, ID)>,
        weight: Option<Weight>,
        data: Option<JsonValue>,
        option: EdgeOption,
    ) -> Self {
        RelationshipDto {
            joint,
            weight,
            data,
            option,
        }
    }

    pub fn to_relationship(self) -> TGResult<Relationship> {
        let joint = self.joint.as_ref().ok_or(TGError::IDNotFound)?;
        let rl = Relationship {
            id: None,
            source: joint.0,
            target: joint.1,
            weight: self.weight,
            data: self.data,
            option: self.option,
        };
        Ok(rl)
    }
}

impl From<RelationshipDto> for Document {
    fn from(value: RelationshipDto) -> Self {
        to_document(&value).unwrap()
    }
}

impl TryFrom<RelationshipDto> for Relationship {
    type Error = anyhow::Error;

    fn try_from(value: RelationshipDto) -> Result<Self, Self::Error> {
        value.to_relationship()
    }
}
