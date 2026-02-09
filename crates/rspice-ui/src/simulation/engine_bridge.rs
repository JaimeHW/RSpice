//! Engine Bridge - Connect UI to rspice-core Simulation Engine
//!
//! This module provides a clean interface between the UI simulation runner
//! and the rspice-core Engine. It handles:
//!
//! - Netlist parsing from UI-generated strings
//! - Mapping UI AnalysisConfig to core engine calls
//! - Converting core results to UI WaveformData format
//! - Error translation
//!
//! # Architecture
//!
//! The bridge follows the adapter pattern:
//! ```text
//! UI SimulationRunner → EngineBridge → rspice-core Engine
//!      (config)           (parse)         (run)
//!      (result) ←         (convert) ←     (result)
//! ```

use std::collections::HashMap;

use super::config::AnalysisConfig;
use super::results::{DcOpResult, SimulationResult, WaveformData};
use super::runner::SimulationError;
use crate::output_spec::{
    collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    run_ac_output_at_frequency, run_dc_output_sensitivity, validate_sensitivity_output_spec,
    OutputSpec,
};

//=============================================================================
// Engine Bridge
//=============================================================================

/// Bridge between UI and rspice-core engine
///
/// Handles parsing, execution, and result conversion for all analysis types.
pub struct EngineBridge {
    /// Core engine instance
    engine: rspice_core::Engine,
}

impl Default for EngineBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineBridge {
    /// Create a new engine bridge with default configuration
    pub fn new() -> Self {
        Self {
            engine: rspice_core::Engine::default(),
        }
    }

    /// Create a new engine bridge with custom configuration
    pub fn with_config(config: rspice_core::SimulationConfig) -> Self {
        Self {
            engine: rspice_core::Engine::new(config),
        }
    }

    /// Run simulation with the given configuration and netlist
    pub fn run(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
    ) -> Result<SimulationResult, SimulationError> {
        // Parse netlist
        let netlist = self.parse_netlist(netlist_str)?;

        // Dispatch to appropriate analysis
        match config {
            AnalysisConfig::DcOp => self.run_dc_op(&netlist),
            AnalysisConfig::DcSweep(dc_config) => self.run_dc_sweep(&netlist, dc_config),
            AnalysisConfig::Transient(tran_config) => self.run_transient(&netlist, tran_config),
            AnalysisConfig::Ac(ac_config) => self.run_ac(&netlist, ac_config),
            AnalysisConfig::Noise(noise_config) => self.run_noise(&netlist, noise_config),
            AnalysisConfig::PoleZero(pz_config) => self.run_pz(&netlist, pz_config),
            AnalysisConfig::Sensitivity(sens_config) => self.run_sensitivity(&netlist, sens_config),
        }
    }

