//! Permanent authored drawing-sheet geometry and canvas presentation.
//!
//! The sheet is document geometry behind the schematic, not a print-preview
//! overlay. It never participates in hit testing, so wiring, selection, and
//! context-menu gestures behave identically on paper and on the surrounding
//! unbounded desk.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use egui::{
    Align2, Color32, FontId, Mesh, Painter, Pos2, Rect, Shape, Stroke, StrokeKind,
    epaint::{TessellationOptions, Tessellator, TextShape},
    pos2, vec2,
};

use crate::session::visibility::DrawingSheetLayerVisibility;
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    component::Component,
    design_note::DesignNote,
    documentation_shape::DocumentationShape,
    net_label::NetLabel,
};
use rspice_design_model::Point;
use rspice_design_model::design_management::{
    DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS, DRAWING_SHEET_TITLE_BLOCK_LETTERING,
    DRAWING_SHEET_TITLE_CAPTION_POSITION, DRAWING_SHEET_TITLE_CELL_PADDING_UM,
    DRAWING_SHEET_TITLE_VALUE_POSITION, DrawingSheetBorderTemplate, DrawingSheetDisplayUnit,
    DrawingSheetGeometry as AuthoredDrawingSheetGeometry, DrawingSheetLettering, DrawingSheetRect,
    DrawingSheetTitleBlockRotation, DrawingSheetZoneEdges, DrawingSheetZoneGrid,
    DrawingSheetZoneLabels, ResolvedDrawingSheetTitleField, SchematicSheetFormat,
    drawing_sheet_mono_em_um, drawing_sheet_title_layout, drawing_sheet_zone_lettering_um,
};
use rspice_ui_kit::theme;
use rspice_ui_kit::tokens::{Mode, Tokens};

use super::design_view::DesignView;
use super::grid::{GridStyle, draw_grid};
use super::symbol_context::SchematicSymbolContext;
use super::viewport::Viewport;

/// Canonical mockup calibration: four schematic world units per millimetre.
pub const DRAWING_SHEET_UNITS_PER_MM: f64 = 4.0;
/// Exact physical calibration paired with [`DRAWING_SHEET_UNITS_PER_MM`].
pub const DRAWING_SHEET_WORLD_UNIT_UM: u64 = 250;
/// Stable authored page origin. Format changes grow or shrink to the right and
/// bottom, so an existing object's sheet coordinates never move.
pub const DRAWING_SHEET_ORIGIN: (f64, f64) = (-140.0, -40.0);

