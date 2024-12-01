/*
    Appellation: config <module>
    Contrib: @FL03
*/
use crate::config::{NetAddr, Settings};
use core::net::SocketAddr;
use std::path::PathBuf;

fn _default_address() -> SocketAddr {
    NetAddr::default().as_socket_addr()
}

fn _default_basepath() -> String {
    "/".to_string()
}

fn _default_workdir() -> PathBuf {
    std::env::current_dir().unwrap_or(crate::DEFAULT_WORKDIR.into())
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ServerConfig {
    #[serde(default = "_default_address")]
    pub(crate) address: SocketAddr,
    #[serde(default = "_default_basepath")]
    pub(crate) basepath: String,
    #[serde(default = "_default_workdir")]
    pub(crate) workdir: PathBuf,
}

impl ServerConfig {
    pub fn new(address: SocketAddr, basepath: impl ToString, workdir: impl ToString) -> Self {
        Self {
            address,
            basepath: basepath.to_string(),
            workdir: PathBuf::from(workdir.to_string()),
        }
    }

    pub fn from_config(settings: Settings) -> Self {
        let network = settings.network;
        let workspace = settings.workspace;
        Self {
            address: network.address.as_socket_addr(),
            basepath: network.basepath().to_string(),
            workdir: PathBuf::from(workspace.path_to_artifacts()),
        }
    }

    gsw! {
        address: SocketAddr,
        basepath: String,
        workdir: PathBuf,
    }

    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(self.address).await
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let workdir = std::env::current_dir().unwrap();
        Self::new(_default_address(), _default_basepath(), workdir.display())
    }
}

impl From<Settings> for ServerConfig {
    fn from(config: Settings) -> Self {
        Self::from_config(config)
    }
}

impl<'a> From<&'a Settings> for ServerConfig {
    fn from(config: &'a Settings) -> Self {
        Self::from_config(config.clone())
    }
}
