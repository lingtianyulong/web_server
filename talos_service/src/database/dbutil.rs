/*
 * 数据库工具类
 * 使用 OnceLock 确保在纯 Rust 环境中的线程安全性
 * 使用 sqlx 连接池
 * 使用 dotenv 加载环境变量
 * 使用 tokio 异步执行
 * 使用 logger 记录日志
 * 使用 env 获取环境变量
 */
use crate::logger;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

/// 全局单例实例
static INSTANCE: OnceLock<DButil> = OnceLock::new();

/// 线程安全的数据库工具单例类
/// 使用 OnceLock 确保在纯 Rust 环境中的线程安全性
pub struct DButil {
    pool: Pool<MySql>,
}

// 确保 DButil 是线程安全的
unsafe impl Send for DButil {}
unsafe impl Sync for DButil {}

impl DButil {
    /// 创建新的数据库实例（私有方法）
    fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let pool = runtime.block_on(Self::establish_connection())?;

        Ok(Self { pool })
    }

    /// 获取数据库单例实例
    /// 使用 OnceLock 实现线程安全的单例模式
    /// 第一次调用时会初始化数据库连接池
    pub fn instance() -> &'static DButil {
        INSTANCE.get_or_init(|| {
            Self::new().unwrap_or_else(|e| {
                panic!("Failed to initialize database: {}", e);
            })
        })
    }

    /// 建立数据库连接池
    async fn establish_connection() -> Result<Pool<MySql>, sqlx::Error> {
        dotenv().ok();
        let url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:rust%40123@192.168.99.86:3306/rust_db".to_string());

        let mut pool_options = MySqlPoolOptions::new();

        // 设置最大连接数
        if let Ok(max_conn) = env::var("DATABASE_MAX_CONNECTIONS") {
            if let Ok(max_conn_num) = max_conn.parse::<u32>() {
                pool_options = pool_options.max_connections(max_conn_num);
            } else {
                pool_options = pool_options.max_connections(100);
            }
        } else {
            pool_options = pool_options.max_connections(100);
        }

        // 设置最小连接数
        if let Ok(min_conn) = env::var("DATABASE_MIN_CONNECTIONS") {
            if let Ok(min_conn_num) = min_conn.parse::<u32>() {
                pool_options = pool_options.min_connections(min_conn_num);
            } else {
                pool_options = pool_options.min_connections(5);
            }
        } else {
            pool_options = pool_options.min_connections(5);
        }

        // 设置获取连接超时时间
        if let Ok(acquire_timeout) = env::var("DATABASE_ACQUIRE_TIMEOUT") {
            if let Ok(timeout_secs) = acquire_timeout.parse::<u64>() {
                pool_options = pool_options.acquire_timeout(Duration::from_secs(timeout_secs));
            } else {
                pool_options = pool_options.acquire_timeout(Duration::from_secs(8));
            }
        } else {
            pool_options = pool_options.acquire_timeout(Duration::from_secs(8));
        }

        // 设置空闲超时时间
        if let Ok(idle_timeout) = env::var("DATABASE_IDLE_TIMEOUT") {
            if let Ok(timeout_secs) = idle_timeout.parse::<u64>() {
                pool_options = pool_options.idle_timeout(Some(Duration::from_secs(timeout_secs)));
            } else {
                pool_options = pool_options.idle_timeout(Some(Duration::from_secs(8)));
            }
        } else {
            pool_options = pool_options.idle_timeout(Some(Duration::from_secs(8)));
        }

        // 设置连接最大生命周期
        if let Ok(max_lifetime) = env::var("DATABASE_MAX_LIFETIME") {
            if let Ok(lifetime_secs) = max_lifetime.parse::<u64>() {
                pool_options = pool_options.max_lifetime(Some(Duration::from_secs(lifetime_secs)));
            } else {
                pool_options = pool_options.max_lifetime(Some(Duration::from_secs(1800)));
            }
        } else {
            pool_options = pool_options.max_lifetime(Some(Duration::from_secs(1800)));
        }

        let pool = pool_options.connect(&url).await?;

        logger::info("数据库连接池创建成功");
        Ok(pool)
    }

    /// 获取数据库连接池的引用
    pub fn pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}
