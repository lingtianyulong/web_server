use actix_web::{App, HttpServer, Responder, Scope, post, web};
mod entity;
mod logger;
mod routes;
mod utils;
use routes::user_route;

pub fn user_scope() -> Scope {
    web::scope("/user").service(user_route::register_user)
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
    HttpServer::new(|| App::new().service(user_scope()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    logger::info("Server started");
    Ok(())
}
