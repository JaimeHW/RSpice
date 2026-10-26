//! Exact, semantic hardcopy source resolution.
//!
//! Hardcopy starts here rather than at a viewport or GPU surface.  Every
//! adapter freezes one durable document revision, resolves authored symbol
//! artwork and retained result samples, computes deterministic physical
//! bounds, and authenticates the semantic snapshot before rendering begins.
//! No type in this module contains pixels, an egui paint command, or a screen
//! rectangle.

mod quick_view_overlay;
mod report_inventory;

pub(crate) use rspice_hardcopy::sources::*;
// Module-private: `noise` and `quick_view_overlay`
// expose only `pub(super)` items, and the siblings reach them through
// `use super::*`.
use quick_view_overlay::*;

#[cfg(test)]
use crate::state::{
    DrawingSheetTitleFieldId, SchematicSheetFormat, SchematicState, Selection, SymbolDocument, Wire,
};

use uuid::Uuid;

use crate::product::DatasetId;
#[cfg(test)]
use crate::product::{ContentDigest, ObjectRevision};
#[cfg(test)]
use crate::results::report_document::{
    FigureSizing, ReportBlockKind, ReportReferenceCurrentness, ReportReferenceMode,
};
#[cfg(test)]
use crate::results::report_document::{ReportDocument, ReportReferenceInventory};
use crate::results::visualization_document::{Page, Pane, VisualizationDocument};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    SheetCatalog, SheetId, SimulationRun, ViewType,
};
use crate::workbench::AppState;

use crate::workbench::SurfaceId;
use rspice_hardcopy_contract::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
use rspice_results::renderability::{
    analysis_supports_viewer, bode_analysis_is_renderable, harmonic_spectrum_is_renderable,
    ordinary_noise_spectrum_is_renderable, phase_noise_is_renderable,
    qpnoise_spectrum_is_renderable, selected_noise_analysis_index, smith_analysis_is_renderable,
    transient_waveform_is_renderable,
};
// The persisted source-set records and the validation they share with these
// adapters are owned one layer down, where `state` can reach them.
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::framework::ports::ResultsPort;
use crate::workbench::state::{WorkbenchSession, Workspace, WorkspaceDocumentId};
use rspice_app_state::workbench::app_state::model::Model;
#[cfg(test)]
use rspice_hardcopy_contract::sources::canonical_digest;
use rspice_hardcopy_contract::sources::{
    HardcopyPublicationIdentity, HardcopySourceError, HardcopySourceIdentity, HardcopySourceSet,
    MAX_HARDCOPY_SOURCE_SET_MEMBERS,
};
use rspice_results_ui::report::session::ReportAuthoringState;

/// What a retained hardcopy source is enumerated and resolved from: the
/// model, the session's open documents, the report editor's selection, and
/// what the results workspace shows.
#[derive(Clone, Copy)]
pub(crate) struct SourceReads<'a> {
    pub(crate) model: &'a Model,
    pub(crate) workbench: &'a WorkbenchSession,
    pub(crate) report_authoring: &'a ReportAuthoringState,
    pub(crate) results: &'a dyn ResultsPort,
}

impl<'a> SourceReads<'a> {
    /// The reads `state` holds.
    pub(crate) fn of(state: &'a AppState) -> Self {
        Self {
            model: &state.model,
            workbench: &state.workbench,
            report_authoring: &state.hardcopy.report_authoring,
            results: &*state.results,
        }
    }
}

/// Cheap, semantic-free descriptor used by command enablement and the
/// hardcopy dialog's document/scope selectors. Building this value never
/// clones an engineering document, resolves a plot scene, or hashes samples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetainedHardcopySourceDescriptor {
    pub source_key: String,
    pub display_name: String,
    pub document_kind: HardcopyDocumentKind,
    pub allowed_scopes: Vec<HardcopyScope>,
    pub availability: RetainedHardcopySourceAvailability,
}

impl RetainedHardcopySourceDescriptor {
    #[must_use]
    pub fn supports_scope(&self, scope: &HardcopyScope) -> bool {
        self.allowed_scopes.contains(scope)
    }
}

/// The retained run a quick-view capture is taken from.
///
/// The active Results document is the authority, exactly as it is for the
/// descriptor that offers the page. The simulation's own selection stands in
/// only when no result document is open — a capture reached from the command
/// palette rather than from the workspace.
fn captured_results_run(reads: SourceReads<'_>) -> Option<&SimulationRun> {
    match reads.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => reads
            .model
            .simulation
            .retained
            .run_by_dataset_id(*dataset_id),
        _ => reads.model.simulation.active_run(),
    }
}

