//! Finite outgoing line rates at a converged periodic endpoint.
//! Audit the physical equations without moving the accepted coordinates.
use super::*;
use crate::circuit::SourceTimeSide;
use crate::device::TransmissionLineTimeSide;
use charge_event::circuit::{EventPhase, PreparedEventCircuit};

impl Engine {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn pss_endpoint_line_rates(
        &self,
        circuit: &crate::CircuitData,
        history: &BjtTransientHistory,
        solution: &[Value],
        time: Value,
        edges: &[Vec<[Value; 3]>],
        solver: crate::solver::SolverOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<[Value; 2]>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if edges.len() != circuit.tlines.len() || history.phase.len() != circuit.bjts.len() {
            return Err(SimulationError::Circuit(
                "PSS endpoint history population differs".into(),
            ));
        }
        self.ensure_result_values(circuit.tlines.iter().zip(edges).fold(
            circuit.matrix_size().saturating_mul(64),
            |total, (line, edges)| {
                total.saturating_add(
                    line.history_storage_values()
                        .saturating_mul(3)
                        .saturating_add(edges.len().saturating_mul(24)),
                )
            },
        ))?;
        // Earlier arrivals must already expose their true finite side rates
        // to the circuit equations. The live orbit remains untouched while
        // this private history conversion and its rate audit can still fail.
        let mut resolved = circuit.clone();
        for (line, edges) in resolved.tlines.iter_mut().zip(edges) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let end = edges.partition_point(|edge| edge[0] < time);
            line.promote_sampled_history_events(&edges[..end])
                .map_err(SimulationError::Circuit)?;
        }
        let options = charge_event::EventOptions {
            limits: self.config.resource_limits,
            solver,
            // Shooting's physical stamp includes authored RSHUNT itself and
            // does not add the adaptive transient conditioning conductance.
            nodal_gmin: 0.0,
            iterations: self.config.max_iterations,
            backtracks: 32,
            voltage_tolerance: self.voltage_abstol(),
            current_tolerance: self.current_abstol(),
            charge_tolerance: self.charge_abstol(),
            relative_tolerance: self.voltage_reltol(),
        };
        let mut sampler = PreparedEventCircuit::new(
            &resolved,
            self.config.transient_event_flux_abstol,
            &options,
            abort,
        )?;
        let inputs = sampler.forward_inputs(solution, abort)?;
        let phases = history
            .phase
            .iter()
            .zip(&inputs)
            .map(|(history, input)| match (history, input) {
                (Some(history), Some(endpoint)) => Ok(Some(EventPhase {
                    history,
                    endpoint: *endpoint,
                })),
                (None, None) => Ok(None),
                _ => Err(SimulationError::Circuit(
                    "PSS endpoint GP history differs from its model".into(),
                )),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let side = SourceTimeSide::RightLimit;
        let topology = sampler.topology(time, side, &options, abort)?;
        let mut trials = 0;
        let state = loop {
            if trials == options.iterations {
                return Err(SimulationError::Circuit(
                    "PSS endpoint charge directions did not settle".into(),
                ));
            }
            trials += 1;
            let charge = sampler
                .sample(time, side, solution, &phases, &options, abort)?
                .charge_values()
                .to_vec();
            let state =
                topology.solve_continuous(solution, &charge, &options, abort, |state, abort| {
                    sampler.sample(time, side, state, &phases, &options, abort)
                })?;
            if !sampler.select_outgoing_charge_limits(&state, abort)? {
                break state;
            }
        };
        let rate = |node: usize| -> Result<Value, SimulationError> {
            if node == 0 {
                return Ok(0.0);
            }
            state
                .coordinate_rates
                .get(node - 1)
                .copied()
                .flatten()
                .filter(|value| value.is_finite())
                .ok_or_else(|| {
                    SimulationError::Circuit(
                        "PSS endpoint line requires an unresolved coordinate rate".into(),
                    )
                })
        };
        let mut result = Vec::with_capacity(resolved.tlines.len());
        for line in &resolved.tlines {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut slopes = [0.0; 2];
            for (index, (p, n, forward)) in [
                (line.node1_pos, line.node1_neg, false),
                (line.node2_pos, line.node2_neg, true),
            ]
            .into_iter()
            .enumerate()
            {
                let incoming_rate = line
                    .lossless_wave_slope_on_side(time, forward, TransmissionLineTimeSide::Outgoing)
                    .map_err(SimulationError::Circuit)?;
                slopes[index] = rspice_veriloga_runtime::arithmetic::sum_products(
                    [(rate(p)?, 2.0), (rate(n)?, -2.0), (incoming_rate, -1.0)].into_iter(),
                )
                .map_err(|_| {
                    SimulationError::Circuit("PSS endpoint line rate is unrepresentable".into())
                })?;
            }
            result.push(slopes);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic_endpoint_line_rates_preserve_charge_and_audit_the_outgoing_limit() {
        let engine = Engine::default();
        let deck = Netlist::parse("endpoint RC\nV1 s 0 PWL(0 0 .5 0 .5 1 1 1 1 0) R=0\nR1 s near 50\nC1 near 0 .1\nT1 near 0 far 0 Z0=50 TD=2\nR2 far 0 50\n.end\n").unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let near = circuit.get_node_by_name("near").unwrap() - 1;
        let source = circuit.get_node_by_name("s").unwrap() - 1;
        let mut solution = vec![0.0; circuit.matrix_size()];
        solution[near] = 1.0;
        circuit.tlines[0].update_history(0.0, 0.0, 0.0, 0.0, 0.0);
        circuit.tlines[0].update_history(1.0, 1.0, 0.02, 0.0, 0.0);
        let saved = circuit.tlines[0].checkpoint_state().unwrap();
        let history =
            Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
        let rates = engine
            .pss_endpoint_line_rates(
                &circuit,
                &history,
                &solution,
                1.0,
                &[vec![]],
                Default::default(),
                &NoAbort,
            )
            .unwrap();
        // At the outgoing V1=0 side, C*dV/dt = -V/50 - V/50.
        // The launched wave has derivative 2*dV/dt = -0.8 V/s.
        assert!((rates[0][0] + 0.8).abs() < 1e-12);
        assert_eq!(rates[0][1], 0.0);
        assert_eq!(circuit.tlines[0].checkpoint_state().unwrap(), saved);
        solution[source] = 1.0; // The incoming source value is not an outgoing limit.
        assert!(
            engine
                .pss_endpoint_line_rates(
                    &circuit,
                    &history,
                    &solution,
                    1.0,
                    &[vec![]],
                    Default::default(),
                    &NoAbort,
                )
                .is_err()
        );
        assert_eq!(circuit.tlines[0].checkpoint_state().unwrap(), saved);
    }
}
