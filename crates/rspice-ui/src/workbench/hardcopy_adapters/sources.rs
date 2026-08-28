//! Exact, semantic hardcopy source resolution.
//!
//! Hardcopy starts here rather than at a viewport or GPU surface.  Every
//! adapter freezes one durable document revision, resolves authored symbol
//! artwork and retained result samples, computes deterministic physical
//! bounds, and authenticates the semantic snapshot before rendering begins.
//! No type in this module contains pixels, an egui paint command, or a screen
//! rectangle.

mod axis;
mod documents;
mod geometry;
mod noise;
mod prepared;
mod quick_plots;
mod quick_view_overlay;
mod report_inventory;
mod results;
mod semantic;

pub use documents::*;
// Crate-private: `geometry` exposes only `pub(super)` helpers, which the
// sibling modules reach through `use super::*`.
pub(crate) use geometry::*;
// Module-private: `axis`, `noise`, `quick_plots` and `quick_view_overlay`
// expose only `pub(super)` items, and the siblings reach them through
// `use super::*`.
use axis::*;
use noise::*;
pub use prepared::*;
use quick_plots::*;
use quick_view_overlay::*;
pub use results::*;
pub use semantic::*;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::io::ProjectSimulationResults;
use crate::product::{ContentDigest, DatasetId, ObjectRevision, ProjectId, RunId};
#[cfg(test)]
use crate::results::report_document::FigureSizing;
use crate::results::report_document::{
    FrozenReportArtifact, ReportBlockId, ReportBlockKind, ReportDocument,
    ReportReferenceCurrentness, ReportReferenceInventory, ReportReferenceMode,
    ReportReferenceSnapshot, ReportSourceId,
};
use crate::results::visualization_document::{
    AnnotationAnchor, AxisScale, Page, PageId, Pane, PaneId, TypedValue, VisualizationDocument,
};
use crate::results::visualization_raster::{
    ResolvedCartesianLineScene, VisualizationRasterError, resolve_cartesian_line_scene,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType, Bus, BusTap,
    Component, ComponentType, DesignNote, DesignSheet, DocumentationShape,
    DrawingSheetBorderTemplate, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId, Junction,
    NetLabel, Point, ResolvedSymbolIssueKind, ResolvedSymbolSource, SchematicSheetFormat,
    SchematicState, Selection, SheetCatalog, SheetId, SimulationRun, SimulationState,
    SymbolDocument, SymbolResolver, SymbolShape, ViewType, WaveformData, Wire,
};
use crate::workbench::AppState;

use crate::hardcopy::{
    ActiveHardcopySource, HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, Length,
    PrintColor, PrintMappingEntry, PrintMappingSaveScope, PrintMappingTable, PrintObjectIdentity,
    PrintObjectKind, PrintRedundancy,
};
use crate::workbench::SurfaceId;
// The persisted source-set records and the validation they share with these
// adapters are owned one layer down, where `state` can reach them.
use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, HardcopyPublicationIdentity, HardcopySourceError, HardcopySourceIdentity,
    HardcopySourceSet, HardcopySourceSetMember, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT,
    canonical_digest, validate_label,
};
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::documents::visualization_studio::{
    VisualizationAnnotation as StudioAnnotation, VisualizationAutoscale,
    VisualizationMarker as StudioMarker, VisualizationPane as StudioPane, VisualizationStudioState,
};
use crate::workbench::lifecycle::session::SymbolSelection;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

/// Natural physical scale for schematic coordinates.
///
/// The authored drawing-sheet contract defines exactly four editor units per
/// millimetre. Page fitting can subsequently scale this scene, but retaining
/// the exact 250 micrometre calibration here keeps canvas coordinates,
/// overflow reports, and 1:1 hardcopy physically identical on every target.
pub const SCHEMATIC_UNIT_UM: i64 = 250;
/// Fixed top-left authored page origin in schematic world units.
pub const SCHEMATIC_SHEET_ORIGIN_X_UNITS: i64 = -140;
pub const SCHEMATIC_SHEET_ORIGIN_Y_UNITS: i64 = -40;
/// Natural active-plot canvas (10 by 5.625 inches, 16:9).
pub const PLOT_WIDTH_UM: i64 = 254_000;
pub const PLOT_HEIGHT_UM: i64 = 142_875;
/// Natural report page used to arrange the report's already-authored pages.
pub const REPORT_PAGE_WIDTH_UM: i64 = 215_900;
pub const REPORT_PAGE_HEIGHT_UM: i64 = 279_400;
pub const REPORT_PAGE_GAP_UM: i64 = 5_000;
#[cfg(test)]
pub const BLANK_SCHEMATIC_SHEET_WIDTH_UM: i64 = 279_400;
#[cfg(test)]
pub const BLANK_SCHEMATIC_SHEET_HEIGHT_UM: i64 = 215_900;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) const MAX_WORKER_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 3;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 8;
const SCHEMATIC_EDGE_ALLOWANCE_UNITS: i64 = 16;
const SYMBOL_EDGE_ALLOWANCE_UNITS: i64 = 10;
const PLOT_INSET_UM: i64 = 12_700;

