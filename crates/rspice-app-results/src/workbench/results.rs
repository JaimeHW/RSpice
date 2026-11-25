//! The results feature: the retained result datasets as the reader works
//! with them, through the result viewers, the result document's forms and
//! tools, and the hops back out of a run.
//!
//! Its commands run in a [`Cx`] over the feature's own slot, the model,
//! whose retained runs they read and clear, the session, whose workspace
//! and route they open, and the platform services' state, which says
//! whether a project is open. The data browser, the Results navigator,
//! draws in the same context.

pub mod calculator;
pub(crate) mod clear_history;
pub mod commands;
mod expression_diagnostics;
mod surface;

use crate::ui::widgets::NotificationAction;
use crate::workbench::commands::result_navigation::{
    dataset_export_is_resolvable, follow_notification_action, open_run_log, producer_log_hop,
    producing_plan_hop, select_newest_retained_run, task_deck_hop,
};
use crate::workbench::documents::result_document::view_edits;
use crate::workbench::documents::result_document::{
    ResultsParts, ResultsReads, ShowingRun, add_pane_available, compare_availability,
    copy_cursor_text, cursor_jump_availability, family_available, fit_gesture_available,
    fit_y_to_visible_x, hide_selected_trace, jump_cursor, linked_cursors, open_create_document,
    open_dataset_browser, opening_sheet, place_marker_at_cursor, request_view_gesture,
    selected_trace_on_sheet, settle_shown_analysis, settling_analysis, viewer_is_available,
    viewer_unavailability_reason, zoom_gesture_available, zoom_next, zoom_next_available,
    zoom_previous, zoom_previous_available,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    FeatureSurface, Reads, RoutedCommand, Session,
};
use crate::workbench::framework::requests::{
    ClipboardText, ExportDataset, RevealExecutedDeck, SelectAnalysisInstance,
};
use crate::workbench::slots::ResultsState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::workflows::result_import_workflow::import_result_dataset;
use crate::workbench::{ResultViewer, SurfaceId};
use rspice_results::feature_search::Direction;
use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::selection::SelectedResultTrace;
use rspice_results_ui::session::ViewGesture;
use rspice_results_ui::waves::navigation::CursorJump;

use commands::ResultsCommand;

/// The results feature.
pub struct ResultsFeature;

impl ResultsFeature {
    /// Open Results on the newest run that holds a dataset, as the status
    /// bar's engine chip asks; false when there is none. The shell answers
    /// the command it raises straight after.
    pub fn open_newest_retained_run(cx: &mut Cx<'_, Self>) -> bool {
        let selected = select_newest_retained_run(&mut cx.model.simulation);
        if selected {
            cx.execute(ids::OPEN_RUN_IN_RESULTS);
        }
        selected
    }

    /// Open Results on the run a press of Run finished, showing the analysis
    /// the plan's view keeps when the run solved it, else its curves when it
    /// has any, as ADE plots on completion, else its first analysis; on that
    /// analysis' default sheet. A run that has left the history opens
    /// nothing. The shell answers the command it raises straight after.
    pub fn open_finished_run(cx: &mut Cx<'_, Self>, run_sequence: u64) {
        if !cx.model.simulation.select_run_by_sequence(run_sequence) {
            return;
        }
        if let Some(index) = settling_analysis(cx.state.reads(cx.model, cx.wb), ShowingRun::Opened)
        {
            settle_shown_analysis(&mut cx.state.results, &mut cx.model.simulation, index);
        }
        cx.state.results.session.viewer =
            opening_sheet(cx.state.reads(cx.model, cx.wb)).unwrap_or(ResultViewer::Waves);
        cx.execute(ids::OPEN_RUN_IN_RESULTS);
    }

