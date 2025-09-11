use serde_json::json;
use tauri::Result;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

#[tauri::command]
pub fn add_extensions(id: &str) -> Result<serde_json::Value> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Google\\Chrome")?;
    let extensions = match cur_ver.open_subkey("Extensions") {
        Ok(subkey) => subkey,
        Err(_) => {
            let (subkey, _) = cur_ver.create_subkey("Extensions")?;
            subkey
        }
    };
    let (id_ver, _) = extensions.create_subkey(id)?;
    id_ver.set_value(
        "update_url",
        &"https://clients2.google.com/service/update2/crx",
    )?;
    Ok(json!({
        "status": true,
    }))
}

#[tauri::command]
pub fn remove_extensions(id: &str) -> Result<serde_json::Value> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Google\\Chrome\\Extensions")?;
    let _ = cur_ver.delete_subkey_all(&id)?;
    Ok(json!({
        "status": true,
    }))
}
