use bson::oid::ObjectId;
use crud::*;
use serde::{Deserialize, Serialize};

const URI: &str = "mongodb://root:secret@localhost:27017";
const DB: &str = "test";
const CL: &str = "dev";

#[derive(Debug, Serialize, Deserialize, Clone, CRUD, PartialEq)]
struct TestCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[crud(id, index = "unique,desc")]
    idx: Option<ObjectId>,
    #[crud(index = "unique,text")]
    name: String,
    content: Option<String>,
    version: i32,
}

#[tokio::test]
async fn test_mongo_client() {
    let client = MongoClient::new(URI, DB, CL).await.unwrap();

    let dbs = client.show_dbs().await;
    assert!(dbs.is_ok());
    println!("{:?}", dbs.unwrap());

    let collections = client.show_collections().await;
    assert!(collections.is_ok());
    println!("{:?}", collections.unwrap());

    let indexes = client.list_indexes::<TestCrud>().await;

    println!("{:?}", indexes);
}

#[tokio::test]
async fn test_curd_operations() {
    let client = MongoClient::new(URI, DB, CL).await.unwrap();

    let value = TestCrud {
        idx: None,
        name: "test".to_string(),
        content: None,
        version: 1,
    };

    let create = client.create(value).await;
    assert!(create.is_ok());

    let create = create.unwrap();
    println!("create: {:?}", create);
    assert!(create.idx.is_some());

    let read = client.read(create.idx.unwrap()).await;
    assert!(read.is_ok());

    let read = read.unwrap().unwrap();
    println!("read: {:?}", read);
    assert_eq!(create, read);
}

#[tokio::test]
async fn test_indexes_operations() {
    let _client = MongoClient::new(URI, DB, CL).await.unwrap();

    todo!()
}
