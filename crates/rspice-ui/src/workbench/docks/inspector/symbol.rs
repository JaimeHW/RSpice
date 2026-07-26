//! Symbol cellview inspector.
//!
//! The right panel edits whatever the symbol editor has selected and,
//! beneath it, always shows the pin contract: the ordered comparison of the
//! ports the schematic interface declares against the pins this symbol
//! places. That table is the reason a symbol can be wrong, so it is never
//! hidden behind a selection.

use egui::Ui;

use crate::common::{AppState, RSpiceApp};
use crate::state::{PinSummary, Point, PortSpec, SymbolDocument, SymbolShape};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::TreeRow;
use crate::workbench::design_system::{
    StatusMark, property_row, property_row_input, property_row_status,
};

use super::{muted_inspector_copy, section_header};

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let ports = app.state.active_symbol_ports();
    let Ok(mut document) = app.state.load_active_symbol_document() else {
        muted_inspector_copy(
            ui,
            "This symbol cellview could not be read. The editor surface reports the exact reason.",
        );
        return;
    };
    document.reconcile_ports(&ports);

    hero(ui, app, &document, &ports);

    let mut changed = false;
    let selection = app.state.ui.symbol.effective_selection();
    match (selection.pins.len(), selection.shapes.len()) {
        (1, 0) => {
            let name = selection
                .pins
                .iter()
                .next()
                .cloned()
                .expect("one selected pin");
            changed |= pin_section(ui, app, &mut document, &ports, &name);
        }
        (0, 1) => {
            let index = selection
                .shapes
                .iter()
                .next()
                .copied()
                .expect("one selected shape");
            changed |= shape_section(ui, app, &mut document, index);
        }
        (0, 0) => empty_selection_section(ui),
        (pins, shapes) => multi_selection_section(ui, pins, shapes),
    }

    contract_section(ui, app, &document, &ports);

    if changed && let Err(error) = app.state.store_active_symbol_document(&document) {
        app.state
            .push_user_message(crate::common::ConsoleMessage::warning(error));
    }
}

// =============================================================================
// Hero
// =============================================================================

fn hero(ui: &mut Ui, app: &mut RSpiceApp, document: &SymbolDocument, ports: &[PortSpec]) {
    let t = Tokens::get(ui.ctx());
    let reference = &app.state.workspace.active_view;
    let summary = document.pin_summary(ports);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 82.0),
        egui::Sense::hover(),
    );
    let preview = egui::Rect::from_min_max(
        rect.min,
        egui::pos2((rect.left() + 82.0).min(rect.right()), rect.bottom()),
    );
    ui.painter().rect_filled(preview, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        preview.right(),
        preview.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    crate::schematic::symbol_editor::draw_document_preview(
        ui.painter(),
        preview.shrink(14.0),
        document,
        ports,
        &reference.cell,
        t.color.symbol,
    );

    let text_left = preview.right() + 10.0;
    let painter = ui
        .painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            text_left..=(rect.right() - 10.0),
            rect.y_range(),
        ));
    let at = |y: f32| egui::pos2(text_left, rect.top() + y);
    painter.text(
        at(12.0),
        egui::Align2::LEFT_CENTER,
        format!(
            "{} / {} · {}",
            reference.library,
            reference.cell,
            reference.view.to_ascii_uppercase()
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    painter.text(
        at(31.0),
        egui::Align2::LEFT_CENTER,
        &reference.cell,
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        t.color.text,
    );
    painter.text(
        at(49.0),
        egui::Align2::LEFT_CENTER,
        format!(
            "{} pins · {} body shapes",
            document.pins.len(),
            document.body.len()
        ),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let (status, tone) = summary_status(&t, summary);
    painter.text(
        at(68.0),
        egui::Align2::LEFT_CENTER,
        status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tone,
    );
}

fn summary_status(t: &Tokens, summary: PinSummary) -> (String, egui::Color32) {
    match summary {
        PinSummary::Match => ("pin contract matches the interface".to_owned(), t.color.ok),
        PinSummary::Unplaced(count) => (
            format!("{count} declared pin{} unplaced", plural(count)),
            t.color.err,
        ),
        PinSummary::Orphaned(count) => (
            format!("{count} pin{} not declared by the interface", plural(count)),
            t.color.err,
        ),
        PinSummary::NoSchematic => (
            "no schematic interface declares this cell".to_owned(),
            t.color.warn,
        ),
    }
}

const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

// =============================================================================
// Selection sections
// =============================================================================

/// Record one undo snapshot for the open inspector field, however many
/// keystrokes it receives.
fn record_once(state: &mut AppState, before: &SymbolDocument) {
    if state.ui.symbol.inspector_undo_recorded {
        return;
    }
    state.record_symbol_edit(before);
    state.ui.symbol.inspector_undo_recorded = true;
}

/// An editable integer coordinate. Illegal text is simply not applied; the
/// row keeps showing the value the document holds.
fn coordinate_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    before: &SymbolDocument,
    label: &str,
    value: &mut i32,
) -> bool {
    let id = ui.id().with(("symbol-coordinate", label));
    let mut buffer = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| value.to_string());
    let parsed = buffer.trim().parse::<i32>();
    let response = property_row_input(ui, label, &mut buffer, parsed.is_err());
    let mut changed = false;
    if response.changed() {
        ui.data_mut(|data| data.insert_temp(id, buffer.clone()));
        if let Ok(parsed) = buffer.trim().parse::<i32>()
            && parsed != *value
        {
            record_once(&mut app.state, before);
            *value = parsed;
            changed = true;
        }
    }
    if !response.has_focus() {
        // The row shows the document's value again the moment the field is
        // not being typed into, and the next session starts a new undo step.
        ui.data_mut(|data| data.remove_temp::<String>(id));
        app.state.ui.symbol.inspector_undo_recorded = false;
    }
    changed
}

