//! The library's sources are compiled through this crate's own front end.
//!
//! Shipped signatures delegate to native bridges. The cases below also cover
//! the language features used by authored connection bodies.

use super::*;
use crate::connect::{
    ConnectDirection, ConnectError, build_connect_rule_table, disciplines_compatible,
};
use crate::disciplines::{DisciplineDb, Domain};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::source::SourceId;

fn parse(source: &str) -> crate::ast::SourceFile {
    let tokens = Lexer::new(source, SourceId::new(0))
        .collect_tokens()
        .unwrap_or_else(|error| panic!("lexes: {error}"));
    Parser::new(&tokens)
        .parse()
        .unwrap_or_else(|error| panic!("parses: {error}"))
}

/// The same source with `connectmodule` spelled `module`.
///
/// [`crate::semantic::SemanticAnalyzer`] walks `Item::Module` and skips
/// `Item::ConnectModule` — a connect module reaches
/// [`crate::semantic::AnalyzedFile::connect_rules`] as a port signature and
/// never as an analyzed body — so this spelling is how the library's contents
/// get analyzed at all. The two forms differ by the keyword and nothing else,
/// which [`the_two_spellings_differ_only_in_the_keyword`] pins.
fn as_plain_module(source: &str) -> String {
    source
        .strip_prefix("connectmodule")
        .map(|rest| format!("module{rest}"))
        .expect("a built-in connect module opens with its keyword")
}

#[test]
fn the_two_spellings_differ_only_in_the_keyword() {
    for (name, source) in BUILTIN_CONNECT_MODULES {
        assert_eq!(
            as_plain_module(source),
            source.replacen("connectmodule", "module", 1),
            "{name}"
        );
        assert!(source.starts_with("connectmodule "), "{name}");
        assert_eq!(source.matches("connectmodule").count(), 1, "{name}");
    }
}

/// Section 7.5 makes a connect module "a module", so every one of these has to
/// survive the analyzer that reads modules.
#[test]
fn every_built_in_connect_module_analyzes() {
    for (name, source) in BUILTIN_CONNECT_MODULES {
        let file = parse(&as_plain_module(source));
        let mut analyzer = crate::semantic::SemanticAnalyzer::new();
        let analyzed = analyzer
            .analyze(&file)
            .unwrap_or_else(|error| panic!("{name} passes semantic analysis: {error:?}"));
        assert!(
            analyzed.modules.contains_key(name),
            "{name} reaches the analyzer"
        );
    }
}

#[test]
fn the_library_builds_one_connect_rule_table() {
    let db = DisciplineDb::with_standard();
    let table = build_connect_rule_table(&parse(&builtin_connect_library_source()), &db)
        .unwrap_or_else(|error| panic!("library builds a rule table: {error}"));
    assert_eq!(table.insertions().len(), 3);
    assert!(table.resolutions().is_empty());
}

/// Section 7.6 and Table 7-2: one continuous port, one discrete port, and one
/// of the three admissible direction combinations. The library is worthless if
/// selection cannot see it.
#[test]
fn the_library_covers_all_three_of_table_7_2s_rows() {
    let db = DisciplineDb::with_standard();
    let table = build_connect_rule_table(&parse(&builtin_connect_library_source()), &db)
        .unwrap_or_else(|error| panic!("library builds a rule table: {error}"));

    for (module, expected) in [
        ("a2d", ConnectDirection::AnalogToDiscrete),
        ("d2a", ConnectDirection::DiscreteToAnalog),
        ("bidir", ConnectDirection::Bidirectional),
    ] {
        let rule = table
            .insertions()
            .iter()
            .find(|rule| rule.connect_module == module)
            .unwrap_or_else(|| panic!("{module} is selectable"));
        assert_eq!(rule.direction, expected, "{module} direction");
        assert_eq!(rule.continuous.discipline, "electrical", "{module}");
        assert_eq!(rule.discrete.discipline, "logic", "{module}");
        assert_eq!(
            db.get_discipline(&rule.continuous.discipline)
                .map(|discipline| discipline.domain),
            Some(Domain::Continuous)
        );
        assert_eq!(
            db.get_discipline(&rule.discrete.discipline)
                .map(|discipline| discipline.domain),
            Some(Domain::Discrete)
        );
        assert!(disciplines_compatible(
            &db,
            &rule.continuous.discipline,
            "electrical"
        ));
    }
}

