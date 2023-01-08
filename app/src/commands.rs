/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Context;
use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command, };
use scsys::AsyncResult;
use std::{path::PathBuf, sync::Arc};
use tokio::task::JoinHandle;

pub async fn handle() -> JoinHandle<AsyncResult> {
    tokio::spawn(async move {

        Ok(())
    })
}

pub async fn handler(ctx: Arc<Context>, matches: ArgMatches) -> AsyncResult {
    if let Some(_up) = matches.get_one::<bool>("up") {
        let api = crate::api::from_context(ctx.as_ref().clone());
        api.start().await?;
    }
    Ok(())
}

pub fn matches() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("action")
                .about("Select an availible actor to command")
                .arg(arg!(service: -s --service <SERVICE> "Select a service").action(ArgAction::SetTrue)),
        )
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .default_value("/config/Conduit.toml"),
        )
        .arg(Arg::new("debug").long("debug").short('d').action(ArgAction::SetTrue).help("Optionally startup the debugger"))
        .arg(Arg::new("release").long("release").short('r').action(ArgAction::SetTrue).help("Optionally startup application in release mode"))
        .arg(Arg::new("up").long("up").short('u').action(ArgAction::SetTrue).help("Signals for a system to turn on"))
        .arg(Arg::new("verbose").long("verbose").short('v').action(ArgAction::SetTrue))
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(true)
        .get_matches()
}