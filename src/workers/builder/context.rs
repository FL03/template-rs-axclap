/*
    Appellation: context <module>
    Contrib: @FL03
*/
use super::BuilderConfig;

#[derive(
    Clone, Debug, Default, Hash, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct BuilderContext {
    pub(crate) config: BuilderConfig,
}