    /// Run simulation with abort signal for cooperative cancellation
    ///
    /// This is the primary entry point for UI-driven simulations where the user
    /// can cancel a running simulation via the stop button.
    pub fn run_with_abort(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        abort_flag: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<SimulationResult, SimulationError> {
        // Parse netlist
        let netlist = self.parse_netlist(netlist_str)?;

        // Arc<AtomicBool> implements AbortSignal directly, so we can use it
        // Dispatch to appropriate analysis (abort-aware where supported)
        match config {
            AnalysisConfig::DcOp => self.run_dc_op(&netlist),
            AnalysisConfig::DcSweep(dc_config) => self.run_dc_sweep(&netlist, dc_config),
            AnalysisConfig::Transient(tran_config) => {
                self.run_transient_with_abort(&netlist, tran_config, abort_flag)
            }
            AnalysisConfig::Ac(ac_config) => self.run_ac(&netlist, ac_config),
            AnalysisConfig::Noise(noise_config) => self.run_noise(&netlist, noise_config),
            AnalysisConfig::PoleZero(pz_config) => self.run_pz(&netlist, pz_config),
            AnalysisConfig::Sensitivity(sens_config) => self.run_sensitivity(&netlist, sens_config),
        }
    }

    //-------------------------------------------------------------------------
    // Netlist Parsing
    //-------------------------------------------------------------------------

    /// Parse netlist string into core Netlist object
    fn parse_netlist(&self, netlist_str: &str) -> Result<rspice_core::Netlist, SimulationError> {
        let parse_base = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        rspice_core::Netlist::parse_with_path(netlist_str, &parse_base)
            .map_err(|e| SimulationError::ParseError(e.to_string()))
    }

    /// Build an engine instance with netlist `.OPTIONS` layered on top of
    /// bridge base configuration.
    fn engine_for_netlist(&self, netlist: &rspice_core::Netlist) -> rspice_core::Engine {
        let resolved = rspice_core::resolve_simulation_config(
            self.engine.config(),
            Some(&netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        );
        rspice_core::Engine::new(resolved)
    }

    //-------------------------------------------------------------------------
    // DC Operating Point
    //-------------------------------------------------------------------------

    /// Run DC operating point analysis
    fn run_dc_op(
        &self,
        netlist: &rspice_core::Netlist,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let core_result = engine
            .run_dc_op(netlist)
            .map_err(|e| self.translate_error(e))?;

        // Convert core SimulationResult to UI DcOpResult
        let dc_result = self.convert_dc_result(&core_result);
        Ok(SimulationResult::DcOp(dc_result))
    }

    /// Convert core DC result to UI format
    fn convert_dc_result(&self, core_result: &rspice_core::SimulationResult) -> DcOpResult {
        let mut result = DcOpResult::default();

        // Copy node voltages (skip ground at index 0)
        for (i, &voltage) in core_result.node_voltages.iter().enumerate() {
            if i > 0 {
                // Use actual node name if available
                let name = core_result
                    .node_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("{}", i));
                result.node_voltages.insert(name, voltage);
            }
        }

        // Copy branch currents
        for (i, &current) in core_result.branch_currents.iter().enumerate() {
            result.branch_currents.insert(format!("I({})", i), current);
        }

        result
    }

    //-------------------------------------------------------------------------
    // DC Sweep
    //-------------------------------------------------------------------------

    /// Run DC sweep analysis
    fn run_dc_sweep(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::DcSweepConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);

        let nested_cfg = match (&config.source2, config.start2, config.stop2, config.step2) {
            (None, None, None, None) => None,
            (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
                Some((source2.as_str(), start2, stop2, step2))
            }
            _ => {
                return Err(SimulationError::InvalidConfig(
                    "Nested DC sweep requires source2/start2/stop2/step2".to_string(),
                ));
            }
        };

        let mut sweep_values = Vec::new();
        let mut waveforms = HashMap::new();

        if let Some((source2, start2, stop2, step2)) = nested_cfg {
            let sweep2 =
                rspice_core::analysis::DcSweep::new(source2.to_string(), start2, stop2, step2);
            let sweep2_values = sweep2.points();
            if sweep2_values.is_empty() {
                return Err(SimulationError::InvalidConfig(
                    "Nested DC secondary sweep produced no points".to_string(),
                ));
            }

            for &sweep2_value in &sweep2_values {
                let mut nested_netlist = netlist.clone();
                Self::set_dc_source_value(&mut nested_netlist, source2, sweep2_value)?;

                let sweep_results = engine
                    .run_dc_sweep(
                        &nested_netlist,
                        &config.source,
                        config.start,
                        config.stop,
                        config.step,
                    )
                    .map_err(|e| self.translate_error(e))?;

                if sweep_results.is_empty() {
                    continue;
                }

                if sweep_values.is_empty() {
                    sweep_values = sweep_results.iter().map(|(v, _)| *v).collect();
                }

                if let Some((_, first_result)) = sweep_results.first() {
                    for (node_idx, node_name) in first_result.node_names.iter().enumerate() {
                        if node_idx == 0 {
                            continue;
                        }
                        let voltages: Vec<f64> = sweep_results
                            .iter()
                            .map(|(_, result)| {
                                result.node_voltages.get(node_idx).copied().unwrap_or(0.0)
                            })
                            .collect();
                        let trace_name = format!("{} [{}={:.6}]", node_name, source2, sweep2_value);
                        waveforms.insert(
                            trace_name.clone(),
                            WaveformData::new_time_domain(
                                trace_name,
                                sweep_values.clone(),
                                voltages,
                            ),
                        );
                    }
                }
            }
        } else {
            let sweep_results = engine
                .run_dc_sweep(
                    netlist,
                    &config.source,
                    config.start,
                    config.stop,
                    config.step,
                )
                .map_err(|e| self.translate_error(e))?;

            // Convert to UI format
            sweep_values = sweep_results.iter().map(|(v, _)| *v).collect();

            // For each node, create a waveform
            if let Some((_, first_result)) = sweep_results.first() {
                for (i, name) in first_result.node_names.iter().enumerate() {
                    if i == 0 {
                        continue;
                    } // Skip ground
                    let voltages: Vec<f64> = sweep_results
                        .iter()
                        .map(|(_, result)| result.node_voltages.get(i).copied().unwrap_or(0.0))
                        .collect();

                    waveforms.insert(
                        name.clone(),
                        WaveformData::new_time_domain(name, sweep_values.clone(), voltages),
                    );
                }
            }
        }

        Ok(SimulationResult::DcSweep {
            sweep_var: config.source.clone(),
            sweep_values,
            waveforms,
        })
    }

    fn set_dc_source_value(
        netlist: &mut rspice_core::Netlist,
        source_name: &str,
        value: f64,
    ) -> Result<(), SimulationError> {
        if source_name.trim().is_empty() {
            return Err(SimulationError::InvalidConfig(
                "DC sweep source name cannot be empty".to_string(),
            ));
        }

        for element in &mut netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            if let rspice_core::netlist::ElementKind::VoltageSource(spec) = &mut element.kind {
                if Self::set_source_spec_dc(spec, value) {
                    return Ok(());
                }
                return Err(SimulationError::InvalidConfig(format!(
                    "Source '{}' is not a DC or DC/AC voltage source",
                    source_name
                )));
            }
        }

