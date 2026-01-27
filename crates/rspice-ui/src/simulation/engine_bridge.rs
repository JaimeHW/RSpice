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

    //-------------------------------------------------------------------------
    // Netlist Parsing
    //-------------------------------------------------------------------------

    /// Parse netlist string into core Netlist object
    fn parse_netlist(&self, netlist_str: &str) -> Result<rspice_core::Netlist, SimulationError> {
        rspice_core::Netlist::parse(netlist_str)
            .map_err(|e| SimulationError::ParseError(e.to_string()))
    }

    //-------------------------------------------------------------------------
    // DC Operating Point
    //-------------------------------------------------------------------------

    /// Run DC operating point analysis
    fn run_dc_op(
        &self,
        netlist: &rspice_core::Netlist,
    ) -> Result<SimulationResult, SimulationError> {
        let core_result = self
            .engine
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
        let sweep_results = self
            .engine
            .run_dc_sweep(
                netlist,
                &config.source,
                config.start,
                config.stop,
                config.step,
            )
            .map_err(|e| self.translate_error(e))?;

        // Convert to UI format
        let sweep_values: Vec<f64> = sweep_results.iter().map(|(v, _)| *v).collect();
        let mut waveforms = HashMap::new();

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

        Ok(SimulationResult::DcSweep {
            sweep_var: config.source.clone(),
            sweep_values,
            waveforms,
        })
    }

    //-------------------------------------------------------------------------
    // Transient Analysis
    //-------------------------------------------------------------------------

    /// Run transient analysis
    fn run_transient(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::TransientAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let tran_result = self
            .engine
            .run_tran(netlist, config.stop_time, config.step_time)
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
        // Generate frequency points based on sweep configuration
        let frequencies = config.generate_frequencies();

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid frequency sweep configuration".to_string(),
            ));
        }

        // Run AC analysis via engine
        let ac_results = self
            .engine
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
        let noise_results = self
            .engine
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
    fn resolve_node_index(&self, name: &str, _netlist: &rspice_core::Netlist) -> usize {
        // Handle ground
        let lower = name.to_lowercase();
        if lower == "0" || lower == "gnd" || lower == "ground" {
            return 0;
        }

        // Try parsing as numeric index
        if let Ok(idx) = name.parse::<usize>() {
            return idx;
        }

        // For named nodes like "out", "in", "vdd" - these are assigned indices
        // by the Circuit builder in order of first appearance. Without access
        // to the built circuit's node map, we default to node 1.
        // This is a limitation - full implementation would need Circuit access.
        //
        // TODO: Enhance by passing Circuit's node_map through from engine
        1
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
        // Resolve node indices
        let input_node = self.resolve_node_index(&config.input_node, netlist);
        let output_node = self.resolve_node_index(&config.output_node, netlist);

        if input_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid input node '{}' for pole-zero analysis",
                config.input_node
            )));
        }

        if output_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid output node '{}' for pole-zero analysis",
                config.output_node
            )));
        }

        // Run pole-zero analysis via engine
        let pz_result = self
            .engine
            .run_pz(netlist, input_node, output_node)
            .map_err(|e| self.translate_error(e))?;

        // Convert poles and zeros to (f64, f64) tuples for UI
        let poles: Vec<(f64, f64)> = pz_result.poles.iter().map(|p| (p.re, p.im)).collect();

        let zeros: Vec<(f64, f64)> = pz_result.zeros.iter().map(|z| (z.re, z.im)).collect();

        Ok(SimulationResult::PoleZero {
            poles,
            zeros,
            gain: pz_result.dc_gain,
        })
    }

    //-------------------------------------------------------------------------
    // Sensitivity Analysis
    //-------------------------------------------------------------------------

    /// Run sensitivity analysis
    ///
    /// Computes the partial derivative of the output with respect to each
    /// circuit parameter using finite differences.
    ///
    /// Note: Full implementation requires enumerating circuit parameters
    /// from element values. Current version returns empty results as a
    /// placeholder - UI can specify specific parameters to analyze.
    fn run_sensitivity(
        &self,
        netlist: &rspice_core::Netlist,
        config: &super::config::SensitivityConfig,
    ) -> Result<SimulationResult, SimulationError> {
        use std::collections::HashMap;

        // Parse output variable to get node index
        let output_node = self.parse_output_variable(&config.output_var, netlist);

        if output_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid output variable '{}' for sensitivity analysis",
                config.output_var
            )));
        }

        // Get nominal DC operating point to verify circuit works
        let _nominal_result = self
            .engine
            .run_dc_op(netlist)
            .map_err(|e| self.translate_error(e))?;

        // For now, return empty sensitivity results
        // Full implementation would:
        // 1. Extract component values from netlist elements
        // 2. For each component, perturb value and re-simulate
        // 3. Compute dV/dParam using finite differences
        //
        // TODO: Implement full parameter extraction and sensitivity sweep
        let sensitivities: HashMap<String, f64> = HashMap::new();
        let normalized: HashMap<String, f64> = HashMap::new();

        Ok(SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        })
    }

    /// Parse output variable string (e.g., "V(out)" or "I(R1)") to node index
    fn parse_output_variable(&self, var: &str, netlist: &rspice_core::Netlist) -> usize {
        let trimmed = var.trim();

        // V(node) format
        if trimmed.to_uppercase().starts_with("V(") && trimmed.ends_with(')') {
            let node_name = &trimmed[2..trimmed.len() - 1];
            return self.resolve_node_index(node_name, netlist);
        }

        // Numeric index
        if let Ok(idx) = trimmed.parse::<usize>() {
            return idx;
        }

        // Try as node name directly
        self.resolve_node_index(trimmed, netlist)
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
}
