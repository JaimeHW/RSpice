//! Exact, semantic hardcopy source resolution.
//!
//! Hardcopy starts here rather than at a viewport or GPU surface.  Every
//! adapter freezes one durable document revision, resolves authored symbol
//! artwork and retained result samples, computes deterministic physical
//! bounds, and authenticates the semantic snapshot before rendering begins.
//! No type in this module contains pixels, an egui paint command, or a screen
//! rectangle.

mod documents;
mod noise;
mod prepared;
mod quick_view_overlay;
mod report_inventory;
mod results;

pub use documents::*;
pub(crate) use rspice_hardcopy::sources::*;
// Module-private: `noise` and `quick_view_overlay`
// expose only `pub(super)` items, and the siblings reach them through
// `use super::*`.
use noise::*;
pub use prepared::*;
use quick_view_overlay::*;
pub(crate) use results::*;
#[cfg(test)]
use rspice_hardcopy::sources::resolve_semantic_source as finish_resolved;
#[cfg(test)]
pub const BLANK_SCHEMATIC_SHEET_WIDTH_UM: i64 = 279_400;
#[cfg(test)]
pub const BLANK_SCHEMATIC_SHEET_HEIGHT_UM: i64 = 215_900;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 8;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::io::ProjectSimulationResults;
use crate::product::{ContentDigest, DatasetId, ObjectRevision, ProjectId};
#[cfg(test)]
use crate::results::report_document::{
    FigureSizing, ReportBlockKind, ReportReferenceCurrentness, ReportReferenceMode,
};
use crate::results::report_document::{ReportDocument, ReportReferenceInventory};
use crate::results::visualization_document::{Page, PageId, Pane, PaneId, VisualizationDocument};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType, Bus, BusTap,
    Component, DesignNote, DocumentationShape, DrawingSheetTitleFieldId, Junction, NetLabel,
    SchematicSheetFormat, SchematicState, Selection, SheetCatalog, SheetId, SimulationRun,
    SimulationState, SymbolDocument, SymbolResolver, ViewType, WaveformData, Wire,
};
use crate::workbench::AppState;

use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
use crate::workbench::SurfaceId;
// The persisted source-set records and the validation they share with these
// adapters are owned one layer down, where `state` can reach them.
use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, HardcopyPublicationIdentity, HardcopySourceError, HardcopySourceIdentity,
    HardcopySourceSet, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT, canonical_digest,
    validate_label,
};
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::documents::visualization_studio::{
    VisualizationPane as StudioPane, VisualizationStudioState,
};
use crate::workbench::lifecycle::session::SymbolSelection;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

fn capture_schematic_selection<'a>(
    selection: &'a Selection,
    scope: &HardcopyScope,
) -> Option<SchematicHardcopySelection<'a>> {
    matches!(scope, HardcopyScope::Selection).then(|| SchematicHardcopySelection {
        components: &selection.components,
        wires: selection
            .wires
            .iter()
            .copied()
            .chain(selection.wire_segments.iter().map(|handle| handle.wire_id))
            .chain(selection.wire_vertices.iter().map(|handle| handle.wire_id))
            .collect(),
        junctions: selection
            .junctions
            .iter()
            .map(|junction| junction.pos)
            .collect(),
        buses: &selection.buses,
        bus_taps: &selection.bus_taps,
        net_labels: &selection.net_labels,
        design_notes: &selection.design_notes,
        documentation_shapes: &selection.documentation_shapes,
        has_probes: !selection.probes.is_empty(),
    })
}

pub struct SymbolHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub document: &'a SymbolDocument,
    pub selection: Option<&'a SymbolSelection>,
    pub scope: HardcopyScope,
}

/// Direct adapter over the application's retained Visualization Studio model.
/// It is crate-visible because the studio state itself is an internal UI
/// document; callers outside the workbench use the canonical
/// `VisualizationDocument` adapter above.
pub(crate) struct ActiveStudioPaneHardcopySource<'a> {
    pub source_key: String,
    pub project_id: ProjectId,
    pub studio: &'a VisualizationStudioState,
    pub simulation: &'a SimulationState,
    pub pane_id: u64,
    pub scope: HardcopyScope,
}

/// The document shown by the Results workspace quick-view. The adapter reads
/// the selected retained dataset and the exact active specialized result
/// state; it never samples the screen or depends on the viewer's paint cache.
#[cfg(test)]
pub(crate) struct ResultsQuickViewHardcopySource<'a> {
    pub source_key: String,
    pub project_id: ProjectId,
    pub state: &'a AppState,
    pub scope: HardcopyScope,
}

