//! What each rule decides, given an already-extracted design.
//!
//! The inputs here are built by hand because the subject is the rule, not the
//! extraction: a component with a named terminal on a node stands for whatever
//! drawing produced it. That the extraction really does produce these inputs —
//! that the checker and the deck agree on every terminal of every fixture — is
//! proved in `services::drc::extraction::tests`, which is where the geometry
//! lives.

use super::*;
use crate::services::drc::input::{ParameterRangeIssue, PinInfo};
use crate::state::Point;

/// A terminal that something else in the drawing meets.
fn pin(name: &str, net: &str) -> PinInfo {
    PinInfo {
        name: name.to_string(),
        net_name: net.to_string(),
        is_output: false,
        point: Point::origin(),
        attached: true,
    }
}

/// A terminal the extraction found nothing meeting.
fn open_pin(name: &str, net: &str, point: Point) -> PinInfo {
    PinInfo {
        name: name.to_string(),
        net_name: net.to_string(),
        is_output: false,
        point,
        attached: false,
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
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
    }
}

fn vsource(id: u64, name: &str, plus_net: &str, minus_net: &str) -> ComponentInfo {
    let mut plus = pin("+", plus_net);
    plus.is_output = true;
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
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
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
            is_output: false,
            point,
            attached,
        }],
        is_voltage_source: false,
        is_current_source: false,
        is_ground_symbol: true,
        reference_required: false,
        reference_error: None,
        component_known: Some(true),
        missing_parameters: Vec::new(),
        out_of_range_parameters: Vec::new(),
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
    assert_eq!(floating[0].severity, DrcSeverity::Error);
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
fn two_voltage_sources_on_one_net_are_source_to_source() {
    let components = vec![
        vsource(0, "V1", "vin", "0"),
        vsource(1, "V2", "vin", "0"),
        resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
    ];
    let result = check(&components, DrcConfig::default());
    let shorted = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(shorted.len(), 1);
    assert_eq!(
        shorted[0].location,
        DrcLocation::Node {
            net_name: "vin".to_string()
        }
    );
    let mut related = shorted[0].related_items.clone();
    related.sort();
    assert_eq!(related, vec!["V1".to_string(), "V2".to_string()]);
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
    first.pins[0].is_output = true;
    let mut second = resistor(2, "X2", vec![pin("OUT", "n1")]);
    second.pins[0].is_output = true;
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
        DrcLocation::Component {
            id: 7,
            name: "R7".to_owned()
        }
    );

    let mut disabled = base;
    disabled.check_unconnected_pins = false;
    let result = check(&components, disabled);
    assert!(of_type(&result, DrcViolationType::UnconnectedPin).is_empty());
}

