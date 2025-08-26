use dotenv::dotenv;
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, MySql};
use std::env;
use tokio::sync::OnceCell;
use utils::database::{DbContext, Model};

// 使用 tokio::sync::OnceCell 来实现完全异步的单例模式
static INSTANCE: OnceCell<DbManager> = OnceCell::const_new();

pub struct DbManager {
    db_context: DbContext,
}

#[allow(dead_code)]
impl DbManager {
    async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db_context = Self::connect().await?;
        Ok(Self { db_context })
    }

    pub async fn instance() -> Result<&'static DbManager, Box<dyn std::error::Error + Send + Sync>>
    {
        INSTANCE
            .get_or_try_init(|| async { Self::new().await })
            .await
    }

    async fn connect() -> Result<DbContext, Box<dyn std::error::Error + Send + Sync>> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_context = DbContext::new(&db_url).await?;
        Ok(db_context)
    }

    // 插入数据
    pub async fn insert<T>(&self, obj: &T) -> Result<u64, sqlx::Error>
    where
        T: Model + Serialize,
    {
        let result = match self.db_context.insert(obj).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 更新数据
    pub async fn update<T>(&self, obj: &T, key_field: &str) -> Result<u64, sqlx::Error>
    where
        T: Model + Serialize,
    {
        let result = match self.db_context.update(obj, key_field).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 删除数据
    pub async fn delete<T, K>(&self, key_field: &str, key_value: K) -> Result<u64, sqlx::Error>
    where
        T: Model,
        K: for<'q> sqlx::Encode<'q, MySql> + sqlx::Type<MySql> + Send + Sync,
    {
        let result = match self.db_context.delete::<T, _>(key_field, key_value).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 查询数据
    pub async fn find<T, K>(&self, key_field: &str, key_value: K) -> Result<T, sqlx::Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
        K: for<'q> sqlx::Encode<'q, MySql> + sqlx::Type<MySql> + Send + Sync,
    {
        let result = match self.db_context.find(key_field, key_value).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 查询所有数据
    pub async fn find_all<T>(&self) -> Result<Vec<T>, sqlx::Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
    {
        let result = match self.db_context.find_all().await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 查询数据
    pub async fn query<T>(
        &self,
        where_clause: &str,
        params: Vec<&Value>,
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: Model + for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send + Sync,
    {
        let result = match self.db_context.query(where_clause, params).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }

    // 判断数据是否存在
    pub async fn exists<T>(
        &self,
        where_clause: &str,
        params: Vec<&Value>,
    ) -> Result<bool, sqlx::Error>
    where
        T: Model,
    {
        let result = match self.db_context.exists::<T>(where_clause, params).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(result)
    }
}
