//! Symbol cellview navigator.
//!
//! A symbol is a design document, so its structure lives in the shared left
//! panel exactly as a schematic's hierarchy does: the ordered pin contract,
//! the body geometry, the label anchors, and the interface the symbol is
//! answerable to.

use egui::{ScrollArea, Ui};

use crate::workbench::RSpiceApp;
use crate::state::{
    PortDirection, PortSpec, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata,
    SymbolElectricalType, SymbolPin, SymbolPinSide, SymbolShape,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::TreeRow;

use super::super::super::design_system::{WorkbenchIcon, property_row, section_header};

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let reference = app.state.workspace.active_view.clone();
    let ports = app.state.active_symbol_ports();
    let Ok(mut document) = app.state.load_active_symbol_document() else {
        // The document could not be read. The editor surface reports the
        // exact reason; the navigator must not invent a structure for it.
        super::empty_navigator_row(ui, "This symbol cellview could not be read");
        return;
    };
    let Ok(metadata) = app.state.load_active_symbol_editor_metadata(&document) else {
        super::empty_navigator_row(ui, "This symbol's authoring metadata could not be read");
        return;
    };

    hierarchy_path(ui, &reference);
    ScrollArea::vertical()
        .id_salt("workbench.symbol.navigator")
        .show(ui, |ui| {
            pins_section(ui, app, &mut document, &metadata, &ports);
            body_section(ui, app, &document);
            attributes_section(ui, app, &metadata);
            texts_section(ui, app, &metadata);
            interface_section(ui, app, &document, &ports);
        });
}

fn hierarchy_path(ui: &mut Ui, reference: &crate::state::CellViewRef) {
    let t = Tokens::get(ui.ctx());
    let frame = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!(
                        "/ {} / {} / {}",
                        reference.library, reference.cell, reference.view
                    ))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// The ordered pin contract. Row order **is** netlist order, so the index is
/// shown rather than implied.
fn pins_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &mut SymbolDocument,
    metadata: &SymbolEditorMetadata,
    ports: &[PortSpec],
) {
    section_header(
        ui,
        "Pins · netlist order",
        Some(&document.pins.len().to_string()),
    );
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let width = (ui.available_width() - 8.0).max(1.0);
        if ui
            .add_enabled_ui(!app.state.active_view_read_only(), |ui| {
                super::super::super::design_system::labeled_icon_button(
                    ui,
                    WorkbenchIcon::Add,
                    "Add pin",
                    false,
                    width,
                )
            })
            .inner
            .clicked()
        {
            let before = document.clone();
            let name = next_pin_name(document);
            document.pins.push(
                SymbolPin::new(&name, PortDirection::InOut, None).with_contract(
                    SymbolElectricalType::Analog,
                    SymbolPinSide::Right,
                    0,
                ),
            );
            app.state.record_symbol_edit(&before);
            match app
                .state
                .store_active_symbol_editor_bundle(document, metadata)
            {
                Ok(()) => app.state.ui.symbol.select_pin(name),
                Err(error) => app
                    .state
                    .push_user_message(crate::workbench::ConsoleMessage::warning(error)),
            }
        }
    });
    ui.add_space(4.0);
    if document.pins.is_empty() {
        super::empty_navigator_row(ui, "No pins. Add one or generate the interface contract.");
        return;
    }
    let mut select: Option<String> = None;
    for (index, pin) in document.pins.iter().enumerate() {
        let declared = ports
            .iter()
            .any(|port| port.name.eq_ignore_ascii_case(&pin.name));
        let placed = pin.position.is_some();
        let meta = match (placed, declared) {
            (false, _) => "unplaced".to_owned(),
            (true, false) => "orphaned".to_owned(),
            (true, true) if !pin.terminal_on_grid() => "off grid".to_owned(),
            (true, true) => pin.direction.keyword().to_owned(),
        };
        let label = format!("{}. {}", index + 1, pin.name);
        let t = Tokens::get(ui.ctx());
        let row = TreeRow::new(&label)
            .mono()
            .indent(1)
            .meta(&meta)
            .chip_dot(if placed && declared && pin.terminal_on_grid() {
                t.color.ok
            } else {
                t.color.err
            })
            .selected(app.state.ui.symbol.selection.pins.contains(&pin.name))
            .show(ui);
        if row.response.clicked() {
            select = Some(pin.name.clone());
        }
    }
    if let Some(name) = select {
        app.state.ui.symbol.select_pin(name);
    }
}

fn next_pin_name(document: &SymbolDocument) -> String {
    (1_u64..)
        .map(|index| format!("PIN{index}"))
        .find(|candidate| {
            document
                .pins
                .iter()
                .all(|pin| !pin.name.eq_ignore_ascii_case(candidate))
        })
        .expect("the symbol pin identity space is not exhausted")
}

