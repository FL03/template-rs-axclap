/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_use]
pub mod fmt;
#[macro_use]
pub mod getter;
#[macro_use]
pub mod setters;

macro_rules! gsw {
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            getter!( $name: $rhs );
            setter!( $name: $rhs );
        )*
    };
}
