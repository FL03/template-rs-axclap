/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{ServerConfig, ServerState};
use std::sync::Arc;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[doc(hidden)]
pub struct ServerContext {
    /// The address the server is bound to
    pub(crate) config: Arc<ServerConfig>,
    /// the server's state; i.e. clients, data, etc.
    pub(crate) state: ServerState,
}

impl ServerContext {
    pub fn new(config: crate::Settings) -> Self {
        Self {
            config: Arc::new(ServerConfig::from_config(config)),
            state: ServerState::new(),
        }
    }

    gsw! {
        state: ServerState,
    }

    pub fn address(&self) -> core::net::SocketAddr {
        self.config().address
    }
    /// initialize a new listener, bound to the configured address
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        self.config().bind().await
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ServerConfig {
        Arc::make_mut(&mut self.config)
    }

    pub fn into_shared(self) -> std::sync::Arc<Self> {
        Arc::new(self)
    }

    pub fn create_serve_dir_svc(&self) -> tower_http::services::ServeDir {
        tower_http::services::ServeDir::new(self.config().workdir())
    }
}

unsafe impl Send for ServerContext {}

unsafe impl Sync for ServerContext {}

impl core::ops::Deref for ServerContext {
    type Target = ServerConfig;

    fn deref(&self) -> &Self::Target {
        self.config()
    }
}

impl core::ops::DerefMut for ServerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.config_mut()
    }
}
