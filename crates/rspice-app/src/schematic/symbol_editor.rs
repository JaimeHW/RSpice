//! Symbol view surface.

#[cfg(test)]
mod tests;

use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    PinSummary, Point, PortSpec, SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolDocument,
    SymbolEditorMetadata, SymbolShape, SymbolTextAlign, SymbolTextSize,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::AppState;
use crate::workbench::{SymbolSelection, SymbolTool};
use rspice_design::symbol::publication::SymbolSaveCheck;
pub(crate) use rspice_schematic_editor::symbol_editor::draw_document_preview;
use rspice_schematic_editor::symbol_editor::{
    SymbolViewport, draw_canvas, hit_label, hit_origin, hit_pin, hit_shape, snap_point,
    snap_to_terminal_grid, update_viewport,
};

fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    state: &AppState,
    read_only: bool,
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::ui::accessibility::counted;
    use crate::workbench::commands::vocabulary::Command;
    let placed_pins = document
        .pins
        .iter()
        .filter(|pin| pin.position.is_some())
        .count();
    let selection = state.ui.symbol.editor.effective_selection();
    let selected = selection.pins.len() + selection.shapes.len() + selection.attributes.len();
    let edit_state = if read_only { "Read only." } else { "Editable." };
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
    format!(
        "Symbol editor canvas. {}; {}, {}; {}. Active tool: {}. {}{shortcuts}",
        counted(document.body.len(), "shape", "shapes"),
        counted(document.pins.len(), "pin", "pins"),
        counted(placed_pins, "pin placed", "pins placed"),
        counted(selected, "item selected", "items selected"),
        state.ui.symbol.editor.tool.label(),
        edit_state,
    )
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let ports = state.active_symbol_ports();
    let mut document = match state.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            invalid_symbol_document_state(ui, &error);
            return;
        }
    };
    let mut editor = match state.load_active_symbol_editor_metadata(&document) {
        Ok(metadata) => metadata,
        Err(error) => {
            invalid_symbol_document_state(ui, &error);
            return;
        }
    };

    let mut generate_requested = false;
    if state.active_view_read_only() {
        read_only_banner(ui, state);
    }
    revision_state_strip(ui, state, editor.revision);

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

    // The stage is full-bleed: tools live in the workspace toolbar, the pin
    // contract in the left panel, and object editing in the inspector, so
    // the canvas keeps the whole document area.
    let stage = ui.available_rect_before_wrap();
    let height = ui.available_height();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let canvas_size = vec2(ui.available_width().max(180.0), height);
        let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::click_and_drag());
        let mut changed = false;
        let viewport = update_viewport(ui, &mut state.ui.symbol.editor, rect, &document, &response);
        changed |=
            handle_canvas_interaction(state, &mut document, &mut editor, viewport, &response);
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
        if changed && let Err(error) = state.store_active_symbol_editor_bundle(&document, &editor) {
            state.push_user_message(ConsoleMessage::warning(error));
        }
    });
    canvas_breadcrumb(ui.ctx(), state, stage);
    canvas_check_note(ui.ctx(), &document, &ports, stage);
    show_save_symbol_dialog(ui.ctx(), state, &document, &mut editor, &ports);
}

fn invalid_symbol_document_state(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let stage = ui.available_rect_before_wrap();
    ui.scope_builder(egui::UiBuilder::new().max_rect(stage), |ui| {
        ui.painter().rect_filled(stage, 0.0, c.canvas_bg);
        ui.centered_and_justified(|ui| {
            egui::Frame::new()
                .fill(c.bg_panel)
                .stroke(Stroke::new(1.0, c.err))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(18, 14))
                .show(ui, |ui| {
                    ui.set_max_width(620.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Symbol document could not be opened")
                                .font(theme::sans(tokens::FS_3, FontWeight::Medium))
                                .color(c.err),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(error)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .color(c.text),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "The stored symbol metadata was left unchanged. Restore a valid project revision or repair the imported library before editing this view.",
                            )
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_dim),
                        );
                    });
                });
        });
    });
}

