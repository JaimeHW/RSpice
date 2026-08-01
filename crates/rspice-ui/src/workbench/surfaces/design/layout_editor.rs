//! Governed physical-layout cellview editor.
//!
//! The persisted authority remains [`PhysicalLayoutDocument`]. This module
//! owns only runtime camera/tool state and submits every mutation through the
//! revision-checked application transaction boundary.

use std::collections::BTreeMap;

use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::pdk_config::{PdkDisplayFillStyle, PdkDisplayLayerStyle};
use crate::state::{
    CellViewRef, LayoutEdit, LayoutGeometry, LayoutLayerPurpose, LayoutObjectId, LayoutPoint,
    LayoutShape, PhysicalLayoutDocument,
};
use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;

const DEFAULT_ZOOM_PX_PER_DBU: f32 = 0.05;
const MIN_ZOOM_PX_PER_DBU: f32 = 1.0e-7;
const MAX_ZOOM_PX_PER_DBU: f32 = 1.0e3;
const FIT_MARGIN_PX: f32 = 48.0;
const MAX_RENDERED_GRID_LINES: usize = 512;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LayoutTool {
    #[default]
    Select,
    Rectangle,
}

impl LayoutTool {
    const fn label(self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Rectangle => "Rectangle",
        }
    }
}

#[derive(Debug, Clone)]
struct LayoutEditorSession {
    zoom: f32,
    pan_px: Vec2,
    tool: LayoutTool,
    active_layer: Option<LayoutLayerPurpose>,
    selected: Option<LayoutObjectId>,
    rectangle_start: Option<LayoutPoint>,
    fit_requested: bool,
    status: Option<(bool, String)>,
}

impl Default for LayoutEditorSession {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_ZOOM_PX_PER_DBU,
            pan_px: Vec2::ZERO,
            tool: LayoutTool::Select,
            active_layer: None,
            selected: None,
            rectangle_start: None,
            fit_requested: false,
            status: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ResolvedDisplayContract {
    profile_label: String,
    selection: Color32,
    styles: BTreeMap<(String, String), PdkDisplayLayerStyle>,
    ordered_layers: Vec<LayoutLayerPurpose>,
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let owner = app.state.workspace.active_view.clone();
    let session_id = ui.id().with(("physical-layout-editor", owner.key()));
    let mut session = ui.ctx().data_mut(|data| {
        data.get_temp::<LayoutEditorSession>(session_id)
            .unwrap_or_default()
    });

    let Some(document) = app
        .state
        .workspace
        .physical_layout_document(&owner)
        .cloned()
    else {
        show_missing_document(ui, app, &owner);
        return;
    };
    let display = match resolve_display_contract(app, &document) {
        Ok(display) => display,
        Err(error) => {
            show_fail_closed(ui, &error);
            return;
        }
    };
    if session
        .active_layer
        .as_ref()
        .is_none_or(|active| !display.ordered_layers.contains(active))
    {
        session.active_layer = display.ordered_layers.first().cloned();
    }

    let mut pending_edits = None;
    show_toolbar(
        ui,
        app,
        &document,
        &display,
        &mut session,
        &mut pending_edits,
    );
    if pending_edits.is_none() {
        pending_edits = show_canvas(ui, &document, &display, &mut session);
    }

    if let Some(edits) = pending_edits {
        match app
            .state
            .apply_physical_layout_transaction(&owner, document.revision(), &edits)
        {
            Ok(revision) => {
                session.status = Some((
                    true,
                    format!("Committed physical-layout revision {}", revision.get()),
                ));
                if edits.iter().any(|edit| {
                    matches!(
                        edit,
                        LayoutEdit::RemoveShape { .. }
                            | LayoutEdit::RemoveInstance { .. }
                            | LayoutEdit::RemoveText { .. }
                            | LayoutEdit::RemoveTerminal { .. }
                    )
                }) {
                    session.selected = None;
                }
            }
            Err(error) => {
                session.status = Some((false, error.clone()));
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
    }

    ui.ctx()
        .data_mut(|data| data.insert_temp(session_id, session));
}

fn show_missing_document(ui: &mut Ui, app: &mut RSpiceApp, owner: &CellViewRef) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::same(24))
        .show(ui, |ui| {
            ui.heading("Layout authority is not initialized");
            ui.label(format!(
                "{} is a Layout view but has no authoritative physical document.",
                owner.display_path()
            ));
            let enabled = !app.state.active_view_read_only();
            let initialize =
                ui.add_enabled(enabled, egui::Button::new("Initialize from project PDK"));
            if initialize.clicked() {
                match app.state.initialize_physical_layout_document(owner.clone()) {
                    Ok(_) => app.state.push_user_message(ConsoleMessage::info(format!(
                        "Initialized {} from the exact signed project PDK.",
                        owner.display_path()
                    ))),
                    Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
                }
            }
            if !enabled {
                ui.colored_label(t.color.warn, "The active view is read-only.");
            }
        });
}

fn show_fail_closed(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.err.gamma_multiply(0.12))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.colored_label(t.color.err, "Physical layout is unavailable");
            ui.label(error);
        });
}

