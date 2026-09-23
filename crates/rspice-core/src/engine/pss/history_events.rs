//! Declared delay edges remain owned during the shooting map, not only export.
use super::*;

impl Engine {
    pub(super) fn pss_promote_line_history_events(
        &self,
        circuit: &mut PssCircuit,
        solution: &[Value],
        time: Value,
        solver: crate::solver::SolverOptions,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let mut pending_line_edges = Vec::with_capacity(circuit.tlines.len());
        let mut edge_words = circuit.tlines.len().saturating_mul(3);
        self.ensure_result_values(edge_words)?;
        for line in &circuit.tlines {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut edges = if !line.is_memoryless_two_port()
                && let Some(mesh) = &circuit.integration_mesh
            {
                let mut limits = self.config.resource_limits;
                limits.max_result_values = limits.max_result_values.saturating_sub(edge_words);
                mesh.pending_sampled_edges(line.delay(), time, limits, abort)?
            } else {
                Vec::new()
            };
            edge_words = edge_words.saturating_add(edges.capacity().saturating_mul(3));
            self.ensure_result_values(edge_words)?;
            edges.retain(|edge| !line.owns_history_event(edge[0]));
            pending_line_edges.push(edges);
        }
        let endpoint_event = |edges: &Vec<[Value; 3]>| {
            edges
                .last()
                .is_some_and(|edge| edge[0] == time && edge[2] == time)
        };
        let endpoint_rates = if pending_line_edges.iter().any(endpoint_event) {
            Some(self.pss_endpoint_line_rates(
                &circuit.circuit,
                &circuit.bjt_history,
                solution,
                time,
                &pending_line_edges,
                solver,
                abort,
            )?)
        } else {
            None
        };
        for (index, (line, edges)) in circuit
            .circuit
            .tlines
            .iter_mut()
            .zip(&pending_line_edges)
            .enumerate()
        {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !line.is_memoryless_two_port() {
                // The replacement checkpoint and the accepted history coexist
                // during validation; bound their aggregate scratch first.
                if !edges.is_empty() {
                    self.ensure_result_values(
                        line.history_storage_values()
                            .saturating_mul(3)
                            .saturating_add(edges.len().saturating_mul(24)),
                    )?;
                    let rates = endpoint_event(edges)
                        .then(|| endpoint_rates.as_ref().expect("solved endpoint rates")[index]);
                    line.promote_sampled_history_events_with_endpoint_rates(edges, rates)
                        .map_err(SimulationError::Circuit)?;
                }
            }
        }
        Ok(())
    }
}