/// Floating cell path, matching the schematic stage's overlay.
fn canvas_breadcrumb(ctx: &egui::Context, state: &AppState, stage: Rect) {
    let reference = &state.workspace.content.active_view;
    let t = Tokens::get(ctx);
    let text = format!(
        "{} / {} / {}",
        reference.library, reference.cell, reference.view
    );
    egui::Area::new(egui::Id::new("symbol-editor.canvas-breadcrumb"))
        .order(egui::Order::Middle)
        .fixed_pos(stage.min + vec2(10.0, 9.0))
        .constrain_to(stage)
        .interactable(false)
        .show(ctx, |ui| {
            overlay_frame(ui, &t, t.color.border, |ui| {
                ui.label(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                );
            });
        });
}

/// Floating pin-contract state, matching the schematic stage's check note.
fn canvas_check_note(
    ctx: &egui::Context,
    document: &SymbolDocument,
    ports: &[PortSpec],
    stage: Rect,
) {
    if ctx.content_rect().width() <= 820.0 {
        return;
    }
    let t = Tokens::get(ctx);
    let (message, color) = match document.pin_summary(ports) {
        PinSummary::Match => ("Pin contract matches the interface".to_owned(), t.color.ok),
        PinSummary::Unplaced(count) => (
            format!(
                "{count} declared pin{} unplaced",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
        ),
        PinSummary::Orphaned(count) => (
            format!(
                "{count} pin{} not declared by the interface",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
        ),
        PinSummary::NoSchematic => (
            "No schematic interface declares this cell".to_owned(),
            t.color.warn,
        ),
    };
    egui::Area::new(egui::Id::new("symbol-editor.canvas-check-note"))
        .order(egui::Order::Middle)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(stage.right_top() + vec2(-11.0, 10.0))
        .constrain_to(stage)
        .interactable(false)
        .show(ctx, |ui| {
            overlay_frame(ui, &t, color.gamma_multiply(0.55), |ui| {
                ui.label(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(color),
                );
            });
        });
}

fn show_save_symbol_dialog(
    ctx: &egui::Context,
    state: &mut AppState,
    document: &SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    ports: &[PortSpec],
) {
    if !state.ui.symbol.save_dialog_open {
        return;
    }
    let checks = state.active_symbol_save_checks(document, ports);
    let blocking = checks.iter().filter(|check| !check.passed).count();
    let note_valid = !state.ui.symbol.save_revision_note.trim().is_empty();
    let blocking_title = format!("{blocking} blocking contract issue(s)");
    let mut dialog = Dialog::new(
        "SYMBOL EDITOR \u{00b7} MODEL-BOUND CONTRACT",
        "Validate and save symbol",
        "Save symbol revision",
    )
    .description("Pin order, electrical types and model binding are validated before this symbol revision replaces the current project view.")
    .size(DialogSize::SimulationWorkflow)
    .initial_height(520.0)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost("Cancel")
    .primary_enabled(blocking == 0 && note_valid && !state.active_view_read_only());
    if let Some(error) = state.ui.symbol.save_error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Symbol revision was not saved",
            error,
        );
    } else if blocking > 0 {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            &blocking_title,
            "Resolve every failed contract row before publishing a symbol revision.",
        );
    }
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        symbol_save_dialog_body(
            ui,
            &checks,
            editor.revision,
            &mut state.ui.symbol.save_revision_note,
        )
    });
    match choice {
        DialogChoice::Primary => {
            let note = state.ui.symbol.save_revision_note.clone();
            match state.publish_active_symbol_revision(document, editor, &note) {
                Ok(revision) => {
                    state.ui.symbol.save_dialog_open = false;
                    state.ui.symbol.save_revision_note.clear();
                    state.ui.symbol.save_error = None;
                    state.push_user_message(ConsoleMessage::info(format!(
                        "Saved symbol revision {revision}"
                    )));
                }
                Err(error) => state.ui.symbol.save_error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.ui.symbol.save_dialog_open = false;
            state.ui.symbol.save_error = None;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn symbol_save_dialog_body(
    ui: &mut Ui,
    checks: &[SymbolSaveCheck],
    current_revision: u64,
    revision_note: &mut String,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.accent.gamma_multiply(0.08))
        .stroke(Stroke::new(1.0, t.color.accent.gamma_multiply(0.42)))
        .inner_margin(egui::Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(
                    "Pin count, netlist order, electrical type and model binding are checked atomically.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text),
            );
        });
    ui.add_space(8.0);
    egui::Grid::new("symbol-editor.save-checks")
        .num_columns(4)
        .striped(true)
        .min_col_width(112.0)
        .show(ui, |ui| {
            for heading in ["CHECK", "EXPECTED", "OBSERVED", "STATUS"] {
                ui.label(
                    egui::RichText::new(heading)
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
            }
            ui.end_row();
            for check in checks {
                ui.label(check.label);
                ui.label(
                    egui::RichText::new(&check.expected)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
                ui.label(
                    egui::RichText::new(&check.observed)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
                ui.label(
                    egui::RichText::new(if check.passed { "pass" } else { "blocked" }).color(
                        if check.passed {
                            t.color.ok
                        } else {
                            t.color.err
                        },
                    ),
                );
                ui.end_row();
            }
        });
    ui.add_space(10.0);
    ui.label(
        egui::RichText::new(format!(
            "Revision note \u{00b7} next revision {}",
            current_revision.saturating_add(1)
        ))
        .font(theme::sans(tokens::FS_1, FontWeight::Medium)),
    );
    let response = ui.add(
        egui::TextEdit::singleline(revision_note)
            .hint_text("Describe the reviewed symbol change")
            .desired_width(f32::INFINITY),
    );
    if revision_note.trim().is_empty() {
        ui.label(
            egui::RichText::new("A revision note is required.")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
    }
    Some(response.id)
}

fn overlay_frame(ui: &mut Ui, t: &Tokens, stroke: Color32, body: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            t.color.bg_panel.r(),
            t.color.bg_panel.g(),
            t.color.bg_panel.b(),
            242,
        ))
        .stroke(Stroke::new(1.0, stroke))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(9, 0))
        .shadow(t.shadow())
        .show(ui, |ui| {
            ui.set_min_height(27.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), body);
        });
}

/// Names who holds the lock, not merely that one is held.
///
/// A banner that says only "read only" leaves the author guessing which of
/// four owners to go and ask. The copy affordance is offered only when the
/// owner is a library, because a copy is a project write that safe mode and
/// a held live lease refuse just as firmly as the edit did.
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
            let library = state.workspace.content.active_view.library.clone();
            ui.label(
                egui::RichText::new(state.symbol_editor_lock_message())
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.warn),
            );
            if !state.symbol_editor_copy_available() {
                return;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if Button::new("Copy to editable library...")
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    let cell = state.workspace.content.active_view.cell.clone();
                    if let Err(error) = state.open_copy_cell_dialog(&library, &cell) {
                        state.push_user_message(crate::diagnostics::ConsoleMessage::error(error));
                    }
                }
            });
        },
    );
}