fn resolve_display_contract(
    app: &RSpiceApp,
    document: &PhysicalLayoutDocument,
) -> Result<ResolvedDisplayContract, String> {
    let package = app.state.exact_project_pdk_package()?;
    let technology = document.technology();
    if package.manifest_digest() != technology.manifest_digest()
        || package.archive_digest() != technology.archive_digest()
        || package.manifest().revision != technology.revision()
        || !package
            .manifest()
            .package_id
            .eq_ignore_ascii_case(technology.package_id())
    {
        return Err(format!(
            "{} is not bound to the project's exact validated signed PDK",
            document.owner().display_path()
        ));
    }
    let active = app
        .state
        .pdk_config
        .display_profile_registry
        .active_for_package(package);
    let (profile_label, selection_rgba, entries) = active.map_or_else(
        || {
            let defaults = crate::state::pdk_config::PdkDisplayProfileDraft::signed_defaults(
                package,
                "signed-default",
                "Signed package defaults",
            );
            (defaults.label, defaults.selection_rgba, defaults.entries)
        },
        |profile| {
            (
                profile.label.clone(),
                profile.selection_rgba,
                profile.entries.clone(),
            )
        },
    );
    let ordered_layers = entries
        .iter()
        .map(|style| LayoutLayerPurpose {
            layer: style.layer.clone(),
            purpose: style.purpose.clone(),
        })
        .collect::<Vec<_>>();
    let styles = entries
        .into_iter()
        .map(|style| {
            (
                (
                    style.layer.to_ascii_lowercase(),
                    style.purpose.to_ascii_lowercase(),
                ),
                style,
            )
        })
        .collect();
    Ok(ResolvedDisplayContract {
        profile_label,
        selection: rgba(selection_rgba),
        styles,
        ordered_layers,
    })
}

fn show_toolbar(
    ui: &mut Ui,
    app: &RSpiceApp,
    document: &PhysicalLayoutDocument,
    display: &ResolvedDisplayContract,
    session: &mut LayoutEditorSession,
    pending_edits: &mut Option<Vec<LayoutEdit>>,
) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tool in [LayoutTool::Select, LayoutTool::Rectangle] {
                    if ui
                        .selectable_label(session.tool == tool, tool.label())
                        .clicked()
                    {
                        session.tool = tool;
                        session.rectangle_start = None;
                    }
                }
                ui.separator();
                let active_label = session
                    .active_layer
                    .as_ref()
                    .map_or("No signed layer".to_owned(), |layer| {
                        format!("{}/{}", layer.layer, layer.purpose)
                    });
                let layer_picker = egui::ComboBox::from_id_salt("physical-layout-active-layer")
                    .selected_text(active_label)
                    .show_ui(ui, |ui| {
                        for layer in &display.ordered_layers {
                            let label = format!("{}/{}", layer.layer, layer.purpose);
                            ui.selectable_value(
                                &mut session.active_layer,
                                Some(layer.clone()),
                                label,
                            );
                        }
                    });
                ui.ctx()
                    .accesskit_node_builder(layer_picker.response.id, |node| {
                        node.set_label("Active physical layout layer and purpose");
                    });
                if ui.button("Fit").clicked() {
                    session.fit_requested = true;
                }
                let can_delete = session.selected.is_some() && !app.state.active_view_read_only();
                if ui
                    .add_enabled(can_delete, egui::Button::new("Delete selected"))
                    .clicked()
                    && let Some(id) = session.selected
                {
                    *pending_edits = removal_edit(document, id).map(|edit| vec![edit]);
                }
                ui.separator();
                ui.label(format!(
                    "rev {} · {} shapes · {} instances · {}",
                    document.revision().get(),
                    document.shapes().len(),
                    document.instances().len(),
                    display.profile_label
                ));
            });
            if let Some((ok, message)) = &session.status {
                ui.colored_label(if *ok { t.color.ok } else { t.color.err }, message);
            }
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label("Physical layout tools");
    });
}