fn capture_results_quick_view_presentation(
    reads: SourceReads<'_>,
) -> Result<ResultsQuickViewPresentation, HardcopySourceError> {
    let fft = reads.results.fft_settings();
    let histogram = reads.results.histogram_settings();
    let view = reads.results.plot_view(ResultViewer::Hist, 0);
    let overlay = captured_results_run(reads)
        .map(|run| capture_quick_view_overlays(reads, run))
        .transpose()?
        .unwrap_or_default();
    ResultsQuickViewPresentation::try_new(
        reads.results.result_viewer(),
        overlay,
        reads.model.active_run_specifications(),
        QuickFftSettings {
            selected_source: fft.selected_source.clone(),
            normalization: fft.normalization,
            window: fft.window,
            input_fidelity: fft.input_fidelity,
            time_window_auto: fft.time_window_auto,
            time_window_start: fft.time_window_start,
            time_window_end: fft.time_window_end,
            sample_count_auto: fft.sample_count_auto,
            sample_count: fft.sample_count,
        },
        QuickHistogramSettings {
            x: view.x,
            y: view.y,
            selected: 0,
            measurement: histogram.selected.clone(),
            bin_count: histogram.bin_count,
            custom_range: histogram.custom_range,
            custom_min: histogram.custom_min,
            custom_max: histogram.custom_max,
            mode: histogram.mode,
        },
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetainedHardcopySourceAvailability {
    Available,
    Unavailable { reason: String },
}

impl RetainedHardcopySourceAvailability {
    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }
}

/// Enumerate retained application-owned hardcopy choices without materializing
/// their semantic content. A descriptor can be unavailable when its retained
/// owner exists but lacks the exact evidence needed by the selected viewer.
#[must_use]
pub(crate) fn enumerate_retained_hardcopy_sources(
    reads: SourceReads<'_>,
) -> Vec<RetainedHardcopySourceDescriptor> {
    let project_id = reads.model.workspace.content.project.id();
    let mut descriptors = Vec::new();

    if matches!(
        reads.workbench.documents.active(Workspace::Design),
        Some(WorkspaceDocumentId::CellView(reference)) if reference == &reads.model.workspace.content.active_view
    ) {
        let view_type = reads.model.workspace.content.active_view_type();
        let active_key = reads.model.workspace.content.active_key();
        let design_source_key = format!("project:{}:cell-view:{active_key}", project_id.as_uuid());
        let supported = matches!(
            view_type,
            ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
        );
        let schematic = matches!(view_type, ViewType::Schematic | ViewType::Testbench);
        // Every sheet of a cell view shares one coordinate space, so a page
        // that ignored the catalog would draw the sheets on top of each other.
        // Where there are sheets to name, "All sheets" prints each on its own
        // page, and an "Active document" scope would only repeat it under a
        // second name.
        let has_sheets = schematic
            && reads
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&active_key)
                .is_some_and(|catalog| !catalog.sheets().is_empty());
        let allowed_scopes = match (schematic, has_sheets) {
            (true, true) => vec![
                HardcopyScope::Selection,
                HardcopyScope::CurrentSheet,
                HardcopyScope::AllSheetsOrPanes,
            ],
            (true, false) => vec![
                HardcopyScope::Selection,
                HardcopyScope::CurrentSheet,
                HardcopyScope::ActiveDocument,
            ],
            (false, _) => vec![HardcopyScope::ActiveDocument],
        };
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: design_source_key.clone(),
            display_name: reads.model.workspace.content.active_display_path(),
            document_kind: HardcopyDocumentKind::SchematicOrSymbol,
            allowed_scopes,
            availability: if supported {
                RetainedHardcopySourceAvailability::Available
            } else {
                RetainedHardcopySourceAvailability::Unavailable {
                    reason: format!(
                        "the active {} view cannot be printed",
                        view_type.display_name()
                    ),
                }
            },
        });
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench)
            && let Some(catalog) = reads
                .model
                .workspace
                .content
                .design_management
                .sheet_catalog(&active_key)
        {
            for sheet in catalog.sheets() {
                descriptors.push(RetainedHardcopySourceDescriptor {
                    source_key: format!("{design_source_key}:sheet:{}", sheet.id()),
                    display_name: compact_display(
                        &format!(
                            "{} · {}",
                            reads.model.workspace.content.active_display_path(),
                            sheet.name()
                        ),
                        "Schematic sheet",
                    ),
                    document_kind: HardcopyDocumentKind::SchematicOrSymbol,
                    allowed_scopes: vec![HardcopyScope::CurrentSheet],
                    availability: RetainedHardcopySourceAvailability::Available,
                });
            }
        }
    }

    if let Some(WorkspaceDocumentId::ResultDataset(dataset_id)) =
        reads.workbench.documents.active(Workspace::Results)
        && let Some(run) = reads
            .model
            .simulation
            .retained
            .run_by_dataset_id(*dataset_id)
    {
        let availability = quick_result_availability(reads.model, reads.results, run);
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: format!(
                "project:{}:result-dataset:{}",
                project_id.as_uuid(),
                run.dataset_id
            ),
            display_name: format!("{} · {}", run.label, reads.results.result_viewer().label()),
            document_kind: HardcopyDocumentKind::PlotOrWorksheet,
            allowed_scopes: vec![
                HardcopyScope::ActivePlotDocument,
                HardcopyScope::ActiveDocument,
            ],
            availability,
        });
    }

    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        reads.workbench.documents.active(Workspace::Results)
        && let Some((document, page, pane)) =
            active_visualization_document_pane(reads.model, reads.results, *document_id)
    {
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: visualization_document_pane_source_key(project_id, document.id(), pane.id),
            display_name: compact_display(
                &format!("{} · {} · {}", document.title(), page.title, pane.title),
                "Result document pane",
            ),
            document_kind: HardcopyDocumentKind::PlotOrWorksheet,
            allowed_scopes: vec![
                HardcopyScope::ActivePlotDocument,
                HardcopyScope::ActiveDocument,
                HardcopyScope::AllSheetsOrPanes,
            ],
            availability: visualization_document_pane_availability(document, pane),
        });
    }

    if let Some(document_id) = reads.report_authoring.selected_document
        && let Some(document) = reads
            .model
            .workspace
            .content
            .report_documents
            .iter()
            .find(|document| document.id() == document_id)
    {
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: format!("project:{}:report:{}", project_id.as_uuid(), document_id),
            display_name: document.title().to_owned(),
            document_kind: HardcopyDocumentKind::Report,
            allowed_scopes: vec![HardcopyScope::CompleteReport, HardcopyScope::ActiveDocument],
            availability: report_inventory::availability(reads, document),
        });
    }

    let source_set_descriptors = reads
        .model
        .workspace
        .content
        .hardcopy_source_sets()
        .iter()
        .map(|source_set| source_set_descriptor(source_set, &descriptors))
        .collect::<Vec<_>>();
    descriptors.extend(source_set_descriptors);
    descriptors
}

