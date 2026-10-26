//! The results feature: the retained result datasets as the reader works
//! with them, through the result viewers, the result document's forms and
//! tools, and the hops back out of a run.
//!
//! Its commands run in a [`Cx`] over the feature's own slot, the model,
//! whose retained runs they read and clear, the session, whose workspace
//! and route they open, and the platform services' state, which says
//! whether a project is open. The data browser, the Results navigator,
//! draws in the same context.

mod calculator;
pub(crate) mod commands;
mod expression_diagnostics;
mod surface;

use crate::ui::widgets::NotificationAction;
use crate::workbench::commands::result_navigation::{
    dataset_export_is_resolvable, follow_notification_action, open_run_log, producer_log_hop,
    producing_plan_hop, select_newest_retained_run, task_deck_hop,
};
use crate::workbench::documents::result_document::{
    ResultsParts, ResultsReads, add_pane_available, compare_availability, copy_cursor_text,
    family_available, fit_gesture_available, linked_cursors, open_create_document,
    open_dataset_browser, open_specification_editor, request_view_gesture, viewer_is_available,
    viewer_unavailability_reason, zoom_gesture_available,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    FeatureSplit, FeatureSurface, Reads, RoutedCommand, Session,
};
use crate::workbench::framework::requests::{
    ClipboardText, ExportDataset, RevealExecutedDeck, SelectAnalysisInstance,
};
use crate::workbench::slots::ResultsState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::workflows::result_import_workflow::import_result_dataset;
use crate::workbench::{ResultViewer, SurfaceId};
use rspice_results_ui::session::ViewGesture;

use commands::ResultsCommand;

/// The results feature.
pub(crate) struct ResultsFeature;

impl ResultsFeature {
    /// Open Results on the newest run that holds a dataset, as the status
    /// bar's engine chip asks; false when there is none. The shell answers
    /// the command it raises straight after.
    pub(in crate::workbench) fn open_newest_retained_run(cx: &mut Cx<'_, Self>) -> bool {
        let selected = select_newest_retained_run(&mut cx.model.simulation);
        if selected {
            cx.execute(ids::OPEN_RUN_IN_RESULTS);
        }
        selected
    }

    /// Take the navigation offer a notice carried: open Results on the run
    /// it names, or the Console on the line it names. The shell answers the
    /// command it raises straight after.
    pub(in crate::workbench) fn perform_notification_action(
        cx: &mut Cx<'_, Self>,
        action: NotificationAction,
    ) {
        if follow_notification_action(cx.model, cx.wb, action) {
            cx.execute(ids::OPEN_RUN_IN_RESULTS);
        }
    }
}

impl Feature for ResultsFeature {
    type State = ResultsState;
    type Session = WorkbenchSession;
    type Handles = crate::workbench::framework::feature::NoHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<ResultsCommand>();
        registry.routed(ROUTED);
        registry.navigator(FeatureNavigator {
            show: show_data_browser,
        });
        registry.inspector(FeatureInspector {
            show: show_inspector,
            commit_pending_edit: keep_axis_drafts,
        });
        registry.surface(FeatureSurface { show: show_surface });
        registry.split(FeatureSplit { show: show_split });
        registry.dialogs(DIALOGS);
    }
}

/// The data browser, over the results workspace's parts and Run details…,
/// which it opens on a retained run.
fn show_data_browser(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let (mut parts, run_details) =
        cx.state
            .parts_and_run_details(cx.model, cx.wb, cx.platform, &mut cx.runs.preflight);
    crate::workbench::docks::navigator::results::results(ui, &mut parts, run_details);
}

/// The results inspector, over the same parts and Run details…, which it
/// opens on the run it names.
fn show_inspector(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let (mut parts, run_details) =
        cx.state
            .parts_and_run_details(cx.model, cx.wb, cx.platform, &mut cx.runs.preflight);
    crate::workbench::docks::inspector::results::results(ui, &mut parts, run_details);
}

/// The Results stage, over the workspace's parts, the run controller its
/// derived viewers load through, and the specification simulate has
/// selected; then the result document's two forms, the last things the
/// stage draws.
fn show_surface(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let mut parts = cx
        .state
        .parts(cx.model, cx.wb, cx.platform, &mut cx.runs.preflight);
    surface::show(
        ui,
        &mut parts,
        cx.handles.run_controller,
        cx.peers.simulate().selected_specification(),
    );
    render_create_result_document(cx, ui.ctx());
    render_result_document_dialog(cx, ui.ctx());
}

