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
use std::path::{Path, PathBuf};

use super::config::AnalysisConfig;
use super::results::{DcOpResult, SimulationResult, WaveformData};
use super::runner::SimulationError;
use crate::output_spec::{
    OutputSpec, collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    run_ac_output_at_frequency, run_dc_output_sensitivity, validate_sensitivity_output_spec,
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

struct SimulationInput<'a> {
    config: &'a AnalysisConfig,
    netlist_str: &'a str,
    source_path: Option<&'a Path>,
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
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            None,
        )
    }

    /// Run simulation with a source path used to resolve relative includes and
    /// model file references.
    pub fn run_with_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            None,
        )
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
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            Some(abort_flag),
        )
    }

    /// Run simulation with cooperative cancellation and a source path for
    /// relative include/model resolution.
    pub fn run_with_abort_and_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
        abort_flag: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            Some(abort_flag),
        )
    }

    fn run_request(
        &self,
        input: SimulationInput<'_>,
        abort_flag: Option<&dyn rspice_core::abort_signal::AbortSignal>,
    ) -> Result<SimulationResult, SimulationError> {
        let netlist = self.parse_netlist_with_source_path(input.netlist_str, input.source_path)?;
        self.dispatch_analysis(input.config, &netlist, abort_flag)
    }

    fn dispatch_analysis(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        abort_flag: Option<&dyn rspice_core::abort_signal::AbortSignal>,
    ) -> Result<SimulationResult, SimulationError> {
        match config {
            AnalysisConfig::DcOp => self.run_dc_op(netlist),
            AnalysisConfig::DcSweep(dc_config) => self.run_dc_sweep(netlist, dc_config),
            AnalysisConfig::Transient(tran_config) => {
                if let Some(abort) = abort_flag {
                    self.run_transient_with_abort(netlist, tran_config, abort)
                } else {
                    self.run_transient(netlist, tran_config)
                }
            }
            AnalysisConfig::Ac(ac_config) => self.run_ac(netlist, ac_config),
            AnalysisConfig::Noise(noise_config) => self.run_noise(netlist, noise_config),
            AnalysisConfig::PoleZero(pz_config) => self.run_pz(netlist, pz_config),
            AnalysisConfig::Sensitivity(sens_config) => self.run_sensitivity(netlist, sens_config),
        }
    }

    //-------------------------------------------------------------------------
    // Netlist Parsing
    //-------------------------------------------------------------------------

    fn parse_netlist_with_source_path(
        &self,
        netlist_str: &str,
        source_path: Option<&Path>,
    ) -> Result<rspice_core::Netlist, SimulationError> {
        let parse_source = Self::netlist_parse_source(source_path);
        rspice_core::Netlist::parse_with_path(netlist_str, &parse_source)
            .map_err(|e| SimulationError::ParseError(e.to_string()))
    }

    fn netlist_parse_source(source_path: Option<&Path>) -> PathBuf {
        const GENERATED_NETLIST_NAME: &str = "__rspice_ui_generated__.cir";

        match source_path {
            Some(path) if path.is_dir() => path.join(GENERATED_NETLIST_NAME),
            Some(path) => path.to_path_buf(),
            None => std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(GENERATED_NETLIST_NAME),
        }
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

    #[inline]
    fn ac_node_waveform_name(result: &rspice_core::analysis::AcResult, node_idx: usize) -> String {
        result
            .node_names
            .get(node_idx)
            .filter(|name| !name.is_empty())
            .map(|name| format!("V({})", name))
            .unwrap_or_else(|| format!("V({})", node_idx + 1))
    }

    #[inline]
    fn ac_branch_waveform_name(
        result: &rspice_core::analysis::AcResult,
        branch_idx: usize,
    ) -> String {
        result
            .branch_names
            .get(branch_idx)
            .filter(|name| !name.is_empty())
            .map(|name| format!("I({})", name))
            .unwrap_or_else(|| format!("I({})", branch_idx + 1))
    }

    #[inline]
    fn dc_branch_waveform_name(
        result: &rspice_core::SimulationResult,
        branch_idx: usize,
    ) -> String {
        result
            .branch_names
            .get(branch_idx)
            .filter(|name| !name.is_empty())
            .map(|name| format!("I({})", name))
            .unwrap_or_else(|| format!("I({})", branch_idx + 1))
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
            let name = Self::dc_branch_waveform_name(core_result, i);
            result.branch_currents.insert(name, current);
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
        // SPICE .tran step is an output interval. Since our transient engine
        // emits accepted timesteps directly (without a separate output
        // interpolation stage), keep internal max-step at or below the
        // requested output step by default to preserve waveform fidelity.
        config.max_timestep.unwrap_or(config.step_time).max(1e-18)
    }

    #[inline]
    fn transient_start_index(time: &[f64], start_time: f64) -> usize {
        if !start_time.is_finite() || start_time <= 0.0 {
            return 0;
        }
        time.partition_point(|t| *t < start_time)
    }

    #[inline]
    fn transient_sample_count_after_index(
        time: &[f64],
        voltages: &[Vec<f64>],
        start_idx: usize,
    ) -> usize {
        let max_time_len = time.len().saturating_sub(start_idx);
        voltages.iter().fold(max_time_len, |acc, trace| {
            acc.min(trace.len().saturating_sub(start_idx))
        })
    }

    /// Run transient analysis
    fn run_transient(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::TransientAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        if config.uic {
            log::warn!(
                "Transient UIC requested, but rspice-core transient startup currently uses DC operating-point initialization"
            );
        }
        let engine = self.engine_for_netlist(netlist);
        let max_step = Self::resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran(netlist, config.stop_time, max_step)
            .map_err(|e| self.translate_error(e))?;

        let start_idx = Self::transient_start_index(&tran_result.time, config.start_time);
        let sample_count = Self::transient_sample_count_after_index(
            &tran_result.time,
            &tran_result.voltages,
            start_idx,
        );
        let filtered_time = tran_result.time[start_idx..start_idx + sample_count].to_vec();

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
                WaveformData::new_time_domain(
                    &name,
                    filtered_time.clone(),
                    voltages[start_idx..start_idx + sample_count].to_vec(),
                ),
            );
        }

        Ok(SimulationResult::Transient {
            time: filtered_time,
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
        if config.uic {
            log::warn!(
                "Transient UIC requested, but rspice-core transient startup currently uses DC operating-point initialization"
            );
        }
        let engine = self.engine_for_netlist(netlist);
        let max_step = Self::resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran_with_abort(netlist, config.stop_time, max_step, abort)
            .map_err(|e| self.translate_error(e))?;

        let start_idx = Self::transient_start_index(&tran_result.time, config.start_time);
        let sample_count = Self::transient_sample_count_after_index(
            &tran_result.time,
            &tran_result.voltages,
            start_idx,
        );
        let filtered_time = tran_result.time[start_idx..start_idx + sample_count].to_vec();

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
                WaveformData::new_time_domain(
                    &name,
                    filtered_time.clone(),
                    voltages[start_idx..start_idx + sample_count].to_vec(),
                ),
            );
        }

        Ok(SimulationResult::Transient {
            time: filtered_time,
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

        let first_result = &ac_results[0];
        let num_nodes = first_result.voltages.len();

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

            let name = Self::ac_node_waveform_name(first_result, node_idx);
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(&name, frequencies.clone(), real_values, imag_values),
            );
        }

        // Also extract branch currents if available
        if !first_result.currents.is_empty() {
            let num_branches = first_result.currents.len();

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

                let name = Self::ac_branch_waveform_name(first_result, branch_idx);
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
            if let Ok(idx) = trimmed.parse::<usize>()
                && idx < node_names.len()
            {
                return Some(idx);
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