/// The retained run a quick-view capture is taken from.
///
/// The active Results document is the authority, exactly as it is for the
/// descriptor that offers the page. The simulation's own selection stands in
/// only when no result document is open — a capture reached from the command
/// palette rather than from the workspace.
fn captured_results_run(state: &AppState) -> Option<&SimulationRun> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => {
            state.simulation.run_by_dataset_id(*dataset_id)
        }
        _ => state.simulation.active_run(),
    }
}

fn capture_results_quick_view_presentation(
    state: &AppState,
) -> Result<ResultsQuickViewPresentation, HardcopySourceError> {
    let fft = &state.analysis.fft_state;
    let histogram = &state.analysis.histogram_state;
    let view = state.ui.results.plot_view(ResultViewer::Hist, 0);
    let overlay = captured_results_run(state)
        .map(|run| capture_quick_view_overlays(state, run))
        .transpose()?
        .unwrap_or_default();
    ResultsQuickViewPresentation::try_new(
        state.ui.results.viewer,
        overlay,
        crate::workbench::documents::result_document::run_specifications(state),
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
    state: &AppState,
) -> Vec<RetainedHardcopySourceDescriptor> {
    let project_id = state.workspace.content.project.id();
    let mut descriptors = Vec::new();

    if matches!(
        state.workbench.documents.active(Workspace::Design),
        Some(WorkspaceDocumentId::CellView(reference)) if reference == &state.workspace.content.active_view
    ) {
        let view_type = state.workspace.content.active_view_type();
        let active_key = state.workspace.content.active_key();
        let design_source_key = format!("project:{}:cell-view:{active_key}", project_id.as_uuid());
        let supported = matches!(
            view_type,
            ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
        );
        let mut allowed_scopes = if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            vec![
                HardcopyScope::Selection,
                HardcopyScope::CurrentSheet,
                HardcopyScope::ActiveDocument,
            ]
        } else {
            vec![HardcopyScope::ActiveDocument]
        };
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench)
            && state
                .workspace
                .content
                .design_management
                .sheet_catalog(&active_key)
                .is_some_and(|catalog| !catalog.sheets().is_empty())
        {
            allowed_scopes.push(HardcopyScope::AllSheetsOrPanes);
        }
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: design_source_key.clone(),
            display_name: state.workspace.content.active_display_path(),
            document_kind: HardcopyDocumentKind::SchematicOrSymbol,
            allowed_scopes,
            availability: if supported {
                RetainedHardcopySourceAvailability::Available
            } else {
                RetainedHardcopySourceAvailability::Unavailable {
                    reason: format!(
                        "active design view type {view_type:?} has no semantic hardcopy adapter"
                    ),
                }
            },
        });
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench)
            && let Some(catalog) = state
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
                            state.workspace.content.active_display_path(),
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
        state.workbench.documents.active(Workspace::Results)
        && let Some(run) = state.simulation.run_by_dataset_id(*dataset_id)
    {
        let availability = quick_result_availability(state, run);
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: format!(
                "project:{}:result-dataset:{}",
                project_id.as_uuid(),
                run.dataset_id
            ),
            display_name: format!("{} · {}", run.label, state.ui.results.viewer.label()),
            document_kind: HardcopyDocumentKind::PlotOrWorksheet,
            allowed_scopes: vec![
                HardcopyScope::ActivePlotDocument,
                HardcopyScope::ActiveDocument,
            ],
            availability,
        });
    }

    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        state.workbench.documents.active(Workspace::Results)
        && let Some((document, page, pane)) =
            active_visualization_document_pane(state, *document_id)
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

    for pane in &state.workbench.visualization_studio.panes {
        let pane_id = pane.id;
        let availability = studio_pane_availability(state, pane);
        let mut allowed_scopes = vec![
            HardcopyScope::ActivePlotDocument,
            HardcopyScope::ActiveDocument,
        ];
        if state.workbench.visualization_studio.active_pane == Some(pane_id)
            && state.workbench.visualization_studio.panes.len() > 1
        {
            allowed_scopes.push(HardcopyScope::AllSheetsOrPanes);
        }
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: format!(
                "project:{}:visualization-pane:{pane_id}",
                project_id.as_uuid()
            ),
            display_name: format!("{} · {}", pane.page, pane.viewer.label()),
            document_kind: HardcopyDocumentKind::PlotOrWorksheet,
            allowed_scopes,
            availability,
        });
    }

    if let Some(document_id) = state.workbench.report_authoring.selected_document
        && let Some(document) = state
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
            availability: report_inventory::availability(state, document),
        });
    }

    let source_set_descriptors = state
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
    state: &AppState,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
    let descriptors = enumerate_retained_hardcopy_sources(state);
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

    if let Some(source_set) = state.workspace.content.hardcopy_source_set(source_key) {
        let members = source_set
            .members()
            .iter()
            .map(|member| {
                prepare_retained_hardcopy_resolution(
                    state,
                    member.source_key(),
                    member.scope().clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(PreparedRetainedHardcopyResolution {
            payload: PreparedRetainedHardcopyPayload::SourceSet {
                source_set: source_set.clone(),
                members,
            },
        });
    }

    let project_id = state.workspace.content.project.id();
    let design_key = format!(
        "project:{}:cell-view:{}",
        project_id.as_uuid(),
        state.workspace.content.active_key()
    );
    if matches!(
        state.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        let active_key = state.workspace.content.active_key();
        let catalog = state
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
                state,
                schematic_sheet_identity(&active_cell_view_identity(state)?, sheet)?,
                catalog.cloned(),
                Some(sheet.id()),
                false,
                scope,
            );
        }
    }
    if source_key == design_key {
        let identity = active_cell_view_identity(state)?;
        return match state.workspace.content.active_view_type() {
            ViewType::Schematic | ViewType::Testbench => {
                let active_key = state.workspace.content.active_key();
                let catalog = state
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
                        state,
                        identity,
                        Some(catalog),
                        None,
                        true,
                        scope,
                    );
                }
                let (identity, sheet_catalog, sheet_id) =
                    if matches!(scope, HardcopyScope::CurrentSheet) {
                        if let Some(catalog) = catalog
                            && let Some(sheet_id) = catalog.active_sheet_id()
                        {
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
                        } else {
                            (identity, None, None)
                        }
                    } else {
                        (identity, None, None)
                    };
                prepare_schematic_resolution(state, identity, sheet_catalog, sheet_id, false, scope)
            }
            ViewType::Symbol => {
                let document = state
                    .load_active_symbol_document()
                    .map_err(HardcopySourceError::StaleActiveDocumentAuthority)?;
                Ok(PreparedRetainedHardcopyResolution {
                    payload: PreparedRetainedHardcopyPayload::Symbol {
                        project_id,
                        identity,
                        document,
                        scope,
                    },
                })
            }
            view_type => Err(HardcopySourceError::UnsupportedDocument(format!(
                "active design view type {view_type:?} has no semantic hardcopy adapter"
            ))),
        };
    }

    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        state.workbench.documents.active(Workspace::Results)
        && let Some((document, page, pane)) =
            active_visualization_document_pane(state, *document_id)
        && source_key == visualization_document_pane_source_key(project_id, document.id(), pane.id)
    {
        let all_panes = matches!(scope, HardcopyScope::AllSheetsOrPanes);
        return Ok(PreparedRetainedHardcopyResolution {
            payload: PreparedRetainedHardcopyPayload::VisualizationDocument {
                source_key: source_key.to_owned(),
                project_id,
                document: document.clone(),
                page_id: page.id,
                pane_id: pane.id,
                all_panes,
                scope,
            },
        });
    }

    if let Ok(displayed) =
        crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(
            state,
        )
        && matches!(
            displayed.owner,
            crate::workbench::documents::result_document::view_context::ResultViewOwner::Dataset
        )
        && let Some(run) = displayed.run(state)
    {
        let result_key = format!(
            "project:{}:result-dataset:{}",
            project_id.as_uuid(),
            run.dataset_id
        );
        if source_key == result_key {
            require_active_result_document(state, run.dataset_id)?;
            let viewer = displayed.viewer;
            let prepared_run = if matches!(viewer, ResultViewer::Manifest | ResultViewer::Specs) {
                // Dataset-native report sheets judge the complete immutable
                // run, including missing rows and cross-analysis worst cases.
                run.clone()
            } else if crate::workbench::documents::result_document::viewer_uses_wave_stack(viewer) {
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
                        "no retained analysis can provide exact evidence for {}",
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
            return Ok(PreparedRetainedHardcopyResolution {
                payload: PreparedRetainedHardcopyPayload::Results {
                    source_key: source_key.to_owned(),
                    project_id,
                    run: prepared_run,
                    presentation: capture_results_quick_view_presentation(state)?,
                    scope,
                },
            });
        }
    }

    if let Some(pane) = state
        .workbench
        .visualization_studio
        .panes
        .iter()
        .find(|pane| {
            format!(
                "project:{}:visualization-pane:{}",
                project_id.as_uuid(),
                pane.id
            ) == source_key
        })
    {
        let all_panes = matches!(scope, HardcopyScope::AllSheetsOrPanes);
        let mut studio = state.workbench.visualization_studio.clone();
        let relevant_panes = if all_panes {
            studio.panes.clone()
        } else {
            studio.panes.retain(|candidate| candidate.id == pane.id);
            studio.active_pane = Some(pane.id);
            studio.panes.clone()
        };
        let simulation = prepared_simulation_for_panes(&state.simulation, &relevant_panes);
        return Ok(PreparedRetainedHardcopyResolution {
            payload: PreparedRetainedHardcopyPayload::Studio {
                source_key: source_key.to_owned(),
                project_id,
                studio,
                simulation,
                pane_id: pane.id,
                all_panes,
                scope,
            },
        });
    }

    if let Some(document_id) = state.workbench.report_authoring.selected_document {
        let report_key = format!("project:{}:report:{}", project_id.as_uuid(), document_id);
        if source_key == report_key {
            let document = state
                .workspace
                .content
                .report_documents
                .iter()
                .find(|document| document.id() == document_id)
                .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
            let reference_inventory = report_inventory::reference_inventory(state, document)?;
            return Ok(PreparedRetainedHardcopyResolution {
                payload: PreparedRetainedHardcopyPayload::Report {
                    project_id,
                    source_key: source_key.to_owned(),
                    document: document.clone(),
                    reference_inventory,
                    scope,
                },
            });
        }
    }

    Err(HardcopySourceError::SourceNotRetained(
        source_key.to_owned(),
    ))
}

