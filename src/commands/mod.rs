use clap::ArgAction;
use crate::cli::CommandArgs;
use clap::Subcommand;
use std::fmt::Display;

mod add;
mod update;
mod validate;

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[clap(about = "Add a translation to all locale files")]
    Add { key: Option<String> },
    #[clap(about = "Update a translation in all locale files")]
    Update { key: Option<String> },
    #[clap(about = "Validate all keys are present")]
    Validate {
        #[arg(long, action=ArgAction::SetTrue)]
        fail_on_empty: Option<bool>,
    },
}

pub enum CommandError {
    Generic(String),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn handle_command(command: Commands, args: CommandArgs) -> Result<(), CommandError> {
    match command {
        Commands::Update { key } => update::update_command(args, key),
        Commands::Add { key } => add::add_command(args, key),
        Commands::Validate { fail_on_empty } => {
            validate::validate_command(args, fail_on_empty.unwrap_or(false))
        }
    }
}
