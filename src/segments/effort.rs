use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use crate::config::theme::ResolvedTheme;
use std::fs;

pub struct EffortSegment;

impl EffortSegment {
    pub fn render(&self, _input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let effort = read_effort_level().unwrap_or_else(|| "default".into());
        let (icon, ansi) = match effort.as_str() {
            "high" => ("●", theme.effort_high_ansi),
            "medium" => ("◑", theme.effort_dim_ansi),
            "low" => ("◔", theme.effort_dim_ansi),
            _ => ("◑", theme.effort_dim_ansi),
        };
        let text = ansi_bold(&format!("{} {}", icon, effort), ansi);
        Some(SegmentOutput { text })
    }
}

fn read_effort_level() -> Option<String> {
    let home = dirs::home_dir()?;
    let path = home.join(".claude").join("settings.json");
    let content = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&content).ok()?;
    val.get("effortLevel")?.as_str().map(|s| s.to_string())
}
