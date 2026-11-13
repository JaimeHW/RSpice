//! The runs a saved result document reads.
//!
//! A document references each run by identity and content digest and reads
//! its samples from the run history, so it is no larger for a larger run.
//! When a run is gone, or its results are no longer the ones the document was
//! bound to, the document says so plainly, keeps every page and pane, and
//! offers the retained runs with the same signals to read instead. It never
//! draws another run's data as the original's.
//!
//! An archived document holds each run's own record. Opened without the run,
//! it brings the run back into the history, after checking that the restored
//! run has the bound identity and digest.

use std::collections::HashSet;

use egui::{Sense, Ui, pos2, vec2};
use rspice_formats::project_results::ProjectSimulationRun;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets;

use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::visualization_document::{
    AnalysisRepoint, DocumentEdit, RunArchive, SourceDataset, VisualizationDocument,
};
use crate::state::SimulationRun;

use super::super::create_document::{
    reference_run_label, run_source_analysis, source_analysis_identity,
};
use super::super::{ResultsParts, ResultsReads, retained_dataset_digest};
use super::renderer_supports_analysis;

/// The encoding an archived run's record is written in: the run exactly as
/// a project writes it among its results.
const RUN_ARCHIVE_ENCODING: &str = "rspice-project-run";

/// Why the run history cannot give a document one of the runs it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceProblem {
    /// No retained run has the dataset's identity.
    Missing,
    /// The retained run with that identity no longer holds the bound results.
    Changed,
}

/// One run a document reads that the run history cannot give it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnavailableSource {
    pub binding: DatasetBinding,
    pub run_label: String,
    pub problem: SourceProblem,
    /// Retained runs with the same signals, as (dataset, label).
    pub candidates: Vec<(DatasetId, String)>,
}

impl UnavailableSource {
    /// What the document says about it.
    #[must_use]
    pub fn reason(&self) -> String {
        match self.problem {
            SourceProblem::Missing => format!(
                "{}, which this document reads, is no longer in the run history.",
                self.run_label
            ),
            SourceProblem::Changed => format!(
                "{} no longer holds the results this document was made from.",
                self.run_label
            ),
        }
    }
}

/// Whether Archive into document can run on a document, and why not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveAvailability {
    Available,
    /// The document already holds every sample it shows.
    Archived,
    Unavailable(String),
}

/// Every dataset the document's panes and traces read; a dataset only a
/// comparison receipt names is evidence, not something the document draws.
fn consumed_bindings(document: &VisualizationDocument) -> Vec<DatasetBinding> {
    let mut bindings = Vec::new();
    let panes = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding.map(|binding| binding.dataset));
    let traces = document.traces().iter().map(|trace| trace.binding);
    for binding in panes.chain(traces) {
        if !bindings.contains(&binding) {
            bindings.push(binding);
        }
    }
    bindings
}

fn dataset(document: &VisualizationDocument, binding: DatasetBinding) -> Option<&SourceDataset> {
    document
        .datasets()
        .iter()
        .find(|dataset| dataset.binding() == binding)
}

fn retained_run(state: ResultsReads<'_>, dataset_id: DatasetId) -> Option<&SimulationRun> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
}

fn source_problem(state: ResultsReads<'_>, binding: DatasetBinding) -> Option<SourceProblem> {
    let Some(run) = retained_run(state, binding.dataset_id) else {
        return Some(SourceProblem::Missing);
    };
    (retained_dataset_digest(state, run) != binding.content_digest)
        .then_some(SourceProblem::Changed)
}

fn run_label(document: &VisualizationDocument, binding: DatasetBinding) -> String {
    dataset(document, binding)
        .and_then(SourceDataset::run_source)
        .map_or_else(|| "The run".to_owned(), |run| run.run_label().to_owned())
}

/// The runs `document_id` reads that the run history cannot give it, each
/// with the retained runs that could stand in for it.
pub fn unavailable_sources(
    state: ResultsReads<'_>,
    document_id: ResultDocumentId,
) -> Vec<UnavailableSource> {
    let Some(document) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
    else {
        return Vec::new();
    };
    consumed_bindings(document)
        .into_iter()
        .filter_map(|binding| {
            let problem = source_problem(state, binding)?;
            let candidates = state
                .model
                .simulation
                .retained
                .runs
                .iter()
                .filter(|run| run.dataset_id != binding.dataset_id)
                .filter(|run| repoint_analyses(document, binding, run).is_some())
                .map(|run| (run.dataset_id, reference_run_label(run)))
                .collect();
            Some(UnavailableSource {
                binding,
                run_label: run_label(document, binding),
                problem,
                candidates,
            })
        })
        .collect()
}

