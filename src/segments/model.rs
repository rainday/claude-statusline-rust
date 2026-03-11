use crate::config::theme::ResolvedTheme;
use crate::engine::input::InputData;
use crate::engine::renderer::{ansi_bold, ansi_fg};
use crate::engine::segment::SegmentOutput;

pub struct ModelSegment;

impl ModelSegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let model = input.model.as_ref()?;
        let name = model.display_name.as_ref().or(model.id.as_ref())?;

        let mut parts = vec![ansi_bold(name, theme.model_ansi)];

        // Effort (output_style)
        if let Some(style) = input.output_style.as_ref() {
            if let Some(effort) = style.name.as_ref() {
                parts.push(ansi_fg(&format!("({})", effort), theme.effort_dim_ansi));
            }
        }

        // Context window usage %
        if let Some(ctx) = input.context_window.as_ref() {
            if let Some(pct) = ctx.used_percentage {
                parts.push(ansi_bold(&format!("{}%", pct), theme.ctx_ansi));
            }
        }

        let text = parts.join(" ");
        Some(SegmentOutput { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_display_name() {
        let input = InputData::from_json(
            r#"{
            "model": { "id": "claude-opus-4-6", "display_name": "Opus 4.6" }
        }"#,
        )
        .unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ModelSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("Opus 4.6"));
    }

    #[test]
    fn test_model_with_effort_and_usage() {
        let input = InputData::from_json(
            r#"{
            "model": { "id": "claude-opus-4-6", "display_name": "Opus 4.6" },
            "output_style": { "name": "max" },
            "context_window": { "used_percentage": 64 }
        }"#,
        )
        .unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ModelSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("Opus 4.6"));
        assert!(output.text.contains("max"));
        assert!(output.text.contains("64%"));
    }
}
