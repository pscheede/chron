use crate::commands::{load_day, CommandExecutionError, Day};
use chrono::NaiveDate;
use std::cmp::max;

pub fn log_day(date: NaiveDate) -> Result<(), CommandExecutionError> {
    let day = load_day(date)?;

    let formatted_day = format_day(day);
    println!("{formatted_day}");

    Ok(())
}

fn format_day(mut day: Day) -> String {
    day.chunks.sort_by(|a, b| a.end_time.cmp(&b.end_time));

    detail_table(&day)
}

fn detail_table(day: &Day) -> String {
    let project_width = max(
        day.chunks
            .iter()
            .map(|chunk| chunk.project.len())
            .max()
            .unwrap_or(0),
        "check-in".len(),
    );

    let description_width = max(
        day.chunks
            .iter()
            .map(|chunk| chunk.description.as_ref().map_or(0, String::len))
            .max()
            .unwrap_or(0),
        "description".len(),
    );

    let time_width: usize = 7;

    let format_detail_line = |t: &String, p: &String, d: &String| {
        format!("| {t:time_width$} | {p:project_width$} | {d:description_width$} |")
    };

    let mut table = vec![
        format_detail_line(
            &"time".to_string(),
            &"project".to_string(),
            &"description".to_string(),
        ),
        format_detail_line(
            &"-".repeat(time_width),
            &"-".repeat(project_width),
            &"-".repeat(description_width),
        ),
        format_detail_line(
            &day.check_in_time.format("%H:%M").to_string(),
            &"check-in".to_string(),
            &"".to_string(),
        ),
    ];

    for chunk in day.chunks.iter() {
        table.push(format_detail_line(
            &chunk.end_time.format("- %H:%M").to_string(),
            &chunk.project,
            &chunk.description.as_ref().unwrap_or(&"".to_string()),
        ));
    }

    table.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{Chunk, Day};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_format_day() {
        let day = Day {
            date: NaiveDate::from_ymd_opt(2023, 11, 17).unwrap(),
            check_in_time: NaiveTime::from_hms_opt(8, 6, 0).unwrap(),
            chunks: vec![
                Chunk {
                    project: "kyoshi".to_string(),
                    description: Some("answer messages from colleagues".to_string()),
                    end_time: NaiveTime::from_hms_opt(8, 45, 0).unwrap(),
                },
                Chunk {
                    project: "break".to_string(),
                    description: Some("coffee break".to_string()),
                    end_time: NaiveTime::from_hms_opt(9, 15, 0).unwrap(),
                },
                Chunk {
                    project: "kyoshi".to_string(),
                    description: Some("develop feature #123".to_string()),
                    end_time: NaiveTime::from_hms_opt(11, 23, 0).unwrap(),
                },
                Chunk {
                    project: "kyoshi".to_string(),
                    description: None,
                    end_time: NaiveTime::from_hms_opt(11, 55, 0).unwrap(),
                },
                Chunk {
                    project: "break".to_string(),
                    description: Some("lunch break".to_string()),
                    end_time: NaiveTime::from_hms_opt(12, 43, 0).unwrap(),
                },
                Chunk {
                    project: "lake laogai".to_string(),
                    description: Some("answer emails".to_string()),
                    end_time: NaiveTime::from_hms_opt(13, 00, 0).unwrap(),
                },
                Chunk {
                    project: "korra".to_string(),
                    description: Some("daily scrum".to_string()),
                    end_time: NaiveTime::from_hms_opt(9, 00, 0).unwrap(),
                },
                Chunk {
                    project: "korra".to_string(),
                    description: Some("refinement meeting".to_string()),
                    end_time: NaiveTime::from_hms_opt(14, 00, 0).unwrap(),
                },
                Chunk {
                    project: "kyoshi".to_string(),
                    description: Some("develop feature #123".to_string()),
                    end_time: NaiveTime::from_hms_opt(16, 34, 0).unwrap(),
                },
            ],
        };

        let expected = "\
# Log for: 2023-11-17

## summary

- total amount of work: 8,43h (8h 26m)
- without breaks: 7,38h (7h 23m)

| project     | time           |
|-------------|----------------|
| korra       | 1,25h (1h 15m) |
| kyoshi      | 4,88h (4h 53m) |
| lake laogai | 0,28h (17m)    |
| break       | 1,05h (1h 3m)  |

## details

| time    | project     | description                     |
|---------|-------------|---------------------------------|
| 08:06   | check-in    |                                 |
| - 08:45 | kyoshi      | answer messages from colleagues |
| - 09:00 | korra       | daily scrum                     |
| - 09:15 | break       | coffee break                    |
| - 11:23 | kyoshi      | develop feature #123            |
| - 11:55 | kyoshi      |                                 |
| - 12:43 | break       | lunch break                     |
| - 13:00 | lake laogai | answer emails                   |
| - 14:00 | korra       | refinement meeting              |
| - 16:34 | kyoshi      | develop feature #123            |";

        assert_eq!(format_day(day), expected);
    }
}
