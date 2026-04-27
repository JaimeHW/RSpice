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
use crate::analysis::sensitivity::{ElementDesc, SensitivityAnalyzer, SensitivityResult};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};
use crate::netlist::{ElementKind, SourceSpec, StepCommand, StepTarget};
use crate::solver::SimulationResult;
use crate::{CircuitData, Complex64, Netlist, Value};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

impl Engine {
    #[inline]
    fn noise_node_voltage(voltages: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn differential_noise_output(
        solution: &[Complex64],
        output_pos: usize,
        output_neg: Option<usize>,
        num_nodes: usize,
    ) -> Value {
        let v_pos = if output_pos > 0 && output_pos <= num_nodes {
            solution[output_pos - 1]
        } else {
            Complex64::new(0.0, 0.0)
        };
        let v_neg = match output_neg {
            Some(node) if node > 0 && node <= num_nodes => solution[node - 1],
            _ => Complex64::new(0.0, 0.0),
        };
        (v_pos - v_neg).norm()
    }

    #[inline]
    fn optional_system_index(node_id: usize) -> Option<usize> {
        if node_id == 0 {
            None
        } else {
            Some(node_id - 1)
        }
    }

    #[inline]
    fn ac_linearization_node_voltage(voltages: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    fn descriptor_expand_square(
        g_matrix: &mut Matrix,
        c_matrix: &mut Matrix,
        extra_states: usize,
    ) -> usize {
        let (n, _) = g_matrix.dims();
        let mut g_expanded = Matrix::zeros(n + extra_states, n + extra_states);
        let mut c_expanded = Matrix::zeros(n + extra_states, n + extra_states);
        for row in 0..n {
            for col in 0..n {
                g_expanded.set(row, col, g_matrix.get(row, col));
                c_expanded.set(row, col, c_matrix.get(row, col));
            }
        }
        *g_matrix = g_expanded;
        *c_matrix = c_expanded;
        n
    }

    fn stamp_vbic_pz_descriptor_states(
        circuit: &CircuitData,
        op_voltages: &[Value],
        g_matrix: &mut Matrix,
        c_matrix: &mut Matrix,
    ) {
        for bjt in &circuit.bjts.devices {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::ac_linearization_node_voltage(op_voltages, bjt.node_collector);
            let vb = Self::ac_linearization_node_voltage(op_voltages, bjt.node_base);
            let ve = Self::ac_linearization_node_voltage(op_voltages, bjt.node_emitter);
            let vs = Self::ac_linearization_node_voltage(op_voltages, bjt.node_substrate);
            let snapshot: BjtChargeSnapshot = bjt.charge_snapshot(vc, vb, ve, vs);

            let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
            let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
            let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
            let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
            let mut has_dynamic_charge = false;
            for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
                if !branch.is_active() {
                    continue;
                }
                if branch_idx + 2 >= BJT_DYNAMIC_CHARGE_COUNT {
                    // ngspice small-signal parity excludes VBIC excess-phase TD
                    // companion charges from linearized frequency-domain matrices.
                    continue;
                }
                branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
                has_dynamic_charge = true;
            }
            if !has_dynamic_charge {
                continue;
            }

            let internal_start =
                Self::descriptor_expand_square(g_matrix, c_matrix, BJT_INTERNAL_STATE_DIM);
            let external_nodes = [
                Self::optional_system_index(bjt.node_collector),
                Self::optional_system_index(bjt.node_base),
                Self::optional_system_index(bjt.node_emitter),
                Self::optional_system_index(bjt.node_substrate),
            ];

            for ext_row in 0..BJT_EXTERNAL_STATE_DIM {
                let Some(row_idx) = external_nodes[ext_row] else {
                    continue;
                };

                for ext_col in 0..BJT_EXTERNAL_STATE_DIM {
                    let Some(col_idx) = external_nodes[ext_col] else {
                        continue;
                    };
                    g_matrix.add(
                        row_idx,
                        col_idx,
                        snapshot.reduction.g_ee[ext_row][ext_col]
                            - snapshot.reduction.g_reduced[ext_row][ext_col],
                    );
                    c_matrix.add(row_idx, col_idx, -c_ee[ext_row][ext_col]);
                }

                for int_col in 0..BJT_INTERNAL_STATE_DIM {
                    let col_idx = internal_start + int_col;
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ei[ext_row][int_col]);
                    c_matrix.add(row_idx, col_idx, -c_ei[ext_row][int_col]);
                }
            }

            for int_row in 0..BJT_INTERNAL_STATE_DIM {
                let row_idx = internal_start + int_row;

                for ext_col in 0..BJT_EXTERNAL_STATE_DIM {
                    let Some(col_idx) = external_nodes[ext_col] else {
                        continue;
                    };
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ie[int_row][ext_col]);
                    c_matrix.add(row_idx, col_idx, -c_ie[int_row][ext_col]);
                }

                for int_col in 0..BJT_INTERNAL_STATE_DIM {
                    let col_idx = internal_start + int_col;
                    g_matrix.add(row_idx, col_idx, snapshot.reduction.g_ii[int_row][int_col]);
                    c_matrix.add(row_idx, col_idx, -c_ii[int_row][int_col]);
                }
            }
        }
    }

    fn collect_sensitivity_elements(circuit: &CircuitData) -> Vec<ElementDesc> {
        let mut elements = Vec::new();

        for (idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let name = circuit
                .resistors
                .names
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("R{}", idx + 1));
            let g = circuit
                .resistors
                .small_signal_conductances
                .get(idx)
                .copied()
                .unwrap_or_else(|| {
                    circuit
                        .resistors
                        .conductances
                        .get(idx)
                        .copied()
                        .unwrap_or(0.0)
                });
            if !g.is_finite() || g.abs() <= 1e-30 {
                continue;
            }

            elements.push(ElementDesc::resistor(
                &name,
                Self::optional_system_index(stamp.pp.row),
                Self::optional_system_index(stamp.nn.row),
                1.0 / g,
            ));
        }

        for idx in 0..circuit.current_sources.names.len() {
            let name = circuit.current_sources.names[idx].clone();
            let value = circuit.current_sources.dc_values[idx];
            if !value.is_finite() {
                continue;
            }

            elements.push(ElementDesc::current_source(
                &name,
                Self::optional_system_index(circuit.current_sources.node_pos[idx]),
                Self::optional_system_index(circuit.current_sources.node_neg[idx]),
                value,
            ));
        }

        for idx in 0..circuit.voltage_sources.names.len() {
            let name = circuit.voltage_sources.names[idx].clone();
            let value = circuit.voltage_sources.dc_values[idx];
            let branch_ordinal = circuit.voltage_sources.branch_indices[idx];
            if !value.is_finite() || branch_ordinal == 0 {
                continue;
            }

            elements.push(ElementDesc::voltage_source(
                &name,
                Self::optional_system_index(circuit.voltage_sources.node_pos[idx]),
                Self::optional_system_index(circuit.voltage_sources.node_neg[idx]),
                circuit.get_branch_matrix_index(branch_ordinal) - 1,
                value,
            ));
        }

        elements
    }

    /// Run DC operating-point sensitivity using the linearized MNA system.
    pub fn run_sensitivity_linearized(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
    ) -> Result<SensitivityResult, SimulationError> {
        if output_pos == 0 {
            return Err(SimulationError::Circuit(
                "Sensitivity output node must not be ground".to_string(),
            ));
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }

        let dense_g = Self::build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, 0.0)
            .to_dense_real();
        let elements = Self::collect_sensitivity_elements(&circuit);
        if elements.is_empty() {
            return Err(SimulationError::Circuit(
                "Sensitivity analysis found no eligible linear elements or independent sources"
                    .to_string(),
            ));
        }

        let mut analyzer = SensitivityAnalyzer::new(dense_g, dc_solution, elements);
        analyzer
            .analyze(
                output_pos - 1,
                output_neg.and_then(Self::optional_system_index),
            )
            .ok_or(SimulationError::Solver(
                crate::solver::SolverError::SingularMatrix,
            ))
    }

    fn collect_noise_sources(circuit: &CircuitData, dc_solution: &[Value]) -> Vec<NoiseSource> {
        let mut noise_sources = Vec::new();

        // Thermal noise from resistors (4kT/R).
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let conductance = circuit.resistors.small_signal_conductance(i);
            let resistance = if conductance.abs() > 0.0 {
                1.0 / conductance
            } else {
                f64::INFINITY
            };
            if resistance <= 0.0 || !resistance.is_finite() || resistance >= 1e12 {
                continue;
            }

            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R{}", i + 1));
            noise_sources.push(NoiseSource::thermal(
                name,
                stamp.pp.row,
                stamp.nn.row,
                resistance,
            ));
        }

        // Shot noise from diodes (2qI).
        for diode in &circuit.diodes.devices {
            let vd = Self::noise_node_voltage(dc_solution, diode.node_anode)
                - Self::noise_node_voltage(dc_solution, diode.node_cathode);
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

        // BJT collector/base shot noise and model-card flicker noise.
        for bjt in &circuit.bjts.devices {
            let (ic, ibe, ibc) = bjt.noise_branch_currents();
            if ic > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IC", bjt.name),
                    bjt.node_collector,
                    bjt.node_emitter,
                    ic,
                ));
            }
            if ibe > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IBE", bjt.name),
                    bjt.node_base,
                    bjt.node_emitter,
                    ibe,
                ));
            }
            if ibc > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IBC", bjt.name),
                    bjt.node_base,
                    bjt.node_collector,
                    ibc,
                ));
            }

            if let Some((kf, af, ef)) = bjt.flicker_noise_coefficients() {
                let (_, ib, _) = bjt.operating_point_currents();
                if ib.abs() > 1e-18 {
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", bjt.name),
                        bjt.node_base,
                        bjt.node_emitter,
                        kf,
                        af,
                        ef,
                        ib,
                    ));
                }
            }
        }

        // MOS channel thermal noise and 1/f noise.
        for mos in &circuit.mosfets.devices {
            let gm = mos.transconductance();
            let gamma = mos.channel_thermal_noise_gamma();
            if gm > 1e-18 && gamma > 0.0 {
                let resistance = 1.0 / (gamma * gm).max(1e-30);
                noise_sources.push(NoiseSource::thermal(
                    format!("{}:thermal", mos.name),
                    mos.node_drain,
                    mos.node_source,
                    resistance,
                ));
            }

            if let Some((kf, af, ef)) = mos.flicker_noise_coefficients() {
                let id = mos.drain_current();
                if id.abs() > 1e-18 {
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", mos.name),
                        mos.node_drain,
                        mos.node_source,
                        kf,
                        af,
                        ef,
                        id,
                    ));
                }
            }
        }

        // JFET channel thermal noise, gate shot noise, and flicker noise.
        for jfet in &circuit.jfets {
            let vd = Self::noise_node_voltage(dc_solution, jfet.drain);
            let vg = Self::noise_node_voltage(dc_solution, jfet.gate);
            let vs = Self::noise_node_voltage(dc_solution, jfet.source);
            let vgs = vg - vs;
            let vds = vd - vs;
            let vgd = vg - vd;
            let temp = jfet.params.tnom;
            let (ids, gm, _) = jfet.calculate(vgs, vds, temp);
            if gm.abs() > 1e-18 {
                let resistance = 1.0 / ((2.0 / 3.0) * gm.abs()).max(1e-30);
                noise_sources.push(NoiseSource::thermal(
                    format!("{}:thermal", jfet.name),
                    jfet.drain,
                    jfet.source,
                    resistance,
                ));
            }

            let (igs, igd) = jfet.gate_current(vgs, vgd, temp);
            if igs.abs() > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IGS", jfet.name),
                    jfet.gate,
                    jfet.source,
                    igs,
                ));
            }
            if igd.abs() > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IGD", jfet.name),
                    jfet.gate,
                    jfet.drain,
                    igd,
                ));
            }

            if let Some((kf, af, ef)) = jfet.flicker_noise_coefficients()
                && ids.abs() > 1e-18
            {
                noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", jfet.name),
                    jfet.drain,
                    jfet.source,
                    kf,
                    af,
                    ef,
                    ids,
                ));
            }
        }

        noise_sources
    }

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

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point for bias-dependent noise.
        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        let noise_sources = Self::collect_noise_sources(&circuit, &dc_solution);

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

        let results: Result<Vec<NoiseResult>, SimulationError> = frequencies
            .iter()
            .map(|&freq| {
                let omega = 2.0 * PI * freq;
                let ac_matrix =
                    Self::build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega);

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

                    let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let gain = Self::differential_noise_output(
                        &solution, output_pos, output_neg, num_nodes,
                    );
                    gain * gain
                } else {
                    1.0
                };

                if input_excitation.is_some() && (!input_gain_sq.is_finite() || input_gain_sq <= 1e-30)
                {
                    return Err(SimulationError::Circuit(format!(
                        "Input-referred noise is undefined for source '{}' at {} Hz because the small-signal transfer to the selected output is zero or non-finite",
                        input_source.unwrap_or("<unknown>"),
                        freq
                    )));
                }

                let mut total_noise_v2_hz = 0.0;
                let mut contributions = Vec::new();

                for source in &noise_sources {
                    let si = source.spectral_density(freq, temperature);
                    if !si.is_finite() || si <= 0.0 {
                        continue;
                    }

                    let mut rhs = vec![Complex64::new(0.0, 0.0); size];
                    if source.node_pos > 0 && source.node_pos <= num_nodes {
                        rhs[source.node_pos - 1] += Complex64::new(1.0, 0.0);
                    }
                    if source.node_neg > 0 && source.node_neg <= num_nodes {
                        rhs[source.node_neg - 1] -= Complex64::new(1.0, 0.0);
                    }

                    let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let v_out = Self::differential_noise_output(
                        &solution, output_pos, output_neg, num_nodes,
                    );
                    let output_v2 = si * v_out * v_out;
                    if output_v2.is_finite() && output_v2 > 0.0 {
                        total_noise_v2_hz += output_v2;
                        contributions.push(NoiseContribution {
                            device_name: source.device_name.clone(),
                            noise_type: source.noise_type,
                            output_contribution: output_v2,
                            percentage: 0.0,
                        });
                    }
                }

                for contrib in &mut contributions {
                    contrib.percentage = if total_noise_v2_hz > 0.0 {
                        100.0 * contrib.output_contribution / total_noise_v2_hz
                    } else {
                        0.0
                    };
                }

                Ok(NoiseResult {
                    frequency: freq,
                    output_noise_density: total_noise_v2_hz,
                    input_referred_density: if input_excitation.is_some() {
                        total_noise_v2_hz / input_gain_sq
                    } else {
                        total_noise_v2_hz
                    },
                    contributions,
                })
            })
            .collect();

        results
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

        for node_id in 1..=max_node_id {
            let samples: Vec<Value> = results
                .iter()
                .filter_map(|r| r.get(node_id).copied())
                .collect();

            if !samples.is_empty() {
                let numeric_name = format!("V({})", node_id);
                let numeric_label = numeric_name.clone();
                let stats = VariableStatistics::from_samples(&numeric_name, samples.clone(), 20);
                variables.insert(numeric_name, stats);

                if let Some(node_names) = &first_node_names
                    && let Some(node_name) = node_names.get(node_id)
                {
                    let named_key = format!("V({})", node_name);
                    if named_key != numeric_label {
                        let alias_stats = VariableStatistics::from_samples(&named_key, samples, 20);
                        variables.insert(named_key, alias_stats);
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
        let mut circuit = self.build_circuit(netlist)?;
        let num_nodes = circuit.num_nodes();

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

        if !circuit.tlines.is_empty() {
            return Err(SimulationError::Circuit(
                "Pole-zero analysis does not yet support transmission lines".to_string(),
            ));
        }

        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution = self.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }

        // Reuse the AC linearization path so pole-zero analysis sees the same
        // nonlinear small-signal conductances and capacitances as AC analysis.
        let g_descriptor = Self::build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 0.0);
        let c_descriptor = Self::build_small_signal_pz_matrix(&circuit, &matrix, &dc_solution, 1.0);
        let mut g_matrix = Matrix::from_dense(g_descriptor.to_dense_real());
        let mut c_matrix = Matrix::from_dense(c_descriptor.to_dense_imag());
        Self::stamp_vbic_pz_descriptor_states(&circuit, &dc_solution, &mut g_matrix, &mut c_matrix);

        let input_neg_node = input_neg.unwrap_or(0);
        let matches_input_voltage_port = |np: usize, nn: usize| {
            !input_is_current
                && ((np == input_pos && nn == input_neg_node)
                    || (nn == input_pos && np == input_neg_node))
        };
        let mut input_voltage_branch = None;
        let mut input_voltage_gain = 1.0;

        // Stamp independent voltage sources into G (MNA branch equations).
        // If a deck already contains an ideal source on the requested voltage
        // input port, use that branch directly as the excitation variable
        // instead of synthesizing a parallel source later.
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal) - 1;

            if matches_input_voltage_port(np, nn) {
                if input_voltage_branch.replace(br).is_some() {
                    return Err(SimulationError::Circuit(
                        "Multiple independent voltage sources drive the requested PZ input port"
                            .to_string(),
                    ));
                }
                input_voltage_gain = if np == input_pos && nn == input_neg_node {
                    1.0
                } else {
                    -1.0
                };
            }
        }

        // Create analyzer and run
        let analyzer = PoleZeroAnalyzer::new(g_matrix, c_matrix);
        let mut config = PoleZeroConfig::poles_and_zeros(input_pos - 1, output_pos - 1);
        config.input_neg = input_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.output_neg = output_neg.and_then(|n| if n == 0 { None } else { Some(n - 1) });
        config.input_is_current = input_is_current;
        config.input_voltage_branch = input_voltage_branch;
        config.input_voltage_gain = input_voltage_gain;
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

        let mut reparsed = if let Some(source_path) = netlist.source_path.as_deref() {
            Netlist::parse_with_path(&overridden_source, source_path)
        } else {
            crate::netlist::parse_netlist(&overridden_source)
        }
        .map_err(|e| {
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
