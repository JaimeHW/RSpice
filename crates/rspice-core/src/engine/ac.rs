//! AC Small-Signal Analysis
//!
//! Linearizes the circuit at the DC operating point, then performs
//! frequency-domain analysis at each specified frequency. Supports
//! parallel frequency sweeps when the `parallel` feature is enabled.

use super::{Engine, SimulationError};
use crate::analysis::ac::AcResult;
use crate::device::{MatrixStamper, NonlinearDevice};
use crate::solver::ComplexMatrix;
use crate::{CircuitData, Complex64, Netlist, NodeId, Value};
use std::f64::consts::PI;

impl Engine {
    #[inline]
    fn ac_node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn stamp_complex_four_terminal(
        matrix: &mut ComplexMatrix,
        row_pos: usize,
        row_neg: usize,
        col_pos: usize,
        col_neg: usize,
        y: Complex64,
    ) {
        if row_pos > 0 {
            if col_pos > 0 {
                matrix.add(row_pos - 1, col_pos - 1, y);
            }
            if col_neg > 0 {
                matrix.add(row_pos - 1, col_neg - 1, -y);
            }
        }
        if row_neg > 0 {
            if col_pos > 0 {
                matrix.add(row_neg - 1, col_pos - 1, -y);
            }
            if col_neg > 0 {
                matrix.add(row_neg - 1, col_neg - 1, y);
            }
        }
    }

