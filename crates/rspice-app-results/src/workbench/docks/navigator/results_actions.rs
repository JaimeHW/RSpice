//! What a data-browser row does when the reader acts on it: its tick, the
//! keyboard, selection, and the context menus.

use crate::state::command_catalog::CommandKey;
use crate::workbench::documents::result_document::browser_plot::{
    EVENT_NET_ROW_PREFIX, result_quantity_ticked, set_result_signal_plotted,
};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsParts, compare_availability, exact_result_artifact_text,
    exact_result_signal_last_sample, exact_result_signal_tsv,
};
/// Re-exported where the browser's own actions are found: a tick is how a
/// row is plotted.
pub use crate::workbench::documents::result_document::{
    set_result_quantities_ticked, set_result_quantity_ticked,
};
use crate::workbench::framework::feature::{Execute, held_back};
use crate::workbench::framework::requests::ExportResultQuantities;
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use super::super::super::commands::result_navigation::{open_run_log, producing_run};
use super::super::super::commands::schematic_cross_probe::{self, schematic_refusal};
use super::results::{RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT, ResultArtifact};
use super::results_rows::VisibleKeys;
use rspice_app_workbench::workbench::docks::navigator::row_context_menu;

/// A tick on a row, or a tick drawn across a range of rows with Shift.
pub(super) fn update_result_browser_multi_selection(
    state: &mut ResultsParts<'_>,
    target: &ResultBrowserSelectionKey,
    ordered_visible: &VisibleKeys<'_>,
    modifiers: egui::Modifiers,
) {
    if modifiers.shift {
        tick_result_range(state, target, ordered_visible.keys());
    } else {
        let ticked = result_quantity_ticked(state, target);
        set_result_quantity_ticked(state, target, !ticked);
        state
            .results
            .session
            .set_browser_range_anchor(target.clone());
    }
}

fn tick_result_range(
    state: &mut ResultsParts<'_>,
    target: &ResultBrowserSelectionKey,
    ordered_visible: &[ResultBrowserSelectionKey],
) {
    let range = state.results.session.browser_range(target, ordered_visible);
    set_result_quantities_ticked(state, &range, true);
}

/// A double click on a row: the signal goes on the plot and its statistics
/// come up in the Inspector.
pub(super) fn plot_and_inspect_signal(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
) {
    select_result_signal_by_key(state, key);
    set_result_signal_plotted(state, key, true);
    state.workbench.inspector_visible = true;
}

pub(super) fn result_signal_row_id(key: &ResultBrowserSelectionKey) -> egui::Id {
    egui::Id::new(("workbench.result-browser.signal", key))
}

