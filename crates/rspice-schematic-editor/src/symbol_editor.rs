//! Symbol canvas geometry, viewport and painting over explicit editor inputs.

pub mod arrange;
pub mod commands;
pub mod context_menu;
pub mod handles;
pub mod inspector;
pub mod interaction;
pub mod pin_table;
pub mod placement;
pub mod session;
pub mod surface;

use crate::view::drawing::paint_upward_text;
use crate::view::navigation::{
    CanvasNavigation, SCHEMATIC_ZOOM_MIN, ZoomRequest, apply_zoom_request, frame_world_rect,
    handle_viewport_navigation,
};
use crate::view::resolved_symbol_render::{draw_resolved_symbol, text_hang};
use crate::view::symbol_style::{broken_outline, inside_outline, paint_fill, styled_stroke};
use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, Ui, Vec2, pos2, vec2};
use placement::SymbolPlacement;
use rspice_design::{
    resolved_symbol::ResolvedCellSymbol,
    schematic::{
        component::{Component, LibraryCellInstance},
        component_type::ComponentType,
    },
    symbol::{
        PinDrawing, SYMBOL_PIN_LENGTH, SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolCurve,
        SymbolDash, SymbolDocument, SymbolEditorMetadata, SymbolFill, SymbolShape, SymbolStyle,
        SymbolTextAlign, SymbolTextRotation, pin_drawing, rectangle_corners,
    },
};
use rspice_design_model::{
    Point,
    cell_view::CellViewRef,
    port::{PortDirection, PortSpec},
    symbol_pin::{SymbolPinSide, SymbolPinStyle},
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};
use session::{SymbolEditorSession, SymbolGridSpacing, SymbolTool};

const PIN_HIT_RADIUS: f32 = 10.0;
const PREVIEW_TILE_MAX_WIDTH: f32 = 240.0;
const PREVIEW_TILE_MAX_HEIGHT: f32 = 168.0;
const PREVIEW_TILE_MIN_HEIGHT: f32 = 118.0;
/// How many preview columns wide a canvas is before it gives one up: below
/// this the drawing needs the room more than the preview does.
const PREVIEW_CANVAS_PER_COLUMN: f32 = 3.0;
const PREVIEW_TILE_MARGIN: f32 = 12.0;
const PREVIEW_TILE_HEADER: f32 = 26.0;
const PREVIEW_FIT_PADDING: f32 = 10.0;

/// The map between symbol coordinates and the screen.
///
/// `placement` takes a symbol point to the plane `zoom` and `pan` frame. In
/// the symbol's own view it is the identity; edited in place, it is the
/// instance's transform and the plane is the parent sheet, so the symbol is
/// drawn where the instance stands and the pointer is read back in symbol
/// coordinates.
#[derive(Debug, Clone, Copy)]
pub struct SymbolViewport {
    pub rect: Rect,
    pub zoom: f32,
    pub pan: Vec2,
    pub placement: SymbolPlacement,
}

impl SymbolViewport {
    /// The symbol's own view, framed by `zoom` and `pan`.
    pub fn new(rect: Rect, zoom: f32, pan: Vec2) -> Self {
        Self {
            rect,
            zoom,
            pan,
            placement: SymbolPlacement::default(),
        }
    }

    /// The symbol as `placement` puts it on the sheet `viewport` frames.
    pub fn on_sheet(
        viewport: &crate::view::viewport::Viewport,
        placement: SymbolPlacement,
    ) -> Self {
        let rect = viewport.bounds;
        Self {
            rect,
            zoom: viewport.zoom,
            pan: rect.min + viewport.offset.to_vec2() - rect.center(),
            placement,
        }
    }

    pub fn world_to_screen(self, point: Point) -> Pos2 {
        let (x, y) = self.placement.place_f(point);
        self.rect.center() + self.pan + vec2(x * self.zoom, y * self.zoom)
    }

    /// Where a symbol point between grid units lands on screen.
    pub fn world_to_screen_f(self, [x, y]: [f64; 2]) -> Pos2 {
        let (x, y) = self.placement.place_xy(x as f32, y as f32);
        self.rect.center() + self.pan + vec2(x * self.zoom, y * self.zoom)
    }

    pub fn screen_to_world(self, pos: Pos2) -> Point {
        let rel = pos - self.rect.center() - self.pan;
        let (x, y) = self
            .placement
            .unplace_f(rel.x / self.zoom, rel.y / self.zoom);
        Point::new(x.round() as i32, y.round() as i32)
    }

    /// Which way a symbol-space direction points on screen.
    fn screen_direction(self, x: f32, y: f32) -> Vec2 {
        let (x, y) = self.placement.orient_f(x, y);
        vec2(x, y)
    }
}

/// This frame's view of the symbol after the canvas's own navigation: the
/// schematic canvas's wheel, pinch and pan, its zoom range, its zoom box and
/// the zoom commands' requests, so a symbol navigates as a schematic does.
pub fn update_viewport(
    ui: &mut Ui,
    session: &mut SymbolEditorSession,
    rect: Rect,
    document: &SymbolDocument,
    response: &egui::Response,
    navigation: CanvasNavigation,
) -> SymbolViewport {
    // The session keeps its pan from the canvas's middle; the shared code
    // reads it from the top-left corner.
    let half = (
        f64::from(rect.width()) * 0.5,
        f64::from(rect.height()) * 0.5,
    );
    let mut zoom = f64::from(session.zoom);
    let mut pan = (
        half.0 + f64::from(session.pan.0),
        half.1 + f64::from(session.pan.1),
    );
    if std::mem::take(&mut session.needs_fit) {
        let (min, max) = document_bounds(document);
        frame_world_rect(
            &mut zoom,
            &mut pan,
            rect,
            (
                f64::from(min.x),
                f64::from(min.y),
                f64::from(max.x),
                f64::from(max.y),
            ),
        );
    }
    match session.zoom_request.take() {
        Some(ZoomRequest::Frame(bounds)) => frame_world_rect(&mut zoom, &mut pan, rect, bounds),
        Some(request) => apply_zoom_request(
            &mut zoom,
            &mut pan,
            (half.0 * 2.0, half.1 * 2.0),
            request,
            SCHEMATIC_ZOOM_MIN,
        ),
        None => {}
    }
    handle_viewport_navigation(
        ui,
        response,
        rect,
        &mut zoom,
        &mut pan,
        SCHEMATIC_ZOOM_MIN,
        navigation,
    );
    if session.zoom_box.armed {
        let to_world = |pos: Pos2| {
            (
                (f64::from(pos.x) - f64::from(rect.min.x) - pan.0) / zoom,
                (f64::from(pos.y) - f64::from(rect.min.y) - pan.1) / zoom,
            )
        };
        if let Some(request) = session.zoom_box.drive(ui, response, zoom, to_world) {
            session.zoom_request = Some(request);
        }
    }
    session.zoom = zoom as f32;
    session.pan = ((pan.0 - half.0) as f32, (pan.1 - half.1) as f32);
    let time = ui.input(|input| input.time);
    session
        .view_history
        .observe((session.zoom, session.pan), time);
    SymbolViewport::new(rect, session.zoom, vec2(session.pan.0, session.pan.1))
}