pub struct SchematicHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub schematic: &'a SchematicState,
    pub expected_topology_version: u64,
    pub symbol_resolver: Option<&'a SymbolResolver<'a>>,
    /// Optional governed multi-sheet partition. Absence means the legacy
    /// single-sheet document. When present, `sheet_id` must name an exact
    /// retained catalog sheet and only objects owned by that sheet resolve.
    pub sheet_catalog: Option<&'a SheetCatalog>,
    pub sheet_id: Option<SheetId>,
    /// Current project default used by the canvas for ungoverned documents
    /// and for governed sheets that follow the project default.
    pub project_default_drawing_sheet: Option<&'a SchematicSheetFormat>,
    /// Canonical project-owned values used by every sheet title block.
    pub project_title_block_field_values:
        Option<&'a std::collections::BTreeMap<DrawingSheetTitleFieldId, String>>,
    pub scope: HardcopyScope,
}

pub struct SchematicSheetSetHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub schematic: &'a SchematicState,
    pub expected_topology_version: u64,
    pub symbol_resolver: Option<&'a SymbolResolver<'a>>,
    pub sheet_catalog: &'a SheetCatalog,
    pub project_default_drawing_sheet: &'a SchematicSheetFormat,
    pub project_title_block_field_values:
        &'a std::collections::BTreeMap<DrawingSheetTitleFieldId, String>,
}

pub struct SymbolHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub document: &'a SymbolDocument,
    pub selection: Option<&'a SymbolSelection>,
    pub scope: HardcopyScope,
}

pub struct PlotHardcopySource<'a> {
    pub source_key: String,
    pub display_name: String,
    pub scene: &'a ResolvedCartesianLineScene,
    pub scope: HardcopyScope,
}

/// Active Visualization Studio pane together with the immutable reference
/// manifest that names its exact document revision and retained datasets.
pub struct VisualizationPaneHardcopySource<'a> {
    pub source_key: String,
    pub display_name: String,
    pub document: &'a VisualizationDocument,
    pub reference: &'a ReportReferenceSnapshot,
    pub page_id: PageId,
    pub pane_id: PaneId,
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

#[derive(Debug, Clone)]
struct ResultsQuickViewPresentation {
    viewer: ResultViewer,
    /// The reading each strip was carrying: hidden traces, placed cursors,
    /// anchored markers. Frozen here so the worker resolves the page the
    /// reader reviewed rather than a bare plot of the same samples.
    overlay: RetainedQuickViewOverlays,
    specs: Vec<crate::state::SpecEntry>,
    fft: crate::analysis::FftState,
    histogram_selected: usize,
    histogram_bin_count: usize,
    histogram_custom_range: bool,
    histogram_custom_min: f64,
    histogram_custom_max: f64,
    histogram_mode: crate::analysis::histogram::HistogramDisplayMode,
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

impl ResultsQuickViewPresentation {
    fn from_state(state: &AppState) -> Self {
        let mut fft = crate::analysis::FftState::default();
        fft.selected_source = state.analysis.fft_state.selected_source.clone();
        fft.normalization = state.analysis.fft_state.normalization;
        fft.window = state.analysis.fft_state.window;
        fft.input_fidelity = state.analysis.fft_state.input_fidelity;
        fft.time_window_auto = state.analysis.fft_state.time_window_auto;
        fft.time_window_start = state.analysis.fft_state.time_window_start;
        fft.time_window_end = state.analysis.fft_state.time_window_end;
        fft.sample_count_auto = state.analysis.fft_state.sample_count_auto;
        fft.sample_count = state.analysis.fft_state.sample_count;
        // Every strip of the captured run, because a stacked wave view
        // resolves one page per analysis through this one presentation.
        let overlay = captured_results_run(state)
            .map(|run| RetainedQuickViewOverlays::capture(state, run))
            .unwrap_or_default();
        Self {
            viewer: state.ui.results.viewer,
            overlay,
            specs: crate::workbench::documents::result_document::run_specifications(state),
            fft,
            histogram_selected: state.analysis.histogram_state.selected,
            histogram_bin_count: state.analysis.histogram_state.bin_count,
            histogram_custom_range: state.analysis.histogram_state.custom_range,
            histogram_custom_min: state.analysis.histogram_state.custom_min,
            histogram_custom_max: state.analysis.histogram_state.custom_max,
            histogram_mode: state.analysis.histogram_state.mode,
        }
    }
}

pub struct ReportHardcopySource<'a> {
    pub source_key: String,
    pub document: &'a ReportDocument,
    /// Exact retained live-source inventory for linked references. Frozen
    /// blocks authenticate from their embedded artifact and do not require it.
    pub reference_inventory: Option<&'a ReportReferenceInventory>,
    pub scope: HardcopyScope,
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
    let project_id = state.workspace.project.id();
    let mut descriptors = Vec::new();

