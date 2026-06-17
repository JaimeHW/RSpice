//! Symbol view surface.

use egui::{Align2, Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, pos2, vec2};

use crate::common::{AppState, ConsoleMessage};
use crate::schematic::view::resolved_symbol_render::draw_resolved_symbol;
use crate::shell::{SymbolSelection, SymbolTool};
use crate::state::{
    Component, ComponentType, LibraryCellInstance, PinSummary, Point, PortDirection, PortSpec,
    ResolvedCellSymbol, SYMBOL_TERMINAL_GRID, SymbolDocument, SymbolPin, SymbolShape,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Pill, PillState, crumb_text, docbar};

const BODY_GRID: i32 = SYMBOL_TERMINAL_GRID / 4;
const PIN_HIT_RADIUS: f32 = 10.0;
const RAIL_WIDTH: f32 = 280.0;
const DOT_RADIUS: i32 = 3;

#[derive(Clone, Copy)]
struct SymbolViewport {
    rect: Rect,
    zoom: f32,
    pan: Vec2,
}

impl SymbolViewport {
    fn world_to_screen(self, point: Point) -> Pos2 {
        self.rect.center() + self.pan + vec2(point.x as f32 * self.zoom, point.y as f32 * self.zoom)
    }

    fn screen_to_world(self, pos: Pos2) -> Point {
        let rel = pos - self.rect.center() - self.pan;
        Point::new(
            (rel.x / self.zoom).round() as i32,
            (rel.y / self.zoom).round() as i32,
        )
    }
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let ports = state.active_symbol_ports();
    let mut document = state.load_active_symbol_document().unwrap_or_else(|error| {
        state.push_user_message(ConsoleMessage::warning(error));
        SymbolDocument::default()
    });
    document.reconcile_ports(&ports);

    let mut generate_requested = false;
    docbar(ui, |ui| {
        breadcrumb(ui, state);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if pin_pill(ui, &document, &ports, state.active_view_read_only()) {
                state.shell.symbol.tool = SymbolTool::PlacePin;
                if let Some(pin) = next_unplaced_pin(&document) {
                    state.shell.symbol.select_pin(pin);
                } else {
                    state.shell.symbol.clear_selection();
                }
            }
            tool_echo(ui, state);
        });
    });

    if state.active_view_read_only() {
        read_only_banner(ui, state);
    }

    if document.body.is_empty() && !ports.is_empty() {
        generate_requested |= empty_generate_state(ui, state, &ports);
        if generate_requested {
            if state.deny_read_only_edit() {
                return;
            }
            if let Err(error) = state.generate_active_symbol_document() {
                state.push_user_message(ConsoleMessage::warning(error));
            }
            document = state
                .load_active_symbol_document()
                .unwrap_or_else(|_| SymbolDocument::generated_from_ports(&ports));
        }
    }

    let height = ui.available_height();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let show_rail = ui.available_width() > 760.0;
        let rail_width = if show_rail { RAIL_WIDTH } else { 0.0 };
        let canvas_size = vec2((ui.available_width() - rail_width).max(180.0), height);
        let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::click_and_drag());
        let mut changed = false;
        let viewport = update_viewport(ui, state, rect, &document, &response);
        changed |= handle_symbol_keys(ui, state, &mut document);
        changed |= handle_canvas_interaction(state, &mut document, viewport, &response);
        draw_canvas(ui, viewport, &document, &ports, state);
        if changed {
            if let Err(error) = state.store_active_symbol_document(&document) {
                state.push_user_message(ConsoleMessage::warning(error));
            }
        }
        if show_rail {
            pins_rail(ui, state, &mut document, &ports, height);
        }
    });
}

fn breadcrumb(ui: &mut Ui, state: &AppState) {
    let reference = &state.workspace.active_view;
    crumb_text(
        ui,
        &[
            (reference.library.as_str(), false),
            (reference.cell.as_str(), true),
            (reference.view.as_str(), false),
        ],
    );
}

