use reqwest::{self, Client, header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs,
    collections::HashMap,
};
use std::result::Result;
use tauri::{AppHandle, Manager};

#[derive(Deserialize, Serialize)]
struct IpResult {
    proxy_address: String,
    port: u32,
}

#[derive(Deserialize, Serialize)]
struct IpListResponse {
    results: Vec<IpResult>,
}

static CONFIG_DIR: &str = "config";
static WEBSHARE_TOKEN: &str = "qp3ib2jesv7dxajv53uzdl4ri72h7zzr9n0h3del";
static WEBSHARE_BASE_URL: &str = "https://proxy.webshare.io/api/v2";
static WEBSHARE_IP_FILE: &str = "webshare.txt";

fn create_header() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    let value: header::HeaderValue =
        header::HeaderValue::from_str(&format!("Token {}", WEBSHARE_TOKEN))
            .expect("Invalid header value");
    headers.insert(header::AUTHORIZATION, value);
    headers
}

fn create_url(path: &str) -> String {
    format!("{}{}", WEBSHARE_BASE_URL, path)
}

//本机IP
#[tauri::command]
pub async fn my_ip() -> Result<serde_json::Value, String> {
    let headers = create_header();
    let client = Client::new();
    let response = client
        .get(create_url("/proxy/ipauthorization/whatsmyip/"))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "status": true,
        "data": response
    }))
}

//授权IP列表
#[tauri::command]
pub async fn auth_ip_list() -> Result<serde_json::Value, String> {
    let headers = create_header();
    let client = Client::new();
    let response = client
        .get(create_url("/proxy/ipauthorization/"))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "status": true,
        "data": response
    }))
}

//移除授权IP
#[tauri::command]
pub async fn remove_auth_ip(id: u32) -> Result<serde_json::Value, String> {
    let headers = create_header();
    let client = Client::new();
    client
        .delete(create_url(&format!("/proxy/ipauthorization/{}", id)))
        .headers(headers)
        .send()
        .await
        .map_err(|e| {
            println!("Error during request: {:?}", e);
            e.to_string()
        })?;
    
    Ok(json!({
        "status": true,
    }))
}

//添加授权IP
#[tauri::command]
pub async fn add_auth_ip(ip_address: String) -> Result<serde_json::Value, String> {
    println!("Adding auth IP: {}", ip_address);
    let headers = create_header();
    let client = Client::new();
    let mut payload = HashMap::new();
    payload.insert("ip_address", ip_address);
    println!("Payload: {:?}", payload);
    let response = client
        .post(create_url("/proxy/ipauthorization/"))
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            println!("Error during request: {:?}", e);
            e.to_string()
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            println!("Error parsing JSON: {:?}", e);
            e.to_string()
        })?;

    Ok(json!({
        "status": true,
        "data": response,
    }))
}

//获取购买的IP资源
#[tauri::command]
pub async fn ip_list(
    app: AppHandle,
    page: Option<u32>,
    page_size: Option<u32>,
    mode: Option<String>,
) -> Result<serde_json::Value, String> {
    let headers = create_header();
    let client = Client::new();

    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(100);
    let mode = mode.unwrap_or("direct".to_string());
    let url = &format!(
        "/proxy/list/?page={}&page_size={}&mode={}",
        page, page_size, mode
    );
    let response: IpListResponse = client
        .get(create_url(url))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<IpListResponse>()
        .await
        .map_err(|e| e.to_string())?;

    let ip_list = &response.results;
    let mut ip_string = String::new();
    for ip in ip_list {
        ip_string.push_str(&format!("{}:{}", ip.proxy_address, ip.port));
        ip_string.push('\n');
    }
    let webshare_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join(CONFIG_DIR)
        .join(WEBSHARE_IP_FILE);

    if webshare_path.exists() {
        fs::write(&webshare_path, ip_string).expect("webshare写入失败");
    }
    Ok(json!({
        "status": true,
        "data": response,
    }))
}
