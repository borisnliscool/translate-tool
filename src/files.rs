use crate::config::CONFIG;
use crate::files;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub fn get_files_in_directory(
    target_dir: PathBuf,
    extension: String,
) -> Result<Vec<PathBuf>, String> {
    let mut res: Vec<PathBuf> = Vec::new();

    let files = target_dir.read_dir().map_err(|e| e.to_string())?;
    files.for_each(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() && path.extension() == Some(OsStr::new(&extension)) {
                res.push(path);
            }
        }
    });

    Ok(res)
}

pub fn get_translation_files(target_path: PathBuf) -> Result<(PathBuf, Vec<PathBuf>), String> {
    let config = CONFIG.get().unwrap();

    let translation_files = files::get_files_in_directory(target_path.clone(), "json".to_string())?;

    let default_locale_file_name = format!("{}.json", config.default_locale);
    let default_locale_path = &translation_files
        .iter()
        .find(|p| p.file_name() == Some(OsStr::new(&default_locale_file_name)));

    if default_locale_path.is_none() {
        return Err(format!(
            "Could not find default translation file. Searching for {}",
            default_locale_file_name
        ));
    }

    let default_locale_path = default_locale_path.unwrap();

    Ok((default_locale_path.to_owned(), translation_files))
}

pub fn write_translation_file(file_path: PathBuf, content: Value) -> Result<(), String> {
    fs::write(
        file_path,
        serde_json::to_string_pretty(&content).map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}
