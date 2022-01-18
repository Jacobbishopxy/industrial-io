//! View
//!
//! A view is a graph of companies and relationships.

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entity::*;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub enum Data {
    Company(Either<CompanyDto, Company>),
    Property(Either<PropertyDto, Property>),
    Relationship(Either<RelationshipDto, Relationship>),
}

// TODO: better expression?
pub type Chain = Vec<Data>;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct View {
    pub category: Category,
    pub chains: Vec<Chain>,
}