/// A strip stating whether the drawing on screen is the published revision.
///
/// The tab's dirty dot answers a project question — is anything unsaved —
/// and cannot answer this one: a symbol is written into the project library
/// as it is drawn, so the stored view always agrees with the canvas. Only
/// the editor's own save point knows whether a revision was published since.
fn revision_state_strip(ui: &mut Ui, state: &AppState, revision: u64) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let dirty = state
        .ui
        .symbol
        .history
        .is_dirty(&state.workspace.content.active_key());
    let (text, color) = if dirty {
        (
            format!("Unpublished edits \u{00b7} last published revision {revision}"),
            c.warn,
        )
    } else if revision == 0 {
        ("No revision published yet".to_owned(), c.text_dim)
    } else {
        (format!("Published revision {revision}"), c.ok)
    };
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 22.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, c.border),
            );
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(color),
            );
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

fn handle_canvas_interaction(
    state: &mut AppState,
    document: &mut SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    response: &egui::Response,
) -> bool {
    let center = viewport.screen_to_world(viewport.rect.center());
    state.ui.canvas_view_center = Some((center.x as f64, center.y as f64));
    if let Some(pos) = response.hover_pos() {
        let world = viewport.screen_to_world(pos);
        state.ui.canvas_hover = Some((world.x as f64, world.y as f64));
    } else {
        state.ui.canvas_hover = None;
    }

    let Some(pointer) = response.interact_pointer_pos() else {
        return false;
    };
    let raw_point = viewport.screen_to_world(pointer);
    let body_point = if state.ui.symbol.editor.snap_to_grid {
        snap_point(raw_point, state.ui.symbol.editor.grid_spacing)
    } else {
        raw_point
    };
    // A terminal is the wiring contract, not artwork: it snaps to the
    // terminal pitch whatever the display grid shows and whether or not the
    // author has body snapping on. A pin dropped between grid points is a
    // pin no parent schematic can reach.
    let terminal_point = snap_to_terminal_grid(raw_point);

    if response.secondary_clicked()
        && matches!(
            state.ui.symbol.editor.tool,
            SymbolTool::Line | SymbolTool::Polygon
        )
        && finish_pending_polyline(state, document)
    {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary) {
        // Grabbing something that is already part of a multi-object
        // selection moves the whole selection. Grabbing anything else
        // reduces the selection to it first, which is what makes a
        // mis-grab recoverable rather than a silent group move.
        if grab_belongs_to_group(state, document, editor, viewport, pointer) {
            state.ui.symbol.editor.dragging_group = Some(body_point);
            state.ui.symbol.editor.drag_undo_recorded = false;
        } else if let Some(pin) = hit_pin(document, viewport, pointer) {
            state.ui.symbol.editor.select_pin(pin.clone());
            state.ui.symbol.editor.dragging_pin = Some(pin);
            state.ui.symbol.editor.drag_undo_recorded = false;
        } else if let Some(kind) = hit_label(editor, viewport, pointer) {
            state.ui.symbol.editor.select_attribute(kind);
            state.ui.symbol.editor.dragging_label = Some(kind);
            state.ui.symbol.editor.drag_undo_recorded = false;
        } else if hit_origin(document, viewport, pointer) {
            state.ui.symbol.editor.clear_selection();
            state.ui.symbol.editor.dragging_origin = true;
            state.ui.symbol.editor.drag_undo_recorded = false;
        } else if let Some(shape_index) = hit_shape(document, viewport, pointer) {
            state.ui.symbol.editor.select_shape(shape_index);
            state.ui.symbol.editor.dragging_shape = Some((shape_index, body_point));
            state.ui.symbol.editor.drag_undo_recorded = false;
        } else if matches!(state.ui.symbol.editor.tool, SymbolTool::Select) {
            state.ui.symbol.editor.marquee_start = Some(body_point);
            state.ui.symbol.editor.marquee_current = Some(body_point);
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(last_point) = state.ui.symbol.editor.dragging_group
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.editor.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta == Point::origin() {
            return false;
        }
        record_drag_symbol_edit(state, document);
        translate_selection(state, document, editor, delta);
        state.ui.symbol.editor.dragging_group = Some(body_point);
        return true;
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(name) = state.ui.symbol.editor.dragging_pin.clone()
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.editor.clear_drag_state();
            return false;
        }
        if document.pin(&name).and_then(|pin| pin.position) != Some(terminal_point) {
            record_drag_symbol_edit(state, document);
            let bounds = document.body_bounds();
            if let Some(pin) = document.pin_mut(&name) {
                let side = inferred_side_from_point(terminal_point, bounds);
                let offset = match side {
                    crate::state::SymbolPinSide::Left | crate::state::SymbolPinSide::Right => {
                        terminal_point.y
                    }
                    crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => {
                        terminal_point.x
                    }
                };
                pin.set_side_and_offset(side, offset, bounds);
                return true;
            }
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(kind) = state.ui.symbol.editor.dragging_label
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.editor.clear_drag_state();
            return false;
        }
        if editor
            .attribute(kind)
            .is_some_and(|attribute| attribute.position != body_point)
        {
            record_drag_symbol_edit(state, document);
            if let Some(attribute) = editor.attribute_mut(kind) {
                attribute.position = body_point;
            }
            sync_legacy_attribute_anchor(document, kind, body_point);
            state.ui.symbol.editor.select_attribute(kind);
            return true;
        }
    }
    if response.dragged_by(egui::PointerButton::Primary) && state.ui.symbol.editor.dragging_origin {
        if state.deny_read_only_edit() {
            state.ui.symbol.editor.clear_drag_state();
            return false;
        }
        if document.origin != body_point {
            record_drag_symbol_edit(state, document);
            document.origin = body_point;
            return true;
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some((shape_index, last_point)) = state.ui.symbol.editor.dragging_shape
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.editor.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta != Point::origin() && shape_index < document.body.len() {
            record_drag_symbol_edit(state, document);
            if let Some(shape) = document.body.get_mut(shape_index) {
                shape.translate(delta);
                state.ui.symbol.editor.dragging_shape = Some((shape_index, body_point));
                return true;
            }
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && state.ui.symbol.editor.marquee_start.is_some()
    {
        state.ui.symbol.editor.marquee_current = Some(body_point);
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) {
        if let Some(start) = state.ui.symbol.editor.marquee_start.take() {
            let end = state
                .ui
                .symbol
                .editor
                .marquee_current
                .take()
                .unwrap_or(body_point);
            state
                .ui
                .symbol
                .editor
                .set_selection(SymbolSelection::in_rect(document, editor, start, end));
        }
        state.ui.symbol.editor.clear_drag_state();
    }

    if !response.clicked_by(egui::PointerButton::Primary) {
        return false;
    }

    match state.ui.symbol.editor.tool {
        SymbolTool::Select => {
            let extend = response.ctx.input(|input| input.modifiers.shift);
            if let Some(pin) = hit_pin(document, viewport, pointer) {
                toggle_or_select(
                    state,
                    extend,
                    |selection| selection.toggle_pin(&pin),
                    || SymbolSelection::single_pin(pin.clone()),
                );
            } else if let Some(kind) = hit_label(editor, viewport, pointer) {
                toggle_or_select(
                    state,
                    extend,
                    |selection| selection.toggle_attribute(kind),
                    || SymbolSelection::single_attribute(kind),
                );
            } else if let Some(shape) = hit_shape(document, viewport, pointer) {
                toggle_or_select(
                    state,
                    extend,
                    |selection| selection.toggle_shape(shape),
                    || SymbolSelection::single_shape(shape),
                );
            } else if !extend {
                state.ui.symbol.editor.clear_selection();
            }
            false
        }
        SymbolTool::PlacePin => place_selected_pin(state, document, terminal_point),
        SymbolTool::Line | SymbolTool::Polygon => add_polyline_point(state, body_point),
        SymbolTool::Rectangle => add_rectangle(state, document, body_point),
        SymbolTool::Circle => add_round_shape(state, document, body_point, false),
        SymbolTool::Arc => add_round_shape(state, document, body_point, true),
        SymbolTool::Text => add_text(state, document, body_point),
    }
}

fn record_drag_symbol_edit(state: &mut AppState, document: &SymbolDocument) {
    if state.ui.symbol.editor.drag_undo_recorded {
        return;
    }
    state.record_symbol_edit(document);
    state.ui.symbol.editor.drag_undo_recorded = true;
}

/// Shift-click grows the selection; a plain click replaces it.
fn toggle_or_select(
    state: &mut AppState,
    extend: bool,
    toggle: impl FnOnce(&mut SymbolSelection),
    replace: impl FnOnce() -> SymbolSelection,
) {
    if extend {
        let mut selection = state.ui.symbol.editor.effective_selection();
        toggle(&mut selection);
        state.ui.symbol.editor.set_selection(selection);
        return;
    }
    state.ui.symbol.editor.set_selection(replace());
}

/// Whether the object under `pointer` is one of several already selected.
///
/// A single-object selection is not a group: dragging it must keep the
/// established single-object behaviour, including its side/offset snapping.
fn grab_belongs_to_group(
    state: &AppState,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    viewport: SymbolViewport,
    pointer: Pos2,
) -> bool {
    let selection = state.ui.symbol.editor.effective_selection();
    if selection.len() < 2 {
        return false;
    }
    if let Some(pin) = hit_pin(document, viewport, pointer) {
        return selection.contains_pin(&pin);
    }
    if let Some(kind) = hit_label(editor, viewport, pointer) {
        return selection.attributes.contains(&kind);
    }
    hit_shape(document, viewport, pointer).is_some_and(|index| selection.shapes.contains(&index))
}

/// Move every selected object by `delta` as one edit.
///
/// Terminals travel by the same delta rounded to the terminal pitch, so a
/// group moved on a fine display grid arrives with its pins still on the
/// lattice a parent schematic wires to. Each pin keeps the side it was
/// authored on: a group move is a translation, and re-deriving the edge from
/// the new coordinates would turn a lead through ninety degrees whenever the
/// selection carried the body past it.
fn translate_selection(
    state: &mut AppState,
    document: &mut SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    delta: Point,
) {
    let selection = state.ui.symbol.editor.effective_selection();
    let pin_delta = snap_to_terminal_grid(delta);
    for name in &selection.pins {
        let Some(position) = document.pin(name).and_then(|pin| pin.position) else {
            continue;
        };
        let moved = position + pin_delta;
        let Some(pin) = document.pin_mut(name) else {
            continue;
        };
        let side = pin.side();
        pin.side = Some(side);
        pin.position = Some(moved);
        pin.offset = match side {
            crate::state::SymbolPinSide::Left | crate::state::SymbolPinSide::Right => moved.y,
            crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => moved.x,
        };
    }
    for index in &selection.shapes {
        if let Some(shape) = document.body.get_mut(*index) {
            shape.translate(delta);
        }
    }
    for kind in &selection.attributes {
        let Some(attribute) = editor.attribute_mut(*kind) else {
            continue;
        };
        attribute.position = attribute.position + delta;
        let position = attribute.position;
        sync_legacy_attribute_anchor(document, *kind, position);
    }
}

fn place_selected_pin(state: &mut AppState, document: &mut SymbolDocument, point: Point) -> bool {
    let selected = state
        .ui
        .symbol
        .editor
        .selected_pin
        .clone()
        .or_else(|| next_unplaced_pin(document));
    let Some(name) = selected else {
        return false;
    };
    if state.deny_read_only_edit() {
        return false;
    }
    if document.pin(&name).is_none() {
        return false;
    }
    let changed = document.pin(&name).and_then(|pin| pin.position) != Some(point);
    if changed {
        state.record_symbol_edit(document);
    }
    let bounds = document.body_bounds();
    if let Some(pin) = document.pin_mut(&name) {
        let side = inferred_side_from_point(point, bounds);
        let offset = match side {
            crate::state::SymbolPinSide::Left | crate::state::SymbolPinSide::Right => point.y,
            crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => point.x,
        };
        pin.set_side_and_offset(side, offset, bounds);
        state.ui.symbol.editor.select_pin(name);
        state.ui.symbol.editor.tool = SymbolTool::Select;
        return changed;
    }
    false
}

fn add_polyline_point(state: &mut AppState, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.ui.symbol.editor.pending_polyline.push(point);
    false
}

fn finish_pending_polyline(state: &mut AppState, document: &mut SymbolDocument) -> bool {
    if state.ui.symbol.editor.pending_polyline.len() < 2 {
        return false;
    }
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    let points = std::mem::take(&mut state.ui.symbol.editor.pending_polyline);
    document.body.push(SymbolShape::Polyline {
        points,
        closed: matches!(state.ui.symbol.editor.tool, SymbolTool::Polygon),
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        state.ui.symbol.editor.select_shape(index);
    }
    state.ui.symbol.editor.tool = SymbolTool::Select;
    true
}

/// The edge a dropped terminal belongs to.
///
/// A terminal placed clear of the body belongs to the edge it stands off
/// from — not merely the edge whose coordinate it happens to sit closest to,
/// which on a tall body reads an outer left pin as a rail.
fn inferred_side_from_point(point: Point, bounds: (Point, Point)) -> crate::state::SymbolPinSide {
    crate::state::pin_side_against_body(point, crate::state::PortDirection::InOut, Some(bounds))
}

pub(crate) fn rotate_selected_pin(state: &mut AppState) {
    transform_selected_pin_geometry(state, |side, offset| {
        let side = match side {
            crate::state::SymbolPinSide::Left => crate::state::SymbolPinSide::Top,
            crate::state::SymbolPinSide::Top => crate::state::SymbolPinSide::Right,
            crate::state::SymbolPinSide::Right => crate::state::SymbolPinSide::Bottom,
            crate::state::SymbolPinSide::Bottom => crate::state::SymbolPinSide::Left,
        };
        (side, offset)
    });
}

pub(crate) fn mirror_selected_pin(state: &mut AppState) {
    transform_selected_pin_geometry(state, |side, offset| match side {
        crate::state::SymbolPinSide::Left => (crate::state::SymbolPinSide::Right, offset),
        crate::state::SymbolPinSide::Right => (crate::state::SymbolPinSide::Left, offset),
        crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => (side, -offset),
    });
}

fn transform_selected_pin_geometry(
    state: &mut AppState,
    transform: impl FnOnce(crate::state::SymbolPinSide, i32) -> (crate::state::SymbolPinSide, i32),
) {
    let selection = state.ui.symbol.editor.effective_selection();
    let Some(name) = (selection.pins.len() == 1)
        .then(|| selection.pins.iter().next().cloned())
        .flatten()
    else {
        return;
    };
    if state.deny_read_only_edit() {
        return;
    }
    let mut document = match state.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(error));
            return;
        }
    };
    let metadata = match state.load_active_symbol_editor_metadata(&document) {
        Ok(metadata) => metadata,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(error));
            return;
        }
    };
    let bounds = document.body_bounds();
    let before = document.clone();
    let Some(pin) = document.pin_mut(&name) else {
        return;
    };
    let (side, offset) = transform(pin.side(), pin.offset());
    pin.set_side_and_offset(side, offset, bounds);
    state.record_symbol_edit(&before);
    if let Err(error) = state.store_active_symbol_editor_bundle(&document, &metadata) {
        state.push_user_message(ConsoleMessage::warning(error));
    }
}

