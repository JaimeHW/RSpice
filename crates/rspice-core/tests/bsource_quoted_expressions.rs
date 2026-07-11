//! SPICE single-quoted behavioral expression groups.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::expr::{Expr, Function, parse_expression_strict};
use rspice_core::netlist::expr::{
    ParamContext, eval_expression, parse_expression as parse_netlist_expression,
};
use rspice_core::netlist::{ExpressionDialect, Netlist, NetlistParseOptions};

fn parse_xyce(deck: &str) -> Netlist {
    Netlist::parse_with_options(
        deck,
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..NetlistParseOptions::default()
        },
    )
    .expect("Xyce deck parses")
}

fn xyce_engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..SimulationConfig::default()
    })
}

#[test]
fn both_expression_parsers_treat_single_quotes_as_grouping() {
    let value = eval_expression("'1 + 2' * 3", &ParamContext::new())
        .expect("netlist expression parser accepts quoted groups");
    assert_eq!(value, 9.0);

    let expression = parse_expression_strict("'1 + 2' * 3")
        .expect("strict behavioral parser accepts quoted groups");
    assert!(!matches!(expression, Expr::StringLiteral(_)));
}

fn node_voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    result.node_voltages[index]
}

#[test]
fn single_quoted_expression_groups_follow_parameter_steps() {
    let netlist = parse_xyce(
        "\
quoted expression step
.param eric=0.9
.param vdd={eric*2}
B1 out 0 V={table(0, 0, 'vdd', 1, 'vdd') + 0*eric}
R1 out 0 1k
.step eric list 0.5 0.9
.op
.end
",
    );

    let results = xyce_engine()
        .run_step(&netlist, "eric", &[0.5, 0.9])
        .expect("parameter step runs");
    let voltages = results
        .iter()
        .map(|(_, result)| node_voltage(result, "out"))
        .collect::<Vec<_>>();

    assert_eq!(voltages.len(), 2);
    assert!((voltages[0] - 1.0).abs() < 1.0e-12, "{voltages:?}");
    assert!((voltages[1] - 1.8).abs() < 1.0e-12, "{voltages:?}");
}

#[test]
fn single_quoted_table_ordinates_are_numeric_expressions() {
    let netlist = parse_xyce(
        "\
quoted table ordinates
.param low=0.1 high=1.8
B1 out 0 V={table(time, 0, 'low', 1n, 'low', 1.001n, 'high', 2n, 'high')}
R1 out 0 1k
.tran 0.1n 2n
.end
",
    );
    let result = xyce_engine()
        .run_tran(&netlist, 2.0e-9, 0.1e-9)
        .expect("transient runs");
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node exists");

    let before_edge = result
        .time
        .iter()
        .zip(&result.voltages[node])
        .min_by(|(ta, _), (tb, _)| (*ta - 0.5e-9).abs().total_cmp(&(*tb - 0.5e-9).abs()))
        .expect("transient has samples");
    let after_edge = result
        .time
        .iter()
        .zip(&result.voltages[node])
        .min_by(|(ta, _), (tb, _)| (*ta - 1.5e-9).abs().total_cmp(&(*tb - 1.5e-9).abs()))
        .expect("transient has samples");

    assert!((before_edge.1 - 0.1).abs() < 1.0e-9, "{before_edge:?}");
    assert!((after_edge.1 - 1.8).abs() < 1.0e-9, "{after_edge:?}");
}

#[test]
fn double_quoted_table_filename_remains_a_string_literal() {
    let expression = parse_expression_strict("table(\"engineer's-wave.dat\")")
        .expect("double-quoted table filename parses");
    let Expr::Function {
        func: Function::Table,
        args,
    } = expression
    else {
        panic!("expected table function");
    };
    assert!(
        matches!(args.as_slice(), [Expr::StringLiteral(path)] if path == "engineer's-wave.dat")
    );
}

#[test]
fn unmatched_single_quoted_expression_is_rejected() {
    let netlist_error = parse_netlist_expression("table(time, 0, '1 + 2)")
        .expect_err("permissive expression parser must reject an unterminated quote");
    assert!(
        netlist_error
            .to_string()
            .contains("Missing closing single quote"),
        "unexpected diagnostic: {netlist_error}"
    );

    let error = parse_expression_strict("table(time, 0, '1 + 2)")
        .expect_err("unterminated expression quote must fail");
    assert!(
        error.to_string().contains("Missing closing single quote"),
        "unexpected diagnostic: {error}"
    );
}
