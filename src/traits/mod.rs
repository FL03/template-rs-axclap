/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{action::*, context::*, data::*};

mod action;
mod context;
mod data;

use async_trait::async_trait;

/// [AsyncHandle] is a trait used to define the method used to process a particular command,
/// event, etc.
#[async_trait]
pub trait AsyncHandle<T> {
    type Output;

    async fn handle(self, ctx: &mut T) -> Self::Output;
}

/// A trait for types that can be used as application state in Axum applications.
pub trait AxumState: Clone + Send + Sync + 'static {}

/*
 ************* Implementations *************
*/
impl<T> AxumState for T where T: Clone + Send + Sync + 'static {}
