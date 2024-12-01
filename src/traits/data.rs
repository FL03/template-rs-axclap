/*
    Appellation: container <module>
    Contrib: @FL03
*/

pub unsafe trait RawData {
    type Data;
}

pub trait Container: RawData {}