fn tool_echo(ui: &mut Ui, state: &AppState) {
    ui.add_space(10.0);
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(state.shell.symbol.tool.label())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

fn pin_pill(ui: &mut Ui, document: &SymbolDocument, ports: &[PortSpec], read_only: bool) -> bool {
    let summary = document.pin_summary(ports);
    let (state, label) = match summary {
        PinSummary::Match => (PillState::Ok, "PINS match schematic".to_owned()),
        PinSummary::Unplaced(count) => (PillState::Error, format!("PINS {count} unplaced")),
        PinSummary::Orphaned(count) => (PillState::Error, format!("PINS {count} orphaned")),
        PinSummary::NoSchematic => (PillState::Idle, "PINS no schematic".to_owned()),
    };
    Pill::new(state, &label).show(ui);
    if !matches!(summary, PinSummary::Unplaced(_)) {
        return false;
    }
    ui.add_space(6.0);
    Button::new("Place new pins")
        .enabled(!read_only)
        .show(ui)
        .clicked()
}

fn read_only_banner(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 24.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, 0.0, c.warn.gamma_multiply(0.13));
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, c.border),
            );
            ui.add_space(12.0);
            let library = state.workspace.active_view.library.clone();
            ui.label(
                egui::RichText::new(state.read_only_master_message())
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.warn),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if Button::new("Copy to editable library...")
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    let cell = state.workspace.active_view.cell.clone();
                    state.open_copy_cell_dialog(&library, &cell);
                }
            });
        },
    );
}

fn empty_generate_state(ui: &mut Ui, state: &mut AppState, ports: &[PortSpec]) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut requested = false;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 96.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.add_space(14.0);
            ui.label(
                egui::RichText::new("No symbol drawn yet")
                    .font(theme::sans(tokens::FS_3, FontWeight::Medium))
                    .color(c.text),
            );
            ui.label(
                egui::RichText::new(format!(
                    "Generate from schematic builds a box body with all {} ports placed.",
                    ports.len()
                ))
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text_faint),
            );
            ui.add_space(8.0);
            if Button::new("Generate from schematic")
                .enabled(!state.active_view_read_only())
                .show(ui)
                .clicked()
            {
                requested = true;
            }
        },
    );
    requested
}

fn update_viewport(
    ui: &mut Ui,
    state: &mut AppState,
    rect: Rect,
    document: &SymbolDocument,
    response: &egui::Response,
) -> SymbolViewport {
    if state.shell.symbol.needs_fit {
        fit_symbol_view(state, rect, document);
        state.shell.symbol.needs_fit = false;
    }
    if response.hovered() {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll.abs() > f32::EPSILON {
            let factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
            state.shell.symbol.zoom = (state.shell.symbol.zoom * factor).clamp(1.0, 18.0);
        }
    }
    if response.dragged_by(egui::PointerButton::Middle) {
        let delta = ui.input(|input| input.pointer.delta());
        state.shell.symbol.pan.0 += delta.x;
        state.shell.symbol.pan.1 += delta.y;
    }
    SymbolViewport {
        rect,
        zoom: state.shell.symbol.zoom,
        pan: vec2(state.shell.symbol.pan.0, state.shell.symbol.pan.1),
    }
}

fn fit_symbol_view(state: &mut AppState, rect: Rect, document: &SymbolDocument) {
    let (min, max) = document_bounds(document);
    let width = (max.x - min.x).abs().max(80) as f32;
    let height = (max.y - min.y).abs().max(80) as f32;
    let zoom = ((rect.width() - 96.0).max(80.0) / width)
        .min((rect.height() - 96.0).max(80.0) / height)
        .clamp(1.0, 8.0);
    let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
    state.shell.symbol.zoom = zoom;
    state.shell.symbol.pan = (-(center.x as f32) * zoom, -(center.y as f32) * zoom);
}

