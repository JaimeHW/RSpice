//! Analysis Configuration Builders
//!
//! Configuration structures for each analysis type that can be built from
//! the UI dialog state and converted to rspice-core analysis parameters.

use crate::simulation::dialog::{AcConfig, DcConfig, NoiseConfig, TransientConfig};

//=============================================================================
// Analysis Type
//=============================================================================

/// Type of analysis to run
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// Transient analysis
    Transient,
    /// AC small-signal analysis
    Ac,
    /// Noise analysis
    Noise,
    /// Pole-zero analysis
    PoleZero,
    /// Sensitivity analysis
    Sensitivity,
}

impl AnalysisType {
    /// Get SPICE analysis command
    pub fn spice_command(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => ".op",
            AnalysisType::DcSweep => ".dc",
            AnalysisType::Transient => ".tran",
            AnalysisType::Ac => ".ac",
            AnalysisType::Noise => ".noise",
            AnalysisType::PoleZero => ".pz",
            AnalysisType::Sensitivity => ".sens",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => "DC Operating Point",
            AnalysisType::DcSweep => "DC Sweep",
            AnalysisType::Transient => "Transient",
            AnalysisType::Ac => "AC Analysis",
            AnalysisType::Noise => "Noise Analysis",
            AnalysisType::PoleZero => "Pole-Zero",
            AnalysisType::Sensitivity => "Sensitivity",
        }
    }
}

//=============================================================================
// Analysis Configuration
//=============================================================================

/// Unified analysis configuration
#[derive(Debug, Clone)]
pub enum AnalysisConfig {
    /// DC operating point (no parameters)
    DcOp,

    /// DC sweep configuration
    DcSweep(DcSweepConfig),

    /// Transient analysis configuration
    Transient(TransientAnalysisConfig),

    /// AC analysis configuration
    Ac(AcAnalysisConfig),

    /// Noise analysis configuration
    Noise(NoiseAnalysisConfig),

    /// Pole-zero analysis configuration
    PoleZero(PoleZeroConfig),

    /// Sensitivity analysis configuration
    Sensitivity(SensitivityConfig),
}

impl AnalysisConfig {
    /// Get the analysis type
    pub fn analysis_type(&self) -> AnalysisType {
        match self {
            AnalysisConfig::DcOp => AnalysisType::DcOp,
            AnalysisConfig::DcSweep(_) => AnalysisType::DcSweep,
            AnalysisConfig::Transient(_) => AnalysisType::Transient,
            AnalysisConfig::Ac(_) => AnalysisType::Ac,
            AnalysisConfig::Noise(_) => AnalysisType::Noise,
            AnalysisConfig::PoleZero(_) => AnalysisType::PoleZero,
            AnalysisConfig::Sensitivity(_) => AnalysisType::Sensitivity,
        }
    }

