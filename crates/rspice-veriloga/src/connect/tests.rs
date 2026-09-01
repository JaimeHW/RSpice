//! Tests for clause 7 discipline resolution and connect module planning.
//!
//! Where Verilog-AMS LRM 2.4 states an answer — Figures 7-2 and 7-3 name the
//! discipline each of four interconnects resolves to, section 7.7.2.1 works
//! two `resolveto` examples through — the test reproduces the standard's own
//! topology and asserts the standard's own answer. Those are the tests that
//! can tell a wrong implementation from a right one; the rest pin decisions
//! this compiler made where the standard left room.

use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::source::SourceId;

fn parse(source: &str) -> SourceFile {
    let tokens = Lexer::new(source, SourceId::new(0))
        .collect_tokens()
        .unwrap_or_else(|error| panic!("lexes: {error}"));
    Parser::new(&tokens)
        .parse()
        .unwrap_or_else(|error| panic!("parses: {error}"))
}

/// The discipline database every test resolves against: the standard set plus
/// the logic families the LRM's figures name.
fn db() -> DisciplineDb {
    let mut db = DisciplineDb::with_standard();
    for name in ["cmos1", "cmos2", "cmos3", "cmos4", "ttl", "ddiscrete"] {
        db.add_discipline(crate::disciplines::Discipline::builtin(
            name,
            Domain::Discrete,
            None,
            None,
        ));
    }
    db
}

fn table(source: &str) -> ConnectRuleTable {
    build_connect_rule_table(&parse(source), &db())
        .unwrap_or_else(|error| panic!("connect rules build: {error}"))
}

/// The two connect modules of the LRM's section 7.8 example, verbatim apart
/// from the behavioural bodies, which selection never reads.
const LRM_CONNECT_MODULES: &str = "\
connectmodule elect_to_logic(el, cm);
    input el;
    output cm;
    electrical el;
    ddiscrete cm;
endmodule
connectmodule logic_to_elect(cm, el);
    input cm;
    output el;
    ddiscrete cm;
    electrical el;
endmodule
";

// ---------------------------------------------------------------------------
// Grammar — section 7.7, Syntax 7-5 through 7-7
// ---------------------------------------------------------------------------

#[test]
fn a_connectrules_block_reads_both_forms_of_connect_statement() {
    let source = parse(
        "\
connectrules mixedsignal;
    connect elect_to_logic;
    connect logic_to_elect split #(.vcc(3.3));
    connect cmos3, cmos4 resolveto cmos3;
    connect cmos1, cmos2, cmos3 resolveto exclude;
endconnectrules
",
    );
    let Some(Item::ConnectRules(block)) = source.items.first() else {
        panic!("the file holds one connectrules block");
    };
    assert_eq!(block.name, "mixedsignal");
    assert_eq!(block.items.len(), 4);

    let ConnectRulesItem::Insertion(second) = &block.items[1] else {
        panic!("the second item is an insertion statement");
    };
    assert_eq!(second.connect_module, "logic_to_elect");
    assert_eq!(second.mode, Some(ConnectMode::Split));
    assert_eq!(second.parameters.len(), 1);
    assert_eq!(second.parameters[0].name.as_deref(), Some("vcc"));

    let ConnectRulesItem::Resolution(third) = &block.items[2] else {
        panic!("the third item is a resolution statement");
    };
    assert_eq!(third.disciplines, ["cmos3", "cmos4"]);
    assert!(matches!(
        &third.target,
        ConnectResolveTarget::Discipline(name) if name == "cmos3"
    ));

    let ConnectRulesItem::Resolution(fourth) = &block.items[3] else {
        panic!("the fourth item is a resolution statement");
    };
    assert!(matches!(fourth.target, ConnectResolveTarget::Exclude));
}

