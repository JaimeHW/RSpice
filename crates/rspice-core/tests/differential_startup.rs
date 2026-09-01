use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

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
