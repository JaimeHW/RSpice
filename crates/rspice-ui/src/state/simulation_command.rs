//! Simulation Command Configuration
//!
//! Defines analysis types and simulation commands that can be configured
//! via the simulation dialog and executed by the simulation engine.
//!
//! This module provides a clean abstraction over raw SPICE command strings,
//! allowing the UI to work with structured data while generating valid
//! SPICE syntax for the simulation backend.

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// AC Analysis Types
// =============================================================================

/// AC sweep type (frequency variation method)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AcSweepType {
    /// Decade - points per decade (logarithmic, default)
    #[default]
    Decade,
    /// Octave - points per octave (logarithmic)
    Octave,
    /// Linear - equal frequency spacing
    Linear,
}

impl AcSweepType {
    /// Get the SPICE keyword for this sweep type
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "DEC",
            AcSweepType::Octave => "OCT",
            AcSweepType::Linear => "LIN",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "Decade",
            AcSweepType::Octave => "Octave",
            AcSweepType::Linear => "Linear",
        }
    }

    /// All sweep types for UI iteration
    pub const ALL: [AcSweepType; 3] = [
        AcSweepType::Decade,
        AcSweepType::Octave,
        AcSweepType::Linear,
    ];
}

// =============================================================================
// DC Sweep Source Type
// =============================================================================

/// Type of source to sweep in DC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DcSourceType {
    /// Voltage source
    #[default]
    Voltage,
    /// Current source
    Current,
}

impl DcSourceType {
    pub fn display_name(&self) -> &'static str {
        match self {
            DcSourceType::Voltage => "Voltage Source",
            DcSourceType::Current => "Current Source",
        }
    }
}

// =============================================================================
// Simulation Commands
// =============================================================================

/// Transient analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientConfig {
    /// Stop time in seconds
    pub stop_time: f64,
    /// Suggested time step in seconds
    pub time_step: f64,
    /// Start saving data at this time (default 0)
    pub start_time: f64,
    /// Maximum internal time step (optional, limits adaptive stepping)
    pub max_step: Option<f64>,
    /// Use initial conditions (skip DC operating point)
    pub use_initial_conditions: bool,
}

impl Default for TransientConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-3, // 1ms default
            time_step: 1e-6, // 1µs default
            start_time: 0.0,
            max_step: None,
            use_initial_conditions: false,
        }
    }
}

impl TransientConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(
            ".TRAN {} {}",
            format_engineering(self.time_step),
            format_engineering(self.stop_time)
        );

        if self.start_time > 0.0 {
            cmd.push_str(&format!(" {}", format_engineering(self.start_time)));

            if let Some(max) = self.max_step {
                cmd.push_str(&format!(" {}", format_engineering(max)));
            }
        } else if let Some(max) = self.max_step {
            // Need to include start time as 0 to specify max step
            cmd.push_str(&format!(" 0 {}", format_engineering(max)));
        }

        if self.use_initial_conditions {
            cmd.push_str(" UIC");
        }

        cmd
    }
}

/// AC analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcConfig {
    /// Start frequency in Hz
    pub start_freq: f64,
    /// Stop frequency in Hz
    pub stop_freq: f64,
    /// Number of points (per decade/octave for log sweep, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: AcSweepType,
}

impl Default for AcConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0, // 1 Hz
            stop_freq: 1e6,  // 1 MHz
            num_points: 10,  // 10 points per decade
            sweep_type: AcSweepType::Decade,
        }
    }
}

impl AcConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        format!(
            ".AC {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq)
        )
    }
}

/// DC sweep analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcSweepConfig {
    /// Source name to sweep (e.g., "V1")
    pub source_name: String,
    /// Start value
    pub start_value: f64,
    /// Stop value
    pub stop_value: f64,
    /// Increment step
    pub step_value: f64,
    /// Optional second source for nested sweep
    pub source2: Option<Box<DcSweepSource>>,
}

/// Second source for nested DC sweep
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcSweepSource {
    pub source_name: String,
    pub start_value: f64,
    pub stop_value: f64,
    pub step_value: f64,
}

impl Default for DcSweepConfig {
    fn default() -> Self {
        Self {
            source_name: "V1".to_string(),
            start_value: 0.0,
            stop_value: 5.0,
            step_value: 0.1,
            source2: None,
        }
    }
}

impl DcSweepConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(
            ".DC {} {} {} {}",
            self.source_name,
            format_engineering(self.start_value),
            format_engineering(self.stop_value),
            format_engineering(self.step_value)
        );

        if let Some(src2) = &self.source2 {
            cmd.push_str(&format!(
                " {} {} {} {}",
                src2.source_name,
                format_engineering(src2.start_value),
                format_engineering(src2.stop_value),
                format_engineering(src2.step_value)
            ));
        }

        cmd
    }
}

