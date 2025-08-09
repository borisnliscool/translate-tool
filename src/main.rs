pub mod cli;
mod commands;
pub mod config;
pub mod files;
pub mod parser;

use crate::cli::{Cli, CommandArgs};
use clap::Parser;
use std::path::Path;
use std::process;

fn main() {
    let args = Cli::parse();
    let config = config::get_config(args.config.clone().and_then(|x| Some(x.to_string())));

    let cwd = std::env::current_dir().unwrap();
    let target_path = Path::new(&cwd).join(config.translations_directory);

    if !target_path.exists() {
        panic!(
            "Target directory {:#?} does not exist. You should probably specify a target path by using '--target <path>'",
            target_path
        );
    }

    if !args.command.is_some() {
        return;
    }

    let result = commands::handle_command(
        args.clone().command.unwrap(),
        CommandArgs {
            target_path,
            cli_args: args,
        },
    );

    if result.is_err() {
        eprintln!("{}", result.unwrap_err());
        process::exit(1);
    }
}
