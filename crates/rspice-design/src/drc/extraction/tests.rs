//! Independent source classification, and what the hierarchy walk checks.

use super::*;
use crate::connectivity::terminal_positions_with_hierarchy;
use crate::schematic::component::LibraryCellInstance;
use crate::schematic::wire::Wire;
use rspice_design_model::Point;
use std::collections::HashMap;

/// A cell instance bound to one project cellview.
fn instance(id: u64, name: &str, cell: &str) -> Component {
    Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
        .with_library_cell(LibraryCellInstance::new("work", cell, "schematic"))
        .with_name_value(name, cell)
}

/// A primitive whose two legs the drawing leaves open.
fn open_resistor(id: u64, name: &str) -> Component {
    Component::new(id, ComponentType::Resistor, Point::new(0, 0)).with_name_value(name, "1k")
}

/// A resistor wired to a ground symbol: node 0 bound by something other than
/// the symbol's own terminal, which is what the ground rule asks for.
fn grounded_resistor() -> SchematicDocument {
    let resistor = open_resistor(1, "R1");
    let ground = Component::new(2, ComponentType::Ground, Point::new(0, 200));
    let wire = Wire::segment(
        10,
        terminal_positions_with_hierarchy(&resistor, None)[1].1,
        terminal_positions_with_hierarchy(&ground, None)[0].1,
    );
    SchematicDocument {
        components: vec![resistor, ground],
        wires: vec![wire],
        ..SchematicDocument::default()
    }
}

fn buffers(
    views: impl IntoIterator<Item = (&'static str, SchematicDocument)>,
) -> HashMap<String, SchematicDocument> {
    views
        .into_iter()
        .map(|(cell, document)| (format!("work/{cell}/schematic"), document))
        .collect()
}

fn connectivity_only() -> DrcConfig {
    DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    }
}

fn of_type(result: &DrcResult, violation_type: DrcViolationType) -> Vec<&DrcViolation> {
    result
        .violations()
        .iter()
        .filter(|violation| violation.violation_type == violation_type)
        .collect()
}

