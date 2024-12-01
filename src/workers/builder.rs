/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{config::*, context::*};

mod config;
mod context;

/// The inner type for the builder; i.e., the context of the worker wrapped in an [`Arc`](std::sync::Arc).
pub type BuilderInner = std::sync::Arc<BuilderContext>;

#[derive(Clone, Debug)]
pub struct Builder {
    inner: BuilderInner,
}

impl Builder {
    /// Create a new instance of the builder.
    pub fn new() -> Self {
        let inner = BuilderContext::default();
        let inner = std::sync::Arc::new(inner);
        Self { inner }
    }

    /// Get the inner context of the builder.
    pub fn inner(&self) -> BuilderInner {
        self.inner.clone()
    }
}
