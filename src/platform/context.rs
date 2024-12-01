/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Settings;
use crate::platform::PlatformState;

#[derive(Clone, Debug)]
pub struct PlatformContext {
    pub(crate) config: Settings,
    pub(crate) state: PlatformState,
}

impl PlatformContext {
    pub fn new(config: Settings, state: PlatformState) -> Self {
        Self { config, state }
    }

    gsw! {
        config: Settings,
        state: PlatformState,
    }
}

impl Default for PlatformContext {
    fn default() -> Self {
        Self {
            config: Settings::default(),
            state: PlatformState::default(),
        }
    }
}

impl core::ops::Deref for PlatformContext {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl core::ops::DerefMut for PlatformContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl axum::extract::FromRef<PlatformContext> for Settings {
    fn from_ref(ctx: &PlatformContext) -> Self {
        ctx.config.clone()
    }
}

impl axum::extract::FromRef<PlatformContext> for PlatformState {
    fn from_ref(ctx: &PlatformContext) -> Self {
        ctx.state.clone()
    }
}
