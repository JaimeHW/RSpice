//! Hierarchical names, the glob, and the instance tree they build.

use super::super::results_tree::{
    BrowserQuery, SignalKind, TreeItem, build_tree, hierarchical_path, is_generated_node,
    split_hierarchy,
};

#[test]
fn a_hierarchical_name_splits_at_its_instances() {
    let split = split_hierarchy("V(x1.x2.out)");
    assert_eq!(split.instances, ["x1", "x2"]);
    assert_eq!(split.leaf, "V(out)");
    let split = split_hierarchy("I(X1:V1)");
    assert_eq!(split.instances, ["X1"]);
    assert_eq!(split.leaf, "I(V1)");
    // Top-level, differential, device parameters and bus ranges stay whole.
    for whole in ["V(out)", "V(a.b,c)", "@m1[id]", "b[7:0]", "V(x1.)", "time"] {
        let split = split_hierarchy(whole);
        assert!(split.instances.is_empty(), "{whole} split as {split:?}");
        assert_eq!(split.leaf, whole);
    }
    let split = split_hierarchy("xsar.bits[7:0]");
    assert_eq!(split.instances, ["xsar"]);
    assert_eq!(split.leaf, "bits[7:0]");
    assert_eq!(hierarchical_path("D(xsar.q)"), "xsar.q");
    assert_eq!(hierarchical_path("V(a,b)"), "V(a,b)");
}

/// A node the simulator generated has a path segment beginning `__`, as the
/// engine names a device's internal nodes and inserted elements; a design's
/// own names, and a double underscore inside a segment, are not generated.
#[test]
fn a_generated_node_is_named_by_a_segment_the_engine_prefixes() {
    for generated in [
        "V(xldo.MP.__gint)",
        "I(xldo.MP.__rg)",
        "V(m1.__body.internal)",
        "x1:__dint",
        "V(__int1)",
    ] {
        assert!(is_generated_node(generated), "{generated}");
    }
    for design in [
        "V(xldo.FB)",
        "V(out)",
        "I(X1.V1)",
        "D(xtb.ok__event_0)",
        "V(a_b)",
        "V(__a,b)",
        "@m1[__id]",
        "xsar.bits[7:0]",
    ] {
        assert!(!is_generated_node(design), "{design}");
    }
}

#[test]
fn a_glob_matches_whole_names_without_regard_to_case() {
    let glob = |pattern: &str| BrowserQuery::parse(pattern);
    assert!(matches!(glob("x1.*"), BrowserQuery::Glob(_)));
    assert!(glob("x1.*").matches_name("V(x1.out)"));
    assert!(glob("X1.*").matches_name("V(x1.x2.out)"));
    assert!(!glob("x1.*").matches_name("V(x10out)"));
    assert!(!glob("x1.*").matches_name("V(out)"));
    assert!(glob("V(*)").matches_name("V(out)"));
    assert!(!glob("V(*)").matches_name("I(V1)"));
    assert!(glob("*out").matches_name("V(x1.out)"));
    assert!(!glob("out*").matches_name("V(x1.out)"));
    assert!(glob("b?").matches_name("D(b7)"));
    assert!(!glob("b?").matches_name("D(b10)"));
    assert!(glob("b[0-3]").matches_name("D(b2)"));
    assert!(!glob("b[0-3]").matches_name("D(b5)"));
    assert!(glob("b[!0-3]").matches_name("D(b5)"));
    assert!(glob("*[ab]*").matches_name("V(cab)"));
    // A pattern names signals; it is never a word to find in a label.
    assert!(!glob("tran*").matches_word("TRAN"));
    // Plain text is found anywhere, in names and words alike.
    let text = BrowserQuery::parse(" Out ");
    assert!(text.matches_name("V(x1.out)"));
    assert!(text.matches_word("output stage"));
    assert!(BrowserQuery::parse("   ").is_empty());
    // Stars never make matching exponential.
    let many = glob("*a*a*a*a*a*a*a*a*b");
    assert!(!many.matches_name(&"a".repeat(64)));
}

