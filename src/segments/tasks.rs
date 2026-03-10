use crate::engine::input::InputData;
use crate::engine::renderer::rgb_fg;
use crate::engine::segment::SegmentOutput;
use crate::config::theme::ResolvedTheme;
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
    pub fn render(&self, _input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let tasks_dir = tasks_dir()?;
        if !tasks_dir.exists() { return None; }

        let (pending, in_progress, _completed) = count_tasks(&tasks_dir);
        let active = pending + in_progress;
        if active == 0 { return None; }

        let mut parts = vec![];
        if in_progress > 0 { parts.push(format!("{} in_progress", in_progress)); }
        if pending > 0 { parts.push(format!("{} pending", pending)); }

        let text = rgb_fg(
            &format!("🔄 {} tasks ({})", active, parts.join(", ")),
            &theme.tasks,
        );
        Some(SegmentOutput { text })
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

    let Ok(entries) = fs::read_dir(dir) else { return (0, 0, 0) };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }

        // Skip old task directories — only look at recently modified ones
        let dominated = path.join(".lock");
        let dir_time = if dominated.exists() {
            fs::metadata(&dominated).ok().and_then(|m| m.modified().ok())
        } else {
            fs::metadata(&path).ok().and_then(|m| m.modified().ok())
        };
        if let Some(t) = dir_time {
            if t < one_hour_ago { continue; }
        }

        let Ok(files) = fs::read_dir(&path) else { continue };
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
