//! Domain
//!
//! A graph consists of metadata and topology relation.
//! Industrial-IO topology relation contains `company`, `property` and `relationship`.

pub mod entities;
pub mod handle_relationships;
pub mod maintain_companies;
pub mod maintain_properties;
pub mod objects;
pub mod search_relationships;
pub mod show_catalog;

pub use entities::*;
pub use objects::*;
