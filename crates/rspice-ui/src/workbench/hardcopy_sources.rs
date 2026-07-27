//! Exact, semantic hardcopy source resolution.
//!
//! Hardcopy starts here rather than at a viewport or GPU surface.  Every
//! adapter freezes one durable document revision, resolves authored symbol
//! artwork and retained result samples, computes deterministic physical
//! bounds, and authenticates the semantic snapshot before rendering begins.
//! No type in this module contains pixels, an egui paint command, or a screen
//! rectangle.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::common::AppState;
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

use super::SurfaceId;
use crate::hardcopy::{
    ActiveHardcopySource, ContentExtent, HardcopyContentSection, HardcopyDocumentId,
    HardcopyDocumentKind, HardcopyScope, Length, PrintColor, PrintMappingEntry,
    PrintMappingSaveScope, PrintMappingTable, PrintObjectIdentity, PrintObjectKind,
    PrintRedundancy,
};
// The persisted source-set records and the validation they share with these
// adapters are owned one layer down, where `state` can reach them.
use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, HardcopySourceError, HardcopySourceIdentity, HardcopySourceSet,
    HardcopySourceSetMember, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT, canonical_digest,
    validate_label,
};
use super::result_document::ResultViewer;
use super::session::SymbolSelection;
use super::state::{Workspace, WorkspaceDocumentId};
use super::visualization_studio::{
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
const WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
const SCHEMATIC_EDGE_ALLOWANCE_UNITS: i64 = 16;
const SYMBOL_EDGE_ALLOWANCE_UNITS: i64 = 10;
const PLOT_INSET_UM: i64 = 12_700;

/// One exact point in the source's natural physical coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPoint {
    pub x_um: i64,
    pub y_um: i64,
}

impl SemanticPoint {
    #[must_use]
    pub const fn new(x_um: i64, y_um: i64) -> Self {
        Self { x_um, y_um }
    }
}

/// Signed source bounds.  Negative authored coordinates are retained; only
/// width and height become unsigned when compiling the pagination contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticBounds {
    pub minimum: SemanticPoint,
    pub maximum: SemanticPoint,
}

impl SemanticBounds {
    pub fn try_new(
        minimum: SemanticPoint,
        maximum: SemanticPoint,
    ) -> Result<Self, HardcopySourceError> {
        if minimum.x_um >= maximum.x_um || minimum.y_um >= maximum.y_um {
            return Err(HardcopySourceError::EmptyContent);
        }
        Ok(Self { minimum, maximum })
    }

    fn content_extent(self) -> Result<ContentExtent, HardcopySourceError> {
        let width = self
            .maximum
            .x_um
            .checked_sub(self.minimum.x_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let height = self
            .maximum
            .y_um
            .checked_sub(self.minimum.y_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        ContentExtent::try_new(
            Length::from_micrometres(width),
            Length::from_micrometres(height),
        )
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
    }
}

/// Resolution provenance for a placed cell instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticSymbolSource {
    Authored,
    Generated,
}

/// A resolved cell instance keeps the authored graphic document rather than
/// substituting a generic box during publication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticComponent {
    pub component: Component,
    pub resolved_symbol: Option<SymbolDocument>,
    pub symbol_source: Option<SemanticSymbolSource>,
}

/// Exact durable schematic content after scope filtering and cell-symbol
/// resolution. Runtime tools, pan, zoom, clipboard, and hover state are
/// intentionally absent from the authenticated source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticSchematic {
    /// Canonical active cell/view path used by property-display notes.
    pub view_path: String,
    pub components: Vec<SemanticComponent>,
    pub wires: Vec<Wire>,
    pub buses: Vec<Bus>,
    pub bus_taps: Vec<BusTap>,
    pub junctions: Vec<Junction>,
    pub net_labels: Vec<NetLabel>,
    pub design_notes: Vec<DesignNote>,
    pub documentation_shapes: Vec<DocumentationShape>,
}

/// One immutable, physically mapped result trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPlotTrace {
    pub trace_id: u64,
    pub label: String,
    /// Contiguous visible paths after exact axis-rectangle clipping. A trace
    /// may have several paths when it leaves and re-enters the visible range.
    pub paths: Vec<Vec<SemanticPoint>>,
    /// Exact retained samples as IEEE-754 bit patterns. Specialized viewers
    /// derive their geometry without losing source precision to page mapping.
    pub source_samples: Vec<(u64, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPlotMarker {
    pub marker_id: u64,
    pub label: String,
    pub trace_id: Option<u64>,
    pub source_x_bits: Option<u64>,
    pub source_y_bits: Option<u64>,
    pub position: Option<SemanticPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPlotAnnotation {
    pub annotation_id: u64,
    pub text: String,
    pub trace_id: Option<u64>,
    pub source_x_bits: Option<u64>,
    pub source_y_bits: Option<u64>,
    pub position: Option<SemanticPoint>,
}

/// Exact active Cartesian pane. Source samples are mapped to integer physical
/// coordinates only after the retained scene has validated its document,
/// revision, digest, axes, bindings, and supported viewer semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPlot {
    pub viewer: ResultViewer,
    pub page_id: u64,
    pub pane_id: u64,
    pub traces: Vec<SemanticPlotTrace>,
    pub markers: Vec<SemanticPlotMarker>,
    pub annotations: Vec<SemanticPlotAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTable {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Typed non-curve retained result. Exact native payload remains attached for
/// viewer-specific diagrams, while normalized tables give every publication
/// backend a common semantic surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticResultSummary {
    pub viewer: ResultViewer,
    pub title: String,
    pub tables: Vec<SemanticTable>,
    pub payload: Option<AnalysisResultPayload>,
}

/// Exact report snapshot taken from the authenticated revision ledger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticReport {
    pub pages: Vec<ReportPage>,
    /// Exact external-reference contracts that were authenticated before the
    /// snapshot became publishable. Typed table/evidence content remains in
    /// `pages`; frozen artifact bytes remain attached to their reference mode.
    pub authenticated_references: Vec<SemanticReportReference>,
    /// Validated opaque figures indexed by their authored report block.
    pub figures: Vec<SemanticReportFigure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticReportReference {
    pub block_id: ReportBlockId,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticReportFigure {
    pub block_id: ReportBlockId,
    pub artifact_digest: ContentDigest,
    pub media_type: String,
    pub payload: Vec<u8>,
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub caption: String,
    pub alternative_text: String,
    pub sizing: FigureSizing,
}

/// An authenticated child of an ordered aggregate. Child coordinates remain
/// local to `local_bounds`; `placement_origin` gives the aggregate coordinate
/// corresponding to that local minimum. Every child after the first begins a
/// new logical page group, so renderers never infer pagination from a gap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAggregateChild {
    pub ordinal: u32,
    pub source_key: String,
    pub display_name: String,
    pub document_id: HardcopyDocumentId,
    pub revision: ObjectRevision,
    pub content_digest: ContentDigest,
    pub local_bounds: SemanticBounds,
    pub placement_origin: SemanticPoint,
    pub page_break_before: bool,
    pub document: Box<HardcopySemanticDocument>,
}

/// Stable, ordered multi-document semantic source used for "all
/// sheets/panes" and project-owned named print sets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAggregate {
    pub source_set_digest: ContentDigest,
    pub children: Vec<SemanticAggregateChild>,
}

/// Renderer-neutral content variants. These retain authored engineering
/// objects and numeric traces, not a picture of the UI that displayed them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "document-kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HardcopySemanticDocument {
    Schematic(SemanticSchematic),
    Symbol(SymbolDocument),
    Plot(SemanticPlot),
    ResultSummary(SemanticResultSummary),
    Report(SemanticReport),
    Aggregate(SemanticAggregate),
}

/// Frozen active source plus its semantic scene and natural physical extent.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ResolvedHardcopyDocument {
    source_key: String,
    authority: ActiveHardcopySource,
    semantic_document: HardcopySemanticDocument,
    bounds: SemanticBounds,
    content_extent: ContentExtent,
    default_print_mapping: PrintMappingTable,
}

impl ResolvedHardcopyDocument {
    #[must_use]
    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    #[must_use]
    pub const fn authority(&self) -> &ActiveHardcopySource {
        &self.authority
    }

    #[must_use]
    pub const fn semantic_document(&self) -> &HardcopySemanticDocument {
        &self.semantic_document
    }

    #[must_use]
    pub const fn bounds(&self) -> SemanticBounds {
        self.bounds
    }

    #[must_use]
    pub const fn content_extent(&self) -> ContentExtent {
        self.content_extent
    }

    /// Source-derived defaults keyed by stable semantic identity. Callers can
    /// overlay persisted user choices by `(kind, stable_id)` without relying
    /// on table position or display text.
    #[must_use]
    pub const fn default_print_mapping(&self) -> &PrintMappingTable {
        &self.default_print_mapping
    }

    /// Ordered authenticated page groups for aggregate-aware pagination.
    /// Ordinary documents return an empty vector and retain legacy extent
    /// compilation; aggregate callers must pass every returned section to
    /// `HardcopyPlan::compile_with_sections`.
    pub fn hardcopy_sections(&self) -> Result<Vec<HardcopyContentSection>, HardcopySourceError> {
        let HardcopySemanticDocument::Aggregate(aggregate) = &self.semantic_document else {
            return Ok(Vec::new());
        };
        aggregate
            .children
            .iter()
            .map(|child| {
                let origin_x = u64::try_from(child.placement_origin.x_um)
                    .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
                let origin_y = u64::try_from(child.placement_origin.y_um)
                    .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
                HardcopyContentSection::try_new(
                    child.ordinal,
                    child.content_digest,
                    Length::from_micrometres(origin_x),
                    Length::from_micrometres(origin_y),
                    child.local_bounds.content_extent()?,
                    child.page_break_before,
                )
                .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
            })
            .collect()
    }

    /// Bounded serde envelope for browser dedicated-worker transfer.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn to_worker_snapshot_json(&self) -> Result<Vec<u8>, HardcopySourceError> {
        let snapshot = ResolvedHardcopyWorkerSnapshot::from_resolved(self)?;
        let bytes = serde_json::to_vec(&snapshot)
            .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::WorkerSnapshotTooLarge(bytes.len()));
        }
        Ok(bytes)
    }

    /// Restore and revalidate a dedicated-worker envelope. Callers must pass
    /// the exact byte payload rather than deserializing an unbounded value.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn from_worker_snapshot_json(bytes: &[u8]) -> Result<Self, HardcopySourceError> {
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::WorkerSnapshotTooLarge(bytes.len()));
        }
        let snapshot: ResolvedHardcopyWorkerSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| HardcopySourceError::InvalidWorkerSnapshot(error.to_string()))?;
        snapshot.into_resolved()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) struct ResolvedHardcopyWorkerSnapshot {
    schema_version: u32,
    source_key: String,
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    content_digest: ContentDigest,
    display_name: String,
    document_kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    semantic_document: HardcopySemanticDocument,
    bounds: SemanticBounds,
    content_extent: ContentExtent,
    default_print_mapping: PrintMappingTable,
    transport_digest: ContentDigest,
}

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct WorkerSnapshotDigestMaterial<'a> {
    schema_version: u32,
    source_key: &'a str,
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    content_digest: ContentDigest,
    display_name: &'a str,
    document_kind: HardcopyDocumentKind,
    scope: &'a HardcopyScope,
    semantic_document: &'a HardcopySemanticDocument,
    bounds: SemanticBounds,
    content_extent: ContentExtent,
    default_print_mapping: &'a PrintMappingTable,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl ResolvedHardcopyWorkerSnapshot {
    fn from_resolved(resolved: &ResolvedHardcopyDocument) -> Result<Self, HardcopySourceError> {
        let mut snapshot = Self {
            schema_version: WORKER_SNAPSHOT_SCHEMA_VERSION,
            source_key: resolved.source_key.clone(),
            document_id: resolved.authority.document_id(),
            revision: resolved.authority.revision(),
            content_digest: resolved.authority.content_digest(),
            display_name: resolved.authority.display_name().to_owned(),
            document_kind: resolved.authority.document_kind(),
            scope: resolved.authority.scope().clone(),
            semantic_document: resolved.semantic_document.clone(),
            bounds: resolved.bounds,
            content_extent: resolved.content_extent,
            default_print_mapping: resolved.default_print_mapping.clone(),
            transport_digest: ContentDigest::from_bytes([0; 32]),
        };
        snapshot.transport_digest = snapshot.compute_transport_digest()?;
        snapshot.validate()?;
        Ok(snapshot)
    }

    fn compute_transport_digest(&self) -> Result<ContentDigest, HardcopySourceError> {
        canonical_digest(
            b"rspice-hardcopy-worker-snapshot-v1",
            &WorkerSnapshotDigestMaterial {
                schema_version: self.schema_version,
                source_key: &self.source_key,
                document_id: self.document_id,
                revision: self.revision,
                content_digest: self.content_digest,
                display_name: &self.display_name,
                document_kind: self.document_kind,
                scope: &self.scope,
                semantic_document: &self.semantic_document,
                bounds: self.bounds,
                content_extent: self.content_extent,
                default_print_mapping: &self.default_print_mapping,
            },
        )
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        if self.schema_version != WORKER_SNAPSHOT_SCHEMA_VERSION {
            return Err(HardcopySourceError::InvalidWorkerSnapshot(format!(
                "unsupported schema version {}",
                self.schema_version
            )));
        }
        validate_label("worker source key", &self.source_key, SOURCE_KEY_LIMIT)?;
        validate_label(
            "worker display name",
            &self.display_name,
            DISPLAY_NAME_LIMIT,
        )?;
        if self.bounds.content_extent()? != self.content_extent {
            return Err(HardcopySourceError::InvalidWorkerSnapshot(
                "content extent does not match signed semantic bounds".to_owned(),
            ));
        }
        validate_worker_semantics(&self.semantic_document)?;
        let expected_mapping = default_print_mapping(&self.semantic_document)?;
        if expected_mapping != self.default_print_mapping {
            return Err(HardcopySourceError::InvalidWorkerSnapshot(
                "default print mapping does not match semantic content".to_owned(),
            ));
        }
        let actual = self.compute_transport_digest()?;
        if actual != self.transport_digest {
            return Err(HardcopySourceError::InvalidWorkerSnapshot(
                "transport digest does not authenticate the worker snapshot".to_owned(),
            ));
        }
        ActiveHardcopySource::try_new(
            self.document_id,
            self.revision,
            self.content_digest,
            self.display_name.clone(),
            self.document_kind,
            self.scope.clone(),
        )
        .map_err(|error| HardcopySourceError::InvalidWorkerSnapshot(error.to_string()))?;
        Ok(())
    }

    fn into_resolved(self) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
        self.validate()?;
        let authority = ActiveHardcopySource::try_new(
            self.document_id,
            self.revision,
            self.content_digest,
            self.display_name,
            self.document_kind,
            self.scope,
        )
        .map_err(|error| HardcopySourceError::InvalidWorkerSnapshot(error.to_string()))?;
        Ok(ResolvedHardcopyDocument {
            source_key: self.source_key,
            authority,
            semantic_document: self.semantic_document,
            bounds: self.bounds,
            content_extent: self.content_extent,
            default_print_mapping: self.default_print_mapping,
        })
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn validate_worker_semantics(
    document: &HardcopySemanticDocument,
) -> Result<(), HardcopySourceError> {
    match document {
        HardcopySemanticDocument::Aggregate(aggregate) => {
            if aggregate.children.is_empty()
                || aggregate.children.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS
            {
                return Err(HardcopySourceError::InvalidWorkerSnapshot(
                    "aggregate child count is outside the governed limit".to_owned(),
                ));
            }
            for (index, child) in aggregate.children.iter().enumerate() {
                if child.ordinal != index as u32
                    || child.page_break_before != (index != 0)
                    || child.placement_origin.x_um < 0
                    || child.placement_origin.y_um < 0
                    || matches!(
                        child.document.as_ref(),
                        HardcopySemanticDocument::Aggregate(_)
                    )
                {
                    return Err(HardcopySourceError::InvalidWorkerSnapshot(format!(
                        "aggregate child {index} violates ordering, placement, or nesting"
                    )));
                }
                validate_label("aggregate child key", &child.source_key, SOURCE_KEY_LIMIT)?;
                validate_label(
                    "aggregate child display name",
                    &child.display_name,
                    DISPLAY_NAME_LIMIT,
                )?;
                child.local_bounds.content_extent()?;
                validate_worker_semantics(&child.document)?;
            }
        }
        HardcopySemanticDocument::Report(report) => {
            for figure in &report.figures {
                let artifact =
                    FrozenReportArtifact::new(figure.media_type.clone(), figure.payload.clone())
                        .map_err(|error| {
                            HardcopySourceError::InvalidWorkerSnapshot(error.to_string())
                        })?;
                if artifact.content_digest() != figure.artifact_digest {
                    return Err(HardcopySourceError::InvalidWorkerSnapshot(format!(
                        "report figure {} digest mismatch",
                        figure.block_id
                    )));
                }
                let (width, height) = validate_frozen_report_png(figure.block_id, &artifact)?;
                if (width, height) != (figure.width_pixels, figure.height_pixels) {
                    return Err(HardcopySourceError::InvalidWorkerSnapshot(format!(
                        "report figure {} dimension mismatch",
                        figure.block_id
                    )));
                }
            }
        }
        HardcopySemanticDocument::Schematic(_)
        | HardcopySemanticDocument::Symbol(_)
        | HardcopySemanticDocument::Plot(_)
        | HardcopySemanticDocument::ResultSummary(_) => {}
    }
    Ok(())
}

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

    #[must_use]
    pub fn unavailable_scope_reason(&self, scope: &HardcopyScope) -> Option<&'static str> {
        if self.supports_scope(scope) {
            return None;
        }
        Some(match scope {
            HardcopyScope::AllSheetsOrPanes => {
                "no authenticated ordered aggregate of sheets or panes is retained"
            }
            HardcopyScope::NamedPrintSet(_) => {
                "the project has no source-owning named print-set domain"
            }
            _ => "the selected retained source does not support this scope",
        })
    }
}

