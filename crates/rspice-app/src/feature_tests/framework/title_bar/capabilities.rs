//! Tests for what the menus reach beyond the plain edits: selecting every
//! object of a kind, stepping back through the views a canvas showed,
//! highlighting a net, copying a picture of the drawing and annotating a
//! transient's voltages.

use super::*;
use crate::schematic::view::sheet_visibility::SchematicObjectKind;
use crate::state::{
    ComponentType, DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry,
    Point, Selection,
};

/// A sheet holding objects of every kind Edit ▸ Select ▸ names, each kind's
/// expected selection beside it.
fn sheet_with_every_kind() -> (RSpiceApp, Vec<(SchematicObjectKind, Selection)>) {
    let mut app = title_test_app();
    let schematic = &mut app.state.model.schematic;
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    let ground = schematic.add_component(ComponentType::Ground, Point::new(0, 100));
    let pin = schematic.add_component(ComponentType::Port, Point::new(-100, 0));
    let wire = schematic
        .add_wire(vec![Point::new(-100, 0), Point::new(-40, 0)])
        .expect("a wire is added");
    let label = schematic.add_net_label(Point::new(-70, 0), "IN".to_owned());
    let document = schematic.document_mut_for_test();
    document.design_notes.push(
        DesignNote::new(900, Point::new(0, 200), DesignNoteKind::PlainText, "note")
            .expect("valid note"),
    );
    document.documentation_shapes.push(
        DocumentationShape::new(
            901,
            DocumentationShapeGeometry::Line {
                start: Point::new(0, 300),
                end: Point::new(100, 300),
            },
        )
        .expect("valid line"),
    );
    let expect = |fill: &dyn Fn(&mut Selection)| {
        let mut selection = Selection::default();
        fill(&mut selection);
        selection
    };
    let expected = vec![
        (
            SchematicObjectKind::Instances,
            expect(&|selection| {
                selection.select_component(resistor);
                selection.select_component(ground);
            }),
        ),
        (
            SchematicObjectKind::Wires,
            expect(&|selection| selection.select_wire(wire)),
        ),
        (
            SchematicObjectKind::NetLabels,
            expect(&|selection| selection.select_net_label(label)),
        ),
        (
            SchematicObjectKind::Pins,
            expect(&|selection| selection.select_component(pin)),
        ),
        (
            SchematicObjectKind::Text,
            expect(&|selection| selection.select_design_note(900)),
        ),
        (
            SchematicObjectKind::Shapes,
            expect(&|selection| selection.select_documentation_shape(901)),
        ),
    ];
    (app, expected)
}

/// Each Edit ▸ Select ▸ row selects every object of its kind on the sheet
/// and nothing else, in place of what was selected, whatever the selection
/// filter lets a click pick.
#[test]
fn select_by_kind_selects_exactly_that_kind() {
    let (mut app, expected) = sheet_with_every_kind();
    assert_eq!(
        expected.iter().map(|(kind, _)| *kind).collect::<Vec<_>>(),
        SchematicObjectKind::ALL
    );
    app.state.design.schematic_selection_filter.instances = false;
    app.execute(Command::SelectAll.key());
    for (kind, selection) in expected {
        let key = Command::SelectOfKind(kind).key();
        assert_eq!(
            app.availability(key),
            CommandAvailability::Available,
            "{kind:?}"
        );
        app.execute(key);
        assert_eq!(
            app.state.model.schematic.session.editor.selection, selection,
            "{kind:?}"
        );
    }
}

/// A kind the sheet has none of is offered greyed, saying so.
#[test]
fn select_by_kind_is_grey_for_a_kind_the_sheet_lacks() {
    let mut app = title_test_app();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    assert!(app.is_enabled(Command::SelectOfKind(SchematicObjectKind::Instances).key()));
    assert_eq!(
        app.availability(Command::SelectOfKind(SchematicObjectKind::Pins).key()),
        CommandAvailability::Disabled("the sheet has no pins")
    );
}
