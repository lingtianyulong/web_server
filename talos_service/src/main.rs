use actix_web::{App, HttpServer, get, Responder, HttpResponse};
use reqwest;
use serde_json::json;
use logger::*;
// mod database;
mod entity;
// mod logger;
mod routes;
mod utils;
use routes::user_route;


async fn register_service() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let service = json!({
        "ID": uuid::Uuid::new_v4().to_string(),
        "Name": "user_service",
        "Tags": ["用户服务"],
        "Port": 8080,
        "Check": {
            "HTTP": "http://127.0.0.1:8080/health",
            "Interval": "10s",
            "Timeout": "5s"
        },
        "Meta": {
            "version": "1.0.0",
            "hello_path": "/hello",
            "health_path": "/health"
        }
    });

    // 调用注册服务 API
    let res = client
        .put("http://127.0.0.1:8500/v1/agent/service/register")
        .json(&service)
        .send()
        .await?;

    if res.status().is_success() {
        println!("服务注册成功: {:?}", res);
    } else {
        let err = format!("服务注册失败: {:?}", res);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));
    }

    Ok(())
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust service!")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !logger::init("confgs/logger_config.json") {
        eprintln!("Failed to initialize logger");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize logger",
        ));
    }

    logger::info("Starting server");

    logger::error("error info");


    tokio::spawn(async move {
        if let Err(e) = register_service().await {
            logger::error(&format!("服务注册失败: {:#?}", e));
        } 
    });


    HttpServer::new(|| App::new().service(hello).service(health))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    logger::info("Server started");
    Ok(())
}