/// The compact Results projection beside another workspace's document, over
/// the same.
fn show_split(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let mut parts = cx
        .state
        .parts(cx.model, cx.wb, cx.platform, &mut cx.runs.preflight);
    crate::workbench::documents::result_document::show_compact_split(
        ui,
        &mut parts,
        cx.handles.run_controller,
        cx.peers.simulate().selected_specification(),
    );
}

/// The dialogs the feature draws, each under the key the shell lists it by.
/// The dialog order draws the first three at their steps. The rest hold a
/// modal gate the shell asks of them, and each is drawn where its work is:
/// Export dataset… where the frame answers the export request, Run
/// details… at its frame step, and the result document's two forms at the
/// end of the Results stage.
const DIALOGS: &[FeatureDialog<ResultsFeature>] = &[
    FeatureDialog {
        key: DialogKey("results.result_import"),
        is_open: |results| results.result_import.open,
        render: render_result_import,
    },
    FeatureDialog {
        key: DialogKey("results.calculator"),
        is_open: |results| results.waveform_calculator_dialog,
        render: calculator::show,
    },
    FeatureDialog {
        key: DialogKey("results.expression_diagnostics"),
        is_open: |results| results.expression_diagnostics_dialog,
        render: expression_diagnostics::show,
    },
    FeatureDialog {
        key: DialogKey("results.dataset_export"),
        is_open: |results| results.dataset_export.is_some(),
        render: render_dataset_export,
    },
    FeatureDialog {
        key: DialogKey("results.run_details"),
        is_open: |results| results.run_details.is_some(),
        render: render_run_details,
    },
    FeatureDialog {
        key: DialogKey("results.create_result_document"),
        is_open: |results| results.create_result_document.open,
        render: render_create_result_document,
    },
    FeatureDialog {
        key: DialogKey("results.result_document"),
        is_open: |results| results.result_document_dialog.is_some(),
        render: render_result_document_dialog,
    },
];

/// The staged import of an external result dataset.
fn render_result_import(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    crate::workbench::workflows::result_import_workflow::show_result_import_dialog(
        ctx,
        &mut results_parts(cx),
    );
}

/// "Export dataset…" asks for a format, then writes the displayed result in
/// it.
fn render_dataset_export(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    if crate::workbench::menu_bar::show_dataset_export_picker(
        ctx,
        &mut cx.state.dataset_export,
        &mut cx.model.preferences,
    ) {
        let io = cx.handles.export_io;
        crate::workbench::menu_bar::action_export_csv_with_io(&mut results_parts(cx), io);
    }
}

/// Run details…, and the route out of it the reader took: the producing
/// plan, or the deck the run executed.
fn render_run_details(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    let routes =
        crate::workbench::docks::show_run_details(ctx, &mut cx.state.run_details, cx.model);
    if let Some(run_id) = routes.open_deck {
        cx.wb.outbox().push(RevealExecutedDeck(run_id));
    }
    if routes.open_plan {
        cx.execute(ResultsCommand::OpenProducingPlan.key());
    }
}

/// The form that creates a result document.
fn render_create_result_document(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    crate::workbench::documents::result_document::create_document::show(
        ctx,
        &mut results_parts(cx),
    );
}

/// The open form of the result document: a pane, a comparison, a family, a
/// measurement or the document's properties.
fn render_result_document_dialog(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    crate::workbench::documents::result_document::show_document_dialog(ctx, &mut results_parts(cx));
}

