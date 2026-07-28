//! Exact, semantic hardcopy source resolution.
//!
//! Hardcopy starts here rather than at a viewport or GPU surface.  Every
//! adapter freezes one durable document revision, resolves authored symbol
//! artwork and retained result samples, computes deterministic physical
//! bounds, and authenticates the semantic snapshot before rendering begins.
//! No type in this module contains pixels, an egui paint command, or a screen
//! rectangle.

mod documents;
mod geometry;
mod prepared;
mod results;
mod semantic;

pub use documents::*;
pub use geometry::*;
pub use prepared::*;
pub use results::*;
pub use semantic::*;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::io::ProjectSimulationResults;
use crate::product::{ContentDigest, DatasetId, ObjectRevision, ProjectId, RunId};
use crate::results::report_document::{
    FigureSizing, FrozenReportArtifact, ReportBlockId, ReportBlockKind, ReportDocument, ReportPage,
    ReportReferenceCurrentness, ReportReferenceInventory, ReportReferenceMode,
    ReportReferenceSnapshot,
};
use crate::results::visualization_document::{
    AnnotationAnchor, PageId, PaneId, TypedValue, VisualizationDocument,
};
use crate::results::visualization_raster::{
    ResolvedCartesianLineScene, VisualizationRasterError, resolve_cartesian_line_scene,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, Bus, BusTap, Component,
    ComponentType, DesignNote, DesignSheet, DocumentationShape, Junction, NetLabel, Point,
    ResolvedSymbolIssueKind, ResolvedSymbolSource, SchematicState, Selection, SheetCatalog,
    SheetId, SimulationRun, SimulationState, SymbolDocument, SymbolResolver, SymbolShape, ViewType,
    WaveformData, Wire,
};
use crate::workbench::AppState;

use crate::hardcopy::{
    ActiveHardcopySource, ContentExtent, HardcopyContentSection, HardcopyDocumentId,
    HardcopyDocumentKind, HardcopyScope, Length, PrintColor, PrintMappingEntry,
    PrintMappingSaveScope, PrintMappingTable, PrintObjectIdentity, PrintObjectKind,
    PrintRedundancy,
};
use crate::workbench::SurfaceId;
// The persisted source-set records and the validation they share with these
// adapters are owned one layer down, where `state` can reach them.
use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, HardcopySourceError, HardcopySourceIdentity, HardcopySourceSet,
    HardcopySourceSetMember, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT, canonical_digest,
    validate_label,
};
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::lifecycle::session::SymbolSelection;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};
use crate::workbench::documents::visualization_studio::{
    VisualizationAnnotation as StudioAnnotation, VisualizationAutoscale,
    VisualizationMarker as StudioMarker, VisualizationPane as StudioPane, VisualizationStudioState,
};

/// Natural physical scale for schematic coordinates: ten editor units are
/// one tenth of an inch.  Page fitting can subsequently scale this scene, but
/// this fixed calibration makes 1:1 hardcopy deterministic on every target.
pub const SCHEMATIC_UNIT_UM: i64 = 254;
/// Natural active-plot canvas (10 by 5.625 inches, 16:9).
pub const PLOT_WIDTH_UM: i64 = 254_000;
pub const PLOT_HEIGHT_UM: i64 = 142_875;
/// Natural report page used to arrange the report's already-authored pages.
pub const REPORT_PAGE_WIDTH_UM: i64 = 215_900;
pub const REPORT_PAGE_HEIGHT_UM: i64 = 279_400;
pub const REPORT_PAGE_GAP_UM: i64 = 5_000;
pub const BLANK_SCHEMATIC_SHEET_WIDTH_UM: i64 = 279_400;
pub const BLANK_SCHEMATIC_SHEET_HEIGHT_UM: i64 = 215_900;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) const MAX_WORKER_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
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
    pub scope: HardcopyScope,
}

pub struct SchematicSheetSetHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub schematic: &'a SchematicState,
    pub expected_topology_version: u64,
    pub symbol_resolver: Option<&'a SymbolResolver<'a>>,
    pub sheet_catalog: &'a SheetCatalog,
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
pub(crate) struct ResultsQuickViewHardcopySource<'a> {
    pub source_key: String,
    pub project_id: ProjectId,
    pub state: &'a AppState,
    pub scope: HardcopyScope,
}

