use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Netlist, ParseError, StartupDirectiveKind};

fn waveform_at_zero(result: &rspice_core::engine::TransientResult, node: &str) -> f64 {
    result
        .try_voltage_waveform_named(node)
        .unwrap_or_else(|| panic!("missing waveform for {node}"))[0]
}

fn dc_voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing operating-point node {node}"));
    result.node_voltages[index]
}

#[test]
fn uic_applies_a_true_differential_initial_condition() {
    let netlist = Netlist::parse(
        "differential UIC\n\
         C1 a 0 1u\n\
         C2 b 0 1u\n\
         R1 a 0 1meg\n\
         R2 b 0 1meg\n\
         .IC V(a,b)=1.5\n\
         .TRAN 1u 2u UIC\n\
         .END\n",
    )
    .expect("differential UIC deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 2e-6, 1e-6)
        .expect("differential UIC transient solves");
    let a = waveform_at_zero(&result, "a");
    let b = waveform_at_zero(&result, "b");
    assert_eq!((a - b).to_bits(), 1.5f64.to_bits());
}

#[test]
fn ground_may_be_either_differential_terminal() {
    for (target, expected) in [("V(a,0)=2", 2.0_f64), ("V(0,a)=-2", 2.0_f64)] {
        let source = format!(
            "ground terminal differential IC\nC1 a 0 1u\nR1 a 0 1meg\n.IC {target}\n.TRAN 1u 1u UIC\n.END\n"
        );
        let netlist = Netlist::parse(&source).expect("ground-terminal IC parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 1e-6, 1e-6)
            .expect("ground-terminal UIC transient solves");
        assert_eq!(waveform_at_zero(&result, "a").to_bits(), expected.to_bits());
    }
}

#[test]
fn ordinary_transient_holds_the_difference_during_its_t0_bias_solve() {
    let netlist = Netlist::parse(
        "differential constrained bias\n\
         V1 in 0 2\n\
         R1 in a 1k\n\
         R2 a b 1k\n\
         R3 b 0 1k\n\
         C1 b 0 1u\n\
         .IC V(a,b)=1\n\
         .TRAN 1u 2u\n\
         .END\n",
    )
    .expect("ordinary differential startup parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 2e-6, 1e-6)
        .expect("ordinary differential startup solves");
    let a = waveform_at_zero(&result, "a");
    let b = waveform_at_zero(&result, "b");
    assert!(
        (a - 1.0).abs() <= 5e-12,
        "unexpected constrained V(a)={a:e}"
    );
    assert!(b.abs() <= 5e-12, "unexpected constrained V(b)={b:e}");
    assert_eq!((a - b).to_bits(), 1.0f64.to_bits());
}

#[test]
fn consistent_dependent_constraints_are_reduced_without_a_singular_row() {
    let netlist = Netlist::parse(
        "dependent differential constraints\n\
         C1 a 0 1u\n\
         C2 b 0 1u\n\
         R1 a 0 1meg\n\
         R2 b 0 1meg\n\
         .IC V(a,b)=1 V(b,0)=2 V(a,0)=3\n\
         .TRAN 1u 1u UIC\n\
         .END\n",
    )
    .expect("consistent dependent startup graph parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 1e-6, 1e-6)
        .expect("rank-reduced UIC transient solves");
    assert_eq!(waveform_at_zero(&result, "a").to_bits(), 3.0f64.to_bits());
    assert_eq!(waveform_at_zero(&result, "b").to_bits(), 2.0f64.to_bits());
}

#[test]
fn scoped_differential_ic_targets_concrete_instance_nodes() {
    let netlist = Netlist::parse(
        "hierarchical differential UIC\n\
         X1 a b CELL PARAMS: DV=0.75\n\
         C1 a 0 1u\n\
         C2 b 0 1u\n\
         .SUBCKT CELL p n PARAMS: DV=1\n\
         .IC V(p,n)={DV}\n\
         R1 p n 1k\n\
         .ENDS\n\
         .TRAN 1u 1u UIC\n\
         .END\n",
    )
    .expect("hierarchical differential IC parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 1e-6, 1e-6)
        .expect("hierarchical differential UIC transient solves");
    let difference = waveform_at_zero(&result, "a") - waveform_at_zero(&result, "b");
    assert_eq!(difference.to_bits(), 0.75f64.to_bits());
}

#[test]
fn differential_nodeset_is_a_released_guess_not_a_final_dc_constraint() {
    let netlist = Netlist::parse(
        "differential NODESET release\n\
         V1 in 0 1\n\
         R1 in a 1k\n\
         D1 a b DTEST\n\
         R2 b 0 1k\n\
         .MODEL DTEST D(IS=1e-12 N=1)\n\
         .NODESET V(a,b)=0.1\n\
         .OP\n\
         .END\n",
    )
    .expect("differential NODESET parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("differential NODESET operating point converges");
    let difference = dc_voltage(&result, "a") - dc_voltage(&result, "b");
    assert!(
        (difference - 0.1).abs() > 0.05,
        "NODESET remained installed in the final DC equations: {difference:e}"
    );
}

#[test]
fn consistent_duplicate_and_reversed_constraints_are_equivalent() {
    let baseline = Netlist::parse(
        "baseline differential UIC\n\
         C1 a 0 1u\n\
         C2 b 0 1u\n\
         .IC V(a,b)=1.25\n\
         .TRAN 1u 1u UIC\n\
         .END\n",
    )
    .expect("baseline constraint parses");
    let duplicate = Netlist::parse(
        "duplicate differential UIC\n\
         C1 a 0 1u\n\
         C2 b 0 1u\n\
         .IC V(a,b)=1.25 V(a,b)=1.25 V(b,a)=-1.25\n\
         .TRAN 1u 1u UIC\n\
         .END\n",
    )
    .expect("consistent duplicate constraints parse");
    let engine = Engine::new(SimulationConfig::default());
    let baseline = engine
        .run_tran(&baseline, 1e-6, 1e-6)
        .expect("baseline UIC transient solves");
    let duplicate = engine
        .run_tran(&duplicate, 1e-6, 1e-6)
        .expect("duplicate UIC transient solves");
    for node in ["a", "b"] {
        assert_eq!(
            waveform_at_zero(&duplicate, node).to_bits(),
            waveform_at_zero(&baseline, node).to_bits(),
            "duplicate constraint changed {node}"
        );
    }
}

#[test]
fn inconsistent_duplicate_constraint_reports_both_source_lines() {
    let error = Netlist::parse(
        "duplicate conflict\n\
         R1 a 0 1k\n\
         R2 b 0 1k\n\
         .IC V(a,b)=1\n\
         .IC V(b,a)=-2\n\
         .END\n",
    )
    .expect_err("inconsistent duplicate constraint must fail");
    let ParseError::StartupConstraintConflict(conflict) = error else {
        panic!("expected typed startup conflict, got {error}");
    };
    assert_eq!(conflict.kind, StartupDirectiveKind::Ic);
    assert_eq!(conflict.established.line, 4);
    assert_eq!(conflict.conflicting.line, 5);
    assert_eq!(conflict.positive, "B");
    assert_eq!(conflict.negative, "A");
    assert_eq!(conflict.expected.to_bits(), (-1.0f64).to_bits());
    assert_eq!(conflict.actual.to_bits(), (-2.0f64).to_bits());
}

#[test]
fn malformed_or_nonfinite_voltage_targets_fail_at_the_authored_line() {
    for (directive, expected) in [
        (".IC V(a,b) 1", "requires '=value'"),
        (".NODESET V(a,b) 1", "requires '=value'"),
        (".IC V(a,b)=1e309", "must be finite"),
        (".NODESET V(a,b)=-1e309", "must be finite"),
        (".IC V(a,b,c)=1", "Expected ')'"),
        (".NODESET V(a,)=1", "Expected node"),
    ] {
        let source =
            format!("malformed differential startup\nR1 a 0 1k\nR2 b 0 1k\n{directive}\n.END\n");
        let error = Netlist::parse(&source)
            .expect_err("malformed differential startup syntax must fail")
            .to_string();
        assert!(
            error.contains("line 4") && error.contains(expected),
            "{directive} returned the wrong diagnostic: {error}"
        );
    }
}

#[test]
fn scoped_differential_nodeset_matches_the_flat_constraint_and_is_released() {
    let flat = Netlist::parse(
        "flat differential nodeset\n\
         V1 in 0 1\n\
         R1 in a 1k\n\
         D1 a b DTEST\n\
         R2 b 0 1k\n\
         .MODEL DTEST D(IS=1e-12 N=1)\n\
         .NODESET V(a,b)=0.1\n\
         .OP\n\
         .END\n",
    )
    .expect("flat nodeset parses");
    let scoped = Netlist::parse(
        "scoped differential nodeset\n\
         V1 in 0 1\n\
         X1 in 0 CELL PARAMS: GUESS=0.1\n\
         .SUBCKT CELL p n PARAMS: GUESS=0\n\
         R1 p a 1k\n\
         D1 a b DTEST\n\
         R2 b n 1k\n\
         .MODEL DTEST D(IS=1e-12 N=1)\n\
         .NODESET V(a,b)={GUESS}\n\
         .ENDS\n\
         .OP\n\
         .END\n",
    )
    .expect("scoped nodeset parses");
    let engine = Engine::new(SimulationConfig::default());
    let flat = engine.run_dc_op(&flat).expect("flat nodeset OP solves");
    let scoped = engine.run_dc_op(&scoped).expect("scoped nodeset OP solves");
    let flat_difference = dc_voltage(&flat, "a") - dc_voltage(&flat, "b");
    let scoped_difference = dc_voltage(&scoped, "X1.A") - dc_voltage(&scoped, "X1.B");
    assert!(
        (flat_difference - scoped_difference).abs() <= 1e-12,
        "flat={flat_difference:e}, scoped={scoped_difference:e}"
    );
    assert!(
        (scoped_difference - 0.1).abs() > 0.05,
        "scoped NODESET remained installed: {scoped_difference:e}"
    );
}