fn source_set_descriptor(
    source_set: &HardcopySourceSet,
    retained: &[RetainedHardcopySourceDescriptor],
) -> RetainedHardcopySourceDescriptor {
    let availability = source_set
        .validate()
        .and_then(|()| {
            for member in source_set.members() {
                let mut matching = retained
                    .iter()
                    .filter(|descriptor| descriptor.source_key == member.source_key());
                let descriptor = matching.next().ok_or_else(|| {
                    HardcopySourceError::SourceNotRetained(member.source_key().to_owned())
                })?;
                if matching.next().is_some() {
                    return Err(HardcopySourceError::AmbiguousActiveSource(
                        member.source_key().to_owned(),
                    ));
                }
                if let RetainedHardcopySourceAvailability::Unavailable { reason } =
                    &descriptor.availability
                {
                    return Err(HardcopySourceError::UnavailableRetainedSource {
                        source_key: member.source_key().to_owned(),
                        reason: reason.clone(),
                    });
                }
                if !descriptor.supports_scope(member.scope()) {
                    return Err(HardcopySourceError::UnsupportedScope(
                        member.scope().clone(),
                    ));
                }
            }
            Ok(())
        })
        .map_or_else(
            |error| RetainedHardcopySourceAvailability::Unavailable {
                reason: error.to_string(),
            },
            |()| RetainedHardcopySourceAvailability::Available,
        );
    RetainedHardcopySourceDescriptor {
        source_key: source_set.source_key(),
        display_name: source_set.name().to_owned(),
        document_kind: source_set.document_kind(),
        allowed_scopes: vec![source_set.scope().clone()],
        availability,
    }
}

