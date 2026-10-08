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
                integration_coefficients: None,
                incoming,
                time,
                dt,
                phase_events: PhysicalEventOrders::Declared(&orders),
            },
            &options,
            self.config.transient_event_flux_abstol,
            abort,
        )?;
        // Append the event to the history preceding the incoming interval.
        // The ordinary PSS acceptance may already have sampled its left limit.
        self.ensure_transport_history_copy(incoming_history, history.transport_allocated_bytes())?;
        let retained_transport_bytes = history
            .transport_allocated_bytes()
            .saturating_add(incoming_history.transport_allocated_bytes());
        let mut outgoing_history = incoming_history.try_clone()?;
        point.bjt.reserve_phase_storage(
            &mut outgoing_history,
            retained_transport_bytes,
            &self.config.resource_limits,
        )?;
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
        for (index, winding) in point.windings.into_iter().enumerate() {
            circuit.inductors.i_prev[index] = winding.current;
            circuit.inductors.v_prev[index] = winding.voltage;
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
    fn pss_boundary_commits_coupled_winding_currents_and_finite_voltages() {
        let engine = Engine::default();
        let deck = Netlist::parse(
            "magnetic boundary\nV1 s1 0 DC .4 PWL(0 .4 1 .4 1 1)\nV2 s2 0 DC -.3 PWL(0 -.3 1 -.3 1 -.5)\nR1 s1 a 2\nR2 s2 b 3\nL1 a 0 .5\nL2 b 0 .25\nK1 L1 L2 .4\n.end\n",
        ).unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let incoming = engine
            .solve_dc_operating_point(&deck, &mut circuit, &mut matrix)
            .unwrap();
        for (index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
            circuit.inductors.i_prev[index] = incoming[circuit.num_nodes() + ordinal - 1];
        }
        circuit.reset_coupled_inductor_pair_state(&incoming);
        let old =
            Engine::initialize_bjt_history(&circuit, &incoming, ReactiveHistorySeed::SolvedBias)
                .unwrap();
        let mut history = old.clone();
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
        // Finite applied voltage preserves both independent flux linkages.
        // The two resistors then set the outgoing finite winding voltages.
        for (index, (current, voltage)) in [(0.2, 0.6), (-0.1, -0.2)].into_iter().enumerate() {
            let branch = circuit.num_nodes() + circuit.inductors.branch_indices[index] - 1;
            assert!((outgoing[branch] - current).abs() < 1e-12);
            assert!((circuit.inductors.i_prev[index] - current).abs() < 1e-12);
            assert!((circuit.inductors.v_prev[index] - voltage).abs() < 1e-12);
        }
        assert_eq!(circuit.inductors.i_prev, circuit.inductors.i_prev_prev);
        assert_eq!(circuit.inductors.i_prev, circuit.inductors.i_prev_prev_prev);
    }

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
            Engine::initialize_bjt_history(&circuit, &incoming, ReactiveHistorySeed::SolvedBias)
                .unwrap();
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
