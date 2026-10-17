//! What each rule decides, given an already-extracted design.
//!
//! The inputs here are built by hand because the subject is the rule, not the
//! extraction: a component with a named terminal on a node stands for whatever
//! drawing produced it. That the extraction really does produce these inputs —
//! that the checker and the deck agree on every terminal of every fixture — is
//! proved in `services::drc_tests`, which is where the geometry
//! lives.

use super::*;
use crate::drc::input::{ParameterRangeIssue, PinInfo};
use crate::schematic::document::SchematicDocument;
use crate::schematic::port::PortDirection;
use rspice_design_model::Point;

/// A terminal that something else in the drawing meets, declared by nothing —
/// which is what a passive leg or a primitive device terminal is.
fn pin(name: &str, net: &str) -> PinInfo {
    PinInfo {
        name: name.to_string(),
        net_name: net.to_string(),
        direction: None,
        point: Point::origin(),
        attached: true,
        optional: false,
        on_global: false,
        bulk: false,
    }
}

/// A terminal the extraction found nothing meeting, declared by nothing.
fn open_pin(name: &str, net: &str, point: Point) -> PinInfo {
    PinInfo {
        name: name.to_string(),
        net_name: net.to_string(),
        direction: None,
        point,
        attached: false,
        optional: false,
        on_global: false,
        bulk: false,
    }
}

/// The same open terminal, carrying what a bound cell interface declares it to
/// be.
fn declared_open_pin(name: &str, net: &str, direction: PortDirection) -> PinInfo {
    PinInfo {
        direction: Some(direction),
        ..open_pin(name, net, Point::origin())
    }
}

/// The one card an instance writes, its own element, under `name`.
fn element(name: &str) -> Option<Vec<WrittenCard>> {
    Some(vec![WrittenCard {
        name: name.to_owned(),
        role: CardRole::Element,
    }])
}

/// The card the netlister writes for a resistor called `name`: under its own
/// name, or `R<id>` when it has none.
fn written_name(id: u64, name: &str) -> Option<Vec<WrittenCard>> {
    if name.is_empty() {
        element(&format!("R{id}"))
    } else {
        element(name)
    }
}

fn resistor(id: u64, name: &str, pins: Vec<PinInfo>) -> ComponentInfo {
    ComponentInfo {
        id,
        name: name.to_string(),
        component_type: "resistor".to_string(),
        pins,
        is_voltage_source: false,
        is_current_source: false,
        is_ground_symbol: false,
        reference_required: true,
        reference_error: None,
        written_cards: written_name(id, name),
        is_interface_port: false,
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
        broken_references: Vec::new(),
    }
}

fn vsource(id: u64, name: &str, plus_net: &str, minus_net: &str) -> ComponentInfo {
    let mut plus = pin("+", plus_net);
    plus.direction = Some(PortDirection::Out);
    ComponentInfo {
        id,
        name: name.to_string(),
        component_type: "voltage_source".to_string(),
        pins: vec![plus, pin("-", minus_net)],
        is_voltage_source: true,
        is_current_source: false,
        is_ground_symbol: false,
        reference_required: true,
        reference_error: None,
        written_cards: element(name),
        is_interface_port: false,
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
        broken_references: Vec::new(),
    }
}

/// An interface port standing on `net`, whose own terminal is where the node
/// leaves for the parent.
fn interface_port(id: u64, net: &str, attached: bool) -> ComponentInfo {
    ComponentInfo {
        id,
        name: String::new(),
        component_type: String::new(),
        pins: vec![PinInfo {
            attached,
            ..pin("1", net)
        }],
        is_voltage_source: false,
        is_current_source: false,
        is_ground_symbol: false,
        reference_required: false,
        reference_error: None,
        written_cards: None,
        is_interface_port: true,
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
        broken_references: Vec::new(),
    }
}

/// A drawn ground symbol, whose own terminal is not evidence that the circuit
/// reaches ground.
fn ground_symbol(id: u64, point: Point, attached: bool) -> ComponentInfo {
    ComponentInfo {
        id,
        name: String::new(),
        component_type: String::new(),
        pins: vec![PinInfo {
            name: "GND".to_owned(),
            net_name: "0".to_owned(),
            direction: None,
            point,
            attached,
            optional: false,
            on_global: false,
            bulk: false,
        }],
        is_voltage_source: false,
        is_current_source: false,
        is_ground_symbol: true,
        reference_required: false,
        reference_error: None,
        written_cards: None,
        is_interface_port: false,
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
        broken_references: Vec::new(),
    }
}

fn check(components: &[ComponentInfo], config: DrcConfig) -> DrcResult {
    DrcChecker::with_config(config).check(components, &ExtractedConnectivity::default())
}

fn of_type(result: &DrcResult, vt: DrcViolationType) -> Vec<&DrcViolation> {
    result
        .violations()
        .iter()
        .filter(|v| v.violation_type == vt)
        .collect()
}

// V1 across vin/0 with R1 in parallel: every net has two connections and
// ground is present, so a default check is clean.
fn minimal_circuit() -> Vec<ComponentInfo> {
    vec![
        vsource(0, "V1", "vin", "0"),
        resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
    ]
}

#[test]
fn empty_schematic_reports_only_missing_ground() {
    let result = check(&[], DrcConfig::default());
    assert!(result.completed);
    assert_eq!(result.total_count(), 1);
    assert_eq!(
        result.violations()[0].violation_type,
        DrcViolationType::MissingGround
    );
    assert_eq!(result.violations()[0].location, DrcLocation::Global);
}

#[test]
fn empty_schematic_clean_when_ground_check_disabled() {
    let result = check(
        &[],
        DrcConfig {
            check_missing_ground: false,
            ..DrcConfig::default()
        },
    );
    assert!(result.completed);
    assert_eq!(result.total_count(), 0);
    assert!(result.passed());
}