pub(super) fn handle_result_signal_keyboard(
    response: &egui::Response,
    state: &mut ResultsParts<'_>,
    current: &SourceWaveformPresentationKey,
    visible: &VisibleKeys<'_>,
) {
    if !response.has_focus() {
        return;
    }
    let modifiers = response.ctx.input(|input| input.modifiers);
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Space))
    {
        update_result_browser_multi_selection(
            state,
            &ResultBrowserSelectionKey::Waveform(current.clone()),
            visible,
            modifiers,
        );
        return;
    }
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
    {
        open_signal_table(state, current, false);
        return;
    }
    let direction = if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowUp)
    }) {
        -1_isize
    } else if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowDown)
    }) {
        1_isize
    } else {
        return;
    };
    let current_key = ResultBrowserSelectionKey::Waveform(current.clone());
    let ordered_visible = visible.keys();
    let Some(index) = ordered_visible.iter().position(|key| key == &current_key) else {
        return;
    };
    let next_index = index
        .saturating_add_signed(direction)
        .min(ordered_visible.len().saturating_sub(1));
    let Some(next) = ordered_visible.get(next_index) else {
        return;
    };
    if modifiers.shift {
        tick_result_range(state, next, ordered_visible);
    } else {
        state.results.session.set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(state, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

pub(super) fn handle_result_artifact_keyboard(
    response: &egui::Response,
    state: &mut ResultsParts<'_>,
    artifact: &ResultArtifact,
    visible: &VisibleKeys<'_>,
) {
    if !response.has_focus() {
        return;
    }
    let current = ResultBrowserSelectionKey::Artifact(artifact.identity.clone());
    let modifiers = response.ctx.input(|input| input.modifiers);
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Space))
    {
        update_result_browser_multi_selection(state, &current, visible, modifiers);
        return;
    }
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
    {
        open_result_artifact(state, artifact, false);
        return;
    }
    let direction = if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowUp)
    }) {
        -1_isize
    } else if response.ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
            || input.consume_key(egui::Modifiers::SHIFT, egui::Key::ArrowDown)
    }) {
        1_isize
    } else {
        return;
    };
    let ordered_visible = visible.keys();
    let Some(index) = ordered_visible.iter().position(|key| key == &current) else {
        return;
    };
    let next_index = index
        .saturating_add_signed(direction)
        .min(ordered_visible.len().saturating_sub(1));
    let Some(next) = ordered_visible.get(next_index) else {
        return;
    };
    if modifiers.shift {
        tick_result_range(state, next, ordered_visible);
    } else {
        state.results.session.set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(state, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

pub fn select_result_dataset(state: &mut ResultsParts<'_>, run_index: usize) -> bool {
    let Some(dataset_id) = state
        .model
        .simulation
        .retained
        .runs
        .get(run_index)
        .map(|run| run.dataset_id)
    else {
        return false;
    };
    // The browser picks the run Results shows, an analysis or a signal of it
    // as much as the run's own row.
    if crate::workbench::chrome::document_bar::results::show_run(state, dataset_id).is_err() {
        return false;
    }
    crate::workbench::documents::result_document::open_shown_run(state);
    state.results.session.selected_trace = None;
    state.results.session.selected_result_artifact = None;
    true
}

/// Show analysis `analysis_index` of run `run_index`: the data browser picks
/// what is shown, and the canvas follows on the next frame, staying on its
/// sheet when the analysis has that view and else opening on the analysis'
/// default sheet. The run's view keeps the pick for the plan's next run.
pub fn select_result_analysis(
    state: &mut ResultsParts<'_>,
    run_index: usize,
    analysis_index: usize,
) -> bool {
    if state
        .model
        .simulation
        .retained
        .runs
        .get(run_index)
        .and_then(|run| run.analyses.get(analysis_index))
        .is_none()
    {
        return false;
    }
    if !select_result_dataset(state, run_index) {
        return false;
    }
    if !state.model.simulation.select_analysis(analysis_index) {
        return false;
    }
    crate::workbench::documents::result_document::keep_picked_analysis(state);
    state.results.session.selected_trace = None;
    state.results.session.selected_result_artifact = None;
    true
}

pub(super) fn select_result_analysis_by_key(
    state: &mut ResultsParts<'_>,
    key: AnalysisPresentationKey,
) -> bool {
    let Some((run_index, analysis_index)) = state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .enumerate()
        .find_map(|(run_index, run)| {
            key.resolve(run)
                .map(|(analysis_index, _)| (run_index, analysis_index))
        })
    else {
        return false;
    };
    select_result_analysis(state, run_index, analysis_index)
}

pub fn select_result_signal(
    state: &mut ResultsParts<'_>,
    run_index: usize,
    analysis_index: usize,
    waveform_index: usize,
) -> bool {
    let Some(run) = state.model.simulation.retained.runs.get(run_index) else {
        return false;
    };
    let Some(selected) = rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
        run,
        analysis_index,
        waveform_index,
    ) else {
        return false;
    };
    if !select_result_analysis(state, run_index, analysis_index) {
        return false;
    }
    // Document/run activation advances the canonical simulation data version.
    // Reconcile the viewer before installing the exact trace selection so the
    // next Results frame cannot clear the selection as if it belonged to the
    // previously active dataset.
    crate::workbench::documents::result_document::prepare_viewer_state(state);
    let name = selected.source_name().to_owned();
    let recent = SourceWaveformPresentationKey::new(selected.analysis_key(), name);
    state.results.session.selected_trace = Some(selected);
    state.results.session.selected_result_artifact = None;
    // Selecting a trace is a deliberate act; feed the browser's Recent scope.
    state.results.session.note_recent_signal(recent);
    true
}

pub(super) fn select_result_signal_by_key(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&state.model.simulation.retained.runs)
    else {
        return false;
    };
    select_result_signal(state, run_index, analysis_index, waveform_index)
}

pub(super) fn select_result_artifact(
    state: &mut ResultsParts<'_>,
    key: &ResultArtifactPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, _)) = key.resolve(&state.model.simulation.retained.runs)
    else {
        return false;
    };
    if !select_result_analysis(state, run_index, analysis_index) {
        return false;
    }
    state.results.session.selected_trace = None;
    state.results.session.selected_result_artifact = Some(key.clone());
    state
        .results
        .session
        .note_recent_result_artifact(key.clone());
    true
}