/// Outline the zoom box being dragged over the symbol's own view.
pub fn paint_zoom_box(ui: &Ui, session: &SymbolEditorSession, viewport: SymbolViewport) {
    let Some((min_x, min_y, max_x, max_y)) = session.zoom_box.world_rect() else {
        return;
    };
    let to_screen = |x: f64, y: f64| {
        viewport.rect.center() + viewport.pan + vec2(x as f32, y as f32) * viewport.zoom
    };
    ui.painter().rect_stroke(
        Rect::from_min_max(to_screen(min_x, min_y), to_screen(max_x, max_y)),
        0.0,
        Stroke::new(1.0, Tokens::get(ui.ctx()).color.accent),
        egui::StrokeKind::Inside,
    );
}

pub fn draw_canvas(
    ui: &mut Ui,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    ports: &[PortSpec],
    session: &SymbolEditorSession,
) {
    let t = Tokens::get(ui.ctx());
    let painter = ui.painter_at(viewport.rect);
    painter.rect_filled(viewport.rect, 0.0, t.color.canvas_bg);
    draw_in_place(&painter, viewport, document, editor, ports, session);
}

/// How much of a symbol canvas `width` wide the placed preview keeps for
/// itself: a column at the canvas's right edge, the tile and its margins
/// wide, or nothing where the canvas is too narrow to give a column up.
///
/// The preview has room of its own rather than floating over the drawing,
/// so no zoom or pan can put any of the drawing under it.
pub fn preview_column_width(width: f32) -> f32 {
    let column = PREVIEW_TILE_MAX_WIDTH + PREVIEW_TILE_MARGIN * 2.0;
    if width >= column * PREVIEW_CANVAS_PER_COLUMN {
        column
    } else {
        0.0
    }
}

/// The placed preview in its own `column` beside the drawing: canvas ground
/// under a rule that parts it from the drawing, and the tile at its foot.
pub fn draw_preview_column(
    ui: &mut Ui,
    column: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    reference: &CellViewRef,
) {
    if column.width() <= 0.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let painter = ui.painter_at(column);
    painter.rect_filled(column, 0.0, t.color.canvas_bg);
    painter.vline(
        column.left() + 0.5,
        column.y_range(),
        Stroke::new(1.0, t.color.border),
    );
    draw_preview_tile(
        ui,
        preview_tile_rect(column),
        document,
        ports,
        reference,
        &t,
    );
}

/// Every editable layer of the symbol — grid, artwork, pins, labels, the
/// origin and the band — through `viewport`, over whatever `painter` already
/// holds. The symbol's own view lays it on a bare canvas; editing in place
/// lays it over the parent sheet that places it.
pub fn draw_in_place(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    ports: &[PortSpec],
    session: &SymbolEditorSession,
) {
    let t = Tokens::get(painter.ctx());
    let c = t.color;
    if session.show_grid {
        draw_grid(painter, viewport, c.canvas_grid, session.grid_spacing);
    }
    draw_body(
        painter,
        viewport,
        document,
        c.symbol,
        &session.effective_selection().shapes,
        c.accent,
    );
    draw_bbox_and_origin(painter, viewport, document, &t);
    draw_pins(painter, viewport, document, ports, session);
    draw_pin_tool_pin(painter, viewport, document, session, &t);
    draw_labels(painter, viewport, editor, session, &t);
    draw_handles(painter, viewport, document, session, &t);
    draw_marquee(painter, viewport, session, &t);
    draw_draft(painter, viewport, session, &t);
}

/// An edge's middle offers a new point there; it is drawn smaller than the
/// points the shape has.
const EDGE_HANDLE_RADIUS: f32 = 2.0;

/// The handles of every selected shape, as the schematic's wire grips are
/// drawn: a fixed ring the same at every zoom, filled under the pointer and
/// on the point Delete would take out. A handle Stretch holds draws the
/// shape it would leave in the draft tone.
fn draw_handles(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    session: &SymbolEditorSession,
    t: &Tokens,
) {
    let colour = t.color.canvas_select;
    let hair = Stroke::new(1.0 / painter.pixels_per_point(), colour);
    for &index in &session.effective_selection().shapes {
        let Some(shape) = document.body.get(index) else {
            continue;
        };
        for (handle, at) in handles::shape_handles(shape) {
            let here = handles::SymbolHandleRef {
                shape: index,
                handle,
            };
            let centre = viewport.world_to_screen_f(at);
            let marked = session.hover_handle == Some(here)
                || session.stretch == Some(here)
                || matches!(handle, handles::SymbolHandle::Vertex(vertex)
                    if session.selected_vertex == Some((index, vertex)));
            let (radius, stroke) = match handle {
                handles::SymbolHandle::EdgeMiddle(_) => (
                    EDGE_HANDLE_RADIUS,
                    Stroke::new(hair.width, colour.gamma_multiply(DRAFT_HINT)),
                ),
                _ => (crate::view::scene::VERTEX_HANDLE_RADIUS, hair),
            };
            if marked {
                painter.circle_filled(centre, radius, colour);
            } else {
                painter.circle_filled(centre, radius, t.color.canvas_bg);
                painter.circle_stroke(centre, radius, stroke);
            }
        }
    }
    let (Some(stretch), Some(pointer)) = (session.stretch, session.pointer) else {
        return;
    };
    let Some((stretched, _)) = interaction::stretched_shape(document, stretch, pointer) else {
        return;
    };
    let stroke = Stroke::new(
        DRAFT_STROKE,
        colour.gamma_multiply(DRAFT_ALPHA * DRAFT_HINT),
    );
    draw_shape_outline(painter, viewport, &stretched, stroke);
}

