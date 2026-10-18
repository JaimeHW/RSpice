//! Symbol canvas geometry, viewport and painting over explicit editor inputs.

pub mod commands;
pub mod inspector;
pub mod interaction;
pub mod placement;
pub mod session;
pub mod surface;

use crate::view::navigation::{CanvasNavigation, WheelAction};
use crate::view::resolved_symbol_render::draw_resolved_symbol;
use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, Ui, Vec2, pos2, vec2};
use placement::SymbolPlacement;
use rspice_design::{
    resolved_symbol::ResolvedCellSymbol,
    schematic::{
        component::{Component, LibraryCellInstance},
        component_type::ComponentType,
    },
    symbol::{
        SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolPin,
        SymbolShape, SymbolTextAlign,
    },
};
use rspice_design_model::{
    Point,
    cell_view::CellViewRef,
    port::{PortDirection, PortSpec},
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};
use session::{SymbolEditorSession, SymbolGridSpacing};

const PIN_HIT_RADIUS: f32 = 10.0;
const SCROLL_ZOOM_SENSITIVITY: f32 = 0.001;
const PREVIEW_TILE_MAX_WIDTH: f32 = 240.0;
const PREVIEW_TILE_MAX_HEIGHT: f32 = 168.0;
const PREVIEW_TILE_MIN_WIDTH: f32 = 168.0;
const PREVIEW_TILE_MIN_HEIGHT: f32 = 118.0;
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

pub fn update_viewport(
    ui: &mut Ui,
    session: &mut SymbolEditorSession,
    rect: Rect,
    document: &SymbolDocument,
    response: &egui::Response,
    navigation: CanvasNavigation,
) -> SymbolViewport {
    if session.needs_fit {
        fit_symbol_view(session, rect, document);
        session.needs_fit = false;
    }
    if response.hovered() {
        let (scroll, ctrl_zoom) = ui.input(|input| (input.smooth_scroll_delta, input.zoom_delta()));
        let step = match navigation.wheel {
            // egui turns Shift+wheel into sideways travel, which scrolls.
            WheelAction::Zoom => {
                session.pan.0 += scroll.x;
                symbol_scroll_zoom_factor(scroll.y)
            }
            // egui hands Ctrl+wheel over as a zoom rather than a scroll.
            WheelAction::Scroll => {
                session.pan.0 += scroll.x;
                session.pan.1 += scroll.y;
                (ctrl_zoom != 1.0).then_some(ctrl_zoom)
            }
        };
        if let Some(factor) = step {
            let factor = navigation.scale_zoom_step(f64::from(factor)) as f32;
            session.zoom = (session.zoom * factor).clamp(1.0, 18.0);
        }
    }
    if response.dragged_by(navigation.pan_pointer_button()) {
        let delta = ui.input(|input| input.pointer.delta());
        session.pan.0 += delta.x;
        session.pan.1 += delta.y;
    }
    SymbolViewport::new(rect, session.zoom, vec2(session.pan.0, session.pan.1))
}

fn symbol_scroll_zoom_factor(scroll: f32) -> Option<f32> {
    if scroll.abs() <= f32::EPSILON {
        return None;
    }
    Some((scroll * SCROLL_ZOOM_SENSITIVITY).exp().clamp(0.5, 2.0))
}

fn fit_symbol_view(session: &mut SymbolEditorSession, rect: Rect, document: &SymbolDocument) {
    let (min, max) = document_bounds(document);
    let width = (max.x - min.x).abs().max(80) as f32;
    let height = (max.y - min.y).abs().max(80) as f32;
    let zoom = ((rect.width() - 96.0).max(80.0) / width)
        .min((rect.height() - 96.0).max(80.0) / height)
        .clamp(1.0, 8.0);
    let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
    session.zoom = zoom;
    session.pan = (-(center.x as f32) * zoom, -(center.y as f32) * zoom);
}

pub fn draw_canvas(
    ui: &mut Ui,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    ports: &[PortSpec],
    session: &SymbolEditorSession,
    reference: &CellViewRef,
) {
    let t = Tokens::get(ui.ctx());
    let painter = ui.painter_at(viewport.rect);
    painter.rect_filled(viewport.rect, 0.0, t.color.canvas_bg);
    draw_in_place(&painter, viewport, document, editor, ports, session);
    if session.preview_as_placed {
        draw_preview_tile(ui, viewport.rect, document, ports, reference, &t);
    }
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
    draw_labels(painter, viewport, editor, session, &t);
    draw_marquee(painter, viewport, session, &t);
}

