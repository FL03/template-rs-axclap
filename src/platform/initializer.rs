/*
    Appellation: initializer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Settings;
use crate::platform::{Platform, PlatformContext, PlatformState};

#[derive(Default)]
pub struct Initializer {
    pub(crate) config: Settings,
    pub(crate) state: Option<PlatformState>,
}

impl Initializer {
    pub(crate) fn new() -> Self {
        Self {
            config: Settings::build().unwrap_or_default(),
            state: None,
        }
    }
    /// Create a new [Initializer] with the provided [Settings].
    pub fn from_config(config: Settings) -> Self {
        Self {
            config,
            state: None,
        }
    }

    pub fn config(self, config: Settings) -> Self {
        Self { config, ..self }
    }

    pub fn state(self, state: PlatformState) -> Self {
        Self {
            state: Some(state),
            ..self
        }
    }

    pub fn set_curdir(self) -> Self {
        self.config.workspace().set_current_dir();
        self
    }

    pub fn with_tracing(self) -> Self {
        self.config.init_tracing();
        self
    }

    pub fn init(self) -> Platform {
        let inner = PlatformContext {
            config: self.config,
            state: self.state.unwrap_or_default(),
        };
        Platform {
            inner: std::sync::Arc::new(inner),
        }
    }
}
