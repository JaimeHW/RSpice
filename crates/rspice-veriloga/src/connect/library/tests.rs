//! The library's sources are compiled through this crate's own front end.
//!
//! That is the whole point of keeping them as Verilog-AMS rather than as
//! prose. It is also why they carry no behavioural body: the last two tests
//! here pin the two refusals that would meet one, so the decision is checked
//! rather than remembered.

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

/// A third port has nowhere to go, which is what makes a supply-sensitive
/// connect module a `vsup` *parameter* rather than a supply rail port.
#[test]
fn a_third_port_is_refused_with_its_clause() {
    let db = DisciplineDb::with_standard();
    let source = parse(
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
    );
    let error = build_connect_rule_table(&source, &db).expect_err("three ports are refused");
    assert!(
        matches!(error, ConnectError::ConnectModulePortCount { found: 3, .. }),
        "unexpected error: {error}"
    );
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
    let expected: [(&str, &str, &[(&str, f64)]); 3] = [
        (
            "a2d",
            A2D,
            &[("vsup", 3.3), ("tdrise", 1e-9), ("tdfall", 1e-9)],
        ),
        (
            "d2a",
            D2A,
            &[("vsup", 3.3), ("trise", 1e-9), ("tfall", 1e-9)],
        ),
        (
            "bidir",
            BIDIR,
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
            let default = parameter
                .default
                .unwrap_or_else(|| panic!("{name} folds {parameter_name} to a constant"));
            assert!(
                (default - value).abs() <= 1e-12,
                "{name} parameter {parameter_name} defaults to {default}, expected {value}"
            );
        }
    }
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

/// Every published `a2d` senses the analog side from a discrete process. Half
/// of what that needs now works, and the other half is what still holds the
/// behavioural body back.
///
/// Reading `V(a)` from the process is Verilog-AMS LRM 2.4 section 7.3.3's
/// probe and is accepted — see the mixed-signal tests in
/// `digital_process_execution`. Waking on `above(V(a) - vhi)` is section
/// 7.3.5's *event*, a different construct in a different position: the
/// `event_expression` production admits `analog_event_functions`, and nothing
/// here subscribes a process to one yet. So the refusal that remains is
/// exactly one, and it names the event rather than the probe inside it.
#[test]
fn an_analog_sensing_discrete_process_is_refused_by_name() {
    let error = analysis_error(
        "\
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
",
    );
    assert!(
        error.contains("call to `above` inside a discrete-domain expression is not supported yet"),
        "unexpected error: {error}"
    );
    assert!(
        !error.contains("has no meaning in a discrete-domain expression"),
        "the probe inside the event argument is section 7.3.3's and is no longer refused: {error}"
    );
}

/// The probe on its own — section 7.3.3's read, without section 7.3.5's
/// event — compiles inside a connect module's discrete half.
///
/// This is the half of an `a2d` body that now exists, pinned here rather than
/// only in the front end's own tests because the library's decision to ship
/// signatures rather than bodies rests on which half is missing.
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
/// `analog` block contributes — is refused for the reason that *is* the
/// mixed-signal boundary problem: the two halves advance on different clocks.
///
/// This is the front end's statement of the same wall the engine hits, where
/// `rspice_core`'s `xspice::verilog::MixedSignalHost` refuses any trial time
/// off its integer-nanosecond grid.
#[test]
fn a_cross_clock_variable_is_refused_by_name() {
    let error = analysis_error(
        "\
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
",
    );
    assert!(
        error.contains("they advance on different clocks"),
        "unexpected error: {error}"
    );
}
