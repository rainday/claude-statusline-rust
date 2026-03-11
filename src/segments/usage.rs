use crate::config::theme::ResolvedTheme;
use crate::engine::input::InputData;
use crate::engine::renderer::{render_usage_bar, rgb_fg, RESET};
use crate::engine::segment::SegmentOutput;
use crate::utils::cache;
use crate::utils::credentials::get_oauth_token;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct UsageSegment {
    pub cache_ttl_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageData {
    pub five_hour_utilization: f64,
    pub seven_day_utilization: f64,
    pub five_hour_resets_at: Option<String>,
    pub seven_day_resets_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    five_hour: Option<UsagePeriod>,
    seven_day: Option<UsagePeriod>,
}

#[derive(Debug, Deserialize)]
struct UsagePeriod {
    utilization: Option<f64>,
    resets_at: Option<String>,
}

impl UsageSegment {
    pub fn render(&self, _input: &InputData, theme: &ResolvedTheme) -> Vec<SegmentOutput> {
        let data = self.get_usage_data();
        let Some(data) = data else {
            return vec![];
        };

        let current_bar = render_usage_bar(
            data.five_hour_utilization,
            10,
            &theme.usage_current_fill,
            &theme.usage_empty,
        );
        let current_pct = rgb_fg(
            &format!("{}%", data.five_hour_utilization.round() as u32),
            &theme.usage_current_fill,
        );
        let current_reset = format_reset_time(&data.five_hour_resets_at, false, theme);
        let current_line = format!(
            "{}  {}{}  {}  {}",
            rgb_fg("current", &theme.reset_time),
            current_bar,
            RESET,
            current_pct,
            current_reset
        );

        let weekly_bar = render_usage_bar(
            data.seven_day_utilization,
            10,
            &theme.usage_weekly_fill,
            &theme.usage_empty,
        );
        let weekly_pct = rgb_fg(
            &format!("{}%", data.seven_day_utilization.round() as u32),
            &theme.usage_weekly_fill,
        );
        let weekly_reset = format_reset_time(&data.seven_day_resets_at, true, theme);
        let weekly_line = format!(
            "{}   {}{}  {}  {}",
            rgb_fg("weekly", &theme.reset_time),
            weekly_bar,
            RESET,
            weekly_pct,
            weekly_reset
        );

        vec![
            SegmentOutput { text: current_line },
            SegmentOutput { text: weekly_line },
        ]
    }

    fn get_usage_data(&self) -> Option<UsageData> {
        let cache_path = cache_path();

        // Return fresh cache if still valid
        if let Some(cached) = cache::read_cache::<UsageData>(&cache_path, self.cache_ttl_secs) {
            return Some(cached);
        }

        let token = get_oauth_token()?;

        // Try fresh fetch; on failure, fall back to stale cache
        match fetch_usage(&token) {
            Some(data) => {
                cache::write_cache(&cache_path, &data);
                Some(data)
            }
            None => cache::read_cache_stale::<UsageData>(&cache_path),
        }
    }
}

fn fetch_usage(token: &str) -> Option<UsageData> {
    let resp = ureq::get("https://api.anthropic.com/api/oauth/usage")
        .set("Authorization", &format!("Bearer {}", token))
        .set("anthropic-beta", "oauth-2025-04-20")
        .set("User-Agent", "claude-statusline-rs/0.1.0")
        .timeout(std::time::Duration::from_secs(2))
        .call()
        .ok()?;

    let api: ApiResponse = resp.into_json().ok()?;

    Some(UsageData {
        five_hour_utilization: api.five_hour.as_ref()?.utilization.unwrap_or(0.0),
        seven_day_utilization: api.seven_day.as_ref()?.utilization.unwrap_or(0.0),
        five_hour_resets_at: api.five_hour.as_ref()?.resets_at.clone(),
        seven_day_resets_at: api.seven_day.as_ref()?.resets_at.clone(),
    })
}

fn format_reset_time(
    resets_at: &Option<String>,
    include_date: bool,
    theme: &ResolvedTheme,
) -> String {
    let Some(ts) = resets_at else {
        return String::new();
    };
    let parsed = chrono::DateTime::parse_from_rfc3339(ts).ok();
    let display = match parsed {
        Some(dt) => {
            let local = dt.with_timezone(&chrono::Local);
            if include_date {
                local.format("%b %-d, %-I:%M%P").to_string()
            } else {
                local.format("%-I:%M%P").to_string()
            }
        }
        None => ts.clone(),
    };
    rgb_fg(&format!("↻ {}", display), &theme.reset_time)
}

fn cache_path() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot find home directory");
    home.join(".claude")
        .join("statusline-rs")
        .join("usage_cache.json")
}
