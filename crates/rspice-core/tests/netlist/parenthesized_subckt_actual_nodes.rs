//! Xyce parenthesized actual-node lists on analog subcircuit instances.

use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{
    Element, ElementKind, Netlist, NetlistParseOptions, ParametricValue, ParseError,
    flatten_netlist_with_models,
};

fn xyce_options() -> NetlistParseOptions {
    NetlistParseOptions {
        expression_dialect: ExpressionDialect::Xyce,
        ..NetlistParseOptions::default()
    }
}

fn parse_xyce(source: &str) -> Result<Netlist, ParseError> {
    Netlist::parse_with_options(source, xyce_options())
}

fn flattened_snapshot(elements: &[Element]) -> Vec<(String, Vec<String>, String)> {
    elements
        .iter()
        .map(|element| {
            (
                element.name.clone(),
                element.nodes.clone(),
                format!("{:?}", element.kind),
            )
        })
        .collect()
}

fn nested_deck(top_instance: &str, inner_instance: &str) -> String {
    format!(
        "nested parenthesized actual nodes\n\
         .SUBCKT LEAF p n PARAMS: RVAL=1k\n\
         RLEAF p n {{RVAL}}\n\
         .ENDS LEAF\n\
         .SUBCKT WRAP p n PARAMS: SCALE=1\n\
         {inner_instance}\n\
         .ENDS WRAP\n\
         {top_instance}\n\
         .END\n"
    )
}

#[test]
fn nested_and_continued_parenthesized_instances_flatten_like_plain_instances() {
    let plain = nested_deck(
        "XTOP src out WRAP PARAMS: SCALE=(1+2)",
        "XINNER p n LEAF PARAMS: RVAL=1000*SCALE",
    );
    let parenthesized = nested_deck(
        "XTOP\n+ ( src out )\n+ WRAP\n+ PARAMS: SCALE=(1+2)",
        "XINNER (p n) LEAF PARAMS: RVAL=1000*SCALE",
    );

    let plain = parse_xyce(&plain).expect("plain nested X instances parse");
    let parenthesized =
        parse_xyce(&parenthesized).expect("continued and nested actual-node wrappers parse");
    let plain = flatten_netlist_with_models(&plain).expect("plain hierarchy flattens");
    let parenthesized =
        flatten_netlist_with_models(&parenthesized).expect("parenthesized hierarchy flattens");

    assert_eq!(
        flattened_snapshot(&parenthesized.elements),
        flattened_snapshot(&plain.elements)
    );
    let ElementKind::Resistor { value, .. } = &parenthesized.elements[0].kind else {
        panic!(
            "expected flattened nested resistor, got {:?}",
            parenthesized.elements[0].kind
        );
    };
    assert_eq!(value.to_bits(), 3_000.0f64.to_bits());
}

#[test]
fn parameter_function_parentheses_after_the_subcircuit_name_remain_expressions() {
    let parsed = parse_xyce(
        "parenthesized tail expression\n\
         .SUBCKT CELL p n PARAMS: SCALE=1\n\
         R1 p n {{1000*SCALE}}\n\
         .ENDS CELL\n\
         X1 (in 0) CELL PARAMS: SCALE = if(1==1, pow(2, 3), 0)\n\
         .END\n",
    )
    .expect("parameter function after ')' parses as an expression");

    let ElementKind::Subcircuit { params, .. } = &parsed.elements[0].kind else {
        panic!("expected X instance, got {:?}", parsed.elements[0].kind);
    };
    assert!(params.iter().any(|(name, value)| {
        name.eq_ignore_ascii_case("SCALE")
            && matches!(value, ParametricValue::Expression(expression) if expression == "if(1==1, pow(2, 3), 0)")
    }));
    let flattened = flatten_netlist_with_models(&parsed).expect("parameterized instance flattens");
    let ElementKind::Resistor { value, .. } = &flattened.elements[0].kind else {
        panic!("expected flattened resistor");
    };
    assert!(
        (value - 8_000.0).abs() <= 8_000.0 * f64::EPSILON * 4.0,
        "parameter expression evaluated to {value} instead of 8000"
    );
}

