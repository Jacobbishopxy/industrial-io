//! View
//!
//! A view is a graph of companies and relationships.

// use serde::{Deserialize, Serialize};

use crate::entity::*;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub enum Data<'a> {
    Company(Either<CompanyDto<'a>, Company>),
    Property(Either<PropertyDto<'a>, Property>),
    Relationship(Either<RelationshipDto, Relationship>),
}

// TODO: better expression?
pub type Chain<'a> = Vec<Data<'a>>;

pub struct View<'a> {
    pub category: Category,
    pub chains: Vec<Chain<'a>>,
}
