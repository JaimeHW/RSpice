//! Dialog Framework
//!
//! Base framework for simulation configuration dialogs.

use egui::{Color32, Pos2, Rounding, Sense, Stroke, Ui};

// =============================================================================
// Dialog Result
// =============================================================================

/// Result of dialog interaction
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DialogResult {
    /// User cancelled
    Cancelled,
    /// User accepted/confirmed
    Accepted,
    /// Dialog still open
    #[default]
    Open,
}

impl DialogResult {
    /// Is dialog still open?
    pub fn is_open(&self) -> bool {
        *self == Self::Open
    }

    /// Was accepted?
    pub fn is_accepted(&self) -> bool {
        *self == Self::Accepted
    }
}

// =============================================================================
// Dialog Tab
// =============================================================================

/// Base trait for dialog tabs
pub trait DialogTab {
    /// Tab name for header
    fn name(&self) -> &str;

    /// Tab description
    fn description(&self) -> &str;

    /// Is this tab enabled?
    fn is_enabled(&self) -> bool {
        true
    }

    /// Render tab content
    fn render(&mut self, ui: &mut Ui);

    /// Validate tab configuration
    fn validate(&self) -> Result<(), String>;

    /// Reset to defaults
    fn reset(&mut self);
}

// =============================================================================
// Simulation Dialog
// =============================================================================

/// Main simulation configuration dialog
#[derive(Default)]
pub struct SimulationDialog {
    /// Is dialog visible?
    pub visible: bool,
    /// Dialog title
    pub title: String,
    /// Current tab index
    pub current_tab: usize,
    /// Tab names
    pub tab_names: Vec<String>,
    /// Validation message
    pub validation_message: Option<String>,
    /// Result
    pub result: DialogResult,
}

impl SimulationDialog {
    /// Create new dialog
    pub fn new(title: &str) -> Self {
        Self {
            visible: false,
            title: title.to_string(),
            current_tab: 0,
            tab_names: Vec::new(),
            validation_message: None,
            result: DialogResult::Open,
        }
    }

    /// Open dialog
    pub fn open(&mut self) {
        self.visible = true;
        self.result = DialogResult::Open;
        self.validation_message = None;
    }

    /// Close dialog
    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Cancel and close
    pub fn cancel(&mut self) {
        self.result = DialogResult::Cancelled;
        self.close();
    }

    /// Accept and close
    pub fn accept(&mut self) {
        self.result = DialogResult::Accepted;
        self.close();
    }

    /// Is visible?
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Add tab
    pub fn add_tab(&mut self, name: &str) {
        self.tab_names.push(name.to_string());
    }

    /// Set current tab
    pub fn set_tab(&mut self, index: usize) {
        if index < self.tab_names.len() {
            self.current_tab = index;
        }
    }

    /// Get current tab index
    pub fn current_tab(&self) -> usize {
        self.current_tab
    }

    /// Number of tabs
    pub fn tab_count(&self) -> usize {
        self.tab_names.len()
    }

    /// Set validation error
    pub fn set_validation_error(&mut self, msg: &str) {
        self.validation_message = Some(msg.to_string());
    }

    /// Clear validation error
    pub fn clear_validation_error(&mut self) {
        self.validation_message = None;
    }

    /// Render the dialog shell (call before rendering tabs)
    pub fn begin(&mut self, ui: &mut Ui) -> bool {
        if !self.visible {
            return false;
        }

        // Modal background overlay
        let screen_rect = ui.ctx().screen_rect();
        ui.painter().rect_filled(
            screen_rect,
            Rounding::ZERO,
            Color32::from_rgba_unmultiplied(0, 0, 0, 180),
        );

        true
    }

    /// Render tab bar
    pub fn render_tab_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (i, name) in self.tab_names.iter().enumerate() {
                let is_selected = i == self.current_tab;
                let text_color = if is_selected {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(150, 155, 165)
                };

                let response = ui.add(
                    egui::Label::new(egui::RichText::new(name).size(12.0).color(text_color))
                        .sense(Sense::click()),
                );

                if response.clicked() {
                    self.current_tab = i;
                }

                if is_selected {
                    // Underline
                    let rect = response.rect;
                    ui.painter().line_segment(
                        [
                            Pos2::new(rect.min.x, rect.max.y),
                            Pos2::new(rect.max.x, rect.max.y),
                        ],
                        Stroke::new(2.0, Color32::from_rgb(100, 150, 255)),
                    );
                }

                ui.add_space(8.0);
            }
        });

        ui.add_space(8.0);
        ui.separator();
    }

    /// Render dialog buttons
    pub fn render_buttons(&mut self, ui: &mut Ui) -> DialogResult {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        // Validation message
        if let Some(ref msg) = self.validation_message {
            ui.label(
                egui::RichText::new(msg)
                    .size(11.0)
                    .color(Color32::from_rgb(255, 100, 100)),
            );
            ui.add_space(4.0);
        }

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                self.cancel();
            }

            ui.add_space(8.0);

            if ui.button(egui::RichText::new("Run").strong()).clicked() {
                self.accept();
            }
        });

        self.result.clone()
    }
}

