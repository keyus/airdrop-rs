use crate::utils::Util;
use serde_json::json;
use std::fs;
use tauri::{AppHandle, Manager, Result};

// 读取配置
#[tauri::command]
pub fn get_config(app: AppHandle) -> Result<serde_json::Value> {
    let data_path = app
        .path()
        .app_data_dir()?
        .join(crate::CONFIG_DIR)
        .join(crate::CONFIG_FILE_NAME);
    let json_string = fs::read_to_string(&data_path)?;
    let data: serde_json::Value = serde_json::from_str(&json_string)?;
    Ok(json!({
        "status": true,
        "data": data
    }))
} 

// 写入config json文件
#[tauri::command]
pub fn set_config(app: AppHandle, config:serde_json::Value) -> Result<serde_json::Value> {
    let config_path = app
        .path()
        .app_data_dir()?
        .join(crate::CONFIG_DIR);
    fs::create_dir_all(&config_path)?; 
    let file_path = config_path.join(crate::CONFIG_FILE_NAME);
    let json_string = serde_json::to_string_pretty(&config)?;
    fs::write(&file_path, json_string)?;
    Ok(json!({"status": true}))
}


#[tauri::command]
// 清除重新复制resource config更新用户配置
pub fn clear(app: AppHandle) -> Result<serde_json::Value> {
    let _ = Util::clear(&app);
    Ok(json!({"status": true}))
}

//读取webshare代理
#[tauri::command]
pub fn get_proxy(app: AppHandle) -> Result<serde_json::Value> {
    let data_path = app
        .path()
        .app_data_dir()?
        .join(crate::CONFIG_DIR)
        .join(crate::PROXY_FILE_NAME);
    let json_string = fs::read_to_string(&data_path)?;
    Ok(json!({
        "status": true,
        "data": json_string
    }))
}


//读取webshare代理
#[tauri::command]
pub fn set_proxy(app: AppHandle, proxy: &str) -> Result<serde_json::Value> {
    let data_path = app
        .path()
        .app_data_dir()?
        .join(crate::CONFIG_DIR);
    fs::create_dir_all(&data_path)?;
    let file_path = data_path.join(crate::PROXY_FILE_NAME);
    fs::write(&file_path, proxy)?;
    Ok(json!({
        "status": true,
    }))
}