mod commands;
mod file_handling;

use commands::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_command(args.clone()) {
        Ok(command) => match execute_command(command) {
            Ok(_) => println!("Command executed successfully"),
            Err(e) => match e {
                CommandExecutionError::CheckedInTwice => {
                    println!("You have already checked in today, no need to check in again!")
                }
                CommandExecutionError::IoError(e) => println!("IO Error: {}", e),
                CommandExecutionError::MissingImplementation(c) => {
                    println!("The command '{}' is not implemented yet", c)
                }
            },
        },
        Err(e) => match e {
            ParseCmdError::NoCommand => println!("You must provide a command: chron <command>"),
            ParseCmdError::InvalidCommand(c) => println!("The command '{}' is not valid", c),
        },
    }
}
