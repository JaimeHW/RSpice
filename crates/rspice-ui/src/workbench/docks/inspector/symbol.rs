//! Symbol cellview inspector.
//!
//! The right panel edits whatever the symbol editor has selected and,
//! beneath it, always shows the pin contract: the ordered comparison of the
//! ports the schematic interface declares against the pins this symbol
//! places. That table is the reason a symbol can be wrong, so it is never
//! hidden behind a selection.

use egui::Ui;

use crate::state::{
    PinSummary, Point, PortSpec, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata,
    SymbolPinElectricalKind, SymbolPinSide, SymbolShape, SymbolTextAlign, SymbolTextSize,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::TreeRow;
use crate::workbench::design_system::{
    StatusMark, WorkbenchIcon, labeled_icon_button, property_row, property_row_combo,
    property_row_input, property_row_status,
};
use crate::workbench::{AppState, RSpiceApp};

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
    let Ok(mut metadata) = app.state.load_active_symbol_editor_metadata(&document) else {
        muted_inspector_copy(
            ui,
            "This symbol's authoring metadata could not be read. The editor surface reports the exact reason.",
        );
        return;
    };

    hero(ui, app, &document, &ports);

    let mut changed = false;
    let selection = app.state.ui.symbol.effective_selection();
    let total = selection.pins.len() + selection.shapes.len() + selection.attributes.len();
    if total == 0 {
        empty_selection_section(ui);
    } else if total > 1 {
        multi_selection_section(
            ui,
            selection.pins.len(),
            selection.shapes.len(),
            selection.attributes.len(),
        );
    } else if let Some(name) = selection.pins.iter().next() {
        changed |= pin_section(ui, app, &mut document, &ports, name);
    } else if let Some(index) = selection.shapes.iter().next() {
        changed |= shape_section(ui, app, &mut document, *index);
    } else if let Some(kind) = selection.attributes.iter().next() {
        changed |= attribute_section(ui, app, &mut document, &mut metadata, *kind);
    }

    contract_section(ui, app, &document, &ports);
    definition_section(ui, app, &metadata);

    if changed
        && let Err(error) = app
            .state
            .store_active_symbol_editor_bundle(&document, &metadata)
    {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}

// =============================================================================
// Hero
// =============================================================================

fn hero(ui: &mut Ui, app: &mut RSpiceApp, document: &SymbolDocument, ports: &[PortSpec]) {
    let t = Tokens::get(ui.ctx());
    let reference = &app.state.workspace.active_view;
    let summary = document.pin_summary(ports);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 82.0), egui::Sense::hover());
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
    let painter = ui.painter().with_clip_rect(egui::Rect::from_x_y_ranges(
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
    if response.lost_focus() {
        // The row shows the document's value again the moment the field is
        // not being typed into, and the next session starts a new undo step.
        ui.data_mut(|data| data.remove_temp::<String>(id));
        app.state.ui.symbol.inspector_undo_recorded = false;
    }
    changed
}

fn text_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    before: &SymbolDocument,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    value: &str,
    is_valid: impl Fn(&str) -> bool,
) -> Option<String> {
    let id = ui.id().with(("symbol-text", id_salt));
    let mut buffer = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| value.to_owned());
    let valid = is_valid(buffer.trim());
    let response = property_row_input(ui, label, &mut buffer, !valid);
    let mut accepted = None;
    if response.changed() {
        ui.data_mut(|data| data.insert_temp(id, buffer.clone()));
        let trimmed = buffer.trim();
        if valid && trimmed != value {
            record_once(&mut app.state, before);
            accepted = Some(trimmed.to_owned());
        }
    }
    if response.lost_focus() {
        ui.data_mut(|data| data.remove_temp::<String>(id));
        app.state.ui.symbol.inspector_undo_recorded = false;
    }
    accepted
}

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
    let current_name = document.pins[order].name.clone();

    section_header(
        ui,
        "Selected pin",
        Some(&format!("port {} / {total}", order + 1)),
    );

    let mut changed = false;
    if let Some(new_name) =
        text_row(
            ui,
            app,
            &before,
            ("pin-name", order),
            "Name",
            &current_name,
            |candidate| {
                !candidate.is_empty()
                    && candidate.len() <= crate::state::MAX_SYMBOL_PIN_NAME_BYTES
                    && !candidate.chars().any(char::is_control)
                    && document.pins.iter().enumerate().all(|(index, pin)| {
                        index == order || !pin.name.eq_ignore_ascii_case(candidate)
                    })
            },
        )
    {
        document.pins[order].name = new_name.clone();
        app.state.ui.symbol.select_pin(new_name);
        changed = true;
    }

    let mut electrical = SymbolPinElectricalKind::from_pin(&document.pins[order])
        .label()
        .to_owned();
    let electrical_options = SymbolPinElectricalKind::ALL
        .into_iter()
        .map(|kind| (kind.label().to_owned(), kind.label().to_owned()))
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        "Electrical type",
        ("symbol-pin-electrical", order),
        &mut electrical,
        &electrical_options,
        true,
    ) && let Some(kind) = SymbolPinElectricalKind::ALL
        .into_iter()
        .find(|kind| kind.label() == electrical)
    {
        record_once(&mut app.state, &before);
        let (electrical_type, direction) = kind.contract();
        document.pins[order].set_electrical_contract(electrical_type, direction);
        changed = true;
    }

    let mut side = document.pins[order].side();
    let mut side_value = side.label().to_owned();
    let side_options = SymbolPinSide::ALL
        .into_iter()
        .map(|side| (side.label().to_owned(), side.label().to_owned()))
        .collect::<Vec<_>>();
    let side_changed = property_row_combo(
        ui,
        "Side",
        ("symbol-pin-side", order),
        &mut side_value,
        &side_options,
        true,
    );
    if side_changed
        && let Some(new_side) = SymbolPinSide::ALL
            .into_iter()
            .find(|candidate| candidate.label() == side_value)
    {
        record_once(&mut app.state, &before);
        side = new_side;
        changed = true;
    }

    let mut offset = document.pins[order].offset();
    let offset_changed = coordinate_row(ui, app, &before, "Offset", &mut offset);
    if side_changed || offset_changed {
        let bounds = document.body_bounds();
        document.pins[order].set_side_and_offset(side, offset, bounds);
        changed = true;
    }

    property_row(ui, "Netlist order", &format!("{} of {total}", order + 1));
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui
            .add_enabled(order > 0, egui::Button::new("Move earlier"))
            .clicked()
        {
            app.state.record_symbol_edit(&before);
            document.pins.swap(order, order - 1);
            app.state.ui.symbol.inspector_undo_recorded = false;
            changed = true;
        }
        if ui
            .add_enabled(order + 1 < total, egui::Button::new("Move later"))
            .clicked()
        {
            app.state.record_symbol_edit(&before);
            document.pins.swap(order, order + 1);
            app.state.ui.symbol.inspector_undo_recorded = false;
            changed = true;
        }
    });

    let active_name = document.pins[order].name.clone();
    let declared = ports
        .iter()
        .any(|port| port.name.eq_ignore_ascii_case(&active_name));
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
    let placed = document.pins[order].position.is_some();
    let on_grid = document.pins[order].terminal_on_grid();
    property_row_status(
        ui,
        "Terminal",
        if !placed {
            "unplaced"
        } else if on_grid {
            "on grid"
        } else {
            "off grid"
        },
        if placed && on_grid {
            Tokens::get(ui.ctx()).color.ok
        } else {
            Tokens::get(ui.ctx()).color.err
        },
        if placed && on_grid {
            StatusMark::Success
        } else {
            StatusMark::Warning
        },
    );
    if !placed
        && labeled_icon_button(
            ui,
            WorkbenchIcon::Target,
            "Place on selected side",
            false,
            ui.available_width(),
        )
        .clicked()
    {
        app.state.record_symbol_edit(&before);
        let bounds = document.body_bounds();
        document.pins[order].set_side_and_offset(side, offset, bounds);
        app.state.ui.symbol.inspector_undo_recorded = false;
        changed = true;
    }
    ui.add_space(4.0);
    if ui
        .add_sized(
            [ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h],
            egui::Button::new("Remove pin"),
        )
        .clicked()
    {
        app.state.record_symbol_edit(&before);
        document.pins.remove(order);
        app.state.ui.symbol.clear_selection();
        app.state.ui.symbol.inspector_undo_recorded = false;
        changed = true;
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
        SymbolShape::Text { .. } => "text",
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
        SymbolShape::Text {
            anchor,
            text,
            size,
            align,
        } => {
            if let Some(value) = text_row(
                ui,
                app,
                &before,
                ("symbol-body-text", index),
                "Text",
                text,
                |value| {
                    value.len() <= crate::state::MAX_SYMBOL_TEXT_BYTES
                        && !value.chars().any(char::is_control)
                },
            ) {
                *text = value;
                changed = true;
            }
            let mut size_value = size.label().to_owned();
            let size_options = SymbolTextSize::ALL
                .into_iter()
                .map(|option| (option.label().to_owned(), option.label().to_owned()))
                .collect::<Vec<_>>();
            if property_row_combo(
                ui,
                "Size",
                ("symbol-text-size", index),
                &mut size_value,
                &size_options,
                true,
            ) && let Some(picked) = SymbolTextSize::ALL
                .into_iter()
                .find(|option| option.label() == size_value)
            {
                record_once(&mut app.state, &before);
                *size = picked;
                changed = true;
            }
            let mut align_value = align.label().to_owned();
            let align_options = SymbolTextAlign::ALL
                .into_iter()
                .map(|option| (option.label().to_owned(), option.label().to_owned()))
                .collect::<Vec<_>>();
            if property_row_combo(
                ui,
                "Align",
                ("symbol-text-align", index),
                &mut align_value,
                &align_options,
                true,
            ) && let Some(picked) = SymbolTextAlign::ALL
                .into_iter()
                .find(|option| option.label() == align_value)
            {
                record_once(&mut app.state, &before);
                *align = picked;
                changed = true;
            }
            changed |= coordinate_row(ui, app, &before, "Anchor X", &mut anchor.x);
            changed |= coordinate_row(ui, app, &before, "Anchor Y", &mut anchor.y);
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

fn visibility_row(ui: &mut Ui, label: &str, shown: &mut bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let before = *shown;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add_sized(
            [112.0, t.metrics.ctl_h],
            egui::Label::new(
                egui::RichText::new(label)
                    .size(tokens::FS_0)
                    .color(t.color.text_dim),
            ),
        );
        ui.checkbox(shown, if *shown { "Shown" } else { "Hidden" });
    });
    *shown != before
}

