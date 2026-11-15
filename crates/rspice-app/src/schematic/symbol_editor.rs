//! Symbol view surface.

mod context_menu;
mod in_place;
pub(crate) mod interaction;

use egui::{Sense, Ui, WidgetInfo, WidgetType, vec2};

use super::symbols::SymbolLibrary;
use crate::schematic::view::{CanvasFacts, CanvasFrame, DesignSurfaceRequest};
use crate::state::SymbolDocument;
use crate::ui::theme;
use crate::workbench::app::{
    active_symbol_ports, deny_read_only_edit, generate_active_symbol_document,
    load_active_symbol_editor_metadata, symbol_editor_copy_available, symbol_editor_lock_message,
    symbol_editor_request_source,
};
use rspice_schematic_editor::symbol_editor::surface::{
    self, SymbolSurfaceAction, SymbolSurfaceRequest,
};
use rspice_schematic_editor::symbol_editor::{
    SymbolPointer, draw_canvas, draw_preview_column, paint_zoom_box, preview_column_width,
    update_viewport,
};

pub(crate) fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    state: &CanvasFacts<'_>,
    read_only: bool,
    platform: crate::state::command_catalog::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::workbench::{design::commands::DesignCommand, framework::command_registry::ids};
    let shortcuts = crate::workbench::app_state::accessibility_shortcut_summary(
        state.model.preferences.shortcuts(),
        state.commands,
        state.commands,
        platform,
        operating_system,
        &[
            DesignCommand::SelectTool.key(),
            DesignCommand::SymbolPinTool.key(),
            DesignCommand::SymbolPolylineTool.key(),
            DesignCommand::SymbolRectangleTool.key(),
            DesignCommand::SymbolCircleTool.key(),
            DesignCommand::SymbolArcTool.key(),
            DesignCommand::SymbolPolygonTool.key(),
            DesignCommand::SymbolTextTool.key(),
            ids::ZOOM_FIT,
            ids::CANCEL,
        ],
    );
    let in_place = state.symbol_in_place().map(|session| {
        format!(
            "in place on {} in {}",
            session.instance_name, session.parent.cell
        )
    });
    surface::symbol_canvas_accessibility_label(
        document,
        &state.design.symbol.editor,
        read_only,
        in_place.as_deref(),
        &shortcuts,
    )
}