/// Owned, `Send`-safe retained-source snapshot prepared on the UI thread
/// without hashing samples, resolving symbols, or constructing semantic
/// geometry. The worker consumes it with [`Self::resolve_owned`].
pub(crate) struct PreparedRetainedHardcopyResolution {
    payload: PreparedRetainedHardcopyPayload,
}

enum PreparedRetainedHardcopyPayload {
    Schematic {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        schematic: SchematicState,
        library_manager: crate::state::LibraryManager,
        schematic_buffers: std::collections::HashMap<String, SchematicState>,
        sheet_catalog: Option<SheetCatalog>,
        sheet_id: Option<SheetId>,
        all_sheets: bool,
        scope: HardcopyScope,
    },
    Symbol {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        document: SymbolDocument,
        scope: HardcopyScope,
    },
    Results {
        source_key: String,
        project_id: ProjectId,
        run: SimulationRun,
        presentation: ResultsQuickViewPresentation,
        scope: HardcopyScope,
    },
    Studio {
        source_key: String,
        project_id: ProjectId,
        studio: VisualizationStudioState,
        simulation: SimulationState,
        pane_id: u64,
        all_panes: bool,
        scope: HardcopyScope,
    },
    Report {
        project_id: ProjectId,
        source_key: String,
        document: ReportDocument,
        scope: HardcopyScope,
    },
    SourceSet {
        source_set: HardcopySourceSet,
        members: Vec<PreparedRetainedHardcopyResolution>,
    },
}

/// A canonical owner value keeps the worker schema closed even when an
/// application-owned type accepts omitted/defaulted fields for project-file
/// migration. Decoding must reproduce the exact JSON value; an ignored
/// unknown field, non-canonical alias, or lossy default therefore fails before
/// any source resolution begins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
struct CanonicalHardcopyOwner(serde_json::Value);

impl CanonicalHardcopyOwner {
    fn capture<T: Serialize>(field: &'static str, owner: &T) -> Result<Self, HardcopySourceError> {
        serde_json::to_value(owner).map(Self).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} cannot be serialized: {error}"
            ))
        })
    }

    fn restore<T>(self, field: &'static str) -> Result<T, HardcopySourceError>
    where
        T: DeserializeOwned + Serialize,
    {
        let owner: T = serde_json::from_value(self.0.clone()).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} is invalid: {error}"
            ))
        })?;
        let canonical = serde_json::to_value(&owner).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} cannot be canonicalized: {error}"
            ))
        })?;
        if canonical != self.0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} contains unknown, aliased, or non-canonical fields"
            )));
        }
        Ok(owner)
    }
}

/// Exact schematic owner fields consumed by semantic hardcopy resolution.
/// Editor gestures, clipboard, viewport, history caches, and save paths never
/// cross the worker boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedSchematicOwner {
    components: Vec<Component>,
    wires: Vec<Wire>,
    buses: Vec<Bus>,
    bus_taps: Vec<BusTap>,
    junctions: Vec<Junction>,
    net_labels: Vec<NetLabel>,
    design_notes: Vec<DesignNote>,
    documentation_shapes: Vec<DocumentationShape>,
    selection: Selection,
}

impl PreparedSchematicOwner {
    fn capture(schematic: SchematicState) -> Self {
        Self {
            components: schematic.components,
            wires: schematic.wires,
            buses: schematic.buses,
            bus_taps: schematic.bus_taps,
            junctions: schematic.junctions,
            net_labels: schematic.net_labels,
            design_notes: schematic.design_notes,
            documentation_shapes: schematic.documentation_shapes,
            selection: schematic.selection,
        }
    }

    fn restore(self) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = self.components;
        schematic.wires = self.wires;
        schematic.buses = self.buses;
        schematic.bus_taps = self.bus_taps;
        schematic.junctions = self.junctions;
        schematic.net_labels = self.net_labels;
        schematic.design_notes = self.design_notes;
        schematic.documentation_shapes = self.documentation_shapes;
        schematic.selection = self.selection;
        schematic
    }
}

/// Hierarchical symbol fallback needs only ordered interface-port components
/// from each retained schematic cell, never the rest of its editor document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedSchematicInterfaceOwner {
    components: Vec<Component>,
}

impl PreparedSchematicInterfaceOwner {
    fn capture(schematic: SchematicState) -> Self {
        Self {
            components: schematic.components,
        }
    }