/// Operating point analysis configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpConfig {
    /// Whether OP analysis is enabled
    pub enabled: bool,
}

impl OpConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        if self.enabled {
            ".OP".to_string()
        } else {
            String::new()
        }
    }
}

// =============================================================================
// Noise Analysis Configuration
// =============================================================================

/// Noise analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseConfig {
    /// Output node for noise measurement
    pub output_node: String,
    /// Reference node (usually ground)
    pub reference_node: String,
    /// Input source name for input-referred noise
    pub input_source: String,
    /// Start frequency in Hz
    pub start_freq: f64,
    /// Stop frequency in Hz
    pub stop_freq: f64,
    /// Number of points per decade
    pub points_per_decade: u32,
    /// Sweep type (Decade, Octave, Linear)
    pub sweep_type: AcSweepType,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "Vin".to_string(),
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_decade: 10,
            sweep_type: AcSweepType::Decade,
        }
    }
}

impl NoiseConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        // Format: .NOISE V(out[,ref]) src DEC|OCT|LIN points start stop
        let output_spec = if self.reference_node == "0" || self.reference_node.is_empty() {
            format!("V({})", self.output_node)
        } else {
            format!("V({},{})", self.output_node, self.reference_node)
        };

        format!(
            ".NOISE {} {} {} {} {} {}",
            output_spec,
            self.input_source,
            self.sweep_type.spice_keyword(),
            self.points_per_decade,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq)
        )
    }
}

// =============================================================================
// Monte Carlo Configuration
// =============================================================================

/// Distribution type for Monte Carlo analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum McDistribution {
    /// Uniform distribution ±tolerance%
    #[default]
    Uniform,
    /// Gaussian distribution with sigma
    Gaussian,
}

impl McDistribution {
    pub fn display_name(&self) -> &'static str {
        match self {
            McDistribution::Uniform => "Uniform (±%)",
            McDistribution::Gaussian => "Gaussian (σ)",
        }
    }
}

/// Monte Carlo analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonteCarloConfig {
    /// Number of simulation runs
    pub num_runs: u32,
    /// Random seed (None = random each run)
    pub seed: Option<u64>,
    /// Default tolerance percentage for components
    pub default_tolerance: f64,
    /// Distribution type
    pub distribution: McDistribution,
    /// Analysis to run for each Monte Carlo iteration
    pub run_transient: bool,
    /// Output variable to track (e.g., "V(out)")
    pub track_output: String,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self {
            num_runs: 100,
            seed: None,
            default_tolerance: 5.0, // 5% default
            distribution: McDistribution::Uniform,
            run_transient: true,
            track_output: "V(out)".to_string(),
        }
    }
}

impl MonteCarloConfig {
    /// Generate a description of the Monte Carlo configuration
    /// (Monte Carlo isn't a standard SPICE command, so we describe it)
    pub fn to_spice_string(&self) -> String {
        format!(
            "* Monte Carlo: {} runs, {} ±{}%",
            self.num_runs,
            self.distribution.display_name(),
            self.default_tolerance
        )
    }
}

// =============================================================================
// Pole-Zero Analysis Configuration
// =============================================================================

/// Pole-Zero analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoleZeroConfig {
    /// Input node (positive)
    pub input_pos: String,
    /// Input node (negative, often ground)
    pub input_neg: String,
    /// Output node (positive)
    pub output_pos: String,
    /// Output node (negative, often ground)
    pub output_neg: String,
    /// Transfer type: VOL (voltage), CUR (current), or POL/ZER
    pub transfer_type: PzTransferType,
}

/// Pole-Zero transfer function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PzTransferType {
    /// Voltage transfer function (V/V)
    #[default]
    Voltage,
    /// Current transfer function (I/I)
    Current,
}

impl PzTransferType {
    pub fn display_name(&self) -> &'static str {
        match self {
            PzTransferType::Voltage => "Voltage (V/V)",
            PzTransferType::Current => "Current (I/I)",
        }
    }

    pub fn spice_keyword(&self) -> &'static str {
        match self {
            PzTransferType::Voltage => "VOL",
            PzTransferType::Current => "CUR",
        }
    }
}

impl Default for PoleZeroConfig {
    fn default() -> Self {
        Self {
            input_pos: "in".to_string(),
            input_neg: "0".to_string(),
            output_pos: "out".to_string(),
            output_neg: "0".to_string(),
            transfer_type: PzTransferType::Voltage,
        }
    }
}

