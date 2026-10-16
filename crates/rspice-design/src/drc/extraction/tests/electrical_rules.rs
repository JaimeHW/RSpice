//! The electrical rules a drawing is judged by beyond its attachment.
//!
//! Each rule is first held against circuits that work and must leave it
//! silent — a CMOS op-amp, a resistor divider, an RC with a coupling capacitor
//! and a body-biased MOSFET — and only then shown to fire, flat and one level
//! down. Every input goes through the extraction, so a rule is judged on the
//! nodes the deck writes.

use super::*;

/// A design checked with every rule on, the way the run gate checks it.
fn checked(top: &SchematicDocument, views: &HashMap<String, SchematicDocument>) -> DrcResult {
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let hierarchy = HierarchySource::from_workspace(&libraries, views).with_connectivity(&contract);
    run_check_with_hierarchy(top, &hierarchy, DrcConfig::default())
}

/// A MOSFET with each named terminal on the node beside it.
fn mos(
    id: u64,
    kind: ComponentType,
    name: &str,
    nodes: [&'static str; 4],
) -> (Component, Vec<(&'static str, &'static str)>) {
    let pins = ["D", "G", "S", "B"];
    (part(id, kind, name), pins.into_iter().zip(nodes).collect())
}

/// A two-terminal part from `plus` to `minus`.
fn between(
    id: u64,
    kind: ComponentType,
    name: &str,
    plus: &'static str,
    minus: &'static str,
) -> (Component, Vec<(&'static str, &'static str)>) {
    (part(id, kind, name), vec![("+", plus), ("-", minus)])
}

/// A two-stage CMOS op-amp: an NMOS pair on a tail source into a PMOS mirror,
/// a PMOS second stage with its NMOS sink, Miller compensation, a load, and
/// its supply, bias and inputs. Every bulk is tied, the NMOS ones to ground
/// whatever their source, the PMOS ones to the supply.
fn cmos_op_amp() -> SchematicDocument {
    labelled(vec![
        mos(1, ComponentType::Nmos, "M1", ["x", "inp", "tail", "0"]),
        mos(2, ComponentType::Nmos, "M2", ["y", "inn", "tail", "0"]),
        mos(3, ComponentType::Pmos, "M3", ["x", "x", "vdd", "vdd"]),
        mos(4, ComponentType::Pmos, "M4", ["y", "x", "vdd", "vdd"]),
        mos(5, ComponentType::Nmos, "M5", ["tail", "bias", "0", "0"]),
        mos(6, ComponentType::Pmos, "M6", ["out", "y", "vdd", "vdd"]),
        mos(7, ComponentType::Nmos, "M7", ["out", "bias", "0", "0"]),
        mos(8, ComponentType::Nmos, "M8", ["bias", "bias", "0", "0"]),
        between(9, ComponentType::Capacitor, "CC", "y", "out"),
        between(10, ComponentType::Capacitor, "CL", "out", "0"),
        between(11, ComponentType::CurrentSource, "IBIAS", "vdd", "bias"),
        between(12, ComponentType::VoltageSource, "VDD", "vdd", "0"),
        between(13, ComponentType::VoltageSource, "VINP", "inp", "0"),
        between(14, ComponentType::VoltageSource, "VINN", "inn", "0"),
    ])
}

fn resistor_divider() -> SchematicDocument {
    labelled(vec![
        between(1, ComponentType::VoltageSource, "V1", "vin", "0"),
        between(2, ComponentType::Resistor, "R1", "vin", "mid"),
        between(3, ComponentType::Resistor, "R2", "mid", "0"),
    ])
}

/// An RC low-pass whose output is AC-coupled into a load, so one node is
/// reached from the source only through a capacitor.
fn rc_with_coupling_capacitor() -> SchematicDocument {
    labelled(vec![
        between(1, ComponentType::VoltageSourceSin, "V1", "vin", "0"),
        between(2, ComponentType::Resistor, "R1", "vin", "a"),
        between(3, ComponentType::Capacitor, "C1", "a", "0"),
        between(4, ComponentType::Capacitor, "CC", "a", "b"),
        between(5, ComponentType::Resistor, "RL", "b", "0"),
    ])
}

/// A common-source NMOS whose bulk sits on a negative rail of its own rather
/// than on its source: deliberate body bias.
fn body_biased_mos() -> SchematicDocument {
    labelled(vec![
        mos(1, ComponentType::Nmos, "M1", ["out", "gate", "0", "vbb"]),
        between(2, ComponentType::Resistor, "RD", "vdd", "out"),
        between(3, ComponentType::VoltageSource, "VDD", "vdd", "0"),
        between(4, ComponentType::VoltageSource, "VG", "gate", "0"),
        between(5, ComponentType::VoltageSource, "VBB", "0", "vbb"),
    ])
}

fn working_circuits() -> [(&'static str, SchematicDocument); 4] {
    [
        ("CMOS op-amp", cmos_op_amp()),
        ("resistor divider", resistor_divider()),
        ("RC with a coupling capacitor", rc_with_coupling_capacitor()),
        ("body-biased MOSFET", body_biased_mos()),
    ]
}

/// The findings about a circuit that works must not exist: none of the
/// rules here, no open terminal and no floating node.
#[test]
fn working_circuits_raise_no_electrical_finding() {
    for (circuit, drawing) in working_circuits() {
        let result = checked(&drawing, &buffers([]));
        let wrong = result
            .violations()
            .iter()
            .filter(|finding| {
                matches!(
                    finding.violation_type,
                    DrcViolationType::UnconnectedPin
                        | DrcViolationType::UndrivenInput
                        | DrcViolationType::FloatingNode
                        | DrcViolationType::MissingGround
                        | DrcViolationType::SourceToSource
                )
            })
            .collect::<Vec<_>>();
        assert!(wrong.is_empty(), "{circuit}: {wrong:?}");
    }
}

/// The common-source stage with its bulk left open.
fn open_bulk_stage() -> SchematicDocument {
    labelled(vec![
        (
            part(1, ComponentType::Nmos, "M1"),
            vec![("D", "out"), ("G", "gate"), ("S", "0")],
        ),
        between(2, ComponentType::Resistor, "RD", "vdd", "out"),
        between(3, ComponentType::VoltageSource, "VDD", "vdd", "0"),
        between(4, ComponentType::VoltageSource, "VG", "gate", "0"),
    ])
}

/// The findings located at one terminal of the drawing, wherever it is
/// stated.
fn at_terminal<'a>(result: &'a DrcResult, pin: &str) -> Vec<&'a DrcViolation> {
    result
        .violations()
        .iter()
        .filter(|finding| {
            let location = match &finding.location {
                DrcLocation::InsideInstance { inner, .. } => inner.as_ref(),
                location => location,
            };
            matches!(location, DrcLocation::Terminal { pin_name, .. } if pin_name == pin)
        })
        .collect()
}

