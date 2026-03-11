use std::io::Write;
use std::process::{Command, Stdio};

fn run_statusline(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_claude-statusline-rs"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().expect("Failed to wait");
    String::from_utf8(output.stdout).unwrap()
}

/// Strip ANSI escape sequences for easier assertion
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[test]
fn test_full_statusline_output() {
    let input = r#"{
        "model": { "id": "claude-opus-4-6", "display_name": "Opus 4.6" },
        "cwd": "/home/user/my-project",
        "context_window": {
            "context_window_size": 200000,
            "used_percentage": 32,
            "current_usage": {
                "input_tokens": 50000,
                "cache_creation_input_tokens": 10000,
                "cache_read_input_tokens": 5000
            }
        },
        "thinking": true
    }"#;

    let stdout = run_statusline(input);
    let plain = strip_ansi(&stdout);

    assert!(plain.contains("my-project"), "Missing directory: {}", plain);
    assert!(plain.contains("Opus 4.6"), "Missing model: {}", plain);
    assert!(plain.contains("32%"), "Missing context %: {}", plain);
    assert!(
        plain.contains("thinking"),
        "Missing thinking indicator: {}",
        plain
    );
}

#[test]
fn test_minimal_input() {
    let input = r#"{ "cwd": "/tmp/test-dir" }"#;

    let stdout = run_statusline(input);
    let plain = strip_ansi(&stdout);

    assert!(
        plain.contains("test-dir"),
        "Missing directory for minimal input: {}",
        plain
    );
}

#[test]
fn test_thinking_off() {
    let input = r#"{ "cwd": "/tmp", "thinking": false }"#;

    let stdout = run_statusline(input);
    let plain = strip_ansi(&stdout);

    assert!(
        plain.contains("thinking"),
        "Missing thinking indicator: {}",
        plain
    );
}