// =============================================================================
// Common Input Widgets
// =============================================================================

/// Render a labeled input field
pub fn labeled_input(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(11.0)
                .color(Color32::from_rgb(180, 185, 195)),
        );

        let response = ui.add(
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .desired_width(120.0),
        );

        if response.changed() {
            changed = true;
        }
    });

    changed
}

/// Render a labeled numeric input
pub fn numeric_input(ui: &mut Ui, label: &str, value: &mut f64, unit: &str) -> bool {
    let mut changed = false;
    let mut text = format_value(*value);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(11.0)
                .color(Color32::from_rgb(180, 185, 195)),
        );

        let response = ui.add(egui::TextEdit::singleline(&mut text).desired_width(80.0));

        if response.changed() {
            if let Some(v) = parse_value(&text) {
                *value = v;
                changed = true;
            }
        }

        ui.label(
            egui::RichText::new(unit)
                .size(10.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
    });

    changed
}

/// Render a checkbox with label
pub fn labeled_checkbox(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        if ui.checkbox(value, "").changed() {
            changed = true;
        }

        ui.label(
            egui::RichText::new(label)
                .size(11.0)
                .color(Color32::from_rgb(180, 185, 195)),
        );
    });

    changed
}

/// Format a value with SI prefix
fn format_value(value: f64) -> String {
    let abs = value.abs();
    if abs == 0.0 {
        "0".to_string()
    } else if abs >= 1e9 {
        format!("{:.3}G", value / 1e9)
    } else if abs >= 1e6 {
        format!("{:.3}M", value / 1e6)
    } else if abs >= 1e3 {
        format!("{:.3}k", value / 1e3)
    } else if abs >= 1.0 {
        format!("{:.3}", value)
    } else if abs >= 1e-3 {
        format!("{:.3}m", value * 1e3)
    } else if abs >= 1e-6 {
        format!("{:.3}u", value * 1e6)
    } else if abs >= 1e-9 {
        format!("{:.3}n", value * 1e9)
    } else if abs >= 1e-12 {
        format!("{:.3}p", value * 1e12)
    } else {
        format!("{:.3}f", value * 1e15)
    }
}

