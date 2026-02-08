//! Cross-Probe Rendering
//!
//! UI rendering for cross-probe signal browser and highlighting.

use egui::{Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

use super::event::ProbeSource;
use super::signal::{ProbeableSignal, SignalPath, SignalType};
use super::state::{CrossProbeState, SelectionMode, SignalFilter};

// =============================================================================
// Colors
// =============================================================================

fn selected_bg_color() -> Color32 {
    Color32::from_rgba_unmultiplied(100, 150, 255, 40)
}

fn highlighted_bg_color() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 200, 100, 30)
}

fn signal_voltage_color() -> Color32 {
    Color32::from_rgb(100, 200, 100)
}

fn signal_current_color() -> Color32 {
    Color32::from_rgb(255, 150, 100)
}

fn signal_digital_color() -> Color32 {
    Color32::from_rgb(150, 150, 255)
}

fn signal_other_color() -> Color32 {
    Color32::from_rgb(180, 180, 180)
}

fn text_color() -> Color32 {
    Color32::from_rgb(200, 205, 215)
}

fn dim_text_color() -> Color32 {
    Color32::from_rgb(120, 125, 135)
}

// =============================================================================
// Signal Browser Panel
// =============================================================================

/// Render the signal browser panel
pub fn render_signal_browser(ui: &mut Ui, state: &mut CrossProbeState) {
    let _available = ui.available_rect_before_wrap();

    // Header
    render_browser_header(ui, state);

    // Filter bar
    render_filter_bar(ui, state);

    // Signal list
    if state.browser_expanded {
        render_signal_list(ui, state);
    }
}

fn render_browser_header(ui: &mut Ui, state: &mut CrossProbeState) {
    ui.horizontal(|ui| {
        // Collapse/expand button
        let icon = if state.browser_expanded { "▼" } else { "▶" };
        if ui.small_button(icon).clicked() {
            state.toggle_browser();
        }

        ui.label(
            egui::RichText::new("Signals")
                .size(12.0)
                .strong()
                .color(text_color()),
        );

        ui.add_space(8.0);

        // Signal count
        let total = state.registry.signal_count();
        let selected = state.registry.selection_count();
        ui.label(
            egui::RichText::new(format!("{}/{} selected", selected, total))
                .size(10.0)
                .color(dim_text_color()),
        );
    });
}

fn render_filter_bar(ui: &mut Ui, state: &mut CrossProbeState) {
    ui.horizontal(|ui| {
        ui.add_space(16.0);

        // Type filter buttons
        let volt_color = if state.filter.show_voltages {
            signal_voltage_color()
        } else {
            dim_text_color()
        };
        if ui
            .add(egui::Label::new(egui::RichText::new("V").color(volt_color)).sense(Sense::click()))
            .clicked()
        {
            state.filter.toggle_voltages();
        }

        let curr_color = if state.filter.show_currents {
            signal_current_color()
        } else {
            dim_text_color()
        };
        if ui
            .add(egui::Label::new(egui::RichText::new("I").color(curr_color)).sense(Sense::click()))
            .clicked()
        {
            state.filter.toggle_currents();
        }

        ui.separator();

        // Selection mode
        egui::ComboBox::from_id_salt("sel_mode")
            .selected_text(state.selection_mode.display_name())
            .width(60.0)
            .show_ui(ui, |ui| {
                for mode in SelectionMode::all() {
                    ui.selectable_value(&mut state.selection_mode, *mode, mode.display_name());
                }
            });
    });
}

fn render_signal_list(ui: &mut Ui, state: &mut CrossProbeState) {
    egui::ScrollArea::vertical()
        .max_height(300.0)
        .show(ui, |ui| {
            let signals = state.filtered_signals();
            for signal in signals {
                render_signal_row(ui, signal, state);
            }
        });
}

fn render_signal_row(ui: &mut Ui, signal: &ProbeableSignal, state: &CrossProbeState) {
    let row_height = 20.0;

    ui.horizontal(|ui| {
        // Selection indicator
        let is_selected = signal.is_selected;
        let is_highlighted = signal.is_highlighted;

        // Background color
        if is_selected {
            let rect = ui.available_rect_before_wrap();
            let row_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), row_height));
            ui.painter()
                .rect_filled(row_rect, Rounding::ZERO, selected_bg_color());
        } else if is_highlighted {
            let rect = ui.available_rect_before_wrap();
            let row_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), row_height));
            ui.painter()
                .rect_filled(row_rect, Rounding::ZERO, highlighted_bg_color());
        }

        // Type indicator
        let type_color = match signal.signal_type {
            SignalType::Voltage => signal_voltage_color(),
            SignalType::Current => signal_current_color(),
            SignalType::Digital => signal_digital_color(),
            _ => signal_other_color(),
        };

        if state.show_signal_types {
            ui.label(
                egui::RichText::new(signal.signal_type.unit())
                    .size(9.0)
                    .color(type_color),
            );
        }

        // Signal name
        let display_name = if state.show_full_paths {
            signal.qualified_name()
        } else {
            signal.display_name.clone()
        };

        let text_col = if is_selected {
            Color32::WHITE
        } else {
            text_color()
        };

        ui.label(egui::RichText::new(display_name).size(11.0).color(text_col));
    });

    ui.add_space(2.0);
}

// =============================================================================
// Schematic Node Highlighting
// =============================================================================

