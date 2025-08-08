use flexi_logger::{Cleanup, Criterion, Logger, Naming, WriteMode};
// use log::{error, info, warn};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Once;

static INIT_LOGGER: Once = Once::new();

#[derive(Debug, Serialize, Deserialize)]
struct LoggerConfig {
    log_level: String,   // 日志级别
    log_file: String,    // 日志文件路径
    log_file_size: u64,  // 日志文件大小
    log_file_count: u32, // 日志文件数量
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            log_file: "logs".to_string(),
            log_file_size: 10_000_000,
            log_file_count: 5,
        }
    }
}

impl LoggerConfig {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn my_log_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    write!(
        w,
        "{} [{}] - {} \n",
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        &record.args()
    )
}

// 初始化日志
// cfg_path: 配置文件路径
// 返回值: 是否初始化成功
pub fn init(cfg_path: &str) -> bool {
    // 读取配置文件
    let file_path = Path::new(cfg_path);
    let mut config = LoggerConfig::default();
    if !file_path.exists() {
        // 如果路径不存在，创建目录
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create config directory: {}", e);
                    return false;
                }
            }
        }

        let json = config.to_json();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();

        if let Err(e) = file.write_all(json.as_bytes()) {
            error!("Failed to write config file: {}", e);
            return false;
        }
    } else {
        // let file = File::open(file_path).unwrap();
        let content = match fs::read_to_string(file_path) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to read config file: {}", e);
                return false;
            }
        };
        config = match LoggerConfig::from_json(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to parse config file: {}", e);
                return false;
            }
        };
    }

    let result = std::panic::catch_unwind(|| {
        INIT_LOGGER.call_once(|| {
            Logger::try_with_str(config.log_level.as_str())
                .unwrap()
                .log_to_file(flexi_logger::FileSpec::default().directory(config.log_file.as_str()))
                .rotate(
                    Criterion::Size(config.log_file_size), // 10 MB 进行轮转
                    Naming::Timestamps,
                    Cleanup::KeepLogFiles(config.log_file_count as usize), // 最多保留 5 个旧日志文件
                )
                .write_mode(WriteMode::BufferAndFlush)
                .format(my_log_format) // 启用标准格式()
                .start()
                .unwrap();
        });
    });

    result.is_ok()
}

pub fn info(msg: &str) {
    info!("{}", msg);
}

// pub fn warn(msg: &str) {
//     warn!("{}", msg);
// }

pub fn error(msg: &str) {
    error!("{}", msg);
}
