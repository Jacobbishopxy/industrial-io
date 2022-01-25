//! Category
//!
//! Category is a collection of graphs' metadata.

use crud::*;
use serde::{Deserialize, Serialize};

use super::ID;

/// Category
///
/// name: collection of a graph
#[derive(Serialize, Deserialize, Debug, Clone, Default, GrantCRUD)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn new<T: Into<String>>(name: T, description: Option<T>) -> Self {
        Self {
            id: None,
            name: name.into(),
            description: description.map(Into::into),
        }
    }
}

#[cfg(test)]
mod test_category {
    use super::*;

    const URI: &str = "mongodb://root:secret@localhost:27017";

    #[tokio::test]
    async fn test_category() {
        let category = Category::new("test", None);
        assert_eq!(category.name, "test");
        assert_eq!(category.description, None);

        let client = MongoClient::new(URI, "test").await.unwrap();

        let res = client.create("test", "dev", category).await;
        assert!(res.is_ok());
    }
}