#[test]
fn a_resolveto_list_may_be_written_with_spaces_as_the_figures_write_it() {
    // Figures 7-2 through 7-5 write `connect cmos3 cmos4 resolveto cmos3;`
    // while grammar A.1.8 separates the list with commas. Both are read.
    let source = parse(
        "\
connectrules figures;
    connect cmos3 cmos4 resolveto cmos3;
endconnectrules
",
    );
    let Some(Item::ConnectRules(block)) = source.items.first() else {
        panic!("the file holds one connectrules block");
    };
    let ConnectRulesItem::Resolution(rule) = &block.items[0] else {
        panic!("a resolution statement");
    };
    assert_eq!(rule.disciplines, ["cmos3", "cmos4"]);
}

#[test]
fn a_connectmodule_parses_as_a_module_and_is_no_longer_refused() {
    let source = parse(LRM_CONNECT_MODULES);
    assert_eq!(source.items.len(), 2);
    let Some(Item::ConnectModule(module)) = source.items.first() else {
        panic!("a connectmodule item");
    };
    assert_eq!(module.name, "elect_to_logic");
    assert_eq!(module.ports.len(), 2);
}

#[test]
fn port_override_directions_reach_the_rule() {
    // Section 7.7.1: "the specified directions are used to define the type of
    // connect module". `bidir` is declared unidirectional and made
    // bidirectional by the statement.
    let source = format!(
        "{LRM_CONNECT_MODULES}\
connectrules example;
    connect elect_to_logic inout electrical, inout ddiscrete;
endconnectrules
"
    );
    let table = table(&source);
    assert_eq!(table.insertions().len(), 1);
    assert_eq!(
        table.insertions()[0].direction,
        ConnectDirection::Bidirectional
    );
}

#[test]
fn a_port_override_naming_two_disciplines_of_one_domain_is_refused() {
    let source = format!(
        "{LRM_CONNECT_MODULES}\
connectrules bad;
    connect elect_to_logic cmos1, ddiscrete;
endconnectrules
"
    );
    let error = build_connect_rule_table(&parse(&source), &db())
        .expect_err("two discrete disciplines is not a mixed pair");
    assert!(
        matches!(error, ConnectError::RuleDomains { .. }),
        "got {error}"
    );
}

#[test]
fn a_connect_statement_naming_an_undeclared_module_is_refused() {
    let source = "\
connectrules bad;
    connect nowhere;
endconnectrules
";
    let error = build_connect_rule_table(&parse(source), &db())
        .expect_err("a connect statement must name a declared connectmodule");
    assert!(
        matches!(error, ConnectError::UnknownConnectModule { ref name, .. } if name == "nowhere"),
        "got {error}"
    );
}

#[test]
fn a_connectmodule_bridging_one_domain_twice_is_refused() {
    let source = "\
connectmodule not_a_bridge(a, b);
    input a;
    output b;
    electrical a;
    electrical b;
endmodule
connectrules bad;
    connect not_a_bridge;
endconnectrules
";
    let error = build_connect_rule_table(&parse(source), &db())
        .expect_err("section 7.8 requires one discrete and one continuous port");
    assert!(
        matches!(error, ConnectError::ConnectModuleDomains { .. }),
        "got {error}"
    );
}

#[test]
fn a_connectmodule_outside_table_7_2_is_refused() {
    let source = "\
connectmodule two_inputs(el, cm);
    input el;
    input cm;
    electrical el;
    ddiscrete cm;
endmodule
connectrules bad;
    connect two_inputs;
endconnectrules
";
    let error = build_connect_rule_table(&parse(source), &db())
        .expect_err("Table 7-2 admits three direction combinations, and this is not one");
    assert!(
        matches!(error, ConnectError::ConnectModuleDirections { .. }),
        "got {error}"
    );
}

// ---------------------------------------------------------------------------
// Compatibility — section 3.11.1
// ---------------------------------------------------------------------------

