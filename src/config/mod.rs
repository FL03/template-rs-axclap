/*
    Appellation: cnf <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Configuration
//!
//! This module implements the configuration settings for the platform. It is composed of several
//! sub-modules that define the various settings that can be configured.
#[doc(inline)]
pub use self::{kinds::*, settings::*, types::*, utils::*};

mod settings;

pub mod kinds {
    #[doc(inline)]
    pub use self::{database::*, network::*, scope::*, services::*, tracing::*, workspace::*};

    mod database;
    mod network;
    mod scope;
    mod services;
    mod tracing;
    mod workspace;
}

pub mod types {
    #[doc(inline)]
    pub use self::{dblink::*, log_level::*, mode::*, netaddr::*};

    mod dblink;
    mod log_level;
    mod mode;
    mod netaddr;
}

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::settings::*;
    pub use super::types::*;
}

serde_display! {
    json::<Display>(
        Settings,
        DatabaseConfig,
        NetworkConfig,
        ServicesConfig,
        TracingConfig,
        WorkspaceConfig,
    )
}

pub(crate) mod utils {
    use super::LogLevel;

    pub fn fmt_as_env_filter(level: LogLevel, name: &str) -> String {
        format!("{name}={level},tower_http={level}")
    }

    pub fn init_tracing(level: LogLevel, name: &str) {
        use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt};

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{name}={level},tower_http={level}").into());

        tracing_subscriber::fmt()
            .compact()
            .with_ansi(true)
            .with_env_filter(filter)
            .with_max_level(level.as_tracing_level())
            .with_target(true)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .finish()
            .init();
        tracing::debug!("Successfully initialized tracing with level: {level}");
    }

    /// Initialize the tracer with the given name
    fn _init_tracing(config: &super::TracingConfig, name: &str) {
        use tracing_subscriber::{
            filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt,
        };

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{name}={level},tower_http={level}", level = config.level).into()
        });

        let layer = tracing_subscriber::fmt::layer()
            .compact()
            .with_ansi(true)
            .with_level(true)
            .with_line_number(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_timer(tracing_subscriber::fmt::time::uptime());

        tracing_subscriber::registry()
            .with(filter)
            .with(layer)
            .init();
        tracing::trace!("Initialized tracing modules...");
    }
}
