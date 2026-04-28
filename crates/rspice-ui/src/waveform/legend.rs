//! Waveform Legend System
//!
//! Provides the trace legend for the waveform viewer with visibility toggle,
//! cross-probe highlighting, and color management.

use super::state::TraceData;

// =============================================================================
// Legend Item
// =============================================================================

/// A single legend entry
#[derive(Debug, Clone)]
pub struct LegendItem {
    /// Trace index in the traces array
    pub index: usize,
    /// Trace name for display
    pub name: String,
    /// Display color
    pub color: [u8; 4],
    /// Whether the trace is visible
    pub visible: bool,
    /// Whether the trace is highlighted (cross-probe)
    pub highlighted: bool,
}

impl LegendItem {
    /// Create a legend item from a trace
    pub fn from_trace(index: usize, trace: &TraceData) -> Self {
        Self {
            index,
            name: trace.name.clone(),
            color: trace.style.color,
            visible: trace.visible,
            highlighted: trace.highlighted,
        }
    }
}

// =============================================================================
// Legend State
// =============================================================================

/// Legend configuration and state
#[derive(Debug, Clone, Default)]
pub struct LegendState {
    /// Whether legend is collapsed
    pub collapsed: bool,
    /// Sort order: "name", "index", "visibility"
    pub sort_by: LegendSortOrder,
    /// Search/filter string
    pub filter: String,
}

/// Legend item sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegendSortOrder {
    /// Sort by trace index (order added)
    #[default]
    Index,
    /// Sort alphabetically by name
    Name,
    /// Sort with visible traces first
    Visibility,
}

impl LegendState {
    /// Create a new legend state
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Set sort order
    pub fn set_sort_order(&mut self, order: LegendSortOrder) {
        self.sort_by = order;
    }

    /// Set filter string
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        self.filter = filter.into();
    }

    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter.clear();
    }
}

// =============================================================================
// Legend Operations
// =============================================================================

/// Build legend items from traces with filtering and sorting
pub fn build_legend_items(traces: &[TraceData], state: &LegendState) -> Vec<LegendItem> {
    let mut items: Vec<LegendItem> = traces
        .iter()
        .enumerate()
        .filter(|(_, trace)| {
            if state.filter.is_empty() {
                true
            } else {
                trace
                    .name
                    .to_lowercase()
                    .contains(&state.filter.to_lowercase())
            }
        })
        .map(|(i, trace)| LegendItem::from_trace(i, trace))
        .collect();

    // Apply sorting
    match state.sort_by {
        LegendSortOrder::Index => {
            // Already in index order
        }
        LegendSortOrder::Name => {
            items.sort_by(|a, b| a.name.cmp(&b.name));
        }
        LegendSortOrder::Visibility => {
            items.sort_by(|a, b| {
                // Visible first, then by index
                b.visible.cmp(&a.visible).then(a.index.cmp(&b.index))
            });
        }
    }

    items
}

/// Toggle visibility of a trace by name
pub fn toggle_trace_by_name(traces: &mut [TraceData], name: &str) {
    for trace in traces {
        if trace.name == name {
            trace.visible = !trace.visible;
        }
    }
}

/// Show all traces
pub fn show_all_traces(traces: &mut [TraceData]) {
    for trace in traces {
        trace.visible = true;
    }
}

/// Hide all traces
pub fn hide_all_traces(traces: &mut [TraceData]) {
    for trace in traces {
        trace.visible = false;
    }
}

/// Show only the specified trace (solo mode)
pub fn solo_trace(traces: &mut [TraceData], solo_index: usize) {
    for (i, trace) in traces.iter_mut().enumerate() {
        trace.visible = i == solo_index;
    }
}

/// Set highlight state for a trace
pub fn set_trace_highlight(traces: &mut [TraceData], name: &str, highlighted: bool) {
    for trace in traces {
        if trace.name == name {
            trace.highlighted = highlighted;
        }
    }
}

/// Clear all highlights
pub fn clear_all_highlights(traces: &mut [TraceData]) {
    for trace in traces {
        trace.highlighted = false;
    }
}

// =============================================================================
// Color Palette
// =============================================================================

/// Default trace color palette (10 distinct colors)
pub const COLOR_PALETTE: [[u8; 3]; 10] = [
    [59, 130, 246],  // Blue
    [16, 185, 129],  // Green
    [249, 115, 22],  // Orange
    [139, 92, 246],  // Purple
    [236, 72, 153],  // Pink
    [234, 179, 8],   // Yellow
    [20, 184, 166],  // Teal
    [239, 68, 68],   // Red
    [168, 162, 158], // Gray
    [132, 204, 22],  // Lime
];

/// Get color for a trace index
pub fn get_trace_color(index: usize) -> [u8; 4] {
    let rgb = COLOR_PALETTE[index % COLOR_PALETTE.len()];
    [rgb[0], rgb[1], rgb[2], 255]
}

/// Assign colors to traces based on index
pub fn assign_default_colors(traces: &mut [TraceData]) {
    for (i, trace) in traces.iter_mut().enumerate() {
        trace.style.color = get_trace_color(i);
    }
}

// =============================================================================
// Tests
// =============================================================================

