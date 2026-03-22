//! Cursor System for Waveform Viewer
//!
//! Provides cursor rendering and measurement functionality matching
//! commercial EDA tools like Cadence ViVA.
//!
//! # Features
//!
//! - Single cursor for point measurements
//! - Dual cursors for delta measurements (Δt, Δv)
//! - Cursor readout with interpolated values per trace
//! - Draggable cursor positioning

use super::axis;
use super::state::{CursorState, TraceData, ViewTransform};

// =============================================================================
// Cursor Readout
// =============================================================================

/// Cursor readout data for a single trace
#[derive(Debug, Clone)]
pub struct TraceReadout {
    /// Trace name
    pub name: String,
    /// Value at cursor 1 (interpolated)
    pub value1: Option<f64>,
    /// Value at cursor 2 (interpolated)
    pub value2: Option<f64>,
    /// Delta between cursor values
    pub delta: Option<f64>,
    /// Trace color for display
    pub color: [u8; 4],
}

/// Complete cursor readout for all traces
#[derive(Debug, Clone, Default)]
pub struct CursorReadout {
    /// Time at cursor 1
    pub time1: Option<f64>,
    /// Time at cursor 2
    pub time2: Option<f64>,
    /// Time delta
    pub time_delta: Option<f64>,
    /// Frequency (1/delta)
    pub frequency: Option<f64>,
    /// Per-trace readouts
    pub traces: Vec<TraceReadout>,
}

impl CursorReadout {
    /// Calculate cursor readout from current state
    pub fn calculate(cursors: &CursorState, traces: &[TraceData]) -> Self {
        let time1 = cursors.cursor1_x;
        let time2 = cursors.cursor2_x;

        let time_delta = match (time1, time2) {
            (Some(t1), Some(t2)) => Some((t2 - t1).abs()),
            _ => None,
        };

        let frequency = time_delta.map(|dt| if dt > 0.0 { 1.0 / dt } else { 0.0 });

        let trace_readouts: Vec<TraceReadout> = traces
            .iter()
            .filter(|t| t.visible)
            .map(|trace| {
                let value1 = time1.and_then(|t| trace.interpolate_at(t));
                let value2 = time2.and_then(|t| trace.interpolate_at(t));
                let delta = match (value1, value2) {
                    (Some(v1), Some(v2)) => Some(v2 - v1),
                    _ => None,
                };

                TraceReadout {
                    name: trace.name.clone(),
                    value1,
                    value2,
                    delta,
                    color: trace.style.color,
                }
            })
            .collect();

        Self {
            time1,
            time2,
            time_delta,
            frequency,
            traces: trace_readouts,
        }
    }

    /// Format time readout for display
    pub fn format_time1(&self) -> String {
        self.time1.map_or(String::from("--"), axis::format_time)
    }

    /// Format time readout for cursor 2
    pub fn format_time2(&self) -> String {
        self.time2.map_or(String::from("--"), axis::format_time)
    }

    /// Format time delta for display
    pub fn format_time_delta(&self) -> String {
        self.time_delta
            .map_or(String::from("--"), axis::format_time_delta)
    }

    /// Format frequency for display
    pub fn format_frequency(&self) -> String {
        self.frequency
            .map_or(String::from("--"), axis::format_frequency)
    }
}

// =============================================================================
// Cursor Hit Testing
// =============================================================================

/// Distance threshold for cursor drag detection (in screen pixels)
const CURSOR_HIT_DISTANCE: f32 = 8.0;

/// Check if a screen position is near cursor 1
///
/// # Arguments
/// * `screen_x` - Screen X position to test
/// * `cursor_screen_x` - Cursor 1 screen position
///
/// Returns true if within hit threshold
pub fn is_near_cursor1(screen_x: f32, cursor_screen_x: f32) -> bool {
    (screen_x - cursor_screen_x).abs() < CURSOR_HIT_DISTANCE
}

/// Check if a screen position is near cursor 2
pub fn is_near_cursor2(screen_x: f32, cursor_screen_x: f32) -> bool {
    (screen_x - cursor_screen_x).abs() < CURSOR_HIT_DISTANCE
}