#[test]
fn discipline_compatibility_follows_section_3_11_1() {
    let db = db();
    // Self Rule.
    assert!(disciplines_compatible(&db, "electrical", "electrical"));
    // Domain Incompatibility Rule.
    assert!(!disciplines_compatible(&db, "electrical", "ddiscrete"));
    // Natureless Discipline Rule: `ddiscrete` and `cmos1` both declare no
    // nature, so they are compatible with every discipline of their domain.
    assert!(disciplines_compatible(&db, "ddiscrete", "cmos1"));
    // Potential Incompatibility Rule: voltage and temperature are different
    // natures with different units.
    assert!(!disciplines_compatible(&db, "electrical", "thermal"));
    // Non-Existent Binding Rule: `voltage` declares a potential and no flow,
    // and its potential is `electrical`'s.
    assert!(disciplines_compatible(&db, "electrical", "voltage"));
}

// ---------------------------------------------------------------------------
// Resolution — section 7.4.4.1 and Annex F.2.1, against the LRM's Figure 7-3
// ---------------------------------------------------------------------------

/// The LRM's Figure 7-3 hierarchy, as a signal.
///
/// ```text
/// NetD ─┬─ NetA ─┬─ cmos1 (blk1)
///       │        ├─ cmos2 (blk2)
///       │        └─ NetB ─┬─ cmos3 (blk3)
///       │                 └─ cmos4 (blk4)
///       └─ NetC ─┬─ cmos2      (blk2)
///                └─ electrical (ablk)
/// ```
fn figure_7_3() -> (Signal, usize, usize, usize, usize) {
    let mut signal = Signal::default();
    let cmos3 = signal.push(NetSegment::new("blk3_out").declared("cmos3"));
    let cmos4 = signal.push(NetSegment::new("blk4_out").declared("cmos4"));
    let net_b = signal.push(
        NetSegment::new("NetB")
            .with_child(PortLink::new(cmos3, PortDirection::Output, "blk3", "out"))
            .with_child(PortLink::new(cmos4, PortDirection::Output, "blk4", "out")),
    );
    let cmos1 = signal.push(NetSegment::new("blk1_out").declared("cmos1"));
    let cmos2a = signal.push(NetSegment::new("blk2_out").declared("cmos2"));
    let net_a = signal.push(
        NetSegment::new("NetA")
            .with_child(PortLink::new(cmos1, PortDirection::Output, "blk1", "out"))
            .with_child(PortLink::new(cmos2a, PortDirection::Output, "blk2", "out"))
            .with_child(PortLink::new(
                net_b,
                PortDirection::Output,
                "twoblks",
                "out",
            )),
    );
    let cmos2b = signal.push(NetSegment::new("mix_blk2_out").declared("cmos2"));
    let electrical = signal.push(NetSegment::new("ablk_out").declared("electrical"));
    let net_c = signal.push(
        NetSegment::new("NetC")
            .with_child(PortLink::new(cmos2b, PortDirection::Output, "blk2", "out"))
            .with_child(PortLink::new(
                electrical,
                PortDirection::Output,
                "ablk",
                "out",
            )),
    );
    let net_d = signal.push(
        NetSegment::new("NetD")
            .with_child(PortLink::new(
                net_a,
                PortDirection::Output,
                "digital_blk",
                "out",
            ))
            .with_child(PortLink::new(net_c, PortDirection::Output, "mix", "out")),
    );
    (signal, net_a, net_b, net_c, net_d)
}

const FIGURE_7_3_RULES: &str = "\
connectrules figure_7_3;
    connect cmos3, cmos4 resolveto cmos3;
    connect cmos1, cmos2, cmos3 resolveto cmos1;
endconnectrules
";