    if matches!(
        state.workbench.documents.active(Workspace::Design),
        Some(WorkspaceDocumentId::CellView(reference)) if reference == &state.workspace.active_view
    ) {
        let view_type = state.workspace.active_view_type();
        let active_key = state.workspace.active_key();
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
                .design_management
                .sheet_catalog(&active_key)
                .is_some_and(|catalog| !catalog.sheets().is_empty())
        {
            allowed_scopes.push(HardcopyScope::AllSheetsOrPanes);
        }
        descriptors.push(RetainedHardcopySourceDescriptor {
            source_key: design_source_key.clone(),
            display_name: state.workspace.active_display_path(),
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
            && let Some(catalog) = state.workspace.design_management.sheet_catalog(&active_key)
        {
            for sheet in catalog.sheets() {
                descriptors.push(RetainedHardcopySourceDescriptor {
                    source_key: format!("{design_source_key}:sheet:{}", sheet.id()),
                    display_name: compact_display(
                        &format!(
                            "{} · {}",
                            state.workspace.active_display_path(),
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

    if let Some(source_set) = state.workspace.hardcopy_source_set(source_key) {
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

    let project_id = state.workspace.project.id();
    let design_key = format!(
        "project:{}:cell-view:{}",
        project_id.as_uuid(),
        state.workspace.active_key()
    );
    if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        let active_key = state.workspace.active_key();
        let catalog = state.workspace.design_management.sheet_catalog(&active_key);
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
        return match state.workspace.active_view_type() {
            ViewType::Schematic | ViewType::Testbench => {
                let active_key = state.workspace.active_key();
                let catalog = state.workspace.design_management.sheet_catalog(&active_key);
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
                    presentation: ResultsQuickViewPresentation::from_state(state),
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
            project_id: state.workspace.project.id(),
            identity,
            schematic: state.schematic.clone(),
            library_manager: state.library_manager.clone(),
            schematic_buffers: state.workspace.schematic_buffers.clone(),
            sheet_catalog,
            sheet_id,
            project_default_drawing_sheet: state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .default_format
                .clone(),
            project_title_block_field_values: state
                .workspace
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
                run.analyses
                    .retain(|analysis| analysis_ids.contains(&(run.dataset_id, analysis.id)));
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
                    if reference == &state.workspace.active_view
            ) && matches!(
                state.workspace.active_view_type(),
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
pub fn resolve_hardcopy_source_set_with(
    source_set: &HardcopySourceSet,
    mut resolve_member: impl FnMut(
        &HardcopySourceSetMember,
    ) -> Result<ResolvedHardcopyDocument, HardcopySourceError>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    source_set.validate()?;

    let mut children = Vec::with_capacity(source_set.members().len());
    let mut next_y_um = 0_i64;
    let mut maximum_width_um = 0_i64;
    for (index, member) in source_set.members().iter().enumerate() {
        let resolved = resolve_member(member)?;
        validate_source_set_member_authority(source_set, member, &resolved)?;
        if matches!(
            resolved.semantic_document(),
            HardcopySemanticDocument::Aggregate(_)
        ) {
            return Err(HardcopySourceError::InvalidSourceSet(
                "nested semantic aggregates are not supported".to_owned(),
            ));
        }

        let bounds = resolved.bounds();
        let width = bounds
            .maximum
            .x_um
            .checked_sub(bounds.minimum.x_um)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let height = bounds
            .maximum
            .y_um
            .checked_sub(bounds.minimum.y_um)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        maximum_width_um = maximum_width_um.max(width);
        let ordinal = u32::try_from(index).map_err(|_| HardcopySourceError::CoordinateOverflow)?;
        let ResolvedHardcopyDocument {
            source_key,
            authority,
            semantic_document,
            ..
        } = resolved;
        children.push(SemanticAggregateChild {
            ordinal,
            source_key,
            display_name: authority.display_name().to_owned(),
            document_id: authority.document_id(),
            revision: authority.revision(),
            content_digest: authority.content_digest(),
            local_bounds: bounds,
            placement_origin: SemanticPoint::new(0, next_y_um),
            page_break_before: index != 0,
            publication_page_label: None,
            document: Box::new(semantic_document),
        });
        next_y_um = next_y_um
            .checked_add(height)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        if index + 1 != source_set.members().len() {
            next_y_um = next_y_um
                .checked_add(REPORT_PAGE_GAP_UM)
                .ok_or(HardcopySourceError::CoordinateOverflow)?;
        }
    }

    // A per-print-set schematic is numbered by the exact ordered schematic
    // subset in this authenticated aggregate, not by its full source catalog.
    // The override belongs to the aggregate projection and therefore does not
    // mutate or misrepresent the retained child source digest.
    let per_print_set_count = children
        .iter()
        .filter(|child| {
            matches!(
                child.document.as_ref(),
                HardcopySemanticDocument::Schematic(schematic)
                    if schematic.drawing_sheet_page_numbering
                        == Some(crate::state::SheetPageNumbering::PerPrintSet)
            )
        })
        .count();
    if per_print_set_count > 0 {
        let mut page = 0_usize;
        for child in &mut children {
            if matches!(
                child.document.as_ref(),
                HardcopySemanticDocument::Schematic(schematic)
                    if schematic.drawing_sheet_page_numbering
                        == Some(crate::state::SheetPageNumbering::PerPrintSet)
            ) {
                page += 1;
                child.publication_page_label = Some(format!("{page} of {per_print_set_count}"));
            }
        }
    }

    let aggregate = SemanticAggregate {
        source_set_digest: source_set.definition_digest(),
        children,
    };
    let semantic_document = HardcopySemanticDocument::Aggregate(aggregate);
    let content_digest = canonical_digest(
        b"rspice-hardcopy-source-set-aggregate-v1",
        &(source_set.definition_digest(), &semantic_document),
    )?;
    let bounds = SemanticBounds::try_new(
        SemanticPoint::new(0, 0),
        SemanticPoint::new(maximum_width_um, next_y_um),
    )?;
    finish_resolved(
        HardcopySourceIdentity::try_new(
            source_set.source_key(),
            source_set.document_id(),
            source_set.revision(),
            source_set.name(),
        )?,
        content_digest,
        source_set.document_kind(),
        source_set.scope().clone(),
        semantic_document,
        bounds,
    )
}

fn validate_source_set_member_authority(
    source_set: &HardcopySourceSet,
    expected: &HardcopySourceSetMember,
    actual: &ResolvedHardcopyDocument,
) -> Result<(), HardcopySourceError> {
    let authority = actual.authority();
    let exact = actual.source_key() == expected.source_key()
        && authority.display_name() == expected.display_name()
        && authority.document_id() == expected.document_id()
        && authority.revision() == expected.revision()
        && authority.content_digest() == expected.content_digest()
        && authority.scope() == expected.scope();
    if !exact {
        return Err(HardcopySourceError::StaleSourceSetMember {
            source_key: expected.source_key().to_owned(),
        });
    }
    if source_set.document_kind() != HardcopyDocumentKind::EngineeringDocument
        && authority.document_kind() != source_set.document_kind()
    {
        return Err(HardcopySourceError::InvalidSourceSet(format!(
            "member {} has kind {:?}, expected {:?}",
            expected.source_key(),
            authority.document_kind(),
            source_set.document_kind()
        )));
    }
    Ok(())
}

/// Whether the requirement set this run is judged against is empty.
///
/// The run-scoped spelling of the sheet's `resolved_specifications`: a
/// dispatched run carries the requirements it froze, and only a legacy
/// dataset from before prepared-run receipts falls back to the workspace's
/// live contract.
fn resolved_run_specifications_are_empty(state: &AppState, run: &SimulationRun) -> bool {
    run.prepared_receipt().map_or_else(
        || state.workspace.specs.is_empty(),
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
        // `ResultsQuickViewPresentation::from_state` resolves it through the
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
                if variables
                    .get(state.analysis.histogram_state.selected)
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
        ResultViewer::NoiseContrib => ordinary_noise_spectrum_is_renderable(analysis),
        ResultViewer::Contribution => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Sensitivity { .. })
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
            }) if !digital_traces.is_empty() || !real_traces.is_empty()
        ),
        ResultViewer::Soa => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Soa { evaluations, .. }) if !evaluations.is_empty()
        ),
        ResultViewer::Reliability => matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Reliability { devices }) if !devices.is_empty()
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
    if is_curve_viewer(pane.viewer)
        && !matches!(
            pane.viewer,
            ResultViewer::Waves
                | ResultViewer::DcSweep
                | ResultViewer::NoiseContrib
                | ResultViewer::HarmonicBalance
                | ResultViewer::PhaseNoise
        )
    {
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
    let document = state.workspace.visualization_document(document_id)?;
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

fn visualization_document_pane_source_key(
    project_id: ProjectId,
    document_id: crate::product::ResultDocumentId,
    pane_id: PaneId,
) -> String {
    format!(
        "project:{}:result-document:{}:pane:{}",
        project_id.as_uuid(),
        document_id,
        pane_id.get()
    )
}

fn visualization_document_reference(
    document: &VisualizationDocument,
) -> Result<ReportReferenceSnapshot, HardcopySourceError> {
    let content_digest = document
        .content_digest()
        .map_err(|error| HardcopySourceError::InvalidVisualizationSource(error.to_string()))?;
    ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: document.id(),
        },
        Some(document.revision()),
        content_digest,
        document
            .datasets()
            .iter()
            .map(|dataset| dataset.binding())
            .collect(),
    )
    .map_err(|error| HardcopySourceError::InvalidVisualizationSource(error.to_string()))
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
    let project_id = state.workspace.project.id();
    match state.workbench.current_route().surface_id() {
        SurfaceId::Design => {
            let active = state
                .workbench
                .documents
                .active(Workspace::Design)
                .ok_or(HardcopySourceError::NoActiveDocumentAuthority("design"))?;
            match active {
                WorkspaceDocumentId::CellView(reference)
                    if reference == &state.workspace.active_view => {}
                other => {
                    return Err(HardcopySourceError::StaleActiveDocumentAuthority(format!(
                        "design registry points at {other:?}, but the active view is {}",
                        state.workspace.active_display_path()
                    )));
                }
            }
            let identity = active_cell_view_identity(state)?;
            match state.workspace.active_view_type() {
                ViewType::Schematic | ViewType::Testbench => {
                    let resolver = SymbolResolver::new(
                        &state.library_manager,
                        &state.workspace.schematic_buffers,
                    );
                    resolve_schematic_source(SchematicHardcopySource {
                        identity,
                        schematic: &state.schematic,
                        expected_topology_version: state.schematic.topology_version(),
                        symbol_resolver: Some(&resolver),
                        sheet_catalog: None,
                        sheet_id: None,
                        project_default_drawing_sheet: Some(
                            &state
                                .workspace
                                .design_management
                                .drawing_sheet_settings()
                                .default_format,
                        ),
                        project_title_block_field_values: Some(
                            &state
                                .workspace
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
    let project_id = state.workspace.project.id();
    let view_key = state.workspace.active_key();
    let mut identity_material = b"rspice-cell-view-hardcopy-v1:".to_vec();
    identity_material.extend_from_slice(view_key.as_bytes());
    HardcopySourceIdentity::try_new(
        format!("project:{}:cell-view:{view_key}", project_id.as_uuid()),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_material))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        state.workspace.project.revision(),
        state.workspace.active_display_path(),
    )?
    .with_publication(HardcopyPublicationIdentity::try_new(
        state.workspace.project.name(),
        state.workspace.active_view.display_path(),
        Some(
            state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision
                .clone(),
        ),
        (!state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .document_control
            .revision_date_utc
            .is_empty())
        .then(|| {
            state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision_date_utc
                .clone()
        }),
    )?)
}

pub(super) fn schematic_sheet_identity(
    base: &HardcopySourceIdentity,
    sheet: &DesignSheet,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_material = b"rspice-hardcopy-schematic-sheet-v1:".to_vec();
    identity_material.extend_from_slice(sheet.id().as_uuid().as_bytes());
    let mut identity = HardcopySourceIdentity::try_new(
        format!("{}:sheet:{}", base.source_key, sheet.id()),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &base.document_id.as_uuid(),
            &identity_material,
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(sheet.revision())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        compact_display(
            &format!("{} · {}", base.display_name, sheet.name()),
            "Schematic sheet",
        ),
    )?;
    identity.publication.clone_from(&base.publication);
    Ok(identity)
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

fn resolve_component_symbol(
    component: &Component,
    resolver: Option<&SymbolResolver<'_>>,
) -> Result<(Option<SymbolDocument>, Option<SemanticSymbolSource>), HardcopySourceError> {
    if component.kind != ComponentType::CellInstance {
        return Ok((None, None));
    }
    let binding = component.library_cell.as_ref().ok_or_else(|| {
        HardcopySourceError::UnresolvedCellSymbol {
            component_id: component.id,
            reason: "cell instance has no library/cell/view binding".to_owned(),
        }
    })?;
    if binding.is_executable_builtin() {
        // Compiled catalog devices are not authored project masters. Preserve
        // the instance for deterministic catalog/fallback rendering.
        return Ok((None, None));
    }
    let resolver = resolver.ok_or_else(|| HardcopySourceError::UnresolvedCellSymbol {
        component_id: component.id,
        reason: "no symbol resolver was supplied for the active project snapshot".to_owned(),
    })?;
    let resolved = resolver.resolve_binding(binding).ok_or_else(|| {
        HardcopySourceError::UnresolvedCellSymbol {
            component_id: component.id,
            reason: format!(
                "no authored or generated symbol is retained for {}/{}",
                binding.library, binding.cell
            ),
        }
    })?;
    if resolved
        .issues()
        .iter()
        .any(|issue| issue.kind == ResolvedSymbolIssueKind::InvalidMetadata)
    {
        return Err(HardcopySourceError::InvalidAuthoredSymbol(component.id));
    }
    let source = match resolved.source() {
        ResolvedSymbolSource::Authored => SemanticSymbolSource::Authored,
        ResolvedSymbolSource::Generated => SemanticSymbolSource::Generated,
    };
    let mut printable_document = resolved.document().clone();
    // The reconciled pin contract, not orphan metadata, is what the
    // schematic renderer exposes. Freeze exactly that connectable set.
    printable_document.pins = resolved
        .connectable_pins()
        .map(|pin| crate::state::SymbolPin::new(pin.name.clone(), pin.direction, Some(pin.offset)))
        .collect();
    Ok((Some(printable_document), Some(source)))
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

fn finish_resolved(
    identity: HardcopySourceIdentity,
    content_digest: ContentDigest,
    kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    semantic_document: HardcopySemanticDocument,
    bounds: SemanticBounds,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let content_extent = bounds.content_extent()?;
    let default_print_mapping = default_print_mapping(&semantic_document)?;
    let authority = ActiveHardcopySource::try_new(
        identity.document_id,
        identity.revision,
        content_digest,
        identity.display_name,
        kind,
        scope,
    )
    .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?;
    Ok(ResolvedHardcopyDocument {
        source_key: identity.source_key,
        authority,
        semantic_document,
        bounds,
        content_extent,
        default_print_mapping,
    })
}

fn nondegenerate_range(minimum: f64, maximum: f64) -> (f64, f64) {
    if minimum < maximum {
        (minimum, maximum)
    } else {
        let padding = (minimum.abs() * 0.05).max(1.0e-12);
        (minimum - padding, maximum + padding)
    }
}

fn stable_trace_id(dataset_id: DatasetId, analysis_sequence: u64, name: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-studio-trace-id-v1");
    hasher.update(dataset_id.as_uuid().as_bytes());
    hasher.update(analysis_sequence.to_be_bytes());
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes).max(1)
}

fn stable_page_id(page: &str) -> u64 {
    let digest = Sha256::digest([b"rspice-studio-page-id-v1".as_slice(), page.as_bytes()].concat());
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes).max(1)
}

#[derive(Serialize)]
struct StudioWaveformDigestMaterial<'a> {
    name: &'a str,
    color: &'a str,
    visible: bool,
    x_bits: Vec<u64>,
    y_bits: Vec<u64>,
}

#[derive(Serialize)]
struct StudioPaneDigestMaterial<'a> {
    studio_revision: u64,
    pane: &'a StudioPane,
    run_id: RunId,
    analysis_sequence: u64,
    waveforms: Vec<StudioWaveformDigestMaterial<'a>>,
    markers: &'a [&'a StudioMarker],
    annotations: &'a [&'a StudioAnnotation],
}

fn studio_pane_digest(
    studio: &VisualizationStudioState,
    pane: &StudioPane,
    run_id: RunId,
    analysis_sequence: u64,
    waveforms: &[&WaveformData],
    markers: &[&StudioMarker],
    annotations: &[&StudioAnnotation],
) -> Result<ContentDigest, HardcopySourceError> {
    let waveforms = waveforms
        .iter()
        .map(|waveform| StudioWaveformDigestMaterial {
            name: &waveform.name,
            color: &waveform.color,
            visible: waveform.visible,
            x_bits: waveform.x.iter().map(|value| value.to_bits()).collect(),
            y_bits: waveform.y.iter().map(|value| value.to_bits()).collect(),
        })
        .collect();
    canonical_digest(
        b"rspice-hardcopy-studio-pane-v1",
        &StudioPaneDigestMaterial {
            studio_revision: studio.revision,
            pane,
            run_id,
            analysis_sequence,
            waveforms,
            markers,
            annotations,
        },
    )
}

pub(super) fn default_print_mapping(
    document: &HardcopySemanticDocument,
) -> Result<PrintMappingTable, HardcopySourceError> {
    let mut entries = Vec::new();
    match document {
        HardcopySemanticDocument::Schematic(schematic) => {
            if let Some(format) = &schematic.drawing_sheet {
                entries.push(layer_mapping(
                    "layer:drawing-sheet-paper",
                    "Drawing sheet paper",
                    "authored paper edge and printable boundary",
                )?);
                if format.border != DrawingSheetBorderTemplate::None
                    || format.marks.registration
                    || format.marks.folding
                {
                    entries.push(layer_mapping(
                        "layer:drawing-sheet-frame",
                        "Drawing sheet frame",
                        "authored border, zones, and marks",
                    )?);
                }
                if format.title_block.template != DrawingSheetTitleBlockTemplate::None {
                    entries.push(layer_mapping(
                        "layer:drawing-sheet-title-block",
                        "Drawing sheet title block",
                        "authored title block fields",
                    )?);
                }
                entries.push(layer_mapping(
                    "layer:schematic-grid",
                    "Schematic grid",
                    "authored snap-grid pitch · output inclusion optional",
                )?);
            }
            if !schematic.components.is_empty() {
                entries.push(layer_mapping(
                    "layer:schematic-components",
                    "Components and symbols",
                    "schematic component color · solid",
                )?);
            }
            if !schematic.wires.is_empty()
                || !schematic.junctions.is_empty()
                || !schematic.net_labels.is_empty()
            {
                entries.push(layer_mapping(
                    "layer:schematic-wiring",
                    "Wires and junctions",
                    "schematic wire color · solid",
                )?);
            }
            if !schematic.buses.is_empty() || !schematic.bus_taps.is_empty() {
                entries.push(layer_mapping(
                    "layer:schematic-buses",
                    "Buses and taps",
                    "schematic bus color · heavy solid",
                )?);
            }
            if !schematic.design_notes.is_empty() {
                entries.push(layer_mapping(
                    "layer:drawing-annotation",
                    "Drawing annotations",
                    "drawing / annotation text",
                )?);
            }
            if !schematic.documentation_shapes.is_empty() {
                entries.push(layer_mapping(
                    "layer:drawing-documentation",
                    "Documentation geometry",
                    "drawing / documentation stroke",
                )?);
            }

            let mut nets = std::collections::BTreeMap::new();
            for label in &schematic.net_labels {
                nets.entry(label.name.as_str())
                    .and_modify(|id: &mut u64| *id = (*id).min(label.id))
                    .or_insert(label.id);
            }
            for (net, stable_id) in nets {
                entries.push(mapping_entry(
                    PrintObjectKind::Net,
                    format!("net:{stable_id}"),
                    net,
                    "schematic wire color · solid",
                    PrintColor::Black,
                    PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(250),
                    },
                    true,
                )?);
            }
            let mut notes = schematic.design_notes.iter().collect::<Vec<_>>();
            notes.sort_by_key(|note| note.id);
            for note in notes {
                entries.push(mapping_entry(
                    PrintObjectKind::ReviewAnnotation,
                    format!("schematic-note:{}", note.id),
                    format!("{} {}", note.kind.label(), note.id),
                    note.layer.label(),
                    PrintColor::Black,
                    PrintRedundancy::DottedLeader {
                        width: Length::from_micrometres(200),
                        spacing: Length::from_micrometres(1_250),
                    },
                    false,
                )?);
            }
        }
        HardcopySemanticDocument::Symbol(symbol) => {
            if !symbol.body.is_empty() {
                entries.push(layer_mapping(
                    "layer:symbol-body",
                    "Symbol body",
                    "symbol graphic color · solid",
                )?);
            }
            if !symbol.pins.is_empty() {
                entries.push(layer_mapping(
                    "layer:symbol-pins",
                    "Symbol pins",
                    "terminal color · solid",
                )?);
            }
        }
        HardcopySemanticDocument::Plot(plot) => {
            const SCREEN_STYLES: [&str; 8] = [
                "cyan · solid",
                "amber · solid",
                "green · solid",
                "violet · solid",
                "yellow · solid",
                "blue · solid",
                "orange · solid",
                "gray · solid",
            ];
            for (index, trace) in plot.traces.iter().enumerate() {
                let redundancy = match index % 3 {
                    0 => PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(300),
                    },
                    1 => PrintRedundancy::DashedLine {
                        width: Length::from_micrometres(300),
                        dash: Length::from_micrometres(2_000),
                        gap: Length::from_micrometres(1_000),
                    },
                    _ => PrintRedundancy::DottedLeader {
                        width: Length::from_micrometres(300),
                        spacing: Length::from_micrometres(1_250),
                    },
                };
                entries.push(mapping_entry(
                    PrintObjectKind::Trace,
                    format!("trace:{}", trace.trace_id),
                    trace.label.clone(),
                    SCREEN_STYLES[index % SCREEN_STYLES.len()],
                    PrintColor::Black,
                    redundancy,
                    true,
                )?);
            }
            for cursor in &plot.cursors {
                entries.push(mapping_entry(
                    PrintObjectKind::Marker,
                    format!("cursor:{}", cursor.cursor_id),
                    format!("Cursor {}", cursor.label),
                    "viewer cursor color · dashed line",
                    PrintColor::Black,
                    PrintRedundancy::DashedLine {
                        width: Length::from_micrometres(200),
                        dash: Length::from_micrometres(1_250),
                        gap: Length::from_micrometres(750),
                    },
                    true,
                )?);
            }
            for marker in &plot.markers {
                entries.push(mapping_entry(
                    PrintObjectKind::Marker,
                    format!("marker:{}", marker.marker_id),
                    marker.label.clone(),
                    "viewer marker color · triangle",
                    PrintColor::Black,
                    PrintRedundancy::TriangleWithId {
                        size: Length::from_micrometres(2_500),
                    },
                    true,
                )?);
            }
            for annotation in &plot.annotations {
                entries.push(mapping_entry(
                    PrintObjectKind::ReviewAnnotation,
                    format!("annotation:{}", annotation.annotation_id),
                    compact_display(&annotation.text, "Plot annotation"),
                    "viewer annotation color · leader",
                    PrintColor::Black,
                    PrintRedundancy::DottedLeader {
                        width: Length::from_micrometres(200),
                        spacing: Length::from_micrometres(1_250),
                    },
                    false,
                )?);
            }
        }
        HardcopySemanticDocument::ResultSummary(summary) => {
            entries.push(layer_mapping(
                format!(
                    "layer:result-summary:{}",
                    summary.viewer.label().to_ascii_lowercase()
                ),
                format!("{} result summary", summary.viewer.label()),
                "result table and semantic diagram styles",
            )?);
        }
        HardcopySemanticDocument::Report(report) => {
            entries.push(layer_mapping(
                "layer:report-content",
                "Report content",
                "report template styles",
            )?);
            for page in &report.pages {
                for section in page.sections() {
                    for block in section.blocks() {
                        if let ReportBlockKind::ReviewNote(note) = block.kind() {
                            entries.push(mapping_entry(
                                PrintObjectKind::ReviewAnnotation,
                                format!("report-review:{}", block.id()),
                                compact_display(&note.message, "Report review note"),
                                "report review note · leader",
                                PrintColor::Black,
                                PrintRedundancy::DottedLeader {
                                    width: Length::from_micrometres(200),
                                    spacing: Length::from_micrometres(1_250),
                                },
                                false,
                            )?);
                        }
                    }
                }
            }
        }
        HardcopySemanticDocument::Aggregate(aggregate) => {
            for child in &aggregate.children {
                let child_mapping = default_print_mapping(&child.document)?;
                for entry in child_mapping.entries() {
                    let object = entry.object();
                    let stable_digest = Sha256::digest(
                        [
                            b"rspice-aggregate-print-object-v1:".as_slice(),
                            &child.ordinal.to_be_bytes(),
                            object.stable_id().as_bytes(),
                        ]
                        .concat(),
                    );
                    let stable_suffix = stable_digest[..12]
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect::<String>();
                    entries.push(mapping_entry(
                        object.kind(),
                        format!("aggregate:{}:{stable_suffix}", child.ordinal),
                        compact_display(
                            &format!("{} · {}", child.display_name, object.display_name()),
                            "Aggregate object",
                        ),
                        object.screen_style(),
                        entry.print_color(),
                        entry.redundancy(),
                        entry.include_in_legend(),
                    )?);
                }
            }
        }
    }
    PrintMappingTable::try_new(PrintMappingSaveScope::Document, entries)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
}

#[cfg(test)]
pub(crate) fn resolve_retained_hardcopy_source(
    state: &AppState,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let descriptors = enumerate_retained_hardcopy_sources(state);
    let descriptor = descriptors
        .iter()
        .find(|descriptor| descriptor.source_key == source_key)
        .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
    if let RetainedHardcopySourceAvailability::Unavailable { reason } = &descriptor.availability {
        return Err(HardcopySourceError::UnavailableRetainedSource {
            source_key: source_key.to_owned(),
            reason: reason.clone(),
        });
    }
    if !descriptor.supports_scope(&scope) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    if let Some(source_set) = state.workspace.hardcopy_source_set(source_key) {
        return resolve_retained_hardcopy_source_set(state, source_set);
    }

    let project_id = state.workspace.project.id();
    let design_key = format!(
        "project:{}:cell-view:{}",
        project_id.as_uuid(),
        state.workspace.active_key()
    );
    if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        let active_key = state.workspace.active_key();
        if let Some(catalog) = state.workspace.design_management.sheet_catalog(&active_key)
            && let Some(sheet) = catalog
                .sheets()
                .iter()
                .find(|sheet| format!("{design_key}:sheet:{}", sheet.id()) == source_key)
        {
            let base_identity = active_cell_view_identity(state)?;
            let resolver =
                SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
            let identity = schematic_sheet_identity(&base_identity, sheet)?;
            return resolve_schematic_source(SchematicHardcopySource {
                identity,
                schematic: &state.schematic,
                expected_topology_version: state.schematic.topology_version(),
                symbol_resolver: Some(&resolver),
                sheet_catalog: Some(catalog),
                sheet_id: Some(sheet.id()),
                project_default_drawing_sheet: Some(
                    &state
                        .workspace
                        .design_management
                        .drawing_sheet_settings()
                        .default_format,
                ),
                project_title_block_field_values: Some(
                    &state
                        .workspace
                        .design_management
                        .drawing_sheet_settings()
                        .title_block_field_values,
                ),
                scope,
            });
        }
    }
    if source_key == design_key {
        let identity = active_cell_view_identity(state)?;
        return match state.workspace.active_view_type() {
            ViewType::Schematic | ViewType::Testbench => {
                let resolver =
                    SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
                if matches!(scope, HardcopyScope::AllSheetsOrPanes) {
                    let active_key = state.workspace.active_key();
                    let sheet_catalog = state
                        .workspace
                        .design_management
                        .sheet_catalog(&active_key)
                        .ok_or_else(|| {
                            HardcopySourceError::InvalidSheetPartition(
                                "all-sheets scope has no governed sheet catalog".to_owned(),
                            )
                        })?;
                    return resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
                        identity,
                        schematic: &state.schematic,
                        expected_topology_version: state.schematic.topology_version(),
                        symbol_resolver: Some(&resolver),
                        sheet_catalog,
                        project_default_drawing_sheet: &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .default_format,
                        project_title_block_field_values: &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .title_block_field_values,
                    });
                }
                let active_key = state.workspace.active_key();
                let sheet_catalog = matches!(scope, HardcopyScope::CurrentSheet)
                    .then(|| state.workspace.design_management.sheet_catalog(&active_key))
                    .flatten();
                let sheet_id = sheet_catalog.and_then(SheetCatalog::active_sheet_id);
                let identity = if let (Some(catalog), Some(sheet_id)) = (sheet_catalog, sheet_id) {
                    let sheet = catalog.find(sheet_id).ok_or_else(|| {
                        HardcopySourceError::InvalidSheetPartition(format!(
                            "active sheet {sheet_id} is not retained"
                        ))
                    })?;

                    schematic_sheet_identity(&identity, sheet)?
                } else {
                    identity
                };
                resolve_schematic_source(SchematicHardcopySource {
                    identity,
                    schematic: &state.schematic,
                    expected_topology_version: state.schematic.topology_version(),
                    symbol_resolver: Some(&resolver),
                    sheet_catalog,
                    sheet_id,
                    project_default_drawing_sheet: matches!(
                        scope,
                        HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
                    )
                    .then_some(
                        &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .default_format,
                    ),
                    project_title_block_field_values: Some(
                        &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .title_block_field_values,
                    ),
                    scope,
                })
            }
            ViewType::Symbol => {
                let document = state
                    .load_active_symbol_document()
                    .map_err(HardcopySourceError::StaleActiveDocumentAuthority)?;
                resolve_symbol_source(SymbolHardcopySource {
                    identity,
                    document: &document,
                    selection: None,
                    scope,
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
        return resolve_visualization_document_source(
            source_key.to_owned(),
            project_id,
            document,
            page.id,
            pane.id,
            matches!(scope, HardcopyScope::AllSheetsOrPanes),
            scope,
        );
    }

    if let Some(run) = state.simulation.active_run() {
        let results_key = format!(
            "project:{}:result-dataset:{}",
            project_id.as_uuid(),
            run.dataset_id
        );
        if source_key == results_key {
            require_active_result_document(state, run.dataset_id)?;
            return resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
                source_key: source_key.to_owned(),
                project_id,
                state,
                scope,
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
        if matches!(scope, HardcopyScope::AllSheetsOrPanes) {
            let mut resolved = resolve_all_studio_panes(
                project_id,
                &state.workbench.visualization_studio,
                &state.simulation,
            )?;
            // The transient all-panes definition is addressed through the
            // selected retained pane descriptor, so commit-time revalidation
            // must keep that stable dialog key.
            resolved.source_key = source_key.to_owned();
            return Ok(resolved);
        }
        let pane_id = pane.id;
        return resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
            source_key: source_key.to_owned(),
            project_id,
            studio: &state.workbench.visualization_studio,
            simulation: &state.simulation,
            pane_id,
            scope,
        });
    }

    if let Some(document_id) = state.workbench.report_authoring.selected_document {
        let report_key = format!("project:{}:report:{}", project_id.as_uuid(), document_id);
        if source_key == report_key {
            let document = state
                .workspace
                .report_documents
                .iter()
                .find(|document| document.id() == document_id)
                .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
            return report_inventory::resolve(state, document, scope);
        }
    }

    Err(HardcopySourceError::SourceNotRetained(
        source_key.to_owned(),
    ))
}

#[cfg(test)]
pub(crate) fn resolve_retained_hardcopy_source_set(
    state: &AppState,
    source_set: &HardcopySourceSet,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_hardcopy_source_set_with(source_set, |member| {
        resolve_retained_hardcopy_source(state, member.source_key(), member.scope().clone())
    })
}

#[cfg(test)]
mod tests;
