//! Symbol cellview navigator.
//!
//! A symbol is a design document, so its structure lives in the shared left
//! panel exactly as a schematic's hierarchy does. It is three pick lists:
//! the ordered pins, the body geometry, and the label anchors.

use egui::{ScrollArea, Ui};

use crate::state::{
    PortDirection, PortSpec, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata,
    SymbolElectricalType, SymbolPin, SymbolPinSide, SymbolShape,
};
use crate::ui::widgets::TreeRow;
use crate::workbench::RSpiceApp;

use super::super::super::design_system::{Problem, WorkbenchIcon, section_header};

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let ports = app.state.active_symbol_ports();
    // A document that will not load says why. The navigator must not invent
    // a structure for one it cannot read.
    let mut document = match app.state.load_active_symbol_document() {
        Ok(document) => document,
        Err(reason) => {
            super::empty_navigator_row(ui, &format!("Symbol file is corrupt: {reason}"));
            return;
        }
    };
    let metadata = match app.state.load_active_symbol_editor_metadata(&document) {
        Ok(metadata) => metadata,
        Err(reason) => {
            super::empty_navigator_row(ui, &format!("Symbol layout data is corrupt: {reason}"));
            return;
        }
    };

    // No path line: the document tab names the view, and a symbol has no
    // hierarchy to ascend, so the line carried no click either.
    ScrollArea::vertical()
        .id_salt("workbench.symbol.navigator")
        .show(ui, |ui| {
            pins_section(ui, app, &mut document, &metadata, &ports);
            body_section(ui, app, &document);
            attributes_section(ui, app, &metadata);
        });
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
    // The interface the symbol answers to is stated here, on the list of
    // pins that would have to change, rather than in a section of its own
    // restating the inspector's pin contract.
    let mismatched = !ports.is_empty()
        && !matches!(document.pin_summary(ports), crate::state::PinSummary::Match);
    let count = document.pins.len();
    section_header(
        ui,
        "Pins (netlist order)",
        Some(&if mismatched {
            format!("{count} · mismatch")
        } else {
            count.to_string()
        }),
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
                Ok(()) => app.state.ui.symbol.editor.select_pin(name),
                Err(error) => app
                    .state
                    .push_user_message(crate::diagnostics::ConsoleMessage::warning(error)),
            }
        }
    });
    // The contract is the authority on which pins exist; reconciling against
    // it belongs beside the list it rewrites, not in a menu two surfaces
    // away. It is offered only when there is an interface to reconcile with.
    let contract_command =
        crate::workbench::commands::vocabulary::Command::SymbolUpdatePinsFromContract;
    if contract_command.availability(app).is_available() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            let width = (ui.available_width() - 8.0).max(1.0);
            if super::super::super::design_system::labeled_icon_button(
                ui,
                WorkbenchIcon::Refresh,
                "Update pins from schematic",
                false,
                width,
            )
            .clicked()
            {
                contract_command.execute(app);
                // The command rewrote the stored view; the rows below must
                // list what it produced rather than the pins it replaced.
                if let Ok(refreshed) = app.state.load_active_symbol_document() {
                    *document = refreshed;
                }
            }
        });
    }
    ui.add_space(4.0);
    if document.pins.is_empty() && ports.is_empty() {
        // The two buttons that fill the list sit directly above this line.
        super::empty_navigator_row(ui, "No pins.");
        return;
    }
    let mut select: Option<String> = None;
    for (index, pin) in document.pins.iter().enumerate() {
        let (meta, problem) = pin_meta(pin, ports);
        let label = format!("{}. {}", index + 1, pin.name);
        let row = TreeRow::new(&label)
            .mono()
            .indent(1)
            .meta(&meta)
            .problem(problem)
            .selected(
                app.state
                    .ui
                    .symbol
                    .editor
                    .selection
                    .pins
                    .contains(&pin.name),
            )
            .show(ui);
        if row.response.clicked() {
            select = Some(pin.name.clone());
        }
    }
    // The other half of a mismatch: a port the schematic declares that this
    // symbol places no pin for. It is invisible in a list of the pins that do
    // exist, and this list is the only one that holds the contract, so the
    // missing ones are listed here rather than in a second section beside it.
    // They carry no index: a pin that does not exist has no netlist position.
    for port in ports_without_pins(document, ports) {
        TreeRow::new(&port.name)
            .mono()
            .indent(1)
            .meta("no pin")
            .problem(Some(Problem::Warning))
            .show(ui);
    }
    if let Some(name) = select {
        app.state.ui.symbol.editor.select_pin(name);
    }
}

/// The declared ports this symbol places no pin for, in interface order.
fn ports_without_pins<'p>(document: &SymbolDocument, ports: &'p [PortSpec]) -> Vec<&'p PortSpec> {
    ports
        .iter()
        .filter(|port| {
            !document
                .pins
                .iter()
                .any(|pin| pin.name.eq_ignore_ascii_case(&port.name))
        })
        .collect()
}