/// The selected pin.
///
/// A pin's name and electrical type are declared by the schematic
/// interface and reconciled into the symbol on every load, so they are
/// reported here rather than offered for editing — the symbol owns the
/// pin's placement, and nothing else.
fn pin_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &mut SymbolDocument,
    ports: &[PortSpec],
    name: &str,
) -> bool {
    let Some(order) = document
        .pins
        .iter()
        .position(|pin| pin.name.eq_ignore_ascii_case(name))
    else {
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    let total = document.pins.len();
    let declared = ports
        .iter()
        .any(|port| port.name.eq_ignore_ascii_case(name));
    let pin = &document.pins[order];
    let direction = pin.direction;
    let placed = pin.position;
    let on_grid = pin.terminal_on_grid();

    section_header(
        ui,
        "Selected pin",
        Some(&format!("port {} / {total}", order + 1)),
    );
    property_row(ui, "Name", name);
    property_row(ui, "Electrical type", direction.keyword());
    property_row(ui, "Netlist order", &format!("{} of {total}", order + 1));
    property_row_status(
        ui,
        "Declared by interface",
        if declared { "yes" } else { "orphaned" },
        if declared {
            Tokens::get(ui.ctx()).color.ok
        } else {
            Tokens::get(ui.ctx()).color.err
        },
        if declared {
            StatusMark::Success
        } else {
            StatusMark::Warning
        },
    );

    let mut changed = false;
    match placed {
        Some(mut point) => {
            changed |= coordinate_row(ui, app, &before, "Terminal X", &mut point.x);
            changed |= coordinate_row(ui, app, &before, "Terminal Y", &mut point.y);
            if changed {
                document.pins[order].position = Some(point);
            }
            property_row_status(
                ui,
                "Terminal grid",
                if on_grid { "on grid" } else { "off grid" },
                if on_grid {
                    Tokens::get(ui.ctx()).color.ok
                } else {
                    Tokens::get(ui.ctx()).color.err
                },
                if on_grid {
                    StatusMark::Success
                } else {
                    StatusMark::Warning
                },
            );
        }
        None => {
            property_row_status(
                ui,
                "Terminal",
                "unplaced",
                Tokens::get(ui.ctx()).color.err,
                StatusMark::Warning,
            );
            muted_inspector_copy(
                ui,
                "Choose the pin tool and click the canvas to place this terminal.",
            );
        }
    }
    changed
}

fn shape_kind(shape: &SymbolShape) -> &'static str {
    match shape {
        SymbolShape::Polyline { closed: true, .. } => "polygon",
        SymbolShape::Polyline { .. } => "polyline",
        SymbolShape::Circle { .. } => "circle",
        SymbolShape::Arc { .. } => "arc",
        SymbolShape::Arrow { .. } => "arrow",
        SymbolShape::Dot { .. } => "dot",
    }
}

