//! What a data-browser row does when the reader acts on it: its tick, the
//! keyboard, selection, and the context menus.

use crate::state::command_catalog::CommandKey;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsParts, compare_availability, exact_result_artifact_text,
    exact_result_signal_last_sample, exact_result_signal_tsv, waveform_plotted,
};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Execute, held_back};
use crate::workbench::framework::requests::{ExportResultQuantities, PromptAfterCommands};
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use super::super::super::commands::result_navigation::{open_run_log, producing_run};
use super::results::{RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT, ResultArtifact};
use rspice_app_workbench::workbench::docks::navigator::row_context_menu;

/// A tick on a row, or a tick drawn across a range of rows with Shift.
pub(super) fn update_result_browser_multi_selection(
    state: &mut ResultsParts<'_>,
    target: &ResultBrowserSelectionKey,
    ordered_visible: &[ResultBrowserSelectionKey],
    modifiers: egui::Modifiers,
) {
    if modifiers.shift {
        tick_result_range(state, target, ordered_visible);
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
    for key in &range {
        set_result_quantity_ticked(state, key, true);
    }
}

/// Whether a row's tick is on. A waveform's tick is whether it is on the plot:
/// there is no second record of it that the legend or the Inspector could
/// disagree with.
fn result_quantity_ticked(state: &ResultsParts<'_>, key: &ResultBrowserSelectionKey) -> bool {
    if let ResultBrowserSelectionKey::Artifact(artifact) = key
        && let Some(nets) = event_row_nets(state, artifact)
    {
        return !nets.is_empty()
            && nets.iter().all(|net| {
                crate::workbench::documents::result_document::event_net_plotted(
                    state,
                    artifact.analysis(),
                    net,
                )
            });
    }
    match key {
        ResultBrowserSelectionKey::Waveform(key) => result_signal_ticked(state, key),
        ResultBrowserSelectionKey::Artifact(_) => state
            .results
            .session
            .checked_result_quantities
            .contains(key),
    }
}

pub fn set_result_quantity_ticked(
    state: &mut ResultsParts<'_>,
    key: &ResultBrowserSelectionKey,
    ticked: bool,
) {
    // An event net's row plots its lane; a bus row plots the bus, its
    // members, which the strip then draws as one bus lane.
    if let ResultBrowserSelectionKey::Artifact(artifact) = key
        && let Some(nets) = event_row_nets(state, artifact)
    {
        for net in &nets {
            crate::workbench::documents::result_document::set_event_net_plotted(
                state,
                artifact.analysis(),
                net,
                ticked,
            );
        }
        if ticked && !nets.is_empty() {
            reveal_strip(state, artifact.analysis());
        }
        return;
    }
    match key {
        ResultBrowserSelectionKey::Waveform(key) => set_result_signal_plotted(state, key, ticked),
        ResultBrowserSelectionKey::Artifact(_) => {
            let checked = &mut state.results.session.checked_result_quantities;
            if ticked {
                checked.insert(key.clone());
            } else {
                checked.remove(key);
            }
        }
    }
}

/// The event nets a row plots, by their `D()` or `E()` column names: an
/// event net's own, or a declared bus's members MSB first. `None` for a row
/// that names neither.
fn event_row_nets(
    state: &ResultsParts<'_>,
    artifact: &ResultArtifactPresentationKey,
) -> Option<Vec<String>> {
    let name = artifact.canonical_name();
    if let Some(net) = name.strip_prefix(super::results_rows::EVENT_NET_ROW_PREFIX) {
        return Some(vec![net.to_owned()]);
    }
    let label = name.strip_prefix(super::results_rows::BUS_ROW_PREFIX)?;
    let (_, _, analysis) = artifact.resolve(&state.model.simulation.retained.runs)?;
    let Some(crate::state::AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        analysis.result_payload.as_ref()
    else {
        return None;
    };
    let bus = digital_buses
        .iter()
        .find(|bus| format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb) == label)?;
    Some(bus.members.iter().map(|net| format!("D({net})")).collect())
}

/// The active run's analysis and waveform that a signal of another run
/// overlays: the strip draws a run over the curves of the same name.
fn active_counterpart(
    state: &ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
) -> Option<(usize, usize, SourceWaveformPresentationKey, String, bool)> {
    let simulation = &state.model.simulation;
    let (run_index, analysis_index, _, waveform) = key.resolve(&simulation.retained.runs)?;
    let run = &simulation.retained.runs[run_index];
    let analysis: &rspice_results::analysis_result::AnalysisResult<
        rspice_results::waveform::RetainedTrace,
    > = run.analyses[analysis_index].as_ref();
    let active = simulation.active_run()?;
    active
        .analyses
        .iter()
        .enumerate()
        .find_map(|(index, candidate)| {
            let matched =
                rspice_results_ui::waves::matching_overlay_analysis(candidate.as_ref(), run)?;
            if !std::ptr::eq(matched, analysis) {
                return None;
            }
            let (waveform_index, counterpart) = candidate
                .waveforms
                .iter()
                .enumerate()
                .find(|(_, counterpart)| counterpart.name == waveform.name)?;
            Some((
                index,
                waveform_index,
                SourceWaveformPresentationKey::new(
                    AnalysisPresentationKey::new(active.dataset_id, candidate),
                    counterpart.name.clone(),
                ),
                counterpart.name.clone(),
                counterpart.visible,
            ))
        })
}

/// A signal of the active run is ticked while it is on the plot; one of
/// another run while the strip overlays it.
fn result_signal_ticked(state: &ResultsParts<'_>, key: &SourceWaveformPresentationKey) -> bool {
    let simulation = &state.model.simulation;
    let Some((run_index, _, _, waveform)) = key.resolve(&simulation.retained.runs) else {
        return false;
    };
    if simulation.view.active_run_idx == Some(run_index) {
        return waveform_plotted(
            state.model,
            state.results,
            key,
            &waveform.name,
            waveform.visible,
        );
    }
    simulation.is_dataset_overlaid(simulation.retained.runs[run_index].dataset_id)
        && active_counterpart(state, key).is_some_and(|(.., key, name, default)| {
            waveform_plotted(state.model, state.results, &key, &name, default)
        })
}

/// Put a signal on the plot or take it off, at once. A signal of another run
/// goes on over the active run's curve of its name, and comes off with its
/// run: the strip overlays whole runs.
pub(super) fn set_result_signal_plotted(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    plotted: bool,
) {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&state.model.simulation.retained.runs)
    else {
        return;
    };
    if state.model.simulation.view.active_run_idx == Some(run_index) {
        if result_signal_ticked(state, key) != plotted {
            crate::workbench::documents::result_document::toggle_visibility(
                state,
                analysis_index,
                waveform_index,
            );
        }
        if plotted {
            reveal_strip(state, key.analysis());
        }
        return;
    }
    let dataset = state.model.simulation.retained.runs[run_index].dataset_id;
    let overlaid = state.model.simulation.is_dataset_overlaid(dataset);
    if !plotted {
        if overlaid {
            state.model.simulation.toggle_dataset_overlay(dataset);
        }
        return;
    }
    let Some((analysis, waveform, counterpart, name, default)) = active_counterpart(state, key)
    else {
        state
            .workbench
            .prompt("The active result has no curve of this name to overlay.");
        return;
    };
    if !overlaid {
        state.model.simulation.toggle_dataset_overlay(dataset);
    }
    if !waveform_plotted(state.model, state.results, &counterpart, &name, default) {
        crate::workbench::documents::result_document::toggle_visibility(state, analysis, waveform);
    }
    reveal_strip(state, counterpart.analysis());
}