/// Determine which cursor (if any) is being clicked
///
/// Returns:
/// - `Some(1)` if near cursor 1
/// - `Some(2)` if near cursor 2
/// - `None` if not near any cursor
pub fn hit_test_cursors(
    screen_x: f32,
    cursors: &CursorState,
    view: &ViewTransform,
    plot_width: f32,
) -> Option<u8> {
    if let Some(x1) = cursors.cursor1_x {
        let cursor1_screen = data_to_screen_x(x1, view, plot_width);
        if is_near_cursor1(screen_x, cursor1_screen) {
            return Some(1);
        }
    }

    if let Some(x2) = cursors.cursor2_x {
        let cursor2_screen = data_to_screen_x(x2, view, plot_width);
        if is_near_cursor2(screen_x, cursor2_screen) {
            return Some(2);
        }
    }

    None
}

/// Convert data X to screen X
fn data_to_screen_x(data_x: f64, view: &ViewTransform, plot_width: f32) -> f32 {
    let frac = (data_x - view.x_min) / view.x_range();
    frac as f32 * plot_width
}

// =============================================================================
// Cursor Dragging
// =============================================================================

/// Start dragging a cursor
pub fn start_cursor_drag(cursors: &mut CursorState, cursor_id: u8) {
    match cursor_id {
        1 => cursors.dragging_cursor1 = true,
        2 => cursors.dragging_cursor2 = true,
        _ => {}
    }
}

/// Update cursor position during drag
pub fn update_cursor_drag(
    cursors: &mut CursorState,
    screen_x: f32,
    view: &ViewTransform,
    plot_width: f32,
) {
    let data_x = screen_to_data_x(screen_x, view, plot_width);

    if cursors.dragging_cursor1 {
        cursors.cursor1_x = Some(data_x);
    } else if cursors.dragging_cursor2 {
        cursors.cursor2_x = Some(data_x);
    }
}

/// End cursor dragging
pub fn end_cursor_drag(cursors: &mut CursorState) {
    cursors.dragging_cursor1 = false;
    cursors.dragging_cursor2 = false;
}

/// Convert screen X to data X
fn screen_to_data_x(screen_x: f32, view: &ViewTransform, plot_width: f32) -> f64 {
    let frac = (screen_x / plot_width.max(1.0)).clamp(0.0, 1.0) as f64;
    view.x_min + frac * view.x_range()
}

// =============================================================================
// Readout Formatting
// =============================================================================

/// Format a trace value for readout display
pub fn format_trace_value(value: f64, unit: &str) -> String {
    axis::format_with_si_prefix(value, unit, 4)
}

