use super::*;
use crate::device::Bjt;
use crate::engine::transient::bjt::BjtPhaseTrial;
use rspice_veriloga_runtime::transport_delay::{
    DelayBuffer, DelayCheckpoint, DelayConfiguration, DelayTimeSide,
};

fn transistor(polarity: Value) -> Bjt {
    let parameters = [
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
        ("RB", 100.0),
        ("RBM", 20.0),
        ("IRB", 1e-5),
        ("RC", 2.0),
        ("RE", 1.0),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_owned(), value))
    .collect();
    let mut bjt = if polarity > 0.0 {
        Bjt::new_npn("q".into(), 1, 2, 3)
    } else {
        Bjt::new_pnp("q".into(), 1, 2, 3)
    }
    .with_params(&parameters);
    bjt.set_junction_gmin(0.0);
    bjt
}

fn physical_sample(
    bjt: &mut Bjt,
    state: &[Value],
    options: &EventOptions,
    nodes: usize,
    phase: Option<BjtPhaseTrial<'_>>,
) -> Result<EventSample> {
    let mut sample = EventSample::new(state.len(), options)?;
    bjt.stamp_periodic_fq(state, &mut sample.f, &mut sample.q);
    // Periodic F/Q delegates RBI's linear KCL incidences to its canonical
    // port owner. Supply them once in this complete physical sampler.
    if let Some(column) = bjt.mna_rbi_branch_matrix_node(nodes) {
        for (node, sign) in [(bjt.node_bx, 1.0), (bjt.node_bi, -1.0)] {
            sample.f.stamp_rhs(node, -sign * state[column - 1]);
            sample.f.stamp(node, column, sign);
        }
    }
    if let Some(phase) = phase {
        let internal = bjt.mna_internal_state_at_solution(state);
        let correction = phase.correction(bjt, &internal).map_err(error)?;
        let terminals = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let rate = phase
            .history
            .fixed_trajectory_slope(
                phase.time,
                phase.left_limit.unwrap(),
                bjt.legacy_excess_phase_delay(),
                DelayTimeSide::Outgoing,
            )
            .map_err(error)?;
        for (internal, external, sign) in [
            (correction.pos_internal, correction.pos_external, 1.0),
            (correction.neg_internal, correction.neg_external, -1.0),
        ] {
            let node = internal
                .map(|index| bjt.mna_internal_node(index))
                .or_else(|| external.map(|index| terminals[index]))
                .unwrap_or(0);
            sample.f.stamp_rhs(node, -sign * correction.current);
            for (index, &derivative) in correction.d_internal.iter().enumerate() {
                sample
                    .f
                    .stamp(node, bjt.mna_internal_node(index), sign * derivative);
            }
            for (index, &derivative) in correction.d_external.iter().enumerate() {
                sample.f.stamp(node, terminals[index], sign * derivative);
            }
            if node != 0 {
                sample.f_time[node - 1] += sign * rate;
            }
        }
    }
    Ok(sample)
}

