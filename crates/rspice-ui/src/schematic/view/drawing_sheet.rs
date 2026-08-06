//! Permanent authored drawing-sheet geometry and canvas presentation.
//!
//! The sheet is document geometry behind the schematic, not a print-preview
//! overlay. It never participates in hit testing, so wiring, selection, and
//! context-menu gestures behave identically on paper and on the surrounding
//! unbounded desk.

use std::borrow::Cow;

use egui::{
    Align2, Color32, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, epaint::TextShape, pos2, vec2,
};

use crate::state::{
    DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS, DrawingSheetBorderTemplate,
    DrawingSheetDisplayUnit, DrawingSheetGeometry as AuthoredDrawingSheetGeometry,
    DrawingSheetRect, DrawingSheetScale, DrawingSheetTitleBlockRotation, DrawingSheetTitleFieldId,
    DrawingSheetTitleFieldValueAuthority, DrawingSheetZoneEdges, DrawingSheetZoneGrid,
    DrawingSheetZoneLabels, Point, SchematicSheetFormat,
};
use crate::ui::theme;
use crate::ui::tokens::{Mode, Tokens};
use crate::workbench::app_state::AppState;

use super::SchematicSymbolContext;
use super::grid::draw_grid;
use super::sheet_visibility::object_is_on_active_sheet;
use super::viewport::Viewport;

/// Canonical mockup calibration: four schematic world units per millimetre.
pub(crate) const DRAWING_SHEET_UNITS_PER_MM: f64 = 4.0;
/// Exact physical calibration paired with [`DRAWING_SHEET_UNITS_PER_MM`].
pub(crate) const DRAWING_SHEET_WORLD_UNIT_UM: u64 = 250;
/// Stable authored page origin. Format changes grow or shrink to the right and
/// bottom, so an existing object's sheet coordinates never move.
pub(crate) const DRAWING_SHEET_ORIGIN: (f64, f64) = (-140.0, -40.0);

const FIT_SCREEN_INSET: f64 = 26.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WorldRect {
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
}

impl WorldRect {
    fn from_physical(rect: DrawingSheetRect) -> Self {
        let unit_um = DRAWING_SHEET_WORLD_UNIT_UM as f64;
        let min_x = DRAWING_SHEET_ORIGIN.0 + rect.x_um as f64 / unit_um;
        let min_y = DRAWING_SHEET_ORIGIN.1 + rect.y_um as f64 / unit_um;
        Self {
            min_x,
            min_y,
            max_x: min_x + rect.width_um as f64 / unit_um,
            max_y: min_y + rect.height_um as f64 / unit_um,
        }
    }

    pub(crate) const fn as_tuple(self) -> (f64, f64, f64, f64) {
        (self.min_x, self.min_y, self.max_x, self.max_y)
    }

    fn contains_rect(self, other: Self) -> bool {
        other.min_x >= self.min_x
            && other.min_y >= self.min_y
            && other.max_x <= self.max_x
            && other.max_y <= self.max_y
    }

    fn contains_point(self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    fn intersects(self, other: Self) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }

    pub(super) fn screen_rect(self, viewport: &Viewport) -> Rect {
        let screen = |x: f64, y: f64| {
            pos2(
                viewport.bounds.min.x + viewport.offset.x + x as f32 * viewport.zoom,
                viewport.bounds.min.y + viewport.offset.y + y as f32 * viewport.zoom,
            )
        };
        Rect::from_min_max(
            screen(self.min_x, self.min_y),
            screen(self.max_x, self.max_y),
        )
    }

    fn from_points(min: Point, max: Point) -> Self {
        Self {
            min_x: f64::from(min.x.min(max.x)),
            min_y: f64::from(min.y.min(max.y)),
            max_x: f64::from(min.x.max(max.x)),
            max_y: f64::from(min.y.max(max.y)),
        }
    }
}

/// Exact physical and world-space rectangles consumed by the canvas,
/// inspector, and hardcopy adapters.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DrawingSheetGeometry {
    /// Validated integer-micrometre geometry from the durable domain.
    pub(crate) physical: AuthoredDrawingSheetGeometry,
    pub(crate) paper: WorldRect,
    pub(crate) printable: WorldRect,
    pub(crate) drawing_area: WorldRect,
    pub(crate) content_area: WorldRect,
    pub(crate) title_block: Option<WorldRect>,
}

impl DrawingSheetGeometry {
    pub(crate) fn from_format(format: &SchematicSheetFormat) -> Self {
        // `SchematicSheetFormat` is only materialized through validated
        // constructors, deserialization, or `try_update`. If a caller ever
        // violates that invariant in memory, retain a usable canvas by falling
        // back to the independently validated built-in format.
        let physical = format.geometry().unwrap_or_else(|error| {
            debug_assert!(false, "invalid drawing-sheet geometry: {error}");
            SchematicSheetFormat::default()
                .geometry()
                .expect("the built-in drawing-sheet format must remain valid")
        });
        Self {
            paper: WorldRect::from_physical(physical.paper),
            printable: WorldRect::from_physical(physical.printable),
            drawing_area: WorldRect::from_physical(physical.drawing_area),
            content_area: WorldRect::from_physical(physical.content_area),
            title_block: physical.title_block.map(WorldRect::from_physical),
            physical,
        }
    }

    pub(crate) fn world_to_sheet_mm(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        (
            (world_x - DRAWING_SHEET_ORIGIN.0) / DRAWING_SHEET_UNITS_PER_MM,
            (world_y - DRAWING_SHEET_ORIGIN.1) / DRAWING_SHEET_UNITS_PER_MM,
        )
    }

    pub(crate) fn fit_view(
        &self,
        schematic: &mut crate::state::SchematicState,
        viewport_width: f64,
        viewport_height: f64,
    ) {
        schematic.zoom_to_fit_world_rect(
            self.paper.as_tuple(),
            viewport_width,
            viewport_height,
            FIT_SCREEN_INSET,
        );
    }

    fn zone_reference(&self, world_x: f64, world_y: f64) -> Option<String> {
        let zones = self.physical.zones?;
        if zones.labels == DrawingSheetZoneLabels::Coordinates {
            return None;
        }
        if world_x < self.drawing_area.min_x
            || world_x > self.drawing_area.max_x
            || world_y < self.drawing_area.min_y
            || world_y > self.drawing_area.max_y
        {
            return None;
        }
        let x_fraction = (world_x - self.drawing_area.min_x)
            / (self.drawing_area.max_x - self.drawing_area.min_x).max(f64::EPSILON);
        let y_fraction = (world_y - self.drawing_area.min_y)
            / (self.drawing_area.max_y - self.drawing_area.min_y).max(f64::EPSILON);
        let column = ((x_fraction * f64::from(zones.columns)).floor() as usize)
            .min(usize::from(zones.columns.saturating_sub(1)));
        let row = ((y_fraction * f64::from(zones.rows)).floor() as usize)
            .min(usize::from(zones.rows.saturating_sub(1)));
        Some(zone_reference_label(zones.labels, column, row))
    }
}

/// Resolved active-sheet presentation authority.
#[derive(Debug, Clone)]
pub(crate) struct ActiveDrawingSheet {
    pub(crate) format: SchematicSheetFormat,
    pub(crate) geometry: DrawingSheetGeometry,
    pub(crate) sheet_name: String,
    pub(crate) page_label: String,
}

impl ActiveDrawingSheet {
    pub(crate) fn resolve(state: &AppState) -> Self {
        let key = state.workspace.active_schematic_reference().key();
        if let Some(catalog) = state.workspace.design_management.sheet_catalog(&key)
            && let Some(sheet) = catalog.active()
        {
            let (page, page_count) = catalog
                .page_number_and_count(sheet.id())
                .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
            let format = match sheet.page_format().inheritance {
                crate::state::DrawingSheetInheritance::ProjectDefault => state
                    .workspace
                    .design_management
                    .drawing_sheet_settings()
                    .default_format
                    .with_target_sheet_title_fields(sheet.page_format()),
                crate::state::DrawingSheetInheritance::Explicit
                | crate::state::DrawingSheetInheritance::UserDefault => sheet.page_format().clone(),
            };
            return Self {
                geometry: DrawingSheetGeometry::from_format(&format),
                format,
                sheet_name: sheet.name().to_owned(),
                page_label: format!("{page} of {page_count}"),
            };
        }

        let format = state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .default_format
            .clone();
        Self {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: state.workspace.active_view.cell.clone(),
            page_label: "1 of 1".to_owned(),
        }
    }

    pub(crate) fn format_label(&self) -> String {
        format!(
            "{} \u{00b7} {}",
            self.format.authored_size.label(),
            self.format.orientation.label().to_lowercase()
        )
    }