impl PoleZeroConfig {
    pub fn to_spice_string(&self) -> String {
        format!(
            ".PZ {} {} {} {} {} PZ",
            self.input_pos,
            self.input_neg,
            self.output_pos,
            self.output_neg,
            self.transfer_type.spice_keyword()
        )
    }
}

// =============================================================================
// Sensitivity Analysis Configuration
// =============================================================================

/// Sensitivity analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensitivityConfig {
    /// Output variable to analyze sensitivity of
    pub output_var: String,
    /// DC or AC sensitivity
    pub is_ac: bool,
    /// Frequency for AC sensitivity (if is_ac)
    pub frequency: f64,
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            output_var: "V(out)".to_string(),
            is_ac: false,
            frequency: 1e6,
        }
    }
}

impl SensitivityConfig {
    pub fn to_spice_string(&self) -> String {
        if self.is_ac {
            format!(
                ".SENS {} AC {}",
                self.output_var,
                format_engineering(self.frequency)
            )
        } else {
            format!(".SENS {}", self.output_var)
        }
    }
}

// =============================================================================
// S-Parameter Analysis Configuration
// =============================================================================

/// S-Parameter analysis configuration (for RF/microwave)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParamConfig {
    /// Port 1 positive node
    pub port1_pos: String,
    /// Port 1 negative node
    pub port1_neg: String,
    /// Port 2 positive node
    pub port2_pos: String,
    /// Port 2 negative node
    pub port2_neg: String,
    /// Reference impedance (typically 50Ω)
    pub z0: f64,
    /// Start frequency
    pub start_freq: f64,
    /// Stop frequency
    pub stop_freq: f64,
    /// Points per decade
    pub points_per_decade: u32,
}

impl Default for SParamConfig {
    fn default() -> Self {
        Self {
            port1_pos: "in".to_string(),
            port1_neg: "0".to_string(),
            port2_pos: "out".to_string(),
            port2_neg: "0".to_string(),
            z0: 50.0,
            start_freq: 1e6,
            stop_freq: 10e9,
            points_per_decade: 20,
        }
    }
}

impl SParamConfig {
    pub fn to_spice_string(&self) -> String {
        format!(
            "* S-Parameters: Port1=({},{}) Port2=({},{}) Z0={}Ω, {} - {}",
            self.port1_pos,
            self.port1_neg,
            self.port2_pos,
            self.port2_neg,
            self.z0,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq)
        )
    }
}

// =============================================================================
// PSS (Periodic Steady State) Analysis Configuration
// =============================================================================

/// PSS analysis configuration for oscillators and periodic circuits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PssConfig {
    /// Fundamental frequency (Hz) - use 0 for auto-detection
    pub fundamental_freq: f64,
    /// Auto-detect period from circuit behavior
    pub auto_detect_period: bool,
    /// Number of harmonics to compute
    pub num_harmonics: u32,
    /// Number of stabilization cycles before analysis
    pub stabilization_cycles: u32,
    /// Convergence tolerance
    pub tolerance: f64,
    /// Maximum Newton iterations per period
    pub max_iterations: u32,
    /// Enable Floquet analysis for stability
    pub floquet_analysis: bool,
}

impl Default for PssConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 1e6, // 1 MHz default
            auto_detect_period: false,
            num_harmonics: 10,
            stabilization_cycles: 3,
            tolerance: 1e-6,
            max_iterations: 50,
            floquet_analysis: false,
        }
    }
}

impl PssConfig {
    /// Generate PSS SPICE command string
    pub fn to_spice_string(&self) -> String {
        let freq_str = if self.auto_detect_period {
            "AUTO".to_string()
        } else {
            format_engineering(self.fundamental_freq)
        };
        // .PSS fund harmonics [stabilization] [tolerance]
        format!(
            ".PSS {} {} {} {}",
            freq_str, self.num_harmonics, self.stabilization_cycles, self.tolerance
        )
    }
}

// =============================================================================
// PAC (Periodic AC) Analysis Configuration
// =============================================================================

/// PAC analysis - small-signal analysis around periodic operating point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacConfig {
    /// Start frequency for sweep (Hz)
    pub start_freq: f64,
    /// Stop frequency for sweep (Hz)
    pub stop_freq: f64,
    /// Points per decade
    pub points_per_decade: u32,
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Sideband index (0 = fundamental, 1 = first sideband, etc.)
    pub sideband: i32,
    /// Maximum sideband index to include
    pub max_sidebands: u32,
}

impl Default for PacConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_decade: 10,
            sweep_type: AcSweepType::Decade,
            sideband: 0,
            max_sidebands: 3,
        }
    }
}

impl PacConfig {
    /// Generate PAC SPICE command string
    pub fn to_spice_string(&self) -> String {
        // .PAC sweep_type points start stop [sidebands]
        format!(
            ".PAC {} {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.points_per_decade,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq),
            self.max_sidebands
        )
    }
}

