/*
    Appellation: context <module>
    Contrib: @FL03
*/
use super::RawData;

/// [RawContext] is a trait that defines the context required by an action to operate.
pub unsafe trait RawContext {
    type Config: RawConfig;
    type State: RawState;
}

pub trait RawConfig: RawData {}

pub trait RawState: RawData {}

/// [`Configurable`] trait for types that can be configured with another type, denoted by
/// [Configurable::Config].
pub trait Configurable {
    type Config;

    fn config(&self) -> &Self::Config;
}

pub trait Stateful<Q> {
    type State: RawState<Data = Q>;

    fn state(&self) -> &Self::State;
}

pub trait Contextual<T> {
    type Ctx: Configurable + Stateful<T>;
}
