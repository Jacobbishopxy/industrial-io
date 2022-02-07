use bson::oid::ObjectId;
use crud::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, CRUD, PartialEq)]
struct TestNoneIndexCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    content: Option<String>,
}

#[test]
fn test_custom_derive_no_index() {
    let indexes = TestNoneIndexCrud::show_indexes();

    println!("{:?}", indexes);
}

#[derive(Debug, Serialize, Deserialize, Clone, CRUD)]
struct TestSingleIndexCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[crud(id)]
    idx: Option<ObjectId>,
    #[crud(single_index = "unique,text")]
    name: String,
    data: Option<String>,
}

#[test]
fn test_custom_derive() {
    let indexes = TestSingleIndexCrud::show_indexes();

    println!("{:?}", indexes);
}

#[derive(Debug, Serialize, Deserialize, Clone, CRUD)]
struct TestCompoundIndexCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[crud(compound_index = "unique")]
    name: String,
    #[crud(compound_index)]
    age: u32,
    content: Option<String>,
    version: i32,
}

#[test]
fn test_custom_derive2() {
    let indexes = TestCompoundIndexCrud::show_indexes();

    println!("{:?}", indexes);
}
