/*
    Appellation: state <module>
    Contrib: @FL03
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ServerState {}

impl ServerState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for ServerState {}

unsafe impl Sync for ServerState {}
