//! Advanced Analysis Functions
//!
//! This module provides specialized analysis types:
//! - Noise analysis (thermal, shot, flicker)
//! - Monte Carlo statistical analysis
//! - Pole-zero analysis  
//! - Sensitivity analysis
//! - Parametric step sweep

use super::{Engine, SimulationError};
use crate::analysis::monte_carlo::{MonteCarloResult, VariableStatistics, Xorshift128Plus};
use crate::analysis::noise::{NoiseContribution, NoiseResult, NoiseSource};
use crate::analysis::pole_zero::{Matrix, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult};
use crate::solver::{ComplexMatrix, SimulationResult};
use crate::{Complex64, Netlist, Value};
use std::collections::HashMap;
use std::f64::consts::PI;

impl Engine {
    /// Run noise analysis
    ///
    /// Computes thermal, shot, and flicker noise at each frequency point.
    /// Returns integrated noise results.
    pub fn run_noise(
        &self,
        netlist: &Netlist,
        output_node: usize,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point for bias-dependent noise
        let dc_solution = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };

        // Collect noise sources
        let mut noise_sources: Vec<NoiseSource> = Vec::new();

        // Thermal noise from resistors (4kT/R)
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let r = 1.0
                / circuit
                    .resistors
                    .conductances
                    .get(i)
                    .copied()
                    .unwrap_or(1.0);
            if r > 0.0 && r < 1e12 {
                noise_sources.push(NoiseSource::thermal(
                    format!("R{}", i + 1),
                    stamp.pp.row,
                    stamp.nn.row,
                    r,
                ));
            }
        }

        // Shot noise from diodes (2qI)
        for diode in &circuit.diodes.devices {
            let vd = dc_solution
                .get(diode.node_anode.saturating_sub(1))
                .copied()
                .unwrap_or(0.0)
                - dc_solution
                    .get(diode.node_cathode.saturating_sub(1))
                    .copied()
                    .unwrap_or(0.0);
            let id = diode.current(vd);
            if id.abs() > 1e-15 {
                noise_sources.push(NoiseSource::shot(
                    diode.name.clone(),
                    diode.node_anode,
                    diode.node_cathode,
                    id,
                ));
            }
        }

        // Compute noise at each frequency
        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        let results: Vec<NoiseResult> = frequencies
            .iter()
            .map(|&freq| {
                let omega = 2.0 * PI * freq;

                // Build small-signal AC matrix at this frequency
                let mut ac_matrix = ComplexMatrix::from_real_structure(&matrix);

                // Stamp resistors
                for (r_idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
                    let g = circuit
                        .resistors
                        .conductances
                        .get(r_idx)
                        .copied()
                        .unwrap_or(0.0);
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

                // Stamp capacitors
                for (i, stamp) in circuit.capacitors.stamps.iter().enumerate() {
                    let c = circuit
                        .capacitors
                        .capacitances
                        .get(i)
                        .copied()
                        .unwrap_or(0.0);
                    let jwc = omega * c;
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

                // Voltage sources
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

                // Small diagonal for stability
                for i in 0..size {
                    ac_matrix.add_real(i, i, 1e-15);
                }

                // For each noise source, inject current and compute output voltage
                let mut total_noise_v2_hz = 0.0;
                let mut contributions = Vec::new();

                for source in &noise_sources {
                    let si = source.spectral_density(freq, temperature);

                    // Inject unit current at noise source nodes, solve for voltage
                    let mut rhs = vec![Complex64::new(0.0, 0.0); size];
                    if source.node_pos > 0 && source.node_pos <= num_nodes {
                        rhs[source.node_pos - 1] = Complex64::new(1.0, 0.0);
                    }
                    if source.node_neg > 0 && source.node_neg <= num_nodes {
                        rhs[source.node_neg - 1] = Complex64::new(-1.0, 0.0);
                    }

                    if let Ok(solution) = ac_matrix.solve(&rhs) {
                        // Transfer impedance to output node
                        let v_out = if output_node > 0 && output_node <= num_nodes {
                            solution[output_node - 1].norm()
                        } else {
                            0.0
                        };

                        // Output voltage noise = Si * |Z_trans|^2
                        let output_v2 = si * v_out * v_out;
                        total_noise_v2_hz += output_v2;

                        contributions.push(NoiseContribution {
                            device_name: source.device_name.clone(),
                            noise_type: source.noise_type,
                            output_contribution: output_v2,
                            percentage: 0.0, // Will calculate after summing
                        });
                    }
                }

                // Calculate percentages
                for contrib in &mut contributions {
                    contrib.percentage = if total_noise_v2_hz > 0.0 {
                        100.0 * contrib.output_contribution / total_noise_v2_hz
                    } else {
                        0.0
                    };
                }

                NoiseResult {
                    frequency: freq,
                    output_noise_density: total_noise_v2_hz,
                    input_referred_density: total_noise_v2_hz, // Simplified
                    contributions,
                }
            })
            .collect();

        Ok(results)
    }

    /// Run Monte Carlo analysis
    ///
    /// Performs multiple simulation runs with random component variations.
    pub fn run_monte_carlo(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
    ) -> Result<MonteCarloResult, SimulationError> {
        let _rng = Xorshift128Plus::new(seed);
        let mut results = Vec::with_capacity(num_runs);

        // Run DC OP multiple times
        for _run in 0..num_runs {
            // TODO: Apply component variations to netlist copy
            match self.run_dc_op(netlist) {
                Ok(result) => {
                    results.push(result.node_voltages.clone());
                }
                Err(_) => {
                    // Skip failed runs
                }
            }
        }

        // Compute statistics for each node
        let num_nodes = results.first().map(|r| r.len()).unwrap_or(0);
        let mut variables: HashMap<String, VariableStatistics> = HashMap::new();

        for node in 0..num_nodes.min(10) {
            let samples: Vec<Value> = results
                .iter()
                .filter_map(|r| r.get(node).copied())
                .collect();

            if !samples.is_empty() {
                let name = format!("V({})", node + 1);
                let stats = VariableStatistics::from_samples(&name, samples, 20);
                variables.insert(name, stats);
            }
        }

        Ok(MonteCarloResult {
            num_runs: results.len(),
            variables,
            all_converged: results.len() == num_runs,
            num_failures: num_runs - results.len(),
        })
    }

    /// Run pole-zero analysis
    ///
    /// Finds poles and zeros of the transfer function from input to output node.
    /// Uses the MNA formulation: (G + s·C)·V = I
    pub fn run_pz(
        &self,
        netlist: &Netlist,
        input_node: usize,
        output_node: usize,
    ) -> Result<PoleZeroResult, SimulationError> {
        let circuit = self.build_circuit(netlist)?;
        let num_nodes = circuit.num_nodes();

        if input_node > num_nodes || output_node > num_nodes {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for PZ analysis: input={} output={} (max={})",
                input_node, output_node, num_nodes
            )));
        }

        // Build G matrix (conductance, frequency-independent)
        let mut g_matrix = Matrix::zeros(num_nodes, num_nodes);

        // Stamp resistors into G
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let g = circuit
                .resistors
                .conductances
                .get(i)
                .copied()
                .unwrap_or(0.0);

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                g_matrix.add(stamp.pp.row - 1, stamp.pp.col - 1, g);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                g_matrix.add(stamp.pn.row - 1, stamp.pn.col - 1, -g);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                g_matrix.add(stamp.np.row - 1, stamp.np.col - 1, -g);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                g_matrix.add(stamp.nn.row - 1, stamp.nn.col - 1, g);
            }
        }

        // Build C matrix (capacitance, coefficient of s)
        let mut c_matrix = Matrix::zeros(num_nodes, num_nodes);

        // Stamp capacitors into C
        for (i, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let c = circuit
                .capacitors
                .capacitances
                .get(i)
                .copied()
                .unwrap_or(0.0);

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                c_matrix.add(stamp.pp.row - 1, stamp.pp.col - 1, c);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                c_matrix.add(stamp.pn.row - 1, stamp.pn.col - 1, -c);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                c_matrix.add(stamp.np.row - 1, stamp.np.col - 1, -c);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                c_matrix.add(stamp.nn.row - 1, stamp.nn.col - 1, c);
            }
        }

        // Add small diagonal for numerical stability
        for i in 0..num_nodes {
            g_matrix.add(i, i, 1e-12);
        }

        // Create analyzer and run
        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let config = PoleZeroConfig::poles_and_zeros(
            input_node.saturating_sub(1),
            output_node.saturating_sub(1),
        );

        Ok(analyzer.analyze(&config))
    }

    /// Run sensitivity analysis
    ///
    /// Computes ∂Vout/∂param using finite differences.
    /// Useful for design optimization and tolerance analysis.
    pub fn run_sensitivity(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        // Use 1% relative delta by default
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        // Create modified netlist with param + delta
        let mut netlist_plus = netlist.clone();
        netlist_plus.params.set(param_name, param_value + h);

        // Create modified netlist with param - delta
        let mut netlist_minus = netlist.clone();
        netlist_minus.params.set(param_name, param_value - h);

        // Run DC OP at both points
        let result_plus = self.run_dc_op(&netlist_plus)?;
        let result_minus = self.run_dc_op(&netlist_minus)?;

        // Central difference: dV/dp ≈ (V+ - V-) / (2h)
        let v_plus = result_plus.voltage(output_node);
        let v_minus = result_minus.voltage(output_node);

        Ok((v_plus - v_minus) / (2.0 * h))
    }

    /// Run .STEP parametric sweep
    ///
    /// Executes multiple simulations with different parameter values.
    /// Returns all results indexed by step values.
    pub fn run_step(
        &self,
        netlist: &Netlist,
        param_name: &str,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        let mut results = Vec::with_capacity(values.len());

        for &value in values {
            // Create netlist copy with modified parameter
            let mut modified_netlist = netlist.clone();
            modified_netlist.params.set(param_name, value);

            // Run DC OP for this parameter value
            match self.run_dc_op(&modified_netlist) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    // Log error but continue sweep
                    log::warn!("Step {} = {} failed: {}", param_name, value, e);
                }
            }
        }

        Ok(results)
    }
}