#[test]
fn basic_resolution_reproduces_the_disciplines_figure_7_3_states() {
    let (signal, net_a, net_b, net_c, net_d) = figure_7_3();
    let resolved = resolve_disciplines(
        &signal,
        &table(FIGURE_7_3_RULES),
        &db(),
        None,
        ResolutionMode::Basic,
    )
    .expect("Figure 7-3 resolves");

    // "NetB resolves to cmos3 based on the first resolveto connect statement."
    assert_eq!(resolved.discipline(net_b), Some("cmos3"));
    // "NetA resolves to cmos1 based on the second resolveto connect statement."
    assert_eq!(resolved.discipline(net_a), Some("cmos1"));
    // "NetC resolves to electrical based on continuous (electrical) winning
    //  over discrete (cmos2)."
    assert_eq!(resolved.discipline(net_c), Some("electrical"));
    // "NetD resolves to electrical based on continuous (electrical) winning
    //  over discrete (cmos1)."
    assert_eq!(resolved.discipline(net_d), Some("electrical"));
}

#[test]
fn a_declared_interconnect_coerces_resolution_as_section_7_4_4_3_case_2_says() {
    // Case 2 of section 7.4.4.3: "NetA is declared as cmos1 (the others are
    // undeclared). discipline resolution basic: NetA stays cmos1, NetB is
    // assigned cmos3, and NetC and NetD become electrical."
    let (mut signal, net_a, net_b, net_c, net_d) = figure_7_3();
    signal.segments[net_a].declared = Some("cmos1".into());
    let resolved = resolve_disciplines(
        &signal,
        &table(FIGURE_7_3_RULES),
        &db(),
        None,
        ResolutionMode::Basic,
    )
    .expect("the coerced signal resolves");

    assert_eq!(resolved.discipline(net_a), Some("cmos1"));
    assert_eq!(resolved.discipline(net_b), Some("cmos3"));
    assert_eq!(resolved.discipline(net_c), Some("electrical"));
    assert_eq!(resolved.discipline(net_d), Some("electrical"));
}

#[test]
fn a_default_discipline_fills_a_net_with_no_declared_children() {
    // Annex F.2.1 step 4b: "If there are no disciplines in the list apply any
    // `default_discipline` directives to the net, provided their domain is the
    // same as the domain of the net."
    let mut signal = Signal::default();
    let leaf = signal.push(NetSegment::new("leaf"));
    let root = signal.push(NetSegment::new("top").with_child(PortLink::new(
        leaf,
        PortDirection::Output,
        "u1",
        "out",
    )));
    let resolved = resolve_disciplines(
        &signal,
        &ConnectRuleTable::default(),
        &db(),
        Some("ddiscrete"),
        ResolutionMode::Basic,
    )
    .expect("the default applies");
    assert_eq!(resolved.discipline(leaf), Some("ddiscrete"));
    assert_eq!(resolved.discipline(root), Some("ddiscrete"));
}

#[test]
fn a_continuous_default_discipline_is_not_applied_to_a_discrete_net() {
    // The same clause's proviso, the other way round: a continuous default has
    // no business on a net the traversal decided is discrete.
    let mut signal = Signal::default();
    let leaf = signal.push(NetSegment::new("leaf").digital_behavioral());
    let resolved = resolve_disciplines(
        &signal,
        &ConnectRuleTable::default(),
        &db(),
        Some("electrical"),
        ResolutionMode::Basic,
    )
    .expect("an unresolved net with no mixed port is legal");
    assert_eq!(resolved.discipline(leaf), None);
    assert_eq!(resolved.domain(leaf), Some(Domain::Discrete));
}

#[test]
fn resolveto_matching_follows_section_7_7_2_1_example_1() {
    // "connect x,y,a resolveto a; connect x,y resolveto x;" gives:
    //   x,y   -> x      (exact match on the second rule)
    //   x,y,a -> a      (exact match on the first)
    //   y,a   -> a      (no exact fit; the first rule's subset is the fit)
    let mut db = db();
    for name in ["x", "y", "a"] {
        db.add_discipline(crate::disciplines::Discipline::builtin(
            name,
            Domain::Discrete,
            None,
            None,
        ));
    }
    let table = build_connect_rule_table(
        &parse(
            "\
connectrules example1;
    connect x, y, a resolveto a;
    connect x, y resolveto x;
endconnectrules
",
        ),
        &db,
    )
    .expect("the rules build");

    let resolve = |names: &[&str]| {
        let mut warnings = Vec::new();
        let found: BTreeSet<SmolStr> = names.iter().map(|name| SmolStr::from(*name)).collect();
        table
            .resolve_list(&found, "net", &mut warnings)
            .expect("no exclusion")
    };
    assert_eq!(resolve(&["x", "y"]).as_deref(), Some("x"));
    assert_eq!(resolve(&["x", "y", "a"]).as_deref(), Some("a"));
    assert_eq!(resolve(&["y", "a"]).as_deref(), Some("a"));
}

