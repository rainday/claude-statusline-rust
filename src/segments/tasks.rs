use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct TasksSegment;

#[derive(Debug, Deserialize)]
struct TaskItem {
    status: Option<String>,
}

impl TasksSegment {
    /// Returns the number of active (pending + in_progress) tasks, if any.
    pub fn active_count(&self) -> Option<usize> {
        let tasks_dir = tasks_dir()?;
        if !tasks_dir.exists() {
            return Some(0);
        }
        let (pending, in_progress, _) = count_tasks(&tasks_dir);
        Some(pending + in_progress)
    }
}

fn tasks_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(home.join(".claude").join("tasks"))
}

/// Only count tasks from directories modified in the last hour
fn count_tasks(dir: &PathBuf) -> (usize, usize, usize) {
    let mut pending = 0;
    let mut in_progress = 0;
    let mut completed = 0;
    let one_hour_ago = SystemTime::now() - Duration::from_secs(3600);

    let Ok(entries) = fs::read_dir(dir) else {
        return (0, 0, 0);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // Skip old task directories — only look at recently modified ones
        let dominated = path.join(".lock");
        let dir_time = if dominated.exists() {
            fs::metadata(&dominated)
                .ok()
                .and_then(|m| m.modified().ok())
        } else {
            fs::metadata(&path).ok().and_then(|m| m.modified().ok())
        };
        if let Some(t) = dir_time {
            if t < one_hour_ago {
                continue;
            }
        }

        let Ok(files) = fs::read_dir(&path) else {
            continue;
        };
        for file in files.flatten() {
            let fpath = file.path();
            if fpath.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&fpath) {
                    if let Ok(task) = serde_json::from_str::<TaskItem>(&content) {
                        match task.status.as_deref() {
                            Some("pending") => pending += 1,
                            Some("in_progress") => in_progress += 1,
                            Some("completed") => completed += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    (pending, in_progress, completed)
}