fn show_canvas(
    ui: &mut Ui,
    document: &PhysicalLayoutDocument,
    display: &ResolvedDisplayContract,
    session: &mut LayoutEditorSession,
) -> Option<Vec<LayoutEdit>> {
    let t = Tokens::get(ui.ctx());
    let desired = ui.available_size().max(Vec2::new(1.0, 1.0));
    let (canvas, response) = ui.allocate_exact_size(desired, Sense::click_and_drag());
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("Physical layout canvas");
        node.set_description(
            "Interactive signed-technology layout canvas. Use Select or Rectangle from the toolbar.",
        );
    });
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            ui.is_enabled(),
            "Physical layout canvas",
        )
    });
    let painter = ui.painter_at(canvas);
    painter.rect_filled(canvas, 0.0, t.color.canvas_bg);

    if session.fit_requested {
        fit_document(document, canvas, session);
        session.fit_requested = false;
    }

    if response.hovered() {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll.abs() > f32::EPSILON {
            let pointer = ui
                .input(|input| input.pointer.hover_pos())
                .unwrap_or(canvas.center());
            let before = screen_to_world(pointer, canvas, session);
            session.zoom = (session.zoom * (scroll * 0.002).exp())
                .clamp(MIN_ZOOM_PX_PER_DBU, MAX_ZOOM_PX_PER_DBU);
            let after = world_to_screen(before, canvas, session);
            session.pan_px += pointer - after;
        }
    }
    if response.dragged_by(egui::PointerButton::Middle)
        || response.dragged_by(egui::PointerButton::Secondary)
    {
        session.pan_px += response.drag_delta();
    }

    paint_grid(&painter, canvas, session, t.color.border);
    for (id, shape) in document.shapes() {
        let Some(style) = style_for(display, &shape.layer_purpose) else {
            continue;
        };
        if !style.visible {
            continue;
        }
        paint_shape(
            &painter,
            canvas,
            session,
            shape,
            style,
            session.selected == Some(*id),
            display.selection,
        );
    }
    for (id, instance) in document.instances() {
        let origin = world_to_screen(instance.transform.origin, canvas, session);
        let selected = session.selected == Some(*id);
        let color = if selected {
            display.selection
        } else {
            t.color.text_dim
        };
        let radius = if selected { 8.0 } else { 6.0 };
        painter.rect_stroke(
            Rect::from_center_size(origin, Vec2::splat(radius * 2.0)),
            0.0,
            Stroke::new(if selected { 2.0 } else { 1.0 }, color),
            egui::StrokeKind::Inside,
        );
        painter.line_segment(
            [origin - Vec2::splat(radius), origin + Vec2::splat(radius)],
            Stroke::new(1.0, color),
        );
        painter.text(
            origin + Vec2::new(radius + 4.0, -radius),
            egui::Align2::LEFT_TOP,
            instance.master.display_path(),
            egui::FontId::monospace(11.0),
            color,
        );
    }
    for (id, text) in document.texts() {
        let Some(style) = style_for(display, &text.layer_purpose) else {
            continue;
        };
        if !style.visible {
            continue;
        }
        let color = if session.selected == Some(*id) {
            display.selection
        } else {
            rgba(style.screen_rgba)
        };
        painter.text(
            world_to_screen(text.origin, canvas, session),
            egui::Align2::LEFT_CENTER,
            &text.text,
            egui::FontId::monospace((text.height_dbu as f32 * session.zoom).clamp(8.0, 72.0)),
            color,
        );
    }

    let pointer = response.interact_pointer_pos();
    if session.tool == LayoutTool::Rectangle {
        if response.drag_started_by(egui::PointerButton::Primary) {
            session.rectangle_start =
                pointer.map(|position| screen_to_world(position, canvas, session));
        }
        if let (Some(start), Some(current)) = (session.rectangle_start, pointer) {
            let current = screen_to_world(current, canvas, session);
            let preview = Rect::from_two_pos(
                world_to_screen(start, canvas, session),
                world_to_screen(current, canvas, session),
            );
            painter.rect_stroke(
                preview,
                0.0,
                Stroke::new(1.5, display.selection),
                egui::StrokeKind::Inside,
            );
        }
        if response.drag_stopped_by(egui::PointerButton::Primary) {
            let start = session.rectangle_start.take();
            if let (Some(start), Some(end), Some(layer)) = (
                start,
                pointer.map(|position| screen_to_world(position, canvas, session)),
                session.active_layer.clone(),
            ) {
                if let Some(edit) = rectangle_edit(start, end, layer) {
                    return Some(vec![edit]);
                }
                session.status = Some((
                    false,
                    "Rectangle must have non-zero width and height".to_owned(),
                ));
            }
        }
    } else if response.clicked_by(egui::PointerButton::Primary) {
        session.selected = pointer.and_then(|position| {
            hit_test(
                document,
                display,
                screen_to_world(position, canvas, session),
                session.zoom,
            )
        });
    }

    // Painted last so the layout artwork does not cover the ring.
    crate::ui::theme::paint_focus_ring(ui, &response, canvas);
    None
}