// =============================================================================
// Harmonic Balance Analysis Configuration
// =============================================================================

/// Canonical HB tone spec reused from the HB dialog/execution model.
pub type HbToneSpec = crate::simulation::dialog::hb::HbToneConfig;

/// Canonical HB config reused from the HB dialog/execution model.
pub type HbConfig = crate::simulation::dialog::hb::HbConfig;

// =============================================================================
// STB (Loop Stability) Analysis Configuration
// =============================================================================

/// STB sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StbSweepType {
    /// Frequency sweep for Bode plot
    #[default]
    Frequency,
    /// Parameter sweep at fixed frequency
    Parameter,
}

impl StbSweepType {
    pub fn display_name(&self) -> &'static str {
        match self {
            StbSweepType::Frequency => "Frequency Sweep",
            StbSweepType::Parameter => "Parameter Sweep",
        }
    }
}

/// Loop stability analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StbConfig {
    /// Probe insertion node (positive terminal)
    pub probe_pos: String,
    /// Probe insertion node (negative terminal)
    pub probe_neg: String,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Points per decade
    pub points_per_decade: u32,
    /// Sweep type
    pub sweep_type: StbSweepType,
    /// Show Nyquist plot
    pub show_nyquist: bool,
    /// Desired phase margin (degrees)
    pub target_phase_margin: f64,
    /// Desired gain margin (dB)
    pub target_gain_margin: f64,
}

impl Default for StbConfig {
    fn default() -> Self {
        Self {
            probe_pos: "fb".to_string(),
            probe_neg: "0".to_string(),
            start_freq: 1.0,
            stop_freq: 100e6,
            points_per_decade: 20,
            sweep_type: StbSweepType::Frequency,
            show_nyquist: false,
            target_phase_margin: 45.0,
            target_gain_margin: 10.0,
        }
    }
}

impl StbConfig {
    /// Generate STB SPICE command string
    pub fn to_spice_string(&self) -> String {
        // .STB probe_pos probe_neg sweep_type points start stop
        format!(
            ".STB {} {} {} {} {} {}",
            self.probe_pos,
            self.probe_neg,
            AcSweepType::Decade.spice_keyword(),
            self.points_per_decade,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq)
        )
    }
}

// =============================================================================
// Envelope Transient Analysis Configuration
// =============================================================================

/// Modulation type for envelope transient
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ModulationType {
    #[default]
    Am,
    Fm,
    Pm,
    Iq,
}

impl ModulationType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ModulationType::Am => "AM (Amplitude)",
            ModulationType::Fm => "FM (Frequency)",
            ModulationType::Pm => "PM (Phase)",
            ModulationType::Iq => "IQ (Complex)",
        }
    }
}

/// Envelope transient analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvelopeConfig {
    /// Carrier frequency (Hz)
    pub carrier_freq: f64,
    /// Modulation type
    pub modulation_type: ModulationType,
    /// Envelope bandwidth (Hz)
    pub envelope_bandwidth: f64,
    /// Stop time (s)
    pub stop_time: f64,
    /// Maximum envelope time step (s)
    pub max_step: Option<f64>,
    /// Number of carrier harmonics
    pub carrier_harmonics: u32,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            carrier_freq: 1e9,
            modulation_type: ModulationType::Am,
            envelope_bandwidth: 10e6,
            stop_time: 1e-6,
            max_step: None,
            carrier_harmonics: 3,
        }
    }
}

impl EnvelopeConfig {
    /// Generate Envelope SPICE command string
    pub fn to_spice_string(&self) -> String {
        // .ENVLP carrier stop [modulation_type] [harmonics]
        let mod_type = match self.modulation_type {
            ModulationType::Am => "AM",
            ModulationType::Fm => "FM",
            ModulationType::Pm => "PM",
            ModulationType::Iq => "IQ",
        };
        format!(
            ".ENVLP {} {} {} {}",
            format_engineering(self.carrier_freq),
            format_engineering(self.stop_time),
            mod_type,
            self.carrier_harmonics
        )
    }
}

// =============================================================================
// Multi-Rate Analysis Configuration
// =============================================================================

/// Multi-rate partition rate class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RateClass {
    #[default]
    Fast,
    Medium,
    Slow,
}

impl RateClass {
    pub fn display_name(&self) -> &'static str {
        match self {
            RateClass::Fast => "Fast (carrier)",
            RateClass::Medium => "Medium",
            RateClass::Slow => "Slow (envelope)",
        }
    }
}

