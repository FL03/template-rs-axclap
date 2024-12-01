/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[strum_discriminants(
    name(ErrorKind),
    derive(
        Hash,
        Ord,
        PartialOrd,
        serde::Deserialize,
        serde::Serialize,
        strum::AsRefStr,
        strum::Display,
        strum::EnumIs,
        strum::EnumString,
        strum::VariantNames
    )
)]
pub enum Error {
    Config(String),
    Serde(String),
    Unknown(String),
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl ToString) -> Self {
        let message = message.to_string();
        match kind {
            ErrorKind::Config => Self::Config(message),
            ErrorKind::Serde => Self::Serde(message),
            ErrorKind::Unknown => Self::Unknown(message),
        }
    }
    /// Returns the kind of discriminated error.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Config(_) => ErrorKind::Config,
            Self::Serde(_) => ErrorKind::Serde,
            Self::Unknown(_) => ErrorKind::Unknown,
        }
    }
    /// Returns the message of the error.
    pub fn message(&self) -> &str {
        match self {
            Self::Config(e) => e,
            Self::Serde(e) => e,
            Self::Unknown(e) => e,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "[{tag}] {content}",
            tag = self.kind(),
            content = self.message()
        )
    }
}

impl core::error::Error for Error {}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Unknown(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Unknown(e.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(e.to_string())
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::Config(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e.to_string())
    }
}
