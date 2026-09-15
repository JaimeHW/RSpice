//! Qualification of the exact forward-transport operator before admission.

use super::*;
use crate::device::Bjt;
use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

mod event_trials;
mod promoted;

#[test]
fn gp_phase_history_acceptance_uses_actual_time_and_survives_integration_restarts() {
    let netlist = Netlist::parse("phase acceptance\nQ1 c b 0 qm\nRc c 0 1k\nRb b 0 1k\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=0.2p VAF=20)\n.end\n").unwrap();
    let engine = Engine::default();
    let mut circuit = engine.build_circuit(&netlist).unwrap();
    assert!(!circuit.bjts.devices[0].mna_promoted());
    let mut solution = vec![0.0; circuit.matrix_size()];
    let base = circuit.bjts.devices[0].node_base - 1;
    solution[circuit.bjts.devices[0].node_collector - 1] = 2.0;
    solution[base] = 0.6;
    let mut history =
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    let coeff = CompanionCoefficients::backward_euler();
    for (index, time) in [0.7 * delay, 1.3 * delay, 4.2 * delay]
        .into_iter()
        .enumerate()
    {
        solution[base] += 0.01;
        let before = history.clone();
        let phase = history.phase_trial(0, time).unwrap();
        phase
            .correction(&circuit.bjts.devices[0], &history.dynamic_internal_prev[0])
            .unwrap();
        assert_eq!(
            history, before,
            "speculative phase evaluation changed history"
        );
        // Deliberately distinct from the absolute clock: using dt or summing
        // accepted widths would store the wrong landing time.
        Engine::accept_bjt_history(
            &circuit,
            &mut history,
            &solution,
            &coeff,
            0.4 * delay,
            time,
            None,
        )
        .unwrap();
        let forward = circuit.bjts.devices[0]
            .legacy_forward_transport_branch(&history.dynamic_internal_prev[0])
            .unwrap();
        let buffer = history.phase[0].as_ref().unwrap();
        assert_eq!(
            buffer.accepted_samples().next_back(),
            Some((time, forward.current))
        );
        buffer.validate_accepted_time(time).unwrap();
        assert!(buffer.accepted_sample_count() <= index + 2);
        let accepted = history.clone();
        assert!(
            Engine::accept_bjt_history(
                &circuit,
                &mut history,
                &solution,
                &coeff,
                delay,
                time,
                None
            )
            .is_err()
        );
        assert_eq!(history, accepted, "duplicate acceptance rotated history");
    }
    let accepted = history.phase.clone();
    for restart in [
        AcceptedJunctionHistoryRestart::Preserve,
        AcceptedJunctionHistoryRestart::Reinitialize,
    ] {
        Engine::reseed_reactive_histories_for_restart(
            &mut circuit,
            &solution,
            delay,
            restart,
            TransientDeviceHistories {
                bjt: &mut history,
                jfet: &mut JfetTransientHistory::default(),
                diode: &mut DiodeTransientHistory::default(),
                mosfet: &mut MosfetTransientHistory::default(),
                vdmos: &mut VdmosTransientHistory::default(),
                b3soi: &mut B3SoiTransientHistory::default(),
                bsim3: &mut Bsim3TransientHistory::default(),
                bsim4: &mut Bsim4TransientHistory::default(),
                ekv26: &mut Ekv26TransientHistory::default(),
            },
        );
        assert_eq!(
            history.phase, accepted,
            "integration restart erased physical memory"
        );
    }
}

#[test]
fn gp_phase_rejects_history_from_another_nominal_delay() {
    let bjt = transistor(1.0, false, 1.0);
    let snapshot = bjt.charge_snapshot(2.0, 0.68, 0.0, 0.0);
    let delay = bjt.legacy_excess_phase_delay();
    let mut history = DelayBuffer::new(0);
    history
        .restore_checkpoint(&DelayCheckpoint {
            event_orders: Vec::new(),
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Fixed {
                delay: delay.next_up(),
            }),
            samples: vec![(0.0, 1e-5)],
        })
        .unwrap();
    let checkpoint = history.checkpoint();
    let trial = BjtPhaseTrial {
        history: (&history).into(),
        time: delay,
        left_limit: None,
        incoming_arrival: false,
    };
    assert!(
        trial
            .correction(&bjt, &snapshot.reduction.internal_voltages)
            .unwrap_err()
            .contains("different nominal phase delay")
    );
    assert_eq!(history.checkpoint(), checkpoint);
}

