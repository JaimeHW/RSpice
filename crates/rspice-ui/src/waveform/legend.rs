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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_traces() -> Vec<TraceData> {
        vec![
            TraceData::new("V(out)", vec![0.0], vec![0.0]),
            TraceData::new("V(in)", vec![0.0], vec![0.0]),
            TraceData::new("I(r1)", vec![0.0], vec![0.0]),
        ]
    }

    #[test]
    fn test_legend_item_from_trace() {
        let trace = TraceData::new("V(out)", vec![0.0], vec![0.0]);
        let item = LegendItem::from_trace(0, &trace);

        assert_eq!(item.index, 0);
        assert_eq!(item.name, "V(out)");
        assert!(item.visible);
        assert!(!item.highlighted);
    }

    #[test]
    fn test_legend_state_default() {
        let state = LegendState::new();
        assert!(!state.collapsed);
        assert_eq!(state.sort_by, LegendSortOrder::Index);
        assert!(state.filter.is_empty());
    }

    #[test]
    fn test_legend_state_toggle() {
        let mut state = LegendState::new();
        state.toggle_collapsed();
        assert!(state.collapsed);
        state.toggle_collapsed();
        assert!(!state.collapsed);
    }

    #[test]
    fn test_build_legend_items_no_filter() {
        let traces = make_test_traces();
        let state = LegendState::new();

        let items = build_legend_items(&traces, &state);
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_build_legend_items_with_filter() {
        let traces = make_test_traces();
        let mut state = LegendState::new();
        state.set_filter("V(");

        let items = build_legend_items(&traces, &state);
        assert_eq!(items.len(), 2); // V(out) and V(in)
    }

    #[test]
    fn test_build_legend_items_case_insensitive_filter() {
        let traces = make_test_traces();
        let mut state = LegendState::new();
        state.set_filter("v(out)");

        let items = build_legend_items(&traces, &state);
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_build_legend_items_sort_by_name() {
        let traces = make_test_traces();
        let mut state = LegendState::new();
        state.set_sort_order(LegendSortOrder::Name);

        let items = build_legend_items(&traces, &state);
        assert_eq!(items[0].name, "I(r1)"); // I comes before V
    }

    #[test]
    fn test_build_legend_items_sort_by_visibility() {
        let mut traces = make_test_traces();
        traces[1].visible = false; // Hide V(in)

        let mut state = LegendState::new();
        state.set_sort_order(LegendSortOrder::Visibility);

        let items = build_legend_items(&traces, &state);
        // Visible traces should come first
        assert!(items[0].visible);
        assert!(items[1].visible);
        assert!(!items[2].visible);
    }

    #[test]
    fn test_toggle_trace_by_name() {
        let mut traces = make_test_traces();
        assert!(traces[0].visible);

        toggle_trace_by_name(&mut traces, "V(out)");
        assert!(!traces[0].visible);

        toggle_trace_by_name(&mut traces, "V(out)");
        assert!(traces[0].visible);
    }

    #[test]
    fn test_show_all_traces() {
        let mut traces = make_test_traces();
        traces[0].visible = false;
        traces[1].visible = false;

        show_all_traces(&mut traces);

        assert!(traces.iter().all(|t| t.visible));
    }

    #[test]
    fn test_hide_all_traces() {
        let mut traces = make_test_traces();

        hide_all_traces(&mut traces);

        assert!(traces.iter().all(|t| !t.visible));
    }

    #[test]
    fn test_solo_trace() {
        let mut traces = make_test_traces();

        solo_trace(&mut traces, 1);

        assert!(!traces[0].visible);
        assert!(traces[1].visible);
        assert!(!traces[2].visible);
    }

    #[test]
    fn test_set_trace_highlight() {
        let mut traces = make_test_traces();

        set_trace_highlight(&mut traces, "V(out)", true);

        assert!(traces[0].highlighted);
        assert!(!traces[1].highlighted);
    }

    #[test]
    fn test_clear_all_highlights() {
        let mut traces = make_test_traces();
        traces[0].highlighted = true;
        traces[2].highlighted = true;

        clear_all_highlights(&mut traces);

        assert!(traces.iter().all(|t| !t.highlighted));
    }

    #[test]
    fn test_get_trace_color() {
        let color0 = get_trace_color(0);
        let color1 = get_trace_color(1);

        assert_ne!(color0, color1);
        assert_eq!(color0[3], 255); // Full alpha
    }

    #[test]
    fn test_get_trace_color_wraps() {
        let color0 = get_trace_color(0);
        let color10 = get_trace_color(10);

        assert_eq!(color0, color10); // Should wrap around
    }

    #[test]
    fn test_assign_default_colors() {
        let mut traces = make_test_traces();

        assign_default_colors(&mut traces);

        // Each trace should have a different color
        assert_ne!(traces[0].style.color, traces[1].style.color);
        assert_ne!(traces[1].style.color, traces[2].style.color);
    }
}
