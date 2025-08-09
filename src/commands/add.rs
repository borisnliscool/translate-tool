use crate::cli::CommandArgs;
use crate::commands::CommandError;
use crate::parser::ObjectKeyOption;
use crate::{cli, config, files, parser};
use inquire::Text;

pub fn add_command(args: CommandArgs, key: Option<String>) -> Result<(), CommandError> {
    let (default_locale_path, translation_files) =
        files::get_translation_files(args.target_path.clone())
            .map_err(|e| CommandError::Generic(e.to_string()))?;

    let translation_value = parser::get_parsed_translation_file(default_locale_path.into())
        .map_err(|e| CommandError::Generic(e.to_string()))?;

    let translation_keys = parser::get_translation_keys(
        translation_value.clone(),
        "".to_string(),
        ObjectKeyOption::OnlyObjectKeys,
    )
    .map_err(|e| CommandError::Generic(e.to_string()))?;

    // this could be improved but I can't be bothered
    let key = if key.is_some() {
        key.unwrap()
    } else {
        cli::prompt_translation_key(translation_keys.clone(), true, "Translation key to add:")
            .map_err(|e| CommandError::Generic(e.to_string()))?
    };

    if translation_keys.contains(&key) {
        return Err(CommandError::Generic(format!(
            "'{key}' has nested keys. Could not add as this key."
        )));
    }

    for file_path in &translation_files {
        let translation_value = parser::get_parsed_translation_file(file_path.into())
            .map_err(|e| CommandError::Generic(e.to_string()))?;

        let initial_value = Text::new(
            format!(
                "Value for {}:",
                file_path.file_name().unwrap().to_str().unwrap().to_string()
            )
            .as_str(),
        )
        .prompt();

        if initial_value.is_err() {
            return Err(CommandError::Generic(format!(
                "Could not update value: {:?}",
                initial_value.unwrap_err()
            )));
        }

        let initial_value = initial_value.unwrap();
        let translation_value =
            parser::update_translation_key(translation_value, key.clone(), initial_value)
                .map_err(|e| CommandError::Generic(e.to_string()))?;

        if let Err(err) = files::write_translation_file(file_path.into(), translation_value) {
            return Err(CommandError::Generic(format!(
                "Could not write translation file: {}",
                err
            )));
        }
    }

    config::run_after_write_commands();

    Ok(())
}