#[test]
fn charge_event_solves_native_coupled_bjt_charge_and_held_transport_history() {
    let options = options();
    for polarity in [1.0, -1.0] {
        let mut circuit = crate::CircuitData::new();
        for name in ["c", "b", "e"] {
            circuit.get_or_create_node(name);
        }
        let mut bjt = transistor(polarity);
        bjt.assign_mna_internal_nodes(|suffix| circuit.get_or_create_node(suffix));
        assert!(bjt.needs_mna_rbi_branch());
        bjt.assign_mna_rbi_branch(circuit.allocate_branch_named("irbi"));
        bjt.resolve_mna_rbi_branch(circuit.num_nodes());
        let nodes = circuit.num_nodes();
        let sources: Vec<_> = [(1, 2.0), (2, 0.6), (3, 0.0)]
            .into_iter()
            .map(|(positive, value)| {
                let branch = circuit.allocate_branch();
                circuit.voltage_sources.add(
                    format!("v{positive}"),
                    positive,
                    0,
                    branch,
                    polarity * value,
                );
                EventVoltageSource {
                    positive,
                    negative: 0,
                    branch: nodes + branch - 1,
                    value: polarity * value,
                    slope: 0.0,
                }
            })
            .collect();
        let size = circuit.matrix_size();
        let ports: Vec<_> = bjt.charge_storage_nodes().into_iter().flatten().collect();
        // The incoming endpoint comes from the existing DC analysis owner,
        // including its independently solved private nodes and RBI current.
        circuit.bjts.add(bjt);
        let mut engine = crate::Engine::default();
        let convergence = &mut engine.config.convergence_config;
        convergence.gmin_target = 0.0;
        convergence.junction_gmin_target = 0.0;
        convergence.voltage_reltol = options.relative_tolerance;
        convergence.voltage_abstol = options.voltage_tolerance;
        convergence.current_abstol = options.current_tolerance;
        convergence.residual_reltol = options.relative_tolerance;
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let netlist = crate::Netlist::parse("native event fixture\n.end\n").unwrap();
        let incoming = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .unwrap();
        let mut bjt = circuit.bjts.devices[0].clone();
        let incoming_sample = physical_sample(&mut bjt, &incoming, &options, nodes, None).unwrap();
        for row in 0..nodes {
            let source_current: Value = sources
                .iter()
                .filter(|source| source.positive == row + 1)
                .map(|source| incoming[source.branch])
                .sum();
            assert!(
                (incoming_sample.f.values[row] + source_current).abs() < 1e-11,
                "incoming DC KCL row {row}: F={:e}, I={source_current:e}, state={incoming:?}",
                incoming_sample.f.values[row]
            );
        }
        let forward = bjt
            .legacy_forward_transport_branch(&bjt.mna_internal_state_at_solution(&incoming))
            .unwrap()
            .current;
        let delay = bjt.legacy_excess_phase_delay();
        let mut history = DelayBuffer::new(0);
        history
            .restore_checkpoint(&DelayCheckpoint {
                configuration: Some(DelayConfiguration::Fixed { delay }),
                samples: vec![(0.0, forward)],
                left_limits: vec![],
            })
            .unwrap();
        let checkpoint = history.checkpoint();
        let phase = BjtPhaseTrial {
            history: &history,
            time: delay,
            left_limit: Some(forward),
            incoming_arrival: false,
        };
        let mut outgoing_sources = sources.clone();
        outgoing_sources[1].value = 0.64 * polarity;
        let topology = ChargeEventTopology::new(
            nodes,
            size,
            &ports,
            outgoing_sources.clone(),
            vec![options.voltage_tolerance; size - nodes],
            &options,
            &NoAbort,
        )
        .unwrap();
        let outgoing = topology
            .solve(
                &incoming,
                &incoming_sample.q.values,
                &options,
                &NoAbort,
                |state, _| physical_sample(&mut bjt, state, &options, nodes, Some(phase)),
            )
            .unwrap();
        close(outgoing.solution[1], 0.64 * polarity, 1e-12);
        assert_eq!(
            history.checkpoint(),
            checkpoint,
            "private event probes changed accepted delay history"
        );
        let sample =
            physical_sample(&mut bjt, &outgoing.solution, &options, nodes, Some(phase)).unwrap();
        for row in 0..nodes {
            let mut charge = sample.q.values[row] - incoming_sample.q.values[row];
            let mut current = sample.f.values[row] + sample.q_time[row];
            for (index, source) in outgoing_sources.iter().enumerate() {
                if source.positive == row + 1 {
                    charge += outgoing.source_impulses[index];
                    current += outgoing.solution[source.branch];
                }
            }
            for &(column, coefficient) in &sample.q.rows[row] {
                current += coefficient * outgoing.coordinate_rates[column].unwrap_or(0.0);
            }
            assert!(charge.abs() < 1e-22, "charge row {row}: {charge:e}");
            assert!(current.abs() < 1e-11, "current row {row}: {current:e}");
        }
        assert!(outgoing.solution.iter().all(|value| value.is_finite()));
    }
}
