//! Core engine type and shared orchestration helpers.

use super::{
    SimulationConfig, SimulationConfigOverrides, SimulationError, resolve_simulation_config,
};
use crate::{Netlist, Value};
/// Main simulation engine
pub struct Engine {
    pub(crate) config: SimulationConfig,
}

impl Engine {
    /// Create a new engine with the given configuration
    pub fn new(config: SimulationConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the simulation configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Resolve effective simulation configuration for a specific netlist.
    ///
    /// Applies `.OPTIONS` on top of the engine's base configuration.
    pub(crate) fn resolved_for_netlist(&self, netlist: &Netlist) -> Self {
        let resolved = resolve_simulation_config(
            &self.config,
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        );
        Self::new(resolved)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(SimulationConfig::default())
    }
}

impl Engine {
    /// Collect node-voltage hints from .NODESET and .IC directives.
    ///
    /// .IC entries override .NODESET entries for the same node.
    pub(crate) fn collect_node_voltage_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        let mut by_node: Vec<Option<Value>> = vec![None; circuit.num_nodes() + 1];

        for nodeset in &netlist.node_sets {
            if !nodeset.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = circuit.get_node_by_name(&nodeset.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(nodeset.voltage);
            }
        }

        for ic in &netlist.initial_conditions {
            if !ic.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = circuit.get_node_by_name(&ic.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(ic.voltage);
            }
        }

        by_node
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(node_id, voltage)| voltage.map(|v| (node_id, v)))
            .collect()
    }

    /// Collect node-voltage initial conditions from .IC directives only.
    pub(crate) fn collect_initial_condition_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        netlist
            .initial_conditions
            .iter()
            .filter_map(|ic| {
                if !ic.voltage.is_finite() {
                    return None;
                }
                let node_id = circuit.get_node_by_name(&ic.node)?;
                if node_id == 0 || node_id > circuit.num_nodes() {
                    return None;
                }
                Some((node_id, ic.voltage))
            })
            .collect()
    }

    /// Apply .IC node overrides to an operating-point solution vector.
    ///
    /// Returns the number of nodes overridden.
    pub(crate) fn apply_initial_condition_overrides(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
        solution: &mut [Value],
    ) -> usize {
        let hints = self.collect_initial_condition_hints(netlist, circuit);
        let mut applied = 0usize;

        for (node_id, voltage) in hints {
            let idx = node_id - 1;
            if idx < solution.len() {
                solution[idx] = voltage;
                applied += 1;
            }
        }

        applied
    }

    /// Solve the DC operating point with optional node-voltage hints.
    pub(crate) fn solve_dc_operating_point(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_dc_operating_point_with_abort(
            netlist,
            circuit,
            matrix,
            &crate::abort_signal::NoAbort,
        )
    }

    /// Solve the DC operating point with optional node-voltage hints and abort support.
    pub(crate) fn solve_dc_operating_point_with_abort(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn crate::abort_signal::AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if circuit.has_nonlinear_devices() {
            let hints = self.collect_node_voltage_hints(netlist, circuit);
            if hints.is_empty() {
                self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &[], abort)
            } else {
                self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &hints, abort)
            }
        } else {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            self.solve_linear(circuit, matrix)
        }
    }
}
