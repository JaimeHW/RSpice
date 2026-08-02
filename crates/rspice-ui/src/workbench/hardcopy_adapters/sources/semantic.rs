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

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::hardcopy::sources::{
    DISPLAY_NAME_LIMIT, MAX_HARDCOPY_SOURCE_SET_MEMBERS, SOURCE_KEY_LIMIT, validate_label,
};
use crate::hardcopy::{
    ActiveHardcopySource, AuthoredSheetMedia, ContentExtent, HardcopyContentSection,
    HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, Length, PrintMappingTable,
};
use crate::product::{ContentDigest, ObjectRevision};
use crate::results::report_document::{
    FigureSizing, FrozenReportArtifact, ReportBlockId, ReportPage, ReportReferenceMode,
};
use crate::state::{
    AnalysisResultPayload, Bus, BusTap, Component, DesignNote, DocumentationShape,
    DrawingSheetTitleFieldId, Junction, NetLabel, SchematicSheetFormat, SymbolDocument, Wire,
};
use crate::workbench::documents::result_document::ResultViewer;

use super::{
    HardcopySourceError, MAX_WORKER_SNAPSHOT_BYTES, WORKER_SNAPSHOT_SCHEMA_VERSION,
    canonical_digest, default_print_mapping, validate_frozen_report_png,
};

/// One exact point in the source's natural physical coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPoint {
    pub x_um: i64,
    pub y_um: i64,
}

/// One aggregate child with the extent it occupies and where it sits.
type AggregateChildPlacement<'a> = (&'a SemanticAggregateChild, ContentExtent, SemanticPoint);

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

    pub(crate) fn content_extent(self) -> Result<ContentExtent, HardcopySourceError> {
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
    /// Authored physical drawing-sheet presentation for a governed sheet.
    ///
    /// This is intentionally independent of output media. `None` is retained
    /// only for legacy unbounded schematic documents and selection hardcopy.
    #[serde(default)]
    pub drawing_sheet: Option<SchematicSheetFormat>,
    /// Resolved automatic title-block values frozen with this hardcopy source.
    ///
    /// These values are source metadata, not authored overrides. Freezing
    /// them here keeps desktop, browser-worker, preview, PDF, and raster
    /// output identical even if the active workspace changes after queuing.
    #[serde(default)]
    pub drawing_sheet_title_values: BTreeMap<DrawingSheetTitleFieldId, String>,
    /// Frozen authored snap-grid pitch in schematic world units. One world
    /// unit is exactly [`super::SCHEMATIC_UNIT_UM`] micrometres.
    #[serde(default = "default_schematic_grid_pitch_units")]
    pub grid_pitch_units: i32,
    pub components: Vec<SemanticComponent>,
    pub wires: Vec<Wire>,
    pub buses: Vec<Bus>,
    pub bus_taps: Vec<BusTap>,
    pub junctions: Vec<Junction>,
    pub net_labels: Vec<NetLabel>,
    pub design_notes: Vec<DesignNote>,
    pub documentation_shapes: Vec<DocumentationShape>,
}

