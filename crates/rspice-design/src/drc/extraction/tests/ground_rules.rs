//! Which net is the reference node, judged by what the deck writes.
//!
//! The extractor merges every net a ground name labels — `0`, `GND`, `GND!`,
//! `GROUND`, any case — into the ground net, and the netlister writes it as
//! node 0. Every rule here is held to that, under the default contract and
//! under a technology global-net catalog that promotes `GND!` onto `GND`, flat
//! and one level down. A ground label can still lose its net to a name that
//! claimed the net first, and then the deck writes that name.

use super::*;
use crate::connectivity_contract::{
    ConnectivityAliasGroup, ConnectivityContract, ConnectivityPolicy, GlobalNetPromotionPolicy,
    TechnologyGlobalNetCatalog,
};

/// The project contract every new project starts with: `GND!` is a global
/// declared by its own `!`, and nothing promotes it.
fn unaliased() -> ConnectivityContract {
    ConnectivityContract::default()
}

/// A contract whose technology global-net catalog holds the groups
/// `(canonical, aliases)`.
fn catalog(groups: &[(&str, &[&str])]) -> ConnectivityContract {
    let contract = ConnectivityContract {
        policy: ConnectivityPolicy {
            global_promotion: GlobalNetPromotionPolicy::TechnologyDefinedOnly,
            ..ConnectivityPolicy::default()
        },
        technology_global_nets: Some(TechnologyGlobalNetCatalog {
            authority: "demo-pdk@1.0".to_owned(),
            nets: groups
                .iter()
                .map(|(canonical_name, aliases)| ConnectivityAliasGroup {
                    canonical_name: (*canonical_name).to_owned(),
                    aliases: aliases.iter().map(|alias| (*alias).to_owned()).collect(),
                })
                .collect(),
        }),
        ..ConnectivityContract::default()
    };
    contract.validate().expect("the catalog is valid");
    contract
}

/// A catalog that promotes `GND!` onto `GND` and `VDD!` onto `VDD`.
fn aliased() -> ConnectivityContract {
    catalog(&[("GND", &[]), ("VDD", &[])])
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

/// The messages of every finding of `kind`.
fn messages(result: &DrcResult, kind: DrcViolationType) -> Vec<String> {
    of_type(result, kind)
        .into_iter()
        .map(|finding| finding.message.clone())
        .collect()
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

/// A terminal of `component`, where the drawing places it.
fn terminal(component: &Component, pin: &str) -> Point {
    terminal_positions_with_hierarchy(component, None)
        .into_iter()
        .find_map(|(name, point)| (name == pin).then_some(point))
        .expect("a two-terminal part has both terminals")
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

/// A source and its load returned to `0`, with a `GND` label placed on the
/// source's `-` after a `VSS` label named that net.
fn ground_label_after_vss() -> SchematicDocument {
    let mut drawing = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "VSS"),
        two(2, ComponentType::Resistor, "R1", "in", "0"),
    ]);
    let minus = drawing.net_labels[1].pos;
    drawing.net_labels.push(NetLabel::new(900, minus, "GND"));
    drawing
}