    fn restore(self) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = self.components;
        schematic
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftNormalization {
    Peak,
    Rms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftWindow {
    Rectangular,
    Hanning,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Kaiser,
    Gaussian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftInputFidelity {
    Reference,
    Interactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedHistogramMode {
    Count,
    Pdf,
    Cdf,
    Percent,
}

/// Only the persisted controls that affect quick-result semantic geometry.
/// FFT caches and every other viewer/runtime field are deliberately absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedResultsPresentation {
    viewer: ResultViewer,
    fft_selected_source: Option<String>,
    fft_normalization: PreparedFftNormalization,
    fft_window: PreparedFftWindow,
    fft_input_fidelity: PreparedFftInputFidelity,
    fft_time_window_auto: bool,
    fft_time_window_start: f64,
    fft_time_window_end: f64,
    fft_sample_count_auto: bool,
    fft_sample_count: usize,
    histogram_selected: usize,
    histogram_bin_count: usize,
    histogram_custom_range: bool,
    histogram_custom_min: f64,
    histogram_custom_max: f64,
    histogram_mode: PreparedHistogramMode,
}

impl PreparedResultsPresentation {
    fn capture(value: ResultsQuickViewPresentation) -> Result<Self, HardcopySourceError> {
        validate_optional_label(
            "prepared FFT source",
            value.fft.selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        let captured = Self {
            viewer: value.viewer,
            fft_selected_source: value.fft.selected_source,
            fft_normalization: match value.fft.normalization {
                crate::analysis::fft::data::SpectrumNormalization::Peak => {
                    PreparedFftNormalization::Peak
                }
                crate::analysis::fft::data::SpectrumNormalization::Rms => {
                    PreparedFftNormalization::Rms
                }
            },
            fft_window: match value.fft.window {
                crate::analysis::WindowFunction::Rectangular => PreparedFftWindow::Rectangular,
                crate::analysis::WindowFunction::Hanning => PreparedFftWindow::Hanning,
                crate::analysis::WindowFunction::Hamming => PreparedFftWindow::Hamming,
                crate::analysis::WindowFunction::Blackman => PreparedFftWindow::Blackman,
                crate::analysis::WindowFunction::BlackmanHarris => {
                    PreparedFftWindow::BlackmanHarris
                }
                crate::analysis::WindowFunction::FlatTop => PreparedFftWindow::FlatTop,
                crate::analysis::WindowFunction::Kaiser => PreparedFftWindow::Kaiser,
                crate::analysis::WindowFunction::Gaussian => PreparedFftWindow::Gaussian,
            },
            fft_input_fidelity: match value.fft.input_fidelity {
                crate::analysis::InputFidelity::Reference => PreparedFftInputFidelity::Reference,
                crate::analysis::InputFidelity::Interactive => {
                    PreparedFftInputFidelity::Interactive
                }
            },
            fft_time_window_auto: value.fft.time_window_auto,
            fft_time_window_start: value.fft.time_window_start,
            fft_time_window_end: value.fft.time_window_end,
            fft_sample_count_auto: value.fft.sample_count_auto,
            fft_sample_count: value.fft.sample_count,
            histogram_selected: value.histogram_selected,
            histogram_bin_count: value.histogram_bin_count,
            histogram_custom_range: value.histogram_custom_range,
            histogram_custom_min: value.histogram_custom_min,
            histogram_custom_max: value.histogram_custom_max,
            histogram_mode: match value.histogram_mode {
                crate::analysis::HistogramDisplayMode::Count => PreparedHistogramMode::Count,
                crate::analysis::HistogramDisplayMode::Pdf => PreparedHistogramMode::Pdf,
                crate::analysis::HistogramDisplayMode::Cdf => PreparedHistogramMode::Cdf,
                crate::analysis::HistogramDisplayMode::Percent => PreparedHistogramMode::Percent,
            },
        };
        captured.validate()?;
        Ok(captured)
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        validate_optional_label(
            "prepared FFT source",
            self.fft_selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        for (field, value) in [
            ("FFT time-window start", self.fft_time_window_start),
            ("FFT time-window end", self.fft_time_window_end),
            ("histogram custom minimum", self.histogram_custom_min),
            ("histogram custom maximum", self.histogram_custom_max),
        ] {
            if !value.is_finite() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "{field} is not finite"
                )));
            }
        }
        if self.fft_sample_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "FFT sample count is zero".to_owned(),
            ));
        }
        if self.histogram_bin_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "histogram bin count is zero".to_owned(),
            ));
        }
        if !self.fft_time_window_auto && self.fft_time_window_start >= self.fft_time_window_end {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "manual FFT time window is empty or reversed".to_owned(),
            ));
        }
        if self.histogram_custom_range && self.histogram_custom_min >= self.histogram_custom_max {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "custom histogram range is empty or reversed".to_owned(),
            ));
        }
        Ok(())
    }

    fn restore(self) -> Result<ResultsQuickViewPresentation, HardcopySourceError> {
        self.validate()?;
        let mut fft = crate::analysis::FftState::default();
        fft.selected_source = self.fft_selected_source;
        fft.normalization = match self.fft_normalization {
            PreparedFftNormalization::Peak => {
                crate::analysis::fft::data::SpectrumNormalization::Peak
            }
            PreparedFftNormalization::Rms => crate::analysis::fft::data::SpectrumNormalization::Rms,
        };
        fft.window = match self.fft_window {
            PreparedFftWindow::Rectangular => crate::analysis::WindowFunction::Rectangular,
            PreparedFftWindow::Hanning => crate::analysis::WindowFunction::Hanning,
            PreparedFftWindow::Hamming => crate::analysis::WindowFunction::Hamming,
            PreparedFftWindow::Blackman => crate::analysis::WindowFunction::Blackman,
            PreparedFftWindow::BlackmanHarris => crate::analysis::WindowFunction::BlackmanHarris,
            PreparedFftWindow::FlatTop => crate::analysis::WindowFunction::FlatTop,
            PreparedFftWindow::Kaiser => crate::analysis::WindowFunction::Kaiser,
            PreparedFftWindow::Gaussian => crate::analysis::WindowFunction::Gaussian,
        };
        fft.input_fidelity = match self.fft_input_fidelity {
            PreparedFftInputFidelity::Reference => crate::analysis::InputFidelity::Reference,
            PreparedFftInputFidelity::Interactive => crate::analysis::InputFidelity::Interactive,
        };
        fft.time_window_auto = self.fft_time_window_auto;
        fft.time_window_start = self.fft_time_window_start;
        fft.time_window_end = self.fft_time_window_end;
        fft.sample_count_auto = self.fft_sample_count_auto;
        fft.sample_count = self.fft_sample_count;
        Ok(ResultsQuickViewPresentation {
            viewer: self.viewer,
            fft,
            histogram_selected: self.histogram_selected,
            histogram_bin_count: self.histogram_bin_count,
            histogram_custom_range: self.histogram_custom_range,
            histogram_custom_min: self.histogram_custom_min,
            histogram_custom_max: self.histogram_custom_max,
            histogram_mode: match self.histogram_mode {
                PreparedHistogramMode::Count => crate::analysis::HistogramDisplayMode::Count,
                PreparedHistogramMode::Pdf => crate::analysis::HistogramDisplayMode::Pdf,
                PreparedHistogramMode::Cdf => crate::analysis::HistogramDisplayMode::Cdf,
                PreparedHistogramMode::Percent => crate::analysis::HistogramDisplayMode::Percent,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source-family", rename_all = "kebab-case", deny_unknown_fields)]
enum PreparedRetainedHardcopyWorkerPayload {
    Schematic {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        schematic: CanonicalHardcopyOwner,
        library_manager: CanonicalHardcopyOwner,
        schematic_buffers: CanonicalHardcopyOwner,
        sheet_catalog: Option<CanonicalHardcopyOwner>,
        sheet_id: Option<SheetId>,
        all_sheets: bool,
        scope: HardcopyScope,
    },
    Symbol {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        document: CanonicalHardcopyOwner,
        scope: HardcopyScope,
    },
    Results {
        source_key: String,
        project_id: ProjectId,
        simulation_results: CanonicalHardcopyOwner,
        presentation: PreparedResultsPresentation,
        scope: HardcopyScope,
    },
    Studio {
        source_key: String,
        project_id: ProjectId,
        studio: CanonicalHardcopyOwner,
        simulation_results: CanonicalHardcopyOwner,
        pane_id: u64,
        all_panes: bool,
        scope: HardcopyScope,
    },
    Report {
        project_id: ProjectId,
        source_key: String,
        document: CanonicalHardcopyOwner,
        scope: HardcopyScope,
    },
    SourceSet {
        source_set: HardcopySourceSet,
        members: Vec<PreparedRetainedHardcopyWorkerPayload>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedRetainedHardcopyWorkerSnapshot {
    schema_version: u32,
    payload: PreparedRetainedHardcopyWorkerPayload,
    transport_digest: ContentDigest,
}

#[derive(Serialize)]
struct PreparedRetainedHardcopyWorkerDigestMaterial<'a> {
    schema_version: u32,
    payload: &'a PreparedRetainedHardcopyWorkerPayload,
}

impl PreparedRetainedHardcopyWorkerSnapshot {
    fn capture(prepared: PreparedRetainedHardcopyResolution) -> Result<Self, HardcopySourceError> {
        let payload = PreparedRetainedHardcopyWorkerPayload::capture(prepared.payload)?;
        let mut snapshot = Self {
            schema_version: PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION,
            payload,
            transport_digest: ContentDigest::from_bytes([0; 32]),
        };
        snapshot.validate_shape()?;
        snapshot.transport_digest = snapshot.compute_transport_digest()?;
        Ok(snapshot)
    }

    fn compute_transport_digest(&self) -> Result<ContentDigest, HardcopySourceError> {
        canonical_digest(
            b"rspice-prepared-hardcopy-worker-snapshot-v1",
            &PreparedRetainedHardcopyWorkerDigestMaterial {
                schema_version: self.schema_version,
                payload: &self.payload,
            },
        )
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        self.validate_shape()?;
        let actual = self.compute_transport_digest()?;
        if actual != self.transport_digest {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "transport digest does not authenticate the prepared owner snapshot".to_owned(),
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), HardcopySourceError> {
        if self.schema_version != PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "unsupported schema version {}",
                self.schema_version
            )));
        }
        self.payload.validate_shape(false)
    }

    fn into_prepared(self) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
        self.validate()?;
        Ok(PreparedRetainedHardcopyResolution {
            payload: self.payload.restore()?,
        })
    }
}

impl PreparedRetainedHardcopyWorkerPayload {
    fn capture(payload: PreparedRetainedHardcopyPayload) -> Result<Self, HardcopySourceError> {
        Ok(match payload {
            PreparedRetainedHardcopyPayload::Schematic {
                project_id,
                identity,
                schematic,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                all_sheets,
                scope,
            } => {
                let schematic = PreparedSchematicOwner::capture(schematic);
                let mut library_manager = library_manager;
                library_manager.selected_library = None;
                library_manager.selected_cell = None;
                library_manager.selected_view = None;
                library_manager.filter_text.clear();
                library_manager.show_read_only = false;
                let schematic_buffers = schematic_buffers
                    .into_iter()
                    .map(|(key, schematic)| {
                        (key, PreparedSchematicInterfaceOwner::capture(schematic))
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                Self::Schematic {
                    project_id,
                    identity,
                    schematic: CanonicalHardcopyOwner::capture("prepared schematic", &schematic)?,
                    library_manager: CanonicalHardcopyOwner::capture(
                        "prepared symbol library",
                        &library_manager,
                    )?,
                    schematic_buffers: CanonicalHardcopyOwner::capture(
                        "prepared schematic symbol buffers",
                        &schematic_buffers,
                    )?,
                    sheet_catalog: sheet_catalog
                        .as_ref()
                        .map(|catalog| {
                            CanonicalHardcopyOwner::capture("prepared sheet catalog", catalog)
                        })
                        .transpose()?,
                    sheet_id,
                    all_sheets,
                    scope,
                }
            }
            PreparedRetainedHardcopyPayload::Symbol {
                project_id,
                identity,
                document,
                scope,
            } => Self::Symbol {
                project_id,
                identity,
                document: CanonicalHardcopyOwner::capture("prepared symbol document", &document)?,
                scope,
            },
            PreparedRetainedHardcopyPayload::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
                let mut simulation = SimulationState::default();
                simulation.next_run_id = run.id;
                simulation.active_run_idx = Some(0);
                simulation.active_analysis_idx = Some(0);
                simulation.runs = vec![run];
                Self::Results {
                    source_key,
                    project_id,
                    simulation_results: CanonicalHardcopyOwner::capture(
                        "prepared result history",
                        &ProjectSimulationResults::from_state(&simulation),
                    )?,
                    presentation: PreparedResultsPresentation::capture(presentation)?,
                    scope,
                }
            }
            PreparedRetainedHardcopyPayload::Studio {
                source_key,
                project_id,
                studio,
                simulation,
                pane_id,
                all_panes,
                scope,
            } => Self::Studio {
                source_key,
                project_id,
                studio: CanonicalHardcopyOwner::capture("prepared visualization studio", &studio)?,
                simulation_results: CanonicalHardcopyOwner::capture(
                    "prepared studio result history",
                    &ProjectSimulationResults::from_state(&simulation),
                )?,
                pane_id,
                all_panes,
                scope,
            },
            PreparedRetainedHardcopyPayload::Report {
                project_id,
                source_key,
                document,
                scope,
            } => Self::Report {
                project_id,
                source_key,
                document: CanonicalHardcopyOwner::capture("prepared report document", &document)?,
                scope,
            },
            PreparedRetainedHardcopyPayload::SourceSet {
                source_set,
                members,
            } => Self::SourceSet {
                source_set,
                members: members
                    .into_iter()
                    .map(|member| Self::capture(member.payload))
                    .collect::<Result<Vec<_>, _>>()?,
            },
        })
    }

    fn validate_shape(&self, nested: bool) -> Result<(), HardcopySourceError> {
        match self {
            Self::Schematic {
                project_id,
                identity,
                sheet_catalog,
                sheet_id,
                all_sheets,
                scope,
                ..
            } => {
                validate_project_source_identity(*project_id, identity, "cell-view")?;
                match (*all_sheets, sheet_catalog.is_some(), *sheet_id, scope) {
                    (true, true, None, HardcopyScope::AllSheetsOrPanes)
                    | (false, true, Some(_), HardcopyScope::CurrentSheet)
                    | (
                        false,
                        false,
                        None,
                        HardcopyScope::Selection
                        | HardcopyScope::CurrentSheet
                        | HardcopyScope::ActiveDocument,
                    )
                    | (
                        false,
                        true,
                        None,
                        HardcopyScope::Selection | HardcopyScope::ActiveDocument,
                    ) => {}
                    _ => {
                        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                            "schematic sheet selection and scope are inconsistent".to_owned(),
                        ));
                    }
                }
            }
            Self::Symbol {
                project_id,
                identity,
                scope,
                ..
            } => {
                validate_project_source_identity(*project_id, identity, "cell-view")?;
                if !matches!(scope, HardcopyScope::ActiveDocument) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "symbol worker source has an unsupported scope".to_owned(),
                    ));
                }
            }
            Self::Results {
                source_key,
                project_id,
                presentation,
                scope,
                ..
            } => {
                validate_label("prepared result source key", source_key, SOURCE_KEY_LIMIT)?;
                presentation.validate()?;
                if !matches!(
                    scope,
                    HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
                ) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "result worker source has an unsupported scope".to_owned(),
                    ));
                }
                require_project_source_prefix(*project_id, source_key, "result-dataset")?;
            }
            Self::Studio {
                source_key,
                project_id,
                pane_id,
                all_panes,
                scope,
                ..
            } => {
                validate_label("prepared studio source key", source_key, SOURCE_KEY_LIMIT)?;
                let expected_key = format!(
                    "project:{}:visualization-pane:{pane_id}",
                    project_id.as_uuid()
                );
                if source_key != &expected_key {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "studio pane identity does not match its source key".to_owned(),
                    ));
                }
                if (*all_panes && !matches!(scope, HardcopyScope::AllSheetsOrPanes))
                    || (!*all_panes
                        && !matches!(
                            scope,
                            HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
                        ))
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "studio aggregate flag and scope are inconsistent".to_owned(),
                    ));
                }
            }
            Self::Report {
                project_id,
                source_key,
                scope,
                ..
            } => {
                validate_label("prepared report source key", source_key, SOURCE_KEY_LIMIT)?;
                require_project_source_prefix(*project_id, source_key, "report")?;
                if !matches!(
                    scope,
                    HardcopyScope::CompleteReport | HardcopyScope::ActiveDocument
                ) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "report worker source has an unsupported scope".to_owned(),
                    ));
                }
            }
            Self::SourceSet {
                source_set,
                members,
            } => {
                if nested {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared source sets cannot nest".to_owned(),
                    ));
                }
                source_set.validate()?;
                if members.len() != source_set.members().len()
                    || members.is_empty()
                    || members.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared source-set members do not match its governed definition"
                            .to_owned(),
                    ));
                }
                for member in members {
                    member.validate_shape(true)?;
                }
            }
        }
        Ok(())
    }

    fn restore(self) -> Result<PreparedRetainedHardcopyPayload, HardcopySourceError> {
        self.validate_shape(false)?;
        let restored = match self {
            Self::Schematic {
                project_id,
                identity,
                schematic,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                all_sheets,
                scope,
            } => {
                let schematic = schematic
                    .restore::<PreparedSchematicOwner>("prepared schematic")?
                    .restore();
                let library_manager = library_manager
                    .restore::<crate::state::LibraryManager>("prepared symbol library")?;
                let schematic_buffers = schematic_buffers
                    .restore::<std::collections::BTreeMap<String, PreparedSchematicInterfaceOwner>>(
                        "prepared schematic symbol buffers",
                    )?
                    .into_iter()
                    .map(|(key, schematic)| (key, schematic.restore()))
                    .collect::<std::collections::HashMap<_, _>>();
                let sheet_catalog = sheet_catalog
                    .map(|catalog| catalog.restore::<SheetCatalog>("prepared sheet catalog"))
                    .transpose()?;
                validate_prepared_schematic_identity(
                    project_id,
                    &identity,
                    sheet_catalog.as_ref(),
                    sheet_id,
                )?;
                PreparedRetainedHardcopyPayload::Schematic {
                    project_id,
                    identity,
                    schematic,
                    library_manager,
                    schematic_buffers,
                    sheet_catalog,
                    sheet_id,
                    all_sheets,
                    scope,
                }
            }
            Self::Symbol {
                project_id,
                identity,
                document,
                scope,
            } => {
                validate_prepared_base_design_identity(project_id, &identity)?;
                PreparedRetainedHardcopyPayload::Symbol {
                    project_id,
                    identity,
                    document: document.restore::<SymbolDocument>("prepared symbol document")?,
                    scope,
                }
            }
            Self::Results {
                source_key,
                project_id,
                simulation_results,
                presentation,
                scope,
            } => {
                let simulation_results = simulation_results
                    .restore::<ProjectSimulationResults>("prepared result history")?;
                let simulation = simulation_results
                    .into_simulation_state()
                    .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                if simulation.runs.len() != 1 || simulation.runs[0].analyses.len() != 1 {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared result history must contain exactly one run and analysis"
                            .to_owned(),
                    ));
                }
                let run = simulation.runs.into_iter().next().ok_or_else(|| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared result history lost its run".to_owned(),
                    )
                })?;
                let expected_key = format!(
                    "project:{}:result-dataset:{}",
                    project_id.as_uuid(),
                    run.dataset_id
                );
                if source_key != expected_key {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "result dataset identity does not match its source key".to_owned(),
                    ));
                }
                PreparedRetainedHardcopyPayload::Results {
                    source_key,
                    project_id,
                    run,
                    presentation: presentation.restore()?,
                    scope,
                }
            }
            Self::Studio {
                source_key,
                project_id,
                studio,
                simulation_results,
                pane_id,
                all_panes,
                scope,
            } => {
                let studio =
                    studio.restore::<VisualizationStudioState>("prepared visualization studio")?;
                let simulation = simulation_results
                    .restore::<ProjectSimulationResults>("prepared studio result history")?
                    .into_simulation_state()
                    .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                validate_prepared_studio_snapshot(
                    project_id,
                    &source_key,
                    &studio,
                    &simulation,
                    pane_id,
                    all_panes,
                )?;
                PreparedRetainedHardcopyPayload::Studio {
                    source_key,
                    project_id,
                    studio,
                    simulation,
                    pane_id,
                    all_panes,
                    scope,
                }
            }
            Self::Report {
                project_id,
                source_key,
                document,
                scope,
            } => {
                let document = document.restore::<ReportDocument>("prepared report document")?;
                let expected_key =
                    format!("project:{}:report:{}", project_id.as_uuid(), document.id());
                if source_key != expected_key {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "report document identity does not match its source key".to_owned(),
                    ));
                }
                PreparedRetainedHardcopyPayload::Report {
                    project_id,
                    source_key,
                    document,
                    scope,
                }
            }
            Self::SourceSet {
                source_set,
                members,
            } => {
                let members = members
                    .into_iter()
                    .map(|member| {
                        member
                            .restore()
                            .map(|payload| PreparedRetainedHardcopyResolution { payload })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                validate_prepared_source_set_members(&source_set, &members)?;
                PreparedRetainedHardcopyPayload::SourceSet {
                    source_set,
                    members,
                }
            }
        };
        Ok(restored)
    }
}

fn validate_optional_label(
    field: &'static str,
    value: Option<&str>,
    maximum_bytes: usize,
) -> Result<(), HardcopySourceError> {
    if let Some(value) = value {
        validate_label(field, value, maximum_bytes)?;
    }
    Ok(())
}

fn require_project_source_prefix<'a>(
    project_id: ProjectId,
    source_key: &'a str,
    family: &'static str,
) -> Result<&'a str, HardcopySourceError> {
    let prefix = format!("project:{}:{family}:", project_id.as_uuid());
    source_key
        .strip_prefix(&prefix)
        .filter(|tail| !tail.is_empty())
        .ok_or_else(|| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{family} source key does not belong to its captured project"
            ))
        })
}

