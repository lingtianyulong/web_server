use utils::database;
use utils::database::Model;
use serde::{Deserialize, Serialize};
use model_derive::Model as DeriveModel;

#[derive(DeriveModel, Serialize, Deserialize)]
#[table_name = "user"]
struct User {
    // 此处使用 serde 注解, 让属性名与数据库字段名保持一致, 防止属性名与数据库字段名不一致导致插入失败
    #[serde(rename = "id")]
    user_id: i32,
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
        user_id: 3,
        name: "Lily".to_string(),
        age: 19,
    };

    db_context.insert(&user).await.unwrap();
}
