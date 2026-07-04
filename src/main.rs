mod cli;
mod config;
mod engine;
mod segments;
mod utils;

use cli::Cli;
use config::theme::ResolvedTheme;
use config::Config;
use engine::input::InputData;
use engine::renderer::{join_segments, RESET};
use std::io::{self, Read};

fn main() {
    let _cli = Cli::parse_args();

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap_or(0);

    if input_str.is_empty() {
        eprintln!("No stdin data. Claude Code pipes JSON via stdin.");
        std::process::exit(1);
    }

    let input = match InputData::from_json(&input_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse stdin JSON: {}", e);
            std::process::exit(1);
        }
    };

    let config = Config::load();
    let theme = ResolvedTheme::from_config(&config.theme.name, &config.theme.colors);

    // Collect line-1 segments
    let mut line1_parts: Vec<String> = Vec::new();
    let enabled = &config.segments.enabled;

    if enabled.contains(&"directory".into()) {
        if let Some(out) = segments::directory::DirectorySegment.render(&input, &theme) {
            line1_parts.push(out.text);
        }
    }
    if enabled.contains(&"git".into()) {
        if let Some(out) = segments::git::GitSegment.render(&input, &theme, &config.git) {
            line1_parts.push(out.text);
        }
    }
    if enabled.contains(&"model".into()) {
        if let Some(out) = segments::model::ModelSegment.render(&input, &theme) {
            line1_parts.push(out.text);
        }
    }
    if enabled.contains(&"thinking".into()) {
        if let Some(out) = segments::thinking::ThinkingSegment.render(&input, &theme) {
            line1_parts.push(out.text);
        }
    }

    // Line 1
    let line1 = join_segments(
        &line1_parts,
        &config.general.separator,
        theme.separator_ansi,
    );
    println!("{}{}", line1, RESET);

    // Lines 2-3: Usage
    if enabled.contains(&"usage".into()) {
        let usage_seg = segments::usage::UsageSegment {
            cache_ttl_secs: config.usage.cache_ttl_secs,
        };
        for out in usage_seg.render(&input, &theme) {
            println!("{}{}", out.text, RESET);
        }
    }

    // Tasks count is now shown inline with the thinking segment on line 1
}
