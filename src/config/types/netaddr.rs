/*
    Appellation: server_addr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn default_ip() -> String {
    core::net::IpAddr::V4(core::net::Ipv4Addr::LOCALHOST).to_string()
}

fn default_port() -> u16 {
    crate::DEFAULT_PORT
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct NetAddr {
    #[serde(default = "default_ip")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl NetAddr {
    pub fn new(host: impl ToString, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn from_socket_addr(addr: core::net::SocketAddr) -> Self {
        Self {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }

    pub fn localhost(port: u16) -> Self {
        Self::new(default_ip(), port)
    }

    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.to_string().parse().unwrap()
    }
    /// initialize a new listener, bound to the configured address
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(&self.as_socket_addr()).await
    }

    pub fn ip(&self) -> core::net::IpAddr {
        self.as_socket_addr().ip()
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn set_host(&mut self, host: impl ToString) {
        self.host = host.to_string();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn with_host(self, host: impl ToString) -> Self {
        Self {
            host: host.to_string(),
            ..self
        }
    }

    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }
}

impl Default for NetAddr {
    fn default() -> Self {
        Self {
            host: crate::DEFAULT_HOST.to_string(),
            port: crate::DEFAULT_PORT,
        }
    }
}

impl core::fmt::Display for NetAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{host}:{port}", host = self.host, port = self.port)
    }
}

impl core::str::FromStr for NetAddr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse as a url to get the scheme, host and port
        let url = s.parse::<url::Url>()?;
        let host = url.host_str().expect("Failed to parse host");
        let port = url.port().expect("failed to parse port");

        let res = Self {
            host: host.to_string(),
            port,
        };
        Ok(res)
    }
}

impl From<core::net::SocketAddr> for NetAddr {
    fn from(addr: core::net::SocketAddr) -> Self {
        Self::from_socket_addr(addr)
    }
}
