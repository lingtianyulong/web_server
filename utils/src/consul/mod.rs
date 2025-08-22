/*
 * consul 工具类, 用于对 consul 相关的操作进行封装
 * 包含 consul 的注册、注销、健康检查等功能,
 */

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Port")]
    port: u16,
    #[serde(rename = "Check")]
    check: Check,
    #[serde(rename = "Meta")]
    meta: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Check {
    #[serde(rename = "HTTP")]
    http: String,
    #[serde(rename = "Interval")]
    interval: String,
    #[serde(rename = "Timeout")]
    timeout: String,
}

impl Check {

    pub fn new(http: String, interval: String, timeout: String) -> Self {
        Self { http, interval, timeout }
    }
}

impl Service {
    pub fn new(
        id: String,
        name: String,
        tags: Vec<String>,
        port: u16,
        check: Check,
        meta: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            id,
            name,
            tags,
            port,
            check,
            meta,
        }
    }

    // 注册服务
    // 参数: url: &str, 注册服务的地址
    // 返回: Result<(), Box<dyn std::error::Error>>, 注册服务的结果
    pub async fn register(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let register_url = format!("{}/v1/agent/service/register", url);
        let json_value = json!(self);
        let response = client.put(register_url).json(&json_value).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let err = format!("注册服务失败, 失败原因 {:?}", response);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )))
        }
    }

    // 注销服务
    // 参数: url: &str, 注销服务的地址
    // 返回: Result<(), Box<dyn std::error::Error>>, 注销服务的结果
    pub async fn deregister(url: &str, service_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let deregister_url = format!("{}/v1/agent/service/deregister/{}", url, service_id);
        let response = client.delete(deregister_url).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let err = format!("注销服务失败, 失败原因 {:?}", response);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )))
        }
    }

    // 获取服务
    // 参数: url: &str, 获取服务的地址
    // 参数: service_name: &str, 获取服务的名称
    // 返回: Result<Vec<Service>, Box<dyn std::error::Error>>, 获取服务的列表
    pub async fn get_service(
        url: &str,
        service_name: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let get_service_url = format!("{}/v1/catalog/service/{}?passing", url, service_name);
        let response = client.get(get_service_url).send().await?;
        if response.status().is_success() {
            let services: Vec<serde_json::Value> = response.json().await?;
            Ok(services)
        } else {
            let err = format!("获取服务失败, 失败原因 {:?}", response);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )))
        }
    }
}