/// Nothing to commit when another section shows in the inspector's place:
/// an axis interval commits as its field loses focus, and one half typed
/// stays a draft for when the section shows again.
fn keep_axis_drafts(_: &mut Cx<'_, ResultsFeature>, _: &egui::Context) {}

/// The workbench commands the results workspace answers for itself: Copy
/// takes the cursor readout, the zooms and Fit move the active sheet's
/// viewport, Cancel lets go of the placed cursors, and Reset active view
/// forgets the sheet's view. Each is the workspace's wherever Results is in
/// front, Cancel only while a cursor is placed.
const ROUTED: &[RoutedCommand<ResultsFeature>] = &[
    RoutedCommand {
        key: ids::COPY,
        takes: in_results,
        enabled: |reads| reads.state.results.session.cursors.a.is_some(),
        execute: copy_cursor_readout,
    },
    RoutedCommand {
        key: ids::ZOOM_IN,
        takes: in_results,
        enabled: |reads| zoom_gesture_available(results_reads(reads)),
        execute: |cx| request_view_gesture(&mut cx.state.results, ViewGesture::ZoomIn),
    },
    RoutedCommand {
        key: ids::ZOOM_OUT,
        takes: in_results,
        enabled: |reads| zoom_gesture_available(results_reads(reads)),
        execute: |cx| request_view_gesture(&mut cx.state.results, ViewGesture::ZoomOut),
    },
    // The waveform sheets key their viewports by analysis, so fitting them
    // is not a write to the plot-ordinal store.
    RoutedCommand {
        key: ids::ZOOM_FIT,
        takes: in_results,
        enabled: |reads| fit_gesture_available(results_reads(reads)),
        execute: |cx| request_view_gesture(&mut cx.state.results, ViewGesture::Fit),
    },
    RoutedCommand {
        key: ids::CANCEL,
        takes: |reads| in_results(reads) && reads.state.results.session.cursors.any(),
        enabled: |_| true,
        execute: |cx| cx.state.results.session.clear_cursors(),
    },
    RoutedCommand {
        key: ids::RESET_ACTIVE_VIEW,
        takes: in_results,
        enabled: |_| true,
        execute: reset_active_view,
    },
];

/// Whether the results workspace is the one in front.
fn in_results(reads: &Reads<'_, ResultsFeature>) -> bool {
    reads.wb.workspace == Workspace::Results
}

/// Put the cursor readout on the platform clipboard, when a cursor is placed.
fn copy_cursor_readout(cx: &mut Cx<'_, ResultsFeature>) {
    if let Some(text) = copy_cursor_text(&mut results_parts(cx)) {
        cx.wb.outbox().push(ClipboardText(text));
    }
}

/// Forget the active sheet's view: its viewports, its cursors, its pinned
/// RF marker and, on Waves, the strips hidden or maximized.
fn reset_active_view(cx: &mut Cx<'_, ResultsFeature>) {
    let session = &mut cx.state.results.session;
    let viewer = session.viewer;
    session
        .views
        .retain(|(candidate, _, _), _| *candidate != viewer);
    session.clear_cursors();
    session.rf_pin.remove(&viewer);
    if viewer == ResultViewer::Waves {
        session.hidden_strips.clear();
        session.maximized_strip = None;
    }
}

/// What the results workspace's predicates read of a command's reads.
fn results_reads<'a>(reads: &Reads<'a, ResultsFeature>) -> ResultsReads<'a> {
    reads.state.reads(reads.model, reads.wb)
}

/// What the results workspace's operations write of a command's context.
fn results_parts<'a>(cx: &'a mut Cx<'_, ResultsFeature>) -> ResultsParts<'a> {
    cx.state
        .parts(cx.model, cx.wb, cx.platform, &mut cx.runs.preflight)
}

/// `Available` when `available`, and `refusal` otherwise.
const fn available_or(available: bool, refusal: &'static str) -> CommandAvailability {
    if available {
        CommandAvailability::Available
    } else {
        CommandAvailability::Disabled(refusal)
    }
}