pub(super) fn select_result_browser_key(
    state: &mut ResultsParts<'_>,
    key: &ResultBrowserSelectionKey,
) -> bool {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => select_result_signal_by_key(state, key),
        ResultBrowserSelectionKey::Artifact(key) => select_result_artifact(state, key),
    }
}

pub(super) fn result_artifact_context_menu(
    response: &egui::Response,
    state: &mut ResultsParts<'_>,
    artifact: &ResultArtifact,
) {
    let (popup, _) = row_context_menu(response);
    // Validated only while the menu is open: the check reads every retained
    // sample of the run, and each row offers this menu on every frame.
    popup.show(|ui| {
        let exact_error =
            crate::workbench::documents::result_document::result_browser_selection_evidence(
                &ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
                state.results,
                state.model,
            )
            .err();
        let exact_available = exact_error.is_none();
        let open_refusal = viewer_refusal(state, artifact, artifact.viewer);
        let open = ui.add_enabled(
            exact_available && open_refusal.is_none(),
            egui::Button::new("Open in active result pane"),
        );
        if let Some(reason) = exact_error.as_deref().or(open_refusal) {
            open.clone().on_disabled_hover_text(reason);
        }
        if open.clicked() {
            open_result_artifact(state, artifact, false);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_artifact(state, &artifact.identity) {
                state.results.session.viewer = artifact.viewer;
                crate::workbench::documents::result_document::add_selected_result_as_pane(state);
            }
            ui.close();
        }
        let table_refusal = viewer_refusal(state, artifact, ResultViewer::Table);
        let table = ui.add_enabled(
            exact_available && table_refusal.is_none(),
            egui::Button::new("Open as table"),
        );
        if let Some(reason) = exact_error.as_deref().or(table_refusal) {
            table.clone().on_disabled_hover_text(reason);
        }
        if table.clicked() {
            if select_result_artifact(state, &artifact.identity) {
                execute(
                    state,
                    ResultsCommand::ResultViewer(ResultViewer::Table).key(),
                );
            }
            ui.close();
        }
        let compare_enabled = exact_available && compare_enabled(state);
        let compare = ui.add_enabled(
            compare_enabled,
            egui::Button::new("Compare selected with dataset..."),
        );
        if !compare_enabled {
            compare
                .clone()
                .on_disabled_hover_text(exact_error.as_deref().unwrap_or(
                "Retain a compatible second dataset and open a result document before comparing.",
            ));
        }
        if compare.clicked() {
            select_result_artifact(state, &artifact.identity);
            execute(state, ResultsCommand::CompareResultDatasets.key());
            ui.close();
        }
        ui.separator();
        if ui.button("Copy name").clicked() {
            ui.ctx()
                .copy_text(artifact.identity.canonical_name().to_owned());
            ui.close();
        }
        let copy_exact = ui.add_enabled(exact_available, egui::Button::new("Copy value"));
        if let Some(error) = exact_error.as_deref() {
            copy_exact.clone().on_disabled_hover_text(error);
        }
        if copy_exact.clicked() {
            match exact_result_artifact_text(
                &artifact.identity,
                &state.model.simulation.retained.runs,
            ) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => state.workbench.prompt(error),
            }
            ui.close();
        }
        if super::results_rows::is_plottable_artifact(artifact.identity.canonical_name()) {
            super::results_sets::add_to_set_menu(
                ui,
                state,
                &ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            );
        }
        let export = ui.add_enabled(exact_available, egui::Button::new("Export\u{2026}"));
        if let Some(error) = exact_error.as_deref() {
            export.clone().on_disabled_hover_text(error);
        }
        if export.clicked() {
            open_result_artifact(state, artifact, true);
            ui.close();
        }
        // An event net's lane is drawn by its conductor, as a waveform's
        // net is.
        if let Some(net) = artifact
            .identity
            .canonical_name()
            .strip_prefix(EVENT_NET_ROW_PREFIX)
        {
            show_on_schematic_item(ui, state, net);
        }
        // Offered only while the session holds the run that produced it.
        if let Some(run) = producing_run(
            &state.model.simulation.retained.runs,
            artifact.identity.analysis().dataset_id(),
        ) && ui.button("Open run log").clicked()
        {
            open_run_log(state.model, state.workbench, run);
            ui.close();
        }
    });
}