/// A strip closed or crowded out by another's maximizing comes back for a
/// curve just put on it.
fn reveal_strip(state: &mut ResultsParts<'_>, analysis: AnalysisPresentationKey) {
    let session = &mut state.results.session;
    session.hidden_strips.remove(&analysis);
    if session
        .maximized_strip
        .is_some_and(|maximized| maximized != analysis)
    {
        session.maximized_strip = None;
    }
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
    ordered_visible: &[ResultBrowserSelectionKey],
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
            ordered_visible,
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
    ordered_visible: &[ResultBrowserSelectionKey],
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
        update_result_browser_multi_selection(state, &current, ordered_visible, modifiers);
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
    if !super::super::super::chrome::document_bar::results::show_result_dataset(
        &mut state.model.simulation,
        state.workbench,
        &dataset_id,
    ) {
        return false;
    }
    state.results.session.selected_trace = None;
    state.results.session.selected_result_artifact = None;
    true
}

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
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            &state.model.simulation.retained.runs,
        )
        .err();
    let exact_available = exact_error.is_none();
    let (popup, _) = row_context_menu(response);
    popup.show(|ui| {
        let open = ui.add_enabled(
            exact_available,
            egui::Button::new("Open in active result pane"),
        );
        if let Some(error) = exact_error.as_deref() {
            open.clone().on_disabled_hover_text(error);
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
        let table = ui.add_enabled(exact_available, egui::Button::new("Open as table"));
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
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
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Waveform(key.clone()),
            &state.model.simulation.retained.runs,
        )
        .err();
    let exact_available = exact_error.is_none();
    let (popup, _) = row_context_menu(response);
    popup.show(|ui| {
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
            egui::Button::new("Create result document..."),
        );
        if let Some(error) = exact_error.as_deref() {
            create.clone().on_disabled_hover_text(error);
        }
        if create.clicked() {
            select_result_signal_by_key(state, &key);
            crate::workbench::documents::result_document::open_create_document(state);
            ui.close();
        }
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
        let signal_name = key
            .resolve(&state.model.simulation.retained.runs)
            .map(|(.., waveform)| waveform.name.clone());
        let cross_probe = ui.add_enabled(
            signal_name.is_some(),
            egui::Button::new("Show on schematic"),
        );
        if signal_name.is_none() {
            cross_probe
                .clone()
                .on_disabled_hover_text("This signal is no longer in its run.");
        }
        if cross_probe.clicked() {
            if let Some(signal) = signal_name {
                show_signal_on_schematic(state, &signal);
            }
            ui.close();
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

fn show_signal_on_schematic(state: &mut ResultsParts<'_>, signal: &str) {
    match state.model.select_signal_conductor(signal) {
        Ok(net) => {
            execute(state, ids::DESIGN_WORKSPACE);
            state.workbench.outbox.push(PromptAfterCommands(format!(
                "Selected conductor {net} from {signal}."
            )));
        }
        Err(error) => state.workbench.prompt(error.message(signal)),
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
    let signal = signal.to_owned();
    response.context_menu(|ui| {
        if ui.button("Show on schematic").clicked() {
            show_signal_on_schematic(state, &signal);
            ui.close();
        }
    });
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