    /// Generate SPICE analysis command
    pub fn to_spice(&self) -> String {
        match self {
            AnalysisConfig::DcOp => ".op".to_string(),
            AnalysisConfig::DcSweep(cfg) => cfg.to_spice(),
            AnalysisConfig::Transient(cfg) => cfg.to_spice(),
            AnalysisConfig::Ac(cfg) => cfg.to_spice(),
            AnalysisConfig::Noise(cfg) => cfg.to_spice(),
            AnalysisConfig::PoleZero(cfg) => cfg.to_spice(),
            AnalysisConfig::Sensitivity(cfg) => cfg.to_spice(),
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        match self {
            AnalysisConfig::DcOp => Ok(()),
            AnalysisConfig::DcSweep(cfg) => cfg.validate(),
            AnalysisConfig::Transient(cfg) => cfg.validate(),
            AnalysisConfig::Ac(cfg) => cfg.validate(),
            AnalysisConfig::Noise(cfg) => cfg.validate(),
            AnalysisConfig::PoleZero(cfg) => cfg.validate(),
            AnalysisConfig::Sensitivity(cfg) => cfg.validate(),
        }
    }
}

//=============================================================================
// DC Sweep Configuration
//=============================================================================

/// DC sweep analysis configuration
#[derive(Debug, Clone)]
pub struct DcSweepConfig {
    /// Source to sweep (e.g., "Vin")
    pub source: String,
    /// Start value
    pub start: f64,
    /// Stop value
    pub stop: f64,
    /// Step size
    pub step: f64,
    /// Secondary sweep source (optional)
    pub source2: Option<String>,
    /// Secondary start value
    pub start2: Option<f64>,
    /// Secondary stop value
    pub stop2: Option<f64>,
    /// Secondary step size
    pub step2: Option<f64>,
}

impl Default for DcSweepConfig {
    fn default() -> Self {
        Self {
            source: "Vin".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        }
    }
}

impl DcSweepConfig {
    /// Generate SPICE .dc command
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".dc {} {} {} {}",
            self.source, self.start, self.stop, self.step
        );
        if let (Some(src2), Some(start2), Some(stop2), Some(step2)) =
            (&self.source2, self.start2, self.stop2, self.step2)
        {
            cmd.push_str(&format!(" {} {} {} {}", src2, start2, stop2, step2));
        }
        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.source.is_empty() {
            errors.push("Source name is required".to_string());
        }
        if self.step == 0.0 {
            errors.push("Step size cannot be zero".to_string());
        }
        if (self.stop - self.start).signum() != self.step.signum() {
            errors.push("Step direction must match sweep direction".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate number of points
    pub fn num_points(&self) -> usize {
        if self.step == 0.0 {
            return 0;
        }
        ((self.stop - self.start) / self.step).abs() as usize + 1
    }
}

impl From<DcConfig> for DcSweepConfig {
    fn from(cfg: DcConfig) -> Self {
        let (source2, start2, stop2, step2) = if let Some(src2) = cfg.source2 {
            (
                Some(src2.name),
                Some(src2.start),
                Some(src2.stop),
                Some(src2.step),
            )
        } else {
            (None, None, None, None)
        };
        Self {
            source: cfg.source1.name,
            start: cfg.source1.start,
            stop: cfg.source1.stop,
            step: cfg.source1.step,
            source2,
            start2,
            stop2,
            step2,
        }
    }
}

//=============================================================================
// Transient Analysis Configuration
//=============================================================================

/// Transient analysis configuration
#[derive(Debug, Clone)]
pub struct TransientAnalysisConfig {
    /// Stop time
    pub stop_time: f64,
    /// Step time (output interval)
    pub step_time: f64,
    /// Start time (default 0)
    pub start_time: f64,
    /// Maximum internal timestep
    pub max_timestep: Option<f64>,
    /// Use initial conditions
    pub uic: bool,
}

impl Default for TransientAnalysisConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        }
    }
}

