use crate::commands::CommandExecutionError;
use crate::file_handling;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub projects: Vec<String>,
    // aliases will be an object. keys will be "new name" and values
    // will be all projects mapped to the alias.
    // aliases: HashMap<String, Vec<String>>,
}

pub fn load_config() -> Result<Config, CommandExecutionError> {
    let config_file_path = file_handling::get_config_file_path()?;
    if !config_file_path.exists() {
        save_initial_config()?;
    }

    Ok(serde_json::from_str(&std::fs::read_to_string(
        config_file_path,
    )?)?)
}

fn save_initial_config() -> Result<(), CommandExecutionError> {
    let config = Config { projects: vec![] };
    save_config(config)
}

fn save_config(config: Config) -> Result<(), CommandExecutionError> {
    let config_file_path = file_handling::get_config_file_path()?;
    file_handling::create_dir_if_not_exists(&config_file_path)?;
    std::fs::write(config_file_path, serde_json::to_string(&config)?)
        .map_err(CommandExecutionError::from)
}

pub fn add_project(project: String) -> Result<(), CommandExecutionError> {
    let mut config = load_config()?;

    if config.projects.iter().any(|p| p == &project) {
        Ok(())
    } else {
        config.projects.push(project);
        save_config(config)
    }
}

pub fn delete_project(project: String) -> Result<(), CommandExecutionError> {
    let mut config = load_config()?;
    config.projects.retain(|p| p != &project);
    save_config(config)
}