/// Why `viewer` would not open on `artifact`'s analysis, as the viewer's
/// command answers it, when the artifact belongs to the run shown; `None`
/// when it would. For another run the command answers once the row has
/// shown its run, and says why on the status line if it cannot open.
pub(super) fn viewer_refusal(
    state: &ResultsParts<'_>,
    artifact: &ResultArtifact,
    viewer: ResultViewer,
) -> Option<&'static str> {
    let (run, _, _) = artifact
        .identity
        .resolve(&state.model.simulation.retained.runs)?;
    if state.model.simulation.view.active_run_idx != Some(run) {
        return None;
    }
    crate::workbench::documents::result_document::viewer_unavailability_reason(
        state.results_reads(),
        viewer,
    )
}

fn open_result_artifact(
    state: &mut ResultsParts<'_>,
    artifact: &ResultArtifact,
    request_export: bool,
) {
    if select_result_artifact(state, &artifact.identity) {
        if request_export {
            state.workbench.outbox.push(ExportResultQuantities(vec![
                ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            ]));
        } else {
            execute(state, ResultsCommand::ResultViewer(artifact.viewer).key());
        }
    }
}

pub(super) fn result_signal_context_menu(
    response: &egui::Response,
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    visible: bool,
) {
    let key = key.clone();
    let (popup, _) = row_context_menu(response);
    // Validated only while the menu is open, as the artifact menu is.
    popup.show(|ui| {
        let exact_error =
            crate::workbench::documents::result_document::result_browser_selection_evidence(
                &ResultBrowserSelectionKey::Waveform(key.clone()),
                state.results,
                state.model,
            )
            .err();
        let exact_available = exact_error.is_none();
        let membership = ui.add_enabled(
            exact_available,
            egui::Button::new(if visible {
                "Remove from active pane"
            } else {
                "Add to active pane"
            }),
        );
        if let Some(error) = exact_error.as_deref() {
            membership.clone().on_disabled_hover_text(error);
        }
        if membership.clicked() {
            set_result_signal_plotted(state, &key, !visible);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_signal_by_key(state, &key) {
                crate::workbench::documents::result_document::add_selected_result_as_pane(state);
            }
            ui.close();
        }
        let create = ui.add_enabled(
            exact_available,
            egui::Button::new("New result document…"),
        );
        if let Some(error) = exact_error.as_deref() {
            create.clone().on_disabled_hover_text(error);
        }
        if create.clicked() {
            // Results shows the signal's run, and the form opens on its
            // analysis with the signal ticked.
            select_result_signal_by_key(state, &key);
            crate::workbench::documents::result_document::create_document::open_on(
                state,
                key.analysis(),
                &[key.source_name().to_owned()],
            );
            ui.close();
        }
        ui.separator();
        taken_from_signal_rows(ui, state, &key);
        ui.separator();
        let table = ui.add_enabled(exact_available, egui::Button::new("Open as table"));
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
        }
        if table.clicked() {
            open_signal_table(state, &key, false);
            ui.close();
        }
        let export = ui.add_enabled(
            exact_available,
            egui::Button::new("Export source analysis (CSV)..."),
        );
        if let Some(error) = exact_error.as_deref() {
            export.clone().on_disabled_hover_text(error);
        }
        if export.clicked() {
            open_signal_table(state, &key, true);
            ui.close();
        }
        let compare_enabled = exact_available && compare_enabled(state);
        let compare = ui.add_enabled(
            compare_enabled,
            egui::Button::new("Compare with dataset..."),
        );
        if !compare_enabled {
            compare.clone().on_disabled_hover_text(
                exact_error.as_deref().unwrap_or(
                    "Retain a compatible second dataset and open a result document before comparing.",
                ),
            );
        }
        if compare.clicked() {
            select_result_signal_by_key(state, &key);
            execute(state, ResultsCommand::CompareResultDatasets.key());
            ui.close();
        }
        ui.separator();
        if ui.button("Copy name").clicked() {
            if let Some((.., waveform)) = key.resolve(&state.model.simulation.retained.runs) {
                ui.ctx().copy_text(waveform.name.clone());
            }
            ui.close();
        }
        let copy_last = ui.add_enabled(
            exact_available,
            egui::Button::new("Copy value"),
        );
        if let Some(error) = exact_error.as_deref() {
            copy_last.clone().on_disabled_hover_text(error);
        }
        if copy_last.clicked() {
            match exact_result_signal_last_sample(
                &key,
                &state.model.simulation.retained.runs,
                state.model.preferences.quantity_presentation_policy(),
            ) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => state.workbench.prompt(error),
            }
            ui.close();
        }
        let retained_samples = key
            .resolve(&state.model.simulation.retained.runs)
            .map_or(0, |(.., waveform)| waveform.x.len().max(waveform.y.len()));
        let may_copy_samples =
            exact_available && retained_samples <= RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT;
        let copy_samples = ui.add_enabled(
            may_copy_samples,
            egui::Button::new("Copy samples"),
        );
        if !may_copy_samples {
            copy_samples.clone().on_disabled_hover_text(
                exact_error.clone().unwrap_or_else(|| format!(
                    "This quantity holds {retained_samples} samples; clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT}. Export it as CSV instead."
                )),
            );
        }
        if copy_samples.clicked() {
            match exact_result_signal_tsv(&key, &state.model.simulation.retained.runs) {
                Ok(tsv) => ui.ctx().copy_text(tsv),
                Err(error) => state.workbench.prompt(error),
            }
            ui.close();
        }
        ui.separator();
        super::results_sets::add_to_set_menu(
            ui,
            state,
            &ResultBrowserSelectionKey::Waveform(key.clone()),
        );
        match key
            .resolve(&state.model.simulation.retained.runs)
            .map(|(.., waveform)| waveform.name.clone())
        {
            Some(signal) => show_on_schematic_item(ui, state, &signal),
            None => {
                ui.add_enabled(false, egui::Button::new("Show on schematic"))
                    .on_disabled_hover_text("This signal is no longer in its run.");
            }
        }
        // Offered only while the session holds the run that produced it.
        if let Some(run) = producing_run(
            &state.model.simulation.retained.runs,
            key.analysis().dataset_id(),
        ) && ui.button("Open run log").clicked()
        {
            open_run_log(state.model, state.workbench, run);
            ui.close();
        }
    });
}

