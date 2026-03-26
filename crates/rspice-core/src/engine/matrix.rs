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

        // Behavioral voltage sources use the same MNA branch topology as independent V-sources.
        for source in &circuit.behavioral_sources.voltage_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            let br = circuit.get_branch_matrix_index(source.branch_ordinal);

            if np > 0 {
                triplets.push((br - 1, np - 1, 0.0));
                triplets.push((np - 1, br - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((br - 1, nn - 1, 0.0));
                triplets.push((nn - 1, br - 1, 0.0));
            }

            // Linearized behavioral dependency terms on branch equation row.
            for col in source.bound_solution_indices() {
                triplets.push((br - 1, col, 0.0));
            }
        }

        // Behavioral current sources: row coupling from KCL rows to referenced variables.
        for source in &circuit.behavioral_sources.current_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            let referenced_cols: Vec<usize> = source.bound_solution_indices().collect();
            if np > 0 {
                for &col in &referenced_cols {
                    triplets.push((np - 1, col, 0.0));
                }
            }
            if nn > 0 {
                for &col in &referenced_cols {
                    triplets.push((nn - 1, col, 0.0));
                }
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

        // MOSFET stamps.
        // Include full 4x4 terminal coupling so transient overlap-capacitance
        // companions (Cgs/Cgd/Cgb) can stamp gate-row entries safely.
        for mosfet in &circuit.mosfets.devices {
            let d = mosfet.node_drain;
            let g = mosfet.node_gate;
            let s = mosfet.node_source;
            let b = mosfet.node_bulk;
            for &row in &[d, g, s, b] {
                for &col in &[d, g, s, b] {
                    if row > 0 && col > 0 {
                        triplets.push((row - 1, col - 1, 0.0));
                    }
                }
            }
        }

        // JFET stamps (3-terminal: include full 3x3 topology to support AC capacitances).
        for jfet in &circuit.jfets {
            let d = jfet.drain;
            let g = jfet.gate;
            let s = jfet.source;
            for &row in &[d, g, s] {
                for &col in &[d, g, s] {
                    if row > 0 && col > 0 {
                        triplets.push((row - 1, col - 1, 0.0));
                    }
                }
            }
        }

        // Switch stamps.
        // Voltage-controlled and current-controlled switches are linearized as:
        //   I = g(control) * V(p,n)
        // so topology must include not only the 2x2 main branch stamp but also
        // control-variable coupling Jacobian columns.
        for sw in &circuit.vswitches {
            let p = sw.node_pos;
            let n = sw.node_neg;
            let cp = sw.ctrl_pos;
            let cn = sw.ctrl_neg;
            if p > 0 {
                triplets.push((p - 1, p - 1, 0.0));
            }
            if p > 0 && n > 0 {
                triplets.push((p - 1, n - 1, 0.0));
            }
            if n > 0 && p > 0 {
                triplets.push((n - 1, p - 1, 0.0));
            }
            if n > 0 {
                triplets.push((n - 1, n - 1, 0.0));
            }

            // Control coupling (rows p/n, columns cp/cn).
            if p > 0 && cp > 0 {
                triplets.push((p - 1, cp - 1, 0.0));
            }
            if p > 0 && cn > 0 {
                triplets.push((p - 1, cn - 1, 0.0));
            }
            if n > 0 && cp > 0 {
                triplets.push((n - 1, cp - 1, 0.0));
            }
            if n > 0 && cn > 0 {
                triplets.push((n - 1, cn - 1, 0.0));
            }
        }
        for sw in &circuit.iswitches {
            let p = sw.node_pos;
            let n = sw.node_neg;
            if p > 0 {
                triplets.push((p - 1, p - 1, 0.0));
            }
            if p > 0 && n > 0 {
                triplets.push((p - 1, n - 1, 0.0));
            }
            if n > 0 && p > 0 {
                triplets.push((n - 1, p - 1, 0.0));
            }
            if n > 0 {
                triplets.push((n - 1, n - 1, 0.0));
            }

            // Control-branch coupling (rows p/n, column I(control_source)).
            if let Some(cb) = sw.ctrl_branch {
                if p > 0 && cb > 0 {
                    triplets.push((p - 1, cb - 1, 0.0));
                }
                if n > 0 && cb > 0 {
                    triplets.push((n - 1, cb - 1, 0.0));
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

        // Coupled inductor pair stamps.
        for binding in &circuit.coupled_inductor_pairs {
            let device = &binding.device;
            let br1 = circuit.get_branch_matrix_index(binding.branch1_ordinal);
            let br2 = circuit.get_branch_matrix_index(binding.branch2_ordinal);

            if device.node1_pos > 0 {
                triplets.push((br1 - 1, device.node1_pos - 1, 0.0));
                triplets.push((device.node1_pos - 1, br1 - 1, 0.0));
            }
            if device.node1_neg > 0 {
                triplets.push((br1 - 1, device.node1_neg - 1, 0.0));
                triplets.push((device.node1_neg - 1, br1 - 1, 0.0));
            }
            if device.node2_pos > 0 {
                triplets.push((br2 - 1, device.node2_pos - 1, 0.0));
                triplets.push((device.node2_pos - 1, br2 - 1, 0.0));
            }
            if device.node2_neg > 0 {
                triplets.push((br2 - 1, device.node2_neg - 1, 0.0));
                triplets.push((device.node2_neg - 1, br2 - 1, 0.0));
            }

            triplets.push((br1 - 1, br1 - 1, 0.0));
            triplets.push((br1 - 1, br2 - 1, 0.0));
            triplets.push((br2 - 1, br1 - 1, 0.0));
            triplets.push((br2 - 1, br2 - 1, 0.0));
        }

        // Multi-winding transformer stamps.
        for binding in &circuit.multi_winding_transformers {
            for (winding_idx, &(pos, neg)) in binding.device.nodes.iter().enumerate() {
                let br = circuit.get_branch_matrix_index(binding.branch_ordinals[winding_idx]);
                if pos > 0 {
                    triplets.push((br - 1, pos - 1, 0.0));
                    triplets.push((pos - 1, br - 1, 0.0));
                }
                if neg > 0 {
                    triplets.push((br - 1, neg - 1, 0.0));
                    triplets.push((neg - 1, br - 1, 0.0));
                }
            }
            for row_idx in 0..binding.branch_ordinals.len() {
                let br_row = circuit.get_branch_matrix_index(binding.branch_ordinals[row_idx]);
                for col_idx in 0..binding.branch_ordinals.len() {
                    let br_col = circuit.get_branch_matrix_index(binding.branch_ordinals[col_idx]);
                    triplets.push((br_row - 1, br_col - 1, 0.0));
                }
            }
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

        // CCCS stamps (output node equations depend on controlling branch current)
        for i in 0..circuit.cccs.len() {
            let np = circuit.cccs.node_pos[i];
            let nn = circuit.cccs.node_neg[i];
            let cb_ordinal = circuit.cccs.ctrl_branch[i];
            if cb_ordinal == 0 {
                continue;
            }
            let cb = circuit.get_branch_matrix_index(cb_ordinal);

            if np > 0 {
                triplets.push((np - 1, cb - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((nn - 1, cb - 1, 0.0));
            }
        }

        // CCVS stamps (voltage-source branch equation plus controlling branch coupling)
        for i in 0..circuit.ccvs.len() {
            let np = circuit.ccvs.node_pos[i];
            let nn = circuit.ccvs.node_neg[i];
            let br_ordinal = circuit.ccvs.branch_indices[i];
            let cb_ordinal = circuit.ccvs.ctrl_branch[i];
            if br_ordinal == 0 || cb_ordinal == 0 {
                continue;
            }
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let cb = circuit.get_branch_matrix_index(cb_ordinal);

            if np > 0 {
                triplets.push((br - 1, np - 1, 0.0));
                triplets.push((np - 1, br - 1, 0.0));
            }
            if nn > 0 {
                triplets.push((br - 1, nn - 1, 0.0));
                triplets.push((nn - 1, br - 1, 0.0));
            }
            triplets.push((br - 1, cb - 1, 0.0));
        }

        // Transmission line ports (companion model uses two shunt conductances per line).
        // Dynamic delayed-wave terms are injected on RHS during transient stamping.
        for tl in &circuit.tlines {
            for &(np, nn) in &[(tl.node1_pos, tl.node1_neg), (tl.node2_pos, tl.node2_neg)] {
                if np > 0 {
                    triplets.push((np - 1, np - 1, 0.0));
                }
                if np > 0 && nn > 0 {
                    triplets.push((np - 1, nn - 1, 0.0));
                }
                if nn > 0 && np > 0 {
                    triplets.push((nn - 1, np - 1, 0.0));
                }
                if nn > 0 {
                    triplets.push((nn - 1, nn - 1, 0.0));
                }
            }

            // Cross-port terms required by AC distributed-line Y-parameters.
            let p1 = tl.node1_pos;
            let n1 = tl.node1_neg;
            let p2 = tl.node2_pos;
            let n2 = tl.node2_neg;

            for &(r, c) in &[
                (p1, p2),
                (p1, n2),
                (n1, p2),
                (n1, n2),
                (p2, p1),
                (p2, n1),
                (n2, p1),
                (n2, n1),
            ] {
                if r > 0 && c > 0 {
                    triplets.push((r - 1, c - 1, 0.0));
                }
            }
        }

        // XSPICE analog output ports:
        // - Voltage outputs reserve branch-equation topology (MNA branch variable)
        // - Current outputs reserve nodal conductance/current topology.
        // Passive inout analog terminals reserve full nodal coupling so
        // code-model deferred stamps can inject conductance terms safely.
        for instance in &circuit.xspice_instances {
            let ports = instance.ports();
            let mut inout_analog_nodes: Vec<usize> = Vec::new();
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                let Some(port) = ports.get(port_idx) else {
                    continue;
                };

                if port.direction == crate::xspice::PortDirection::InOut {
                    match connection {
                        crate::xspice::PortConnection::Analog(node) => {
                            if *node > 0 {
                                inout_analog_nodes.push(*node);
                            }
                        }
                        crate::xspice::PortConnection::Differential(pos, neg) => {
                            if *pos > 0 {
                                inout_analog_nodes.push(*pos);
                            }
                            if *neg > 0 {
                                inout_analog_nodes.push(*neg);
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
                if port.direction != crate::xspice::PortDirection::Out {
                    continue;
                }

                match port.default_type {
                    crate::xspice::PortType::Voltage
                    | crate::xspice::PortType::DifferentialVoltage => {
                        let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) else {
                            continue;
                        };
                        let br = circuit.get_branch_matrix_index(branch_ordinal);
                        let br_idx = br - 1;
                        match connection {
                            crate::xspice::PortConnection::Analog(node) => {
                                if *node > 0 {
                                    triplets.push((br_idx, *node - 1, 0.0));
                                    triplets.push((*node - 1, br_idx, 0.0));
                                }
                            }
                            crate::xspice::PortConnection::Differential(pos, neg) => {
                                if *pos > 0 {
                                    triplets.push((br_idx, *pos - 1, 0.0));
                                    triplets.push((*pos - 1, br_idx, 0.0));
                                }
                                if *neg > 0 {
                                    triplets.push((br_idx, *neg - 1, 0.0));
                                    triplets.push((*neg - 1, br_idx, 0.0));
                                }
                            }
                            _ => {}
                        }
                    }
                    crate::xspice::PortType::Current => match connection {
                        crate::xspice::PortConnection::Analog(node) => {
                            if *node > 0 {
                                triplets.push((*node - 1, *node - 1, 0.0));
                            }
                        }
                        crate::xspice::PortConnection::Differential(pos, neg) => {
                            if *pos > 0 {
                                triplets.push((*pos - 1, *pos - 1, 0.0));
                            }
                            if *neg > 0 {
                                triplets.push((*neg - 1, *neg - 1, 0.0));
                            }
                            if *pos > 0 && *neg > 0 {
                                triplets.push((*pos - 1, *neg - 1, 0.0));
                                triplets.push((*neg - 1, *pos - 1, 0.0));
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            inout_analog_nodes.sort_unstable();
            inout_analog_nodes.dedup();
            for &row in &inout_analog_nodes {
                for &col in &inout_analog_nodes {
                    triplets.push((row - 1, col - 1, 0.0));
                }
            }
        }

        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            // Build a conservative matrix topology for this Verilog-A device:
            // full coupling among all external and internal nodes used by the
            // instance. This guarantees all Jacobian stamp locations exist.
            let mut device_nodes: Vec<usize> = Vec::new();
            for term in 0..device.num_terminals() {
                let node = device.node_for_terminal(term);
                if node > 0 {
                    device_nodes.push(node);
                }
            }
            for idx in 0..device.num_internal_nodes() {
                if let Some(node) = device.internal_node_index(idx)
                    && node > 0
                {
                    device_nodes.push(node);
                }
            }
            device_nodes.sort_unstable();
            device_nodes.dedup();

            for &row in &device_nodes {
                for &col in &device_nodes {
                    triplets.push((row - 1, col - 1, 0.0));
                }
            }
        }

        // Add diagonal entries to ensure structure
        for i in 0..size {
            triplets.push((i, i, 1e-12)); // GMIN for numerical stability
        }

        StaticMatrix::from_triplets(size, size, &triplets).map_err(SimulationError::Solver)
    }
}