const fn default_schematic_grid_pitch_units() -> i32 {
    10
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

    /// The physical extent selected by a schematic-output policy.  The
    /// resolved source remains an immutable snapshot of all content; this
    /// accessor is the sole place publication narrows that snapshot to an
    /// authored sheet.
    pub fn content_extent_for_setup(
        &self,
        setup: crate::hardcopy::SchematicHardcopySetup,
    ) -> Result<ContentExtent, HardcopySourceError> {
        if setup.extent() == crate::hardcopy::SchematicHardcopyExtent::AuthoredDrawingSheet
            && setup.outside_content()
                == crate::hardcopy::OutsideSheetContentPolicy::ClipToAuthoredSheet
            && let HardcopySemanticDocument::Schematic(schematic) = &self.semantic_document
            && let Some(format) = &schematic.drawing_sheet
        {
            return super::authored_sheet_bounds(format)?.content_extent();
        }
        if let HardcopySemanticDocument::Aggregate(aggregate) = &self.semantic_document {
            let layout = aggregate_output_layout(aggregate, setup)?;
            let width = layout
                .iter()
                .map(|(_, extent, _)| extent.width().micrometres())
                .max()
                .unwrap_or(0);
            let height = layout.last().map_or(Ok(0_i64), |(_, extent, origin)| {
                origin
                    .y_um
                    .checked_add(
                        i64::try_from(extent.height().micrometres())
                            .map_err(|_| HardcopySourceError::CoordinateOverflow)?,
                    )
                    .ok_or(HardcopySourceError::CoordinateOverflow)
            })?;
            return ContentExtent::try_new(
                Length::from_micrometres(width),
                Length::from_micrometres(
                    u64::try_from(height).map_err(|_| HardcopySourceError::CoordinateOverflow)?,
                ),
            )
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()));
        }
        Ok(self.content_extent)
    }

    pub(crate) fn authored_sheet_bounds(
        &self,
        setup: crate::hardcopy::SchematicHardcopySetup,
    ) -> Result<Option<SemanticBounds>, HardcopySourceError> {
        if setup.extent() != crate::hardcopy::SchematicHardcopyExtent::AuthoredDrawingSheet
            || setup.outside_content()
                != crate::hardcopy::OutsideSheetContentPolicy::ClipToAuthoredSheet
        {
            return Ok(None);
        }
        match &self.semantic_document {
            HardcopySemanticDocument::Schematic(schematic) => schematic
                .drawing_sheet
                .as_ref()
                .map(super::authored_sheet_bounds)
                .transpose(),
            // An aggregate owns independently translated sheets. Its
            // sections need their own projection, so retain the established
            // aggregate extent rather than claiming one global sheet clip.
            HardcopySemanticDocument::Aggregate(_) => Ok(None),
            _ => Ok(None),
        }
    }

    pub(crate) fn aggregate_layout_for_setup(
        &self,
        setup: crate::hardcopy::SchematicHardcopySetup,
    ) -> Result<Option<Vec<AggregateChildPlacement<'_>>>, HardcopySourceError> {
        match &self.semantic_document {
            HardcopySemanticDocument::Aggregate(aggregate) => {
                aggregate_output_layout(aggregate, setup).map(Some)
            }
            _ => Ok(None),
        }
    }

    /// Source-derived defaults keyed by stable semantic identity. Callers can
    /// overlay persisted user choices by `(kind, stable_id)` without relying
    /// on table position or display text.
    #[must_use]
    pub const fn default_print_mapping(&self) -> &PrintMappingTable {
        &self.default_print_mapping
    }

    /// Return whether any resolved schematic content crosses an authored
    /// drawing-sheet boundary. `None` means this source has no authored
    /// schematic drawing sheet, including symbol-only and result documents.
    ///
    /// Aggregate sources are evaluated child-by-child so an all-sheets print
    /// fails closed when any one governed sheet needs an outside-content
    /// decision.
    pub fn schematic_drawing_sheet_has_outside_content(
        &self,
    ) -> Result<Option<bool>, HardcopySourceError> {
        fn visit(document: &HardcopySemanticDocument) -> Result<(bool, bool), HardcopySourceError> {
            match document {
                HardcopySemanticDocument::Schematic(schematic) => {
                    let Some(format) = schematic.drawing_sheet.as_ref() else {
                        return Ok((false, false));
                    };
                    if super::semantic_is_empty(schematic) {
                        return Ok((true, false));
                    }
                    let content = super::schematic_bounds(schematic)?;
                    let sheet = super::authored_sheet_bounds(format)?;
                    let outside = content.minimum.x_um < sheet.minimum.x_um
                        || content.minimum.y_um < sheet.minimum.y_um
                        || content.maximum.x_um > sheet.maximum.x_um
                        || content.maximum.y_um > sheet.maximum.y_um;
                    Ok((true, outside))
                }
                HardcopySemanticDocument::Aggregate(aggregate) => {
                    let mut found = false;
                    let mut outside = false;
                    for child in &aggregate.children {
                        let (child_found, child_outside) = visit(&child.document)?;
                        found |= child_found;
                        outside |= child_outside;
                    }
                    Ok((found, outside))
                }
                HardcopySemanticDocument::Symbol(_)
                | HardcopySemanticDocument::Plot(_)
                | HardcopySemanticDocument::ResultSummary(_)
                | HardcopySemanticDocument::Report(_) => Ok((false, false)),
            }
        }

        let (found, outside) = visit(&self.semantic_document)?;
        Ok(found.then_some(outside))
    }

    /// Ordered authenticated page groups for aggregate-aware pagination.
    /// Ordinary documents return an empty vector and retain legacy extent
    /// compilation; aggregate callers must pass every returned section to
    /// `HardcopyPlan::compile_with_sections`.
    pub fn hardcopy_sections(&self) -> Result<Vec<HardcopyContentSection>, HardcopySourceError> {
        self.hardcopy_sections_for_setup(crate::hardcopy::SchematicHardcopySetup::default())
    }

    pub fn hardcopy_sections_for_setup(
        &self,
        setup: crate::hardcopy::SchematicHardcopySetup,
    ) -> Result<Vec<HardcopyContentSection>, HardcopySourceError> {
        let HardcopySemanticDocument::Aggregate(aggregate) = &self.semantic_document else {
            return Ok(Vec::new());
        };
        aggregate_output_layout(aggregate, setup)?
            .into_iter()
            .map(|(child, extent, origin)| {
                let origin_x = u64::try_from(origin.x_um)
                    .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
                let origin_y = u64::try_from(origin.y_um)
                    .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
                HardcopyContentSection::try_new(
                    child.ordinal,
                    child.content_digest,
                    Length::from_micrometres(origin_x),
                    Length::from_micrometres(origin_y),
                    extent,
                    child.page_break_before,
                )
                .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
            })
            .collect()
    }

    /// Exact physical media for governed authored-sheet output, in the same
    /// canonical order as aggregate hardcopy sections. This inventory is
    /// sealed into [`PaperSize::MatchAuthoredSheets`] by the dialog before a
    /// plan is compiled, so preview and publication resolve identical mixed
    /// sheet sizes and orientations.
    pub fn authored_sheet_output_media(
        &self,
    ) -> Result<Vec<AuthoredSheetMedia>, HardcopySourceError> {
        fn from_format(
            ordinal: u32,
            format: &SchematicSheetFormat,
        ) -> Result<AuthoredSheetMedia, HardcopySourceError> {
            let (width_um, height_um) = format.oriented_dimensions_um();
            AuthoredSheetMedia::try_new(
                ordinal,
                format.display(),
                Length::from_micrometres(width_um),
                Length::from_micrometres(height_um),
            )
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
        }

        match &self.semantic_document {
            HardcopySemanticDocument::Schematic(schematic) => {
                let format = schematic.drawing_sheet.as_ref().ok_or_else(|| {
                    HardcopySourceError::InvalidSheetPartition(
                        "the selected schematic has no authored drawing sheet".to_owned(),
                    )
                })?;
                Ok(vec![from_format(0, format)?])
            }
            HardcopySemanticDocument::Aggregate(aggregate) => aggregate
                .children
                .iter()
                .map(|child| match child.document.as_ref() {
                    HardcopySemanticDocument::Schematic(schematic) => schematic
                        .drawing_sheet
                        .as_ref()
                        .ok_or_else(|| {
                            HardcopySourceError::InvalidSheetPartition(format!(
                                "sheet section {} has no authored drawing sheet",
                                child.ordinal + 1
                            ))
                        })
                        .and_then(|format| from_format(child.ordinal, format)),
                    _ => Err(HardcopySourceError::InvalidSheetPartition(format!(
                        "section {} is not a governed schematic sheet",
                        child.ordinal + 1
                    ))),
                })
                .collect(),
            HardcopySemanticDocument::Symbol(_)
            | HardcopySemanticDocument::Plot(_)
            | HardcopySemanticDocument::ResultSummary(_)
            | HardcopySemanticDocument::Report(_) => {
                Err(HardcopySourceError::InvalidSheetPartition(
                    "the selected source does not own authored drawing-sheet media".to_owned(),
                ))
            }
        }
    }

    /// Bounded serde envelope for browser dedicated-worker transfer.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn worker_snapshot_json(&self) -> Result<Vec<u8>, HardcopySourceError> {
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

