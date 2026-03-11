use crate::config::theme::ResolvedTheme;
use crate::engine::input::InputData;
use crate::engine::renderer::{ansi_bold, ansi_fg};
use crate::engine::segment::SegmentOutput;

pub struct DirectorySegment;

impl DirectorySegment {
    pub fn render(&self, input: &InputData, theme: &ResolvedTheme) -> Option<SegmentOutput> {
        let cwd = input.cwd.as_ref()?;
        let folder_name = cwd
            .rsplit(&['/', '\\'][..])
            .find(|s| !s.is_empty())
            .unwrap_or(cwd);
        let icon = ansi_fg("\u{f07b}", theme.directory_icon_ansi);
        let name = ansi_bold(folder_name, theme.directory_text_ansi);
        let text = format!("{} {}", icon, name);
        Some(SegmentOutput { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::theme::ResolvedTheme;

    #[test]
    fn test_directory_unix() {
        let input = InputData::from_json(r#"{ "cwd": "/home/user/my-project" }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = DirectorySegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("my-project"));
    }

    #[test]
    fn test_directory_windows() {
        let input = InputData::from_json(r#"{ "cwd": "C:\\Users\\test\\project" }"#).unwrap();
        let theme = ResolvedTheme::morandi();
        let output = DirectorySegment.render(&input, &theme).unwrap();
        assert!(output.text.contains("project"));
    }
}
