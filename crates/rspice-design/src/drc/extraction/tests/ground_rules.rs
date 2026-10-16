//! Which net is the reference node, judged by what the deck writes.
//!
//! The netlister writes a net named `0`, `GND` or `GROUND` as node 0 and a
//! global spelled `GND!` as a node of its own — unless the project's
//! technology global-net catalog promotes `GND!` onto `GND`. The canvas reads
//! all four as ground, and every rule here is held to the deck instead: with
//! the catalog off and on, flat and one level down.

use super::*;
use crate::connectivity_contract::{
    ConnectivityAliasGroup, ConnectivityContract, ConnectivityPolicy, GlobalNetPromotionPolicy,
    TechnologyGlobalNetCatalog,
};

/// The project contract every new project starts with: `GND!` is a global
/// declared by its own `!`, and nothing promotes it onto ground.
fn unaliased() -> ConnectivityContract {
    ConnectivityContract::default()
}

/// A contract whose technology global-net catalog promotes `GND!` onto
/// `GND`, which the deck writes as node 0, and `VDD!` onto `VDD`.
fn aliased() -> ConnectivityContract {
    let contract = ConnectivityContract {
        policy: ConnectivityPolicy {
            global_promotion: GlobalNetPromotionPolicy::TechnologyDefinedOnly,
            ..ConnectivityPolicy::default()
        },
        technology_global_nets: Some(TechnologyGlobalNetCatalog {
            authority: "demo-pdk@1.0".to_owned(),
            nets: ["GND", "VDD"]
                .map(|canonical_name| ConnectivityAliasGroup {
                    canonical_name: canonical_name.to_owned(),
                    aliases: Vec::new(),
                })
                .to_vec(),
        }),
        ..ConnectivityContract::default()
    };
    contract.validate().expect("the catalog is valid");
    contract
}

/// `top` checked with every rule on under `contract`, the way the run gate
/// checks it. A global is declared by a label in the project's views, so the
/// views that label one are among `views`.
fn checked_under(
    contract: &ConnectivityContract,
    top: &SchematicDocument,
    views: &HashMap<String, SchematicDocument>,
) -> DrcResult {
    let libraries = LibraryCatalog::default();
    let hierarchy = HierarchySource::from_workspace(&libraries, views).with_connectivity(contract);
    run_check_with_hierarchy(top, &hierarchy, DrcConfig::default())
}

/// A two-terminal part from `plus` to `minus`.
fn two(
    id: u64,
    kind: ComponentType,
    name: &str,
    plus: &'static str,
    minus: &'static str,
) -> (Component, Vec<(&'static str, &'static str)>) {
    (part(id, kind, name), vec![("+", plus), ("-", minus)])
}

/// A source and its load, returned only through `GND!`.
fn grounded_by_bang() -> SchematicDocument {
    labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "GND!"),
        two(2, ComponentType::Resistor, "R1", "in", "GND!"),
    ])
}

/// V1 from `vdd` to `0` beside V2 from `vdd` to `ground`, with a load.
fn sources_across(ground: &'static str) -> SchematicDocument {
    labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "vdd", "0"),
        two(2, ComponentType::VoltageSource, "V2", "vdd", ground),
        two(3, ComponentType::Resistor, "RL", "vdd", "0"),
    ])
}

/// R2 with its `+` wired to a ground symbol and labelled `label` there, and
/// its `-` labelled `x`.
fn tied_to_ground(label: &'static str) -> SchematicDocument {
    let resistor = part(1, ComponentType::Resistor, "R2");
    let ground = Component::new(2, ComponentType::Ground, Point::new(0, 200));
    let terminal = |pin: &str| {
        terminal_positions_with_hierarchy(&resistor, None)
            .into_iter()
            .find_map(|(name, point)| (name == pin).then_some(point))
            .expect("a resistor has both terminals")
    };
    let (plus, minus) = (terminal("+"), terminal("-"));
    let wire = Wire::segment(
        10,
        plus,
        terminal_positions_with_hierarchy(&ground, None)[0].1,
    );
    SchematicDocument {
        components: vec![resistor, ground],
        wires: vec![wire],
        net_labels: vec![
            NetLabel::new(20, plus, label),
            NetLabel::new(21, minus, "x"),
        ],
        ..SchematicDocument::default()
    }
}

/// The top of a design whose only content is one instance of `cell`.
fn only(cell: &str) -> SchematicDocument {
    SchematicDocument {
        components: vec![instance(1, "X1", cell)],
        ..SchematicDocument::default()
    }
}

/// A design returned only through `GND!` has no node 0 in its deck, so it
/// has no ground; promoted onto `GND`, `GND!` is node 0 and grounds it.
#[test]
fn a_design_returned_only_through_an_unpromoted_ground_global_has_no_ground() {
    let views = buffers([("top", grounded_by_bang())]);
    let top = &views["work/top/schematic"];

    let result = checked_under(&unaliased(), top, &views);
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(missing.len(), 1, "{:?}", result.violations());
    assert_eq!(missing[0].location, DrcLocation::Global);

    let result = checked_under(&aliased(), top, &views);
    assert!(
        of_type(&result, DrcViolationType::MissingGround).is_empty(),
        "{:?}",
        result.violations()
    );
}

