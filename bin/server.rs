/*
    Appellation: pzzld <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # pzzld
use template::{cli::Cli, platform::Platform, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build the settings
    let cnf = Settings::build()?;
    // initialize a new context
    let mut platform = Platform::from_config(dbg!(cnf)).with_tracing().init();
    // parse the command line
    let cli = Cli::new();
    // handle the command
    cli.handle(platform.get_mut()).await
}