fn open_signal_table(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    request_export: bool,
) {
    if select_result_signal_by_key(state, key) {
        if request_export {
            state.workbench.outbox.push(ExportResultQuantities(vec![
                ResultBrowserSelectionKey::Waveform(key.clone()),
            ]));
        } else {
            if let Some(selected) = &state.results.session.selected_trace {
                let (analysis, column) = selected.table_binding();
                state.results.session.table.analysis = Some(analysis);
                state.results.session.table.columns = vec![column];
            }
            execute(
                state,
                ResultsCommand::ResultViewer(ResultViewer::Table).key(),
            );
        }
    }
}

/// Show what carries `signal` on the schematic, or say on the status line
/// why nothing does.
fn show_signal_on_schematic(state: &mut ResultsParts<'_>, signal: &str) {
    if let Err(reason) = schematic_cross_probe::show_signal_on_schematic(state, signal) {
        state.workbench.prompt(reason);
    }
}

/// The other direction of the probe loop: from a trace back to the conductor
/// that produced it.
///
/// A derived signal explains itself rather than pretending to be a net, and
/// a result whose drawing has since changed says so instead of selecting
/// geometry that no longer means the same thing.
pub(super) fn locate_on_schematic_menu(
    response: &egui::Response,
    state: &mut ResultsParts<'_>,
    signal: &str,
) {
    response.context_menu(|ui| show_on_schematic_item(ui, state, signal));
}

/// The menu row that shows `signal` on the schematic: disabled, saying why,
/// when no one net or part carries it.
fn show_on_schematic_item(ui: &mut egui::Ui, state: &mut ResultsParts<'_>, signal: &str) {
    let refusal = schematic_refusal(signal);
    let show = ui.add_enabled(refusal.is_none(), egui::Button::new("Show on schematic"));
    match refusal {
        Some(refusal) => {
            show.on_disabled_hover_text(refusal);
        }
        None if show.clicked() => {
            show_signal_on_schematic(state, signal);
            ui.close();
        }
        None => {}
    }
}

/// Run the command `key` names once the data browser's contribution
/// returns, behind the prologue every command shares.
pub(super) fn execute(state: &mut ResultsParts<'_>, key: CommandKey) {
    state.workbench.outbox.push(Execute(key));
}

/// Whether Compare datasets would run if raised now, as the dispatcher
/// answers it: its prologue first, then whether there is a baseline to
/// compare the result with.
pub(super) fn compare_enabled(state: &ResultsParts<'_>) -> bool {
    held_back(
        ResultsCommand::CompareResultDatasets.declared_spec().flags,
        state.platform,
    )
    .is_none()
        && compare_availability(state.results_reads()).is_available()
}