#[test]
fn minimal_connected_circuit_has_no_violations() {
    let result = check(&minimal_circuit(), DrcConfig::default());
    assert_eq!(result.total_count(), 0, "{:?}", result.violations());
    assert!(result.passed());
}

#[test]
fn duplicate_component_names_are_flagged() {
    let mut components = minimal_circuit();
    components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

    let result = check(&components, DrcConfig::default());
    let dups = of_type(&result, DrcViolationType::DuplicateName);
    assert_eq!(dups.len(), 1);
    assert_eq!(dups[0].severity, DrcSeverity::Critical);
    match &dups[0].location {
        DrcLocation::Component { name, .. } => assert_eq!(name, "R1"),
        other => panic!("expected component location, got {:?}", other),
    }
}

#[test]
fn triplicate_name_reported_once_not_per_pair() {
    let components = vec![
        vsource(0, "V1", "vin", "0"),
        resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        resistor(3, "R1", vec![pin("1", "vin"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    let dups = of_type(&result, DrcViolationType::DuplicateName);
    assert_eq!(dups.len(), 1);
    assert!(dups[0].message.contains("3 instances"));
}

#[test]
fn duplicate_name_check_can_be_disabled() {
    let mut components = minimal_circuit();
    components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

    let result = check(
        &components,
        DrcConfig {
            check_duplicate_names: false,
            ..DrcConfig::default()
        },
    );
    assert!(of_type(&result, DrcViolationType::DuplicateName).is_empty());
}

#[test]
fn missing_ground_flagged_with_global_location() {
    let components = vec![
        vsource(0, "V1", "vin", "vee"),
        resistor(1, "R1", vec![pin("1", "vin"), pin("2", "vee")]),
    ];
    let result = check(&components, DrcConfig::default());
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].location, DrcLocation::Global);
    assert_eq!(missing[0].severity, DrcSeverity::Critical);
}

/// A ground symbol standing on its own binds node 0 to nothing, so the circuit
/// still has no reference — and the finding points at the symbol that failed to
/// reach it rather than at the design as a whole.
#[test]
fn a_ground_symbol_that_reaches_nothing_is_reported_on_its_own_terminal() {
    let terminal = Point::new(40, -10);
    let alone = vec![
        ground_symbol(1, terminal, false),
        resistor(2, "R1", vec![pin("1", "a"), pin("2", "b")]),
    ];
    let result = check(
        &alone,
        DrcConfig {
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(missing.len(), 1);
    assert_eq!(
        missing[0].location,
        DrcLocation::Point { x: 40.0, y: -10.0 }
    );

    // The same symbol with the circuit on it grounds the design.
    let wired = vec![
        ground_symbol(1, terminal, true),
        resistor(2, "R1", vec![pin("1", "a"), pin("2", "0")]),
    ];
    let result = check(
        &wired,
        DrcConfig {
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );
    assert!(of_type(&result, DrcViolationType::MissingGround).is_empty());
}

#[test]
fn gnd_name_satisfies_ground_check_case_insensitive() {
    let components = vec![
        vsource(0, "V1", "vin", "GND"),
        resistor(1, "R1", vec![pin("1", "vin"), pin("2", "GND")]),
    ];
    let result = check(&components, DrcConfig::default());
    assert!(of_type(&result, DrcViolationType::MissingGround).is_empty());
}

#[test]
fn single_connection_net_is_floating() {
    let mut components = minimal_circuit();
    components.push(resistor(2, "R2", vec![pin("1", "vin"), pin("2", "out")]));

    let result = check(&components, DrcConfig::default());
    let floating = of_type(&result, DrcViolationType::FloatingNode);
    assert_eq!(floating.len(), 1);
    assert_eq!(floating[0].severity, DrcSeverity::Warning);
    assert_eq!(
        floating[0].location,
        DrcLocation::Node {
            net_name: "out".to_string()
        }
    );
    // Related items identify the component so the UI can highlight it.
    assert_eq!(floating[0].related_items, vec!["R2".to_string()]);
}

#[test]
fn ground_net_is_never_floating() {
    let components = vec![resistor(0, "R1", vec![pin("1", "0"), pin("2", "a")])];
    let result = check(&components, DrcConfig::default());
    let floating = of_type(&result, DrcViolationType::FloatingNode);
    assert_eq!(floating.len(), 1);
    assert_eq!(
        floating[0].location,
        DrcLocation::Node {
            net_name: "a".to_string()
        }
    );
}

#[test]
fn min_connections_config_is_honored() {
    let result = check(
        &minimal_circuit(),
        DrcConfig {
            min_connections: 3,
            ..DrcConfig::default()
        },
    );
    let floating = of_type(&result, DrcViolationType::FloatingNode);
    // "vin" has two connections, below the threshold; "0" is exempt as ground.
    assert_eq!(floating.len(), 1);
    assert_eq!(
        floating[0].location,
        DrcLocation::Node {
            net_name: "vin".to_string()
        }
    );
}

#[test]
fn two_voltage_sources_across_one_pair_are_source_to_source() {
    // The second source is drawn the other way round, and ground is spelled
    // differently: neither changes the pair of nodes the engine sees.
    let components = vec![
        vsource(0, "V1", "vin", "0"),
        vsource(1, "V2", "GND", "vin"),
        resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    let shorted = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(shorted.len(), 1, "{:?}", result.violations());
    assert_eq!(
        shorted[0].location,
        DrcLocation::Node {
            net_name: "vin".to_string()
        }
    );
    assert_eq!(
        shorted[0].related_items,
        vec!["V1".to_string(), "V2".to_string()]
    );
}

/// Sources that share a node are an ordinary circuit: the engine solves V1
/// from `vin` to ground beside V2 from `vin` to `vb`, so the check must not
/// refuse it.
#[test]
fn sources_sharing_a_drive_node_are_not_a_loop() {
    let components = vec![
        vsource(0, "V1", "vin", "0"),
        vsource(1, "V2", "vin", "vb"),
        resistor(2, "R1", vec![pin("1", "vb"), pin("2", "0")]),
        resistor(3, "R2", vec![pin("1", "vin"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );
    assert!(result.passed(), "{:?}", result.violations());
}

/// A ring of sources is a loop however long it is. It is named by the source
/// that closes it, after the sources it runs through, in the order the ring
/// visits them.
#[test]
fn a_ring_of_three_sources_is_one_loop() {
    let components = vec![
        vsource(0, "V1", "a", "b"),
        vsource(1, "V2", "b", "0"),
        vsource(2, "V3", "0", "a"),
        resistor(3, "R1", vec![pin("1", "a"), pin("2", "0")]),
        resistor(4, "R2", vec![pin("1", "b"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    let loops = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(loops.len(), 1, "{:?}", result.violations());
    assert_eq!(loops[0].severity, DrcSeverity::Error);
    assert_eq!(
        loops[0].related_items,
        vec!["V2".to_string(), "V1".to_string(), "V3".to_string()]
    );
    assert_eq!(
        loops[0].location,
        DrcLocation::Node {
            net_name: "a".to_string()
        }
    );

    // A source across one node is a loop of its own.
    let result = check(
        &[
            vsource(0, "V1", "a", "a"),
            resistor(1, "R1", vec![pin("1", "a"), pin("2", "0")]),
        ],
        DrcConfig::default(),
    );
    let loops = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(loops.len(), 1, "{:?}", result.violations());
    assert_eq!(loops[0].related_items, vec!["V1".to_string()]);
}

#[test]
fn sources_on_distinct_nets_are_not_shorted() {
    let components = vec![
        vsource(0, "V1", "vin", "0"),
        vsource(1, "V2", "vcc", "0"),
        resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        resistor(3, "R2", vec![pin("1", "vcc"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    assert!(of_type(&result, DrcViolationType::SourceToSource).is_empty());
    assert!(result.passed());
}

#[test]
fn declared_outputs_on_one_node_are_shorted_outputs() {
    let mut first = resistor(1, "X1", vec![pin("OUT", "n1")]);
    first.pins[0].direction = Some(PortDirection::Out);
    let mut second = resistor(2, "X2", vec![pin("OUT", "n1")]);
    second.pins[0].direction = Some(PortDirection::Out);
    let result = check(
        &[second, first],
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        },
    );
    let shorted = of_type(&result, DrcViolationType::ShortedOutputs);
    assert_eq!(shorted.len(), 1);
    assert_eq!(
        shorted[0].related_items,
        vec!["X1".to_owned(), "X2".to_owned()]
    );
}

#[test]
fn duplicate_typed_bus_member_drivers_are_reported_once() {
    let first = vsource(1, "V1", "DATA[3]", "0");
    let second = vsource(2, "V2", "DATA[3]", "0");
    let result = check(
        &[first, second],
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        },
    );
    let drivers = of_type(&result, DrcViolationType::DuplicateBusMemberDriver);
    assert_eq!(drivers.len(), 1);
    assert!(drivers[0].message.contains("V1"));
    assert!(drivers[0].message.contains("V2"));
}

#[test]
fn an_unattached_terminal_is_reported_and_the_gate_silences_it() {
    let components = vec![resistor(
        7,
        "R7",
        vec![open_pin("1", "net1", Point::new(10, 20))],
    )];
    let base = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    };
    let result = check(&components, base.clone());
    let unconnected = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(unconnected.len(), 1);
    assert_eq!(
        unconnected[0].location,
        DrcLocation::Terminal {
            component_id: 7,
            component_name: "R7".to_owned(),
            pin_name: "1".to_owned(),
            point: Point::new(10, 20),
        }
    );

    let mut disabled = base;
    disabled.check_unconnected_pins = false;
    let result = check(&components, disabled);
    assert!(of_type(&result, DrcViolationType::UnconnectedPin).is_empty());
}

/// An unattached terminal is three findings, not one at three severities: the
/// declaration decides which, and the absence of a declaration is not a
/// direction.
#[test]
fn an_unattached_terminal_is_classified_by_what_declares_it() {
    let base = DrcConfig {
        check_missing_ground: false,
        ..DrcConfig::default()
    };
    let classify = |pin: PinInfo| {
        let result = check(&[resistor(1, "X1", vec![pin])], base.clone());
        let findings = result.violations();
        assert_eq!(findings.len(), 1, "{findings:?}");
        (
            findings[0].violation_type,
            findings[0].severity,
            findings[0].message.clone(),
        )
    };

    let (kind, severity, _) = classify(declared_open_pin("A", "n1", PortDirection::In));
    assert_eq!(
        (kind, severity),
        (DrcViolationType::UndrivenInput, DrcSeverity::Error)
    );
    // A supply pin states that the parent feeds this rail. Open, nothing does.
    let (kind, severity, message) = classify(declared_open_pin("VDD", "n1", PortDirection::Supply));
    assert_eq!(
        (kind, severity),
        (DrcViolationType::UndrivenInput, DrcSeverity::Error)
    );
    assert!(message.contains("Supply pin X1.VDD"), "{message}");
    assert!(message.contains("rail"), "{message}");
    let (kind, severity, _) = classify(declared_open_pin("Y", "n1", PortDirection::Out));
    assert_eq!(
        (kind, severity),
        (DrcViolationType::UnloadedOutput, DrcSeverity::Warning)
    );
    for undeclared in [
        declared_open_pin("B", "n1", PortDirection::InOut),
        open_pin("1", "n1", Point::origin()),
    ] {
        let (kind, severity, _) = classify(undeclared);
        assert_eq!(
            (kind, severity),
            (DrcViolationType::UnconnectedPin, DrcSeverity::Warning)
        );
    }
}

/// An open terminal is alone on a node of its own, and the open-pin finding
/// names the terminal. Saying it again as a floating node would report one
/// slip twice; with the open-pin rule off, the node is the only thing left to
/// say it.
#[test]
fn an_open_terminal_is_reported_once() {
    let components = [resistor(
        1,
        "R1",
        vec![open_pin("1", "net1", Point::origin()), pin("2", "0")],
    )];
    let base = DrcConfig {
        check_missing_ground: false,
        ..DrcConfig::default()
    };

    let result = check(&components, base.clone());
    assert_eq!(result.total_count(), 1, "{:?}", result.violations());
    assert_eq!(
        result.violations()[0].violation_type,
        DrcViolationType::UnconnectedPin
    );

    let result = check(
        &components,
        DrcConfig {
            check_unconnected_pins: false,
            ..base
        },
    );
    assert_eq!(result.total_count(), 1, "{:?}", result.violations());
    assert_eq!(
        result.violations()[0].violation_type,
        DrcViolationType::FloatingNode
    );
}

/// An input the netlister writes `null` for may stay open. It is neither an
/// open pin nor a node: the deck never writes one.
#[test]
fn an_optional_input_left_open_is_not_reported() {
    let mut set = declared_open_pin("set", "n1", PortDirection::In);
    set.optional = true;
    for check_unconnected_pins in [true, false] {
        let result = check(
            &[resistor(1, "A1", vec![set.clone()])],
            DrcConfig {
                check_missing_ground: false,
                check_unconnected_pins,
                ..DrcConfig::default()
            },
        );
        assert!(result.violations().is_empty(), "{:?}", result.violations());
    }
}

/// A node that continues outside the cellview is connected to something no
/// terminal here represents. An interface port a cell does not use inside is
/// the parent's net in the deck, and a global is the same node in every
/// cellview that names it; neither is a dangling node because one cellview
/// touches it once.
#[test]
fn a_node_that_continues_outside_the_cellview_is_not_floating() {
    let config = DrcConfig {
        check_missing_ground: false,
        ..DrcConfig::default()
    };

    // The port alone on its net — bare, or with a stub of wire and nothing on
    // it — is not an open pin and not a floating node.
    for attached in [false, true] {
        let unused_port = check(&[interface_port(1, "IN", attached)], config.clone());
        assert!(
            unused_port.violations().is_empty(),
            "{:?}",
            unused_port.violations()
        );
    }
    // The port and one leg: two terminals here, and the parent's net beyond
    // makes three, which is what a rule asking for three connections needs.
    let used_port = check(
        &[
            interface_port(1, "IN", true),
            resistor(2, "R1", vec![pin("1", "IN"), pin("2", "0")]),
        ],
        DrcConfig {
            min_connections: 3,
            ..config.clone()
        },
    );
    assert!(
        of_type(&used_port, DrcViolationType::FloatingNode).is_empty(),
        "{:?}",
        used_port.violations()
    );

    let mut on_global = pin("1", "vdd!");
    on_global.on_global = true;
    let global = check(
        &[resistor(1, "R1", vec![on_global, pin("2", "0")])],
        config.clone(),
    );
    assert!(
        of_type(&global, DrcViolationType::FloatingNode).is_empty(),
        "{:?}",
        global.violations()
    );

    // The same single touch on a local node is still floating.
    let local = check(
        &[resistor(1, "R1", vec![pin("1", "vdd"), pin("2", "0")])],
        config,
    );
    assert_eq!(of_type(&local, DrcViolationType::FloatingNode).len(), 1);
}

/// A voltage source has no interface to declare its drive terminal, so the `+`
/// terminal is the declaration. An open one loads nothing; the return terminal
/// is still unclassified.
#[test]
fn an_open_voltage_source_drive_terminal_is_an_unloaded_output() {
    let mut source = vsource(1, "V1", "vin", "0");
    source.pins[0].attached = false;
    source.pins[1].attached = false;
    let result = check(
        &[source],
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        },
    );
    let outputs = of_type(&result, DrcViolationType::UnloadedOutput);
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].message.contains("V1.+"));
    assert_eq!(of_type(&result, DrcViolationType::UnconnectedPin).len(), 1);
    assert!(of_type(&result, DrcViolationType::UndrivenInput).is_empty());
}

/// A reference the schema refuses is an error only where the deck cannot
/// write the instance. A primitive the netlister names or renames still runs,
/// under the name the finding states, because the results will carry it.
#[test]
fn a_refused_reference_is_an_error_only_where_the_deck_cannot_write_it() {
    let config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        check_unconnected_pins: false,
        ..DrcConfig::default()
    };

    // What the extraction says of a resistor with no name, and of one named
    // with another device's letter.
    let unnamed = resistor(3, "", vec![]);
    let mut misnamed = resistor(4, "C4", vec![]);
    misnamed.reference_error = Some("Resistor designators must begin with `R`.".to_owned());
    misnamed.written_cards = element("RC4");
    let mut structural = resistor(5, "", vec![]);
    structural.reference_required = false;
    structural.written_cards = None;
    structural.component_type = "port".to_owned();
    let result = check(&[unnamed, misnamed, structural], config.clone());

    let empty = of_type(&result, DrcViolationType::EmptyName);
    assert_eq!(empty.len(), 1, "a structural object owns no reference");
    assert_eq!(empty[0].severity, DrcSeverity::Warning);
    assert!(
        empty[0].message.ends_with("the netlist writes it as R3"),
        "{}",
        empty[0].message
    );
    let invalid = of_type(&result, DrcViolationType::InvalidName);
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0].severity, DrcSeverity::Warning);
    assert!(
        invalid[0].message.ends_with("the netlist writes it as RC4"),
        "{}",
        invalid[0].message
    );
    assert!(result.passed(), "{:?}", result.violations());

    // A cell instance whose reference is not a path segment has no name the
    // deck can write it under.
    let mut unusable = resistor(6, "X1-a", vec![]);
    unusable.reference_error =
        Some("Enter `X` followed by one or more ASCII letters, digits, or underscores.".to_owned());
    unusable.written_cards = None;
    let mut unnamed_cell = resistor(7, "", vec![]);
    unnamed_cell.written_cards = None;
    let result = check(&[unusable, unnamed_cell], config);
    let invalid = of_type(&result, DrcViolationType::InvalidName);
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0].severity, DrcSeverity::Error);
    assert!(
        invalid[0]
            .message
            .ends_with("the netlist cannot write this instance"),
        "{}",
        invalid[0].message
    );
    let empty = of_type(&result, DrcViolationType::EmptyName);
    assert_eq!(empty.len(), 1);
    assert_eq!(empty[0].severity, DrcSeverity::Error);
    assert!(result.has_errors());
}

#[test]
fn schema_parameter_findings_are_structured_gated_and_overrideable() {
    let mut component = resistor(8, "R8", vec![]);
    component.missing_parameters = vec!["Resistance".to_owned()];
    component.out_of_range_parameters = vec![ParameterRangeIssue {
        name: "m".to_owned(),
        display_name: "Multiplier".to_owned(),
        value: 0.0,
        min: Some(1.0),
        max: Some(10_000.0),
    }];
    let mut config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        check_unconnected_pins: false,
        ..DrcConfig::default()
    };
    config
        .severity_overrides
        .insert(DrcViolationType::ValueOutOfRange, DrcSeverity::Critical);
    let result = check(std::slice::from_ref(&component), config.clone());

    let missing = of_type(&result, DrcViolationType::MissingParameter);
    assert_eq!(missing.len(), 1);
    assert!(missing[0].message.contains("Resistance"));
    let range = of_type(&result, DrcViolationType::ValueOutOfRange);
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].severity, DrcSeverity::Critical);
    assert!(range[0].message.contains("between 1 and 10000"));

    config.check_component_parameters = false;
    let disabled = check(&[component], config);
    assert!(of_type(&disabled, DrcViolationType::MissingParameter).is_empty());
    assert!(of_type(&disabled, DrcViolationType::ValueOutOfRange).is_empty());
}

#[test]
fn unknown_component_requires_authoritative_resolution_and_honors_gate() {
    let mut unresolved = resistor(9, "X9", vec![]);
    unresolved.component_known = Some(false);
    let mut unknown_authority = resistor(10, "X10", vec![]);
    unknown_authority.component_known = None;
    let config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        check_unconnected_pins: false,
        ..DrcConfig::default()
    };
    let result = check(&[unknown_authority, unresolved.clone()], config.clone());
    let unknown = of_type(&result, DrcViolationType::UnknownComponent);
    assert_eq!(unknown.len(), 1, "{:?}", result.violations());
    assert_eq!(
        unknown[0].location,
        DrcLocation::Component {
            id: 9,
            name: "X9".to_owned()
        }
    );
    // An instance with no master cannot be netlisted, so the check fails.
    assert_eq!(unknown[0].severity, DrcSeverity::Error);
    assert!(result.has_errors());

    let mut disabled = config;
    disabled.check_unknown_components = false;
    let result = check(&[unresolved], disabled);
    assert!(of_type(&result, DrcViolationType::UnknownComponent).is_empty());
}

/// The project's configured policy outranks the rule's own judgement, which
/// reads the component rather than the violation type alone — down to the
/// floor for that instance and no further. A deck that lowers the warning on
/// an instance the netlister renamed does not lower the error on one it
/// cannot write: the run would be refused all the same.
#[test]
fn severity_overrides_outrank_the_severity_a_component_gives_a_finding() {
    let empty = resistor(3, "", vec![]);
    let mut invalid = resistor(4, "C4", vec![]);
    invalid.reference_error = Some("Resistor designators must begin with `R`.".to_owned());
    let mut unwritable = resistor(6, "", vec![]);
    unwritable.written_cards = None;
    let mut unresolved = resistor(5, "X5", vec![]);
    unresolved.component_known = Some(false);

    let mut config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        check_unconnected_pins: false,
        ..DrcConfig::default()
    };
    for violation_type in [
        DrcViolationType::EmptyName,
        DrcViolationType::InvalidName,
        DrcViolationType::UnknownComponent,
    ] {
        config
            .severity_overrides
            .insert(violation_type, DrcSeverity::Info);
    }

    let result = check(&[empty, invalid, unwritable, unresolved], config);
    let severity_at = |violation_type, id| {
        of_type(&result, violation_type)
            .into_iter()
            .find(|finding| matches!(finding.location, DrcLocation::Component { id: at, .. } if at == id))
            .map(|finding| finding.severity)
    };
    assert_eq!(
        severity_at(DrcViolationType::EmptyName, 3),
        Some(DrcSeverity::Info)
    );
    assert_eq!(
        severity_at(DrcViolationType::InvalidName, 4),
        Some(DrcSeverity::Info)
    );
    assert_eq!(
        severity_at(DrcViolationType::EmptyName, 6),
        Some(DrcSeverity::Error)
    );
    assert_eq!(
        severity_at(DrcViolationType::UnknownComponent, 5),
        Some(DrcSeverity::Error)
    );
    assert!(result.has_errors());
}

/// A deck that sets an instance rule to its floor, or raises it, means the
/// same thing on every object it reaches, and promotes nothing it does not.
///
/// An object that emits no instance — a port, a ground, a drawn symbol whose
/// type resolves to nothing — is never asked an instance rule at all, so no
/// deck can make one of its findings an error. On an emitted instance the deck
/// sets what the finding is reported at, and the floor for an instance the
/// deck cannot write holds that one at an error.
#[test]
fn a_deck_at_an_instance_rules_floor_promotes_no_structural_object() {
    let mut structural = resistor(7, "", vec![]);
    structural.reference_required = false;
    structural.written_cards = None;
    structural.component_known = Some(false);
    structural.component_type = "port".to_owned();
    let written = resistor(8, "", vec![]);
    let mut unwritable = resistor(9, "", vec![]);
    unwritable.written_cards = None;
    unwritable.component_known = Some(false);

    let base = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        check_unconnected_pins: false,
        ..DrcConfig::default()
    };
    let severities = |deck: &crate::drc::DesignRuleDeck| {
        let result = check(
            &[structural.clone(), written.clone(), unwritable.clone()],
            DrcConfig {
                severity_overrides: deck.severity_overrides(),
                ..base.clone()
            },
        );
        let at = |violation_type, id| {
            of_type(&result, violation_type)
                .into_iter()
                .filter(|finding| matches!(finding.location, DrcLocation::Component { id: at, .. } if at == id))
                .map(|finding| finding.severity)
                .collect::<Vec<_>>()
        };
        (
            [
                at(DrcViolationType::EmptyName, 7),
                at(DrcViolationType::UnknownComponent, 7),
            ],
            at(DrcViolationType::EmptyName, 8),
            [
                at(DrcViolationType::EmptyName, 9),
                at(DrcViolationType::UnknownComponent, 9),
            ],
        )
    };

    // Each rule at its floor: an empty reference may go as low as a note, an
    // unresolved master no lower than an error.
    let mut at_floor = crate::drc::DesignRuleDeck::default();
    at_floor
        .set_severity(DrcViolationType::EmptyName, DrcSeverity::Info)
        .expect("an empty reference may be lowered to a note");
    at_floor
        .set_severity(DrcViolationType::UnknownComponent, DrcSeverity::Error)
        .expect("an unresolved master may be held at an error");
    let (structural_findings, written_findings, unwritable_findings) = severities(&at_floor);
    assert_eq!(
        structural_findings,
        [vec![], vec![]],
        "a structural object is not asked"
    );
    assert_eq!(written_findings, vec![DrcSeverity::Info]);
    assert_eq!(
        unwritable_findings,
        [vec![DrcSeverity::Error], vec![DrcSeverity::Error]]
    );

    // A raise is a raise on every instance it reaches, and still on no
    // structural object.
    let mut raised = crate::drc::DesignRuleDeck::default();
    raised
        .set_severity(DrcViolationType::EmptyName, DrcSeverity::Critical)
        .expect("an empty reference may be raised");
    raised
        .set_severity(DrcViolationType::UnknownComponent, DrcSeverity::Critical)
        .expect("an unresolved master may be raised");
    let (structural_findings, written_findings, unwritable_findings) = severities(&raised);
    assert_eq!(
        structural_findings,
        [vec![], vec![]],
        "a structural object is not asked"
    );
    assert_eq!(written_findings, vec![DrcSeverity::Critical]);
    assert_eq!(
        unwritable_findings,
        [vec![DrcSeverity::Critical], vec![DrcSeverity::Critical]]
    );
}