fn prepare_schematic_resolution(
    state: &AppState,
    identity: HardcopySourceIdentity,
    sheet_catalog: Option<SheetCatalog>,
    sheet_id: Option<SheetId>,
    all_sheets: bool,
    scope: HardcopyScope,
) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
    Ok(PreparedRetainedHardcopyResolution {
        payload: PreparedRetainedHardcopyPayload::Schematic {
            project_id: state.workspace.content.project.id(),
            identity,
            schematic: state.schematic.clone(),
            library_manager: state.library_manager.clone(),
            schematic_buffers: state.workspace.content.schematic_buffers.clone(),
            sheet_catalog,
            sheet_id,
            project_default_drawing_sheet: state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .default_format
                .clone(),
            project_title_block_field_values: state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values
                .clone(),
            all_sheets,
            scope,
        },
    })
}

fn prepared_simulation_for_panes(
    simulation: &SimulationState,
    panes: &[StudioPane],
) -> SimulationState {
    let dataset_ids = panes
        .iter()
        .map(|pane| pane.dataset_id)
        .collect::<std::collections::HashSet<_>>();
    let analysis_ids = panes
        .iter()
        .map(|pane| (pane.dataset_id, pane.analysis_sequence))
        .collect::<std::collections::HashSet<_>>();
    SimulationState {
        runs: simulation
            .runs
            .iter()
            .filter(|run| dataset_ids.contains(&run.dataset_id))
            .cloned()
            .map(|mut run| {
                run.data
                    .analyses
                    .retain(|analysis| analysis_ids.contains(&(run.data.dataset_id, analysis.id)));
                run
            })
            .collect(),
        ..Default::default()
    }
}