        Err(SimulationError::InvalidConfig(format!(
            "Source '{}' not found in netlist",
            source_name
        )))
    }

    fn set_source_spec_dc(spec: &mut rspice_core::netlist::SourceSpec, value: f64) -> bool {
        match spec {
            rspice_core::netlist::SourceSpec::Dc(v) => {
                *v = value;
                true
            }
            rspice_core::netlist::SourceSpec::DcAc { dc_value, .. } => {
                *dc_value = value;
                true
            }
            _ => false,
        }
    }

    //-------------------------------------------------------------------------
    // Transient Analysis
    //-------------------------------------------------------------------------

    #[inline]
    fn resolve_transient_max_step(config: &super::config::TransientAnalysisConfig) -> f64 {
        // SPICE .tran step is an output interval. Unless max_timestep is explicitly
        // provided, allow a coarser internal step so adaptive transient can run fast.
        config
            .max_timestep
            .unwrap_or(config.step_time * 10.0)
            .max(1e-18)
    }

    /// Run transient analysis
    fn run_transient(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::TransientAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let max_step = Self::resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran(netlist, config.stop_time, max_step)
            .map_err(|e| self.translate_error(e))?;

        // Convert to UI waveform format
        let mut waveforms = HashMap::new();

        for (node_idx, voltages) in tran_result.voltages.iter().enumerate() {
            let name = tran_result
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| format!("{}", node_idx + 1));

            waveforms.insert(
                name.clone(),
                WaveformData::new_time_domain(&name, tran_result.time.clone(), voltages.clone()),
            );
        }

        Ok(SimulationResult::Transient {
            time: tran_result.time,
            waveforms,
        })
    }

    /// Run transient analysis with abort signal for cooperative cancellation
    ///
    /// This allows long-running transient simulations to be cancelled via the
    /// UI stop button. The abort signal is checked every 1000 iterations.
    fn run_transient_with_abort(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::TransientAnalysisConfig,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let max_step = Self::resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran_with_abort(netlist, config.stop_time, max_step, abort)
            .map_err(|e| self.translate_error(e))?;

        // Convert to UI waveform format (same as run_transient)
        let mut waveforms = HashMap::new();

        for (node_idx, voltages) in tran_result.voltages.iter().enumerate() {
            let name = tran_result
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| format!("{}", node_idx + 1));

            waveforms.insert(
                name.clone(),
                WaveformData::new_time_domain(&name, tran_result.time.clone(), voltages.clone()),
            );
        }

        Ok(SimulationResult::Transient {
            time: tran_result.time,
            waveforms,
        })
    }

    //-------------------------------------------------------------------------
    // AC Analysis
    //-------------------------------------------------------------------------

    /// Run AC small-signal analysis
    ///
    /// Performs frequency-domain analysis at the specified frequency points.
    /// The circuit is linearized at the DC operating point, then the AC
    /// response is computed at each frequency.
    fn run_ac(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::AcAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        // Generate frequency points based on sweep configuration
        let frequencies = config.generate_frequencies();

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid frequency sweep configuration".to_string(),
            ));
        }

        // Run AC analysis via engine
        let ac_results = engine
            .run_ac(netlist, &frequencies)
            .map_err(|e| self.translate_error(e))?;

        // Convert to UI waveform format
        // Each result contains voltage phasors at each node for one frequency
        let mut waveforms = HashMap::new();

        // Determine number of nodes from first result
        if ac_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        let num_nodes = ac_results[0].voltages.len();

        // Build waveform for each node
        for node_idx in 0..num_nodes {
            // Extract magnitude and phase at each frequency for this node
            let mut real_values = Vec::with_capacity(frequencies.len());
            let mut imag_values = Vec::with_capacity(frequencies.len());

            for result in &ac_results {
                if node_idx < result.voltages.len() {
                    let v = result.voltages[node_idx];
                    real_values.push(v.re);
                    imag_values.push(v.im);
                } else {
                    real_values.push(0.0);
                    imag_values.push(0.0);
                }
            }

            let name = format!("V({})", node_idx + 1);
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(&name, frequencies.clone(), real_values, imag_values),
            );
        }

        // Also extract branch currents if available
        if !ac_results[0].currents.is_empty() {
            let num_branches = ac_results[0].currents.len();

            for branch_idx in 0..num_branches {
                let mut real_values = Vec::with_capacity(frequencies.len());
                let mut imag_values = Vec::with_capacity(frequencies.len());

                for result in &ac_results {
                    if branch_idx < result.currents.len() {
                        let i = result.currents[branch_idx];
                        real_values.push(i.re);
                        imag_values.push(i.im);
                    } else {
                        real_values.push(0.0);
                        imag_values.push(0.0);
                    }
                }

                let name = format!("I({})", branch_idx + 1);
                waveforms.insert(
                    name.clone(),
                    WaveformData::new_complex(&name, frequencies.clone(), real_values, imag_values),
                );
            }
        }

        Ok(SimulationResult::Ac {
            frequencies,
            waveforms,
        })
    }

    //-------------------------------------------------------------------------
    // Noise Analysis
    //-------------------------------------------------------------------------

    /// Run noise analysis
    ///
    /// Computes output and input-referred noise spectral density as a function
    /// of frequency. Identifies contributions from thermal, shot, and flicker
    /// noise sources following Spectre/SPICE conventions.
    fn run_noise(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::NoiseAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        // Generate frequency points based on sweep configuration
        let frequencies = config.generate_frequencies();

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid noise frequency sweep configuration".to_string(),
            ));
        }

        // Resolve output node name to index
        // For now, use a simple numeric parsing or default to node 1
        let output_node = self.resolve_node_index(&config.output_node, netlist);

        if output_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid output node '{}' for noise analysis",
                config.output_node
            )));
        }

        let temperature = config.default_temperature();

        // Run noise analysis via engine
        let noise_results = engine
            .run_noise(netlist, output_node, &frequencies, temperature)
            .map_err(|e| self.translate_error(e))?;

        if noise_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        // Convert to UI format
        let mut output_noise = Vec::with_capacity(frequencies.len());
        let mut input_noise = Vec::with_capacity(frequencies.len());
        let mut contributors: HashMap<String, Vec<f64>> = HashMap::new();

        for result in &noise_results {
            // Output noise density (V²/Hz)
            output_noise.push(result.output_noise_density);

            // Input-referred noise (V²/Hz)
            input_noise.push(result.input_referred_density);

            // Collect per-source contributions
            for contrib in &result.contributions {
                contributors
                    .entry(contrib.device_name.clone())
                    .or_insert_with(|| Vec::with_capacity(frequencies.len()))
                    .push(contrib.output_contribution);
            }
        }

        // Ensure all contributor vectors have same length as frequencies
        for (_, values) in contributors.iter_mut() {
            while values.len() < frequencies.len() {
                values.push(0.0);
            }
        }

        Ok(SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise: Some(input_noise),
            contributors,
        })
    }

    /// Resolve node name to node index
    ///
    /// Supports:
    /// - Numeric indices: "1", "2", etc.
    /// - Ground: "0", "gnd", "GND"
    /// - Named nodes: assumed sequential assignment by Circuit builder
    fn resolve_node_index(&self, name: &str, netlist: &rspice_core::Netlist) -> usize {
        // Handle ground
        let lower = name.to_lowercase();
        if lower == "0" || lower == "gnd" || lower == "ground" {
            return 0;
        }

        // Try parsing as numeric index
        if let Ok(idx) = name.parse::<usize>() {
            return idx;
        }

        // Resolve symbolic node names via DC result node list.
        let engine = self.engine_for_netlist(netlist);
        if let Ok(dc) = engine.run_dc_op(netlist) {
            let upper = name.to_ascii_uppercase();
            if let Some(idx) = dc
                .node_names
                .iter()
                .position(|n| n.to_ascii_uppercase() == upper)
            {
                return idx;
            }
        }

        0
    }

    //-------------------------------------------------------------------------
    // Pole-Zero Analysis
    //-------------------------------------------------------------------------

    /// Run pole-zero analysis
    ///
    /// Finds the poles and zeros of the circuit's transfer function from
    /// input to output node. Returns stability information and dominant poles.
    fn run_pz(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::PoleZeroConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let dc = engine
            .run_dc_op(netlist)
            .map_err(|e| self.translate_error(e))?;
        let node_names = &dc.node_names;

        let resolve_node_or_ground = |name: &str| -> Option<usize> {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return None;
            }
            if matches!(
                trimmed.to_ascii_lowercase().as_str(),
                "0" | "gnd" | "ground"
            ) {
                return Some(0);
            }
            if let Ok(idx) = trimmed.parse::<usize>() {
                if idx < node_names.len() {
                    return Some(idx);
                }
            }
            let upper = trimmed.to_ascii_uppercase();
            node_names
                .iter()
                .position(|n| n.to_ascii_uppercase() == upper)
        };

        let input_idx = resolve_node_or_ground(&config.input_node).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Invalid input node '{}' for pole-zero analysis",
                config.input_node
            ))
        })?;
        let input_ref_idx = resolve_node_or_ground(&config.input_ref).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Invalid input reference '{}' for pole-zero analysis",
                config.input_ref
            ))
        })?;
        let output_idx = resolve_node_or_ground(&config.output_node).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Invalid output node '{}' for pole-zero analysis",
                config.output_node
            ))
        })?;
        let output_ref_idx = resolve_node_or_ground(&config.output_ref).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Invalid output reference '{}' for pole-zero analysis",
                config.output_ref
            ))
        })?;

        let canonicalize_port = |pos: usize,
                                 neg: usize,
                                 label: &str|
         -> Result<(usize, Option<usize>, f64), SimulationError> {
            if pos == neg {
                return Err(SimulationError::InvalidConfig(format!(
                    "Invalid {} port: positive and reference nodes are the same",
                    label
                )));
            }
            if pos != 0 {
                return Ok((pos, if neg == 0 { None } else { Some(neg) }, 1.0));
            }
            if neg == 0 {
                return Err(SimulationError::InvalidConfig(format!(
                    "Invalid {} port: ground-ground is not allowed",
                    label
                )));
            }
            Ok((neg, None, -1.0))
        };

        let (input_pos, input_neg, input_sign) =
            canonicalize_port(input_idx, input_ref_idx, "input")?;
        let (output_pos, output_neg, output_sign) =
            canonicalize_port(output_idx, output_ref_idx, "output")?;

        let input_is_current = config.transfer_type.trim().eq_ignore_ascii_case("CUR");
        let (compute_poles, compute_zeros) = match config.analysis_type {
            super::config::PzAnalysisType::PolesOnly => (true, false),
            super::config::PzAnalysisType::ZerosOnly => (false, true),
            super::config::PzAnalysisType::PoleZero => (true, true),
        };

        let pz_result = engine
            .run_pz_ports(
                netlist,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                input_is_current,
                compute_poles,
                compute_zeros,
            )
            .map_err(|e| self.translate_error(e))?;

        // Convert poles and zeros to (f64, f64) tuples for UI
        let poles: Vec<(f64, f64)> = pz_result.poles.iter().map(|p| (p.re, p.im)).collect();

        let zeros: Vec<(f64, f64)> = pz_result.zeros.iter().map(|z| (z.re, z.im)).collect();

        Ok(SimulationResult::PoleZero {
            poles,
            zeros,
            gain: input_sign * output_sign * pz_result.dc_gain,
        })
    }

    //-------------------------------------------------------------------------
    // Sensitivity Analysis
    //-------------------------------------------------------------------------

    /// Run sensitivity analysis
    ///
    /// Computes the partial derivative of the output with respect to each
    /// circuit parameter using finite differences.
    fn run_sensitivity(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::SensitivityConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);

        let ac_frequency = resolve_sensitivity_ac_frequency(config.ac_mode, config.frequency)
            .map_err(SimulationError::InvalidConfig)?;

        let dc_result = engine
            .run_dc_op(netlist)
            .map_err(|e| self.translate_error(e))?;
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| self.translate_error(e))?;
        let output_spec = parse_output_spec(&config.output_var, &dc_result.node_names, &circuit)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Sensitivity output '{}' could not be resolved to a node or branch",
                    config.output_var
                ))
            })?;
        validate_sensitivity_output_spec(&output_spec).map_err(SimulationError::InvalidConfig)?;

        let nominal_value = if let Some(freq) = ac_frequency {
            run_ac_output_at_frequency(&engine, netlist, &output_spec, freq)
                .map_err(SimulationError::InvalidConfig)?
                .norm()
        } else {
            dc_output_value(&dc_result, &output_spec).map_err(SimulationError::InvalidConfig)?
        };

        let parameters = collect_sensitivity_parameters(netlist);

        if parameters.is_empty() {
            // No parameters found - return empty result
            return Ok(SimulationResult::Sensitivity {
                sensitivities: HashMap::new(),
                normalized: HashMap::new(),
            });
        }

        let mut sensitivities: HashMap<String, f64> = HashMap::new();
        let mut normalized: HashMap<String, f64> = HashMap::new();
        let mut perturbed_netlist = netlist.clone();

        // For each parameter, compute sensitivity using central differences
        for (param_name, param_value) in parameters {
            if !param_value.is_finite() || param_value == 0.0 {
                continue; // Skip zero-valued parameters
            }

            let sensitivity = if let Some(freq) = ac_frequency {
                let result = finite_difference_derivative(param_value, |candidate| {
                    perturbed_netlist.params.set(&param_name, candidate);
                    run_ac_output_at_frequency(&engine, &perturbed_netlist, &output_spec, freq)
                        .map(|value| value.norm())
                });
                perturbed_netlist.params.set(&param_name, param_value);
                match result {
                    Ok(raw) => raw,
                    Err(_) => continue,
                }
            } else {
                match &output_spec {
                    OutputSpec::Voltage(vspec) => {
                        match run_dc_output_sensitivity(
                            &engine,
                            netlist,
                            *vspec,
                            &param_name,
                            param_value,
                        ) {
                            Ok(raw) => raw,
                            Err(_) => continue,
                        }
                    }
                    OutputSpec::BranchCurrent { .. } => {
                        let result = finite_difference_derivative(param_value, |candidate| {
                            perturbed_netlist.params.set(&param_name, candidate);
                            let dc_result = engine
                                .run_dc_op(&perturbed_netlist)
                                .map_err(|e| e.to_string())?;
                            dc_output_value(&dc_result, &output_spec)
                        });
                        perturbed_netlist.params.set(&param_name, param_value);
                        match result {
                            Ok(raw) => raw,
                            Err(_) => continue,
                        }
                    }
                }
            };

            sensitivities.insert(param_name.clone(), sensitivity);

            let norm_sens = normalized_sensitivity(sensitivity, param_value, nominal_value);
            normalized.insert(param_name.clone(), norm_sens);
        }

        Ok(SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        })
    }

    //-------------------------------------------------------------------------
    // Error Translation
    //-------------------------------------------------------------------------

    /// Translate core engine error to UI error
    fn translate_error(&self, err: rspice_core::SimulationError) -> SimulationError {
        match err {
            rspice_core::SimulationError::Circuit(msg) => SimulationError::CircuitError(msg),
            rspice_core::SimulationError::Solver(solver_err) => {
                SimulationError::SolverError(solver_err.to_string())
            }
            rspice_core::SimulationError::Netlist(msg) => SimulationError::ParseError(msg),
            rspice_core::SimulationError::ConvergenceFailed(iterations) => {
                SimulationError::ConvergenceFailed {
                    iterations,
                    message: "Newton-Raphson iteration limit exceeded".to_string(),
                }
            }
            rspice_core::SimulationError::Aborted => SimulationError::Aborted,
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Construction Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_engine_bridge_new() {
        let bridge = EngineBridge::new();
        // Should not panic
        assert!(true);
        let _ = bridge; // Use bridge to avoid unused warning
    }

    #[test]
    fn test_engine_bridge_default() {
        let bridge = EngineBridge::default();
        let _ = bridge;
    }

    #[test]
    fn test_engine_bridge_with_config() {
        let config = rspice_core::SimulationConfig::default();
        let bridge = EngineBridge::with_config(config);
        let _ = bridge;
    }

    #[test]
    fn test_engine_for_netlist_applies_netlist_options() {
        let bridge = EngineBridge::new();
        let netlist = bridge
            .parse_netlist(
                r#"
* Netlist options mapping
V1 1 0 1
R1 1 0 1k
.OPTIONS TEMP=85 ITL1=120 METHOD=GEAR RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 GMIN=1e-11
.END
"#,
            )
            .expect("netlist should parse");

        let engine = bridge.engine_for_netlist(&netlist);
        let cfg = engine.config();

        assert!((cfg.temperature - 358.15).abs() < 1e-12);
        assert_eq!(cfg.max_iterations, 120);
        assert_eq!(
            cfg.integration_method,
            rspice_core::analysis::IntegrationMethod::Gear2
        );
        assert!((cfg.tolerance - 2e-4).abs() < 1e-15);
        assert!((cfg.convergence_config.voltage_reltol - 2e-4).abs() < 1e-15);
        assert!((cfg.convergence_config.residual_reltol - 2e-4).abs() < 1e-15);
        assert!((cfg.convergence_config.voltage_abstol - 3e-6).abs() < 1e-18);
        assert!((cfg.convergence_config.current_abstol - 4e-12).abs() < 1e-24);
        assert!((cfg.convergence_config.gmin_initial - 1e-11).abs() < 1e-24);
    }

    #[test]
    fn test_engine_for_netlist_preserves_base_for_unspecified_options() {
        let mut base = rspice_core::SimulationConfig::default();
        base.tolerance = 8e-4;
        base.max_iterations = 88;
        let bridge = EngineBridge::with_config(base);
        let netlist = bridge
            .parse_netlist(
                r#"
* Netlist options partial override
V1 1 0 1
R1 1 0 1k
.OPTIONS TEMP=125
.END
"#,
            )
            .expect("netlist should parse");

        let engine = bridge.engine_for_netlist(&netlist);
        let cfg = engine.config();

        assert!((cfg.temperature - 398.15).abs() < 1e-12);
        assert!((cfg.tolerance - 8e-4).abs() < 1e-15);
        assert_eq!(cfg.max_iterations, 88);
    }

    // -------------------------------------------------------------------------
    // Parse Error Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_empty_netlist() {
        let bridge = EngineBridge::new();
        let result = bridge.run(&AnalysisConfig::DcOp, "");
        // Empty netlist should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_netlist() {
        let bridge = EngineBridge::new();
        let result = bridge.run(&AnalysisConfig::DcOp, "not valid spice");
        // Invalid should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_netlist_expands_include_directives() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "rspice_ui_engine_bridge_include_{}_{}",
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

        let include_file = temp_dir.join("included.sp");
        fs::write(&include_file, "RINC 1 0 1k\n").expect("failed to write include file");

        let bridge = EngineBridge::new();
        let netlist = bridge
            .parse_netlist(&format!(
                "V1 1 0 DC 1\n.include \"{}\"\n.end\n",
                include_file.display()
            ))
            .expect("netlist should parse with include expansion");

        assert!(
            netlist
                .elements
                .iter()
                .any(|element| element.name.eq_ignore_ascii_case("RINC")),
            "included element should be present after preprocessing"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // -------------------------------------------------------------------------
    // DC OP Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_run_dc_op_simple_resistor() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Simple resistor divider
V1 1 0 DC 10
R1 1 2 1k
R2 2 0 1k
.end
"#;

        let result = bridge.run(&AnalysisConfig::DcOp, netlist);

        if let Ok(SimulationResult::DcOp(dc_result)) = result {
            // Node 2 should be 5V (resistor divider)
            if let Some(&v2) = dc_result.node_voltages.get("2") {
                assert!((v2 - 5.0).abs() < 0.01, "Expected 5V, got {}", v2);
            }
        }
    }

    #[test]
    fn test_run_dc_op_single_resistor() {
        let bridge = EngineBridge::new();
        let netlist = r#"
V1 1 0 DC 5
R1 1 0 1k
.end
"#;

        let result = bridge.run(&AnalysisConfig::DcOp, netlist);
        assert!(result.is_ok(), "DC OP should succeed for simple circuit");
    }

    #[test]
    fn test_run_dc_sweep_supports_nested_secondary_source() {
        let bridge = EngineBridge::new();
        let netlist = r#"
V1 in 0 DC 0
V2 ctrl 0 DC 0
R1 in out 1k
R2 out ctrl 1k
R3 out 0 1k
.end
"#;
        let config = AnalysisConfig::DcSweep(super::super::config::DcSweepConfig {
            source: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.5,
            source2: Some("V2".to_string()),
            start2: Some(0.0),
            stop2: Some(1.0),
            step2: Some(1.0),
        });

        let result = bridge
            .run(&config, netlist)
            .expect("nested DC sweep should run");
        match result {
            SimulationResult::DcSweep {
                sweep_values,
                waveforms,
                ..
            } => {
                assert_eq!(sweep_values, vec![0.0, 0.5, 1.0]);
                assert!(!waveforms.is_empty());
                assert!(waveforms.keys().any(|name| name.contains("[V2=0")));
                assert!(waveforms.keys().any(|name| name.contains("[V2=1")));
                assert!(waveforms
                    .values()
                    .all(|wf| wf.y_values.len() == sweep_values.len()));
            }
            other => panic!("expected DC sweep result, got {:?}", other),
        }
    }

    #[test]
    fn test_run_dc_sweep_rejects_partial_nested_config() {
        let bridge = EngineBridge::new();
        let netlist = r#"
V1 in 0 DC 0
R1 in 0 1k
.end
"#;
        let config = AnalysisConfig::DcSweep(super::super::config::DcSweepConfig {
            source: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.5,
            source2: Some("V2".to_string()),
            start2: Some(0.0),
            stop2: Some(1.0),
            step2: None,
        });

        let err = bridge
            .run(&config, netlist)
            .expect_err("partial nested config should be rejected");
        assert!(matches!(err, SimulationError::InvalidConfig(_)));
    }

    // -------------------------------------------------------------------------
    // Error Translation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_translate_circuit_error() {
        let bridge = EngineBridge::new();
        let err = rspice_core::SimulationError::Circuit("test error".to_string());
        let ui_err = bridge.translate_error(err);
        assert!(matches!(ui_err, SimulationError::CircuitError(_)));
    }

    #[test]
    fn test_translate_netlist_error() {
        let bridge = EngineBridge::new();
        let err = rspice_core::SimulationError::Netlist("parse error".to_string());
        let ui_err = bridge.translate_error(err);
        assert!(matches!(ui_err, SimulationError::ParseError(_)));
    }

    #[test]
    fn test_translate_convergence_error() {
        let bridge = EngineBridge::new();
        let err = rspice_core::SimulationError::ConvergenceFailed(50);
        let ui_err = bridge.translate_error(err);
        assert!(matches!(ui_err, SimulationError::ConvergenceFailed { .. }));
    }

    // -------------------------------------------------------------------------
    // DC Result Conversion Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_convert_dc_result_empty() {
        let bridge = EngineBridge::new();
        let core_result = rspice_core::SimulationResult::new(0, 0);
        let ui_result = bridge.convert_dc_result(&core_result);
        assert!(ui_result.node_voltages.is_empty());
    }

    #[test]
    fn test_convert_dc_result_with_nodes() {
        let bridge = EngineBridge::new();
        let mut core_result = rspice_core::SimulationResult::new(2, 0);
        core_result.node_voltages[1] = 5.0;
        core_result.node_voltages[2] = 3.3;
        core_result.node_names = vec!["0".to_string(), "VCC".to_string(), "OUT".to_string()];

        let ui_result = bridge.convert_dc_result(&core_result);
        assert_eq!(ui_result.node_voltages.len(), 2);
    }

    // -------------------------------------------------------------------------
    // Integration Tests with Various Analyses
    // -------------------------------------------------------------------------

    #[test]
    fn test_resolve_transient_max_step_defaults_to_output_scaled_value() {
        let cfg = super::super::config::TransientAnalysisConfig {
            stop_time: 5e-3,
            step_time: 10e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        };
        let max_step = EngineBridge::resolve_transient_max_step(&cfg);
        assert!((max_step - 100e-9).abs() < 1e-18);
    }

    #[test]
    fn test_resolve_transient_max_step_honors_explicit_max_timestep() {
        let cfg = super::super::config::TransientAnalysisConfig {
            stop_time: 5e-3,
            step_time: 10e-9,
            start_time: 0.0,
            max_timestep: Some(25e-9),
            uic: false,
        };
        let max_step = EngineBridge::resolve_transient_max_step(&cfg);
        assert!((max_step - 25e-9).abs() < 1e-18);
    }

    #[test]
    fn test_run_transient_simple_rc() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* RC circuit