/// For each analysis the document reads from `previous`, the analysis of
/// `run` that stands in for it: the same authored analysis when the run is a
/// rerun of it, otherwise the one analysis of the same kind. Every pane must
/// be able to show it and every trace must find its signal there by name.
fn repoint_analyses(
    document: &VisualizationDocument,
    previous: DatasetBinding,
    run: &SimulationRun,
) -> Option<Vec<AnalysisRepoint>> {
    let source = dataset(document, previous)?;
    let mut read = Vec::<AnalysisInstanceId>::new();
    let pane_analyses = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding)
        .filter(|binding| binding.dataset == previous)
        .map(|binding| binding.analysis_id);
    let trace_analyses = document
        .traces()
        .iter()
        .filter(|trace| trace.binding == previous)
        .filter_map(|trace| trace.source_analysis());
    for analysis in pane_analyses.chain(trace_analyses) {
        if !read.contains(&analysis) {
            read.push(analysis);
        }
    }
    let mut mappings = Vec::with_capacity(read.len());
    for previous_analysis in read {
        let same_authored = run
            .analyses
            .iter()
            .find(|analysis| source_analysis_identity(run, analysis) == previous_analysis);
        let candidate = same_authored.or_else(|| {
            let kind = source
                .run_source()?
                .analysis(previous_analysis)?
                .analysis_type();
            let mut same_kind = run
                .analyses
                .iter()
                .filter(|analysis| analysis.analysis_type == kind);
            let only = same_kind.next()?;
            same_kind.next().is_none().then_some(only)
        })?;
        let panes_show_it = document
            .panes()
            .iter()
            .filter(|pane| {
                pane.binding.is_some_and(|binding| {
                    binding.dataset == previous && binding.analysis_id == previous_analysis
                })
            })
            .all(|pane| renderer_supports_analysis(&pane.viewer_id, candidate));
        let signals_found = document
            .traces()
            .iter()
            .filter(|trace| {
                trace.binding == previous
                    && trace
                        .source_analysis()
                        .is_none_or(|analysis| analysis == previous_analysis)
            })
            .all(|trace| {
                let signal = trace.source_signal().unwrap_or(&trace.label);
                candidate
                    .waveforms
                    .iter()
                    .filter(|waveform| waveform.name == signal)
                    .count()
                    == 1
            });
        if !panes_show_it || !signals_found {
            return None;
        }
        mappings.push(AnalysisRepoint {
            previous: previous_analysis,
            next: source_analysis_identity(run, candidate),
        });
    }
    (!mappings.is_empty()).then_some(mappings)
}

