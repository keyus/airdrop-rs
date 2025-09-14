use serde_json::json;
use tauri::Result;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

fn create_subkey (key: &str, cur_ver: &RegKey)-> Result<RegKey>{
      let sub_key = match cur_ver.open_subkey(key) {
        Ok(subkey) => subkey,
        Err(_) => {
            let (subkey, _) = cur_ver.create_subkey(&key)?;
            subkey
        }
    };
    Ok(sub_key)
}


#[tauri::command]
pub fn add_extensions(id: &str) -> Result<serde_json::Value> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\WOW6432Node")?;
    let cur_ver = create_subkey("Google", &cur_ver)?;
    let cur_ver = create_subkey("Chrome", &cur_ver)?;
    let extensions = create_subkey("Extensions", &cur_ver)?;
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
