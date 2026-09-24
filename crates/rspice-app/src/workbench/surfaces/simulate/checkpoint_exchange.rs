//! Portable trial journals and a small, cached inspection view.
use crate::io::file_exchange::{self, FileKind};
use crate::product::ProjectId;
use crate::state::MonteCarloCheckpointEvidence;
use crate::workbench::{AppState, RSpiceApp};

pub(super) enum Action {
    Import,
    Export(MonteCarloCheckpointEvidence),
    Inspect(String, MonteCarloCheckpointEvidence),
    Remove(crate::product::ContentDigest),
}
pub(super) fn apply(ctx: &egui::Context, app: &mut RSpiceApp, action: Action) {
    match action {
        Action::Import => begin_import(ctx, &mut app.state),
        Action::Export(checkpoint) => begin_export(ctx, &mut app.state, &checkpoint),
        Action::Inspect(label, checkpoint) => inspect(ctx, &mut app.state, label, checkpoint),
        Action::Remove(digest) => {
            if app
                .state
                .simulation
                .imported_monte_carlo_checkpoints
                .remove(digest)
            {
                app.invalidate_simulation_preflight();
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
    fn of(state: &AppState) -> Self {
        Self {
            project: state.workspace.project.id(),
            epoch: state.design_execution_epoch,
        }
    }
}

pub(super) fn begin_import(ctx: &egui::Context, state: &mut AppState) {
    match file_exchange::open_file(
        ctx,
        import_id(),
        KIND,
        MonteCarloCheckpointEvidence::portable_file_limit(),
    ) {
        Ok(()) => {
            ctx.data_mut(|data| {
                data.insert_temp(owner_id(), Owner::of(state));
            });
        }
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Checkpoint import", error),
    }
}

pub(super) fn begin_export(
    ctx: &egui::Context,
    state: &mut AppState,
    checkpoint: &MonteCarloCheckpointEvidence,
) {
    let result = checkpoint.to_portable_file().and_then(|bytes| {
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
        state
            .ui
            .toasts
            .error_with_title(ctx, "Checkpoint export", error);
    }
}

#[derive(Clone)]
struct Inspection {
    owner: Owner,
    label: String,
    checkpoint: MonteCarloCheckpointEvidence,
    summary: std::sync::Arc<
        crate::simulation::runner::study::monte_carlo::checkpoint::CheckpointInspection,
    >,
}
pub(super) fn inspect(
    ctx: &egui::Context,
    state: &mut AppState,
    label: String,
    checkpoint: MonteCarloCheckpointEvidence,
) {
    let decoded = crate::simulation::runner::study::monte_carlo::checkpoint::StudyMonteCarloCheckpoint::from_bytes_with_limits(
        checkpoint.bytes(), rspice_core::ResourceLimits::default(), &rspice_core::NoAbort,
    );
    match decoded {
        Ok(journal) => ctx.data_mut(|data| {
            data.insert_temp(
                inspection_id(),
                Inspection {
                    owner: Owner::of(state),
                    label,
                    checkpoint,
                    summary: std::sync::Arc::new(journal.inspection()),
                },
            );
        }),
        Err(error) => {
            state
                .ui
                .toasts
                .error_with_title(ctx, "Checkpoint inspection", error.to_string())
        }
    }
}

/// Hosted every frame, even when the user closes the analysis editor or changes
/// workspace. Taking stale picker results releases their bounded mailbox.
pub(super) fn poll(ctx: &egui::Context, app: &mut RSpiceApp) {
    if let Some(outcome) = file_exchange::take_opened(ctx, import_id()) {
        let owner = ctx.data_mut(|data| {
            let owner = data.get_temp::<Owner>(owner_id());
            data.remove::<Owner>(owner_id());
            owner
        });
        if owner.as_ref() == Some(&Owner::of(&app.state)) {
            match outcome {
                Ok(Some(file)) => {
                    let result = MonteCarloCheckpointEvidence::from_portable_file(&file.text)
                        .and_then(|checkpoint| {
                            app.state
                                .simulation
                                .imported_monte_carlo_checkpoints
                                .insert(file.name, checkpoint)
                        });
                    match result {
                        Ok(inserted) => {
                            app.invalidate_simulation_preflight();
                            app.state.ui.toasts.success(ctx, "Checkpoint imported", if inserted { "Select it under Resume previous trials to reuse it. Compatibility is checked when preparing the run." } else { "This checkpoint is already imported." });
                        }
                        Err(error) => {
                            app.state
                                .ui
                                .toasts
                                .error_with_title(ctx, "Checkpoint import", error)
                        }
                    }
                }
                Ok(None) => {}
                Err(error) => app
                    .state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Checkpoint import", error),
            }
        }
    }
    if let Some(outcome) = file_exchange::take_saved(ctx, export_id()) {
        match outcome {
            Ok(Some(file)) => app.state.ui.toasts.success(
                ctx,
                "Checkpoint exported",
                format!("Saved {}", file.name),
            ),
            Ok(None) => {}
            Err(error) => app
                .state
                .ui
                .toasts
                .error_with_title(ctx, "Checkpoint export", error),
        }
    }
    let Some(inspection) = ctx.data(|data| data.get_temp::<Inspection>(inspection_id())) else {
        return;
    };
    let mut open = inspection.owner == Owner::of(&app.state);
    if open {
        egui::Window::new("Monte Carlo checkpoint").id(inspection_id()).open(&mut open).default_width(520.0).show(ctx, |ui| {
            ui.label(&inspection.label);
            let summary = &inspection.summary;
            egui::Grid::new("checkpoint-summary").show(ui, |ui| {
                for (label, value) in [
                    ("Committed trials", inspection.checkpoint.completed_trials().to_string()),
                    ("Trials with values", summary.successful_trials.to_string()),
                    ("Failed trials", summary.failed_trials.to_string()),
                    ("Trials missing measurement goals", summary.failed_measurement_trials.to_string()),
                    ("Retained bytes", inspection.checkpoint.bytes().len().to_string()),
                    ("Trial indices (from 0)", summary.ranges.join(", ")),
                    ("Measurements", summary.measurement_count.to_string()),
                ] { ui.label(label); ui.label(value); ui.end_row(); }
            });
            if summary.additional_ranges > 0 { ui.label(format!("{} more trial ranges are retained in the file.", summary.additional_ranges)); }
            ui.label(summary.measurements.join("; "));
            if summary.measurement_count > summary.measurements.len() { ui.label("Additional measurements are retained in the file."); }
            ui.label("Failed trials are retained too. Resuming reuses their recorded outcome; change the trial range or run fresh to evaluate new trials.");
            if ui.button("Export checkpoint…").clicked() { begin_export(ctx, &mut app.state, &inspection.checkpoint); }
        });
    }
    if !open {
        ctx.data_mut(|data| data.remove::<Inspection>(inspection_id()));
    }
}

#[cfg(test)]
mod tests;
