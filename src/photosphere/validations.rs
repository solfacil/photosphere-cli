use crate::photosphere::str_utils;
use anyhow::{bail, Result};

pub fn validate_project_name(path: &str) -> Result<String> {
    let name = get_project_name(path);
    let is_same_default = name.to_lowercase().eq(crate::SNAKE_CASE_DEFAULT);
    let has_hifen = name.contains('-');
    let is_valid_name = name.chars().all(str_utils::is_lower_alphanumeric);

    match (is_same_default, has_hifen, is_valid_name) {
        (true, _, _) => {
            bail!(
                "Hey...that's my name! Please name your project something other than {}.",
                name
            )
        }
        (_, true, _) => bail!("Please use snake_case for your project name."),
        (_, _, false) => {
            bail!("The project name can only contain lower alphanumeric characters and underscore.")
        }
        _ => Ok(path.to_string()),
    }
}

pub fn get_project_name(path: &str) -> &str {
    let path_entries = path.split('/').collect::<Vec<&str>>();

    path_entries.last().unwrap_or(crate::REPO_NAME)
}