/// Node 0 is global, so the design's ground is asked over the whole walk —
/// and `GND!` drawn inside a cell is no more node 0 there than at the top.
#[test]
fn an_unpromoted_ground_global_below_the_subject_grounds_nothing() {
    let views = buffers([("leg", grounded_by_bang())]);

    let result = checked_under(&unaliased(), &only("leg"), &views);
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(missing.len(), 1, "{:?}", result.violations());

    let result = checked_under(&aliased(), &only("leg"), &views);
    assert!(
        of_type(&result, DrcViolationType::MissingGround).is_empty(),
        "{:?}",
        result.violations()
    );
}

/// V1 from `vdd` to `0` beside V2 from `vdd` to `GND!` is two sources into
/// two nodes of the deck, which the engine solves. Spelled as the deck writes
/// node 0, or with `GND!` promoted onto it, the same pair is a loop.
#[test]
fn sources_into_node_zero_and_into_an_unpromoted_ground_global_close_no_loop() {
    let views = buffers([("top", sources_across("GND!"))]);
    let top = &views["work/top/schematic"];
    let result = checked_under(&unaliased(), top, &views);
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    let result = checked_under(&aliased(), top, &views);
    assert_eq!(
        of_type(&result, DrcViolationType::SourceToSource).len(),
        1,
        "{:?}",
        result.violations()
    );

    let views = buffers([("top", sources_across("GND"))]);
    let result = checked_under(&unaliased(), &views["work/top/schematic"], &views);
    assert_eq!(
        of_type(&result, DrcViolationType::SourceToSource).len(),
        1,
        "{:?}",
        result.violations()
    );
}

/// Across cellviews the same holds: V2 in a cell from `VDD!` to `GND!`
/// closes a loop with V1 at the top only where `GND!` is node 0.
#[test]
fn a_loop_across_cellviews_closes_only_through_node_zero() {
    let views = buffers([(
        "rail",
        labelled(vec![two(
            1,
            ComponentType::VoltageSource,
            "V2",
            "VDD!",
            "GND!",
        )]),
    )]);
    let mut top = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "VDD!", "0"),
        two(2, ComponentType::Resistor, "RL", "VDD!", "0"),
    ]);
    top.components.push(instance(9, "X1", "rail"));

    let result = checked_under(&unaliased(), &top, &views);
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    let result = checked_under(&aliased(), &top, &views);
    assert_eq!(
        of_type(&result, DrcViolationType::SourceToSource).len(),
        1,
        "{:?}",
        result.violations()
    );
}

const BANG_NOT_ZERO: &str =
    "Net label 'GND!' reads as ground, but the deck writes its net as node 'GND!', not as node 0";

/// The drawing says ground where the deck writes a node of its own: once per
/// node, at the label that says it. Promoted onto `GND`, it is node 0.
#[test]
fn a_ground_global_the_deck_writes_as_its_own_node_is_an_error() {
    let views = buffers([("top", grounded_by_bang())]);
    let top = &views["work/top/schematic"];

    let result = checked_under(&unaliased(), top, &views);
    let stray = of_type(&result, DrcViolationType::GroundNotNodeZero);
    assert_eq!(stray.len(), 1, "{:?}", result.violations());
    assert_eq!(stray[0].severity, DrcSeverity::Error);
    assert_eq!(stray[0].message, BANG_NOT_ZERO);
    assert_eq!(
        stray[0].location,
        DrcLocation::NetLabel {
            name: "GND!".to_owned()
        }
    );

    let result = checked_under(&aliased(), top, &views);
    assert!(
        of_type(&result, DrcViolationType::GroundNotNodeZero).is_empty(),
        "{:?}",
        result.violations()
    );
}

/// Inside a placed cell, the label is stated at the occurrence.
#[test]
fn a_ground_global_below_the_subject_is_stated_inside_its_occurrence() {
    let views = buffers([("leg", grounded_by_bang())]);

    let result = checked_under(&unaliased(), &only("leg"), &views);
    let stray = of_type(&result, DrcViolationType::GroundNotNodeZero);
    assert_eq!(stray.len(), 1, "{:?}", result.violations());
    let DrcLocation::InsideInstance { path, view, inner } = &stray[0].location else {
        panic!("stated inside the occurrence: {:?}", stray[0].location);
    };
    assert_eq!(
        (path.to_string(), view.key(), inner.as_ref().clone()),
        (
            "/X1".to_owned(),
            "work/leg/schematic".to_owned(),
            DrcLocation::NetLabel {
                name: "GND!".to_owned()
            }
        )
    );

    let result = checked_under(&aliased(), &only("leg"), &views);
    assert!(
        of_type(&result, DrcViolationType::GroundNotNodeZero).is_empty(),
        "{:?}",
        result.violations()
    );
}

