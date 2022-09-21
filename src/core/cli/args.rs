/*
    Appellation: args <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames};

#[derive(
ArgEnum,
Clone,
Copy,
Debug,
Deserialize,
EnumString,
EnumVariantNames,
Hash,
PartialEq,
Serialize
)]
#[strum(serialize_all = "snake_case")]
pub enum PowerArgs {
    On,
    Off,
}

#[derive(
ArgEnum,
Clone,
Copy,
Debug,
Deserialize,
EnumString,
EnumVariantNames,
Hash,
PartialEq,
Serialize
)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDArgs {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(
ArgEnum,
Clone,
Copy,
Debug,
Deserialize,
EnumString,
EnumVariantNames,
Hash,
PartialEq,
Serialize
)]
#[strum(serialize_all = "snake_case")]
pub enum WebRequestArgs {
    Get,
    Post,
    Push,
}