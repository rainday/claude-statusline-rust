use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use crate::config::theme::ResolvedTheme;

pub struct ThinkingSegment;

impl ThinkingSegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let thinking_on = input.thinking.unwrap_or(false);
        let ansi = if thinking_on { theme.thinking_on_ansi } else { theme.thinking_off_ansi };
        let text = ansi_bold("● thinking", ansi);
        Some(SegmentOutput { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thinking_on() {
        let input = InputData::from_json(r#"{ "thinking": true }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ThinkingSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("● thinking"));
        // bright green (ANSI 10 = code 92)
        assert!(output.text.contains("\x1b[1;92m"));
    }

    #[test]
    fn test_thinking_off() {
        let input = InputData::from_json(r#"{ "thinking": false }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ThinkingSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("● thinking"));
        // white (ANSI 7 = code 37)
        assert!(output.text.contains("\x1b[1;37m"));
    }
}
