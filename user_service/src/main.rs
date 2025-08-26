use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use utils::consul;
mod entity;
mod managers;
mod routes;
use actix_web::Scope;
use routes::user_route;
use serde_json::json;
use std::collections::HashMap;

async fn register_service() -> Result<(), Box<dyn std::error::Error>> {
    let meta: Option<HashMap<String, String>> = Some(
        [
            ("version".to_string(), "1.0.0".to_string()),
            ("hello_path".to_string(), "/hello".to_string()),
            ("health_path".to_string(), "/health".to_string()),
            ("register_path".to_string(), "/register".to_string()),
            ("login_path".to_string(), "/login".to_string()),
            ("user_exist_path".to_string(), "/user_exist".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    let service_name = "user_service";
    let service = consul::Service::new(
        uuid::Uuid::new_v4().to_string(),
        service_name.to_string(),
        vec!["用户服务".to_string()],
        8080,
        consul::Check::new(
            "http://127.0.0.1:8080/health".to_string(),
            "10s".to_string(),
            "5s".to_string(),
        ),
        meta,
    );

    // 先注销, 防止服务重复注册
    let services = consul::Service::get_service("http://127.0.0.1:8500", service_name).await?;
    if !services.is_empty() {
        println!("服务列表: {:?}", services);
        let svc = services
            .iter()
            .find(|service| service["ServiceName"].as_str().unwrap_or_default() == service_name);
        if let Some(svc) = svc {
            if let Some(service_id) = svc.get("ServiceID").and_then(|v| v.as_str()) {
                println!("注销服务: {}", service_id);
                // 调用注销接口
                consul::Service::deregister("http://127.0.0.1:8500", service_id).await?;
            } else {
                println!("找不到 ServiceID, 无法注销");
            }
        }
    }

    match service.register("http://127.0.0.1:8500").await {
        Ok(()) => {
            logger::info("服务注册成功");
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

#[get("/hello")]
async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello from Rust service!")
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "Hello from Rust service!",
        "data": true
    }))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

fn user_scope() -> Scope {
    web::scope("/user")
        .service(user_route::register_user)
        .service(user_route::login_user)
        .service(user_route::user_exist)
        .service(user_route::reset_password)
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

    println!("开始注册服务!");

    tokio::spawn(async move {
        if let Err(e) = register_service().await {
            logger::error(&format!("服务注册失败: {:#?}", e));
            println!("服务注册失败: {:#?}", e);
            return;
        }
    });
    println!("服务注册完成!");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health)
            .service(user_scope())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    logger::info("Server started");
    Ok(())
}