fn draw_grid(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    color: Color32,
    spacing: SymbolGridSpacing,
) {
    let step = spacing.model_step() * viewport.zoom;
    if step < 8.0 {
        return;
    }
    if !viewport.placement.is_identity() {
        draw_placed_grid(painter, viewport, color, spacing.model_step());
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
        let stroke = Stroke::new(if is_selected { 2.0 } else { 1.3 }, shape_color);
        match shape {
            SymbolShape::Polyline { points, closed } => {
                for pair in points.windows(2) {
                    painter.line_segment(
                        [
                            viewport.world_to_screen(pair[0]),
                            viewport.world_to_screen(pair[1]),
                        ],
                        stroke,
                    );
                }
                if *closed
                    && points.len() > 2
                    && let (Some(first), Some(last)) = (points.first(), points.last())
                {
                    painter.line_segment(
                        [
                            viewport.world_to_screen(*last),
                            viewport.world_to_screen(*first),
                        ],
                        stroke,
                    );
                }
            }
            SymbolShape::Circle { center, radius } => {
                painter.circle_stroke(
                    viewport.world_to_screen(*center),
                    *radius as f32 * viewport.zoom,
                    stroke,
                );
            }
            SymbolShape::Arc {
                center,
                radius,
                start_degrees,
                sweep_degrees,
            } => {
                let points = arc_points(viewport, *center, *radius, *start_degrees, *sweep_degrees);
                painter.add(Shape::line(points, stroke));
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
            } => {
                painter.text(
                    viewport.world_to_screen(*anchor),
                    editor_text_align(viewport.placement, *align),
                    text,
                    theme::mono(
                        (size.height() as f32 * viewport.zoom).max(1.0),
                        FontWeight::Regular,
                    ),
                    shape_color,
                );
            }
        }
    }
}