/// Multi-rate analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiRateConfig {
    /// Enable automatic partitioning
    pub auto_partition: bool,
    /// Fast partition rate (Hz)
    pub fast_rate: f64,
    /// Slow partition rate (Hz)
    pub slow_rate: f64,
    /// Stop time (s)
    pub stop_time: f64,
    /// Interface latency tolerance
    pub latency_tolerance: f64,
}

impl Default for MultiRateConfig {
    fn default() -> Self {
        Self {
            auto_partition: true,
            fast_rate: 1e9,
            slow_rate: 1e6,
            stop_time: 1e-3,
            latency_tolerance: 1e-12,
        }
    }
}

impl MultiRateConfig {
    /// Generate MultiRate SPICE command string
    pub fn to_spice_string(&self) -> String {
        // .MULTIRATE fast_rate slow_rate stop [AUTO|MANUAL]
        format!(
            ".MULTIRATE {} {} {} {}",
            format_engineering(self.fast_rate),
            format_engineering(self.slow_rate),
            format_engineering(self.stop_time),
            if self.auto_partition {
                "AUTO"
            } else {
                "MANUAL"
            }
        )
    }
}

// =============================================================================
// Corner Analysis Configuration
// =============================================================================

/// Process corner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProcessCorner {
    #[default]
    Tt, // Typical-Typical
    Ff, // Fast-Fast
    Ss, // Slow-Slow
    Fs, // Fast-Slow
    Sf, // Slow-Fast
}

impl ProcessCorner {
    pub fn display_name(&self) -> &'static str {
        match self {
            ProcessCorner::Tt => "TT (Typical)",
            ProcessCorner::Ff => "FF (Fast)",
            ProcessCorner::Ss => "SS (Slow)",
            ProcessCorner::Fs => "FS (Fast NMOS/Slow PMOS)",
            ProcessCorner::Sf => "SF (Slow NMOS/Fast PMOS)",
        }
    }

    pub const ALL: [ProcessCorner; 5] = [
        ProcessCorner::Tt,
        ProcessCorner::Ff,
        ProcessCorner::Ss,
        ProcessCorner::Fs,
        ProcessCorner::Sf,
    ];
}

/// Corner analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CornerConfig {
    /// Process corners to simulate
    pub process_corners: Vec<ProcessCorner>,
    /// Voltage corners (as percentages, e.g., [90, 100, 110])
    pub voltage_corners: Vec<f64>,
    /// Temperature corners (°C)
    pub temperature_corners: Vec<f64>,
    /// Run all combinations or selected only
    pub full_matrix: bool,
}

impl Default for CornerConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![ProcessCorner::Tt, ProcessCorner::Ff, ProcessCorner::Ss],
            voltage_corners: vec![90.0, 100.0, 110.0],
            temperature_corners: vec![-40.0, 25.0, 125.0],
            full_matrix: false,
        }
    }
}

impl CornerConfig {
    /// Generate Corner analysis SPICE commands
    /// Generates .TEMP and .CORNER statements for PVT corner sweeps
    pub fn to_spice_string(&self) -> String {
        let mut commands = Vec::new();

        // Generate .CORNER for process corners (RSpice extension)
        let corners: Vec<&str> = self
            .process_corners
            .iter()
            .map(|c| match c {
                ProcessCorner::Tt => "TT",
                ProcessCorner::Ff => "FF",
                ProcessCorner::Ss => "SS",
                ProcessCorner::Fs => "FS",
                ProcessCorner::Sf => "SF",
            })
            .collect();
        commands.push(format!(".CORNER {}", corners.join(" ")));

        // Generate .TEMP for temperature corners
        let temps: Vec<String> = self
            .temperature_corners
            .iter()
            .map(|t| t.to_string())
            .collect();
        commands.push(format!(".TEMP {}", temps.join(" ")));

        commands.join("\n")
    }

    /// Calculate total number of corner combinations
    pub fn total_combinations(&self) -> usize {
        if self.full_matrix {
            self.process_corners.len() * self.voltage_corners.len() * self.temperature_corners.len()
        } else {
            self.process_corners
                .len()
                .max(self.voltage_corners.len())
                .max(self.temperature_corners.len())
        }
    }
}

// =============================================================================
// Transfer Function Analysis Configuration
// =============================================================================

/// Transfer function analysis configuration (.TF)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferConfig {
    /// Output variable (e.g., V(out))
    pub output_var: String,
    /// Input source name (e.g., Vin)
    pub input_source: String,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            output_var: "V(out)".to_string(),
            input_source: "Vin".to_string(),
        }
    }
}

impl TransferConfig {
    pub fn to_spice_string(&self) -> String {
        format!(".TF {} {}", self.output_var, self.input_source)
    }
}

// =============================================================================
// Fourier Analysis Configuration
// =============================================================================

