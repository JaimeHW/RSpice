//! Direct physical `Q/F/B` loading for the narrowly proven Xyce Core path.
//!
//! The matrix remains the Newton Jacobian.  This module supplies the physical
//! DAE vectors that Xyce's OneStep residual consumes; reconstructing them from
//! `A*x-b` is not equivalent at the Core's cancellation-sensitive branch row.
//! The capability is deliberately fail-closed until every device family in a
//! circuit has a source-faithful direct loader.

use super::CircuitData;
use super::dae::XyceDaeVectors;
use super::xyce_load::XyceDeviceRef;
use crate::Value;

#[inline]
fn solution_node_value(solution: &[Value], node: usize) -> Option<Value> {
    if node == 0 {
        Some(0.0)
    } else {
        solution
            .get(node - 1)
            .copied()
            .filter(|value| value.is_finite())
    }
}

#[inline]
fn validate_vector(name: &str, values: &[Value]) -> Result<(), String> {
    if let Some((index, &value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(format!(
            "direct Xyce DAE {name}[{index}] is non-finite: {value}"
        ));
    }
    Ok(())
}

impl CircuitData {
    /// Whether this circuit is covered by the direct, source-faithful loader.
    ///
    /// This predicate intentionally names every unsupported family instead of
    /// relying on an incidental empty plan.  A future family must be added to
    /// this proof boundary before it can use the direct residual path.
    pub(crate) fn supports_direct_xyce_level2_core_dae(&self) -> bool {
        if self.jiles_atherton_inductors.len() != 1
            || self.inductors.len() != 1
            || self.num_branches != 1
            || !self.xyce_core_groups.is_empty()
            || self.hidden_state_count != 0
            || !self.non_electrical_state_nodes.is_empty()
            || !self.resistor_branches.is_empty()
            || !self.capacitors.is_empty()
            || !self.voltage_sources.is_empty()
            || !self.inductors.ic.iter().all(Option::is_none)
            || !self.resistors.thermal.iter().all(Option::is_none)
        {
            return false;
        }

        let binding = &self.jiles_atherton_inductors[0];
        let Some(core_output_name) = binding.core_output_name.as_deref() else {
            return false;
        };
        if core_output_name.is_empty()
            || binding.inductor_index != 0
            || binding.branch_ordinal == 0
            || self.inductors.branch_indices.first().copied() != Some(binding.branch_ordinal)
            || !binding.device.is_xyce_core_level2()
        {
            return false;
        }

        if self.xyce_load_plan.cores() != [0]
            || !self.xyce_load_plan.core_groups().is_empty()
            || self.xyce_load_plan.ordered_devices().iter().any(|device| {
                !matches!(
                    device,
                    XyceDeviceRef::Resistor(_)
                        | XyceDeviceRef::CurrentSource(_)
                        | XyceDeviceRef::Core(0)
                )
            })
        {
            return false;
        }

        let planned_resistors = self
            .xyce_load_plan
            .ordered_devices()
            .iter()
            .filter_map(|device| match device {
                XyceDeviceRef::Resistor(index) => Some(*index),
                _ => None,
            })
            .collect::<Vec<_>>();
        let planned_sources = self.xyce_load_plan.current_sources();
        if planned_resistors.len() != self.resistors.len()
            || planned_resistors
                .iter()
                .enumerate()
                .any(|(position, &index)| {
                    index >= self.resistors.len() || planned_resistors[..position].contains(&index)
                })
            || planned_sources.len() != self.current_sources.names.len()
            || planned_sources
                .iter()
                .enumerate()
                .any(|(position, &index)| {
                    index >= self.current_sources.names.len()
                        || planned_sources[..position].contains(&index)
                })
        {
            return false;
        }

        if !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.b3soi.is_empty()
            || !self.b3soi_fd.is_empty()
            || !self.b3soi_pd.is_empty()
            || !self.bsim3v3.is_empty()
            || !self.bsim4v8.is_empty()
            || !self.ekv26s.is_empty()
            || !self.ekv3s.is_empty()
            || !self.vdmoses.is_empty()
            || !self.jfets.is_empty()
            || !self.xyce_memristors.is_empty()
            || !self.vcvs.is_empty()
            || !self.vccs.is_empty()
            || !self.cccs.is_empty()
            || !self.ccvs.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.generic_switches.is_empty()
            || !self.tlines.is_empty()
            || !self.coupled_tlines.is_empty()
            || !self.couplings.is_empty()
            || !self.coupled_inductor_pairs.is_empty()
            || !self.multi_winding_transformers.is_empty()
            || !self.behavioral_sources.is_empty()
            || !self.xspice_instances.is_empty()
            || !self.pending_cccs.is_empty()
            || !self.pending_ccvs.is_empty()
            || !self.pending_iswitch.is_empty()
        {
            return false;
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_devices.is_empty() {
            return false;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if !self.generated_veriloga_devices.is_empty() {
            return false;
        }

        true
    }

    /// Initialize the accepted physical Core charge vector for a new run.
    pub(crate) fn initialize_direct_xyce_accepted_q(
        &self,
        accepted_q: &mut Vec<Value>,
    ) -> Result<(), String> {
        if !self.supports_direct_xyce_level2_core_dae() {
            return Err("direct Xyce Level-2 Core DAE is unsupported for this circuit".into());
        }
        let binding = &self.jiles_atherton_inductors[0];
        let branch_ordinal = binding.branch_ordinal;
        let branch_row = self
            .num_nodes
            .checked_add(branch_ordinal)
            .and_then(|row| row.checked_sub(1))
            .filter(|&row| row < self.matrix_size())
            .ok_or_else(|| "direct Xyce Core branch row is outside the matrix".to_owned())?;
        let history = binding.device.xyce_core_q_history();
        if !history.is_finite() {
            return Err(format!(
                "direct Xyce Core accepted Q is non-finite: {history}"
            ));
        }
        accepted_q.resize(self.matrix_size(), 0.0);
        accepted_q.fill(0.0);
        accepted_q[branch_row] = history;
        Ok(())
    }

    /// Load the physical direct DAE vectors at the exact cached Newton trial.
    ///
    /// The caller must invoke the canonical Core transient stamp first.  This
    /// method only reads the matching cached endpoint and never advances or
    /// mutates magnetic state.
    pub(crate) fn load_direct_xyce_level2_core_dae(
        &self,
        solution: &[Value],
        time: Value,
        nodal_gmin: Value,
        vectors: &mut XyceDaeVectors,
    ) -> Result<(), String> {
        if !self.supports_direct_xyce_level2_core_dae() {
            return Err("direct Xyce Level-2 Core DAE is unsupported for this circuit".into());
        }
        let dimension = self.matrix_size();
        if solution.len() != dimension {
            return Err(format!(
                "direct Xyce DAE solution has length {}, expected {dimension}",
                solution.len()
            ));
        }
        if !time.is_finite() {
            return Err(format!("direct Xyce DAE time is non-finite: {time}"));
        }
        if !nodal_gmin.is_finite() {
            return Err(format!(
                "direct Xyce DAE nodal GMIN is non-finite: {nodal_gmin}"
            ));
        }

        vectors.resize_and_clear(dimension);
        let (q, f, b) = vectors.q_f_b_mut();

        for &core_index in self.xyce_load_plan.cores() {
            let binding = self
                .jiles_atherton_inductors
                .get(core_index)
                .ok_or_else(|| format!("direct Xyce Core index {core_index} is missing"))?;
            let inductor_index = binding.inductor_index;
            let branch_ordinal = self
                .inductors
                .branch_indices
                .get(inductor_index)
                .copied()
                .filter(|&ordinal| ordinal > 0)
                .ok_or_else(|| {
                    format!("direct Xyce Core inductor {inductor_index} has no branch")
                })?;
            let branch_row = self
                .num_nodes
                .checked_add(branch_ordinal)
                .and_then(|row| row.checked_sub(1))
                .filter(|&row| row < dimension)
                .ok_or_else(|| "direct Xyce Core branch row is outside the matrix".to_owned())?;
            let node_pos = *self.inductors.node_pos.get(inductor_index).ok_or_else(|| {
                format!("direct Xyce Core inductor {inductor_index} has no + node")
            })?;
            let node_neg = *self.inductors.node_neg.get(inductor_index).ok_or_else(|| {
                format!("direct Xyce Core inductor {inductor_index} has no - node")
            })?;
            if node_pos > self.num_nodes || node_neg > self.num_nodes {
                return Err(format!(
                    "direct Xyce Core nodes ({node_pos}, {node_neg}) exceed {} electrical nodes",
                    self.num_nodes
                ));
            }
            let voltage_pos = solution_node_value(solution, node_pos).ok_or_else(|| {
                format!("direct Xyce Core positive node {node_pos} is missing or non-finite")
            })?;
            let voltage_neg = solution_node_value(solution, node_neg).ok_or_else(|| {
                format!("direct Xyce Core negative node {node_neg} is missing or non-finite")
            })?;
            let voltage = voltage_pos - voltage_neg;
            let current = solution[branch_row];
            if !current.is_finite() || !voltage.is_finite() {
                return Err("direct Xyce Core trial endpoint is non-finite".into());
            }
            let endpoint = binding
                .device
                .xyce_core_cached_dae_endpoint(current, voltage)
                .ok_or_else(|| {
                    format!(
                        "direct Xyce Core trial is missing or stale at current {current}, voltage {voltage}"
                    )
                })?;

            q[branch_row] += endpoint.q;
            if node_pos > 0 {
                f[node_pos - 1] += current;
            }
            if node_neg > 0 {
                f[node_neg - 1] -= current;
            }
            f[branch_row] += endpoint.f;
        }

        self.xyce_linear_f_operator.add_product(solution, f)?;
        for node_row in 0..self.num_nodes {
            if !self.is_non_electrical_state_matrix_index(node_row) {
                f[node_row] += (self.global_shunt_conductance + nodal_gmin) * solution[node_row];
            }
        }

        for &source_index in self.xyce_load_plan.current_sources() {
            let source = self.current_sources.value_at_time(source_index, time);
            if !source.is_finite() {
                return Err(format!(
                    "direct Xyce current source {source_index} is non-finite at time {time}"
                ));
            }
            let node_pos = *self
                .current_sources
                .node_pos
                .get(source_index)
                .ok_or_else(|| {
                    format!("direct Xyce current source {source_index} has no + node")
                })?;
            let node_neg = *self
                .current_sources
                .node_neg
                .get(source_index)
                .ok_or_else(|| {
                    format!("direct Xyce current source {source_index} has no - node")
                })?;
            if node_pos > self.num_nodes || node_neg > self.num_nodes {
                return Err(format!(
                    "direct Xyce current source nodes ({node_pos}, {node_neg}) exceed {} electrical nodes",
                    self.num_nodes
                ));
            }
            if node_pos > 0 {
                b[node_pos - 1] -= source;
            }
            if node_neg > 0 {
                b[node_neg - 1] += source;
            }
        }

        validate_vector("Q", q)?;
        validate_vector("F", f)?;
        validate_vector("B", b)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, Netlist, SimulationConfig, SpiceDialect};

    const BH_LEVEL2_DECK: &str = "Xyce direct Core DAE capability\n\
        .tran 0 4\n\
        R1 1 0 1\n\
        L1 1 0 20\n\
        I1 1 0 SIN(0 .1 1 1)\n\
        I2 1 0 SIN(0 .2 1 2)\n\
        I3 1 0 SIN(0 .8 1 3)\n\
        K1 L1 1 CORE_MODEL\n\
        .model CORE_MODEL CORE (LEVEL=2 MS=510K A=62 C=.92 K=25 ALPHA=3.7e-4 AREA=1.12 GAP=0 PATH=8.49)\n\
        .end\n";

    #[test]
    fn direct_level2_core_capability_is_fail_closed_and_requires_cached_trial() {
        assert!(!CircuitData::new().supports_direct_xyce_level2_core_dae());

        let netlist = Netlist::parse(BH_LEVEL2_DECK).expect("Core fixture parses");
        let circuit =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
                .build_circuit(&netlist)
                .expect("Core fixture builds");
        assert!(circuit.supports_direct_xyce_level2_core_dae());

        let mut accepted_q = Vec::new();
        circuit
            .initialize_direct_xyce_accepted_q(&mut accepted_q)
            .expect("accepted Q history initializes");
        assert_eq!(accepted_q.len(), circuit.matrix_size());

        let mut vectors = XyceDaeVectors::new(circuit.matrix_size());
        let error = circuit
            .load_direct_xyce_level2_core_dae(
                &vec![0.0; circuit.matrix_size()],
                0.0,
                0.0,
                &mut vectors,
            )
            .expect_err("a direct load must require the exact cached Newton endpoint");
        assert!(error.contains("trial"));
    }
}
