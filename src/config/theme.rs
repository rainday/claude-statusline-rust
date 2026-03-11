use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeColors {
    pub directory: Option<String>,
    pub git: Option<String>,
    pub model: Option<String>,
    pub ctx_green: Option<String>,
    pub ctx_yellow: Option<String>,
    pub ctx_orange: Option<String>,
    pub ctx_red: Option<String>,
    pub thinking_on: Option<String>,
    pub thinking_off: Option<String>,
    pub effort_high: Option<String>,
    pub effort_dim: Option<String>,
    pub usage_current_fill: Option<String>,
    pub usage_weekly_fill: Option<String>,
    pub usage_empty: Option<String>,
    pub reset_time: Option<String>,
    pub tasks: Option<String>,
    pub separator: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedTheme {
    // Line 1: ANSI 16-color codes (matching CCometixLine cometix theme)
    pub directory_icon_ansi: u8, // bright yellow (11)
    pub directory_text_ansi: u8, // bright green (10)
    pub git_ansi: u8,            // bright blue (12)
    pub model_ansi: u8,          // bright cyan (14)
    pub ctx_ansi: u8,            // bright magenta (13)
    pub thinking_on_ansi: u8,    // bright green (10)
    pub thinking_off_ansi: u8,   // white (7)
    pub effort_dim_ansi: u8,     // white (7)
    pub separator_ansi: u8,      // white (7)
    // Lines 2-3: usage bars (hex RGB for gradient)
    pub usage_current_fill: String,
    pub usage_weekly_fill: String,
    pub usage_empty: String,
    pub reset_time: String,
    pub tasks: String,
    // Legacy hex (for usage segment and other RGB needs)
    pub directory: String,
    pub git: String,
    pub model: String,
    pub ctx_green: String,
    pub ctx_yellow: String,
    pub ctx_orange: String,
    pub ctx_red: String,
    pub thinking_on: String,
    pub thinking_off: String,
    pub effort_high: String,
    pub effort_dim: String,
    pub separator: String,
}

impl ResolvedTheme {
    pub fn morandi() -> Self {
        Self {
            // ANSI 16-color codes (CCometixLine cometix theme)
            directory_icon_ansi: 11, // bright yellow
            directory_text_ansi: 10, // bright green
            git_ansi: 12,            // bright blue
            model_ansi: 14,          // bright cyan
            ctx_ansi: 13,            // bright magenta
            thinking_on_ansi: 10,    // bright green
            thinking_off_ansi: 7,    // white
            effort_dim_ansi: 7,      // white
            separator_ansi: 7,       // white
            // Hex RGB for usage bars
            usage_current_fill: "#7BA58A".into(),
            usage_weekly_fill: "#D4B896".into(),
            usage_empty: "#4A4A4A".into(),
            reset_time: "#8B8B8B".into(),
            tasks: "#A0B9C6".into(),
            // Legacy hex
            directory: "#A0B9C6".into(),
            git: "#A0B9C6".into(),
            model: "#B5C4B1".into(),
            ctx_green: "#B5C4B1".into(),
            ctx_yellow: "#D4B896".into(),
            ctx_orange: "#C9A96E".into(),
            ctx_red: "#C17C74".into(),
            thinking_on: "#B5C4B1".into(),
            thinking_off: "#8B8B8B".into(),
            effort_high: "#C4A4B0".into(),
            effort_dim: "#8B8B8B".into(),
            separator: "#6B6B6B".into(),
        }
    }

    pub fn from_config(_name: &str, overrides: &Option<ThemeColors>) -> Self {
        let mut theme = Self::morandi();
        if let Some(o) = overrides {
            macro_rules! apply {
                ($field:ident) => {
                    if let Some(ref c) = o.$field {
                        theme.$field = c.clone();
                    }
                };
            }
            apply!(directory);
            apply!(git);
            apply!(model);
            apply!(ctx_green);
            apply!(ctx_yellow);
            apply!(ctx_orange);
            apply!(ctx_red);
            apply!(thinking_on);
            apply!(thinking_off);
            apply!(effort_high);
            apply!(effort_dim);
            apply!(usage_current_fill);
            apply!(usage_weekly_fill);
            apply!(usage_empty);
            apply!(reset_time);
            apply!(tasks);
            apply!(separator);
        }
        theme
    }
}