    /// Show the run a press of Run on the drawing, the netlist or the plan
    /// finished on the dock's Plot page, as Virtuoso, ADS and KiCad plot
    /// beside the schematic: the page stays the active one. The run settles
    /// on the analysis it opens on, as a run opened in Results does: the one
    /// the plan's view keeps when the run solved it, else its curves, else
    /// its first analysis, and the data browser marks it. A plot already
    /// showing keeps its sheet, signals and zoom while that analysis offers
    /// the sheet, so a re-run updates the plot in place; a first run opens it
    /// on that analysis' default sheet. A dock the reader collapsed, or left
    /// on another page, stays as they left it. Returns whether the plot is on
    /// screen.
    pub fn plot_finished_run(cx: &mut Cx<'_, Self>, run_sequence: u64) -> bool {
        if !cx.model.simulation.select_run_by_sequence(run_sequence) {
            return false;
        }
        if let Some(index) = settling_analysis(cx.state.reads(cx.model, cx.wb), ShowingRun::Opened)
        {
            settle_shown_analysis(&mut cx.state.results, &mut cx.model.simulation, index);
        }
        let viewer = cx.state.results.session.viewer;
        let keep = cx.wb.dock_shows(crate::workbench::state::ConsolePage::Plot)
            && viewer_is_available(cx.state.reads(cx.model, cx.wb), viewer);
        if !keep {
            let sheet =
                opening_sheet(cx.state.reads(cx.model, cx.wb)).unwrap_or(ResultViewer::Waves);
            cx.state.results.session.viewer = sheet;
        }
        if cx.wb.run_opens_plot_page() {
            cx.wb.show_plot_page();
        }
        cx.wb.dock_shows(crate::workbench::state::ConsolePage::Plot)
    }

    /// Take the navigation offer a notice carried: open Results on the run
    /// it names, or the Console on the line it names. The shell answers the
    /// command it raises straight after.
    pub fn perform_notification_action(cx: &mut Cx<'_, Self>, action: NotificationAction) {
        if follow_notification_action(cx.model, cx.wb, action) {
            cx.execute(ids::OPEN_RUN_IN_RESULTS);
        }
    }
}

/// What the shell's drains hand the results feature once a call has
/// returned, each written as the part that raised it once wrote the results
/// workspace in place.
impl ResultsFeature {
    /// Start the open waveform view again from Results ▸ Waveform defaults,
    /// as a change to them asks.
    pub fn apply_waveform_defaults(cx: &mut Cx<'_, Self>) {
        cx.state
            .start_waveform_viewer_session(&cx.model.preferences);
    }

    /// Show the waveforms a hand-off into Results names, each as its dataset
    /// shows it, with `selected` the selected trace and no cursor left
    /// standing.
    pub fn show_waveforms(
        cx: &mut Cx<'_, Self>,
        visibility: Vec<(SourceWaveformPresentationKey, bool)>,
        selected: Option<SelectedResultTrace>,
    ) {
        let results = &mut cx.state.results;
        results.reveal_waveforms(visibility);
        results.session.selected_trace = selected;
        results.session.clear_cursors();
    }

    /// Narrow the open family form to the members `filter` names.
    pub fn set_family_filter(cx: &mut Cx<'_, Self>, filter: String) {
        crate::workbench::documents::result_document::set_family_filter(
            &mut cx.state.result_document_dialog,
            filter,
        );
    }

    /// Hold that the failure banner of run `run_sequence`, or of none, has
    /// its sites marked on the schematic.
    pub fn mark_failure_run(cx: &mut Cx<'_, Self>, run_sequence: Option<u64>) {
        cx.state.results.marked_failure_run = run_sequence;
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
        registry.dialogs(DIALOGS);
    }
}

/// The data browser, over the results workspace's parts and Run details…,
/// which it opens on a retained run.
fn show_data_browser(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let (mut parts, run_details) = cx.state.parts_and_run_details(
        cx.model,
        cx.wb,
        cx.platform,
        &mut cx.runs.preflight,
        cx.commands,
    );
    crate::workbench::docks::navigator::results::results(ui, &mut parts, run_details);
}

