use crud::*;
use domain::entities::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, CRUD)]
struct TestCrud {
    id: Option<ID>,
    #[oid]
    #[crud(index = "asc,unique,text")]
    name: String,
}

#[test]
fn test_custom_derive() {
    let test_crud = TestCrud {
        id: None,
        name: "test".to_string(),
    };

    println!("{:?}", test_crud);
}
