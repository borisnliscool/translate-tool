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

    let config = config::get_config(
        args.config
            .clone()
            .and_then(|x| Some(x.to_string()))
            .or_else(|| Some("tt.config.json".to_string())),
        args.translations_dir
            .clone()
            .and_then(|x| Some(x.to_string())),
    );

    // If config flag is set, we use that as the parent directory,
    // else, we use the current working directory.
    let parent = args
        .config
        .clone()
        .and_then(|x| {
            if !x.exists() {
                panic!("'{}' does not exist", x)
            }
            if x.parent().is_none() {
                panic!("'{}' does not have a parent", x)
            }

            Some(
                x.as_std_path()
                    .parent()
                    .unwrap()
                    .canonicalize()
                    .unwrap()
                    .to_owned(),
            )
        })
        .unwrap_or(std::env::current_dir().unwrap());
    
    let translations_directory = Path::new(&parent).join(config.translations_directory);

    if !translations_directory.exists() {
        panic!(
            "Translations directory {:#?} does not exist. You should probably specify a \
            translations directory by using '--translations-dir <path>' (or `-t <path>` for short)",
            translations_directory
        );
    }

    let result = commands::handle_command(
        args.clone().command,
        CommandArgs {
            translations_directory,
            cli_args: args,
        },
    );

    if result.is_err() {
        eprintln!("{}", result.unwrap_err());
        process::exit(1);
    }
}
