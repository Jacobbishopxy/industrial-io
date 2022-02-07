use bson::oid::ObjectId;
use crud::*;
use serde::{Deserialize, Serialize};

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
    let test_crud = TestSingleIndexCrud {
        idx: None,
        name: "test".to_string(),
        data: None,
    };

    let indexes = test_crud.show_indexes();

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
    let test_crud = TestCompoundIndexCrud {
        id: None,
        name: "test".to_string(),
        age: 12,
        content: None,
        version: 1,
    };

    let indexes = test_crud.show_indexes();

    println!("{:?}", indexes);
}
