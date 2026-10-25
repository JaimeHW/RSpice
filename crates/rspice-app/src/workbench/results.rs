//! The results feature: the retained result datasets as the reader works
//! with them, through the result viewers, the result document's forms and
//! tools, and the hops back out of a run.
//!
//! Its commands run in a [`Cx`] over the feature's own slot, the model,
//! whose retained runs they read and clear, the session, whose workspace
//! and route they open, and the platform services' state, which says
//! whether a project is open.

pub(crate) mod commands;

use crate::workbench::commands::result_navigation::{
    dataset_export_is_resolvable, open_run_log, producer_log_hop, producing_plan_hop, task_deck_hop,
};
use crate::workbench::documents::result_document::{
    ResultsParts, ResultsReads, add_pane_available, compare_available, family_available,
    linked_cursors, open_create_document, open_dataset_browser, open_specification_editor,
    viewer_is_available, viewer_unavailability_reason,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureRegistry, Reads, Session,
};
use crate::workbench::framework::requests::{
    ExportDataset, RevealExecutedDeck, SelectAnalysisInstance,
};
use crate::workbench::slots::ResultsState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::workflows::result_import_workflow::import_result_dataset;
use crate::workbench::{ResultViewer, SurfaceId};

use commands::ResultsCommand;

/// The results feature.
pub(crate) struct ResultsFeature;

impl Feature for ResultsFeature {
    type State = ResultsState;
    type Session = WorkbenchSession;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<ResultsCommand>();
    }
}

/// What the results workspace's predicates read of a command's reads.
fn results_reads<'a>(reads: &Reads<'a, ResultsFeature>) -> ResultsReads<'a> {
    reads.state.reads(reads.model, reads.wb)
}

/// What the results workspace's operations write of a command's context.
fn results_parts<'a>(cx: &'a mut Cx<'_, ResultsFeature>) -> ResultsParts<'a> {
    cx.state.parts(cx.model, cx.wb, cx.platform)
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
            Self::CompareResultDatasets => available_or(
                compare_available(state),
                if workbench.workspace != Workspace::Results {
                    "open an active result document"
                } else {
                    "a second compatible run is required"
                },
            ),
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
