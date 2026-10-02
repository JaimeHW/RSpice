//! Symbol view surface.

mod interaction;

#[cfg(test)]
mod tests;

use egui::{Align2, Color32, Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{PinSummary, PortSpec, SymbolDocument, SymbolEditorMetadata};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::AppState;
use rspice_design::symbol::publication::SymbolSaveCheck;
pub(crate) use rspice_schematic_editor::symbol_editor::draw_document_preview;
use rspice_schematic_editor::symbol_editor::{draw_canvas, update_viewport};

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
