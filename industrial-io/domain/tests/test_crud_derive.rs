use crud::*;
use domain::entities::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, CRUD)]
struct TestCrud {
    #[crud(id, index = "unique,desc")]
    idx: Option<ID>,
    #[crud(index = "unique,asc,text")]
    name: String,
}

#[test]
fn test_custom_derive() {
    let test_crud = TestCrud {
        idx: None,
        name: "test".to_string(),
    };

    println!("{:?}", test_crud);
}
