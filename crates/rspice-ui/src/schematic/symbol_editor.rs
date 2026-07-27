//! Symbol view surface.

use egui::{
    Align2, Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::workbench::{AppState, ConsoleMessage};
use crate::schematic::view::resolved_symbol_render::draw_resolved_symbol;
use crate::state::{
    Component, ComponentType, LibraryCellInstance, PinSummary, Point, PortDirection, PortSpec,
    ResolvedCellSymbol, SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolDocument,
    SymbolEditorMetadata, SymbolPin, SymbolShape,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::{SymbolGridSpacing, SymbolSelection, SymbolTool};

const PIN_HIT_RADIUS: f32 = 10.0;
const SCROLL_ZOOM_SENSITIVITY: f32 = 0.001;
const PREVIEW_TILE_MAX_WIDTH: f32 = 240.0;
const PREVIEW_TILE_MAX_HEIGHT: f32 = 168.0;
const PREVIEW_TILE_MIN_WIDTH: f32 = 168.0;
const PREVIEW_TILE_MIN_HEIGHT: f32 = 118.0;
const PREVIEW_TILE_MARGIN: f32 = 12.0;
const PREVIEW_TILE_HEADER: f32 = 26.0;
const PREVIEW_FIT_PADDING: f32 = 10.0;

#[derive(Clone, Copy)]
struct SymbolViewport {
    rect: Rect,
    zoom: f32,
    pan: Vec2,
}

fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    state: &AppState,
    read_only: bool,
    platform: crate::workbench::commands::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::ui::accessibility::counted;
    use crate::workbench::commands::Command;
    let placed_pins = document
        .pins
        .iter()
        .filter(|pin| pin.position.is_some())
        .count();
    let selection = state.ui.symbol.effective_selection();
    let selected = selection.pins.len()
        + selection.shapes.len()
        + selection.attributes.len()
        + selection.texts.len();
    let edit_state = if read_only { "Read only." } else { "Editable." };
    let shortcuts = crate::workbench::app::accessibility_shortcut_summary(
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
        state.ui.symbol.tool.label(),
        edit_state,
    )
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
        let viewport = update_viewport(ui, state, rect, &document, &response);
        changed |=
            handle_canvas_interaction(state, &mut document, &mut editor, viewport, &response);
        draw_canvas(ui, viewport, &document, &editor, &ports, state);
        let shortcut_platform = crate::workbench::app::runtime_command_platform(ui.ctx());
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
        crate::workbench::app::report_engineering_canvas_focus(
            &response,
            state.workspace.active_view_type(),
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
    let reference = &state.workspace.active_view;
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

#[derive(Debug, Clone)]
struct SymbolSaveCheck {
    label: &'static str,
    expected: String,
    observed: String,
    passed: bool,
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
    let checks = symbol_save_checks(state, document, ports);
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
            let mut candidate = editor.clone();
            match candidate
                .publish_revision(document, &state.ui.symbol.save_revision_note)
                .and_then(|revision| {
                    state
                        .store_active_symbol_editor_bundle(document, &candidate)
                        .map(|_| revision)
                }) {
                Ok(revision) => {
                    *editor = candidate;
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

fn symbol_save_checks(
    state: &AppState,
    document: &SymbolDocument,
    ports: &[PortSpec],
) -> Vec<SymbolSaveCheck> {
    let reference = &state.workspace.active_view;
    let definition = state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .and_then(|view| {
            crate::state::ModelBoundSymbolDefinition::load_from_view(view)
                .ok()
                .flatten()
        });
    let mut expected_names = ports
        .iter()
        .map(|port| port.name.clone())
        .collect::<Vec<_>>();
    let expected_directions = if let Some(definition) = definition.as_ref() {
        let mut pins = definition.pins.iter().collect::<Vec<_>>();
        pins.sort_by_key(|pin| pin.order);
        expected_names = pins.iter().map(|pin| pin.name.clone()).collect();
        pins.into_iter()
            .map(|pin| {
                (
                    pin.electrical_type,
                    pin.direction,
                    pin.side,
                    pin.name.clone(),
                )
            })
            .collect::<Vec<_>>()
    } else {
        ports
            .iter()
            .map(|port| {
                (
                    match port.direction {
                        PortDirection::Supply => crate::state::SymbolElectricalType::Power,
                        _ => crate::state::SymbolElectricalType::Analog,
                    },
                    port.direction,
                    match port.direction {
                        PortDirection::In => crate::state::SymbolPinSide::Left,
                        PortDirection::Out | PortDirection::InOut => {
                            crate::state::SymbolPinSide::Right
                        }
                        PortDirection::Supply => crate::state::SymbolPinSide::Top,
                    },
                    port.name.clone(),
                )
            })
            .collect()
    };
    let observed_names = document
        .pins
        .iter()
        .map(|pin| pin.name.clone())
        .collect::<Vec<_>>();
    let electrical_match = expected_directions.len() == document.pins.len()
        && expected_directions.iter().zip(&document.pins).all(
            |((electrical, direction, _, name), pin)| {
                pin.name.eq_ignore_ascii_case(name)
                    && pin.electrical_type() == *electrical
                    && pin.direction == *direction
            },
        );
    let placement_valid = document
        .pins
        .iter()
        .all(|pin| pin.position.is_some() && pin.terminal_on_grid());
    let family = definition
        .as_ref()
        .map(|definition| definition.identity.cell.as_str())
        .unwrap_or("standalone symbol");
    vec![
        SymbolSaveCheck {
            label: "Pin count",
            expected: expected_names.len().to_string(),
            observed: document.pins.len().to_string(),
            passed: expected_names.len() == document.pins.len(),
        },
        SymbolSaveCheck {
            label: "Netlist order",
            expected: expected_names.join(" "),
            observed: observed_names.join(" "),
            passed: expected_names.len() == observed_names.len()
                && expected_names
                    .iter()
                    .zip(&observed_names)
                    .all(|(expected, observed)| expected.eq_ignore_ascii_case(observed)),
        },
        SymbolSaveCheck {
            label: "Electrical types",
            expected: "bound contract".to_owned(),
            observed: if electrical_match {
                "matched".to_owned()
            } else {
                "mismatch".to_owned()
            },
            passed: electrical_match,
        },
        SymbolSaveCheck {
            label: "Pin placement",
            expected: "placed on terminal grid".to_owned(),
            observed: if placement_valid {
                "all placed".to_owned()
            } else {
                "unplaced or off grid".to_owned()
            },
            passed: placement_valid,
        },
        SymbolSaveCheck {
            label: "Hidden power pins",
            expected: "forbidden".to_owned(),
            observed: "none".to_owned(),
            passed: true,
        },
        SymbolSaveCheck {
            label: "Model family",
            expected: family.to_owned(),
            observed: reference.cell.clone(),
            passed: definition
                .as_ref()
                .is_none_or(|definition| definition.identity.cell == reference.cell),
        },
    ]
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
    if state.ui.symbol.needs_fit {
        fit_symbol_view(state, rect, document);
        state.ui.symbol.needs_fit = false;
    }
    if response.hovered() {
        let factor = ui.input(|input| symbol_scroll_zoom_factor(input.smooth_scroll_delta.y));
        if let Some(factor) = factor {
            state.ui.symbol.zoom = (state.ui.symbol.zoom * factor).clamp(1.0, 18.0);
        }
    }
    if response.dragged_by(egui::PointerButton::Middle) {
        let delta = ui.input(|input| input.pointer.delta());
        state.ui.symbol.pan.0 += delta.x;
        state.ui.symbol.pan.1 += delta.y;
    }
    SymbolViewport {
        rect,
        zoom: state.ui.symbol.zoom,
        pan: vec2(state.ui.symbol.pan.0, state.ui.symbol.pan.1),
    }
}

fn symbol_scroll_zoom_factor(scroll: f32) -> Option<f32> {
    if scroll.abs() <= f32::EPSILON {
        return None;
    }
    Some((scroll * SCROLL_ZOOM_SENSITIVITY).exp().clamp(0.5, 2.0))
}

fn fit_symbol_view(state: &mut AppState, rect: Rect, document: &SymbolDocument) {
    let (min, max) = document_bounds(document);
    let width = (max.x - min.x).abs().max(80) as f32;
    let height = (max.y - min.y).abs().max(80) as f32;
    let zoom = ((rect.width() - 96.0).max(80.0) / width)
        .min((rect.height() - 96.0).max(80.0) / height)
        .clamp(1.0, 8.0);
    let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
    state.ui.symbol.zoom = zoom;
    state.ui.symbol.pan = (-(center.x as f32) * zoom, -(center.y as f32) * zoom);
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
    let body_point = if state.ui.symbol.snap_to_grid {
        snap_point(raw_point, state.ui.symbol.grid_spacing)
    } else {
        raw_point
    };
    let terminal_point = body_point;

    if response.secondary_clicked()
        && matches!(state.ui.symbol.tool, SymbolTool::Line | SymbolTool::Polygon)
        && finish_pending_polyline(state, document)
    {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(pin) = hit_pin(document, viewport, pointer)
    {
        state.ui.symbol.select_pin(pin.clone());
        state.ui.symbol.dragging_pin = Some(pin);
        state.ui.symbol.drag_undo_recorded = false;
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.ui.symbol.dragging_pin.is_none()
        && let Some(label) = hit_label(editor, viewport, pointer)
    {
        state.ui.symbol.dragging_label = Some(label);
        state.ui.symbol.drag_undo_recorded = false;
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.ui.symbol.dragging_pin.is_none()
        && state.ui.symbol.dragging_label.is_none()
        && hit_origin(document, viewport, pointer)
    {
        state.ui.symbol.clear_selection();
        state.ui.symbol.dragging_origin = true;
        state.ui.symbol.drag_undo_recorded = false;
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && state.ui.symbol.dragging_pin.is_none()
        && state.ui.symbol.dragging_label.is_none()
        && !state.ui.symbol.dragging_origin
        && let Some(shape_index) = hit_shape(document, viewport, pointer)
    {
        state.ui.symbol.select_shape(shape_index);
        state.ui.symbol.dragging_shape = Some((shape_index, body_point));
        state.ui.symbol.drag_undo_recorded = false;
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && matches!(state.ui.symbol.tool, SymbolTool::Select)
        && state.ui.symbol.dragging_pin.is_none()
        && state.ui.symbol.dragging_label.is_none()
        && !state.ui.symbol.dragging_origin
        && state.ui.symbol.dragging_shape.is_none()
    {
        state.ui.symbol.marquee_start = Some(body_point);
        state.ui.symbol.marquee_current = Some(body_point);
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(name) = state.ui.symbol.dragging_pin.clone()
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.clear_drag_state();
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
        && let Some(label) = state.ui.symbol.dragging_label.clone()
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.clear_drag_state();
            return false;
        }
        if let Some(kind) = attribute_kind_from_drag_id(&label) {
            if editor
                .attribute(kind)
                .is_some_and(|attribute| attribute.position != body_point)
            {
                record_drag_symbol_edit(state, document);
                if let Some(attribute) = editor.attribute_mut(kind) {
                    attribute.position = body_point;
                    sync_legacy_attribute_anchor(document, attribute);
                }
                state.ui.symbol.select_attribute(kind);
                return true;
            }
        } else if let Some(id) = text_id_from_drag_id(&label)
            && editor
                .texts
                .iter()
                .find(|text| text.id == id)
                .is_some_and(|text| text.position != body_point)
        {
            record_drag_symbol_edit(state, document);
            if let Some(text) = editor.texts.iter_mut().find(|text| text.id == id) {
                text.position = body_point;
            }
            state.ui.symbol.select_text(id);
            return true;
        }
    }
    if response.dragged_by(egui::PointerButton::Primary) && state.ui.symbol.dragging_origin {
        if state.deny_read_only_edit() {
            state.ui.symbol.clear_drag_state();
            return false;
        }
        if document.origin != body_point {
            record_drag_symbol_edit(state, document);
            document.origin = body_point;
            return true;
        }
    }
    if response.dragged_by(egui::PointerButton::Primary)
        && let Some((shape_index, last_point)) = state.ui.symbol.dragging_shape
    {
        if state.deny_read_only_edit() {
            state.ui.symbol.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta != Point::origin() && shape_index < document.body.len() {
            record_drag_symbol_edit(state, document);
            if let Some(shape) = document.body.get_mut(shape_index) {
                shape.translate(delta);
                state.ui.symbol.dragging_shape = Some((shape_index, body_point));
                return true;
            }
        }
    }
    if response.dragged_by(egui::PointerButton::Primary) && state.ui.symbol.marquee_start.is_some()
    {
        state.ui.symbol.marquee_current = Some(body_point);
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) {
        if let Some(start) = state.ui.symbol.marquee_start.take() {
            let end = state.ui.symbol.marquee_current.take().unwrap_or(body_point);
            state
                .ui
                .symbol
                .set_selection(SymbolSelection::in_rect(document, start, end));
        }
        state.ui.symbol.clear_drag_state();
    }

    if !response.clicked_by(egui::PointerButton::Primary) {
        return false;
    }

    match state.ui.symbol.tool {
        SymbolTool::Select => {
            if let Some(pin) = hit_pin(document, viewport, pointer) {
                state.ui.symbol.select_pin(pin);
            } else if let Some(label) = hit_label(editor, viewport, pointer) {
                if let Some(kind) = attribute_kind_from_drag_id(&label) {
                    state.ui.symbol.select_attribute(kind);
                } else if let Some(id) = text_id_from_drag_id(&label) {
                    state.ui.symbol.select_text(id);
                }
            } else if let Some(shape) = hit_shape(document, viewport, pointer) {
                state.ui.symbol.select_shape(shape);
            } else {
                state.ui.symbol.clear_selection();
            }
            false
        }
        SymbolTool::PlacePin => place_selected_pin(state, document, terminal_point),
        SymbolTool::Line | SymbolTool::Polygon => add_polyline_point(state, body_point),
        SymbolTool::Rectangle => add_rectangle(state, document, body_point),
        SymbolTool::Circle => add_round_shape(state, document, body_point, false),
        SymbolTool::Arc => add_round_shape(state, document, body_point, true),
        SymbolTool::Text => add_text(state, document, editor, body_point),
    }
}

fn record_drag_symbol_edit(state: &mut AppState, document: &SymbolDocument) {
    if state.ui.symbol.drag_undo_recorded {
        return;
    }
    state.record_symbol_edit(document);
    state.ui.symbol.drag_undo_recorded = true;
}

fn place_selected_pin(state: &mut AppState, document: &mut SymbolDocument, point: Point) -> bool {
    let selected = state
        .ui
        .symbol
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
        state.ui.symbol.select_pin(name);
        state.ui.symbol.tool = SymbolTool::Select;
        return changed;
    }
    false
}

fn add_polyline_point(state: &mut AppState, point: Point) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.ui.symbol.pending_polyline.push(point);
    false
}

fn finish_pending_polyline(state: &mut AppState, document: &mut SymbolDocument) -> bool {
    if state.ui.symbol.pending_polyline.len() < 2 {
        return false;
    }
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    let points = std::mem::take(&mut state.ui.symbol.pending_polyline);
    document.body.push(SymbolShape::Polyline {
        points,
        closed: matches!(state.ui.symbol.tool, SymbolTool::Polygon),
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        state.ui.symbol.select_shape(index);
    }
    state.ui.symbol.tool = SymbolTool::Select;
    true
}

fn inferred_side_from_point(
    point: Point,
    (min, max): (Point, Point),
) -> crate::state::SymbolPinSide {
    let distances = [
        (
            point.x.saturating_sub(min.x).unsigned_abs(),
            crate::state::SymbolPinSide::Left,
        ),
        (
            point.x.saturating_sub(max.x).unsigned_abs(),
            crate::state::SymbolPinSide::Right,
        ),
        (
            point.y.saturating_sub(min.y).unsigned_abs(),
            crate::state::SymbolPinSide::Top,
        ),
        (
            point.y.saturating_sub(max.y).unsigned_abs(),
            crate::state::SymbolPinSide::Bottom,
        ),
    ];
    distances
        .into_iter()
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, side)| side)
        .unwrap_or(crate::state::SymbolPinSide::Left)
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
    let selection = state.ui.symbol.effective_selection();
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
    let Some(start) = state.ui.symbol.shape_start.take() else {
        state.ui.symbol.shape_start = Some(point);
        return false;
    };
    if start == point {
        state.ui.symbol.shape_start = Some(start);
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
        state.ui.symbol.select_shape(index);
    }
    state.ui.symbol.tool = SymbolTool::Select;
    true
}

fn add_text(
    state: &mut AppState,
    document: &SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    point: Point,
) -> bool {
    if state.deny_read_only_edit() {
        return false;
    }
    state.record_symbol_edit(document);
    let id = editor.allocate_text("Text", point);
    state.ui.symbol.select_text(id);
    state.ui.symbol.tool = SymbolTool::Select;
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
    if let Some(center) = state.ui.symbol.shape_start.take() {
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
            state.ui.symbol.select_shape(index);
        }
        state.ui.symbol.tool = SymbolTool::Select;
        true
    } else {
        state.ui.symbol.shape_start = Some(point);
        false
    }
}

fn draw_canvas(
    ui: &mut Ui,
    viewport: SymbolViewport,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    ports: &[PortSpec],
    state: &AppState,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter_at(viewport.rect);
    painter.rect_filled(viewport.rect, 0.0, c.canvas_bg);
    if state.ui.symbol.show_grid {
        draw_grid(
            &painter,
            viewport,
            c.canvas_grid,
            state.ui.symbol.grid_spacing,
        );
    }
    draw_body(
        &painter,
        viewport,
        document,
        c.symbol,
        &state.ui.symbol.effective_selection().shapes,
        c.accent,
    );
    draw_bbox_and_origin(&painter, viewport, document, &t);
    draw_pins(&painter, viewport, document, ports, state);
    draw_labels(&painter, viewport, editor, state, &t);
    draw_marquee(&painter, viewport, state, &t);
    if state.ui.symbol.preview_as_placed {
        draw_preview_tile(ui, viewport.rect, document, ports, state, &t);
    }
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
    let selection = state.ui.symbol.effective_selection();
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
        painter.rect_stroke(pad, 0.0, pad_stroke, egui::StrokeKind::Inside);
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
        state.ui.symbol.marquee_start,
        state.ui.symbol.marquee_current,
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
    editor: &SymbolEditorMetadata,
    state: &AppState,
    t: &Tokens,
) {
    let selection = state.ui.symbol.effective_selection();
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
    for text in &editor.texts {
        if !text.shown {
            continue;
        }
        painter.text(
            viewport.world_to_screen(text.position),
            Align2::LEFT_CENTER,
            &text.text,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if selection.texts.contains(&text.id) {
                t.color.accent
            } else {
                t.color.symbol
            },
        );
    }
}

fn draw_preview_tile(
    ui: &mut Ui,
    canvas: Rect,
    document: &SymbolDocument,
    ports: &[PortSpec],
    state: &AppState,
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
pub(crate) fn draw_document_preview(
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
    SymbolViewport {
        rect,
        zoom,
        pan: vec2(-(center.x as f32) * zoom, -(center.y as f32) * zoom),
    }
}

fn preview_effective_bounds(document: &SymbolDocument) -> (Point, Point) {
    let (min, max) = document_bounds(document);
    (min - document.origin, max - document.origin)
}

fn hit_pin(document: &SymbolDocument, viewport: SymbolViewport, pos: Pos2) -> Option<String> {
    document
        .pins
        .iter()
        .filter_map(|pin| Some((pin.name.clone(), viewport.world_to_screen(pin.position?))))
        .find(|(_, pin_pos)| pin_pos.distance(pos) <= PIN_HIT_RADIUS)
        .map(|(name, _)| name)
}

fn hit_label(editor: &SymbolEditorMetadata, viewport: SymbolViewport, pos: Pos2) -> Option<String> {
    for attribute in editor.attributes.iter().rev() {
        if attribute.shown && viewport.world_to_screen(attribute.position).distance(pos) <= 22.0 {
            return Some(format!("attribute:{}", attribute.kind.key()));
        }
    }
    for text in editor.texts.iter().rev() {
        if text.shown && viewport.world_to_screen(text.position).distance(pos) <= 22.0 {
            return Some(format!("text:{}", text.id));
        }
    }
    None
}

fn attribute_kind_from_drag_id(value: &str) -> Option<SymbolAttributeKind> {
    let key = value.strip_prefix("attribute:")?;
    SymbolAttributeKind::ALL
        .into_iter()
        .find(|kind| kind.key() == key)
}

fn text_id_from_drag_id(value: &str) -> Option<u64> {
    value.strip_prefix("text:")?.parse().ok()
}

fn sync_legacy_attribute_anchor(
    document: &mut SymbolDocument,
    attribute: &crate::state::SymbolAttribute,
) {
    match attribute.kind {
        SymbolAttributeKind::Reference => document.name_anchor = attribute.position,
        SymbolAttributeKind::Value => document.value_anchor = attribute.position,
        SymbolAttributeKind::Model => {}
    }
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

fn snap_point(point: Point, spacing: SymbolGridSpacing) -> Point {
    let grid = spacing.model_step();
    let snap = |v: i32| ((v as f32 / grid).round() * grid).round() as i32;
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
    fn symbol_editor_has_no_direct_product_key_bypass() {
        let source = include_str!("symbol_editor.rs");
        assert!(!source.contains(concat!("input.", "key_pressed(")));
        assert!(!source.contains(concat!("ctx.input_mut(|input| input.", "consume_key")));
        assert!(!source.contains(concat!("Use S", " for select")));
        assert!(!source.contains(concat!("Escape", " to cancel")));
    }

    #[test]
    fn finishing_pending_polyline_creates_one_selected_shape() {
        let mut state = AppState::default();
        let mut document = SymbolDocument::default();
        state.ui.symbol.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

        assert!(finish_pending_polyline(&mut state, &mut document));

        assert_eq!(document.body.len(), 1);
        assert_eq!(state.ui.symbol.selected_shape, Some(0));
        assert!(state.ui.symbol.selection.shapes.contains(&0));
        assert!(state.ui.symbol.pending_polyline.is_empty());
    }

    #[test]
    fn place_pin_without_available_pin_does_not_record_undo() {
        let mut state = AppState::default();
        let mut document = SymbolDocument::default();

        assert!(!place_selected_pin(
            &mut state,
            &mut document,
            Point::new(10, 0)
        ));

        assert!(!state.can_undo_active_symbol_document());
        assert!(document.pins.is_empty());
    }

    #[test]
    fn drag_symbol_edit_records_one_undo_snapshot_per_gesture() {
        let mut state = AppState::default();
        let document = SymbolDocument::default();

        record_drag_symbol_edit(&mut state, &document);
        record_drag_symbol_edit(&mut state, &document);

        let key = state.workspace.active_key();
        assert_eq!(
            state.ui.symbol.undo_stacks.get(&key).map(Vec::len),
            Some(1),
            "a drag must create one undo transaction no matter how many snap buckets it crosses"
        );
        assert!(state.ui.symbol.drag_undo_recorded);

        state.ui.symbol.clear_drag_state();

        assert!(!state.ui.symbol.drag_undo_recorded);
    }

    #[test]
    fn smooth_scroll_zoom_factor_is_proportional_not_binary() {
        let tiny = symbol_scroll_zoom_factor(1.0).expect("tiny smooth scroll should zoom");
        assert!(
            tiny > 1.0 && tiny < 1.01,
            "tiny smooth-scroll residue must not apply a full wheel notch: {tiny}"
        );

        let wheel = symbol_scroll_zoom_factor(120.0).expect("wheel scroll should zoom");
        assert!(
            wheel > tiny && wheel < 1.2,
            "a normal wheel notch should be noticeable but restrained: {wheel}"
        );
    }

    #[test]
    fn preview_viewport_fits_large_symbols_inside_tile_body() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-300, -220),
                    Point::new(300, -220),
                    Point::new(300, 220),
                    Point::new(-300, 220),
                ],
                closed: true,
            }],
            ..SymbolDocument::default()
        };
        let body_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(168.0, 102.0));

        let viewport = preview_viewport_for_tile(body_rect, &document);
        let (min, max) = document_bounds(&document);
        let min_screen = viewport.world_to_screen(min);
        let max_screen = viewport.world_to_screen(max);

        assert!(
            body_rect.shrink(10.0).contains(min_screen),
            "min={min_screen:?}"
        );
        assert!(
            body_rect.shrink(10.0).contains(max_screen),
            "max={max_screen:?}"
        );
        assert!(
            viewport.zoom < 0.25,
            "large authored symbols must be scaled down for preview: {}",
            viewport.zoom
        );
    }

    #[test]
    fn preview_viewport_fits_nonzero_origin_as_placed_symbol() {
        let document = SymbolDocument {
            origin: Point::new(200, 100),
            name_anchor: Point::new(200, 70),
            value_anchor: Point::new(200, 130),
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(160, 80),
                    Point::new(240, 80),
                    Point::new(240, 120),
                    Point::new(160, 120),
                ],
                closed: true,
            }],
            ..SymbolDocument::default()
        };
        let body_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(168.0, 102.0));

        let viewport = preview_viewport_for_tile(body_rect, &document);
        let (min, max) = document_bounds(&document);
        let min_screen = viewport.world_to_screen(min - document.origin);
        let max_screen = viewport.world_to_screen(max - document.origin);

        assert!(
            body_rect.shrink(10.0).contains(min_screen),
            "effective min={min_screen:?}"
        );
        assert!(
            body_rect.shrink(10.0).contains(max_screen),
            "effective max={max_screen:?}"
        );
    }

    #[test]
    fn preview_tile_uses_larger_size_when_canvas_allows() {
        let canvas = Rect::from_min_size(pos2(0.0, 0.0), vec2(960.0, 640.0));

        let tile = preview_tile_rect(canvas);

        assert!(
            tile.width() >= 220.0 && tile.height() >= 156.0,
            "preview tile should be large enough for readable symbols: {tile:?}"
        );
        assert!(canvas.contains(tile.min));
        assert!(canvas.contains(tile.max));
    }

    #[test]
    fn symbol_canvas_accessibility_label_reports_editing_contract() {
        let mut state = AppState::default();
        state.ui.symbol.tool = SymbolTool::Circle;
        state.ui.symbol.select_shape(0);
        let document = SymbolDocument {
            pins: vec![SymbolPin::new(
                "OUT",
                PortDirection::Out,
                Some(Point::new(20, 0)),
            )],
            body: vec![SymbolShape::Circle {
                center: Point::origin(),
                radius: 10,
            }],
            ..SymbolDocument::default()
        };

        let label = symbol_canvas_accessibility_label(
            &document,
            &state,
            false,
            crate::workbench::commands::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        );

        assert!(label.starts_with(
            "Symbol editor canvas. 1 shape; 1 pin, 1 pin placed; 1 item selected. Active tool: Circle. Editable."
        ));
        assert!(label.contains("P: Place symbol pin"));
        assert!(label.contains("Escape: Cancel active command"));
    }
}
