pub mod defaults;
pub mod theme;

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub segments: SegmentsConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub usage: UsageConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralConfig {
    #[serde(default = "defaults::separator")]
    pub separator: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SegmentsConfig {
    #[serde(default = "defaults::enabled_segments")]
    pub enabled: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeConfig {
    #[serde(default = "defaults::theme_name")]
    pub name: String,
    #[serde(default)]
    pub colors: Option<theme::ThemeColors>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UsageConfig {
    #[serde(default = "defaults::cache_ttl")]
    pub cache_ttl_secs: u64,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            separator: defaults::separator(),
        }
    }
}

impl Default for SegmentsConfig {
    fn default() -> Self {
        Self {
            enabled: defaults::enabled_segments(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: defaults::theme_name(),
            colors: None,
        }
    }
}

impl Default for UsageConfig {
    fn default() -> Self {
        Self {
            cache_ttl_secs: defaults::cache_ttl(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Cannot find home directory");
        home.join(".claude")
            .join("statusline-rs")
            .join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert_eq!(cfg.general.separator, " │ ");
        assert_eq!(cfg.segments.enabled.len(), 8);
        assert_eq!(cfg.theme.name, "morandi");
        assert_eq!(cfg.usage.cache_ttl_secs, 60);
    }

    #[test]
    fn test_parse_toml_config() {
        let toml_str = r#"
[general]
separator = " | "

[segments]
enabled = ["model", "git"]

[theme]
name = "morandi"

[usage]
cache_ttl_secs = 120
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.separator, " | ");
        assert_eq!(cfg.segments.enabled, vec!["model", "git"]);
        assert_eq!(cfg.usage.cache_ttl_secs, 120);
    }
}