/// Deterministic aggregate layout after applying the selected per-sheet
/// output policy.  Only an authored-sheet clip changes a child footprint;
/// all other document kinds retain their immutable resolved bounds.
fn aggregate_output_layout(
    aggregate: &SemanticAggregate,
    setup: crate::hardcopy::SchematicHardcopySetup,
) -> Result<Vec<(&SemanticAggregateChild, ContentExtent, SemanticPoint)>, HardcopySourceError> {
    let mut next_y_um = 0_i64;
    let mut layout = Vec::with_capacity(aggregate.children.len());
    for (index, child) in aggregate.children.iter().enumerate() {
        let extent = match child.document.as_ref() {
            HardcopySemanticDocument::Schematic(schematic)
                if setup.extent()
                    == crate::hardcopy::SchematicHardcopyExtent::AuthoredDrawingSheet
                    && setup.outside_content()
                        == crate::hardcopy::OutsideSheetContentPolicy::ClipToAuthoredSheet =>
            {
                schematic.drawing_sheet.as_ref().map_or_else(
                    || child.local_bounds.content_extent(),
                    |format| super::authored_sheet_bounds(format)?.content_extent(),
                )?
            }
            _ => child.local_bounds.content_extent()?,
        };
        layout.push((child, extent, SemanticPoint::new(0, next_y_um)));
        next_y_um = next_y_um
            .checked_add(
                i64::try_from(extent.height().micrometres())
                    .map_err(|_| HardcopySourceError::CoordinateOverflow)?,
            )
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        if index + 1 != aggregate.children.len() {
            next_y_um = next_y_um
                .checked_add(super::REPORT_PAGE_GAP_UM)
                .ok_or(HardcopySourceError::CoordinateOverflow)?;
        }
    }
    Ok(layout)
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
        HardcopySemanticDocument::Schematic(schematic) => {
            if schematic.grid_pitch_units <= 0 {
                return Err(HardcopySourceError::InvalidWorkerSnapshot(
                    "schematic grid pitch must be positive".to_owned(),
                ));
            }
        }
        HardcopySemanticDocument::Symbol(_)
        | HardcopySemanticDocument::Plot(_)
        | HardcopySemanticDocument::ResultSummary(_) => {}
    }
    Ok(())
}
