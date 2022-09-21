/*
    Appellation: cmds <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/


#[derive(Clone, Debug, Hash, PartialEq, clap::Subcommand, serde::Deserialize, serde::Serialize)]
pub enum Commands {
    Application {
        #[clap(long, short, value_parser)]
        mode: Option<String>
    }
}