#[test]
fn malformed_parenthesized_actual_node_forms_fail_at_the_instance_line() {
    for (instance, expected) in [
        ("X1 (in 0 CELL", "unterminated"),
        ("X1 in 0) CELL", "one balanced outer list"),
        ("X1 ((in 0)) CELL", "nested parentheses"),
        ("X1 () CELL", "cannot be empty"),
        ("X1 (in, 0) CELL", "commas are not separators"),
        ("X1(in 0) CELL", "separated from the instance name"),
        ("X1 (in 0)CELL", "separated by whitespace"),
        ("X1 (in 0)", "requires a subcircuit reference"),
        (
            "X1 (in 0) PARAMS: SCALE=1",
            "requires a subcircuit reference before",
        ),
        ("X1 (in 0) CELL extra", "unexpected trailing token"),
        ("X1 (in 0)) CELL", "separated by whitespace"),
        ("X1 (in P=1) CELL", "ambiguous token"),
        ("X1 (in {N}) CELL", "ambiguous token"),
        ("X1 (in 0) CELL P=", "malformed"),
        ("X1 (in 0) CELL P= Q=2", "malformed"),
        ("X1 (in 0) CELL P = Q=2", "malformed"),
        ("X1 (in 0) CELL P==1", "malformed"),
        ("X1 (in 0) CELL PARAMS: PARAMS: P=1", "at most one"),
        ("X1 (in 0) CELL P=1 PARAMS:", "before all"),
        ("X1 (in 0) CELL P=pow(2,3", "unterminated"),
        ("X1 (in 0) CELL P={1+2", "Unterminated"),
        ("X1 (in 0) CELL P='1+2", "Unterminated"),
    ] {
        let source = format!(
            "malformed parenthesized actual nodes\n\
             .SUBCKT CELL p n\n\
             R1 p n 1k\n\
             .ENDS CELL\n\
             {instance}\n\
             .END\n"
        );
        let error = parse_xyce(&source)
            .expect_err("malformed parenthesized actual nodes must fail")
            .to_string();
        assert!(
            error.to_ascii_lowercase().contains("line 5") && error.contains(expected),
            "'{instance}' returned the wrong diagnostic: {error}"
        );
    }
}

#[test]
fn continued_parenthesized_reference_keeps_its_editor_source_span() {
    let source = "parenthesized source span\n\
                  X1\n\
                  + (in 0)\n\
                  + CELL PARAMS: SCALE=pow(2, 3)\n\
                  .SUBCKT CELL p n PARAMS: SCALE=1\n\
                  R1 p n {SCALE}\n\
                  .ENDS\n\
                  .END\n";
    parse_xyce(source).expect("continued parenthesized instance parses");

    let map = rspice_core::netlist::source_map_for_editor(source);
    let reference = map
        .references
        .iter()
        .find(|reference| reference.kind == rspice_core::netlist::ReferenceKind::Subcircuit)
        .expect("subcircuit reference is mapped");
    assert_eq!(&source[reference.span.clone()], "CELL");
    assert_eq!(source[..reference.span.start].lines().count(), 4);
}

#[test]
fn parenthesized_actual_nodes_do_not_bypass_strict_subcircuit_arity() {
    let parsed = parse_xyce(
        "parenthesized arity\n\
         .SUBCKT CELL a b c\n\
         R1 a b 1k\n\
         .ENDS CELL\n\
         X1 (in out) CELL\n\
         .END\n",
    )
    .expect("instance syntax parses before hierarchy binding is validated");
    let error = flatten_netlist_with_models(&parsed)
        .expect_err("two actual nodes cannot bind a three-port subcircuit")
        .to_string();
    assert!(
        error.contains("connects 2 node(s)") && error.contains("declares 3 port(s)"),
        "strict arity returned the wrong diagnostic: {error}"
    );
}