impl CommandSet for ResultsCommand {
    type Feature = ResultsFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    fn availability(self, reads: &Reads<'_, ResultsFeature>) -> CommandAvailability {
        let state = results_reads(reads);
        let (model, workbench) = (reads.model, reads.wb);
        let retained = &model.simulation.retained;
        let executing = model.simulation.execution.has_active_execution();
        let on_results_surface = workbench.current_route().surface_id() == SurfaceId::Results;
        match self {
            Self::ImportResultDataset => {
                if executing {
                    CommandAvailability::Disabled(
                        "an active simulation execution still owns result history",
                    )
                } else {
                    available_or(
                        !model.safe_mode.project_read_only(),
                        "the project is open read-only",
                    )
                }
            }
            Self::ExportWaveformsCsv => available_or(
                dataset_export_is_resolvable(state),
                if retained.has_results() {
                    "open a result document in Results"
                } else {
                    "no result dataset is available"
                },
            ),
            Self::ClearResults => {
                if retained.has_results() && !executing {
                    CommandAvailability::Available
                } else if executing {
                    CommandAvailability::Disabled(
                        "an active simulation execution still owns result history",
                    )
                } else {
                    CommandAvailability::Disabled("no result dataset is available")
                }
            }
            Self::ToggleLinkedCursors => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    available_or(retained.has_results(), "no result dataset is available")
                }
            }
            Self::ToggleResultsSplit => {
                if !workbench.supports_results_split() {
                    CommandAvailability::Disabled("open Design, Netlist, or Simulation setup")
                } else {
                    available_or(
                        retained.has_retained_result_dataset(),
                        "no retained result dataset is available",
                    )
                }
            }
            Self::DatasetManifestBrowser => available_or(
                !retained.runs.is_empty(),
                "no retained result dataset is available",
            ),
            Self::CreateResultDocument => available_or(
                retained.has_results(),
                "no retained result dataset is available",
            ),
            // Both return edges are judged on the dataset, never on the
            // surface the reader is standing on: they exist precisely to be
            // taken from somewhere else.
            Self::OpenRunInResults => available_or(
                model
                    .simulation
                    .active_run()
                    .is_some_and(|run| !run.analyses.is_empty()),
                "the selected run has no retained dataset to open in Results",
            ),
            // Each refusal comes from the resolver that would perform the
            // hop, so the reason a control is grey is the reason it would
            // have failed rather than a second account of it.
            Self::OpenProducingPlan => producing_plan_hop(state)
                .map_or_else(CommandAvailability::Disabled, |_| {
                    CommandAvailability::Available
                }),
            Self::OpenTaskDeck => task_deck_hop(state)
                .map_or_else(CommandAvailability::Disabled, |_| {
                    CommandAvailability::Available
                }),
            Self::RevealProducerLog => producer_log_hop(state)
                .map_or_else(CommandAvailability::Disabled, |_| {
                    CommandAvailability::Available
                }),
            Self::WaveformCalculator | Self::EditSpecifications => CommandAvailability::Available,
            Self::ExpressionDiagnostics => {
                available_or(retained.has_results(), "no result dataset is available")
            }
            Self::CompareResultDatasets => compare_availability(state),
            // Add measurement draws over the result viewer, and over a quick
            // view it evaluates without keeping, so it needs only a result.
            Self::MeasurementLibrary | Self::VisualizationDocumentProperties => {
                if !on_results_surface {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    available_or(
                        retained.has_results(),
                        "no retained result dataset is available",
                    )
                }
            }
            Self::FamilySlicing => {
                if on_results_surface && family_available(state) {
                    CommandAvailability::Available
                } else if on_results_surface && retained.has_results() {
                    CommandAvailability::Disabled(
                        "the result holds no sweep, corner or Monte Carlo family",
                    )
                } else if !on_results_surface {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    CommandAvailability::Disabled("no retained result dataset is available")
                }
            }
            Self::AddVisualizationPane => available_or(
                on_results_surface && add_pane_available(state),
                "command is unavailable in this context",
            ),
            // Waves is the workspace's default sheet, and its empty state is
            // the workspace's landing, so this route stays actionable before a
            // dataset exists. Every other viewer is only actionable when the
            // active retained dataset satisfies the same compatibility
            // contract used by the in-workspace viewer tabs.
            Self::ResultViewer(ResultViewer::Waves) => CommandAvailability::Available,
            Self::ResultViewer(viewer) => {
                if viewer_is_available(state, viewer) {
                    CommandAvailability::Available
                } else {
                    CommandAvailability::Disabled(
                        viewer_unavailability_reason(state, viewer)
                            .unwrap_or("the active dataset is incompatible with this viewer"),
                    )
                }
            }
        }
    }

    fn execute(self, cx: &mut Cx<'_, ResultsFeature>) {
        match self {
            Self::ImportResultDataset => {
                import_result_dataset(&mut results_parts(cx));
            }
            // The File menu's export: it lands the edit typed into the
            // inspector before it asks for a format.
            Self::ExportWaveformsCsv => cx.wb.outbox().push(ExportDataset),
            Self::ClearResults => results_parts(cx).clear_simulation_results(),
            Self::ToggleLinkedCursors => {
                crate::workbench::documents::result_document::toggle_linked_cursors(
                    &mut results_parts(cx),
                );
            }
            Self::ToggleResultsSplit => {
                if cx.wb.split_with_results {
                    cx.wb.split_with_results = false;
                } else if let Some(run_index) = cx
                    .model
                    .simulation
                    .retained
                    .newest_retained_result_run_index()
                {
                    // The split starts on the newest materialized dataset,
                    // but remains the one canonical Results selection: there
                    // is no copied run, waveform, or document owner.
                    cx.model.simulation.select_run(run_index);
                    cx.wb.split_with_results = true;
                }
            }
            Self::DatasetManifestBrowser => open_dataset_browser(&mut results_parts(cx)),
            Self::CreateResultDocument => open_create_document(&mut results_parts(cx)),
            Self::WaveformCalculator => cx.state.waveform_calculator_dialog = true,
            Self::ExpressionDiagnostics => cx.state.expression_diagnostics_dialog = true,
            Self::CompareResultDatasets => {
                crate::workbench::documents::result_document::open_compare(&mut results_parts(cx));
            }
            Self::ResultViewer(viewer) => {
                if viewer == ResultViewer::Waves
                    || viewer_is_available(cx.state.reads(cx.model, cx.wb), viewer)
                {
                    cx.state.results.session.viewer = viewer;
                    cx.wb.activate(Workspace::Results);
                }
            }
            Self::EditSpecifications => open_specification_editor(&mut results_parts(cx)),
            Self::AddVisualizationPane => {
                crate::workbench::documents::result_document::open_add_pane(&mut results_parts(cx));
            }
            Self::MeasurementLibrary => {
                crate::workbench::documents::result_document::open_measurement(&mut results_parts(
                    cx,
                ));
            }
            Self::FamilySlicing => {
                crate::workbench::documents::result_document::open_family(&mut results_parts(cx));
            }
            Self::VisualizationDocumentProperties => {
                crate::workbench::documents::result_document::open_properties(&mut results_parts(
                    cx,
                ));
            }
            // The run is already the selected one: every affordance that hops
            // here selects it first, so this command carries the workspace
            // half of the destination and nothing else.
            Self::OpenRunInResults => cx.wb.activate(Workspace::Results),
            Self::OpenProducingPlan => {
                match producing_plan_hop(cx.state.reads(cx.model, cx.wb)) {
                    Ok(hop) => {
                        // Selected unconditionally, `None` included. A hop
                        // whose producing instance is gone used to leave the
                        // previously highlighted one selected, so the page
                        // arrived showing an analysis that produced nothing
                        // the reader was looking at — while the note said
                        // nothing had been selected.
                        cx.wb.outbox().push(SelectAnalysisInstance(hop.instance));
                        let notes = [hop.selected_note, hop.instance_note]
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>();
                        if !notes.is_empty() {
                            cx.wb.prompt(notes.join(" "));
                        }
                        cx.execute(ids::SIMULATION_ANALYSES);
                    }
                    Err(reason) => cx.wb.prompt(format!(
                        "Could not open the producing simulation plan: {reason}."
                    )),
                }
            }
            // Both re-resolve rather than trusting the availability read that
            // drew the row: the palette is not modal, and a run can be pruned
            // or a check-mark cleared between the frame that greyed the row
            // and the keystroke that took it.
            Self::OpenTaskDeck => match task_deck_hop(cx.state.reads(cx.model, cx.wb)) {
                Ok(sequence) => cx.wb.outbox().push(RevealExecutedDeck(sequence)),
                Err(reason) => cx
                    .wb
                    .prompt(format!("Could not open the task deck: {reason}.")),
            },
            Self::RevealProducerLog => match producer_log_hop(cx.state.reads(cx.model, cx.wb)) {
                Ok(run_sequence) => open_run_log(cx.model, cx.wb, run_sequence),
                Err(reason) => cx
                    .wb
                    .prompt(format!("Could not open the run log: {reason}.")),
            },
        }
    }

    fn checked(self, reads: &Reads<'_, ResultsFeature>) -> Option<bool> {
        match self {
            Self::ToggleResultsSplit => Some(reads.wb.split_with_results),
            Self::ToggleLinkedCursors => Some(linked_cursors(results_reads(reads))),
            _ => None,
        }
    }
}
