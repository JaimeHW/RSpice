//! DC stamping functions
//!
//! Functions for stamping DC values into the matrix and RHS vector.

use super::Engine;
use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

impl Engine {
    /// Add a numerical conditioning term to every MNA equation except private
    /// non-electrical DAE state rows. Unlike circuit GMIN, this intentionally
    /// covers branch-current equations so inductive and controlled-source
    /// systems retain the conditioning contract used by continuation/startup.
    pub(in crate::engine) fn stamp_matrix_conditioning_diagonal(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        matrix_size: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let node_count = circuit.num_nodes().min(matrix_size);
        for index in 0..matrix_size {
            if index >= node_count || !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, value);
            }
        }
    }

    /// Add a diagonal term to electrical node-voltage equations only.
    /// Private DAE state rows share the matrix's nodal prefix but are not
    /// voltages and therefore must not receive circuit GMIN.
    pub(in crate::engine) fn stamp_nodal_gmin(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) {
        if gmin == 0.0 {
            return;
        }
        for index in 0..circuit.num_nodes() {
            if !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, gmin);
            }
        }
    }

    /// Final DC nodal gmin floor.
    ///
    /// B3SOI floating-body currents can be in the e-18 A range at a valid DC
    /// operating point. Xyce/ngspice apply their BSIMSOI GMIN inside the device
    /// load; adding a second simulator-level final shunt visibly deforms those
    /// roots. Keep explicit homotopy GMIN steps unchanged, but solve the final
    /// native BSIM3-SOI DC system without an extra nodal floor. An authored
    /// global RSHUNT likewise makes every electrical nodal diagonal physical;
    /// retaining a parallel numerical floor would systematically perturb the
    /// user-selected resistance even though it is no longer needed for nodal
    /// rank regularization.
    pub(in crate::engine) fn dc_nodal_gmin_floor(&self, circuit: &CircuitData) -> Value {
        let gmin = self.config.convergence_config.gmin_target.max(0.0);
        if circuit.has_b3soi_devices() || circuit.has_global_shunt() {
            0.0
        } else {
            gmin
        }
    }

    /// Conductance of the `.OPTIONS RSHUNT` node-to-ground shunt, or zero when
    /// the deck did not ask for one.
    pub(in crate::engine) fn nodal_shunt_conductance(&self) -> Value {
        self.config
            .rshunt
            .map_or(0.0, |resistance| 1.0 / resistance)
    }

    /// Stamp all DC values into matrix using O(1) direct stamping
    pub(crate) fn stamp_dc_direct(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        // Use the optimized direct stamping from CircuitData
        circuit.stamp_dc_direct(matrix, rhs);
    }

    /// Stamp with scaled sources for source stepping
    pub(crate) fn stamp_dc_scaled(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
        scale: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        circuit.stamp_dc_direct_scaled(matrix, rhs, scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::Netlist;

    #[test]
    fn physical_rshunt_is_additive_with_numerical_gmin_and_not_source_scaled() {
        let netlist = Netlist::parse(
            "additive RSHUNT stamp\n\
             I1 0 out 2\n\
             .OPTIONS RSHUNT=2k\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let gmin = 2.5e-4;
        let residual = matrix.with_probe_values(|probe, rhs| {
            engine.stamp_dc_scaled(&circuit, probe, rhs, gmin, 0.25);
            probe.residual_vector(&[1.0], rhs).expect("residual forms")
        });

        // GMIN=250 uS and RSHUNT=500 uS add to 750 uS. The 2 A source is
        // stepped to 0.5 A, while the physical shunt is deliberately not.
        assert!((residual[0] + 0.5 - 7.5e-4).abs() <= 1.0e-15);
    }

    #[test]
    fn physical_rshunt_excludes_private_non_electrical_state_rows() {
        let engine = Engine::default();
        let mut circuit = CircuitData::new();
        let electrical = circuit.get_or_create_node("electrical");
        let private_state = circuit.get_or_create_node("private_state");
        circuit.global_shunt_conductance = 2.0e-3;
        circuit.non_electrical_state_nodes.insert(private_state);
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let residual = matrix.with_probe_values(|probe, rhs| {
            circuit.stamp_dc_direct(probe, rhs);
            probe
                .residual_vector(&[1.0, 1.0], rhs)
                .expect("residual forms")
        });
        assert_eq!(residual[electrical - 1].to_bits(), 2.0e-3_f64.to_bits());
        assert_eq!(residual[private_state - 1], 0.0);
    }

    #[test]
    fn physical_rshunt_replaces_the_final_numerical_nodal_floor() {
        let netlist = Netlist::parse(
            "physical RSHUNT final floor\n\
             I1 0 out 1m\n\
             .OPTIONS RSHUNT=1e9\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");

        assert!(engine.config.convergence_config.gmin_target > 0.0);
        assert_eq!(engine.dc_nodal_gmin_floor(&circuit), 0.0);
    }
}