#[test]
fn two_rules_matching_one_list_warn_and_take_the_first() {
    // Section 7.7.2.1 Example 2: "disciplines x,y,a would resolve to
    // discipline y with a warning."
    let mut db = db();
    for name in ["x", "y", "a", "b"] {
        db.add_discipline(crate::disciplines::Discipline::builtin(
            name,
            Domain::Discrete,
            None,
            None,
        ));
    }
    let table = build_connect_rule_table(
        &parse(
            "\
connectrules example2;
    connect x, y, a resolveto y;
    connect x, y, a resolveto a;
    connect x, y, b resolveto b;
endconnectrules
",
        ),
        &db,
    )
    .expect("the rules build");

    let resolve = |names: &[&str]| {
        let mut warnings = Vec::new();
        let found: BTreeSet<SmolStr> = names.iter().map(|name| SmolStr::from(*name)).collect();
        let resolved = table
            .resolve_list(&found, "net", &mut warnings)
            .expect("no exclusion");
        (resolved, warnings.len())
    };

    // "disciplines x,y would resolve to discipline y with a warning" — no
    // exact fit, and all three rules contain {x,y}.
    let (resolved, warnings) = resolve(&["x", "y"]);
    assert_eq!(resolved.as_deref(), Some("y"));
    assert_eq!(warnings, 1);

    // "disciplines x,y,a would resolve to discipline y with a warning" — two
    // exact fits, and the first wins.
    let (resolved, warnings) = resolve(&["x", "y", "a"]);
    assert_eq!(resolved.as_deref(), Some("y"));
    assert_eq!(warnings, 1);

    // "disciplines y,b would resolve to b" — only the third rule contains it,
    // so there is nothing to warn about.
    let (resolved, warnings) = resolve(&["y", "b"]);
    assert_eq!(resolved.as_deref(), Some("b"));
    assert_eq!(warnings, 0);
}

#[test]
fn resolveto_exclude_makes_the_listed_disciplines_an_error() {
    // Section 7.7.2's own example: two supplies that must never meet.
    let mut db = db();
    for name in ["logic18", "logic32"] {
        db.add_discipline(crate::disciplines::Discipline::builtin(
            name,
            Domain::Discrete,
            None,
            None,
        ));
    }
    let table = build_connect_rule_table(
        &parse(
            "\
connectrules supplies;
    connect logic18, logic32 resolveto exclude;
endconnectrules
",
        ),
        &db,
    )
    .expect("the rules build");

    let mut signal = Signal::default();
    let low = signal.push(NetSegment::new("low").declared("logic18"));
    let high = signal.push(NetSegment::new("high").declared("logic32"));
    signal.push(
        NetSegment::new("shared")
            .with_child(PortLink::new(low, PortDirection::Output, "u1", "out"))
            .with_child(PortLink::new(high, PortDirection::Output, "u2", "out")),
    );

    let error = resolve_disciplines(&signal, &table, &db, None, ResolutionMode::Basic)
        .expect_err("the excluded pair is an error");
    assert!(
        matches!(error, ConnectError::ExcludedDisciplines { ref net, .. } if net == "shared"),
        "got {error}"
    );
}

