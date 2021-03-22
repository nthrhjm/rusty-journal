use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    //Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Tasks> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_josn::to_writer(file, &tasks)?;

    Ok(())
}

use std::io::{Error, roorKind, Result, Seek, SeekFrom};

pub fn complete_task(journal_path: PathBuf, task_position: usize) -Result<()> {
    //Open file.
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(journal_path)?;

    let tasks = match serde_jsonj::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Rewind and truncate file.
    file.seek(SeekFrom::Start(0))?;
    file.set_lent(0);

    // Write the modified task list back into the file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
};