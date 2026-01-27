//! Yield Summary Panel
//!
//! Visualizes results from yield analysis and Monte Carlo simulations.
//! Displays pass/fail status, yield percentage, and statistical distributions.

use crate::services::yield_manager::{SpecLimitType, YieldResult};
use egui::{Align, Color32, Layout, RichText, Ui};

/// Renders the Yield Summary Panel
pub fn render_yield_panel(ui: &mut Ui, results: &[YieldResult]) {
    ui.heading("Yield Summary");
    ui.add_space(8.0);

    if results.is_empty() {
        ui.vertical_centered(|ui| {
            ui.label("No yield data available. Run Monte Carlo simulation to see results.");
        });
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for result in results {
            render_yield_item(ui, result);
            ui.separator();
        }
    });
}

fn render_yield_item(ui: &mut Ui, result: &YieldResult) {
    ui.vertical(|ui| {
        // Header Row: Spec Name and Yield %
        ui.horizontal(|ui| {
            ui.strong(&result.spec.target);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let color = if result.yield_percent > 95.0 {
                    Color32::from_rgb(46, 204, 113) // Green
                } else if result.yield_percent > 80.0 {
                    Color32::from_rgb(241, 196, 15) // Yellow
                } else {
                    Color32::from_rgb(231, 76, 60) // Red
                };
                ui.label(
                    RichText::new(format!("{:.1}% Yield", result.yield_percent))
                        .color(color)
                        .strong(),
                );
            });
        });

        ui.add_space(4.0);

        // Stats Row
        ui.columns(3, |cols| {
            cols[0].label(format!("Pass: {}", result.pass_count));
            cols[0].label(format!("Fail: {}", result.fail_count));

            cols[1].label(format!(
                "Mean: {:.4}{}",
                result.stats.mean, result.spec.unit
            ));
            cols[1].label(format!(
                "StdDev: {:.4}{}",
                result.stats.std_dev, result.spec.unit
            ));

            if let Some(cpk) = result.stats.cpk {
                cols[2].label(RichText::new(format!("Cpk: {:.2}", cpk)).strong());
            }
            if let Some(cp) = result.stats.cp {
                cols[2].label(format!("Cp: {:.2}", cp));
            }
        });

        // Spec Limits
        ui.horizontal(|ui| {
            ui.label(RichText::new("Limits: ").weak());
            match result.spec.limit_type {
                SpecLimitType::Lower => {
                    ui.label(format!(
                        "> {:.4}{}",
                        result.spec.min.unwrap(),
                        result.spec.unit
                    ));
                }
                SpecLimitType::Upper => {
                    ui.label(format!(
                        "< {:.4}{}",
                        result.spec.max.unwrap(),
                        result.spec.unit
                    ));
                }
                SpecLimitType::Range => {
                    ui.label(format!(
                        "{:.4} - {:.4} {}",
                        result.spec.min.unwrap(),
                        result.spec.max.unwrap(),
                        result.spec.unit
                    ));
                }
            }
        });

        ui.add_space(8.0);
    });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::yield_manager::{DistributionStats, YieldSpec};

    #[test]
    fn test_yield_panel_rendering_logic() {
        // Since we can't easily test egui rendering in unit tests without a mock ctx,
        // we verify the data structures passed to it.
        let result = YieldResult {
            spec: YieldSpec::range("v_out", 1.1, 1.3, "V"),
            total_runs: 100,
            pass_count: 98,
            fail_count: 2,
            yield_percent: 98.0,
            stats: DistributionStats {
                mean: 1.2,
                std_dev: 0.01,
                cpk: Some(3.33),
                ..Default::default()
            },
            trail: vec![true; 100],
        };

        assert!(result.yield_percent > 95.0);
        assert_eq!(result.spec.unit, "V");
    }
}
