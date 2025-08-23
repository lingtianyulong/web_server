use sqlx::{ MySql, MySqlPool, Error };
use serde::Serialize;
use serde_json::Value;
use indexmap::IndexMap;

pub trait Model {
    fn table_name() -> &'static str;
}

pub struct DbContext {
    pool_: MySqlPool,
}

impl DbContext {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let pool = match MySqlPool::connect(url).await {
            Ok(pool) => pool,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Self { pool_: pool})
    }

    // 插入数据
    pub async fn insert<T>(&self, obj: &T) -> Result<u64, sqlx::Error> where T: Model + Serialize, {

        let json_map: Value = match serde_json::to_value(obj) {
            Ok(val) => val,
            Err(e) => {
                return Err(sqlx::Error::Protocol(e.to_string()));
            }
        };
        println!("json_map: {:?}", json_map);

        let obj_map: IndexMap<String, Value> = match serde_json::from_value(json_map) {
            Ok(val) => val,
            Err(e) => {
                return Err(sqlx::Error::Protocol(e.to_string()));
            }
        };
        println!("obj_map: {:?}", obj_map);
        
        let fields: Vec<String> = obj_map.keys().cloned().collect();
        let values: Vec<&Value> = obj_map.values().collect();
        println!("fields: {:?}", fields);
        println!("values: {:?}", values);

        let placeholders: Vec<String> = fields.iter().map(|_| "?".to_string()).collect();
        println!("placeholders: {:?}", placeholders);
   
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", T::table_name(), fields.join(","), placeholders.join(","));
        println!("sql: {:?}", sql);

        let table_name = T::table_name();
        println!("table_name: {:?}", table_name);

        let mut query = sqlx::query(&sql);
        for v in values {
            query = Self::bind_value(query, v);
        }

        let result = query.execute(&self.pool_).await?;
        Ok(result.rows_affected())
    }

    /// 绑定 JSON Value 到 SQL 参数
fn bind_value<'q>(
    mut query: sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments>,
    v: &'q Value,
) -> sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments> {
    if let Some(s) = v.as_str() {
        query = query.bind(s);
    } else if let Some(i) = v.as_i64() {
        query = query.bind(i);
    } else if let Some(b) = v.as_bool() {
        query = query.bind(b);
    } else if let Some(f) = v.as_f64() {
        query = query.bind(f);
    } else {
        query = query.bind(None::<String>);
    }
    query
}

}