impl TransientAnalysisConfig {
    /// Generate SPICE .tran command
    pub fn to_spice(&self) -> String {
        let mut cmd = if self.start_time > 0.0 {
            format!(
                ".tran {} {} {}",
                self.step_time, self.stop_time, self.start_time
            )
        } else {
            format!(".tran {} {}", self.step_time, self.stop_time)
        };
        if let Some(max_ts) = self.max_timestep {
            cmd.push_str(&format!(" {}", max_ts));
        }
        if self.uic {
            cmd.push_str(" UIC");
        }
        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.stop_time <= 0.0 {
            errors.push("Stop time must be positive".to_string());
        }
        if self.step_time <= 0.0 {
            errors.push("Step time must be positive".to_string());
        }
        if self.start_time >= self.stop_time {
            errors.push("Start time must be less than stop time".to_string());
        }
        if self.step_time > self.stop_time {
            errors.push("Step time should not exceed stop time".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate approximate number of output points
    pub fn num_points(&self) -> usize {
        if self.step_time <= 0.0 {
            return 0;
        }
        ((self.stop_time - self.start_time) / self.step_time) as usize + 1
    }
}

impl From<TransientConfig> for TransientAnalysisConfig {
    fn from(cfg: TransientConfig) -> Self {
        Self {
            stop_time: cfg.stop_time,
            step_time: cfg.max_step.unwrap_or(cfg.stop_time / 100.0),
            start_time: cfg.start_time,
            max_timestep: cfg.max_step,
            uic: cfg.uic,
        }
    }
}

//=============================================================================
// AC Analysis Configuration
//=============================================================================

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcSweepType {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave
    Octave,
    /// Linear
    Linear,
}

impl AcSweepType {
    fn spice_name(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "dec",
            AcSweepType::Octave => "oct",
            AcSweepType::Linear => "lin",
        }
    }
}

/// AC analysis configuration
#[derive(Debug, Clone)]
pub struct AcAnalysisConfig {
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Number of points (per decade/octave, or total for linear)
    pub num_points: usize,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
}

impl Default for AcAnalysisConfig {
    fn default() -> Self {
        Self {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        }
    }
}

impl AcAnalysisConfig {
    /// Generate SPICE .ac command
    pub fn to_spice(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.start_freq <= 0.0 {
            errors.push("Start frequency must be positive".to_string());
        }
        if self.stop_freq <= 0.0 {
            errors.push("Stop frequency must be positive".to_string());
        }
        if self.start_freq >= self.stop_freq {
            errors.push("Start frequency must be less than stop frequency".to_string());
        }
        if self.num_points == 0 {
            errors.push("Number of points must be positive".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate total number of frequency points
    pub fn total_points(&self) -> usize {
        match self.sweep_type {
            AcSweepType::Decade => {
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64) as usize + 1
            }
            AcSweepType::Octave => {
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64) as usize + 1
            }
            AcSweepType::Linear => self.num_points,
        }
    }

    /// Generate array of frequency points based on sweep configuration
    ///
    /// This is the definitive frequency generation used by the engine.
    /// Matches Spectre/SPICE semantics:
    /// - Decade: num_points per decade, logarithmically spaced
    /// - Octave: num_points per octave, logarithmically spaced  
    /// - Linear: num_points total, linearly spaced
    pub fn generate_frequencies(&self) -> Vec<f64> {
        match self.sweep_type {
            AcSweepType::Decade => self.generate_logarithmic_sweep(10.0),
            AcSweepType::Octave => self.generate_logarithmic_sweep(2.0),
            AcSweepType::Linear => self.generate_linear_sweep(),
        }
    }

    /// Generate logarithmic frequency sweep
    ///
    /// # Arguments
    /// * `base` - Base for logarithm (10.0 for decade, 2.0 for octave)
    fn generate_logarithmic_sweep(&self, base: f64) -> Vec<f64> {
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 || self.start_freq >= self.stop_freq {
            return vec![];
        }

        let log_start = self.start_freq.log(base);
        let log_stop = self.stop_freq.log(base);
        let num_units = log_stop - log_start; // Number of decades/octaves

        // Total points = num_points per unit * num_units + 1
        let total_points = ((num_units * self.num_points as f64) as usize).max(1) + 1;

        let mut frequencies = Vec::with_capacity(total_points);
        for i in 0..total_points {
            let t = i as f64 / (total_points - 1).max(1) as f64;
            let log_freq = log_start + t * (log_stop - log_start);
            frequencies.push(base.powf(log_freq));
        }

        // Ensure we hit exact start and stop values
        if let Some(first) = frequencies.first_mut() {
            *first = self.start_freq;
        }
        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }

    /// Generate linear frequency sweep
    fn generate_linear_sweep(&self) -> Vec<f64> {
        if self.start_freq >= self.stop_freq || self.num_points == 0 {
            return vec![];
        }

        let mut frequencies = Vec::with_capacity(self.num_points);
        let step = (self.stop_freq - self.start_freq) / (self.num_points - 1).max(1) as f64;

        for i in 0..self.num_points {
            frequencies.push(self.start_freq + i as f64 * step);
        }

        // Ensure exact stop value
        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }
}

impl From<AcConfig> for AcAnalysisConfig {
    fn from(cfg: AcConfig) -> Self {
        use crate::simulation::dialog::ac::FrequencySweep;
        let sweep_type = match cfg.sweep_type {
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
            FrequencySweep::Decade => AcSweepType::Decade,
        };
        Self {
            sweep_type,
            num_points: cfg.num_points as usize,
            start_freq: cfg.start_freq,
            stop_freq: cfg.stop_freq,
        }
    }
}

//=============================================================================
// Noise Analysis Configuration
//=============================================================================

/// Noise analysis configuration
#[derive(Debug, Clone)]
pub struct NoiseAnalysisConfig {
    /// Output node (voltage probe point)
    pub output_node: String,
    /// Reference node (usually ground)
    pub reference_node: String,
    /// Input source (for input-referred noise)
    pub input_source: String,
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Number of points per decade/octave
    pub num_points: usize,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
}

impl Default for NoiseAnalysisConfig {
    fn default() -> Self {
        Self {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "Vin".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        }
    }
}

impl NoiseAnalysisConfig {
    /// Generate SPICE .noise command
    pub fn to_spice(&self) -> String {
        format!(
            ".noise V({},{}) {} {} {} {} {}",
            self.output_node,
            self.reference_node,
            self.input_source,
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_node.is_empty() {
            errors.push("Output node is required".to_string());
        }
        if self.input_source.is_empty() {
            errors.push("Input source is required".to_string());
        }
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
            errors.push("Frequencies must be positive".to_string());
        }
        if self.start_freq >= self.stop_freq {
            errors.push("Start frequency must be less than stop frequency".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Generate array of frequency points based on sweep configuration
    ///
    /// Uses same logic as AcAnalysisConfig for consistency.
    pub fn generate_frequencies(&self) -> Vec<f64> {
        match self.sweep_type {
            AcSweepType::Decade => self.generate_logarithmic_sweep(10.0),
            AcSweepType::Octave => self.generate_logarithmic_sweep(2.0),
            AcSweepType::Linear => self.generate_linear_sweep(),
        }
    }

    /// Generate logarithmic frequency sweep
    fn generate_logarithmic_sweep(&self, base: f64) -> Vec<f64> {
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 || self.start_freq >= self.stop_freq {
            return vec![];
        }

        let log_start = self.start_freq.log(base);
        let log_stop = self.stop_freq.log(base);
        let num_units = log_stop - log_start;

        let total_points = ((num_units * self.num_points as f64) as usize).max(1) + 1;

        let mut frequencies = Vec::with_capacity(total_points);
        for i in 0..total_points {
            let t = i as f64 / (total_points - 1).max(1) as f64;
            let log_freq = log_start + t * (log_stop - log_start);
            frequencies.push(base.powf(log_freq));
        }

        if let Some(first) = frequencies.first_mut() {
            *first = self.start_freq;
        }
        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }

    /// Generate linear frequency sweep
    fn generate_linear_sweep(&self) -> Vec<f64> {
        if self.start_freq >= self.stop_freq || self.num_points == 0 {
            return vec![];
        }

        let mut frequencies = Vec::with_capacity(self.num_points);
        let step = (self.stop_freq - self.start_freq) / (self.num_points - 1).max(1) as f64;

        for i in 0..self.num_points {
            frequencies.push(self.start_freq + i as f64 * step);
        }

        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }

    /// Get default temperature for noise analysis (300K = 27°C)
    pub fn default_temperature(&self) -> f64 {
        300.0
    }
}

impl From<NoiseConfig> for NoiseAnalysisConfig {
    fn from(cfg: NoiseConfig) -> Self {
        use crate::simulation::dialog::ac::FrequencySweep;
        let sweep_type = match cfg.sweep_type {
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
            FrequencySweep::Decade => AcSweepType::Decade,
        };
        Self {
            output_node: cfg.output_node,
            reference_node: cfg.reference_node,
            input_source: cfg.input_source,
            sweep_type,
            num_points: cfg.num_points as usize,
            start_freq: cfg.start_freq,
            stop_freq: cfg.stop_freq,
        }
    }
}

//=============================================================================
// Pole-Zero Configuration
//=============================================================================

/// Pole-zero analysis type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PzAnalysisType {
    /// Find both poles and zeros
    #[default]
    PoleZero,
    /// Find poles only
    PolesOnly,
    /// Find zeros only
    ZerosOnly,
}

/// Pole-zero analysis configuration
#[derive(Debug, Clone)]
pub struct PoleZeroConfig {
    /// Input node
    pub input_node: String,
    /// Input reference node
    pub input_ref: String,
    /// Output node
    pub output_node: String,
    /// Output reference node
    pub output_ref: String,
    /// Transfer function type (VOL or CUR)
    pub transfer_type: String,
    /// Analysis type
    pub analysis_type: PzAnalysisType,
}

impl Default for PoleZeroConfig {
    fn default() -> Self {
        Self {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: PzAnalysisType::PoleZero,
        }
    }
}

impl PoleZeroConfig {
    /// Generate SPICE .pz command
    pub fn to_spice(&self) -> String {
        let pz_type = match self.analysis_type {
            PzAnalysisType::PoleZero => "PZ",
            PzAnalysisType::PolesOnly => "POL",
            PzAnalysisType::ZerosOnly => "ZER",
        };
        format!(
            ".pz {} {} {} {} {} {}",
            self.input_node,
            self.input_ref,
            self.output_node,
            self.output_ref,
            self.transfer_type,
            pz_type
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.input_node.is_empty() {
            errors.push("Input node is required".to_string());
        }
        if self.output_node.is_empty() {
            errors.push("Output node is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

//=============================================================================
// Sensitivity Configuration
//=============================================================================

/// Sensitivity analysis configuration
#[derive(Debug, Clone)]
pub struct SensitivityConfig {
    /// Output variable (e.g., "V(out)", "I(R1)")
    pub output_var: String,
    /// AC analysis (if true, does AC sensitivity)
    pub ac_mode: bool,
    /// Frequency for AC sensitivity
    pub frequency: Option<f64>,
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        }
    }
}

impl SensitivityConfig {
    /// Generate SPICE .sens command
    pub fn to_spice(&self) -> String {
        if self.ac_mode {
            if let Some(freq) = self.frequency {
                format!(".sens {} AC DEC 1 {} {}", self.output_var, freq, freq)
            } else {
                format!(".sens {} AC", self.output_var)
            }
        } else {
            format!(".sens {}", self.output_var)
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_var.is_empty() {
            errors.push("Output variable is required".to_string());
        }
        if self.ac_mode && self.frequency.is_some() && self.frequency.unwrap() <= 0.0 {
            errors.push("Frequency must be positive for AC sensitivity".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_type_spice_command() {
        assert_eq!(AnalysisType::DcOp.spice_command(), ".op");
        assert_eq!(AnalysisType::Transient.spice_command(), ".tran");
        assert_eq!(AnalysisType::Ac.spice_command(), ".ac");
    }

    #[test]
    fn test_dc_sweep_to_spice() {
        let cfg = DcSweepConfig {
            source: "Vin".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
            ..Default::default()
        };
        assert_eq!(cfg.to_spice(), ".dc Vin 0 5 0.1");
    }

    #[test]
    fn test_dc_sweep_nested() {
        let cfg = DcSweepConfig {
            source: "Vin".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
            source2: Some("Vbias".to_string()),
            start2: Some(0.0),
            stop2: Some(1.0),
            step2: Some(0.25),
        };
        assert_eq!(cfg.to_spice(), ".dc Vin 0 5 0.1 Vbias 0 1 0.25");
    }

    #[test]
    fn test_dc_sweep_validate() {
        let cfg = DcSweepConfig::default();
        assert!(cfg.validate().is_ok());

        let bad = DcSweepConfig {
            step: 0.0,
            ..Default::default()
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn test_dc_sweep_num_points() {
        let cfg = DcSweepConfig {
            start: 0.0,
            stop: 1.0,
            step: 0.1,
            ..Default::default()
        };
        assert_eq!(cfg.num_points(), 11);
    }

    #[test]
    fn test_transient_to_spice_basic() {
        let cfg = TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        };
        assert_eq!(cfg.to_spice(), ".tran 0.000000001 0.000001");
    }

    #[test]
    fn test_transient_to_spice_with_uic() {
        let cfg = TransientAnalysisConfig {
            uic: true,
            ..Default::default()
        };
        assert!(cfg.to_spice().contains("UIC"));
    }

    #[test]
    fn test_transient_validate() {
        let cfg = TransientAnalysisConfig::default();
        assert!(cfg.validate().is_ok());

        let bad = TransientAnalysisConfig {
            stop_time: -1.0,
            ..Default::default()
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn test_ac_to_spice() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        };
        assert_eq!(cfg.to_spice(), ".ac dec 10 1 1000000000");
    }

    #[test]
    fn test_ac_total_points_decade() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e3, // 3 decades
        };
        assert_eq!(cfg.total_points(), 31);
    }

    #[test]
    fn test_ac_validate() {
        let cfg = AcAnalysisConfig::default();
        assert!(cfg.validate().is_ok());

        let bad = AcAnalysisConfig {
            start_freq: 1e9,
            stop_freq: 1.0, // reversed
            ..Default::default()
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn test_noise_to_spice() {
        let cfg = NoiseAnalysisConfig {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "Vin".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e6,
        };
        assert_eq!(cfg.to_spice(), ".noise V(out,0) Vin dec 10 1 1000000");
    }

    #[test]
    fn test_pz_to_spice() {
        let cfg = PoleZeroConfig::default();
        assert!(cfg.to_spice().starts_with(".pz"));
        assert!(cfg.to_spice().contains("VOL"));
        assert!(cfg.to_spice().contains("PZ"));
    }

    #[test]
    fn test_pz_poles_only() {
        let cfg = PoleZeroConfig {
            analysis_type: PzAnalysisType::PolesOnly,
            ..Default::default()
        };
        assert!(cfg.to_spice().contains("POL"));
    }

    #[test]
    fn test_sensitivity_dc() {
        let cfg = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        };
        assert_eq!(cfg.to_spice(), ".sens V(out)");
    }

    #[test]
    fn test_sensitivity_ac() {
        let cfg = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e6),
        };
        assert!(cfg.to_spice().contains("AC"));
    }

    #[test]
    fn test_analysis_config_type() {
        let cfg = AnalysisConfig::Transient(TransientAnalysisConfig::default());
        assert_eq!(cfg.analysis_type(), AnalysisType::Transient);
    }

    #[test]
    fn test_analysis_config_validate() {
        let cfg = AnalysisConfig::DcOp;
        assert!(cfg.validate().is_ok());

        let cfg = AnalysisConfig::Transient(TransientAnalysisConfig::default());
        assert!(cfg.validate().is_ok());
    }

    //=========================================================================
    // AC Frequency Generation Tests - Commercial Grade Coverage
    //=========================================================================

    #[test]
    fn test_ac_generate_frequencies_decade_basic() {
        // 1Hz to 1kHz = 3 decades, 10 points per decade = 31 points
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1000.0,
        };

        let freqs = cfg.generate_frequencies();

        // Should have approximately 30 + 1 = 31 points for 3 decades
        assert!(!freqs.is_empty());
        assert!(
            freqs.len() >= 20,
            "Expected at least 20 points, got {}",
            freqs.len()
        );

        // First and last should match start/stop
        assert!((freqs[0] - 1.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 1000.0).abs() < 1e-6);

        // Should be monotonically increasing
        for i in 1..freqs.len() {
            assert!(
                freqs[i] > freqs[i - 1],
                "Not monotonically increasing at index {}",
                i
            );
        }
    }

    #[test]
    fn test_ac_generate_frequencies_decade_one_decade() {
        // 1Hz to 10Hz = 1 decade, 20 points per decade = 21 points
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 20,
            start_freq: 1.0,
            stop_freq: 10.0,
        };

        let freqs = cfg.generate_frequencies();

        assert_eq!(
            freqs.len(),
            21,
            "Expected 21 points for 1 decade with 20 pts/decade"
        );
        assert!((freqs[0] - 1.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_ac_generate_frequencies_octave_basic() {
        // 1Hz to 8Hz = 3 octaves, 10 points per octave
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Octave,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 8.0,
        };

        let freqs = cfg.generate_frequencies();

        assert!(!freqs.is_empty());
        assert!(
            freqs.len() >= 25,
            "Expected at least 25 points, got {}",
            freqs.len()
        );

        // First and last should match
        assert!((freqs[0] - 1.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 8.0).abs() < 1e-6);
    }

    #[test]
    fn test_ac_generate_frequencies_linear_basic() {
        // Linear sweep: 100 to 200 Hz, 11 points
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 11,
            start_freq: 100.0,
            stop_freq: 200.0,
        };

        let freqs = cfg.generate_frequencies();

        assert_eq!(freqs.len(), 11);
        assert!((freqs[0] - 100.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 200.0).abs() < 1e-10);

        // Linear sweep should have uniform spacing
        let step = (200.0 - 100.0) / 10.0;
        for i in 0..freqs.len() {
            let expected = 100.0 + i as f64 * step;
            assert!(
                (freqs[i] - expected).abs() < 1e-10,
                "Point {} expected {}, got {}",
                i,
                expected,
                freqs[i]
            );
        }
    }

    #[test]
    fn test_ac_generate_frequencies_linear_single_point() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 1,
            start_freq: 100.0,
            stop_freq: 200.0,
        };

        let freqs = cfg.generate_frequencies();

        assert_eq!(freqs.len(), 1);
        // Single point should be at start, adjusted to stop for final value
        assert!((freqs[0] - 200.0).abs() < 1e-10);
    }

    #[test]
    fn test_ac_generate_frequencies_linear_two_points() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 2,
            start_freq: 100.0,
            stop_freq: 200.0,
        };

        let freqs = cfg.generate_frequencies();

        assert_eq!(freqs.len(), 2);
        assert!((freqs[0] - 100.0).abs() < 1e-10);
        assert!((freqs[1] - 200.0).abs() < 1e-10);
    }

    #[test]
    fn test_ac_generate_frequencies_invalid_zero_start() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 0.0,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();
        assert!(
            freqs.is_empty(),
            "Should return empty for zero start frequency"
        );
    }

    #[test]
    fn test_ac_generate_frequencies_invalid_negative_start() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: -1.0,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();
        assert!(
            freqs.is_empty(),
            "Should return empty for negative start frequency"
        );
    }

    #[test]
    fn test_ac_generate_frequencies_invalid_start_greater_than_stop() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1000.0,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();
        assert!(freqs.is_empty(), "Should return empty when start > stop");
    }

    #[test]
    fn test_ac_generate_frequencies_invalid_start_equals_stop() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 100.0,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();
        assert!(freqs.is_empty(), "Should return empty when start == stop");
    }

    #[test]
    fn test_ac_generate_frequencies_linear_zero_points() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 0,
            start_freq: 100.0,
            stop_freq: 1000.0,
        };