fn style_for<'a>(
    display: &'a ResolvedDisplayContract,
    layer: &LayoutLayerPurpose,
) -> Option<&'a PdkDisplayLayerStyle> {
    display.styles.get(&(
        layer.layer.to_ascii_lowercase(),
        layer.purpose.to_ascii_lowercase(),
    ))
}

fn paint_shape(
    painter: &egui::Painter,
    canvas: Rect,
    session: &LayoutEditorSession,
    shape: &LayoutShape,
    style: &PdkDisplayLayerStyle,
    selected: bool,
    selection: Color32,
) {
    let base = rgba(style.screen_rgba);
    let outline = if selected { selection } else { base };
    let fill = match style.screen_fill {
        PdkDisplayFillStyle::Hollow => Color32::TRANSPARENT,
        PdkDisplayFillStyle::Solid => base,
        PdkDisplayFillStyle::Diagonal
        | PdkDisplayFillStyle::Crosshatch
        | PdkDisplayFillStyle::Dots => base.gamma_multiply(0.42),
    };
    let width = if selected {
        2.5
    } else {
        (f32::from(style.outline_width_milli_px) / 1_000.0).clamp(0.5, 8.0)
    };
    match &shape.geometry {
        LayoutGeometry::Rectangle {
            lower_left,
            upper_right,
        } => {
            let rect = Rect::from_two_pos(
                world_to_screen(*lower_left, canvas, session),
                world_to_screen(*upper_right, canvas, session),
            );
            painter.rect(
                rect,
                0.0,
                fill,
                Stroke::new(width, outline),
                egui::StrokeKind::Inside,
            );
        }
        LayoutGeometry::Polygon { vertices } => {
            let points = vertices
                .iter()
                .copied()
                .map(|point| world_to_screen(point, canvas, session))
                .collect();
            painter.add(egui::Shape::Path(egui::epaint::PathShape {
                points,
                closed: true,
                fill,
                stroke: Stroke::new(width, outline).into(),
            }));
        }
        LayoutGeometry::Path {
            centerline,
            width_dbu,
            ..
        } => {
            let points = centerline
                .iter()
                .copied()
                .map(|point| world_to_screen(point, canvas, session))
                .collect::<Vec<_>>();
            painter.add(egui::Shape::line(
                points,
                Stroke::new((*width_dbu as f32 * session.zoom).max(width), outline),
            ));
        }
    }
}

