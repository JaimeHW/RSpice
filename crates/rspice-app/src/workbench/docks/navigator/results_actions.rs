//! What a data-browser row does when the reader acts on it: batch membership,
//! the keyboard, plot visibility, selection, and the context menus.

use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, exact_result_artifact_text, exact_result_signal_last_sample,
    exact_result_signal_tsv,
};
use crate::workbench::framework::requests::ExportResultQuantities;
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use super::super::super::commands::result_navigation::{open_run_log, producing_run};
use super::super::super::commands::vocabulary::Command;
use super::super::super::state::Workspace;
use super::results::{RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT, ResultArtifact};
use super::row_context_menu;

/// Put every checked quantity of the active dataset into the requested plot
/// state. Toggling one at a time is the same act repeated, so the batch runs
/// through the same visibility path rather than writing the flag itself.
pub(super) fn update_result_browser_multi_selection(
    app: &mut RSpiceApp,
    target: &ResultBrowserSelectionKey,
    ordered_visible: &[ResultBrowserSelectionKey],
    modifiers: egui::Modifiers,
) {
    if modifiers.shift {
        app.state
            .results
            .results
            .session
            .select_checked_result_range(target, ordered_visible);
    } else {
        app.state
            .results
            .results
            .session
            .toggle_checked_result_quantity(target.clone());
        app.state
            .results
            .results
            .session
            .set_browser_range_anchor(target.clone());
    }
}

pub(super) fn result_signal_row_id(key: &ResultBrowserSelectionKey) -> egui::Id {
    egui::Id::new(("workbench.result-browser.signal", key))
}

pub(super) fn handle_result_signal_keyboard(
    response: &egui::Response,
    app: &mut RSpiceApp,
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
            app,
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
        open_signal_table(app, current, false);
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
        app.state
            .results
            .results
            .session
            .select_checked_result_range(next, ordered_visible);
    } else {
        app.state
            .results
            .results
            .session
            .set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(app, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

pub(super) fn handle_result_artifact_keyboard(
    response: &egui::Response,
    app: &mut RSpiceApp,
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
        update_result_browser_multi_selection(app, &current, ordered_visible, modifiers);
        return;
    }
    if response
        .ctx
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
    {
        open_result_artifact(app, artifact, false);
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
        app.state
            .results
            .results
            .session
            .select_checked_result_range(next, ordered_visible);
    } else {
        app.state
            .results
            .results
            .session
            .set_browser_range_anchor(next.clone());
    }
    select_result_browser_key(app, next);
    response
        .ctx
        .memory_mut(|memory| memory.request_focus(result_signal_row_id(next)));
}

pub(super) fn set_checked_signal_visibility(
    app: &mut RSpiceApp,
    checked: &std::collections::HashSet<ResultBrowserSelectionKey>,
    visible: bool,
) {
    let Some(run_index) = app.state.model.simulation.view.active_run_idx else {
        return;
    };
    let targets = checked
        .iter()
        .filter_map(|key| {
            let key = key.waveform()?;
            let (target_run, analysis_index, waveform_index, waveform) =
                key.resolve(&app.state.model.simulation.retained.runs)?;
            let currently_visible = app
                .state
                .results
                .results
                .session
                .waveform_visibility(key, waveform.visible);
            (target_run == run_index && currently_visible != visible)
                .then_some((analysis_index, waveform_index))
        })
        .collect::<Vec<_>>();
    for (analysis_index, waveform_index) in targets {
        crate::workbench::documents::result_document::toggle_visibility(
            &mut app.state,
            analysis_index,
            waveform_index,
        );
    }
}

pub(super) fn toggle_result_signal_visibility(
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&app.state.model.simulation.retained.runs)
    else {
        return false;
    };
    if app.state.model.simulation.view.active_run_idx != Some(run_index) {
        return false;
    }
    crate::workbench::documents::result_document::toggle_visibility(
        &mut app.state,
        analysis_index,
        waveform_index,
    );
    true
}

pub(super) fn select_result_dataset(app: &mut RSpiceApp, run_index: usize) -> bool {
    let Some(dataset_id) = app
        .state
        .model
        .simulation
        .retained
        .runs
        .get(run_index)
        .map(|run| run.dataset_id)
    else {
        return false;
    };
    let document = super::super::super::state::WorkspaceDocumentId::ResultDataset(dataset_id);
    if !super::super::super::chrome::document_bar::activate_document_by_id(
        &mut app.state,
        &document,
    ) {
        return false;
    }
    app.state.results.results.session.selected_trace = None;
    app.state.results.results.session.selected_result_artifact = None;
    true
}

pub(super) fn select_result_analysis(
    app: &mut RSpiceApp,
    run_index: usize,
    analysis_index: usize,
) -> bool {
    if app
        .state
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
    if !select_result_dataset(app, run_index) {
        return false;
    }
    if !app.state.model.simulation.select_analysis(analysis_index) {
        return false;
    }
    app.state.results.results.session.selected_trace = None;
    app.state.results.results.session.selected_result_artifact = None;
    true
}