/// Format trace delta for readout display
pub fn format_trace_delta(delta: f64, unit: &str) -> String {
    let sign = if delta >= 0.0 { "+" } else { "" };
    format!("{}{}", sign, axis::format_with_si_prefix(delta, unit, 4))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_trace() -> TraceData {
        TraceData::new(
            "V(out)",
            vec![0.0, 1e-6, 2e-6, 3e-6],
            vec![0.0, 1.0, 2.0, 1.5],
        )
    }

    #[test]
    fn test_cursor_readout_no_cursors() {
        let cursors = CursorState::default();
        let traces = vec![make_test_trace()];

        let readout = CursorReadout::calculate(&cursors, &traces);

        assert!(readout.time1.is_none());
        assert!(readout.time2.is_none());
        assert!(readout.time_delta.is_none());
    }

    #[test]
    fn test_cursor_readout_single_cursor() {
        let mut cursors = CursorState::default();
        cursors.place(1e-6);

        let traces = vec![make_test_trace()];
        let readout = CursorReadout::calculate(&cursors, &traces);

        assert!(readout.time1.is_some());
        assert_eq!(readout.time1.unwrap(), 1e-6);
        assert!(readout.time2.is_none());
        assert!(readout.traces[0].value1.is_some());
        assert!((readout.traces[0].value1.unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_cursor_readout_dual_cursors() {
        let mut cursors = CursorState::default();
        cursors.place(1e-6);
        cursors.place(2e-6);

        let traces = vec![make_test_trace()];
        let readout = CursorReadout::calculate(&cursors, &traces);

        assert!(readout.time1.is_some());
        assert!(readout.time2.is_some());
        assert!(readout.time_delta.is_some());
        assert!((readout.time_delta.unwrap() - 1e-6).abs() < 1e-12);
        assert!(readout.frequency.is_some());
        assert!((readout.frequency.unwrap() - 1e6).abs() < 1.0);
    }

    #[test]
    fn test_cursor_readout_trace_delta() {
        let mut cursors = CursorState::default();
        cursors.place(1e-6); // value = 1.0
        cursors.place(2e-6); // value = 2.0

        let traces = vec![make_test_trace()];
        let readout = CursorReadout::calculate(&cursors, &traces);

        assert!(readout.traces[0].delta.is_some());
        assert!((readout.traces[0].delta.unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_cursor_readout_hidden_trace_excluded() {
        let mut cursors = CursorState::default();
        cursors.place(1e-6);

        let mut trace = make_test_trace();
        trace.visible = false;
        let traces = vec![trace];

        let readout = CursorReadout::calculate(&cursors, &traces);

        assert!(readout.traces.is_empty());
    }

    #[test]
    fn test_hit_test_near_cursor1() {
        let mut cursors = CursorState::default();
        cursors.cursor1_x = Some(50.0);
        let view = ViewTransform::new(0.0, 100.0, 0.0, 100.0);

        // At cursor position
        let result = hit_test_cursors(500.0, &cursors, &view, 1000.0);
        assert_eq!(result, Some(1));

        // Far from cursor
        let result2 = hit_test_cursors(0.0, &cursors, &view, 1000.0);
        assert_eq!(result2, None);
    }

    #[test]
    fn test_hit_test_near_cursor2() {
        let mut cursors = CursorState::default();
        cursors.cursor1_x = Some(25.0);
        cursors.cursor2_x = Some(75.0);
        let view = ViewTransform::new(0.0, 100.0, 0.0, 100.0);

        let result = hit_test_cursors(750.0, &cursors, &view, 1000.0);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_cursor_drag() {
        let mut cursors = CursorState::default();
        cursors.cursor1_x = Some(50.0);
        let view = ViewTransform::new(0.0, 100.0, 0.0, 100.0);

        start_cursor_drag(&mut cursors, 1);
        assert!(cursors.dragging_cursor1);

        update_cursor_drag(&mut cursors, 750.0, &view, 1000.0);
        assert!((cursors.cursor1_x.unwrap() - 75.0).abs() < 0.1);

        end_cursor_drag(&mut cursors);
        assert!(!cursors.dragging_cursor1);
    }

    #[test]
    fn test_format_trace_value() {
        let result = format_trace_value(0.001, "V");
        assert!(result.contains("mV"));
    }

    #[test]
    fn test_format_trace_delta_positive() {
        let result = format_trace_delta(0.5, "V");
        assert!(result.starts_with('+'));
    }

    #[test]
    fn test_format_trace_delta_negative() {
        let result = format_trace_delta(-0.5, "V");
        assert!(result.starts_with('-') || result.contains("-"));
    }

    #[test]
    fn test_data_to_screen_x() {
        let view = ViewTransform::new(0.0, 100.0, 0.0, 100.0);

        assert!((data_to_screen_x(0.0, &view, 1000.0) - 0.0).abs() < 0.1);
        assert!((data_to_screen_x(50.0, &view, 1000.0) - 500.0).abs() < 0.1);
        assert!((data_to_screen_x(100.0, &view, 1000.0) - 1000.0).abs() < 0.1);
    }

    #[test]
    fn test_screen_to_data_x() {
        let view = ViewTransform::new(0.0, 100.0, 0.0, 100.0);

        assert!((screen_to_data_x(0.0, &view, 1000.0) - 0.0).abs() < 0.1);
        assert!((screen_to_data_x(500.0, &view, 1000.0) - 50.0).abs() < 0.1);
        assert!((screen_to_data_x(1000.0, &view, 1000.0) - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_cursor_readout_format_methods() {
        let mut cursors = CursorState::default();
        cursors.place(1e-6);
        cursors.place(2e-6);

        let traces = vec![make_test_trace()];
        let readout = CursorReadout::calculate(&cursors, &traces);

        let t1 = readout.format_time1();
        assert!(!t1.is_empty());
        assert!(!t1.contains("--"));

        let delta = readout.format_time_delta();
        assert!(delta.contains("Δt"));

        let freq = readout.format_frequency();
        assert!(freq.contains("Hz") || freq.contains("M") || freq.contains("k"));
    }
}
