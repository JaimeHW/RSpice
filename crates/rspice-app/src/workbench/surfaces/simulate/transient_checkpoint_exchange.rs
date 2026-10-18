//! Exchange accepted transient images without converting packed bytes to text.
use super::page_kit::{Tone, note_line, rule_row};
use crate::io::file_exchange::{self, FileKind};
use crate::product::{ContentDigest, ProjectId};
use crate::ui::widgets::Button;
use crate::workbench::{AppState, RSpiceApp};
use rspice_results::transient_checkpoint::TransientCheckpointEvidence;

pub(super) enum Action {
    Import,
    Export(TransientCheckpointEvidence),
    Inspect(String, TransientCheckpointEvidence),
    Remove(ContentDigest),
}

const KIND: FileKind = FileKind {
    label: "RSpice transient checkpoint",
    extensions: &["rspice-tran"],
    subject: "The transient checkpoint",
    fallback_name: "Transient checkpoint",
};
fn import_id() -> egui::Id {
    egui::Id::new("simulation.tran.import")
}
fn export_id() -> egui::Id {
    egui::Id::new("simulation.tran.export")
}
fn owner_id() -> egui::Id {
    egui::Id::new("simulation.tran.import-owner")
}
fn inspection_id() -> egui::Id {
    egui::Id::new("simulation.tran.inspection")
}

#[derive(Clone, PartialEq, Eq)]
struct Owner {
    project: ProjectId,
    epoch: u64,
}
impl Owner {
    fn of(state: &AppState) -> Self {
        Self {
            project: state.workspace.content.project.id(),
            epoch: state.design_execution_epoch,
        }
    }
}

pub(super) fn apply(ctx: &egui::Context, app: &mut RSpiceApp, action: Action) {
    match action {
        Action::Import => {
            let limits = rspice_core::ResourceLimits::default();
            match file_exchange::open_binary_file(
                ctx,
                import_id(),
                KIND,
                limits.max_external_data_bytes,
            ) {
                Ok(()) => ctx.data_mut(|data| {
                    data.insert_temp(owner_id(), Owner::of(&app.state));
                }),
                Err(error) => app.state.report_failure("Checkpoint import failed", error),
            }
        }
        Action::Export(checkpoint) => export(ctx, &mut app.state, &checkpoint),
        Action::Inspect(label, checkpoint) => ctx.data_mut(|data| {
            data.insert_temp(
                inspection_id(),
                Inspection {
                    owner: Owner::of(&app.state),
                    label,
                    checkpoint,
                },
            );
        }),
        Action::Remove(digest) => {
            if app
                .state
                .simulation
                .retained
                .imported_transient_checkpoints
                .remove(digest)
            {
                app.invalidate_simulation_preflight();
            }
        }
    }
}

fn export(ctx: &egui::Context, state: &mut AppState, checkpoint: &TransientCheckpointEvidence) {
    let digest = checkpoint.digest().to_string();
    if let Err(error) = file_exchange::save_file(
        ctx,
        export_id(),
        KIND,
        format!("tran-{}.rspice-tran", &digest[..12]),
        checkpoint.bytes().to_vec(),
    ) {
        state.report_failure("Checkpoint export failed", error);
    }
}

#[derive(Clone)]
struct Inspection {
    owner: Owner,
    label: String,
    checkpoint: TransientCheckpointEvidence,
}

/// Drain even when the editor closes; a delayed picker cannot target a new design.
pub(super) fn poll(ctx: &egui::Context, app: &mut RSpiceApp) {
    if let Some(outcome) = file_exchange::take_opened_binary(ctx, import_id()) {
        let owner = ctx.data_mut(|data| {
            let owner = data.get_temp::<Owner>(owner_id());
            data.remove::<Owner>(owner_id());
            owner
        });
        if owner.as_ref() == Some(&Owner::of(&app.state)) {
            match outcome {
                Ok(Some(file)) => {
                    let name = file.name.clone();
                    let result = TransientCheckpointEvidence::from_bytes(file.bytes.into())
                        .map_err(|error| error.to_string())
                        .and_then(|checkpoint| {
                            app.state
                                .simulation
                                .retained
                                .imported_transient_checkpoints
                                .insert(file.name, checkpoint)
                        });
                    match result {
                        Ok(inserted) => {
                            app.invalidate_simulation_preflight();
                            app.state.prompt(if inserted {
                                format!("Imported {name}; select it under Resume transient.")
                            } else {
                                format!("{name} is already imported.")
                            });
                        }
                        Err(error) => app.state.report_failure("Checkpoint import failed", error),
                    }
                }
                Ok(None) => {}
                Err(error) => app.state.report_failure("Checkpoint import failed", error),
            }
        }
    }
    if let Some(outcome) = file_exchange::take_saved(ctx, export_id()) {
        match outcome {
            Ok(Some(file)) => app.state.prompt(if cfg!(target_arch = "wasm32") {
                format!("Finish saving {} in your browser.", file.name)
            } else {
                format!("Saved checkpoint {}", file.name)
            }),
            Ok(None) => {}
            Err(error) => app.state.report_failure("Checkpoint export failed", error),
        }
    }
    let Some(inspection) = ctx.data(|data| data.get_temp::<Inspection>(inspection_id())) else {
        return;
    };
    let mut open = inspection.owner == Owner::of(&app.state);
    if open {
        egui::Window::new("Transient checkpoint")
            .id(inspection_id())
            .open(&mut open)
            .default_width(520.0)
            .show(ctx, |ui| {
                let checkpoint = &inspection.checkpoint;
                rule_row(ui, "Checkpoint", &inspection.label);
                rule_row(
                    ui,
                    "Accepted time",
                    &format!("{:.17e} s", checkpoint.accepted_time()),
                );
                rule_row(ui, "Retained bytes", &checkpoint.bytes().len().to_string());
                rule_row(ui, "Content identity", &checkpoint.digest().to_string());
                note_line(
                    ui,
                    "Resume continues from this accepted state. Results begin no earlier than \
                     this time; the analysis Start time still applies. Earlier waveform samples \
                     remain in the original run. Set the stop time later than the checkpoint and \
                     keep compatible circuit and solver settings.",
                    Tone::Dim,
                );
                if Button::new("Export checkpoint…").show(ui).clicked() {
                    export(ctx, &mut app.state, checkpoint);
                }
            });
    }
    if !open {
        ctx.data_mut(|data| data.remove::<Inspection>(inspection_id()));
    }
}

#[cfg(test)]
mod tests;