/// Section 7.8.4 rule 3 has to reach exactly one module for each direction a
/// mixed port can need. `bidir` admits all three, so the two unidirectional
/// rows must out-rank it rather than tie with it.
#[test]
fn selection_reaches_one_module_for_each_direction() {
    let db = DisciplineDb::with_standard();
    let table = build_connect_rule_table(&parse(&builtin_connect_library_source()), &db)
        .unwrap_or_else(|error| panic!("library builds a rule table: {error}"));

    for (required, expected) in [
        (ConnectDirection::AnalogToDiscrete, "a2d"),
        (ConnectDirection::DiscreteToAnalog, "d2a"),
        (ConnectDirection::Bidirectional, "bidir"),
    ] {
        let rule = table
            .select("electrical", "logic", required, &db)
            .unwrap_or_else(|error| panic!("{} selects: {error}", required.label()));
        assert_eq!(rule.connect_module, expected, "{}", required.label());
    }
}

/// Section 7.8.3: "The default is merged." Several discrete ports on one node
/// share one bridge because of it, which is the shape the engine's auto-bridge
/// already has — one bridge per node, not one per port.
#[test]
fn every_built_in_rule_is_merged() {
    let db = DisciplineDb::with_standard();
    let table = build_connect_rule_table(&parse(&builtin_connect_library_source()), &db)
        .unwrap_or_else(|error| panic!("library builds a rule table: {error}"));
    for rule in table.insertions() {
        assert_eq!(
            rule.mode,
            crate::ast::ConnectMode::Merged,
            "{}",
            rule.connect_module
        );
    }
}

