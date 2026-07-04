pub fn separator() -> String {
    " │ ".into()
}

pub fn enabled_segments() -> Vec<String> {
    vec![
        "directory".into(),
        "git".into(),
        "model".into(),
        "context_window".into(),
        "thinking".into(),
        "effort".into(),
        "usage".into(),
        "tasks".into(),
    ]
}

pub fn git_show_worktree() -> bool {
    true
}

pub fn git_port_file() -> String {
    ".dev-port".into()
}

pub fn theme_name() -> String {
    "morandi".into()
}

pub fn cache_ttl() -> u64 {
    60
}
