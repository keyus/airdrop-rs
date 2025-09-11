pub mod api;
pub mod utils;
use api::config::{clear, get_config, get_proxy, set_config, set_proxy};
use api::webshare::{add_auth_ip, auth_ip_list, ip_list, my_ip, remove_auth_ip};
use api::open::{
    open_chrome,
    open_telegram,
    close_chrome,
    get_all_open,
    close_chrome_all,
    close_telegram_all,
    close_telegram,
};
use api::chrome_app::{
    add_extensions,
    remove_extensions,
};
use utils::Util;


pub static CONFIG_DIR:&str = "config";
pub static CONFIG_FILE_NAME:&str = "config.json";
pub static PROXY_FILE_NAME:&str = "webshare.txt";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            Util::init_config(&app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_config,
            get_proxy,
            set_proxy,
            clear,
            my_ip,
            ip_list,
            add_auth_ip,
            remove_auth_ip,
            auth_ip_list,

            open_chrome,
            open_telegram,
            close_chrome,
            get_all_open,
            close_chrome_all,
            close_telegram_all,
            close_telegram,

            add_extensions,
            remove_extensions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