/// What one pin row says after its name, and whether that is a problem.
///
/// A pin is normal when it is placed on the grid and the interface declares
/// it. With no schematic, no Verilog-A module and no generated ports there
/// is no interface for a pin to be missing from, so a hand-drawn symbol's
/// pins are not all of them orphans — which is how every one of them used to
/// read.
fn pin_meta(pin: &SymbolPin, ports: &[PortSpec]) -> (String, Option<Problem>) {
    let declared = ports.is_empty()
        || ports
            .iter()
            .any(|port| port.name.eq_ignore_ascii_case(&pin.name));
    match (pin.position.is_some(), declared) {
        (false, _) => ("unplaced".to_owned(), Some(Problem::Warning)),
        (true, false) => ("not in schematic".to_owned(), Some(Problem::Warning)),
        (true, true) if !pin.terminal_on_grid() => ("off grid".to_owned(), Some(Problem::Warning)),
        (true, true) => (pin.direction.keyword().to_owned(), None),
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

/// What a body shape is, for a list whose job is picking one out from under
/// another. Point counts, radii and sweeps are geometry the inspector edits.
fn shape_label(shape: &SymbolShape) -> String {
    match shape {
        SymbolShape::Polyline { closed, .. } => {
            if *closed { "polygon" } else { "polyline" }.to_owned()
        }
        SymbolShape::Circle { .. } => "circle".to_owned(),
        SymbolShape::Arc { .. } => "arc".to_owned(),
        SymbolShape::Arrow { .. } => "arrow".to_owned(),
        SymbolShape::Dot { .. } => "dot".to_owned(),
        SymbolShape::Text { text, .. } => {
            let label = text.trim();
            if label.is_empty() {
                "text".to_owned()
            } else {
                format!("text {label}")
            }
        }
    }
}

fn body_section(ui: &mut Ui, app: &mut RSpiceApp, document: &SymbolDocument) {
    // A new symbol's empty list says so with the "0" on its header.
    section_header(ui, "Body geometry", Some(&document.body.len().to_string()));
    let mut select: Option<usize> = None;
    for (index, shape) in document.body.iter().enumerate() {
        let row = TreeRow::new(&shape_label(shape))
            .indent(1)
            .selected(app.state.ui.symbol.editor.selection.shapes.contains(&index))
            .show(ui);
        if row.response.clicked() {
            select = Some(index);
        }
    }
    if let Some(index) = select {
        app.state.ui.symbol.editor.select_shape(index);
    }
}

fn attributes_section(ui: &mut Ui, app: &mut RSpiceApp, metadata: &SymbolEditorMetadata) {
    // No count: the list is the three keys `SymbolAttributeKind::ALL` holds,
    // in every symbol there is.
    section_header(ui, "Attributes", None);
    for kind in SymbolAttributeKind::ALL {
        let Some(attribute) = metadata.attribute(kind) else {
            continue;
        };
        // A label that is shown and carries a value is the normal case.
        let meta = if !attribute.shown {
            "hidden"
        } else if attribute.default_value.is_empty() {
            "empty"
        } else {
            ""
        };
        let row = TreeRow::new(kind.key())
            .mono()
            .indent(1)
            .meta(meta)
            .selected(
                app.state
                    .ui
                    .symbol
                    .editor
                    .selection
                    .attributes
                    .contains(&kind),
            )
            .show(ui);
        if row.response.clicked() {
            app.state.ui.symbol.editor.select_attribute(kind);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    /// A pin the interface does not declare is flagged — but only when there
    /// is an interface. A hand-drawn symbol whose cell has no schematic, no
    /// Verilog-A module and no generated ports read every one of its pins as
    /// "orphaned", in warning tone, with nothing the reader could do.
    #[test]
    fn a_symbol_with_no_interface_does_not_orphan_its_own_pins() {
        let on_grid = SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0)));

        assert_eq!(pin_meta(&on_grid, &[]), ("in".to_owned(), None));

        let declared = [PortSpec {
            name: "in".to_owned(),
            direction: PortDirection::In,
        }];
        assert_eq!(pin_meta(&on_grid, &declared), ("in".to_owned(), None));

        let other = [PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        }];
        assert_eq!(
            pin_meta(&on_grid, &other),
            ("not in schematic".to_owned(), Some(Problem::Warning))
        );

        let unplaced = SymbolPin::new("IN", PortDirection::In, None);
        assert_eq!(
            pin_meta(&unplaced, &[]),
            ("unplaced".to_owned(), Some(Problem::Warning))
        );

        let off_grid = SymbolPin::new("IN", PortDirection::In, Some(Point::new(-43, 0)));
        assert_eq!(
            pin_meta(&off_grid, &[]),
            ("off grid".to_owned(), Some(Problem::Warning))
        );
    }

    /// A port the interface declares and the symbol never placed is listed
    /// here, because this list is the only statement of the pin contract: the
    /// inspector's second copy of it is gone, and a reader who cannot see the
    /// missing port cannot see half of a mismatch.
    #[test]
    fn the_pin_list_names_a_declared_port_with_no_pin() {
        let mut document = SymbolDocument {
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(-40, 0)),
            )],
            ..SymbolDocument::default()
        };
        let declared = [
            PortSpec {
                name: "in".to_owned(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "OUT".to_owned(),
                direction: PortDirection::Out,
            },
        ];

        let missing = ports_without_pins(&document, &declared);

        assert_eq!(missing.len(), 1, "{missing:?}");
        assert_eq!(missing[0].name, "OUT");

        document
            .pins
            .push(SymbolPin::new("out", PortDirection::Out, None));
        assert!(
            ports_without_pins(&document, &declared).is_empty(),
            "a pin that exists but is unplaced is a pin, not a missing port"
        );
    }

    /// The body list is a pick list: a shape row says what the shape is, and
    /// the text one carries its words. Point counts, radii and sweeps are the
    /// inspector's editable geometry.
    #[test]
    fn a_body_row_names_the_shape_and_the_text_it_carries() {
        assert_eq!(
            shape_label(&SymbolShape::Polyline {
                points: vec![Point::origin(), Point::new(10, 0), Point::new(10, 10)],
                closed: true,
            }),
            "polygon"
        );
        assert_eq!(
            shape_label(&SymbolShape::Text {
                anchor: Point::new(120, -40),
                text: "VDD".to_owned(),
                size: rspice_design::symbol::SymbolTextSize::Normal,
                align: rspice_design::symbol::SymbolTextAlign::Center,
            }),
            "text VDD"
        );
    }
}
