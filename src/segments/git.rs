use crate::config::theme::ResolvedTheme;
use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use std::process::Command;

pub struct GitSegment;

impl GitSegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let cwd = input.cwd.as_ref()?;
        let branch = get_branch(cwd)?;
        let dirty = is_dirty(cwd);
        let suffix = if dirty { "*" } else { "" };
        let text = ansi_bold(&format!("\u{e725} {}{}", branch, suffix), theme.git_ansi);
        Some(SegmentOutput { text })
    }
}

fn get_branch(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(cwd)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}

fn is_dirty(cwd: &str) -> bool {
    Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(cwd)
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false)
}