#[test]
fn the_detail_resolution_mode_is_refused_by_name() {
    let (signal, ..) = figure_7_3();
    let error = resolve_disciplines(
        &signal,
        &table(FIGURE_7_3_RULES),
        &db(),
        None,
        ResolutionMode::Detail,
    )
    .expect_err("the detail mode is not implemented");
    let message = error.to_string();
    assert!(message.contains("7.4.4.2"), "got {message}");
    assert!(message.contains("Annex F.2.2"), "got {message}");
}

#[test]
fn a_mixed_net_with_no_resolvable_discipline_is_an_error() {
    // Annex F.2.1's last bullet: unknown is legal only when the net has no
    // mixed-port connection.
    let mut db = db();
    for name in ["p", "q"] {
        db.add_discipline(crate::disciplines::Discipline::builtin(
            name,
            Domain::Discrete,
            None,
            None,
        ));
    }
    let mut signal = Signal::default();
    let analog = signal.push(NetSegment::new("a").declared("electrical"));
    let first = signal.push(NetSegment::new("d1").declared("p"));
    let second = signal.push(NetSegment::new("d2").declared("q"));
    // Digital behavioural use makes the net discrete despite its analog child,
    // so the two discrete children are the list — and with no rule to resolve
    // them the net stays unknown while still carrying a mixed port.
    signal.push(
        NetSegment::new("undeclared")
            .digital_behavioral()
            .with_child(PortLink::new(analog, PortDirection::Input, "ablk", "in"))
            .with_child(PortLink::new(first, PortDirection::Output, "u1", "out"))
            .with_child(PortLink::new(second, PortDirection::Output, "u2", "out")),
    );

    let error = resolve_disciplines(
        &signal,
        &ConnectRuleTable::default(),
        &db,
        None,
        ResolutionMode::Basic,
    )
    .expect_err("two discrete disciplines and no rule leave the net unknown");
    assert!(
        matches!(error, ConnectError::UnresolvedDiscipline { ref net } if net == "undeclared"),
        "got {error}"
    );
}

// ---------------------------------------------------------------------------
// Insertion planning — section 7.8
// ---------------------------------------------------------------------------

/// One analog upper connection with `count` discrete ports below it.
fn fanout(count: usize, direction: PortDirection) -> Signal {
    let mut signal = Signal::default();
    let mut links = Vec::new();
    for index in 0..count {
        let lower = signal.push(NetSegment::new(format!("d{index}")).declared("ddiscrete"));
        links.push(PortLink::new(lower, direction, format!("u{index}"), "port"));
    }
    let mut top = NetSegment::new("mixed").declared("electrical");
    top.children = links;
    signal.push(top);
    signal
}

fn plan(signal: &Signal, rules: &str) -> Result<ConnectModulePlan, ConnectError> {
    let db = db();
    let table = table(rules);
    let resolved = resolve_disciplines(signal, &table, &db, None, ResolutionMode::Basic)
        .expect("the signal resolves");
    plan_connect_modules(signal, &resolved, &table, &db)
}

const LRM_RULES: &str = "\
connectrules mixedsignal;
    connect elect_to_logic;
    connect logic_to_elect;
endconnectrules
";

fn lrm_source() -> String {
    format!("{LRM_CONNECT_MODULES}{LRM_RULES}")
}

#[test]
fn an_analog_upper_connection_driving_a_digital_input_takes_the_a2d_module() {
    // The port is an `input`, so section 7.6 makes the upper connection the
    // driver: analog drives discrete, and `elect_to_logic` is the module whose
    // continuous port is the input.
    let plan = plan(&fanout(1, PortDirection::Input), &lrm_source()).expect("planned");
    assert_eq!(plan.insertions.len(), 1);
    assert_eq!(plan.insertions[0].connect_module, "elect_to_logic");
    assert_eq!(
        plan.insertions[0].direction,
        ConnectDirection::AnalogToDiscrete
    );
    assert_eq!(plan.insertions[0].continuous, "electrical");
    assert_eq!(plan.insertions[0].discrete, "ddiscrete");
}

