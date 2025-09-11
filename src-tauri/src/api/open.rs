use crate::utils::Util;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    process::Command,
    sync::{LazyLock, Mutex},
};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Result};

#[derive(Debug, Deserialize, Serialize)]
struct OpenItem {
    name: String,
    pid: u32,
}

static CHROME_PID: LazyLock<Mutex<Vec<OpenItem>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static TELEGRAM_PID: LazyLock<Mutex<Vec<OpenItem>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

//打开chrome
#[tauri::command]
pub fn open_chrome(app: AppHandle, names: Vec<&str>) -> Result<serde_json::Value> {
    let config = Util::get_config(&app)?;
    let user_data_dir = config.chrome_user_data_dir;
    let chrome_install_dir = config.chrome_install_dir;
    let proxy = Util::get_proxy(&app)?;
    let url = config.url;
    let use_url = config.use_url;
    let use_proxy = config.use_proxy;
    let wallet = config.wallet;
    let chrome_exe_path = format!("{}\\chrome.exe", chrome_install_dir);

    if chrome_install_dir.is_empty() {
        return Ok(json!({
            "status": false,
            "msg": "请先设置chrome安装目录",
        }));
    }

    println!("name:{:?}", names);

    for name in names {
        let wallet_index = wallet.iter().position(|w| w == name);
        let proxy_postion = proxy.get(wallet_index.unwrap());
        let mut command = Command::new(&chrome_exe_path);
        command.arg(&format!("--user-data-dir={}\\{}", &user_data_dir, &name));
        if use_proxy && proxy_postion.is_some() {
            command.arg(&format!(
                "--proxy-server=socks5://{}",
                proxy_postion.unwrap()
            ));
        }
        if use_url && url.len() > 0 {
            command.args(&url);
        }
        let child = command.spawn().expect("chrome start failed");
        let chrome_item = OpenItem {
            name: name.to_string(),
            pid: child.id(),
        };
        CHROME_PID.lock().unwrap().push(chrome_item);
        // println!("启动进程信息：{:?}", CHROME_PID.lock().unwrap());
    }
    Ok(json!({
        "status": true,
    }))
}


//打开chrome
#[tauri::command]
pub fn open_telegram(app: AppHandle, names: Vec<&str>) -> Result<serde_json::Value> {
    let config = Util::get_config(&app)?;
    let telegram_install_dir = config.telegram_install_dir;
    if telegram_install_dir.is_empty() {
        return Ok(json!({
            "status": false,
            "msg": "请先设置chrome安装目录",
        }));
    }

    for name in names {
        let telegram_exe = format!("{}\\{}\\Telegram.exe", telegram_install_dir, name,);
        let mut command = Command::new(&telegram_exe);
        let child = command.spawn().expect("telegram start failed");
        let telegram_item = OpenItem {
            name: name.to_string(),
            pid: child.id(),
        };
        TELEGRAM_PID.lock().unwrap().push(telegram_item);
        // println!("启动进程信息：{:?}", CHROME_PID.lock().unwrap());
    }
    Ok(json!({
        "status": true,
    }))
}

#[tauri::command]
pub fn close_chrome(names: Vec<&str>) -> Result<serde_json::Value> {
    let mut open_chrome = CHROME_PID.lock().unwrap();
    for (index,name) in names.iter().enumerate() {
        let pid = open_chrome
            .iter()
            .find(|item| item.name == *name)
            .map(|item| item.pid);

        let sys = System::new_all();
        if let Some(pid) = pid {
            if let Some(process) = sys.process(Pid::from(pid as usize)) {
                process.kill();
                open_chrome.remove(index);
            }
        }
    }
    Ok(json!({
        "status": true,
    }))
}

//关闭telegram
#[tauri::command]
pub fn close_telegram(names: Vec<&str>) -> Result<serde_json::Value> {
    let mut telegram = TELEGRAM_PID.lock().unwrap();
    for (index,name) in names.iter().enumerate() {
        let pid = telegram
            .iter()
            .find(|item| item.name == *name)
            .map(|item| item.pid);

        let sys = System::new_all();
        if let Some(pid) = pid {
            if let Some(process) = sys.process(Pid::from(pid as usize)) {
                process.kill();
                telegram.remove(index);
            }
        }
    }
    Ok(json!({
        "status": true,
    }))
}

//获取所有打开的进程
#[tauri::command]
pub fn get_all_open() -> Result<serde_json::Value> {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()), // 只刷新进程数据
    );
    let pids = sys
        .processes()
        .keys()
        .map(|pid| pid.as_u32())
        .collect::<Vec<u32>>();
    let mut chrome = CHROME_PID.lock().unwrap();
    let mut telegram = TELEGRAM_PID.lock().unwrap();

    // println!("所有进程信息：{:?}", chrome);
    chrome.retain(|item| pids.contains(&item.pid));
    telegram.retain(|item| pids.contains(&item.pid));

    Ok(json!({
        "status": true,
        "data": {
            "chrome": *chrome,
            "telegram": *telegram,
        }
    }))
}


//关闭所有chrome
#[tauri::command]
pub fn close_chrome_all() -> Result<serde_json::Value> {
    let mut chrome = CHROME_PID.lock().unwrap();
    let sys = System::new_all();

    chrome.iter().for_each(|item| {
        let pid = item.pid;
        let process = sys.process(Pid::from_u32(pid));
        if let Some(process) = process {
            process.kill();
        }
    });
    chrome.clear();
    Ok(json!({
        "status": true,
    }))
}


//关闭所有telegram程序
#[tauri::command]
pub fn close_telegram_all() -> Result<serde_json::Value> {
    let mut telegram = TELEGRAM_PID.lock().unwrap();
    let sys = System::new_all();

    telegram.iter().for_each(|item| {
        let pid = item.pid;
        let process = sys.process(Pid::from_u32(pid));
        if let Some(process) = process {
            process.kill();
        }
    });
    telegram.clear();
    Ok(json!({
        "status": true,
    }))
}
