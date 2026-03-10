use crate::engine::input::InputData;
use crate::engine::renderer::ansi_bold;
use crate::engine::segment::SegmentOutput;
use crate::config::theme::ResolvedTheme;

pub struct ModelSegment;

impl ModelSegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let model = input.model.as_ref()?;
        let name = model.display_name.as_ref()
            .or(model.id.as_ref())?;
        let text = ansi_bold(name, theme.model_ansi);
        Some(SegmentOutput { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_display_name() {
        let input = InputData::from_json(r#"{
            "model": { "id": "claude-opus-4-6", "display_name": "Opus 4.6" }
        }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ModelSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("Opus 4.6"));
    }

    #[test]
    fn test_model_fallback_to_id() {
        let input = InputData::from_json(r#"{
            "model": { "id": "claude-opus-4-6" }
        }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = ModelSegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("claude-opus-4-6"));
    }
}
