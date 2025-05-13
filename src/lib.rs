/*
    Appellation: puzzled <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # puzzled
//!
//! `puzzled` is a library for creating and managing a network of nodes.
#[doc(inline)]
pub use self::{
    config::Settings,
    consts::*,
    error::Error,
    platform::{Platform, PlatformContext},
    traits::*,
    types::*,
};

#[macro_use]
pub(crate) mod macros;

pub mod cli;
pub mod config;
pub mod error;
pub mod platform;
pub mod traits;
pub mod workers;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::consts::*;

    pub use super::cli::prelude::*;
    pub use super::config::prelude::*;
    pub use super::error::Error;
    pub use super::platform::Platform;
    pub use super::traits::*;
    pub use super::types::*;
    pub use super::workers::prelude::*;
}

pub mod consts {
    pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
    /// The name of the application.
    pub const APP_NAME: &str = "templated";
    /// A str constant for the localhost address.
    pub const LOCALHOST: &str = "127.0.0.1";
    /// The default host for the application.
    pub const DEFAULT_HOST: &str = "0.0.0.0";
    /// The default port for the application.
    pub const DEFAULT_PORT: u16 = 8080;
    /// The default name of the executable
    pub const DEFAULT_APPLICATION: &str = "templated";
    /// The default basepath for the application.
    pub const DEFAULT_BASEPATH: &str = "/";
    /// The default database url for the application.
    pub const DEFAULT_DB_URL: &str = "postgresql://postgres:postgres@localhost:5432/postgres";
    /// The default directory name to the artifacts directory.
    pub const DEFAULT_DIR_ARTIFACTS: &str = ".artifacts";
    /// The default location for the configuration directory.
    pub const DEFAULT_DIR_CONFIG: &str = ".config";
    /// The name of the artifacts directory.
    pub const DEFAULT_DIR_CONTEXT: &str = "/opt/templated";
    /// The name of the default working directory.
    pub const DEFAULT_WORKDIR: &str = "dist";
    /// The default name for the primary configuration file.
    pub const DEFAULT_CONFIG_FILE: &str = "Templated.toml";

    #[allow(unused)]
    pub(crate) const ROOT: &str = env!("CARGO_MANIFEST_DIR");
}

#[allow(dead_code)]
pub mod types {
    /// A type alias for [`Sender<T>`](tokio::sync::broadcast::Sender).
    pub(crate) type PowerTx = tokio::sync::broadcast::Sender<()>;
    /// A type alias for [`u32`] used to identify an object
    pub type Uid = u32;
}
