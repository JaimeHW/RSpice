//! A `.func` formal written inside `v()` or `i()` in the function's body reads
//! as the simulator the deck was written for reads it. LTspice binds it to the
//! node or device the call passes; ngspice and Xyce read the node or device
//! literally named after the formal.

use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};

fn op_voltages(netlist: &Netlist, config: SimulationConfig, nodes: &[&str]) -> Vec<f64> {
    let op = Engine::new(config)
        .run_dc_op(netlist)
        .expect("operating point solves");
    nodes
        .iter()
        .map(|node| {
            op.try_voltage_named(node)
                .unwrap_or_else(|| panic!("no voltage for {node}"))
        })
        .collect()
}

fn parse(deck: &str, options: NetlistParseOptions) -> Netlist {
    Netlist::parse_with_options(deck, options)
        .unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

/// ngspice's own regression deck `regression/func/func-1`, without its
/// `.control` script, which compares each `n<k>_t` with `n<k>_g`. Nodes `n`,
/// `p` and `x` and the source `vp` share their names with `.func` formals:
/// `bar2(17.0)` reads `v(p)+17` = 119 and `baz2(1000.0,1e20)` reads
/// `v(p)+1000+v(n)` = 1203. ngspice-46 prints "0 of 15 tests failed"
/// (func-1.out).
#[test]
fn the_ngspice_dialect_reads_ngspice_s_func_regression_deck_as_ngspice_does() {
    let deck = "* 'func-1' check .func processing
.options rspice_dialect=ngspice
vncol1  n 0  101.0
vncol2  p 0  102.0
vncol3  x 0  103.0
vp  1 0  105.0
rp  1 0  1.0
.func foo0()  '1013.0'
v1001_t  n1001_t 0  '100*foo0()'
v1001_g  n1001_g 0  '101300.0'
.func bar1(p) 'p'
v1002_t  n1002_t 0  'bar1(117.0)'
v1003_t  n1003_t 0  '2*bar1(117.0+2.0)'
v1002_g  n1002_g 0  '117.0'
v1003_g  n1003_g 0  '238.0'
.func bar2(p) 'v(p)+p'
b1004_t  n1004_t 0  v = 'bar2(17.0)'
v1004_g  n1004_g 0  '119.0'
.func bar3(p) 'p+v(p)'
b1005_t  n1005_t 0  v = 'bar3(17.0)'
v1005_g  n1005_g 0  '119.0'
.func bar4(p) 'p+v(p)+p'
b1006_t  n1006_t 0  v = 'bar4(17.0)'
v1006_g  n1006_g 0  '136.0'
.func baz1(n,vp) 'n+i(vp)+vp'
b1007_t  n1007_t 0  v = 'baz1(17.0,10000)'
v1007_g  n1007_g 0  '9912'
.func baz2(p,n)  'v(p)+p+v(n)'
b1008_t  n1008_t 0  v = 'baz2(1000.0,1e20)'
v1008_g  n1008_g 0  '1203.0'
.func baz3(x)  'x+x'
v1009_t  n1009_t 0  '17*baz3(100.0)'
v1009_g  n1009_g 0  '3400.0'
.func moo1(x,p,n) 'x<p?p:n'
v1010_t  n1010_t 0  '10*moo1(2,3,5)'
v1010_g  n1010_g 0  '30'
v1011_t  n1011_t 0  '10*moo1(4,3,5)'
v1011_g  n1011_g 0  '50'
.func moo2(x,p,n) 'x<p?p:x>n?n:x'
v1012_t  n1012_t 0  '10*moo2(4,3,5)*100'
v1012_g  n1012_g 0  '4000'
v1013_t  n1013_t 0  '10*moo2(2,3,5)*100'
v1013_g  n1013_g 0  '3000'
v1014_t  n1014_t 0  '10*moo2(6,3,5)*100'
v1014_g  n1014_g 0  '5000'
.param xoo = 100
.func fun1(a, xoo) 'a*xoo'
v1015_t  n1015_t 0  'fun1(xoo,2)'
v1015_g  n1015_g 0  '200'
.op
.end
";
    let netlist = parse(deck, NetlistParseOptions::default());
    let tests = (1001..=1015)
        .flat_map(|n| [format!("n{n}_t"), format!("n{n}_g")])
        .collect::<Vec<_>>();
    let nodes = tests.iter().map(String::as_str).collect::<Vec<_>>();
    let voltages = op_voltages(&netlist, SimulationConfig::default(), &nodes);
    for (pair, values) in nodes.chunks(2).zip(voltages.chunks(2)) {
        assert!(
            (values[0] - values[1]).abs() <= 1e-9,
            "v({}) = {} but ngspice-46 gives {}",
            pair[0],
            values[0],
            values[1]
        );
    }
}

/// One deck whose `.func` formals share their names with nodes `p` and `n`
/// and the source `vd`. LTspice 17.2.4 (macOS) binds each formal to the
/// call's argument: V(o1..o3) = 4, 7, -2, with "Questionable use of curly
/// braces" warnings. ngspice-46 reads the nodes and the source named after the
/// formals: 103, 203, -5. Xyce 7.10 builds `v()` and `i()` on the name as
/// written and binds a formal only where it is a parameter operand
/// (ExpressionParser.yxx `createVoltageOp`, newExpression.C), so it reads them
/// as ngspice does.
const FORMALS_NAMED_LIKE_NODES: &str = "* .func formals inside v() and i()
vncol1 n 0 101
vncol2 p 0 102
va a 0 3
vb b 0 4
rb b 0 2
vd d 0 5
rd d 0 1
.func bar2(p) {v(p)+1}
.func baz2(p,n) {v(p)+v(n)}
.func cur(vd) {i(vd)}
b1 o1 0 v=bar2(a)
r1 o1 0 1k
b2 o2 0 v=baz2(a,b)
r2 o2 0 1k
b3 o3 0 v=cur(vb)
r3 o3 0 1k
.op
.end
";

const BOUND: [f64; 3] = [4.0, 7.0, -2.0];
const LITERAL: [f64; 3] = [103.0, 203.0, -5.0];

fn deck_with(options: &str) -> String {
    FORMALS_NAMED_LIKE_NODES.replacen(".op\n", &format!("{options}\n.op\n"), 1)
}

fn assert_outputs(label: &str, actual: &[f64], expected: [f64; 3]) {
    for (node, (actual, expected)) in ["o1", "o2", "o3"].iter().zip(actual.iter().zip(expected)) {
        assert!(
            (actual - expected).abs() <= 1e-9,
            "{label}: V({node}) = {actual}, expected {expected}"
        );
    }
}

#[test]
fn a_formal_inside_a_probe_reads_as_the_deck_s_simulator_reads_it() {
    let outputs = ["o1", "o2", "o3"];

    // The default reads as LTspice, the highest-ranked simulator whose
    // reading is established; a deck drawn in LTspice says so.
    for options in ["", ".options rspice_source=ltspice"] {
        let netlist = parse(&deck_with(options), NetlistParseOptions::default());
        let actual = op_voltages(&netlist, SimulationConfig::default(), &outputs);
        assert_outputs(&format!("LTspice ({options:?})"), &actual, BOUND);
    }

    // The ngspice dialect, from the deck or from the caller.
    let netlist = parse(
        &deck_with(".options rspice_dialect=ngspice"),
        NetlistParseOptions::default(),
    );
    let actual = op_voltages(&netlist, SimulationConfig::default(), &outputs);
    assert_outputs("ngspice deck option", &actual, LITERAL);
    let netlist = parse(
        FORMALS_NAMED_LIKE_NODES,
        NetlistParseOptions {
            spice_dialect: Some(SpiceDialect::Ngspice),
            ..Default::default()
        },
    );
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let actual = op_voltages(&netlist, config, &outputs);
    assert_outputs("ngspice dialect", &actual, LITERAL);

    // The Xyce dialect as `--spice-dialect xyce` selects it, and Xyce's
    // expression functions alone, as the Xyce regression suite reads a deck.
    for spice_dialect in [Some(SpiceDialect::Xyce), None] {
        let netlist = parse(
            FORMALS_NAMED_LIKE_NODES,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                spice_dialect,
                ..Default::default()
            },
        );
        let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        let actual = op_voltages(&netlist, config, &outputs);
        assert_outputs(&format!("Xyce ({spice_dialect:?})"), &actual, LITERAL);
    }

    // A deck drawn in LTspice reads as LTspice in any dialect.
    let netlist = parse(
        &deck_with(".options rspice_source=ltspice"),
        NetlistParseOptions {
            spice_dialect: Some(SpiceDialect::Ngspice),
            ..Default::default()
        },
    );
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let actual = op_voltages(&netlist, config, &outputs);
    assert_outputs("LTspice deck in the ngspice dialect", &actual, BOUND);
}
