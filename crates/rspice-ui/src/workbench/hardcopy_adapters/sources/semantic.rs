//! The hardcopy semantic document.
//!
//! What a printable source *is*, independent of what draws it: points and
//! bounds in the source's own physical units, the schematic, plot, table,
//! report, and aggregate forms, and the worker snapshot that carries one
//! across a thread or worker boundary.
//!
//! Coordinates are integral micrometres, not screen points. A printed sheet
//! must be reproducible at any device resolution, so nothing here depends on
//! the zoom the document happened to be viewed at.
//!
//! Resolving a live document into one of these lives in the parent module;
//! this half is a value model and validates only itself.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};

use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT, validate_label,
};
use crate::hardcopy::{
    ActiveHardcopySource, ContentExtent, HardcopyContentSection, HardcopyDocumentId,
    HardcopyDocumentKind, HardcopyScope, Length, PrintColor, PrintMappingEntry, PrintMappingTable,
    PrintObjectIdentity, PrintObjectKind, PrintRedundancy,
};
use crate::product::{ContentDigest, ObjectRevision};
use crate::results::report_document::{
    FigureSizing, FrozenReportArtifact, ReportBlockId, ReportBlockKind, ReportPage,
    ReportReferenceMode,
};
use crate::state::{
    AnalysisResultPayload, Bus, BusTap, Component, DesignNote, DocumentationShape, Junction,
    NetLabel, ResolvedSymbolIssueKind, ResolvedSymbolSource, SheetId, SymbolDocument, SymbolShape,
    Wire,
};
use crate::workbench::documents::result_document::ResultViewer;

use super::{
    HardcopySourceError, MAX_WORKER_SNAPSHOT_BYTES, PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION,
    WORKER_SNAPSHOT_SCHEMA_VERSION, canonical_digest, default_print_mapping,
    validate_frozen_report_png,
};

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

    pub(super) fn content_extent(self) -> Result<ContentExtent, HardcopySourceError> {
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
    pub(super) source_key: String,
    pub(super) authority: ActiveHardcopySource,
    pub(super) semantic_document: HardcopySemanticDocument,
    pub(super) bounds: SemanticBounds,
    pub(super) content_extent: ContentExtent,
    pub(super) default_print_mapping: PrintMappingTable,
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
