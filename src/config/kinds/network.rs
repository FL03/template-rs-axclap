/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::NetAddr;

fn _default_basepath() -> String {
    crate::DEFAULT_BASEPATH.to_string()
}

fn _default_max_connections() -> u16 {
    15
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(default)]
pub struct NetworkConfig {
    // #[serde(flatten)]
    pub(crate) address: NetAddr,
    #[serde(default = "_default_basepath")]
    pub(crate) basepath: String,
    #[serde(default = "_default_max_connections")]
    pub(crate) max_connections: u16,
    pub(crate) open: bool,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            address: NetAddr::default(),
            basepath: _default_basepath(),
            max_connections: _default_max_connections(),
            open: false,
        }
    }

    gsw! {
        address: NetAddr,
        basepath: String,
        max_connections: u16,
    }
    /// Returns an instance of the address as a [SocketAddr](core::net::SocketAddr)
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.address.as_socket_addr()
    }
    /// Binds the address to a [TcpListener](tokio::net::TcpListener)
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        self.address().bind().await
    }
    /// Returns the host of the address
    pub fn host(&self) -> &str {
        self.address().host()
    }
    /// Returns the ip of the address
    pub fn ip(&self) -> core::net::IpAddr {
        self.address().ip()
    }
    /// Determines if the server should open the link in a browser
    pub fn open(&self) -> bool {
        self.open
    }
    /// Returns the port of the address
    pub fn port(&self) -> u16 {
        self.address.port
    }
    /// Sets the port of the address
    pub fn set_port(&mut self, port: u16) {
        self.address.port = port;
    }
    /// consumes the current instance and returns a new instance with the port set
    pub fn with_port(self, port: u16) -> Self {
        Self {
            address: NetAddr {
                port,
                ..self.address
            },
            ..self
        }
    }
    /// Sets the host of the address
    pub fn set_host(&mut self, host: impl ToString) {
        self.address.host = host.to_string();
    }
    /// consumes the current instance and returns a new instance with the host set
    pub fn with_host(self, host: impl ToString) -> Self {
        Self {
            address: self.address.with_host(host),
            ..self
        }
    }
}