/// Fourier analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FourierConfig {
    /// Fundamental frequency for analysis (Hz)
    pub fundamental_freq: f64,
    /// Number of harmonics to compute
    pub num_harmonics: u32,
    /// Output variable to analyze
    pub output_var: String,
    /// Calculate THD
    pub calculate_thd: bool,
}

impl Default for FourierConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 1e3,
            num_harmonics: 10,
            output_var: "V(out)".to_string(),
            calculate_thd: true,
        }
    }
}

impl FourierConfig {
    pub fn to_spice_string(&self) -> String {
        format!(
            ".FOUR {} {} {}",
            format_engineering(self.fundamental_freq),
            self.num_harmonics,
            self.output_var
        )
    }
}

// =============================================================================
// Parametric Analysis Configuration
// =============================================================================

/// Parameter step type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParametricStepType {
    #[default]
    Linear,
    Decade,
    Octave,
    List,
}

impl ParametricStepType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ParametricStepType::Linear => "Linear",
            ParametricStepType::Decade => "Decade (Log)",
            ParametricStepType::Octave => "Octave",
            ParametricStepType::List => "Value List",
        }
    }
}

/// Parametric sweep configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParametricConfig {
    /// Parameter name to sweep
    pub param_name: String,
    /// Start value
    pub start_value: f64,
    /// Stop value
    pub stop_value: f64,
    /// Step type
    pub step_type: ParametricStepType,
    /// Number of steps (for linear) or points per decade (for log)
    pub num_steps: u32,
    /// Explicit value list (for List step type)
    pub values: Vec<f64>,
}

impl Default for ParametricConfig {
    fn default() -> Self {
        Self {
            param_name: "R1".to_string(),
            start_value: 1e3,
            stop_value: 10e3,
            step_type: ParametricStepType::Linear,
            num_steps: 10,
            values: vec![],
        }
    }
}

impl ParametricConfig {
    pub fn to_spice_string(&self) -> String {
        match self.step_type {
            ParametricStepType::List => {
                let vals: Vec<String> =
                    self.values.iter().map(|v| format_engineering(*v)).collect();
                format!(".STEP PARAM {} LIST {}", self.param_name, vals.join(" "))
            }
            ParametricStepType::Linear => {
                let step = (self.stop_value - self.start_value) / self.num_steps as f64;
                format!(
                    ".STEP PARAM {} {} {} {}",
                    self.param_name,
                    format_engineering(self.start_value),
                    format_engineering(self.stop_value),
                    format_engineering(step)
                )
            }
            _ => {
                format!(
                    ".STEP {} PARAM {} {} {} {}",
                    match self.step_type {
                        ParametricStepType::Decade => "DEC",
                        ParametricStepType::Octave => "OCT",
                        _ => "LIN",
                    },
                    self.param_name,
                    format_engineering(self.start_value),
                    format_engineering(self.stop_value),
                    self.num_steps
                )
            }
        }
    }
}

// =============================================================================
// Unified Simulation Configuration
// =============================================================================

/// Complete simulation configuration containing all analysis settings
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Transient analysis (enabled if Some)
    pub transient: Option<TransientConfig>,
    /// AC analysis (enabled if Some)
    pub ac: Option<AcConfig>,
    /// DC sweep analysis (enabled if Some)
    pub dc_sweep: Option<DcSweepConfig>,
    /// Operating point analysis
    pub op: OpConfig,
    /// Noise analysis (enabled if Some)
    pub noise: Option<NoiseConfig>,
    /// Monte Carlo analysis (enabled if Some)
    pub monte_carlo: Option<MonteCarloConfig>,
    /// Pole-Zero analysis (enabled if Some)
    pub pole_zero: Option<PoleZeroConfig>,
    /// Sensitivity analysis (enabled if Some)
    pub sensitivity: Option<SensitivityConfig>,
    /// S-Parameter analysis (enabled if Some)
    pub s_param: Option<SParamConfig>,

    // =========================================================================
    // Advanced Analysis Types (Phase 1+)
    // =========================================================================
    /// Periodic Steady State analysis (enabled if Some)
    pub pss: Option<PssConfig>,
    /// Periodic AC analysis (enabled if Some)
    pub pac: Option<PacConfig>,
    /// Harmonic Balance analysis (enabled if Some)
    pub harmonic_balance: Option<HbConfig>,
    /// Loop Stability analysis (enabled if Some)
    pub stb: Option<StbConfig>,
    /// Envelope Transient analysis (enabled if Some)
    pub envelope: Option<EnvelopeConfig>,
    /// Multi-Rate analysis (enabled if Some)
    pub multi_rate: Option<MultiRateConfig>,
    /// Corner analysis (enabled if Some)
    pub corner: Option<CornerConfig>,
    /// Transfer function analysis (enabled if Some)
    pub transfer: Option<TransferConfig>,
    /// Fourier/THD analysis (enabled if Some)
    pub fourier: Option<FourierConfig>,
    /// Parametric sweep (enabled if Some)
    pub parametric: Option<ParametricConfig>,
}

