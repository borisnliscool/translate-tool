use crate::commands::Commands;
use camino::Utf8PathBuf;
use clap::Parser;
use inquire::autocompletion::Replacement;
use inquire::validator::{ErrorMessage, StringValidator, Validation};
use inquire::{Autocomplete, CustomUserError, Text};
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help = true)]
pub struct Cli {
    #[clap(short, long, default_value = "tt.config.json")]
    pub config: Option<Utf8PathBuf>,

    #[clap(short, long)]
    pub translations_dir: Option<Utf8PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

pub struct CommandArgs {
    pub cli_args: Cli,
    pub target_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct UpdateAutocomplete {
    translation_keys: Vec<String>,
}

impl UpdateAutocomplete {
    fn new(translation_keys: Vec<String>) -> Self {
        UpdateAutocomplete { translation_keys }
    }
}

impl Autocomplete for UpdateAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let split = input.split(" ").collect::<Vec<&str>>();

        Ok(self
            .translation_keys
            .iter()
            .filter(|t| split.iter().all(|i| t.contains(i)))
            .cloned()
            .collect())
    }

    fn get_completion(
        &mut self,
        _: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(highlighted_suggestion)
    }
}

#[derive(Clone)]
struct TranslationKeyValidator {
    translation_keys: Vec<String>,
    inverted: bool,
}

impl TranslationKeyValidator {
    fn new(translation_keys: Vec<String>, inverted: bool) -> Self {
        Self {
            translation_keys,
            inverted,
        }
    }
}

impl StringValidator for TranslationKeyValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if self.translation_keys.contains(&input.to_string()) {
            if self.inverted {
                Ok(Validation::Invalid(ErrorMessage::Custom(format!(
                    "'{}' has sub-keys. Can not select this key.",
                    input
                ))))
            } else {
                Ok(Validation::Valid)
            }
        } else {
            if self.inverted {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(ErrorMessage::Custom(format!(
                    "'{}' is not a valid key in the main locale file",
                    input
                ))))
            }
        }
    }
}

pub fn prompt_translation_key(
    translation_keys: Vec<String>,
    invert_validator: bool,
    prompt_text: &str,
) -> Result<String, String> {
    let key = Text::new(prompt_text)
        .with_autocomplete(UpdateAutocomplete::new(translation_keys.clone()))
        .with_validator(TranslationKeyValidator::new(
            translation_keys,
            invert_validator,
        ))
        .prompt();

    if key.is_err() {
        return Err(format!("Could not select key: {:?}", key.unwrap_err()));
    }

    Ok(key.unwrap())
}