    #[inline]
    fn stamp_transmission_line_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        omega: Value,
    ) {
        // Distributed-line Y-parameters:
        // Y11 = Y22 = Y0 * coth(gamma)
        // Y12 = Y21 = -Y0 * csch(gamma)
        // where gamma = alpha + j*omega*td (dimensionless over one delay length).
        let y0 = 1.0 / tline.z0;
        let attenuation = tline.attenuation().clamp(1e-12, 1.0);
        let alpha = (-attenuation.ln()).max(1e-12); // avoid exact lossless singular poles
        let gamma = Complex64::new(alpha, omega * tline.td);
        let sinh_gamma = gamma.sinh();

        let (y11, y12) = if sinh_gamma.norm() < 1e-12 {
            // Series expansion around gamma=0 for numerical stability.
            let inv_gamma = Complex64::new(1.0, 0.0) / gamma;
            let coth_gamma = inv_gamma + gamma / 3.0;
            let csch_gamma = inv_gamma - gamma / 6.0;
            (
                Complex64::new(y0, 0.0) * coth_gamma,
                -Complex64::new(y0, 0.0) * csch_gamma,
            )
        } else {
            let cosh_gamma = gamma.cosh();
            (
                Complex64::new(y0, 0.0) * (cosh_gamma / sinh_gamma),
                -Complex64::new(y0, 0.0) / sinh_gamma,
            )
        };
        let y21 = y12;
        let y22 = y11;

        // Stamp differential 2-port:
        // i1 = y11*v1 + y12*v2
        // i2 = y21*v1 + y22*v2
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node1_pos,
            tline.node1_neg,
            y11,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node2_pos,
            tline.node2_neg,
            y12,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node1_pos,
            tline.node1_neg,
            y21,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node2_pos,
            tline.node2_neg,
            y22,
        );
    }

    #[inline]
    fn stamp_imag_two_terminal(
        matrix: &mut ComplexMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        susceptance: Value,
    ) {
        if node_pos > 0 {
            matrix.add_imag(node_pos - 1, node_pos - 1, susceptance);
            if node_neg > 0 {
                matrix.add_imag(node_pos - 1, node_neg - 1, -susceptance);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add_imag(node_neg - 1, node_pos - 1, -susceptance);
            }
            matrix.add_imag(node_neg - 1, node_neg - 1, susceptance);
        }
    }

    #[inline]
    fn stamp_nonlinear_small_signal_real(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
    ) {
        struct AcRealStamper<'a> {
            matrix: &'a mut ComplexMatrix,
        }

        impl MatrixStamper for AcRealStamper<'_> {
            #[inline]
            fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
                if row > 0 && col > 0 {
                    self.matrix.add_real(row - 1, col - 1, value);
                }
            }

            #[inline]
            fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
                // AC uses only Jacobian matrix terms from nonlinear devices.
            }
        }

        let mut stamper = AcRealStamper { matrix };
        let mut rhs_dummy: [Value; 0] = [];
        circuit
            .diodes
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .bjts
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .mosfets
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        for jfet in &circuit.jfets {
            jfet.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.vswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.iswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices.iter() {
            // AC linearization uses Jacobian terms at the DC operating point.
            // Verilog-A device stamping exposes Jacobian through matrix callbacks.
            let mut cloned = device.clone();
            cloned.stamp(
                op_voltages,
                |row, col, value| matrix.add_real(row, col, value),
                |_index, _value| {},
            );
        }
    }

    #[inline]
    fn stamp_nonlinear_capacitances(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        // Diode junction capacitance Cj(Vd) + diffusion capacitance.
        for diode in &circuit.diodes.devices {
            let va = Self::ac_node_voltage(op_voltages, diode.node_anode);
            let vc = Self::ac_node_voltage(op_voltages, diode.node_cathode);
            let c = diode.junction_capacitance(va - vc);
            if c.is_finite() && c > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    diode.node_anode,
                    diode.node_cathode,
                    omega * c,
                );
            }
        }

        // BJT base-emitter and base-collector depletion/diffusion capacitances.
        for bjt in &circuit.bjts.devices {
            let vc = Self::ac_node_voltage(op_voltages, bjt.node_collector);
            let vb = Self::ac_node_voltage(op_voltages, bjt.node_base);
            let ve = Self::ac_node_voltage(op_voltages, bjt.node_emitter);
            let (cbe, cbc) = bjt.junction_capacitances(vb - ve, vb - vc);

            if cbe.is_finite() && cbe > 0.0 {
                Self::stamp_imag_two_terminal(matrix, bjt.node_base, bjt.node_emitter, omega * cbe);
            }
            if cbc.is_finite() && cbc > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    bjt.node_base,
                    bjt.node_collector,
                    omega * cbc,
                );
            }
        }

        // JFET gate-source and gate-drain depletion capacitances.
        for jfet in &circuit.jfets {
            let vd = Self::ac_node_voltage(op_voltages, jfet.drain);
            let vg = Self::ac_node_voltage(op_voltages, jfet.gate);
            let vs = Self::ac_node_voltage(op_voltages, jfet.source);
            let pol = jfet.jfet_type.polarity();
            let (cgs, cgd) = jfet.capacitances(pol * (vg - vs), pol * (vg - vd));

            if cgs.is_finite() && cgs > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.source, omega * cgs);
            }
            if cgd.is_finite() && cgd > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.drain, omega * cgd);
            }
        }
    }

    /// Run AC small-signal analysis
    ///
    /// Linearizes circuit at DC operating point, then solves at each frequency.
    /// When the `parallel` feature is enabled and there are many frequency points,
    /// the frequency sweep is parallelized for better performance.
    pub fn run_ac(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
    ) -> Result<Vec<AcResult>, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point
        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = if has_nonlinear {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };
        if has_nonlinear {
            // Align stateful nonlinear models (limited junction voltages, operating region)
            // with the final converged operating-point solution before AC linearization.
            circuit.update_nonlinear(&dc_solution);
        }

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        // Closure to solve at a single frequency
        let solve_at_freq = |freq: Value| -> Result<AcResult, SimulationError> {
            let omega = 2.0 * PI * freq;

            // Create fresh complex matrix for this frequency (thread-safe)
            let mut ac_matrix = ComplexMatrix::from_real_structure(&matrix);

            // Stamp resistors (real conductance)
            for (r_idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
                let g = circuit.resistors.conductances[r_idx];

                if stamp.pp.row > 0 && stamp.pp.col > 0 {
                    ac_matrix.add_real(stamp.pp.row - 1, stamp.pp.col - 1, g);
                }
                if stamp.pn.row > 0 && stamp.pn.col > 0 {
                    ac_matrix.add_real(stamp.pn.row - 1, stamp.pn.col - 1, -g);
                }
                if stamp.np.row > 0 && stamp.np.col > 0 {
                    ac_matrix.add_real(stamp.np.row - 1, stamp.np.col - 1, -g);
                }
                if stamp.nn.row > 0 && stamp.nn.col > 0 {
                    ac_matrix.add_real(stamp.nn.row - 1, stamp.nn.col - 1, g);
                }
            }

            // Stamp transmission lines as distributed 2-port Y-parameters.
            for tline in &circuit.tlines {
                Self::stamp_transmission_line_ac(&mut ac_matrix, tline, omega);
            }

            // Nonlinear device Jacobian (real part) evaluated at DC operating point.
            if has_nonlinear {
                Self::stamp_nonlinear_small_signal_real(&mut ac_matrix, &circuit, &dc_solution);
            }

            // Stamp capacitors: jωC
            for (i, stamp) in circuit.capacitors.stamps.iter().enumerate() {
                let c = circuit
                    .capacitors
                    .capacitances
                    .get(i)
                    .copied()
                    .unwrap_or(0.0);
                let jwc = omega * c; // Imaginary part

                if stamp.pp.row > 0 && stamp.pp.col > 0 {
                    ac_matrix.add_imag(stamp.pp.row - 1, stamp.pp.col - 1, jwc);
                }
                if stamp.pn.row > 0 && stamp.pn.col > 0 {
                    ac_matrix.add_imag(stamp.pn.row - 1, stamp.pn.col - 1, -jwc);
                }
                if stamp.np.row > 0 && stamp.np.col > 0 {
                    ac_matrix.add_imag(stamp.np.row - 1, stamp.np.col - 1, -jwc);
                }
                if stamp.nn.row > 0 && stamp.nn.col > 0 {
                    ac_matrix.add_imag(stamp.nn.row - 1, stamp.nn.col - 1, jwc);
                }
            }

            // Nonlinear semiconductor junction capacitances at the operating point.
            if has_nonlinear {
                Self::stamp_nonlinear_capacitances(&mut ac_matrix, &circuit, &dc_solution, omega);
            }

            // Stamp MOSFET capacitances: jωCgs, jωCgd, jωCgb (Meyer model)
            for mos in &circuit.mosfets.devices {
                let (cgs, cgd, cgb) = mos.ac_capacitances();
                let ng = mos.node_gate;
                let nd = mos.node_drain;
                let ns = mos.node_source;
                let nb = mos.node_bulk;

                // Cgs: gate-source capacitance
                let jwcgs = omega * cgs;
                if ng > 0 && ng > 0 {
                    ac_matrix.add_imag(ng - 1, ng - 1, jwcgs);
                }
                if ng > 0 && ns > 0 {
                    ac_matrix.add_imag(ng - 1, ns - 1, -jwcgs);
                }
                if ns > 0 && ng > 0 {
                    ac_matrix.add_imag(ns - 1, ng - 1, -jwcgs);
                }
                if ns > 0 && ns > 0 {
                    ac_matrix.add_imag(ns - 1, ns - 1, jwcgs);
                }

                // Cgd: gate-drain capacitance
                let jwcgd = omega * cgd;
                if ng > 0 && ng > 0 {
                    ac_matrix.add_imag(ng - 1, ng - 1, jwcgd);
                }
                if ng > 0 && nd > 0 {
                    ac_matrix.add_imag(ng - 1, nd - 1, -jwcgd);
                }
                if nd > 0 && ng > 0 {
                    ac_matrix.add_imag(nd - 1, ng - 1, -jwcgd);
                }
                if nd > 0 && nd > 0 {
                    ac_matrix.add_imag(nd - 1, nd - 1, jwcgd);
                }

                // Cgb: gate-bulk capacitance
                let jwcgb = omega * cgb;
                if ng > 0 && ng > 0 {
                    ac_matrix.add_imag(ng - 1, ng - 1, jwcgb);
                }
                if ng > 0 && nb > 0 {
                    ac_matrix.add_imag(ng - 1, nb - 1, -jwcgb);
                }
                if nb > 0 && ng > 0 {
                    ac_matrix.add_imag(nb - 1, ng - 1, -jwcgb);
                }
                if nb > 0 && nb > 0 {
                    ac_matrix.add_imag(nb - 1, nb - 1, jwcgb);
                }
            }

            // Voltage sources for AC (MNA branch equations)
            for i in 0..circuit.voltage_sources.len() {
                let np = circuit.voltage_sources.node_pos[i];
                let nn = circuit.voltage_sources.node_neg[i];
                let br_ordinal = circuit.voltage_sources.branch_indices[i];
                let br = circuit.get_branch_matrix_index(br_ordinal);

                if np > 0 {
                    ac_matrix.add_real(br - 1, np - 1, 1.0);
                    ac_matrix.add_real(np - 1, br - 1, 1.0);
                }
                if nn > 0 {
                    ac_matrix.add_real(br - 1, nn - 1, -1.0);
                    ac_matrix.add_real(nn - 1, br - 1, -1.0);
                }
            }

            // Inductors for AC:
            // V(np)-V(nn)-jωL*I = 0
            for i in 0..circuit.inductors.len() {
                let np = circuit.inductors.node_pos[i];
                let nn = circuit.inductors.node_neg[i];
                let br_ordinal = circuit.inductors.branch_indices[i];
                let br = circuit.get_branch_matrix_index(br_ordinal);
                let l = circuit.inductors.inductances[i];

                if np > 0 {
                    ac_matrix.add_real(br - 1, np - 1, 1.0);
                    ac_matrix.add_real(np - 1, br - 1, 1.0);
                }
                if nn > 0 {
                    ac_matrix.add_real(br - 1, nn - 1, -1.0);
                    ac_matrix.add_real(nn - 1, br - 1, -1.0);
                }
                ac_matrix.add_imag(br - 1, br - 1, -omega * l);
            }

            // Controlled sources: VCVS
            for i in 0..circuit.vcvs.len() {
                let np = circuit.vcvs.node_pos[i];
                let nn = circuit.vcvs.node_neg[i];
                let cp = circuit.vcvs.ctrl_pos[i];
                let cn = circuit.vcvs.ctrl_neg[i];
                let br_ordinal = circuit.vcvs.branch_indices[i];
                let br = circuit.get_branch_matrix_index(br_ordinal);
                let gain = circuit.vcvs.gains[i];

                if np > 0 {
                    ac_matrix.add_real(br - 1, np - 1, 1.0);
                    ac_matrix.add_real(np - 1, br - 1, 1.0);
                }
                if nn > 0 {
                    ac_matrix.add_real(br - 1, nn - 1, -1.0);
                    ac_matrix.add_real(nn - 1, br - 1, -1.0);
                }
                if cp > 0 {
                    ac_matrix.add_real(br - 1, cp - 1, -gain);
                }
                if cn > 0 {
                    ac_matrix.add_real(br - 1, cn - 1, gain);
                }
            }

            // Controlled sources: VCCS
            for i in 0..circuit.vccs.len() {
                let np = circuit.vccs.node_pos[i];
                let nn = circuit.vccs.node_neg[i];
                let cp = circuit.vccs.ctrl_pos[i];
                let cn = circuit.vccs.ctrl_neg[i];
                let gm = circuit.vccs.transconductances[i];

                if np > 0 && cp > 0 {
                    ac_matrix.add_real(np - 1, cp - 1, gm);
                }
                if np > 0 && cn > 0 {
                    ac_matrix.add_real(np - 1, cn - 1, -gm);
                }
                if nn > 0 && cp > 0 {
                    ac_matrix.add_real(nn - 1, cp - 1, -gm);
                }
                if nn > 0 && cn > 0 {
                    ac_matrix.add_real(nn - 1, cn - 1, gm);
                }
            }

            // Controlled sources: CCCS
            for i in 0..circuit.cccs.len() {
                let np = circuit.cccs.node_pos[i];
                let nn = circuit.cccs.node_neg[i];
                let ctrl_branch_ordinal = circuit.cccs.ctrl_branch[i];
                let gain = circuit.cccs.gains[i];
                if ctrl_branch_ordinal == 0 {
                    continue;
                }
                let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

                if np > 0 {
                    ac_matrix.add_real(np - 1, cb - 1, gain);
                }
                if nn > 0 {
                    ac_matrix.add_real(nn - 1, cb - 1, -gain);
                }
            }

            // Controlled sources: CCVS
            for i in 0..circuit.ccvs.len() {
                let np = circuit.ccvs.node_pos[i];
                let nn = circuit.ccvs.node_neg[i];
                let br_ordinal = circuit.ccvs.branch_indices[i];
                let ctrl_branch_ordinal = circuit.ccvs.ctrl_branch[i];
                let rm = circuit.ccvs.transresistances[i];
                if br_ordinal == 0 || ctrl_branch_ordinal == 0 {
                    continue;
                }
                let br = circuit.get_branch_matrix_index(br_ordinal);
                let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

                if np > 0 {
                    ac_matrix.add_real(br - 1, np - 1, 1.0);
                    ac_matrix.add_real(np - 1, br - 1, 1.0);
                }
                if nn > 0 {
                    ac_matrix.add_real(br - 1, nn - 1, -1.0);
                    ac_matrix.add_real(nn - 1, br - 1, -1.0);
                }
                ac_matrix.add_real(br - 1, cb - 1, -rm);
            }

            // Add small diagonal for numerical stability
            for i in 0..size {
                ac_matrix.add_real(i, i, 1e-15);
            }

            // RHS: stamp AC excitation for each voltage source with AC specification
            let mut rhs = vec![Complex64::new(0.0, 0.0); size];
            for i in 0..circuit.voltage_sources.len() {
                let ac_mag = circuit.voltage_sources.ac_magnitudes[i];
                let ac_phase = circuit.voltage_sources.ac_phases[i];

                // Only stamp sources with non-zero AC magnitude
                if ac_mag.abs() > 1e-15 {
                    let br_ordinal = circuit.voltage_sources.branch_indices[i];
                    let br = circuit.get_branch_matrix_index(br_ordinal);
                    // Convert magnitude and phase to complex: mag * e^(j*phase)
                    let ac_value = Complex64::from_polar(ac_mag, ac_phase);
                    rhs[br - 1] = ac_value;
                }
            }

            // Independent current sources AC excitation
            for i in 0..circuit.current_sources.len() {
                let ac_mag = circuit.current_sources.ac_magnitudes[i];
                let ac_phase = circuit.current_sources.ac_phases[i];
                if ac_mag.abs() <= 1e-15 {
                    continue;
                }
                let i_ac = Complex64::from_polar(ac_mag, ac_phase);
                let np = circuit.current_sources.node_pos[i];
                let nn = circuit.current_sources.node_neg[i];

                if np > 0 {
                    rhs[np - 1] -= i_ac;
                }
                if nn > 0 {
                    rhs[nn - 1] += i_ac;
                }
            }

            // Solve
            let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;

            Ok(AcResult {
                frequency: freq,
                voltages: solution[..num_nodes].to_vec(),
                currents: if size > num_nodes {
                    solution[num_nodes..].to_vec()
                } else {
                    vec![]
                },
            })
        };

        // Use parallel iteration when feature is enabled and we have many frequencies
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;

            // Parallel threshold: use parallel for 10+ frequency points
            if frequencies.len() >= 10 {
                let results: Result<Vec<_>, _> = frequencies
                    .par_iter()
                    .map(|&freq| solve_at_freq(freq))
                    .collect();
                return results;
            }
        }

        // Sequential fallback (or when parallel feature disabled)
        frequencies
            .iter()
            .map(|&freq| solve_at_freq(freq))
            .collect()
    }
}
