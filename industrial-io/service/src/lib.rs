//! Service
//!
//! Implementation of Domain's `Repository`

pub mod provider;

use async_trait::async_trait;

// use crud::*;
use domain::Repository;
use provider::Provider;

#[async_trait]
impl Repository for Provider {}
