use crate::config;
use crate::file_handling;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Debug)]
pub enum Command {
    Track {
        project: String,
        description: Option<String>,
    },
    Projects(ProjectsSubCommand),
    Break(Option<String>),
    CheckIn,
    RetroTrack {
        end_time: NaiveTime,
        project: String,
        description: Option<String>,
    },
    Reset,
    Report(ReportSubCommand),
    Version,
}

#[derive(PartialEq, Debug)]
pub enum ProjectsSubCommand {
    Add(String),
    Delete(String),
    List,
}

#[derive(PartialEq, Debug)]
pub enum ReportSubCommand {
    Day(NaiveDate),
    Week(NaiveDate),
    Month(NaiveDate),
}

pub fn execute_command(command: Command) -> Result<(), CommandExecutionError> {
    match command {
        Command::CheckIn => check_in(),
        Command::Track {
            project,
            description,
        } => {
            let now = chrono::offset::Local::now();
            track(now.time(), project, description)
        }
        Command::Projects(subcommand) => match subcommand {
            ProjectsSubCommand::Add(project) => config::add_project(project),
            ProjectsSubCommand::Delete(project) => config::delete_project(project),
            ProjectsSubCommand::List => {
                let config = config::load_config()?;
                println!("Projects:");
                for project in config.projects {
                    println!("  - {project}");
                }
                Ok(())
            }
        },
        Command::Break(description) => {
            let now = chrono::offset::Local::now();
            track(now.time(), "break".to_string(), description)
        }
        Command::RetroTrack {
            end_time,
            project,
            description,
        } => track(end_time, project, description),
        Command::Reset => reset(),
        Command::Report(subcommand) => match subcommand {
            ReportSubCommand::Day(date) => crate::reporting::report_day(date),
            ReportSubCommand::Week(date) => {
                crate::reporting::report_week(date);
                Ok(())
            }
            ReportSubCommand::Month(date) => {
                crate::reporting::report_month(date);
                Ok(())
            }
        },
        Command::Version => {
            println!("chron version: {}", env!("GIT_VERSION"));
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseCmdError {
    NoCommand,
    InvalidCommand(String),
    MissingParameter(String),
    InvalidTimeFormat(String),
}

fn parse_project(cmd_name: &str, arguments: Option<&String>) -> Result<String, ParseCmdError> {
    arguments
        .ok_or(ParseCmdError::MissingParameter(format!(
            "The command '{cmd_name}' requires a parameter 'project'"
        )))
        .map(std::clone::Clone::clone)
}

fn parse_description(arguments: Option<&[String]>) -> Option<String> {
    arguments.filter(|a| !a.is_empty()).map(|a| a.join(" "))
}

pub fn parse_command(arguments: Vec<String>) -> Result<Command, ParseCmdError> {
    if arguments.len() == 1 {
        return Err(ParseCmdError::NoCommand);
    }
    match arguments
        .get(1)
        .expect("should always return Some(), because of the .len() check before")
        .as_str()
    {
        "track" => {
            let project = parse_project("track", arguments.get(2))?;
            let description = parse_description(arguments.get(3..));
            Ok(Command::Track {
                project,
                description,
            })
        }
        "projects" => {
            let subcommand = arguments.get(2).ok_or(ParseCmdError::MissingParameter(
                "The command 'projects' requires a subcommand: chron projects <subcommand>"
                    .to_string(),
            ))?;
            match subcommand.as_str() {
                "add" => {
                    let project = parse_project("projects add", arguments.get(3))?;
                    Ok(Command::Projects(ProjectsSubCommand::Add(project)))
                }
                "delete" => {
                    let project = parse_project("projects delete", arguments.get(3))?;
                    Ok(Command::Projects(ProjectsSubCommand::Delete(project)))
                }
                "list" => Ok(Command::Projects(ProjectsSubCommand::List)),
                _ => Err(ParseCmdError::InvalidCommand(format!(
                    "projects {subcommand}"
                ))),
            }
        }
        "break" => {
            let description = parse_description(arguments.get(2..));
            Ok(Command::Break(description))
        }
        "check-in" => Ok(Command::CheckIn),
        "retrotrack" => {
            let end_time = arguments.get(2).ok_or(ParseCmdError::MissingParameter(
                "The command 'retrotrack' requires a parameter 'end_time'".to_string(),
            ))?;

            let project = parse_project("retrotrack", arguments.get(3))?;
            let description = parse_description(arguments.get(4..));

            Ok(Command::RetroTrack {
                end_time: NaiveTime::parse_from_str(&end_time, "%H:%M")
                    .map_err(|_| ParseCmdError::InvalidTimeFormat(end_time.clone()))?,
                project,
                description,
            })
        }
        "reset" => Ok(Command::Reset),
        "report" | "rep" => {
            let subcommand = arguments.get(2).map_or("day".to_string(), String::clone);

            match subcommand.as_str() {
                "day" => Ok(Command::Report(ReportSubCommand::Day(
                    chrono::offset::Local::now().date_naive(),
                ))),
                "week" => Ok(Command::Report(ReportSubCommand::Week(
                    chrono::offset::Local::now().date_naive(),
                ))),
                "month" => Ok(Command::Report(ReportSubCommand::Month(
                    chrono::offset::Local::now().date_naive(),
                ))),
                _ => Err(ParseCmdError::InvalidCommand(format!(
                    "report {subcommand}"
                ))),
            }
        }
        "version" => Ok(Command::Version),
        _ => Err(ParseCmdError::InvalidCommand(arguments[1].clone())),
    }
}

#[derive(Debug)]
pub enum CommandExecutionError {
    CheckedInTwice,
    NotCheckedIn(NaiveDate),
    IoError(std::io::Error),
    InvalidJsonFormat(String),
    UnexpectedError(String),
    NoTrackingBeforeCheckIn,
    NoTrackingAfterCurrentTime,
    ProjectNotConfigured(String),
}

impl From<std::io::Error> for CommandExecutionError {
    fn from(error: std::io::Error) -> Self {
        CommandExecutionError::IoError(error)
    }
}

impl From<serde_json::Error> for CommandExecutionError {
    fn from(error: serde_json::Error) -> Self {
        CommandExecutionError::InvalidJsonFormat(error.to_string())
    }
}

impl From<file_handling::DirsError<'_>> for CommandExecutionError {
    fn from(error: file_handling::DirsError) -> Self {
        CommandExecutionError::UnexpectedError(format!(
            "The OS specific dir for {error} could not be found!"
        ))
    }
}

fn check_in() -> Result<(), CommandExecutionError> {
    let now = chrono::offset::Local::now();

    let today = Day {
        date: now.date_naive(),
        check_in_time: now.time(),
        chunks: vec![],
    };

    let serialized = serde_json::to_string(&today)?;

    let file_path = file_handling::get_today_file_path()?;
    file_handling::create_dir_if_not_exists(&file_path).map_err(CommandExecutionError::from)?;

    if file_path.exists() {
        return Err(CommandExecutionError::CheckedInTwice);
    }

    fs::write(file_path, serialized).map_err(CommandExecutionError::from)
}

fn track(
    time: NaiveTime,
    project: String,
    description: Option<String>,
) -> Result<(), CommandExecutionError> {
    let now = chrono::offset::Local::now();
    let date = now.date_naive();

    let mut day: Day = load_day(date)?;

    let config = config::load_config()?;
    if !config.projects.contains(&project) && project != "break" {
        return Err(CommandExecutionError::ProjectNotConfigured(project));
    }

    if time < day.check_in_time {
        return Err(CommandExecutionError::NoTrackingBeforeCheckIn);
    }
    if time > now.time() {
        return Err(CommandExecutionError::NoTrackingAfterCurrentTime);
    }

    let chunk = Chunk {
        end_time: time,
        project,
        description,
    };

    day.chunks.push(chunk);

    let serialized = serde_json::to_string(&day)?;

    let file_path = file_handling::get_file_path_for_date(day.date)?;
    fs::write(file_path, serialized).map_err(CommandExecutionError::from)
}

pub fn load_day(date: NaiveDate) -> Result<Day, CommandExecutionError> {
    let file_path = file_handling::get_file_path_for_date(date)
        .map_err(CommandExecutionError::from)
        .and_then(|path| {
            if path.exists() {
                Ok(path)
            } else {
                Err(CommandExecutionError::NotCheckedIn(date))
            }
        });

    Ok(serde_json::from_str(&fs::read_to_string(file_path?)?)?)
}

fn reset() -> Result<(), CommandExecutionError> {
    let file_path = file_handling::get_today_file_path()?;
    fs::remove_file(file_path).map_err(CommandExecutionError::from)
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    #[serde(with = "date_format")]
    pub date: NaiveDate,

    #[serde(with = "time_format")]
    pub check_in_time: NaiveTime,

    pub chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    pub project: String,

    pub description: Option<String>,

    #[serde(with = "time_format")]
    pub end_time: NaiveTime,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?)
    }
}

