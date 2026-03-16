//! FTP 服务器封装：基于 libunftp，负责启停与状态

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use libunftp::{ServerBuilder, auth::DefaultUser};
use unftp_sbe_fs::Filesystem;

use crate::ftp::auth::SimpleAuthenticator;

struct FtpState {
    handle: Option<JoinHandle<()>>,
    running: bool,
    port: u16,
    root_dir: String,
}

/// FTP 服务器管理器：启动、停止、查询状态
pub struct FtpManager {
    inner: Arc<Mutex<FtpState>>,
}

impl FtpManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(FtpState {
                handle: None,
                running: false,
                port: 0,
                root_dir: String::new(),
            })),
        }
    }

     /// 启动 FTP 服务器（单用户：仅接受指定的用户名和密码，禁止匿名）
     pub async fn start(
        &self,
        port: u16,
        root_dir: String,
        username: String,
        password: String,
    ) -> Result<(), String> {
        let mut state = self.inner.lock().await;
        if state.running {
            return Err("FTP 服务器已在运行".into());
        }

        let root_path = PathBuf::from(&root_dir);
        let sbe_generator: Box<dyn Fn() -> Filesystem + Send + Sync> =
            Box::new(move || Filesystem::new(root_path.clone()));
        let authenticator: Arc<dyn libunftp::auth::Authenticator<DefaultUser> + Send + Sync> =
            Arc::new(SimpleAuthenticator {
                username: username.clone(),
                password: password.clone(),
            });

            let server = ServerBuilder::with_authenticator(sbe_generator, authenticator)
            .greeting("Welcome to Ferry FTP Server")
            .passive_ports(50000..65535)
            .build()
            .map_err(|e| e.to_string())?;

        let addr = format!("0.0.0.0:{}", port);
        let handle = tokio::spawn(async move {
            if let Err(e) = server.listen(&addr).await {
                log::error!("FTP server listen error: {}", e);
            }
        });

        state.handle = Some(handle);
        state.running = true;
        state.port = port;
        state.root_dir = root_dir;
        Ok(())
    }

    /// 停止 FTP 服务器
    pub async fn stop(&self) -> Result<(), String> {
        let mut state = self.inner.lock().await;
        if let Some(h) = state.handle.take() {
            h.abort();
        }
        state.running = false;
        Ok(())
    }

    /// 当前是否在运行
    pub async fn is_running(&self) -> bool {
        let state = self.inner.lock().await;
        state.running
    }

    /// 当前配置：(port, root_dir)
    pub async fn get_config(&self) -> (u16, String) {
        let state = self.inner.lock().await;
        (state.port, state.root_dir.clone())
    }
}