//! Entity
//!
//! 1. Company
//! 1. Property
//! 1. Relationship
//!

pub mod company;
pub mod property;
pub mod relationship;

pub use company::*;
pub use property::*;
pub use relationship::*;

use mongodb::bson::oid::ObjectId;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct PureId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Position {
    Left,
    Right,
    Top,
    Bottom,
}

/*
Vertex option
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VertexType {
    Default,
    Input,
    Output,
    Custom(String),
}

impl Default for VertexType {
    fn default() -> Self {
        VertexType::Default
    }
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VertexOption {
    pub position: (i64, i64),
    pub vtype: VertexType,
    pub data: Option<JsonValue>,
    pub target_position: Position,
    pub source_position: Position,
}

impl Default for VertexOption {
    fn default() -> Self {
        Self {
            position: (0, 0),
            vtype: Default::default(),
            data: Default::default(),
            target_position: Position::Top,
            source_position: Position::Bottom,
        }
    }
}

/*
Edge option
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EdgeType {
    Bezier,
    Straight,
    Step,
    Smoothstep,
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::Bezier
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArrowType {
    Arrow,
    ArrowClosed,
}

impl Default for ArrowType {
    fn default() -> Self {
        ArrowType::Arrow
    }
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeOption {
    pub etype: EdgeType,
    pub style: Option<JsonValue>,
    pub data: Option<JsonValue>,
    pub animated: bool,
    pub label: Option<String>,
    pub label_style: Option<JsonValue>,
    pub arrow_type: ArrowType,
    pub source_position: Position,
    pub target_position: Position,
}

impl Default for EdgeOption {
    fn default() -> Self {
        Self {
            etype: EdgeType::Bezier,
            style: None,
            data: None,
            animated: false,
            label: None,
            label_style: None,
            arrow_type: ArrowType::Arrow,
            source_position: Position::Bottom,
            target_position: Position::Top,
        }
    }
}