#[test]
fn a_digital_lower_connection_driving_an_analog_net_takes_the_d2a_module() {
    let plan = plan(&fanout(1, PortDirection::Output), &lrm_source()).expect("planned");
    assert_eq!(plan.insertions.len(), 1);
    assert_eq!(plan.insertions[0].connect_module, "logic_to_elect");
    assert_eq!(
        plan.insertions[0].direction,
        ConnectDirection::DiscreteToAnalog
    );
}

#[test]
fn a_bidirectional_port_with_no_bidirectional_module_is_refused_by_name() {
    let error = plan(&fanout(1, PortDirection::Inout), &lrm_source())
        .expect_err("neither module bridges an inout port");
    let ConnectError::NoConnectRule {
        net,
        continuous,
        discrete,
        direction,
    } = &error
    else {
        panic!("got {error}");
    };
    assert_eq!(net, "mixed");
    assert_eq!(continuous, "electrical");
    assert_eq!(discrete, "ddiscrete");
    assert_eq!(*direction, ConnectDirection::Bidirectional);
    let message = error.to_string();
    assert!(message.contains("'mixed'"), "got {message}");
    assert!(message.contains("electrical"), "got {message}");
    assert!(message.contains("ddiscrete"), "got {message}");
}

#[test]
fn merged_is_the_default_and_gives_one_instance_for_a_whole_fanout() {
    // Section 7.8.3: "The default is merged." Section 7.8.4 insertion rule 3:
    // ports sharing the upper connection, the module, and the bottom
    // discipline share one instance.
    let plan = plan(&fanout(3, PortDirection::Output), &lrm_source()).expect("planned");
    assert_eq!(plan.insertions.len(), 1);
    assert_eq!(plan.insertions[0].mode, ConnectMode::Merged);
    assert_eq!(plan.insertions[0].bindings.len(), 3);
    // Section 7.8.5: SigName__ModuleName__BottomDiscipline.
    assert_eq!(
        plan.insertions[0].instance,
        "mixed__logic_to_elect__ddiscrete"
    );
}

#[test]
fn split_gives_one_instance_per_port_named_after_the_port() {
    let source = format!(
        "{LRM_CONNECT_MODULES}\
connectrules split_rules;
    connect logic_to_elect split;
endconnectrules
"
    );
    let plan = plan(&fanout(3, PortDirection::Output), &source).expect("planned");
    assert_eq!(plan.insertions.len(), 3);
    // Section 7.8.5: SigName__InstName__PortName.
    let names: Vec<&str> = plan
        .insertions
        .iter()
        .map(|insertion| insertion.instance.as_str())
        .collect();
    assert_eq!(
        names,
        ["mixed__u0__port", "mixed__u1__port", "mixed__u2__port"]
    );
    for insertion in &plan.insertions {
        assert_eq!(insertion.bindings.len(), 1);
    }
}

#[test]
fn an_analog_only_signal_needs_no_connect_module() {
    // Section 7.8.4: the rules "apply only to mixed signals".
    let mut signal = Signal::default();
    let lower = signal.push(NetSegment::new("lo").declared("electrical"));
    signal.push(
        NetSegment::new("hi")
            .declared("electrical")
            .with_child(PortLink::new(lower, PortDirection::Output, "u0", "out")),
    );
    let plan = plan(&signal, &lrm_source()).expect("planned");
    assert!(plan.insertions.is_empty());
}

#[test]
fn the_plan_names_the_insertion_context_and_both_sides_of_every_binding() {
    let signal = fanout(2, PortDirection::Output);
    let plan = plan(&signal, &lrm_source()).expect("planned");
    let insertion = &plan.insertions[0];
    // Section 7.8.4 insertion rule 2: the instance lives in the context of the
    // upper connection, which is the last segment the fanout builder pushed.
    assert_eq!(insertion.context, signal.segments.len() - 1);
    assert_eq!(
        insertion.bindings,
        vec![
            ConnectModuleBinding {
                upper: 2,
                lower: 0,
                instance: "u0".into(),
                port: "port".into(),
                direction: PortDirection::Output,
            },
            ConnectModuleBinding {
                upper: 2,
                lower: 1,
                instance: "u1".into(),
                port: "port".into(),
                direction: PortDirection::Output,
            },
        ]
    );
}