mod time_format {
    use chrono::NaiveTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%H:%M";

    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", time.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::manual_string_new)]
mod tests {
    use super::*;

    fn to_args(args: &[&str]) -> Vec<String> {
        args.iter().map(std::string::ToString::to_string).collect()
    }

    #[test]
    fn test_parse_invalid_and_missing_and_version() {
        let args = to_args(&[""]);
        assert_eq!(parse_command(args), Err(ParseCmdError::NoCommand));

        let args = to_args(&["", "invalid"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::InvalidCommand("invalid".to_string()))
        );

        let args = to_args(&["", "version"]);
        assert_eq!(parse_command(args), Ok(Command::Version));
    }

    #[test]
    fn test_parse_track() {
        let args = to_args(&["", "track"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'track' requires a parameter 'project'".to_string()
            ))
        );

        let args = to_args(&["", "track", "project"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Track {
                project: "project".to_string(),
                description: None,
            })
        );

        let args = to_args(&["", "track", "project", "a", "description"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Track {
                project: "project".to_string(),
                description: Some("a description".to_string()),
            })
        );

        let args = to_args(&["", "break"]);
        assert_eq!(parse_command(args), Ok(Command::Break(None)));

        let args = to_args(&["", "break", "a", "description"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Break(Some("a description".to_string())))
        );
    }

