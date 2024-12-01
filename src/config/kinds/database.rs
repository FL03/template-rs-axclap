/*
    Appellation: database <config>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::{DatabaseUrl, DbUrl};

fn default_max_connections() -> u32 {
    200
}

fn default_pool_size() -> u32 {
    15
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct DatabaseConfig {
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(alias = "link", alias = "uri")]
    pub url: DbUrl,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self {
            max_connections: default_max_connections(),
            pool_size: default_pool_size(),
            url: DbUrl::default(),
        }
    }

    gsw! {
        max_connections: u32,
        pool_size: u32,
    }

    pub fn url(&self) -> DatabaseUrl {
        self.url.as_db_url()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}