/// A further electrical port is a supply: the conversion pair is the discrete
/// port and the first continuous port that completes a Table 7-2 row, unless
/// `inh_conn_prop_name` marks that port as a supply.
#[test]
fn a_further_electrical_port_is_a_supply() {
    let db = DisciplineDb::with_standard();
    let decl = |source: &str| {
        let table = build_connect_rule_table(&parse(source), &db);
        table.map(|table| {
            let decl = table.modules.values().next().expect("one module").clone();
            (
                decl.continuous.name.to_string(),
                decl.discrete.name.to_string(),
                decl.supplies
                    .iter()
                    .map(|port| {
                        (
                            port.name.to_string(),
                            port.inherited_property.as_deref().map(str::to_owned),
                            port.inherited_default.as_deref().map(str::to_owned),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
    };
    let supplied = decl(
        "\
connectmodule a2d_supplied(a, d, vdd);
    input a;
    output d;
    input vdd;
    electrical a;
    logic d;
    electrical vdd;
endmodule
",
    )
    .expect("a third electrical port is a supply");
    assert_eq!(
        supplied,
        ("a".into(), "d".into(), vec![("vdd".into(), None, None)])
    );

    // The marked port is the supply although it comes first and completes a
    // Table 7-2 row; its default is the net its escaped global spelling names.
    let marked = decl(
        r#"
connectmodule d2a_marked(vsup, d, a);
    inout vsup;
    input d;
    output a;
    electrical (* integer inh_conn_prop_name = "vdd"; integer inh_conn_def_value = "cds_globals.\\vdd! "; *) vsup;
    logic d;
    electrical a;
endmodule
"#,
    )
    .expect("the marked port is the supply");
    assert_eq!(
        marked,
        (
            "a".into(),
            "d".into(),
            vec![("vsup".into(), Some("vdd".into()), Some("vdd!".into()))]
        )
    );

    for (port, kind, source) in [
        (
            "vdd",
            "an output",
            "connectmodule m(a, d, vdd); input a; output d; output vdd; electrical a; logic d; electrical vdd; endmodule",
        ),
        (
            "en",
            "a discrete port",
            "connectmodule m(a, d, en); input a; output d; input en; electrical a; logic d; logic en; endmodule",
        ),
    ] {
        let error = decl(source).expect_err("refused");
        assert!(
            matches!(
                &error,
                ConnectError::ConnectModuleSupplyPort { port: refused, kind: why, .. }
                    if refused == port && *why == kind
            ),
            "unexpected error: {error}"
        );
    }
}

// ---------------------------------------------------------------------------
// The delegation contract
// ---------------------------------------------------------------------------

/// The parameter names and folded defaults the engine's delegation reads.
///
/// This is the front end's half of a contract whose other half is pinned in
/// `rspice-core`. Renaming a parameter here, or changing what it folds to,
/// without moving that pin is the exact drift two independent transcriptions
/// of one bridge semantics produced once already.
///
/// The count is asserted too: a parameter added here that the delegation does
/// not read would be a knob a deck can set and nothing can hear.
#[test]
fn the_delegated_parameters_are_named_and_defaulted_as_the_engine_reads_them() {
    /// A default that names another parameter, which does not fold.
    const DEPENDENT: f64 = f64::NAN;
    const A2D_FULL: &[(&str, f64)] = &[
        ("vsup", 3.3),
        ("vsupmin", 0.0),
        ("fthi", 0.5),
        ("ftlo", 0.5),
        ("vthi", DEPENDENT),
        ("vtlo", DEPENDENT),
        ("txdel", f64::INFINITY),
        ("tdrise", 0.0),
        ("tdfall", 0.0),
    ];
    const D2A_FULL: &[(&str, f64)] = &[
        ("vsup", 3.3),
        ("vsupmin", 0.0),
        ("vlo", 0.0),
        ("vhi", DEPENDENT),
        ("vx", DEPENDENT),
        ("vz", DEPENDENT),
        ("rlo", 20.0),
        ("rhi", 20.0),
        ("rx", DEPENDENT),
        ("rz", f64::INFINITY),
        ("rsup", DEPENDENT),
        ("rpull", 10e3),
        ("rweak", 100e3),
        ("trise", 0.0),
        ("tfall", 0.0),
        ("tdelay", 0.0),
    ];
    let bidir_full: Vec<(&str, f64)> = A2D_FULL
        .iter()
        .chain(D2A_FULL.iter().skip(2))
        .copied()
        .collect();
    let expected: [(&str, &str, &[(&str, f64)]); 6] = [
        ("a2d", A2D, A2D_FULL),
        ("d2a", D2A, D2A_FULL),
        ("bidir", BIDIR, &bidir_full),
        (
            "a2d",
            LEGACY_BUILTIN_CONNECT_MODULES[0].1,
            &[("vsup", 3.3), ("tdrise", 1e-9), ("tdfall", 1e-9)],
        ),
        (
            "d2a",
            LEGACY_BUILTIN_CONNECT_MODULES[1].1,
            &[("vsup", 3.3), ("trise", 1e-9), ("tfall", 1e-9)],
        ),
        (
            "bidir",
            LEGACY_BUILTIN_CONNECT_MODULES[2].1,
            &[("vsup", 3.3), ("trise", 1e-9), ("tfall", 1e-9)],
        ),
    ];

    for (name, source, parameters) in expected {
        let file = parse(&as_plain_module(source));
        let mut analyzer = crate::semantic::SemanticAnalyzer::new();
        let analyzed = analyzer
            .analyze(&file)
            .unwrap_or_else(|error| panic!("{name} analyzes: {error:?}"));
        let module = analyzed
            .modules
            .get(name)
            .unwrap_or_else(|| panic!("{name} is analyzed"));
        assert_eq!(
            module.parameters.len(),
            parameters.len(),
            "{name} declares exactly the delegated parameters"
        );
        for (parameter_name, value) in parameters {
            let parameter = module
                .parameters
                .iter()
                .find(|parameter| parameter.name == *parameter_name)
                .unwrap_or_else(|| panic!("{name} declares {parameter_name}"));
            if value.is_nan() {
                assert!(
                    parameter.default.is_none() && parameter.default_expr.is_some(),
                    "{name} parameter {parameter_name} defaults to an expression"
                );
                continue;
            }
            let default = parameter
                .default
                .unwrap_or_else(|| panic!("{name} folds {parameter_name} to a constant"));
            assert!(
                default == *value || (default - value).abs() <= 1e-12,
                "{name} parameter {parameter_name} defaults to {default}, expected {value}"
            );
        }
    }
}

/// A verbatim copy of either signature delegates, and says which; a copy
/// that changes one default delegates as neither.
#[test]
fn both_signatures_delegate_and_an_edited_copy_neither() {
    let delegations =
        |source: &str| equivalent_declarations(source, &parse(source), &Default::default());
    for (index, (name, full)) in BUILTIN_CONNECT_MODULES.iter().enumerate() {
        let legacy = LEGACY_BUILTIN_CONNECT_MODULES[index].1;
        assert_eq!(
            delegations(full).get(*name),
            Some(&BuiltinSignature::Full),
            "{name}"
        );
        assert_eq!(
            delegations(legacy).get(*name),
            Some(&BuiltinSignature::Legacy),
            "{name}"
        );
        let edited = full.replacen("vsupmin = 0", "vsupmin = 1", 1);
        assert!(delegations(&edited).is_empty(), "{name}");
    }
    // The full declarations as first shipped, with volt thresholds only,
    // still delegate, and say so.
    for (name, absolute) in ABSOLUTE_BUILTIN_CONNECT_MODULES {
        assert_eq!(
            delegations(absolute).get(name),
            Some(&BuiltinSignature::Absolute),
            "{name}"
        );
        let full = BUILTIN_CONNECT_MODULES
            .iter()
            .find(|(full, _)| *full == name)
            .map(|(_, full)| *full)
            .expect("a shipped full declaration");
        assert_eq!(
            absolute,
            full.replace(
                "    parameter real fthi = 0.5;\n    parameter real ftlo = 0.5;\n",
                ""
            )
            .replace("fthi * vsup", "vsup / 2")
            .replace("ftlo * vsup", "vsup / 2"),
            "{name}: the fractions are the only difference"
        );
    }
}

/// Every shipped rule set analyzes against the shipped modules, each is
/// named with the reserved prefix, and the names are distinct.
#[test]
fn every_shipped_rule_set_selects_the_shipped_modules() {
    let db = DisciplineDb::with_standard();
    let source = shipped_connect_library_source();
    let table = build_connect_rule_table(&parse(&source), &db)
        .unwrap_or_else(|error| panic!("the shipped library builds: {error:?}"));
    let sets = shipped_rule_sets();
    assert_eq!(sets.len(), 15);
    let mut names = std::collections::BTreeSet::new();
    for (name, _) in &sets {
        assert!(name.starts_with(SHIPPED_RULE_SET_PREFIX), "{name}");
        assert!(names.insert(name.clone()), "{name} is named once");
        let block = table
            .select_block(name)
            .unwrap_or_else(|error| panic!("{name} selects: {error:?}"));
        assert_eq!(block.insertions().len(), 3, "{name}");
    }
    let fast = table.select_block("rspice_1v8_fast").expect("1v8 fast");
    let a2d = fast
        .insertions()
        .iter()
        .find(|rule| rule.connect_module == "a2d")
        .expect("a2d");
    let parameters = a2d.numeric_parameters().expect("the overrides fold");
    for (parameter, value) in [
        ("vsup", 1.8),
        ("ftlo", 0.3),
        ("fthi", 0.7),
        ("txdel", 1.0e-9),
    ] {
        let given = parameters
            .iter()
            .find(|(name, _)| name == parameter)
            .map(|(_, value)| *value);
        assert_eq!(given, Some(value), "rspice_1v8_fast a2d {parameter}");
    }
}

/// A block may name the shipped modules without declaring them: each takes
/// the shipped declaration and delegates as the full signature, its overrides
/// stay the statement's, and a module the source declares stays its own.
#[test]
fn a_block_names_the_shipped_modules_without_declaring_them() {
    let db = DisciplineDb::with_standard();
    let source = "connectrules mine;\n    connect d2a #(.vsup(1.8));\n    connect a2d \
                  #(.vsup(1.8), .fthi(0.6));\n    connect r2e;\nendconnectrules\n";
    let file = parse(source);
    let table = build_connect_rule_table(&file, &db)
        .unwrap_or_else(|error| panic!("the undeclared shipped modules resolve: {error:?}"));
    let delegations = equivalent_declarations(source, &file, &table);
    for name in ["d2a", "a2d", "r2e"] {
        assert_eq!(
            delegations.get(name),
            Some(&BuiltinSignature::Full),
            "{name}"
        );
    }
    assert!(!delegations.contains_key("bidir"), "only the named modules");
    let block = table.select_block("mine").expect("mine");
    let rules = block.insertions();
    assert_eq!(rules[0].direction, ConnectDirection::DiscreteToAnalog);
    assert_eq!(rules[0].continuous.name, "a");
    assert_eq!(rules[0].discrete.name, "d");
    assert_eq!(rules[1].direction, ConnectDirection::AnalogToDiscrete);
    assert_eq!(
        rules[1].numeric_parameters().expect("the overrides fold"),
        vec![("vsup".into(), 1.8), ("fthi".into(), 0.6)]
    );

    // A module the source declares is the source's, shipped name or not.
    let declared = format!("{D2A}connectrules mine;\n    connect d2a;\nendconnectrules\n")
        .replace("vsup = 3.3", "vsup = 2.5");
    let file = parse(&declared);
    let table = build_connect_rule_table(&file, &db).expect("a declared d2a builds");
    assert!(equivalent_declarations(&declared, &file, &table).is_empty());

    // A name the library does not ship is still unknown.
    let error = build_connect_rule_table(
        &parse("connectrules mine;\n    connect d2b;\nendconnectrules\n"),
        &db,
    )
    .expect_err("d2b is not shipped");
    assert!(
        matches!(error, ConnectError::UnknownConnectModule { ref name, .. } if name == "d2b"),
        "{error:?}"
    );
}

// ---------------------------------------------------------------------------
// Why there are no behavioural bodies
// ---------------------------------------------------------------------------

fn analysis_error(source: &str) -> String {
    let file = parse(source);
    let mut analyzer = crate::semantic::SemanticAnalyzer::new();
    format!(
        "{:?}",
        analyzer
            .analyze(&file)
            .expect_err("the body is refused; if it is not, the library can carry one")
    )
}

/// Why the levels derive from `vsup` on the engine side and not here: a
/// parameter default that names another parameter does not fold, so the
/// library could only hand the delegation an unevaluated expression.
///
/// The day this folds, `vlo`/`vhi`/`vx` can move into the sources as absolute
/// volts and the delegation can read them instead of deriving them — this test
/// is what will say so.
#[test]
fn a_dependent_parameter_default_does_not_fold() {
    let file = parse(
        "\
module dependent_default(a);
    inout a;
    electrical a;
    parameter real vsup = 3.3;
    parameter real vlo = vsup / 2.0;
endmodule
",
    );
    let mut analyzer = crate::semantic::SemanticAnalyzer::new();
    let analyzed = analyzer.analyze(&file).expect("analyzes");
    let module = analyzed.modules.get("dependent_default").expect("module");
    let vsup = module
        .parameters
        .iter()
        .find(|parameter| parameter.name == "vsup")
        .expect("vsup");
    let vlo = module
        .parameters
        .iter()
        .find(|parameter| parameter.name == "vlo")
        .expect("vlo");
    assert_eq!(vsup.default, Some(3.3), "a literal default folds");
    assert_eq!(
        vlo.default, None,
        "a default naming another parameter does not fold; if it does now, \
         the library can carry absolute thresholds again"
    );
    assert!(
        vlo.default_expr.is_some(),
        "only the unevaluated expression survives"
    );
}

/// An authored A/D body can subscribe to an analog threshold event.
#[test]
fn an_analog_sensing_discrete_process_retains_its_subscription() {
    let file = parse(
        r#"
module a2d_with_body(a, d);
    input a;
    output d;
    electrical a;
    logic d;
    reg d;
    parameter real vhi = 1.65;
    always @(above(V(a) - vhi))
        d <= 1'b1;
endmodule
"#,
    );
    let analyzed = crate::SemanticAnalyzer::new().analyze(&file).unwrap();
    assert_eq!(
        analyzed.modules["a2d_with_body"]
            .digital
            .analog_events
            .len(),
        1
    );
}

/// The probe on its own — section 7.3.3's read, without section 7.3.5's
/// event — compiles inside a connect module's discrete half.
///
/// An independently clocked sampler and a threshold subscription both retain
/// the continuous port's probe binding.
#[test]
fn a_connect_module_process_may_probe_its_continuous_port() {
    // Spelled `module`, for the reason [`as_plain_module`] gives: the analyzer
    // walks modules and reads a connect module as a port signature only.
    let file = parse(
        "\
module a2d_sampling(a, d);
    input a;
    output d;
    electrical a;
    logic d;
    reg d;
    wire tick;
    parameter real vhi = 1.65;
    always @(posedge tick)
        d <= (V(a) > vhi);
endmodule
",
    );
    let mut analyzer = crate::semantic::SemanticAnalyzer::new();
    analyzer
        .analyze(&file)
        .expect("a discrete process probing its continuous port analyzes");
}

/// A `d2a` written the other way — a discrete process setting a `real` that an
/// `analog` block contributes — is refused for what is now a much narrower
/// reason, and one that names a clause rather than a wall.
///
/// A real written by the discrete domain reaches analog equations through
/// a canonical state-variable input (Verilog-AMS section 7.3.6.5).
#[test]
fn a_cross_clock_variable_uses_a_state_input() {
    let source = "\
module d2a_with_body(d, a);
    input d;
    output a;
    logic d;
    electrical a;
    parameter real vhi = 3.3;
    parameter real trise = 1e-9;
    real vout;
    always @(d)
        vout = vhi;
    analog
        V(a) <+ transition(vout, 0.0, trise, trise);
endmodule
";
    let analyzed = crate::SemanticAnalyzer::new()
        .analyze(&parse(source))
        .unwrap();
    let module = &analyzed.modules["d2a_with_body"];
    let slot = module
        .variables
        .iter()
        .position(|variable| variable.name == "vout")
        .unwrap();
    assert!(module.variables[slot].is_state);
    assert!(module.event_state_variables.contains(&slot));
}

/// The same module with the analog body *writing* the shared variable is a
/// different refusal, and a permanent one.
///
/// Verilog-AMS LRM 2.4 section 7.3: "Write operations of nets and variables are
/// only allowed from the context of their domain." Two writers is not a
/// synchronization problem waiting for a seam — it is a program the standard
/// does not admit, so the message says which sentence rather than which wave.
#[test]
fn a_variable_both_domains_write_is_refused_permanently() {
    let error = analysis_error(
        "\
module two_writers(d, a);
    input d;
    output a;
    logic d;
    electrical a;
    parameter real vhi = 3.3;
    real vout;
    always @(d)
        vout = vhi;
    analog begin
        vout = 0.0;
        V(a) <+ vout;
    end
endmodule
",
    );
    assert!(
        error.contains("is written by both the analog body and a discrete process"),
        "unexpected error: {error}"
    );
}

/// And an analog body that neither reads nor writes the variable does not
/// refuse at all.
///
/// This is the refusal that went away. The rule used to be about the *module* —
/// any analog block disqualified every module-level `real` a process wrote —
/// which refused a perfectly ordinary mixed module and refused it with a
/// message about clocks that had nothing to do with the program. Section 7.3's
/// rule is about the name.
#[test]
fn an_unrelated_analog_body_leaves_a_process_owned_real_alone() {
    let file = parse(
        "\
module independent_halves(d, a);
    input d;
    inout a;
    logic d;
    electrical a;
    real acc;
    always @(d)
        acc = acc + 1.0;
    analog V(a) <+ 1.0;
endmodule
",
    );
    let mut analyzer = crate::semantic::SemanticAnalyzer::new();
    let analyzed = analyzer
        .analyze(&file)
        .expect("an analog body that never names the variable is not a second owner");
    let module = analyzed.modules.get("independent_halves").expect("module");
    assert!(
        module
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "acc"),
        "the process's `real` moved into the discrete domain"
    );
}

/// The real-valued modules analyze, delegate as the full signature, and a
/// rule block naming them selects each by its direction with a real
/// discrete port.
#[test]
fn the_real_modules_delegate_and_select_by_direction() {
    let delegations =
        |source: &str| equivalent_declarations(source, &parse(source), &Default::default());
    let mut source = String::new();
    for (name, module) in BUILTIN_REAL_CONNECT_MODULES {
        let file = parse(&as_plain_module(module));
        crate::semantic::SemanticAnalyzer::new()
            .analyze(&file)
            .unwrap_or_else(|error| panic!("{name} passes semantic analysis: {error:?}"));
        assert_eq!(
            delegations(module).get(name),
            Some(&BuiltinSignature::Full),
            "{name}"
        );
        let edited = module.replacen("ttol = 0", "ttol = 1", 1);
        if edited != module {
            assert!(delegations(&edited).is_empty(), "{name}");
        }
        source.push_str(module);
    }
    source.push_str("connectrules real;\n    connect r2e;\n    connect e2r;\n    connect rbidir;\nendconnectrules\n");
    let db = DisciplineDb::with_standard();
    let table = build_connect_rule_table(&parse(&source), &db)
        .unwrap_or_else(|error| panic!("real modules build a rule table: {error}"));
    for (module, direction) in [
        ("e2r", ConnectDirection::AnalogToDiscrete),
        ("r2e", ConnectDirection::DiscreteToAnalog),
        ("rbidir", ConnectDirection::Bidirectional),
    ] {
        let rule = table
            .select_typed(
                "electrical",
                "logic",
                direction,
                Some(crate::connect::ConnectValueKind::Real),
                &db,
            )
            .unwrap_or_else(|error| panic!("{module} is selectable: {error}"));
        assert_eq!(rule.connect_module, module);
    }
}