/// Per-frame command predicate. This deliberately performs identity and
/// evidence-shape checks only; full digesting and semantic resolution occur
/// when the dialog opens or commits.
#[must_use]
pub(crate) fn active_app_hardcopy_source_available(state: &AppState) -> bool {
    match state.workbench.current_route().surface_id() {
        SurfaceId::Design => {
            matches!(
                state.workbench.documents.active(Workspace::Design),
                Some(WorkspaceDocumentId::CellView(reference))
                    if reference == &state.workspace.content.active_view
            ) && matches!(
                state.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
            )
        }
        SurfaceId::Results => match state.workbench.documents.active(Workspace::Results) {
            Some(WorkspaceDocumentId::ResultDataset(dataset)) => state
                .simulation
                .run_by_dataset_id(*dataset)
                .is_some_and(|run| quick_result_availability(state, run).is_available()),
            Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
                active_visualization_document_pane(state, *document_id).is_some_and(
                    |(document, _, pane)| {
                        visualization_document_pane_availability(document, pane).is_available()
                    },
                )
            }
            _ => false,
        },
        SurfaceId::VisualizationStudio => {
            if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
                state.workbench.documents.active(Workspace::Results)
            {
                active_visualization_document_pane(state, *document_id).is_some_and(
                    |(document, _, pane)| {
                        visualization_document_pane_availability(document, pane).is_available()
                    },
                )
            } else {
                state
                    .workbench
                    .visualization_studio
                    .active_pane
                    .and_then(|pane_id| {
                        state
                            .workbench
                            .visualization_studio
                            .panes
                            .iter()
                            .find(|pane| pane.id == pane_id)
                    })
                    .is_some_and(|pane| studio_pane_availability(state, pane).is_available())
            }
        }
        SurfaceId::ReportAuthoring => state
            .workbench
            .report_authoring
            .selected_document
            .and_then(|document_id| {
                state
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
fn resolved_run_specifications_are_empty(state: &AppState, run: &SimulationRun) -> bool {
    run.prepared_receipt().map_or_else(
        || state.workspace.content.specs.is_empty(),
        |receipt| receipt.specifications().is_empty(),
    )
}

fn quick_result_availability(
    state: &AppState,
    run: &SimulationRun,
) -> RetainedHardcopySourceAvailability {
    let unavailable = |reason: String| RetainedHardcopySourceAvailability::Unavailable { reason };
    if !run.lifecycle.is_terminal() {
        return unavailable(format!(
            "dataset {} belongs to a non-terminal run",
            run.dataset_id
        ));
    }
    let viewer = state.ui.results.viewer;
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
        let has_evidence = !resolved_run_specifications_are_empty(state, run)
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
    let Some(index) = quick_result_analysis_index(state, run, viewer) else {
        return unavailable(format!(
            "no retained analysis can provide exact evidence for {}",
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
            waveform.x.len() >= crate::analysis::fft::MIN_FFT_SAMPLES
                && waveform.x.len() == waveform.y.len()
        }),
        ResultViewer::HarmonicBalance => {
            crate::workbench::documents::result_document::harmonic_balance_analysis_is_renderable(
                analysis,
            )
        }
        ResultViewer::PhaseNoise => {
            crate::workbench::documents::result_document::phase_noise_analysis_is_renderable(
                analysis,
            )
        }
        ResultViewer::Eye => visible_waveforms()
            .any(|waveform| waveform.x.len() >= 8 && waveform.x.len() == waveform.y.len()),
        ResultViewer::Hist => matches!(
            analysis.family_metadata.as_ref(),
            Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. })
                if crate::analysis::histogram::state::measurement_index(
                    state.analysis.histogram_state.selected.as_deref(), 0,
                    &variables.iter().map(|variable| variable.name.as_str()).collect::<Vec<_>>(),
                ).and_then(|index| variables.get(index))
                    .is_some_and(|variable| !variable.samples.is_empty())
        ),
        ResultViewer::Nyquist => visible_waveforms().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty() && complex.real.len() == complex.imag.len()
            })
        }),
        ResultViewer::Smith => {
            crate::workbench::documents::result_document::smith_analysis_is_renderable(analysis)
        }
        ResultViewer::NetworkMatrix => {
            crate::workbench::documents::result_document::view_context::analysis_supports_viewer(
                viewer, analysis,
            )
        }
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
            ordinary_noise_spectrum_is_renderable(analysis)
                || crate::workbench::documents::result_document::qpnoise_spectrum_is_renderable(
                    analysis,
                )
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
            "active analysis {} has no exact evidence for {}",
            analysis.id,
            viewer.label()
        ))
    }
}