/// Highlight data for a schematic node
#[derive(Debug, Clone)]
pub struct NodeHighlight {
    /// Node name
    pub name: String,
    /// Highlight color
    pub color: Color32,
    /// Center position in schematic coords
    pub position: (f32, f32),
    /// Highlight radius
    pub radius: f32,
    /// Is pulsing animation active
    pub pulsing: bool,
}

impl NodeHighlight {
    /// Create new highlight
    pub fn new(name: &str, x: f32, y: f32, color: Color32) -> Self {
        Self {
            name: name.to_string(),
            color,
            position: (x, y),
            radius: 8.0,
            pulsing: false,
        }
    }

    /// Set pulsing animation
    pub fn with_pulsing(mut self) -> Self {
        self.pulsing = true;
        self
    }

    /// Set radius
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

/// Render a node highlight on schematic
pub fn render_node_highlight(painter: &egui::Painter, highlight: &NodeHighlight, time: f64) {
    let (x, y) = highlight.position;
    let center = Pos2::new(x, y);

    // Animated radius if pulsing
    let radius = if highlight.pulsing {
        let pulse = (time * 3.0).sin() * 0.3 + 1.0;
        highlight.radius * pulse as f32
    } else {
        highlight.radius
    };

    // Outer glow
    let glow_color = Color32::from_rgba_unmultiplied(
        highlight.color.r(),
        highlight.color.g(),
        highlight.color.b(),
        50,
    );
    painter.circle_filled(center, radius * 2.0, glow_color);

    // Main circle
    painter.circle_stroke(center, radius, Stroke::new(2.0, highlight.color));

    // Center dot
    painter.circle_filled(center, 3.0, highlight.color);
}

// =============================================================================
// Waveform Trace Highlighting
// =============================================================================

/// Highlight data for a waveform trace
#[derive(Debug, Clone)]
pub struct TraceHighlight {
    /// Signal ID
    pub signal_name: String,
    /// Highlight color
    pub color: Color32,
    /// Trace index in waveform viewer
    pub trace_index: usize,
    /// Draw thickened line
    pub thickened: bool,
}

impl TraceHighlight {
    /// Create new highlight
    pub fn new(signal_name: &str, trace_index: usize, color: Color32) -> Self {
        Self {
            signal_name: signal_name.to_string(),
            color,
            trace_index,
            thickened: true,
        }
    }
}

// =============================================================================
// Cross-Probe Cursor
// =============================================================================

/// Render sync cursor line across all views
pub fn render_sync_cursor(painter: &egui::Painter, rect: Rect, time: f64, time_range: (f64, f64)) {
    let (t_min, t_max) = time_range;
    if t_max <= t_min || time < t_min || time > t_max {
        return;
    }

    let x = rect.min.x + ((time - t_min) / (t_max - t_min)) as f32 * rect.width();

    // Cursor line
    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
        Stroke::new(1.0, Color32::from_rgb(255, 200, 100)),
    );

    // Time label
    painter.text(
        Pos2::new(x + 3.0, rect.min.y + 2.0),
        egui::Align2::LEFT_TOP,
        format_time(time),
        FontId::proportional(9.0),
        Color32::from_rgb(255, 200, 100),
    );
}

fn format_time(time: f64) -> String {
    let abs_time = time.abs();
    if abs_time >= 1.0 {
        format!("{:.3}s", time)
    } else if abs_time >= 1e-3 {
        format!("{:.3}ms", time * 1e3)
    } else if abs_time >= 1e-6 {
        format!("{:.3}µs", time * 1e6)
    } else if abs_time >= 1e-9 {
        format!("{:.3}ns", time * 1e9)
    } else {
        format!("{:.3}ps", time * 1e12)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_highlight_new() {
        let h = NodeHighlight::new("out", 100.0, 200.0, Color32::RED);
        assert_eq!(h.name, "out");
        assert_eq!(h.position, (100.0, 200.0));
        assert!(!h.pulsing);
    }

    #[test]
    fn test_node_highlight_with_pulsing() {
        let h = NodeHighlight::new("out", 0.0, 0.0, Color32::RED).with_pulsing();
        assert!(h.pulsing);
    }

    #[test]
    fn test_node_highlight_with_radius() {
        let h = NodeHighlight::new("out", 0.0, 0.0, Color32::RED).with_radius(20.0);
        assert!((h.radius - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_trace_highlight_new() {
        let h = TraceHighlight::new("v(out)", 0, Color32::BLUE);
        assert_eq!(h.signal_name, "v(out)");
        assert_eq!(h.trace_index, 0);
        assert!(h.thickened);
    }

    #[test]
    fn test_format_time_seconds() {
        let s = format_time(1.5);
        assert!(s.contains("1.500s"));
    }

    #[test]
    fn test_format_time_ms() {
        let s = format_time(1.5e-3);
        assert!(s.contains("1.500ms"));
    }

    #[test]
    fn test_format_time_us() {
        let s = format_time(1.5e-6);
        assert!(s.contains("1.500µs"));
    }

    #[test]
    fn test_format_time_ns() {
        let s = format_time(1.5e-9);
        assert!(s.contains("1.500ns"));
    }

    #[test]
    fn test_format_time_ps() {
        let s = format_time(1.5e-12);
        assert!(s.contains("1.500ps"));
    }

    #[test]
    fn test_selected_bg_color() {
        let c = selected_bg_color();
        assert!(c.a() < 255); // Should be semi-transparent
    }

    #[test]
    fn test_signal_colors() {
        let _v = signal_voltage_color();
        let _i = signal_current_color();
        let _d = signal_digital_color();
    }
}
