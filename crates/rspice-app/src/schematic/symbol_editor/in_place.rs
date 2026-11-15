//! A symbol edited in place: drawn where one of its instances stands on the
//! parent schematic, over that schematic dimmed and inert.
//!
//! The canvas frames the parent sheet with the parent's own camera. The
//! symbol is drawn through the instance's transform and the pointer is read
//! back through its inverse, so the symbol editor's own operations edit the
//! symbol in symbol coordinates. The parent is painted by the schematic
//! renderer itself, with nothing selected or hovered, under a veil.

use std::collections::HashMap;

use egui::{Rect, Ui};
use rspice_design::symbol_resolver::SymbolResolver;
use rspice_schematic_editor::symbol_editor::{SymbolViewport, draw_in_place};
use rspice_schematic_editor::view::{
    coordinates::viewport_from_camera,
    design_view::DesignView,
    drawing::ProbeVisualStatus,
    navigation::{
        SCHEMATIC_ZOOM_MIN, ZoomRequest, apply_zoom_request, frame_world_rect,
        handle_viewport_navigation,
    },
    scene,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};

use super::SymbolLibrary;
use crate::schematic::view::{CanvasFacts, CanvasParts};
use crate::state::{
    PortSpec, SchematicNetHighlighting, SchematicProbe, SymbolDocument, SymbolEditorMetadata,
};
use crate::ui::tokens::Tokens;

/// How far the parent recedes behind the symbol: the share of the canvas
/// colour laid over it. What it draws stays legible, its wires included, so
/// a moved pin can be judged against them.
const PARENT_VEIL: f32 = 0.6;

/// The canvas's two views for one frame: the parent sheet, and the symbol as
/// the instance places it on that sheet.
pub(super) struct InPlaceFrame {
    parent: Viewport,
    pub(super) symbol: SymbolViewport,
}

/// The frame an in-place edit draws and reads the pointer through, after
/// the frame's pan and zoom; `None` when the open symbol is not edited in
/// place.
pub(super) fn frame(
    ui: &Ui,
    state: &mut CanvasParts<'_>,
    rect: Rect,
    response: &egui::Response,
) -> Option<InPlaceFrame> {
    state.symbol_in_place()?;
    let Some(placement) = state.symbol_in_place_placement() else {
        // The parent or the instance is gone: nothing is placed to edit.
        state.design.symbol.in_place = None;
        crate::workbench::app::sync_in_place_layout(&state.design.symbol.in_place, state.model);
        return None;
    };
    let navigation = state.model.preferences.canvas_navigation();
    let mut zoom_box = state.design.symbol.editor.zoom_box;
    let instance = state.symbol_in_place_bounds();
    let session = state.symbol_in_place_mut()?;
    if std::mem::take(&mut session.entering)
        && let Some(bounds) = instance
        && !shows(session.zoom, session.pan, rect, bounds)
    {
        session.zoom_request = Some(ZoomRequest::Frame(bounds));
    }
    match session.zoom_request.take() {
        Some(ZoomRequest::Frame(bounds)) => {
            frame_world_rect(&mut session.zoom, &mut session.pan, rect, bounds);
        }
        Some(request) => apply_zoom_request(
            &mut session.zoom,
            &mut session.pan,
            (f64::from(rect.width()), f64::from(rect.height())),
            request,
            SCHEMATIC_ZOOM_MIN,
        ),
        None => {}
    }
    handle_viewport_navigation(
        ui,
        response,
        rect,
        &mut session.zoom,
        &mut session.pan,
        SCHEMATIC_ZOOM_MIN,
        navigation,
    );
    // The zoom box frames a rectangle of the parent sheet.
    if zoom_box.armed {
        let (zoom, pan) = (session.zoom, session.pan);
        let to_world = |pos: egui::Pos2| {
            (
                (f64::from(pos.x) - f64::from(rect.min.x) - pan.0) / zoom,
                (f64::from(pos.y) - f64::from(rect.min.y) - pan.1) / zoom,
            )
        };
        if let Some(request) = zoom_box.drive(ui, response, zoom, to_world) {
            session.zoom_request = Some(request);
        }
    }
    session
        .view_history
        .observe((session.zoom, session.pan), ui.input(|input| input.time));
    let parent = viewport_from_camera(session.pan, session.zoom, rect, ui.ctx().pixels_per_point());
    state.design.symbol.editor.zoom_box = zoom_box;
    Some(InPlaceFrame {
        symbol: SymbolViewport::on_sheet(&parent, placement),
        parent,
    })
}

