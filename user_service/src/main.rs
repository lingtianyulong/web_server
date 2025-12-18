use logger::*;
use std::env;
use std::error::Error;
use std::path::PathBuf;

mod db;
mod entity;

// use db::UserDb;

fn exe_dir() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    if let Some(exe_dir) = exe_path.parent() {
        Ok(exe_dir.to_path_buf())
    } else {
        Err("Failed to get the parrent directory of executable path".into())
    }
}


fn main() {
    let log_level = std::env::var("LOG_LEVEL").unwrap();
    let log_save_folder = env::var("LOG_SAVE_PATH").unwrap();

    let log_save_dir = match exe_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting executable directory: {}", e);
            return;
        }
    };
    let save_path = log_save_dir.join(log_save_folder);
    if !save_path.as_path().exists() {
        if let Err(e) = std::fs::create_dir_all(&save_path) {
            eprintln!("Failed to create log directory: {}", e);
            return;
        }
    }

    let save_path_str = match save_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Failed to convert log save path to string");
            return;
        }
    };

    println!("log_level: {}", log_level);
    println!("log_save_path: {}", save_path.display());

    let config = LoggerConfig::new(log_level, save_path_str.to_string());
    let _ = match init_log(config) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to initialize logger: {}", e);
            return;
        }
    };

    info("Hello, world!");
    warn("Hello, world!");
    error("Hello, world!");

    // 初始化数据库连接池（全局单例）
    // if let Err(e) = UserDb::init() {
    //     error(&format!("Failed to initialize database: {}", e));
    //     return;
    // }

    // // 测试数据库连接
    // match UserDb::connected() {
    //     Ok(true) => info("Database connection is OK."),
    //     Ok(false) => warn("Database connection check returned false."),
    //     Err(e) => {
    //         error(&format!("Database connection failed: {}", e));
    //         return;
    //     }
    // }

    // 之后在任何地方都可以直接调用 UserDb 的静态方法
    // 例如：UserDb::insert(&new_user)?
    // 例如：UserDb::insert_async(&new_user).await?

    std::thread::sleep(std::time::Duration::from_secs(1));
}
