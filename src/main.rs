#![deny(clippy::unwrap_used)]
#![warn(clippy::pedantic)]

mod commands;
mod config;
mod file_handling;
mod logging;

use commands::{execute_command, parse_command, CommandExecutionError, ParseCmdError};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_command(args.clone()) {
        Ok(command) => match execute_command(command) {
            Ok(_) => (),
            Err(e) => match e {
                CommandExecutionError::CheckedInTwice => {
                    println!("You have already checked in today, no need to check in again!");
                }
                CommandExecutionError::IoError(e) => println!("IO Error: {e}"),
                CommandExecutionError::MissingImplementation(c) => {
                    println!("The command '{c}' is not implemented yet");
                }
                CommandExecutionError::NotCheckedIn => {
                    println!("Before tracking any time, you need to check in!");
                }
                CommandExecutionError::InvalidJsonFormat(e) => println!("Invalid JSON format: {e}"),
                CommandExecutionError::UnexpectedError(e) => println!("Unexpected error: {e}"),
                CommandExecutionError::NoTrackingBeforeCheckIn => {
                    println!("You cannot retro-track time before your check-in!");
                }
                CommandExecutionError::NoTrackingAfterCurrentTime => {
                    println!("You cannot retro-track time after the current time!");
                }
                CommandExecutionError::ProjectNotConfigured(p) => println!("You are not allowed to track time for the project '{p}' since it is not configured."),
            },
        },
        Err(e) => match e {
            ParseCmdError::NoCommand => println!("You must provide a command: chron <command>"),
            ParseCmdError::InvalidCommand(c) => println!("The command '{c}' is not valid"),
            ParseCmdError::MissingParameter(e) => println!("{e}"),
            ParseCmdError::InvalidTimeFormat(t) => {
                println!("Your time input '{t}' does not match expected format 'HH:MM'");
            }
        },
    }
}
