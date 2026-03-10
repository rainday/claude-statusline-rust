use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputData {
    pub model: Option<Model>,
    pub context_window: Option<ContextWindow>,
    pub cwd: Option<String>,
    #[allow(dead_code)]
    pub session: Option<Session>,
    pub thinking: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContextWindow {
    pub context_window_size: Option<u64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUsage {
    pub input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Session {
    pub start_time: Option<String>,
}

impl InputData {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_input() {
        let json = r#"{
            "model": { "id": "claude-opus-4-6", "display_name": "Opus 4.6" },
            "context_window": {
                "context_window_size": 200000,
                "current_usage": {
                    "input_tokens": 50000,
                    "cache_creation_input_tokens": 10000,
                    "cache_read_input_tokens": 5000
                }
            },
            "cwd": "/home/user/project",
            "session": { "start_time": "2026-03-10T12:00:00Z" },
            "thinking": true
        }"#;
        let data = InputData::from_json(json).unwrap();
        assert_eq!(data.model.unwrap().display_name.unwrap(), "Opus 4.6");
        assert_eq!(data.context_window.unwrap().context_window_size.unwrap(), 200000);
        assert_eq!(data.thinking.unwrap(), true);
    }

    #[test]
    fn test_parse_minimal_input() {
        let json = r#"{ "cwd": "/tmp" }"#;
        let data = InputData::from_json(json).unwrap();
        assert!(data.model.is_none());
        assert_eq!(data.cwd.unwrap(), "/tmp");
        assert!(data.thinking.is_none());
    }
}