fn validate_project_source_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
    family: &'static str,
) -> Result<(), HardcopySourceError> {
    validate_label(
        "prepared source key",
        &identity.source_key,
        SOURCE_KEY_LIMIT,
    )?;
    validate_label(
        "prepared source display name",
        &identity.display_name,
        DISPLAY_NAME_LIMIT,
    )?;
    require_project_source_prefix(project_id, &identity.source_key, family)?;
    Ok(())
}

fn prepared_base_design_document_id(
    project_id: ProjectId,
    view_key: &str,
) -> Result<HardcopyDocumentId, HardcopySourceError> {
    let mut identity_material = b"rspice-cell-view-hardcopy-v1:".to_vec();
    identity_material.extend_from_slice(view_key.as_bytes());
    HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_material))
        .map_err(|error| HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string()))
}

fn validate_prepared_base_design_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
) -> Result<(), HardcopySourceError> {
    let view_key = require_project_source_prefix(project_id, &identity.source_key, "cell-view")?;
    if view_key.contains(":sheet:") {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "base design source unexpectedly names a sheet".to_owned(),
        ));
    }
    let expected = prepared_base_design_document_id(project_id, view_key)?;
    if identity.document_id != expected {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "design document identity is not derived from its project and cell view".to_owned(),
        ));
    }
    Ok(())
}

fn validate_prepared_schematic_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
    sheet_catalog: Option<&SheetCatalog>,
    sheet_id: Option<SheetId>,
) -> Result<(), HardcopySourceError> {
    let qualified = require_project_source_prefix(project_id, &identity.source_key, "cell-view")?;
    match sheet_id {
        None => validate_prepared_base_design_identity(project_id, identity),
        Some(sheet_id) => {
            let suffix = format!(":sheet:{sheet_id}");
            let view_key = qualified.strip_suffix(&suffix).ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet identity does not match its source key".to_owned(),
                )
            })?;
            if view_key.is_empty() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet source has an empty cell-view key".to_owned(),
                ));
            }
            let catalog = sheet_catalog.ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet source has no governed catalog".to_owned(),
                )
            })?;
            catalog.validate().map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            let sheet = catalog.find(sheet_id).ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "schematic sheet {sheet_id} is absent from its governed catalog"
                ))
            })?;
            let base_document_id = prepared_base_design_document_id(project_id, view_key)?;
            let mut identity_material = b"rspice-hardcopy-schematic-sheet-v1:".to_vec();
            identity_material.extend_from_slice(sheet_id.as_uuid().as_bytes());
            let expected_document_id = HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
                &base_document_id.as_uuid(),
                &identity_material,
            ))
            .map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            let expected_revision = ObjectRevision::new(sheet.revision()).map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            if identity.document_id != expected_document_id
                || identity.revision != expected_revision
            {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet document identity or revision is stale".to_owned(),
                ));
            }
            Ok(())
        }
    }
}

fn validate_prepared_studio_snapshot(
    project_id: ProjectId,
    source_key: &str,
    studio: &VisualizationStudioState,
    simulation: &SimulationState,
    pane_id: u64,
    all_panes: bool,
) -> Result<(), HardcopySourceError> {
    let expected_key = format!(
        "project:{}:visualization-pane:{pane_id}",
        project_id.as_uuid()
    );
    if source_key != expected_key {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "studio pane identity does not match its captured project".to_owned(),
        ));
    }
    if studio.panes.is_empty() || studio.panes.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared studio pane count is outside the governed boundary".to_owned(),
        ));
    }
    if !all_panes
        && (studio.panes.len() != 1
            || studio.panes[0].id != pane_id
            || studio.active_pane != Some(pane_id))
    {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "single-pane studio snapshot contains unrelated pane state".to_owned(),
        ));
    }
    if all_panes && !studio.panes.iter().any(|pane| pane.id == pane_id) {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "aggregate studio snapshot lost its selected pane".to_owned(),
        ));
    }
    let mut pane_ids = std::collections::HashSet::new();
    let expected_analyses = studio
        .panes
        .iter()
        .map(|pane| {
            if !pane_ids.insert(pane.id) {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "duplicate prepared studio pane {}",
                    pane.id
                )));
            }
            Ok((pane.dataset_id, pane.analysis_sequence))
        })
        .collect::<Result<std::collections::HashSet<_>, _>>()?;
    let actual_analyses = simulation
        .runs
        .iter()
        .flat_map(|run| {
            run.analyses
                .iter()
                .map(move |analysis| (run.dataset_id, analysis.id))
        })
        .collect::<std::collections::HashSet<_>>();
    if expected_analyses != actual_analyses {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "studio result history is not the exact pane-owned analysis set".to_owned(),
        ));
    }
    Ok(())
}

fn prepared_payload_identity(
    prepared: &PreparedRetainedHardcopyResolution,
) -> Result<(HardcopySourceIdentity, HardcopyScope), HardcopySourceError> {
    match &prepared.payload {
        PreparedRetainedHardcopyPayload::Schematic {
            identity, scope, ..
        }
        | PreparedRetainedHardcopyPayload::Symbol {
            identity, scope, ..
        } => Ok((identity.clone(), scope.clone())),
        PreparedRetainedHardcopyPayload::Results {
            source_key,
            project_id,
            run,
            presentation,
            scope,
        } => {
            let analysis = run.analyses.first().ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "prepared result lost its analysis".to_owned(),
                )
            })?;
            Ok((
                results_quick_view_identity(
                    source_key,
                    *project_id,
                    presentation.viewer,
                    run,
                    analysis,
                )?,
                scope.clone(),
            ))
        }
        PreparedRetainedHardcopyPayload::Studio {
            source_key,
            project_id,
            studio,
            pane_id,
            scope,
            ..
        } => {
            let pane = studio
                .panes
                .iter()
                .find(|pane| pane.id == *pane_id)
                .ok_or_else(|| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared studio source lost its selected pane".to_owned(),
                    )
                })?;
            Ok((
                studio_source_identity(source_key, *project_id, studio, pane)?,
                scope.clone(),
            ))
        }
        PreparedRetainedHardcopyPayload::Report {
            source_key,
            document,
            scope,
            ..
        } => Ok((
            HardcopySourceIdentity::try_new(
                source_key,
                HardcopyDocumentId::try_from_uuid(document.id().as_uuid()).map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?,
                document.revision(),
                document.title(),
            )?,
            scope.clone(),
        )),
        PreparedRetainedHardcopyPayload::SourceSet { .. } => {
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "prepared source sets cannot nest".to_owned(),
            ))
        }
    }
}

fn validate_prepared_source_set_members(
    source_set: &HardcopySourceSet,
    members: &[PreparedRetainedHardcopyResolution],
) -> Result<(), HardcopySourceError> {
    source_set.validate()?;
    if source_set.members().len() != members.len() {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared source-set member count changed during transfer".to_owned(),
        ));
    }
    for (expected, prepared) in source_set.members().iter().zip(members) {
        let (identity, scope) = prepared_payload_identity(prepared)?;
        if identity.source_key != expected.source_key()
            || identity.display_name != expected.display_name()
            || identity.document_id != expected.document_id()
            || identity.revision != expected.revision()
            || &scope != expected.scope()
        {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "prepared source-set member `{}` is stale or belongs to another owner",
                expected.source_key()
            )));
        }
    }
    Ok(())
}

impl PreparedRetainedHardcopyResolution {
    /// Serialize the exact prepared owner snapshot for a browser dedicated
    /// worker. Consuming `self` avoids cloning large retained result arrays.
    /// The returned bytes are bounded and authenticated as one atomic unit.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn to_worker_snapshot_json(self) -> Result<Vec<u8>, HardcopySourceError> {
        let snapshot = PreparedRetainedHardcopyWorkerSnapshot::capture(self)?;
        let bytes = serde_json::to_vec(&snapshot)
            .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(
                bytes.len(),
            ));
        }
        Ok(bytes)
    }

    /// Deserialize a dedicated-worker request only after its byte boundary,
    /// closed schema, transport digest, owner schemas, and source identities
    /// all validate. No partially restored source can escape on failure.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn from_worker_snapshot_json(bytes: &[u8]) -> Result<Self, HardcopySourceError> {
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(
                bytes.len(),
            ));
        }
        if bytes.is_empty() {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "worker request is empty".to_owned(),
            ));
        }
        let snapshot: PreparedRetainedHardcopyWorkerSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
        snapshot.into_prepared()
    }

    pub(crate) fn resolve_owned(self) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
        match self.payload {
            PreparedRetainedHardcopyPayload::Schematic {
                project_id: _,
                identity,
                schematic,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                all_sheets,
                scope,
            } => {
                let resolver = SymbolResolver::new(&library_manager, &schematic_buffers);
                if all_sheets {
                    let catalog = sheet_catalog.as_ref().ok_or_else(|| {
                        HardcopySourceError::InvalidSheetPartition(
                            "prepared all-sheets source lost its catalog".to_owned(),
                        )
                    })?;
                    return resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
                        identity,
                        schematic: &schematic,
                        expected_topology_version: schematic.topology_version(),
                        symbol_resolver: Some(&resolver),
                        sheet_catalog: catalog,
                    });
                }
                if let (Some(catalog), Some(sheet_id)) = (sheet_catalog.as_ref(), sheet_id)
                    && !schematic_has_objects_on_sheet(&schematic, catalog, sheet_id)
                {
                    return resolve_blank_schematic_sheet(identity, scope);
                }
                resolve_schematic_source(SchematicHardcopySource {
                    identity,
                    schematic: &schematic,
                    expected_topology_version: schematic.topology_version(),
                    symbol_resolver: Some(&resolver),
                    sheet_catalog: sheet_catalog.as_ref(),
                    sheet_id,
                    scope,
                })
            }
            PreparedRetainedHardcopyPayload::Symbol {
                project_id: _,
                identity,
                document,
                scope,
            } => resolve_symbol_source(SymbolHardcopySource {
                identity,
                document: &document,
                selection: None,
                scope,
            }),
            PreparedRetainedHardcopyPayload::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
                let analysis = run.analyses.first().ok_or_else(|| {
                    HardcopySourceError::UnretainedResult(
                        "prepared result lost its exact analysis".to_owned(),
                    )
                })?;
                if !run.lifecycle.is_terminal() || !analysis.success {
                    return Err(HardcopySourceError::UnretainedResult(
                        "prepared result is not terminal and successful".to_owned(),
                    ));
                }
                analysis
                    .validate_retained_evidence()
                    .map_err(HardcopySourceError::InvalidVisualizationSource)?;
                resolve_results_quick_view_parts(
                    source_key,
                    project_id,
                    scope,
                    ActiveQuickResult {
                        run: &run,
                        analysis,
                    },
                    &presentation,
                )
            }
            PreparedRetainedHardcopyPayload::Studio {
                source_key,
                project_id,
                studio,
                simulation,
                pane_id,
                all_panes,
                scope,
            } => {
                if all_panes {
                    let mut resolved = resolve_all_studio_panes(project_id, &studio, &simulation)?;
                    resolved.source_key = source_key;
                    Ok(resolved)
                } else {
                    resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
                        source_key,
                        project_id,
                        studio: &studio,
                        simulation: &simulation,
                        pane_id,
                        scope,
                    })
                }
            }
            PreparedRetainedHardcopyPayload::Report {
                project_id: _,
                source_key,
                document,
                scope,
            } => resolve_report_source(ReportHardcopySource {
                source_key,
                document: &document,
                reference_inventory: None,
                scope,
            }),
            PreparedRetainedHardcopyPayload::SourceSet {
                source_set,
                members,
            } => {
                let mut members = members.into_iter();
                resolve_hardcopy_source_set_with(&source_set, |_| {
                    members
                        .next()
                        .ok_or_else(|| {
                            HardcopySourceError::InvalidSourceSet(
                                "prepared source set lost an ordered member".to_owned(),
                            )
                        })?
                        .resolve_owned()
                })
            }
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

/// Resolve a persisted source set against the currently retained application
/// authorities. Members are processed in definition order and the operation
/// returns no partial aggregate if any member is missing, stale, or invalid.
#[cfg(test)]
pub(crate) fn resolve_retained_hardcopy_source_set(
    state: &AppState,
    source_set: &HardcopySourceSet,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_hardcopy_source_set_with(source_set, |member| {
        resolve_retained_hardcopy_source(state, member.source_key(), member.scope().clone())
    })
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

/// Resolve an active Visualization Studio pane without depending on a window,
/// screenshot, framebuffer, or transient viewer cache.
pub fn resolve_visualization_pane_source(
    source: VisualizationPaneHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let scene = resolve_cartesian_line_scene(
        source.document,
        source.reference,
        source.page_id,
        source.pane_id,
    )
    .map_err(map_visualization_error)?;
    let mut resolved = resolve_plot_source(PlotHardcopySource {
        source_key: source.source_key,
        display_name: source.display_name,
        scene: &scene,
        scope: source.scope,
    })?;
    let HardcopySemanticDocument::Plot(plot) = &mut resolved.semantic_document else {
        unreachable!("plot source resolver always returns plot semantics")
    };
    plot.markers = source
        .document
        .markers()
        .iter()
        .filter(|marker| marker.pane_id == source.pane_id)
        .map(|marker| canonical_marker_semantics(&scene, marker))
        .collect();
    plot.annotations = source
        .document
        .annotations()
        .iter()
        .filter(|annotation| annotation.pane_id == source.pane_id)
        .map(|annotation| canonical_annotation_semantics(&scene, annotation))
        .collect();
    let content_digest = canonical_digest(
        b"rspice-hardcopy-visualization-pane-v2",
        &(scene.source_digest(), &resolved.semantic_document),
    )?;
    let document_id = resolved.authority.document_id();
    let revision = resolved.authority.revision();
    let display_name = resolved.authority.display_name().to_owned();
    let document_kind = resolved.authority.document_kind();
    let scope = resolved.authority.scope().clone();
    resolved.authority = ActiveHardcopySource::try_new(
        document_id,
        revision,
        content_digest,
        display_name,
        document_kind,
        scope,
    )
    .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?;
    resolved.default_print_mapping = default_print_mapping(&resolved.semantic_document)?;
    Ok(resolved)
}

pub(crate) fn resolve_all_studio_panes(
    project_id: ProjectId,
    studio: &VisualizationStudioState,
    simulation: &SimulationState,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if studio.panes.is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-panes scope requires at least one retained pane".to_owned(),
        ));
    }
    let mut resolved_panes = Vec::with_capacity(studio.panes.len());
    for pane in &studio.panes {
        resolved_panes.push(resolve_active_studio_pane_source(
            ActiveStudioPaneHardcopySource {
                source_key: format!(
                    "project:{}:visualization-pane:{}",
                    project_id.as_uuid(),
                    pane.id
                ),
                project_id,
                studio,
                simulation,
                pane_id: pane.id,
                scope: HardcopyScope::ActivePlotDocument,
            },
        )?);
    }
    let members = resolved_panes
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &project_id.as_uuid(),
            b"rspice-hardcopy-all-visualization-panes-v1",
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio.revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        "All visualization panes",
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_panes = resolved_panes.into_iter();
    resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_panes.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })
}