/// The selected body shape. Body geometry is cosmetic: it never changes the
/// port contract, which the section says outright.
fn shape_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &mut SymbolDocument,
    index: usize,
) -> bool {
    let Some(shape) = document.body.get(index).cloned() else {
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    section_header(ui, "Selected shape", Some(shape_kind(&shape)));

    let mut changed = false;
    let mut edited = shape;
    match &mut edited {
        SymbolShape::Polyline { points, closed } => {
            property_row(ui, "Type", if *closed { "closed" } else { "open" });
            property_row(ui, "Points", &points.len().to_string());
            if let Some(first) = points.first_mut() {
                let mut anchor = *first;
                let moved = coordinate_row(ui, app, &before, "Start X", &mut anchor.x)
                    | coordinate_row(ui, app, &before, "Start Y", &mut anchor.y);
                if moved {
                    let delta = Point::new(anchor.x - first.x, anchor.y - first.y);
                    for point in points.iter_mut() {
                        *point = *point + delta;
                    }
                    changed = true;
                }
            }
        }
        SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
            changed |= coordinate_row(ui, app, &before, "Center X", &mut center.x);
            changed |= coordinate_row(ui, app, &before, "Center Y", &mut center.y);
            changed |= coordinate_row(ui, app, &before, "Radius", radius);
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
        } => {
            changed |= coordinate_row(ui, app, &before, "Center X", &mut center.x);
            changed |= coordinate_row(ui, app, &before, "Center Y", &mut center.y);
            changed |= coordinate_row(ui, app, &before, "Radius", radius);
            changed |= coordinate_row(ui, app, &before, "Start °", start_degrees);
            changed |= coordinate_row(ui, app, &before, "Sweep °", sweep_degrees);
        }
        SymbolShape::Arrow {
            tip,
            rotation_quarters,
        } => {
            changed |= coordinate_row(ui, app, &before, "Tip X", &mut tip.x);
            changed |= coordinate_row(ui, app, &before, "Tip Y", &mut tip.y);
            changed |= coordinate_row(ui, app, &before, "Quarter turns", rotation_quarters);
        }
    }
    if changed {
        document.body[index] = edited;
    }
    muted_inspector_copy(
        ui,
        "Body geometry is cosmetic; it never changes the port contract.",
    );
    changed
}

fn empty_selection_section(ui: &mut Ui) {
    section_header(ui, "Selection", Some("none"));
    muted_inspector_copy(
        ui,
        "Select a pin or body shape on the canvas, or in the Symbol editor panel, to edit it here.",
    );
}

fn multi_selection_section(ui: &mut Ui, pins: usize, shapes: usize) {
    section_header(ui, "Selection", Some(&(pins + shapes).to_string()));
    property_row(ui, "Pins", &pins.to_string());
    property_row(ui, "Body shapes", &shapes.to_string());
    muted_inspector_copy(
        ui,
        "Reduce the selection to one object to edit its geometry. Canvas transforms apply to the whole selection.",
    );
}

// =============================================================================
// Pin contract
// =============================================================================

/// The ordered comparison the symbol is answerable to: for each declared
/// port, the pin this symbol places for it.
fn contract_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &SymbolDocument,
    ports: &[PortSpec],
) {
    let summary = document.pin_summary(ports);
    section_header(
        ui,
        "Pin contract · netlist order",
        Some(match summary {
            PinSummary::Match => "matches",
            PinSummary::NoSchematic => "no interface",
            _ => "mismatch",
        }),
    );
    if ports.is_empty() {
        muted_inspector_copy(
            ui,
            "No schematic in this project declares an interface for this cell, so there is no contract to satisfy.",
        );
        return;
    }

    let t = Tokens::get(ui.ctx());
    let mut select: Option<String> = None;
    for (index, port) in ports.iter().enumerate() {
        let pin = document
            .pins
            .iter()
            .find(|pin| pin.name.eq_ignore_ascii_case(&port.name));
        let placed = pin.is_some_and(|pin| pin.position.is_some());
        let meta = match pin {
            Some(pin) => match pin.position {
                Some(point) => format!("{}, {}", point.x, point.y),
                None => "unplaced".to_owned(),
            },
            None => "no pin".to_owned(),
        };
        let label = format!("{}. {}", index + 1, port.name);
        let row = TreeRow::new(&label)
            .mono()
            .indent(1)
            .meta(&meta)
            .chip_dot(if placed { t.color.ok } else { t.color.err })
            .selected(app.state.ui.symbol.selection.pins.contains(&port.name))
            .show(ui);
        if row.response.clicked() && pin.is_some() {
            select = Some(port.name.clone());
        }
    }
    if let Some(name) = select {
        app.state.ui.symbol.select_pin(name);
    }

    // Pins the symbol places that no port declares are the other half of a
    // mismatch, and are invisible in a port-ordered list.
    let orphaned: Vec<&crate::state::SymbolPin> = document
        .pins
        .iter()
        .filter(|pin| {
            !ports
                .iter()
                .any(|port| port.name.eq_ignore_ascii_case(&pin.name))
        })
        .collect();
    if !orphaned.is_empty() {
        section_header(ui, "Undeclared pins", Some(&orphaned.len().to_string()));
        for pin in orphaned {
            property_row_status(
                ui,
                &pin.name,
                "not declared by the interface",
                t.color.err,
                StatusMark::Warning,
            );
        }
    }
}
