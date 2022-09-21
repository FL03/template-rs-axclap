/*
    Appellation: mod <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{args::{PowerArgs, CRUDArgs, WebRequestArgs}, cmds::Commands};
use clap::Parser;

mod args;
mod cmds;

#[derive(Clone, Debug, Hash, Parser, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Welcome to Conduit")]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Commands,
}

impl CommandLineInterface {
    pub fn data() -> Self {
        Self::parse()
    }
}
