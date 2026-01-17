//! AC Small-Signal Analysis
//!
//! Linearizes the circuit at the DC operating point, then performs
//! frequency-domain analysis at each specified frequency. Supports
//! parallel frequency sweeps when the `parallel` feature is enabled.

use super::{Engine, SimulationError};
use crate::analysis::ac::AcResult;
use crate::solver::ComplexMatrix;
use crate::{Complex64, Netlist, Value};
use std::f64::consts::PI;

impl Engine {
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
        let _dc_solution = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };

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

            // Add small diagonal for numerical stability
            for i in 0..size {
                ac_matrix.add_real(i, i, 1e-15);
            }

            // RHS: AC source magnitude (1V for first voltage source)
            let mut rhs = vec![Complex64::new(0.0, 0.0); size];
            if !circuit.voltage_sources.is_empty() {
                let br_ordinal = circuit.voltage_sources.branch_indices[0];
                let br = circuit.get_branch_matrix_index(br_ordinal);
                rhs[br - 1] = Complex64::new(1.0, 0.0); // 1V AC magnitude
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