    #[test]
    fn test_parse_projects() {
        let args = to_args(&["", "projects"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'projects' requires a subcommand: chron projects <subcommand>"
                    .to_string()
            ))
        );

        let args = to_args(&["", "projects", "add"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'projects add' requires a parameter 'project'".to_string()
            ))
        );

        let args = to_args(&["", "projects", "add", "project"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Projects(ProjectsSubCommand::Add(
                "project".to_string()
            )))
        );

        let args = to_args(&["", "projects", "delete"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'projects delete' requires a parameter 'project'".to_string()
            ))
        );

        let args = to_args(&["", "projects", "delete", "project"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Projects(ProjectsSubCommand::Delete(
                "project".to_string()
            )))
        );

        let args = to_args(&["", "projects", "invalid"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::InvalidCommand(
                "projects invalid".to_string()
            ))
        );

        let args = to_args(&["", "projects", "list"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Projects(ProjectsSubCommand::List))
        );
    }

    #[test]
    fn test_parse_check_in_and_reset() {
        let args = to_args(&["", "check-in"]);
        assert_eq!(parse_command(args), Ok(Command::CheckIn));

        let args = to_args(&["", "reset"]);
        assert_eq!(parse_command(args), Ok(Command::Reset));
    }

    #[test]
    fn test_parse_retrotrack() {
        let args = to_args(&["", "retrotrack"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'retrotrack' requires a parameter 'end_time'".to_string()
            ))
        );

        let args = to_args(&["", "retrotrack", "10:00"]);
        assert_eq!(
            parse_command(args),
            Err(ParseCmdError::MissingParameter(
                "The command 'retrotrack' requires a parameter 'project'".to_string()
            ))
        );

        let args = to_args(&["", "retrotrack", "10:00", "project"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::RetroTrack {
                end_time: NaiveTime::parse_from_str("10:00", "%H:%M").unwrap(),
                project: "project".to_string(),
                description: None,
            })
        );

        let args = to_args(&["", "retrotrack", "10:00", "project", "a", "description"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::RetroTrack {
                end_time: NaiveTime::parse_from_str("10:00", "%H:%M").unwrap(),
                project: "project".to_string(),
                description: Some("a description".to_string()),
            })
        );
    }

    #[test]
    fn test_parse_report() {
        let args = to_args(&["", "report"]);
        assert_eq!(
            parse_command(args),
            Ok(Command::Report(ReportSubCommand::Day(
                chrono::offset::Local::now().date_naive()
            )))
        );
    }
}
