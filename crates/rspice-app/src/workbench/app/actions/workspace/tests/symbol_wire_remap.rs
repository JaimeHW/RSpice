//! What storing a symbol document owes the wires of its open instances.
//!
//! A pin that moves on the symbol drags the wire endpoint that met it in every
//! open parent schematic, through the instance's origin, rotation and mirror,
//! exactly once.

use super::*;

#[test]
fn storing_symbol_document_remaps_open_instance_wires_by_pin_name() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
        ],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut symbol_view)
    .expect("initial symbol stores");
    amp.add_view(symbol_view);
    library.add_cell(amp);
    let mut top = Cell::new("top");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    state.library_manager.add_library(library);

    let ports = vec![
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&ports);

    let mut parent = SchematicState::default();
    parent.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    parent.document_mut_for_test().wires.push(Wire::segment(
        7,
        Point::new(60, 50),
        Point::new(0, 50),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "top", "schematic").key(), parent);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
        .store_active_symbol_document(&SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-20, 10))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            ],
            ..SymbolDocument::default()
        })
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .content
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.document().wires[0].points[0], Point::new(80, 60));
    assert_eq!(parent.document().wires[0].points[1], Point::new(0, 50));
}

#[test]
fn storing_symbol_document_remaps_instance_wires_when_origin_moves() {
    let before = SymbolDocument {
        origin: Point::new(20, 0),
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(30, 0)),
        )],
        ..SymbolDocument::default()
    };
    let mut state = state_with_amp_symbol(before);

    let mut parent = SchematicState::default();
    parent.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(amp_binding(&[("IN", PortDirection::In)])),
    );
    parent.document_mut_for_test().wires.push(Wire::segment(
        7,
        Point::new(110, 50),
        Point::new(0, 50),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&SymbolDocument {
            origin: Point::origin(),
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(30, 0)),
            )],
            ..SymbolDocument::default()
        })
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .content
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.document().wires[0].points[0], Point::new(130, 50));
    assert_eq!(parent.document().wires[0].points[1], Point::new(0, 50));
}

#[test]
fn storing_symbol_document_remaps_rotated_and_mirrored_instance_wires() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-30, 10)),
        ("OUT", PortDirection::Out, Point::new(40, 0)),
    ]));

    let mut parent = SchematicState::default();
    parent.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_rotation(Rotation::R90)
            .with_mirror_h(true)
            .with_library_cell(amp_binding(&[
                ("IN", PortDirection::In),
                ("OUT", PortDirection::Out),
            ])),
    );
    parent.document_mut_for_test().wires.push(Wire::segment(
        8,
        Point::new(90, 80),
        Point::new(90, 120),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(-10, -20)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]))
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .content
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.document().wires[0].points[0], Point::new(120, 60));
    assert_eq!(parent.document().wires[0].points[1], Point::new(90, 120));
}

#[test]
fn storing_symbol_document_applies_wire_remaps_once() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(0, 0)),
        ("OUT", PortDirection::Out, Point::new(10, 0)),
    ]));

    let mut parent = SchematicState::default();
    parent.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50)).with_library_cell(
            amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
        ),
    );
    parent.document_mut_for_test().wires.push(Wire::segment(
        9,
        Point::new(100, 50),
        Point::new(100, 0),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(10, 0)),
            ("OUT", PortDirection::Out, Point::new(20, 0)),
        ]))
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .content
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(
        parent.document().wires[0].points[0],
        Point::new(110, 50),
        "the IN endpoint must stop at IN's new location, not then match OUT's old location"
    );
}

#[test]
fn storing_symbol_document_remaps_all_open_parent_buffers() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-40, 0)),
        ("OUT", PortDirection::Out, Point::new(40, 0)),
    ]));
    let binding = amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]);

    let mut top = SchematicState::default();
    top.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding.clone()),
    );
    top.document_mut_for_test().components.push(
        Component::new(2, ComponentType::CellInstance, Point::new(200, 0))
            .with_library_cell(binding.clone()),
    );
    top.document_mut_for_test().wires.push(Wire::segment(
        10,
        Point::new(60, 50),
        Point::new(0, 50),
    ));
    top.document_mut_for_test().wires.push(Wire::segment(
        11,
        Point::new(160, 0),
        Point::new(160, -50),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "top", "schematic").key(), top);

    let mut tb = SchematicState::default();
    tb.document_mut_for_test().components.push(
        Component::new(3, ComponentType::CellInstance, Point::new(-10, 20))
            .with_library_cell(binding),
    );
    tb.document_mut_for_test().wires.push(Wire::segment(
        12,
        Point::new(-50, 20),
        Point::new(-90, 20),
    ));
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "tb", "schematic").key(), tb);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(-20, 10)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]))
        .expect("updated symbol stores");

    let top = state
        .workspace
        .content
        .schematic_buffers
        .get("work/top/schematic")
        .expect("top schematic remains open");
    assert_eq!(top.document().wires[0].points[0], Point::new(80, 60));
    assert_eq!(top.document().wires[1].points[0], Point::new(180, 10));

    let tb = state
        .workspace
        .content
        .schematic_buffers
        .get("work/tb/schematic")
        .expect("testbench schematic remains open");
    assert_eq!(tb.document().wires[0].points[0], Point::new(-30, 30));
}
