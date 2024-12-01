/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{context::ServerContext, ServerInner};
use core::iter::once;
use hyper::header::AUTHORIZATION;
use std::sync::Arc;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;

/// [Server] is a
///
///
#[derive(Clone, Debug)]
pub struct Server {
    inner: ServerInner,
}

impl Server {
    /// A functional constructor for the [Server] struct simply taking the inner
    /// [ServerContext] and wrapping it in an [Arc].
    pub fn from_inner(inner: ServerContext) -> Self {
        Self {
            inner: inner.into_shared(),
        }
    }
    /// Create a new instance of [Server]
    pub fn from_config(config: crate::Settings) -> Self {
        Self {
            inner: ServerContext::new(config).into_shared(),
        }
    }
    /// Perform an [Arc::clone] on the inner [ServerInner] and return it.
    pub fn inner_clone(&self) -> ServerInner {
        std::sync::Arc::clone(&self.inner)
    }
    /// Consumes the current instance of [Server] and returns the inner [ServerInner].
    pub fn get(self) -> ServerInner {
        self.inner
    }
    /// Get a mutable reference to the inner [ServerInner].
    pub fn get_mut(&mut self) -> &mut ServerInner {
        &mut self.inner
    }
    /// Get an immutable reference to the inner [ServerInner].
    pub fn get_ref(&self) -> &ServerInner {
        &self.inner
    }
    /// Set the inner [ServerInner] to a new instance.
    pub fn set(&mut self, ctx: ServerContext) {
        self.inner = ServerInner::new(ctx);
    }
    /// Overwrite the inner [ServerInner] with a new instance.
    pub fn with(self, ctx: ServerContext) -> Self {
        Self {
            inner: ServerInner::new(ctx),
        }
    }
    /// Returns an immutable reference to the inner context.
    pub fn ctx(&self) -> &ServerContext {
        &self.inner
    }
    /// Returns a mutable reference to the inner context.
    pub fn ctx_mut(&mut self) -> &mut ServerContext {
        Arc::make_mut(&mut self.inner)
    }
    /// a utilitarian function used to run the application without returning anything.
    pub async fn serve(self) {
        self.start().await.expect("failed to start server");
    }
    /// spawn the server on the tokio runtime
    pub async fn spawn(self) -> tokio::task::JoinHandle<std::io::Result<()>> {
        tokio::spawn(self.start())
    }
    /// startup the server, bound to the configured address
    #[tracing::instrument(skip(self), name = "start", target = "server")]
    pub async fn start(self) -> std::io::Result<()> {
        // bind the listener
        let listener = self.bind().await?;
        tracing::info!("listening on http://{}", self.address());
        // run the server
        axum::serve(listener, self.router())
            .with_graceful_shutdown(super::graceful_shutdown(self.get()))
            .await
    }
    /// The platform router; merges the api with the neccessary middleware.
    fn router(&self) -> axum::Router {
        let basepath = self.config().basepath();
        axum::Router::new()
            // nest the serve-directory service
            .nest_service(basepath, self.create_serve_dir_svc())
            // iclude a
            .layer(axum::Extension(self.inner_clone()))
            // Mark the `Authorization` request header as sensitive so it doesn't show in logs
            .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
            // compress responses
            .layer(tower_http::compression::CompressionLayer::new())
            // tracing incoming requests
            .layer(tower_http::trace::TraceLayer::new_for_http())
    }
}

impl core::ops::Deref for Server {
    type Target = ServerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx()
    }
}

impl core::ops::DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx_mut()
    }
}

unsafe impl Send for Server {}

unsafe impl Sync for Server {}