impl SimulationConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any analysis is configured
    pub fn has_analysis(&self) -> bool {
        self.transient.is_some()
            || self.ac.is_some()
            || self.dc_sweep.is_some()
            || self.op.enabled
            || self.noise.is_some()
            || self.monte_carlo.is_some()
            || self.pole_zero.is_some()
            || self.sensitivity.is_some()
            || self.s_param.is_some()
            // Advanced analyses
            || self.pss.is_some()
            || self.pac.is_some()
            || self.harmonic_balance.is_some()
            || self.stb.is_some()
            || self.envelope.is_some()
            || self.multi_rate.is_some()
            || self.corner.is_some()
            || self.transfer.is_some()
            || self.fourier.is_some()
            || self.parametric.is_some()
    }

    /// Generate all SPICE command strings
    pub fn to_spice_commands(&self) -> Vec<String> {
        let mut commands = Vec::new();

        if self.op.enabled {
            commands.push(self.op.to_spice_string());
        }

        if let Some(dc) = &self.dc_sweep {
            commands.push(dc.to_spice_string());
        }

        if let Some(ac) = &self.ac {
            commands.push(ac.to_spice_string());
        }

        if let Some(tran) = &self.transient {
            commands.push(tran.to_spice_string());
        }

        if let Some(noise) = &self.noise {
            commands.push(noise.to_spice_string());
        }

        if let Some(mc) = &self.monte_carlo {
            commands.push(mc.to_spice_string());
        }

        if let Some(pz) = &self.pole_zero {
            commands.push(pz.to_spice_string());
        }

        if let Some(sens) = &self.sensitivity {
            commands.push(sens.to_spice_string());
        }

        if let Some(sp) = &self.s_param {
            commands.push(sp.to_spice_string());
        }

        // Advanced analysis types
        if let Some(pss) = &self.pss {
            commands.push(pss.to_spice_string());
        }

        if let Some(pac) = &self.pac {
            commands.push(pac.to_spice_string());
        }

        if let Some(hb) = &self.harmonic_balance {
            commands.push(hb.to_spice_string());
        }

        if let Some(stb) = &self.stb {
            commands.push(stb.to_spice_string());
        }

        if let Some(env) = &self.envelope {
            commands.push(env.to_spice_string());
        }

        if let Some(mr) = &self.multi_rate {
            commands.push(mr.to_spice_string());
        }

        if let Some(corner) = &self.corner {
            commands.push(corner.to_spice_string());
        }

        if let Some(tf) = &self.transfer {
            commands.push(tf.to_spice_string());
        }

        if let Some(four) = &self.fourier {
            commands.push(four.to_spice_string());
        }

        if let Some(param) = &self.parametric {
            commands.push(param.to_spice_string());
        }

        commands
    }

    /// Generate commands as a single string (for insertion into netlist)
    pub fn to_spice_string(&self) -> String {
        self.to_spice_commands().join("\n")
    }
}

// =============================================================================
// Formatting Utilities
// =============================================================================

/// Format a number using engineering notation with SI prefixes
/// This matches how SPICE values are typically written
pub fn format_engineering(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let abs_val = value.abs();
    let sign = if value < 0.0 { "-" } else { "" };

    let (scaled, suffix) = if abs_val >= 1e12 {
        (abs_val / 1e12, "T")
    } else if abs_val >= 1e9 {
        (abs_val / 1e9, "G")
    } else if abs_val >= 1e6 {
        (abs_val / 1e6, "MEG")
    } else if abs_val >= 1e3 {
        (abs_val / 1e3, "k")
    } else if abs_val >= 1.0 {
        (abs_val, "")
    } else if abs_val >= 1e-3 {
        (abs_val * 1e3, "m")
    } else if abs_val >= 1e-6 {
        (abs_val * 1e6, "u")
    } else if abs_val >= 1e-9 {
        (abs_val * 1e9, "n")
    } else if abs_val >= 1e-12 {
        (abs_val * 1e12, "p")
    } else if abs_val >= 1e-15 {
        (abs_val * 1e15, "f")
    } else {
        // Fall back to scientific notation for very small values
        return format!("{:e}", value);
    };

    // Format with minimum necessary precision
    let formatted = if scaled == scaled.floor() {
        format!("{}{}{}", sign, scaled as i64, suffix)
    } else if (scaled * 10.0).fract().abs() < 1e-9 {
        format!("{}{:.1}{}", sign, scaled, suffix)
    } else if (scaled * 100.0).fract().abs() < 1e-9 {
        format!("{}{:.2}{}", sign, scaled, suffix)
    } else {
        format!("{}{:.3}{}", sign, scaled, suffix)
    };

    formatted
}

