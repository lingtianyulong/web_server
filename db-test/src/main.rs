use utils::database;
use utils::database::Model;
use serde::{Deserialize, Serialize};
use model_derive::Model as DeriveModel;

#[derive(DeriveModel, Serialize, Deserialize)]
#[table_name = "user"]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let db_context = match database::DbContext::new("mysql://root:123456@localhost:3306/rust_test").await {
        Ok(db_context) => db_context,
        Err(e) => {
            println!("Failed to connect to database: {:?}", e);
            return;
        }
    };

    let user = User {
        id: 1,
        name: "John".to_string(),
        age: 20,
    };

    db_context.insert(&user).await.unwrap();
}