/// Capture only the exact retained owner needed by one dialog selection.
/// This performs bounded identity/shape checks and cloning, but deliberately
/// defers sample validation, digesting, symbol resolution, and semantic scene
/// construction to [`PreparedRetainedHardcopyResolution::resolve_owned`].
pub(crate) fn prepare_retained_hardcopy_resolution(
    reads: SourceReads<'_>,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
    let descriptors = enumerate_retained_hardcopy_sources(reads);
    let mut matching = descriptors
        .iter()
        .filter(|descriptor| descriptor.source_key == source_key);
    let descriptor = matching
        .next()
        .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
    if matching.next().is_some() {
        return Err(HardcopySourceError::AmbiguousActiveSource(
            source_key.to_owned(),
        ));
    }
    if let RetainedHardcopySourceAvailability::Unavailable { reason } = &descriptor.availability {
        return Err(HardcopySourceError::UnavailableRetainedSource {
            source_key: source_key.to_owned(),
            reason: reason.clone(),
        });
    }
    if !descriptor.supports_scope(&scope) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }

    if let Some(source_set) = reads
        .model
        .workspace
        .content
        .hardcopy_source_set(source_key)
    {
        let members = source_set
            .members()
            .iter()
            .map(|member| {
                prepare_retained_hardcopy_resolution(
                    reads,
                    member.source_key(),
                    member.scope().clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        return PreparedRetainedHardcopyResolution::try_capture(
            RetainedHardcopySourceInput::SourceSet {
                source_set: source_set.clone(),
                members,
            },
        );
    }

    let project_id = reads.model.workspace.content.project.id();
    let design_key = format!(
        "project:{}:cell-view:{}",
        project_id.as_uuid(),
        reads.model.workspace.content.active_key()
    );
    if matches!(
        reads.model.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        let active_key = reads.model.workspace.content.active_key();
        let catalog = reads
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&active_key);
        if let Some(sheet) = catalog.and_then(|catalog| {
            catalog
                .sheets()
                .iter()
                .find(|sheet| format!("{design_key}:sheet:{}", sheet.id()) == source_key)
        }) {
            return prepare_schematic_resolution(
                reads,
                schematic_sheet_identity(&active_cell_view_identity(reads)?, sheet)?,
                catalog.cloned(),
                Some(sheet.id()),
                false,
                scope,
            );
        }
    }
    if source_key == design_key {
        let identity = active_cell_view_identity(reads)?;
        return match reads.model.workspace.content.active_view_type() {
            ViewType::Schematic | ViewType::Testbench => {
                let active_key = reads.model.workspace.content.active_key();
                let catalog = reads
                    .model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&active_key);
                if matches!(scope, HardcopyScope::AllSheetsOrPanes) {
                    let catalog = catalog.cloned().ok_or_else(|| {
                        HardcopySourceError::InvalidSheetPartition(
                            "all-sheets scope has no governed sheet catalog".to_owned(),
                        )
                    })?;
                    return prepare_schematic_resolution(
                        reads,
                        identity,
                        Some(catalog),
                        None,
                        true,
                        scope,
                    );
                }
                let active_sheet = catalog.and_then(|catalog| {
                    catalog
                        .active_sheet_id()
                        .map(|sheet_id| (catalog, sheet_id))
                });
                let (identity, sheet_catalog, sheet_id) = match (&scope, active_sheet) {
                    (HardcopyScope::CurrentSheet, Some((catalog, sheet_id))) => {
                        let sheet = catalog.find(sheet_id).ok_or_else(|| {
                            HardcopySourceError::InvalidSheetPartition(format!(
                                "active sheet {sheet_id} is not retained"
                            ))
                        })?;
                        (
                            schematic_sheet_identity(&identity, sheet)?,
                            Some(catalog.clone()),
                            Some(sheet_id),
                        )
                    }
                    // A selection can still hold objects of a sheet that is
                    // not in view; the catalog holds it to the active sheet.
                    (HardcopyScope::Selection, _) => (identity, catalog.cloned(), None),
                    _ => (identity, None, None),
                };
                prepare_schematic_resolution(reads, identity, sheet_catalog, sheet_id, false, scope)
            }
            ViewType::Symbol => {
                let document = reads
                    .model
                    .load_active_symbol_document()
                    .map_err(HardcopySourceError::StaleActiveDocumentAuthority)?;
                PreparedRetainedHardcopyResolution::try_capture(
                    RetainedHardcopySourceInput::Symbol {
                        project_id,
                        identity,
                        document,
                        scope,
                    },
                )
            }
            view_type => Err(HardcopySourceError::UnsupportedDocument(format!(
                "the active {} view cannot be printed",
                view_type.display_name()
            ))),
        };
    }

    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        reads.workbench.documents.active(Workspace::Results)
        && let Some((document, page, pane)) =
            active_visualization_document_pane(reads.model, reads.results, *document_id)
        && source_key == visualization_document_pane_source_key(project_id, document.id(), pane.id)
    {
        let all_panes = matches!(scope, HardcopyScope::AllSheetsOrPanes);
        return PreparedRetainedHardcopyResolution::try_capture(
            RetainedHardcopySourceInput::VisualizationDocument {
                source_key: source_key.to_owned(),
                project_id,
                document: document.clone(),
                page_id: page.id,
                pane_id: pane.id,
                all_panes,
                scope,
            },
        );
    }

    if let Some(displayed) = reads
        .results
        .displayed_dataset_view(reads.model, reads.workbench)
        && let Some(run) = reads
            .model
            .simulation
            .retained
            .runs
            .get(displayed.run_index)
    {
        let result_key = format!(
            "project:{}:result-dataset:{}",
            project_id.as_uuid(),
            run.dataset_id
        );
        if source_key == result_key {
            require_active_result_document(reads, run.dataset_id)?;
            let viewer = displayed.viewer;
            let prepared_run = if matches!(viewer, ResultViewer::Manifest | ResultViewer::Specs) {
                // Dataset-native report sheets judge the complete immutable
                // run, including missing rows and cross-analysis worst cases.
                run.clone()
            } else if rspice_results::result_presentation::viewer_uses_wave_stack(viewer) {
                if displayed.analysis_indices.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS {
                    return Err(HardcopySourceError::InvalidVisualizationSource(format!(
                        "{} displays {} analyses, exceeding the {}-sheet hardcopy limit; maximize one strip before exporting",
                        viewer.label(),
                        displayed.analysis_indices.len(),
                        MAX_HARDCOPY_SOURCE_SET_MEMBERS,
                    )));
                }
                let mut prepared_run = run.clone();
                prepared_run.analyses = displayed
                    .analysis_indices
                    .iter()
                    .map(|&index| {
                        run.analyses.get(index).cloned().ok_or_else(|| {
                            HardcopySourceError::UnretainedResult(format!(
                                "displayed analysis index {index} is not retained"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                prepared_run
            } else {
                let analysis_index = displayed.primary_analysis_index.ok_or_else(|| {
                    HardcopySourceError::UnretainedResult(format!(
                        "no saved analysis has results for {}",
                        viewer.label()
                    ))
                })?;
                let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
                    HardcopySourceError::UnretainedResult(format!(
                        "active analysis index {analysis_index} is not retained"
                    ))
                })?;
                let mut prepared_run = run.clone();
                prepared_run.analyses = vec![analysis.clone()];
                prepared_run
            };
            return PreparedRetainedHardcopyResolution::try_capture(
                RetainedHardcopySourceInput::Results {
                    source_key: source_key.to_owned(),
                    project_id,
                    run: capture_hardcopy_run(prepared_run),
                    presentation: capture_results_quick_view_presentation(reads)?,
                    scope,
                },
            );
        }
    }

    if let Some(document_id) = reads.report_authoring.selected_document {
        let report_key = format!("project:{}:report:{}", project_id.as_uuid(), document_id);
        if source_key == report_key {
            let document = reads
                .model
                .workspace
                .content
                .report_documents
                .iter()
                .find(|document| document.id() == document_id)
                .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
            let reference_inventory = report_inventory::reference_inventory(reads, document)?;
            return PreparedRetainedHardcopyResolution::try_capture(
                RetainedHardcopySourceInput::Report {
                    project_id,
                    source_key: source_key.to_owned(),
                    document: document.clone(),
                    reference_inventory,
                    scope,
                },
            );
        }
    }

    Err(HardcopySourceError::SourceNotRetained(
        source_key.to_owned(),
    ))
}

fn prepare_schematic_resolution(
    reads: SourceReads<'_>,
    identity: HardcopySourceIdentity,
    sheet_catalog: Option<SheetCatalog>,
    sheet_id: Option<SheetId>,
    all_sheets: bool,
    scope: HardcopyScope,
) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
    PreparedRetainedHardcopyResolution::try_capture(RetainedHardcopySourceInput::Schematic {
        project_id: reads.model.workspace.content.project.id(),
        identity,
        schematic: reads.model.schematic.editor_ref().design.clone(),
        selection: reads.model.schematic.session.editor.selection.clone(),
        library_manager: reads.model.library_manager.clone(),
        schematic_buffers: reads.model.workspace.content.schematic_buffers.clone(),
        sheet_catalog,
        sheet_id,
        project_default_drawing_sheet: reads
            .model
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .default_format
            .clone(),
        project_title_block_field_values: reads
            .model
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .title_block_field_values
            .clone(),
        all_sheets,
        scope,
    })
}

fn capture_hardcopy_run(run: SimulationRun) -> HardcopyRun {
    run.data.map_analyses(|analysis| {
        analysis.data.map_waveforms(|waveform| HardcopyWaveform {
            data: waveform.data,
            color: waveform.color,
            visible: waveform.visible,
        })
    })
}

/// Per-frame command predicate. This deliberately performs identity and
/// evidence-shape checks only; full digesting and semantic resolution occur
/// when the dialog opens or commits.
#[must_use]
pub(crate) fn active_app_hardcopy_source_available(
    report_authoring: &ReportAuthoringState,
    model: &Model,
    workbench: &WorkbenchSession,
    results: &dyn ResultsPort,
) -> bool {
    match workbench.current_route().surface_id() {
        SurfaceId::Design => {
            matches!(
                workbench.documents.active(Workspace::Design),
                Some(WorkspaceDocumentId::CellView(reference))
                    if reference == &model.workspace.content.active_view
            ) && matches!(
                model.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
            )
        }
        SurfaceId::Results => match workbench.documents.active(Workspace::Results) {
            Some(WorkspaceDocumentId::ResultDataset(dataset)) => model
                .simulation
                .retained
                .run_by_dataset_id(*dataset)
                .is_some_and(|run| quick_result_availability(model, results, run).is_available()),
            Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
                active_visualization_document_pane(model, results, *document_id).is_some_and(
                    |(document, _, pane)| {
                        visualization_document_pane_availability(document, pane).is_available()
                    },
                )
            }
            _ => false,
        },
        SurfaceId::ReportAuthoring => report_authoring
            .selected_document
            .and_then(|document_id| {
                model
                    .workspace
                    .content
                    .report_documents
                    .iter()
                    .find(|document| document.id() == document_id)
            })
            .is_some_and(|document| !document.pages().is_empty()),
        _ => false,
    }
}

/// Resolve an exact ordered source set with a caller-provided retained-source
/// lookup. This is the state-facing boundary used both by project persistence
/// and by worker-owned source snapshots.
/// Whether the requirement set this run is judged against is empty.
///
/// The run-scoped spelling of the sheet's `resolved_specifications`: a
/// dispatched run carries the requirements it froze, and only a legacy
/// dataset from before prepared-run receipts falls back to the workspace's
/// live contract.
fn resolved_run_specifications_are_empty(model: &Model, run: &SimulationRun) -> bool {
    run.prepared_receipt().map_or_else(
        || model.workspace.content.specs.is_empty(),
        |receipt| receipt.specifications().is_empty(),
    )
}

pub(crate) fn quick_result_availability(
    model: &Model,
    results: &dyn ResultsPort,
    run: &SimulationRun,
) -> RetainedHardcopySourceAvailability {
    let unavailable = |reason: String| RetainedHardcopySourceAvailability::Unavailable { reason };
    if !run.lifecycle.is_terminal() {
        return unavailable(format!(
            "dataset {} belongs to a non-terminal run",
            run.dataset_id
        ));
    }
    let viewer = results.result_viewer();
    if viewer == ResultViewer::Manifest {
        // Manifest hardcopy is bound to the terminal dataset as a whole and
        // must not require an arbitrarily selected analysis.
        return RetainedHardcopySourceAvailability::Available;
    }
    if viewer == ResultViewer::Specs {
        // The requirement set a dispatched run was judged against is the one
        // it froze into its receipt, and that is what the capture writes —
        // `capture_results_quick_view_presentation` resolves it through the
        // shared `run_specifications`. Offering the page on the workspace's
        // currently authored set instead made the two disagree in both
        // directions: a receipt-backed run whose frozen requirements had since
        // been deleted from the workspace was refused a page it could fill,
        // and a run prepared with no requirements at all was offered one that
        // resolves to an empty table the moment a limit is authored.
        let has_evidence = !resolved_run_specifications_are_empty(model, run)
            || run
                .analyses
                .iter()
                .any(|analysis| !analysis.measurements.is_empty());
        return if has_evidence {
            RetainedHardcopySourceAvailability::Available
        } else {
            unavailable("the dataset has no specifications or retained measurements".to_owned())
        };
    }
    let Some(index) = quick_result_analysis_index(model, run, viewer) else {
        return unavailable(format!(
            "no saved analysis has results for {}",
            viewer.label()
        ));
    };
    let Some(analysis) = run.analyses.get(index) else {
        return unavailable(format!("active analysis index {index} is not retained"));
    };
    if !analysis.success {
        return unavailable(format!(
            "active analysis {} was not successful",
            analysis.id
        ));
    }
    let visible_waveforms = || {
        analysis
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible)
    };
    let has_waveform = || {
        visible_waveforms()
            .any(|waveform| !waveform.x.is_empty() && waveform.x.len() == waveform.y.len())
    };
    let available = match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep | ResultViewer::Bode => has_waveform(),
        ResultViewer::Fft => visible_waveforms().any(|waveform| {
            waveform.x.len() >= rspice_results_ui::fft::MIN_FFT_SAMPLES
                && waveform.x.len() == waveform.y.len()
        }),
        ResultViewer::HarmonicBalance => harmonic_spectrum_is_renderable(analysis, |_| {}),
        ResultViewer::PhaseNoise => phase_noise_is_renderable(analysis, |_| {}),
        ResultViewer::Eye => visible_waveforms()
            .any(|waveform| waveform.x.len() >= 8 && waveform.x.len() == waveform.y.len()),
        ResultViewer::Hist => matches!(
            analysis.family_metadata.as_ref(),
            Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. })
                if rspice_results_ui::histogram::state::measurement_index(
                    results.histogram_selection(), 0,
                    &variables.iter().map(|variable| variable.name.as_str()).collect::<Vec<_>>(),
                ).and_then(|index| variables.get(index))
                    .is_some_and(|variable| !variable.samples.is_empty())
        ),
        ResultViewer::Nyquist => visible_waveforms().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty() && complex.real.len() == complex.imag.len()
            })
        }),
        ResultViewer::Smith => smith_analysis_is_renderable(analysis, |_| {}),
        ResultViewer::NetworkMatrix => analysis_supports_viewer(viewer, analysis, |_| {}),
        ResultViewer::Polar => visible_waveforms().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty() && complex.real.len() == complex.imag.len()
            })
        }),
        // The population itself is the printable evidence, so either half of
        // it — the sampled variables or what the trials measured — is enough.
        ResultViewer::Scatter | ResultViewer::BoxViolin => matches!(
            analysis.family_metadata.as_ref(),
            Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, member_measurements, .. })
                if member_measurements.len() >= 2
                    || variables.iter().any(|variable| variable.samples.len() >= 2)
        ),
        ResultViewer::Op => {
            analysis.dc_op.is_some()
                || analysis
                    .device_op
                    .as_ref()
                    .is_some_and(|report| !report.is_empty())
                || matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::OperatingPoint { .. })
                )
        }
        ResultViewer::NoiseContrib => {
            ordinary_noise_spectrum_is_renderable(analysis, |_| {})
                || qpnoise_spectrum_is_renderable(analysis)
        }
        // Two families rank contributions to one number, and both have an
        // exact semantic table to print.
        ResultViewer::Contribution => matches!(
            analysis.result_payload.as_ref(),
            Some(
                AnalysisResultPayload::Sensitivity { .. }
                    | AnalysisResultPayload::SensitivityStudy { .. }
                    | AnalysisResultPayload::DcMismatch { .. }
            )
        ),
        ResultViewer::TransferFunction => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction { .. })
        ),
        ResultViewer::Specs => {
            !analysis.measurements.is_empty()
                || matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::ScalarMeasurements { .. })
                )
        }
        ResultViewer::PoleZero => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::PoleZero { .. })
        ),
        // Periodic payloads have an exact semantic table even when a
        // zero-order map or a payload-only result retained no display curve.
        ResultViewer::Table => {
            has_waveform()
                || matches!(
                    analysis.result_payload.as_ref(),
                    Some(
                        AnalysisResultPayload::PssFloquet { .. }
                            | AnalysisResultPayload::Pstb { .. }
                    )
                )
        }
        ResultViewer::Events => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty()
        ),
        ResultViewer::Soa => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Soa { evaluations, .. }) if !evaluations.is_empty()
        ),
        ResultViewer::Optimization => matches!(
            analysis.family_metadata.as_ref(),
            Some(AnalysisResultFamilyMetadata::Optimization { iterations, .. })
                if !iterations.is_empty()
        ),
        // Handled before analysis selection because this is dataset-native.
        ResultViewer::Manifest => true,
    };
    if available {
        RetainedHardcopySourceAvailability::Available
    } else {
        unavailable(format!(
            "active analysis {} has no results for {}",
            analysis.id,
            viewer.label()
        ))
    }
}