pub(super) fn select_result_analysis_by_key(
    app: &mut RSpiceApp,
    key: AnalysisPresentationKey,
) -> bool {
    let Some((run_index, analysis_index)) = app
        .state
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
    select_result_analysis(app, run_index, analysis_index)
}

pub(super) fn select_result_signal(
    app: &mut RSpiceApp,
    run_index: usize,
    analysis_index: usize,
    waveform_index: usize,
) -> bool {
    let Some(run) = app.state.model.simulation.retained.runs.get(run_index) else {
        return false;
    };
    let Some(selected) = rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
        run,
        analysis_index,
        waveform_index,
    ) else {
        return false;
    };
    if !select_result_analysis(app, run_index, analysis_index) {
        return false;
    }
    // Document/run activation advances the canonical simulation data version.
    // Reconcile the viewer before installing the exact trace selection so the
    // next Results frame cannot clear the selection as if it belonged to the
    // previously active dataset.
    crate::workbench::documents::result_document::prepare_viewer_state(app);
    let name = selected.source_name().to_owned();
    let recent = SourceWaveformPresentationKey::new(selected.analysis_key(), name);
    app.state.results.results.session.selected_trace = Some(selected);
    app.state.results.results.session.selected_result_artifact = None;
    // Selecting a trace is a deliberate act; feed the browser's Recent scope.
    app.state.results.results.session.note_recent_signal(recent);
    true
}

pub(super) fn select_result_signal_by_key(
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&app.state.model.simulation.retained.runs)
    else {
        return false;
    };
    select_result_signal(app, run_index, analysis_index, waveform_index)
}

pub(super) fn select_result_artifact(
    app: &mut RSpiceApp,
    key: &ResultArtifactPresentationKey,
) -> bool {
    let Some((run_index, analysis_index, _)) =
        key.resolve(&app.state.model.simulation.retained.runs)
    else {
        return false;
    };
    if !select_result_analysis(app, run_index, analysis_index) {
        return false;
    }
    app.state.results.results.session.selected_trace = None;
    app.state.results.results.session.selected_result_artifact = Some(key.clone());
    app.state
        .results
        .results
        .session
        .note_recent_result_artifact(key.clone());
    true
}

pub(super) fn select_result_browser_key(
    app: &mut RSpiceApp,
    key: &ResultBrowserSelectionKey,
) -> bool {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => select_result_signal_by_key(app, key),
        ResultBrowserSelectionKey::Artifact(key) => select_result_artifact(app, key),
    }
}

pub(super) fn result_artifact_context_menu(
    response: &egui::Response,
    app: &mut RSpiceApp,
    artifact: &ResultArtifact,
    favorite: bool,
) {
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            &app.state.model.simulation.retained.runs,
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
            open_result_artifact(app, artifact, false);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_artifact(app, &artifact.identity) {
                app.state.results.results.session.viewer = artifact.viewer;
                crate::workbench::documents::result_document::add_selected_result_as_pane(app);
            }
            ui.close();
        }
        let table = ui.add_enabled(exact_available, egui::Button::new("Open as table"));
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
        }
        if table.clicked() {
            if select_result_artifact(app, &artifact.identity) {
                app.execute_command_key(ResultsCommand::ResultViewer(ResultViewer::Table).key());
            }
            ui.close();
        }
        let compare_enabled =
            exact_available && app.command_key_enabled(ResultsCommand::CompareResultDatasets.key());
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
            select_result_artifact(app, &artifact.identity);
            app.execute_command_key(ResultsCommand::CompareResultDatasets.key());
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
                &app.state.model.simulation.retained.runs,
            ) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => app.state.prompt(error),
            }
            ui.close();
        }
        if ui
            .button(if favorite {
                "Remove from favorites"
            } else {
                "Add to favorites"
            })
            .clicked()
        {
            app.state
                .results
                .results
                .session
                .toggle_favorite_result_artifact(artifact.identity.clone());
            ui.close();
        }
        let export = ui.add_enabled(exact_available, egui::Button::new("Export\u{2026}"));
        if let Some(error) = exact_error.as_deref() {
            export.clone().on_disabled_hover_text(error);
        }
        if export.clicked() {
            open_result_artifact(app, artifact, true);
            ui.close();
        }
        // Offered only while the session holds the run that produced it.
        if let Some(run) = producing_run(
            &app.state.model.simulation.retained.runs,
            artifact.identity.analysis().dataset_id(),
        ) && ui.button("Open run log").clicked()
        {
            open_run_log(&app.state.model, &mut app.state.workbench, run);
            ui.close();
        }
    });
}

