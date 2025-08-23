use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value;
use sqlx::{Error, FromRow, MySql, MySqlPool};

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

        Ok(Self { pool_: pool })
    }

    // 插入数据
    pub async fn insert<T>(&self, obj: &T) -> Result<u64, sqlx::Error>
    where
        T: Model + Serialize,
    {
        let json_map: Value = match serde_json::to_value(obj) {
            Ok(val) => val,
            Err(e) => {
                return Err(sqlx::Error::Protocol(e.to_string()));
            }
        };
        println!("json_map: {:?}", json_map);

        let obj_map = match json_map.as_object() {
            Some(val) => val,
            None => {
                let desc =
                    "the input obj is not valid json object which is required by insert function";
                return Err(sqlx::Error::Protocol(desc.to_string()));
            }
        };

        println!("obj_map: {:?}", obj_map);

        let fields: Vec<String> = obj_map.keys().cloned().collect();
        let values: Vec<&Value> = obj_map.values().collect();
        println!("fields: {:?}", fields);
        println!("values: {:?}", values);

        let placeholders: Vec<String> = fields.iter().map(|_| "?".to_string()).collect();
        println!("placeholders: {:?}", placeholders);

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::table_name(),
            fields.join(","),
            placeholders.join(",")
        );
        println!("sql: {:?}", sql);

        let table_name = T::table_name();
        println!("table_name: {:?}", table_name);

        let mut query = sqlx::query(&sql);
        for v in values {
            query = Self::bind_value(query, v);
        }

        let result = match query.execute(&self.pool_).await {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(result.rows_affected())
    }

    /// 更新对象（根据主键字段）
    pub async fn update<T>(&self, obj: &T, key_field: &str) -> Result<u64, Error>
    where
        T: Model + Serialize,
    {
        let map = match serde_json::to_value(obj) {
            Ok(val) => val,
            Err(e) => {
                return Err(sqlx::Error::Protocol(e.to_string()));
            }
        };

        let obj_map = match map.as_object() {
            Some(val) => val,
            None => {
                let desc =
                    "the input obj is not valid json object which is required by update function";
                return Err(sqlx::Error::Protocol(desc.to_string()));
            }
        };

        let mut set_fields: Vec<String> = Vec::new();
        let mut values: Vec<&Value> = Vec::new();
        let mut key_value: Option<&Value> = None;

        for (k, v) in obj_map {
            if k == key_field {
                key_value = Some(v);
            } else {
                set_fields.push(format!("{} = ?", k));
                values.push(v);
            }
        }

        let sql = format!(
            "UPDATE {} SET {} WHERE {} = ?",
            T::table_name(),
            set_fields.join(", "),
            key_field
        );

        let mut query = sqlx::query(&sql);
        for v in values {
            query = Self::bind_value(query, v);
        }
        if let Some(kv) = key_value {
            query = Self::bind_value(query, kv);
        }

        let result = match query.execute(&self.pool_).await {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(result.rows_affected())

        // todo!("暂时只根据主键更新, 后续再根据其他字段更新");
    }

    /// 删除
    pub async fn delete<T, K>(&self, key_field: &str, key_value: K) -> Result<u64, Error>
    where
        T: Model,
        K: for<'q> sqlx::Encode<'q, MySql> + sqlx::Type<MySql> + Send + Sync,
    {
        let sql = format!("DELETE FROM {} WHERE {} = ?", T::table_name(), key_field);
        let result = match sqlx::query(&sql).bind(key_value).execute(&self.pool_).await {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(result.rows_affected())
    }

    /// 查询单个对象（根据主键字段）
    pub async fn find<T, K>(&self, key_field: &str, key_value: K) -> Result<T, Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
        K: for<'q> sqlx::Encode<'q, MySql> + sqlx::Type<MySql> + Send + Sync,
    {
        let sql = format!("SELECT * FROM {} WHERE {} = ?", T::table_name(), key_field);
        let row = match sqlx::query_as::<_, T>(&sql)
            .bind(key_value)
            .fetch_one(&self.pool_)
            .await
        {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(row)
    }

    /// 查询所有对象
    pub async fn find_all<T>(&self) -> Result<Vec<T>, Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
    {
        let sql = format!("SELECT * FROM {}", T::table_name());

        let rows = match sqlx::query_as::<_, T>(&sql).fetch_all(&self.pool_).await {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(rows)
    }

    // 查询所有符合条件的对象
    pub async fn query<T>(&self, where_clause: &str, params: Vec<&Value>) -> Result<Vec<T>, Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
    {
        let sql = format!("SELECT * FROM {} WHERE {}", T::table_name(), where_clause);
        let mut query = sqlx::query_as::<_, T>(&sql);
        for p in params {
            query = Self::bind_value_query(query, p);
        }

        let rows = match query.fetch_all(&self.pool_).await {
            Ok(val) => val,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(rows)
    }

    /// 绑定 JSON Value 到 SQL 参数
    fn bind_value<'q>(
        mut query: sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments>,
        v: &'q Value,
    ) -> sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments> {
        if let Some(s) = v.as_str() {
            if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                query = query.bind(dt);
            } else {
                query = query.bind(s);
            }
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

    /// 绑定 JSON Value 到 SQL 参数 (query_as 版本)
    fn bind_value_query<'q, T>(
        mut query: sqlx::query::QueryAs<'q, MySql, T, sqlx::mysql::MySqlArguments>,
        v: &'q Value,
    ) -> sqlx::query::QueryAs<'q, MySql, T, sqlx::mysql::MySqlArguments> {
        if let Some(s) = v.as_str() {
            if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                query = query.bind(dt);
            } else {
                query = query.bind(s);
            }
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