/// A name the deck writes as node 0 is ground however it is spelled, and so
/// is a `GND!` that shares its net with a ground symbol, which names the net
/// `0`. A `GND` label whose net an earlier label already named is not: the
/// deck writes that name.
#[test]
fn a_ground_name_is_judged_by_the_node_its_net_is_written_as() {
    let stray = |drawing: SchematicDocument| {
        let views = buffers([("top", drawing)]);
        let result = checked_under(&unaliased(), &views["work/top/schematic"], &views);
        of_type(&result, DrcViolationType::GroundNotNodeZero)
            .into_iter()
            .map(|finding| finding.message.clone())
            .collect::<Vec<_>>()
    };
    for ground in ["0", "GND", "gnd", "Ground"] {
        let returned = labelled(vec![
            two(1, ComponentType::VoltageSource, "V1", "in", ground),
            two(2, ComponentType::Resistor, "R1", "in", ground),
        ]);
        assert_eq!(stray(returned), Vec::<String>::new(), "{ground}");
    }
    assert_eq!(stray(tied_to_ground("GND!")), Vec::<String>::new());

    let mut renamed = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "VSS"),
        two(2, ComponentType::Resistor, "R1", "in", "0"),
    ]);
    let minus = renamed.net_labels[1].pos;
    renamed.net_labels.push(NetLabel::new(900, minus, "GND"));
    assert_eq!(
        stray(renamed),
        [
            "Net label 'GND' reads as ground, but the deck writes its net as node 'VSS', not as node 0"
        ]
    );
}

/// The top of a design that drives `VDD!` from V1 into a load, beside an
/// instance of `cell`.
fn supplied_beside(cell: &str) -> SchematicDocument {
    let mut top = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "VDD!", "0"),
        two(2, ComponentType::Resistor, "RL", "VDD!", "0"),
    ]);
    top.components.push(instance(9, "X1", cell));
    top
}

/// A ground symbol names its net `0` over every label on it, so a cell that
/// ties `VDD!` to one writes that net as node 0 while the top writes `VDD!` as
/// the rail. The deck shorts nothing — no source closes a loop — and the tie
/// the drawing shows is simply not there.
#[test]
fn a_supply_global_tied_to_ground_in_one_cellview_is_an_error() {
    let views = buffers([("tie", tied_to_ground("VDD!"))]);
    let result = checked_under(&unaliased(), &supplied_beside("tie"), &views);

    let tied = of_type(&result, DrcViolationType::SupplyTiedToGround);
    assert_eq!(tied.len(), 1, "{:?}", result.violations());
    assert_eq!(tied[0].severity, DrcSeverity::Error);
    assert_eq!(
        tied[0].message,
        "Global net 'VDD!' is tied to ground here: the deck writes this net as node 0, apart \
         from 'VDD!' in the rest of the design"
    );
    let DrcLocation::InsideInstance { path, view, inner } = &tied[0].location else {
        panic!("stated inside the occurrence: {:?}", tied[0].location);
    };
    assert_eq!(
        (path.to_string(), view.key(), inner.as_ref().clone()),
        (
            "/X1".to_owned(),
            "work/tie/schematic".to_owned(),
            DrcLocation::NetLabel {
                name: "VDD!".to_owned()
            }
        )
    );
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );
}

/// A global no other cellview names is only another name for node 0 where it
/// is tied. Tied in the cellview that drives it, every label of the name
/// joins the one net, the source then stands across node 0, and that is the
/// source rule's to say. A ground global on a ground symbol's net is ground.
#[test]
fn a_global_tied_to_ground_is_a_finding_only_where_the_design_parts_it() {
    let tied = |top: &SchematicDocument, views: &HashMap<String, SchematicDocument>| {
        let result = checked_under(&unaliased(), top, views);
        (
            of_type(&result, DrcViolationType::SupplyTiedToGround).len(),
            of_type(&result, DrcViolationType::SourceToSource).len(),
        )
    };

    let views = buffers([("tie", tied_to_ground("VDD!"))]);
    let mut alone = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "0"),
        two(2, ComponentType::Resistor, "RL", "in", "0"),
    ]);
    alone.components.push(instance(9, "X1", "tie"));
    assert_eq!(tied(&alone, &views), (0, 0));

    let mut flat = tied_to_ground("VDD!");
    let driven = labelled(vec![
        two(5, ComponentType::VoltageSource, "V1", "VDD!", "0"),
        two(6, ComponentType::Resistor, "RL", "VDD!", "0"),
    ]);
    flat.components.extend(driven.components);
    flat.net_labels.extend(driven.net_labels);
    let views = buffers([("top", flat)]);
    assert_eq!(tied(&views["work/top/schematic"], &views), (0, 1));

    let views = buffers([("tie", tied_to_ground("GND!"))]);
    let mut returned = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "0"),
        two(2, ComponentType::Resistor, "RL", "in", "GND!"),
    ]);
    returned.components.push(instance(9, "X1", "tie"));
    assert_eq!(tied(&returned, &views), (0, 0));
}