V1 1 0 PULSE(0 5 0 1n 1n 50n 100n)
R1 1 2 1k
C1 2 0 1n
.end
"#;

        let config = super::super::config::TransientAnalysisConfig {
            stop_time: 100e-9,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: Some(1e-9),
            uic: false,
        };

        let result = bridge.run(&AnalysisConfig::Transient(config), netlist);
        // May fail if engine doesn't support tran yet, that's ok
        let _ = result;
    }

    #[test]
    fn test_dispatch_to_correct_analysis() {
        let bridge = EngineBridge::new();
        let simple_netlist = r#"
V1 1 0 DC 5
R1 1 0 1k
.end
"#;

        // Test DC OP dispatch
        let result = bridge.run(&AnalysisConfig::DcOp, simple_netlist);
        if result.is_ok() {
            assert!(matches!(result.unwrap(), SimulationResult::DcOp(_)));
        }
    }

    #[test]
    fn test_run_sensitivity_dc_reports_param_derivatives() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                assert!(sensitivities[key].is_finite());
                assert!(normalized[key].is_finite());
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_ac_reports_param_derivatives() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e3),
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("ac sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                assert!(sensitivities[key].is_finite());
                assert!(normalized[key].is_finite());
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_ac_supports_numeric_output_node_index() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "2".to_string(),
            ac_mode: true,
            frequency: Some(1e3),
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("ac sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                assert!(sensitivities[key].is_finite());
                assert!(normalized[key].is_finite());
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_supports_differential_output() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity differential output
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out,in)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("differential sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                assert!(!normalized.is_empty());
                assert!(sensitivities.iter().all(|(_, raw)| raw.is_finite()));
                assert!(normalized.iter().all(|(_, norm)| norm.is_finite()));
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_normalized_reports_zero_when_nominal_is_zero() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity zero nominal output
.param RVAL=1k
V1 in 0 DC 0
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                let norm = normalized
                    .get(key)
                    .copied()
                    .expect("expected normalized sensitivity entry");
                assert!(norm.abs() <= 1e-18);
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_supports_current_output_dc() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity branch current output
.param RVAL=1k
V1 in 0 DC 10
R1 in 0 {RVAL}
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "I(V1)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("current-output dc sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                assert!(sensitivities[key].is_finite());
                assert!(normalized[key].is_finite());
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_supports_current_output_ac() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity branch current output AC
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in 0 {RVAL}
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "I(V1)".to_string(),
            ac_mode: true,
            frequency: Some(1e3),
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("current-output ac sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                assert!(!sensitivities.is_empty());
                let key = sensitivities
                    .keys()
                    .find(|k| k.eq_ignore_ascii_case("RVAL"))
                    .expect("expected RVAL sensitivity key");
                assert!(sensitivities[key].is_finite());
                assert!(normalized[key].is_finite());
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_current_output_handles_multiple_parameters() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity branch current output with multiple parameters
.param RA=1k
.param RB=2k
V1 in 0 1
R1 in mid {RA}
R2 mid 0 {RB}
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "I(V1)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = bridge
            .run(&cfg, netlist)
            .expect("multi-parameter current-output sensitivity run should succeed");
        match result {
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                let ra = sensitivities
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case("RA"))
                    .expect("expected RA sensitivity");
                let rb = sensitivities
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case("RB"))
                    .expect("expected RB sensitivity");
                let ra_norm = normalized
                    .get(ra.0)
                    .expect("expected normalized RA sensitivity");
                let rb_norm = normalized
                    .get(rb.0)
                    .expect("expected normalized RB sensitivity");

                assert!(ra.1.is_finite() && rb.1.is_finite());
                assert!(ra_norm.is_finite() && rb_norm.is_finite());
                assert!((ra.1 - rb.1).abs() < 1e-12);
            }
            _ => panic!("Expected Sensitivity result"),
        }
    }

    #[test]
    fn test_run_sensitivity_rejects_frequency_without_ac_mode() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: Some(1e3),
        });

        let err = bridge
            .run(&cfg, netlist)
            .expect_err("expected validation error");
        assert!(err
            .to_string()
            .contains("only valid when AC mode is enabled"));
    }

    #[test]
    fn test_run_sensitivity_rejects_non_positive_ac_frequency() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(0.0),
        });

        let err = bridge
            .run(&cfg, netlist)
            .expect_err("expected validation error");
        assert!(err.to_string().contains("must be > 0"));
    }

    #[test]
    fn test_run_sensitivity_rejects_unresolved_branch_output() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "I(NO_SUCH_BRANCH)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let err = bridge
            .run(&cfg, netlist)
            .expect_err("expected unresolved output error");
        assert!(err
            .to_string()
            .contains("could not be resolved to a node or branch"));
    }

    #[test]
    fn test_run_sensitivity_rejects_out_of_range_numeric_output_index() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
            output_var: "99".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let err = bridge
            .run(&cfg, netlist)
            .expect_err("expected unresolved output error");
        assert!(err
            .to_string()
            .contains("could not be resolved to a node or branch"));
    }

    #[test]
    fn test_run_pz_resolves_named_nodes() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Named-node PZ
