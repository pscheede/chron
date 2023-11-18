extern crate dirs;

use std::path::PathBuf;

pub fn get_today_file_path() -> Option<PathBuf> {
    let now = chrono::offset::Local::now();
    dirs::data_dir().map(|dir| {
        dir.join(now.format("chron-timetracking/%Y/%B").to_string())
            .join(now.format("%Y-%m-%d.json").to_string())
    })
}

pub fn create_dir_if_not_exists(file_path: &PathBuf) -> std::io::Result<()> {
    let dir = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No parent directory",
    ))?;

    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }

    Ok(())
}
