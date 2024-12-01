/*
    Appellation: workers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{builder::Builder, serve::Server};

pub mod builder;
pub mod serve;

pub mod prelude {
    pub use super::builder::Builder;
    pub use super::serve::Server;
}

use async_trait::async_trait;

/// The trait for a worker.
#[async_trait]
pub trait AsyncWorker: Send + Sync {
    type Ctx;

    /// Start the worker.
    async fn start(&self, ctx: Self::Ctx) -> anyhow::Result<()>;
}

#[async_trait]
pub trait AsyncStart {
    type Ctx;

    /// Start the worker.
    async fn start(self) -> anyhow::Result<()>;
}