fn open_result_artifact(app: &mut RSpiceApp, artifact: &ResultArtifact, request_export: bool) {
    if select_result_artifact(app, &artifact.identity) {
        if request_export {
            app.state.workbench.outbox.push(ExportResultQuantities(vec![
                ResultBrowserSelectionKey::Artifact(artifact.identity.clone()),
            ]));
        } else {
            app.execute_command_key(ResultsCommand::ResultViewer(artifact.viewer).key());
        }
    }
}

pub(super) fn result_signal_context_menu(
    response: &egui::Response,
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
    visible: bool,
    favorite: bool,
) {
    let key = key.clone();
    let exact_error =
        crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
            &ResultBrowserSelectionKey::Waveform(key.clone()),
            &app.state.model.simulation.retained.runs,
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
            toggle_result_signal_visibility(app, &key);
            ui.close();
        }
        let add = ui.add_enabled(exact_available, egui::Button::new("Add to new pane..."));
        if let Some(error) = exact_error.as_deref() {
            add.clone().on_disabled_hover_text(error);
        }
        if add.clicked() {
            if select_result_signal_by_key(app, &key) {
                crate::workbench::documents::result_document::add_selected_result_as_pane(app);
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
            select_result_signal_by_key(app, &key);
            crate::workbench::documents::result_document::open_create_document(&mut app.state.results_parts());
            ui.close();
        }
        ui.separator();
        let table = ui.add_enabled(exact_available, egui::Button::new("Open as table"));
        if let Some(error) = exact_error.as_deref() {
            table.clone().on_disabled_hover_text(error);
        }
        if table.clicked() {
            open_signal_table(app, &key, false);
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
            open_signal_table(app, &key, true);
            ui.close();
        }
        let compare_enabled = exact_available && app.command_key_enabled(ResultsCommand::CompareResultDatasets.key());
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
            select_result_signal_by_key(app, &key);
            app.execute_command_key(ResultsCommand::CompareResultDatasets.key());
            ui.close();
        }
        ui.separator();
        if ui.button("Copy name").clicked() {
            if let Some((.., waveform)) = key.resolve(&app.state.model.simulation.retained.runs) {
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
                &app.state.model.simulation.retained.runs,
                app.state.model.preferences.quantity_presentation_policy(),
            ) {
                Ok(value) => ui.ctx().copy_text(value),
                Err(error) => app.state.prompt(error),
            }
            ui.close();
        }
        let retained_samples = key
            .resolve(&app.state.model.simulation.retained.runs)
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
            match exact_result_signal_tsv(&key, &app.state.model.simulation.retained.runs) {
                Ok(tsv) => ui.ctx().copy_text(tsv),
                Err(error) => app.state.prompt(error),
            }
            ui.close();
        }
        ui.separator();
        if ui
            .button(if favorite {
                "Remove from favorites"
            } else {
                "Add to favorites"
            })
            .clicked()
        {
            app.state.results.results.session.toggle_favorite_signal(key.clone());
            ui.close();
        }
        let signal_name = key
            .resolve(&app.state.model.simulation.retained.runs)
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
                show_signal_on_schematic(app, &signal);
            }
            ui.close();
        }
        // Offered only while the session holds the run that produced it.
        if let Some(run) = producing_run(
            &app.state.model.simulation.retained.runs,
            key.analysis().dataset_id(),
        ) && ui.button("Open run log").clicked()
        {
            open_run_log(&app.state.model, &mut app.state.workbench, run);
            ui.close();
        }
    });
}

fn open_signal_table(
    app: &mut RSpiceApp,
    key: &SourceWaveformPresentationKey,
    request_export: bool,
) {
    if select_result_signal_by_key(app, key) {
        if request_export {
            app.state.workbench.outbox.push(ExportResultQuantities(vec![
                ResultBrowserSelectionKey::Waveform(key.clone()),
            ]));
        } else {
            if let Some(selected) = &app.state.results.results.session.selected_trace {
                let (analysis, column) = selected.table_binding();
                app.state.results.results.session.table.analysis = Some(analysis);
                app.state.results.results.session.table.columns = vec![column];
            }
            app.execute_command_key(ResultsCommand::ResultViewer(ResultViewer::Table).key());
        }
    }
}

fn show_signal_on_schematic(app: &mut RSpiceApp, signal: &str) {
    match app.state.model.select_signal_conductor(signal) {
        Ok(net) => {
            Command::OpenWorkspace(Workspace::Design).execute(app);
            app.state
                .prompt(format!("Selected conductor {net} from {signal}."));
        }
        Err(error) => app.state.prompt(error.message(signal)),
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
    app: &mut RSpiceApp,
    signal: &str,
) {
    let signal = signal.to_owned();
    response.context_menu(|ui| {
        if ui.button("Show on schematic").clicked() {
            show_signal_on_schematic(app, &signal);
            ui.close();
        }
    });
}
