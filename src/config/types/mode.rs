/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Mode] enumerates the possible runtime modes of the application.
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
    clap::ValueEnum,
    scsys::VariantConstructors,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[clap(name = "debug")]
    #[serde(alias = "d", alias = "debug", alias = "dev", alias = "development")]
    Debug,
    #[clap(name = "release")]
    #[serde(alias = "r", alias = "release", alias = "prod", alias = "production")]
    Release,
}
