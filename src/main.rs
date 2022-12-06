/*
    Appellation: template-cli-rs <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::settings::*;

pub(crate) mod settings;

pub mod cli;
pub mod states;

use crate::states::Appstate;
use scsys::prelude::{BoxResult, Context};
use std::sync::Arc;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let mut app = Application::default();
    println!("{:?}", &app);
    

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context<Settings>,
    pub state: Arc<Appstate>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context<Settings>, state: Arc<Appstate>) -> Self {
        Self { cnf, ctx, state }
    }
    /// Initialize the command line interface
    pub async fn cli(&mut self) -> BoxResult<Cli> {
        let cli = Cli::default();
        if cli.debug > 0 {
            println!("Debug");
        }
        if cli.command.is_some() {
            match cli.clone().command.unwrap() {
                Commands::Connect { address } => {
                    println!("{:?}", address);
                }
                Commands::System { up } => {
                    if up {
                        tracing::info!("Message Recieved: Initializing the platform...");
                    }
                }
            }
        }

        Ok(cli)
    }
    /// AIO method for running the initialized application
    pub async fn quickstart(&mut self) -> BoxResult<&Self> {
        self.with_logging();
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        self.set_state(Appstate::Process(scsys::prelude::Message::from(
            serde_json::json!({"result": "success"}),
        )))?;
        // Fetch the initialized cli and process the results
        let cli = self.cli().await?;
        tracing::debug!("{:?}", cli);

        Ok(())
    }
    /// Change the application state
    pub fn set_state(&mut self, state: Appstate) -> BoxResult<&Self> {
        self.state = Arc::new(state.clone());
        tracing::info!("Update: Application State updated to {}", state);
        Ok(self)
    }
    /// Function wrapper for returning the current application state
    pub fn state(&self) -> &Arc<Appstate> {
        &self.state
    }
    /// Initialize application logging
    pub fn with_logging(&mut self) -> &Self {
        self.cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: Initialized the logging protocols");
        self
    }
}

impl std::convert::From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::from(Context::new(data))
    }
}

impl std::convert::From<Context<Settings>> for Application {
    fn from(data: Context<Settings>) -> Self {
        Self::new(data.clone().settings, data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
