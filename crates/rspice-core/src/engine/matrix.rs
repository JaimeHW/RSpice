//! Matrix topology builder
//!
//! Constructs the sparse matrix structure from circuit topology.
//! This is done once during setup and the structure is reused
//! throughout simulation.

use super::{Engine, SimulationError};
use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

impl Engine {
    /// Build static matrix structure from circuit topology
    pub fn build_matrix(&self, circuit: &CircuitData) -> Result<StaticMatrix, SimulationError> {
        let size = circuit.matrix_size();
        if size == 0 {
            return Err(SimulationError::Circuit("Empty circuit".to_string()));
        }

        // First pass: collect all stamp positions to determine structure
        let mut triplets: Vec<(usize, usize, Value)> = Vec::with_capacity(size * 6);

        // Resistor stamps
        for stamp in &circuit.resistors.stamps {
            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                triplets.push((stamp.pp.row - 1, stamp.pp.col - 1, 0.0));
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                triplets.push((stamp.pn.row - 1, stamp.pn.col - 1, 0.0));
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                triplets.push((stamp.np.row - 1, stamp.np.col - 1, 0.0));
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                triplets.push((stamp.nn.row - 1, stamp.nn.col - 1, 0.0));
            }
        }

        // Capacitor stamps (same structure as resistors)
        for stamp in &circuit.capacitors.stamps {
            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                triplets.push((stamp.pp.row - 1, stamp.pp.col - 1, 0.0));
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                triplets.push((stamp.pn.row - 1, stamp.pn.col - 1, 0.0));
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                triplets.push((stamp.np.row - 1, stamp.np.col - 1, 0.0));
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                triplets.push((stamp.nn.row - 1, stamp.nn.col - 1, 0.0));
            }
        }

        // Voltage source stamps
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);

            if np > 0 {
                triplets.push((br - 1, np - 1, 0.0));
                triplets.push((np - 1, br - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((br - 1, nn - 1, 0.0));
                triplets.push((nn - 1, br - 1, 0.0));
            }
        }

        // Diode stamps (2-terminal: 2x2 matrix)
        for diode in &circuit.diodes.devices {
            let a = diode.node_anode;
            let c = diode.node_cathode;
            // 2x2 stamp pattern
            if a > 0 {
                triplets.push((a - 1, a - 1, 0.0));
            }
            if a > 0 && c > 0 {
                triplets.push((a - 1, c - 1, 0.0));
            }
            if c > 0 && a > 0 {
                triplets.push((c - 1, a - 1, 0.0));
            }
            if c > 0 {
                triplets.push((c - 1, c - 1, 0.0));
            }
        }

        // BJT stamps (3-terminal: 3x3 matrix for C, B, E)
        for bjt in &circuit.bjts.devices {
            let c = bjt.node_collector;
            let b = bjt.node_base;
            let e = bjt.node_emitter;
            // 3x3 stamp pattern
            for &row in &[c, b, e] {
                for &col in &[c, b, e] {
                    if row > 0 && col > 0 {
                        triplets.push((row - 1, col - 1, 0.0));
                    }
                }
            }
        }

        // MOSFET stamps (4-terminal but only D, S rows with D, G, S, B columns)
        for mosfet in &circuit.mosfets.devices {
            let d = mosfet.node_drain;
            let g = mosfet.node_gate;
            let s = mosfet.node_source;
            let b = mosfet.node_bulk;
            // Drain and Source rows, all columns
            for &row in &[d, s] {
                for &col in &[d, g, s, b] {
                    if row > 0 && col > 0 {
                        triplets.push((row - 1, col - 1, 0.0));
                    }
                }
            }
        }

        // Inductor stamps (branch variable like voltage source)
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let br_ordinal = circuit.inductors.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);

            // Same pattern as voltage source: branch row connects to node columns
            if np > 0 {
                triplets.push((br - 1, np - 1, 0.0));
                triplets.push((np - 1, br - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((br - 1, nn - 1, 0.0));
                triplets.push((nn - 1, br - 1, 0.0));
            }
            // Branch diagonal for companion model resistance
            triplets.push((br - 1, br - 1, 0.0));
        }

        // VCVS stamps (branch variable for output voltage)
        for i in 0..circuit.vcvs.len() {
            let np = circuit.vcvs.node_pos[i];
            let nn = circuit.vcvs.node_neg[i];
            let cp = circuit.vcvs.ctrl_pos[i];
            let cn = circuit.vcvs.ctrl_neg[i];
            let br_ordinal = circuit.vcvs.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);

            // Branch row: connects to output and control nodes
            if np > 0 {
                triplets.push((br - 1, np - 1, 0.0));
                triplets.push((np - 1, br - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((br - 1, nn - 1, 0.0));
                triplets.push((nn - 1, br - 1, 0.0));
            }
            if cp > 0 {
                triplets.push((br - 1, cp - 1, 0.0));
            }
            if cn > 0 {
                triplets.push((br - 1, cn - 1, 0.0));
            }
        }

        // VCCS stamps (no branch variable, stamps conductance)
        for i in 0..circuit.vccs.len() {
            let np = circuit.vccs.node_pos[i];
            let nn = circuit.vccs.node_neg[i];
            let cp = circuit.vccs.ctrl_pos[i];
            let cn = circuit.vccs.ctrl_neg[i];

            // VCCS stamps gm from control to output nodes
            if np > 0 && cp > 0 {
                triplets.push((np - 1, cp - 1, 0.0));
            }
            if np > 0 && cn > 0 {
                triplets.push((np - 1, cn - 1, 0.0));
            }
            if nn > 0 && cp > 0 {
                triplets.push((nn - 1, cp - 1, 0.0));
            }
            if nn > 0 && cn > 0 {
                triplets.push((nn - 1, cn - 1, 0.0));
            }
        }

        // Add diagonal entries to ensure structure
        for i in 0..size {
            triplets.push((i, i, 1e-12)); // GMIN for numerical stability
        }

        StaticMatrix::from_triplets(size, size, &triplets).map_err(SimulationError::Solver)
    }
}