fn transient_waveform_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && analysis.analysis_type.is_time_domain()
        && analysis.waveforms.iter().any(|waveform| {
            waveform.visible && !waveform.x.is_empty() && waveform.x.len() == waveform.y.len()
        })
}

fn bode_response_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    crate::workbench::documents::result_document::bode_analysis_is_renderable(analysis)
}

fn quick_result_analysis_index(
    state: &AppState,
    run: &SimulationRun,
    viewer: ResultViewer,
) -> Option<usize> {
    let globally_selected = (state.simulation.active_run_idx
        == state
            .simulation
            .runs
            .iter()
            .position(|candidate| candidate.dataset_id == run.dataset_id))
    .then_some(state.simulation.active_analysis_idx)
    .flatten();
    match viewer {
        ResultViewer::Waves => globally_selected
            .filter(|&index| {
                run.analyses
                    .get(index)
                    .is_some_and(transient_waveform_analysis_is_renderable)
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(transient_waveform_analysis_is_renderable)
            }),
        // The one binding the sheet uses, so the page and the screen name the
        // same analysis. `filter` + `or_else` is not that binding: it steps to
        // the next renderable result whenever the reader's own selection is a
        // noise analysis that carries no ordinary spectrum, and prints another
        // analysis's contributors under the selected one's name.
        ResultViewer::NoiseContrib => selected_noise_analysis_index(globally_selected, run),
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
                    .is_some_and(bode_response_analysis_is_renderable)
            })
            .or_else(|| {
                run.analyses
                    .iter()
                    .position(bode_response_analysis_is_renderable)
            }),
        ResultViewer::PhaseNoise => globally_selected
            .filter(|&index| {
                run.analyses.get(index).is_some_and(|analysis| {
                    crate::workbench::documents::result_document::phase_noise_analysis_is_renderable(
                        analysis,
                    )
                })
            })
            .or_else(|| {
                run.analyses.iter().position(|analysis| {
                    crate::workbench::documents::result_document::phase_noise_analysis_is_renderable(
                        analysis,
                    )
                })
            }),
        ResultViewer::HarmonicBalance => globally_selected
            .filter(|&index| {
                run.analyses.get(index).is_some_and(|analysis| {
                    crate::workbench::documents::result_document::harmonic_balance_analysis_is_renderable(
                        analysis,
                    )
                })
            })
            .or_else(|| {
                run.analyses.iter().position(|analysis| {
                    crate::workbench::documents::result_document::harmonic_balance_analysis_is_renderable(
                        analysis,
                    )
                })
            }),
        _ => globally_selected
            .filter(|&index| {
                run.analyses.get(index).is_some_and(|analysis| {
                    crate::workbench::documents::result_document::view_context::analysis_supports_viewer(
                        viewer, analysis,
                    )
                })
            })
            .or_else(|| {
                run.analyses.iter().position(|analysis| {
                    crate::workbench::documents::result_document::view_context::analysis_supports_viewer(
                        viewer, analysis,
                    )
                })
            }),
    }
}