/// How a text run hangs off its anchor once `placement` has turned the
/// symbol: glyphs stay upright and the run moves around its anchor, by the
/// rule every placed instance is drawn with.
fn editor_text_align(placement: SymbolPlacement, align: SymbolTextAlign) -> Align2 {
    use rspice_design::symbol::SymbolTextPlacement;
    match align.placement(|point| placement.orient(point)) {
        SymbolTextPlacement::On => Align2::CENTER_CENTER,
        SymbolTextPlacement::After => Align2::LEFT_CENTER,
        SymbolTextPlacement::Before => Align2::RIGHT_CENTER,
        SymbolTextPlacement::Below => Align2::CENTER_TOP,
        SymbolTextPlacement::Above => Align2::CENTER_BOTTOM,
    }
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
    let body = document.drawn_body_bounds();
    for pin in &document.pins {
        let Some(position) = pin.position else {
            continue;
        };
        let start = viewport.world_to_screen(position);
        let inner = viewport.world_to_screen(rspice_design::symbol_generation::lead_inner(
            position,
            document.pin_side(pin),
            body,
        ));
        let orphan = !ports.is_empty() && !port_names.contains(&pin.name.to_ascii_lowercase());
        let color = if orphan { t.color.err } else { t.color.wire };
        let stroke = Stroke::new(1.2, color);
        painter.line_segment([start, inner], stroke);
        let pad = Rect::from_center_size(start, vec2(8.0, 8.0));
        let pad_stroke = if selection.pins.contains(&pin.name) {
            Stroke::new(1.8, t.color.accent)
        } else if !pin.terminal_on_grid() {
            Stroke::new(1.6, t.color.err)
        } else {
            stroke
        };
        painter.rect_stroke(pad, 0.0, pad_stroke, egui::StrokeKind::Inside);
        let label_pos = pin_label_pos(position, viewport);
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            &pin.name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.symbol,
        );
        draw_direction_mark(painter, viewport, document, pin, color);
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
    canvas: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    reference: &CellViewRef,
    t: &Tokens,
) {
    let rect = preview_tile_rect(canvas);
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
    painter.text(
        header.left_center() + vec2(8.0, 0.0),
        Align2::LEFT_CENTER,
        "AS PLACED - 100%",
        theme::mono(10.0, FontWeight::Regular),
        t.color.text_faint,
    );
    let body = Rect::from_min_max(
        pos2(rect.left(), rect.top() + PREVIEW_TILE_HEADER),
        rect.max,
    );
    let viewport = preview_viewport_for_tile(body, document);
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

fn preview_tile_rect(canvas: Rect) -> Rect {
    let available_width = (canvas.width() - PREVIEW_TILE_MARGIN * 2.0).max(96.0);
    let available_height = (canvas.height() - PREVIEW_TILE_MARGIN * 2.0).max(96.0);
    let width = PREVIEW_TILE_MAX_WIDTH
        .min(available_width)
        .max(PREVIEW_TILE_MIN_WIDTH.min(available_width));
    let height = PREVIEW_TILE_MAX_HEIGHT
        .min(available_height)
        .max(PREVIEW_TILE_MIN_HEIGHT.min(available_height));
    Rect::from_min_size(
        canvas.right_bottom() - vec2(width + PREVIEW_TILE_MARGIN, height + PREVIEW_TILE_MARGIN),
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
    match shape {
        SymbolShape::Polyline { points, closed } => {
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
        SymbolShape::Circle { center, radius } => {
            let center = viewport.world_to_screen(*center);
            let radius = *radius as f32 * viewport.zoom;
            (center.distance(pos) - radius).abs() <= HIT_PX
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
        } => arc_points(viewport, *center, *radius, *start_degrees, *sweep_degrees)
            .windows(2)
            .any(|pair| distance_to_screen_segment(pos, pair[0], pair[1]) <= HIT_PX),
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
        } => {
            let (min, max) =
                rspice_design::symbol::symbol_text_bounds(*anchor, text, *size, *align);
            Rect::from_two_pos(viewport.world_to_screen(min), viewport.world_to_screen(max))
                .expand(HIT_PX)
                .contains(pos)
        }
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
    let grid = spacing.model_step();
    let snap = |v: i32| ((v as f32 / grid).round() * grid).round() as i32;
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

/// Where a pin's name stands: out past its terminal, on the side the
/// terminal faces once the symbol is turned.
fn pin_label_pos(position: Point, viewport: SymbolViewport) -> Pos2 {
    let pad = SYMBOL_TERMINAL_GRID as f32 * viewport.zoom * 0.75;
    let base = viewport.world_to_screen(position);
    let outward = match (position.x.signum(), position.y.signum()) {
        (0, y) => viewport.screen_direction(0.0, if y < 0 { -1.0 } else { 1.0 }),
        (x, _) => viewport.screen_direction(x as f32, 0.0),
    };
    if outward.x.abs() > outward.y.abs() {
        base + vec2(pad * outward.x.signum(), -8.0)
    } else {
        base + vec2(26.0, pad * 0.4 * outward.y.signum())
    }
}

/// Signal-flow arrow on a pin's lead: an input points into the body it
/// drives, an output points out toward the net it drives.
fn draw_direction_mark(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    pin: &SymbolPin,
    color: Color32,
) {
    if !matches!(pin.direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let Some(position) = pin.position else {
        return;
    };
    let terminal = viewport.world_to_screen(position);
    let inner = viewport.world_to_screen(rspice_design::symbol_generation::lead_inner(
        position,
        document.pin_side(pin),
        document.drawn_body_bounds(),
    ));
    let lead = inner - terminal;
    let inward = lead.normalized();
    if !inward.is_finite() {
        return;
    }
    let length = (MARK_LENGTH * viewport.zoom).min(lead.length() * 0.5);
    let base_offset = (lead.length() - length) * 0.5;
    let (tip, base) = if pin.direction == PortDirection::In {
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

fn arc_points(
    viewport: SymbolViewport,
    center: Point,
    radius: i32,
    start_degrees: i32,
    sweep_degrees: i32,
) -> Vec<Pos2> {
    let steps = 24.max(sweep_degrees.abs() / 8) as usize;
    (0..=steps)
        .map(|step| {
            let t = step as f32 / steps as f32;
            let degrees = start_degrees as f32 + sweep_degrees as f32 * t;
            let radians = degrees.to_radians();
            let point = Point::new(
                center.x + (radius as f32 * radians.cos()).round() as i32,
                center.y + (radius as f32 * radians.sin()).round() as i32,
            );
            viewport.world_to_screen(point)
        })
        .collect()
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
            SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                xs.extend([center.x - radius, center.x + radius]);
                ys.extend([center.y - radius, center.y + radius]);
            }
            SymbolShape::Arc { center, radius, .. } => {
                xs.extend([center.x - radius, center.x + radius]);
                ys.extend([center.y - radius, center.y + radius]);
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
            } => {
                let (min, max) =
                    rspice_design::symbol::symbol_text_bounds(*anchor, text, *size, *align);
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
