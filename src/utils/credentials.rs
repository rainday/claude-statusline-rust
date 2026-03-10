use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    #[serde(rename = "claudeAiOauth")]
    claude_ai_oauth: Option<OAuthCreds>,
}

#[derive(Debug, Deserialize)]
struct OAuthCreds {
    #[serde(rename = "accessToken")]
    access_token: Option<String>,
}

/// Resolve OAuth token: env var → credentials file → macOS keychain
pub fn get_oauth_token() -> Option<String> {
    if let Ok(token) = std::env::var("CLAUDE_CODE_OAUTH_TOKEN") {
        if !token.is_empty() {
            return Some(token);
        }
    }

    if let Some(token) = read_credentials_file() {
        return Some(token);
    }

    #[cfg(target_os = "macos")]
    if let Some(token) = read_macos_keychain() {
        return Some(token);
    }

    None
}

fn read_credentials_file() -> Option<String> {
    let path = credentials_path()?;
    let content = fs::read_to_string(path).ok()?;
    let creds: CredentialsFile = serde_json::from_str(&content).ok()?;
    creds.claude_ai_oauth?.access_token
}

fn credentials_path() -> Option<PathBuf> {
    if let Ok(config_dir) = std::env::var("CLAUDE_CONFIG_DIR") {
        let path = PathBuf::from(config_dir).join(".credentials.json");
        if path.exists() { return Some(path); }
    }
    let home = dirs::home_dir()?;
    let path = home.join(".claude").join(".credentials.json");
    if path.exists() { Some(path) } else { None }
}

#[cfg(target_os = "macos")]
fn read_macos_keychain() -> Option<String> {
    let output = std::process::Command::new("security")
        .args(["find-generic-password", "-w", "-s", "Claude Code-credentials"])
        .output()
        .ok()?;
    if !output.status.success() { return None; }
    let json_str = String::from_utf8(output.stdout).ok()?;
    let creds: CredentialsFile = serde_json::from_str(json_str.trim()).ok()?;
    creds.claude_ai_oauth?.access_token
}
