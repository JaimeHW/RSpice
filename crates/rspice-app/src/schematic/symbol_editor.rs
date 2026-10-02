//! Symbol view surface.

mod interaction;

#[cfg(test)]
mod tests;

use egui::{Sense, Ui, WidgetInfo, WidgetType, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{PortSpec, SymbolDocument, SymbolEditorMetadata};
use crate::ui::theme;
use crate::ui::widgets::DialogChoice;
use crate::workbench::AppState;
pub(crate) use rspice_schematic_editor::symbol_editor::draw_document_preview;
use rspice_schematic_editor::symbol_editor::surface::{
    self, SymbolSurfaceAction, SymbolSurfaceRequest,
};
use rspice_schematic_editor::symbol_editor::{draw_canvas, update_viewport};

fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    state: &AppState,
    read_only: bool,
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::workbench::commands::vocabulary::Command;
    let shortcuts = crate::workbench::app_state::accessibility_shortcut_summary(
        state.ui.preferences.shortcuts(),
        platform,
        operating_system,
        &[
            Command::SelectTool,
            Command::SymbolPinTool,
            Command::SymbolPolylineTool,
            Command::SymbolRectangleTool,
            Command::SymbolCircleTool,
            Command::SymbolArcTool,
            Command::SymbolPolygonTool,
            Command::SymbolTextTool,
            Command::ZoomFit,
            Command::Cancel,
        ],
    );
    surface::symbol_canvas_accessibility_label(
        document,
        &state.ui.symbol.editor,
        read_only,
        &shortcuts,
    )
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let ports = state.active_symbol_ports();
    let mut document = match state.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            surface::invalid_symbol_document_state(ui, &error);
            return;
        }
    };
    let mut editor = match state.load_active_symbol_editor_metadata(&document) {
        Ok(metadata) => metadata,
        Err(error) => {
            surface::invalid_symbol_document_state(ui, &error);
            return;
        }
    };

    let source = state.symbol_editor_request_source();
    if state.active_view_read_only()
        && let Some(request) = surface::read_only_banner(
            ui,
            &source,
            &state.symbol_editor_lock_message(),
            state.symbol_editor_copy_available(),
        )
    {
        apply_surface_request(state, request);
    }
    let dirty = state
        .ui
        .symbol
        .history
        .is_dirty(&state.workspace.content.active_key());
    surface::revision_state_strip(ui, dirty, editor.revision);
    if document.body.is_empty()
        && !ports.is_empty()
        && let Some(request) =
            surface::empty_generate_state(ui, &source, &ports, !state.active_view_read_only())
    {
        if !apply_surface_request(state, request) {
            return;
        }
        document = state
            .load_active_symbol_document()
            .unwrap_or_else(|_| SymbolDocument::generated_from_ports(&ports));
    }

    interaction::bind_active_canvas(state);
    // The stage is full-bleed: tools live in the workspace toolbar, the pin
    // contract in the left panel, and object editing in the inspector, so
    // the canvas keeps the whole document area.
    let stage = ui.available_rect_before_wrap();
    let height = ui.available_height();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let canvas_size = vec2(ui.available_width().max(180.0), height);
        let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::click_and_drag());
        let viewport = update_viewport(ui, &mut state.ui.symbol.editor, rect, &document, &response);
        interaction::handle_canvas(state, &mut document, &mut editor, viewport, &response);
        draw_canvas(
            ui,
            viewport,
            &document,
            &editor,
            &ports,
            &state.ui.symbol.editor,
            &state.workspace.content.active_view,
        );
        // The canvas takes keyboard focus for tool and nudge shortcuts, so it
        // owes the same visible focus ring as every other custom click target.
        // Painted after the artwork so the ring is not drawn over.
        theme::paint_focus_ring(ui, &response, rect);
        let shortcut_platform = crate::workbench::app_state::runtime_command_platform(ui.ctx());
        let operating_system = ui.ctx().os();
        response.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Image,
                ui.is_enabled(),
                symbol_canvas_accessibility_label(
                    &document,
                    state,
                    state.active_view_read_only(),
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
            state.workspace.content.active_view_type(),
        );
    });
    surface::canvas_breadcrumb(ui.ctx(), &state.workspace.content.active_view, stage);
    surface::canvas_check_note(ui.ctx(), &document, &ports, stage);
    show_save_symbol_dialog(ui.ctx(), state, &document, &mut editor, &ports);
}

fn show_save_symbol_dialog(
    ctx: &egui::Context,
    state: &mut AppState,
    document: &SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    ports: &[PortSpec],
) {
    if !state.ui.symbol.editor.save_dialog_open {
        return;
    }
    let checks = state.active_symbol_save_checks(document, ports);
    let can_publish = !state.active_view_read_only();
    let choice = surface::show_save_symbol_dialog(
        ctx,
        &checks,
        editor.revision,
        &mut state.ui.symbol.editor.save_revision_note,
        state.ui.symbol.editor.save_error.as_deref(),
        can_publish,
    );
    match choice {
        DialogChoice::Primary => {
            let note = state.ui.symbol.editor.save_revision_note.clone();
            match state.publish_active_symbol_revision(document, editor, &note) {
                Ok(revision) => {
                    state.ui.symbol.editor.save_dialog_open = false;
                    state.ui.symbol.editor.save_revision_note.clear();
                    state.ui.symbol.editor.save_error = None;
                    state.push_user_message(ConsoleMessage::info(format!(
                        "Saved symbol revision {revision}"
                    )));
                }
                Err(error) => state.ui.symbol.editor.save_error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.ui.symbol.editor.save_dialog_open = false;
            state.ui.symbol.editor.save_error = None;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn apply_surface_request(state: &mut AppState, request: SymbolSurfaceRequest) -> bool {
    if request.source != state.symbol_editor_request_source() || state.application_modal_open() {
        return false;
    }
    match request.action {
        SymbolSurfaceAction::CopyToEditableLibrary => {
            if !state.symbol_editor_copy_available() {
                return false;
            }
            let reference = &request.source.document;
            if let Err(error) = state.open_copy_cell_dialog(&reference.library, &reference.cell) {
                state.push_user_message(ConsoleMessage::error(error));
            }
        }
        SymbolSurfaceAction::GenerateFromSchematic => {
            if state.deny_read_only_edit() {
                return false;
            }
            if let Err(error) = state.generate_active_symbol_document() {
                state.push_user_message(ConsoleMessage::warning(error));
            }
        }
    }
    true
}