/// The results inspector, over the same parts and Run details…, which it
/// opens on the run it names.
fn show_inspector(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let (mut parts, run_details) = cx.state.parts_and_run_details(
        cx.model,
        cx.wb,
        cx.platform,
        &mut cx.runs.preflight,
        cx.commands,
    );
    crate::workbench::docks::inspector::results::results(ui, &mut parts, run_details);
}

/// The Results stage, over the workspace's parts, the run controller its
/// derived viewers load through, and the specification simulate has
/// selected; then the result document's two forms, the last things the
/// stage draws.
fn show_surface(cx: &mut Cx<'_, ResultsFeature>, ui: &mut egui::Ui) {
    let mut parts = cx.state.parts(
        cx.model,
        cx.wb,
        cx.platform,
        &mut cx.runs.preflight,
        cx.commands,
    );
    surface::show(ui, &mut parts, cx.handles.run_controller);
    render_create_result_document(cx, ui.ctx());
    render_result_document_dialog(cx, ui.ctx());
}

/// The dialogs the feature draws, each under the key the shell lists it by.
/// The dialog order draws the first four at their steps. The rest hold a
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
        key: DialogKey("results.clear_history"),
        is_open: |results| results.clear_history.is_some(),
        render: clear_history::show,
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

/// "Export dataset…" asks for a format, the analysis and the signals, then
/// writes the displayed result as chosen. What the form offers is read from
/// the view once, on the frame it opens. An export written as it is read runs
/// beside the window, one at a time, and is said here when it ends.
fn render_dataset_export(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    if let Some(export) = cx.state.dataset_export_job.take() {
        let running = crate::workbench::menu_bar::waveform_export::finish_export_job(
            &mut results_parts(cx),
            export,
        );
        if running.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        cx.state.dataset_export_job = running;
    }
    if cx.state.dataset_export_form.is_none() {
        let form = crate::workbench::menu_bar::waveform_export::dataset_export_form(
            cx.state.reads(cx.model, cx.wb),
        );
        cx.state.dataset_export_form = Some(Box::new(form));
    }
    if let Some(choice) = crate::workbench::menu_bar::show_dataset_export_picker(
        ctx,
        &mut cx.state.dataset_export,
        &mut cx.state.dataset_export_form,
        &mut cx.model.preferences,
    ) {
        if cx.state.dataset_export_job.is_some() {
            results_parts(cx).prompt(
                "An export is being written. Wait for it to finish, or cancel it in the status \
                 bar, then export again.",
            );
            return;
        }
        let io = cx.handles.export_io;
        cx.state.dataset_export_job =
            crate::workbench::menu_bar::waveform_export::action_export_dataset_with_io(
                &mut results_parts(cx),
                io,
                &choice,
            );
    }
}