/// Resolve the exact active Visualization Studio pane directly from its
/// retained simulation dataset. This closes the application integration gap
/// without manufacturing a report reference or consulting the currently
/// rendered plot widget.
pub(crate) fn resolve_active_studio_pane_source(
    source: ActiveStudioPaneHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let pane_id = source.pane_id;
    let panes = source
        .studio
        .panes
        .iter()
        .filter(|pane| pane.id == pane_id)
        .collect::<Vec<_>>();
    let [pane] = panes.as_slice() else {
        return if panes.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "active pane {pane_id} is not retained"
            )))
        } else {
            Err(HardcopySourceError::AmbiguousActiveSource(format!(
                "visualization pane {pane_id}"
            )))
        };
    };
    if is_curve_viewer(pane.viewer) && source.studio.family_policies.contains_key(&pane.id) {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "active family presentation requires its exact resolved family slice".to_owned(),
        ));
    }
    if is_curve_viewer(pane.viewer)
        && source.studio.autoscale == VisualizationAutoscale::SpecificationBounds
    {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "specification-bound autoscale requires the active project specification authority"
                .to_owned(),
        ));
    }

    let runs = source
        .simulation
        .runs
        .iter()
        .filter(|run| run.dataset_id == pane.dataset_id)
        .collect::<Vec<_>>();
    let [run] = runs.as_slice() else {
        return if runs.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "dataset {} is not retained",
                pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedDataset(
                pane.dataset_id.to_string(),
            ))
        };
    };
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "dataset {} belongs to a non-terminal run",
            pane.dataset_id
        )));
    }
    let analyses = run
        .analyses
        .iter()
        .filter(|analysis| analysis.id == pane.analysis_sequence)
        .collect::<Vec<_>>();
    let [analysis] = analyses.as_slice() else {
        return if analyses.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "analysis {} is not retained in dataset {}",
                pane.analysis_sequence, pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedAnalysis(
                pane.analysis_sequence,
            ))
        };
    };
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "analysis {} did not complete successfully",
            analysis.id
        )));
    }
    if !is_curve_viewer(pane.viewer) {
        return resolve_studio_result_summary(source, pane, run.run_id, analysis);
    }
    let visible = analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .collect::<Vec<_>>();
    if visible.is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no visible retained waveform".to_owned(),
        ));
    }
    for waveform in &visible {
        if waveform.x.is_empty()
            || waveform.x.len() != waveform.y.len()
            || waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .any(|value| !value.is_finite())
        {
            return Err(HardcopySourceError::InvalidRetainedWaveform(
                waveform.name.clone(),
            ));
        }
    }

    let source_x_minimum = visible
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let source_x_maximum = visible
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let (x_minimum, x_maximum) = source
        .studio
        .pane_x_ranges
        .get(&pane.id)
        .copied()
        .filter(|(minimum, maximum)| {
            minimum.is_finite() && maximum.is_finite() && minimum < maximum
        })
        .unwrap_or_else(|| nondegenerate_range(source_x_minimum, source_x_maximum));
    let source_y_minimum = visible
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let source_y_maximum = visible
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let (mut y_minimum, mut y_maximum) = nondegenerate_range(source_y_minimum, source_y_maximum);
    if source.studio.autoscale == VisualizationAutoscale::RobustVisible {
        let padding = ((y_maximum - y_minimum) * 0.05).max(f64::EPSILON);
        y_minimum -= padding;
        y_maximum += padding;
    }

    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let mut traces = Vec::with_capacity(visible.len());
    let mut trace_ids = std::collections::HashSet::new();
    for waveform in &visible {
        let trace_id = stable_trace_id(pane.dataset_id, analysis.id, &waveform.name);
        if !trace_ids.insert(trace_id) {
            return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
        }
        let source_points = waveform
            .x
            .iter()
            .copied()
            .zip(waveform.y.iter().copied())
            .collect::<Vec<_>>();
        traces.push(SemanticPlotTrace {
            trace_id,
            label: waveform.name.clone(),
            paths: clipped_plot_paths(
                &source_points,
                x_minimum,
                x_maximum,
                y_minimum,
                y_maximum,
                plot_width,
                plot_height,
            )?,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let markers = source
        .studio
        .markers
        .iter()
        .filter(|marker| {
            marker.dataset_id == pane.dataset_id
                && marker.analysis_sequence == pane.analysis_sequence
        })
        .map(|marker| {
            Ok(SemanticPlotMarker {
                marker_id: marker.id,
                label: marker.label.clone(),
                trace_id: Some(stable_trace_id(
                    marker.dataset_id,
                    marker.analysis_sequence,
                    &marker.waveform_name,
                )),
                source_x_bits: Some(marker.x.to_bits()),
                source_y_bits: Some(marker.y.to_bits()),
                position: Some(map_plot_point(
                    marker.x.clamp(x_minimum, x_maximum),
                    marker.y.clamp(y_minimum, y_maximum),
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let annotations = source
        .studio
        .annotations
        .iter()
        .filter(|annotation| {
            annotation.dataset_id == pane.dataset_id
                && annotation.analysis_sequence == pane.analysis_sequence
        })
        .map(|annotation| {
            Ok(SemanticPlotAnnotation {
                annotation_id: annotation.id,
                text: annotation.text.clone(),
                trace_id: None,
                source_x_bits: Some(annotation.x.to_bits()),
                source_y_bits: None,
                position: Some(map_plot_point(
                    annotation.x.clamp(x_minimum, x_maximum),
                    y_maximum,
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let semantic = SemanticPlot {
        viewer: pane.viewer,
        page_id: stable_page_id(&pane.page),
        pane_id: pane.id,
        traces,
        markers,
        annotations,
    };
    let digest = studio_pane_digest(
        source.studio,
        pane,
        run.run_id,
        analysis.id,
        &visible,
        &source
            .studio
            .markers
            .iter()
            .filter(|marker| {
                marker.dataset_id == pane.dataset_id
                    && marker.analysis_sequence == pane.analysis_sequence
            })
            .collect::<Vec<_>>(),
        &source
            .studio
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.dataset_id == pane.dataset_id
                    && annotation.analysis_sequence == pane.analysis_sequence
            })
            .collect::<Vec<_>>(),
    )?;
    let identity =
        studio_source_identity(&source.source_key, source.project_id, source.studio, pane)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

/// Resolve the exact result document currently selected in the ordinary
/// Results workspace. Specialized viewers read their durable analysis model
/// directly; table viewers read the selected immutable simulation result.
pub(crate) fn resolve_results_quick_view_source(
    source: ResultsQuickViewHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let presentation = ResultsQuickViewPresentation::from_state(source.state);
    let active = active_quick_result(source.state)?;
    resolve_results_quick_view_parts(
        source.source_key,
        source.project_id,
        source.scope,
        active,
        &presentation,
    )
}

fn resolve_results_quick_view_parts(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    active: ActiveQuickResult<'_>,
    presentation: &ResultsQuickViewPresentation,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let viewer = presentation.viewer;
    let semantic_document = match viewer {
        ResultViewer::Waves | ResultViewer::Bode => {
            HardcopySemanticDocument::Plot(quick_waveform_plot(active, viewer)?)
        }
        ResultViewer::Fft => HardcopySemanticDocument::Plot(quick_fft_plot(presentation, active)?),
        ResultViewer::Eye => HardcopySemanticDocument::Plot(quick_eye_plot(presentation, active)?),
        ResultViewer::Hist => {
            HardcopySemanticDocument::Plot(quick_histogram_plot(presentation, active)?)
        }
        ResultViewer::Nyquist => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Nyquist)?)
        }
        ResultViewer::Smith => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Smith)?)
        }
        ResultViewer::Op
        | ResultViewer::NoiseContrib
        | ResultViewer::Contribution
        | ResultViewer::TransferFunction
        | ResultViewer::Specs
        | ResultViewer::Table
        | ResultViewer::PoleZero => HardcopySemanticDocument::ResultSummary(
            semantic_result_summary(viewer, active.analysis)?,
        ),
    };
    let digest = canonical_digest(
        b"rspice-hardcopy-results-quick-view-v2",
        &(
            active.run.dataset_id,
            active.run.run_id,
            active.analysis.id,
            active.analysis.result_data_digest(),
            viewer,
            &semantic_document,
        ),
    )?;
    let identity =
        results_quick_view_identity(&source_key, project_id, viewer, active.run, active.analysis)?;
    let bounds = match &semantic_document {
        HardcopySemanticDocument::Plot(_) => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
        _ => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    };
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        bounds,
    )
}

#[derive(Debug)]
struct QuickResultSeries {
    identity: String,
    label: String,
    points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Copy)]
struct ActiveQuickResult<'a> {
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
}

fn active_quick_result(state: &AppState) -> Result<ActiveQuickResult<'_>, HardcopySourceError> {
    let run = active_terminal_run(state)?;
    let analysis_index = state.simulation.active_analysis_idx.ok_or_else(|| {
        HardcopySourceError::UnretainedResult(
            "no active analysis is selected in the active terminal dataset".to_owned(),
        )
    })?;
    let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "active analysis index {analysis_index} is not retained in dataset {}",
            run.dataset_id
        ))
    })?;
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active analysis {} did not complete successfully",
            analysis.id
        )));
    }
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    Ok(ActiveQuickResult { run, analysis })
}

fn quick_waveform_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(viewer, "Results", 0, series)
}

fn active_terminal_run(state: &AppState) -> Result<&SimulationRun, HardcopySourceError> {
    let run = state.simulation.active_run().ok_or_else(|| {
        HardcopySourceError::UnretainedResult("no active result dataset is selected".to_owned())
    })?;
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active dataset {} belongs to a non-terminal run",
            run.dataset_id
        )));
    }
    Ok(run)
}

fn quick_fft_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "FFT source waveform",
    )?;
    let input = crate::analysis::fft::prepare_fft_input_with_options(
        &waveform.name,
        &waveform.x,
        &waveform.y,
        presentation.fft.input_options_for_waveform(&waveform.x),
    )
    .ok_or(HardcopySourceError::MissingViewerEvidence(
        "FFT source waveform",
    ))?;
    let data = crate::analysis::fft::data::FftData::from_time_domain_with_normalization(
        &format!("FFT({})", waveform.name),
        &input.samples,
        input.sample_rate,
        presentation.fft.window,
        presentation.fft.normalization,
    );
    if data.points.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("FFT spectrum"));
    }
    quick_plot_from_series(
        ResultViewer::Fft,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:fft:{}",
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                data.fft_size
            ),
            label: data.name.clone(),
            points: data
                .points
                .iter()
                .map(|point| (point.frequency, point.magnitude))
                .collect(),
        }],
    )
}

fn quick_eye_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "eye source waveform",
    )?;
    let bit_period = retained_eye_bit_period(&waveform.x, &waveform.y)?;
    let data = crate::analysis::eye_diagram::EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(2)
        .build(&waveform.x, &waveform.y);
    if data.traces.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("eye diagram"));
    }
    let series = data
        .traces
        .iter()
        .enumerate()
        .map(|(index, trace)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:eye:{}:{index}",
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                bit_period.to_bits()
            ),
            label: format!("Eye trace {}", index + 1),
            points: trace
                .time
                .iter()
                .copied()
                .zip(trace.amplitude.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(ResultViewer::Eye, "Results", 0, series)
}

fn quick_histogram_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } =
        active.analysis.family_metadata.as_ref().ok_or(
            HardcopySourceError::MissingViewerEvidence("Monte Carlo family metadata"),
        )?
    else {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo family metadata",
        ));
    };
    let variable = variables.get(presentation.histogram_selected).ok_or(
        HardcopySourceError::MissingViewerEvidence("selected Monte Carlo variable"),
    )?;
    if variable.samples.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo samples",
        ));
    }
    let mut builder = crate::analysis::HistogramBuilder::new()
        .name(&variable.name)
        .bin_count(presentation.histogram_bin_count.clamp(1, 1000));
    if presentation.histogram_custom_range {
        let minimum = presentation.histogram_custom_min;
        let maximum = presentation.histogram_custom_max;
        if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
            return Err(HardcopySourceError::InvalidResultRange);
        }
        builder = builder.range(minimum, maximum);
    }
    let histogram = builder.build(&variable.samples);
    let ordinates = match presentation.histogram_mode {
        crate::analysis::HistogramDisplayMode::Count => histogram
            .bins
            .iter()
            .map(|bin| bin.count as f64)
            .collect::<Vec<_>>(),
        crate::analysis::HistogramDisplayMode::Pdf => histogram.pdf(),
        crate::analysis::HistogramDisplayMode::Cdf => histogram.cdf(),
        crate::analysis::HistogramDisplayMode::Percent => histogram
            .bins
            .iter()
            .map(|bin| {
                if histogram.total_count == 0 {
                    0.0
                } else {
                    bin.count as f64 * 100.0 / histogram.total_count as f64
                }
            })
            .collect(),
    };
    quick_plot_from_series(
        ResultViewer::Hist,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:monte-carlo:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, variable.name
            ),
            label: histogram.name.clone(),
            points: histogram
                .bins
                .iter()
                .zip(ordinates)
                .map(|(bin, ordinate)| (bin.center(), ordinate))
                .collect(),
        }],
    )
}

fn quick_complex_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .filter_map(|waveform| waveform.complex.as_ref().map(|complex| (waveform, complex)))
        .map(|(waveform, complex)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:complex",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: complex
                .real
                .iter()
                .copied()
                .zip(complex.imag.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(viewer, "Results", 0, series)
}

fn selected_retained_waveform<'a>(
    active: ActiveQuickResult<'a>,
    preferred_name: Option<&str>,
    evidence: &'static str,
) -> Result<&'a WaveformData, HardcopySourceError> {
    let mut candidates = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            waveform.x.len().min(waveform.y.len()) >= crate::analysis::fft::MIN_FFT_SAMPLES
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.name.cmp(&right.name));
    let selected = preferred_name
        .and_then(|name| {
            candidates
                .iter()
                .copied()
                .find(|waveform| waveform.name == name || waveform.name.eq_ignore_ascii_case(name))
                .or_else(|| {
                    let preferred_core = derived_waveform_source_core(name);
                    candidates.iter().copied().find(|waveform| {
                        derived_waveform_source_core(&waveform.name) == preferred_core
                    })
                })
        })
        .or_else(|| candidates.first().copied())
        .ok_or(HardcopySourceError::MissingViewerEvidence(evidence))?;
    let sample_count = selected.x.len().min(selected.y.len());
    if selected
        .x
        .iter()
        .take(sample_count)
        .chain(selected.y.iter().take(sample_count))
        .any(|value| !value.is_finite())
    {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            selected.name.clone(),
        ));
    }
    Ok(selected)
}

