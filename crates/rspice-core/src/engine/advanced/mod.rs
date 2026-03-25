//! Advanced Analysis Functions
//!
//! This module provides specialized analysis types:
//! - Noise analysis (thermal, shot, flicker)
//! - Monte Carlo statistical analysis
//! - Pole-zero analysis  
//! - Sensitivity analysis
//! - Parametric step sweep

#![allow(clippy::too_many_arguments)]
use super::{Engine, SimulationError};
use crate::analysis::monte_carlo::{
    Distribution, MonteCarloResult, VariableStatistics, Xorshift128Plus,
};
use crate::analysis::noise::{NoiseContribution, NoiseResult, NoiseSource};
use crate::analysis::pole_zero::{Matrix, PoleZeroAnalyzer, PoleZeroConfig, PoleZeroResult};
use crate::netlist::{ElementKind, SourceSpec, StepCommand, StepTarget};
use crate::solver::{ComplexMatrix, SimulationResult};
use crate::{Complex64, Netlist, Value};
use std::collections::{HashMap, HashSet};
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
        self.run_noise_ports(netlist, output_node, None, frequencies, temperature)
    }

    /// Run noise analysis with optional differential output reference and
    /// explicit input source for input-referred normalization.
    pub fn run_noise_with_input_source(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: &str,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            output_pos,
            output_neg,
            Some(input_source),
            frequencies,
            temperature,
        )
    }

    /// Run noise analysis with optional differential output reference.
    ///
    /// The measured output noise is based on:
    /// - `V(output_pos)` when `output_neg` is `None`
    /// - `V(output_pos) - V(output_neg)` when `output_neg` is provided
    pub fn run_noise_ports(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            output_pos,
            output_neg,
            None,
            frequencies,
            temperature,
        )
    }

    fn run_noise_internal(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: Option<&str>,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        #[derive(Clone, Copy)]
        enum InputExcitation {
            VoltageSource { branch_matrix_index: usize },
            CurrentSource { node_pos: usize, node_neg: usize },
        }

        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point for bias-dependent noise.
        let dc_solution = self.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;

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
                let name = circuit
                    .resistors
                    .names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("R{}", i + 1));
                noise_sources.push(NoiseSource::thermal(name, stamp.pp.row, stamp.nn.row, r));
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

        if output_pos > num_nodes {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for noise analysis: output_pos={} (max={})",
                output_pos, num_nodes
            )));
        }
        if let Some(node) = output_neg {
            if node > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "Invalid node for noise analysis: output_neg={} (max={})",
                    node, num_nodes
                )));
            }
            if node == output_pos {
                return Err(SimulationError::Circuit(
                    "Invalid noise output port: output_pos and output_neg cannot be the same"
                        .to_string(),
                ));
            }
        }

        let input_excitation = match input_source {
            None => None,
            Some(source_name) => {
                if let Some(voltage_idx) = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(source_name))
                {
                    let branch_ordinal = circuit.voltage_sources.branch_indices[voltage_idx];
                    let branch_matrix_index = circuit.get_branch_matrix_index(branch_ordinal) - 1;
                    Some(InputExcitation::VoltageSource {
                        branch_matrix_index,
                    })
                } else if let Some(current_idx) = circuit
                    .current_sources
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(source_name))
                {
                    Some(InputExcitation::CurrentSource {
                        node_pos: circuit.current_sources.node_pos[current_idx],
                        node_neg: circuit.current_sources.node_neg[current_idx],
                    })
                } else {
                    return Err(SimulationError::Circuit(format!(
                        "Noise input source '{}' not found (expected independent V/I source)",
                        source_name
                    )));
                }
            }
        };

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

                let input_gain_sq = if let Some(excitation) = input_excitation {
                    let mut rhs = vec![Complex64::new(0.0, 0.0); size];
                    match excitation {
                        InputExcitation::VoltageSource {
                            branch_matrix_index,
                        } => {
                            if branch_matrix_index < rhs.len() {
                                rhs[branch_matrix_index] = Complex64::new(1.0, 0.0);
                            }
                        }
                        InputExcitation::CurrentSource { node_pos, node_neg } => {
                            if node_pos > 0 && node_pos <= num_nodes {
                                rhs[node_pos - 1] -= Complex64::new(1.0, 0.0);
                            }
                            if node_neg > 0 && node_neg <= num_nodes {
                                rhs[node_neg - 1] += Complex64::new(1.0, 0.0);
                            }
                        }
                    }
                    match ac_matrix.solve(&rhs) {
                        Ok(solution) => {
                            let v_pos = if output_pos > 0 && output_pos <= num_nodes {
                                solution[output_pos - 1]
                            } else {
                                Complex64::new(0.0, 0.0)
                            };
                            let v_neg = match output_neg {
                                Some(node) if node > 0 && node <= num_nodes => solution[node - 1],
                                _ => Complex64::new(0.0, 0.0),
                            };
                            let gain = (v_pos - v_neg).norm();
                            gain * gain
                        }
                        Err(_) => 0.0,
                    }
                } else {
                    1.0
                };

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
                        let v_pos = if output_pos > 0 && output_pos <= num_nodes {
                            solution[output_pos - 1]
                        } else {
                            Complex64::new(0.0, 0.0)
                        };
                        let v_neg = match output_neg {
                            Some(node) if node > 0 && node <= num_nodes => solution[node - 1],
                            _ => Complex64::new(0.0, 0.0),
                        };
                        let v_out = (v_pos - v_neg).norm();

                        // Output voltage noise = Si * |Z_trans|^2
                        let output_v2 = si * v_out * v_out;
                        if output_v2.is_finite() {
                            total_noise_v2_hz += output_v2;

                            contributions.push(NoiseContribution {
                                device_name: source.device_name.clone(),
                                noise_type: source.noise_type,
                                output_contribution: output_v2,
                                percentage: 0.0, // Will calculate after summing
                            });
                        }
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
                    input_referred_density: if input_excitation.is_some() {
                        if input_gain_sq > 1e-30 {
                            total_noise_v2_hz / input_gain_sq
                        } else {
                            Value::INFINITY
                        }
                    } else {
                        total_noise_v2_hz
                    },
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
        self.run_monte_carlo_with_options(
            netlist,
            num_runs,
            seed,
            Distribution::Gaussian { sigma: 0.01 },
            None,
        )
    }

    /// Run Monte Carlo analysis with configurable distribution and parameter filter.
    pub fn run_monte_carlo_with_options(
        &self,
        netlist: &Netlist,
        num_runs: usize,
        seed: u64,
        distribution: Distribution,
        parameter_filter: Option<&[String]>,
    ) -> Result<MonteCarloResult, SimulationError> {
        let spread = match distribution {
            Distribution::Gaussian { sigma } => sigma,
            Distribution::Uniform { tolerance } => tolerance,
            Distribution::WorstCase { tolerance } => tolerance,
        };
        if !spread.is_finite() || spread < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Monte Carlo spread must be finite and non-negative, got {}",
                spread
            )));
        }

        let normalized_filter: Option<HashSet<String>> = parameter_filter.and_then(|params| {
            let normalized: HashSet<String> = params
                .iter()
                .map(|p| p.trim().to_ascii_uppercase())
                .filter(|p| !p.is_empty())
                .collect();
            if normalized.is_empty() {
                None
            } else {
                Some(normalized)
            }
        });

        let mut all_eligible_params: Vec<(String, Value)> = netlist
            .params
            .all_params()
            .into_iter()
            .filter(|(_, value)| value.is_finite() && value.abs() > 0.0)
            .collect();
        all_eligible_params.sort_by(|a, b| a.0.cmp(&b.0));

        if let Some(filter) = &normalized_filter {
            let available: HashSet<String> = all_eligible_params
                .iter()
                .map(|(name, _)| name.clone())
                .collect();
            let mut unknown: Vec<String> = filter
                .iter()
                .filter(|name| !available.contains(*name))
                .cloned()
                .collect();
            unknown.sort();
            if !unknown.is_empty() {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo parameter(s) not defined or not eligible: {}",
                    unknown.join(", ")
                )));
            }
        }

        let mut monte_params: Vec<(String, Value)> = all_eligible_params
            .into_iter()
            .filter(|(name, _)| {
                normalized_filter
                    .as_ref()
                    .map(|filter| filter.contains(name))
                    .unwrap_or(true)
            })
            .collect();

        if normalized_filter.is_some() && monte_params.is_empty() {
            return Err(SimulationError::Circuit(
                "Monte Carlo parameter filter did not match any eligible parameters".to_string(),
            ));
        }

        if let Some(source) = &netlist.source_text {
            let mut bound_params = Vec::new();
            let mut unbound_params = Vec::new();
            for (name, nominal) in std::mem::take(&mut monte_params) {
                if Self::source_references_param(source, &name) {
                    bound_params.push((name, nominal));
                } else {
                    unbound_params.push(name);
                }
            }

            if normalized_filter.is_some() && !unbound_params.is_empty() {
                unbound_params.sort();
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo parameter(s) are not bound to any netlist expression: {}",
                    unbound_params.join(", ")
                )));
            }
            if !bound_params.is_empty() {
                monte_params = bound_params;
            } else if !unbound_params.is_empty() {
                return Err(SimulationError::Circuit(
                    "Monte Carlo parameter set is not bound to any netlist expression".to_string(),
                ));
            }
        }

        let mut rng = Xorshift128Plus::new(seed);
        let mut results = Vec::with_capacity(num_runs);
        let mut first_node_names: Option<Vec<String>> = None;

        for _run in 0..num_runs {
            let netlist_for_run = if monte_params.is_empty() {
                netlist.clone()
            } else {
                let overrides: Vec<(String, Value)> = monte_params
                    .iter()
                    .map(|(name, nominal)| {
                        let varied =
                            Self::sample_monte_carlo_value(&mut rng, *nominal, distribution);
                        (name.clone(), varied)
                    })
                    .collect();
                let (perturbed, _) = Self::create_perturbed_netlist_multi(netlist, &overrides)?;
                perturbed
            };

            match self.run_dc_op(&netlist_for_run) {
                Ok(result) => {
                    if first_node_names.is_none() {
                        first_node_names = Some(result.node_names.clone());
                    }
                    results.push(result.node_voltages.clone());
                }
                Err(_) => {
                    // Skip failed runs
                }
            }
        }

        // Compute statistics for each non-ground node.
        // node_voltages[0] is always ground.
        let max_node_id = results
            .first()
            .map(|r| r.len().saturating_sub(1))
            .unwrap_or(0);
        let mut variables: HashMap<String, VariableStatistics> = HashMap::new();

        for node_id in 1..=max_node_id.min(10) {
            let samples: Vec<Value> = results
                .iter()
                .filter_map(|r| r.get(node_id).copied())
                .collect();

            if !samples.is_empty() {
                let numeric_name = format!("V({})", node_id);
                let numeric_label = numeric_name.clone();
                let stats = VariableStatistics::from_samples(&numeric_name, samples.clone(), 20);
                variables.insert(numeric_name, stats);

                if let Some(node_names) = &first_node_names {
                    if let Some(node_name) = node_names.get(node_id) {
                        let named_key = format!("V({})", node_name);
                        if named_key != numeric_label {
                            let alias_stats =
                                VariableStatistics::from_samples(&named_key, samples, 20);
                            variables.insert(named_key, alias_stats);
                        }
                    }
                }
            }
        }

        Ok(MonteCarloResult {
            num_runs: results.len(),
            variables,
            all_converged: results.len() == num_runs,
            num_failures: num_runs - results.len(),
        })
    }

    fn sample_monte_carlo_value(
        rng: &mut Xorshift128Plus,
        nominal: Value,
        distribution: Distribution,
    ) -> Value {
        let magnitude = nominal.abs();
        match distribution {
            Distribution::Gaussian { sigma } => {
                let sigma = sigma.abs();
                nominal + rng.next_gaussian() * magnitude * sigma
            }
            Distribution::Uniform { tolerance } => {
                let tolerance = tolerance.abs();
                let delta = magnitude * tolerance;
                nominal + (2.0 * rng.next_f64() - 1.0) * delta
            }
            Distribution::WorstCase { tolerance } => {
                let tolerance = tolerance.abs();
                let delta = magnitude * tolerance;
                let sign = if (rng.next_u64() & 1) == 0 { -1.0 } else { 1.0 };
                nominal + sign * delta
            }
        }
    }

    /// Run pole-zero analysis
    ///
    /// Finds poles and zeros of the transfer function from input to output node.
    /// Uses the MNA formulation: (G + sÂ·C)Â·V = I
    pub fn run_pz(
        &self,
        netlist: &Netlist,
        input_node: usize,
        output_node: usize,
    ) -> Result<PoleZeroResult, SimulationError> {
        self.run_pz_ports(
            netlist,
            input_node,
            None,
            output_node,
            None,
            true,
            true,
            true,
        )
    }

    /// Run pole-zero analysis with explicit differential ports and mode control.
    pub fn run_pz_ports(
        &self,
        netlist: &Netlist,
        input_pos: usize,
        input_neg: Option<usize>,
        output_pos: usize,
        output_neg: Option<usize>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
    ) -> Result<PoleZeroResult, SimulationError> {
        let circuit = self.build_circuit(netlist)?;
        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        let validate_node = |node: usize, label: &str| -> Result<(), SimulationError> {
            if node > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "Invalid node for PZ analysis: {}={} (max={})",
                    label, node, num_nodes
                )));
            }
            Ok(())
        };

        validate_node(input_pos, "input_pos")?;
        if let Some(node) = input_neg {
            validate_node(node, "input_neg")?;
        }
        validate_node(output_pos, "output_pos")?;
        if let Some(node) = output_neg {
            validate_node(node, "output_neg")?;
        }

        if input_pos == 0 {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for PZ analysis: input_pos={} (must be non-ground)",
                input_pos
            )));
        }
        if output_pos == 0 {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for PZ analysis: output_pos={} (must be non-ground)",
                output_pos
            )));
        }
        if input_neg == Some(input_pos) {
            return Err(SimulationError::Circuit(
                "Invalid PZ input port: input_pos and input_neg cannot be the same".to_string(),
            ));
        }
        if output_neg == Some(output_pos) {
            return Err(SimulationError::Circuit(
                "Invalid PZ output port: output_pos and output_neg cannot be the same".to_string(),
            ));
        }

        // Build descriptor MNA matrices:
        // (G + s*C) x = b, where x includes node voltages and branch currents.
        let mut g_matrix = Matrix::zeros(size, size);
        let mut c_matrix = Matrix::zeros(size, size);

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

        // Stamp independent voltage sources into G (MNA branch equations)
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;

            if np > 0 {
                g_matrix.add(br, np - 1, 1.0);
                g_matrix.add(np - 1, br, 1.0);
            }
            if nn > 0 {
                g_matrix.add(br, nn - 1, -1.0);
                g_matrix.add(nn - 1, br, -1.0);
            }
        }

        // Stamp inductors:
        // V(np)-V(nn)-s*L*I = 0  => C(br,br) = -L
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let br_ordinal = circuit.inductors.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;
            let l = circuit.inductors.inductances[i];

            if np > 0 {
                g_matrix.add(br, np - 1, 1.0);
                g_matrix.add(np - 1, br, 1.0);
            }
            if nn > 0 {
                g_matrix.add(br, nn - 1, -1.0);
                g_matrix.add(nn - 1, br, -1.0);
            }
            c_matrix.add(br, br, -l);
        }

        // Controlled sources: VCVS
        for i in 0..circuit.vcvs.len() {
            let np = circuit.vcvs.node_pos[i];
            let nn = circuit.vcvs.node_neg[i];
            let cp = circuit.vcvs.ctrl_pos[i];
            let cn = circuit.vcvs.ctrl_neg[i];
            let br_ordinal = circuit.vcvs.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;
            let gain = circuit.vcvs.gains[i];

            if np > 0 {
                g_matrix.add(br, np - 1, 1.0);
                g_matrix.add(np - 1, br, 1.0);
            }
            if nn > 0 {
                g_matrix.add(br, nn - 1, -1.0);
                g_matrix.add(nn - 1, br, -1.0);
            }
            if cp > 0 {
                g_matrix.add(br, cp - 1, -gain);
            }
            if cn > 0 {
                g_matrix.add(br, cn - 1, gain);
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
                g_matrix.add(np - 1, cp - 1, gm);
            }
            if np > 0 && cn > 0 {
                g_matrix.add(np - 1, cn - 1, -gm);
            }
            if nn > 0 && cp > 0 {
                g_matrix.add(nn - 1, cp - 1, -gm);
            }
            if nn > 0 && cn > 0 {
                g_matrix.add(nn - 1, cn - 1, gm);
            }
        }

        // Controlled sources: CCCS
        for i in 0..circuit.cccs.len() {
            let np = circuit.cccs.node_pos[i];
            let nn = circuit.cccs.node_neg[i];
            let ctrl_branch_ordinal = circuit.cccs.ctrl_branch[i];
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal) - 1;
            let gain = circuit.cccs.gains[i];

            if np > 0 {
                g_matrix.add(np - 1, cb, gain);
            }
            if nn > 0 {
                g_matrix.add(nn - 1, cb, -gain);
            }
        }

        // Controlled sources: CCVS
        for i in 0..circuit.ccvs.len() {
            let np = circuit.ccvs.node_pos[i];
            let nn = circuit.ccvs.node_neg[i];
            let br_ordinal = circuit.ccvs.branch_indices[i];
            let ctrl_branch_ordinal = circuit.ccvs.ctrl_branch[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal) - 1;
            let rm = circuit.ccvs.transresistances[i];

            if np > 0 {
                g_matrix.add(br, np - 1, 1.0);
                g_matrix.add(np - 1, br, 1.0);
            }
            if nn > 0 {
                g_matrix.add(br, nn - 1, -1.0);
                g_matrix.add(nn - 1, br, -1.0);
            }
            g_matrix.add(br, cb, -rm);
        }

        // Add small diagonal for numerical stability
        for i in 0..num_nodes {
            g_matrix.add(i, i, 1e-12);
        }

        // Create analyzer and run
        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::poles_and_zeros(input_pos - 1, output_pos - 1);
        config.input_neg = input_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.output_neg = output_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.input_is_current = input_is_current;
        config.compute_poles = compute_poles;
        config.compute_zeros = compute_zeros;

        Ok(analyzer.analyze(&config))
    }

    fn create_perturbed_netlist(
        netlist: &Netlist,
        param_name: &str,
        param_value: Value,
    ) -> Result<(Netlist, usize), SimulationError> {
        Self::create_perturbed_netlist_multi(
            netlist,
            &[(param_name.to_ascii_uppercase(), param_value)],
        )
    }

    fn create_perturbed_netlist_multi(
        netlist: &Netlist,
        overrides: &[(String, Value)],
    ) -> Result<(Netlist, usize), SimulationError> {
        let mut override_map: HashMap<String, Value> = HashMap::new();
        for (name, value) in overrides {
            override_map.insert(name.to_ascii_uppercase(), *value);
        }

        let mut ordered_overrides: Vec<(String, Value)> = override_map.into_iter().collect();
        ordered_overrides.sort_by(|a, b| a.0.cmp(&b.0));

        let mut perturbed = netlist.clone();
        for (name, value) in &ordered_overrides {
            perturbed.params.set(name, *value);
        }

        let Some(source) = &netlist.source_text else {
            return Ok((perturbed, 0));
        };

        let referenced = ordered_overrides
            .iter()
            .filter(|(name, _)| Self::source_references_param(source, name))
            .count();
        let overridden_source = Self::build_overridden_source_multi(source, &ordered_overrides);

        let mut reparsed = crate::netlist::parse_netlist(&overridden_source).map_err(|e| {
            SimulationError::Netlist(format!(
                "Failed to reparse netlist for parameter override set {:?}: {}",
                ordered_overrides, e
            ))
        })?;
        for (name, value) in &ordered_overrides {
            reparsed.params.set(name, *value);
        }

        Ok((reparsed, referenced))
    }

    fn logical_lines_after_title(source: &str) -> Vec<String> {
        let mut lines = Vec::new();
        let mut continuation = String::new();

        for raw in source.lines().skip(1) {
            let line = raw.split(';').next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with('*') || line.starts_with('$') {
                continue;
            }

            if line.starts_with('+') {
                if !continuation.is_empty() {
                    continuation.push(' ');
                    continuation.push_str(line.trim_start_matches('+').trim());
                }
                continue;
            }

            if !continuation.is_empty() {
                lines.push(std::mem::take(&mut continuation));
                continuation.clear();
            }
            continuation.push_str(line);
        }

        if !continuation.is_empty() {
            lines.push(continuation);
        }

        lines
    }

    fn contains_identifier(haystack_upper: &str, needle_upper: &str) -> bool {
        if needle_upper.is_empty() {
            return false;
        }
        let haystack_bytes = haystack_upper.as_bytes();
        let needle_len = needle_upper.len();

        for (idx, _) in haystack_upper.match_indices(needle_upper) {
            let before_ok = idx == 0 || !Self::is_identifier_byte(haystack_bytes[idx - 1]);
            let after_idx = idx + needle_len;
            let after_ok = after_idx >= haystack_bytes.len()
                || !Self::is_identifier_byte(haystack_bytes[after_idx]);

            if before_ok && after_ok {
                return true;
            }
        }
        false
    }

    fn is_identifier_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }

    fn param_assignment_present(line: &str, param_upper: &str) -> bool {
        let trimmed = line.trim();
        let upper = trimmed.to_ascii_uppercase();
        if !upper.starts_with(".PARAM") {
            return false;
        }

        let mut idx = ".PARAM".len();
        let bytes = trimmed.as_bytes();
        while idx < bytes.len() {
            while idx < bytes.len() && (bytes[idx].is_ascii_whitespace() || bytes[idx] == b',') {
                idx += 1;
            }
            if idx >= bytes.len() {
                break;
            }

            let start = idx;
            while idx < bytes.len() && Self::is_identifier_byte(bytes[idx]) {
                idx += 1;
            }
            if idx == start {
                idx += 1;
                continue;
            }

            let name = &trimmed[start..idx];
            while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                idx += 1;
            }

            if idx < bytes.len() && bytes[idx] == b'=' && name.eq_ignore_ascii_case(param_upper) {
                return true;
            }
        }

        false
    }

    fn source_references_param(source: &str, param_name: &str) -> bool {
        let param_upper = param_name.to_ascii_uppercase();

        Self::logical_lines_after_title(source).iter().any(|line| {
            let upper = line.to_ascii_uppercase();
            if upper.starts_with(".PARAM")
                || upper.starts_with(".IC")
                || upper.starts_with(".NODESET")
            {
                return false;
            }
            Self::contains_identifier(&upper, &param_upper)
        })
    }

    fn build_overridden_source_multi(source: &str, overrides: &[(String, Value)]) -> String {
        use std::fmt::Write;

        let title = source.lines().next().unwrap_or("Untitled");
        let mut out = String::new();

        let _ = writeln!(out, "{}", title);
        for (name, value) in overrides {
            let _ = writeln!(out, ".PARAM {}={:.17e}", name, value);
        }

        for line in Self::logical_lines_after_title(source) {
            let mut override_suffix = String::new();
            for (name, value) in overrides {
                if Self::param_assignment_present(&line, name) {
                    let _ = write!(override_suffix, " {}={:.17e}", name, value);
                }
            }

            if override_suffix.is_empty() {
                let _ = writeln!(out, "{}", line);
            } else {
                let _ = writeln!(out, "{}{}", line, override_suffix);
            }
        }

        out
    }

    /// Run sensitivity analysis
    ///
    /// Computes dVout/dparam using finite differences.
    /// Useful for design optimization and tolerance analysis.
    pub fn run_sensitivity(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        delta: Option<Value>,
    ) -> Result<Value, SimulationError> {
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        let (netlist_plus, rebuilt_plus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value + h)?;
        let (netlist_minus, rebuilt_minus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value - h)?;

        if netlist.source_text.is_some() && rebuilt_plus == 0 && rebuilt_minus == 0 {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        let result_plus = self.run_dc_op(&netlist_plus)?;
        let result_minus = self.run_dc_op(&netlist_minus)?;

        let v_plus = result_plus.voltage(output_node);
        let v_minus = result_minus.voltage(output_node);

        Ok((v_plus - v_minus) / (2.0 * h))
    }

    /// Run AC sensitivity analysis for a parameter across frequencies.
    ///
    /// Computes central differences of output voltage magnitude:
    /// d|Vout|/dp ~= (|Vout(p+h)| - |Vout(p-h)|) / (2h)
    pub fn run_sensitivity_ac(
        &self,
        netlist: &Netlist,
        output_node: usize,
        param_name: &str,
        param_value: Value,
        frequencies: &[Value],
        delta: Option<Value>,
    ) -> Result<Vec<Value>, SimulationError> {
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        let (netlist_plus, rebuilt_plus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value + h)?;
        let (netlist_minus, rebuilt_minus) =
            Self::create_perturbed_netlist(netlist, param_name, param_value - h)?;

        if netlist.source_text.is_some() && rebuilt_plus == 0 && rebuilt_minus == 0 {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        let plus = self.run_ac(&netlist_plus, frequencies)?;
        let minus = self.run_ac(&netlist_minus, frequencies)?;
        if plus.len() != minus.len() {
            return Err(SimulationError::Circuit(
                "AC sensitivity produced inconsistent sweep lengths".to_string(),
            ));
        }

        Ok(plus
            .iter()
            .zip(minus.iter())
            .map(|(p, m)| {
                (p.voltage_magnitude(output_node) - m.voltage_magnitude(output_node)) / (2.0 * h)
            })
            .collect())
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
        if values.is_empty() {
            return Ok(Vec::new());
        }

        let mut results = Vec::with_capacity(values.len());
        let mut any_binding = false;

        for &value in values {
            let (modified_netlist, rebuilt) =
                Self::create_perturbed_netlist(netlist, param_name, value)?;
            any_binding |= rebuilt > 0;

            match self.run_dc_op(&modified_netlist) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    log::warn!("Step {} = {} failed: {}", param_name, value, e);
                }
            }
        }

        if netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "Parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        Ok(results)
    }

    /// Run `.STEP` command execution for PARAM/DEVICE/MODEL targets.
    pub fn run_step_command(
        &self,
        netlist: &Netlist,
        step_cmd: &StepCommand,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        match step_cmd.target {
            StepTarget::Param => self.run_step(netlist, &step_cmd.name, values),
            StepTarget::Device => self.run_step_device(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
            ),
            StepTarget::Model => self.run_step_model(
                netlist,
                &step_cmd.name,
                step_cmd.param_name.as_deref(),
                values,
            ),
            StepTarget::Temp => Err(SimulationError::Circuit(
                "Engine `.STEP TEMP` execution is handled via temperature-configured runs"
                    .to_string(),
            )),
        }
    }

    fn run_step_device(
        &self,
        netlist: &Netlist,
        device_name: &str,
        param_name: Option<&str>,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if values.is_empty() {
            return Ok(Vec::new());
        }

        let device_idx = netlist
            .elements
            .iter()
            .position(|e| e.name.eq_ignore_ascii_case(device_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP DEVICE target '{}' not found in netlist",
                    device_name
                ))
            })?;

        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let element = stepped.elements.get_mut(device_idx).ok_or_else(|| {
                SimulationError::Circuit("Internal step device index error".to_string())
            })?;
            Self::apply_device_step_value(&mut element.kind, param_name, value)?;

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    log::warn!(
                        "Step DEVICE {}{} = {} failed: {}",
                        device_name,
                        param_name.map(|p| format!(".{}", p)).unwrap_or_default(),
                        value,
                        e
                    );
                }
            }
        }

        Ok(results)
    }

    fn run_step_model(
        &self,
        netlist: &Netlist,
        model_name: &str,
        param_name: Option<&str>,
        values: &[Value],
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        if values.is_empty() {
            return Ok(Vec::new());
        }

        let param_name = param_name.ok_or_else(|| {
            SimulationError::Circuit(format!(
                ".STEP MODEL {} requires an explicit parameter name",
                model_name
            ))
        })?;

        let model_idx = netlist
            .models
            .iter()
            .position(|m| m.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    ".STEP MODEL target '{}' not found in netlist",
                    model_name
                ))
            })?;

        let param_upper = param_name.to_ascii_uppercase();
        let mut results = Vec::with_capacity(values.len());
        for &value in values {
            let mut stepped = netlist.clone();
            let model = stepped.models.get_mut(model_idx).ok_or_else(|| {
                SimulationError::Circuit("Internal step model index error".to_string())
            })?;

            if let Some((_, v)) = model
                .params
                .iter_mut()
                .find(|(name, _)| name.eq_ignore_ascii_case(&param_upper))
            {
                *v = value;
            } else {
                model.params.push((param_upper.clone(), value));
            }

            match self.run_dc_op(&stepped) {
                Ok(result) => results.push((value, result)),
                Err(e) => {
                    log::warn!(
                        "Step MODEL {}.{} = {} failed: {}",
                        model_name,
                        param_upper,
                        value,
                        e
                    );
                }
            }
        }

        Ok(results)
    }

    fn apply_device_step_value(
        kind: &mut ElementKind,
        param_name: Option<&str>,
        value: Value,
    ) -> Result<(), SimulationError> {
        let param_upper = param_name.map(|p| p.trim().to_ascii_uppercase());
        let matches_param = |aliases: &[&str]| -> bool {
            match &param_upper {
                None => true,
                Some(name) => aliases.iter().any(|alias| name.eq_ignore_ascii_case(alias)),
            }
        };

        match kind {
            ElementKind::Resistor {
                value: r,
                model,
                instance_params,
                ..
            } => {
                if !matches_param(&["R", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported resistor step parameter; use R or VALUE".to_string(),
                    ));
                }
                *r = value;
                if model.is_some() {
                    if let Some((_, existing)) = instance_params
                        .iter_mut()
                        .find(|(name, _)| name.eq_ignore_ascii_case("R"))
                    {
                        *existing = value;
                    } else {
                        instance_params.push(("R".to_string(), value));
                    }
                }
                Ok(())
            }
            ElementKind::Capacitor { value: c, .. } => {
                if !matches_param(&["C", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported capacitor step parameter; use C or VALUE".to_string(),
                    ));
                }
                *c = value;
                Ok(())
            }
            ElementKind::Inductor { value: l, .. }
            | ElementKind::JilesAthertonInductor { value: l, .. } => {
                if !matches_param(&["L", "VALUE", "INDUCTANCE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported inductor step parameter; use L or VALUE".to_string(),
                    ));
                }
                *l = value;
                Ok(())
            }
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                if !matches_param(&["DC", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported source step parameter; use DC or VALUE".to_string(),
                    ));
                }
                Self::set_source_dc_value(spec, value)
            }
            ElementKind::Vcvs { gain, .. } | ElementKind::Cccs { gain, .. } => {
                if !matches_param(&["GAIN", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported controlled-source step parameter; use GAIN".to_string(),
                    ));
                }
                *gain = value;
                Ok(())
            }
            ElementKind::Vccs {
                transconductance, ..
            } => {
                if !matches_param(&["GM", "TRANSCONDUCTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported VCCS step parameter; use GM".to_string(),
                    ));
                }
                *transconductance = value;
                Ok(())
            }
            ElementKind::Ccvs {
                transresistance, ..
            } => {
                if !matches_param(&["RM", "TRANSRESISTANCE", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported CCVS step parameter; use RM".to_string(),
                    ));
                }
                *transresistance = value;
                Ok(())
            }
            ElementKind::Coupling { coefficient, .. } => {
                if !matches_param(&["K", "COUPLING", "VALUE"]) {
                    return Err(SimulationError::Circuit(
                        "Unsupported coupling step parameter; use K".to_string(),
                    ));
                }
                *coefficient = value;
                Ok(())
            }
            ElementKind::TransmissionLine {
                z0, td, freq, nl, ..
            } => {
                match param_upper.as_deref() {
                    None => *z0 = Some(value),
                    Some("Z0") | Some("VALUE") => *z0 = Some(value),
                    Some("TD") => *td = Some(value),
                    Some("F") | Some("FREQ") => *freq = Some(value),
                    Some("NL") => *nl = Some(value),
                    Some(other) => {
                        return Err(SimulationError::Circuit(format!(
                            "Unsupported transmission-line step parameter '{}' (use Z0, TD, FREQ, NL)",
                            other
                        )));
                    }
                }
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Unsupported .STEP DEVICE target for this element type".to_string(),
            )),
        }
    }

    fn set_source_dc_value(spec: &mut SourceSpec, value: Value) -> Result<(), SimulationError> {
        match spec {
            SourceSpec::Dc(v) => {
                *v = value;
                Ok(())
            }
            SourceSpec::DcAc { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            SourceSpec::DcAcTransient { dc_value, .. } => {
                *dc_value = value;
                Ok(())
            }
            _ => Err(SimulationError::Circuit(
                "Stepping source VALUE/DC is supported for DC, DC+AC, DC+transient, and DC+AC+transient source definitions only"
                    .to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests;
