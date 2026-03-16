mod ftp;
mod network;

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
    let ip = network::get_local_ip().unwrap_or_else(|_| "?".to_string());
    Ok(format!("ftp://{}:{}", ip, port))
}

#[tauri::command]
async fn stop_ftp(state: tauri::State<'_, FtpManager>) -> Result<(), String> {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}