pub(crate) fn quick_result_analysis_index(
    model: &Model,
    run: &SimulationRun,
    viewer: ResultViewer,
) -> Option<usize> {
    let globally_selected = (model.simulation.view.active_run_idx
        == model
            .simulation
            .retained
            .runs
            .iter()
            .position(|candidate| candidate.dataset_id == run.dataset_id))
    .then_some(model.simulation.view.active_analysis_idx)
    .flatten();
    match viewer {
        ResultViewer::Waves => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| transient_waveform_is_renderable(analysis))
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| transient_waveform_is_renderable(analysis))
            }),
        // The one binding the sheet uses, so the page and the screen name the
        // same analysis. `filter` + `or_else` is not that binding: it steps to
        // the next renderable result whenever the reader's own selection is a
        // noise analysis that carries no ordinary spectrum, and prints another
        // analysis's contributors under the selected one's name.
        ResultViewer::NoiseContrib => {
            selected_noise_analysis_index(globally_selected, &run.analyses, |_| {})
        }
        ResultViewer::DcSweep => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| analysis.analysis_type == AnalysisType::DcSweep)
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| analysis.analysis_type == AnalysisType::DcSweep)
            }),
        ResultViewer::Bode => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| bode_analysis_is_renderable(analysis, |_| {}))
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| bode_analysis_is_renderable(analysis, |_| {}))
            }),
        ResultViewer::PhaseNoise => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| phase_noise_is_renderable(analysis, |_| {}))
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| phase_noise_is_renderable(analysis, |_| {}))
            }),
        ResultViewer::HarmonicBalance => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| harmonic_spectrum_is_renderable(analysis, |_| {}))
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| harmonic_spectrum_is_renderable(analysis, |_| {}))
            }),
        _ => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(|analysis| analysis_supports_viewer(viewer, analysis, |_| {}))
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(|analysis| analysis_supports_viewer(viewer, analysis, |_| {}))
            }),
    }
}

