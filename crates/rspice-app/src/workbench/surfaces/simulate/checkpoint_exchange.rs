//! Portable trial journals and a small, cached inspection view.
use rspice_formats::monte_carlo_checkpoint as checkpoint_file;

use super::page_kit::{Tone, note_line, rule_row};
use crate::io::file_exchange::{self, FileKind};
use crate::product::ProjectId;
use crate::state::MonteCarloCheckpointEvidence;
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, invalidate_preflight};

pub(super) enum Action {
    Transient(super::transient_checkpoint_exchange::Action),
    Import,
    Export(MonteCarloCheckpointEvidence),
    Inspect(String, MonteCarloCheckpointEvidence),
    Remove(crate::product::ContentDigest),
}
pub(super) fn apply(ctx: &egui::Context, cx: &mut Cx<'_, SimulateFeature>, action: Action) {
    match action {
        Action::Transient(action) => super::transient_checkpoint_exchange::apply(ctx, cx, action),
        Action::Import => begin_import(ctx, cx.model),
        Action::Export(checkpoint) => begin_export(ctx, cx.model, &checkpoint),
        Action::Inspect(label, checkpoint) => inspect(ctx, cx.model, label, checkpoint),
        Action::Remove(digest) => {
            if cx
                .model
                .simulation
                .retained
                .imported_monte_carlo_checkpoints
                .remove(digest)
            {
                invalidate_preflight(cx);
            }
        }
    }
}

const KIND: FileKind = FileKind {
    label: "RSpice Monte Carlo checkpoint",
    extensions: &["rspice-mc", "json"],
    subject: "The Monte Carlo checkpoint",
    fallback_name: "Monte Carlo checkpoint",
};
fn import_id() -> egui::Id {
    egui::Id::new("simulation.mc.import")
}
fn export_id() -> egui::Id {
    egui::Id::new("simulation.mc.export")
}
fn owner_id() -> egui::Id {
    egui::Id::new("simulation.mc.import-owner")
}
fn inspection_id() -> egui::Id {
    egui::Id::new("simulation.mc.inspection")
}

#[derive(Clone, PartialEq, Eq)]
struct Owner {
    project: ProjectId,
    epoch: u64,
}
impl Owner {
    fn of(model: &Model) -> Self {
        Self {
            project: model.workspace.content.project.id(),
            epoch: model.design_execution_epoch,
        }
    }
}

pub(super) fn begin_import(ctx: &egui::Context, model: &mut Model) {
    match file_exchange::open_file(ctx, import_id(), KIND, checkpoint_file::size_limit()) {
        Ok(()) => {
            ctx.data_mut(|data| {
                data.insert_temp(owner_id(), Owner::of(model));
            });
        }
        Err(error) => model.report_failure("Checkpoint import failed", error),
    }
}

pub(super) fn begin_export(
    ctx: &egui::Context,
    model: &mut Model,
    checkpoint: &MonteCarloCheckpointEvidence,
) {
    let result = checkpoint_file::encode(checkpoint)
        .map_err(|error| error.to_string())
        .and_then(|bytes| {
            let digest = checkpoint.digest().to_string();
            file_exchange::save_file(
                ctx,
                export_id(),
                KIND,
                format!("mc-{}.rspice-mc", &digest[..12]),
                bytes,
            )
        });
    if let Err(error) = result {
        model.report_failure("Checkpoint export failed", error);
    }
}

#[derive(Clone)]
struct Inspection {
    owner: Owner,
    label: String,
    checkpoint: MonteCarloCheckpointEvidence,
    summary: std::sync::Arc<rspice_results::monte_carlo_checkpoint::CheckpointInspection>,
}
pub(super) fn inspect(
    ctx: &egui::Context,
    model: &mut Model,
    label: String,
    checkpoint: MonteCarloCheckpointEvidence,
) {
    let decoded =
        rspice_results::monte_carlo_checkpoint::StudyMonteCarloCheckpoint::from_bytes_with_limits(
            checkpoint.bytes(),
            rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        );
    match decoded {
        Ok(journal) => ctx.data_mut(|data| {
            data.insert_temp(
                inspection_id(),
                Inspection {
                    owner: Owner::of(model),
                    label,
                    checkpoint,
                    summary: std::sync::Arc::new(journal.inspection()),
                },
            );
        }),
        Err(error) => model.report_failure("Checkpoint inspection failed", error.to_string()),
    }
}

