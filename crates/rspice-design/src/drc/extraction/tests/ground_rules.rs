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
