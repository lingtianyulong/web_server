use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize, Clone)]
struct UserService {
    ServiceID: String,
    ServiceName: String,
    Address: String,
    ServicePort: u16,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let consul_url = "http://192.168.99.86:8500/v1/catalog/service/user_service?passing";

    println!("before get request");
    let response = reqwest::get(consul_url).await?;
    let services: Vec<serde_json::Value> = response.json().await?;
    if services.is_empty() {
        println!("No services found");
    } else {
        println!("service: {:?}", services);
    }

    let address = services[0]["Address"].as_str().unwrap();
    let port = services[0]["ServicePort"].as_u64().unwrap();

    println!("address: {}", address);
    println!("port: {}", port);

    let hello_url = format!("http://{}:{}/hello", address, port);

    let response = reqwest::get(hello_url).await?;
    let body = response.text().await?;
    println!("body: {}", body);

    Ok(())
}