/// Run details…, and the route out of it the reader took: the producing
/// plan, the deck the run executed, or an analysis or signal of its
/// inventory.
fn render_run_details(cx: &mut Cx<'_, ResultsFeature>, ctx: &egui::Context) {
    let routes =
        crate::workbench::docks::show_run_details(ctx, &mut cx.state.run_details, cx.model);
    crate::workbench::docks::inspector::run_details::follow_inventory_route(
        &mut results_parts(cx),
        &routes,
    );
    if let Some(run_id) = routes.open_deck {
        cx.wb.outbox().push(RevealExecutedDeck {
            seq: run_id,
            close_manager_on_success: false,
        });
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
    use crate::workbench::documents::result_document::document_edit::{DialogRequest, compare};

    let request = crate::workbench::documents::result_document::show_document_dialog(
        ctx,
        &mut results_parts(cx),
    );
    if let Some(DialogRequest::SaveComparisonReport) = request {
        let io = cx.handles.export_io;
        compare::save_report_with_io(&mut results_parts(cx), io);
    }
}

/// Nothing to commit when another section shows in the inspector's place:
/// an axis interval commits as its field loses focus, and one half typed
/// stays a draft for when the section shows again.
fn keep_axis_drafts(_: &mut Cx<'_, ResultsFeature>, _: &egui::Context) {}

/// The workbench commands the results workspace answers for itself: Copy
/// takes the cursor readout, the zooms and Fit move the active sheet's
/// viewport, Zoom previous steps the waveform sheet back through the views it
/// showed, Undo and Redo step the shown view's edits, Delete takes the
/// selected trace off the sheet, and Cancel lets go of the placed cursors.
/// Each is the workspace's wherever Results is in front, Cancel only while a
/// cursor is placed.
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
        key: ids::ZOOM_PREVIOUS,
        takes: in_results,
        enabled: |reads| zoom_previous_available(results_reads(reads)),
        execute: |cx| zoom_previous(&mut results_parts(cx)),
    },
    // Undo and Redo step the shown view's own edits; in Results they never
    // reach another workspace's history.
    RoutedCommand {
        key: ids::UNDO,
        takes: in_results,
        enabled: |reads| {
            view_edits::shown_view_can_step(results_reads(reads), view_edits::Step::Undo)
        },
        execute: |cx| view_edits::step_shown_view(&mut results_parts(cx), view_edits::Step::Undo),
    },
    RoutedCommand {
        key: ids::REDO,
        takes: in_results,
        enabled: |reads| {
            view_edits::shown_view_can_step(results_reads(reads), view_edits::Step::Redo)
        },
        execute: |cx| view_edits::step_shown_view(&mut results_parts(cx), view_edits::Step::Redo),
    },
    // Hiding is the reversible removal: the trace comes back from the
    // pane's signal menu, and Undo puts it back.
    RoutedCommand {
        key: ids::DELETE,
        takes: in_results,
        enabled: |reads| selected_trace_on_sheet(results_reads(reads)).is_some(),
        execute: |cx| hide_selected_trace(&mut results_parts(cx)),
    },
    RoutedCommand {
        key: ids::CANCEL,
        takes: |reads| in_results(reads) && reads.state.results.session.cursors.any(),
        enabled: |_| true,
        execute: |cx| cx.state.results.session.clear_cursors(),
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

/// Move the stepping cursor, saying on the status bar why when it cannot.
fn jump(cx: &mut Cx<'_, ResultsFeature>, to: CursorJump) {
    let mut parts = results_parts(cx);
    if let Err(refusal) = jump_cursor(&mut parts, &rspice_ui_kit::tokens::Tokens::default(), to) {
        parts.prompt(format!("{refusal}."));
    }
}

/// Move the stepping cursor to the next or previous place the compared runs
/// part, saying on the status bar why when it cannot.
fn step_difference(cx: &mut Cx<'_, ResultsFeature>, direction: Direction) {
    let mut parts = results_parts(cx);
    if let Err(refusal) =
        crate::workbench::documents::result_document::document_edit::compare::step_difference(
            &mut parts,
            &rspice_ui_kit::tokens::Tokens::default(),
            direction,
        )
    {
        parts.prompt(refusal);
    }
}

/// What the results workspace's predicates read of a command's reads.
fn results_reads<'a>(reads: &Reads<'a, ResultsFeature>) -> ResultsReads<'a> {
    reads.state.reads(reads.model, reads.wb)
}

