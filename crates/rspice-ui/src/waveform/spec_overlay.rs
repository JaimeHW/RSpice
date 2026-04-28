//! Specification Overlay for Waveform Viewer
//!
//! Visualizes design limits (Min/Max/Range) directly on the plot.
//! Highlights pass/fail zones and specification violations.

use crate::services::yield_manager::YieldSpec;
use egui::{Color32, Painter, Rect, Rounding};
use serde::{Deserialize, Serialize};

/// Visual representation of a design specification on a plot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecOverlay {
    /// Associated specification
    pub spec: YieldSpec,
    /// Color for the pass zone (RGBA)
    pub pass_color: [u8; 4],
    /// Color for the fail zone (RGBA)
    pub fail_color: [u8; 4],
    /// Opacity of the shaded area (0.0 - 1.0)
    pub opacity: f32,
    /// Whether to show the target line
    pub show_target: bool,
    /// Whether to highlight violations on the trace
    pub highlight_violations: bool,
    /// Whether this overlay is visible
    pub visible: bool,
}

impl Default for SpecOverlay {
    fn default() -> Self {
        Self {
            spec: YieldSpec::range("v_out", 0.0, 1.0, "V"),
            pass_color: [46, 204, 113, 40], // Translucent Green
            fail_color: [231, 76, 60, 40],  // Translucent Red
            opacity: 0.2,
            show_target: true,
            highlight_violations: true,
            visible: true,
        }
    }
}

impl SpecOverlay {
    /// Create from a YieldSpec
    pub fn from_spec(spec: YieldSpec) -> Self {
        Self {
            spec,
            ..Default::default()
        }
    }

    /// Check if a coordinate (x, y) violates the spec
    pub fn is_violating(&self, _x: f64, y: f64) -> bool {
        !self.spec.evaluates(y)
    }

    /// Get pass zone color as Color32
    pub fn pass_color32(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(
            self.pass_color[0],
            self.pass_color[1],
            self.pass_color[2],
            (self.pass_color[3] as f32 * self.opacity) as u8,
        )
    }

    /// Get fail zone color as Color32
    pub fn fail_color32(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(
            self.fail_color[0],
            self.fail_color[1],
            self.fail_color[2],
            (self.fail_color[3] as f32 * self.opacity) as u8,
        )
    }

    /// Render this spec overlay on the waveform plot
    ///
    /// Draws pass/fail zones and optional target line based on the spec type.
    pub fn render(&self, painter: &Painter, plot_rect: Rect, y_min: f64, y_max: f64) {
        if !self.visible || y_max <= y_min {
            return;
        }

        let y_range = y_max - y_min;
        let plot_height = plot_rect.height() as f64;

        // Helper: convert data Y to screen Y (inverted)
        let y_to_screen = |y: f64| -> f32 {
            let frac = (y_max - y) / y_range;
            plot_rect.top() + (frac * plot_height) as f32
        };

        // Draw based on spec type
        match (self.spec.min, self.spec.max) {
            // Range: min <= value <= max
            (Some(min), Some(max)) if min < max => {
                // Fail zone: above max
                if max < y_max {
                    let top = plot_rect.top();
                    let bottom = y_to_screen(max);
                    let fail_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), top),
                        egui::pos2(plot_rect.right(), bottom),
                    );
                    painter.rect_filled(fail_rect, Rounding::ZERO, self.fail_color32());
                }

                // Pass zone: between min and max
                let pass_top = y_to_screen(max.min(y_max));
                let pass_bottom = y_to_screen(min.max(y_min));
                let pass_rect = Rect::from_min_max(
                    egui::pos2(plot_rect.left(), pass_top),
                    egui::pos2(plot_rect.right(), pass_bottom),
                );
                painter.rect_filled(pass_rect, Rounding::ZERO, self.pass_color32());

                // Fail zone: below min
                if min > y_min {
                    let top = y_to_screen(min);
                    let bottom = plot_rect.bottom();
                    let fail_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), top),
                        egui::pos2(plot_rect.right(), bottom),
                    );
                    painter.rect_filled(fail_rect, Rounding::ZERO, self.fail_color32());
                }

                // Target line (midpoint)
                if self.show_target {
                    let target = (min + max) / 2.0;
                    if target >= y_min && target <= y_max {
                        let target_y = y_to_screen(target);
                        painter.hline(
                            plot_rect.x_range(),
                            target_y,
                            egui::Stroke::new(1.0, Color32::from_rgb(255, 193, 7)),
                        );
                    }
                }
            }

            // Minimum only: value >= min
            (Some(min), None) => {
                // Pass zone: above min
                if min < y_max {
                    let pass_top = plot_rect.top();
                    let pass_bottom = y_to_screen(min.max(y_min));
                    let pass_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), pass_top),
                        egui::pos2(plot_rect.right(), pass_bottom),
                    );
                    painter.rect_filled(pass_rect, Rounding::ZERO, self.pass_color32());
                }

                // Fail zone: below min
                if min > y_min {
                    let fail_top = y_to_screen(min);
                    let fail_bottom = plot_rect.bottom();
                    let fail_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), fail_top),
                        egui::pos2(plot_rect.right(), fail_bottom),
                    );
                    painter.rect_filled(fail_rect, Rounding::ZERO, self.fail_color32());
                }

                // Target line at min threshold
                if self.show_target && min >= y_min && min <= y_max {
                    painter.hline(
                        plot_rect.x_range(),
                        y_to_screen(min),
                        egui::Stroke::new(1.0, Color32::from_rgb(255, 193, 7)),
                    );
                }
            }

            // Maximum only: value <= max
            (None, Some(max)) => {
                // Fail zone: above max
                if max < y_max {
                    let fail_top = plot_rect.top();
                    let fail_bottom = y_to_screen(max);
                    let fail_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), fail_top),
                        egui::pos2(plot_rect.right(), fail_bottom),
                    );
                    painter.rect_filled(fail_rect, Rounding::ZERO, self.fail_color32());
                }

                // Pass zone: below max
                if max > y_min {
                    let pass_top = y_to_screen(max.min(y_max));
                    let pass_bottom = plot_rect.bottom();
                    let pass_rect = Rect::from_min_max(
                        egui::pos2(plot_rect.left(), pass_top),
                        egui::pos2(plot_rect.right(), pass_bottom),
                    );
                    painter.rect_filled(pass_rect, Rounding::ZERO, self.pass_color32());
                }

                // Target line at max threshold
                if self.show_target && max >= y_min && max <= y_max {
                    painter.hline(
                        plot_rect.x_range(),
                        y_to_screen(max),
                        egui::Stroke::new(1.0, Color32::from_rgb(255, 193, 7)),
                    );
                }
            }

            // No valid limits
            _ => {}
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