/// Read the retained run `dataset_id` in place of the run behind `previous`,
/// keeping every page, pane and trace of the document.
pub fn repoint(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    previous: DatasetBinding,
    dataset_id: DatasetId,
) -> Result<(), String> {
    let reads = state.results_reads();
    let document = reads
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The result document is no longer in the project.".to_owned())?;
    let run = retained_run(reads, dataset_id)
        .ok_or_else(|| "The chosen run is no longer in the run history.".to_owned())?;
    let analyses = repoint_analyses(document, previous, run).ok_or_else(|| {
        format!(
            "{} does not hold the signals this document reads.",
            reference_run_label(run)
        )
    })?;
    let catalogs = analyses
        .iter()
        .map(|mapping| {
            let analysis = run
                .analyses
                .iter()
                .find(|analysis| source_analysis_identity(run, analysis) == mapping.next)
                .ok_or_else(|| "The chosen run no longer holds the analysis.".to_owned())?;
            run_source_analysis(run, analysis).map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    let next = SourceDataset::run_reference(
        DatasetBinding::new(run.dataset_id, retained_dataset_digest(reads, run)),
        reference_run_label(run),
        catalogs,
    )
    .map_err(|error| error.to_string())?;
    let revision = document.revision();
    let label = reference_run_label(run);
    state
        .model
        .workspace
        .content
        .transact_visualization_document(
            document_id,
            revision,
            vec![DocumentEdit::RepointDataset {
                previous,
                next,
                analyses,
            }],
        )
        .map_err(|error| error.to_string())?;
    state
        .workbench
        .prompt(format!("The result document now reads {label}."));
    Ok(())
}

/// Bring back each archived run the document reads that the run history no
/// longer holds. A run is restored only when it has the bound identity and
/// digest; a refusal is remembered so it is not re-attempted every frame.
pub fn restore_archived_runs(state: &mut ResultsParts<'_>, document_id: ResultDocumentId) {
    let pending = {
        let reads = state.results_reads();
        let Some(document) = reads
            .model
            .workspace
            .content
            .visualization_document(document_id)
        else {
            return;
        };
        consumed_bindings(document)
            .into_iter()
            .filter(|binding| {
                retained_run(reads, binding.dataset_id).is_none()
                    && reads.results.archive_restore_failure(*binding).is_none()
            })
            .filter_map(|binding| {
                let archive = dataset(document, binding)?.run_source()?.archive()?.clone();
                Some((binding, archive))
            })
            .collect::<Vec<_>>()
    };
    for (binding, archive) in pending {
        if let Err(reason) = restore_archived_run(state, binding, &archive) {
            state
                .results
                .record_archive_restore_failure(binding, reason);
        }
    }
}

fn restore_archived_run(
    state: &mut ResultsParts<'_>,
    binding: DatasetBinding,
    archive: &RunArchive,
) -> Result<(), String> {
    if archive.encoding() != RUN_ARCHIVE_ENCODING {
        return Err(format!(
            "The document archives its run as '{}', which this version of RSpice does not read.",
            archive.encoding()
        ));
    }
    let record: ProjectSimulationRun = serde_json::from_str(archive.record())
        .map_err(|error| format!("The document's archived run does not read back: {error}."))?;
    let restored = rspice_app_services::io::restore_project_run(record)?;
    if restored.dataset_id != binding.dataset_id
        || restored.dataset_content_digest() != binding.content_digest
    {
        return Err(
            "The document's archived run is not the run it was made from, so it is not restored."
                .to_owned(),
        );
    }
    let simulation = &mut state.model.simulation;
    let slot = simulation.start_imported_run()?;
    let sequence = slot.id;
    *slot = restored;
    slot.id = sequence;
    simulation.complete_run();
    Ok(())
}

/// Whether Archive into document can run on `document_id`.
pub fn archive_availability(
    state: ResultsReads<'_>,
    document_id: ResultDocumentId,
) -> ArchiveAvailability {
    let Some(document) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
    else {
        return ArchiveAvailability::Unavailable(
            "The result document is no longer in the project.".to_owned(),
        );
    };
    if document.is_archived() {
        return ArchiveAvailability::Archived;
    }
    for dataset in document.datasets() {
        if dataset.is_self_contained() {
            continue;
        }
        let binding = dataset.binding();
        if source_problem(state, binding).is_some() {
            return ArchiveAvailability::Unavailable(format!(
                "{} is not in the run history with the results this document was made from, so its samples cannot be archived.",
                run_label(document, binding)
            ));
        }
    }
    ArchiveAvailability::Available
}

/// Embed every run the document reads, so it stands alone: shared without
/// the project's run history, it still shows what it showed.
pub fn archive_document(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
) -> Result<(), String> {
    match archive_availability(state.results_reads(), document_id) {
        ArchiveAvailability::Available => {}
        ArchiveAvailability::Archived => {
            return Err("This document already holds its runs' samples.".to_owned());
        }
        ArchiveAvailability::Unavailable(reason) => return Err(reason),
    }
    let reads = state.results_reads();
    let document = reads
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The result document is no longer in the project.".to_owned())?;
    let mut edits = Vec::new();
    let mut archived = HashSet::new();
    for dataset in document.datasets() {
        let binding = dataset.binding();
        if dataset.is_self_contained() || !archived.insert(binding) {
            continue;
        }
        let run = retained_run(reads, binding.dataset_id)
            .ok_or_else(|| "A run this document reads left the run history.".to_owned())?;
        let record = serde_json::to_string(&ProjectSimulationRun::from(run))
            .map_err(|error| format!("The run could not be written into the document: {error}."))?;
        let archive =
            RunArchive::new(RUN_ARCHIVE_ENCODING, record).map_err(|error| error.to_string())?;
        edits.push(DocumentEdit::ArchiveDataset { binding, archive });
    }
    let revision = document.revision();
    state
        .model
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)
        .map_err(|error| error.to_string())?;
    state.workbench.prompt(
        "The result document now holds its runs' samples and reads without the run history.",
    );
    Ok(())
}

/// Say which runs the document cannot read, and offer the runs that could
/// stand in for each. Bring archived runs back first.
pub(super) fn show_banner(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
) {
    restore_archived_runs(state, document_id);
    let sources = unavailable_sources(state.results_reads(), document_id);
    let mut chosen = None;
    for source in &sources {
        let reason = state
            .results
            .archive_restore_failure(source.binding)
            .map_or_else(
                || source.reason(),
                |failure| format!("{} {failure}", source.reason()),
            );
        if let Some(dataset_id) = banner_row(ui, &reason, &source.candidates) {
            chosen = Some((source.binding, dataset_id));
        }
    }
    if let Some((previous, dataset_id)) = chosen
        && let Err(error) = repoint(state, document_id, previous, dataset_id)
    {
        state.workbench.prompt(error);
    }
}

/// One line of the banner: the reason, then a choice of run to read instead.
fn banner_row(ui: &mut Ui, reason: &str, candidates: &[(DatasetId, String)]) -> Option<DatasetId> {
    let t = Tokens::get(ui.ctx());
    let height = ui.text_style_height(&egui::TextStyle::Body) + 10.0;
    let mut chosen = None;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    widgets::paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(pos2(rect.left() + 13.5, rect.center().y), vec2(11.0, 11.0)),
        widgets::Problem::Warning,
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(0.0, 2.0)).with_min_x(rect.left() + 25.0))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.label(
                egui::RichText::new(reason)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, reason));
            match candidates {
                [] => {}
                [(dataset_id, label)] => {
                    if ui.button(format!("Read {label} instead")).clicked() {
                        chosen = Some(*dataset_id);
                    }
                }
                _ => {
                    ui.menu_button("Read another run instead", |ui| {
                        for (dataset_id, label) in candidates {
                            if ui.button(label).clicked() {
                                chosen = Some(*dataset_id);
                                ui.close();
                            }
                        }
                    });
                }
            }
        },
    );
    chosen
}

#[cfg(test)]
mod tests;