/// The walk checks the cellviews a design instantiates, and says which
/// occurrence each finding belongs to. Before it did, a hierarchical design
/// passed with every defect below the subject invisible.
#[test]
fn a_sub_cellviews_findings_are_stated_at_every_occurrence() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X1", "amp"));
    top.components.push(instance(2, "X2", "amp"));
    let mut amp = SchematicDocument::default();
    amp.components.push(open_resistor(1, "R1"));
    let views = buffers([("amp", amp)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(open.len(), 4, "two legs at each of two occurrences");
    let mut paths = open
        .iter()
        .map(|finding| match &finding.location {
            DrcLocation::InsideInstance { path, view, inner } => {
                assert_eq!(view.key(), "work/amp/schematic");
                assert!(
                    matches!(inner.as_ref(), DrcLocation::Terminal { .. }),
                    "{inner:?}"
                );
                path.to_string()
            }
            other => panic!("a sub-cell finding carries its occurrence path: {other:?}"),
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    assert_eq!(paths, vec!["/X1".to_owned(), "/X2".to_owned()]);
    // The report names the terminal inside the occurrence, which is the whole
    // point of carrying the path.
    assert!(
        open[0]
            .location
            .display()
            .starts_with("/X1 \u{00b7} Pin R1."),
        "{}",
        open[0].location.display()
    );
}

/// A cellview placed more times than the report lists is stated once, with the
/// exact count. Repeating it would bury every other finding, and dropping the
/// occurrences silently would understate the design.
#[test]
fn a_flood_of_occurrences_is_counted_rather_than_repeated() {
    let mut top = SchematicDocument::default();
    for id in 1..=12u64 {
        top.components.push(instance(id, &format!("X{id}"), "amp"));
    }
    let mut amp = SchematicDocument::default();
    amp.components.push(open_resistor(1, "R1"));
    let views = buffers([("amp", amp)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(
        open.len(),
        2,
        "one finding per open leg, not per occurrence"
    );
    assert!(
        open.iter().all(|finding| finding
            .message
            .contains("(12 occurrences; reported once, at /X1)")),
        "{:?}",
        open.iter()
            .map(|finding| &finding.message)
            .collect::<Vec<_>>()
    );
}

/// A cellview that reaches itself has no finite expansion. The walk states it
/// and stops rather than following the cycle.
#[test]
fn a_cycle_is_reported_rather_than_followed() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "XA", "a"));
    let mut a = SchematicDocument::default();
    a.components.push(instance(1, "XB", "b"));
    let mut b = SchematicDocument::default();
    b.components.push(instance(1, "XA", "a"));
    let views = buffers([("a", a), ("b", b)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let recursive = of_type(&result, DrcViolationType::RecursiveInstantiation);
    assert_eq!(recursive.len(), 1, "{:?}", result.violations());
    assert_eq!(recursive[0].severity, DrcSeverity::Critical);
    assert!(
        recursive[0]
            .message
            .contains("work/a/schematic -> work/b/schematic -> work/a/schematic"),
        "{}",
        recursive[0].message
    );
    assert!(matches!(
        &recursive[0].location,
        DrcLocation::InsideInstance { path, .. } if path.to_string() == "/XA/XB"
    ));
}

/// A cellview that instantiates itself directly terminates too.
#[test]
fn a_self_referential_cellview_terminates() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "XA", "a"));
    let mut a = SchematicDocument::default();
    a.components.push(instance(1, "XA", "a"));
    let views = buffers([("a", a)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    assert_eq!(
        of_type(&result, DrcViolationType::RecursiveInstantiation).len(),
        1,
        "{:?}",
        result.violations()
    );
}

/// The reference node is global, so a ground symbol inside a sub-cell grounds
/// the design above it. Asking the rule per cellview instead would fail the
/// top-level design of every correct hierarchy whose ground lives one level
/// down, and would fail every reusable cell that takes its reference from the
/// parent.
#[test]
fn the_ground_rule_is_asked_once_over_the_whole_hierarchy() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X1", "bias"));
    let views = buffers([("bias", grounded_resistor())]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let grounded_below = run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default());
    assert!(
        of_type(&grounded_below, DrcViolationType::MissingGround).is_empty(),
        "{:?}",
        grounded_below.violations()
    );

    let ungrounded = buffers([(
        "bias",
        SchematicDocument {
            components: vec![open_resistor(1, "R1")],
            ..SchematicDocument::default()
        },
    )]);
    let hierarchy = HierarchySource::from_buffers(&ungrounded);
    let result = run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default());
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(
        missing.len(),
        1,
        "one design, one verdict: {:?}",
        result.violations()
    );
    assert_eq!(missing[0].location, DrcLocation::Global);
}

/// Every independent source is claimed by exactly one side of the pair, and
/// nothing else is claimed at all.
///
/// `VoltageSourcePwlFile` was missing from [`is_voltage_source`] while
/// `CurrentSourcePwlFile` was present in [`is_current_source`], so a voltage
/// source driven from a PWL file was not a driver as far as any rule that asks
/// whether a net has one — an asymmetry, not a decision. The oracle is the
/// deck: an independent source is a placed source whose SPICE card starts with
/// `V` or `I`, which is exactly what the emitter writes, so a family added
/// tomorrow is classified by this test before anyone has to remember the pair.
#[test]
fn every_independent_source_is_claimed_by_exactly_one_side() {
    for kind in ComponentType::ALL {
        let voltage = is_voltage_source(kind);
        let current = is_current_source(kind);
        // `LoopProbe` writes a `V` card and is not a source: it is a 0 V
        // probe, and the rules must not read it as a driver.
        let independent = kind.is_source().then(|| kind.spice_prefix());
        match independent {
            Some("V") => assert!(
                voltage && !current,
                "{kind:?} writes a V card and is a source; only the voltage \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            Some("I") => assert!(
                current && !voltage,
                "{kind:?} writes an I card and is a source; only the current \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            // A dependent or behavioural source drives through an expression
            // rather than by itself, and a non-source drives nothing.
            _ => assert!(
                !voltage && !current,
                "{kind:?} is not an independent source and must be claimed by \
                 neither side (voltage {voltage}, current {current})"
            ),
        }
    }
}