/// R2 with its `+` wired to a ground symbol and labelled `label` there, and
/// its `-` labelled `x`.
fn tied_to_ground(label: &'static str) -> SchematicDocument {
    let resistor = part(1, ComponentType::Resistor, "R2");
    let ground = Component::new(2, ComponentType::Ground, Point::new(0, 200));
    let (plus, minus) = (terminal(&resistor, "+"), terminal(&resistor, "-"));
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

/// R2 with `first`, then `second`, labelling its `+`, and its `-` labelled
/// `x`.
fn named_twice(first: &'static str, second: &'static str) -> SchematicDocument {
    let resistor = part(1, ComponentType::Resistor, "R2");
    let (plus, minus) = (terminal(&resistor, "+"), terminal(&resistor, "-"));
    SchematicDocument {
        components: vec![resistor],
        net_labels: vec![
            NetLabel::new(20, plus, first),
            NetLabel::new(21, plus, second),
            NetLabel::new(22, minus, "x"),
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

/// A design returned only through `GND!` is returned to node 0, promoted or
/// not, so it has its ground and nothing says otherwise.
#[test]
fn a_design_returned_only_through_a_ground_global_has_its_ground() {
    let views = buffers([("top", grounded_by_bang())]);
    let top = &views["work/top/schematic"];
    for contract in [unaliased(), aliased()] {
        let result = checked_under(&contract, top, &views);
        for kind in [
            DrcViolationType::MissingGround,
            DrcViolationType::GroundNotNodeZero,
        ] {
            assert!(
                of_type(&result, kind).is_empty(),
                "{kind:?}: {:?}",
                result.violations()
            );
        }
    }
}

/// Node 0 is global, so the design's ground is asked over the whole walk, and
/// `GND!` drawn inside a cell is node 0 there as at the top.
#[test]
fn a_ground_global_below_the_subject_grounds_the_design() {
    let views = buffers([("leg", grounded_by_bang())]);
    for contract in [unaliased(), aliased()] {
        let result = checked_under(&contract, &only("leg"), &views);
        for kind in [
            DrcViolationType::MissingGround,
            DrcViolationType::GroundNotNodeZero,
        ] {
            assert!(
                of_type(&result, kind).is_empty(),
                "{kind:?}: {:?}",
                result.violations()
            );
        }
    }
}

/// V1 from `vdd` to `0` beside V2 from `vdd` to ground, however the drawing
/// spells it, is two sources across one pair of nodes of the deck: a loop the
/// engine refuses, promoted or not.
#[test]
fn sources_into_node_zero_under_any_ground_name_close_a_loop() {
    for ground in ["GND!", "gnd!", "GND", "Ground"] {
        let views = buffers([("top", sources_across(ground))]);
        let top = &views["work/top/schematic"];
        for contract in [unaliased(), aliased()] {
            let result = checked_under(&contract, top, &views);
            assert_eq!(
                of_type(&result, DrcViolationType::SourceToSource).len(),
                1,
                "{ground}: {:?}",
                result.violations()
            );
        }
    }
}

/// Across cellviews the same holds: V2 in a cell from `VDD!` to `GND!` closes
/// a loop with V1 at the top from `VDD!` to `0`.
#[test]
fn a_loop_across_cellviews_closes_through_a_ground_global() {
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

    for contract in [unaliased(), aliased()] {
        let result = checked_under(&contract, &top, &views);
        assert_eq!(
            of_type(&result, DrcViolationType::SourceToSource).len(),
            1,
            "{:?}",
            result.violations()
        );
    }
}

const GND_WRITTEN_AS_VSS: &str =
    "Net label 'GND' reads as ground, but the deck writes its net as node 'VSS', not as node 0";

/// A ground label on a net that another name claimed first is not node 0: the
/// deck writes that name. Once per node, at the label that says ground.
#[test]
fn a_ground_label_whose_net_took_another_name_is_an_error() {
    let views = buffers([("top", ground_label_after_vss())]);
    let result = checked_under(&unaliased(), &views["work/top/schematic"], &views);

    let stray = of_type(&result, DrcViolationType::GroundNotNodeZero);
    assert_eq!(stray.len(), 1, "{:?}", result.violations());
    assert_eq!(stray[0].severity, DrcSeverity::Error);
    assert_eq!(stray[0].message, GND_WRITTEN_AS_VSS);
    assert_eq!(
        stray[0].location,
        DrcLocation::NetLabel {
            name: "GND".to_owned()
        }
    );
}

/// Inside a placed cell, the label is stated at the occurrence.
#[test]
fn a_ground_label_below_the_subject_is_stated_inside_its_occurrence() {
    let views = buffers([("leg", ground_label_after_vss())]);

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
                name: "GND".to_owned()
            }
        )
    );
}

/// Every ground name is node 0 however it is spelled, and so is any label
/// that shares its net with a ground symbol, which names the net `0`. A
/// ground label whose net took another name is not, whether an earlier label
/// claimed the net or the project's catalog promotes the label onto a rail.
#[test]
fn a_ground_name_is_judged_by_the_node_its_net_is_written_as() {
    let stray = |contract: &ConnectivityContract, drawing: SchematicDocument| {
        let views = buffers([("top", drawing)]);
        let result = checked_under(contract, &views["work/top/schematic"], &views);
        messages(&result, DrcViolationType::GroundNotNodeZero)
    };
    for ground in ["0", "GND", "gnd", "Ground", "GND!", "gnd!"] {
        let returned = labelled(vec![
            two(1, ComponentType::VoltageSource, "V1", "in", ground),
            two(2, ComponentType::Resistor, "R1", "in", ground),
        ]);
        assert_eq!(
            stray(&unaliased(), returned),
            Vec::<String>::new(),
            "{ground}"
        );
    }
    assert_eq!(
        stray(&unaliased(), tied_to_ground("GND!")),
        Vec::<String>::new()
    );
    assert_eq!(
        stray(&unaliased(), ground_label_after_vss()),
        [GND_WRITTEN_AS_VSS]
    );

    let returned_through_gnd = labelled(vec![
        two(1, ComponentType::VoltageSource, "V1", "in", "GND"),
        two(2, ComponentType::Resistor, "R1", "in", "0"),
    ]);
    assert_eq!(
        stray(&catalog(&[("VSS", &["GND"])]), returned_through_gnd),
        [GND_WRITTEN_AS_VSS]
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

const VDD_TIED: &str = "Global net 'VDD!' is tied to ground here: the deck writes this net as \
                        node 0, apart from 'VDD!' in the rest of the design";

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
    assert_eq!(tied[0].message, VDD_TIED);
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

/// A ground label that named the net before the global's label did ties the
/// global to node 0 as a ground symbol does. Placed after it, the ground label
/// is the one that lost: the net is the global's, and the label is the
/// finding.
#[test]
fn a_global_and_a_ground_label_on_one_net_are_judged_by_the_name_the_net_took() {
    let findings = |cell: SchematicDocument| {
        let views = buffers([("tie", cell)]);
        let result = checked_under(&unaliased(), &supplied_beside("tie"), &views);
        (
            messages(&result, DrcViolationType::SupplyTiedToGround),
            messages(&result, DrcViolationType::GroundNotNodeZero),
        )
    };

    assert_eq!(
        findings(named_twice("GND", "VDD!")),
        (vec![VDD_TIED.to_owned()], Vec::new())
    );
    assert_eq!(
        findings(named_twice("VDD!", "GND")),
        (
            Vec::new(),
            vec![
                "Net label 'GND' reads as ground, but the deck writes its net as node 'VDD!', \
                 not as node 0"
                    .to_owned()
            ]
        )
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