fn handle_symbol_keys(ui: &mut Ui, state: &mut AppState, document: &mut SymbolDocument) -> bool {
    if ui.ctx().wants_keyboard_input() {
        return false;
    }
    let mut changed = false;
    ui.input(|input| {
        let plain = plain_symbol_tool_hotkey(input.modifiers);
        if plain && input.key_pressed(egui::Key::S) {
            state.shell.symbol.tool = SymbolTool::Select;
        }
        if plain && input.key_pressed(egui::Key::P) {
            state.shell.symbol.tool = SymbolTool::PlacePin;
            if let Some(pin) = next_unplaced_pin(document) {
                state.shell.symbol.select_pin(pin);
            } else {
                state.shell.symbol.clear_selection();
            }
        }
        if plain && input.key_pressed(egui::Key::W) {
            state.shell.symbol.tool = SymbolTool::Polyline;
            state.shell.symbol.pending_polyline.clear();
        }
        if plain && input.key_pressed(egui::Key::C) {
            state.shell.symbol.tool = SymbolTool::Circle;
            state.shell.symbol.shape_start = None;
        }
        if plain && input.key_pressed(egui::Key::A) {
            state.shell.symbol.tool = SymbolTool::Arc;
            state.shell.symbol.shape_start = None;
        }
        if plain && input.key_pressed(egui::Key::D) {
            state.shell.symbol.tool = SymbolTool::Arrow;
        }
        if plain && input.key_pressed(egui::Key::O) {
            state.shell.symbol.tool = SymbolTool::Dot;
        }
        if input.key_pressed(egui::Key::Escape) {
            if state.shell.symbol.pending_polyline.len() >= 2 {
                changed |= finish_pending_polyline(state, document);
            } else {
                state.shell.symbol.pending_polyline.clear();
            }
            state.shell.symbol.tool = SymbolTool::Select;
            state.shell.symbol.shape_start = None;
            state.shell.symbol.clear_drag_state();
            state.shell.symbol.marquee_start = None;
            state.shell.symbol.marquee_current = None;
        }
        if plain && input.key_pressed(egui::Key::F) {
            state.shell.symbol.needs_fit = true;
        }
    });
    changed
}

fn plain_symbol_tool_hotkey(modifiers: egui::Modifiers) -> bool {
    !modifiers.alt && !modifiers.ctrl && !modifiers.command && !modifiers.shift
}