fn derived_waveform_source_core(name: &str) -> String {
    let trimmed = name.trim().trim_matches('|');
    trimmed
        .strip_prefix("V(")
        .and_then(|value| value.strip_suffix(')'))
        .or_else(|| {
            trimmed
                .strip_prefix("I(")
                .and_then(|value| value.strip_suffix(')'))
        })
        .unwrap_or(trimmed)
        .trim()
        .to_ascii_lowercase()
}

fn retained_eye_bit_period(time: &[f64], values: &[f64]) -> Result<f64, HardcopySourceError> {
    let sample_count = time.len().min(values.len());
    if sample_count < 8 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let minimum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    let maximum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let threshold = (minimum + maximum) * 0.5;
    let edges = crate::analysis::eye_diagram::find_edges(
        &time[..sample_count],
        &values[..sample_count],
        threshold,
    );
    if edges.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut rising_times = edges
        .iter()
        .filter(|edge| edge.rising && edge.time.is_finite())
        .map(|edge| edge.time)
        .collect::<Vec<_>>();
    rising_times.sort_by(f64::total_cmp);
    let edge_times = if rising_times.len() >= 3 {
        rising_times
    } else {
        let mut all = edges
            .iter()
            .map(|edge| edge.time)
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        all.sort_by(f64::total_cmp);
        all
    };
    if edge_times.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut intervals = edge_times
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|interval| interval.is_finite() && *interval > 0.0)
        .collect::<Vec<_>>();
    if intervals.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    intervals.sort_by(f64::total_cmp);
    let period = intervals[intervals.len() / 2];
    if period.is_finite() && period > 0.0 {
        Ok(period)
    } else {
        Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))
    }
}

fn quick_plot_from_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series",
        ));
    }
    if series.iter().any(|series| {
        series.points.is_empty()
            || series
                .points
                .iter()
                .any(|(x, y)| !x.is_finite() || !y.is_finite())
    }) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series".to_owned(),
        ));
    }
    let x_minimum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.0))
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let x_maximum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.0))
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let y_minimum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.1))
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let y_maximum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.1))
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let (x_minimum, x_maximum) = nondegenerate_range(x_minimum, x_maximum);
    let (y_minimum, y_maximum) = nondegenerate_range(y_minimum, y_maximum);
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let mut trace_ids = std::collections::HashSet::new();
    let traces = series
        .into_iter()
        .enumerate()
        .map(|(index, series)| {
            let trace_id = stable_quick_trace_id(viewer, index, &series.identity);
            if !trace_ids.insert(trace_id) {
                return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
            }
            Ok(SemanticPlotTrace {
                trace_id,
                label: series.label,
                paths: clipped_plot_paths(
                    &series.points,
                    x_minimum,
                    x_maximum,
                    y_minimum,
                    y_maximum,
                    plot_width,
                    plot_height,
                )?,
                source_samples: series
                    .points
                    .iter()
                    .map(|(x, y)| (x.to_bits(), y.to_bits()))
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    Ok(SemanticPlot {
        viewer,
        page_id: stable_page_id(page),
        pane_id,
        traces,
        markers: Vec::new(),
        annotations: Vec::new(),
    })
}

fn results_quick_view_identity(
    source_key: &str,
    project_id: ProjectId,
    viewer: ResultViewer,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = source_key.as_bytes().to_vec();
    identity_name.extend_from_slice(viewer.label().as_bytes());
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&analysis.id.to_be_bytes());
    identity_name.extend_from_slice(analysis.result_data_digest().as_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {}", viewer.label()),
    )
}

fn stable_quick_trace_id(viewer: ResultViewer, index: usize, identity: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-hardcopy-results-trace-v1");
    hasher.update(viewer.label().as_bytes());
    hasher.update((index as u64).to_be_bytes());
    hasher.update(identity.as_bytes());
    let bytes: [u8; 8] = hasher.finalize()[..8]
        .try_into()
        .expect("SHA-256 prefix has fixed length");
    u64::from_be_bytes(bytes)
}

const fn is_curve_viewer(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::Bode
            | ResultViewer::Fft
            | ResultViewer::Eye
            | ResultViewer::Hist
            | ResultViewer::Nyquist
            | ResultViewer::Smith
    )
}

fn resolve_studio_result_summary(
    source: ActiveStudioPaneHardcopySource<'_>,
    pane: &StudioPane,
    run_id: RunId,
    analysis: &AnalysisResult,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let summary = semantic_result_summary(pane.viewer, analysis)?;
    let digest = canonical_digest(
        b"rspice-hardcopy-studio-result-summary-v1",
        &(source.studio.revision, pane, run_id, analysis.id, &summary),
    )?;
    let identity =
        studio_source_identity(&source.source_key, source.project_id, source.studio, pane)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::ResultSummary(summary),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    )
}

fn semantic_result_summary(
    viewer: ResultViewer,
    analysis: &AnalysisResult,
) -> Result<SemanticResultSummary, HardcopySourceError> {
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    let mut tables = Vec::new();
    match viewer {
        ResultViewer::Op => {
            if let Some(operating_point) = &analysis.dc_op {
                for (title, values) in [
                    ("Node voltages", &operating_point.node_voltages),
                    ("Branch currents", &operating_point.branch_currents),
                    ("Device power", &operating_point.power_dissipation),
                ] {
                    if !values.is_empty() {
                        tables.push(SemanticTable {
                            title: title.to_owned(),
                            columns: vec![
                                "Quantity".to_owned(),
                                "Value".to_owned(),
                                "Unit".to_owned(),
                            ],
                            rows: values
                                .iter()
                                .map(|value| {
                                    vec![
                                        value.name.clone(),
                                        exact_number(value.value),
                                        value.unit.clone(),
                                    ]
                                })
                                .collect(),
                        });
                    }
                }
            }
            if tables.is_empty()
                && !matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::OperatingPoint { .. })
                )
            {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "operating point",
                ));
            }
        }
        ResultViewer::NoiseContrib => {
            let summary = analysis.noise_summary.as_ref().ok_or(
                HardcopySourceError::MissingViewerEvidence("noise contributor summary"),
            )?;
            tables.push(SemanticTable {
                title: format!(
                    "Noise contributors · {} Hz to {} Hz",
                    exact_number(summary.band.0),
                    exact_number(summary.band.1)
                ),
                columns: vec![
                    "Device".to_owned(),
                    "Mechanism".to_owned(),
                    "Power (V²)".to_owned(),
                    "Share (%)".to_owned(),
                ],
                rows: summary
                    .rows
                    .iter()
                    .map(|row| {
                        vec![
                            row.device.clone(),
                            row.mechanism.clone(),
                            exact_number(row.power),
                            exact_number(row.share_pct),
                        ]
                    })
                    .collect(),
            });
            tables.push(SemanticTable {
                title: "Integrated totals".to_owned(),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec![
                        "Output referred RMS".to_owned(),
                        summary
                            .total_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                    vec![
                        "Input referred RMS".to_owned(),
                        summary
                            .input_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                ],
            });
        }
        ResultViewer::Contribution => {
            let Some(AnalysisResultPayload::Sensitivity { output, rows, .. }) =
                &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("sensitivity"));
            };
            tables.push(SemanticTable {
                title: format!("Sensitivity of {output}"),
                columns: vec![
                    "Parameter".to_owned(),
                    "Raw".to_owned(),
                    "Normalized".to_owned(),
                ],
                rows: rows
                    .iter()
                    .map(|row| {
                        vec![
                            row.parameter.clone(),
                            exact_number(row.raw),
                            exact_number(row.normalized),
                        ]
                    })
                    .collect(),
            });
        }
        ResultViewer::TransferFunction => {
            let Some(AnalysisResultPayload::TransferFunction {
                input_source,
                output_expression,
                gain,
                input_resistance,
                output_resistance,
                ..
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "transfer function",
                ));
            };
            tables.push(SemanticTable {
                title: format!("{output_expression} / {input_source}"),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec!["Gain".to_owned(), format_optional_scalar(*gain)],
                    vec![
                        "Input resistance".to_owned(),
                        format_optional_scalar(*input_resistance),
                    ],
                    vec![
                        "Output resistance".to_owned(),
                        format_optional_scalar(*output_resistance),
                    ],
                ],
            });
        }
        ResultViewer::Specs => {
            if !analysis.measurements.is_empty() {
                tables.push(SemanticTable {
                    title: "Measurements and specifications".to_owned(),
                    columns: vec![
                        "Measurement".to_owned(),
                        "Value".to_owned(),
                        "Expected".to_owned(),
                        "Tolerance".to_owned(),
                        "Status".to_owned(),
                    ],
                    rows: analysis
                        .measurements
                        .iter()
                        .map(|measurement| {
                            vec![
                                measurement.name.clone(),
                                measurement
                                    .value
                                    .map_or_else(|| "not available".to_owned(), exact_number),
                                measurement
                                    .expected
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                measurement
                                    .tolerance
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                if measurement.passed { "pass" } else { "fail" }.to_owned(),
                            ]
                        })
                        .collect(),
                });
            } else if let Some(AnalysisResultPayload::ScalarMeasurements { values }) =
                &analysis.result_payload
            {
                tables.push(SemanticTable {
                    title: "Scalar measurements".to_owned(),
                    columns: vec!["Measurement".to_owned(), "Value".to_owned()],
                    rows: values
                        .iter()
                        .map(|(name, value)| vec![name.clone(), exact_number(*value)])
                        .collect(),
                });
            } else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "measurement/specification",
                ));
            }
        }
        ResultViewer::PoleZero => {
            let Some(AnalysisResultPayload::PoleZero { poles, zeros, gain }) =
                &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("pole-zero"));
            };
            let mut rows = Vec::with_capacity(poles.len() + zeros.len());
            rows.extend(poles.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("P{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            rows.extend(zeros.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("Z{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            tables.push(SemanticTable {
                title: format!("Pole-zero roots · gain {}", exact_number(*gain)),
                columns: vec!["Root".to_owned(), "Real".to_owned(), "Imaginary".to_owned()],
                rows,
            });
        }
        viewer if is_curve_viewer(viewer) => unreachable!("curve viewers resolve as plots"),
        viewer => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                viewer.label().to_owned(),
            ));
        }
    }
    Ok(SemanticResultSummary {
        viewer,
        title: analysis.label.clone(),
        tables,
        payload: analysis.result_payload.clone(),
    })
}

fn studio_source_identity(
    source_key: &str,
    project_id: ProjectId,
    studio: &VisualizationStudioState,
    pane: &StudioPane,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = Vec::with_capacity(24);
    identity_name.extend_from_slice(pane.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&pane.id.to_be_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio.revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        format!("{} · {}", pane.page, pane.viewer.label()),
    )
}

fn exact_number(value: f64) -> String {
    format!("{value:.17e}")
}

fn format_optional_scalar(scalar: Option<crate::state::TransferFunctionScalarEvidence>) -> String {
    scalar.map_or_else(
        || "not requested".to_owned(),
        |value| match value {
            crate::state::TransferFunctionScalarEvidence::Finite(value) => exact_number(value),
            crate::state::TransferFunctionScalarEvidence::PositiveInfinity => {
                "+infinity".to_owned()
            }
            crate::state::TransferFunctionScalarEvidence::NegativeInfinity => {
                "-infinity".to_owned()
            }
        },
    )
}

fn map_visualization_error(error: VisualizationRasterError) -> HardcopySourceError {
    match error {
        error @ (VisualizationRasterError::PageNotFound(_)
        | VisualizationRasterError::PaneNotFound(_)
        | VisualizationRasterError::DatasetNotFound(_)
        | VisualizationRasterError::EmptyTrace(_)
        | VisualizationRasterError::NoVisibleTraces) => {
            HardcopySourceError::UnretainedResult(error.to_string())
        }
        error => HardcopySourceError::InvalidVisualizationSource(error.to_string()),
    }
}

fn resolve_blank_schematic_sheet(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(scope, HardcopyScope::CurrentSheet) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    let digest = canonical_digest(b"rspice-hardcopy-blank-schematic-sheet-v1", &semantic)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        scope,
        HardcopySemanticDocument::Schematic(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(
                BLANK_SCHEMATIC_SHEET_WIDTH_UM,
                BLANK_SCHEMATIC_SHEET_HEIGHT_UM,
            ),
        )?,
    )
}

/// Resolve every governed schematic sheet in exact catalog order. Each sheet
/// is filtered independently before its authority is pinned into the
/// aggregate, so an assignment can never leak into a neighboring page.
pub fn resolve_all_schematic_sheets(
    source: SchematicSheetSetHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    source
        .sheet_catalog
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if source.sheet_catalog.sheets().is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-sheets scope requires at least one governed sheet".to_owned(),
        ));
    }
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }

    let aggregate_source_key = source.identity.source_key.clone();
    let mut resolved_sheets = Vec::with_capacity(source.sheet_catalog.sheets().len());
    for sheet in source.sheet_catalog.sheets() {
        let sheet_identity = schematic_sheet_identity(&source.identity, sheet)?;
        if schematic_has_objects_on_sheet(source.schematic, source.sheet_catalog, sheet.id()) {
            resolved_sheets.push(resolve_schematic_source(SchematicHardcopySource {
                identity: sheet_identity,
                schematic: source.schematic,
                expected_topology_version: source.expected_topology_version,
                symbol_resolver: source.symbol_resolver,
                sheet_catalog: Some(source.sheet_catalog),
                sheet_id: Some(sheet.id()),
                scope: HardcopyScope::CurrentSheet,
            })?);
        } else {
            resolved_sheets.push(resolve_blank_schematic_sheet(
                sheet_identity,
                HardcopyScope::CurrentSheet,
            )?);
        }
    }

    let members = resolved_sheets
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let mut set_identity_material = b"rspice-hardcopy-all-schematic-sheets-v1:".to_vec();
    set_identity_material.extend_from_slice(source.identity.document_id.as_uuid().as_bytes());
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &source.identity.document_id.as_uuid(),
            &set_identity_material,
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(source.sheet_catalog.revision())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        compact_display(
            &format!("{} · All sheets", source.identity.display_name),
            "All schematic sheets",
        ),
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_sheets = resolved_sheets.into_iter();
    let mut resolved = resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_sheets.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })?;
    // `AllSheetsOrPanes` is a transient scope of the owning design
    // descriptor, not a separately persisted named set.
    resolved.source_key = aggregate_source_key;
    Ok(resolved)
}