fn transistor(p: Value, private: bool, scale: Value) -> Bjt {
    let params = [
        ("LEVEL", 1.0),
        ("IS", 1e-16),
        ("BF", 100.0),
        ("BR", 2.0),
        ("VAF", 20.0),
        ("VAR", 10.0),
        ("IKF", 0.01),
        ("IKR", 0.02),
        ("TF", 1e-9),
        ("PTF", 30.0),
        ("CJE", 1e-12),
        ("CJC", 2e-13),
        ("RB", if private { 100.0 } else { 0.0 }),
        ("RBM", if private { 20.0 } else { 0.0 }),
        ("IRB", 1e-5),
        ("RC", if private { 2.0 } else { 0.0 }),
        ("RE", if private { 1.0 } else { 0.0 }),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_owned(), value))
    .collect();
    let mut bjt = if p > 0.0 {
        Bjt::new_npn("q".into(), 1, 2, 3)
    } else {
        Bjt::new_pnp("q".into(), 1, 2, 3)
    }
    .with_params(&params)
    .with_instance_params(&[("M".into(), scale)]);
    bjt.set_junction_gmin(0.0);
    bjt
}

fn forward_reference(bjt: &Bjt, p: Value, v: &[Value; BJT_INTERNAL_STATE_DIM]) -> Value {
    let vbe = p * (v[BJT_VBI_STATE_INDEX] - v[BJT_VEI_STATE_INDEX]);
    let vbc = p * (v[BJT_VBI_STATE_INDEX] - v[BJT_VCI_STATE_INDEX]);
    let forward = bjt.is * (vbe / bjt.vt).exp_m1();
    let reverse = bjt.is * (vbc / bjt.vt).exp_m1();
    let early = 1.0 - vbe / bjt.var - vbc / bjt.vaf;
    p * 2.0 * early * forward / (1.0 + (1.0 + 4.0 * (forward / bjt.ikf + reverse / bjt.ikr)).sqrt())
}

fn delay_history(bjt: &Bjt, anchor: Value) -> DelayBuffer {
    let delay = bjt.legacy_excess_phase_delay();
    let mut history = DelayBuffer::new(0);
    history
        .restore_checkpoint(&DelayCheckpoint {
            event_orders: Vec::new(),
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Fixed { delay }),
            samples: vec![(0.0, anchor), (2.0 * delay, anchor)],
        })
        .unwrap();
    history
}

#[test]
fn gp_phase_forward_transport_has_full_junction_gradient() {
    for p in [1.0, -1.0] {
        for scale in [1.0, 6.0] {
            let bjt = transistor(p, true, scale);
            for vbc in [-2.0, 0.2, 0.55] {
                let mut internal = [0.0; BJT_INTERNAL_STATE_DIM];
                internal[BJT_VBI_STATE_INDEX] = p * 0.68;
                internal[BJT_VCI_STATE_INDEX] = p * (0.68 - vbc);
                let branch = bjt.legacy_forward_transport_branch(&internal).unwrap();
                let expected = forward_reference(&bjt, p, &internal);
                assert!((branch.current - expected).abs() <= expected.abs() * 2e-14);
                assert_ne!(branch.d_internal[BJT_VCI_STATE_INDEX], 0.0);
                let h = 1e-7;
                for index in [
                    BJT_VBI_STATE_INDEX,
                    BJT_VCI_STATE_INDEX,
                    BJT_VEI_STATE_INDEX,
                ] {
                    let mut plus = internal;
                    let mut minus = internal;
                    plus[index] += h;
                    minus[index] -= h;
                    let expected = (forward_reference(&bjt, p, &plus)
                        - forward_reference(&bjt, p, &minus))
                        / (2.0 * h);
                    assert!(
                        (branch.d_internal[index] - expected).abs() <= expected.abs() * 2e-7,
                        "p={p} scale={scale} vbc={vbc} index={index}: {} != {expected}",
                        branch.d_internal[index]
                    );
                }
                assert!(branch.d_internal.iter().sum::<Value>().abs() < 1e-16);
                assert_eq!(branch.pos_internal, Some(BJT_VCX_STATE_INDEX));
                assert_eq!(branch.neg_internal, Some(BJT_VEI_STATE_INDEX));
            }
        }
    }
}

