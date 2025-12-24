use axum::{Router, routing::post};
use dotenvy;
use logger::*;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;

mod db;
mod entity;
mod route;

use crate::route::user_route::AppState;
use route::user_route::*;
use db::UserDb;

fn exe_dir() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    if let Some(exe_dir) = exe_path.parent() {
        Ok(exe_dir.to_path_buf())
    } else {
        Err("Failed to get the parrent directory of executable path".into())
    }
}

fn init_logger(log_level: String, log_save_folder: String) -> Result<(), Box<dyn Error>> {
    let log_save_dir = match exe_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting executable directory: {}", e);
            return Err(e);
        }
    };
    let save_path = log_save_dir.join(log_save_folder);
    if !save_path.as_path().exists() {
        if let Err(e) = std::fs::create_dir_all(&save_path) {
            eprintln!("Failed to create log directory: {}", e);
            return Err(e.into());
        }
    }
    let save_path_str = match save_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Failed to convert log save path to string");
            return Err("Failed to convert log save path to string".into());
        }
    };
    let config = LoggerConfig::new(log_level, save_path_str.to_string());
    let _ = match init_log(config) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to initialize logger: {}", e);
            return Err(e.into());
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());
    let log_save_folder = env::var("LOG_SAVE_PATH").unwrap_or("logs".to_string());

    match init_logger(log_level, log_save_folder) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to initialize logger: {}", e);
        }
    };

    match UserDb::init().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
        }
    };

    let app_state = AppState {
        jwt_secret: SECRET_KEY.to_vec(),
    };
    let api_v1 = Router::new()
        .route("/user", post(get_user))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), jwt_middleware));

    let api_user = Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler));

    let app = Router::new()
        .nest("/api/v1", api_v1)
        .nest("/api/v1/user", api_user)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server is running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
