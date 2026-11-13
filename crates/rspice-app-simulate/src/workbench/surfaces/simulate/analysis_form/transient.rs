//! The transient form: the window the run walks, and whether it starts from
//! the deck's initial conditions rather than the operating point.
//!
//! Max step is the one row with two spellings. `auto` is a word rather than a
//! time, so it is edited as text and only a stated bound is normalized as a
//! quantity: a field that rewrote `auto` into a number would be answering a
//! question the reader deliberately left to the solver.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::workbench::app_state::TranSetup;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, input_row, quantity_input_row, switch_row,
};

/// Render the transient fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut TranSetup,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Stop time",
        &mut setup.stop,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Step time",
        &mut setup.step,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Start time",
        &mut setup.start,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    if !setup.max_step.eq_ignore_ascii_case("auto") {
        quantity_input_row(
            ui,
            "Max step",
            &mut setup.max_step,
            QuantityInputKind::Time,
            policy,
            locale,
        );
    } else {
        input_row(ui, "Max step", &mut setup.max_step);
    }
    switch_row(ui, "Use initial conditions", &mut setup.uic);
    let mut capture = setup
        .checkpoint
        .as_ref()
        .is_some_and(|draft| draft.interval.is_some());
    if switch_row(ui, "Retain checkpoints", &mut capture) {
        if capture {
            setup
                .checkpoint
                .get_or_insert_with(Default::default)
                .interval = Some("10u".into());
        } else if let Some(draft) = &mut setup.checkpoint {
            draft.interval = None;
        }
    }
    if let Some(interval) = setup
        .checkpoint
        .as_mut()
        .and_then(|draft| draft.interval.as_mut())
    {
        quantity_input_row(
            ui,
            "Checkpoint interval",
            interval,
            QuantityInputKind::Time,
            policy,
            locale,
        );
        super::field_note(
            ui,
            "Keep the latest accepted state at this interval and at the stop time. Stopping a run preserves its last completed checkpoint. Capture may occur just after a requested time.",
        );
    }
}

/// Immutable image buffers are shared while rendering; compatibility is checked
/// during preparation after the actual circuit and Run Set point are known.
pub(in crate::workbench::surfaces::simulate) fn checkpoint_sources(
    ui: &mut Ui,
    setup: &mut TranSetup,
    simulation: &crate::state::SimulationState,
    selected: crate::product::AnalysisInstanceId,
) -> Option<crate::workbench::surfaces::simulate::transient_checkpoint_exchange::Action> {
    use crate::workbench::surfaces::simulate::transient_checkpoint_exchange::Action;
    let mut action = None;
    super::sub_header(ui, "Resume transient");
    if super::action_line(ui, "Import transient checkpoint…") {
        action = Some(Action::Import);
    }
    let mut seen = std::collections::HashSet::new();
    let selected_images = setup
        .checkpoint
        .as_ref()
        .map_or(&[][..], |draft| draft.resume.as_slice());
    let candidates = simulation
        .retained
        .runs
        .iter()
        .rev()
        .flat_map(|run| {
            run.analyses.iter().filter_map(move |analysis| {
                let checkpoint = analysis.transient_checkpoint.as_ref()?;
                // Cloning an analysis preserves its selected input. Keep that
                // image visible even when it belongs to the original analysis.
                if analysis.provenance()?.authored_source_instance_id() != selected
                    && !selected_images.contains(&checkpoint.digest())
                {
                    return None;
                }
                Some((
                    format!(
                        "{} · {} · {:.6e} s",
                        run.label,
                        analysis.label,
                        checkpoint.accepted_time()
                    ),
                    checkpoint.clone(),
                ))
            })
        })
        .chain(
            simulation
                .retained
                .imported_transient_checkpoints
                .iter()
                .map(|entry| {
                    (
                        format!(
                            "Imported · {} · {:.6e} s",
                            entry.name(),
                            entry.checkpoint().accepted_time()
                        ),
                        entry.checkpoint().clone(),
                    )
                }),
        )
        .filter(|(_, image)| seen.insert(image.digest()))
        .collect::<Vec<_>>();
    super::field_note(
        ui,
        if candidates.is_empty() {
            "No checkpoints available. Enable retention for a run or import a checkpoint file."
        } else {
            "Select one compatible checkpoint for each requested Run Set point. Every point must match exactly one selection. Set stop time after its accepted time. Resumed results contain the continuation; earlier samples remain in the original run. Imported checkpoints are saved with the project."
        },
    );
    for (label, checkpoint) in candidates {
        let digest = checkpoint.digest();
        let mut chosen = setup
            .checkpoint
            .as_ref()
            .is_some_and(|draft| draft.resume.contains(&digest));
        ui.push_id(digest, |ui| {
            if switch_row(ui, &label, &mut chosen) {
                let draft = setup.checkpoint.get_or_insert_with(|| {
                    rspice_simulation_contract::tran_checkpoint::TranCheckpointDraft {
                        interval: None,
                        resume: Vec::new(),
                    }
                });
                if chosen {
                    draft.resume.push(digest);
                } else {
                    draft.resume.retain(|id| *id != digest);
                }
            }
            if super::action_line(ui, "Inspect checkpoint") {
                action = Some(Action::Inspect(label, checkpoint.clone()));
            }
            if super::action_line(ui, "Export checkpoint…") {
                action = Some(Action::Export(checkpoint.clone()));
            }
            if simulation
                .retained
                .imported_transient_checkpoints
                .get(digest)
                .is_some()
                && super::action_line(ui, "Remove imported copy")
            {
                action = Some(Action::Remove(digest));
                if let Some(draft) = &mut setup.checkpoint {
                    draft.resume.retain(|id| *id != digest);
                }
            }
        });
    }
    if let Some(draft) = &mut setup.checkpoint {
        if draft.resume.iter().any(|digest| !seen.contains(digest)) {
            super::field_note(
                ui,
                "A selected checkpoint is unavailable. Restore its run, import its file, or clear the selection.",
            );
        }
        if !draft.resume.is_empty() && super::action_line(ui, "Clear selection — start fresh") {
            draft.resume.clear();
        }
        if draft.resume.is_empty() && draft.interval.is_none() {
            setup.checkpoint = None;
        }
    }
    super::clear_pending_cell(ui);
    action
}