fn paint_grid(
    painter: &egui::Painter,
    canvas: Rect,
    session: &LayoutEditorSession,
    color: Color32,
) {
    let mut step_dbu = 1_i64;
    while step_dbu as f32 * session.zoom < 18.0 && step_dbu < 1_000_000_000_000 {
        step_dbu = step_dbu.saturating_mul(10);
    }
    let top_left = screen_to_world(canvas.left_top(), canvas, session);
    let bottom_right = screen_to_world(canvas.right_bottom(), canvas, session);
    let min_x = top_left.x.min(bottom_right.x);
    let max_x = top_left.x.max(bottom_right.x);
    let min_y = top_left.y.min(bottom_right.y);
    let max_y = top_left.y.max(bottom_right.y);
    let first_x = min_x.div_euclid(step_dbu) * step_dbu;
    let first_y = min_y.div_euclid(step_dbu) * step_dbu;
    for (index, x) in (first_x..=max_x)
        .step_by(usize::try_from(step_dbu).unwrap_or(usize::MAX))
        .take(MAX_RENDERED_GRID_LINES)
        .enumerate()
    {
        let screen_x = world_to_screen(LayoutPoint::new(x, 0), canvas, session).x;
        let emphasized = x == 0;
        painter.line_segment(
            [
                Pos2::new(screen_x, canvas.top()),
                Pos2::new(screen_x, canvas.bottom()),
            ],
            Stroke::new(
                if emphasized { 1.0 } else { 0.5 },
                color.gamma_multiply(if emphasized { 0.8 } else { 0.35 }),
            ),
        );
        if index + 1 == MAX_RENDERED_GRID_LINES {
            break;
        }
    }
    for (index, y) in (first_y..=max_y)
        .step_by(usize::try_from(step_dbu).unwrap_or(usize::MAX))
        .take(MAX_RENDERED_GRID_LINES)
        .enumerate()
    {
        let screen_y = world_to_screen(LayoutPoint::new(0, y), canvas, session).y;
        let emphasized = y == 0;
        painter.line_segment(
            [
                Pos2::new(canvas.left(), screen_y),
                Pos2::new(canvas.right(), screen_y),
            ],
            Stroke::new(
                if emphasized { 1.0 } else { 0.5 },
                color.gamma_multiply(if emphasized { 0.8 } else { 0.35 }),
            ),
        );
        if index + 1 == MAX_RENDERED_GRID_LINES {
            break;
        }
    }
}

fn rectangle_edit(
    first: LayoutPoint,
    second: LayoutPoint,
    layer_purpose: LayoutLayerPurpose,
) -> Option<LayoutEdit> {
    let lower_left = LayoutPoint::new(first.x.min(second.x), first.y.min(second.y));
    let upper_right = LayoutPoint::new(first.x.max(second.x), first.y.max(second.y));
    (lower_left.x < upper_right.x && lower_left.y < upper_right.y).then(|| {
        LayoutEdit::InsertShape {
            id: LayoutObjectId::new(),
            value: LayoutShape {
                layer_purpose,
                geometry: LayoutGeometry::Rectangle {
                    lower_left,
                    upper_right,
                },
                net: None,
                properties: BTreeMap::new(),
            },
        }
    })
}