/// Parse a value with SI prefix
fn parse_value(text: &str) -> Option<f64> {
    let text = text.trim();
    if text.is_empty() {
        return Some(0.0);
    }

    // Check for SI suffix
    let (num_str, multiplier) = if text.ends_with('G') || text.ends_with('g') {
        (&text[..text.len() - 1], 1e9)
    } else if text.ends_with("meg") || text.ends_with("MEG") {
        (&text[..text.len() - 3], 1e6)
    } else if let Some(stripped) = text.strip_suffix('M') {
        (stripped, 1e6)
    } else if text.ends_with('k') || text.ends_with('K') {
        (&text[..text.len() - 1], 1e3)
    } else if let Some(stripped) = text.strip_suffix('m') {
        (stripped, 1e-3)
    } else if text.ends_with('u') || text.ends_with('U') {
        (&text[..text.len() - 1], 1e-6)
    } else if text.ends_with('n') || text.ends_with('N') {
        (&text[..text.len() - 1], 1e-9)
    } else if text.ends_with('p') || text.ends_with('P') {
        (&text[..text.len() - 1], 1e-12)
    } else if text.ends_with('f') || text.ends_with('F') {
        (&text[..text.len() - 1], 1e-15)
    } else {
        (text, 1.0)
    };

    num_str.parse::<f64>().ok().map(|v| v * multiplier)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DialogResult Tests
    // =========================================================================

    #[test]
    fn test_dialog_result_default() {
        let r = DialogResult::default();
        assert!(r.is_open());
    }

    #[test]
    fn test_dialog_result_open() {
        let r = DialogResult::Open;
        assert!(r.is_open());
        assert!(!r.is_accepted());
    }

    #[test]
    fn test_dialog_result_accepted() {
        let r = DialogResult::Accepted;
        assert!(r.is_accepted());
        assert!(!r.is_open());
    }

    #[test]
    fn test_dialog_result_cancelled() {
        let r = DialogResult::Cancelled;
        assert!(!r.is_open());
        assert!(!r.is_accepted());
    }

    // =========================================================================
    // SimulationDialog Tests
    // =========================================================================

    #[test]
    fn test_dialog_new() {
        let d = SimulationDialog::new("Test");
        assert_eq!(d.title, "Test");
        assert!(!d.is_visible());
    }

    #[test]
    fn test_dialog_open() {
        let mut d = SimulationDialog::new("Test");
        d.open();
        assert!(d.is_visible());
    }

    #[test]
    fn test_dialog_close() {
        let mut d = SimulationDialog::new("Test");
        d.open();
        d.close();
        assert!(!d.is_visible());
    }

    #[test]
    fn test_dialog_cancel() {
        let mut d = SimulationDialog::new("Test");
        d.open();
        d.cancel();
        assert!(!d.is_visible());
        assert_eq!(d.result, DialogResult::Cancelled);
    }

    #[test]
    fn test_dialog_accept() {
        let mut d = SimulationDialog::new("Test");
        d.open();
        d.accept();
        assert!(!d.is_visible());
        assert_eq!(d.result, DialogResult::Accepted);
    }

    #[test]
    fn test_dialog_add_tab() {
        let mut d = SimulationDialog::new("Test");
        d.add_tab("Tab1");
        d.add_tab("Tab2");
        assert_eq!(d.tab_count(), 2);
    }

    #[test]
    fn test_dialog_set_tab() {
        let mut d = SimulationDialog::new("Test");
        d.add_tab("Tab1");
        d.add_tab("Tab2");
        d.set_tab(1);
        assert_eq!(d.current_tab(), 1);
    }

    #[test]
    fn test_dialog_set_tab_bounds() {
        let mut d = SimulationDialog::new("Test");
        d.add_tab("Tab1");
        d.set_tab(10); // Out of bounds
        assert_eq!(d.current_tab(), 0); // Should not change
    }

    #[test]
    fn test_dialog_validation_error() {
        let mut d = SimulationDialog::new("Test");
        d.set_validation_error("Error!");
        assert!(d.validation_message.is_some());

        d.clear_validation_error();
        assert!(d.validation_message.is_none());
    }

    // =========================================================================
    // Value Parsing Tests
    // =========================================================================

    #[test]
    fn test_parse_plain_number() {
        assert!((parse_value("100").unwrap() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_giga() {
        assert!((parse_value("1G").unwrap() - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_parse_mega() {
        assert!((parse_value("1M").unwrap() - 1e6).abs() < 1.0);
    }

    #[test]
    fn test_parse_kilo() {
        assert!((parse_value("1k").unwrap() - 1e3).abs() < 0.01);
    }

    #[test]
    fn test_parse_milli() {
        assert!((parse_value("1m").unwrap() - 1e-3).abs() < 1e-6);
    }

    #[test]
    fn test_parse_micro() {
        assert!((parse_value("1u").unwrap() - 1e-6).abs() < 1e-9);
    }

    #[test]
    fn test_parse_nano() {
        assert!((parse_value("1n").unwrap() - 1e-9).abs() < 1e-12);
    }

    #[test]
    fn test_parse_pico() {
        assert!((parse_value("1p").unwrap() - 1e-12).abs() < 1e-15);
    }

    #[test]
    fn test_parse_femto() {
        assert!((parse_value("1f").unwrap() - 1e-15).abs() < 1e-18);
    }

    #[test]
    fn test_parse_empty() {
        assert!((parse_value("").unwrap() - 0.0).abs() < 1e-15);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_value("abc").is_none());
    }

    // =========================================================================
    // Value Formatting Tests
    // =========================================================================

    #[test]
    fn test_format_zero() {
        assert_eq!(format_value(0.0), "0");
    }

    #[test]
    fn test_format_giga() {
        let s = format_value(1e9);
        assert!(s.contains("G"));
    }

    #[test]
    fn test_format_mega() {
        let s = format_value(1e6);
        assert!(s.contains("M"));
    }

    #[test]
    fn test_format_kilo() {
        let s = format_value(1e3);
        assert!(s.contains("k"));
    }

    #[test]
    fn test_format_milli() {
        let s = format_value(1e-3);
        assert!(s.contains("m"));
    }

    #[test]
    fn test_format_micro() {
        let s = format_value(1e-6);
        assert!(s.contains("u"));
    }

    #[test]
    fn test_format_nano() {
        let s = format_value(1e-9);
        assert!(s.contains("n"));
    }

    #[test]
    fn test_format_pico() {
        let s = format_value(1e-12);
        assert!(s.contains("p"));
    }
}