pub const FIT_SCREEN_INSET: f64 = 26.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldRect {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
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

    pub const fn as_tuple(self) -> (f64, f64, f64, f64) {
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

    pub fn screen_rect(self, viewport: &Viewport) -> Rect {
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
pub struct DrawingSheetGeometry {
    /// Validated integer-micrometre geometry from the durable domain.
    pub physical: AuthoredDrawingSheetGeometry,
    pub paper: WorldRect,
    pub printable: WorldRect,
    pub drawing_area: WorldRect,
    pub content_area: WorldRect,
    pub title_block: Option<WorldRect>,
}

impl DrawingSheetGeometry {
    pub fn from_format(format: &SchematicSheetFormat) -> Self {
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

    pub fn world_to_sheet_mm(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        (
            (world_x - DRAWING_SHEET_ORIGIN.0) / DRAWING_SHEET_UNITS_PER_MM,
            (world_y - DRAWING_SHEET_ORIGIN.1) / DRAWING_SHEET_UNITS_PER_MM,
        )
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
pub struct ActiveDrawingSheet {
    pub format: SchematicSheetFormat,
    pub geometry: DrawingSheetGeometry,
    pub sheet_name: String,
    pub page_label: String,
}

impl ActiveDrawingSheet {
    pub fn format_label(&self) -> String {
        format!(
            "{} \u{00b7} {}",
            self.format.authored_size.label(),
            self.format.orientation.label().to_lowercase()
        )
    }

    /// Status-bar readout in the authored sheet's configured display unit.
    /// Coordinates retain the permanent page origin even while the pointer is
    /// on the surrounding desk; the trailing state explains that condition.
    pub fn cursor_status(&self, world_x: f64, world_y: f64) -> String {
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

/// Sheet lettering is paper annotation, exactly as it is on the plot: its
/// height is authored in millimetres on the page and simply magnifies with
/// the sheet. Nothing pins its size on screen, because a pinned size makes
/// the canvas disagree with hardcopy and overruns cells whose text budget was
/// derived from the authored height. Legibility at small zoom is the level of
/// detail's decision — it withholds text it cannot render legibly — not the
/// font's.
fn drawing_sheet_font_size(zoom: f32, authored_size: f32) -> f32 {
    // Quarter-pixel quantization keeps glyph rasterization stable across
    // wheel steps without decoupling the size from the page.
    ((authored_size * zoom.max(0.0) * 4.0).round() * 0.25).max(1.0)
}

/// Em size, in world units, that letters a field at the given ISO 3098
/// height — four world units spanning one millimetre.
///
/// The standard measures a height by its capitals and egui sizes a line by
/// its em, so the two are not the same number. The conversion between them
/// comes from the durable sheet domain rather than from here, so canvas and
/// hardcopy letter a field identically.
fn authored_lettering(lettering_um: u64) -> f32 {
    drawing_sheet_mono_em_um(lettering_um) as f32 * DRAWING_SHEET_UNITS_PER_MM as f32 / 1_000.0
}

/// Font a zone reference in the border band is lettered in: the height and
/// weight the plot letters it at, which depend on the band and not on the
/// sheet.
fn zone_label_font(sheet: &ActiveDrawingSheet, zoom: f32) -> FontId {
    let lettering_um = drawing_sheet_zone_lettering_um(sheet.geometry.physical.border_band_um);
    theme::mono(
        drawing_sheet_font_size(zoom, authored_lettering(lettering_um)),
        theme::FontWeight::Regular,
    )
}

/// Smallest em height, in physical pixels, at which title-block lettering is
/// worth drawing. Below it the glyphs carry no readable detail and the field
/// text is withheld rather than drawn as grey mush.
const DRAWING_SHEET_LEGIBLE_LETTERING_PX: f32 = 8.0;

impl DrawingSheetLod {
    fn resolve(zoom: f32, pixels_per_point: f32, lettering: DrawingSheetLettering) -> Self {
        let millimetre_pixels =
            DRAWING_SHEET_UNITS_PER_MM as f32 * zoom * pixels_per_point.max(0.5);
        let data_lettering_px =
            millimetre_pixels * drawing_sheet_mono_em_um(lettering.data_um) as f32 / 1_000.0;
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
            // Title-block lettering is drawn at its authored paper height, so
            // it is legible only once that height resolves on screen. Below
            // that the block stays ruled and empty, which is what the sheet
            // looks like from the same distance on paper.
            show_title_block_fields: data_lettering_px >= DRAWING_SHEET_LEGIBLE_LETTERING_PX,
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

/// Local display inputs; authored sheet geometry remains in the design model.
/// The camera is the viewport's, so the grid and the content share it.
#[derive(Debug, Clone, Copy)]
pub struct DrawingSheetCanvas {
    pub layers: DrawingSheetLayerVisibility,
    pub grid: GridStyle,
    pub grid_size: i32,
}

/// Paint the permanent pointer-transparent paper and its optional supporting
/// layers. Schematic objects are painted by the caller afterwards.
pub fn draw_base(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    input: DrawingSheetCanvas,
    sheet: &ActiveDrawingSheet,
    resolve_title_fields: impl FnOnce() -> Vec<ResolvedDrawingSheetTitleField>,
) {
    let tokens = Tokens::get(painter.ctx());
    let palette = DrawingSheetPalette::resolve(&tokens);
    let layers = input.layers;
    let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
    let lod = DrawingSheetLod::resolve(viewport.zoom, painter.ctx().pixels_per_point(), lettering);
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
        // The lattice is phased on the viewport's own offset, which content
        // is placed with (rounded to device pixels), never on the raw pan:
        // a grid point and the pin end on it share one screen position.
        draw_grid(
            painter,
            viewport.bounds,
            drawing_area.intersect(available),
            input.grid,
            input.grid_size,
            (f64::from(viewport.offset.x), f64::from(viewport.offset.y)),
            f64::from(viewport.zoom),
        );
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
            let label_font = lod
                .show_zone_labels
                .then(|| zone_label_font(sheet, viewport.zoom));
            draw_zone_band(painter, printable, drawing_area, zones, label_font, palette);
        }
    }
    if layers.title_block
        && let Some(block) = sheet
            .geometry
            .title_block
            .map(|rect| rect.screen_rect(viewport))
    {
        draw_title_block(
            painter,
            block,
            lod,
            viewport.zoom,
            palette,
            resolve_title_fields,
            sheet,
        );
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
        draw_origin(painter, paper, viewport.zoom, palette, lettering);
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

/// Whether the canvas shows overflow advisories at this zoom. Asked before
/// the advisories are found, so a zoom that hides them costs nothing.
pub fn overflow_advisories_shown(painter: &Painter, viewport: &Viewport) -> bool {
    let lod = DrawingSheetLod::resolve(
        viewport.zoom,
        painter.ctx().pixels_per_point(),
        DRAWING_SHEET_TITLE_BLOCK_LETTERING,
    );
    overflow_advisories_visible(lod.tier)
}

/// Everything the advisory halos are painted from besides the advisories
/// themselves: where the sheet sits on screen, the canvas, the display
/// scale, the halo ink and how a stroke is tessellated.
#[derive(Clone, Copy, PartialEq)]
struct OverflowHaloPaint {
    offset: Pos2,
    zoom: f32,
    bounds: Rect,
    available: Rect,
    pixels_per_point: f32,
    ink: Color32,
    tessellation: TessellationOptions,
}

/// Everything one halo is tessellated from besides the paint: its outline
/// on screen, by the bits of its corners, and the severity its dash shows.
type HaloOutline = ([u32; 4], DrawingSheetOverflowSeverity);

/// Where one halo's triangles lie in a mesh: its vertices, then its
/// indices, which name only those vertices.
type HaloTriangles = (Range<usize>, Range<usize>);

/// The halos last painted, tessellated, with what they were painted from.
struct HeldOverflowHalos {
    /// Held, not just compared by address, so the address is never reused
    /// by other advisories while these halos stand for it.
    summary: Arc<DrawingSheetOverflowSummary>,
    paint: OverflowHaloPaint,
    mesh: Arc<Mesh>,
    /// Where each outline's triangles lie in `mesh`.
    halos: HashMap<HaloOutline, HaloTriangles>,
}

thread_local! {
    /// Painting is single-threaded; one canvas is live at a time.
    static HELD_OVERFLOW_HALOS: RefCell<Option<HeldOverflowHalos>> = const { RefCell::new(None) };
}

/// Paint advisory halos above authored objects. These never block saves,
/// checks, netlisting, or simulation.
///
/// A sheet whose content runs off it can carry thousands of advisories, each
/// a dashed outline of several dashes. They are tessellated once and the
/// mesh is painted again while the advisories and the view stand still, so
/// a hover frame neither re-dashes them nor hands the host thousands of
/// strokes to tessellate. When the advisories change and the view does not,
/// as on every frame of a drag, each halo whose outline was painted last
/// time is copied from the last mesh, and only the others are tessellated.
pub fn draw_overflow_advisories(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    summary: &Arc<DrawingSheetOverflowSummary>,
) {
    if summary.is_clear() || !overflow_advisories_shown(painter, viewport) {
        return;
    }
    let ctx = painter.ctx();
    let paint = OverflowHaloPaint {
        offset: viewport.offset,
        zoom: viewport.zoom,
        bounds: viewport.bounds,
        available,
        pixels_per_point: ctx.pixels_per_point(),
        ink: DrawingSheetPalette::resolve(&Tokens::get(ctx)).overflow,
        tessellation: ctx.tessellation_options(|options| *options),
    };
    let mesh = HELD_OVERFLOW_HALOS.with(|held| {
        let mut held = held.borrow_mut();
        if let Some(halos) = held
            .as_ref()
            .filter(|halos| Arc::ptr_eq(&halos.summary, summary) && halos.paint == paint)
        {
            return Arc::clone(&halos.mesh);
        }
        let painted = held.as_ref().filter(|halos| halos.paint == paint);
        let (mesh, halos) = overflow_halo_mesh(&paint, viewport, summary, painted);
        let mesh = Arc::new(mesh);
        *held = Some(HeldOverflowHalos {
            summary: Arc::clone(summary),
            paint,
            mesh: Arc::clone(&mesh),
            halos,
        });
        mesh
    });
    painter.add(Shape::Mesh(mesh));
}

/// Every advisory's dashed halo, tessellated as egui would tessellate the
/// dashes painted one by one, with where each outline's triangles lie.
///
/// `painted` is the last mesh when it was painted with this same paint: a
/// halo's triangles depend only on its outline and the paint, so a halo
/// whose outline it holds is copied from it, its indices moved to where its
/// vertices now start, and the mesh is the one tessellating every dash would
/// give.
fn overflow_halo_mesh(
    paint: &OverflowHaloPaint,
    viewport: &Viewport,
    summary: &DrawingSheetOverflowSummary,
    painted: Option<&HeldOverflowHalos>,
) -> (Mesh, HashMap<HaloOutline, HaloTriangles>) {
    // Dashes are strokes, which read neither the font texture's size nor
    // the prepared discs; epaint documents both as safe to leave unset.
    let mut tessellator = Tessellator::new(
        paint.pixels_per_point,
        paint.tessellation,
        [1, 1],
        Vec::new(),
    );
    let mut mesh = Mesh::default();
    if let Some(painted) = painted {
        mesh.vertices.reserve(painted.mesh.vertices.len());
        mesh.indices.reserve(painted.mesh.indices.len());
    }
    let mut halos = HashMap::new();
    let mut dashes = Vec::new();
    for item in &summary.items {
        let mut screen = item.bounds.screen_rect(viewport).expand(3.0);
        if screen.width() < 8.0 {
            screen = Rect::from_center_size(screen.center(), vec2(8.0, screen.height().max(8.0)));
        }
        if screen.height() < 8.0 {
            screen = Rect::from_center_size(screen.center(), vec2(screen.width().max(8.0), 8.0));
        }
        screen = screen.intersect(paint.available);
        if !screen.is_positive() {
            continue;
        }
        let outline: HaloOutline = (
            [
                screen.min.x.to_bits(),
                screen.min.y.to_bits(),
                screen.max.x.to_bits(),
                screen.max.y.to_bits(),
            ],
            item.severity,
        );
        let (vertices_from, indices_from) = (mesh.vertices.len(), mesh.indices.len());
        if let Some((painted, (vertices, indices))) = painted.and_then(|painted| {
            painted
                .halos
                .get(&outline)
                .map(|triangles| (&painted.mesh, triangles))
        }) {
            let (from, to) = (vertices.start as u32, vertices_from as u32);
            mesh.vertices
                .extend_from_slice(&painted.vertices[vertices.clone()]);
            mesh.indices.extend(
                painted.indices[indices.clone()]
                    .iter()
                    .map(|&index| index - from + to),
            );
        } else {
            let (dash, gap, width) = match item.severity {
                DrawingSheetOverflowSeverity::OutsideBorder => (5.0, 4.0, 1.4),
                DrawingSheetOverflowSeverity::OffPaper => (7.0, 3.0, 1.6),
                DrawingSheetOverflowSeverity::TitleBlockCollision => (2.0, 3.0, 1.4),
            };
            Shape::dashed_line_many(
                &rect_outline(screen),
                Stroke::new(width, paint.ink),
                dash,
                gap,
                &mut dashes,
            );
            for dash in dashes.drain(..) {
                tessellator.tessellate_shape(dash, &mut mesh);
            }
        }
        halos.entry(outline).or_insert((
            vertices_from..mesh.vertices.len(),
            indices_from..mesh.indices.len(),
        ));
    }
    (mesh, halos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrawingSheetOverflowTarget {
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
    pub const fn kind_label(self) -> &'static str {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawingSheetOverflowSeverity {
    OutsideBorder,
    OffPaper,
    TitleBlockCollision,
}

impl DrawingSheetOverflowSeverity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OutsideBorder => "Outside the drawing border",
            Self::OffPaper => "Beyond the paper edge",
            Self::TitleBlockCollision => "Overlapping the title block",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawingSheetOverflowItem {
    pub target: DrawingSheetOverflowTarget,
    pub identity: String,
    pub kind: &'static str,
    pub outside_border: bool,
    pub off_paper: bool,
    pub title_block_collision: bool,
    /// Primary row treatment only. Summary facts use the independent flags
    /// above so off-paper content remains part of the outside-border total
    /// and title-block collisions remain separately reviewable.
    pub severity: DrawingSheetOverflowSeverity,
    pub sheet_coordinates_mm: (f64, f64),
    pub zone: Option<String>,
    pub bounds: WorldRect,
}

impl DrawingSheetOverflowItem {
    pub fn center(&self) -> Point {
        Point::new(
            ((self.bounds.min_x + self.bounds.max_x) * 0.5).round() as i32,
            ((self.bounds.min_y + self.bounds.max_y) * 0.5).round() as i32,
        )
    }
}

#[derive(Debug, Clone)]
pub struct DrawingSheetOverflowSummary {
    pub outside_border: usize,
    pub off_paper: usize,
    pub title_block_collisions: usize,
    pub first_target: Option<DrawingSheetOverflowTarget>,
    pub items: Vec<DrawingSheetOverflowItem>,
}

impl DrawingSheetOverflowSummary {
    pub fn is_clear(&self) -> bool {
        self.items.is_empty()
    }

    pub fn finding_count(&self) -> usize {
        self.items.len()
    }
}

/// Resolve the non-blocking authored-content advisories used by both the
/// canvas and inspector. Keeping one report authority prevents their counts
/// and navigation targets from drifting.
pub fn drawing_sheet_overflow_summary(
    view: &DesignView<'_>,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> DrawingSheetOverflowSummary {
    let items = overflow_items(view, symbol_context, &sheet.geometry);
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

/// The object a printable extent belongs to, kept so its name is spelt only
/// if it is reported. Every object's extent is measured; few overflow.
#[derive(Debug, Clone, Copy)]
enum PrintableObject<'a> {
    Component(&'a Component),
    Wire(u64),
    Bus(&'a Bus),
    BusTap(&'a BusTap),
    Junction(u64),
    NetLabel(&'a NetLabel),
    DesignNote(&'a DesignNote),
    DocumentationShape(&'a DocumentationShape),
}

impl PrintableObject<'_> {
    /// How an overflow report names the object.
    fn identity(self) -> String {
        match self {
            Self::Component(component) if component.name.trim().is_empty() => {
                format!("instance:{}", component.id)
            }
            Self::Component(component) => component.name.clone(),
            Self::Wire(id) => format!("wire:{id}"),
            Self::Bus(bus) => bus
                .declaration
                .as_ref()
                .map_or_else(|| format!("bus:{}", bus.id), ToString::to_string),
            Self::BusTap(tap) => format!("tap:{} \u{00b7} {}", tap.id, tap.slice),
            Self::Junction(id) => format!("junction:{id}"),
            Self::NetLabel(label) if label.name.trim().is_empty() => {
                format!("net-label:{}", label.id)
            }
            Self::NetLabel(label) => label.name.clone(),
            Self::DesignNote(note) => format!("note:{} \u{00b7} {}", note.id, note.kind.label()),
            Self::DocumentationShape(shape) => {
                format!("shape:{} \u{00b7} {}", shape.id, shape.kind().label())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PrintableObjectBounds<'a> {
    target: DrawingSheetOverflowTarget,
    object: PrintableObject<'a>,
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
pub enum DrawingSheetPrintablePreviewKind {
    Component,
    Conductor,
    Junction,
    NetLabel,
    DesignNote,
    DocumentationShape,
}

#[derive(Debug, Clone)]
pub struct DrawingSheetPrintablePreview {
    pub kind: DrawingSheetPrintablePreviewKind,
    pub bounds: WorldRect,
    pub path: Vec<Point>,
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
    view: &DesignView<'_>,
    symbol_context: &SchematicSymbolContext,
    geometry: &DrawingSheetGeometry,
) -> Vec<DrawingSheetOverflowItem> {
    active_object_bounds(view, symbol_context)
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
                identity: object.object.identity(),
                kind: object.kind,
                bounds: object.bounds,
            })
        })
        .collect()
}

fn active_object_bounds<'a>(
    view: &DesignView<'a>,
    symbol_context: &SchematicSymbolContext,
) -> Vec<PrintableObjectBounds<'a>> {
    let mut bounds = Vec::new();
    for component in &view.document.components {
        if view.object_is_visible(component.id) {
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
                object: PrintableObject::Component(component),
                kind: DrawingSheetOverflowTarget::Component(component.id).kind_label(),
                bounds: WorldRect::from_points(
                    Point::new(min.x.min(component.pos.x), min.y.min(component.pos.y)),
                    Point::new(max.x.max(label_max.x), max.y.max(label_max.y)),
                ),
                route: None,
            });
        }
    }
    for wire in &view.document.wires {
        if view.object_is_visible(wire.id)
            && let Some(rect) = points_bounds(&wire.points)
        {
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Wire(wire.id),
                object: PrintableObject::Wire(wire.id),
                kind: DrawingSheetOverflowTarget::Wire(wire.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Borrowed(&wire.points)),
            });
        }
    }
    for bus in &view.document.buses {
        if view.object_is_visible(bus.id)
            && let Some(rect) = points_bounds(&bus.points)
        {
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Bus(bus.id),
                object: PrintableObject::Bus(bus),
                kind: DrawingSheetOverflowTarget::Bus(bus.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Borrowed(&bus.points)),
            });
        }
    }
    for tap in &view.document.bus_taps {
        if view.object_is_visible(tap.id) {
            let route = crate::bus_geometry::bus_tap_route_points(tap);
            let Some(rect) = points_bounds(&route) else {
                continue;
            };
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::BusTap(tap.id),
                object: PrintableObject::BusTap(tap),
                kind: DrawingSheetOverflowTarget::BusTap(tap.id).kind_label(),
                bounds: rect,
                route: Some(Cow::Owned(route)),
            });
        }
    }
    for junction in &view.document.junctions {
        if view.object_is_visible(junction.id) {
            // Hardcopy renders an exact 900 µm filled junction dot. Include
            // that physical radius rather than treating the junction as a
            // dimensionless point at the paper edge.
            const JUNCTION_RADIUS_WORLD: f64 = 900.0 / DRAWING_SHEET_WORLD_UNIT_UM as f64;
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::Junction(junction.id),
                object: PrintableObject::Junction(junction.id),
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
    for label in &view.document.net_labels {
        if view.object_is_visible(label.id) {
            let (min, max) = super::net_labels::world_bounds(label);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::NetLabel(label.id),
                object: PrintableObject::NetLabel(label),
                kind: DrawingSheetOverflowTarget::NetLabel(label.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    for note in &view.document.design_notes {
        if view.object_is_visible(note.id) {
            let (min, max) = super::design_notes::conservative_world_bounds(note);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::DesignNote(note.id),
                object: PrintableObject::DesignNote(note),
                kind: DrawingSheetOverflowTarget::DesignNote(note.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    for shape in &view.document.documentation_shapes {
        if view.object_is_visible(shape.id) {
            let (min, max) = super::documentation_shapes::world_bounds(shape);
            bounds.push(PrintableObjectBounds {
                target: DrawingSheetOverflowTarget::DocumentationShape(shape.id),
                object: PrintableObject::DocumentationShape(shape),
                kind: DrawingSheetOverflowTarget::DocumentationShape(shape.id).kind_label(),
                bounds: WorldRect::from_points(min, max),
                route: None,
            });
        }
    }
    bounds
}

pub fn drawing_sheet_printable_preview(
    view: &DesignView<'_>,
    symbol_context: &SchematicSymbolContext,
) -> Vec<DrawingSheetPrintablePreview> {
    active_object_bounds(view, symbol_context)
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
    painter.extend(Shape::dashed_line(
        &rect_outline(rect),
        stroke,
        dash_length,
        gap_length,
    ));
}

/// A rectangle's outline as a closed polyline from its top-left corner.
fn rect_outline(rect: Rect) -> [Pos2; 5] {
    [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
        rect.left_top(),
    ]
}

fn draw_zone_band(
    painter: &Painter,
    printable: Rect,
    drawing_area: Rect,
    zones: DrawingSheetZoneGrid,
    label_font: Option<FontId>,
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
    let Some(font) = label_font.filter(|_| zone_edge_labels_visible(zones.labels)) else {
        return;
    };
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
    resolve_title_fields: impl FnOnce() -> Vec<ResolvedDrawingSheetTitleField>,
    sheet: &ActiveDrawingSheet,
) {
    let rotation = sheet.format.title_block.rotation;
    let angle = title_block_rotation_angle(rotation);
    // Domain geometry exposes the rotated bounding box. Author the complete
    // block in its natural orientation, then rotate every primitive around
    // the shared center so the outline, grid, and text cannot drift.
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
    // The grid hardcopy and Page Setup draw too, in physical units on the
    // upright block; the authored block maps those onto the screen.
    let Some(layout) = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical) else {
        return;
    };
    let x_at = |x_um: u64| {
        authored_block.left() + authored_block.width() * x_um as f32 / layout.width_um as f32
    };
    let y_at = |row: usize| {
        authored_block.top()
            + authored_block.height() * layout.row_top_um(row) as f32 / layout.height_um as f32
    };
    let effective_template = sheet.geometry.physical.effective_title_block_template;
    let mut field_grid = authored_block;
    field_grid.min.x = x_at(layout.grid_x_um);
    if let Some(logo) = sheet.format.title_block_logo(effective_template) {
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
    let fields = resolve_title_fields();
    if fields.is_empty() {
        return;
    }
    let lettering = DRAWING_SHEET_TITLE_BLOCK_LETTERING;
    for row in 1..layout.rows {
        let y = y_at(row);
        painter.line_segment(
            [
                transform(pos2(field_grid.left(), y)),
                transform(pos2(field_grid.right(), y)),
            ],
            Stroke::new(0.8, palette.zone),
        );
    }
    for (layout_cell, field) in layout.cells.iter().zip(fields) {
        debug_assert_eq!(layout_cell.id, field.id);
        let (left, right) = (
            x_at(layout_cell.x_um),
            x_at(layout_cell.x_um + layout_cell.width_um),
        );
        let (top, bottom) = (y_at(layout_cell.row), y_at(layout_cell.row + 1));
        if layout_cell.x_um > layout.grid_x_um {
            painter.line_segment(
                [transform(pos2(left, top)), transform(pos2(left, bottom))],
                Stroke::new(0.8, palette.zone),
            );
        }
        let authored_cell = Rect::from_min_max(pos2(left, top), pos2(right, bottom))
            .shrink2(vec2(authored_title_cell_padding() * zoom, 0.0));
        let cell = Rect::from_two_pos(
            transform(authored_cell.left_top()),
            transform(authored_cell.right_bottom()),
        );
        // Caption and value sit on the shared fractions of the cell that
        // hardcopy also places them on. Anchoring each line on its own centre
        // leaves the vertical placement to the font's real metrics rather
        // than to a guessed line box.
        let at = |(numerator, denominator): (u64, u64)| {
            authored_cell.top() + authored_cell.height() * numerator as f32 / denominator as f32
        };
        let clipped = painter.with_clip_rect(cell);
        paint_title_text(
            &clipped,
            transform(pos2(
                authored_cell.left(),
                at(DRAWING_SHEET_TITLE_CAPTION_POSITION),
            )),
            Align2::LEFT_CENTER,
            // The field's own wording, in the case the plot and the dialogs
            // letter it in, so a caption reads the same on screen and on paper.
            layout_cell.letter_caption(),
            theme::mono(
                drawing_sheet_font_size(zoom, authored_lettering(lettering.caption_um)),
                theme::FontWeight::Regular,
            ),
            palette.block_label,
            rotation,
        );
        // An authored field nobody has filled in is a box to fill in or sign
        // on the print, lettered with its caption alone.
        if field.value.is_empty() {
            continue;
        }
        paint_title_text(
            &clipped,
            transform(pos2(
                authored_cell.left(),
                at(DRAWING_SHEET_TITLE_VALUE_POSITION),
            )),
            Align2::LEFT_CENTER,
            // Cut to the cell's physical budget, so zoom and monitor DPI
            // cannot disagree with Page Setup or the plot.
            layout_cell.letter_value(&field.value),
            // A value stands a step of the series above its caption, which is
            // what makes it read as the field's content; only the document
            // identifier is set in the heavier weight. The plot letters both
            // the same way.
            theme::mono(
                drawing_sheet_font_size(zoom, authored_lettering(lettering.data_um)),
                if field.id.emphasised() {
                    theme::FontWeight::Medium
                } else {
                    theme::FontWeight::Regular
                },
            ),
            palette.block_ink,
            rotation,
        );
    }
}

/// Horizontal padding between a title-grid rule and its lettering, in world
/// units — scale it by the zoom before applying it to a screen rectangle.
/// Canvas and hardcopy inset by the same physical amount, and the character
/// budget subtracts exactly this much from the cell, so a padding that drifts
/// with zoom would put the budget and the painted text back out of step.
fn authored_title_cell_padding() -> f32 {
    DRAWING_SHEET_TITLE_CELL_PADDING_UM as f32 * DRAWING_SHEET_UNITS_PER_MM as f32 / 1_000.0
}

fn draw_managed_title_logo(
    painter: &Painter,
    reserved: Rect,
    transform: &impl Fn(Pos2) -> Pos2,
    logo: &rspice_design_model::design_management::DrawingSheetManagedLogo,
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

fn draw_origin(
    painter: &Painter,
    paper: Rect,
    zoom: f32,
    palette: DrawingSheetPalette,
    lettering: DrawingSheetLettering,
) {
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
            drawing_sheet_font_size(zoom, authored_lettering(lettering.data_um)),
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
    let lettering = sheet.format.authored_size.lettering();
    painter.text(
        paper.center_top() - vec2(0.0, 7.0),
        Align2::CENTER_BOTTOM,
        format!("{} {}", unit.format_um(width), unit.suffix()),
        theme::mono(
            drawing_sheet_font_size(zoom, authored_lettering(lettering.data_um)),
            theme::FontWeight::Regular,
        ),
        palette.zone_ink,
    );
    painter.text(
        paper.left_center() - vec2(7.0, 0.0),
        Align2::RIGHT_CENTER,
        format!("{} {}", unit.format_um(height), unit.suffix()),
        theme::mono(
            drawing_sheet_font_size(zoom, authored_lettering(lettering.data_um)),
            theme::FontWeight::Regular,
        ),
        palette.zone_ink,
    );
}

#[cfg(test)]
mod tests;