fn removal_edit(document: &PhysicalLayoutDocument, id: LayoutObjectId) -> Option<LayoutEdit> {
    if document.shapes().contains_key(&id) {
        Some(LayoutEdit::RemoveShape { id })
    } else if document.instances().contains_key(&id) {
        Some(LayoutEdit::RemoveInstance { id })
    } else if document.texts().contains_key(&id) {
        Some(LayoutEdit::RemoveText { id })
    } else if document.terminals().contains_key(&id) {
        Some(LayoutEdit::RemoveTerminal { id })
    } else {
        None
    }
}

fn hit_test(
    document: &PhysicalLayoutDocument,
    display: &ResolvedDisplayContract,
    point: LayoutPoint,
    zoom: f32,
) -> Option<LayoutObjectId> {
    let tolerance_dbu = (6.0 / zoom.max(MIN_ZOOM_PX_PER_DBU)).ceil() as i64;
    for (id, text) in document.texts().iter().rev() {
        let Some(style) = style_for(display, &text.layer_purpose) else {
            continue;
        };
        if style.visible
            && style.selectable
            && (text.origin.x - point.x).abs() <= tolerance_dbu
            && (text.origin.y - point.y).abs() <= tolerance_dbu
        {
            return Some(*id);
        }
    }
    for (id, shape) in document.shapes().iter().rev() {
        let Some(style) = style_for(display, &shape.layer_purpose) else {
            continue;
        };
        if style.visible
            && style.selectable
            && geometry_contains(&shape.geometry, point, tolerance_dbu)
        {
            return Some(*id);
        }
    }
    document
        .instances()
        .iter()
        .rev()
        .find_map(|(id, instance)| {
            ((instance.transform.origin.x - point.x).abs() <= tolerance_dbu
                && (instance.transform.origin.y - point.y).abs() <= tolerance_dbu)
                .then_some(*id)
        })
}

fn geometry_contains(geometry: &LayoutGeometry, point: LayoutPoint, tolerance: i64) -> bool {
    match geometry {
        LayoutGeometry::Rectangle {
            lower_left,
            upper_right,
        } => {
            point.x >= lower_left.x.saturating_sub(tolerance)
                && point.x <= upper_right.x.saturating_add(tolerance)
                && point.y >= lower_left.y.saturating_sub(tolerance)
                && point.y <= upper_right.y.saturating_add(tolerance)
        }
        LayoutGeometry::Polygon { vertices } => {
            let mut inside = false;
            let mut previous = vertices[vertices.len() - 1];
            for &current in vertices {
                let crosses = (current.y > point.y) != (previous.y > point.y)
                    && (point.x as f64)
                        < (previous.x - current.x) as f64 * (point.y - current.y) as f64
                            / (previous.y - current.y) as f64
                            + current.x as f64;
                if crosses {
                    inside = !inside;
                }
                previous = current;
            }
            inside
        }
        LayoutGeometry::Path {
            centerline,
            width_dbu,
            ..
        } => {
            let radius = (*width_dbu as f64 * 0.5) + tolerance as f64;
            centerline.windows(2).any(|segment| {
                squared_distance_to_segment(point, segment[0], segment[1]) <= radius * radius
            })
        }
    }
}

fn squared_distance_to_segment(point: LayoutPoint, first: LayoutPoint, second: LayoutPoint) -> f64 {
    let dx = (second.x - first.x) as f64;
    let dy = (second.y - first.y) as f64;
    let length_squared = dx * dx + dy * dy;
    if length_squared <= f64::EPSILON {
        return ((point.x - first.x) as f64).powi(2) + ((point.y - first.y) as f64).powi(2);
    }
    let projection = (((point.x - first.x) as f64 * dx + (point.y - first.y) as f64 * dy)
        / length_squared)
        .clamp(0.0, 1.0);
    let nearest_x = first.x as f64 + projection * dx;
    let nearest_y = first.y as f64 + projection * dy;
    (point.x as f64 - nearest_x).powi(2) + (point.y as f64 - nearest_y).powi(2)
}

