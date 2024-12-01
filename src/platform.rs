/*
    Appellation: platform <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{context::PlatformContext, initializer::Initializer, state::PlatformState};

mod context;
mod initializer;
mod state;

use crate::config::Settings;
use std::sync::Arc;

pub type PlatformInner = std::sync::Arc<PlatformContext>;

/// [Platform] is used as the primary entry point for the application. The struct is used to
/// manage the application state and configuration.
#[derive(Clone, Debug)]
pub struct Platform {
    pub(crate) inner: Arc<PlatformContext>,
}

impl Platform {
    pub fn new() -> Initializer {
        Initializer::new()
    }

    pub fn from_config(config: Settings) -> Initializer {
        Initializer::from_config(config)
    }

    pub fn inner(&self) -> &Arc<PlatformContext> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Arc<PlatformContext> {
        &mut self.inner
    }

    pub fn get_ref(&self) -> &PlatformContext {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut PlatformContext {
        Arc::make_mut(&mut self.inner)
    }
}

impl AsRef<PlatformContext> for Platform {
    fn as_ref(&self) -> &PlatformContext {
        self.get_ref()
    }
}

impl AsMut<PlatformContext> for Platform {
    fn as_mut(&mut self) -> &mut PlatformContext {
        self.get_mut()
    }
}

impl core::ops::Deref for Platform {
    type Target = PlatformContext;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

impl core::ops::DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl From<PlatformContext> for Platform {
    fn from(inner: PlatformContext) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl From<PlatformInner> for Platform {
    fn from(inner: PlatformInner) -> Self {
        Self { inner }
    }
}

impl From<Platform> for PlatformInner {
    fn from(platform: Platform) -> Self {
        platform.inner
    }
}
