use crate::user_entity::entity::*;
use crate::user_entity::schema::users;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use logger;
use std::error::Error;
use std::sync::OnceLock;
use utils::time_util;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// 全局单例 UserDb 实例（线程安全）
/// 使用标准库的 OnceLock 保证初始化只执行一次且线程安全
static USER_DB: OnceLock<UserDb> = OnceLock::new();

#[allow(dead_code)]
pub struct UserDb {
    pool: DbPool,
}

#[allow(dead_code)]
impl UserDb {
    /// 初始化全局 UserDb 单例，只能调用一次
    /// 应该在应用启动时调用
    pub fn init() -> Result<(), Box<dyn Error>> {
        logger::info("Initialize Database Connection Pool.");
        let database_url = std::env::var("DATABASE_URL")?;
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder().max_size(10).build(manager)?;

        let db = Self { pool };
        USER_DB.set(db).map_err(|_| "UserDb already initialized")?;

        logger::info("Database Connection Pool Initialized.");
        Ok(())
    }

    /// 获取全局 UserDb 单例引用
    /// 如果未初始化会 panic
    pub fn instance() -> &'static UserDb {
        USER_DB
            .get()
            .expect("UserDb not initialized. Call UserDb::init() first.")
    }

    /// 尝试获取全局 UserDb 单例引用
    /// 如果未初始化返回 None
    pub fn try_instance() -> Result<&'static UserDb, Box<dyn Error>> {
        let instance = match USER_DB.get() {
            Some(db) => db,
            None => return Err("UserDb not initialized. Call UserDb::init() first.".into()),
        };
        Ok(instance)
    }

    /// 检查数据库连接是否正常（静态方法）
    /// @return true 连接正常，false 连接异常
    pub fn connected() -> Result<bool, Box<dyn Error>> {
        let db = Self::try_instance()?;
        let mut conn = db.pool.get()?;
        diesel::sql_query("SELECT 1").execute(&mut conn)?;
        Ok(true)
    }

    /// 获取数据库连接池引用（静态方法）
    /// @return 数据库连接池引用
    pub fn pool() -> Result<&'static DbPool, Box<dyn Error>> {
        let db = Self::try_instance()?;
        Ok(&db.pool)
    }

    /// 插入用户, 同步方式（静态方法）
    /// @param user 用户信息, 原 NewUser 中有生命周期参数,
    ///             在使用 &NewUser 作为参数类型时，
    ///             Rust 编译器无法自动推断这个生命周期参数，
    ///             必须显式标注,'_ 是匿名生命周期占位符，
    ///             表示让编译器自动推断生命周期，但明确告知这里需要生命周期参数
    /// @return 插入的行数
    pub fn insert(user: &NewUser<'_>) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        let mut conn = db.pool.get()?;
        let res = diesel::insert_into(users::table)
            .values(user)
            .execute(&mut conn)?;
        Ok(res)
    }

    /// 插入用户, 异步方式（静态方法）
    /// @param user 用户信息
    /// @return 插入的行数
    pub async fn insert_async(user: &NewUser<'_>) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        // pool clone 只是一个 Arc, 不会产生效率问题
        let pool = db.pool.clone();
        let user_name = user.user_name.to_string();
        let password = user.password.to_string();
        let create_time = user.create_time;
        let unregistered = user.unregistered;

        // 使用 tokio::task::spawn_blocking 将阻塞操作放入新线程中执行
        let result =
            tokio::task::spawn_blocking(move || -> Result<usize, Box<dyn Error + Send + Sync>> {
                let mut conn = pool
                    .get()
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                let new_user = NewUser {
                    user_name: &user_name,
                    password: &password,
                    create_time,
                    unregistered,
                };
                let res = diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(&mut conn)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                Ok(res)
            })
            .await
            .map_err(|e| -> Box<dyn Error> { format!("insert_async error: {}", e).into() })?;

        result.map_err(|e| -> Box<dyn Error> { e.to_string().into() })
    }

    /// 更新用户, 同步方式（静态方法）
    /// @param user 用户信息
    /// @return 更新的行数
    pub fn update(user: &UpdateUser<'_>) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        let mut conn = db.pool.get()?;
        let res = diesel::update(users::table)
            .set(user)
            .execute(&mut conn)?;
        Ok(res)
    }

    /// 更新用户, 异步方式（静态方法）
    /// @param user 用户信息
    /// @return 更新的行数
    pub async fn update_async(user: &UpdateUser<'_>) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        // pool clone 只是一个 Arc, 不会产生效率问题
        let pool = db.pool.clone();
        let user_name = user.user_name.to_string();
        let password = user.password.to_string();
        let update_time = user.update_time;
        let unregistered = user.unregistered;

        let result = tokio::task::spawn_blocking(move || -> Result<usize, Box<dyn Error + Send + Sync>> {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            let update_user = UpdateUser {
                user_name: &user_name,
                password: &password,
                update_time,
                unregistered,
            };
            let res = diesel::update(users::table)
                .set(&update_user)
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            Ok(res)
        })
        .await
        .map_err(|e| -> Box<dyn Error> { format!("update_async error: {}", e).into() })?;

        result.map_err(|e| -> Box<dyn Error> { e.to_string().into() })
    }

    /// 删除用户（逻辑删除，将 unregistered 设置为 1）
    /// @param user_id 用户ID
    /// @return 删除的行数
    pub fn delete(user_id: i64) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        let mut conn = db.pool.get()?;
        let delete_time = time_util::now()?;
        let res = diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set((users::unregistered.eq(1), users::delete_time.eq(delete_time)))
            .execute(&mut conn)?;
        Ok(res)
    }

    /// 删除用户, 异步方式（静态方法）
    /// @param user_id 用户ID
    /// @return 删除的行数
    pub async fn delete_async(user_id: i64) -> Result<usize, Box<dyn Error>> {
        let db = Self::try_instance()?;
        let pool = db.pool.clone();
        let user_id = user_id;
        let result = tokio::task::spawn_blocking(move || -> Result<usize, Box<dyn Error + Send + Sync>> {
            let mut conn = pool.get()?;
            let delete_time = time_util::now()?;
            let res = diesel::update(users::table)
                .filter(users::id.eq(user_id))
                .set((users::unregistered.eq(1), users::delete_time.eq(delete_time)))
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            Ok(res)
        })
        .await
        .map_err(|e| -> Box<dyn Error> { format!("delete_async error: {}", e).into() })?;

        result.map_err(|e| -> Box<dyn Error> { e.to_string().into() })
    }

}