fn fit_document(
    document: &PhysicalLayoutDocument,
    canvas: Rect,
    session: &mut LayoutEditorSession,
) {
    let bounds = document
        .shapes()
        .values()
        .filter_map(|shape| geometry_bounds(&shape.geometry))
        .chain(
            document
                .texts()
                .values()
                .map(|text| (text.origin, text.origin)),
        )
        .chain(
            document
                .instances()
                .values()
                .map(|instance| (instance.transform.origin, instance.transform.origin)),
        )
        .reduce(|left, right| {
            (
                LayoutPoint::new(left.0.x.min(right.0.x), left.0.y.min(right.0.y)),
                LayoutPoint::new(left.1.x.max(right.1.x), left.1.y.max(right.1.y)),
            )
        });
    let Some((lower, upper)) = bounds else {
        session.zoom = DEFAULT_ZOOM_PX_PER_DBU;
        session.pan_px = Vec2::ZERO;
        return;
    };
    let width = (upper.x - lower.x).unsigned_abs().max(1) as f32;
    let height = (upper.y - lower.y).unsigned_abs().max(1) as f32;
    let available = (canvas.size() - Vec2::splat(FIT_MARGIN_PX * 2.0)).max(Vec2::splat(1.0));
    session.zoom = (available.x / width)
        .min(available.y / height)
        .clamp(MIN_ZOOM_PX_PER_DBU, MAX_ZOOM_PX_PER_DBU);
    let center = LayoutPoint::new(
        lower.x.saturating_add((upper.x - lower.x) / 2),
        lower.y.saturating_add((upper.y - lower.y) / 2),
    );
    session.pan_px = Vec2::new(
        -(center.x as f32) * session.zoom,
        (center.y as f32) * session.zoom,
    );
}

fn geometry_bounds(geometry: &LayoutGeometry) -> Option<(LayoutPoint, LayoutPoint)> {
    match geometry {
        LayoutGeometry::Rectangle {
            lower_left,
            upper_right,
        } => Some((*lower_left, *upper_right)),
        LayoutGeometry::Polygon { vertices } => point_bounds(vertices),
        LayoutGeometry::Path {
            centerline,
            width_dbu,
            begin_extension_dbu,
            end_extension_dbu,
        } => {
            let (mut lower, mut upper) = point_bounds(centerline)?;
            let expansion = i64::try_from(
                (width_dbu / 2)
                    .saturating_add(*begin_extension_dbu)
                    .saturating_add(*end_extension_dbu),
            )
            .unwrap_or(i64::MAX);
            lower.x = lower.x.saturating_sub(expansion);
            lower.y = lower.y.saturating_sub(expansion);
            upper.x = upper.x.saturating_add(expansion);
            upper.y = upper.y.saturating_add(expansion);
            Some((lower, upper))
        }
    }
}

fn point_bounds(points: &[LayoutPoint]) -> Option<(LayoutPoint, LayoutPoint)> {
    let first = *points.first()?;
    Some(
        points
            .iter()
            .copied()
            .skip(1)
            .fold((first, first), |(lower, upper), point| {
                (
                    LayoutPoint::new(lower.x.min(point.x), lower.y.min(point.y)),
                    LayoutPoint::new(upper.x.max(point.x), upper.y.max(point.y)),
                )
            }),
    )
}

fn world_to_screen(point: LayoutPoint, canvas: Rect, session: &LayoutEditorSession) -> Pos2 {
    canvas.center()
        + session.pan_px
        + Vec2::new(
            point.x as f32 * session.zoom,
            -(point.y as f32) * session.zoom,
        )
}

fn screen_to_world(position: Pos2, canvas: Rect, session: &LayoutEditorSession) -> LayoutPoint {
    let relative = position - canvas.center() - session.pan_px;
    LayoutPoint::new(
        clamp_layout_coordinate((relative.x / session.zoom).round()),
        clamp_layout_coordinate((-relative.y / session.zoom).round()),
    )
}

fn clamp_layout_coordinate(value: f32) -> i64 {
    const LIMIT: f32 = 1_000_000_000_000.0;
    value.clamp(-LIMIT, LIMIT) as i64
}

