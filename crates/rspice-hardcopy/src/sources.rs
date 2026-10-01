//! Frozen, authenticated hardcopy sources and their physical geometry.

mod aggregation;
mod artifacts;
mod design;
mod geometry;
mod mapping;
mod plots;
mod quick_plot;
mod reports;
mod result_summary;
mod result_view;
mod schematic;
mod semantic;

pub(crate) use geometry::authored_sheet_bounds;

pub use aggregation::resolve_hardcopy_source_set_with;
use artifacts::validate_frozen_report_png;
pub use design::{drawing_sheet_title_values, resolve_symbol_document};
use geometry::{
    canonical_annotation_semantics, canonical_cursor_semantics, canonical_marker_semantics,
    layer_mapping, mapping_entry, symbol_bounds,
};
pub use geometry::{
    clipped_plot_paths, compact_display, drawing_sheet_artwork_bounds, map_plot_point,
    schematic_bounds, semantic_is_empty, source_set_member_from_resolved, union_bounds,
};
use mapping::default_print_mapping;
pub use plots::{resolve_plot_source, resolve_visualization_pane_source};
pub use quick_plot::{
    PlotFrame, QuickResultSeries, RetainedCursorInterpolation, RetainedQuickMarker,
    RetainedQuickViewOverlay, RetainedQuickViewOverlays, RetainedQuickViewport,
    nondegenerate_range, plot_axes, quick_plot_from_scaled_series, quick_plot_from_series,
    stable_page_id, stable_quick_trace_id,
};
pub use reports::{ReportHardcopySource, resolve_report_source};
pub use result_summary::{is_curve_viewer, semantic_result_summary};
pub use result_view::{
    QuickFftSettings, QuickHistogramSettings, ResultsQuickViewPresentation,
    RetainedQuickViewSource, resolve_results_quick_view_parts, results_quick_view_identity,
};
use rspice_app_types::product::ContentDigest;
use rspice_design::schematic::component::Component;
use rspice_design::symbol::{SymbolDocument, SymbolShape};
use rspice_design_model::{Point, design_management::SchematicSheetFormat};
use rspice_hardcopy_contract::sources::{
    DISPLAY_NAME_LIMIT, HardcopySourceError, HardcopySourceIdentity, HardcopySourceSet,
    HardcopySourceSetMember, SOURCE_KEY_LIMIT, canonical_digest, validate_label,
};
use rspice_hardcopy_contract::{
    ActiveHardcopySource, HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, Length,
    PrintColor, PrintMappingEntry, PrintMappingSaveScope, PrintMappingTable, PrintObjectIdentity,
    PrintObjectKind, PrintRedundancy,
};
use rspice_results::report_document::{
    FrozenReportArtifact, ReportBlockId, ReportBlockKind, ReportReferenceSnapshot,
};
use rspice_results::result_presentation::ResultViewer;
use rspice_results::visualization_document::{
    AnnotationAnchor, AxisScale, PageId, PaneId, TypedValue, VisualizationDocument,
};
use rspice_results::visualization_raster::{
    ResolvedCartesianLineScene, VisualizationRasterError, resolve_cartesian_line_scene,
};
pub use schematic::{
    SchematicHardcopySelection, SchematicHardcopySource, SchematicSheetSetHardcopySource,
    resolve_all_schematic_sheets, resolve_schematic_source, schematic_sheet_identity,
};
pub use semantic::*;
use sha2::{Digest as _, Sha256};

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
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const MAX_WORKER_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(super) const WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 3;
const SCHEMATIC_EDGE_ALLOWANCE_UNITS: i64 = 16;
const SYMBOL_EDGE_ALLOWANCE_UNITS: i64 = 10;
pub const PLOT_INSET_UM: i64 = 12_700;

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

/// Validate captured source semantics and bind their exact publication identity.
pub fn resolve_semantic_source(
    identity: HardcopySourceIdentity,
    content_digest: ContentDigest,
    kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    semantic_document: HardcopySemanticDocument,
    bounds: SemanticBounds,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    identity.validate()?;
    semantic::validate_worker_semantics(&semantic_document)?;
    finish_resolved(
        identity,
        content_digest,
        kind,
        scope,
        semantic_document,
        bounds,
    )
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