pub(crate) fn active_visualization_document_pane<'m>(
    model: &'m Model,
    results: &dyn ResultsPort,
    document_id: crate::product::ResultDocumentId,
) -> Option<(&'m VisualizationDocument, &'m Page, &'m Pane)> {
    let document = model
        .workspace
        .content
        .visualization_document(document_id)?;
    if let Some(pane) = results
        .persistent_document_pane(document_id)
        .and_then(|pane_id| document.panes().iter().find(|pane| pane.id == pane_id))
        && let Some(page) = document.pages().iter().find(|page| page.id == pane.page_id)
    {
        return Some((document, page, pane));
    }
    let selected_page_id = results
        .persistent_document_page(document_id)
        .filter(|selected| document.pages().iter().any(|page| page.id == *selected))
        .or_else(|| document.pages().first().map(|page| page.id))?;
    let page = document
        .pages()
        .iter()
        .find(|page| page.id == selected_page_id)?;
    let pane = document
        .panes()
        .iter()
        .filter(|pane| pane.page_id == page.id)
        .min_by_key(|pane| (pane.order, pane.id.get()))?;
    Some((document, page, pane))
}

fn visualization_document_pane_availability(
    document: &VisualizationDocument,
    pane: &Pane,
) -> RetainedHardcopySourceAvailability {
    let unavailable = |reason: &str| RetainedHardcopySourceAvailability::Unavailable {
        reason: reason.to_owned(),
    };
    if pane.binding.is_none() {
        return unavailable("the selected result pane is not bound to a run");
    }
    if pane.kind != crate::results::visualization_document::PaneKind::Cartesian
        || pane.viewer_id != "viewer-waveform"
    {
        return unavailable("the selected result pane has no semantic figure writer");
    }
    if pane.family_policy.is_some() {
        return unavailable("the selected result pane has an unresolved family presentation");
    }
    if document
        .measurements()
        .iter()
        .any(|measurement| measurement.pane_id == pane.id)
    {
        return unavailable(
            "the selected result pane contains a measurement overlay not supported by the semantic figure writer",
        );
    }
    if !document
        .traces()
        .iter()
        .any(|trace| trace.pane_id == pane.id && trace.visible)
    {
        return unavailable("the selected result pane has no visible retained trace");
    }
    RetainedHardcopySourceAvailability::Available
}

