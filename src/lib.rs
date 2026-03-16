mod ftp;
mod network;

#[cfg(target_os = "android")]
mod android;

use ftp::FtpManager;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServerStatus {
    pub running: bool,
    pub port: u16,
    pub root_dir: String,
    pub address: String,
}

#[tauri::command]
async fn start_ftp(
    state: tauri::State<'_, FtpManager>,
    port: u16,
    root_dir: String,
    username: String,
    password: String,
) -> Result<String, String> {
    state.start(port, root_dir, username, password).await?;
    #[cfg(target_os = "android")]
    android::start_foreground_service();
    let ip = network::get_local_ip().unwrap_or_else(|_| "?".to_string());
    Ok(format!("ftp://{}:{}", ip, port))
}

#[tauri::command]
async fn stop_ftp(state: tauri::State<'_, FtpManager>) -> Result<(), String> {
    #[cfg(target_os = "android")]
    android::stop_foreground_service();
    state.stop().await
}

#[tauri::command]
async fn get_status(state: tauri::State<'_, FtpManager>) -> Result<ServerStatus, String> {
    let running = state.is_running().await;
    let (port, root_dir) = state.get_config().await;
    let address = if running {
        let ip = network::get_local_ip().unwrap_or_else(|_| "?".to_string());
        format!("ftp://{}:{}", ip, port)
    } else {
        String::new()
    };
    Ok(ServerStatus {
        running,
        port,
        root_dir,
        address,
    })
}

#[tauri::command]
fn get_local_ip() -> Result<String, String> {
    network::get_local_ip()
}

/// 返回当前平台推荐的 FTP 根目录（Android 为内置存储，桌面为 /tmp）
#[tauri::command]
fn get_default_root_dir() -> String {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0".into()
    }
    #[cfg(not(target_os = "android"))]
    {
        std::env::temp_dir()
            .to_str()
            .unwrap_or("/tmp")
            .to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(FtpManager::new())
        .invoke_handler(tauri::generate_handler![
            start_ftp,
            stop_ftp,
            get_status,
            get_local_ip,
            get_default_root_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}