#[test]
fn gp_phase_companion_has_conservative_current_and_matrix_incidence() {
    for p in [1.0, -1.0] {
        for private in [false, true] {
            let bjt = transistor(p, private, 1.0);
            let snapshot = bjt.charge_snapshot(p * 2.0, p * 0.68, 0.0, 0.0);
            let internal = snapshot.reduction.internal_voltages;
            let forward = bjt.legacy_forward_transport_branch(&internal).unwrap();
            let anchor = p * 1e-5;
            let history = delay_history(&bjt, anchor);
            let time = 4.0 * bjt.legacy_excess_phase_delay();
            let q = snapshot.branches.map(|branch| branch.charge);
            let zero = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
            let coeff = CompanionCoefficients::backward_euler();
            let step = BjtChargeStep {
                coeff: &coeff,
                dt: time,
                q_prev: &q,
                q_prev_prev: &q,
                cq_prev: &zero,
                phase: None,
            };
            let base =
                Engine::assemble_legacy_bjt_transient_linearization(&bjt, &snapshot, step).unwrap();
            let phase = BjtPhaseTrial {
                history: (&history).into(),
                time,
                left_limit: None,
                incoming_arrival: false,
            };
            let changed = Engine::assemble_legacy_bjt_transient_linearization(
                &bjt,
                &snapshot,
                BjtChargeStep {
                    phase: Some(phase),
                    ..step
                },
            )
            .unwrap();
            let expected_current = 0.5 * (anchor - forward.current);
            let physical = phase.correction(&bjt, &internal).unwrap();
            assert!((physical.current - expected_current).abs() < 1e-18);
            for (sign, irow, erow) in [
                (1.0, forward.pos_internal, forward.pos_external),
                (-1.0, forward.neg_internal, forward.neg_external),
            ] {
                let (sign, before, after, source_before, source_after) = if let Some(row) = irow {
                    (
                        -sign,
                        base.g_ii[row],
                        changed.g_ii[row],
                        base.z_i[row],
                        changed.z_i[row],
                    )
                } else {
                    let row = erow.unwrap();
                    (
                        sign,
                        base.g_ei[row],
                        changed.g_ei[row],
                        base.z_e[row],
                        changed.z_e[row],
                    )
                };
                let mut current = -(source_after - source_before);
                for index in 0..BJT_INTERNAL_STATE_DIM {
                    let derivative = after[index] - before[index];
                    let expected = -0.5 * sign * forward.d_internal[index];
                    assert!((derivative - expected).abs() < 2e-14);
                    current += derivative * internal[index];
                }
                assert!(
                    (current - sign * expected_current).abs() < 2e-14,
                    "p={p} private={private}: {current} != {}",
                    sign * expected_current
                );
            }
        }
    }
}

#[test]
fn gp_phase_reduced_jacobian_matches_resolved_terminal_currents() {
    for p in [1.0, -1.0] {
        let bjt = transistor(p, true, 1.0);
        let external = [p * 2.0, p * 0.68, 0.0, 0.0];
        let previous = bjt.charge_snapshot(p * 2.0, p * 0.67, 0.0, 0.0);
        let forward = bjt
            .legacy_forward_transport_branch(&previous.reduction.internal_voltages)
            .unwrap();
        let history = delay_history(&bjt, forward.current);
        let accepted = history.checkpoint();
        let q = previous.branches.map(|branch| branch.charge);
        let zero = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let coeff = CompanionCoefficients::backward_euler();
        for (factor, left_limit) in [
            (2.5, None),
            (4.0, None),
            (2.5, Some(1.2 * forward.current)),
            (4.0, Some(1.2 * forward.current)),
        ] {
            let time = factor * bjt.legacy_excess_phase_delay();
            let step = BjtChargeStep {
                coeff: &coeff,
                dt: time - 2.0 * bjt.legacy_excess_phase_delay(),
                q_prev: &q,
                q_prev_prev: &q,
                cq_prev: &zero,
                phase: Some(BjtPhaseTrial {
                    history: (&history).into(),
                    time,
                    left_limit,
                    incoming_arrival: false,
                }),
            };
            let solve = |external| {
                Engine::resolve_legacy_bjt_transient_snapshot(
                    &bjt,
                    external,
                    step,
                    BjtPredictorHistory {
                        internal_prev: None,
                        linear_prev: None,
                        linear_prev_prev: None,
                        previous_dt: 0.0,
                    },
                    None,
                )
                .unwrap()
            };
            let snapshot = solve(external);
            let linearization =
                Engine::assemble_legacy_bjt_transient_linearization(&bjt, &snapshot, step).unwrap();
            let (jacobian, _) =
                Engine::reduce_bjt_transient_external_system(&linearization).unwrap();
            let currents =
                Engine::reduced_bjt_transient_terminal_currents(&bjt, &snapshot, step).unwrap();
            assert!(currents.iter().sum::<Value>().abs() < 1e-12);
            let h = 1e-6;
            for column in 0..3 {
                let mut plus = external;
                let mut minus = external;
                plus[column] += h;
                minus[column] -= h;
                let plus =
                    Engine::reduced_bjt_transient_terminal_currents(&bjt, &solve(plus), step)
                        .unwrap();
                let minus =
                    Engine::reduced_bjt_transient_terminal_currents(&bjt, &solve(minus), step)
                        .unwrap();
                for row in 0..4 {
                    let expected = (plus[row] - minus[row]) / (2.0 * h);
                    assert!(
                        (jacobian[row][column] - expected).abs() < 2e-9 + 5e-6 * expected.abs(),
                        "p={p} factor={factor} left={left_limit:?} row={row} column={column}: {} != {expected}",
                        jacobian[row][column]
                    );
                }
            }
            assert_eq!(history.checkpoint(), accepted);
            assert!(history.validate_checkpoint_ready().is_ok());
        }
    }
}
