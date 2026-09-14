//! Core candidate ownership across refresh, reuse and speculative probes.

use super::*;
use crate::device::passive::JilesAthertonInductor;
use crate::device::veriloga_builtins::GeneratedEvaluationMode;
use crate::{SimulationConfig, StaticMatrix};

fn core(circuit: &crate::CircuitData) -> &JilesAthertonInductor {
    if let Some(group) = circuit.xyce_core_groups.first() {
        &group.device
    } else {
        &circuit.jiles_atherton_inductors[0].device
    }
}

fn check_core_evaluation_ownership(shared: bool) {
    let coupling = if shared {
        "L2 s 0 100\nR2 s 0 1k\nK1 L1 L2 1 nlcore"
    } else {
        "K1 L1 1 nlcore"
    };
    let netlist = Netlist::parse(&format!(
        "Core candidate ownership\nV1 in 0 0\nR1 in p 1k\nL1 p 0 10\n{coupling}\n.model nlcore core level=2 c=.001\n.end\n"
    )).unwrap();
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..Default::default()
    });
    let mut circuit = engine.build_circuit(&netlist).unwrap();
    let mut matrix = engine.build_matrix(&circuit).unwrap();
    circuit.link_indices(&matrix);
    let size = circuit.matrix_size();
    let mut solution = vec![0.0; size];
    let p = circuit.get_node_by_name("p").unwrap() - 1;
    let branch = circuit.get_branch_matrix_index(circuit.inductors.branch_indices[0]) - 1;
    circuit.initialize_xyce_core_q_histories();
    let bjt = Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    let jfet =
        Engine::initialize_jfet_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    let diode =
        Engine::initialize_diode_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    let mos =
        Engine::initialize_mosfet_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    let vdmos = Engine::initialize_vdmos_history(&circuit, &solution);
    let b3soi = Engine::initialize_b3soi_history(&circuit, &solution);
    let bsim3 = Engine::initialize_bsim3_history(&circuit, &solution);
    let bsim4 = Engine::initialize_bsim4_history(&circuit, &solution);
    let ekv = Engine::initialize_ekv26_history(&circuit, &solution);
    let coeff = CompanionCoefficients::backward_euler();
    let ctx = TransientSystemContext {
        source_time_side: crate::circuit::SourceTimeSide::Published,
        coeff: &coeff,
        xyce_one_step: true,
        xyce_one_step_order2: false,
        xyce_static_history: None,
        bsim4_trnqs_coeff: &coeff,
        bjt_history: &bjt,
        jfet_history: &jfet,
        diode_history: &diode,
        diode_attempt_cache: None,
        mosfet_history: &mos,
        mosfet_companion_slots: &[],
        vdmos_history: &vdmos,
        vdmos_companion_slots: &[],
        b3soi_history: &b3soi,
        b3soi_zero_first_transient_charge_derivative: false,
        bsim3_history: &bsim3,
        bsim4_history: &bsim4,
        ekv26_history: &ekv,
        suppress_gate_charge: false,
        baseline_diag_gmin: 0.0,
        tline_dc_refs: &[],
        coupled_tline_refs: &[],
        analysis_initial_step: false,
        analysis_final_step: false,
    };
    let mut rhs = vec![0.0; size];
    let mut vbic = Vec::new();
    let stamp = |circuit: &mut crate::CircuitData,
                 matrix: &mut StaticMatrix,
                 rhs: &mut Vec<Value>,
                 vbic: &mut Vec<Option<BjtChargeSnapshot>>,
                 point: &[Value],
                 refresh,
                 policy,
                 mode| {
        engine
            .stamp_transient_system_with_generated_mode(
                circuit, matrix, rhs, point, 1e-6, 1e-6, &ctx, vbic, refresh, policy, 0.0, mode,
            )
            .unwrap();
    };
    solution[p] = 0.1;
    solution[branch] = 0.001;
    stamp(
        &mut circuit,
        &mut matrix,
        &mut rhs,
        &mut vbic,
        &solution,
        true,
        CoreEvaluation::NewCandidate,
        GeneratedEvaluationMode::NewtonLimited,
    );
    let initial_carry = core(&circuit).xyce_core_mag_update();
    assert!(initial_carry > 0.0);
    let accepted_m = core(&circuit).magnetization();
    let accepted_ampere_turns = core(&circuit).xyce_core_old_ampere_turns();

    // Ordinary devices have already been refreshed by the postsolve walk.
    // The new Core endpoint still needs its own intermediate-state update.
    solution[p] = 0.3;
    solution[branch] = 0.003;
    engine
        .update_transient_nonlinear_devices(&mut circuit, &solution)
        .unwrap();
    let before_newton = circuit.nonlinear_state_snapshot();
    stamp(
        &mut circuit,
        &mut matrix,
        &mut rhs,
        &mut vbic,
        &solution,
        false,
        CoreEvaluation::NewCandidate,
        GeneratedEvaluationMode::NewtonLimited,
    );
    let candidate_carry = core(&circuit).xyce_core_mag_update();
    assert_ne!(
        candidate_carry, initial_carry,
        "a new Core endpoint must advance even when generic devices are refreshed"
    );
    assert_eq!(core(&circuit).magnetization(), accepted_m);
    assert_eq!(
        core(&circuit).xyce_core_old_ampere_turns(),
        accepted_ampere_turns
    );
    let candidate_values = matrix.values_mut().to_vec();
    let candidate_rhs = rhs.clone();
    let candidate_state = circuit.nonlinear_state_snapshot();

    for _ in 0..3 {
        stamp(
            &mut circuit,
            &mut matrix,
            &mut rhs,
            &mut vbic,
            &solution,
            false,
            CoreEvaluation::ReuseCandidate,
            GeneratedEvaluationMode::NewtonLimited,
        );
        assert_eq!(core(&circuit).xyce_core_mag_update(), candidate_carry);
        assert_eq!(matrix.values_mut(), candidate_values);
        assert_eq!(rhs, candidate_rhs);
    }
    // An actual next Newton evaluation may have identical electrical bits.
    // Its carried magnetic fixed-point update still differs from reuse.
    stamp(
        &mut circuit,
        &mut matrix,
        &mut rhs,
        &mut vbic,
        &solution,
        false,
        CoreEvaluation::NewCandidate,
        GeneratedEvaluationMode::NewtonLimited,
    );
    assert_ne!(core(&circuit).xyce_core_mag_update(), candidate_carry);
    circuit.restore_nonlinear_state(candidate_state.clone());
    let mut probe = solution.clone();
    probe[p] += 0.05;
    probe[branch] += 0.0001;
    for _ in 0..3 {
        stamp(
            &mut circuit,
            &mut matrix,
            &mut rhs,
            &mut vbic,
            &probe,
            true,
            CoreEvaluation::ReuseCandidate,
            GeneratedEvaluationMode::StaticProbe,
        );
        assert_eq!(core(&circuit).xyce_core_mag_update(), candidate_carry);
        assert_eq!(core(&circuit).magnetization(), accepted_m);
        assert_eq!(
            core(&circuit).xyce_core_old_ampere_turns(),
            accepted_ampere_turns
        );
    }
    circuit.restore_nonlinear_state(candidate_state);
    let _ = engine
        .capture_xyce_static_residual(&mut circuit, &mut matrix, &solution, 1e-6, 0.0)
        .unwrap();
    assert_eq!(core(&circuit).xyce_core_mag_update(), candidate_carry);
    circuit.restore_nonlinear_state(before_newton);
    stamp(
        &mut circuit,
        &mut matrix,
        &mut rhs,
        &mut vbic,
        &solution,
        false,
        CoreEvaluation::NewCandidate,
        GeneratedEvaluationMode::NewtonLimited,
    );
    assert_eq!(core(&circuit).xyce_core_mag_update(), candidate_carry);
    assert_eq!(matrix.values_mut(), candidate_values);
    assert_eq!(rhs, candidate_rhs);
    circuit.commit_xyce_core_inductances(&solution, 1e-6, false);
    assert_eq!(core(&circuit).magnetization(), accepted_m + candidate_carry);
    assert_eq!(core(&circuit).xyce_core_mag_update(), candidate_carry);
    assert_eq!(
        core(&circuit).xyce_core_old_ampere_turns(),
        10.0 * solution[branch]
    );
}

#[test]
fn shared_core_separates_candidate_advancement_from_generic_refresh_and_probes() {
    check_core_evaluation_ownership(true);
}

#[test]
fn standalone_core_separates_candidate_advancement_from_cached_jacobian() {
    check_core_evaluation_ownership(false);
}
