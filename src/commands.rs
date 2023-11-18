use crate::commands::CommandExecutionError::MissingImplementation;
use crate::file_handling;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Debug)]
pub enum Command {
    Track,
    Projects,
    Break,
    CheckIn,
    RetroTrack,
    Reset,
}

pub fn execute_command(command: Command) -> Result<(), CommandExecutionError> {
    match command {
        Command::CheckIn => check_in(),
        Command::Track => Err(MissingImplementation("track".to_string())),
        Command::Projects => Err(MissingImplementation("projects".to_string())),
        Command::Break => Err(MissingImplementation("break".to_string())),
        Command::RetroTrack => Err(MissingImplementation("retrotrack".to_string())),
        Command::Reset => reset(),
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseCmdError {
    NoCommand,
    InvalidCommand(String),
}

pub fn parse_command(arguments: Vec<String>) -> Result<Command, ParseCmdError> {
    if arguments.len() == 1 {
        return Err(ParseCmdError::NoCommand);
    }
    match arguments[1].as_str() {
        "track" => Ok(Command::Track),
        "projects" => Ok(Command::Projects),
        "break" => Ok(Command::Break),
        "check-in" => Ok(Command::CheckIn),
        "retrotrack" => Ok(Command::RetroTrack),
        "reset" => Ok(Command::Reset),
        _ => Err(ParseCmdError::InvalidCommand(arguments[1].clone())),
    }
}

#[derive(Debug)]
pub enum CommandExecutionError {
    CheckedInTwice,
    IoError(std::io::Error),
    MissingImplementation(String),
}

impl From<std::io::Error> for CommandExecutionError {
    fn from(error: std::io::Error) -> Self {
        CommandExecutionError::IoError(error)
    }
}

fn check_in() -> Result<(), CommandExecutionError> {
    let now = chrono::offset::Local::now();

    let today = Day {
        date: now.format("%Y-%m-%d").to_string(),
        check_in_time: now.format("%H:%M").to_string(),
        chunks: vec![],
    };

    let serialized = serde_json::to_string(&today).unwrap();

    let file_path = file_handling::get_today_file_path().unwrap();
    file_handling::create_dir_if_not_exists(&file_path).map_err(CommandExecutionError::from)?;

    if file_path.exists() {
        return Err(CommandExecutionError::CheckedInTwice);
    }

    fs::write(file_path, serialized).map_err(CommandExecutionError::from)
}

fn reset() -> Result<(), CommandExecutionError> {
    let file_path = file_handling::get_today_file_path().unwrap();
    fs::remove_file(file_path).map_err(CommandExecutionError::from)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Day {
    date: String,
    check_in_time: String,
    chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Chunk {
    project: String,
    description: Option<String>,
    end_time: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let args = vec!["".to_string()];
        assert_eq!(parse_command(args), Err(ParseCmdError::NoCommand));

        let args = vec!["".to_string(), "track".to_string()];
        assert_eq!(parse_command(args), Ok(Command::Track));

        let args = vec!["".to_string(), "projects".to_string()];
        assert_eq!(parse_command(args), Ok(Command::Projects));

        let args = vec!["".to_string(), "break".to_string()];
        assert_eq!(parse_command(args), Ok(Command::Break));

        let args = vec!["".to_string(), "check-in".to_string()];
        assert_eq!(parse_command(args), Ok(Command::CheckIn));

        let args = vec!["".to_string(), "retrotrack".to_string()];
        assert_eq!(parse_command(args), Ok(Command::RetroTrack));

        let args = vec!["".to_string(), "invalid".to_string()];
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::InvalidCommand("invalid".to_string()))
        );
    }
}
