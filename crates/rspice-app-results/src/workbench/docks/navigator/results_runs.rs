//! What a run's row in the data browser offers: show it, overlay it, name
//! it, write a note on it, pin it and delete it, as Assembler's History tab
//! offers a history item View Results, Rename, Notes, Lock and Delete. A
//! pinned run is what Assembler calls a locked one: retention never
//! discards it, and it cannot be deleted until it is unpinned.

use egui::Ui;

use crate::product::DatasetId;
use crate::state::{RunRetention, SimulationRun, SimulationState};
use crate::ui::theme;
use crate::workbench::documents::result_document::ResultsParts;

/// The reason Delete and Restore give while a run owns the history.
const RUNNING: &str = "A run is in progress; the history can change when it ends.";

fn run<'a>(state: &'a ResultsParts<'_>, dataset: DatasetId) -> Result<&'a SimulationRun, String> {
    state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .ok_or_else(|| "That run is no longer in the run history.".to_owned())
}

fn run_mut<'a>(
    state: &'a mut ResultsParts<'_>,
    dataset: DatasetId,
) -> Result<&'a mut SimulationRun, String> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter_mut()
        .find(|run| run.dataset_id == dataset)
        .ok_or_else(|| "That run is no longer in the run history.".to_owned())
}

/// Name the run `name`; its label goes on carrying the date and time it
/// started.
pub fn rename_run(
    state: &mut ResultsParts<'_>,
    dataset: DatasetId,
    name: &str,
) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("A run needs a name.".to_owned());
    }
    run_mut(state, dataset)?.rename(name);
    Ok(())
}

/// Write `note` on the run; an empty note removes the one it had.
pub fn write_run_note(
    state: &mut ResultsParts<'_>,
    dataset: DatasetId,
    note: &str,
) -> Result<(), String> {
    run_mut(state, dataset)?.note = note.trim().to_owned();
    Ok(())
}

/// Why the run cannot be released from its pin, if it cannot: the plan's
/// regression checks compare against it.
fn release_refusal(state: &ResultsParts<'_>, run: &SimulationRun) -> Option<&'static str> {
    let plan = run.prepared_receipt()?.simulation_plan_id()?;
    let baseline = state
        .model
        .workspace
        .content
        .plan_data(plan)?
        .regression_baseline_run?;
    (baseline == run.run_id).then_some(
        "This run is the plan's regression baseline; choose another in Verify to release it.",
    )
}

/// Pin the run, so retention never discards it, or release it.
pub fn set_run_pinned(
    state: &mut ResultsParts<'_>,
    dataset: DatasetId,
    pinned: bool,
) -> Result<(), String> {
    let run = run(state, dataset)?;
    if !pinned && let Some(reason) = release_refusal(state, run) {
        return Err(reason.to_owned());
    }
    let run_id = run.run_id;
    let retention = if pinned {
        RunRetention::GoldenBaseline
    } else {
        RunRetention::Pruneable
    };
    state.model.simulation.set_run_retention(run_id, retention);
    Ok(())
}

/// Why the run cannot be deleted, if it cannot.
pub fn delete_refusal(state: &ResultsParts<'_>, dataset: DatasetId) -> Option<String> {
    if state.model.simulation.execution.has_active_execution() {
        return Some(RUNNING.to_owned());
    }
    match run(state, dataset) {
        Ok(run) if run.retention().is_pinned() => {
            Some("The run is pinned; unpin it to delete it.".to_owned())
        }
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

/// Delete the run into the holding area, from which it can be restored
/// until the project closes. Returns what the status bar says.
pub fn delete_run(state: &mut ResultsParts<'_>, dataset: DatasetId) -> Result<String, String> {
    if let Some(reason) = delete_refusal(state, dataset) {
        return Err(reason);
    }
    let label = run(state, dataset)?.label.clone();
    state
        .model
        .simulation
        .delete_runs(|run| run.dataset_id != dataset);
    Ok(format!(
        "Deleted {label}. Restore deleted runs brings it back until the project closes."
    ))
}

/// Why the deleted runs cannot be restored, if they cannot, as a menu row
/// says it.
pub fn restore_refusal(simulation: &SimulationState) -> Option<&'static str> {
    if simulation.retained.deleted_runs.is_empty() {
        Some("no run has been deleted since the project opened")
    } else if simulation.execution.has_active_execution() {
        Some("a run is in progress; the history can change when it ends")
    } else {
        None
    }
}

/// Put every run deleted since the project opened back into the history.
/// Returns what the status bar says.
pub fn restore_deleted_runs(state: &mut ResultsParts<'_>) -> Result<String, String> {
    if let Some(reason) = restore_refusal(&state.model.simulation) {
        return Err(format!("Could not restore deleted runs: {reason}."));
    }
    let restored = state.model.simulation.restore_deleted_runs();
    Ok(format!(
        "Restored {restored} {}.",
        if restored == 1 { "run" } else { "runs" }
    ))
}

/// A menu item that acts when enabled, and says why when not.
fn item(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    let response = ui.add_enabled(refusal.is_none(), egui::Button::new(label));
    match refusal {
        Some(reason) => {
            response.on_disabled_hover_text(reason);
            false
        }
        None => response.clicked(),
    }
}

/// A one-line field on the menu that commits on Enter, its draft kept while
/// the menu stays open.
fn committed_field(
    ui: &mut Ui,
    id: egui::Id,
    current: &str,
    hint: &str,
    hover: &str,
) -> Option<String> {
    let mut draft = ui
        .ctx()
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| current.to_owned());
    let field = ui
        .add(
            egui::TextEdit::singleline(&mut draft)
                .font(theme::field_sans())
                .hint_text(hint)
                .desired_width(220.0),
        )
        .on_hover_text(hover);
    if field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        ui.ctx().data_mut(|data| data.remove::<String>(id));
        return Some(draft);
    }
    ui.ctx().data_mut(|data| data.insert_temp(id, draft));
    None
}

