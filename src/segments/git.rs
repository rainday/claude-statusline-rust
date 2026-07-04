use crate::config::GitConfig;
use crate::config::theme::ResolvedTheme;
use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use std::path::Path;
use std::process::Command;

pub struct GitSegment;

impl GitSegment {
    pub fn render(
        &self,
        input: &InputData,
        theme: &ResolvedTheme,
        git_cfg: &GitConfig,
    ) -> Option<SegmentOutput> {
        let cwd = input.cwd.as_ref()?;
        let branch = get_branch(cwd)?;
        let dirty = is_dirty(cwd);
        let suffix = if dirty { "*" } else { "" };

        let mut label = format!("\u{e725} {}{}", branch, suffix);

        // Worktree name (only when in a linked worktree).
        if git_cfg.show_worktree {
            if let Some(wt) = worktree_name(cwd) {
                label.push_str(&format!(" \u{2387}{}", wt));
            }
        }

        // Dev server port from convention file at repo/worktree root.
        if !git_cfg.port_file.is_empty() {
            if let Some(port) = read_port(cwd, &git_cfg.port_file) {
                label.push_str(&format!(" :{}", port));
            }
        }

        let text = ansi_bold(&label, theme.git_ansi);
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

/// Returns the worktree name when cwd is a linked worktree, else None.
/// In a linked worktree `git rev-parse --git-dir` points at
/// `.../.git/worktrees/<name>`; the main worktree does not.
fn worktree_name(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(cwd)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let git_dir = String::from_utf8(output.stdout).ok()?.trim().replace('\\', "/");
    let name = git_dir.rsplit_once("/worktrees/")?.1;
    // `name` is the trailing path component after `/worktrees/`.
    let name = name.split('/').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Reads the dev port from `<repo-root>/<port_file>` (trimmed first line).
fn read_port(cwd: &str, port_file: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(cwd)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let root = String::from_utf8(output.stdout).ok()?.trim().to_string();
    let content = std::fs::read_to_string(Path::new(&root).join(port_file)).ok()?;
    let port = content.lines().next()?.trim();
    // ponytail: accept only digits so a stray file can't inject escape codes
    if port.is_empty() || !port.chars().all(|c| c.is_ascii_digit()) {
        None
    } else {
        Some(port.to_string())
    }
}