fn handle_canvas_interaction(
    state: &mut AppState,
    document: &mut SymbolDocument,
    viewport: SymbolViewport,
    response: &egui::Response,
) -> bool {
    if let Some(pos) = response.hover_pos() {
        let world = viewport.screen_to_world(pos);
        state.shell.canvas_hover = Some((world.x as f64, world.y as f64));
        state.shell.canvas_view_center = Some((
            viewport.screen_to_world(viewport.rect.center()).x as f64,
            viewport.screen_to_world(viewport.rect.center()).y as f64,
        ));
    }

    let Some(pointer) = response.interact_pointer_pos() else {
        return false;
    };
    let terminal_point = snap_point(viewport.screen_to_world(pointer), SYMBOL_TERMINAL_GRID);
    let body_point = snap_point(viewport.screen_to_world(pointer), BODY_GRID);

    if response.secondary_clicked()
        && matches!(state.shell.symbol.tool, SymbolTool::Polyline)
        && finish_pending_polyline(state, document)
    {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(pin) = hit_pin(document, viewport, pointer)
    {
        state.shell.symbol.select_pin(pin.clone());
        state.shell.symbol.dragging_pin = Some(pin);
        if !state.active_view_read_only() {
            state.record_symbol_edit(document);
        }
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.shell.symbol.dragging_pin.is_none()
        && let Some(label) = hit_label(document, viewport, pointer)
    {
        state.shell.symbol.dragging_label = Some(label);
        if !state.active_view_read_only() {
            state.record_symbol_edit(document);
        }
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.shell.symbol.dragging_pin.is_none()
        && state.shell.symbol.dragging_label.is_none()
        && hit_origin(document, viewport, pointer)
    {
        state.shell.symbol.clear_selection();
        state.shell.symbol.dragging_origin = true;
        if !state.active_view_read_only() {
            state.record_symbol_edit(document);
        }
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.shell.symbol.dragging_pin.is_none()
        && state.shell.symbol.dragging_label.is_none()
        && !state.shell.symbol.dragging_origin
        && let Some(shape_index) = hit_shape(document, viewport, pointer)
    {
        state.shell.symbol.select_shape(shape_index);
        state.shell.symbol.dragging_shape = Some((shape_index, body_point));
        if !state.active_view_read_only() {
            state.record_symbol_edit(document);
        }
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && matches!(state.shell.symbol.tool, SymbolTool::Select)
        && state.shell.symbol.dragging_pin.is_none()
        && state.shell.symbol.dragging_label.is_none()
        && !state.shell.symbol.dragging_origin
        && state.shell.symbol.dragging_shape.is_none()
    {
        state.shell.symbol.marquee_start = Some(body_point);
        state.shell.symbol.marquee_current = Some(body_point);
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(name) = state.shell.symbol.dragging_pin.clone()
    {
        if state.deny_read_only_edit() {
            state.shell.symbol.dragging_pin = None;
            return false;
        }
        if let Some(pin) = document.pin_mut(&name) {
            pin.position = Some(terminal_point);
            return true;
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(label) = state.shell.symbol.dragging_label.clone()
    {
        if state.deny_read_only_edit() {
            state.shell.symbol.dragging_label = None;
            return false;
        }
        if label == "name" {
            document.name_anchor = body_point;
        } else {
            document.value_anchor = body_point;
        }
        return true;
    }
    if response.dragged_by(egui::PointerButton::Primary) && state.shell.symbol.dragging_origin {
        if state.deny_read_only_edit() {
            state.shell.symbol.dragging_origin = false;
            return false;
        }
        document.origin = body_point;
        return true;
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some((shape_index, last_point)) = state.shell.symbol.dragging_shape
    {
        if state.deny_read_only_edit() {
            state.shell.symbol.dragging_shape = None;
            return false;
        }
        let delta = body_point - last_point;
        if delta != Point::origin() {
            if let Some(shape) = document.body.get_mut(shape_index) {
                shape.translate(delta);
                state.shell.symbol.dragging_shape = Some((shape_index, body_point));
                return true;
            }
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && state.shell.symbol.marquee_start.is_some()
    {
        state.shell.symbol.marquee_current = Some(body_point);
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) {
        if let Some(start) = state.shell.symbol.marquee_start.take() {
            let end = state
                .shell
                .symbol
                .marquee_current
                .take()
                .unwrap_or(body_point);
            state
                .shell
                .symbol
                .set_selection(SymbolSelection::in_rect(document, start, end));
        }
        state.shell.symbol.dragging_pin = None;
        state.shell.symbol.dragging_shape = None;
        state.shell.symbol.dragging_label = None;
        state.shell.symbol.dragging_origin = false;
    }

    if !response.clicked_by(egui::PointerButton::Primary) {
        return false;
    }

    match state.shell.symbol.tool {
        SymbolTool::Select => {
            if let Some(pin) = hit_pin(document, viewport, pointer) {
                state.shell.symbol.select_pin(pin);
            } else if let Some(shape) = hit_shape(document, viewport, pointer) {
                state.shell.symbol.select_shape(shape);
            } else {
                state.shell.symbol.clear_selection();
            }
            false
        }
        SymbolTool::PlacePin => place_selected_pin(state, document, terminal_point),
        SymbolTool::Polyline => add_polyline_point(state, body_point),
        SymbolTool::Circle => add_round_shape(state, document, body_point, false),
        SymbolTool::Arc => add_round_shape(state, document, body_point, true),
        SymbolTool::Arrow => add_simple_shape(
            state,
            document,
            SymbolShape::Arrow {
                tip: body_point,
                rotation_quarters: 0,
            },
        ),
        SymbolTool::Dot => add_simple_shape(
            state,
            document,
            SymbolShape::Dot {
                center: body_point,
                radius: DOT_RADIUS,
            },
        ),
    }
}

fn place_selected_pin(state: &mut AppState, document: &mut SymbolDocument, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    let selected = state
        .shell
        .symbol
        .selected_pin
        .clone()
        .or_else(|| next_unplaced_pin(document));
    let Some(name) = selected else {
        return false;
    };
    if let Some(pin) = document.pin_mut(&name) {
        pin.position = Some(point);
        state.shell.symbol.select_pin(name);
        state.shell.symbol.tool = SymbolTool::Select;
        return true;
    }
    false
}

fn add_polyline_point(state: &mut AppState, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.shell.symbol.pending_polyline.push(point);
    false
}

fn finish_pending_polyline(state: &mut AppState, document: &mut SymbolDocument) -> bool {
    if state.shell.symbol.pending_polyline.len() < 2 {
        return false;
    }
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    let points = std::mem::take(&mut state.shell.symbol.pending_polyline);
    document.body.push(SymbolShape::Polyline {
        points,
        closed: false,
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        state.shell.symbol.select_shape(index);
    }
    state.shell.symbol.tool = SymbolTool::Select;
    true
}

fn add_round_shape(
    state: &mut AppState,
    document: &mut SymbolDocument,
    point: Point,
    arc: bool,
) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    if let Some(center) = state.shell.symbol.shape_start.take() {
        state.record_symbol_edit(document);
        let radius = center
            .distance_squared(point)
            .isqrt()
            .max(SYMBOL_TERMINAL_GRID);
        let shape = if arc {
            SymbolShape::Arc {
                center,
                radius,
                start_degrees: 0,
                sweep_degrees: 180,
            }
        } else {
            SymbolShape::Circle { center, radius }
        };
        document.body.push(shape);
        if let Some(index) = document.body.len().checked_sub(1) {
            state.shell.symbol.select_shape(index);
        }
        state.shell.symbol.tool = SymbolTool::Select;
        true
    } else {
        state.shell.symbol.shape_start = Some(point);
        false
    }
}

fn add_simple_shape(
    state: &mut AppState,
    document: &mut SymbolDocument,
    shape: SymbolShape,
) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    document.body.push(shape);
    if let Some(index) = document.body.len().checked_sub(1) {
        state.shell.symbol.select_shape(index);
    }
    state.shell.symbol.tool = SymbolTool::Select;
    true
}

fn draw_canvas(
    ui: &mut Ui,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    ports: &[PortSpec],
    state: &AppState,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter_at(viewport.rect);
    painter.rect_filled(viewport.rect, 0.0, c.canvas_bg);
    draw_grid(&painter, viewport, c.canvas_grid);
    draw_body(
        &painter,
        viewport,
        document,
        c.symbol,
        &state.shell.symbol.effective_selection().shapes,
        c.accent,
    );
    draw_bbox_and_origin(&painter, viewport, document, &t);
    draw_pins(&painter, viewport, document, ports, state);
    draw_labels(&painter, viewport, document, &t);
    draw_marquee(&painter, viewport, state, &t);
    draw_preview_tile(ui, viewport.rect, document, ports, state, &t);
}

fn draw_grid(painter: &egui::Painter, viewport: SymbolViewport, color: Color32) {
    let step = SYMBOL_TERMINAL_GRID as f32 * viewport.zoom;
    if step < 8.0 {
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
                if *closed && points.len() > 2 {
                    painter.line_segment(
                        [
                            viewport.world_to_screen(*points.last().expect("last point")),
                            viewport.world_to_screen(points[0]),
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
        }
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
    let rect = Rect::from_min_max(min, max);
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
    state: &AppState,
) {
    let t = Tokens::get(painter.ctx());
    let selection = state.shell.symbol.effective_selection();
    let port_names: std::collections::HashSet<String> = ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect();
    for pin in &document.pins {
        let Some(position) = pin.position else {
            continue;
        };
        let start = viewport.world_to_screen(position);
        let inner = viewport.world_to_screen(stub_inner(position));
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
        painter.rect_stroke(pad, 0.0, pad_stroke);
        let label_pos = pin_label_pos(position, viewport);
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            &pin.name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.symbol,
        );
        draw_direction_mark(painter, viewport, pin, color);
    }
}

fn draw_marquee(painter: &egui::Painter, viewport: SymbolViewport, state: &AppState, t: &Tokens) {
    let (Some(start), Some(current)) = (
        state.shell.symbol.marquee_start,
        state.shell.symbol.marquee_current,
    ) else {
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
    document: &SymbolDocument,
    t: &Tokens,
) {
    painter.text(
        viewport.world_to_screen(document.name_anchor),
        Align2::LEFT_CENTER,
        "@name",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.net_label,
    );
    painter.text(
        viewport.world_to_screen(document.value_anchor),
        Align2::LEFT_CENTER,
        "@value",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.net_label,
    );
}

fn draw_preview_tile(
    ui: &mut Ui,
    canvas: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    state: &AppState,
    t: &Tokens,
) {
    let rect = Rect::from_min_size(
        canvas.right_bottom() - vec2(180.0, 138.0),
        vec2(168.0, 126.0),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, t.radius_lg, t.color.bg_panel);
    painter.rect_stroke(rect, t.radius_lg, Stroke::new(1.0, t.color.border_strong));
    let header = Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + 24.0));
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
    let viewport = SymbolViewport {
        rect: Rect::from_min_max(pos2(rect.left(), rect.top() + 24.0), rect.max),
        zoom: 1.8,
        pan: Vec2::ZERO,
    };
    let mut binding = LibraryCellInstance::new(
        &state.workspace.active_view.library,
        &state.workspace.active_view.cell,
        "schematic",
    );
    binding.bind_interface(ports);
    let mut component =
        Component::new(0, ComponentType::CellInstance, Point::origin()).with_library_cell(binding);
    component.name = "X1".to_owned();
    component.value = state.workspace.active_view.cell.clone();
    let resolved = ResolvedCellSymbol::from_authored_document(document.clone(), ports);
    draw_resolved_symbol(
        &painter,
        viewport.world_to_screen(Point::origin()),
        viewport.zoom,
        &component,
        &resolved,
        Stroke::new(1.1, t.color.symbol),
    );
}

fn pins_rail(
    ui: &mut Ui,
    state: &mut AppState,
    document: &mut SymbolDocument,
    ports: &[PortSpec],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.allocate_ui_with_layout(vec2(RAIL_WIDTH, height), egui::Layout::top_down(egui::Align::Min), |ui| {
        let rect = ui.max_rect();
        ui.painter().rect_filled(rect, 0.0, c.bg_panel);
        ui.painter().vline(rect.left() + 0.5, rect.y_range(), Stroke::new(1.0, c.border));
        rail_header(ui, "PINS - SCHEMATIC CONTRACT");
        let mut changed = false;
        let listed: Vec<SymbolPin> = if ports.is_empty() {
            document.pins.clone()
        } else {
            ports
                .iter()
                .map(|port| {
                    document
                        .pin(&port.name)
                        .cloned()
                        .unwrap_or_else(|| SymbolPin::new(&port.name, port.direction, None))
                })
                .collect()
        };
        for pin in listed {
            let selected = state.shell.symbol.selected_pin.as_deref() == Some(pin.name.as_str());
            let response = pin_row(ui, &pin, selected);
            if response.clicked() {
                state.shell.symbol.select_pin(pin.name.clone());
                state.shell.symbol.tool = SymbolTool::PlacePin;
            }
        }
        let orphaned: Vec<String> = document
            .pins
            .iter()
            .filter(|pin| {
                !ports.is_empty()
                    && !ports
                        .iter()
                        .any(|port| port.name.eq_ignore_ascii_case(&pin.name))
            })
            .map(|pin| pin.name.clone())
            .collect();
        if !orphaned.is_empty() {
            rail_header(ui, "ORPHANED");
            for name in orphaned {
                let label = format!("{name} - delete");
                if Button::new(&label)
                    .ghost()
                    .enabled(!state.active_view_read_only())
                    .show(ui)
                    .clicked()
                {
                    if state.deny_read_only_edit() {
                        continue;
                    }
                    state.record_symbol_edit(document);
                    document.pins.retain(|pin| pin.name != name);
                    changed = true;
                }
            }
        }
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "Rows come from the schematic ports; the symbol places pins, it does not invent the contract.",
                )
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text_faint),
            );
        });
        if changed && let Err(error) = state.store_active_symbol_document(document) {
            state.push_user_message(ConsoleMessage::warning(error));
        }
    });
}

fn rail_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 30.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
        },
    );
}

fn pin_row(ui: &mut Ui, pin: &SymbolPin, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), t.metrics.row_h), Sense::click());
    if response.hovered() || selected {
        ui.painter()
            .rect_filled(rect, 0.0, if selected { c.accent_dim } else { c.bg_hover });
    }
    let state = if pin.position.is_some() {
        if pin.terminal_on_grid() {
            "placed"
        } else {
            "off-grid"
        }
    } else {
        "unplaced"
    };
    let state_color = match state {
        "placed" => c.ok,
        "off-grid" => c.err,
        _ => c.err,
    };
    let y = rect.center().y;
    ui.painter().text(
        pos2(rect.left() + 12.0, y),
        Align2::LEFT_CENTER,
        &pin.name,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        c.text,
    );
    ui.painter().text(
        pos2(rect.right() - 128.0, y),
        Align2::LEFT_CENTER,
        pin.direction.keyword(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    ui.painter().text(
        pos2(rect.right() - 72.0, y),
        Align2::LEFT_CENTER,
        state,
        theme::mono(10.0, FontWeight::Regular),
        state_color,
    );
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn hit_pin(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<String> {
    document
        .pins
        .iter()
        .filter_map(|pin| Some((pin.name.clone(), viewport.world_to_screen(pin.position?))))
        .find(|(_, pin_pos)| pin_pos.distance(pos) <= PIN_HIT_RADIUS)
        .map(|(name, _)| name)
}

fn hit_label(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<String> {
    let name = viewport.world_to_screen(document.name_anchor);
    if name.distance(pos) <= 18.0 {
        return Some("name".to_owned());
    }
    let value = viewport.world_to_screen(document.value_anchor);
    if value.distance(pos) <= 18.0 {
        return Some("value".to_owned());
    }
    None
}

fn hit_origin(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> bool {
    viewport.world_to_screen(document.origin).distance(pos) <= 10.0
}

fn hit_shape(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<usize> {
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
                && distance_to_screen_segment(
                    pos,
                    viewport.world_to_screen(*points.last().expect("last point")),
                    viewport.world_to_screen(points[0]),
                ) <= HIT_PX)
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

fn next_unplaced_pin(document: &SymbolDocument) -> Option<String> {
    document
        .pins
        .iter()
        .find(|pin| pin.position.is_none())
        .map(|pin| pin.name.clone())
}

fn snap_point(point: Point, grid: i32) -> Point {
    let snap = |v: i32| ((v as f32 / grid as f32).round() as i32) * grid;
    Point::new(snap(point.x), snap(point.y))
}

fn stub_inner(position: Point) -> Point {
    if position.x.abs() >= position.y.abs() {
        Point::new(
            position.x - SYMBOL_TERMINAL_GRID * position.x.signum(),
            position.y,
        )
    } else {
        Point::new(
            position.x,
            position.y - SYMBOL_TERMINAL_GRID * position.y.signum(),
        )
    }
}

fn pin_label_pos(position: Point, viewport: SymbolViewport) -> Pos2 {
    let pad = SYMBOL_TERMINAL_GRID as f32 * viewport.zoom * 0.75;
    let base = viewport.world_to_screen(position);
    if position.x < 0 {
        base + vec2(-pad, -8.0)
    } else if position.x > 0 {
        base + vec2(pad, -8.0)
    } else if position.y < 0 {
        base + vec2(26.0, -pad * 0.4)
    } else {
        base + vec2(26.0, pad * 0.4)
    }
}

fn draw_direction_mark(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    pin: &SymbolPin,
    color: Color32,
) {
    if !matches!(pin.direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let Some(position) = pin.position else {
        return;
    };
    let inner = viewport.world_to_screen(stub_inner(position));
    let outward = viewport.world_to_screen(position);
    let dir = (outward - inner).normalized();
    let tip = if pin.direction == PortDirection::In {
        inner + dir * 5.0
    } else {
        outward - dir * 5.0
    };
    let side = vec2(-dir.y, dir.x);
    painter.add(Shape::convex_polygon(
        vec![
            tip,
            tip - dir * 7.0 + side * 3.5,
            tip - dir * 7.0 - side * 3.5,
        ],
        color,
        Stroke::NONE,
    ));
}

fn draw_arrow(
    painter: &egui::Painter,
    viewport: SymbolViewport,
    tip: Point,
    rotation_quarters: i32,
    color: Color32,
) {
    let tip = viewport.world_to_screen(tip);
    let angle = rotation_quarters.rem_euclid(4) as f32 * std::f32::consts::FRAC_PI_2;
    let dir = vec2(angle.cos(), angle.sin());
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
        }
    }
    let min_x = xs.iter().min().copied().unwrap_or(-40) - 20;
    let max_x = xs.iter().max().copied().unwrap_or(40) + 20;
    let min_y = ys.iter().min().copied().unwrap_or(-40) - 20;
    let max_y = ys.iter().max().copied().unwrap_or(40) + 20;
    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modified_shortcuts_do_not_switch_symbol_tools() {
        assert!(plain_symbol_tool_hotkey(egui::Modifiers::NONE));

        for modifiers in [
            egui::Modifiers {
                alt: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                ctrl: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                command: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                shift: true,
                ..egui::Modifiers::NONE
            },
        ] {
            assert!(
                !plain_symbol_tool_hotkey(modifiers),
                "modified key input must stay available for global shortcuts and text entry"
            );
        }
    }

    #[test]
    fn finishing_pending_polyline_creates_one_selected_shape() {
        let mut state = AppState::default();
        let mut document = SymbolDocument::default();
        state.shell.symbol.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

        assert!(finish_pending_polyline(&mut state, &mut document));

        assert_eq!(document.body.len(), 1);
        assert_eq!(state.shell.symbol.selected_shape, Some(0));
        assert!(state.shell.symbol.selection.shapes.contains(&0));
        assert!(state.shell.symbol.pending_polyline.is_empty());
    }
}
