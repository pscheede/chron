extern crate dirs;

use std::path::{Path, PathBuf};

const DIR_NAME: &str = "chron-timetracking";
const DATA_DIR_FORMAT: &str = "chron-timetracking/%Y/%B";

#[derive(Debug, Clone)]
pub struct DirsError<'a>(&'a str);

impl std::fmt::Display for DirsError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn get_today_file_path<'a>() -> Result<PathBuf, DirsError<'a>> {
    let now = chrono::offset::Local::now().naive_local().date();
    get_file_path_for_date(now)
}

pub fn get_file_path_for_date<'a>(date: chrono::NaiveDate) -> Result<PathBuf, DirsError<'a>> {
    dirs::data_dir().ok_or(DirsError("data dir")).map(|dir| {
        dir.join(date.format(DATA_DIR_FORMAT).to_string())
            .join(date.format("%Y-%m-%d.json").to_string())
    })
}

pub fn get_config_file_path<'a>() -> Result<PathBuf, DirsError<'a>> {
    dirs::config_dir()
        .ok_or(DirsError("config dir"))
        .map(|dir| dir.join(DIR_NAME).join("config.json"))
}

pub fn create_dir_if_not_exists(file_path: &Path) -> std::io::Result<()> {
    let dir = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No parent directory",
    ))?;

    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }

    Ok(())
}
