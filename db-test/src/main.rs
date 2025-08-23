use model_derive::Model as DeriveModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utils::database;
use utils::database::Model;

#[derive(Debug, DeriveModel, Serialize, Deserialize, FromRow)]
#[table_name = "user"]
struct User {
    #[sqlx(rename = "id")] // 此处用于查询时, 让查询结果的数据库中的列名与属性名一致
    #[serde(rename = "id")] // 此处用于插入时, 让插入的数据库中的列名与属性名一致
    user_id: i32,
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let db_context =
        match database::DbContext::new("mysql://root:123456@localhost:3306/rust_test").await {
            Ok(db_context) => db_context,
            Err(e) => {
                println!("Failed to connect to database: {:?}", e);
                return;
            }
        };

    let _user = User {
        user_id: 5,
        name: "LiLei".to_string(),
        age: 20,
    };

    // 查询单个对象
    // let user = db_context.find::<User, i32>("id", 1).await.unwrap();
    // println!("user: {:?}", user);

    // 查询所有对象
    // let users = db_context.find_all::<User>().await.unwrap();
    // println!("users: {:?}", users);

    // 条件查询
    // let users = db_context.query::<User>("id = ?", vec![&Value::Int(1)]).await.unwrap();
    // println!("users: {:?}", users);

    // 条件查询
    let mut users = db_context
        .query::<User>("name = ?", vec![&Value::String("LiLei".to_string())])
        .await
        .unwrap();
    // 按照 age 字段升序排序
    // users.sort_by_key(|u| u.age);
    // 按照 age 字段降序排序
    users.sort_by_key(|u| std::cmp::Reverse(u.age));

    println!("users: {:?}", users);

    // 查询特定时间之后的记录
    // let users = db_context
    //     .query::<User>("created_at > ?", vec![&json!("2024-01-01 10:30:00")])
    //     .await
    //     .unwrap();

    // let users = db_context
    //     .query::<User>(
    //         "created_at > ?",
    //         vec![&Value::String("2024-01-01 10:30:00".to_string())],
    //     )
    //     .await
    //     .unwrap();

    // 插入对象
    // db_context.insert(&_user).await.unwrap();

    // 更新对象
    // db_context.update(&user, "id").await.unwrap();

    // 删除对象
    // db_context.delete::<User, &str>("name", "Lily").await.unwrap();
}
