use super::usage::UsageSegment;
use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Default)]
pub struct CodexUsageSegment;

impl CodexUsageSegment {
    pub fn new() -> Self {
        Self
    }

    /// Find the most recent Codex session file with rate_limits data.
    fn find_latest_rate_limits() -> Option<CodexRateLimits> {
        let home = dirs::home_dir()?;
        let sessions_dir = home.join(".codex").join("sessions");
        if !sessions_dir.exists() {
            return None;
        }

        // Collect all .jsonl files recursively, sorted by mtime (newest first)
        let mut files: Vec<PathBuf> = Vec::new();
        collect_jsonl_files(&sessions_dir, &mut files);
        files.sort_by_key(|p| {
            fs::metadata(p)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH)
        });
        files.reverse();

        // Search the most recent files for rate_limits
        for path in files.iter().take(10) {
            if let Some(rl) = parse_rate_limits_from_file(path) {
                return Some(rl);
            }
        }

        None
    }
}

#[derive(Debug)]
struct CodexRateLimits {
    primary_used: f64,   // already in 0-100 range
    primary_window_minutes: Option<u64>,
    primary_resets_at: Option<i64>,
    secondary_used: f64,
    secondary_window_minutes: Option<u64>,
    secondary_resets_at: Option<i64>,
}

fn ts_to_rfc3339(ts: Option<i64>) -> Option<String> {
    ts.and_then(|t| DateTime::<Utc>::from_timestamp(t, 0))
        .map(|dt| dt.to_rfc3339())
}

fn window_label(d: Duration) -> String {
    let mins = d.num_minutes();
    if mins < 60 {
        format!("{}m", mins)
    } else if mins < 1440 {
        format!("{}h", mins / 60)
    } else {
        format!("{}d", mins / 1440)
    }
}

fn collect_jsonl_files(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_jsonl_files(&path, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                out.push(path);
            }
        }
    }
}

fn parse_rate_limits_from_file(path: &std::path::Path) -> Option<CodexRateLimits> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    // Search from the end for the last rate_limits entry with data
    for line in lines.iter().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !trimmed.contains("rate_limits") {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if let Some(rl) = extract_rate_limits(&val) {
                return Some(rl);
            }
        }
    }

    None
}

fn extract_rate_limits(val: &serde_json::Value) -> Option<CodexRateLimits> {
    // Try direct path: .payload.rate_limits or .rate_limits
    let rl = val
        .get("payload")
        .and_then(|p| p.get("rate_limits"))
        .or_else(|| val.get("rate_limits"))?;

    let primary = rl.get("primary").filter(|v| !v.is_null())?;
    let secondary = rl.get("secondary").filter(|v| !v.is_null());

    let primary_used = primary.get("used_percent")?.as_f64()?;
    let primary_window = primary.get("window_minutes").and_then(|v| v.as_u64());
    let primary_resets = primary.get("resets_at").and_then(|v| v.as_i64());

    let (secondary_used, secondary_window, secondary_resets) = match secondary {
        Some(s) => (
            s.get("used_percent").and_then(|v| v.as_f64()).unwrap_or(0.0),
            s.get("window_minutes").and_then(|v| v.as_u64()),
            s.get("resets_at").and_then(|v| v.as_i64()),
        ),
        None => (0.0, None, None),
    };

    Some(CodexRateLimits {
        primary_used,
        primary_window_minutes: primary_window,
        primary_resets_at: primary_resets,
        secondary_used,
        secondary_window_minutes: secondary_window,
        secondary_resets_at: secondary_resets,
    })
}

impl Segment for CodexUsageSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        let rl = Self::find_latest_rate_limits()?;

        let primary_duration = rl
            .primary_window_minutes
            .map(|m| Duration::minutes(m as i64))
            .unwrap_or_else(|| Duration::hours(5));
        let secondary_duration = rl
            .secondary_window_minutes
            .map(|m| Duration::minutes(m as i64))
            .unwrap_or_else(|| Duration::days(7));

        let primary_resets_str = ts_to_rfc3339(rl.primary_resets_at);
        let secondary_resets_str = ts_to_rfc3339(rl.secondary_resets_at);

        let primary_pace =
            UsageSegment::calc_budget_pace(primary_resets_str.as_deref(), primary_duration);
        let secondary_pace =
            UsageSegment::calc_budget_pace(secondary_resets_str.as_deref(), secondary_duration);
        let secondary_ttl = UsageSegment::calc_time_to_limit(
            rl.secondary_used,
            secondary_resets_str.as_deref(),
            secondary_duration,
        );

        let primary_label = window_label(primary_duration);
        let secondary_label = window_label(secondary_duration);
        let primary_pct = rl.primary_used.round() as u8;
        let secondary_pct = rl.secondary_used.round() as u8;

        let cells = super::read_bar_cells(SegmentId::CodexUsage);

        let primary_part = if cells > 0 {
            let bar = super::render_progress_bar(primary_pct, cells);
            match primary_pace {
                Some(p) => format!("{} {} {}% (pace {}%)", primary_label, bar, primary_pct, p),
                None => format!("{} {} {}%", primary_label, bar, primary_pct),
            }
        } else {
            match primary_pace {
                Some(p) => format!("{} {}%({}%)", primary_label, primary_pct, p),
                None => format!("{} {}%", primary_label, primary_pct),
            }
        };
        let mut secondary_part = if cells > 0 {
            let bar = super::render_progress_bar(secondary_pct, cells);
            match secondary_pace {
                Some(p) => format!("{} {} {}% (pace {}%)", secondary_label, bar, secondary_pct, p),
                None => format!("{} {} {}%", secondary_label, bar, secondary_pct),
            }
        } else {
            match secondary_pace {
                Some(p) => format!("{} {}%({}%)", secondary_label, secondary_pct, p),
                None => format!("{} {}%", secondary_label, secondary_pct),
            }
        };
        if let Some(ref t) = secondary_ttl {
            secondary_part.push_str(&format!(" {}", t));
        }

        let primary = format!("{} · {}", primary_part, secondary_part);

        let secondary = match rl.secondary_resets_at {
            Some(ts) => {
                use chrono::{Local, TimeZone};
                if let Some(dt) = Local.timestamp_opt(ts, 0).single() {
                    format!("@{}-{} {}", dt.format("%m"), dt.format("%-d"), dt.format("%-H"))
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        let mut metadata = HashMap::new();
        metadata.insert("codex_5h_used".to_string(), format!("{:.1}", rl.primary_used));
        metadata.insert("codex_7d_used".to_string(), format!("{:.1}", rl.secondary_used));

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::CodexUsage
    }
}