        let freqs = cfg.generate_frequencies();
        assert!(freqs.is_empty(), "Should return empty for zero points");
    }

    #[test]
    fn test_ac_generate_frequencies_high_frequency_range() {
        // Typical RF frequency sweep: 1MHz to 10GHz
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1e6,
            stop_freq: 10e9,
        };

        let freqs = cfg.generate_frequencies();

        // 4 decades = 41 points
        assert!(freqs.len() >= 35);
        assert!((freqs[0] - 1e6).abs() < 1e-3);
        assert!((freqs.last().unwrap() - 10e9).abs() / 10e9 < 1e-6);
    }

    #[test]
    fn test_ac_generate_frequencies_sub_hz() {
        // Very low frequency sweep for power supply applications
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 0.01,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();

        // 4 decades = 41 points
        assert!(freqs.len() >= 35);
        assert!((freqs[0] - 0.01).abs() < 1e-12);
    }

    #[test]
    fn test_ac_generate_frequencies_decade_logarithmic_spacing() {
        // Verify truly logarithmic spacing
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 100.0,
        };

        let freqs = cfg.generate_frequencies();

        // The ratio between consecutive points should be approximately constant
        // for logarithmic spacing
        if freqs.len() >= 3 {
            let ratio1 = freqs[1] / freqs[0];
            let ratio2 = freqs[2] / freqs[1];
            // Ratios should be very close for log spacing
            assert!(
                (ratio1 - ratio2).abs() / ratio1 < 0.01,
                "Logarithmic spacing violated: ratios {} and {}",
                ratio1,
                ratio2
            );
        }
    }

    #[test]
    fn test_ac_total_points_decade_comprehensive() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1000.0, // 3 decades
        };

        let total = cfg.total_points();
        // 3 decades * 10 points/decade + 1 = 31
        assert_eq!(total, 31);
    }

    #[test]
    fn test_ac_total_points_octave_comprehensive() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Octave,
            num_points: 5,
            start_freq: 1.0,
            stop_freq: 8.0, // 3 octaves
        };

        let total = cfg.total_points();
        // 3 octaves * 5 points/octave + 1 = 16
        assert_eq!(total, 16);
    }

    #[test]
    fn test_ac_total_points_linear_comprehensive() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 50,
            start_freq: 100.0,
            stop_freq: 1000.0,
        };

        let total = cfg.total_points();
        assert_eq!(total, 50);
    }

    #[test]
    fn test_ac_validate_valid_config() {
        let cfg = AcAnalysisConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_ac_validate_zero_start() {
        let cfg = AcAnalysisConfig {
            start_freq: 0.0,
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("positive")));
    }

    #[test]
    fn test_ac_validate_zero_stop() {
        let cfg = AcAnalysisConfig {
            stop_freq: 0.0,
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_ac_validate_start_greater_than_stop() {
        let cfg = AcAnalysisConfig {
            start_freq: 1000.0,
            stop_freq: 100.0,
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("less than")));
    }

    #[test]
    fn test_ac_validate_zero_points() {
        let cfg = AcAnalysisConfig {
            num_points: 0,
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_ac_to_spice_decade() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        };
        let spice = cfg.to_spice();
        assert!(spice.starts_with(".ac"));
        assert!(spice.contains("dec"));
        assert!(spice.contains("10"));
    }

    #[test]
    fn test_ac_to_spice_octave() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Octave,
            num_points: 5,
            start_freq: 100.0,
            stop_freq: 10000.0,
        };
        let spice = cfg.to_spice();
        assert!(spice.contains("oct"));
    }

    #[test]
    fn test_ac_to_spice_linear() {
        let cfg = AcAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 100,
            start_freq: 1000.0,
            stop_freq: 2000.0,
        };
        let spice = cfg.to_spice();
        assert!(spice.contains("lin"));
    }

    //=========================================================================
    // Noise Analysis Tests - Commercial Grade Coverage
    //=========================================================================

    #[test]
    fn test_noise_generate_frequencies_decade() {
        let cfg = NoiseAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1000.0,
            ..Default::default()
        };

        let freqs = cfg.generate_frequencies();

        // 3 decades * 10 points/decade + 1
        assert!(freqs.len() >= 20);
        assert!((freqs[0] - 1.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 1000.0).abs() < 1e-6);

        // Monotonically increasing
        for i in 1..freqs.len() {
            assert!(freqs[i] > freqs[i - 1]);
        }
    }

    #[test]
    fn test_noise_generate_frequencies_octave() {
        let cfg = NoiseAnalysisConfig {
            sweep_type: AcSweepType::Octave,
            num_points: 5,
            start_freq: 100.0,
            stop_freq: 1600.0, // 4 octaves
            ..Default::default()
        };

        let freqs = cfg.generate_frequencies();

        assert!(!freqs.is_empty());
        assert!((freqs[0] - 100.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 1600.0).abs() < 1e-6);
    }

    #[test]
    fn test_noise_generate_frequencies_linear() {
        let cfg = NoiseAnalysisConfig {
            sweep_type: AcSweepType::Linear,
            num_points: 21,
            start_freq: 100.0,
            stop_freq: 1100.0,
            ..Default::default()
        };

        let freqs = cfg.generate_frequencies();

        assert_eq!(freqs.len(), 21);
        assert!((freqs[0] - 100.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 1100.0).abs() < 1e-10);

        // Uniform spacing
        let step = (1100.0 - 100.0) / 20.0; // 50 Hz
        for i in 0..freqs.len() {
            let expected = 100.0 + i as f64 * step;
            assert!((freqs[i] - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn test_noise_generate_frequencies_invalid() {
        // Zero start
        let cfg = NoiseAnalysisConfig {
            start_freq: 0.0,
            stop_freq: 1000.0,
            ..Default::default()
        };
        assert!(cfg.generate_frequencies().is_empty());

        // Negative start
        let cfg = NoiseAnalysisConfig {
            start_freq: -1.0,
            stop_freq: 1000.0,
            ..Default::default()
        };
        assert!(cfg.generate_frequencies().is_empty());

        // Start >= stop
        let cfg = NoiseAnalysisConfig {
            start_freq: 1000.0,
            stop_freq: 100.0,
            ..Default::default()
        };
        assert!(cfg.generate_frequencies().is_empty());
    }

    #[test]
    fn test_noise_validate_valid() {
        let cfg = NoiseAnalysisConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_noise_validate_missing_output_node() {
        let cfg = NoiseAnalysisConfig {
            output_node: "".to_string(),
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .iter()
            .any(|e| e.contains("Output node")));
    }

    #[test]
    fn test_noise_validate_missing_input_source() {
        let cfg = NoiseAnalysisConfig {
            input_source: "".to_string(),
            ..Default::default()
        };
        let result = cfg.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .iter()
            .any(|e| e.contains("Input source")));
    }

    #[test]
    fn test_noise_validate_frequency_range() {
        // Zero frequency
        let cfg = NoiseAnalysisConfig {
            start_freq: 0.0,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());

        // Negative frequency
        let cfg = NoiseAnalysisConfig {
            stop_freq: -100.0,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());

        // Start >= stop
        let cfg = NoiseAnalysisConfig {
            start_freq: 1e9,
            stop_freq: 1e6,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_noise_to_spice_comprehensive() {
        let cfg = NoiseAnalysisConfig {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "Vin".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e6,
        };
        let spice = cfg.to_spice();
        assert!(spice.starts_with(".noise"));
        assert!(spice.contains("V(out,0)"));
        assert!(spice.contains("Vin"));
        assert!(spice.contains("dec"));
    }

    #[test]
    fn test_noise_default_temperature() {
        let cfg = NoiseAnalysisConfig::default();
        assert_eq!(cfg.default_temperature(), 300.0); // 27°C
    }

    #[test]
    fn test_noise_audio_band() {
        // Standard audio band: 20Hz to 20kHz
        let cfg = NoiseAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 20,
            start_freq: 20.0,
            stop_freq: 20000.0,
            ..Default::default()
        };

        let freqs = cfg.generate_frequencies();

        // 3 decades * 20 points + 1
        assert!(freqs.len() >= 55);
        assert!((freqs[0] - 20.0).abs() < 1e-10);
        assert!((freqs.last().unwrap() - 20000.0).abs() < 1.0);
    }

    #[test]
    fn test_noise_rf_band() {
        // RF noise analysis: 1MHz to 1GHz
        let cfg = NoiseAnalysisConfig {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1e6,
            stop_freq: 1e9,
            ..Default::default()
        };

        let freqs = cfg.generate_frequencies();

        // 3 decades * 10 points + 1
        assert!(freqs.len() >= 25);
        // Verify precision at high frequencies
        assert!((freqs[0] / 1e6 - 1.0).abs() < 1e-6);
    }
}