#[test]
fn the_tree_groups_quantities_under_their_instances_in_retained_order() {
    let names = [
        "V(out)",
        "V(x1.a)",
        "I(X1.V1)",
        "V(x1.x2.b)",
        "V(x3.c)",
        "V(in)",
    ];
    let tree = build_tree(
        names
            .iter()
            .enumerate()
            .map(|(index, name)| (TreeItem::Signal(index), *name)),
    );
    assert_eq!(tree.count, 6);
    assert_eq!(tree.items, [TreeItem::Signal(0), TreeItem::Signal(5)]);
    let instances = tree
        .children
        .iter()
        .map(|child| (child.name.as_str(), child.count))
        .collect::<Vec<_>>();
    assert_eq!(instances, [("x1", 3), ("x3", 1)]);
    let x1 = &tree.children[0];
    assert_eq!(x1.items, [TreeItem::Signal(1), TreeItem::Signal(2)]);
    assert_eq!(x1.children[0].path, "x1.x2");
    assert_eq!(x1.children[0].key, "x1.x2");
    assert_eq!(
        tree.leaves(),
        [3, 1, 2, 4, 0, 5].map(TreeItem::Signal),
        "instances list before the quantities at their own level"
    );
}

#[test]
fn a_level_lists_by_kind_then_by_name_with_numbers_counted() {
    let names = [
        "V(x1.l10)",
        "I(x1.r1)",
        "V(x1.l2)",
        "D(x1.q)",
        "V(x1.m10.g)",
        "V(x1.m2.g)",
    ];
    let mut tree = build_tree(
        names
            .iter()
            .enumerate()
            .map(|(index, name)| (TreeItem::Signal(index), *name)),
    );
    tree.sort(&|item| {
        let TreeItem::Signal(index) = item else {
            panic!("signals only")
        };
        let name = names[index];
        let kind = match &name[..2] {
            "I(" => SignalKind::Current,
            "D(" => SignalKind::Digital,
            _ => SignalKind::Voltage,
        };
        (kind.rank(), split_hierarchy(name).leaf)
    });
    let x1 = &tree.children[0];
    let instances = x1
        .children
        .iter()
        .map(|child| child.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(instances, ["m2", "m10"]);
    assert_eq!(x1.items, [2, 0, 1, 3].map(TreeItem::Signal));
}

#[test]
fn an_instance_takes_the_spelling_the_design_wrote() {
    // A converter's endpoint comes back in SPICE's capitals; the testbench's
    // own variables keep the deck's spelling. A design written in capitals
    // keeps them.
    let names = ["XTB.OK__EVENT_0", "xtb.pass", "V(X1.A)", "V(X1.B)"];
    let tree = build_tree(
        names
            .iter()
            .enumerate()
            .map(|(index, name)| (TreeItem::Signal(index), *name)),
    );
    let instances = tree
        .children
        .iter()
        .map(|child| (child.name.as_str(), child.path.as_str(), child.count))
        .collect::<Vec<_>>();
    assert_eq!(instances, [("xtb", "xtb", 2), ("X1", "X1", 2)]);
}

#[test]
fn every_row_states_its_kind() {
    assert_eq!(SignalKind::of_waveform("V(out)", "Voltage").glyph(), "V");
    assert_eq!(SignalKind::of_waveform("I(V1)", "Current").glyph(), "I");
    assert_eq!(SignalKind::of_waveform("D(x1.q)", "Voltage").glyph(), "D");
    assert_eq!(SignalKind::of_waveform("E(vref)", "Quantity").glyph(), "R");
    assert_eq!(SignalKind::of_waveform("VP(out)", "Phase").glyph(), "°");
    assert_eq!(SignalKind::Bus.label(), "Digital bus");
}