    /// Status-bar readout in the authored sheet's configured display unit.
    /// Coordinates retain the permanent page origin even while the pointer is
    /// on the surrounding desk; the trailing state explains that condition.
    pub(crate) fn cursor_status(&self, world_x: f64, world_y: f64) -> String {
        let (x_mm, y_mm) = self.geometry.world_to_sheet_mm(world_x, world_y);
        let unit = self.format.display_unit;
        let coordinates = format!(
            "x {} \u{00b7} y {} {}",
            format_sheet_coordinate(x_mm, unit),
            format_sheet_coordinate(y_mm, unit),
            unit.suffix()
        );
        if !self.geometry.paper.contains_point(world_x, world_y) {
            return format!("{coordinates} \u{00b7} off sheet");
        }
        self.geometry
            .zone_reference(world_x, world_y)
            .map_or(coordinates.clone(), |zone| {
                format!("{coordinates} \u{00b7} zone {zone}")
            })
    }
}

fn format_sheet_coordinate(value_mm: f64, unit: DrawingSheetDisplayUnit) -> String {
    let (value, precision) = match unit {
        DrawingSheetDisplayUnit::Millimetres => (value_mm, 1),
        DrawingSheetDisplayUnit::Centimetres => (value_mm / 10.0, 2),
        DrawingSheetDisplayUnit::Inches => (value_mm / 25.4, 3),
    };
    let threshold = 0.5 * 10.0_f64.powi(-precision);
    let value = if value.abs() < threshold { 0.0 } else { value };
    let fixed = format!("{value:.precision$}", precision = precision as usize);
    fixed.trim_end_matches('0').trim_end_matches('.').to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawingSheetLodTier {
    Thumbnail,
    Outline,
    Working,
    Detail,
}

#[derive(Debug, Clone, Copy)]
struct DrawingSheetLod {
    tier: DrawingSheetLodTier,
    show_border: bool,
    show_margins: bool,
    show_zone_labels: bool,
    show_title_block_fields: bool,
    show_grid: bool,
    show_origin: bool,
    show_marks: bool,
    show_dimensions: bool,
}

/// Desktop fit-sheet views land between roughly 0.55x and 0.85x world scale,
/// depending on dock width and window geometry. Treat authored sizes smaller
/// than the requested readable floor at the lower bound as under-specified:
/// otherwise the floor remains active through several normal wheel steps,
/// making the title-block cells grow while their text appears frozen. Below
/// this reference the floor still protects overview legibility; at and above
/// it the text grows continuously with the sheet.
const DRAWING_SHEET_TEXT_FIT_REFERENCE_ZOOM: f32 = 0.55;

fn drawing_sheet_font_size(zoom: f32, authored_size: f32, readable_floor: f32) -> f32 {
    let authored_for_fit =
        authored_size.max(readable_floor / DRAWING_SHEET_TEXT_FIT_REFERENCE_ZOOM);
    let scaled = authored_for_fit * zoom.max(0.0);
    ((scaled.max(readable_floor) * 4.0).round() * 0.25).max(1.0)
}

impl DrawingSheetLod {
    fn resolve(zoom: f32, pixels_per_point: f32) -> Self {
        let millimetre_pixels =
            DRAWING_SHEET_UNITS_PER_MM as f32 * zoom * pixels_per_point.max(0.5);
        let tier = if millimetre_pixels < 0.55 {
            DrawingSheetLodTier::Thumbnail
        } else if millimetre_pixels < 1.5 {
            DrawingSheetLodTier::Outline
        } else if millimetre_pixels < 5.5 {
            DrawingSheetLodTier::Working
        } else {
            DrawingSheetLodTier::Detail
        };
        Self {
            tier,
            show_border: tier != DrawingSheetLodTier::Thumbnail,
            show_margins: tier != DrawingSheetLodTier::Thumbnail,
            show_zone_labels: matches!(
                tier,
                DrawingSheetLodTier::Working | DrawingSheetLodTier::Detail
            ),
            show_title_block_fields: matches!(
                tier,
                DrawingSheetLodTier::Working | DrawingSheetLodTier::Detail
            ),
            show_grid: tier != DrawingSheetLodTier::Thumbnail,
            show_origin: tier == DrawingSheetLodTier::Detail,
            show_marks: tier == DrawingSheetLodTier::Detail,
            show_dimensions: tier == DrawingSheetLodTier::Detail,
        }
    }
}

/// Overflow is actionable document state, not decorative page detail. It
/// remains visible in fit/thumbnail views where off-sheet content is otherwise
/// easiest to overlook.
const fn overflow_advisories_visible(_tier: DrawingSheetLodTier) -> bool {
    true
}

#[derive(Clone, Copy)]
struct DrawingSheetPalette {
    desk: Color32,
    paper: Color32,
    edge: Color32,
    shadow: Color32,
    margin: Color32,
    border: Color32,
    zone: Color32,
    zone_ink: Color32,
    block: Color32,
    block_ink: Color32,
    block_label: Color32,
    overflow: Color32,
}

impl DrawingSheetPalette {
    fn resolve(tokens: &Tokens) -> Self {
        if tokens.mode == Mode::Light {
            Self {
                desk: Color32::from_rgb(0xd5, 0xda, 0xdd),
                paper: Color32::WHITE,
                edge: Color32::from_rgb(0x99, 0xa2, 0xa7),
                shadow: Color32::from_black_alpha(64),
                margin: Color32::from_rgb(0xb6, 0xbe, 0xc2),
                border: Color32::from_rgb(0x4d, 0x54, 0x59),
                zone: Color32::from_rgb(0x8d, 0x95, 0x9a),
                zone_ink: Color32::from_rgb(0x45, 0x4b, 0x4f),
                block: Color32::from_rgb(0xf3, 0xf5, 0xf6),
                block_ink: Color32::from_rgb(0x1f, 0x25, 0x28),
                block_label: Color32::from_rgb(0x67, 0x6e, 0x72),
                overflow: tokens.color.warn,
            }
        } else {
            Self {
                desk: Color32::from_rgb(0x04, 0x08, 0x0b),
                paper: Color32::from_rgb(0x0b, 0x12, 0x17),
                edge: Color32::from_rgb(0x4d, 0x59, 0x60),
                shadow: Color32::from_black_alpha(96),
                margin: Color32::from_rgb(0x30, 0x3d, 0x45),
                border: Color32::from_rgb(0x6f, 0x7f, 0x89),
                zone: Color32::from_rgb(0x41, 0x4e, 0x56),
                zone_ink: Color32::from_rgb(0x8e, 0x99, 0x9f),
                block: Color32::from_rgba_unmultiplied(255, 255, 255, 7),
                block_ink: Color32::from_rgb(0xc5, 0xcd, 0xd2),
                block_label: Color32::from_rgb(0x7f, 0x89, 0x8f),
                overflow: tokens.color.warn,
            }
        }
    }
}

/// Paint the permanent pointer-transparent paper and its optional supporting
/// layers. Schematic objects are painted by the caller afterwards.
pub(super) fn draw_base(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) {
    let tokens = Tokens::get(painter.ctx());
    let palette = DrawingSheetPalette::resolve(&tokens);
    let layers = state.ui.drawing_sheet_layers;
    let lod = DrawingSheetLod::resolve(viewport.zoom, painter.ctx().pixels_per_point());
    let paper = sheet.geometry.paper.screen_rect(viewport);
    let printable = sheet.geometry.printable.screen_rect(viewport);
    let drawing_area = sheet.geometry.drawing_area.screen_rect(viewport);

    painter.rect_filled(available, 0.0, palette.desk);
    for (spread, alpha, y_offset) in [(12.0, 20, 5.0), (7.0, 34, 3.0), (3.0, 52, 1.5)] {
        let shadow = Rect::from_min_max(
            paper.min - vec2(spread, spread - y_offset),
            paper.max + vec2(spread, spread + y_offset),
        );
        painter.rect_filled(
            shadow,
            0.0,
            Color32::from_rgba_unmultiplied(
                palette.shadow.r(),
                palette.shadow.g(),
                palette.shadow.b(),
                alpha,
            ),
        );
    }
    painter.rect_filled(paper, 0.0, palette.paper);

    if lod.show_grid {
        draw_grid(painter, available, drawing_area.intersect(available), state);
    }
    if lod.show_margins && layers.margins && sheet.format.border != DrawingSheetBorderTemplate::None
    {
        dashed_rect(painter, printable, Stroke::new(1.0, palette.margin));
    }
    if lod.show_border && sheet.format.border != DrawingSheetBorderTemplate::None {
        let border_width = match sheet.format.border {
            DrawingSheetBorderTemplate::Plain => 1.0,
            DrawingSheetBorderTemplate::Standard => 1.25,
            DrawingSheetBorderTemplate::OrganizationManaged => 1.5,
            DrawingSheetBorderTemplate::None => 0.0,
        };
        painter.rect_stroke(
            drawing_area,
            0.0,
            Stroke::new(border_width, palette.border),
            StrokeKind::Middle,
        );
        if layers.zones
            && let Some(zones) = sheet.geometry.physical.zones
        {
            draw_zone_band(
                painter,
                printable,
                drawing_area,
                zones,
                lod,
                viewport.zoom,
                palette,
            );
        }
    }
    if layers.title_block
        && let Some(block) = sheet
            .geometry
            .title_block
            .map(|rect| rect.screen_rect(viewport))
    {
        draw_title_block(painter, block, lod, viewport.zoom, palette, state, sheet);
    }
    if lod.show_marks && layers.registration_marks {
        if sheet.format.marks.registration {
            draw_registration_marks(painter, paper, printable, palette);
        }
        if sheet.format.marks.folding {
            draw_folding_marks(painter, paper, viewport.zoom, palette);
        }
    }
    if lod.show_origin {
        draw_origin(painter, paper, viewport.zoom, palette);
    }
    if lod.show_dimensions && layers.dimensions_at_detail_zoom {
        draw_dimensions(painter, paper, viewport.zoom, palette, sheet);
    }

    // The paper edge is permanent and is deliberately painted last.
    painter.rect_stroke(
        paper,
        0.0,
        Stroke::new(1.6, palette.edge),
        StrokeKind::Middle,
    );
}

/// Paint advisory halos above authored objects. These never block saves,
/// checks, netlisting, or simulation.
pub(super) fn draw_overflow_advisories(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) {
    let lod = DrawingSheetLod::resolve(viewport.zoom, painter.ctx().pixels_per_point());
    if !overflow_advisories_visible(lod.tier) {
        return;
    }
    let palette = DrawingSheetPalette::resolve(&Tokens::get(painter.ctx()));
    let summary = drawing_sheet_overflow_summary(state, symbol_context, sheet);
    if summary.is_clear() {
        return;
    }
    for item in summary.items {
        let mut screen = item.bounds.screen_rect(viewport).expand(3.0);
        if screen.width() < 8.0 {
            screen = Rect::from_center_size(screen.center(), vec2(8.0, screen.height().max(8.0)));
        }
        if screen.height() < 8.0 {
            screen = Rect::from_center_size(screen.center(), vec2(screen.width().max(8.0), 8.0));
        }
        screen = screen.intersect(available);
        if !screen.is_positive() {
            continue;
        }
        let (dash, gap, width) = match item.severity {
            DrawingSheetOverflowSeverity::OutsideBorder => (5.0, 4.0, 1.4),
            DrawingSheetOverflowSeverity::OffPaper => (7.0, 3.0, 1.6),
            DrawingSheetOverflowSeverity::TitleBlockCollision => (2.0, 3.0, 1.4),
        };
        dashed_rect_pattern(
            painter,
            screen,
            Stroke::new(width, palette.overflow),
            dash,
            gap,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DrawingSheetOverflowTarget {
    Component(u64),
    Wire(u64),
    Bus(u64),
    BusTap(u64),
    Junction(u64),
    NetLabel(u64),
    DesignNote(u64),
    DocumentationShape(u64),
}

impl DrawingSheetOverflowTarget {
    pub(crate) const fn kind_label(self) -> &'static str {
        match self {
            Self::Component(_) => "Instance",
            Self::Wire(_) => "Wire",
            Self::Bus(_) => "Bus",
            Self::BusTap(_) => "Bus tap",
            Self::Junction(_) => "Junction",
            Self::NetLabel(_) => "Net label",
            Self::DesignNote(_) => "Design note",
            Self::DocumentationShape(_) => "Documentation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrawingSheetOverflowSeverity {
    OutsideBorder,
    OffPaper,
    TitleBlockCollision,
}

impl DrawingSheetOverflowSeverity {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::OutsideBorder => "Outside the drawing border",
            Self::OffPaper => "Beyond the paper edge",
            Self::TitleBlockCollision => "Overlapping the title block",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetOverflowItem {
    pub(crate) target: DrawingSheetOverflowTarget,
    pub(crate) identity: String,
    pub(crate) kind: &'static str,
    pub(crate) outside_border: bool,
    pub(crate) off_paper: bool,
    pub(crate) title_block_collision: bool,
    /// Primary row treatment only. Summary facts use the independent flags
    /// above so off-paper content remains part of the outside-border total
    /// and title-block collisions remain separately reviewable.
    pub(crate) severity: DrawingSheetOverflowSeverity,
    pub(crate) sheet_coordinates_mm: (f64, f64),
    pub(crate) zone: Option<String>,
    pub(crate) bounds: WorldRect,
}

impl DrawingSheetOverflowItem {
    pub(crate) fn center(&self) -> Point {
        Point::new(
            ((self.bounds.min_x + self.bounds.max_x) * 0.5).round() as i32,
            ((self.bounds.min_y + self.bounds.max_y) * 0.5).round() as i32,
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetOverflowSummary {
    pub(crate) outside_border: usize,
    pub(crate) off_paper: usize,
    pub(crate) title_block_collisions: usize,
    pub(crate) first_target: Option<DrawingSheetOverflowTarget>,
    pub(crate) items: Vec<DrawingSheetOverflowItem>,
}

impl DrawingSheetOverflowSummary {
    pub(crate) fn is_clear(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn finding_count(&self) -> usize {
        self.items.len()
    }
}

/// Resolve the non-blocking authored-content advisories used by both the
/// canvas and inspector. Keeping one report authority prevents their counts
/// and navigation targets from drifting.
pub(crate) fn drawing_sheet_overflow_summary(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> DrawingSheetOverflowSummary {
    let items = overflow_items(state, symbol_context, &sheet.geometry);
    DrawingSheetOverflowSummary {
        outside_border: items.iter().filter(|item| item.outside_border).count(),
        off_paper: items.iter().filter(|item| item.off_paper).count(),
        title_block_collisions: items
            .iter()
            .filter(|item| item.title_block_collision)
            .count(),
        first_target: items.first().map(|item| item.target),
        items,
    }
}

/// Select and center the first current outside-sheet item.
///
/// The report is recomputed at invocation time so an inspector action cannot
/// navigate to an object that was moved or removed after the prior frame.
pub(crate) fn show_first_drawing_sheet_overflow(state: &mut AppState) -> bool {
    let first_target = {
        let symbol_context = SchematicSymbolContext::from_state(state);
        let sheet = ActiveDrawingSheet::resolve(state);
        drawing_sheet_overflow_summary(state, &symbol_context, &sheet).first_target
    };
    first_target.is_some_and(|target| show_drawing_sheet_overflow_target(state, target))
}

/// Select and center one exact current outside-sheet item.
pub(crate) fn show_drawing_sheet_overflow_target(
    state: &mut AppState,
    target: DrawingSheetOverflowTarget,
) -> bool {
    let center = {
        let symbol_context = SchematicSymbolContext::from_state(state);
        let sheet = ActiveDrawingSheet::resolve(state);
        drawing_sheet_overflow_summary(state, &symbol_context, &sheet)
            .items
            .into_iter()
            .find(|item| item.target == target)
            .map(|item| item.center())
    };
    let Some(center) = center else {
        return false;
    };
    match target {
        DrawingSheetOverflowTarget::Component(id) => {
            state.schematic.selection.select_only_component(id);
        }
        DrawingSheetOverflowTarget::Wire(id) => {
            state.schematic.selection.select_only_wire(id);
        }
        DrawingSheetOverflowTarget::Bus(id) => {
            state.schematic.selection.select_only_bus(id);
        }
        DrawingSheetOverflowTarget::BusTap(id) => {
            state.schematic.selection.select_only_bus_tap(id);
        }
        DrawingSheetOverflowTarget::Junction(id) => {
            let Some(position) = state
                .schematic
                .junctions
                .iter()
                .find(|junction| junction.id == id)
                .map(|junction| junction.pos)
            else {
                return false;
            };
            state.schematic.selection.select_only_junction(position);
        }
        DrawingSheetOverflowTarget::NetLabel(id) => {
            state.schematic.selection.select_only_net_label(id);
        }
        DrawingSheetOverflowTarget::DesignNote(id) => {
            state.schematic.selection.select_only_design_note(id);
        }
        DrawingSheetOverflowTarget::DocumentationShape(id) => {
            state
                .schematic
                .selection
                .select_only_documentation_shape(id);
        }
    }
    state.schematic.center_request = Some(center);
    true
}

#[derive(Debug, Clone)]
struct PrintableObjectBounds<'a> {
    target: DrawingSheetOverflowTarget,
    identity: String,
    kind: &'static str,
    bounds: WorldRect,
    /// Conductors retain their exact routed centerline so concave route
    /// envelopes cannot manufacture title-block collisions. Instances use
    /// their symbol bounds because that rectangle is their printable extent.
    route: Option<Cow<'a, [Point]>>,
}

/// Compact, modal-safe snapshot of one durable object that hardcopy emits.
///
/// Page Setup cannot borrow the live schematic while its modal state is
/// active, so the preview takes this owned representation from the same
/// object/bounds authority as canvas overflow detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrawingSheetPrintablePreviewKind {
    Component,
    Conductor,
    Junction,
    NetLabel,
    DesignNote,
    DocumentationShape,
}

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetPrintablePreview {
    pub(crate) kind: DrawingSheetPrintablePreviewKind,
    pub(crate) bounds: WorldRect,
    pub(crate) path: Vec<Point>,
}

impl PrintableObjectBounds<'_> {
    fn is_contained_by(&self, rect: WorldRect) -> bool {
        self.route.as_ref().map_or_else(
            || rect.contains_rect(self.bounds),
            |route| {
                route
                    .iter()
                    .all(|point| rect.contains_point(f64::from(point.x), f64::from(point.y)))
            },
        )
    }

    fn intersects(&self, rect: WorldRect) -> bool {
        self.route.as_ref().map_or_else(
            || rect.intersects(self.bounds),
            |route| polyline_intersects_rect(route, rect),
        )
    }
}

fn overflow_items(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    geometry: &DrawingSheetGeometry,
) -> Vec<DrawingSheetOverflowItem> {
    active_object_bounds(state, symbol_context)
        .into_iter()
        .filter_map(|object| {
            let outside_border = !object.is_contained_by(geometry.content_area);
            let off_paper = !object.is_contained_by(geometry.paper);
            let title_block_collision = geometry
                .title_block
                .is_some_and(|title_block| object.intersects(title_block));
            (outside_border || title_block_collision).then_some(DrawingSheetOverflowItem {
                outside_border,
                off_paper,
                title_block_collision,
                severity: if off_paper {
                    DrawingSheetOverflowSeverity::OffPaper
                } else if title_block_collision {
                    DrawingSheetOverflowSeverity::TitleBlockCollision
                } else {
                    DrawingSheetOverflowSeverity::OutsideBorder
                },
                sheet_coordinates_mm: geometry
                    .world_to_sheet_mm(object.bounds.min_x, object.bounds.min_y),
                zone: geometry.zone_reference(
                    (object.bounds.min_x + object.bounds.max_x) * 0.5,
                    (object.bounds.min_y + object.bounds.max_y) * 0.5,
                ),
                target: object.target,
                identity: object.identity,
                kind: object.kind,
                bounds: object.bounds,
            })
        })
        .collect()
}

fn active_object_bounds<'a>(
    state: &'a AppState,
    symbol_context: &SchematicSymbolContext,
) -> Vec<PrintableObjectBounds<'a>> {
    let mut bounds = Vec::new();
    for component in &state.schematic.components {
        if object_is_on_active_sheet(state, component.id) {
            let (min, max) = symbol_context.component_bounds(component);
            // Hardcopy always emits the retained instance name and value at
            // the instance anchor. Its semantic bounds authority gives the
            // longer label two world units per source character, so include
            // the same deterministic horizontal extent in Page Setup and
            // canvas overflow rather than reviewing only the symbol body.
            let label_width = component
                .name
                .len()
                .max(component.value.len())
                .saturating_mul(2)
                .min(i32::MAX as usize) as i32;
            let label_max =
                Point::new(component.pos.x.saturating_add(label_width), component.pos.y);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Component(component.id),
                identity: if component.name.trim().is_empty() {
                    format!("instance:{}", component.id)
                } else {
                    component.name.clone()
                },
                kind: DrawingSheetOverflowTarget::Component(component.id).kind_label(),
                bounds: WorldRect::from_points(
                    Point::new(min.x.min(component.pos.x), min.y.min(component.pos.y)),
                    Point::new(max.x.max(label_max.x), max.y.max(label_max.y)),
                ),
                route: None,
            });
        }
    }
    for wire in &state.schematic.wires {
        if object_is_on_active_sheet(state, wire.id)
            && let Some(rect) = points_bounds(&wire.points)
        {
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Wire(wire.id),
                identity: format!("wire:{}", wire.id),
                kind: DrawingSheetOverflowTarget::Wire(wire.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Borrowed(&wire.points)),
            });
        }
    }
    for bus in &state.schematic.buses {
        if object_is_on_active_sheet(state, bus.id)
            && let Some(rect) = points_bounds(&bus.points)
        {
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Bus(bus.id),
                identity: bus
                    .declaration
                    .as_ref()
                    .map_or_else(|| format!("bus:{}", bus.id), ToString::to_string),
                kind: DrawingSheetOverflowTarget::Bus(bus.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Borrowed(&bus.points)),
            });
        }
    }
    for tap in &state.schematic.bus_taps {
        if object_is_on_active_sheet(state, tap.id) {
            let route = crate::schematic::bus_geometry::bus_tap_route_points(tap);
            let Some(rect) = points_bounds(&route) else {
                continue;
            };
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::BusTap(tap.id),
                identity: format!("tap:{} \u{00b7} {}", tap.id, tap.slice),
                kind: DrawingSheetOverflowTarget::BusTap(tap.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Owned(route)),
            });
        }
    }
    for junction in &state.schematic.junctions {
        if object_is_on_active_sheet(state, junction.id) {
            // Hardcopy renders an exact 900 µm filled junction dot. Include
            // that physical radius rather than treating the junction as a
            // dimensionless point at the paper edge.
            const JUNCTION_RADIUS_WORLD: f64 = 900.0 / DRAWING_SHEET_WORLD_UNIT_UM as f64;
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Junction(junction.id),
                identity: format!("junction:{}", junction.id),
                kind: DrawingSheetOverflowTarget::Junction(junction.id).kind_label(),
                bounds: WorldRect {
                    min_x: f64::from(junction.pos.x) - JUNCTION_RADIUS_WORLD,
                    min_y: f64::from(junction.pos.y) - JUNCTION_RADIUS_WORLD,
                    max_x: f64::from(junction.pos.x) + JUNCTION_RADIUS_WORLD,
                    max_y: f64::from(junction.pos.y) + JUNCTION_RADIUS_WORLD,
                },
                route: None,
            });
        }
    }
    for label in &state.schematic.net_labels {
        if object_is_on_active_sheet(state, label.id) {
            let (min, max) = super::net_labels::world_bounds(label);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::NetLabel(label.id),
                identity: if label.name.trim().is_empty() {
                    format!("net-label:{}", label.id)
                } else {
                    label.name.clone()
                },
                kind: DrawingSheetOverflowTarget::NetLabel(label.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    for note in &state.schematic.design_notes {
        if object_is_on_active_sheet(state, note.id) {
            let (min, max) = super::design_notes::conservative_world_bounds(note);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::DesignNote(note.id),
                identity: format!("note:{} \u{00b7} {}", note.id, note.kind.label()),
                kind: DrawingSheetOverflowTarget::DesignNote(note.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    for shape in &state.schematic.documentation_shapes {
        if object_is_on_active_sheet(state, shape.id) {
            let (min, max) = super::documentation_shapes::world_bounds(shape);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::DocumentationShape(shape.id),
                identity: format!("shape:{} \u{00b7} {}", shape.id, shape.kind().label()),
                kind: DrawingSheetOverflowTarget::DocumentationShape(shape.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    bounds
}

pub(crate) fn drawing_sheet_printable_preview(
    state: &AppState,
) -> Vec<DrawingSheetPrintablePreview> {
    let symbol_context = SchematicSymbolContext::from_state(state);
    active_object_bounds(state, &symbol_context)
        .into_iter()
        .map(|object| {
            let kind = match object.target {
                DrawingSheetOverflowTarget::Component(_) => {
                    DrawingSheetPrintablePreviewKind::Component
                }
                DrawingSheetOverflowTarget::Wire(_)
                | DrawingSheetOverflowTarget::Bus(_)
                | DrawingSheetOverflowTarget::BusTap(_) => {
                    DrawingSheetPrintablePreviewKind::Conductor
                }
                DrawingSheetOverflowTarget::Junction(_) => {
                    DrawingSheetPrintablePreviewKind::Junction
                }
                DrawingSheetOverflowTarget::NetLabel(_) => {
                    DrawingSheetPrintablePreviewKind::NetLabel
                }
                DrawingSheetOverflowTarget::DesignNote(_) => {
                    DrawingSheetPrintablePreviewKind::DesignNote
                }
                DrawingSheetOverflowTarget::DocumentationShape(_) => {
                    DrawingSheetPrintablePreviewKind::DocumentationShape
                }
            };
            DrawingSheetPrintablePreview {
                kind,
                bounds: object.bounds,
                path: object.route.map(Cow::into_owned).unwrap_or_default(),
            }
        })
        .collect()
}

fn points_bounds(points: &[Point]) -> Option<WorldRect> {
    let first = *points.first()?;
    let mut min = first;
    let mut max = first;
    for point in &points[1..] {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }
    Some(WorldRect::from_points(min, max))
}

fn polyline_intersects_rect(points: &[Point], rect: WorldRect) -> bool {
    points
        .iter()
        .any(|point| rect.contains_point(f64::from(point.x), f64::from(point.y)))
        || points
            .windows(2)
            .any(|segment| segment_intersects_rect(segment[0], segment[1], rect))
}

/// Liang-Barsky clipping against an axis-aligned rectangle. Boundary contact
/// counts as an intersection, matching the drawing-sheet collision contract.
fn segment_intersects_rect(start: Point, end: Point, rect: WorldRect) -> bool {
    let x0 = f64::from(start.x);
    let y0 = f64::from(start.y);
    let dx = f64::from(end.x) - x0;
    let dy = f64::from(end.y) - y0;
    let mut entry = 0.0_f64;
    let mut exit = 1.0_f64;

    for (direction, distance) in [
        (-dx, x0 - rect.min_x),
        (dx, rect.max_x - x0),
        (-dy, y0 - rect.min_y),
        (dy, rect.max_y - y0),
    ] {
        if direction.abs() <= f64::EPSILON {
            if distance < 0.0 {
                return false;
            }
            continue;
        }
        let ratio = distance / direction;
        if direction < 0.0 {
            entry = entry.max(ratio);
        } else {
            exit = exit.min(ratio);
        }
        if entry > exit {
            return false;
        }
    }
    true
}

fn dashed_rect(painter: &Painter, rect: Rect, stroke: Stroke) {
    dashed_rect_pattern(painter, rect, stroke, 4.0, 3.0);
}

fn dashed_rect_pattern(
    painter: &Painter,
    rect: Rect,
    stroke: Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    let points = [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
        rect.left_top(),
    ];
    painter.extend(Shape::dashed_line(&points, stroke, dash_length, gap_length));
}

fn draw_zone_band(
    painter: &Painter,
    printable: Rect,
    drawing_area: Rect,
    zones: DrawingSheetZoneGrid,
    lod: DrawingSheetLod,
    zoom: f32,
    palette: DrawingSheetPalette,
) {
    let column_width = drawing_area.width() / f32::from(zones.columns);
    let row_height = drawing_area.height() / f32::from(zones.rows);
    let stroke = Stroke::new(1.0, palette.zone);
    let (top, right, bottom, left) = zone_edges(zones.edges);
    for index in 1..zones.columns {
        let x = drawing_area.left() + column_width * f32::from(index);
        if top {
            painter.line_segment(
                [pos2(x, printable.top()), pos2(x, drawing_area.top())],
                stroke,
            );
        }
        if bottom {
            painter.line_segment(
                [pos2(x, drawing_area.bottom()), pos2(x, printable.bottom())],
                stroke,
            );
        }
    }
    for index in 1..zones.rows {
        let y = drawing_area.top() + row_height * f32::from(index);
        if left {
            painter.line_segment(
                [pos2(printable.left(), y), pos2(drawing_area.left(), y)],
                stroke,
            );
        }
        if right {
            painter.line_segment(
                [pos2(drawing_area.right(), y), pos2(printable.right(), y)],
                stroke,
            );
        }
    }
    if !lod.show_zone_labels || !zone_edge_labels_visible(zones.labels) {
        return;
    }
    let font = theme::mono(
        drawing_sheet_font_size(zoom, 8.0, 8.0),
        theme::FontWeight::Medium,
    );
    let top_y = (printable.top() + drawing_area.top()) * 0.5;
    let bottom_y = (drawing_area.bottom() + printable.bottom()) * 0.5;
    for column in 0..zones.columns {
        let x = drawing_area.left() + column_width * (f32::from(column) + 0.5);
        let label = zone_column_label(zones.labels, usize::from(column));
        if top {
            painter.text(
                pos2(x, top_y),
                Align2::CENTER_CENTER,
                &label,
                font.clone(),
                palette.zone_ink,
            );
        }
        if bottom {
            painter.text(
                pos2(x, bottom_y),
                Align2::CENTER_CENTER,
                label,
                font.clone(),
                palette.zone_ink,
            );
        }
    }
    let left_x = (printable.left() + drawing_area.left()) * 0.5;
    let right_x = (drawing_area.right() + printable.right()) * 0.5;
    for row in 0..zones.rows {
        let y = drawing_area.top() + row_height * (f32::from(row) + 0.5);
        let label = zone_row_label(zones.labels, usize::from(row));
        if left {
            painter.text(
                pos2(left_x, y),
                Align2::CENTER_CENTER,
                &label,
                font.clone(),
                palette.zone_ink,
            );
        }
        if right {
            painter.text(
                pos2(right_x, y),
                Align2::CENTER_CENTER,
                label,
                font.clone(),
                palette.zone_ink,
            );
        }
    }
}

const fn zone_edge_labels_visible(labels: DrawingSheetZoneLabels) -> bool {
    !matches!(labels, DrawingSheetZoneLabels::Coordinates)
}

fn zone_edges(edges: DrawingSheetZoneEdges) -> (bool, bool, bool, bool) {
    match edges {
        DrawingSheetZoneEdges::All => (true, true, true, true),
        DrawingSheetZoneEdges::TopAndLeft => (true, false, false, true),
        DrawingSheetZoneEdges::BottomAndRight => (false, true, true, false),
    }
}

fn zone_alpha_label(index: usize) -> String {
    // Engineering drawing zones omit I, O, Q, and Z to avoid ambiguous
    // references in printed reviews.
    const LETTERS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXY";
    LETTERS.get(index).map_or_else(
        || (index + 1).to_string(),
        |letter| char::from(*letter).to_string(),
    )
}

fn zone_column_label(labels: DrawingSheetZoneLabels, index: usize) -> String {
    match labels {
        DrawingSheetZoneLabels::AlphaNumeric | DrawingSheetZoneLabels::Coordinates => {
            (index + 1).to_string()
        }
        DrawingSheetZoneLabels::NumericAlpha => zone_alpha_label(index),
    }
}

fn zone_row_label(labels: DrawingSheetZoneLabels, index: usize) -> String {
    match labels {
        DrawingSheetZoneLabels::AlphaNumeric => zone_alpha_label(index),
        DrawingSheetZoneLabels::NumericAlpha | DrawingSheetZoneLabels::Coordinates => {
            (index + 1).to_string()
        }
    }
}

fn zone_reference_label(labels: DrawingSheetZoneLabels, column: usize, row: usize) -> String {
    match labels {
        DrawingSheetZoneLabels::AlphaNumeric => {
            format!("{}{}", zone_alpha_label(row), column + 1)
        }
        DrawingSheetZoneLabels::NumericAlpha => {
            format!("{}{}", row + 1, zone_alpha_label(column))
        }
        DrawingSheetZoneLabels::Coordinates => format!("X{} Y{}", column + 1, row + 1),
    }
}

fn draw_title_block(
    painter: &Painter,
    block: Rect,
    lod: DrawingSheetLod,
    zoom: f32,
    palette: DrawingSheetPalette,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) {
    let rotation = sheet.format.title_block.rotation;
    let angle = title_block_rotation_angle(rotation);
    // Domain geometry exposes the rotated bounding box. Author the complete
    // block in its natural orientation, then rotate every primitive around
    // the shared center so the outline, grid, markers, and text cannot drift.
    let authored_block = if angle == 0.0 {
        block
    } else {
        Rect::from_center_size(block.center(), vec2(block.height(), block.width()))
    };
    let transform = |point: Pos2| rotate_title_block_point(point, authored_block.center(), angle);
    let outline = [
        transform(authored_block.left_top()),
        transform(authored_block.right_top()),
        transform(authored_block.right_bottom()),
        transform(authored_block.left_bottom()),
    ];
    painter.add(Shape::convex_polygon(
        outline.to_vec(),
        palette.block,
        Stroke::new(1.25, palette.border),
    ));
    let effective_template = sheet.geometry.physical.effective_title_block_template;
    let mut field_grid = authored_block;
    if let Some(logo) = sheet.format.title_block_logo(effective_template)
        && let Some((template_width_um, _)) =
            sheet.format.title_block_dimensions_um(effective_template)
    {
        let reserved_width =
            authored_block.width() * (logo.reserved_width_um() as f32 / template_width_um as f32);
        field_grid.min.x = (authored_block.left() + reserved_width).min(authored_block.right());
        painter.line_segment(
            [
                transform(pos2(field_grid.left(), authored_block.top())),
                transform(pos2(field_grid.left(), authored_block.bottom())),
            ],
            Stroke::new(0.8, palette.zone),
        );
        draw_managed_title_logo(
            painter,
            Rect::from_min_max(
                authored_block.left_top(),
                pos2(field_grid.left(), authored_block.bottom()),
            ),
            &transform,
            logo,
            palette,
        );
    }
    if !lod.show_title_block_fields || field_grid.width() < 80.0 || field_grid.height() < 30.0 {
        return;
    }
    let fields = resolved_title_block_fields(state, sheet);
    if fields.is_empty() {
        return;
    }
    let fields = fields.into_iter().collect::<Vec<_>>();
    let Some(rows) = sheet
        .format
        .title_block_rows(sheet.geometry.physical.effective_title_block_template)
    else {
        return;
    };
    let max_chars = crate::state::drawing_sheet_title_cell_capacity(
        &sheet.format,
        &sheet.geometry.physical,
        fields.len(),
    )
    .unwrap_or(4);
    let columns = fields.len().div_ceil(rows).max(1);
    let cell_width = field_grid.width() / columns as f32;
    let cell_height = field_grid.height() / rows as f32;
    for row in 1..rows {
        let y = field_grid.top() + cell_height * row as f32;
        painter.line_segment(
            [
                transform(pos2(field_grid.left(), y)),
                transform(pos2(field_grid.right(), y)),
            ],
            Stroke::new(0.8, palette.zone),
        );
    }
    for (index, field) in fields.into_iter().enumerate() {
        let row = index / columns;
        let column = index % columns;
        if column > 0 {
            let x = field_grid.left() + cell_width * column as f32;
            let top = field_grid.top() + cell_height * row as f32;
            painter.line_segment(
                [
                    transform(pos2(x, top)),
                    transform(pos2(x, top + cell_height)),
                ],
                Stroke::new(0.8, palette.zone),
            );
        }
        let authored_cell = Rect::from_min_max(
            pos2(
                field_grid.left() + cell_width * column as f32,
                field_grid.top() + cell_height * row as f32,
            ),
            pos2(
                field_grid.left() + cell_width * (column + 1) as f32,
                field_grid.top() + cell_height * (row + 1) as f32,
            ),
        )
        .shrink2(vec2(4.0 * zoom.max(1.0), 2.0 * zoom.max(1.0)));
        let cell = Rect::from_two_pos(
            transform(authored_cell.left_top()),
            transform(authored_cell.right_bottom()),
        );
        let clipped = painter.with_clip_rect(cell);
        paint_title_text(
            &clipped,
            transform(authored_cell.left_top()),
            Align2::LEFT_TOP,
            field.id.display_label().to_uppercase(),
            theme::mono(
                drawing_sheet_font_size(zoom, 5.5, 7.0),
                theme::FontWeight::Regular,
            ),
            palette.block_label,
            rotation,
        );
        if field.authority == DrawingSheetTitleFieldValueAuthority::Automatic {
            clipped.circle_filled(
                transform(pos2(authored_cell.right() - 2.0, authored_cell.top() + 2.0)),
                1.25 * zoom.max(1.0),
                palette.block_label,
            );
        }
        paint_title_text(
            &clipped,
            transform(pos2(authored_cell.left(), authored_cell.bottom())),
            Align2::LEFT_BOTTOM,
            truncate_canvas_title_value(&field.value, max_chars),
            theme::mono(
                drawing_sheet_font_size(zoom, 7.0, 9.0),
                theme::FontWeight::Medium,
            ),
            palette.block_ink,
            rotation,
        );
    }
}

fn draw_managed_title_logo(
    painter: &Painter,
    reserved: Rect,
    transform: &impl Fn(Pos2) -> Pos2,
    logo: &crate::state::DrawingSheetManagedLogo,
    palette: DrawingSheetPalette,
) {
    let inset = (reserved.width().min(reserved.height()) * 0.08).max(1.0);
    let content = reserved.shrink(inset);
    if content.width() <= 0.0 || content.height() <= 0.0 {
        return;
    }
    let basis = f32::from(DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS);
    let stroke = Stroke::new(0.9, palette.block_ink);
    for primitive in logo.primitives() {
        let points = primitive
            .points()
            .iter()
            .map(|point| {
                transform(pos2(
                    egui::lerp(
                        content.left()..=content.right(),
                        f32::from(point.x()) / basis,
                    ),
                    egui::lerp(
                        content.top()..=content.bottom(),
                        f32::from(point.y()) / basis,
                    ),
                ))
            })
            .collect::<Vec<_>>();
        if primitive.filled() {
            painter.add(Shape::convex_polygon(points, palette.block_ink, stroke));
        } else if primitive.closed() {
            painter.add(Shape::closed_line(points, stroke));
        } else {
            painter.add(Shape::line(points, stroke));
        }
    }
}

pub(crate) fn resolved_title_block_fields(
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) -> Vec<crate::state::ResolvedDrawingSheetTitleField> {
    let authority_values = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .map(|id| (id, default_title_field_value(id, state, sheet)))
        .collect();
    crate::state::resolve_drawing_sheet_title_fields(&sheet.format, &authority_values)
}

fn truncate_canvas_title_value(value: &str, max_chars: usize) -> String {
    // Capacity is derived from physical title-block geometry, so canvas zoom
    // and monitor DPI cannot disagree with Page Setup or hardcopy output.
    truncate_title_text(value, max_chars)
}

fn truncate_title_text(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_owned();
    }
    let mut value = value
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    value.push('…');
    value
}

fn title_block_rotation_angle(rotation: DrawingSheetTitleBlockRotation) -> f32 {
    match rotation {
        DrawingSheetTitleBlockRotation::Upright => 0.0,
        DrawingSheetTitleBlockRotation::Clockwise90 => std::f32::consts::FRAC_PI_2,
        DrawingSheetTitleBlockRotation::CounterClockwise90 => -std::f32::consts::FRAC_PI_2,
    }
}

fn rotate_title_block_point(point: Pos2, center: Pos2, angle: f32) -> Pos2 {
    if angle == 0.0 {
        return point;
    }
    let offset = point - center;
    let (sin, cos) = angle.sin_cos();
    center
        + vec2(
            offset.x * cos - offset.y * sin,
            offset.x * sin + offset.y * cos,
        )
}

fn paint_title_text(
    painter: &Painter,
    position: Pos2,
    anchor: Align2,
    text: impl ToString,
    font: egui::FontId,
    color: Color32,
    rotation: DrawingSheetTitleBlockRotation,
) {
    let angle = title_block_rotation_angle(rotation);
    if angle == 0.0 {
        painter.text(position, anchor, text, font, color);
        return;
    }
    let galley = painter.layout_no_wrap(text.to_string(), font, color);
    let rect = anchor.anchor_size(position, galley.size());
    painter.add(
        TextShape::new(rect.min, galley, color)
            .with_override_text_color(color)
            .with_angle_and_anchor(angle, anchor),
    );
}

fn default_title_field_value(
    field: DrawingSheetTitleFieldId,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) -> String {
    match field {
        DrawingSheetTitleFieldId::Project => state.workspace.project.name().to_owned(),
        DrawingSheetTitleFieldId::CellView => state.workspace.active_view.display_path().to_owned(),
        DrawingSheetTitleFieldId::SheetTitle => sheet.sheet_name.clone(),
        DrawingSheetTitleFieldId::Page => sheet.page_label.clone(),
        DrawingSheetTitleFieldId::Revision => state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .document_control
            .revision
            .clone(),
        DrawingSheetTitleFieldId::Format => sheet.format_label(),
        DrawingSheetTitleFieldId::Scale => match sheet.format.title_block.scale {
            DrawingSheetScale::NotToScale => "NTS".to_owned(),
            DrawingSheetScale::Ratio {
                drawing_units,
                reality_units,
            } => format!("{drawing_units}:{reality_units}"),
        },
        DrawingSheetTitleFieldId::Date => state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .document_control
            .display_date()
            .to_owned(),
        DrawingSheetTitleFieldId::Organization
        | DrawingSheetTitleFieldId::DocumentId
        | DrawingSheetTitleFieldId::Classification => state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .title_block_field_values
            .get(&field)
            .cloned()
            .unwrap_or_default(),
        DrawingSheetTitleFieldId::DrawnBy
        | DrawingSheetTitleFieldId::CheckedBy
        | DrawingSheetTitleFieldId::ApprovedBy => String::new(),
    }
}

fn draw_registration_marks(
    painter: &Painter,
    _paper: Rect,
    printable: Rect,
    palette: DrawingSheetPalette,
) {
    let arm = 7.0;
    let stroke = Stroke::new(1.0, palette.zone);
    for (point, dx, dy) in [
        (printable.left_top(), 1.0, 1.0),
        (printable.right_top(), -1.0, 1.0),
        (printable.left_bottom(), 1.0, -1.0),
        (printable.right_bottom(), -1.0, -1.0),
    ] {
        painter.line_segment(
            [
                point - vec2(dx * arm * 0.4, 0.0),
                point + vec2(dx * arm, 0.0),
            ],
            stroke,
        );
        painter.line_segment(
            [
                point - vec2(0.0, dy * arm * 0.4),
                point + vec2(0.0, dy * arm),
            ],
            stroke,
        );
    }
}

fn draw_folding_marks(painter: &Painter, paper: Rect, zoom: f32, palette: DrawingSheetPalette) {
    let stroke = Stroke::new(1.0, palette.zone);
    let arm = 6.0;
    let fold_pitch = 105.0 * DRAWING_SHEET_UNITS_PER_MM as f32 * zoom;
    if !fold_pitch.is_finite() || fold_pitch <= 0.0 {
        return;
    }
    let mut y = paper.top() + fold_pitch;
    while y < paper.bottom() - fold_pitch * 0.25 {
        painter.line_segment([pos2(paper.left(), y), pos2(paper.left() + arm, y)], stroke);
        y += fold_pitch;
    }
}

fn draw_origin(painter: &Painter, paper: Rect, zoom: f32, palette: DrawingSheetPalette) {
    let stroke = Stroke::new(1.0, palette.zone_ink);
    painter.line_segment(
        [paper.left_top(), paper.left_top() + vec2(9.0, 0.0)],
        stroke,
    );
    painter.line_segment(
        [paper.left_top(), paper.left_top() + vec2(0.0, 9.0)],
        stroke,
    );
    painter.text(
        paper.left_top() + vec2(5.0, 11.0),
        Align2::LEFT_TOP,
        "0,0",
        theme::mono(
            drawing_sheet_font_size(zoom, 7.0, 8.0),
            theme::FontWeight::Regular,
        ),
        palette.zone_ink,
    );
}

fn draw_dimensions(
    painter: &Painter,
    paper: Rect,
    zoom: f32,
    palette: DrawingSheetPalette,
    sheet: &ActiveDrawingSheet,
) {
    let (width, height) = sheet.format.oriented_dimensions_um();
    let unit = sheet.format.display_unit;
    painter.text(
        paper.center_top() - vec2(0.0, 7.0),
        Align2::CENTER_BOTTOM,
        format!("{} {}", unit.format_um(width), unit.suffix()),
        theme::mono(
            drawing_sheet_font_size(zoom, 7.0, 8.0),
            theme::FontWeight::Regular,
        ),
        palette.zone_ink,
    );
    painter.text(
        paper.left_center() - vec2(7.0, 0.0),
        Align2::RIGHT_CENTER,
        format!("{} {}", unit.format_um(height), unit.suffix()),
        theme::mono(
            drawing_sheet_font_size(zoom, 7.0, 8.0),
            theme::FontWeight::Regular,
        ),
        palette.zone_ink,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Component, ComponentType, DesignNote, DesignNoteKind, DocumentationShape,
        DocumentationShapeGeometry, DrawingSheetLayerVisibility, Junction, NetLabel,
        SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat, Wire,
    };

    #[test]
    fn a4_landscape_geometry_uses_the_mockup_origin_and_exact_calibration() {
        let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::standard(
            SchematicPageSize::A4,
            SchematicPageOrientation::Landscape,
        ));

        assert_eq!(
            geometry.physical.paper,
            DrawingSheetRect {
                x_um: 0,
                y_um: 0,
                width_um: 297_000,
                height_um: 210_000,
            }
        );
        assert_eq!(
            geometry.physical.printable,
            DrawingSheetRect {
                x_um: 20_000,
                y_um: 10_000,
                width_um: 267_000,
                height_um: 190_000,
            }
        );
        assert_eq!(
            geometry.physical.drawing_area,
            DrawingSheetRect {
                x_um: 25_000,
                y_um: 15_000,
                width_um: 257_000,
                height_um: 180_000,
            }
        );
        assert_eq!(geometry.paper.min_x, -140.0);
        assert_eq!(geometry.paper.min_y, -40.0);
        assert_eq!(geometry.paper.max_x, 1_048.0);
        assert_eq!(geometry.paper.max_y, 800.0);
        assert_eq!(
            geometry.physical.zones,
            Some(DrawingSheetZoneGrid {
                columns: 4,
                rows: 4,
                labels: DrawingSheetZoneLabels::AlphaNumeric,
                edges: DrawingSheetZoneEdges::All,
            })
        );
    }

    #[test]
    fn letter_dimensions_retain_fractional_world_edges_without_rounding() {
        let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::standard(
            SchematicPageSize::UsLetter,
            SchematicPageOrientation::Portrait,
        ));

        assert_eq!(geometry.physical.paper.width_um, 215_900);
        assert!((geometry.paper.max_x - 723.6).abs() < f64::EPSILON);
        assert!((geometry.paper.max_y - 1_077.6).abs() < f64::EPSILON);
    }

    #[test]
    fn level_of_detail_is_driven_by_physical_pixel_density() {
        assert_eq!(
            DrawingSheetLod::resolve(0.1, 1.0).tier,
            DrawingSheetLodTier::Thumbnail
        );
        assert_eq!(
            DrawingSheetLod::resolve(0.25, 1.0).tier,
            DrawingSheetLodTier::Outline
        );
        assert_eq!(
            DrawingSheetLod::resolve(0.5, 1.0).tier,
            DrawingSheetLodTier::Working
        );
        assert_eq!(
            DrawingSheetLod::resolve(1.5, 1.0).tier,
            DrawingSheetLodTier::Detail
        );
    }

    #[test]
    fn zone_reference_and_sheet_coordinates_share_one_origin() {
        let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::default());
        assert_eq!(geometry.world_to_sheet_mm(-140.0, -40.0), (0.0, 0.0));
        assert_eq!(
            geometry.zone_reference(
                geometry.drawing_area.min_x + 1.0,
                geometry.drawing_area.min_y + 1.0
            ),
            Some("A1".to_owned())
        );
    }

    #[test]
    fn coordinates_only_zones_retain_bands_but_suppress_labels_and_references() {
        let format = SchematicSheetFormat::default()
            .try_update(|draft| draft.zones.labels = DrawingSheetZoneLabels::Coordinates)
            .expect("coordinates-only zones are valid");
        let sheet = ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: "top".to_owned(),
            page_label: "1 of 1".to_owned(),
        };
        let point = (
            sheet.geometry.drawing_area.min_x + 1.0,
            sheet.geometry.drawing_area.min_y + 1.0,
        );

        assert!(!zone_edge_labels_visible(
            DrawingSheetZoneLabels::Coordinates
        ));
        let context = egui::Context::default();
        let output = context.run_ui(Default::default(), |context| {
            draw_zone_band(
                &context.layer_painter(egui::LayerId::background()),
                Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0)),
                Rect::from_min_max(pos2(10.0, 10.0), pos2(90.0, 90.0)),
                DrawingSheetZoneGrid {
                    columns: 4,
                    rows: 4,
                    labels: DrawingSheetZoneLabels::Coordinates,
                    edges: DrawingSheetZoneEdges::All,
                },
                DrawingSheetLod::resolve(1.5, 1.0),
                1.5,
                DrawingSheetPalette::resolve(&Tokens::get(context)),
            );
        });
        assert_eq!(
            output.shapes.len(),
            12,
            "coordinates-only mode retains all twelve ruled band segments"
        );
        assert_eq!(sheet.geometry.zone_reference(point.0, point.1), None);
        assert!(
            !sheet.cursor_status(point.0, point.1).contains("zone"),
            "coordinates-only mode must not append a zone reference to status text"
        );
    }

    #[test]
    fn drawing_sheet_text_has_a_readable_floor_and_scales_from_fit_sheet_zoom() {
        assert_eq!(drawing_sheet_font_size(0.25, 7.0, 9.0), 9.0);
        assert_eq!(drawing_sheet_font_size(0.55, 7.0, 9.0), 9.0);
        assert!(
            drawing_sheet_font_size(0.6, 7.0, 9.0) > drawing_sheet_font_size(0.55, 7.0, 9.0),
            "text must begin growing as soon as the operator zooms in from a normal fit-sheet view"
        );
        assert!(drawing_sheet_font_size(2.0, 5.5, 7.0) > drawing_sheet_font_size(1.0, 5.5, 7.0));
    }

    #[test]
    fn cursor_status_uses_the_configured_unit_and_explicit_sheet_context() {
        let format = SchematicSheetFormat::default()
            .try_update(|draft| draft.display_unit = DrawingSheetDisplayUnit::Inches)
            .expect("inch display unit is valid");
        let sheet = ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: "top".to_owned(),
            page_label: "1 of 1".to_owned(),
        };
        let one_inch = 25.4 * DRAWING_SHEET_UNITS_PER_MM;

        assert_eq!(
            sheet.cursor_status(
                DRAWING_SHEET_ORIGIN.0 + one_inch,
                DRAWING_SHEET_ORIGIN.1 + one_inch
            ),
            "x 1 · y 1 in · zone A1"
        );
        assert_eq!(
            sheet.cursor_status(DRAWING_SHEET_ORIGIN.0 - one_inch, DRAWING_SHEET_ORIGIN.1),
            "x -1 · y 0 in · off sheet"
        );
    }

    #[test]
    fn fit_and_detail_contracts_match_the_permanent_canvas_mockup() {
        assert_eq!(FIT_SCREEN_INSET, 26.0);
        let detail = DrawingSheetLod::resolve(1.5, 1.0);
        assert!(detail.show_origin);
        assert!(detail.show_dimensions);
    }

    #[test]
    fn overflow_advisories_remain_visible_in_thumbnail_views() {
        let thumbnail = DrawingSheetLod::resolve(0.1, 1.0);
        assert_eq!(thumbnail.tier, DrawingSheetLodTier::Thumbnail);
        assert!(!thumbnail.show_grid);
        assert!(overflow_advisories_visible(thumbnail.tier));
    }

    #[test]
    fn title_block_rotation_preserves_center_and_swaps_complete_block_extent() {
        let target = Rect::from_center_size(pos2(200.0, 120.0), vec2(40.0, 100.0));
        let authored =
            Rect::from_center_size(target.center(), vec2(target.height(), target.width()));
        let angle = title_block_rotation_angle(DrawingSheetTitleBlockRotation::Clockwise90);
        let corners = [
            rotate_title_block_point(authored.left_top(), authored.center(), angle),
            rotate_title_block_point(authored.right_top(), authored.center(), angle),
            rotate_title_block_point(authored.right_bottom(), authored.center(), angle),
            rotate_title_block_point(authored.left_bottom(), authored.center(), angle),
        ];
        let bounds = Rect::from_points(&corners);

        assert_eq!(bounds.center(), target.center());
        assert!((bounds.width() - target.width()).abs() < f32::EPSILON);
        assert!((bounds.height() - target.height()).abs() < f32::EPSILON);
    }

    #[test]
    fn title_block_text_uses_an_ellipsis_instead_of_a_clipped_final_glyph() {
        assert_eq!(truncate_title_text("ABCDEFG", 5), "ABCD…");
        assert_eq!(truncate_title_text("ABCD", 5), "ABCD");
        assert_eq!(truncate_title_text("ééééé", 4), "ééé…");
    }

    #[test]
    fn paper_edge_has_no_visibility_switch() {
        let layers = DrawingSheetLayerVisibility::default();
        assert!(layers.margins);
        assert!(layers.zones);
        assert!(layers.title_block);
        assert!(layers.registration_marks);
        assert!(layers.dimensions_at_detail_zoom);
    }

    #[test]
    fn overflow_report_retains_exact_item_identity_severity_coordinates_and_zone() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(11, ComponentType::Resistor, Point::new(1_100, 100))
                .with_name_value("R_OUT", "1k"),
        );
        state.schematic.wires.push(Wire::new(
            21,
            vec![Point::new(-100, 100), Point::new(-80, 100)],
        ));
        state.schematic.wires.push(Wire::new(
            22,
            vec![Point::new(500, 650), Point::new(520, 650)],
        ));
        let symbol_context = SchematicSymbolContext::from_state(&state);
        let sheet = ActiveDrawingSheet::resolve(&state);

        let report = drawing_sheet_overflow_summary(&state, &symbol_context, &sheet);

        assert_eq!(report.items.len(), 3);
        assert_eq!(report.off_paper, 1);
        assert_eq!(report.outside_border, 2);
        assert_eq!(report.title_block_collisions, 1);
        assert_eq!(report.finding_count(), 3);
        assert_eq!(
            report.first_target,
            Some(DrawingSheetOverflowTarget::Component(11))
        );
        let instance = &report.items[0];
        assert_eq!(instance.identity, "R_OUT");
        assert_eq!(instance.kind, "Instance");
        assert!(instance.outside_border);
        assert!(instance.off_paper);
        assert!(!instance.title_block_collision);
        assert_eq!(instance.severity, DrawingSheetOverflowSeverity::OffPaper);
        let border_wire = report
            .items
            .iter()
            .find(|item| item.target == DrawingSheetOverflowTarget::Wire(21))
            .expect("outside-border wire");
        assert_eq!(border_wire.sheet_coordinates_mm, (10.0, 35.0));
        assert_eq!(border_wire.zone, None);
        let title_wire = report
            .items
            .iter()
            .find(|item| item.target == DrawingSheetOverflowTarget::Wire(22))
            .expect("title-block wire");
        assert_eq!(
            title_wire.severity,
            DrawingSheetOverflowSeverity::TitleBlockCollision
        );
        assert!(title_wire.zone.is_some());
    }

    #[test]
    fn every_durable_printable_object_class_participates_in_off_paper_review() {
        let mut state = AppState::default();
        state
            .schematic
            .junctions
            .push(Junction::new(31, Point::new(1_100, 100)));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(32, Point::new(1_100, 140), "OUTSIDE"));
        state.schematic.design_notes.push(
            DesignNote::new(
                33,
                Point::new(1_100, 180),
                DesignNoteKind::PlainText,
                "outside note",
            )
            .expect("valid design note"),
        );
        state.schematic.documentation_shapes.push(
            DocumentationShape::new(
                34,
                DocumentationShapeGeometry::Line {
                    start: Point::new(1_100, 220),
                    end: Point::new(1_140, 240),
                },
            )
            .expect("valid documentation line"),
        );
        let symbol_context = SchematicSymbolContext::from_state(&state);
        let sheet = ActiveDrawingSheet::resolve(&state);

        let report = drawing_sheet_overflow_summary(&state, &symbol_context, &sheet);

        assert_eq!(report.items.len(), 4);
        assert_eq!(report.outside_border, 4);
        assert_eq!(report.off_paper, 4);
        assert_eq!(
            report
                .items
                .iter()
                .map(|item| item.target)
                .collect::<Vec<_>>(),
            vec![
                DrawingSheetOverflowTarget::Junction(31),
                DrawingSheetOverflowTarget::NetLabel(32),
                DrawingSheetOverflowTarget::DesignNote(33),
                DrawingSheetOverflowTarget::DocumentationShape(34),
            ]
        );
        assert!(
            report
                .items
                .iter()
                .all(|item| item.severity == DrawingSheetOverflowSeverity::OffPaper)
        );
    }

    #[test]
    fn component_overflow_includes_the_hardcopy_name_and_value_extent() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(61, ComponentType::Resistor, Point::new(1_000, 100))
                .with_name_value("R_INSTANCE_NAME_REACHES_BEYOND_THE_PAPER_EDGE", "1k"),
        );
        let symbol_context = SchematicSymbolContext::from_state(&state);
        let sheet = ActiveDrawingSheet::resolve(&state);
        let (body_min, body_max) = symbol_context.component_bounds(&state.schematic.components[0]);
        let body = WorldRect::from_points(body_min, body_max);
        let object = active_object_bounds(&state, &symbol_context)
            .into_iter()
            .find(|object| object.target == DrawingSheetOverflowTarget::Component(61))
            .expect("component is printable");

        assert!(
            sheet.geometry.paper.contains_rect(body),
            "the symbol body itself remains on the authored paper"
        );
        assert!(
            !sheet.geometry.paper.contains_rect(object.bounds),
            "the retained hardcopy label extends the reviewed printable bounds"
        );
        let item = drawing_sheet_overflow_summary(&state, &symbol_context, &sheet)
            .items
            .into_iter()
            .find(|item| item.target == DrawingSheetOverflowTarget::Component(61))
            .expect("label-only off-paper overflow is reported");
        assert_eq!(item.severity, DrawingSheetOverflowSeverity::OffPaper);
    }

    #[test]
    fn conductor_title_collision_uses_segments_not_the_route_bounding_box() {
        let title_block = WorldRect {
            min_x: 10.0,
            min_y: 10.0,
            max_x: 20.0,
            max_y: 20.0,
        };
        let route = vec![Point::new(0, 15), Point::new(0, 0), Point::new(15, 0)];
        let route_bounds = points_bounds(&route).expect("the route has points");
        let mut state = AppState::default();
        state.schematic.wires.push(Wire::new(91, route));
        let symbol_context = SchematicSymbolContext::from_state(&state);
        let object = active_object_bounds(&state, &symbol_context)
            .into_iter()
            .find(|object| object.target == DrawingSheetOverflowTarget::Wire(91))
            .expect("the routed wire is printable on the active sheet");

        assert!(
            title_block.intersects(route_bounds),
            "the historical route-AABB implementation would report a collision"
        );
        assert!(
            !object.intersects(title_block),
            "neither segment of the L-shaped route intersects the title block"
        );
        assert_eq!(
            object.bounds, route_bounds,
            "navigation retains full bounds"
        );

        let mut geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::default());
        geometry.paper = WorldRect {
            min_x: -100.0,
            min_y: -100.0,
            max_x: 100.0,
            max_y: 100.0,
        };
        geometry.content_area = geometry.paper;
        geometry.title_block = Some(title_block);
        assert!(
            overflow_items(&state, &symbol_context, &geometry).is_empty(),
            "the overflow report must not contain the historical false-positive finding"
        );
    }

    #[test]
    fn overflow_navigation_selects_and_centers_the_exact_current_target() {
        let mut state = AppState::default();
        state.schematic.wires.push(Wire::new(
            41,
            vec![Point::new(1_080, 120), Point::new(1_120, 120)],
        ));
        let expected_center = {
            let symbol_context = SchematicSymbolContext::from_state(&state);
            let sheet = ActiveDrawingSheet::resolve(&state);
            drawing_sheet_overflow_summary(&state, &symbol_context, &sheet).items[0].center()
        };

        assert!(show_first_drawing_sheet_overflow(&mut state));
        assert_eq!(state.schematic.selection.single_wire(), Some(41));
        assert_eq!(state.schematic.center_request, Some(expected_center));
        assert!(!show_drawing_sheet_overflow_target(
            &mut state,
            DrawingSheetOverflowTarget::Wire(404)
        ));
    }
}