/// Hosted every frame, even when the user closes the analysis editor or changes
/// workspace. Taking stale picker results releases their bounded mailbox.
pub(super) fn poll(ctx: &egui::Context, cx: &mut Cx<'_, SimulateFeature>) {
    if let Some(outcome) = file_exchange::take_opened(ctx, import_id()) {
        let owner = ctx.data_mut(|data| {
            let owner = data.get_temp::<Owner>(owner_id());
            data.remove::<Owner>(owner_id());
            owner
        });
        if owner.as_ref() == Some(&Owner::of(cx.model)) {
            match outcome {
                Ok(Some(file)) => {
                    let name = file.name.clone();
                    let result = checkpoint_file::decode(&file.text)
                        .map_err(|error| error.to_string())
                        .and_then(|checkpoint| {
                            cx.model
                                .simulation
                                .retained
                                .imported_monte_carlo_checkpoints
                                .insert(file.name, checkpoint)
                        });
                    match result {
                        Ok(inserted) => {
                            invalidate_preflight(cx);
                            cx.wb.prompt(if inserted {
                                format!(
                                    "Imported {name}; select it under Resume previous trials \
                                     to reuse it."
                                )
                            } else {
                                format!("{name} is already imported.")
                            });
                        }
                        Err(error) => cx.model.report_failure("Checkpoint import failed", error),
                    }
                }
                Ok(None) => {}
                Err(error) => cx.model.report_failure("Checkpoint import failed", error),
            }
        }
    }
    if let Some(outcome) = file_exchange::take_saved(ctx, export_id()) {
        match outcome {
            Ok(Some(file)) => cx.wb.prompt(format!("Saved checkpoint {}", file.name)),
            Ok(None) => {}
            Err(error) => cx.model.report_failure("Checkpoint export failed", error),
        }
    }
    let Some(inspection) = ctx.data(|data| data.get_temp::<Inspection>(inspection_id())) else {
        return;
    };
    let mut open = inspection.owner == Owner::of(cx.model);
    if open {
        egui::Window::new("Monte Carlo checkpoint")
            .id(inspection_id())
            .open(&mut open)
            .default_width(520.0)
            .show(ctx, |ui| {
                let summary = &inspection.summary;
                rule_row(ui, "Checkpoint", &inspection.label);
                for (label, value) in [
                    (
                        "Committed trials",
                        inspection.checkpoint.completed_trials().to_string(),
                    ),
                    ("Trials with values", summary.successful_trials.to_string()),
                    ("Failed trials", summary.failed_trials.to_string()),
                    (
                        "Trials missing measurement goals",
                        summary.failed_measurement_trials.to_string(),
                    ),
                    (
                        "Retained bytes",
                        inspection.checkpoint.bytes().len().to_string(),
                    ),
                    ("Trial indices (from 0)", summary.ranges.join(", ")),
                    ("Measurements", summary.measurement_count.to_string()),
                ] {
                    rule_row(ui, label, &value);
                }
                if summary.additional_ranges > 0 {
                    note_line(
                        ui,
                        &format!(
                            "{} more trial ranges are retained in the file.",
                            summary.additional_ranges
                        ),
                        Tone::Dim,
                    );
                }
                note_line(ui, &summary.measurements.join("; "), Tone::Dim);
                if summary.measurement_count > summary.measurements.len() {
                    note_line(
                        ui,
                        "Additional measurements are retained in the file.",
                        Tone::Dim,
                    );
                }
                note_line(
                    ui,
                    "Failed trials are retained too. Resuming reuses their recorded outcome; \
                     change the trial range or run fresh to evaluate new trials.",
                    Tone::Dim,
                );
                if Button::new("Export checkpoint…").show(ui).clicked() {
                    begin_export(ctx, cx.model, &inspection.checkpoint);
                }
            });
    }
    if !open {
        ctx.data_mut(|data| data.remove::<Inspection>(inspection_id()));
    }
}

#[cfg(test)]
mod tests;