/// What the results workspace's operations write of a command's context.
fn results_parts<'a>(cx: &'a mut Cx<'_, ResultsFeature>) -> ResultsParts<'a> {
    cx.state.parts(
        cx.model,
        cx.wb,
        cx.platform,
        &mut cx.runs.preflight,
        cx.commands,
    )
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
        let on_results_surface = workbench.working_route().surface_id() == SurfaceId::Results;
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
            // Its dialog also offers back the runs deleted or cleared, so it
            // stays open to them once every run is gone.
            Self::ClearResults => {
                if executing {
                    CommandAvailability::Disabled(
                        "an active simulation execution still owns result history",
                    )
                } else if retained.has_results()
                    || crate::workbench::docks::navigator::results_runs::restore_refusal(
                        &model.simulation,
                    )
                    .is_none()
                {
                    CommandAvailability::Available
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
            Self::ShowPlot => {
                if !workbench.plots_in_dock() {
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
            Self::WaveformCalculator => CommandAvailability::Available,
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
                "open an active result document",
            ),
            // Waves is the workspace's default sheet, and its empty state is
            // the workspace's landing, so this route stays actionable before a
            // dataset exists. Every other viewer is only actionable when the
            // active retained dataset satisfies the same compatibility
            // contract used by the in-workspace viewer tabs.
            Self::CursorToMaximum
            | Self::CursorToMinimum
            | Self::CursorToNextPeak
            | Self::CursorToPreviousPeak
            | Self::CursorToNextCrossing
            | Self::CursorToPreviousCrossing => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    cursor_jump_availability(state)
                        .map_or_else(CommandAvailability::Disabled, |()| {
                            CommandAvailability::Available
                        })
                }
            }
            Self::CursorToNextDifference | Self::CursorToPreviousDifference => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else if !crate::workbench::documents::result_document::document_edit::compare::compared(state) {
                    CommandAvailability::Disabled("compare the result with a baseline first")
                } else {
                    cursor_jump_availability(state)
                        .map_or_else(CommandAvailability::Disabled, |()| {
                            CommandAvailability::Available
                        })
                }
            }
            // The arrows step a cursor that is there to step; with none
            // placed, or a widget of the sheet holding the keyboard, they
            // are left to whatever else takes them.
            Self::CursorToNextSample | Self::CursorToPreviousSample => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else if let Err(reason) = cursor_jump_availability(state) {
                    CommandAvailability::Disabled(reason)
                } else if state.results.sheet_focus {
                    CommandAvailability::Disabled(
                        "the focused plot, lane or ruler takes the arrows; press Escape to give them back",
                    )
                } else {
                    available_or(
                        state.results.session.cursor_readout_active(),
                        "place cursor A on a waveform in Results",
                    )
                }
            }
            Self::ToggleCursorTool => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    cursor_jump_availability(state)
                        .map_or_else(CommandAvailability::Disabled, |()| {
                            CommandAvailability::Available
                        })
                }
            }
            Self::SwitchSteppingCursor => available_or(
                workbench.workspace == Workspace::Results && state.results.session.cursors.any(),
                "place a cursor in Results",
            ),
            Self::SaveViewAsDocument => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else if model.safe_mode.project_read_only() {
                    CommandAvailability::Disabled("the project is open read-only")
                } else {
                    crate::workbench::chrome::document_bar::results::save_view_refusal(state)
                        .map_or(
                            CommandAvailability::Available,
                            CommandAvailability::Disabled,
                        )
                }
            }
            Self::SendTraceToCalculator => selected_trace(state.results, &model.simulation).map_or(
                CommandAvailability::Disabled("select a trace in Results"),
                |_| CommandAvailability::Available,
            ),
            Self::TraceSpectrum => match selected_trace(state.results, &model.simulation) {
                None => CommandAvailability::Disabled("select a trace in Results"),
                Some((_, false)) => {
                    CommandAvailability::Disabled("a spectrum is taken of a transient waveform")
                }
                Some((_, true)) => CommandAvailability::Available,
            },
            Self::OverlayPrevious(_) => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    available_or(retained.has_results(), "no result dataset is available")
                }
            }
            Self::RenameDocument | Self::DuplicateDocument | Self::DeleteDocument => {
                if workbench.workspace != Workspace::Results {
                    CommandAvailability::Disabled("open a result in Results")
                } else {
                    crate::workbench::chrome::document_bar::results::document_refusal(state).map_or(
                        CommandAvailability::Available,
                        CommandAvailability::Disabled,
                    )
                }
            }
            Self::RestoreDeletedRuns => {
                crate::workbench::docks::navigator::results_runs::restore_refusal(&model.simulation)
                    .map_or(
                        CommandAvailability::Available,
                        CommandAvailability::Disabled,
                    )
            }
            Self::MarkerAtCursor => available_or(
                workbench.workspace == Workspace::Results
                    && state.results.session.cursors.a.is_some(),
                "place cursor A on a waveform in Results",
            ),
            Self::FitYToVisibleX => available_or(
                workbench.workspace == Workspace::Results && zoom_gesture_available(state),
                "open a waveform sheet in Results",
            ),
            Self::AtPointer(_) | Self::ToggleMinorGrid
                if workbench.workspace != Workspace::Results =>
            {
                CommandAvailability::Disabled("open a result in Results")
            }
            Self::AtPointer(action) => {
                crate::workbench::documents::result_document::waves::pointer::availability(
                    state, action,
                )
                .map_or_else(CommandAvailability::Disabled, |()| {
                    CommandAvailability::Available
                })
            }
            Self::ToggleMinorGrid => available_or(
                zoom_gesture_available(state),
                "open a waveform sheet in Results",
            ),
            Self::ZoomNext if workbench.workspace != Workspace::Results => {
                CommandAvailability::Hidden
            }
            Self::ZoomNext => available_or(
                zoom_next_available(state),
                "Zoom previous has not stepped back from a later view",
            ),
            Self::PlotAgainst => available_or(
                workbench.workspace == Workspace::Results
                    && state.results.session.active_wave_pane.is_some()
                    && zoom_gesture_available(state),
                "click a waveform strip in Results",
            ),
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
            Self::ClearResults => {
                cx.state.clear_history = Some(clear_history::ClearHistoryDialog::default());
            }
            Self::ToggleLinkedCursors => {
                crate::workbench::documents::result_document::toggle_linked_cursors(
                    &mut results_parts(cx),
                );
            }
            Self::ShowPlot => {
                if cx.wb.dock_shows(crate::workbench::state::ConsolePage::Plot) {
                    cx.wb.collapse_dock();
                } else {
                    // A run already shown stays shown; otherwise the plot
                    // starts on the newest materialized dataset. Either way it
                    // is the one canonical Results selection: there is no
                    // copied run, waveform, or document owner.
                    let shown = cx
                        .model
                        .simulation
                        .active_run()
                        .is_some_and(|run| !run.analyses.is_empty());
                    if !shown
                        && let Some(run_index) = cx
                            .model
                            .simulation
                            .retained
                            .newest_retained_result_run_index()
                    {
                        cx.model.simulation.select_run(run_index);
                    }
                    cx.wb.show_plot_page();
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
            Self::CursorToMaximum => jump(cx, CursorJump::Maximum),
            Self::CursorToMinimum => jump(cx, CursorJump::Minimum),
            Self::CursorToNextPeak => jump(cx, CursorJump::Peak(Direction::Next)),
            Self::CursorToPreviousPeak => jump(cx, CursorJump::Peak(Direction::Previous)),
            Self::CursorToNextCrossing => jump(cx, CursorJump::Crossing(Direction::Next)),
            Self::CursorToPreviousCrossing => {
                jump(cx, CursorJump::Crossing(Direction::Previous));
            }
            Self::CursorToNextDifference => step_difference(cx, Direction::Next),
            Self::CursorToPreviousDifference => step_difference(cx, Direction::Previous),
            Self::CursorToNextSample => jump(cx, CursorJump::Sample(Direction::Next)),
            Self::CursorToPreviousSample => jump(cx, CursorJump::Sample(Direction::Previous)),
            Self::ToggleCursorTool => cx.state.results.session.toggle_cursor_tool(),
            Self::SwitchSteppingCursor => cx.state.results.session.switch_stepping_cursor(),
            Self::SaveViewAsDocument => {
                let mut parts = results_parts(cx);
                let said =
                    crate::workbench::chrome::document_bar::results::save_shown_view(&mut parts)
                        .unwrap_or_else(|error| format!("Could not save the view: {error}"));
                parts.prompt(said);
            }
            Self::SendTraceToCalculator => {
                if let Some((name, _)) = selected_trace(&cx.state.results, &cx.model.simulation) {
                    let panel = &mut cx.state.calculator_panel;
                    panel.clear();
                    panel.expression = name;
                    cx.state.waveform_calculator_dialog = true;
                }
            }
            Self::TraceSpectrum => {
                if let Some((name, true)) = selected_trace(&cx.state.results, &cx.model.simulation)
                {
                    cx.state.analysis.fft_state.choose_source_from_trace(name);
                    // The spectrum on hand is of another trace; the next frame
                    // takes this one's.
                    cx.state.analysis.clear_fft_viewer_cache_authority();
                    cx.state.results.session.viewer = ResultViewer::Fft;
                    cx.wb.activate(Workspace::Results);
                }
            }
            Self::OverlayPrevious(depth) => {
                let view = &mut cx.model.simulation.view;
                view.overlay_previous = usize::from(depth);
                view.data_version = view.data_version.wrapping_add(1);
            }
            Self::RenameDocument => {
                crate::workbench::chrome::document_bar::results::open_rename(&mut results_parts(
                    cx,
                ));
            }
            Self::DuplicateDocument => {
                let mut parts = results_parts(cx);
                let said =
                    crate::workbench::chrome::document_bar::results::duplicate_shown_document(
                        &mut parts,
                    )
                    .unwrap_or_else(|error| format!("Could not duplicate the document: {error}"));
                parts.prompt(said);
            }
            Self::DeleteDocument => {
                let mut parts = results_parts(cx);
                let said = crate::workbench::chrome::document_bar::results::delete_shown_document(
                    &mut parts,
                )
                .unwrap_or_else(|error| format!("Could not delete the document: {error}"));
                parts.prompt(said);
            }
            Self::RestoreDeletedRuns => {
                let mut parts = results_parts(cx);
                let said = crate::workbench::docks::navigator::results_runs::restore_deleted_runs(
                    &mut parts,
                )
                .unwrap_or_else(|error| error);
                parts.prompt(said);
            }
            Self::MarkerAtCursor => place_marker_at_cursor(&mut results_parts(cx)),
            Self::FitYToVisibleX => fit_y_to_visible_x(&mut results_parts(cx)),
            Self::ZoomNext => zoom_next(&mut results_parts(cx)),
            Self::AtPointer(action) => {
                crate::workbench::documents::result_document::waves::pointer::act(
                    &mut results_parts(cx),
                    action,
                );
            }
            Self::ToggleMinorGrid => {
                let session = &mut cx.state.results.session;
                session.show_minor_grid = !session.show_minor_grid;
            }
            Self::PlotAgainst => {
                let session = &mut cx.state.results.session;
                session.plot_against_request = session.active_wave_pane.clone();
            }
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
                Ok(sequence) => cx.wb.outbox().push(RevealExecutedDeck {
                    seq: sequence,
                    close_manager_on_success: false,
                }),
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
            Self::ShowPlot => Some(
                reads
                    .wb
                    .dock_shows(crate::workbench::state::ConsolePage::Plot),
            ),
            Self::ToggleLinkedCursors => Some(linked_cursors(results_reads(reads))),
            Self::ToggleCursorTool => Some(reads.state.results.session.cursor_tool.is_armed()),
            Self::ToggleMinorGrid => Some(reads.state.results.session.show_minor_grid),
            Self::OverlayPrevious(depth) => {
                Some(reads.model.simulation.view.overlay_previous == usize::from(depth))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;

/// The trace selected in the analysis Results shows, by name, and whether
/// that analysis runs in time, so a spectrum can be taken of it.
fn selected_trace(
    results: &crate::workbench::documents::result_document::ResultsState,
    simulation: &crate::state::SimulationState,
) -> Option<(String, bool)> {
    let selected = results.valid_selected_trace(simulation)?;
    let run = simulation.active_run()?;
    let (analysis_index, _, analysis, _) = selected.resolve(run)?;
    (simulation.view.active_analysis_idx == Some(analysis_index)).then(|| {
        (
            selected.source_name().to_owned(),
            analysis.analysis_type.is_time_domain(),
        )
    })
}
