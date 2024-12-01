/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{cmd::Command, interface::Cli};

mod interface;

pub mod cmd;

pub(crate) mod prelude {
    pub use super::cmd::*;
    pub use super::interface::Cli;
}

serde_display!(json::<Display>(Cli));