/// Parse a SPICE-style value string (with SI suffix) to f64
pub fn parse_spice_value(s: &str) -> Option<f64> {
    let s = s.trim().to_uppercase();

    if s.is_empty() {
        return None;
    }

    // Find where the number ends and suffix begins
    let mut num_end = s.len();
    for (i, c) in s.char_indices() {
        if !c.is_ascii_digit() && c != '.' && c != '-' && c != '+' && c != 'E' {
            // Check if this is part of scientific notation
            if i > 0 {
                let prev = s.chars().nth(i - 1).unwrap_or(' ');
                if prev == 'E' && (c == '-' || c == '+') {
                    continue;
                }
            }
            num_end = i;
            break;
        }
    }

    let num_str = &s[..num_end];
    let suffix = s[num_end..].trim();

    let base: f64 = num_str.parse().ok()?;

    let multiplier = match suffix {
        "T" => 1e12,
        "G" => 1e9,
        "MEG" => 1e6, // MEG = mega (1e6)
        "K" => 1e3,
        "" => 1.0,
        "M" => 1e-3,      // M = milli in SPICE (not mega!)
        "MIL" => 25.4e-6, // 1 mil = 25.4 micrometers
        "U" | "µ" => 1e-6,
        "N" => 1e-9,
        "P" => 1e-12,
        "F" => 1e-15,
        _ => return None,
    };

    Some(base * multiplier)
}

impl fmt::Display for SimulationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_spice_string())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transient_to_spice() {
        let tran = TransientConfig {
            stop_time: 1e-3,
            time_step: 1e-6,
            start_time: 0.0,
            max_step: None,
            use_initial_conditions: false,
        };
        assert_eq!(tran.to_spice_string(), ".TRAN 1u 1m");
    }

    #[test]
    fn test_transient_with_max_step() {
        let tran = TransientConfig {
            stop_time: 10e-3,
            time_step: 10e-6,
            start_time: 0.0,
            max_step: Some(1e-6),
            use_initial_conditions: false,
        };
        assert_eq!(tran.to_spice_string(), ".TRAN 10u 10m 0 1u");
    }

    #[test]
    fn test_ac_to_spice() {
        let ac = AcConfig {
            start_freq: 1.0,
            stop_freq: 1e6,
            num_points: 20,
            sweep_type: AcSweepType::Decade,
        };
        assert_eq!(ac.to_spice_string(), ".AC DEC 20 1 1MEG");
    }

    #[test]
    fn test_dc_sweep_to_spice() {
        let dc = DcSweepConfig {
            source_name: "V1".to_string(),
            start_value: 0.0,
            stop_value: 5.0,
            step_value: 0.1,
            source2: None,
        };
        assert_eq!(dc.to_spice_string(), ".DC V1 0 5 100m");
    }

    #[test]
    fn test_hb_config_reuses_dialog_model_for_spice_generation() {
        let hb = HbConfig::new(1e9, 9)
            .add_tone(HbToneSpec::new(900e6, 5).with_name("LO").with_source("VLO"));
        let spice = hb.to_spice_string();
        assert!(spice.starts_with(".hb "));
        assert!(spice.contains("harmonics=9"));
        assert!(spice.contains("tone2=900Meg"));
    }

    #[test]
    fn test_format_engineering() {
        assert_eq!(format_engineering(1e-6), "1u");
        assert_eq!(format_engineering(10e-3), "10m");
        assert_eq!(format_engineering(1e3), "1k");
        assert_eq!(format_engineering(1e6), "1MEG");
        assert_eq!(format_engineering(2.5e-9), "2.5n");
    }

    #[test]
    fn test_parse_spice_value() {
        // Use approximate comparison for floating-point values
        fn approx_eq(a: Option<f64>, b: f64) -> bool {
            match a {
                Some(v) => (v - b).abs() < 1e-15 * b.abs().max(1.0),
                None => false,
            }
        }
        assert!(approx_eq(parse_spice_value("1k"), 1e3));
        assert!(approx_eq(parse_spice_value("10u"), 10e-6));
        assert!(approx_eq(parse_spice_value("1MEG"), 1e6));
        assert!(approx_eq(parse_spice_value("2.5n"), 2.5e-9));
        assert!(approx_eq(parse_spice_value("1m"), 1e-3)); // Test milli
    }
}