fn rgba(value: [u8; 4]) -> Color32 {
    Color32::from_rgba_unmultiplied(value[0], value[1], value[2], value[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_layout() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.provision_test_project_technology_contract();
        let owner = CellViewRef::new("user", "top", "layout");
        app.state
            .library_manager
            .get_library_mut("user")
            .expect("default project library")
            .get_or_create_cell("top")
            .add_view(crate::state::View::new(
                "layout",
                crate::state::ViewType::Layout,
            ));
        app.state
            .workspace
            .open_view(owner.clone(), crate::state::ViewType::Layout);
        app.state
            .initialize_physical_layout_document(owner)
            .expect("initialize governed physical layout");
        app
    }

    fn render(app: &mut RSpiceApp, size: Vec2) -> egui::FullOutput {
        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        context.enable_accesskit();
        context.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                ..Default::default()
            },
            |context| {
                egui::CentralPanel::default().show(context, |ui| show(ui, app));
            },
        )
    }

    #[test]
    fn screen_and_integral_dbu_transforms_round_trip() {
        let canvas = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(800.0, 600.0));
        let session = LayoutEditorSession {
            zoom: 0.25,
            pan_px: Vec2::new(17.0, -23.0),
            ..Default::default()
        };
        let point = LayoutPoint::new(1_234, -9_876);
        assert_eq!(
            screen_to_world(world_to_screen(point, canvas, &session), canvas, &session),
            point
        );
    }

    #[test]
    fn rectangle_tool_canonicalizes_drag_direction_and_rejects_zero_area() {
        let layer = LayoutLayerPurpose::try_new("metal1", "drawing").unwrap();
        let edit = rectangle_edit(LayoutPoint::new(40, -10), LayoutPoint::new(-20, 70), layer)
            .expect("non-zero rectangle");
        assert!(matches!(
            edit,
            LayoutEdit::InsertShape {
                value: LayoutShape {
                    geometry: LayoutGeometry::Rectangle {
                        lower_left: LayoutPoint { x: -20, y: -10 },
                        upper_right: LayoutPoint { x: 40, y: 70 },
                    },
                    ..
                },
                ..
            }
        ));
        assert!(
            rectangle_edit(
                LayoutPoint::new(5, 5),
                LayoutPoint::new(5, 10),
                LayoutLayerPurpose::try_new("metal1", "drawing").unwrap(),
            )
            .is_none()
        );
    }

    #[test]
    fn geometry_hit_testing_handles_rectangle_polygon_and_path() {
        assert!(geometry_contains(
            &LayoutGeometry::Rectangle {
                lower_left: LayoutPoint::new(0, 0),
                upper_right: LayoutPoint::new(100, 100),
            },
            LayoutPoint::new(50, 50),
            0
        ));
        assert!(geometry_contains(
            &LayoutGeometry::Polygon {
                vertices: vec![
                    LayoutPoint::new(0, 0),
                    LayoutPoint::new(100, 0),
                    LayoutPoint::new(50, 100),
                ],
            },
            LayoutPoint::new(50, 25),
            0
        ));
        assert!(geometry_contains(
            &LayoutGeometry::Path {
                centerline: vec![LayoutPoint::new(0, 0), LayoutPoint::new(100, 0)],
                width_dbu: 20,
                begin_extension_dbu: 0,
                end_extension_dbu: 0,
            },
            LayoutPoint::new(50, 9),
            0
        ));
    }

    #[test]
    fn governed_layout_editor_renders_with_accessible_controls_at_phone_width() {
        let mut app = app_with_layout();
        let output = render(&mut app, Vec2::new(390.0, 844.0));
        assert!(!output.shapes.is_empty());
        let nodes = &output
            .platform_output
            .accesskit_update
            .expect("physical layout accessibility tree")
            .nodes;
        for label in [
            "Physical layout tools",
            "Select",
            "Rectangle",
            "Active physical layout layer and purpose",
            "Fit",
            "Delete selected",
            "Physical layout canvas",
        ] {
            assert!(
                nodes.iter().any(|(_, node)| node.label() == Some(label)),
                "missing accessible physical-layout control {label}"
            );
        }
    }
}
