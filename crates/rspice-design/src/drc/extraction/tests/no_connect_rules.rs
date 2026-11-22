//! A pin marked no-connect, judged from the drawing.
//!
//! The mark says the pin stays open on purpose. Open, the check says nothing
//! about it; wired, the check says the mark and the drawing disagree.

use super::*;

/// The grounded resistor's free leg, `R1.+`, and its wired one, `R1.-`.
fn marked(pins: &[&str]) -> SchematicDocument {
    let mut document = grounded_resistor();
    for pin in pins {
        assert!(document.components[0].set_no_connect(pin, true));
    }
    document
}

fn checked(document: &SchematicDocument, config: DrcConfig) -> DrcResult {
    let views = HashMap::<String, SchematicDocument>::new();
    run_check_with_hierarchy(document, &HierarchySource::from_buffers(&views), config)
}

fn pins_of(result: &DrcResult, kind: DrcViolationType) -> Vec<String> {
    of_type(result, kind)
        .into_iter()
        .map(|finding| match &finding.location {
            DrcLocation::Terminal {
                component_name,
                pin_name,
                ..
            } => format!("{component_name}.{pin_name}"),
            other => panic!("a pin finding points at its pin: {other:?}"),
        })
        .collect()
}

#[test]
fn an_open_pin_is_reported_until_it_is_marked_no_connect() {
    let plain = checked(&grounded_resistor(), connectivity_only());
    assert_eq!(pins_of(&plain, DrcViolationType::UnconnectedPin), ["R1.+"]);

    let marked = checked(&marked(&["+"]), connectivity_only());
    assert!(marked.violations().is_empty(), "{:?}", marked.violations());
}

/// With the open-pin rule off, the floating-node rule speaks for an open
/// pin's node; a marked pin's node is meant to be alone, so it does not.
#[test]
fn a_marked_pins_node_is_not_floating() {
    let floating_only = DrcConfig {
        check_unconnected_pins: false,
        check_missing_ground: false,
        ..DrcConfig::default()
    };
    let plain = checked(&grounded_resistor(), floating_only.clone());
    assert_eq!(
        of_type(&plain, DrcViolationType::FloatingNode).len(),
        1,
        "{:?}",
        plain.violations()
    );
    let marked = checked(&marked(&["+"]), floating_only);
    assert!(
        of_type(&marked, DrcViolationType::FloatingNode).is_empty(),
        "{:?}",
        marked.violations()
    );
}

#[test]
fn a_wire_onto_a_marked_pin_is_its_own_finding() {
    let result = checked(&marked(&["-"]), connectivity_only());
    assert_eq!(
        pins_of(&result, DrcViolationType::ConnectedNoConnect),
        ["R1.-"]
    );
    let finding = of_type(&result, DrcViolationType::ConnectedNoConnect)[0];
    assert_eq!(
        finding.message,
        "Pin R1.- is marked no-connect but is connected"
    );
    // The free leg is still an open pin nobody marked.
    assert_eq!(pins_of(&result, DrcViolationType::UnconnectedPin), ["R1.+"]);
}
