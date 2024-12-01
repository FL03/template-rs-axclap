/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

macro_rules! serde_display {
    (@impl json::<Debug>($($T:ty),*)) => {
        debug!(json: $($T),*);
    };
    (@impl json::<Display>($($T:ty),*)) => {
        display!(json: $($T),*);
    };
    (@impl json($($T:ty),*)) => {
        $(
            serde_display!(@impl json::<Debug>($T));
            serde_display!(@impl json::<Display>($T));
        )*
    };
    (json$(::<$fmt:ident>)?($($T:ty),* $(,)?)) => {
        serde_display!(@impl json$(::<$fmt>)?($($T),*));
    };
}

macro_rules! debug {
    (@impl $T:ty) => {
        impl ::core::fmt::Debug for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
            }
        }
    };
    (json: $($T:ty),* $(,)?) => {
        $(debug!(@impl $T);)*
    };
}

macro_rules! display {
    (@impl $T:ty) => {
        impl ::core::fmt::Display for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    };
    (json: $($T:ty),* $(,)?) => {
        $(display!(@impl $T);)*
    };
}