/// The symbol canvas: in its own view, or in place on a parent schematic
/// whose built-in part symbols `symbol_library` draws. Returns where the
/// drawing is, beside the placed preview's column, for the touch strip to
/// stand on; nothing when there is no symbol to draw.
pub fn show(
    ui: &mut Ui,
    canvas: &mut CanvasFrame<'_>,
    symbol_library: Option<&SymbolLibrary>,
) -> Option<egui::Rect> {
    let ports = active_symbol_ports(canvas.model);
    let mut document = match canvas.model.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            surface::invalid_symbol_document_state(ui, &error);
            return None;
        }
    };
    let mut editor = match load_active_symbol_editor_metadata(canvas.model, &document) {
        Ok(metadata) => metadata,
        Err(error) => {
            surface::invalid_symbol_document_state(ui, &error);
            return None;
        }
    };

    let source = symbol_editor_request_source(canvas.model, canvas.workbench);
    if canvas.model.active_view_read_only()
        && let Some(request) = surface::read_only_banner(
            ui,
            &source,
            &symbol_editor_lock_message(canvas.model),
            symbol_editor_copy_available(canvas.model),
        )
    {
        apply_surface_request(canvas, request);
    }

    interaction::bind_active_canvas(&mut canvas.parts());
    // The stage is full-bleed: tools live in the workspace toolbar, the pin
    // contract in the left panel, and object editing in the inspector, so
    // the canvas keeps the whole document area.
    let stage = ui.available_rect_before_wrap();
    let height = ui.available_height();
    // The placed preview, in its own view only, keeps a column of its own
    // beside the drawing, so no zoom or pan puts the drawing under it.
    let preview_width = if canvas.design.symbol.editor.preview_as_placed
        && canvas.design.symbol.in_place.is_none()
    {
        preview_column_width(ui.available_width())
    } else {
        0.0
    };
    let viewport = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let canvas_size = vec2((ui.available_width() - preview_width).max(180.0), height);
            let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::click_and_drag());
            let preview_column = (preview_width > 0.0).then(|| {
                ui.allocate_exact_size(vec2(preview_width, height), Sense::hover())
                    .0
            });
            let navigation = canvas.model.preferences.canvas_navigation();
            // A Space or Alt pan, or the armed zoom box, holds the primary
            // drag; the tools act on none of it.
            let navigating = canvas.design.symbol.editor.zoom_box.armed
                || rspice_schematic_editor::view::navigation::primary_pan_gesture_active(
                    ui, &response,
                );
            // Edited in place, the canvas frames the parent sheet and the symbol
            // stands where its instance does.
            let in_place = in_place::frame(ui, &mut canvas.parts(), rect, &response);
            let viewport = match &in_place {
                Some(frame) => frame.symbol,
                None => update_viewport(
                    ui,
                    &mut canvas.design.symbol.editor,
                    rect,
                    &document,
                    &response,
                    navigation,
                ),
            }
            .pressed_by(SymbolPointer::of(ui.ctx()));
            if !navigating {
                interaction::handle_canvas(
                    &mut canvas.parts(),
                    &mut document,
                    &mut editor,
                    viewport,
                    &response,
                );
            }
            context_menu::handle_context_menu(
                &mut canvas.parts(),
                &mut document,
                &mut editor,
                viewport,
                &response,
            );
            match &in_place {
                Some(frame) => {
                    in_place::draw(
                        ui,
                        &canvas.facts(),
                        frame,
                        &document,
                        &editor,
                        &ports,
                        symbol_library,
                    );
                }
                None => draw_canvas(
                    ui,
                    viewport,
                    &document,
                    &editor,
                    &ports,
                    &canvas.design.symbol.editor,
                ),
            }
            if let Some(column) = preview_column {
                draw_preview_column(
                    ui,
                    column,
                    &document,
                    &ports,
                    &canvas.model.workspace.content.active_view,
                );
            }
            paint_zoom_box(ui, &canvas.design.symbol.editor, viewport);
            // The canvas takes keyboard focus on every press for its shortcuts, so
            // only focus that Tab moved here shows the ring: a press never frames
            // the drawing. Painted after the artwork so the ring is not drawn over.
            if theme::focus_is_visible(ui.ctx(), &response) {
                theme::paint_focus_ring(ui, &response, rect);
            }
            let shortcut_platform = crate::workbench::app_state::runtime_command_platform(ui.ctx());
            let operating_system = ui.ctx().os();
            response.widget_info(|| {
                WidgetInfo::labeled(
                    WidgetType::Image,
                    ui.is_enabled(),
                    symbol_canvas_accessibility_label(
                        &document,
                        &canvas.facts(),
                        canvas.model.active_view_read_only(),
                        shortcut_platform,
                        operating_system,
                    ),
                )
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Canvas);
            });
            crate::workbench::app_state::report_engineering_canvas_focus(
                &response,
                canvas.model.workspace.content.active_view_type(),
            );
            viewport
        })
        .inner;
    // No breadcrumb overlay: the document tab and the inspector's eyebrow
    // both name the view this canvas is drawing.
    let reference = &canvas.model.workspace.content.active_view;
    surface::canvas_check_note(
        ui.ctx(),
        document.pin_summary(&canvas.model.symbol_interface(reference)),
        canvas.model.symbol_contract_source(reference),
        stage,
    );
    // The way to a first drawing lies on the canvas until a shape is drawn
    // or a pin placed, then fades where it stands: it takes no room the
    // canvas would get back, so the drawing never jumps.
    let undrawn =
        document.body.is_empty() && document.pins.iter().all(|pin| pin.position.is_none());
    if !ports.is_empty()
        && let Some(request) = surface::generate_overlay(
            ui.ctx(),
            viewport.rect,
            &source,
            &ports,
            !canvas.model.active_view_read_only(),
            canvas.model.symbol_contract_source(reference),
            undrawn,
        )
    {
        apply_surface_request(canvas, request);
    }
    // After the canvas has taken the keyboard for the click that opened it.
    interaction::show_text_entry(ui.ctx(), &mut canvas.parts(), viewport);
    interaction::show_pin_name_entry(ui.ctx(), &mut canvas.parts(), viewport, &document);
    Some(viewport.rect)
}

pub(crate) fn apply_surface_request(
    canvas: &mut CanvasFrame<'_>,
    request: SymbolSurfaceRequest,
) -> bool {
    if request.source != symbol_editor_request_source(canvas.model, canvas.workbench)
        || canvas.application_modal_open()
    {
        return false;
    }
    match request.action {
        // The project's copy-cell dialog opens once the surface has drawn.
        // Until then the surface reads it as open, as it did when the
        // banner opened it in place.
        SymbolSurfaceAction::CopyToEditableLibrary => {
            if !symbol_editor_copy_available(canvas.model) {
                return false;
            }
            let reference = &request.source.document;
            canvas
                .workbench
                .outbox
                .push(DesignSurfaceRequest::CopyToEditableLibrary {
                    library: reference.library.clone(),
                    cell: reference.cell.clone(),
                });
        }
        SymbolSurfaceAction::GenerateFromSchematic => {
            if deny_read_only_edit(canvas.model, canvas.workbench) {
                return false;
            }
            if let Err(error) =
                generate_active_symbol_document(&mut canvas.design.symbol, canvas.model)
            {
                canvas.prompt(error);
            }
        }
    }
    true
}