fn studio_pane_availability(
    state: &AppState,
    pane: &StudioPane,
) -> RetainedHardcopySourceAvailability {
    let unavailable = |reason: String| RetainedHardcopySourceAvailability::Unavailable { reason };
    let Some(run) = state.simulation.run_by_dataset_id(pane.dataset_id) else {
        return unavailable(format!("dataset {} is not retained", pane.dataset_id));
    };
    if !run.lifecycle.is_terminal() {
        return unavailable(format!("dataset {} is not terminal", pane.dataset_id));
    }
    let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == pane.analysis_sequence)
    else {
        return unavailable(format!(
            "analysis {} is not retained in dataset {}",
            pane.analysis_sequence, pane.dataset_id
        ));
    };
    if !analysis.success {
        return unavailable(format!(
            "analysis {} is unsuccessful",
            pane.analysis_sequence
        ));
    }
    if is_curve_viewer(pane.viewer) && !studio_curve_viewer_is_supported(pane.viewer) {
        return unavailable(format!(
            "{} has no faithful semantic Studio figure writer",
            pane.viewer.label()
        ));
    }
    let specialist_evidence_available = match pane.viewer {
        ResultViewer::HarmonicBalance => {
            crate::workbench::documents::result_document::harmonic_balance_analysis_is_renderable(
                analysis,
            ) && analysis.waveforms.iter().any(|waveform| {
                waveform.visible
                    && crate::workbench::documents::result_document::harmonic_balance_waveform_is_renderable(
                        waveform,
                    )
            })
        }
        ResultViewer::PhaseNoise => {
            crate::workbench::documents::result_document::phase_noise_analysis_is_renderable(
                analysis,
            ) && analysis.waveforms.iter().any(|waveform| {
                waveform.visible
                    && crate::workbench::documents::result_document::phase_noise_waveform_is_renderable(
                        waveform,
                    )
            })
        }
        _ => true,
    };
    if !specialist_evidence_available {
        return unavailable(format!(
            "analysis {} does not retain visible exact evidence for {}",
            pane.analysis_sequence,
            pane.viewer.label()
        ));
    }
    RetainedHardcopySourceAvailability::Available
}

