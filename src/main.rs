mod commands;

use std::{env, fs};
use commands::*;
use serde::{Deserialize, Serialize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_command = parse_command(args.clone());

    match parsed_command {
        Command::NoCommand => println!("No command"),
        Command::Track => println!("Track"),
        Command::Projects => println!("Projects"),
        Command::Break => println!("Break"),
        Command::CheckIn => {println!("CheckIn"); check_in().unwrap();},
        Command::RetroTrack => println!("RetroTrack"),
        Command::Invalid(command) => println!("Invalid command: {}", command),
    }
}

fn check_in() -> std::io::Result<()> {
    let now = chrono::offset::Local::now();

    let today = Day {
        date: now.format("%Y-%m-%d").to_string(),
        check_in_time: now.format("%H:%M").to_string(),
        chunks: vec![],
    };

    let serialized = serde_json::to_string(&today).unwrap();

    fs::write("test.json", serialized)
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
