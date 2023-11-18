#[derive(PartialEq)]
#[derive(Debug)]
pub enum Command {
    Track,
    Projects,
    Break,
    CheckIn,
    RetroTrack,
    NoCommand,
    Invalid(String),
}

pub fn parse_command(arguments: Vec<String>) -> Command {
    if arguments.len() == 1 {
        return Command::NoCommand;
    }
    match arguments[1].as_str() {
        "track" => Command::Track,
        "projects" => Command::Projects,
        "break" => Command::Break,
        "check-in" => Command::CheckIn,
        "retrotrack" => Command::RetroTrack,
        _ => Command::Invalid(arguments[1].clone()),
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let args = vec!["".to_string()];
        assert_eq!(parse_command(args), Command::NoCommand);

        let args = vec!["".to_string(), "track".to_string()];
        assert_eq!(parse_command(args), Command::Track);

        let args = vec!["".to_string(), "projects".to_string()];
        assert_eq!(parse_command(args), Command::Projects);

        let args = vec!["".to_string(), "break".to_string()];
        assert_eq!(parse_command(args), Command::Break);

        let args = vec!["".to_string(), "check-in".to_string()];
        assert_eq!(parse_command(args), Command::CheckIn);

        let args = vec!["".to_string(), "retrotrack".to_string()];
        assert_eq!(parse_command(args), Command::RetroTrack);

        let args = vec!["".to_string(), "invalid".to_string()];
        assert_eq!(parse_command(args), Command::Invalid("invalid".to_string()));
    }
}