fn active_visualization_document_pane(
    state: &AppState,
    document_id: crate::product::ResultDocumentId,
) -> Option<(&VisualizationDocument, &Page, &Pane)> {
    let document = state
        .workspace
        .content
        .visualization_document(document_id)?;
    if let Some(pane) = state
        .workbench
        .visualization_studio
        .active_pane
        .and_then(|pane_id| {
            document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == pane_id)
        })
        && let Some(page) = document.pages().iter().find(|page| page.id == pane.page_id)
    {
        return Some((document, page, pane));
    }
    let selected_page_id = state
        .ui
        .results
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
        return unavailable("the selected result pane has no immutable dataset binding");
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

/// Resolve the one application document that owns the current route.
///
/// This is the sole AppState integration boundary for File > Print and page
/// preview. Every branch verifies the stable open-document selection before
/// borrowing engineering content; background buffers and most-recent results
/// are never substituted for an absent or stale active authority.
// The fail-closed single-route compatibility boundary. Nothing in the
// application reaches it; the hardcopy tests do.
#[cfg(test)]
pub(crate) fn resolve_active_app_hardcopy_source(
    state: &AppState,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let project_id = state.workspace.content.project.id();
    match state.workbench.current_route().surface_id() {
        SurfaceId::Design => {
            let active = state
                .workbench
                .documents
                .active(Workspace::Design)
                .ok_or(HardcopySourceError::NoActiveDocumentAuthority("design"))?;
            match active {
                WorkspaceDocumentId::CellView(reference)
                    if reference == &state.workspace.content.active_view => {}
                other => {
                    return Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
                        "design registry points at {other:?}, but the active view is {}",
                        state.workspace.content.active_display_path()
                    )));
                }
            }
            let identity = active_cell_view_identity(state)?;
            match state.workspace.content.active_view_type() {
                ViewType::Schematic | ViewType::Testbench => {
                    let resolver = SymbolResolver::new(
                        &state.library_manager,
                        &state.workspace.content.schematic_buffers,
                    );
                    resolve_schematic_source(SchematicHardcopySource {
                        identity,
                        schematic: state.schematic.editor_ref().design,
                        selection: None,
                        expected_topology_version: state.schematic.topology_version(),
                        symbol_resolver: Some(resolver.design_resolver()),
                        sheet_catalog: None,
                        sheet_id: None,
                        project_default_drawing_sheet: Some(
                            &state
                                .workspace
                                .content
                                .design_management
                                .drawing_sheet_settings()
                                .default_format,
                        ),
                        project_title_block_field_values: Some(
                            &state
                                .workspace
                                .content
                                .design_management
                                .drawing_sheet_settings()
                                .title_block_field_values,
                        ),
                        scope: HardcopyScope::ActiveDocument,
                    })
                }
                ViewType::Symbol => {
                    let document = state.load_active_symbol_document().map_err(|reason| {
                        HardcopySourceError::StaleActiveDocumentAuthority(reason)
                    })?;
                    resolve_symbol_source(SymbolHardcopySource {
                        identity,
                        document: &document,
                        selection: None,
                        scope: HardcopyScope::ActiveDocument,
                    })
                }
                view_type => Err(HardcopySourceError::UnsupportedDocument(format!(
                    "active design view type {view_type:?} has no semantic hardcopy adapter"
                ))),
            }
        }
        SurfaceId::Results => match state.workbench.documents.active(Workspace::Results) {
            Some(WorkspaceDocumentId::ResultDataset(_)) => {
                let run = active_terminal_run(state)?;
                require_active_result_document(state, run.dataset_id)?;
                resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
                    source_key: format!(
                        "project:{}:result-dataset:{}",
                        project_id.as_uuid(),
                        run.dataset_id
                    ),
                    project_id,
                    state,
                    scope: HardcopyScope::ActivePlotDocument,
                })
            }
            Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
                let (document, page, pane) =
                    active_visualization_document_pane(state, *document_id).ok_or(
                        HardcopySourceError::NoActiveDocumentAuthority("result document pane"),
                    )?;
                resolve_visualization_document_source(
                    visualization_document_pane_source_key(project_id, document.id(), pane.id),
                    project_id,
                    document,
                    page.id,
                    pane.id,
                    false,
                    HardcopyScope::ActivePlotDocument,
                )
            }
            other => Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
                "results registry points at {other:?}"
            ))),
        },
        SurfaceId::VisualizationStudio => {
            let pane_id = state.workbench.visualization_studio.active_pane.ok_or(
                HardcopySourceError::NoActiveDocumentAuthority("visualization pane"),
            )?;
            let pane = state
                .workbench
                .visualization_studio
                .panes
                .iter()
                .find(|pane| pane.id == pane_id)
                .ok_or_else(|| {
                    HardcopySourceError::StaleActiveDocumentAuthority(format!(
                        "visualization pane {pane_id} is not retained"
                    ))
                })?;
            require_active_result_document(state, pane.dataset_id)?;
            resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
                source_key: format!(
                    "project:{}:visualization-pane:{pane_id}",
                    project_id.as_uuid()
                ),
                project_id,
                studio: &state.workbench.visualization_studio,
                simulation: &state.simulation,
                pane_id,
                scope: HardcopyScope::ActivePlotDocument,
            })
        }
        SurfaceId::ReportAuthoring => {
            let document_id = state.workbench.report_authoring.selected_document.ok_or(
                HardcopySourceError::NoActiveDocumentAuthority("report document"),
            )?;
            let matching = state
                .workspace
                .content
                .report_documents
                .iter()
                .filter(|document| document.id() == document_id)
                .collect::<Vec<_>>();
            let [document] = matching.as_slice() else {
                return if matching.is_empty() {
                    Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
                        "selected report {document_id} is not retained"
                    )))
                } else {
                    Err(HardcopySourceError::AmbiguousActiveSource(format!(
                        "report {document_id}"
                    )))
                };
            };
            report_inventory::resolve(state, document, HardcopyScope::CompleteReport)
        }
        surface => Err(HardcopySourceError::UnsupportedDocument(format!(
            "surface {} does not own a printable engineering document",
            surface.as_str()
        ))),
    }
}

