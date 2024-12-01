/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::cmd::*;
use crate::{platform::PlatformContext, AsyncHandle};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Command>,
    #[clap(long, short = 'C', default_value_t = String::from("Puzzled.toml"))]
    pub config: String,
    #[clap(action = clap::ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl Cli {
    pub fn new() -> Self {
        clap::Parser::parse()
    }

    pub fn command(&self) -> Option<&Command> {
        self.command.as_ref()
    }

    pub fn config(&self) -> &str {
        &self.config
    }

    pub fn release(&self) -> bool {
        self.release
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    #[tracing::instrument(skip_all, name = "handle", target = "cli")]
    pub async fn handle<Ctx>(self, ctx: &mut Ctx) -> <Self as AsyncHandle<Ctx>>::Output
    where
        Self: AsyncHandle<Ctx>,
        Ctx: core::fmt::Debug,
    {
        <Self as AsyncHandle<Ctx>>::handle(self, ctx).await
    }
}

#[async_trait::async_trait]
impl AsyncHandle<PlatformContext> for Cli {
    type Output = anyhow::Result<()>;

    async fn handle(self, ctx: &mut PlatformContext) -> Self::Output {
        if let Some(cmd) = self.command {
            cmd.handle(ctx).await?;
        }

        Ok(())
    }
}
