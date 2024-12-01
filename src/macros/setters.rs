/*
    Appellation: setters <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
macro_rules! set_with {
    ($($name:ident: $rhs:tt),* $(,)?) => {
        $(
            set!(alt($name: $rhs));
            with!(alt($name: $rhs));
        )*
    };
}

macro_rules! setter {
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            set!(@base $name: $rhs);
            with!(@base $name: $rhs);
        )*
    };
}

#[allow(unused_macros)]
macro_rules! set_some {
    (@impl $name:ident::<$rhs:ty>) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = Some($name);
            }
        }
    };
    (@impl $name:ident::<$rhs:ty>$(.$call:ident)+()) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = Some($name$(.$call)+());
            }
        }
    };
    ($($name:ident::<$T:ty>$($(.$call:ident)+())?),* $(,)?) => {
        $(
            set_some!(@impl $name::<$T>$($(.$call)+())?);
        )*
    };
}

#[allow(unused_macros)]
macro_rules! setme {
    (@impl $name:ident::<$rhs:ty>) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = $name;
            }
        }
    };
    (@impl $name:ident::<$rhs:ty>$(.$call:ident)+()) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = $name$(.$call)+();
            }
        }
    };
    ($($name:ident::<$T:ty>$($(.$call:ident)+())?),* $(,)?) => {
        $(
            setme!(@impl $name::<$T>$($(.$call)+())?);
        )*
    };
}

macro_rules! set_option {
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [<set_ $name _option>](&mut self, $name: Option<$rhs>) {
                    $name.map(|data| self.$name = data);
                }
            }
        )*
    };
}

macro_rules! set {
    (@impl $name:ident: String) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: impl ToString) {
                self.$name = $name.to_string();
            }
        }
    };
    (@impl $name:ident: Option<String>) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: impl ToString) {
                self.$name = Some($name.to_string());
            }
        }
    };
    (@impl $name:ident: Option<$rhs:ty>) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = Some($name);
            }
        }
    };
    (@impl $name:ident: $rhs:ty) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $rhs) {
                self.$name = $name;
            }
        }
    };
    (@base $name:ident: $($rhs:tt)*) => {
        set!(@impl $name: $($rhs)*);
    };
    ($($name:ident: $rhs:ty),* $(,)?) => {
        $(
            set!(@impl $name: $rhs);
        )*
    };
    (alt($($name:ident: $rhs:tt),* $(,)?)) => {
        $(
            set!(@impl $name: $rhs);
        )*
    };

}

macro_rules! with {
    (@impl $name:ident: String) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: impl ToString) -> Self {
                Self {
                    $name: $name.to_string(),
                    ..self
                }
            }
        }
    };
    (@impl $name:ident: Option<String>) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: impl ToString) -> Self {
                Self {
                    $name: Some($name.to_string()),
                    ..Self
                }
            }
        }
    };
    (@impl $name:ident: Option<$rhs:ty>) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: $rhs) -> Self {
                Self {
                    $name: Some($name),
                    ..Self
                }
            }
        }
    };
    (@impl $name:ident: $rhs:ty) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: $rhs) -> Self {
                Self {
                    $name,
                    ..self
                }
            }
        }
    };
    (@base $name:ident: $($rhs:tt)*) => {
        with!(@impl $name: $($rhs)*);

    };
    (alt($($name:ident: $rhs:ty),* $(,)?)) => {
        $(
            with!(@base $name: $rhs);
        )*
    };
}
