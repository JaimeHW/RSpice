//! Versioned entity inspection, comparison receipts, and integrity-scan controls.

use super::{
    stage::{ResultEntityRow, result_entity_table},
    widgets::{empty_note, panel_heading, separator},
};
use egui::{RichText, ScrollArea, Ui};
use rspice_app_types::product::short_identity as short_dataset;
use rspice_results::visualization_document::ComparisonReceipt;
use rspice_ui_kit::{
    panels::property_row,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationState {
    #[default]
    NotStarted,
    Running,
    Cancelled,
    Completed,
}

pub struct OperationProgress {
    pub state: OperationState,
    pub processed: usize,
    pub total: usize,
    pub checksum: u64,
}

#[derive(Clone, Copy)]
pub enum InspectorAction {
    OpenComparison,
    Start,
    Advance,
    Cancel,
    Recover,
}

/// Source queries and immediate actions stay with the application coordinator.
pub trait InspectorHost {
    fn entities(&self) -> Vec<ResultEntityRow>;
    fn latest_comparison(&self) -> Option<&ComparisonReceipt>;
    fn operation(&self) -> OperationProgress;
    fn source_available(&self) -> bool;
    fn request(&mut self, action: InspectorAction);
}

pub fn viewer_inspector(ui: &mut Ui, host: &mut impl InspectorHost, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let entities = host.entities();
    panel_heading(ui, "Versioned result entities", &entities.len().to_string());
    ScrollArea::vertical()
        .id_salt("visualization.entity-inspector")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            result_entity_table(ui, &entities);

            separator(ui, t.color.border);
            let latest_comparison = host.latest_comparison();
            panel_heading(
                ui,
                "Comparison receipt",
                if latest_comparison.is_some() {
                    "current"
                } else {
                    "none"
                },
            );
            ui.label(
                RichText::new(
                    "Select explicit dataset alignment, units, interpolation, resampling, extrapolation, and precision before comparing immutable sources.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
            );
            if let Some(receipt) = latest_comparison {
                property_row(
                    ui,
                    "Datasets",
                    &format!(
                        "{} → {}",
                        short_dataset(receipt.baseline.dataset_id),
                        short_dataset(receipt.candidate.dataset_id)
                    ),
                );
                property_row(ui, "Rows", &receipt.rows_compared.to_string());
                property_row(ui, "Alignment", "Exact coordinate rows");
                property_row(ui, "Units", "Identical units required");
                property_row(ui, "Interpolation", "None · exact only");
                property_row(ui, "Resampling", "None · source grid retained");
                property_row(ui, "Extrapolation", "Forbidden");
                property_row(ui, "Precision", "Source f64 · no rounding");
                property_row(
                    ui,
                    "Disposition",
                    match receipt.disposition {
                        rspice_results::visualization_document::ComparisonDisposition::Passed => {
                            "passed"
                        }
                        rspice_results::visualization_document::ComparisonDisposition::Failed => {
                            "failed"
                        }
                    },
                );
            } else {
                empty_note(
                    ui,
                    "No comparison receipt has been created for the selected immutable datasets.",
                );
            }
            if Button::new("Plan explicit comparison").show(ui).clicked() {
                host.request(InspectorAction::OpenComparison);
            }

            separator(ui, t.color.border);
            let progress = host.operation();
            let operation = progress.state;
            panel_heading(ui, "Progressive operation", operation_label(operation));
            ui.label(
                RichText::new("Exact source-sample integrity scan")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            let fraction = if progress.total == 0 {
                0.0
            } else {
                progress.processed as f32 / progress.total as f32
            };
            ui.add(egui::ProgressBar::new(fraction).show_percentage());
            if operation == OperationState::Completed {
                ui.monospace(format!(
                    "{} samples · checksum {:016x}",
                    progress.total, progress.checksum
                ));
            }
            let source_available = host.source_available();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(
                        source_available
                            && matches!(
                                operation,
                                OperationState::NotStarted | OperationState::Completed
                            ),
                        egui::Button::new("Start"),
                    )
                    .on_disabled_hover_text(
                        "Start requires a selected retained analysis with exact source samples",
                    )
                    .clicked()
                {
                    host.request(InspectorAction::Start);
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Advance"),
                    )
                    .on_disabled_hover_text("Advance requires a running selected operation")
                    .clicked()
                {
                    host.request(InspectorAction::Advance);
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Cancel"),
                    )
                    .on_disabled_hover_text("Cancel requires a running selected operation")
                    .clicked()
                {
                    host.request(InspectorAction::Cancel);
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Cancelled,
                        egui::Button::new("Recover"),
                    )
                    .on_disabled_hover_text("Recover requires a cancelled operation")
                    .clicked()
                {
                    host.request(InspectorAction::Recover);
                }
            });
            if compact {
                ui.add_space(16.0);
            }
        });
}

const fn operation_label(state: OperationState) -> &'static str {
    match state {
        OperationState::NotStarted => "not started",
        OperationState::Running => "running",
        OperationState::Cancelled => "cancelled",
        OperationState::Completed => "completed",
    }
}
