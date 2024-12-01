/*
    Appellation: getter <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! getter {
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            get!(@base $name: $rhs);
            get_mut!(@base $name: mut $rhs);
        )*
    };
}

macro_rules! get {
    (@impl $name:ident: String) => {
        pub fn $name(&self) -> &str {
            &self.$name
        }
    };
    (@impl $name:ident: Option<String>) => {
        pub fn $name(&self) -> Option<&str> {
            self.$name.as_deref()
        }
    };
    (@impl $name:ident: Option<$T:ty>) => {
        pub fn $name(&self) -> Option<&$T> {
            self.$name.as_ref()
        }
    };
    (@impl $name:ident: Vec<$T:ty>) => {
        pub fn $name(&self) -> &[$T] {
            &self.$name
        }
    };

    (@impl $name:ident: $rhs:ty) => {
        pub fn $name(&self) -> &$rhs {
            &self.$name
        }
    };
    (@base $name:ident: $($rhs:tt)*) => {
        get!(@impl $name: $($rhs)*);

    };
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            get!(@base $name: $rhs);
        )*
    };
}

macro_rules! get_mut {
    (@impl $name:ident: mut Vec<$T:ty>) => {
        pub fn $name(&mut self) -> &mut [$T] {
            &mut self.$name
        }
    };
    (@impl $name:ident: mut $rhs:ty) => {
        paste::paste! {
            pub fn [<$name _mut>](&mut self) -> &mut $rhs {
                &mut self.$name
            }
        }
    };
    (@base $name:ident: $($rhs:tt)*) => {
        get_mut!(@impl $name: $($rhs)*);

    };
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            get_mut!(@base $name: $rhs);
        )*
    };
}
