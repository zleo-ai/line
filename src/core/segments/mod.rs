pub mod codex_usage;
pub mod context_window;
pub mod cost;
pub mod directory;
pub mod git;
pub mod model;
pub mod output_style;
pub mod session;
pub mod update;
pub mod usage;

use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

// New Segment trait for data collection only
pub trait Segment {
    fn collect(&self, input: &InputData) -> Option<SegmentData>;
    fn id(&self) -> SegmentId;
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub primary: String,
    pub secondary: String,
    pub metadata: HashMap<String, String>,
}

/// Render a unicode progress bar of `cells` width filled to `percent` (0-100+).
/// Uses ▰ (filled) / ▱ (empty). Caps at cells filled.
pub fn render_progress_bar(percent: u8, cells: usize) -> String {
    if cells == 0 { return String::new(); }
    let p = (percent as usize).min(100);
    let filled = (p * cells + 50) / 100;
    let filled = filled.min(cells);
    let empty = cells - filled;
    let mut bar = String::with_capacity(cells * 3);
    for _ in 0..filled { bar.push('▰'); }
    for _ in 0..empty  { bar.push('▱'); }
    bar
}

/// Read `bar_cells` from a segment's [segments.options] (default 0 = no bar).
pub fn read_bar_cells(segment_id: SegmentId) -> usize {
    crate::config::Config::load()
        .ok()
        .and_then(|cfg| {
            cfg.segments
                .iter()
                .find(|s| s.id == segment_id)
                .and_then(|sc| sc.options.get("bar_cells"))
                .and_then(|v| v.as_u64())
                .map(|n| n as usize)
        })
        .unwrap_or(0)
}

// Re-export all segment types
pub use codex_usage::CodexUsageSegment;
pub use context_window::ContextWindowSegment;
pub use cost::CostSegment;
pub use directory::DirectorySegment;
pub use git::GitSegment;
pub use model::ModelSegment;
pub use output_style::OutputStyleSegment;
pub use session::SessionSegment;
pub use update::UpdateSegment;
pub use usage::UsageSegment;
pub use usage::HourlyUsageSegment;
pub use usage::WeeklyUsageSegment;
