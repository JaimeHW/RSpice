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
        SCHEMATIC_ZOOM_MAX, SCHEMATIC_ZOOM_MIN, ZoomRequest, apply_zoom_request,
        handle_viewport_navigation,
    },
    scene,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};

use super::{AppState, PortSpec, SymbolDocument, SymbolEditorMetadata, SymbolLibrary};
use crate::state::{SchematicNetHighlighting, SchematicProbe};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

/// How far the parent recedes behind the symbol: the share of the canvas
/// colour laid over it. What it draws stays legible, its wires included, so
/// a moved pin can be judged against them.
const PARENT_VEIL: f32 = 0.6;

/// Screen space kept clear around the symbol when Fit frames it.
const FIT_INSET: f64 = 48.0;

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
    state: &mut AppState,
    rect: Rect,
    response: &egui::Response,
) -> Option<InPlaceFrame> {
    state.symbol_in_place()?;
    let Some(placement) = state.symbol_in_place_placement() else {
        // The parent or the instance is gone: nothing is placed to edit.
        state.ui.symbol.in_place = None;
        return None;
    };
    let navigation = state.ui.preferences.canvas_navigation();
    let session = state.symbol_in_place_mut()?;
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
    let parent = viewport_from_camera(session.pan, session.zoom, rect, ui.ctx().pixels_per_point());
    Some(InPlaceFrame {
        symbol: SymbolViewport::on_sheet(&parent, placement),
        parent,
    })
}

/// Centre `bounds` on the canvas at the zoom that shows all of it.
fn frame_world_rect(
    zoom: &mut f64,
    pan: &mut (f64, f64),
    rect: Rect,
    (min_x, min_y, max_x, max_y): (f64, f64, f64, f64),
) {
    let (width, height) = (f64::from(rect.width()), f64::from(rect.height()));
    let fit = ((width - 2.0 * FIT_INSET).max(1.0) / (max_x - min_x).max(1.0))
        .min((height - 2.0 * FIT_INSET).max(1.0) / (max_y - min_y).max(1.0));
    *zoom = fit.clamp(SCHEMATIC_ZOOM_MIN, SCHEMATIC_ZOOM_MAX);
    *pan = (
        width * 0.5 - (min_x + max_x) * 0.5 * *zoom,
        height * 0.5 - (min_y + max_y) * 0.5 * *zoom,
    );
}

/// The parent, dimmed, and the symbol over it at full strength.
pub(super) fn draw(
    ui: &Ui,
    state: &AppState,
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
        &state.ui.symbol.editor,
    );
}

/// The parent sheet as its own canvas draws it, with nothing selected,
/// hovered, highlighted or annotated: it is context for the edit.
fn draw_parent(
    painter: &egui::Painter,
    frame: &InPlaceFrame,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    let Some(session) = state.symbol_in_place() else {
        return;
    };
    let key = session.parent.key();
    let buffers = &state.workspace.content.schematic_buffers;
    let Some(parent) = buffers.get(&key) else {
        return;
    };
    let resolver = SymbolResolver::new(state.library_manager.catalog(), buffers);
    let symbols = SchematicSymbolContext::new(
        parent.document(),
        None,
        &resolver,
        state.library_manager.revision(),
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
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&key),
                review_markers: state.ui.schematic_visibility.review_markers,
            },
            editor: &session.parent_editor,
            net_highlighting: SchematicNetHighlighting::Off,
            parameter_labels: state.ui.schematic_visibility.parameter_labels,
            hover_wire_vertex: None,
            selection_filter: state.ui.schematic_selection_filter,
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

/// Ask what becomes of unsaved symbol edits before the in-place edit ends.
pub(super) fn show_leaving_question(ctx: &egui::Context, state: &mut AppState) {
    let Some(session) = state
        .symbol_in_place()
        .filter(|session| session.leaving.is_some())
    else {
        return;
    };
    let cell = session.symbol.cell.clone();
    let title = format!("Save the {cell} symbol?");
    let consequence = format!(
        "The {cell} symbol has edits that are not saved. Saving publishes them to every \
         instance of {cell}; discarding puts the symbol back as it was."
    );
    let choice = Dialog::new(&title, "Save")
        .description(&consequence)
        .size(DialogSize::Confirmation)
        .secondary("Discard")
        .secondary_leading()
        .ghost("Cancel")
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(&consequence)
                        .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                        .color(t.color.text),
                )
                .wrap(),
            );
        });
    match choice {
        DialogChoice::Primary => state.save_and_leave_symbol_in_place(),
        DialogChoice::Secondary => state.discard_and_leave_symbol_in_place(),
        DialogChoice::Ghost | DialogChoice::Cancelled => state.stay_in_symbol_in_place(),
        DialogChoice::None => {}
    }
}