/// The signal menu's rows that take something from the signal: the
/// calculator holding it, its spectrum, and a plot of it against another
/// signal of its analysis. Each is offered only where it will act.
fn taken_from_signal_rows(
    ui: &mut egui::Ui,
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
) {
    let runs = &state.model.simulation.retained.runs;
    let Some((run_index, analysis_index, _, _)) = key.resolve(runs) else {
        return;
    };
    let analysis = &runs[run_index].analyses[analysis_index];
    let in_time = analysis.analysis_type.is_time_domain();
    let against_refusal = plot_against_refusal(analysis.analysis_type);
    let others: Vec<String> = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.clone())
        .filter(|name| name != key.source_name())
        .collect();
    if ui.button("Send to calculator").clicked() {
        if select_result_signal_by_key(state, key) {
            execute(state, ResultsCommand::SendTraceToCalculator.key());
        }
        ui.close();
    }
    let spectrum = ui.add_enabled(in_time, egui::Button::new("Spectrum"));
    if !in_time {
        spectrum
            .clone()
            .on_disabled_hover_text("A spectrum is taken of a transient waveform.");
    }
    if spectrum.clicked() {
        if select_result_signal_by_key(state, key) {
            execute(state, ResultsCommand::TraceSpectrum.key());
        }
        ui.close();
    }
    let refusal = against_refusal.or(others
        .is_empty()
        .then_some("Its analysis holds no other signal to plot it against."));
    match refusal {
        Some(reason) => {
            ui.add_enabled(false, egui::Button::new("Plot against"))
                .on_disabled_hover_text(reason);
        }
        None => {
            rspice_ui_kit::widgets::menu_button(ui, "Plot against", |ui| {
                let t = crate::ui::tokens::Tokens::get(ui.ctx());
                egui::ScrollArea::vertical()
                    .max_height(t.metrics.row_h * 16.0)
                    .show(ui, |ui| {
                        for name in &others {
                            if ui.button(name).clicked() {
                                if let Err(error) = plot_against(state, key, name) {
                                    state.workbench.prompt(error);
                                }
                                ui.close();
                            }
                        }
                    });
            });
        }
    }
}

/// Why a signal of an analysis of `kind` cannot be drawn against another,
/// if it cannot: a transfer curve, a hysteresis loop or a Lissajous figure
/// reads real samples of one sweep.
fn plot_against_refusal(kind: crate::state::AnalysisType) -> Option<&'static str> {
    (!kind.is_time_domain() && kind != crate::state::AnalysisType::DcSweep)
        .then_some("A signal is plotted against another in a transient or DC sweep.")
}

/// Draw the pane `key`'s signal is on against `abscissa`, another signal of
/// its analysis, in a strip of its own after the analysis' strip, as the
/// pane's own "Plot against" and ViVA's YvsY do. The signal is plotted first
/// if it was not, and its run shown.
pub(super) fn plot_against(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    abscissa: &str,
) -> Result<(), String> {
    let runs = &state.model.simulation.retained.runs;
    let (run_index, analysis_index, _, waveform) = key
        .resolve(runs)
        .ok_or("The signal is no longer in its run.")?;
    let analysis = &runs[run_index].analyses[analysis_index];
    if let Some(reason) = plot_against_refusal(analysis.analysis_type) {
        return Err(reason.to_owned());
    }
    if !analysis
        .waveforms
        .iter()
        .any(|waveform| waveform.name == abscissa)
    {
        return Err(format!("{abscissa} is not a signal of this analysis."));
    }
    let unit = rspice_results_ui::waves::browser_signal_unit(
        &waveform.name,
        waveform.unit.as_deref(),
        rspice_results_ui::waves::analysis_default_unit(analysis.analysis_type),
    );
    // A plain number's pane has no unit.
    let unit = if unit == "1" { "" } else { unit }.to_owned();
    set_result_signal_plotted(state, key, true);
    select_result_signal_by_key(state, key);
    let analysis = key.analysis();
    let abscissa = rspice_results::visualization_document::Abscissa::Signal {
        name: abscissa.to_owned(),
    };
    let strips = &mut state.results.session.xy_strips;
    if !strips
        .iter()
        .any(|strip| strip.is(analysis, &unit, &abscissa))
    {
        strips.push(rspice_results_ui::session::XyStrip::new(
            analysis, unit, abscissa,
        ));
    }
    execute(
        state,
        ResultsCommand::ResultViewer(ResultViewer::Waves).key(),
    );
    Ok(())
}