const OPEN_BULK: &str =
    "Bulk pin M1.B is not connected; tie it to the source or to its well or substrate rail";

/// An open bulk is one open terminal, stated once with where to tie it. The
/// deck writes it as a node only the device's junctions reach and the engine
/// solves it, so it stays a warning, and its node is not reported again as
/// floating.
#[test]
fn an_open_bulk_is_one_open_pin_that_says_where_to_tie_it() {
    let result = checked(&open_bulk_stage(), &buffers([]));
    let bulk = at_terminal(&result, "B");
    assert_eq!(bulk.len(), 1, "{:?}", result.violations());
    assert_eq!(bulk[0].violation_type, DrcViolationType::UnconnectedPin);
    assert_eq!(bulk[0].severity, DrcSeverity::Warning);
    assert_eq!(bulk[0].message, OPEN_BULK);
    assert!(
        of_type(&result, DrcViolationType::FloatingNode).is_empty(),
        "{:?}",
        result.violations()
    );

    // An SOI device's fourth node is its substrate; its body tie may float.
    let soi = labelled(vec![
        (
            part(1, ComponentType::NmosSoi, "M1"),
            vec![("D", "vdd"), ("G", "vdd"), ("S", "0")],
        ),
        between(2, ComponentType::VoltageSource, "VDD", "vdd", "0"),
    ]);
    let result = checked(&soi, &buffers([]));
    let messages = |pin: &str| {
        at_terminal(&result, pin)
            .iter()
            .map(|finding| finding.message.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        messages("E"),
        ["Bulk pin M1.E is not connected; tie it to the source or to its well or substrate rail"]
    );
    assert_eq!(messages("P"), ["Pin M1.P is not connected"]);

    // A bulk on its own rail is body bias, which nothing judges.
    let result = checked(&body_biased_mos(), &buffers([]));
    assert!(
        at_terminal(&result, "B").is_empty(),
        "{:?}",
        result.violations()
    );
}

/// Inside a placed cell, the open bulk is stated at the occurrence.
#[test]
fn an_open_bulk_below_the_subject_is_stated_inside_its_occurrence() {
    let views = buffers([("stage", open_bulk_stage())]);
    let top = SchematicDocument {
        components: vec![instance(1, "X1", "stage")],
        ..SchematicDocument::default()
    };
    let result = checked(&top, &views);
    let bulk = at_terminal(&result, "B");
    assert_eq!(bulk.len(), 1, "{:?}", result.violations());
    assert_eq!(bulk[0].message, OPEN_BULK);
    let DrcLocation::InsideInstance { path, view, .. } = &bulk[0].location else {
        panic!("stated inside the occurrence: {:?}", bulk[0].location);
    };
    assert_eq!(
        (path.to_string(), view.key()),
        ("/X1".to_owned(), "work/stage/schematic".to_owned())
    );
}

