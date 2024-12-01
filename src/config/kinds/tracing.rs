/*
    Appellation: tracing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::LogLevel;

fn default_true() -> bool {
    true
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct TracingConfig {
    #[serde(default = "default_true")]
    pub(crate) ansi: bool,
    pub(crate) file: bool,
    pub(crate) level: LogLevel,
    pub(crate) line_number: bool,
    #[serde(default = "default_true")]
    pub(crate) target: bool,
    pub(crate) thread_ids: bool,
    pub(crate) thread_names: bool,
}

impl TracingConfig {
    pub fn new(level: LogLevel) -> Self {
        Self {
            ansi: true,
            file: false,
            level,
            line_number: false,
            target: true,
            thread_ids: false,
            thread_names: false,
        }
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    set_with! {
        ansi: bool,
        level: LogLevel,
        target: bool,
        thread_ids: bool,
        thread_names: bool,
    }
    /// Initialize the tracer with the given name
    pub fn init_tracing(&self, name: &str) {
        use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt};

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{name}={level},tower_http={level}", level = self.level).into()
        });

        tracing_subscriber::fmt()
            .compact()
            .with_ansi(self.ansi)
            .with_env_filter(filter)
            .with_file(self.file)
            .with_line_number(self.line_number)
            .with_max_level(self.level.as_tracing_level())
            .with_target(self.target)
            .with_thread_ids(self.thread_ids)
            .with_thread_names(self.thread_names)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .finish()
            .init();
        tracing::debug!("success: initialized tracing modules...");
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self::new(LogLevel::Trace)
    }
}