/// `shape`'s outline alone, in `stroke`: the shape a handle would leave.
fn draw_shape_outline(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    shape: &SymbolShape,
    stroke: Stroke,
) {
    let mut outline: Vec<Pos2> = match shape {
        SymbolShape::Polyline { points, closed, .. } => {
            let mut outline: Vec<Pos2> = points
                .iter()
                .map(|point| viewport.world_to_screen(*point))
                .collect();
            if *closed && let Some(first) = outline.first().copied() {
                outline.push(first);
            }
            outline
        }
        SymbolShape::Rectangle { min, max, .. } => {
            let corners = rectangle_corners(*min, *max);
            corners
                .iter()
                .chain(corners.first())
                .map(|point| viewport.world_to_screen(*point))
                .collect()
        }
        SymbolShape::Circle { center, radius, .. } | SymbolShape::Dot { center, radius } => {
            curve_points(viewport, &SymbolCurve::of_arc(*center, *radius, 0, 360))
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
            ..
        } => curve_points(
            viewport,
            &SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees),
        ),
        SymbolShape::Curve { curve, .. } => curve_points(viewport, curve),
        SymbolShape::Arrow { .. } | SymbolShape::Text { .. } => {
            let (min, max) = session::symbol_shape_bounds(shape);
            let corners = rectangle_corners(min, max);
            corners
                .iter()
                .chain(corners.first())
                .map(|point| viewport.world_to_screen(*point))
                .collect()
        }
    };
    outline.dedup();
    painter.add(Shape::line(outline, stroke));
}

/// A shape being drawn is the selection colour held back a little, as a
/// wire being routed is, so a committed selection still reads above it.
const DRAFT_ALPHA: f32 = 0.85;
/// The part of a draft that follows the pointer, against the points placed.
const DRAFT_HINT: f32 = 0.6;
/// A draft is drawn at the weight the shape will carry selected.
const DRAFT_STROKE: f32 = 2.0;

/// The shape the armed tool is drawing: what has been placed in full tone,
/// what runs to the pointer in the hint tone, and a mark where a click would
/// land.
fn draw_draft(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    session: &SymbolEditorSession,
    t: &Tokens,
) {
    if !session.tool.draws() {
        return;
    }
    let placed = t.color.canvas_select.gamma_multiply(DRAFT_ALPHA);
    let hint = placed.gamma_multiply(DRAFT_HINT);
    let full = Stroke::new(DRAFT_STROKE, placed);
    let following = Stroke::new(DRAFT_STROKE, hint);
    let screen = |point: Point| viewport.world_to_screen(point);
    let pointer = session.pointer;
    match session.tool {
        SymbolTool::Line | SymbolTool::Polygon => {
            let points = &session.pending_polyline;
            let mut route: Vec<Pos2> = points.iter().copied().map(screen).collect();
            if let Some(pointer) = pointer.filter(|_| !points.is_empty()) {
                route.push(screen(pointer));
            }
            // A polygon always closes, so its closing edge is shown too.
            if session.tool == SymbolTool::Polygon
                && route.len() > 2
                && let Some(first) = route.first().copied()
            {
                route.push(first);
            }
            painter.add(Shape::line(route, following));
            let committed: Vec<Pos2> = points.iter().copied().map(screen).collect();
            painter.add(Shape::line(committed.clone(), full));
            for point in committed {
                painter.circle_filled(point, DRAFT_STROKE * 1.25, placed);
            }
        }
        SymbolTool::Rectangle => {
            if let Some(start) = session.shape_start {
                if let Some(end) = pointer.filter(|end| end.x != start.x && end.y != start.y) {
                    painter.add(Shape::closed_line(
                        [
                            start,
                            Point::new(end.x, start.y),
                            end,
                            Point::new(start.x, end.y),
                        ]
                        .map(screen)
                        .to_vec(),
                        following,
                    ));
                }
                painter.circle_filled(screen(start), DRAFT_STROKE * 1.25, placed);
            }
        }
        SymbolTool::Circle => {
            if let Some(center) = session.shape_start {
                if let Some(radius) = pointer
                    .map(|point| interaction::radius_through(center, point))
                    .filter(|radius| *radius >= 1)
                {
                    painter.circle_stroke(screen(center), radius as f32 * viewport.zoom, following);
                }
                draw_center_mark(painter, screen(center), placed);
            }
        }
        SymbolTool::Arc => {
            if let Some(center) = session.shape_start {
                match session.arc_sweep {
                    None => {
                        if let Some(pointer) = pointer {
                            painter.line_segment([screen(center), screen(pointer)], following);
                        }
                    }
                    Some(sweep) => {
                        painter.line_segment([screen(center), screen(sweep.start)], full);
                        if let Some(pointer) = pointer {
                            painter.line_segment(
                                [screen(center), screen(pointer)],
                                Stroke::new(1.0, hint),
                            );
                        }
                        if let Some(SymbolShape::Arc {
                            radius,
                            start_degrees,
                            sweep_degrees,
                            ..
                        }) = interaction::arc_shape(center, &sweep)
                        {
                            let arc =
                                SymbolCurve::of_arc(center, radius, start_degrees, sweep_degrees);
                            painter.add(Shape::line(curve_points(viewport, &arc), following));
                        }
                        painter.circle_filled(screen(sweep.start), DRAFT_STROKE * 1.25, placed);
                    }
                }
                draw_center_mark(painter, screen(center), placed);
            }
        }
        SymbolTool::Text => {
            if let Some(draft) = &session.text_draft {
                let size = rspice_design::symbol::SymbolTextSize::default();
                let align = SymbolTextAlign::default();
                if !draft.text.is_empty() {
                    painter.text(
                        screen(draft.anchor),
                        editor_text_align(viewport.placement, align, SymbolTextRotation::R0),
                        &draft.text,
                        theme::mono(
                            (size.height() as f32 * viewport.zoom).max(1.0),
                            FontWeight::Regular,
                        ),
                        placed,
                    );
                }
                painter.circle_filled(screen(draft.anchor), DRAFT_STROKE * 1.25, placed);
            }
        }
        SymbolTool::Select | SymbolTool::PlacePin => {}
    }
    if let Some(pointer) = pointer {
        draw_snap_mark(painter, screen(pointer), placed);
    }
}

