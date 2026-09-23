//! A periodic boundary consumes the independently solved incoming orbit.
//! It does not use the DC/IC startup seed or divide an impulse by a timestep.
use super::*;

impl Engine {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn transition_pss_boundary(
        &self,
        circuit: &mut crate::CircuitData,
        history: &mut BjtTransientHistory,
        incoming_history: &BjtTransientHistory,
        incoming: &[Value],
        time: Value,
        dt: Value,
        solver: crate::solver::SolverOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let options = self.pss_physical_event_options(solver);
        let orders: Vec<_> = incoming_history
            .phase
            .iter()
            .map(|phase| phase.as_ref().map(|_| DelayEventOrder::Unknown))
            .collect();
        let point = self.prepare_physical_event(
            circuit,
            incoming_history,
            PhysicalEventStep {
                incoming,
                time,
                dt,
                phase_events: PhysicalEventOrders::Declared(&orders),
            },
            &options,
            self.config.transient_event_flux_abstol,
            abort,
        )?;
        let mut inductors = Vec::with_capacity(circuit.inductors.len());
        for (index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let voltage = sum([
                (
                    Self::node_voltage(&point.state.solution, circuit.inductors.node_pos[index]),
                    1.0,
                ),
                (
                    Self::node_voltage(&point.state.solution, circuit.inductors.node_neg[index]),
                    -1.0,
                ),
            ]
            .into_iter())?;
            inductors.push((
                point.state.solution[circuit.num_nodes() + ordinal - 1],
                voltage,
            ));
        }
        // Append the event to the history preceding the incoming interval.
        // The ordinary PSS acceptance may already have sampled its left limit.
        let mut outgoing_history = incoming_history.clone();
        Self::commit_bjt_history(&mut outgoing_history, point.bjt);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        for (line, prepared) in circuit.tlines.iter_mut().zip(point.lines) {
            line.commit_history_event(prepared.sample);
        }
        for (index, value) in point.capacitors.into_iter().enumerate() {
            circuit.capacitors.v_prev[index] = value.voltage;
            circuit.capacitors.i_prev[index] = value.current;
        }
        for (index, (current, voltage)) in inductors.into_iter().enumerate() {
            circuit.inductors.i_prev[index] = current;
            circuit.inductors.v_prev[index] = voltage;
        }
        circuit.update_coupled_inductor_pair_state(&point.state.solution);
        *history = outgoing_history;
        Self::restart_physical_event_history(circuit, history);
        Ok(point.state.solution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pss_boundary_transition_conserves_charge_and_solves_finite_line_rates() {
        let engine = Engine::default();
        let deck = Netlist::parse("periodic RC\nV1 s 0 PWL(0 0 0 1 .5 1 .5 0 1 0) R=0\nR1 s near 50\nC1 near 0 .01\nT1 near 0 far 0 Z0=50 TD=2\nR2 far 0 50\n.end\n").unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let near = circuit.get_node_by_name("near").unwrap() - 1;
        let source = circuit.get_node_by_name("s").unwrap() - 1;
        let mut incoming = vec![0.0; circuit.matrix_size()];
        incoming[near] = 0.25;
        circuit.tlines[0].update_history(0.0, 0.0, 0.0, 0.0, 0.0);
        circuit.tlines[0].update_history(1.0, 0.25, 0.005, 0.0, 0.0);
        let old =
            Engine::initialize_bjt_history(&circuit, &incoming, ReactiveHistorySeed::SolvedBias);
        let mut history = old.clone();
        let saved = circuit.tlines[0].checkpoint_state().unwrap();
        let mut bad = incoming.clone();
        bad[source] = 0.5;
        assert!(
            engine
                .transition_pss_boundary(
                    &mut circuit,
                    &mut history,
                    &old,
                    &bad,
                    1.0,
                    0.25,
                    Default::default(),
                    &NoAbort
                )
                .is_err()
        );
        assert_eq!(circuit.tlines[0].checkpoint_state().unwrap(), saved);
        let outgoing = engine
            .transition_pss_boundary(
                &mut circuit,
                &mut history,
                &old,
                &incoming,
                1.0,
                0.25,
                Default::default(),
                &NoAbort,
            )
            .unwrap();
        assert!((outgoing[source] - 1.0).abs() < 1e-14);
        assert!((outgoing[near] - 0.25).abs() < 1e-14);
        assert!((circuit.capacitors.v_prev[0] - 0.25).abs() < 1e-14);
        assert!((circuit.capacitors.i_prev[0] - 0.01).abs() < 1e-14);
        let event = circuit.tlines[0].checkpoint_state().unwrap().events[0];
        assert!((event[5] + 2.0).abs() < 1e-12);
        assert!((event[7] - 2.0).abs() < 1e-12);
    }
}
