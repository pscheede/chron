use crate::commands::{load_day, CommandExecutionError, Day};
use chrono::{Datelike, NaiveDate};
use std::cmp::max;
use std::collections::HashMap;

pub fn report_day(date: NaiveDate) -> Result<(), CommandExecutionError> {
    let day = load_day(date)?;

    let formatted_day = format_day(day);
    println!("{formatted_day}");

    Ok(())
}

pub fn report_week(date: NaiveDate) {
    let weekdays: Vec<NaiveDate> = [
        chrono::Weekday::Mon,
        chrono::Weekday::Tue,
        chrono::Weekday::Wed,
        chrono::Weekday::Thu,
        chrono::Weekday::Fri,
        chrono::Weekday::Sat,
        chrono::Weekday::Sun,
    ]
    .into_iter()
    .map(|wd| {
        NaiveDate::from_isoywd_opt(date.year(), date.iso_week().week(), wd)
            .expect("date should be valid")
    })
    .collect();

    let days: Vec<Day> = load_available_days(&weekdays);

    println!(
        "# Log for week: {}

{}",
        date.iso_week().week(),
        project_summary(&days)
    );
}

fn load_available_days(dates: &[NaiveDate]) -> Vec<Day> {
    dates
        .iter()
        .filter_map(|date| {
            load_day(*date).ok().map(|mut day| {
                day.chunks.sort_by(|a, b| a.end_time.cmp(&b.end_time));
                day
            })
        })
        .collect()
}

pub fn report_month(date: NaiveDate) {
    let days_of_month = get_days_of_month(date);
    let days = load_available_days(&days_of_month);

    println!(
        "# Log for month: {}

{}",
        date.format("%Y-%m"),
        project_summary(&days)
    );
}

fn get_days_of_month(date: NaiveDate) -> Vec<NaiveDate> {
    (1..=31)
        .filter_map(|day| NaiveDate::from_ymd_opt(date.year(), date.month(), day))
        .collect::<Vec<NaiveDate>>()
}

fn format_day(mut day: Day) -> String {
    day.chunks.sort_by(|a, b| a.end_time.cmp(&b.end_time));

    format!(
        "# Log for: {}

{}

{}",
        day.date.format("%Y-%m-%d"),
        project_summary(&vec![day.clone()]),
        detail_table(&day)
    )
}

/// Returns a summary of projects over the given days.
///
/// Expects the chunks for each day to be sorted by end time.
fn project_summary(days: &Vec<Day>) -> String {
    let mut project_durations: HashMap<String, chrono::Duration> = HashMap::new();

    for day in days {
        let mut previous_chunk_end_time = day.check_in_time;

        for chunk in &day.chunks {
            let project_duration = project_durations
                .entry(chunk.project.clone())
                .or_insert(chrono::Duration::zero());

            *project_duration = *project_duration + (chunk.end_time - previous_chunk_end_time);

            previous_chunk_end_time = chunk.end_time;
        }
    }

    let project_width = max(
        project_durations.keys().map(String::len).max().unwrap_or(0),
        "project".len(),
    );

    let format_duration = |duration: &chrono::Duration| {
        let fraction = duration.num_minutes() as f64 / 60.0;
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - hours * 60;
        format!("{fraction:.2}h ({hours}h {minutes}m)")
    };

    let time_width = project_durations
        .values()
        .map(|duration| format_duration(duration).len())
        .max()
        .unwrap_or(0);

    let format_summary_line =
        |p: &String, d: &String| format!("| {p:project_width$} | {d:time_width$} |");

    let total_duration = project_durations.values().sum::<chrono::Duration>();

    let break_duration = project_durations
        .remove("break")
        .unwrap_or(chrono::Duration::zero());

    let duration_without_breaks = project_durations.values().sum::<chrono::Duration>();

    let mut table = vec![
        format_summary_line(&"project".to_string(), &"time".to_string()),
        format!(
            "|{}|{}|",
            "-".repeat(project_width + 2),
            "-".repeat(time_width + 2)
        ),
    ];

    let mut sorted_projects: Vec<String> = project_durations.keys().cloned().collect();
    sorted_projects.sort();
    for project in sorted_projects {
        table.push(format_summary_line(
            &project,
            &format_duration(
                project_durations
                    .get(&project)
                    .expect("project will not be empty, because it's definitely a key in the map"),
            ),
        ));
    }

    table.push(format_summary_line(
        &"break".to_string(),
        &format_duration(&break_duration),
    ));

    let table = table.join("\n");
    format!(
        "## summary

- total amount of work: {total}
- without breaks: {without_breaks}

{table}",
        total = format_duration(&total_duration),
        without_breaks = format_duration(&duration_without_breaks)
    )
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
        format!(
            "|{}|{}|{}|",
            "-".repeat(time_width + 2),
            "-".repeat(project_width + 2),
            "-".repeat(description_width + 2)
        ),
        format_detail_line(
            &day.check_in_time.format("%H:%M").to_string(),
            &"check-in".to_string(),
            &String::new(),
        ),
    ];

    for chunk in &day.chunks {
        table.push(format_detail_line(
            &chunk.end_time.format("- %H:%M").to_string(),
            &chunk.project,
            chunk.description.as_ref().unwrap_or(&String::new()),
        ));
    }

    let table = table.join("\n");

    format!(
        "## details

{table}",
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::commands::{Chunk, Day};
    use chrono::{NaiveDate, NaiveTime};

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_days_of_month() {
        // January
        let result = get_days_of_month(NaiveDate::from_ymd_opt(2021, 1, 23).unwrap());

        assert_eq!(result.len(), 31);
        assert_eq!(result[0], NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
        assert_eq!(result[30], NaiveDate::from_ymd_opt(2021, 1, 31).unwrap());

        // February
        let result = get_days_of_month(NaiveDate::from_ymd_opt(2021, 2, 23).unwrap());

        assert_eq!(result.len(), 28);
        assert_eq!(result[0], NaiveDate::from_ymd_opt(2021, 2, 1).unwrap());
        assert_eq!(result[27], NaiveDate::from_ymd_opt(2021, 2, 28).unwrap());

        // February leap year
        let result = get_days_of_month(NaiveDate::from_ymd_opt(2020, 2, 23).unwrap());

        assert_eq!(result.len(), 29);
        assert_eq!(result[0], NaiveDate::from_ymd_opt(2020, 2, 1).unwrap());
        assert_eq!(result[28], NaiveDate::from_ymd_opt(2020, 2, 29).unwrap());

        // March
        let result = get_days_of_month(NaiveDate::from_ymd_opt(2021, 3, 23).unwrap());

        assert_eq!(result.len(), 31);
        assert_eq!(result[0], NaiveDate::from_ymd_opt(2021, 3, 1).unwrap());
        assert_eq!(result[30], NaiveDate::from_ymd_opt(2021, 3, 31).unwrap());

        // April
        let result = get_days_of_month(NaiveDate::from_ymd_opt(2021, 4, 23).unwrap());

        assert_eq!(result.len(), 30);
        assert_eq!(result[0], NaiveDate::from_ymd_opt(2021, 4, 1).unwrap());
        assert_eq!(result[29], NaiveDate::from_ymd_opt(2021, 4, 30).unwrap());
    }

    #[test]
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

- total amount of work: 8.47h (8h 28m)
- without breaks: 7.42h (7h 25m)

| project     | time           |
|-------------|----------------|
| korra       | 1.25h (1h 15m) |
| kyoshi      | 5.88h (5h 53m) |
| lake laogai | 0.28h (0h 17m) |
| break       | 1.05h (1h 3m)  |

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

        assert_eq!(expected, format_day(day));
    }
}
