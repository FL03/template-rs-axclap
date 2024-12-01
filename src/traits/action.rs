/*
    Appellation: action <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::traits::{RawContext, RawState};
use async_trait::async_trait;

/// An [Action] generally requires some context and operates upon some data to produce some
/// output.
///
/// Considering physical mechanics where every action has an equal an opposite reaction allows
/// us to explore the recusive nature of actionable outcomes.
pub trait Action<T>
where
    T: RawState,
{
    type Ctx: RawContext<State = T>;
    type Output;

    fn execute(&self, ctx: &mut Self::Ctx) -> Self::Output;
}

#[async_trait]
pub trait AsyncAction<T> {
    type Ctx: RawContext<State = T>;
    type Output;

    async fn execute(&self, ctx: &mut Self::Ctx) -> Self::Output;
}
