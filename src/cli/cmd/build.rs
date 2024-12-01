/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
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
pub struct BuildCmd {
    #[clap(subcommand)]
    pub args: Option<BuildOpts>,
    #[clap(long, short)]
    pub platform: Option<String>,
    #[clap(long, short)]
    pub target: Option<String>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIs,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BuildOpts {
    Wasm,
}

impl BuildCmd {
    #[tracing::instrument(skip(self, ctx), name = "handle", target = "build")]
    pub async fn handle<Ctx>(self, ctx: &mut Ctx) -> <Self as AsyncHandle<Ctx>>::Output
    where
        Self: AsyncHandle<Ctx>,
    {
        <Self as AsyncHandle<Ctx>>::handle(self, ctx).await
    }
}

#[async_trait::async_trait]
impl AsyncHandle<PlatformContext> for BuildCmd {
    type Output = anyhow::Result<()>;

    async fn handle(self, ctx: &mut PlatformContext) -> Self::Output {
        let _ws = ctx.config().workspace();
        if let Some(args) = self.args {
            match args {
                BuildOpts::Wasm => {
                    tracing::info!("Building for WebAssembly...");
                }
            }
        }

        Ok(())
    }
}
