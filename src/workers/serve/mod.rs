/*
    Appellation: serve <module>
    Contrib: @FL03
*/
//! The `serve` module is responsible for serving the platform's HTTP server.
//!
//!
#[doc(inline)]
pub use self::{config::ServerConfig, server::Server, state::ServerState, utils::*};

mod context;
mod server;

pub mod config;
pub mod state;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::config::ServerConfig;
    pub use super::server::Server;
    pub use super::state::ServerState;
    pub use super::utils::*;
}

pub(crate) type ServerInner = std::sync::Arc<context::ServerContext>;

pub(crate) mod utils {
    /// [graceful_shutdown] is a utilitarian function determining how the server should handle
    /// a shutdown signal.
    pub async fn graceful_shutdown(_ctx: super::ServerInner) {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to gracefully shutdown the platform");
        tracing::trace!("Signal received; shutting down the platform and related services...");
    }
}
