use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use crate::config::theme::ResolvedTheme;

pub struct ContextWindowSegment;

impl ContextWindowSegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let ctx = input.context_window.as_ref()?;
        let size = ctx.context_window_size.unwrap_or(200_000);
        let usage = ctx.current_usage.as_ref()?;
        let used = usage.input_tokens.unwrap_or(0)
            + usage.cache_creation_input_tokens.unwrap_or(0)
            + usage.cache_read_input_tokens.unwrap_or(0);
        let pct = if size > 0 { (used * 100 / size) as u8 } else { 0 };

        let text = ansi_bold(&format!("✍️ {}%", pct), theme.ctx_ansi);
        Some(SegmentOutput { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_usage() {
        let input = InputData::from_json(r#"{
            "context_window": {
                "context_window_size": 200000,
                "current_usage": { "input_tokens": 20000, "cache_creation_input_tokens": 0, "cache_read_input_tokens": 0 }
            }
        }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ContextWindowSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("10%"));
        // Uses ANSI bright magenta (95m) with bold
        assert!(output.text.contains("\x1b[1;95m"));
    }

    #[test]
    fn test_high_usage() {
        let input = InputData::from_json(r#"{
            "context_window": {
                "context_window_size": 200000,
                "current_usage": { "input_tokens": 190000, "cache_creation_input_tokens": 0, "cache_read_input_tokens": 0 }
            }
        }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ContextWindowSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("95%"));
        assert!(output.text.contains("\x1b[1;95m"));
    }
}
