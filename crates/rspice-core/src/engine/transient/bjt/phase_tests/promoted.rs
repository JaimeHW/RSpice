use super::*;
use crate::device::NonlinearDevice;

struct Sample {
    a: Vec<Vec<Value>>,
    residual: Vec<Value>,
}

fn promoted_circuit(bjt: &Bjt) -> crate::CircuitData {
    let mut circuit = crate::CircuitData::new();
    for name in ["c", "b", "e"] {
        circuit.get_or_create_node(name);
    }
    let mut device = bjt.clone();
    device.assign_mna_internal_nodes(|suffix| circuit.get_or_create_node(suffix));
    if device.needs_mna_rbi_branch() {
        device.assign_mna_rbi_branch(circuit.allocate_branch_named("irbi"));
    }
    device.resolve_mna_rbi_branch(circuit.num_nodes());
    circuit.bjts.add(device);
    circuit
}

fn initial_state(circuit: &crate::CircuitData, snapshot: &BjtChargeSnapshot) -> Vec<Value> {
    let mut solution = vec![0.0; circuit.matrix_size()];
    solution[..3].copy_from_slice(&snapshot.reduction.external_voltages[..3]);
    let device = &circuit.bjts.devices[0];
    for (index, value) in snapshot.reduction.internal_voltages.iter().enumerate() {
        let node = device.mna_internal_node(index);
        if node != 0 {
            solution[node - 1] = *value;
        }
    }
    if let Some(node) = device.mna_rbi_branch_matrix_node(circuit.num_nodes()) {
        solution[node - 1] = 1e-5;
    }
    solution
}

fn accepted_history(circuit: &crate::CircuitData, solution: &[Value]) -> BjtTransientHistory {
    let mut history =
        Engine::initialize_bjt_history(circuit, solution, ReactiveHistorySeed::SolvedBias);
    let device = &circuit.bjts.devices[0];
    let forward = device
        .legacy_forward_transport_branch(&history.dynamic_internal_prev[0])
        .unwrap()
        .current;
    history.phase[0] = Some(delay_history(device, forward));
    history
}

fn sample(
    circuit: &mut crate::CircuitData,
    solution: &[Value],
    history: &BjtTransientHistory,
    coeff: &CompanionCoefficients,
    time: Value,
    one_step: bool,
) -> Sample {
    circuit.bjts.devices[0].update_mna_static_probe(solution);
    sample_cached(circuit, solution, history, coeff, time, one_step)
}

fn sample_cached(
    circuit: &mut crate::CircuitData,
    solution: &[Value],
    history: &BjtTransientHistory,
    coeff: &CompanionCoefficients,
    time: Value,
    one_step: bool,
) -> Sample {
    sample_cached_on_side(
        circuit,
        solution,
        history,
        coeff,
        time,
        one_step,
        Default::default(),
    )
}

fn sample_cached_on_side(
    circuit: &mut crate::CircuitData,
    solution: &[Value],
    history: &BjtTransientHistory,
    coeff: &CompanionCoefficients,
    time: Value,
    one_step: bool,
    phase_context: BjtPhaseContext<'_>,
) -> Sample {
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    let dt = time - 2.0 * delay;
    let mut matrix = Engine::default().build_matrix(circuit).unwrap();
    circuit.link_indices(&matrix);
    let mut rhs = vec![0.0; solution.len()];
    circuit.bjts.devices[0].stamp_nonlinear(
        solution,
        &mut StaticMatrixChargeStamper {
            matrix: &mut matrix,
            rhs: &mut rhs,
        },
        &mut [],
    );
    if one_step {
        for value in matrix.values_mut() {
            *value *= 0.5;
        }
        for value in &mut rhs {
            *value *= 0.5;
        }
    }
    Engine::stamp_bjt_transient_companions(
        TransientCompanionStamp {
            circuit,
            matrix: &mut matrix,
            rhs: &mut rhs,
            voltages: solution,
            coeff,
            dt,
        },
        time,
        history,
        &mut [None],
        one_step,
        phase_context,
    )
    .unwrap();
    // This checks the real builder's frozen sparsity pattern as well as values.
    let residual = matrix.residual_vector(solution, &rhs).unwrap();
    let mut a = vec![vec![0.0; solution.len()]; solution.len()];
    let positions: Vec<_> = matrix.stored_positions().collect();
    for (row, column) in positions {
        let index = matrix.get_index(row, column).unwrap().offset();
        a[row][column] = matrix.values_mut()[index];
    }
    Sample { a, residual }
}