/// Where a click would land: the ring and cross the wire tool marks its
/// acquisition point with.
fn draw_snap_mark(painter: &egui::Painter, at: Pos2, color: Color32) {
    painter.circle_stroke(at, 5.0, Stroke::new(1.25, color));
    draw_center_mark(painter, at, color);
}

fn draw_center_mark(painter: &egui::Painter, at: Pos2, color: Color32) {
    let stroke = Stroke::new(1.0, color);
    painter.line_segment([at - vec2(2.5, 0.0), at + vec2(2.5, 0.0)], stroke);
    painter.line_segment([at - vec2(0.0, 2.5), at + vec2(0.0, 2.5)], stroke);
}

fn draw_grid(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    color: Color32,
    spacing: SymbolGridSpacing,
) {
    let model_step = spacing.model_step() as f32;
    let step = model_step * viewport.zoom;
    if step < 8.0 {
        return;
    }
    if !viewport.placement.is_identity() {
        draw_placed_grid(painter, viewport, color, model_step);
        return;
    }
    let min = viewport.rect.min;
    let max = viewport.rect.max;
    let center = viewport.rect.center() + viewport.pan;
    let mut x = center.x % step;
    while x < min.x {
        x += step;
    }
    while x <= max.x {
        let mut y = center.y % step;
        while y < min.y {
            y += step;
        }
        while y <= max.y {
            painter.circle_filled(pos2(x, y), 1.1, color);
            y += step;
        }
        x += step;
    }
}

/// The symbol's grid where an instance puts it. The lattice stays in symbol
/// coordinates, because that is where every edit made in place snaps.
fn draw_placed_grid(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    color: Color32,
    model_step: f32,
) {
    let rect = viewport.rect;
    let mut low = (f32::INFINITY, f32::INFINITY);
    let mut high = (f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
    ] {
        let rel = corner - rect.center() - viewport.pan;
        let (x, y) = viewport
            .placement
            .unplace_f(rel.x / viewport.zoom, rel.y / viewport.zoom);
        low = (low.0.min(x), low.1.min(y));
        high = (high.0.max(x), high.1.max(y));
    }
    let span =
        |from: f32, to: f32| (from / model_step).floor() as i64..=(to / model_step).ceil() as i64;
    for column in span(low.0, high.0) {
        for row in span(low.1, high.1) {
            let (x, y) = viewport
                .placement
                .place_xy(column as f32 * model_step, row as f32 * model_step);
            let dot = rect.center() + viewport.pan + vec2(x * viewport.zoom, y * viewport.zoom);
            if rect.contains(dot) {
                painter.circle_filled(dot, 1.1, color);
            }
        }
    }
}

fn draw_body(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    color: Color32,
    selected_shapes: &std::collections::BTreeSet<usize>,
    selected_color: Color32,
) {
    for (index, shape) in document.body.iter().enumerate() {
        let is_selected = selected_shapes.contains(&index);
        let shape_color = if is_selected { selected_color } else { color };
        let style = shape.style().unwrap_or_default();
        let stroke = styled_stroke(
            Stroke::new(if is_selected { 2.0 } else { 1.3 }, shape_color),
            style,
        );
        let screen = |point: Point| viewport.world_to_screen(point);
        match shape {
            SymbolShape::Polyline { points, .. } => {
                let outline: Vec<Pos2> = points.iter().copied().map(screen).collect();
                let closed = shape.is_closed();
                if closed {
                    paint_fill(painter, &outline, style, shape_color);
                }
                draw_outline(painter, &outline, closed, stroke, style, viewport.zoom);
            }
            SymbolShape::Rectangle { min, max, .. } => {
                let outline = rectangle_corners(*min, *max).map(screen);
                paint_fill(painter, &outline, style, shape_color);
                draw_outline(painter, &outline, true, stroke, style, viewport.zoom);
            }
            SymbolShape::Circle { center, radius, .. } => {
                let (middle, reach) = (screen(*center), *radius as f32 * viewport.zoom);
                if style.fill == SymbolFill::Solid {
                    painter.circle_filled(middle, reach, shape_color);
                }
                if style.dash == SymbolDash::Solid {
                    painter.circle_stroke(middle, reach, stroke);
                } else {
                    let ring = SymbolCurve::of_arc(*center, *radius, 0, 360);
                    draw_curve(
                        painter,
                        curve_points(viewport, &ring),
                        stroke,
                        style,
                        viewport,
                    );
                }
            }
            SymbolShape::Arc {
                center,
                radius,
                start_degrees,
                sweep_degrees,
                ..
            } => {
                let arc = SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees);
                draw_curve(
                    painter,
                    curve_points(viewport, &arc),
                    stroke,
                    style,
                    viewport,
                );
            }
            SymbolShape::Curve { curve, .. } => {
                draw_curve(
                    painter,
                    curve_points(viewport, curve),
                    stroke,
                    style,
                    viewport,
                );
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => draw_arrow(painter, viewport, *tip, *rotation_quarters, shape_color),
            SymbolShape::Dot { center, radius } => {
                painter.circle_filled(
                    viewport.world_to_screen(*center),
                    *radius as f32 * viewport.zoom,
                    shape_color,
                );
            }
            SymbolShape::Text {
                anchor,
                text,
                size,
                align,
                rotation,
            } => {
                let at = viewport.world_to_screen(*anchor);
                let hang = editor_text_align(viewport.placement, *align, *rotation);
                let font = theme::mono(
                    (size.height() as f32 * viewport.zoom).max(1.0),
                    FontWeight::Regular,
                );
                match rotation {
                    SymbolTextRotation::R0 => {
                        painter.text(at, hang, text, font, shape_color);
                    }
                    SymbolTextRotation::R90 => {
                        paint_upward_text(painter, at, hang, text, font, shape_color);
                    }
                }
            }
        }
    }
}

