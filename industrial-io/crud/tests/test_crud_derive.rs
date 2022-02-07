use bson::oid::ObjectId;
use crud::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, CRUD)]
struct TestCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[crud(id)]
    idx: Option<ObjectId>,
    #[crud(single_index = "unique,text")]
    name: String,
    data: Option<String>,
}

#[test]
fn test_custom_derive() {
    let test_crud = TestCrud {
        idx: None,
        name: "test".to_string(),
        data: None,
    };

    let indexes = test_crud.show_indexes();

    println!("{:?}", indexes);
}
