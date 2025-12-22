use crate::entity::{user::*, user_dto::UserDto};
use logger;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    EntityTrait, QueryFilter, Statement,
};
use std::error::Error;
use std::sync::OnceLock;
use utils::time_util;

/// 全局单例 UserDb 实例（线程安全）
/// 使用标准库的 OnceLock 保证初始化只执行一次且线程安全
static USER_DB: OnceLock<UserDb> = OnceLock::new();

#[allow(dead_code)]
pub struct UserDb {
    conn: DatabaseConnection,
}

#[allow(dead_code)]
impl UserDb {
    /// 初始化全局 UserDb 单例，只能调用一次
    /// 应该在应用启动时调用
    pub async fn init() -> Result<(), Box<dyn Error>> {
        logger::info("Initialize Database Connection Pool.");
        let database_url = std::env::var("DATABASE_URL").unwrap_or("mysql://root:123456@localhost/talos".to_string());
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(10).min_connections(5);
        let db = Database::connect(opt).await?;

        USER_DB
            .set(UserDb { conn: db })
            .map_err(|_| "UserDb already initialized")?;
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

    pub fn db(&self) -> &DatabaseConnection {
        &self.conn
    }

    /// 检查数据库连接是否正常（静态方法）
    /// @return true 连接正常，false 连接异常
    pub async fn connected() -> Result<bool, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();
        let stmt = Statement::from_string(db.get_database_backend(), "SELECT 1");
        let res = db.query_one(stmt).await?;
        match res {
            Some(_) => Ok(true),
            None => Err("Database connection check failed".into()),
        }
    }

    /// 获取所有用户（异步方法）
    /// @return 用户列表
    pub async fn get_all_users() -> Result<Vec<Model>, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();
        let users = Entity::find().all(db).await?;
        logger::info("All users fetched successfully");
        Ok(users)
    }

    pub async fn get_user_by_username(username: &str) -> Result<Option<Model>, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();
        let user = Entity::find()
            .filter(Column::UserName.eq(username))
            .one(db)
            .await?;
        logger::info("User fetched successfully");
        Ok(user)
    }

    /// 插入用户（异步方法）
    /// @param user 用户信息
    /// @return 插入的行数
    pub async fn insert(input_user: &UserDto<'_>) -> Result<u64, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();

        let active = ActiveModel {
            user_name: Set(input_user.user_name.to_string()),
            password: Set(input_user.password.to_string()),
            create_time: Set(input_user.create_time),
            update_time: Set(input_user.update_time),
            delete_time: Set(input_user.delete_time),
            unregistered: Set(input_user.unregistered),
            ..Default::default()
        };

        let res = active.insert(db).await?;
        logger::info("User inserted successfully");

        Ok(res.id as u64)
    }

    /// 更新用户（异步方法）
    /// @param input_user 用户信息
    /// @return 更新的行数
    pub async fn update(input_user: UserDto<'_>) -> Result<usize, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();

        let active = ActiveModel {
            user_name: Set(input_user.user_name.to_string()),
            password: Set(input_user.password.to_string()),
            update_time: Set(input_user.update_time),
            ..Default::default()
        };

        let res = active.update(db).await?;
        logger::info("User updated successfully");

        Ok(res.id as usize)
    }

    /// 删除用户（逻辑删除）
    /// @param user_id 用户ID
    /// @return 受影响的行数
    pub async fn delete(user_id: i64) -> Result<usize, Box<dyn Error>> {
        let instance = Self::try_instance()?;
        let db = instance.db();
        // 查找用户
        let user = Entity::find_by_id(user_id).one(db).await?;
        if user.is_none() {
            return Err("User not found".into());
        }
        // 转换为 ActiveModel 并更新
        let mut active: ActiveModel = user.unwrap().into();
        active.unregistered = Set(1);
        active.delete_time = Set(Some(time_util::now()?));
        let res = active.update(db).await?;
        logger::info("User deleted successfully");

        Ok(res.id as usize)
    }
}