/// The outline `outline` in `style`: broken where the style breaks it, else
/// segment by segment, the way the canvas has always drawn a body.
fn draw_outline(
    painter: &egui::Painter,
    outline: &[Pos2],
    closed: bool,
    stroke: Stroke,
    style: SymbolStyle,
    zoom: f32,
) {
    if let Some(pieces) = broken_outline(outline, closed, stroke, style, zoom) {
        painter.extend(pieces);
        return;
    }
    for pair in outline.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
    if closed && let (Some(first), Some(last)) = (outline.first(), outline.last()) {
        painter.line_segment([*last, *first], stroke);
    }
}

/// A curve's tessellated `points` in `style`.
fn draw_curve(
    painter: &egui::Painter,
    points: Vec<Pos2>,
    stroke: Stroke,
    style: SymbolStyle,
    viewport: SymbolViewport,
) {
    match broken_outline(&points, false, stroke, style, viewport.zoom) {
        Some(pieces) => painter.extend(pieces),
        None => {
            painter.add(Shape::line(points, stroke));
        }
    }
}

/// How a text run hangs off its anchor once `placement` has turned the
/// symbol: the run keeps its reading and moves around its anchor, by the
/// rule every placed instance is drawn with.
fn editor_text_align(
    placement: SymbolPlacement,
    align: SymbolTextAlign,
    rotation: SymbolTextRotation,
) -> Align2 {
    text_hang(
        align.placement(rotation, |point| placement.orient(point)),
        rotation,
    )
}

fn draw_bbox_and_origin(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    t: &Tokens,
) {
    let (min, max) = document_bounds(document);
    let min = viewport.world_to_screen(min);
    let max = viewport.world_to_screen(max);
    // A turned symbol swaps which corner is the minimum on screen.
    let rect = Rect::from_two_pos(min, max);
    draw_dashed_rect(
        painter,
        rect,
        Stroke::new(1.0, t.color.text_faint.gamma_multiply(0.65)),
        6.0,
        4.0,
    );
    let origin = viewport.world_to_screen(document.origin);
    painter.line_segment(
        [origin + vec2(-8.0, 0.0), origin + vec2(8.0, 0.0)],
        Stroke::new(1.0, t.color.accent),
    );
    painter.line_segment(
        [origin + vec2(0.0, -8.0), origin + vec2(0.0, 8.0)],
        Stroke::new(1.0, t.color.accent),
    );
}

fn draw_pins(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    ports: &[PortSpec],
    session: &SymbolEditorSession,
) {
    let t = Tokens::get(painter.ctx());
    let selection = session.effective_selection();
    let port_names: std::collections::HashSet<String> = ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect();
    for pin in &document.pins {
        let Some(position) = pin.position else {
            continue;
        };
        let start = viewport.world_to_screen(position);
        let drawing = pin_drawing(position, pin.side, pin.length, pin.style);
        let orphan = !ports.is_empty() && !port_names.contains(&pin.name.to_ascii_lowercase());
        let color = if orphan { t.color.err } else { t.color.wire };
        let stroke = Stroke::new(1.2, color);
        paint_pin_drawing(painter, viewport, drawing, stroke);
        let pad = Rect::from_center_size(start, vec2(8.0, 8.0));
        let pad_stroke = if selection.pins.contains(&pin.name) {
            Stroke::new(1.8, t.color.accent)
        } else if !pin.terminal_on_grid() {
            Stroke::new(1.6, t.color.err)
        } else {
            stroke
        };
        painter.rect_stroke(pad, 0.0, pad_stroke, egui::StrokeKind::Inside);
        let label_pos = pin_label_pos(position, pin.side, viewport);
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            &pin.name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.symbol,
        );
        draw_direction_mark(painter, viewport, pin.direction, drawing.lead, color);
    }
}