/// The run operations on a run row's menu, above the details and the log.
/// What happened, or why it could not, goes to the status bar.
pub fn run_menu(ui: &mut Ui, state: &mut ResultsParts<'_>, dataset: DatasetId) {
    let Ok(row) = run(state, dataset) else {
        return;
    };
    let shown = state
        .model
        .simulation
        .active_run()
        .map(|run| run.dataset_id)
        == Some(dataset);
    let (name, note, pinned) = (
        row.name().to_owned(),
        row.note.clone(),
        row.retention().is_pinned(),
    );
    // A saved document shows its own run; Show goes back to the browser's
    // view of it.
    let in_view = crate::workbench::chrome::document_bar::results::active_result_document(
        state.results_reads(),
    ) == Some(crate::workbench::state::WorkspaceDocumentId::ResultDataset(
        dataset,
    ));
    let overlaid = state.model.simulation.is_dataset_overlaid(dataset);
    let release = pinned.then(|| release_refusal(state, row)).flatten();
    let mut said: Option<String> = None;

    if item(ui, "Show", in_view.then_some("Results shows this run.")) {
        said = crate::workbench::chrome::document_bar::results::show_run(state, dataset)
            .err()
            .map(|error| format!("Could not show {name}: {error}"));
        ui.close();
    }
    let overlay_label = if overlaid {
        "Remove overlay"
    } else {
        "Overlay on the run shown"
    };
    if item(ui, overlay_label, shown.then_some("This is the run shown.")) {
        let enabled = state.model.simulation.toggle_dataset_overlay(dataset);
        said = Some(if enabled {
            format!("Overlaying {name} on the run shown.")
        } else {
            format!("Removed {name} from the run shown.")
        });
        ui.close();
    }
    ui.separator();
    let field = ui.id().with(("workbench.results.run-name", dataset));
    if let Some(draft) = committed_field(
        ui,
        field,
        &name,
        "Name",
        "Rename: type a name and press Enter",
    ) {
        said = Some(match rename_run(state, dataset, &draft) {
            Ok(()) => format!("Renamed {name} to {}.", draft.trim()),
            Err(error) => error,
        });
        ui.close();
    }
    let field = ui.id().with(("workbench.results.run-note", dataset));
    if let Some(draft) = committed_field(
        ui,
        field,
        &note,
        "Note",
        "Note: type a note and press Enter; an empty note removes it",
    ) {
        said = Some(match write_run_note(state, dataset, &draft) {
            Ok(()) if draft.trim().is_empty() => format!("Removed the note on {name}."),
            Ok(()) => format!("Noted {name}."),
            Err(error) => error,
        });
        ui.close();
    }
    if item(ui, if pinned { "Unpin" } else { "Pin" }, release) {
        said = Some(match set_run_pinned(state, dataset, !pinned) {
            Ok(()) if pinned => format!("Unpinned {name}: retention may discard it."),
            Ok(()) => format!("Pinned {name}: retention never discards it."),
            Err(error) => error,
        });
        ui.close();
    }
    ui.separator();
    let refusal = delete_refusal(state, dataset);
    if item(ui, "Delete run", refusal.as_deref()) {
        said = Some(delete_run(state, dataset).unwrap_or_else(|error| error));
        ui.close();
    }
    let held = state.model.simulation.retained.deleted_runs.len();
    if held > 0
        && item(
            ui,
            &format!("Restore deleted runs ({held})"),
            restore_refusal(&state.model.simulation),
        )
    {
        said = Some(restore_deleted_runs(state).unwrap_or_else(|error| error));
        ui.close();
    }
    if let Some(text) = said {
        state.workbench.prompt(text);
    }
}