/// Whether the canvas `rect`, at `zoom` and `pan`, shows the whole of the
/// sheet rectangle `bounds`.
fn shows(
    zoom: f64,
    pan: (f64, f64),
    rect: Rect,
    (min_x, min_y, max_x, max_y): (f64, f64, f64, f64),
) -> bool {
    let (width, height) = (f64::from(rect.width()), f64::from(rect.height()));
    let inside = |at: f64, span: f64| (0.0..=span).contains(&at);
    inside(pan.0 + min_x * zoom, width)
        && inside(pan.0 + max_x * zoom, width)
        && inside(pan.1 + min_y * zoom, height)
        && inside(pan.1 + max_y * zoom, height)
}

/// The parent, dimmed, and the symbol over it at full strength.
pub(super) fn draw(
    ui: &Ui,
    state: &CanvasFacts<'_>,
    frame: &InPlaceFrame,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    ports: &[PortSpec],
    symbol_library: Option<&SymbolLibrary>,
) {
    let rect = frame.parent.bounds;
    let painter = ui.painter_at(rect);
    let canvas = Tokens::get(ui.ctx()).color.canvas_bg;
    painter.rect_filled(rect, 0.0, canvas);
    draw_parent(&painter, frame, state, symbol_library);
    painter.rect_filled(rect, 0.0, canvas.gamma_multiply(PARENT_VEIL));
    draw_in_place(
        &painter,
        frame.symbol,
        document,
        editor,
        ports,
        &state.design.symbol.editor,
    );
}

/// The parent sheet as its own canvas draws it, with nothing selected,
/// hovered, highlighted or annotated: it is context for the edit.
fn draw_parent(
    painter: &egui::Painter,
    frame: &InPlaceFrame,
    state: &CanvasFacts<'_>,
    symbol_library: Option<&SymbolLibrary>,
) {
    let Some(session) = state.symbol_in_place() else {
        return;
    };
    let key = session.parent.key();
    let buffers = &state.model.workspace.content.schematic_buffers;
    let Some(parent) = buffers.get(&key) else {
        return;
    };
    let resolver = SymbolResolver::new(state.model.library_manager.catalog(), buffers);
    let symbols = SchematicSymbolContext::new(
        parent.document(),
        None,
        &resolver,
        state.model.library_manager.revision(),
    );
    scene::draw_content(
        painter,
        frame.parent.bounds,
        &frame.parent,
        &scene::SceneView {
            design: DesignView {
                document: parent.document(),
                canvas_cache: None,
                sheet_catalog: state
                    .model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&key),
                review_markers: state.design.schematic_visibility.review_markers,
            },
            editor: &session.parent_editor,
            net_highlighting: SchematicNetHighlighting::Off,
            parameter_labels: state.design.schematic_visibility.parameter_labels,
            hover_wire_vertex: None,
            selection_filter: state.design.schematic_selection_filter,
            hover: None,
        },
        scene::SceneSymbols {
            library: symbol_library,
            context: &symbols,
        },
        scene::SceneOverlays {
            net_class_colors: HashMap::new(),
            operating_point: Vec::new(),
            operating_point_layout: None,
            probe_status: &unannotated_probe,
            selected_trace_expression: &|| None,
            open_pins: &HashMap::new(),
        },
        || session.parent.display_path(),
    );
}

fn unannotated_probe(_: &SchematicProbe) -> ProbeVisualStatus {
    ProbeVisualStatus::Pending
}
