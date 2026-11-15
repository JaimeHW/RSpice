//! The pin table reads and writes pins by the inspector's rules.

use super::*;
use rspice_design::symbol::{SYMBOL_PIN_LENGTH, SymbolPin};
use rspice_design_model::Point;
use rspice_design_model::port::PortDirection;

fn amplifier() -> SymbolDocument {
    SymbolDocument {
        pins: vec![
            SymbolPin::new("IN+", PortDirection::In, Some(Point::new(-40, -10))),
            SymbolPin::new("IN-", PortDirection::In, Some(Point::new(-40, 10))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0)))
                .with_side(SymbolPinSide::Right),
            SymbolPin::new("VDD", PortDirection::Supply, None).with_side(SymbolPinSide::Top),
        ],
        ..SymbolDocument::default()
    }
}

fn names(document: &SymbolDocument) -> Vec<&str> {
    document.pins.iter().map(|pin| pin.name.as_str()).collect()
}

fn pins(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

#[test]
fn every_column_reads_the_field_the_inspector_shows() {
    let document = amplifier();
    let row = |index| {
        PinTableColumn::ALL
            .map(|column| column.value(&document, index))
            .to_vec()
    };
    let lead = SYMBOL_PIN_LENGTH.to_string();
    assert_eq!(
        row(2),
        [
            "OUT",
            "analog output",
            "40",
            "0",
            "right",
            lead.as_str(),
            "plain",
            "shown",
            "3"
        ]
    );
    // An unplaced pin has no coordinates to show.
    assert_eq!(row(3)[2..4], ["", ""]);
    assert_eq!(PinTableColumn::X.number(&document, 3), None);
    assert_eq!(PinTableColumn::NetlistOrder.number(&document, 3), Some(4.0));
    for column in PinTableColumn::ALL {
        assert_eq!(PinTableColumn::from_id(column.id()), Some(column));
    }
}

#[test]
fn a_multi_row_edit_sets_every_selected_pin_as_one_named_step() {
    let mut document = amplifier();
    let edit = set_pin_cells(
        &mut document,
        &pins(&["IN+", "IN-", "OUT"]),
        PinTableColumn::Side,
        "bottom",
    )
    .expect("a side is a side")
    .expect("three pins change side");

    assert_eq!(edit.step, "Set side of 3 pins");
    assert_eq!(edit.pins, pins(&["IN+", "IN-", "OUT"]));
    assert!(edit.renames.is_empty());
    assert!(
        document.pins[..3]
            .iter()
            .all(|pin| pin.side == SymbolPinSide::Bottom)
    );
    assert_eq!(document.pins[3].side, SymbolPinSide::Top, "unselected pin");

    let edit = set_pin_cells(
        &mut document,
        &pins(&["OUT"]),
        PinTableColumn::Style,
        "Clock",
    )
    .expect("a style is a style")
    .expect("one pin changes style");
    assert_eq!(edit.step, "Set style of pin OUT");
    assert_eq!(document.pins[2].style, SymbolPinStyle::Clock);
}

#[test]
fn a_value_every_pin_already_holds_is_no_step() {
    let mut document = amplifier();
    let before = document.clone();
    assert_eq!(
        set_pin_cells(
            &mut document,
            &pins(&["IN+", "IN-"]),
            PinTableColumn::Side,
            "left"
        ),
        Ok(None)
    );
    assert_eq!(document, before);
}

#[test]
fn the_table_refuses_what_the_inspector_refuses() {
    let mut document = amplifier();
    let before = document.clone();

    let duplicate = set_pin_cells(&mut document, &pins(&["IN-"]), PinTableColumn::Name, "in+");
    assert!(duplicate.is_err(), "pin names are unique in any case");
    let negative = set_pin_cells(
        &mut document,
        &pins(&["IN+", "OUT"]),
        PinTableColumn::LeadLength,
        "-10",
    );
    assert!(negative.is_err(), "a lead runs 0 or more");
    let unknown = set_pin_cells(
        &mut document,
        &pins(&["OUT"]),
        PinTableColumn::Side,
        "north",
    );
    assert_eq!(
        unknown,
        Err("Side is one of: left, right, top, bottom.".to_owned())
    );
    let unplaced = set_pin_cells(
        &mut document,
        &pins(&["OUT", "VDD"]),
        PinTableColumn::X,
        "60",
    );
    assert_eq!(
        unplaced,
        Err("Pin VDD is not placed, so it has no X.".to_owned())
    );
    let two_names = set_pin_cells(
        &mut document,
        &pins(&["IN+", "IN-"]),
        PinTableColumn::Name,
        "A",
    );
    assert!(two_names.is_err(), "one name cannot name two pins");

    assert_eq!(document, before, "a refused edit changes nothing");
}

#[test]
fn a_rename_carries_the_old_name_so_instances_follow_it() {
    let mut document = amplifier();
    let edit = set_pin_cells(&mut document, &pins(&["OUT"]), PinTableColumn::Name, "Y")
        .expect("Y is free")
        .expect("the pin is renamed");
    assert_eq!(edit.step, "Rename pin OUT to Y");
    assert_eq!(edit.renames.get("OUT").map(String::as_str), Some("Y"));
    assert_eq!(edit.pins, pins(&["Y"]));
    assert_eq!(names(&document), ["IN+", "IN-", "Y", "VDD"]);
}

#[test]
fn an_electrical_type_sets_the_contract_the_inspector_sets() {
    let mut document = amplifier();
    let edit = set_pin_cells(
        &mut document,
        &pins(&["IN+", "IN-"]),
        PinTableColumn::ElectricalType,
        "digital input",
    )
    .expect("a kind is a kind")
    .expect("both pins change kind");
    assert_eq!(edit.step, "Set electrical type of 2 pins");
    for pin in &document.pins[..2] {
        assert_eq!(
            SymbolPinElectricalKind::from_pin(pin),
            SymbolPinElectricalKind::DigitalInput
        );
    }

    let edit = set_pin_cells(
        &mut document,
        &pins(&["OUT"]),
        PinTableColumn::ShowName,
        "hidden",
    )
    .expect("hidden is a choice")
    .expect("the name is hidden");
    assert_eq!(edit.step, "Hide the name of pin OUT");
    assert!(document.pins[2].name_hidden);
}

#[test]
fn a_netlist_place_moves_the_selected_pins_there_in_their_order() {
    let mut document = amplifier();
    let edit = set_pin_cells(
        &mut document,
        &pins(&["OUT", "VDD"]),
        PinTableColumn::NetlistOrder,
        "1",
    )
    .expect("1 is a place")
    .expect("the pins move");
    assert_eq!(edit.step, "Move 2 pins to netlist place 1");
    assert_eq!(names(&document), ["OUT", "VDD", "IN+", "IN-"]);

    assert!(
        set_pin_cells(
            &mut document,
            &pins(&["OUT"]),
            PinTableColumn::NetlistOrder,
            "5"
        )
        .is_err(),
        "a place past the last pin"
    );
}

#[test]
fn reordering_rows_moves_pins_one_netlist_place() {
    let mut document = amplifier();
    let edit = move_pins(&mut document, &pins(&["IN-", "VDD"]), false).expect("both move");
    assert_eq!(edit.step, "Move 2 pins earlier in netlist order");
    assert_eq!(names(&document), ["IN-", "IN+", "VDD", "OUT"]);

    // The first pin stays first; the pin behind it in the selection closes up.
    move_pins(&mut document, &pins(&["IN-", "IN+"]), false);
    assert_eq!(names(&document), ["IN-", "IN+", "VDD", "OUT"]);
    assert_eq!(
        move_pins(&mut document, &pins(&["IN-", "IN+"]), false),
        None
    );

    let edit = move_pins(&mut document, &pins(&["VDD"]), true).expect("VDD moves later");
    assert_eq!(edit.step, "Move pin VDD later in netlist order");
    assert_eq!(names(&document), ["IN-", "IN+", "OUT", "VDD"]);
    assert_eq!(move_pins(&mut document, &pins(&["VDD"]), true), None);
}