pub fn resolve_schematic_source(
    source: SchematicHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let selection_only = matches!(&source.scope, HardcopyScope::Selection);
    if selection_only && source.schematic.selection.is_empty() {
        return Err(HardcopySourceError::EmptySelection);
    }
    let governed_sheet = match (source.sheet_catalog, source.sheet_id) {
        (Some(catalog), Some(sheet_id)) if matches!(source.scope, HardcopyScope::CurrentSheet) => {
            catalog
                .validate()
                .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
            if catalog.find(sheet_id).is_none() {
                return Err(HardcopySourceError::InvalidSheetPartition(format!(
                    "sheet {sheet_id} is not retained in the supplied catalog"
                )));
            }
            Some((catalog, sheet_id))
        }
        (None, None) => None,
        _ => {
            return Err(HardcopySourceError::InvalidSheetPartition(
                "sheet catalog and sheet identity must be supplied together only for CurrentSheet"
                    .to_owned(),
            ));
        }
    };
    let object_is_in_scope = |object_id: u64| {
        governed_sheet.is_none_or(|(catalog, sheet_id)| {
            // The canvas's existing governed-sheet contract assigns legacy
            // unassigned objects to the active sheet. Reusing it here makes
            // every object publish on exactly one sheet without leakage.
            catalog
                .sheet_for_object(object_id)
                .or(catalog.active_sheet_id())
                == Some(sheet_id)
        })
    };

    let mut semantic = SemanticSchematic {
        view_path: governed_sheet.map_or_else(
            || source.identity.source_key.clone(),
            |(_, sheet_id)| format!("{}:sheet:{sheet_id}", source.identity.source_key),
        ),
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    for component in &source.schematic.components {
        if !object_is_in_scope(component.id) {
            continue;
        }
        if selection_only
            && !source
                .schematic
                .selection
                .components
                .contains(&component.id)
        {
            continue;
        }
        let (resolved_symbol, symbol_source) =
            resolve_component_symbol(component, source.symbol_resolver)?;
        semantic.components.push(SemanticComponent {
            component: component.clone(),
            resolved_symbol,
            symbol_source,
        });
    }
    let selected_wire_ids = || {
        source
            .schematic
            .selection
            .wires
            .iter()
            .copied()
            .chain(
                source
                    .schematic
                    .selection
                    .wire_segments
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .chain(
                source
                    .schematic
                    .selection
                    .wire_vertices
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .collect::<std::collections::HashSet<_>>()
    };
    let selected_wire_ids = selection_only.then(selected_wire_ids);
    semantic.wires.extend(
        source
            .schematic
            .wires
            .iter()
            .filter(|wire| {
                object_is_in_scope(wire.id)
                    && selected_wire_ids
                        .as_ref()
                        .is_none_or(|selected| selected.contains(&wire.id))
            })
            .cloned(),
    );
    semantic.buses.extend(
        source
            .schematic
            .buses
            .iter()
            .filter(|bus| {
                object_is_in_scope(bus.id)
                    && (!selection_only || source.schematic.selection.buses.contains(&bus.id))
            })
            .cloned(),
    );
    semantic.bus_taps.extend(
        source
            .schematic
            .bus_taps
            .iter()
            .filter(|tap| {
                object_is_in_scope(tap.id)
                    && (!selection_only || source.schematic.selection.bus_taps.contains(&tap.id))
            })
            .cloned(),
    );
    semantic.junctions.extend(
        source
            .schematic
            .junctions
            .iter()
            .filter(|junction| {
                object_is_in_scope(junction.id)
                    && (!selection_only
                        || source
                            .schematic
                            .selection
                            .junctions
                            .iter()
                            .any(|selection| selection.pos == junction.pos))
            })
            .copied(),
    );
    semantic.net_labels.extend(
        source
            .schematic
            .net_labels
            .iter()
            .filter(|label| {
                object_is_in_scope(label.id)
                    && (!selection_only
                        || source.schematic.selection.net_labels.contains(&label.id))
            })
            .cloned(),
    );
    semantic.design_notes.extend(
        source
            .schematic
            .design_notes
            .iter()
            .filter(|note| {
                object_is_in_scope(note.id)
                    && (!selection_only
                        || source.schematic.selection.design_notes.contains(&note.id))
            })
            .cloned(),
    );
    semantic.documentation_shapes.extend(
        source
            .schematic
            .documentation_shapes
            .iter()
            .filter(|shape| {
                object_is_in_scope(shape.id)
                    && (!selection_only
                        || source
                            .schematic
                            .selection
                            .documentation_shapes
                            .contains(&shape.id))
            })
            .cloned(),
    );
    if semantic_is_empty(&semantic) {
        return Err(HardcopySourceError::EmptyContent);
    }

    let bounds = schematic_bounds(&semantic)?;
    let digest = canonical_digest(b"rspice-hardcopy-schematic-v1", &semantic)?;
    finish_resolved(
        source.identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        source.scope,
        HardcopySemanticDocument::Schematic(semantic),
        bounds,
    )
}

pub fn resolve_symbol_source(
    source: SymbolHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let document = if matches!(&source.scope, HardcopyScope::Selection) {
        let selection = source
            .selection
            .ok_or(HardcopySourceError::EmptySelection)?;
        selected_symbol_document(source.document, selection)?
    } else {
        source.document.clone()
    };
    let bounds = symbol_bounds(&document)?;
    let digest = canonical_digest(b"rspice-hardcopy-symbol-v1", &document)?;
    finish_resolved(
        source.identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        source.scope,
        HardcopySemanticDocument::Symbol(document),
        bounds,
    )
}

pub fn resolve_plot_source(
    source: PlotHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    validate_label("display name", &source.display_name, DISPLAY_NAME_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    if source.scene.traces().is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no retained visible trace samples".to_owned(),
        ));
    }
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let x_range = source.scene.x_range();
    let y_range = source.scene.y_range();
    let x_span = x_range.maximum - x_range.minimum;
    let y_span = y_range.maximum - y_range.minimum;
    if !x_span.is_finite() || !y_span.is_finite() || x_span <= 0.0 || y_span <= 0.0 {
        return Err(HardcopySourceError::InvalidResultRange);
    }
    let mut traces = Vec::with_capacity(source.scene.traces().len());
    for trace in source.scene.traces() {
        if trace.points().is_empty() {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "visible trace `{}` has no retained samples",
                trace.label()
            )));
        }
        let source_points = trace
            .points()
            .iter()
            .map(|point| (point.x(), point.y()))
            .collect::<Vec<_>>();
        let paths = clipped_plot_paths(
            &source_points,
            x_range.minimum,
            x_range.maximum,
            y_range.minimum,
            y_range.maximum,
            plot_width,
            plot_height,
        )?;
        traces.push(SemanticPlotTrace {
            trace_id: trace.trace_id().get(),
            label: trace.label().to_owned(),
            paths,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let semantic = SemanticPlot {
        viewer: ResultViewer::Waves,
        page_id: source.scene.page_id().get(),
        pane_id: source.scene.pane_id().get(),
        traces,
        markers: Vec::new(),
        annotations: Vec::new(),
    };
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.scene.document_id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.scene.revision(),
        source.display_name,
    )?;
    finish_resolved(
        identity,
        source.scene.source_digest(),
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

fn validate_frozen_report_png(
    block_id: ReportBlockId,
    artifact: &FrozenReportArtifact,
) -> Result<(u32, u32), HardcopySourceError> {
    if artifact.media_type() != "image/png" {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "frozen plot figure",
            reason: format!(
                "unsupported artifact media type `{}`; expected image/png",
                artifact.media_type()
            ),
        });
    }
    let computed = ContentDigest::from_bytes(Sha256::digest(artifact.payload()).into());
    if computed != artifact.content_digest() {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} artifact digest does not authenticate its payload"
        )));
    }
    if artifact.payload().len() > crate::results::visualization_raster::MAX_RASTER_ARTIFACT_BYTES {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the PNG artifact byte limit"
        )));
    }
    if !png_has_exact_terminal_iend(artifact.payload()) {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has malformed chunks or bytes after IEND"
        )));
    }

    let decoder = png::Decoder::new(std::io::Cursor::new(artifact.payload()));
    let mut reader = decoder.read_info().map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} is not a valid PNG: {error}"
        ))
    })?;
    let header = reader.info();
    let width = header.width;
    let height = header.height;
    if !(crate::results::visualization_raster::MIN_RASTER_DIMENSION
        ..=crate::results::visualization_raster::MAX_RASTER_DIMENSION)
        .contains(&width)
        || !(crate::results::visualization_raster::MIN_RASTER_DIMENSION
            ..=crate::results::visualization_raster::MAX_RASTER_DIMENSION)
            .contains(&height)
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} dimensions {width}x{height} are outside the governed raster bounds"
        )));
    }
    let pixels = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or_else(|| {
            HardcopySourceError::InvalidReportSource(format!(
                "frozen plot block {block_id} dimensions overflow"
            ))
        })?;
    if pixels > crate::results::visualization_raster::MAX_RASTER_PIXELS {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the governed pixel limit"
        )));
    }
    if header.color_type != png::ColorType::Rgb
        || header.bit_depth != png::BitDepth::Eight
        || header.trns.is_some()
        || header.animation_control.is_some()
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} must be a single-frame opaque RGB8 PNG"
        )));
    }
    let expected_bytes = pixels.checked_mul(3).ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count overflowed"
        ))
    })?;
    let output_size = reader.output_buffer_size().ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has no bounded decoded size"
        ))
    })?;
    if output_size != expected_bytes {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count does not match RGB8 dimensions"
        )));
    }
    let mut decoded = Vec::new();
    decoded.try_reserve_exact(output_size).map_err(|_| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded buffer allocation failed"
        ))
    })?;
    decoded.resize(output_size, 0);
    let frame = reader.next_frame(&mut decoded).map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} PNG payload failed full decode: {error}"
        ))
    })?;
    if frame.width != width
        || frame.height != height
        || frame.color_type != png::ColorType::Rgb
        || frame.bit_depth != png::BitDepth::Eight
        || frame.buffer_size() != expected_bytes
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded frame contradicts its authenticated header"
        )));
    }
    Ok((width, height))
}

fn png_has_exact_terminal_iend(payload: &[u8]) -> bool {
    const SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if payload.len() < SIGNATURE.len() || &payload[..8] != SIGNATURE {
        return false;
    }
    let mut offset = 8usize;
    let mut saw_ihdr = false;
    while offset < payload.len() {
        let Some(header_end) = offset.checked_add(8) else {
            return false;
        };
        if header_end > payload.len() {
            return false;
        }
        let length = u32::from_be_bytes(
            payload[offset..offset + 4]
                .try_into()
                .expect("four-byte PNG length"),
        ) as usize;
        let chunk_type = &payload[offset + 4..offset + 8];
        if !saw_ihdr {
            if chunk_type != b"IHDR" {
                return false;
            }
            saw_ihdr = true;
        }
        let Some(chunk_end) = header_end
            .checked_add(length)
            .and_then(|end| end.checked_add(4))
        else {
            return false;
        };
        if chunk_end > payload.len() {
            return false;
        }
        if chunk_type == b"IEND" {
            return length == 0 && chunk_end == payload.len();
        }
        offset = chunk_end;
    }
    false
}

pub fn resolve_report_source(
    source: ReportHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::CompleteReport | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    source
        .document
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let record = source
        .document
        .revision_record(source.document.id(), source.document.revision())
        .ok_or_else(|| HardcopySourceError::UnretainedReportRevision(source.document.revision()))?;
    if record.snapshot().pages().is_empty() {
        return Err(HardcopySourceError::EmptyContent);
    }
    let mut authenticated_references = Vec::new();
    let mut figures = Vec::new();
    let mut linked_figures = Vec::new();
    let mut contains_linked_reference = false;
    for block in record
        .snapshot()
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
    {
        let Some(reference) = block.kind().reference() else {
            continue;
        };
        if let ReportBlockKind::PlotFigure(figure) = block.kind() {
            match reference {
                ReportReferenceMode::Frozen { artifact, .. } => {
                    let (width_pixels, height_pixels) =
                        validate_frozen_report_png(block.id(), artifact)?;
                    figures.push(SemanticReportFigure {
                        block_id: block.id(),
                        artifact_digest: artifact.content_digest(),
                        media_type: artifact.media_type().to_owned(),
                        payload: artifact.payload().to_vec(),
                        width_pixels,
                        height_pixels,
                        caption: figure.caption.clone(),
                        alternative_text: figure.alternative_text.clone(),
                        sizing: figure.sizing,
                    });
                }
                ReportReferenceMode::Linked { .. } => linked_figures.push(block.id()),
            }
        }
        contains_linked_reference |= matches!(reference, ReportReferenceMode::Linked { .. });
        authenticated_references.push(SemanticReportReference {
            block_id: block.id(),
            reference: reference.clone(),
        });
    }
    let empty_inventory = ReportReferenceInventory::default();
    let inventory = match (contains_linked_reference, source.reference_inventory) {
        (true, None) => {
            return Err(HardcopySourceError::ReportReferenceInventoryRequired);
        }
        (_, Some(inventory)) => inventory,
        (false, None) => &empty_inventory,
    };
    inventory
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let audit = source
        .document
        .audit_references(inventory)
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    if audit.entries.len() != authenticated_references.len() {
        return Err(HardcopySourceError::InvalidReportSource(
            "reference audit does not cover every referenced report block".to_owned(),
        ));
    }
    for entry in &audit.entries {
        if !matches!(
            entry.currentness,
            ReportReferenceCurrentness::Current | ReportReferenceCurrentness::Frozen
        ) {
            return Err(HardcopySourceError::UnauthenticatedReportReference {
                block_id: entry.block_id,
                currentness: entry.currentness,
            });
        }
    }
    if let Some(block_id) = linked_figures.first().copied() {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "linked plot figure",
            reason:
                "the source inventory authenticates identity but supplies no exact semantic or raster figure payload"
                    .to_owned(),
        });
    }
    let page_count = i64::try_from(record.snapshot().pages().len())
        .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
    let height = REPORT_PAGE_HEIGHT_UM
        .checked_mul(page_count)
        .and_then(|value| {
            REPORT_PAGE_GAP_UM
                .checked_mul(page_count.saturating_sub(1))
                .and_then(|gaps| value.checked_add(gaps))
        })
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.document.id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.document.revision(),
        source.document.title(),
    )?;
    finish_resolved(
        identity,
        record.snapshot_digest(),
        HardcopyDocumentKind::Report,
        source.scope,
        HardcopySemanticDocument::Report(SemanticReport {
            pages: record.snapshot().pages().to_vec(),
            authenticated_references,
            figures,
        }),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, height),
        )?,
    )
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

fn default_print_mapping(
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

fn layer_mapping(
    stable_id: impl Into<String>,
    display_name: impl Into<String>,
    screen_style: impl Into<String>,
) -> Result<PrintMappingEntry, HardcopySourceError> {
    mapping_entry(
        PrintObjectKind::Layer,
        stable_id,
        display_name,
        screen_style,
        PrintColor::GrayPercent(60),
        PrintRedundancy::SourceStyle,
        true,
    )
}

#[allow(clippy::too_many_arguments)]
fn mapping_entry(
    kind: PrintObjectKind,
    stable_id: impl Into<String>,
    display_name: impl Into<String>,
    screen_style: impl Into<String>,
    color: PrintColor,
    redundancy: PrintRedundancy,
    include_in_legend: bool,
) -> Result<PrintMappingEntry, HardcopySourceError> {
    let identity = PrintObjectIdentity::try_new(kind, stable_id, display_name, screen_style)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?;
    PrintMappingEntry::try_new(identity, color, redundancy, include_in_legend)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
}

fn compact_display(text: &str, fallback: &str) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let compact = compact.chars().take(96).collect::<String>();
    if compact.is_empty() {
        fallback.to_owned()
    } else {
        compact
    }
}

