//! Independent-source equation sides through actual solves and residual proofs.

use super::*;

#[test]
fn source_event_sides_solve_and_prove_distinct_analog_equilibria() {
    // Both a voltage jump and a current jump occur at exactly t=1. The
    // independent resistor equations give all three unknowns analytically.
    // This tests trial equations, not event acceptance or reactive impulses.
    let netlist = Netlist::parse(
        "source equation sides\n\
         V1 v 0 PWL(0 0 1 3 1 7 2 7)\n\
         RV v 0 1k\n\
         I1 0 i PWL(0 0 1 .002 1 .004 2 .004)\n\
         RI i 0 1k\n.end\n",
    )
    .unwrap();
    let engine = Engine::default().resolved_for_netlist(&netlist);
    let mut circuit = engine.build_circuit(&netlist).unwrap();
    let mut matrix = engine.build_matrix(&circuit).unwrap();
    circuit.link_indices(&matrix);
    let v = circuit.get_node_by_name("v").unwrap() - 1;
    let i = circuit.get_node_by_name("i").unwrap() - 1;
    let branch = circuit.num_nodes();
    let size = circuit.matrix_size();
    assert_eq!(size, 3);
    let coeff = CompanionCoefficients::backward_euler();
    let static_history = vec![0.0; size];
    let mut ctx = TransientSystemContext {
        bjt_phase: Default::default(),
        source_time_side: SourceTimeSide::Published,
        coeff: &coeff,
        xyce_one_step: false,
        xyce_one_step_order2: false,
        xyce_static_history: None,
        bsim4_trnqs_coeff: &coeff,
        bjt_history: &Default::default(),
        jfet_history: &Default::default(),
        diode_history: &Default::default(),
        diode_attempt_cache: None,
        mosfet_history: &Default::default(),
        mosfet_companion_slots: &[],
        vdmos_history: &Default::default(),
        vdmos_companion_slots: &[],
        b3soi_history: &Default::default(),
        b3soi_zero_first_transient_charge_derivative: false,
        bsim3_history: &Default::default(),
        bsim4_history: &Default::default(),
        ekv26_history: &Default::default(),
        suppress_gate_charge: false,
        baseline_diag_gmin: 0.0,
        tline_dc_refs: &[],
        coupled_tline_refs: &[],
        analysis_initial_step: false,
        analysis_final_step: false,
    };
    for (one_step, order2) in [(false, false), (true, false), (true, true)] {
        ctx.xyce_one_step = one_step;
        ctx.xyce_one_step_order2 = order2;
        ctx.xyce_static_history = order2.then_some(static_history.as_slice());
        for (side, expected_v, expected_i) in [
            (SourceTimeSide::LeftLimit, 3.0, 2.0),
            (SourceTimeSide::RightLimit, 7.0, 4.0),
        ] {
            ctx.source_time_side = side;
            let seed = vec![0.0; size];
            let mut rhs = vec![0.0; size];
            let solution = engine
                .rescue_transient_step_with_gmin_continuation(
                    &mut circuit,
                    &mut matrix,
                    &mut rhs,
                    &seed,
                    1.0,
                    0.25,
                    &ctx,
                    &mut [],
                    &NoAbort,
                )
                .unwrap()
                .expect("sided resistor equations converge");
            for (row, expected) in [
                (v, expected_v),
                (i, expected_i),
                (branch, -expected_v / 1000.0),
            ] {
                assert!(
                    (solution[row] - expected).abs() < 1e-12,
                    "{side:?}, OneStep={one_step}/{order2}, row={row}: {} != {expected}",
                    solution[row]
                );
            }
            assert!(
                engine
                    .transient_nonlinear_residual_converged(
                        &mut circuit,
                        &mut matrix,
                        &mut rhs,
                        &solution,
                        1.0,
                        0.25,
                        &ctx,
                        None,
                        &mut [],
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap()
            );
            let physical = engine
                .capture_xyce_static_residual_on_side(
                    &mut circuit,
                    &mut matrix,
                    &solution,
                    1.0,
                    0.0,
                    side,
                )
                .unwrap();
            assert!(physical.iter().all(|r| r.abs() < 1e-12));

            ctx.source_time_side = if side == SourceTimeSide::LeftLimit {
                SourceTimeSide::RightLimit
            } else {
                SourceTimeSide::LeftLimit
            };
            assert!(
                !engine
                    .transient_nonlinear_residual_converged(
                        &mut circuit,
                        &mut matrix,
                        &mut rhs,
                        &solution,
                        1.0,
                        0.25,
                        &ctx,
                        None,
                        &mut [],
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                "a root of one side must fail the opposite-side proof"
            );
            let opposite = engine
                .capture_xyce_static_residual_on_side(
                    &mut circuit,
                    &mut matrix,
                    &solution,
                    1.0,
                    0.0,
                    ctx.source_time_side,
                )
                .unwrap();
            assert!((opposite[branch].abs() - 4.0).abs() < 1e-12);
            assert!((opposite[i].abs() - 0.002).abs() < 1e-12);

            let mut projected = seed.clone();
            assert!(
                circuit
                    .enforce_ideal_voltage_constraints_on_side(&mut projected, 1.0, side)
                    .unwrap()
            );
            assert_eq!(projected[v], expected_v);
            // Projection only imposes voltage constraints; it cannot supply
            // the solved current-driven node or voltage-source lead current.
            assert_eq!(projected[i], 0.0);
            assert_eq!(projected[branch], 0.0);
        }
    }
}
