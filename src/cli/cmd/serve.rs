/*
    Appellation: serve <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::platform::PlatformContext;
use crate::traits::AsyncHandle;
use crate::workers::Server;

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
pub struct ServeCmd {
    #[clap(subcommand)]
    pub args: Option<ServeOpts>,
    #[clap(long, short = 'H')]
    pub host: Option<String>,
    #[clap(long, short)]
    pub port: Option<u16>,
    #[clap(long, short)]
    pub workdir: Option<String>,
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
    scsys::VariantConstructors,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum ServeOpts {
    Run {
        #[clap(long, short)]
        prefix: Option<String>,
    },
}

impl ServeCmd {
    pub fn new() -> Self {
        clap::Parser::parse()
    }

    pub fn args(&self) -> Option<&ServeOpts> {
        self.args.as_ref()
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    #[tracing::instrument(skip_all, name = "handle", target = "serve")]
    pub async fn handle<Ctx>(self, ctx: &mut Ctx) -> <Self as AsyncHandle<Ctx>>::Output
    where
        Self: AsyncHandle<Ctx>,
        Ctx: core::fmt::Debug,
    {
        <Self as AsyncHandle<Ctx>>::handle(self, ctx).await
    }
}

#[async_trait::async_trait]
impl AsyncHandle<PlatformContext> for ServeCmd {
    type Output = anyhow::Result<()>;

    async fn handle(self, ctx: &mut PlatformContext) -> Self::Output {
        // update the settings with the host and port; if they were set
        if let Some(host) = self.host() {
            ctx.config_mut().network_mut().set_host(host);
        }
        if let Some(port) = self.port() {
            ctx.config_mut().network_mut().set_port(port);
        }
        // update the workdir; if it was set
        ctx.config_mut().set_workdir_option(self.workdir);
        // create a new server instance
        let server = Server::from_config(ctx.config().clone());
        // start the server
        tokio::join!(server.serve());
        Ok(())
    }
}
