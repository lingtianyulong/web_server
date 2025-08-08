use actix_web::{App, HttpServer, Responder, web};
mod entity;
mod logger;
mod utils;

async fn index() -> impl Responder {
    "Hello, world!"
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
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
        // .service(greet)
        // .service(create_user)
        // .service(user_exist)
        // .service(login)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    logger::info("Server started");
    Ok(())
}