fn semantic_is_empty(schematic: &SemanticSchematic) -> bool {
    schematic.components.is_empty()
        && schematic.wires.is_empty()
        && schematic.buses.is_empty()
        && schematic.bus_taps.is_empty()
        && schematic.junctions.is_empty()
        && schematic.net_labels.is_empty()
        && schematic.design_notes.is_empty()
        && schematic.documentation_shapes.is_empty()
}

#[derive(Default)]
struct BoundsAccumulator {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
    initialized: bool,
}

impl BoundsAccumulator {
    fn include(&mut self, point: Point) {
        self.include_i64(i64::from(point.x), i64::from(point.y));
    }

    fn include_i64(&mut self, x: i64, y: i64) {
        if self.initialized {
            self.min_x = self.min_x.min(x);
            self.min_y = self.min_y.min(y);
            self.max_x = self.max_x.max(x);
            self.max_y = self.max_y.max(y);
        } else {
            self.min_x = x;
            self.min_y = y;
            self.max_x = x;
            self.max_y = y;
            self.initialized = true;
        }
    }

    fn finish(self, allowance_units: i64) -> Result<SemanticBounds, HardcopySourceError> {
        if !self.initialized {
            return Err(HardcopySourceError::EmptyContent);
        }
        let min_x = self
            .min_x
            .checked_sub(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let min_y = self
            .min_y
            .checked_sub(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let max_x = self
            .max_x
            .checked_add(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let max_y = self
            .max_y
            .checked_add(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        SemanticBounds::try_new(
            SemanticPoint::new(min_x, min_y),
            SemanticPoint::new(max_x, max_y),
        )
    }
}

fn schematic_bounds(schematic: &SemanticSchematic) -> Result<SemanticBounds, HardcopySourceError> {
    let mut bounds = BoundsAccumulator::default();
    for component in &schematic.components {
        if let Some(symbol) = &component.resolved_symbol {
            include_symbol_document_bounds(&mut bounds, symbol, Some(&component.component));
        } else {
            let (min_x, min_y, max_x, max_y) = component.component.bounding_box();
            bounds.include(Point::new(min_x, min_y));
            bounds.include(Point::new(max_x, max_y));
        }
        // Component labels are semantic document content and can extend past
        // the body. The edge allowance covers the font ascent and short names;
        // long labels add deterministic horizontal width.
        let label_chars = component
            .component
            .name
            .len()
            .max(component.component.value.len());
        let label_width = i64::try_from(label_chars)
            .unwrap_or(i64::MAX)
            .saturating_mul(2);
        bounds.include_i64(
            i64::from(component.component.pos.x).saturating_add(label_width),
            i64::from(component.component.pos.y),
        );
    }
    for wire in &schematic.wires {
        wire.points
            .iter()
            .copied()
            .for_each(|point| bounds.include(point));
    }
    for bus in &schematic.buses {
        bus.points
            .iter()
            .copied()
            .for_each(|point| bounds.include(point));
    }
    for tap in &schematic.bus_taps {
        bounds.include(tap.bus_point);
        bounds.include(tap.connection_point);
    }
    for junction in &schematic.junctions {
        bounds.include(junction.pos);
    }
    for label in &schematic.net_labels {
        bounds.include(label.pos);
        bounds.include_i64(
            i64::from(label.pos.x).saturating_add(
                i64::try_from(label.name.len())
                    .unwrap_or(i64::MAX)
                    .saturating_mul(2),
            ),
            i64::from(label.pos.y),
        );
    }
    for note in &schematic.design_notes {
        bounds.include(note.pos);
        let lines = note.text.lines().collect::<Vec<_>>();
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(1);
        bounds.include_i64(
            i64::from(note.pos.x)
                .saturating_add(i64::try_from(width).unwrap_or(i64::MAX).saturating_mul(2)),
            i64::from(note.pos.y).saturating_add(
                i64::try_from(lines.len())
                    .unwrap_or(i64::MAX)
                    .saturating_mul(4),
            ),
        );
    }
    for shape in &schematic.documentation_shapes {
        shape
            .geometry
            .points()
            .into_iter()
            .for_each(|point| bounds.include(point));
    }
    bounds.finish(SCHEMATIC_EDGE_ALLOWANCE_UNITS)
}

fn symbol_bounds(document: &SymbolDocument) -> Result<SemanticBounds, HardcopySourceError> {
    let mut bounds = BoundsAccumulator::default();
    include_symbol_document_bounds(&mut bounds, document, None);
    bounds.finish(SYMBOL_EDGE_ALLOWANCE_UNITS)
}

fn include_symbol_document_bounds(
    bounds: &mut BoundsAccumulator,
    document: &SymbolDocument,
    component: Option<&Component>,
) {
    let map = |point: Point| {
        let effective = point - document.origin;
        component.map_or(effective, |component| {
            component.pos + component.transform_point(effective)
        })
    };
    for shape in &document.body {
        match shape {
            SymbolShape::Polyline { points, .. } => {
                points
                    .iter()
                    .copied()
                    .map(map)
                    .for_each(|point| bounds.include(point));
            }
            SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                let center = map(*center);
                let radius = radius.unsigned_abs().min(i32::MAX as u32) as i32;
                bounds.include(Point::new(
                    center.x.saturating_sub(radius),
                    center.y.saturating_sub(radius),
                ));
                bounds.include(Point::new(
                    center.x.saturating_add(radius),
                    center.y.saturating_add(radius),
                ));
            }
            SymbolShape::Arc { center, radius, .. } => {
                // Conservative full-circle bounds avoid platform-dependent
                // trigonometric extrema while never clipping an authored arc.
                let center = map(*center);
                let radius = radius.unsigned_abs().min(i32::MAX as u32) as i32;
                bounds.include(Point::new(
                    center.x.saturating_sub(radius),
                    center.y.saturating_sub(radius),
                ));
                bounds.include(Point::new(
                    center.x.saturating_add(radius),
                    center.y.saturating_add(radius),
                ));
            }
            SymbolShape::Arrow { tip, .. } => bounds.include(map(*tip)),
        }
    }
    for pin in &document.pins {
        if let Some(point) = pin.position {
            bounds.include(map(point));
        }
    }
    bounds.include(map(document.name_anchor));
    bounds.include(map(document.value_anchor));
}

fn map_result_coordinate(
    value: f64,
    minimum: f64,
    span: f64,
    inset: i64,
    physical_span: i64,
) -> Result<i64, HardcopySourceError> {
    let normalized = (value - minimum) / span;
    let mapped = inset as f64 + normalized * physical_span as f64;
    if !mapped.is_finite() || mapped < i64::MIN as f64 || mapped > i64::MAX as f64 {
        return Err(HardcopySourceError::CoordinateOverflow);
    }
    Ok(mapped.round() as i64)
}

fn clipped_plot_paths(
    points: &[(f64, f64)],
    x_minimum: f64,
    x_maximum: f64,
    y_minimum: f64,
    y_maximum: f64,
    plot_width: i64,
    plot_height: i64,
) -> Result<Vec<Vec<SemanticPoint>>, HardcopySourceError> {
    if points
        .iter()
        .any(|point| !point.0.is_finite() || !point.1.is_finite())
    {
        return Err(HardcopySourceError::NonFiniteResultSample);
    }
    let x_span = x_maximum - x_minimum;
    let y_span = y_maximum - y_minimum;
    if points.len() == 1 {
        let point = points[0];
        if point.0 < x_minimum || point.0 > x_maximum || point.1 < y_minimum || point.1 > y_maximum
        {
            return Ok(Vec::new());
        }
        return Ok(vec![vec![map_plot_point(
            point.0,
            point.1,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?]]);
    }

    let mut paths: Vec<Vec<SemanticPoint>> = Vec::new();
    for pair in points.windows(2) {
        let Some(((start_x, start_y), (end_x, end_y))) =
            clip_line_to_axis_rect(pair[0], pair[1], x_minimum, x_maximum, y_minimum, y_maximum)
        else {
            continue;
        };
        let start = map_plot_point(
            start_x,
            start_y,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?;
        let end = map_plot_point(
            end_x,
            end_y,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?;
        if let Some(path) = paths.last_mut()
            && path.last() == Some(&start)
        {
            if path.last() != Some(&end) {
                path.push(end);
            }
        } else {
            paths.push(if start == end {
                vec![start]
            } else {
                vec![start, end]
            });
        }
    }
    Ok(paths)
}

fn canonical_marker_semantics(
    scene: &ResolvedCartesianLineScene,
    marker: &crate::results::visualization_document::Marker,
) -> SemanticPlotMarker {
    let source_x = typed_numeric_value(&marker.coordinate);
    let source_y = source_x.and_then(|x| {
        scene
            .traces()
            .iter()
            .find(|trace| trace.trace_id() == marker.trace_id)
            .and_then(|trace| trace_y_at_x(trace.points(), x))
    });
    SemanticPlotMarker {
        marker_id: marker.id.get(),
        label: marker.label.clone(),
        trace_id: Some(marker.trace_id.get()),
        source_x_bits: source_x.map(f64::to_bits),
        source_y_bits: source_y.map(f64::to_bits),
        position: source_x
            .zip(source_y)
            .and_then(|(x, y)| semantic_position_in_scene(scene, x, y).ok()),
    }
}

fn canonical_annotation_semantics(
    scene: &ResolvedCartesianLineScene,
    annotation: &crate::results::visualization_document::Annotation,
) -> SemanticPlotAnnotation {
    let (trace_id, source_x, source_y, position) = match &annotation.anchor {
        AnnotationAnchor::Pane {
            x_fraction,
            y_fraction,
        } => {
            let x_fraction = f64::from(*x_fraction).clamp(0.0, 1.0);
            let y_fraction = f64::from(*y_fraction).clamp(0.0, 1.0);
            let x = scene.x_range().minimum
                + x_fraction * (scene.x_range().maximum - scene.x_range().minimum);
            let y = scene.y_range().maximum
                - y_fraction * (scene.y_range().maximum - scene.y_range().minimum);
            (
                None,
                Some(x),
                Some(y),
                semantic_position_in_scene(scene, x, y).ok(),
            )
        }
        AnnotationAnchor::Trace {
            trace_id,
            coordinate,
        } => {
            let x = typed_numeric_value(coordinate);
            let y = x.and_then(|x| {
                scene
                    .traces()
                    .iter()
                    .find(|trace| trace.trace_id() == *trace_id)
                    .and_then(|trace| trace_y_at_x(trace.points(), x))
            });
            (
                Some(trace_id.get()),
                x,
                y,
                x.zip(y)
                    .and_then(|(x, y)| semantic_position_in_scene(scene, x, y).ok()),
            )
        }
    };
    SemanticPlotAnnotation {
        annotation_id: annotation.id.get(),
        text: annotation.text.clone(),
        trace_id,
        source_x_bits: source_x.map(f64::to_bits),
        source_y_bits: source_y.map(f64::to_bits),
        position,
    }
}

fn typed_numeric_value(value: &TypedValue) -> Option<f64> {
    match value {
        TypedValue::Real(value) => Some(*value),
        TypedValue::Integer(value) => Some(*value as f64),
        TypedValue::Boolean(_) | TypedValue::Text(_) => None,
    }
}

fn trace_y_at_x(
    points: &[crate::results::visualization_raster::ResolvedRasterPoint],
    x: f64,
) -> Option<f64> {
    if let Some(point) = points
        .iter()
        .find(|point| point.x().to_bits() == x.to_bits())
    {
        return Some(point.y());
    }
    points.windows(2).find_map(|pair| {
        let left = pair[0];
        let right = pair[1];
        if (left.x() <= x && x <= right.x()) || (right.x() <= x && x <= left.x()) {
            let span = right.x() - left.x();
            if span == 0.0 {
                Some(left.y())
            } else {
                let fraction = (x - left.x()) / span;
                Some(left.y() + fraction * (right.y() - left.y()))
            }
        } else {
            None
        }
    })
}

fn semantic_position_in_scene(
    scene: &ResolvedCartesianLineScene,
    x: f64,
    y: f64,
) -> Result<SemanticPoint, HardcopySourceError> {
    let x_range = scene.x_range();
    let y_range = scene.y_range();
    map_plot_point(
        x.clamp(x_range.minimum, x_range.maximum),
        y.clamp(y_range.minimum, y_range.maximum),
        x_range.minimum,
        y_range.minimum,
        x_range.maximum - x_range.minimum,
        y_range.maximum - y_range.minimum,
        PLOT_WIDTH_UM - 2 * PLOT_INSET_UM,
        PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM,
    )
}

#[allow(clippy::too_many_arguments)]
fn map_plot_point(
    x: f64,
    y: f64,
    x_minimum: f64,
    y_minimum: f64,
    x_span: f64,
    y_span: f64,
    plot_width: i64,
    plot_height: i64,
) -> Result<SemanticPoint, HardcopySourceError> {
    let mapped_x = map_result_coordinate(x, x_minimum, x_span, PLOT_INSET_UM, plot_width)?;
    let mapped_y = map_result_coordinate(y, y_minimum, y_span, PLOT_INSET_UM, plot_height)?;
    Ok(SemanticPoint::new(mapped_x, PLOT_HEIGHT_UM - mapped_y))
}

/// Liang-Barsky clipping avoids raster-size-dependent results and retains
/// exact line intersections with the authored axis rectangle.
fn clip_line_to_axis_rect(
    start: (f64, f64),
    end: (f64, f64),
    x_minimum: f64,
    x_maximum: f64,
    y_minimum: f64,
    y_maximum: f64,
) -> Option<((f64, f64), (f64, f64))> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let mut lower = 0.0_f64;
    let mut upper = 1.0_f64;
    for (p, q) in [
        (-dx, start.0 - x_minimum),
        (dx, x_maximum - start.0),
        (-dy, start.1 - y_minimum),
        (dy, y_maximum - start.1),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            if ratio > upper {
                return None;
            }
            lower = lower.max(ratio);
        } else {
            if ratio < lower {
                return None;
            }
            upper = upper.min(ratio);
        }
    }
    Some((
        (start.0 + lower * dx, start.1 + lower * dy),
        (start.0 + upper * dx, start.1 + upper * dy),
    ))
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

/// Pin a resolved live document into a persistable source-set member.
///
/// This constructor stays with the adapters because it reads
/// [`ResolvedHardcopyDocument`], which resolves live schematic and result
/// documents. The record type itself is owned by `crate::hardcopy::sources`.
impl HardcopySourceSetMember {
    pub fn from_resolved(resolved: &ResolvedHardcopyDocument) -> Result<Self, HardcopySourceError> {
        Self::try_new(
            resolved.source_key(),
            resolved.authority().display_name(),
            resolved.authority().document_id(),
            resolved.authority().revision(),
            resolved.authority().content_digest(),
            resolved.authority().scope().clone(),
        )
    }
}
