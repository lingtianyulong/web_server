use reqwest::Error;
// #[warn(unused_imports)]
// use serde::Deserialize;
// use serde_json::json;
// use tokio::runtime::Runtime;


#[tokio::main]
async fn main() -> Result<(), Error> {

    let service_name = "user_service";
    let consul_url = format!("http://192.168.99.86:8500/v1/catalog/service/{}?passing", service_name);

    println!("before get request");
    let response = reqwest::get(consul_url).await?;
    let services: Vec<serde_json::Value> = response.json().await?;
    if services.is_empty() {
        println!("No services found");
    } else {
        println!("service: {:?}", services);
    }

    let address = services[0]["Address"].as_str().unwrap_or("127.0.0.1");
    let port = services[0]["ServicePort"].as_u64().unwrap_or(8080);
    let hello_path = services[0]["ServiceMeta"]["hello_path"].as_str().unwrap_or("/hello");

    println!("address: {}", address);
    println!("port: {}", port);
    println!("hello_path: {}", hello_path);

    let hello_url = format!("http://{}:{}{}", address, port, hello_path);

    let response = reqwest::get(hello_url).await?;
    let body = response.text().await?;
    println!("body: {}", body);

    Ok(())
}