fn add_rectangle(state: &mut AppState, document: &mut SymbolDocument, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    let Some(start) = state.ui.symbol.editor.shape_start.take() else {
        state.ui.symbol.editor.shape_start = Some(point);
        return false;
    };
    if start == point {
        state.ui.symbol.editor.shape_start = Some(start);
        return false;
    }
    state.record_symbol_edit(document);
    document.body.push(SymbolShape::Polyline {
        points: vec![
            start,
            Point::new(point.x, start.y),
            point,
            Point::new(start.x, point.y),
        ],
        closed: true,
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        state.ui.symbol.editor.select_shape(index);
    }
    state.ui.symbol.editor.tool = SymbolTool::Select;
    true
}

fn add_text(state: &mut AppState, document: &mut SymbolDocument, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    document.body.push(SymbolShape::Text {
        anchor: point,
        text: "Text".to_owned(),
        size: SymbolTextSize::default(),
        align: SymbolTextAlign::default(),
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        state.ui.symbol.editor.select_shape(index);
    }
    state.ui.symbol.editor.tool = SymbolTool::Select;
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
    if let Some(center) = state.ui.symbol.editor.shape_start.take() {
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
            state.ui.symbol.editor.select_shape(index);
        }
        state.ui.symbol.editor.tool = SymbolTool::Select;
        true
    } else {
        state.ui.symbol.editor.shape_start = Some(point);
        false
    }
}

fn sync_legacy_attribute_anchor(
    document: &mut SymbolDocument,
    kind: SymbolAttributeKind,
    position: Point,
) {
    match kind {
        SymbolAttributeKind::Reference => document.name_anchor = position,
        SymbolAttributeKind::Value => document.value_anchor = position,
        SymbolAttributeKind::Model => {}
    }
}

fn next_unplaced_pin(document: &SymbolDocument) -> Option<String> {
    document
        .pins
        .iter()
        .find(|pin| pin.position.is_none())
        .map(|pin| pin.name.clone())
}
