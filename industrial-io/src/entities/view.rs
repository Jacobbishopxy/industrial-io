//! View
//!
//! This is the view of the entity.

use serde::{Deserialize, Serialize};

use super::{Category, Company, Property, Relationship};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct View {
    pub category: Category,
    pub companies: Vec<Company>,
    pub properties: Vec<Property>,
    pub relationships: Vec<Relationship>,
}