#[test]
fn gp_phase_promoted_companion_retains_the_limited_newton_anchor() {
    let coeff = CompanionCoefficients::backward_euler();
    for p in [1.0, -1.0] {
        for private in [false, true] {
            let mut device = transistor(p, private, 1.0);
            device.set_voltage_limiting_enabled(true);
            let previous = device.charge_snapshot(p * 2.0, p * 0.6, 0.0, 0.0);
            let mut circuit = promoted_circuit(&device);
            let anchor = initial_state(&circuit, &previous);
            let history = accepted_history(&circuit, &anchor);
            let before = history.clone();
            circuit.bjts.devices[0].update_mna_static_probe(&anchor);
            let mut raw = anchor.clone();
            raw[circuit.bjts.devices[0].node_bi - 1] += p * 1.0;
            circuit.bjts.devices[0].update(&raw);
            let (_, internal, external) = circuit.bjts.devices[0].mna_charge_state();
            let mut limited = raw.clone();
            limited[..3].copy_from_slice(&external[..3]);
            for (index, value) in internal.iter().enumerate() {
                let node = circuit.bjts.devices[0].mna_internal_node(index);
                if node != 0 {
                    limited[node - 1] = *value;
                }
            }
            assert_ne!(
                raw, limited,
                "fixture must actually engage junction limiting"
            );
            let time = 2.5 * device.legacy_excess_phase_delay();
            let actual = sample_cached(&mut circuit, &raw, &history, &coeff, time, false);
            let mut probe = promoted_circuit(&device);
            let expected = sample(&mut probe, &limited, &history, &coeff, time, false);
            for row in 0..raw.len() {
                let movement = raw
                    .iter()
                    .zip(&limited)
                    .zip(&expected.a[row])
                    .map(|((raw, limited), derivative)| (raw - limited) * derivative)
                    .sum::<Value>();
                let residual = expected.residual[row] + movement;
                assert!(
                    (actual.residual[row] - residual).abs() < 1e-11 + 1e-11 * residual.abs(),
                    "p={p} private={private} row={row}: {} != {residual}",
                    actual.residual[row]
                );
                for column in 0..raw.len() {
                    let a = actual.a[row][column];
                    let e = expected.a[row][column];
                    assert!((a - e).abs() < 1e-12 + 1e-12 * e.abs());
                }
            }
            assert_eq!(history, before);
        }
    }
}

#[test]
fn gp_phase_promoted_matrix_matches_every_physical_derivative() {
    let coeff = CompanionCoefficients::backward_euler();
    for p in [1.0, -1.0] {
        for private in [false, true] {
            for irb in [0.0, 1e-5] {
                let mut device = transistor(p, private, 1.0);
                device.irb = irb;
                let previous = device.charge_snapshot(p * 2.0, p * 0.67, 0.0, 0.0);
                let mut circuit = promoted_circuit(&device);
                let mut solution = initial_state(&circuit, &previous);
                let history = accepted_history(&circuit, &solution);
                let before = history.clone();
                solution[1] += p * 0.01;
                for factor in [2.5, 4.0] {
                    let time = factor * device.legacy_excess_phase_delay();
                    for one_step in [false, true] {
                        let base =
                            sample(&mut circuit, &solution, &history, &coeff, time, one_step);
                        for column in 0..solution.len() {
                            let h = if column >= circuit.num_nodes() {
                                1e-8
                            } else {
                                1e-6
                            };
                            let mut plus = solution.clone();
                            plus[column] += h;
                            let mut minus = solution.clone();
                            minus[column] -= h;
                            let plus =
                                sample(&mut circuit, &plus, &history, &coeff, time, one_step);
                            let minus =
                                sample(&mut circuit, &minus, &history, &coeff, time, one_step);
                            for row in 0..solution.len() {
                                let expected =
                                    (plus.residual[row] - minus.residual[row]) / (2.0 * h);
                                let actual = base.a[row][column];
                                let rounding = 64.0
                                    * Value::EPSILON
                                    * (plus.residual[row].abs() + minus.residual[row].abs())
                                    / h;
                                assert!(
                                    (actual - expected).abs()
                                        <= 2e-9
                                            + 2e-5 * expected.abs().max(actual.abs())
                                            + rounding,
                                    "p={p} private={private} IRB={irb} t/tau={factor} one_step={one_step} ({row},{column}): {actual:e} != {expected:e}"
                                );
                            }
                        }
                        assert_eq!(history, before);
                    }
                }
            }
        }
    }
}