#[test]
fn exact_reference_contract_emits_empty_and_invalid_names_only_when_required() {
    let mut empty = resistor(3, "", vec![]);
    let mut invalid = resistor(4, "C4", vec![]);
    invalid.reference_error = Some("Resistor designators must begin with `R`.".to_owned());
    let mut structural = resistor(5, "", vec![]);
    structural.reference_required = false;
    structural.component_type = "port".to_owned();
    empty.reference_error = None;
    let result = check(
        &[empty, invalid, structural],
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );
    assert_eq!(of_type(&result, DrcViolationType::EmptyName).len(), 1);
    assert_eq!(of_type(&result, DrcViolationType::InvalidName).len(), 1);
    assert_eq!(of_type(&result, DrcViolationType::DuplicateName).len(), 0);
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
    assert_eq!(unknown.len(), 1);
    assert_eq!(
        unknown[0].location,
        DrcLocation::Component {
            id: 9,
            name: "X9".to_owned()
        }
    );

    let mut disabled = config;
    disabled.check_unknown_components = false;
    let result = check(&[unresolved], disabled);
    assert!(of_type(&result, DrcViolationType::UnknownComponent).is_empty());
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

    components[2].name = "R2".to_string();
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
    let components = vec![
        // No ground (critical), floating "out" (error), duplicate name (critical).
        resistor(0, "R1", vec![pin("1", "a"), pin("2", "out")]),
        resistor(1, "R1", vec![pin("1", "a"), pin("2", "b")]),
        resistor(2, "R2", vec![pin("1", "a"), pin("2", "b")]),
    ];
    let result = check(&components, DrcConfig::default());
    let summary = result.summary();
    assert_eq!(summary.total, result.total_count());
    assert_eq!(summary.critical, 2);
    assert_eq!(summary.errors, 1);
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
fn width_mismatch_schematic(bus: &str, port: &str) -> crate::state::SchematicState {
    use crate::state::{Bus, BusDeclaration, ComponentType};

    let mut schematic = crate::state::SchematicState::default();
    schematic.buses.push(
        Bus::segment(
            1,
            Point::new(90, 0),
            Point::new(150, 0),
            Some(BusDeclaration::parse(bus).expect("fixture bus")),
        )
        .expect("fixture bus geometry"),
    );
    let id = schematic.add_component(ComponentType::Port, Point::new(100, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed port")
        .value = port.to_owned();
    schematic
}

fn width_findings(
    schematic: &crate::state::SchematicState,
    width_mismatch: crate::state::BundleWidthMismatchPolicy,
) -> Vec<DrcViolation> {
    let mut result = DrcResult::new();
    let policy = crate::state::ConnectivityPolicy {
        width_mismatch,
        ..crate::state::ConnectivityPolicy::default()
    };
    let connectivity = super::super::netlist_gen::extraction::extract(schematic, None);
    append_vector_width_violations(&connectivity, &policy, &mut result, &HashMap::new());
    result.violations().to_vec()
}

#[test]
fn the_width_mismatch_policy_decides_whether_a_narrow_bus_blocks() {
    use crate::state::BundleWidthMismatchPolicy;

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

    let permitted = width_findings(&sliceable, BundleWidthMismatchPolicy::ExplicitSliceOrExtend);
    assert_eq!(permitted[0].severity, DrcSeverity::Warning);

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

fn off_sheet_findings(schematic: &crate::state::SchematicState) -> Vec<DrcViolation> {
    let mut result = DrcResult::new();
    append_off_sheet_connector_violations(schematic, &mut result, &HashMap::new());
    result.violations().to_vec()
}

#[test]
fn a_lone_off_sheet_connector_is_advised_and_a_paired_one_is_not() {
    use crate::state::{CrossSheetPortDirection, NetLabel};

    let mut schematic = crate::state::SchematicState::default();
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
    use crate::state::{CrossSheetPortDirection, NetLabel, NetNamingPolicy};

    let mut schematic = crate::state::SchematicState::default();
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
    let overrides = HashMap::from([(
        DrcViolationType::OffSheetConnectorWithoutPartner,
        DrcSeverity::Error,
    )]);
    append_off_sheet_connector_violations(&schematic, &mut result, &overrides);
    assert!(result.has_errors());
}

#[test]
fn case_colliding_net_names_are_a_drc_error() {
    use crate::state::{ComponentType, NetLabel, NetNamingPolicy, SchematicState};

    let named_port = |schematic: &mut SchematicState, name: &str| {
        let id = schematic.add_component(ComponentType::Port, Point::new(200, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("placed port")
            .value = name.to_owned();
    };

    for policy in [
        NetNamingPolicy::StrictCaseSensitive,
        NetNamingPolicy::SpiceCompatibleRelaxed,
    ] {
        let mut schematic = SchematicState::default();
        schematic.document_policy.net_naming = policy;
        schematic
            .net_labels
            .push(NetLabel::new(1, Point::origin(), "Out"));
        schematic
            .net_labels
            .push(NetLabel::new(2, Point::new(40, 0), "out"));

        let mut result = DrcResult::new();
        append_case_collision_violations(&schematic, &mut result, &HashMap::new());
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
    let mut repeated = SchematicState::default();
    repeated
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "Out"));
    repeated
        .net_labels
        .push(NetLabel::new(2, Point::new(40, 0), "Out"));
    let mut result = DrcResult::new();
    append_case_collision_violations(&repeated, &mut result, &HashMap::new());
    assert!(result.violations().is_empty());

    // A lone name collides with nothing.
    let mut lone = SchematicState::default();
    lone.net_labels
        .push(NetLabel::new(1, Point::origin(), "Out"));
    let mut result = DrcResult::new();
    append_case_collision_violations(&lone, &mut result, &HashMap::new());
    assert!(result.violations().is_empty());

    // An interface port carries an authored name too, and is reported on
    // the port rather than on the label that named the net first.
    let mut mixed = SchematicState::default();
    mixed
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "Bias"));
    named_port(&mut mixed, "BIAS");
    let mut result = DrcResult::new();
    append_case_collision_violations(&mixed, &mut result, &HashMap::new());
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
    use crate::state::{CrossSheetPortDirection, NetLabel};

    let mut schematic = crate::state::SchematicState::default();
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
    append_off_sheet_connector_violations(&schematic, &mut result, &HashMap::new());

    let ids: Vec<usize> = result
        .violations()
        .iter()
        .map(|violation| violation.id)
        .collect();
    assert_eq!(ids, vec![0, 1]);
}