fn active_cell_view_identity(
    reads: SourceReads<'_>,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let project_id = reads.model.workspace.content.project.id();
    let view_key = reads.model.workspace.content.active_key();
    let mut identity_material = b"rspice-cell-view-hardcopy-v1:".to_vec();
    identity_material.extend_from_slice(view_key.as_bytes());
    let document_control = &reads
        .model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .document_control;
    let revision = document_control.revision_index();
    let date = &document_control.revision_date_utc;
    HardcopySourceIdentity::try_new(
        format!("project:{}:cell-view:{view_key}", project_id.as_uuid()),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_material))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        reads.model.workspace.content.project.revision(),
        reads.model.workspace.content.active_display_path(),
    )?
    .with_publication(HardcopyPublicationIdentity::try_new(
        reads.model.workspace.content.project.name(),
        reads.model.workspace.content.active_view.display_path(),
        (!revision.is_empty()).then(|| revision.to_owned()),
        (!date.is_empty()).then(|| date.clone()),
        Some(document_control.status.label().to_owned()),
    )?)
}

fn require_active_result_document(
    reads: SourceReads<'_>,
    expected_dataset: DatasetId,
) -> Result<(), HardcopySourceError> {
    match reads.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset)) if *dataset == expected_dataset => Ok(()),
        Some(_) => Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
            "the Results workspace shows a different document than dataset {expected_dataset}"
        ))),
        None => Err(HardcopySourceError::NoActiveDocumentAuthority(
            "result dataset",
        )),
    }
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(crate) use test_support::{
    resolve_blank_schematic_sheet, resolve_blank_schematic_sheet_with_format,
    resolve_retained_hardcopy_source,
};
#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::results::report_document::FrozenReportArtifact;
#[cfg(test)]
use crate::state::{Point, SymbolShape};
#[cfg(test)]
use rspice_hardcopy_contract::PrintObjectKind;