fn attribute_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    kind: SymbolAttributeKind,
) -> bool {
    let Some(index) = metadata
        .attributes
        .iter()
        .position(|attribute| attribute.kind == kind)
    else {
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    let mut changed = false;
    section_header(ui, "Selected attribute", Some(kind.key()));

    let current_value = metadata.attributes[index].default_value.clone();
    if let Some(value) = text_row(
        ui,
        app,
        &before,
        ("attribute-value", kind),
        "Default value",
        &current_value,
        |value| {
            value.len() <= crate::state::MAX_SYMBOL_TEXT_BYTES
                && !value.chars().any(char::is_control)
        },
    ) {
        metadata.attributes[index].default_value = value;
        changed = true;
    }

    let mut shown = metadata.attributes[index].shown;
    if visibility_row(ui, "Visibility", &mut shown) {
        record_once(&mut app.state, &before);
        metadata.attributes[index].shown = shown;
        changed = true;
    }

    let mut position = metadata.attributes[index].position;
    let moved = coordinate_row(ui, app, &before, "Position X", &mut position.x)
        | coordinate_row(ui, app, &before, "Position Y", &mut position.y);
    if moved {
        metadata.attributes[index].position = position;
        match kind {
            SymbolAttributeKind::Reference => document.name_anchor = position,
            SymbolAttributeKind::Value => document.value_anchor = position,
            SymbolAttributeKind::Model => {}
        }
        changed = true;
    }
    muted_inspector_copy(
        ui,
        "Attributes are rendered on every placed instance when visibility is enabled.",
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

fn multi_selection_section(ui: &mut Ui, pins: usize, shapes: usize, attributes: usize) {
    section_header(
        ui,
        "Selection",
        Some(&(pins + shapes + attributes).to_string()),
    );
    property_row(ui, "Pins", &pins.to_string());
    property_row(ui, "Body shapes", &shapes.to_string());
    property_row(ui, "Attributes", &attributes.to_string());
    muted_inspector_copy(
        ui,
        "Reduce the selection to one object to edit its geometry. Canvas transforms apply to the whole selection.",
    );
}

fn definition_section(ui: &mut Ui, app: &mut RSpiceApp, metadata: &SymbolEditorMetadata) {
    section_header(ui, "Symbol definition", Some("CDF"));
    property_row(ui, "Published revision", &metadata.revision.to_string());
    property_row(
        ui,
        "Revision note",
        if metadata.revision_note.is_empty() {
            "not published"
        } else {
            &metadata.revision_note
        },
    );
    if labeled_icon_button(
        ui,
        WorkbenchIcon::Sliders,
        "Open CDF / form designer",
        false,
        ui.available_width(),
    )
    .clicked()
    {
        crate::workbench::app::open_symbol_parameter_form_dialog(&mut app.state);
    }
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