fn internal_solutions<const C: usize>(
    a: &[Vec<Value>],
    rhs: impl Fn(usize, usize) -> Value,
) -> [[Value; C]; 16] {
    let count = a.len() - 3;
    let mut matrix = [[0.0; 16]; 16];
    let mut right = [[0.0; C]; 16];
    for row in 0..count {
        for column in 0..count {
            matrix[row][column] = a[row + 3][column + 3];
        }
        for (column, value) in right[row].iter_mut().enumerate() {
            *value = rhs(row + 3, column);
        }
    }
    crate::numerics::solve_small_dense_many(&matrix, &right, count).unwrap()
}

#[test]
fn gp_phase_promoted_newton_solution_matches_private_currents_and_reduced_jacobian() {
    for p in [1.0, -1.0] {
        for scale in [1.0, 6.0] {
            for private in [false, true] {
                let device = transistor(p, private, scale);
                let external = [p * 2.0, p * 0.68, 0.0, 0.0];
                let previous = device.charge_snapshot(p * 2.0, p * 0.67, 0.0, 0.0);
                let mut circuit = promoted_circuit(&device);
                let initial = initial_state(&circuit, &previous);
                let history = accepted_history(&circuit, &initial);
                for factor in [2.5, 4.0] {
                    for coeff in [
                        CompanionCoefficients::backward_euler(),
                        CompanionCoefficients::trapezoidal(),
                    ] {
                        let time = factor * device.legacy_excess_phase_delay();
                        let dt = time - 2.0 * device.legacy_excess_phase_delay();
                        let step = BjtChargeStep {
                            coeff: &coeff,
                            dt,
                            q_prev: &history.charge_q_prev[0],
                            q_prev_prev: &history.charge_q_prev_prev[0],
                            cq_prev: &history.charge_cq_prev[0],
                            phase: history.phase_trial(0, time),
                        };
                        let snapshot = Engine::resolve_legacy_bjt_transient_snapshot(
                            &device,
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
                        .unwrap();
                        let linearization = Engine::assemble_legacy_bjt_transient_linearization(
                            &device, &snapshot, step,
                        )
                        .unwrap();
                        let (private_jacobian, _) =
                            Engine::reduce_bjt_transient_external_system(&linearization).unwrap();
                        let private_current = Engine::reduced_bjt_transient_terminal_currents(
                            &device, &snapshot, step,
                        )
                        .unwrap();
                        let mut solution = initial.clone();
                        solution[..3].copy_from_slice(&external[..3]);
                        let mut converged = solution.len() == 3;
                        for _ in 0..30 {
                            if solution.len() == 3 {
                                break;
                            }
                            let system =
                                sample(&mut circuit, &solution, &history, &coeff, time, false);
                            let change =
                                internal_solutions::<1>(&system.a, |row, _| -system.residual[row]);
                            let mut largest = 0.0_f64;
                            for (index, value) in solution[3..].iter_mut().enumerate() {
                                *value += change[index][0];
                                largest = largest.max(change[index][0].abs());
                            }
                            if largest < 1e-13 {
                                converged = true;
                                break;
                            }
                        }
                        assert!(
                            converged,
                            "p={p} M={scale} private={private} t/tau={factor}"
                        );
                        let system = sample(&mut circuit, &solution, &history, &coeff, time, false);
                        let movement = internal_solutions::<3>(&system.a, |row, column| {
                            -system.a[row][column]
                        });
                        for (row, private_row) in private_jacobian.iter().enumerate().take(3) {
                            for (column, expected) in private_row.iter().enumerate().take(3) {
                                let actual = system.a[row][column]
                                    + system.a[row][3..]
                                        .iter()
                                        .enumerate()
                                        .map(|(index, value)| value * movement[index][column])
                                        .sum::<Value>();
                                assert!(
                                    (actual - expected).abs() < 2e-9 + 5e-6 * expected.abs(),
                                    "p={p} M={scale} private={private} t/tau={factor} ({row},{column}): {actual:e} != {expected:e}"
                                );
                            }
                        }
                        let mut accepted = history.clone();
                        Engine::accept_bjt_history(
                            &circuit,
                            &mut accepted,
                            &solution,
                            &coeff,
                            dt,
                            time,
                            None,
                        )
                        .unwrap();
                        let reported = accepted.accepted_terminal_currents[0].unwrap();
                        for row in 0..4 {
                            assert!(
                                (reported[row] - private_current[row]).abs()
                                    < 2e-12 + 2e-7 * private_current[row].abs(),
                                "p={p} M={scale} private={private} t/tau={factor} lead {row}: {} != {}",
                                reported[row],
                                private_current[row]
                            );
                        }
                        assert!(reported.iter().sum::<Value>().abs() < 1e-11);
                    }
                }
            }
        }
    }
}

#[test]
fn gp_phase_promoted_one_step_includes_previous_physical_phase_current() {
    let coeff = CompanionCoefficients::backward_euler();
    for p in [1.0, -1.0] {
        let device = transistor(p, false, 1.0);
        let mut circuit = promoted_circuit(&device);
        let previous = device.charge_snapshot(p * 2.0, p * 0.67, 0.0, 0.0);
        let mut solution = initial_state(&circuit, &previous);
        let mut history = accepted_history(&circuit, &solution);
        let delay = device.legacy_excess_phase_delay();
        let forward = history.phase[0]
            .as_ref()
            .unwrap()
            .accepted_samples()
            .next_back()
            .unwrap()
            .1;
        history.phase[0] = Some(
            DelayBuffer::from_checkpoint(DelayCheckpoint {
                event_orders: Vec::new(),
                left_limits: Vec::new(),
                configuration: Some(DelayConfiguration::Fixed { delay }),
                samples: vec![(0.0, 0.8 * forward), (2.0 * delay, forward)],
            })
            .unwrap(),
        );
        let mut without_phase = history.clone();
        without_phase.phase[0] = None;
        solution[1] += p * 0.01;
        let (_, internal, _) = circuit.bjts.devices[0].mna_charge_state_at_solution(&solution);
        let now = forward_reference(&device, p, &internal);
        for (factor, correction) in [
            (2.5, 0.95 * forward - now),
            (4.0, 0.5 * forward - 0.5 * now),
        ] {
            let enabled = sample(
                &mut circuit,
                &solution,
                &history,
                &coeff,
                factor * delay,
                true,
            );
            let disabled = sample(
                &mut circuit,
                &solution,
                &without_phase,
                &coeff,
                factor * delay,
                true,
            );
            let expected = 0.5 * correction - 0.05 * forward;
            for (row, sign) in [(0, 1.0), (1, 0.0), (2, -1.0)] {
                let actual = enabled.residual[row] - disabled.residual[row];
                assert!(
                    (actual - sign * expected).abs() < 1e-14 + 1e-11 * expected.abs(),
                    "p={p} t/tau={factor} row={row}: {actual:e} != {:e}",
                    sign * expected
                );
            }
        }
    }
}

#[test]
fn gp_right_trial_promoted_stamp_matches_physical_current_and_tangent() {
    for polarity in [1.0, -1.0] {
        for private in [false, true] {
            let device = transistor(polarity, private, 1.0);
            let incoming = device.charge_snapshot(polarity * 2.0, polarity * 0.67, 0.0, 0.0);
            let left = forward_reference(&device, polarity, &incoming.reduction.internal_voltages);
            let delay = device.legacy_excess_phase_delay();
            let mut history = DelayBuffer::new(0);
            history
                .accept_sample(0.0, 0.25 * left, delay, None)
                .unwrap();
            let accepted = history.clone();
            let phase = BjtPhaseTrial {
                history: &history,
                time: 2.5 * delay,
                left_limit: Some(left),
                incoming_arrival: false,
            };
            let mut circuit = promoted_circuit(&device);
            let right = device.charge_snapshot(polarity * 2.0, polarity * 0.69, 0.0, 0.0);
            let solution = initial_state(&circuit, &right);
            let collector = circuit.bjts.devices[0].mna_internal_node(BJT_VCX_STATE_INDEX) - 1;
            let emitter = circuit.bjts.devices[0].mna_internal_node(BJT_VEI_STATE_INDEX) - 1;
            for one_step in [false, true] {
                let weight = if one_step { 0.5 } else { 1.0 };
                let (_, internal, _) =
                    circuit.bjts.devices[0].mna_charge_state_at_solution(&solution);
                let expected =
                    weight * (0.7 * left - forward_reference(&device, polarity, &internal));
                let mut sample = |values: &[Value]| {
                    let mut matrix = Engine::default().build_matrix(&circuit).unwrap();
                    circuit.link_indices(&matrix);
                    // The builder seeds a diagonal GMIN. This fixture reads
                    // the phase stamp alone, so retain its pattern but clear
                    // the unrelated baseline conductance before loading it.
                    matrix.values_mut().fill(0.0);
                    circuit.bjts.devices[0].update_mna_static_probe(values);
                    let mut rhs = vec![0.0; values.len()];
                    phase
                        .stamp_promoted(
                            &circuit.bjts.devices[0],
                            &mut StaticMatrixChargeStamper {
                                matrix: &mut matrix,
                                rhs: &mut rhs,
                            },
                            one_step,
                        )
                        .unwrap();
                    let residual = matrix.residual_vector(values, &rhs).unwrap();
                    let mut a = vec![vec![0.0; values.len()]; values.len()];
                    let positions: Vec<_> = matrix.stored_positions().collect();
                    for (row, column) in positions {
                        let offset = matrix.get_index(row, column).unwrap().offset();
                        a[row][column] = matrix.values_mut()[offset];
                    }
                    Sample { a, residual }
                };
                let actual = sample(&solution);
                for (row, sign) in [(collector, 1.0), (emitter, -1.0)] {
                    assert!(
                        (actual.residual[row] - sign * expected).abs() < 1e-14,
                        "polarity={polarity} private={private} one_step={one_step} row={row}: {} != {}",
                        actual.residual[row],
                        sign * expected
                    );
                    for column in 0..solution.len() {
                        let h = 1e-7;
                        let mut plus = solution.clone();
                        let mut minus = solution.clone();
                        plus[column] += h;
                        minus[column] -= h;
                        let tangent = (sample(&plus).residual[row] - sample(&minus).residual[row])
                            / (2.0 * h);
                        assert!(
                            (actual.a[row][column] - tangent).abs() < 2e-12 + 2e-7 * tangent.abs(),
                            "polarity={polarity} private={private} one_step={one_step} row={row} column={column}"
                        );
                    }
                }
            }
            assert_eq!(history, accepted);
        }
    }
}

#[test]
fn gp_phase_event_context_reaches_complete_promoted_companion_assembly() {
    let coeff = CompanionCoefficients::backward_euler();
    for polarity in [1.0, -1.0] {
        for private in [false, true] {
            let device = transistor(polarity, private, 1.0);
            let left_state = device.charge_snapshot(polarity * 2.0, polarity * 0.67, 0.0, 0.0);
            let mut circuit = promoted_circuit(&device);
            let initial = initial_state(&circuit, &left_state);
            let mut history = accepted_history(&circuit, &initial);
            let anchor = history.phase[0]
                .as_ref()
                .unwrap()
                .accepted_samples()
                .next_back()
                .unwrap()
                .1;
            let delay = device.legacy_excess_phase_delay();
            let mut buffer = DelayBuffer::new(0);
            buffer.accept_sample(0.0, anchor, delay, None).unwrap();
            buffer
                .accept_discontinuity(2.0 * delay, anchor, 3.0 * anchor, delay, None)
                .unwrap();
            let arrival = buffer
                .next_discontinuity_after(2.0 * delay)
                .unwrap()
                .unwrap();
            history.phase[0] = Some(buffer);
            let accepted = history.clone();
            let right = device.charge_snapshot(polarity * 2.0, polarity * 0.69, 0.0, 0.0);
            let solution = initial_state(&circuit, &right);
            circuit.bjts.devices[0].update_mna_static_probe(&solution);
            let (_, internal, _) = circuit.bjts.devices[0].mna_charge_state_at_solution(&solution);
            let current = forward_reference(&device, polarity, &internal);
            let collector = circuit.bjts.devices[0].mna_internal_node(BJT_VCX_STATE_INDEX) - 1;
            let emitter = circuit.bjts.devices[0].mna_internal_node(BJT_VEI_STATE_INDEX) - 1;
            let limits = [Some(anchor)];
            for one_step in [false, true] {
                let weight = if one_step { 0.5 } else { 1.0 };
                for (time, context, difference) in [
                    (
                        arrival,
                        BjtPhaseContext {
                            incoming_arrival: true,
                            input_left_limits: None,
                        },
                        -2.0 * anchor,
                    ),
                    (
                        4.5 * delay,
                        BjtPhaseContext {
                            incoming_arrival: false,
                            input_left_limits: Some(&limits),
                        },
                        0.6 * (anchor - current),
                    ),
                ] {
                    let ordinary = sample_cached_on_side(
                        &mut circuit,
                        &solution,
                        &history,
                        &coeff,
                        time,
                        one_step,
                        Default::default(),
                    );
                    let sided = sample_cached_on_side(
                        &mut circuit,
                        &solution,
                        &history,
                        &coeff,
                        time,
                        one_step,
                        context,
                    );
                    for row in 0..solution.len() {
                        let sign = if row == collector {
                            1.0
                        } else if row == emitter {
                            -1.0
                        } else {
                            0.0
                        };
                        let actual = sided.residual[row] - ordinary.residual[row];
                        let expected = sign * weight * difference;
                        assert!(
                            (actual - expected).abs() < 1e-13,
                            "polarity={polarity} private={private} order2={one_step} row={row}: {actual} != {expected}"
                        );
                    }
                }
            }
            assert_eq!(history, accepted);
        }
    }
}