#[test]
fn new_connectivity_findings_honor_severity_overrides() {
    let mut config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    };
    config
        .severity_overrides
        .insert(DrcViolationType::UnconnectedPin, DrcSeverity::Critical);
    let result = check(
        &[resistor(
            1,
            "R1",
            vec![open_pin("1", "net1", Point::origin())],
        )],
        config,
    );
    assert_eq!(
        of_type(&result, DrcViolationType::UnconnectedPin)[0].severity,
        DrcSeverity::Critical
    );
}

#[test]
fn finding_order_and_ids_are_repeatable() {
    let components = vec![
        resistor(2, "R2", vec![pin("1", "z")]),
        resistor(1, "R1", vec![pin("1", "a")]),
        resistor(3, "R1", vec![pin("1", "b")]),
    ];
    let capture = |components: &[ComponentInfo]| {
        check(components, DrcConfig::default())
            .violations()
            .iter()
            .map(|finding| {
                (
                    finding.id,
                    finding.violation_type,
                    finding.location.display(),
                    finding.message.clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    let reversed = components.iter().cloned().rev().collect::<Vec<_>>();
    assert_eq!(capture(&components), capture(&reversed));
}

#[test]
fn fixed_violation_disappears_on_recheck() {
    let mut components = minimal_circuit();
    components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

    let result = check(&components, DrcConfig::default());
    assert_eq!(of_type(&result, DrcViolationType::DuplicateName).len(), 1);

    // A rename changes the name the deck writes, which is what collided.
    components[2].name = "R2".to_string();
    components[2].written_cards = element("R2");
    let result = check(&components, DrcConfig::default());
    assert!(of_type(&result, DrcViolationType::DuplicateName).is_empty());
    assert_eq!(result.total_count(), 0);
}

#[test]
fn violation_ids_are_unique_within_a_result() {
    let components = vec![
        vsource(0, "V1", "vin", "x"),
        vsource(1, "V1", "vin", "y"),
        resistor(2, "R1", vec![pin("1", "vin"), pin("2", "z")]),
    ];
    let result = check(&components, DrcConfig::default());
    assert!(result.total_count() >= 3);
    let mut ids: Vec<usize> = result.violations().iter().map(|v| v.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), result.total_count());
}

#[test]
fn summary_counts_match_violations() {
    let mut unvalued = resistor(2, "R2", vec![pin("1", "a"), pin("2", "b")]);
    unvalued.missing_parameters = vec!["Resistance".to_owned()];
    let components = vec![
        // No ground (critical), duplicate name (critical), a missing value
        // (error), and floating "out" (warning).
        resistor(0, "R1", vec![pin("1", "a"), pin("2", "out")]),
        resistor(1, "R1", vec![pin("1", "a"), pin("2", "b")]),
        unvalued,
    ];
    let result = check(&components, DrcConfig::default());
    let summary = result.summary();
    assert_eq!(summary.total, result.total_count());
    assert_eq!(summary.critical, 2);
    assert_eq!(summary.errors, 1);
    assert_eq!(summary.warnings, 1);
    assert!(!summary.passed);
    assert!(result.has_errors());
}

#[test]
fn configured_severity_override_is_applied_to_checker_findings() {
    let mut config = DrcConfig::default();
    config
        .severity_overrides
        .insert(DrcViolationType::MissingGround, DrcSeverity::Info);

    let result = check(&[], config);

    assert_eq!(result.total_count(), 1);
    assert_eq!(result.violations()[0].severity, DrcSeverity::Info);
    assert!(result.passed());
}

/// A four-bit port standing on a two-bit bus, plus the wider case that no
/// slice can rescue.
fn width_mismatch_schematic(bus: &str, port: &str) -> SchematicDocument {
    use crate::schematic::bus::Bus;
    use crate::schematic::bus::BusDeclaration;
    use crate::schematic::component_type::ComponentType;

    let mut schematic = SchematicDocument::default();
    schematic.buses.push(
        Bus::segment(
            1,
            Point::new(90, 0),
            Point::new(150, 0),
            Some(BusDeclaration::parse(bus).expect("fixture bus")),
        )
        .expect("fixture bus geometry"),
    );
    schematic.components.push(
        crate::schematic::component::Component::new(2, ComponentType::Port, Point::new(100, 0))
            .with_name_value("", port),
    );
    schematic
}

fn width_findings(
    schematic: &SchematicDocument,
    width_mismatch: crate::connectivity_contract::BundleWidthMismatchPolicy,
) -> Vec<DrcViolation> {
    let mut result = DrcResult::new();
    let policy = crate::connectivity_contract::ConnectivityPolicy {
        width_mismatch,
        ..crate::connectivity_contract::ConnectivityPolicy::default()
    };
    let connectivity = crate::connectivity::extract(schematic, |_| None, |_| None);
    append_vector_width_violations(&connectivity, &policy, &mut result, &BTreeMap::new());
    result.violations().to_vec()
}

/// The netlister refuses a mismatched join under either policy, so the check
/// does too; the permissive policy changes what the finding tells the
/// designer to do, not whether the run can carry it.
#[test]
fn a_width_mismatch_blocks_under_either_policy() {
    use crate::connectivity_contract::BundleWidthMismatchPolicy;

    // The bus is wider than the connection asks for, so an explicit slice
    // could express what the drawing means.
    let sliceable = width_mismatch_schematic("DATA[7:0]", "DATA[3:0]");
    let blocked = width_findings(&sliceable, BundleWidthMismatchPolicy::BlockConnection);
    assert_eq!(blocked.len(), 1);
    assert_eq!(
        blocked[0].violation_type,
        DrcViolationType::VectorWidthMismatch
    );
    assert_eq!(blocked[0].severity, DrcSeverity::Error);
    assert!(!blocked[0].message.contains("explicit slice"));

    let permitted = width_findings(&sliceable, BundleWidthMismatchPolicy::ExplicitSliceOrExtend);
    assert_eq!(permitted[0].severity, DrcSeverity::Error);
    assert!(
        permitted[0].message.contains("tap an explicit slice"),
        "{}",
        permitted[0].message
    );

    // A bus with too few conductors cannot be sliced into a wider
    // connection, so neither policy lets it through.
    let impossible = width_mismatch_schematic("DATA[1:0]", "DATA[3:0]");
    for policy in BundleWidthMismatchPolicy::ALL {
        let findings = width_findings(&impossible, policy);
        assert_eq!(findings.len(), 1, "{policy:?}");
        assert_eq!(findings[0].severity, DrcSeverity::Error, "{policy:?}");
        assert!(
            findings[0].message.contains("4 bits") && findings[0].message.contains("2 bits"),
            "{}",
            findings[0].message
        );
    }
}

fn off_sheet_findings(schematic: &SchematicDocument) -> Vec<DrcViolation> {
    let mut result = DrcResult::new();
    append_off_sheet_connector_violations(schematic, &mut result, &BTreeMap::new());
    result.violations().to_vec()
}

#[test]
fn a_lone_off_sheet_connector_is_advised_and_a_paired_one_is_not() {
    use crate::schematic::net_label::NetLabel;
    use rspice_design_model::design_management::CrossSheetPortDirection;

    let mut schematic = SchematicDocument::default();
    schematic.net_labels.push(NetLabel::off_sheet(
        1,
        Point::origin(),
        "BIAS",
        CrossSheetPortDirection::Output,
    ));
    schematic.net_labels.push(NetLabel::off_sheet(
        2,
        Point::new(1_000_000, 0),
        "SENSE",
        CrossSheetPortDirection::Input,
    ));
    schematic.net_labels.push(NetLabel::off_sheet(
        3,
        Point::new(2_000_000, 0),
        "SENSE",
        CrossSheetPortDirection::Output,
    ));
    // A local label of the same name is not a partner: it makes no
    // crossing claim of its own.
    schematic
        .net_labels
        .push(NetLabel::new(4, Point::new(40, 0), "BIAS"));

    let findings = off_sheet_findings(&schematic);
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].violation_type,
        DrcViolationType::OffSheetConnectorWithoutPartner
    );
    assert_eq!(findings[0].severity, DrcSeverity::Info);
    assert_eq!(
        findings[0].message,
        "Off-sheet connector `BIAS` has no partner on another sheet."
    );
    assert_eq!(
        findings[0].location,
        DrcLocation::NetLabel {
            name: "BIAS".to_owned()
        }
    );
}

#[test]
fn partner_matching_folds_case_under_every_policy_and_honors_severity_overrides() {
    use crate::schematic::document_policy::NetNamingPolicy;
    use crate::schematic::net_label::NetLabel;
    use rspice_design_model::design_management::CrossSheetPortDirection;

    let mut schematic = SchematicDocument::default();
    schematic.net_labels.push(NetLabel::off_sheet(
        1,
        Point::origin(),
        "bias",
        CrossSheetPortDirection::Output,
    ));
    schematic.net_labels.push(NetLabel::off_sheet(
        2,
        Point::new(1_000_000, 0),
        "BIAS",
        CrossSheetPortDirection::Input,
    ));

    for policy in [
        NetNamingPolicy::StrictCaseSensitive,
        NetNamingPolicy::SpiceCompatibleRelaxed,
    ] {
        schematic.document_policy.net_naming = policy;
        assert!(
            off_sheet_findings(&schematic).is_empty(),
            "{policy:?}: the netlister pairs these two, so the crossing is complete"
        );
    }

    schematic.net_labels.pop();
    let mut result = DrcResult::new();
    let overrides = BTreeMap::from([(
        DrcViolationType::OffSheetConnectorWithoutPartner,
        DrcSeverity::Error,
    )]);
    append_off_sheet_connector_violations(&schematic, &mut result, &overrides);
    assert!(result.has_errors());
}

#[test]
fn case_colliding_net_names_are_a_drc_error() {
    use crate::schematic::component_type::ComponentType;
    use crate::schematic::document_policy::NetNamingPolicy;
    use crate::schematic::net_label::NetLabel;

    let named_port = |schematic: &mut SchematicDocument, name: &str| {
        schematic.components.push(
            crate::schematic::component::Component::new(2, ComponentType::Port, Point::new(200, 0))
                .with_name_value("", name),
        );
    };

    for policy in [
        NetNamingPolicy::StrictCaseSensitive,
        NetNamingPolicy::SpiceCompatibleRelaxed,
    ] {
        let mut schematic = SchematicDocument::default();
        schematic.document_policy.net_naming = policy;
        schematic
            .net_labels
            .push(NetLabel::new(1, Point::origin(), "Out"));
        schematic
            .net_labels
            .push(NetLabel::new(2, Point::new(40, 0), "out"));

        let mut result = DrcResult::new();
        append_case_collision_violations(&schematic, &mut result, &BTreeMap::new());
        let findings = result.violations();
        assert_eq!(findings.len(), 1, "{policy:?}: one finding for one pair");
        assert_eq!(
            findings[0].violation_type,
            DrcViolationType::CaseCollidingNetNames
        );
        assert_eq!(findings[0].severity, DrcSeverity::Error);
        assert_eq!(
            findings[0].message,
            "Nets `Out` and `out` differ only by case; the netlist joins them into one node."
        );
        assert_eq!(
            findings[0].location,
            DrcLocation::NetLabel {
                name: "out".to_owned()
            }
        );
    }

    // One name, however often it is written, is one net.
    let mut repeated = SchematicDocument::default();
    repeated
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "Out"));
    repeated
        .net_labels
        .push(NetLabel::new(2, Point::new(40, 0), "Out"));
    let mut result = DrcResult::new();
    append_case_collision_violations(&repeated, &mut result, &BTreeMap::new());
    assert!(result.violations().is_empty());

    // A lone name collides with nothing.
    let mut lone = SchematicDocument::default();
    lone.net_labels
        .push(NetLabel::new(1, Point::origin(), "Out"));
    let mut result = DrcResult::new();
    append_case_collision_violations(&lone, &mut result, &BTreeMap::new());
    assert!(result.violations().is_empty());

    // An interface port carries an authored name too, and is reported on
    // the port rather than on the label that named the net first.
    let mut mixed = SchematicDocument::default();
    mixed
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "Bias"));
    named_port(&mut mixed, "BIAS");
    let mut result = DrcResult::new();
    append_case_collision_violations(&mixed, &mut result, &BTreeMap::new());
    let findings = result.violations();
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].message,
        "Nets `Bias` and `BIAS` differ only by case; the netlist joins them into one node."
    );
    assert!(matches!(
        &findings[0].location,
        DrcLocation::Component { name, .. } if name == "BIAS"
    ));
}

#[test]
fn the_advisory_continues_the_result_id_sequence_it_is_appended_to() {
    use crate::schematic::net_label::NetLabel;
    use rspice_design_model::design_management::CrossSheetPortDirection;

    let mut schematic = SchematicDocument::default();
    schematic.net_labels.push(NetLabel::off_sheet(
        1,
        Point::origin(),
        "BIAS",
        CrossSheetPortDirection::Supply,
    ));

    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        0,
        DrcViolationType::MissingGround,
        "existing finding",
        DrcLocation::Global,
    ));
    append_off_sheet_connector_violations(&schematic, &mut result, &BTreeMap::new());

    let ids: Vec<usize> = result
        .violations()
        .iter()
        .map(|violation| violation.id)
        .collect();
    assert_eq!(ids, vec![0, 1]);
}