/// The pin the Pin tool would place, at the pointer, in the draft tone: its
/// lead on the side it would take and its name, so where a click puts it
/// and what it is called are on the canvas before it is in the document.
fn draw_pin_tool_pin(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    session: &SymbolEditorSession,
    t: &Tokens,
) {
    if session.tool != SymbolTool::PlacePin || session.pin_name_draft.is_some() {
        return;
    }
    let Some(tip) = session.pointer else {
        return;
    };
    let offer = interaction::next_pin_offer(document, session);
    let side = interaction::pin_tool_side(document, session, tip);
    let (length, style) = document
        .pin(offer.name())
        .map_or((SYMBOL_PIN_LENGTH, SymbolPinStyle::Plain), |pin| {
            (pin.length, pin.style)
        });
    let colour = t.color.canvas_select.gamma_multiply(DRAFT_ALPHA);
    let stroke = Stroke::new(DRAFT_STROKE, colour);
    paint_pin_drawing(
        painter,
        viewport,
        pin_drawing(tip, side, length, style),
        stroke,
    );
    let pad = Rect::from_center_size(viewport.world_to_screen(tip), vec2(8.0, 8.0));
    painter.rect_stroke(pad, 0.0, stroke, egui::StrokeKind::Inside);
    painter.text(
        pin_label_pos(tip, side, viewport),
        Align2::CENTER_CENTER,
        offer.name(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        colour,
    );
}

/// A pin's lead and the marks its style puts where the lead meets the body.
fn paint_pin_drawing(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    drawing: PinDrawing,
    stroke: Stroke,
) {
    if let Some((tip, end)) = drawing.lead {
        painter.line_segment(
            [viewport.world_to_screen(tip), viewport.world_to_screen(end)],
            stroke,
        );
    }
    if let Some((centre, radius)) = drawing.bubble {
        painter.circle_stroke(
            viewport.world_to_screen(centre),
            radius as f32 * viewport.zoom,
            stroke,
        );
    }
    if let Some(wedge) = drawing.wedge {
        let corners = wedge.map(|corner| viewport.world_to_screen(corner));
        painter.add(Shape::line(corners.to_vec(), stroke));
    }
}

fn draw_marquee(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    session: &SymbolEditorSession,
    t: &Tokens,
) {
    let (Some(start), Some(current)) = (session.marquee_start, session.marquee_current) else {
        return;
    };
    let rect = Rect::from_two_pos(
        viewport.world_to_screen(start),
        viewport.world_to_screen(current),
    );
    painter.rect_filled(rect, 0.0, t.color.accent.gamma_multiply(0.08));
    draw_dashed_rect(
        painter,
        rect,
        Stroke::new(1.0, t.color.accent.gamma_multiply(0.85)),
        5.0,
        4.0,
    );
}

fn draw_dashed_rect(painter: &egui::Painter, rect: Rect, stroke: Stroke, dash: f32, gap: f32) {
    let corners = [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
    ];
    for index in 0..corners.len() {
        draw_dashed_line(
            painter,
            corners[index],
            corners[(index + 1) % corners.len()],
            stroke,
            dash,
            gap,
        );
    }
}

fn draw_dashed_line(
    painter: &egui::Painter,
    start: Pos2,
    end: Pos2,
    stroke: Stroke,
    dash: f32,
    gap: f32,
) {
    let delta = end - start;
    let length = delta.length();
    if length <= f32::EPSILON {
        return;
    }
    let direction = delta / length;
    let mut cursor = 0.0;
    while cursor < length {
        let next = (cursor + dash).min(length);
        painter.line_segment(
            [start + direction * cursor, start + direction * next],
            stroke,
        );
        cursor += dash + gap;
    }
}

fn draw_labels(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    editor: &SymbolEditorMetadata,
    session: &SymbolEditorSession,
    t: &Tokens,
) {
    let selection = session.effective_selection();
    for attribute in &editor.attributes {
        if !attribute.shown {
            continue;
        }
        painter.text(
            viewport.world_to_screen(attribute.position),
            Align2::LEFT_CENTER,
            format!("@{}", attribute.kind.key()),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            if selection.attributes.contains(&attribute.kind) {
                t.color.accent
            } else {
                t.color.net_label
            },
        );
    }
}

fn draw_preview_tile(
    ui: &mut Ui,
    rect: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    reference: &CellViewRef,
    t: &Tokens,
) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, t.radius_lg, t.color.bg_panel);
    painter.rect_stroke(
        rect,
        t.radius_lg,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    let header = Rect::from_min_max(
        rect.min,
        pos2(rect.right(), rect.top() + PREVIEW_TILE_HEADER),
    );
    painter.hline(
        header.x_range(),
        header.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let body = Rect::from_min_max(
        pos2(rect.left(), rect.top() + PREVIEW_TILE_HEADER),
        rect.max,
    );
    let viewport = preview_viewport_for_tile(body, document);
    painter.text(
        header.left_center() + vec2(8.0, 0.0),
        Align2::LEFT_CENTER,
        preview_scale_label(viewport.zoom),
        theme::mono(10.0, FontWeight::Regular),
        t.color.text_faint,
    );
    let mut binding = LibraryCellInstance::new(&reference.library, &reference.cell, "schematic");
    binding.bind_interface(ports);
    let mut component =
        Component::new(0, ComponentType::CellInstance, Point::origin()).with_library_cell(binding);
    component.name = "X1".to_owned();
    component.value = reference.cell.clone();
    let resolved = ResolvedCellSymbol::from_authored_document(document.clone(), ports);
    let symbol_painter = painter.with_clip_rect(body.shrink(2.0));
    draw_resolved_symbol(
        &symbol_painter,
        viewport.world_to_screen(Point::origin()),
        viewport.zoom,
        &component,
        &resolved,
        Stroke::new(1.1, t.color.symbol),
    );
}

/// Draw a symbol document fitted into `rect`, exactly as it will appear on
/// a sheet. Shared by the editor's as-placed tile and the inspector hero so
/// the two can never disagree about what the symbol looks like.
pub fn draw_document_preview(
    painter: &egui::Painter,
    rect: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    cell: &str,
    color: egui::Color32,
) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let viewport = preview_viewport_for_tile(rect, document);
    let mut component = Component::new(0, ComponentType::CellInstance, Point::origin());
    component.name = "X1".to_owned();
    component.value = cell.to_owned();
    let resolved = ResolvedCellSymbol::from_authored_document(document.clone(), ports);
    draw_resolved_symbol(
        &painter.with_clip_rect(rect),
        viewport.world_to_screen(Point::origin()),
        viewport.zoom,
        &component,
        &resolved,
        Stroke::new(1.1, color),
    );
}

/// The preview's header: the scale the tile draws the symbol at, in the
/// status bar's terms, where 100% is a schematic at 100%.
fn preview_scale_label(zoom: f32) -> String {
    format!("AS PLACED \u{b7} {}%", (zoom * 100.0).round())
}

/// The tile, at the foot of the preview's own `column`.
fn preview_tile_rect(column: Rect) -> Rect {
    let width = (column.width() - PREVIEW_TILE_MARGIN * 2.0).max(0.0);
    let available_height = (column.height() - PREVIEW_TILE_MARGIN * 2.0).max(0.0);
    let height = PREVIEW_TILE_MAX_HEIGHT
        .min(available_height)
        .max(PREVIEW_TILE_MIN_HEIGHT.min(available_height));
    Rect::from_min_size(
        column.right_bottom() - vec2(width + PREVIEW_TILE_MARGIN, height + PREVIEW_TILE_MARGIN),
        vec2(width, height),
    )
}

fn preview_viewport_for_tile(rect: Rect, document: &SymbolDocument) -> SymbolViewport {
    let (min, max) = preview_effective_bounds(document);
    let width = (max.x - min.x).abs().max(1) as f32;
    let height = (max.y - min.y).abs().max(1) as f32;
    let fit_rect = rect.shrink(PREVIEW_FIT_PADDING);
    let zoom = (fit_rect.width() / width)
        .min(fit_rect.height() / height)
        .clamp(0.05, 1.8);
    let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
    SymbolViewport::new(
        rect,
        zoom,
        vec2(-(center.x as f32) * zoom, -(center.y as f32) * zoom),
    )
}

fn preview_effective_bounds(document: &SymbolDocument) -> (Point, Point) {
    let (min, max) = document_bounds(document);
    (min - document.origin, max - document.origin)
}

