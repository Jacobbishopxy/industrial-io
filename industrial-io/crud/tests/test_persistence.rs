use bson::oid::ObjectId;
use crud::*;
use serde::{Deserialize, Serialize};

const URI: &str = "mongodb://root:secret@localhost:27017";
const DB: &str = "test";
const CL: &str = "dev";

#[derive(Debug, Serialize, Deserialize, Clone, CRUD, PartialEq)]
struct TestSingleIndexCrud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[crud(id)]
    idx: Option<ObjectId>,
    #[crud(single_index = "unique,text")]
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

    let indexes = client.list_indexes().await;

    println!("{:?}", indexes);
}

#[tokio::test]
async fn test_curd_operations() {
    let client = MongoClient::new(URI, DB, CL).await.unwrap();

    let value = TestSingleIndexCrud {
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

    let mut update_value = read;
    update_value.name = "update".to_string();
    update_value.version += 1;

    let update = client.update(update_value).await;
    assert!(update.is_ok());

    let update = update.unwrap();
    println!("update: {:?}", update);
    assert_eq!(update.name, "update");
    assert_eq!(update.version, 2);

    let delete = client.delete(update.idx.unwrap()).await;
    assert!(delete.is_ok());

    let delete = delete.unwrap().unwrap();
    println!("delete: {:?}", delete);
    assert_eq!(update, delete);
}

#[derive(Debug, Serialize, Deserialize, Clone, CRUD, PartialEq)]
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

#[tokio::test]
async fn test_indexes_operations() {
    let client = MongoClient::new(URI, DB, CL).await.unwrap();

    let indexes = client.list_indexes().await;
    assert!(indexes.is_ok());

    let indexes_names = client.list_indexes_name().await;
    assert!(indexes_names.is_ok());
    let indexes_names = indexes_names.unwrap();
    println!("indexes names: {:?}", indexes_names);

    let compound_index_create = client
        .create_indexes_by_type::<TestCompoundIndexCrud>()
        .await;
    assert!(compound_index_create.is_ok());
    let compound_index_create = compound_index_create.unwrap();
    println!("create indexes: {:?}", compound_index_create);

    // we know `TestCompoundIndexCrud` has a compound index, and currently single_index
    // and compound_index are not coexisted, so we are sure the first index in the result
    // is the compound index
    let index_name = compound_index_create;
    assert_eq!(index_name.len(), 1);
    let index_name = index_name[0].clone();
    println!("index name: {:?}", index_name);

    let indexes_names = client.list_indexes_name().await;
    assert!(indexes_names.is_ok());
    let indexes_names = indexes_names.unwrap();
    println!("indexes names: {:?}", indexes_names);
}