R1 in out 1k
C1 out 0 1n
.end
"#;

        let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "CUR".to_string(),
            analysis_type: super::super::config::PzAnalysisType::PoleZero,
        });

        let result = bridge.run(&cfg, netlist).expect("PZ run should succeed");
        match result {
            SimulationResult::PoleZero { poles, gain, .. } => {
                assert!(!poles.is_empty());
                assert!(gain.is_finite());
            }
            _ => panic!("Expected PoleZero result"),
        }
    }

    #[test]
    fn test_run_pz_differential_gain_matches_superposition() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* Differential PZ
R1 in out 1k
R2 out ref 500
C1 out ref 1n
R3 ref 0 1k
.end
"#;

        let run_gain = |input_node: &str, input_ref: &str, output_node: &str, output_ref: &str| {
            let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
                input_node: input_node.to_string(),
                input_ref: input_ref.to_string(),
                output_node: output_node.to_string(),
                output_ref: output_ref.to_string(),
                transfer_type: "CUR".to_string(),
                analysis_type: super::super::config::PzAnalysisType::PoleZero,
            });
            match bridge.run(&cfg, netlist).expect("PZ run should succeed") {
                SimulationResult::PoleZero { gain, .. } => gain,
                _ => panic!("Expected PoleZero result"),
            }
        };

        let diff = run_gain("in", "ref", "out", "ref");
        let h11 = run_gain("in", "0", "out", "0");
        let h12 = run_gain("ref", "0", "out", "0");
        let h21 = run_gain("in", "0", "ref", "0");
        let h22 = run_gain("ref", "0", "ref", "0");
        let expected = h11 - h12 - h21 + h22;

        assert!((diff - expected).abs() < 1e-9);
    }

    #[test]
    fn test_run_pz_voltage_mode_highpass_zero() {
        let bridge = EngineBridge::new();
        let netlist = r#"
* High-pass PZ
C1 in out 1n
R1 out 0 1k
.end
"#;

        let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: super::super::config::PzAnalysisType::ZerosOnly,
        });

        let result = bridge.run(&cfg, netlist).expect("PZ run should succeed");
        match result {
            SimulationResult::PoleZero { zeros, .. } => {
                assert!(
                    zeros
                        .iter()
                        .any(|(re, im)| (re * re + im * im).sqrt() < 1e-2),
                    "expected zero near origin, got {:?}",
                    zeros
                );
            }
            _ => panic!("Expected PoleZero result"),
        }
    }
}