pub fn hit_pin(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<String> {
    document
        .pins
        .iter()
        .filter_map(|pin| Some((pin.name.clone(), viewport.world_to_screen(pin.position?))))
        .find(|(_, pin_pos)| pin_pos.distance(pos) <= PIN_HIT_RADIUS)
        .map(|(name, _)| name)
}

pub fn hit_label(
    editor: &SymbolEditorMetadata,
    viewport: SymbolViewport,
    pos: Pos2,
) -> Option<SymbolAttributeKind> {
    editor
        .attributes
        .iter()
        .rev()
        .find(|attribute| {
            attribute.shown && viewport.world_to_screen(attribute.position).distance(pos) <= 22.0
        })
        .map(|attribute| attribute.kind)
}

/// The document's own label anchors are the pre-attribute spelling of the
/// refdes and value positions, and every renderer outside the editor still
/// reads them. Moving an attribute has to move both or the two disagree.
pub fn hit_origin(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> bool {
    viewport.world_to_screen(document.origin).distance(pos) <= 10.0
}

/// The handle of a selected shape nearest `pos`, within the reach the
/// schematic's own grips are taken from.
pub fn hit_handle(
    document: &SymbolDocument,
    selection: &session::SymbolSelection,
    viewport: SymbolViewport,
    pos: Pos2,
) -> Option<handles::SymbolHandleRef> {
    let reach = crate::view::snap_resolution::TARGET_ACQUISITION_RADIUS_POINTS;
    selection
        .shapes
        .iter()
        .filter_map(|&index| Some((index, document.body.get(index)?)))
        .flat_map(|(index, shape)| {
            handles::shape_handles(shape)
                .into_iter()
                .map(move |(handle, at)| (index, handle, at))
        })
        .map(|(shape, handle, at)| {
            let distance = viewport.world_to_screen_f(at).distance(pos);
            (handles::SymbolHandleRef { shape, handle }, distance)
        })
        .filter(|(_, distance)| *distance <= reach)
        .min_by(|left, right| left.1.total_cmp(&right.1))
        .map(|(handle, _)| handle)
}

pub fn hit_shape(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<usize> {
    document
        .body
        .iter()
        .enumerate()
        .rev()
        .find(|(_, shape)| shape_hit(shape, viewport, pos))
        .map(|(index, _)| index)
}

fn shape_hit(shape: &SymbolShape, viewport: SymbolViewport, pos: Pos2) -> bool {
    const HIT_PX: f32 = 7.0;
    // A filled shape is ink all the way across, so it is picked anywhere
    // inside, as a text run is, and not only on its edge.
    if shape.is_filled() && inside_filled_shape(shape, viewport, pos) {
        return true;
    }
    match shape {
        SymbolShape::Polyline { points, closed, .. } => {
            points.windows(2).any(|pair| {
                distance_to_screen_segment(
                    pos,
                    viewport.world_to_screen(pair[0]),
                    viewport.world_to_screen(pair[1]),
                ) <= HIT_PX
            }) || (*closed
                && points.len() > 2
                && points
                    .first()
                    .zip(points.last())
                    .is_some_and(|(first, last)| {
                        distance_to_screen_segment(
                            pos,
                            viewport.world_to_screen(*last),
                            viewport.world_to_screen(*first),
                        ) <= HIT_PX
                    }))
        }
        SymbolShape::Rectangle { min, max, .. } => {
            let corners =
                rectangle_corners(*min, *max).map(|point| viewport.world_to_screen(point));
            (0..corners.len()).any(|index| {
                distance_to_screen_segment(
                    pos,
                    corners[index],
                    corners[(index + 1) % corners.len()],
                ) <= HIT_PX
            })
        }
        SymbolShape::Circle { center, radius, .. } => {
            let center = viewport.world_to_screen(*center);
            let radius = *radius as f32 * viewport.zoom;
            (center.distance(pos) - radius).abs() <= HIT_PX
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
            ..
        } => curve_hit(
            &SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees),
            viewport,
            pos,
            HIT_PX,
        ),
        SymbolShape::Curve { curve, .. } => curve_hit(curve, viewport, pos, HIT_PX),
        SymbolShape::Arrow { tip, .. } => viewport.world_to_screen(*tip).distance(pos) <= 18.0,
        SymbolShape::Dot { center, radius } => {
            viewport.world_to_screen(*center).distance(pos)
                <= (*radius as f32 * viewport.zoom + HIT_PX)
        }
        // Text is picked anywhere on the run, not on an outline it has none
        // of, so a short label is as easy to grab as a long one.
        SymbolShape::Text {
            anchor,
            text,
            size,
            align,
            rotation,
        } => {
            let (min, max) =
                rspice_design::symbol::symbol_text_bounds(*anchor, text, *size, *align, *rotation);
            Rect::from_two_pos(viewport.world_to_screen(min), viewport.world_to_screen(max))
                .expand(HIT_PX)
                .contains(pos)
        }
    }
}

fn inside_filled_shape(shape: &SymbolShape, viewport: SymbolViewport, pos: Pos2) -> bool {
    let screen = |point: Point| viewport.world_to_screen(point);
    match shape {
        SymbolShape::Polyline { points, .. } => {
            inside_outline(&points.iter().copied().map(screen).collect::<Vec<_>>(), pos)
        }
        SymbolShape::Rectangle { min, max, .. } => {
            inside_outline(&rectangle_corners(*min, *max).map(screen), pos)
        }
        SymbolShape::Circle { center, radius, .. } => {
            screen(*center).distance(pos) <= *radius as f32 * viewport.zoom
        }
        _ => false,
    }
}

fn distance_to_screen_segment(point: Pos2, start: Pos2, end: Pos2) -> f32 {
    let segment = end - start;
    let len_sq = segment.length_sq();
    if len_sq <= f32::EPSILON {
        return point.distance(start);
    }
    let t = ((point - start).dot(segment) / len_sq).clamp(0.0, 1.0);
    let closest = start + segment * t;
    point.distance(closest)
}

pub fn snap_point(point: Point, spacing: SymbolGridSpacing) -> Point {
    let grid = spacing.model_step() as f32;
    let snap = |v: i32| ((v as f32 / grid).round() * grid) as i32;
    Point::new(snap(point.x), snap(point.y))
}

/// Terminal placement is pinned to [`SYMBOL_TERMINAL_GRID`] independently of
/// the display lattice and of the body snap toggle.
pub fn snap_to_terminal_grid(point: Point) -> Point {
    let snap = |value: i32| {
        let grid = SYMBOL_TERMINAL_GRID;
        let remainder = value.rem_euclid(grid);
        if remainder * 2 >= grid {
            value - remainder + grid
        } else {
            value - remainder
        }
    };
    Point::new(snap(point.x), snap(point.y))
}

/// Where a pin's name stands: out past its terminal, away from the body on
/// the side the pin sits on, as that side faces once the symbol is turned.
fn pin_label_pos(position: Point, side: SymbolPinSide, viewport: SymbolViewport) -> Pos2 {
    let pad = SYMBOL_TERMINAL_GRID as f32 * viewport.zoom * 0.75;
    let base = viewport.world_to_screen(position);
    let inward = rspice_design::symbol_generation::inward_step(side);
    let outward = viewport.screen_direction(-inward.x as f32, -inward.y as f32);
    if outward.x.abs() > outward.y.abs() {
        base + vec2(pad * outward.x.signum(), -8.0)
    } else {
        base + vec2(26.0, pad * 0.4 * outward.y.signum())
    }
}

/// Signal-flow arrow on a pin's drawn lead: an input points into the body
/// it drives, an output points out toward the net it drives.
fn draw_direction_mark(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    direction: PortDirection,
    lead: Option<(Point, Point)>,
    color: Color32,
) {
    if !matches!(direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let Some((tip, end)) = lead else {
        return;
    };
    let terminal = viewport.world_to_screen(tip);
    let inner = viewport.world_to_screen(end);
    let lead = inner - terminal;
    let inward = lead.normalized();
    if !inward.is_finite() {
        return;
    }
    let length = (MARK_LENGTH * viewport.zoom).min(lead.length() * 0.5);
    let base_offset = (lead.length() - length) * 0.5;
    let (tip, base) = if direction == PortDirection::In {
        (
            terminal + inward * (base_offset + length),
            terminal + inward * base_offset,
        )
    } else {
        (
            terminal + inward * base_offset,
            terminal + inward * (base_offset + length),
        )
    };
    let across = vec2(-inward.y, inward.x) * (MARK_HALF_WIDTH * viewport.zoom);
    painter.add(Shape::convex_polygon(
        vec![tip, base + across, base - across],
        color,
        Stroke::NONE,
    ));
}

/// Direction-mark extents in symbol units.
const MARK_LENGTH: f32 = 5.0;
const MARK_HALF_WIDTH: f32 = 2.0;

fn draw_arrow(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    tip: Point,
    rotation_quarters: i32,
    color: Color32,
) {
    let tip = viewport.world_to_screen(tip);
    let (x, y) = match rotation_quarters.rem_euclid(4) {
        0 => (1.0, 0.0),
        1 => (0.0, 1.0),
        2 => (-1.0, 0.0),
        _ => (0.0, -1.0),
    };
    let dir = viewport.screen_direction(x, y);
    let side = vec2(-dir.y, dir.x);
    painter.add(Shape::convex_polygon(
        vec![
            tip,
            tip - dir * 12.0 + side * 6.0,
            tip - dir * 12.0 - side * 6.0,
        ],
        color,
        Stroke::NONE,
    ));
}

/// Points along `curve` on screen, as many as the view's size needs.
fn curve_points(viewport: SymbolViewport, curve: &SymbolCurve) -> Vec<Pos2> {
    let origin = viewport.world_to_screen(Point::origin());
    let across = viewport.world_to_screen(Point::new(1, 0)) - origin;
    let down = viewport.world_to_screen(Point::new(0, 1)) - origin;
    curve
        .tessellated(f64::from(viewport.zoom))
        .into_iter()
        .map(|[x, y]| origin + across * x as f32 + down * y as f32)
        .collect()
}

fn curve_hit(curve: &SymbolCurve, viewport: SymbolViewport, pos: Pos2, reach: f32) -> bool {
    curve_points(viewport, curve)
        .windows(2)
        .any(|pair| distance_to_screen_segment(pos, pair[0], pair[1]) <= reach)
}

fn document_bounds(document: &SymbolDocument) -> (Point, Point) {
    let mut xs = vec![
        document.origin.x,
        document.name_anchor.x,
        document.value_anchor.x,
    ];
    let mut ys = vec![
        document.origin.y,
        document.name_anchor.y,
        document.value_anchor.y,
    ];
    for pin in &document.pins {
        if let Some(position) = pin.position {
            xs.push(position.x);
            ys.push(position.y);
        }
    }
    for shape in &document.body {
        match shape {
            SymbolShape::Polyline { points, .. } => {
                for point in points {
                    xs.push(point.x);
                    ys.push(point.y);
                }
            }
            SymbolShape::Rectangle { min, max, .. } => {
                xs.extend([min.x, max.x]);
                ys.extend([min.y, max.y]);
            }
            SymbolShape::Circle { center, radius, .. } | SymbolShape::Dot { center, radius } => {
                xs.extend([center.x - radius, center.x + radius]);
                ys.extend([center.y - radius, center.y + radius]);
            }
            SymbolShape::Arc { center, radius, .. } => {
                xs.extend([center.x - radius, center.x + radius]);
                ys.extend([center.y - radius, center.y + radius]);
            }
            SymbolShape::Curve { curve, .. } => {
                let (least, greatest) = curve.whole_unit_bounds();
                xs.extend([least.x, greatest.x]);
                ys.extend([least.y, greatest.y]);
            }
            SymbolShape::Arrow { tip, .. } => {
                xs.extend([tip.x - 10, tip.x + 10]);
                ys.extend([tip.y - 10, tip.y + 10]);
            }
            SymbolShape::Text {
                anchor,
                text,
                size,
                align,
                rotation,
            } => {
                let (min, max) = rspice_design::symbol::symbol_text_bounds(
                    *anchor, text, *size, *align, *rotation,
                );
                xs.extend([min.x, max.x]);
                ys.extend([min.y, max.y]);
            }
        }
    }
    let min_x = xs.iter().min().copied().unwrap_or(-40) - 20;
    let max_x = xs.iter().max().copied().unwrap_or(40) + 20;
    let min_y = ys.iter().min().copied().unwrap_or(-40) - 20;
    let max_y = ys.iter().max().copied().unwrap_or(40) + 20;
    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

#[cfg(test)]
mod tests;