fn active_cell_view_identity(
    state: &AppState,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let project_id = state.workspace.content.project.id();
    let view_key = state.workspace.content.active_key();
    let mut identity_material = b"rspice-cell-view-hardcopy-v1:".to_vec();
    identity_material.extend_from_slice(view_key.as_bytes());
    HardcopySourceIdentity::try_new(
        format!("project:{}:cell-view:{view_key}", project_id.as_uuid()),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_material))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        state.workspace.content.project.revision(),
        state.workspace.content.active_display_path(),
    )?
    .with_publication(HardcopyPublicationIdentity::try_new(
        state.workspace.content.project.name(),
        state.workspace.content.active_view.display_path(),
        Some(
            state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision
                .clone(),
        ),
        (!state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .revision_date_utc
            .is_empty())
        .then(|| {
            state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision_date_utc
                .clone()
        }),
    )?)
}

fn require_active_result_document(
    state: &AppState,
    expected_dataset: DatasetId,
) -> Result<(), HardcopySourceError> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset)) if *dataset == expected_dataset => Ok(()),
        Some(other) => Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
            "results registry points at {other:?}, expected dataset {expected_dataset}"
        ))),
        None => Err(HardcopySourceError::NoActiveDocumentAuthority(
            "result dataset",
        )),
    }
}

fn selected_symbol_document(
    document: &SymbolDocument,
    selection: &SymbolSelection,
) -> Result<SymbolDocument, HardcopySourceError> {
    if selection.is_empty() {
        return Err(HardcopySourceError::EmptySelection);
    }
    let pins = document
        .pins
        .iter()
        .filter(|pin| selection.pins.contains(&pin.name))
        .cloned()
        .collect();
    let body = document
        .body
        .iter()
        .enumerate()
        .filter(|(index, _)| selection.shapes.contains(index))
        .map(|(_, shape)| shape.clone())
        .collect();
    let selected = SymbolDocument {
        pins,
        body,
        origin: document.origin,
        // Anchor handles are not selectable symbol objects. Collapse them to
        // the origin so an unrelated label anchor cannot enlarge a selection
        // hardcopy extent.
        name_anchor: document.origin,
        value_anchor: document.origin,
    };
    if selected.pins.is_empty() && selected.body.is_empty() {
        return Err(HardcopySourceError::EmptySelection);
    }
    Ok(selected)
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(crate) use test_support::resolve_retained_hardcopy_source;
#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::hardcopy::PrintObjectKind;
#[cfg(test)]
use crate::results::report_document::FrozenReportArtifact;
#[cfg(test)]
use crate::state::{Point, SymbolShape};
