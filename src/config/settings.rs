/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{kinds::*, types::*};

type ConfigBuilder<S = config::builder::DefaultState> = config::builder::ConfigBuilder<S>;

type ConfigResult<T> = core::result::Result<T, config::ConfigError>;

fn _load_env_or_default(
    env: &str,
    default: impl ToString,
) -> String {
    std::env::var(env).unwrap_or_else(|_| default.to_string())
}

fn fmt_src(dir: impl core::fmt::Display, fname: impl core::fmt::Display) -> String {
    format!("{dir}/{fname}")
}

fn with_sources<T: core::fmt::Display>(
    ctx: ConfigBuilder,
    workdir: &str,
    names: impl IntoIterator<Item = T>,
) -> ConfigBuilder {
    let mut tmp = ctx;
    for name in names {
        tmp = tmp.add_source(config::File::with_name(&fmt_src(workdir, name)).required(false));
    }
    tmp
}

fn set_default(builder: ConfigBuilder) -> ConfigResult<ConfigBuilder> {
    let builder = builder
        .set_default("mode", "debug")?
        .set_default("name", crate::APP_NAME)?
        .set_default("version", env!("CARGO_PKG_VERSION"))?
        .set_default("scope.context", ".")?
        .set_default("scope.workdir", crate::DEFAULT_WORKDIR)?
        .set_default("network.address.host", crate::DEFAULT_HOST)?
        .set_default("network.address.port", crate::DEFAULT_PORT)?
        .set_default("services.tracing.level", "info")?;
    Ok(builder)
}

fn add_sources(builder: ConfigBuilder) -> ConfigBuilder {
    let workdir = _load_env_or_default(
        "APP_CONFIG_DIR",
        crate::DEFAULT_DIR_CONFIG,
    );
    // get the settings file name
    let fname = _load_env_or_default(
        "APP_CONFIG_FILE",
        crate::DEFAULT_CONFIG_FILE,
    );
    // setup the builder's sources
    let builder = with_sources(
        builder,
        &workdir,
        &[
            "default.config",
            "server.config",
            "docker.config",
            "app.config",
            "prod.config",
            &fname,
        ],
    );

    builder
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .add_source(config::File::with_name(&fname).required(false))
}

fn set_overrides(builder: ConfigBuilder) -> ConfigResult<ConfigBuilder> {
    Ok({
        builder
            .set_override_option("mode", std::env::var("APP_MODE").ok())?
            .set_override_option("name", std::env::var("APP_NAME").ok())?
            .set_override_option("network.address.host", std::env::var("APP_HOST").ok())?
            .set_override_option("network.address.port", std::env::var("APP_PORT").ok())?
            .set_override_option("workspace.workdir", std::env::var("APP_WORKDIR").ok())?
    })
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct Settings {
    pub mode: Mode,
    pub name: String,
    pub network: NetworkConfig,
    pub scope: Scope,
    pub services: ServicesConfig,
    pub version: String,
    pub workspace: WorkspaceConfig,
}

impl Settings {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            name: crate::APP_NAME.to_string(),
            network: NetworkConfig::default(),
            scope: Scope::default(),
            services: ServicesConfig::default(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            workspace: WorkspaceConfig::default(),
        }
    }

    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder_base()?.build()?.try_deserialize()
    }

    gsw! {
        network: NetworkConfig,
        services: ServicesConfig,
        workspace: WorkspaceConfig,
    }

    getter!(name: String, version: String);
    /// binds a listener to the given address
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        self.network().bind().await
    }
    /// Returns a new [Settings] instance with the [Mode] set to [Mode::Debug].
    pub fn debug(self) -> Self {
        Self {
            mode: Mode::debug(),
            ..self
        }
    }
    /// returns a copy of the [Mode] of the instance
    pub fn mode(&self) -> Mode {
        self.mode
    }
    /// Returns a new [Settings] instance with the [Mode] set to [Mode::Release].
    pub fn release(self) -> Self {
        Self {
            mode: Mode::release(),
            ..self
        }
    }
    /// Initialize tracing modules
    pub fn init_tracing(&self) {
        self.services().tracing().init_tracing(crate::APP_NAME);
    }
    /// set the working directory of the scope
    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        std::path::PathBuf: From<T>,
    {
        self.workspace_mut().set_workdir(workdir.into());
    }
    /// if the workdir is set, set it to the given workdir
    pub fn set_workdir_option<T>(&mut self, workdir: Option<T>)
    where
        std::path::PathBuf: From<T>,
    {
        workdir.map(|w| self.set_workdir(w));
    }

    pub fn set_port(&mut self, port: u16) {
        self.network_mut().set_port(port);
    }

    pub fn set_log_level(&mut self, level: LogLevel) {
        self.services_mut().tracing_mut().set_level(level);
    }

    fn builder_base() -> ConfigResult<ConfigBuilder> {
        // initialize the builder
        let mut builder = config::Config::builder();
        // set defaults
        builder = set_default(builder)?;
        // add sources
        builder = add_sources(builder);
        // set overrides
        builder = set_overrides(builder)?;
        // return the builder
        Ok(builder)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(Mode::Debug)
    }
}
