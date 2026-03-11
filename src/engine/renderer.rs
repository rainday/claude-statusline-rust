/// Parse "#RRGGBB" hex string to (r, g, b)
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Wrap text with RGB foreground ANSI escape
pub fn rgb_fg(text: &str, hex: &str) -> String {
    if let Some((r, g, b)) = hex_to_rgb(hex) {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
    } else {
        text.to_string()
    }
}

/// Wrap text with 16-color ANSI foreground + bold (like CCometixLine)
pub fn ansi_bold(text: &str, c16: u8) -> String {
    let code = if c16 < 8 {
        30 + c16 as u32
    } else {
        90 + (c16 - 8) as u32
    };
    format!("\x1b[1;{}m{}\x1b[0m", code, text)
}

/// Wrap text with 16-color ANSI foreground (no bold)
pub fn ansi_fg(text: &str, c16: u8) -> String {
    let code = if c16 < 8 {
        30 + c16 as u32
    } else {
        90 + (c16 - 8) as u32
    };
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

/// Return just the ANSI escape prefix for RGB foreground (no reset)
pub fn rgb_fg_code(hex: &str) -> String {
    if let Some((r, g, b)) = hex_to_rgb(hex) {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    } else {
        String::new()
    }
}

pub const RESET: &str = "\x1b[0m";

/// Join line-1 segments with separator (using ANSI 16-color)
pub fn join_segments(segments: &[String], separator: &str, sep_ansi: u8) -> String {
    let colored_sep = ansi_fg(separator, sep_ansi);
    segments
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(&colored_sep)
}

/// Render a usage bar: ●●●●○○○○○○
pub fn render_usage_bar(
    percent: f64,
    total_dots: usize,
    fill_color: &str,
    empty_color: &str,
) -> String {
    let filled = ((percent / 100.0) * total_dots as f64).round() as usize;
    let filled = filled.min(total_dots);
    let empty = total_dots - filled;
    format!(
        "{}{}{}{}",
        rgb_fg_code(fill_color),
        "●".repeat(filled),
        rgb_fg_code(empty_color),
        "○".repeat(empty),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#A0B9C6"), Some((160, 185, 198)));
        assert_eq!(hex_to_rgb("#000000"), Some((0, 0, 0)));
        assert_eq!(hex_to_rgb("#FFFFFF"), Some((255, 255, 255)));
        assert_eq!(hex_to_rgb("invalid"), None);
    }

    #[test]
    fn test_rgb_fg() {
        let result = rgb_fg("hello", "#FF0000");
        assert!(result.contains("\x1b[38;2;255;0;0m"));
        assert!(result.contains("hello"));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_render_usage_bar() {
        let bar = render_usage_bar(30.0, 10, "#7BA58A", "#4A4A4A");
        assert!(bar.contains("●●●"));
        assert!(bar.contains("○○○○○○○"));
    }
}
