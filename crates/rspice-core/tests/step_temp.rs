use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, Netlist, StepCommand, StepSweep, StepTarget};
use rspice_core::solver::SimulationResult;

fn step_values(step: &StepCommand) -> Vec<f64> {
    match &step.sweep {
        StepSweep::List(values) => values.clone(),
        other => panic!("test expects LIST sweep, got {other:?}"),
    }
}

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn explicit_temp_deck(temp_c: f64) -> String {
    format!(
        "* diode temperature reference\n\
         .options temp={temp_c:.15}\n\
         v1 in 0 dc 0.7\n\
         d1 in 0 dmod\n\
         .model dmod d (is=1e-14 n=1 rs=0 cjo=0)\n\
         .op\n\
         .end\n"
    )
}

#[test]
fn step_temp_list_rebuilds_operating_point_at_each_temperature() {
    let deck = "* .STEP TEMP should execute as temperature-configured OP runs\n\
                v1 in 0 dc 0.7\n\
                d1 in 0 dmod\n\
                .model dmod d (is=1e-14 n=1 rs=0 cjo=0)\n\
                .step temp list 25 100\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect(".STEP TEMP deck parses");
    let step = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .expect(".STEP TEMP command captured");
    assert_eq!(step.target, StepTarget::Temp);

    let values = step_values(step);
    let stepped = Engine::default()
        .run_step_command(&netlist, step, &values)
        .expect(".STEP TEMP should execute OP runs at each requested temperature");

    assert_eq!(stepped.len(), 2, "both temperature points should solve");
    for ((temp_c, result), expected_temp) in stepped.iter().zip([25.0, 100.0]) {
        assert!(
            (*temp_c - expected_temp).abs() < 1.0e-12,
            "step result should preserve requested temperature, got {temp_c}"
        );

        let direct_netlist =
            Netlist::parse(&explicit_temp_deck(expected_temp)).expect("direct temp deck parses");
        let direct = Engine::default()
            .run_dc_op(&direct_netlist)
            .expect("direct temperature OP solves");
        let got = branch_current(result, "v1");
        let expected = branch_current(&direct, "v1");
        let abs = (got - expected).abs();
        let tol = 1.0e-12_f64.max(1.0e-10 * expected.abs());
        assert!(
            abs <= tol,
            ".STEP TEMP {expected_temp}C I(V1) mismatch: got {got:.12e}, expected {expected:.12e}, abs {abs:.3e} > {tol:.3e}"
        );
    }

    let low_temp_current = branch_current(&stepped[0].1, "v1");
    let high_temp_current = branch_current(&stepped[1].1, "v1");
    assert!(
        (low_temp_current - high_temp_current).abs() > 1.0e-9,
        "temperature-stepped diode current should visibly change: 25C={low_temp_current:.12e}, 100C={high_temp_current:.12e}"
    );
}
