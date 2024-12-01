/*
    Appellation: log_level <module>
    Contrib: @FL03
*/
use tracing::Level;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::VariantConstructors,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Debug,
    Error,
    Info,
    Trace,
    Warn,
    Off,
}
impl LogLevel {
    pub fn from_str<S>(s: S) -> Self
    where
        S: AsRef<str>,
    {
        match s.as_ref() {
            "debug" => Self::Debug,
            "error" => Self::Error,
            "info" => Self::Info,
            "trace" => Self::Trace,
            "warn" => Self::Warn,
            _ => Self::Off,
        }
    }
    /// Converts a [tracing::Level] to a [LogLevel]
    pub fn from_tracing_level(level: Level) -> Self {
        match level {
            Level::DEBUG => Self::Debug,
            Level::ERROR => Self::Error,
            Level::INFO => Self::Info,
            Level::TRACE => Self::Trace,
            Level::WARN => Self::Warn,
        }
    }
    /// Converts a [LevelFilter] to a [LogLevel]
    pub fn from_level_filter(filter: LevelFilter) -> Self {
        match filter {
            LevelFilter::DEBUG => Self::Debug,
            LevelFilter::ERROR => Self::Error,
            LevelFilter::INFO => Self::Info,
            LevelFilter::TRACE => Self::Trace,
            LevelFilter::WARN => Self::Warn,
            LevelFilter::OFF => Self::Off,
        }
    }

    pub fn as_tracing_level(&self) -> Option<tracing::Level> {
        use tracing::Level;

        match self {
            Self::Debug => Some(Level::DEBUG),
            Self::Error => Some(Level::ERROR),
            Self::Info => Some(Level::INFO),
            Self::Trace => Some(Level::TRACE),
            Self::Warn => Some(Level::WARN),
            Self::Off => None,
        }
    }

    pub fn as_level_filter(&self) -> LevelFilter {
        match self {
            Self::Debug => LevelFilter::DEBUG,
            Self::Error => LevelFilter::ERROR,
            Self::Info => LevelFilter::INFO,
            Self::Trace => LevelFilter::TRACE,
            Self::Warn => LevelFilter::WARN,
            Self::Off => LevelFilter::OFF,
        }
    }

    pub fn as_env_filter(&self, name: &str) -> EnvFilter {
        let filter = self.fmt_as_env_filter(name);
        EnvFilter::try_from_default_env().unwrap_or(filter.into())
    }

    pub fn fmt_as_env_filter(&self, name: &str) -> String {
        format!("{name}={level},tower_http={level}", level = self)
    }

    pub fn init_tracing(&self, name: &str) {
        crate::config::init_tracing(*self, name);
    }
}

impl From<LogLevel> for config::Value {
    fn from(level: LogLevel) -> Self {
        level.to_string().into()
    }
}

impl From<Level> for LogLevel {
    fn from(level: Level) -> Self {
        Self::from_tracing_level(level)
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        level.as_tracing_level().unwrap_or(Level::INFO)
    }
}

impl From<LevelFilter> for LogLevel {
    fn from(filter: LevelFilter) -> Self {
        Self::from_level_filter(filter)
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        level.as_level_filter()
    }
}

unsafe impl Send for LogLevel {}

unsafe impl Sync for LogLevel {}