#[test]
fn a_bidirectional_module_serves_a_unidirectional_port_when_it_is_the_only_rule() {
    // Section 7.6 Example 3: an inout/inout connect module "can bridge any
    // mixed port".
    let source = "\
connectmodule bidir(cm, el);
    inout cm;
    inout el;
    ddiscrete cm;
    electrical el;
endmodule
connectrules only_bidir;
    connect bidir;
endconnectrules
";
    let plan = plan(&fanout(1, PortDirection::Output), source).expect("planned");
    assert_eq!(plan.insertions[0].connect_module, "bidir");
}

#[test]
fn a_directed_rule_beats_a_bidirectional_one_and_a_tie_is_refused() {
    let modules = "\
connectmodule bidir(cm, el);
    inout cm;
    inout el;
    ddiscrete cm;
    electrical el;
endmodule
connectmodule d2a(cm, el);
    input cm;
    output el;
    ddiscrete cm;
    electrical el;
endmodule
connectmodule d2a_alt(cm, el);
    input cm;
    output el;
    ddiscrete cm;
    electrical el;
endmodule
";
    // The directed rule is the closer match, so the bidirectional one does not
    // make the choice ambiguous.
    let ranked = format!(
        "{modules}\
connectrules ranked;
    connect bidir;
    connect d2a;
endconnectrules
"
    );
    let ranked_plan = plan(&fanout(1, PortDirection::Output), &ranked).expect("planned");
    assert_eq!(ranked_plan.insertions[0].connect_module, "d2a");

    // Two rules at the same rank are section 7.8.4 rule 3's "one and only one"
    // violated, and the refusal names both.
    let tied = format!(
        "{modules}\
connectrules tied;
    connect d2a;
    connect d2a_alt;
endconnectrules
"
    );
    let error =
        plan(&fanout(1, PortDirection::Output), &tied).expect_err("two equally specific rules");
    let ConnectError::AmbiguousConnectRule { first, second, .. } = &error else {
        panic!("got {error}");
    };
    assert_eq!(first, "d2a");
    assert_eq!(second, "d2a_alt");
}

#[test]
fn a_compatible_discipline_selects_a_rule_written_for_its_sibling() {
    // Section 7.7.1: "Connect modules can be reused for different, but
    // compatible disciplines". `cmos1` is natureless, so section 3.11.1's
    // Natureless Discipline Rule makes it compatible with `ddiscrete`.
    let mut signal = Signal::default();
    let lower = signal.push(NetSegment::new("d").declared("cmos1"));
    signal.push(
        NetSegment::new("mixed")
            .declared("electrical")
            .with_child(PortLink::new(lower, PortDirection::Output, "u0", "out")),
    );
    let plan = plan(&signal, &lrm_source()).expect("planned");
    assert_eq!(plan.insertions[0].connect_module, "logic_to_elect");
    assert_eq!(plan.insertions[0].discrete, "cmos1");
    assert_eq!(
        plan.insertions[0].instance, "mixed__logic_to_elect__cmos1",
        "section 7.8.5 names the instance after the bottom discipline, which is the net's"
    );
}

#[test]
fn a_connectrules_block_is_validated_by_the_compiler_front_end() {
    // The table is built during semantic analysis, so a connect statement that
    // names nothing reaches the author as a compile error rather than being
    // carried as an inert item.
    let source = parse(
        "\
connectrules bad;
    connect nowhere;
endconnectrules
module dut(p);
    inout p;
    electrical p;
    analog V(p) <+ 0.0;
endmodule
",
    );
    let error = SemanticAnalyzer::new()
        .analyze(&source)
        .expect_err("the connect statement names no declared connectmodule");
    let message = error.to_string();
    assert!(message.contains("nowhere"), "got {message}");
}