#[derive(Debug, Clone)]
struct ResultsQuickViewPresentation {
    viewer: ResultViewer,
    fft: crate::analysis::FftState,
    histogram_selected: usize,
    histogram_bin_count: usize,
    histogram_custom_range: bool,
    histogram_custom_min: f64,
    histogram_custom_max: f64,
    histogram_mode: crate::analysis::histogram::HistogramDisplayMode,
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
        Self {
            viewer: state.ui.results.viewer,
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

pub enum HardcopySourceCandidate<'a> {
    Schematic(SchematicHardcopySource<'a>),
    Symbol(SymbolHardcopySource<'a>),
    VisualizationPane(VisualizationPaneHardcopySource<'a>),
    Plot(PlotHardcopySource<'a>),
    Report(ReportHardcopySource<'a>),
    Unsupported { source_key: String, reason: String },
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

    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Available => None,
            Self::Unavailable { reason } => Some(reason),
        }
    }
}
impl HardcopySourceCandidate<'_> {
    fn source_key(&self) -> &str {
        match self {
            Self::Schematic(source) => &source.identity.source_key,
            Self::Symbol(source) => &source.identity.source_key,
            Self::VisualizationPane(source) => &source.source_key,
            Self::Plot(source) => &source.source_key,
            Self::Report(source) => &source.source_key,
            Self::Unsupported { source_key, .. } => source_key,
        }
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

    if let Some(run) = state.simulation.active_run()
        && matches!(
            state.workbench.documents.active(Workspace::Results),
            Some(WorkspaceDocumentId::ResultDataset(dataset)) if *dataset == run.dataset_id
        )
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
            availability: report_app_availability(document),
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
                    .map_err(|reason| HardcopySourceError::StaleActiveDocumentAuthority(reason))?;
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

    if let Some(run) = state.simulation.active_run() {
        let result_key = format!(
            "project:{}:result-dataset:{}",
            project_id.as_uuid(),
            run.dataset_id
        );
        if source_key == result_key {
            require_active_result_document(state, run.dataset_id)?;
            let analysis_index = state.simulation.active_analysis_idx.ok_or_else(|| {
                HardcopySourceError::UnretainedResult(
                    "no active analysis is selected in the active terminal dataset".to_owned(),
                )
            })?;
            let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
                HardcopySourceError::UnretainedResult(format!(
                    "active analysis index {analysis_index} is not retained"
                ))
            })?;
            let mut run = run.clone();
            run.analyses = vec![analysis.clone()];
            return Ok(PreparedRetainedHardcopyResolution {
                payload: PreparedRetainedHardcopyPayload::Results {
                    source_key: source_key.to_owned(),
                    project_id,
                    run,
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
            return Ok(PreparedRetainedHardcopyResolution {
                payload: PreparedRetainedHardcopyPayload::Report {
                    project_id,
                    source_key: source_key.to_owned(),
                    document: document.clone(),
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
    let mut prepared = SimulationState::default();
    prepared.runs = simulation
        .runs
        .iter()
        .filter(|run| dataset_ids.contains(&run.dataset_id))
        .cloned()
        .map(|mut run| {
            run.analyses
                .retain(|analysis| analysis_ids.contains(&(run.dataset_id, analysis.id)));
            run
        })
        .collect();
    prepared
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
        SurfaceId::Results => state.simulation.active_run().is_some_and(|run| {
            matches!(
                state.workbench.documents.active(Workspace::Results),
                Some(WorkspaceDocumentId::ResultDataset(dataset)) if *dataset == run.dataset_id
            ) && quick_result_availability(state, run).is_available()
        }),
        SurfaceId::VisualizationStudio => state
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
            .is_some_and(|pane| studio_pane_availability(state, pane).is_available()),
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

/// Strict dialog selection resolver. The selected stable key and exact scope
/// must both be advertised by the retained descriptor; no active/background
/// fallback is attempted.

/// Resolve a persisted source set against the currently retained application
/// authorities. Members are processed in definition order and the operation
/// returns no partial aggregate if any member is missing, stale, or invalid.

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
    let Some(index) = state.simulation.active_analysis_idx else {
        return unavailable("no active analysis is selected".to_owned());
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
    let viewer = state.ui.results.viewer;
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
        ResultViewer::Waves | ResultViewer::Bode => has_waveform(),
        ResultViewer::Fft => visible_waveforms().any(|waveform| {
            waveform.x.len() >= crate::analysis::fft::MIN_FFT_SAMPLES
                && waveform.x.len() == waveform.y.len()
        }),
        ResultViewer::Eye => visible_waveforms()
            .any(|waveform| waveform.x.len() >= 8 && waveform.x.len() == waveform.y.len()),
        ResultViewer::Hist => matches!(
            analysis.family_metadata.as_ref(),
            Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. })
                if variables
                    .get(state.analysis.histogram_state.selected)
                    .is_some_and(|variable| !variable.samples.is_empty())
        ),
        ResultViewer::Nyquist | ResultViewer::Smith => visible_waveforms().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty() && complex.real.len() == complex.imag.len()
            })
        }),
        ResultViewer::Op => {
            analysis.dc_op.is_some()
                || matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::OperatingPoint { .. })
                )
        }
        ResultViewer::NoiseContrib => analysis.noise_summary.is_some(),
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
        // The sample table needs nothing but retained samples.
        ResultViewer::Table => has_waveform(),
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

fn schematic_has_objects_on_sheet(
    schematic: &SchematicState,
    catalog: &SheetCatalog,
    sheet_id: SheetId,
) -> bool {
    let belongs = |object_id| {
        catalog
            .sheet_for_object(object_id)
            .or(catalog.active_sheet_id())
            == Some(sheet_id)
    };
    schematic.components.iter().any(|object| belongs(object.id))
        || schematic.wires.iter().any(|object| belongs(object.id))
        || schematic.buses.iter().any(|object| belongs(object.id))
        || schematic.bus_taps.iter().any(|object| belongs(object.id))
        || schematic.junctions.iter().any(|object| belongs(object.id))
        || schematic.net_labels.iter().any(|object| belongs(object.id))
        || schematic
            .design_notes
            .iter()
            .any(|object| belongs(object.id))
        || schematic
            .documentation_shapes
            .iter()
            .any(|object| belongs(object.id))
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
    RetainedHardcopySourceAvailability::Available
}

fn report_app_availability(document: &ReportDocument) -> RetainedHardcopySourceAvailability {
    if document.pages().is_empty() {
        return RetainedHardcopySourceAvailability::Unavailable {
            reason: "report has no authored pages".to_owned(),
        };
    }
    for block in document
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
    {
        if matches!(
            block.kind().reference(),
            Some(ReportReferenceMode::Linked { .. })
        ) {
            return RetainedHardcopySourceAvailability::Unavailable {
                reason: format!(
                    "linked report block {} has no retained reference inventory",
                    block.id()
                ),
            };
        }
        if let ReportBlockKind::PlotFigure(figure) = block.kind()
            && figure
                .reference
                .frozen_artifact()
                .is_none_or(|artifact| artifact.media_type() != "image/png")
        {
            return RetainedHardcopySourceAvailability::Unavailable {
                reason: format!(
                    "frozen plot block {} has no supported opaque PNG artifact",
                    block.id()
                ),
            };
        }
    }
    RetainedHardcopySourceAvailability::Available
}

/// Resolve exactly one candidate matching the application-owned active key.
/// Missing, duplicated, and explicitly unsupported authorities are rejected;
/// this prevents a background tab or stale retained result from being printed
/// merely because it happens to be available.
pub fn resolve_active_hardcopy_source<'a>(
    active_source_key: &str,
    candidates: impl IntoIterator<Item = HardcopySourceCandidate<'a>>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("active source key", active_source_key, SOURCE_KEY_LIMIT)?;
    let mut matching = candidates
        .into_iter()
        .filter(|candidate| candidate.source_key() == active_source_key);
    let candidate = matching
        .next()
        .ok_or_else(|| HardcopySourceError::SourceNotRetained(active_source_key.to_owned()))?;
    if matching.next().is_some() {
        return Err(HardcopySourceError::AmbiguousActiveSource(
            active_source_key.to_owned(),
        ));
    }
    match candidate {
        HardcopySourceCandidate::Schematic(source) => resolve_schematic_source(source),
        HardcopySourceCandidate::Symbol(source) => resolve_symbol_source(source),
        HardcopySourceCandidate::VisualizationPane(source) => {
            resolve_visualization_pane_source(source)
        }
        HardcopySourceCandidate::Plot(source) => resolve_plot_source(source),
        HardcopySourceCandidate::Report(source) => resolve_report_source(source),
        HardcopySourceCandidate::Unsupported { reason, .. } => {
            Err(HardcopySourceError::UnsupportedDocument(reason))
        }
    }
}

/// Resolve the one application document that owns the current route.
///
/// This is the sole AppState integration boundary for File > Print and page
/// preview. Every branch verifies the stable open-document selection before
/// borrowing engineering content; background buffers and most-recent results
/// are never substituted for an absent or stale active authority.
#[allow(dead_code)] // Retained as the fail-closed single-route compatibility boundary.
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
        SurfaceId::Results => {
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
            resolve_report_source(ReportHardcopySource {
                source_key: format!("project:{}:report:{}", project_id.as_uuid(), document_id),
                document,
                reference_inventory: None,
                scope: HardcopyScope::CompleteReport,
            })
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
    )
}

fn schematic_sheet_identity(
    base: &HardcopySourceIdentity,
    sheet: &DesignSheet,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_material = b"rspice-hardcopy-schematic-sheet-v1:".to_vec();
    identity_material.extend_from_slice(sheet.id().as_uuid().as_bytes());
    HardcopySourceIdentity::try_new(
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
    )
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
            if !schematic_has_objects_on_sheet(&state.schematic, catalog, sheet.id()) {
                return resolve_blank_schematic_sheet(identity, scope);
            }
            return resolve_schematic_source(SchematicHardcopySource {
                identity,
                schematic: &state.schematic,
                expected_topology_version: state.schematic.topology_version(),
                symbol_resolver: Some(&resolver),
                sheet_catalog: Some(catalog),
                sheet_id: Some(sheet.id()),
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
                    let sheet_identity = schematic_sheet_identity(&identity, sheet)?;
                    if !schematic_has_objects_on_sheet(&state.schematic, catalog, sheet_id) {
                        return resolve_blank_schematic_sheet(sheet_identity, scope);
                    }
                    sheet_identity
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
                    scope,
                })
            }
            ViewType::Symbol => {
                let document = state
                    .load_active_symbol_document()
                    .map_err(|reason| HardcopySourceError::StaleActiveDocumentAuthority(reason))?;
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
            return resolve_report_source(ReportHardcopySource {
                source_key: source_key.to_owned(),
                document,
                reference_inventory: None,
                scope,
            });
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
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::product::{DatasetBinding, DatasetId, ResultDocumentId, VerificationEvidenceId};
    use crate::results::report_document::{
        DataTableBlock, EvidenceBlock, PlotFigureBlock, ReportEdit, ReportEntityRef,
        ReportReferenceInventoryEntry, ReportReferenceSnapshot, ReportSourceId, TableCell,
        TableColumn,
    };
    use crate::results::visualization_document::{
        AxisOrientation, AxisScale, ColumnRole, DocumentEdit, EntityRef, NewAxis, NewTrace,
        SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType, VisualizationDocument,
    };
    use crate::state::{
        AnalysisType, Cell, ComplexResultValue, Library, LibraryCellInstance, LibraryManager,
        MonteCarloVariableMetadata, PortDirection, SheetDefinition, SheetPortPolicy, SheetTemplate,
        SimulationRunLifecycle, View, ViewType,
    };

    fn identity(key: &str) -> HardcopySourceIdentity {
        HardcopySourceIdentity::try_new(
            key,
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            "Active document",
        )
        .unwrap()
    }

    fn quick_view_state(analysis: AnalysisResult, viewer: ResultViewer) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(analysis);
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.ui.results.viewer = viewer;
        state
    }

    fn resolve_quick_view(
        state: &AppState,
    ) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
        resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
            source_key: "results-quick-view".to_owned(),
            project_id: state.workspace.project.id(),
            state,
            scope: HardcopyScope::ActivePlotDocument,
        })
    }

    fn report_with_block(kind: ReportBlockKind) -> ReportDocument {
        let mut report = ReportDocument::new("Authenticated report").unwrap();
        let receipt = report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Results".to_owned(),
                }],
                1,
            )
            .unwrap();
        let ReportEntityRef::Page(page_id) = receipt.created[0] else {
            unreachable!()
        };
        let receipt = report
            .transact(
                report.revision(),
                vec![ReportEdit::AddSection {
                    page_id,
                    title: "Evidence".to_owned(),
                }],
                2,
            )
            .unwrap();
        let ReportEntityRef::Section(section_id) = receipt.created[0] else {
            unreachable!()
        };
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddBlock { section_id, kind }],
                3,
            )
            .unwrap();
        report
    }

    fn opaque_rgb8_png(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, width, height);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer
                .write_image_data(&vec![0x55; width as usize * height as usize * 3])
                .unwrap();
        }
        bytes
    }

    #[test]
    fn resolver_fails_closed_for_missing_and_ambiguous_active_authority() {
        let error = resolve_active_hardcopy_source("doc", Vec::new()).unwrap_err();
        assert!(matches!(error, HardcopySourceError::SourceNotRetained(_)));

        let candidates = vec![
            HardcopySourceCandidate::Unsupported {
                source_key: "doc".to_owned(),
                reason: "one".to_owned(),
            },
            HardcopySourceCandidate::Unsupported {
                source_key: "doc".to_owned(),
                reason: "two".to_owned(),
            },
        ];
        let error = resolve_active_hardcopy_source("doc", candidates).unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceError::AmbiguousActiveSource(_)
        ));
    }

    #[test]
    fn schematic_digest_ignores_viewport_state_but_changes_with_authored_content() {
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(1, Point::new(-10, 0), Point::new(20, 0)));
        let topology = schematic.topology_version();
        let first = resolve_schematic_source(SchematicHardcopySource {
            identity: identity("schematic"),
            schematic: &schematic,
            expected_topology_version: topology,
            symbol_resolver: None,
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap();

        schematic.zoom = 7.5;
        schematic.pan = (123.0, -44.0);
        let second = resolve_schematic_source(SchematicHardcopySource {
            identity: HardcopySourceIdentity {
                document_id: first.authority().document_id(),
                ..identity("schematic")
            },
            schematic: &schematic,
            expected_topology_version: topology,
            symbol_resolver: None,
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap();
        assert_eq!(
            first.authority().content_digest(),
            second.authority().content_digest()
        );

        schematic.wires[0].points[1].x += 1;
        let third = resolve_schematic_source(SchematicHardcopySource {
            identity: HardcopySourceIdentity {
                document_id: first.authority().document_id(),
                ..identity("schematic")
            },
            schematic: &schematic,
            expected_topology_version: topology,
            symbol_resolver: None,
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap();
        assert_ne!(
            first.authority().content_digest(),
            third.authority().content_digest()
        );
    }

    fn sheet_definition(name: &str) -> SheetDefinition {
        SheetDefinition {
            name: name.to_owned(),
            template: SheetTemplate::AnalogSchematic,
            port_policy: SheetPortPolicy::TypedOffSheetPorts,
            explicit_page_number: None,
        }
    }

    #[test]
    fn governed_current_sheet_never_leaks_and_all_sheets_preserve_catalog_order() {
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
        schematic
            .wires
            .push(Wire::segment(22, Point::new(100, 0), Point::new(120, 0)));
        let mut catalog = SheetCatalog::default();
        let first_id = catalog
            .create_sheet(sheet_definition("Input"), None)
            .unwrap();
        let second_id = catalog
            .create_sheet(sheet_definition("Output"), Some(first_id))
            .unwrap();
        let empty_id = catalog
            .create_sheet(sheet_definition("Reserved"), Some(second_id))
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), first_id, [11])
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), second_id, [22])
            .unwrap();

        let base_identity = identity("governed-schematic");
        let second = resolve_schematic_source(SchematicHardcopySource {
            identity: schematic_sheet_identity(&base_identity, catalog.find(second_id).unwrap())
                .unwrap(),
            schematic: &schematic,
            expected_topology_version: schematic.topology_version(),
            symbol_resolver: None,
            sheet_catalog: Some(&catalog),
            sheet_id: Some(second_id),
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap();
        let HardcopySemanticDocument::Schematic(second_semantic) = second.semantic_document()
        else {
            panic!("expected schematic")
        };
        assert_eq!(
            second_semantic
                .wires
                .iter()
                .map(|wire| wire.id)
                .collect::<Vec<_>>(),
            [22]
        );

        let all = resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
            identity: base_identity,
            schematic: &schematic,
            expected_topology_version: schematic.topology_version(),
            symbol_resolver: None,
            sheet_catalog: &catalog,
        })
        .unwrap();
        let HardcopySemanticDocument::Aggregate(aggregate) = all.semantic_document() else {
            panic!("expected aggregate")
        };
        assert_eq!(aggregate.children.len(), 3);
        assert_eq!(
            aggregate
                .children
                .iter()
                .map(|child| child.source_key.clone())
                .collect::<Vec<_>>(),
            [
                format!("governed-schematic:sheet:{first_id}"),
                format!("governed-schematic:sheet:{second_id}"),
                format!("governed-schematic:sheet:{empty_id}"),
            ]
        );
        for (index, expected_wire) in [Some(11), Some(22), None].into_iter().enumerate() {
            let HardcopySemanticDocument::Schematic(sheet) =
                aggregate.children[index].document.as_ref()
            else {
                panic!("expected schematic child")
            };
            assert_eq!(
                sheet.wires.first().map(|wire| wire.id),
                expected_wire,
                "sheet {index} must contain only its own assigned wire"
            );
            assert_eq!(aggregate.children[index].page_break_before, index != 0);
        }
        assert_eq!(
            aggregate.children[2]
                .local_bounds
                .content_extent()
                .unwrap()
                .width()
                .micrometres(),
            BLANK_SCHEMATIC_SHEET_WIDTH_UM as u64
        );
        assert_eq!(all.hardcopy_sections().unwrap().len(), 3);

        let worker_bytes = all.to_worker_snapshot_json().unwrap();
        let round_trip =
            ResolvedHardcopyDocument::from_worker_snapshot_json(&worker_bytes).unwrap();
        assert_eq!(round_trip, all);

        let mut tampered: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
        tampered["source_key"] = serde_json::Value::String("tampered-source".to_owned());
        assert!(matches!(
            ResolvedHardcopyDocument::from_worker_snapshot_json(
                &serde_json::to_vec(&tampered).unwrap()
            ),
            Err(HardcopySourceError::InvalidWorkerSnapshot(_))
        ));
        let mut unknown: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
        unknown["unexpected"] = serde_json::Value::Bool(true);
        assert!(matches!(
            ResolvedHardcopyDocument::from_worker_snapshot_json(
                &serde_json::to_vec(&unknown).unwrap()
            ),
            Err(HardcopySourceError::InvalidWorkerSnapshot(_))
        ));
    }

    #[test]
    fn source_sets_authenticate_definition_and_every_member_atomically() {
        let first =
            resolve_blank_schematic_sheet(identity("sheet-one"), HardcopyScope::CurrentSheet)
                .unwrap();
        let second =
            resolve_blank_schematic_sheet(identity("sheet-two"), HardcopyScope::CurrentSheet)
                .unwrap();
        let members = [&first, &second]
            .into_iter()
            .map(HardcopySourceSetMember::from_resolved)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let source_set = HardcopySourceSet::try_new(
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            "Release set",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet("Release set".to_owned()),
            members,
        )
        .unwrap();
        source_set.validate().unwrap();

        let stale_second = resolve_blank_schematic_sheet(
            HardcopySourceIdentity {
                display_name: "Changed sheet".to_owned(),
                ..identity("sheet-two")
            },
            HardcopyScope::CurrentSheet,
        )
        .unwrap();
        let mut candidates = vec![first, stale_second].into_iter();
        let error =
            resolve_hardcopy_source_set_with(&source_set, |_| Ok(candidates.next().unwrap()))
                .unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceError::StaleSourceSetMember { ref source_key }
                if source_key == "sheet-two"
        ));

        let mut tampered = source_set.clone();
        tampered.reverse_members_for_test();
        assert!(matches!(
            tampered.validate(),
            Err(HardcopySourceError::SourceSetDigestMismatch { .. })
        ));
    }

    #[test]
    fn schematic_selection_exports_only_selected_durable_objects() {
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
        schematic
            .wires
            .push(Wire::segment(2, Point::new(100, 0), Point::new(110, 0)));
        schematic.selection.select_wire(2);
        let resolved = resolve_schematic_source(SchematicHardcopySource {
            identity: identity("schematic"),
            schematic: &schematic,
            expected_topology_version: schematic.topology_version(),
            symbol_resolver: None,
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::Selection,
        })
        .unwrap();
        let HardcopySemanticDocument::Schematic(scene) = resolved.semantic_document() else {
            panic!("expected schematic")
        };
        assert_eq!(
            scene.wires.iter().map(|wire| wire.id).collect::<Vec<_>>(),
            [2]
        );
    }

    #[test]
    fn authored_cell_symbol_is_frozen_into_the_semantic_source() {
        let document = SymbolDocument {
            pins: vec![crate::state::SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(-20, 0)),
            )],
            body: vec![SymbolShape::Circle {
                center: Point::origin(),
                radius: 9,
            }],
            ..SymbolDocument::default()
        };
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document.store_in_view(&mut symbol_view).unwrap();
        let mut cell = Cell::new("amp");
        cell.add_view(symbol_view);
        let mut library = Library::new("work");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);
        let buffers = HashMap::new();
        let resolver = SymbolResolver::new(&libraries, &buffers);

        let mut schematic = SchematicState::default();
        schematic.components.push(
            Component::new(7, ComponentType::CellInstance, Point::new(20, 30))
                .with_library_cell(LibraryCellInstance::new("work", "amp", "symbol")),
        );
        let resolved = resolve_schematic_source(SchematicHardcopySource {
            identity: identity("schematic"),
            schematic: &schematic,
            expected_topology_version: schematic.topology_version(),
            symbol_resolver: Some(&resolver),
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap();
        let HardcopySemanticDocument::Schematic(scene) = resolved.semantic_document() else {
            panic!("expected schematic")
        };
        assert_eq!(
            scene.components[0].resolved_symbol.as_ref(),
            Some(&document)
        );
        assert_eq!(
            scene.components[0].symbol_source,
            Some(SemanticSymbolSource::Authored)
        );
    }

    #[test]
    fn stale_schematic_authority_is_rejected_before_digesting() {
        let schematic = SchematicState::default();
        let error = resolve_schematic_source(SchematicHardcopySource {
            identity: identity("schematic"),
            schematic: &schematic,
            expected_topology_version: schematic.topology_version() + 1,
            symbol_resolver: None,
            sheet_catalog: None,
            sheet_id: None,
            scope: HardcopyScope::CurrentSheet,
        })
        .unwrap_err();
        assert!(matches!(error, HardcopySourceError::StaleSchematic { .. }));
    }

    #[test]
    fn symbol_scene_retains_negative_authored_coordinates_and_deterministic_extent() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Polyline {
                points: vec![Point::new(-20, -10), Point::new(30, 10)],
                closed: false,
            }],
            ..SymbolDocument::default()
        };
        let resolved = resolve_symbol_source(SymbolHardcopySource {
            identity: identity("symbol"),
            document: &document,
            selection: None,
            scope: HardcopyScope::ActiveDocument,
        })
        .unwrap();
        assert!(resolved.bounds().minimum.x_um < 0);
        assert_eq!(
            resolved.content_extent().width().micrometres(),
            70 * SCHEMATIC_UNIT_UM as u64
        );
    }

    #[test]
    fn report_source_uses_authenticated_current_revision_snapshot() {
        let mut report = ReportDocument::new("Release report").unwrap();
        report
            .transact_with_context(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Summary".to_owned(),
                }],
                10,
                "tester",
                "Add summary",
            )
            .unwrap();
        let record = report
            .revision_record(report.id(), report.revision())
            .unwrap();
        let resolved = resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: None,
            scope: HardcopyScope::CompleteReport,
        })
        .unwrap();
        assert_eq!(
            resolved.authority().content_digest(),
            record.snapshot_digest()
        );
        assert_eq!(
            resolved.content_extent().height().micrometres(),
            REPORT_PAGE_HEIGHT_UM as u64
        );
    }

    #[test]
    fn linked_report_table_requires_exact_source_and_dataset_inventory() {
        let dataset_id = DatasetId::new();
        let dataset_digest = ContentDigest::from_bytes([0x31; 32]);
        let binding = DatasetBinding::new(dataset_id, dataset_digest);
        let snapshot = ReportReferenceSnapshot::new(
            ReportSourceId::Dataset { dataset_id },
            None,
            dataset_digest,
            vec![binding],
        )
        .unwrap();
        let report = report_with_block(ReportBlockKind::DataTable(DataTableBlock {
            title: "Exact samples".to_owned(),
            columns: vec![TableColumn {
                key: "value".to_owned(),
                heading: "Value".to_owned(),
                unit: Some("V".to_owned()),
            }],
            rows: vec![vec![TableCell::Number {
                value: 1.25,
                unit: Some("V".to_owned()),
            }]],
            reference: ReportReferenceMode::Linked {
                snapshot: snapshot.clone(),
            },
        }));
        assert!(matches!(
            resolve_report_source(ReportHardcopySource {
                source_key: "report".to_owned(),
                document: &report,
                reference_inventory: None,
                scope: HardcopyScope::CompleteReport,
            }),
            Err(HardcopySourceError::ReportReferenceInventoryRequired)
        ));

        // Application preparation intentionally has no live report inventory:
        // linked blocks remain unavailable rather than silently following a
        // mutable source on the UI or worker thread.
        let mut app = AppState::default();
        let report_id = report.id();
        app.workspace.report_documents.push(report.clone());
        app.workbench.report_authoring.selected_document = Some(report_id);
        let report_key = format!(
            "project:{}:report:{}",
            app.workspace.project.id().as_uuid(),
            report_id
        );
        assert!(matches!(
            prepare_retained_hardcopy_resolution(&app, &report_key, HardcopyScope::CompleteReport),
            Err(HardcopySourceError::UnavailableRetainedSource { .. })
        ));

        let missing_dataset = ReportReferenceInventory {
            sources: vec![
                ReportReferenceInventoryEntry::new(
                    snapshot.source.clone(),
                    snapshot.source_revision,
                    snapshot.content_digest,
                    snapshot.dataset_bindings.clone(),
                )
                .unwrap(),
            ],
            available_datasets: Vec::new(),
        };
        assert!(matches!(
            resolve_report_source(ReportHardcopySource {
                source_key: "report".to_owned(),
                document: &report,
                reference_inventory: Some(&missing_dataset),
                scope: HardcopyScope::CompleteReport,
            }),
            Err(HardcopySourceError::UnauthenticatedReportReference {
                currentness: ReportReferenceCurrentness::DatasetMissing,
                ..
            })
        ));

        let exact_inventory = ReportReferenceInventory {
            sources: missing_dataset.sources.clone(),
            available_datasets: vec![binding],
        };
        let resolved = resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: Some(&exact_inventory),
            scope: HardcopyScope::CompleteReport,
        })
        .unwrap();
        let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
            panic!("expected semantic report")
        };
        assert_eq!(semantic.authenticated_references.len(), 1);
        assert!(matches!(
            semantic.pages[0].sections()[0].blocks()[0].kind(),
            ReportBlockKind::DataTable(_)
        ));
    }

    #[test]
    fn frozen_report_evidence_is_self_contained_and_remains_typed() {
        let digest = ContentDigest::from_bytes([0x44; 32]);
        let snapshot = ReportReferenceSnapshot::new(
            ReportSourceId::VerificationEvidence {
                evidence_id: VerificationEvidenceId::new(),
            },
            Some(ObjectRevision::INITIAL),
            digest,
            Vec::new(),
        )
        .unwrap();
        let report = report_with_block(ReportBlockKind::Evidence(EvidenceBlock {
            title: "Verification receipt".to_owned(),
            summary: "All retained checks passed.".to_owned(),
            reference: ReportReferenceMode::Frozen {
                snapshot,
                artifact: FrozenReportArtifact::new(
                    "application/json",
                    br#"{"status":"pass"}"#.to_vec(),
                )
                .unwrap(),
            },
        }));
        let resolved = resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: None,
            scope: HardcopyScope::CompleteReport,
        })
        .unwrap();
        let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
            panic!("expected semantic report")
        };
        let ReportBlockKind::Evidence(evidence) =
            semantic.pages[0].sections()[0].blocks()[0].kind()
        else {
            panic!("expected typed evidence")
        };
        assert_eq!(evidence.summary, "All retained checks passed.");
        assert_eq!(semantic.authenticated_references.len(), 1);
    }

    #[test]
    fn frozen_png_figure_is_fully_validated_and_retained_semantically() {
        let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x52; 32]));
        let snapshot = ReportReferenceSnapshot::new(
            ReportSourceId::VisualizationDocument {
                document_id: ResultDocumentId::new(),
            },
            Some(ObjectRevision::INITIAL),
            ContentDigest::from_bytes([0x53; 32]),
            vec![binding],
        )
        .unwrap();
        let png = opaque_rgb8_png(128, 128);
        let report = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
            caption: "Authenticated locus".to_owned(),
            alternative_text: "Exact retained visualization.".to_owned(),
            sizing: FigureSizing::FitWidth,
            reference: ReportReferenceMode::Frozen {
                snapshot: snapshot.clone(),
                artifact: FrozenReportArtifact::new("image/png", png.clone()).unwrap(),
            },
        }));
        let resolved = resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: None,
            scope: HardcopyScope::CompleteReport,
        })
        .unwrap();
        let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
            panic!("expected semantic report")
        };
        assert_eq!(semantic.figures.len(), 1);
        assert_eq!(semantic.figures[0].payload, png);
        assert_eq!(
            (
                semantic.figures[0].width_pixels,
                semantic.figures[0].height_pixels
            ),
            (128, 128)
        );
        assert_eq!(semantic.figures[0].caption, "Authenticated locus");

        let linked = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
            caption: "Linked".to_owned(),
            alternative_text: "Identity only".to_owned(),
            sizing: FigureSizing::FitPage,
            reference: ReportReferenceMode::Linked {
                snapshot: snapshot.clone(),
            },
        }));
        let linked_inventory = ReportReferenceInventory {
            sources: vec![
                ReportReferenceInventoryEntry::new(
                    snapshot.source.clone(),
                    snapshot.source_revision,
                    snapshot.content_digest,
                    snapshot.dataset_bindings.clone(),
                )
                .unwrap(),
            ],
            available_datasets: snapshot.dataset_bindings.clone(),
        };
        assert!(matches!(
            resolve_report_source(ReportHardcopySource {
                source_key: "linked-report".to_owned(),
                document: &linked,
                reference_inventory: Some(&linked_inventory),
                scope: HardcopyScope::CompleteReport,
            }),
            Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
                kind: "linked plot figure",
                ..
            })
        ));

        let mut trailing = opaque_rgb8_png(128, 128);
        trailing.extend_from_slice(b"trailing");
        let invalid = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
            caption: "Invalid".to_owned(),
            alternative_text: "Trailing data".to_owned(),
            sizing: FigureSizing::Natural,
            reference: ReportReferenceMode::Frozen {
                snapshot,
                artifact: FrozenReportArtifact::new("image/png", trailing).unwrap(),
            },
        }));
        assert!(matches!(
            resolve_report_source(ReportHardcopySource {
                source_key: "invalid-report".to_owned(),
                document: &invalid,
                reference_inventory: None,
                scope: HardcopyScope::CompleteReport,
            }),
            Err(HardcopySourceError::InvalidReportSource(reason))
                if reason.contains("after IEND")
        ));
    }

    // The public pane adapter deliberately invokes the visualization domain's
    // exact-scene resolver; it never accepts an unverified pane plus loose
    // samples. This fixture therefore exercises document/revision/digest and
    // immutable-dataset validation before physical mapping.
    #[test]
    fn retained_plot_scene_maps_to_platform_independent_integer_geometry() {
        let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x5a; 32]));
        let dataset = SourceDataset::new(
            binding,
            vec![
                SourceColumn::new(
                    "time",
                    "Time",
                    ValueType::Real,
                    ColumnRole::Coordinate,
                    Some("s".to_owned()),
                )
                .unwrap(),
                SourceColumn::new(
                    "out",
                    "V(out)",
                    ValueType::Real,
                    ColumnRole::Signal,
                    Some("V".to_owned()),
                )
                .unwrap(),
            ],
            vec![
                SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(-1.0)]),
                SourceRow::new(vec![TypedValue::Real(1.0), TypedValue::Real(1.0)]),
            ],
        )
        .unwrap();
        let mut document = VisualizationDocument::new("Waveform", vec![dataset]).unwrap();
        let page_id = document.pages()[0].id;
        let pane_id = document.panes()[0].id;
        let axes = document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id,
                        label: "Time".to_owned(),
                        orientation: AxisOrientation::Horizontal,
                        scale: AxisScale::Linear,
                        unit: Some("s".to_owned()),
                        range: None,
                    }),
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id,
                        label: "V".to_owned(),
                        orientation: AxisOrientation::VerticalLeft,
                        scale: AxisScale::Linear,
                        unit: Some("V".to_owned()),
                        range: None,
                    }),
                ],
            )
            .unwrap();
        let x_axis = match axes.created[0] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        let y_axis = match axes.created[1] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddTrace(NewTrace {
                    pane_id,
                    binding,
                    signal_key: "out".to_owned(),
                    coordinate_key: "time".to_owned(),
                    x_axis_id: x_axis,
                    y_axis_id: y_axis,
                    label: "V(out)".to_owned(),
                })],
            )
            .unwrap();
        let digest = document.content_digest().unwrap();
        let reference = ReportReferenceSnapshot::new(
            ReportSourceId::VisualizationDocument {
                document_id: document.id(),
            },
            Some(document.revision()),
            digest,
            vec![binding],
        )
        .unwrap();
        let first = resolve_visualization_pane_source(VisualizationPaneHardcopySource {
            source_key: "plot".to_owned(),
            display_name: "Waveform".to_owned(),
            document: &document,
            reference: &reference,
            page_id,
            pane_id,
            scope: HardcopyScope::ActivePlotDocument,
        })
        .unwrap();
        let second = resolve_visualization_pane_source(VisualizationPaneHardcopySource {
            source_key: "plot".to_owned(),
            display_name: "Waveform".to_owned(),
            document: &document,
            reference: &reference,
            page_id,
            pane_id,
            scope: HardcopyScope::ActivePlotDocument,
        })
        .unwrap();
        assert_eq!(first.semantic_document(), second.semantic_document());
        assert_eq!(
            first.authority().content_digest(),
            second.authority().content_digest()
        );
        assert_ne!(
            first.authority().content_digest(),
            digest,
            "the publication digest also binds the resolved physical semantics"
        );
    }

    #[test]
    fn plot_line_clipping_preserves_true_axis_boundary_intersections() {
        let clipped = clip_line_to_axis_rect((-1.0, 0.25), (3.0, 0.75), 0.0, 2.0, 0.0, 1.0)
            .expect("line crosses the visible axis rectangle");
        assert_eq!(clipped.0, (0.0, 0.375));
        assert_eq!(clipped.1, (2.0, 0.625));
        assert!(clip_line_to_axis_rect((-2.0, 2.0), (-1.0, 3.0), 0.0, 1.0, 0.0, 1.0).is_none());
    }

    #[test]
    fn viewer_partition_covers_every_results_family() {
        let curve_viewers = [
            ResultViewer::Waves,
            ResultViewer::Bode,
            ResultViewer::Fft,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::Nyquist,
            ResultViewer::Smith,
        ];
        let summary_viewers = [
            ResultViewer::Op,
            ResultViewer::NoiseContrib,
            ResultViewer::Contribution,
            ResultViewer::TransferFunction,
            ResultViewer::Specs,
            ResultViewer::PoleZero,
        ];
        assert!(curve_viewers.into_iter().all(is_curve_viewer));
        assert!(
            summary_viewers
                .into_iter()
                .all(|viewer| !is_curve_viewer(viewer))
        );
    }

    #[test]
    fn quick_view_reads_exact_active_retained_waveform_without_report_reference() {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(
            AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![-0.25, 0.5, 1.25],
                    "#00ffff",
                ),
            ]),
        );
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.simulation.data_version = 9;
        state.ui.results.viewer = ResultViewer::Waves;

        let resolved = resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
            source_key: "results-quick-view".to_owned(),
            project_id: state.workspace.project.id(),
            state: &state,
            scope: HardcopyScope::ActivePlotDocument,
        })
        .unwrap();
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected semantic plot")
        };
        assert_eq!(plot.viewer, ResultViewer::Waves);
        assert_eq!(
            plot.traces[0].source_samples,
            vec![
                (0.0f64.to_bits(), (-0.25f64).to_bits()),
                (1.0f64.to_bits(), 0.5f64.to_bits()),
                (2.0f64.to_bits(), 1.25f64.to_bits()),
            ]
        );
        assert!(
            resolved
                .default_print_mapping()
                .entries()
                .iter()
                .any(|entry| {
                    entry.object().kind() == PrintObjectKind::Trace
                        && entry.object().stable_id()
                            == format!("trace:{}", plot.traces[0].trace_id)
                })
        );
        assert_eq!(resolved.authority().revision(), ObjectRevision::INITIAL);
    }

    #[test]
    fn fft_quick_view_ignores_stale_cache_and_global_data_version() {
        let time = (0..64)
            .map(|index| index as f64 * 1.0e-6)
            .collect::<Vec<_>>();
        let values = (0..64)
            .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
            .collect::<Vec<_>>();
        let analysis =
            AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("V(active)", time, values, "#00ffff"),
            ]);
        let mut state = quick_view_state(analysis, ResultViewer::Fft);
        state.analysis.fft_state.selected_source = Some("V(active)".to_owned());
        state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
            "stale",
            &[9_999.0, 10_000.0],
            &[8_888.0, 7_777.0],
            &[0.0, 0.0],
            20_000.0,
        ));
        state.simulation.data_version = 9;

        let first = resolve_quick_view(&state).unwrap();
        let HardcopySemanticDocument::Plot(plot) = first.semantic_document() else {
            panic!("expected FFT plot")
        };
        assert_eq!(plot.viewer, ResultViewer::Fft);
        assert!(
            plot.traces[0]
                .source_samples
                .iter()
                .all(|(x, y)| *x != 9_999.0f64.to_bits() && *y != 8_888.0f64.to_bits())
        );

        state.simulation.data_version = 10_000;
        state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
            "different stale cache",
            &[123_456.0],
            &[654_321.0],
            &[0.0],
            1.0,
        ));
        let second = resolve_quick_view(&state).unwrap();
        assert_eq!(
            first.authority().document_id(),
            second.authority().document_id()
        );
        assert_eq!(first.authority().revision(), second.authority().revision());
        assert_eq!(
            first.authority().content_digest(),
            second.authority().content_digest()
        );
    }

    #[test]
    fn eye_quick_view_reconstructs_the_interactive_source_contract() {
        let time = (0..161)
            .map(|index| index as f64 * 0.25)
            .collect::<Vec<_>>();
        let ignored = vec![42.0; time.len()];
        let selected = time
            .iter()
            .map(|time| if (*time as i64) % 2 == 0 { -1.0 } else { 1.0 })
            .collect::<Vec<_>>();
        let analysis =
            AnalysisResult::new(8, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("A(first)", time.clone(), ignored, "#ff00ff"),
                WaveformData::new("V(selected)", time.clone(), selected.clone(), "#00ffff"),
            ]);
        let mut state = quick_view_state(analysis, ResultViewer::Eye);
        state.analysis.fft_state.selected_source = Some("|V(selected)|".to_owned());
        let mut stale_eye = crate::analysis::EyeData::new(99.0, 7);
        stale_eye.add_trace(crate::analysis::EyeTrace::new(
            vec![0.0, 1.0],
            vec![9_999.0, 9_999.0],
        ));
        state.analysis.eye_diagram_state.load_data(stale_eye);

        let period = retained_eye_bit_period(&time, &selected).unwrap();
        let expected = crate::analysis::eye_diagram::EyeDataBuilder::new()
            .bit_period(period)
            .ui_count(2)
            .skip_initial(2)
            .build(&time, &selected);
        let resolved = resolve_quick_view(&state).unwrap();
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected eye plot")
        };
        assert_eq!(plot.traces.len(), expected.traces.len());
        for (actual, expected) in plot.traces.iter().zip(expected.traces.iter()) {
            assert_eq!(
                actual.source_samples,
                expected
                    .time
                    .iter()
                    .copied()
                    .zip(expected.amplitude.iter().copied())
                    .map(|(x, y)| (x.to_bits(), y.to_bits()))
                    .collect::<Vec<_>>()
            );
            assert!(
                actual
                    .source_samples
                    .iter()
                    .all(|(_, y)| *y != 9_999.0f64.to_bits())
            );
        }
    }

    #[test]
    fn histogram_quick_view_derives_only_from_active_monte_carlo_metadata() {
        let samples = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let analysis = AnalysisResult::new(9, AnalysisType::MonteCarlo, "Monte Carlo")
            .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 17,
                runs_requested: samples.len(),
                runs_completed: samples.len(),
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    name: "gain".to_owned(),
                    samples: samples.clone(),
                    mean: 0.0,
                    std_dev: 2.0f64.sqrt(),
                    min: -2.0,
                    max: 2.0,
                }],
            });
        let mut state = quick_view_state(analysis, ResultViewer::Hist);
        state.analysis.histogram_state.load_histogram(
            crate::analysis::HistogramBuilder::new()
                .name("stale")
                .bin_count(3)
                .build(&[9_999.0; 20]),
        );
        state.analysis.histogram_state.bin_count = 5;

        let resolved = resolve_quick_view(&state).unwrap();
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected histogram plot")
        };
        assert_eq!(plot.traces[0].label, "gain");
        let retained_count = plot.traces[0]
            .source_samples
            .iter()
            .map(|(_, count)| f64::from_bits(*count))
            .sum::<f64>();
        assert_eq!(retained_count, samples.len() as f64);
        assert!(
            plot.traces[0]
                .source_samples
                .iter()
                .all(|(center, _)| *center != 9_999.0f64.to_bits())
        );
    }

    #[test]
    fn nyquist_and_smith_require_active_retained_complex_samples() {
        let real_only = AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![1.0; 16], vec![2.0; 16], "#00ffff"),
        ]);
        let mut stale = quick_view_state(real_only, ResultViewer::Nyquist);
        stale
            .analysis
            .nyquist_state
            .load_data(crate::analysis::NyquistData::from_arrays(
                "stale",
                &[1.0, 2.0],
                &[9_999.0, 8_888.0],
                &[7_777.0, 6_666.0],
            ));
        assert!(matches!(
            resolve_quick_view(&stale),
            Err(HardcopySourceError::MissingViewerEvidence(
                "visible plot series"
            ))
        ));

        let complex = AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new(
                "S(1,1)",
                (0..16).map(|index| index as f64).collect::<Vec<_>>(),
                vec![0.0; 16],
                "#00ffff",
            )
            .with_complex_components(
                "S(1,1)",
                (0..16).map(|index| index as f64 / 16.0).collect::<Vec<_>>(),
                (0..16)
                    .map(|index| -(index as f64) / 32.0)
                    .collect::<Vec<_>>(),
            ),
        ]);
        for viewer in [ResultViewer::Nyquist, ResultViewer::Smith] {
            let state = quick_view_state(complex.clone(), viewer);
            let resolved = resolve_quick_view(&state).unwrap();
            let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
                panic!("expected complex plot")
            };
            assert_eq!(plot.viewer, viewer);
            assert_eq!(
                plot.traces[0].source_samples[1],
                ((1.0f64 / 16.0).to_bits(), (-1.0f64 / 32.0).to_bits())
            );
        }
    }

    #[test]
    fn studio_adapter_reads_retained_dataset_and_places_markers_without_report_reference() {
        let project_id = ProjectId::new();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(
            AnalysisResult::new(12, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![-1.0, 0.5, 1.0],
                    "#00ffff",
                ),
            ]),
        );
        let dataset_id = run.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs.push(run);
        let mut studio = VisualizationStudioState::default();
        studio.revision = 4;
        studio.panes.push(StudioPane {
            id: 19,
            viewer: ResultViewer::Waves,
            viewer_document_id: "viewer-waves".to_owned(),
            dataset_id,
            analysis_sequence: 12,
            x_link: None,
            cursor_group: None,
            page: "Transient results".to_owned(),
            placement: Default::default(),
        });
        studio.active_pane = Some(19);
        studio.markers.push(StudioMarker {
            id: 23,
            dataset_id,
            analysis_sequence: 12,
            waveform_name: "V(out)".to_owned(),
            sample_index: 1,
            x: 1.0,
            y: 0.5,
            label: "M1".to_owned(),
        });

        let resolved = resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
            source_key: "studio-active-pane".to_owned(),
            project_id,
            studio: &studio,
            simulation: &simulation,
            pane_id: studio.active_pane.unwrap(),
            scope: HardcopyScope::ActivePlotDocument,
        })
        .unwrap();
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected studio plot")
        };
        assert_eq!(plot.viewer, ResultViewer::Waves);
        assert_eq!(plot.markers[0].source_x_bits, Some(1.0f64.to_bits()));
        assert_eq!(plot.markers[0].source_y_bits, Some(0.5f64.to_bits()));
        assert!(plot.markers[0].position.is_some());
        assert_eq!(
            resolved.authority().revision(),
            ObjectRevision::new(4).unwrap()
        );

        let initial_digest = resolved.authority().content_digest();
        studio.markers[0].label = "M1 changed".to_owned();
        let marker_changed = resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
            source_key: "studio-active-pane".to_owned(),
            project_id,
            studio: &studio,
            simulation: &simulation,
            pane_id: studio.active_pane.unwrap(),
            scope: HardcopyScope::ActivePlotDocument,
        })
        .unwrap();
        assert_ne!(
            initial_digest,
            marker_changed.authority().content_digest(),
            "marker semantics must bind the resolved visualization digest"
        );

        studio.annotations.push(StudioAnnotation {
            id: 24,
            dataset_id,
            analysis_sequence: 12,
            x: 1.5,
            text: "review point".to_owned(),
        });
        let annotation_changed =
            resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
                source_key: "studio-active-pane".to_owned(),
                project_id,
                studio: &studio,
                simulation: &simulation,
                pane_id: studio.active_pane.unwrap(),
                scope: HardcopyScope::ActivePlotDocument,
            })
            .unwrap();
        assert_ne!(
            marker_changed.authority().content_digest(),
            annotation_changed.authority().content_digest(),
            "annotation semantics must bind the resolved visualization digest"
        );
    }

    #[test]
    fn all_visualization_panes_preserve_retained_pane_order() {
        let project_id = ProjectId::new();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(
            AnalysisResult::new(4, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 0.0],
                    "#00ffff",
                ),
            ]),
        );
        let dataset_id = run.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs.push(run);
        let mut studio = VisualizationStudioState::default();
        studio.revision = 7;
        studio.panes = vec![
            StudioPane {
                id: 41,
                viewer: ResultViewer::Waves,
                viewer_document_id: "viewer-waves".to_owned(),
                dataset_id,
                analysis_sequence: 4,
                x_link: None,
                cursor_group: None,
                page: "Page B".to_owned(),
                placement: Default::default(),
            },
            StudioPane {
                id: 17,
                viewer: ResultViewer::Waves,
                viewer_document_id: "viewer-waves".to_owned(),
                dataset_id,
                analysis_sequence: 4,
                x_link: None,
                cursor_group: None,
                page: "Page A".to_owned(),
                placement: Default::default(),
            },
        ];
        studio.active_pane = Some(41);

        let resolved = resolve_all_studio_panes(project_id, &studio, &simulation).unwrap();
        let HardcopySemanticDocument::Aggregate(aggregate) = resolved.semantic_document() else {
            panic!("expected aggregate")
        };
        assert_eq!(
            aggregate
                .children
                .iter()
                .map(|child| child.source_key.clone())
                .collect::<Vec<_>>(),
            [
                format!("project:{}:visualization-pane:41", project_id.as_uuid()),
                format!("project:{}:visualization-pane:17", project_id.as_uuid()),
            ]
        );
        assert_eq!(
            aggregate
                .children
                .iter()
                .map(|child| child.display_name.clone())
                .collect::<Vec<_>>(),
            ["Page B · WAVES", "Page A · WAVES"]
        );
    }

    #[test]
    fn typed_pole_zero_summary_preserves_native_payload_and_exact_values() {
        let payload = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: vec![ComplexResultValue {
                real: -3.0,
                imaginary: 0.0,
            }],
            gain: 4.0,
        };
        let analysis = AnalysisResult::new(3, AnalysisType::PoleZero, "PZ")
            .with_result_payload(payload.clone());
        let summary = semantic_result_summary(ResultViewer::PoleZero, &analysis).unwrap();
        assert_eq!(summary.viewer, ResultViewer::PoleZero);
        assert_eq!(summary.payload, Some(payload));
        assert_eq!(summary.tables[0].rows.len(), 2);
        assert_eq!(summary.tables[0].rows[0][1], exact_number(-1.0));
        assert_eq!(summary.tables[0].rows[0][2], exact_number(2.0));
    }

    #[test]
    fn global_app_resolver_uses_exact_active_design_registry_identity() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
        let reference = state.workspace.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference.clone()));

        let first = resolve_active_app_hardcopy_source(&state).unwrap();
        let second = resolve_active_app_hardcopy_source(&state).unwrap();
        assert_eq!(
            first.authority().document_id(),
            second.authority().document_id()
        );
        assert_eq!(
            first.authority().revision(),
            state.workspace.project.revision()
        );
        assert!(first.source_key().contains(&reference.key()));
        assert!(matches!(
            first.semantic_document(),
            HardcopySemanticDocument::Schematic(_)
        ));
    }

    #[test]
    fn prepared_resolution_is_send_owned_and_snapshot_isolated() {
        fn assert_send<T: Send>() {}
        assert_send::<PreparedRetainedHardcopyResolution>();

        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
        let reference = state.workspace.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference));
        let source_key = format!(
            "project:{}:cell-view:{}",
            state.workspace.project.id().as_uuid(),
            state.workspace.active_key()
        );
        let synchronous =
            resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::ActiveDocument)
                .unwrap();
        let prepared = prepare_retained_hardcopy_resolution(
            &state,
            &source_key,
            HardcopyScope::ActiveDocument,
        )
        .unwrap();
        state.schematic.wires[0].points[1].x = 9_999;
        let worker_resolved = prepared.resolve_owned().unwrap();
        assert_eq!(worker_resolved, synchronous);
    }

    fn prepared_design_worker_fixture()
    -> (PreparedRetainedHardcopyResolution, ResolvedHardcopyDocument) {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(771, Point::new(-4, 3), Point::new(29, 3)));
        let active_view = state.workspace.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(active_view));
        let source_key = format!(
            "project:{}:cell-view:{}",
            state.workspace.project.id().as_uuid(),
            state.workspace.active_key()
        );
        let expected =
            resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::ActiveDocument)
                .unwrap();
        let prepared = prepare_retained_hardcopy_resolution(
            &state,
            &source_key,
            HardcopyScope::ActiveDocument,
        )
        .unwrap();
        (prepared, expected)
    }

    #[test]
    fn prepared_worker_snapshot_round_trips_exact_owner_before_resolution() {
        let (prepared, expected) = prepared_design_worker_fixture();
        let bytes = prepared.to_worker_snapshot_json().unwrap();
        assert!(bytes.len() <= MAX_WORKER_SNAPSHOT_BYTES);
        let restored =
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&bytes).unwrap();
        assert_eq!(restored.resolve_owned().unwrap(), expected);
    }

    #[test]
    fn prepared_worker_snapshot_rejects_tamper_unknown_fields_and_stale_identity() {
        let (prepared, _) = prepared_design_worker_fixture();
        let bytes = prepared.to_worker_snapshot_json().unwrap();

        let mut tampered: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        tampered["payload"]["identity"]["display_name"] =
            serde_json::Value::String("Tampered owner".to_owned());
        assert!(matches!(
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
                &serde_json::to_vec(&tampered).unwrap()
            ),
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
        ));

        let mut unknown: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        unknown
            .as_object_mut()
            .unwrap()
            .insert("future-field".to_owned(), serde_json::Value::Bool(true));
        assert!(matches!(
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
                &serde_json::to_vec(&unknown).unwrap()
            ),
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
        ));

        let mut stale: PreparedRetainedHardcopyWorkerSnapshot =
            serde_json::from_slice(&bytes).unwrap();
        let PreparedRetainedHardcopyWorkerPayload::Schematic { identity, .. } = &mut stale.payload
        else {
            panic!("expected prepared schematic")
        };
        identity.document_id = HardcopyDocumentId::new();
        stale.transport_digest = stale.compute_transport_digest().unwrap();
        assert!(matches!(
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
                &serde_json::to_vec(&stale).unwrap()
            ),
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
        ));
    }

    #[test]
    fn prepared_worker_snapshot_rejects_unknown_owner_fields_even_with_resealed_transport() {
        let (prepared, _) = prepared_design_worker_fixture();
        let bytes = prepared.to_worker_snapshot_json().unwrap();
        let mut snapshot: PreparedRetainedHardcopyWorkerSnapshot =
            serde_json::from_slice(&bytes).unwrap();
        let PreparedRetainedHardcopyWorkerPayload::Schematic { schematic, .. } =
            &mut snapshot.payload
        else {
            panic!("expected prepared schematic")
        };
        schematic
            .0
            .as_object_mut()
            .unwrap()
            .insert("future-owner-field".to_owned(), serde_json::json!(17));
        snapshot.transport_digest = snapshot.compute_transport_digest().unwrap();
        assert!(matches!(
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
                &serde_json::to_vec(&snapshot).unwrap()
            ),
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
        ));
    }

    #[test]
    fn prepared_worker_snapshot_rejects_oversized_input_before_parsing() {
        let oversized = vec![b' '; MAX_WORKER_SNAPSHOT_BYTES + 1];
        assert!(matches!(
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&oversized),
            Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(actual))
                if actual == MAX_WORKER_SNAPSHOT_BYTES + 1
        ));
    }

    #[test]
    fn enumeration_exposes_all_sheets_exact_members_and_available_named_sets() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(91, Point::new(0, 0), Point::new(20, 0)));
        let reference = state.workspace.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference));
        let active_key = state.workspace.active_key();
        let first_id = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&active_key, "First", [91])
            .unwrap();
        let second_id = state
            .workspace
            .design_management
            .sheet_catalog_mut(&active_key)
            .unwrap()
            .create_sheet(sheet_definition("Second"), Some(first_id))
            .unwrap();
        let base_key = format!(
            "project:{}:cell-view:{}",
            state.workspace.project.id().as_uuid(),
            active_key
        );
        let first_key = format!("{base_key}:sheet:{first_id}");
        let first =
            resolve_retained_hardcopy_source(&state, &first_key, HardcopyScope::CurrentSheet)
                .unwrap();
        let source_set = HardcopySourceSet::try_new(
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            "First only",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet("First only".to_owned()),
            vec![HardcopySourceSetMember::from_resolved(&first).unwrap()],
        )
        .unwrap();
        let set_key = source_set.source_key();
        state
            .workspace
            .save_hardcopy_source_set(source_set)
            .unwrap();

        let descriptors = enumerate_retained_hardcopy_sources(&state);
        let base = descriptors
            .iter()
            .find(|descriptor| descriptor.source_key == base_key)
            .unwrap();
        assert!(base.supports_scope(&HardcopyScope::AllSheetsOrPanes));
        let sheet_keys = descriptors
            .iter()
            .filter(|descriptor| {
                descriptor
                    .source_key
                    .starts_with(&format!("{base_key}:sheet:"))
            })
            .map(|descriptor| descriptor.source_key.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            sheet_keys,
            [first_key, format!("{base_key}:sheet:{second_id}"),]
        );
        let named = descriptors
            .iter()
            .find(|descriptor| descriptor.source_key == set_key)
            .unwrap();
        assert!(named.availability.is_available());
        assert_eq!(
            named.allowed_scopes,
            [HardcopyScope::NamedPrintSet("First only".to_owned())]
        );
    }

    #[test]
    fn global_app_resolver_rejects_stale_results_registry() {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00ffff"),
            ]),
        );
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.workbench.activate(Workspace::Results);
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::ResultDataset(DatasetId::new()));

        let error = resolve_active_app_hardcopy_source(&state).unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceError::StaleActiveDocumentAuthority(_)
        ));
    }

    #[test]
    fn global_app_resolver_does_not_guess_a_background_design_document() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
        let error = resolve_active_app_hardcopy_source(&state).unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceError::NoActiveDocumentAuthority("design")
        ));
    }
}