#[test]
fn ordinary_x_instance_syntax_is_unchanged_in_xyce_and_default_dialects() {
    let source = "plain X syntax\n\
                  .SUBCKT CELL p n\n\
                  R1 p n 1k\n\
                  .ENDS CELL\n\
                  X1 in 0 CELL\n\
                  .END\n";
    let xyce = parse_xyce(source).expect("plain X syntax parses in Xyce mode");
    let default = Netlist::parse(source).expect("plain X syntax parses in the default dialect");

    assert_eq!(xyce.elements[0].nodes, default.elements[0].nodes);
    assert_eq!(xyce.elements[0].nodes, ["IN", "0"]);
    let xyce = flatten_netlist_with_models(&xyce).expect("Xyce plain instance flattens");
    let default = flatten_netlist_with_models(&default).expect("default plain instance flattens");
    assert_eq!(
        flattened_snapshot(&xyce.elements),
        flattened_snapshot(&default.elements)
    );
}

/// ngspice's `inp_subcktexpand` blanks the first balanced outer parenthesis
/// pair on every non-directive card before its tokenizer runs, so the
/// parenthesized actual-node list is part of the default dialect's grammar
/// too. Reading it as node names `(IN` and `0)` was a misparse, not a dialect
/// difference.
#[test]
fn parenthesized_actual_nodes_are_accepted_in_the_default_dialect() {
    let source = "default-dialect parenthesized instance\n\
                  .SUBCKT CELL p n\n\
                  R1 p n 1k\n\
                  .ENDS CELL\n\
                  X1 (in 0) CELL\n\
                  .END\n";
    let default = Netlist::parse(source).expect("default dialect accepts the parenthesized form");
    assert_eq!(default.elements[0].nodes, ["IN", "0"]);

    let xyce = parse_xyce(source).expect("Xyce accepts the parenthesized form");
    let default = flatten_netlist_with_models(&default).expect("default flattens");
    let xyce = flatten_netlist_with_models(&xyce).expect("Xyce flattens");
    assert_eq!(
        flattened_snapshot(&default.elements),
        flattened_snapshot(&xyce.elements)
    );
}

#[test]
fn default_dialect_parenthesized_instance_keeps_its_parameter_expressions() {
    let source = "default-dialect parenthesized parameters\n\
                  .SUBCKT CELL p n PARAMS: RVAL=1k\n\
                  R1 p n {RVAL}\n\
                  .ENDS CELL\n\
                  X1 (in 0) CELL PARAMS: RVAL=pow(2, 3)\n\
                  .END\n";
    let netlist = Netlist::parse(source).expect("parenthesized instance with parameters parses");
    let ElementKind::Subcircuit { params, .. } = &netlist.elements[0].kind else {
        panic!("X1 must remain a subcircuit instance");
    };
    let (_, value) = params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RVAL"))
        .expect("RVAL is retained");
    assert!(
        matches!(value, ParametricValue::Expression(expression) if expression.contains("pow")),
        "expected the pow() expression to survive, got {value:?}"
    );
}

#[test]
fn default_dialect_rejects_unbalanced_actual_node_parentheses() {
    for instance in [
        "X1 in 0) CELL",
        "X1 (in 0 CELL",
        "X1 ((in 0)) CELL",
        "X1 () CELL",
    ] {
        let source = format!(
            "default-dialect malformed parenthesized actual nodes\n\
             .SUBCKT CELL p n\n\
             R1 p n 1k\n\
             .ENDS CELL\n\
             {instance}\n\
             .END\n"
        );
        let error = Netlist::parse(&source)
            .expect_err("malformed parenthesized actual nodes must fail in every dialect")
            .to_string();
        assert!(
            error.contains("line 5") && error.to_ascii_lowercase().contains("parenthes"),
            "'{instance}' returned the wrong default-dialect diagnostic: {error}"
        );
    }
}

/// Xyce forbids a comma as an actual-node separator; ngspice's tokenizer
/// splits on commas everywhere, so the default dialect keeps that leniency.
#[test]
fn comma_separated_actual_nodes_follow_the_dialect_that_authored_them() {
    let source = "comma-separated actual nodes\n\
                  .SUBCKT CELL p n\n\
                  R1 p n 1k\n\
                  .ENDS CELL\n\
                  X1 (in, 0) CELL\n\
                  .END\n";
    let default = Netlist::parse(source).expect("the default dialect splits on commas");
    assert_eq!(default.elements[0].nodes, ["IN", "0"]);

    let error = parse_xyce(source)
        .expect_err("Xyce rejects commas inside the actual-node list")
        .to_string();
    assert!(error.contains("commas are not separators"), "{error}");
}
