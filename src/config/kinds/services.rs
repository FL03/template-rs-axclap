/*
    Appellation: services <module>
    Contrib: @FL03
*/
use super::{DatabaseConfig, TracingConfig};

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
pub struct ServicesConfig {
    pub database: DatabaseConfig,
    pub tracing: TracingConfig,
}

impl ServicesConfig {
    pub fn new() -> Self {
        Self {
            database: DatabaseConfig::new(),
            tracing: TracingConfig::new(crate::config::LogLevel::Info),
        }
    }

    gsw! {
        database: DatabaseConfig,
        tracing: TracingConfig
    }
}
