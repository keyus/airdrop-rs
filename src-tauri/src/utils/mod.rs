use fs_extra::dir::{CopyOptions, copy, remove};
use serde_json;
use tauri::{AppHandle, Manager, Result};
use serde;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config{
    pub chrome_install_dir:String,
    pub chrome_user_data_dir:String,
    pub telegram_install_dir:String,
    pub use_url: bool,
    pub url: Vec<String>,
    pub use_proxy: bool,
    pub wallet: Vec<String>,
}

pub struct Util;
impl Util {
    // 初始化配置文件到用户目录
    pub fn init_config(app: &AppHandle) -> Result<()> {
        let path = app.path();
        let app_data_dir = path.app_data_dir()?;
        let resource_dir = path.resource_dir()?.join(crate::CONFIG_DIR);
        let app_resource = path.app_data_dir()?.join(crate::CONFIG_DIR);
        if app_resource.exists() == false {
            copy(&resource_dir, &app_data_dir, &CopyOptions::new())
                .map_err(|e| tauri::Error::AssetNotFound(e.to_string()))?;
        }
        Ok(())
    }
    // 清除配置并重新复制默认配置
    pub fn clear(app: &AppHandle) -> Result<()> {
        let path = app.path();
        let app_data_dir = path.app_data_dir()?;
        let resource_dir = path.resource_dir()?.join(crate::CONFIG_DIR);
        let app_resource = path.app_data_dir()?.join(crate::CONFIG_DIR);
        if app_resource.exists() {
            let _ = remove(&app_resource);
        }
        copy(&resource_dir, &app_data_dir, &CopyOptions::new())
            .map_err(|e| tauri::Error::AssetNotFound(e.to_string()))?;
        Ok(())
    }

    // 获取配置文件返回json struct
    pub fn get_config(app: &AppHandle) -> Result<Config> {
        let path = app.path().app_data_dir()?;
        let config_path = path.join(crate::CONFIG_DIR).join(crate::CONFIG_FILE_NAME);
        let json_string = std::fs::read_to_string(&config_path)?;
        let config:Config = serde_json::from_str(&json_string)?;
        Ok(config)
    }
    
    // 获取代理列表
    pub fn get_proxy(app: &AppHandle) -> Result<Vec<String>> {
        let path = app.path().app_data_dir()?;
        let proxy_path = path.join(crate::CONFIG_DIR).join(crate::PROXY_FILE_NAME);
        let proxy_string = std::fs::read_to_string(&proxy_path)?;
        Ok(proxy_string.lines().map(|s| s.to_string()).collect())
    }
}