fn shape_label(shape: &SymbolShape) -> String {
    match shape {
        SymbolShape::Polyline { points, closed } => format!(
            "{} · {} points",
            if *closed { "polygon" } else { "polyline" },
            points.len()
        ),
        SymbolShape::Circle { radius, .. } => format!("circle · r{radius}"),
        SymbolShape::Arc {
            radius,
            sweep_degrees,
            ..
        } => format!("arc · r{radius} · {sweep_degrees}°"),
        SymbolShape::Arrow { .. } => "arrow".to_owned(),
        SymbolShape::Dot { radius, .. } => format!("dot · r{radius}"),
    }
}

fn shape_anchor(shape: &SymbolShape) -> crate::state::Point {
    match shape {
        SymbolShape::Polyline { points, .. } => points
            .first()
            .copied()
            .unwrap_or_else(crate::state::Point::origin),
        SymbolShape::Circle { center, .. }
        | SymbolShape::Arc { center, .. }
        | SymbolShape::Dot { center, .. } => *center,
        SymbolShape::Arrow { tip, .. } => *tip,
    }
}

fn body_section(ui: &mut Ui, app: &mut RSpiceApp, document: &SymbolDocument) {
    section_header(ui, "Body geometry", Some(&document.body.len().to_string()));
    if document.body.is_empty() {
        super::empty_navigator_row(
            ui,
            "No body shapes. Draw with the line, rectangle, circle, arc or polygon tools.",
        );
        return;
    }
    let mut select: Option<usize> = None;
    for (index, shape) in document.body.iter().enumerate() {
        let anchor = shape_anchor(shape);
        let meta = format!("{}, {}", anchor.x, anchor.y);
        let row = TreeRow::new(&shape_label(shape))
            .indent(1)
            .meta(&meta)
            .selected(app.state.ui.symbol.selection.shapes.contains(&index))
            .show(ui);
        if row.response.clicked() {
            select = Some(index);
        }
    }
    if let Some(index) = select {
        app.state.ui.symbol.select_shape(index);
    }
}

fn attributes_section(ui: &mut Ui, app: &mut RSpiceApp, metadata: &SymbolEditorMetadata) {
    section_header(
        ui,
        "Attributes",
        Some(&metadata.attributes.len().to_string()),
    );
    for kind in SymbolAttributeKind::ALL {
        let Some(attribute) = metadata.attribute(kind) else {
            continue;
        };
        let meta = if !attribute.shown {
            "hidden"
        } else if attribute.default_value.is_empty() {
            "empty"
        } else {
            "shown"
        };
        let row = TreeRow::new(kind.key())
            .mono()
            .indent(1)
            .meta(meta)
            .selected(app.state.ui.symbol.selection.attributes.contains(&kind))
            .show(ui);
        if row.response.clicked() {
            app.state.ui.symbol.select_attribute(kind);
        }
    }
}

fn texts_section(ui: &mut Ui, app: &mut RSpiceApp, metadata: &SymbolEditorMetadata) {
    section_header(ui, "Text", Some(&metadata.texts.len().to_string()));
    if metadata.texts.is_empty() {
        super::empty_navigator_row(ui, "No free text objects");
        return;
    }
    for text in &metadata.texts {
        let label = if text.text.trim().is_empty() {
            "Untitled text"
        } else {
            text.text.trim()
        };
        let row = TreeRow::new(label)
            .indent(1)
            .meta(if text.shown { "shown" } else { "hidden" })
            .selected(app.state.ui.symbol.selection.texts.contains(&text.id))
            .show(ui);
        if row.response.clicked() {
            app.state.ui.symbol.select_text(text.id);
        }
    }
}

/// The interface the symbol answers to, and whether it currently does.
fn interface_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &SymbolDocument,
    ports: &[PortSpec],
) {
    let summary = document.pin_summary(ports);
    let matched = matches!(summary, crate::state::PinSummary::Match);
    section_header(
        ui,
        "Bound interface",
        Some(if matched { "matched" } else { "mismatch" }),
    );
    property_row(
        ui,
        "Declared ports",
        &if ports.is_empty() {
            "no schematic interface".to_owned()
        } else {
            ports.len().to_string()
        },
    );
    property_row(
        ui,
        "Port order",
        &if ports.is_empty() {
            "—".to_owned()
        } else {
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        },
    );
    property_row(
        ui,
        "Cell",
        &format!(
            "{}/{}",
            app.state.workspace.active_view.library, app.state.workspace.active_view.cell
        ),
    );

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        let width = (ui.available_width() - 10.0).max(1.0);
        if crate::workbench::design_system::labeled_icon_button(
            ui,
            WorkbenchIcon::Models,
            "Open in model manager",
            false,
            width,
        )
        .clicked()
        {
            crate::workbench::commands::Command::OpenWorkspace(
                crate::workbench::state::Workspace::Models,
            )
            .execute(app);
        }
    });
    ui.add_space(2.0);
}
