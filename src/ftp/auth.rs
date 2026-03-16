use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};

/// 只接受一对固定用户名和密码的认证器
#[derive(Debug)]
pub struct SimpleAuthenticator {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
impl Authenticator<DefaultUser> for SimpleAuthenticator {
    async fn authenticate(&self, username: &str, creds: &Credentials) -> Result<DefaultUser, AuthenticationError> {
        let password_ok = creds
            .password
            .as_deref()
            .map(|p| p == self.password)
            .unwrap_or(false);
        if username == self.username && password_ok {
            Ok(DefaultUser)
        } else if username != self.username {
            Err(AuthenticationError::BadUser)
        } else {
            Err(AuthenticationError::BadPassword)
        }
    }

    fn name(&self) -> &str {
        "SimpleAuthenticator"
    }
}