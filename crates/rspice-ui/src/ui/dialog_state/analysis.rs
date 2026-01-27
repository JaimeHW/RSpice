//! Analysis Dialog State
//!
//! Centralized state management for all analysis types.
//! Each analysis has its own typed state struct for type safety
//! and clear parameter grouping.

use serde::{Deserialize, Serialize};

// =============================================================================
// Main Analysis State Container
// =============================================================================

/// Container for all analysis-specific dialog parameters
///
/// This groups all analysis types into a single container for easy
/// access while maintaining typed state for each analysis type.
#[derive(Debug, Clone, Default)]
pub struct AnalysisDialogState {
    /// Currently active analysis tab index
    pub active_tab: usize,

    /// Transient analysis parameters
    pub transient: TransientState,

    /// AC small-signal analysis parameters
    pub ac: AcState,

    /// DC sweep analysis parameters
    pub dc: DcState,

    /// Noise analysis parameters
    pub noise: NoiseState,

    /// Pole-zero analysis parameters  
    pub pole_zero: PoleZeroState,

    /// Sensitivity analysis parameters
    pub sensitivity: SensitivityState,

    /// Monte Carlo analysis parameters
    pub monte_carlo: MonteCarloState,

    /// PSS (Periodic Steady State) analysis parameters
    pub pss: PssState,

    /// STB (Stability) analysis parameters
    pub stb: StbState,

    /// Temperature sweep parameters
    pub temp_sweep: TempSweepState,
}

impl AnalysisDialogState {
    /// Get the currently selected analysis type
    pub fn current_analysis(&self) -> AnalysisType {
        AnalysisType::from_tab_index(self.active_tab)
    }

    /// Set the active analysis by type
    pub fn set_analysis(&mut self, analysis: AnalysisType) {
        self.active_tab = analysis.tab_index();
    }
}

// =============================================================================
// Analysis Type Enum
// =============================================================================

/// All supported analysis types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisType {
    /// DC Operating Point
    Op,
    /// Transient (time-domain)
    Transient,
    /// AC small-signal
    Ac,
    /// DC Sweep
    Dc,
    /// Noise analysis
    Noise,
    /// Pole-Zero
    PoleZero,
    /// Sensitivity
    Sensitivity,
    /// Monte Carlo
    MonteCarlo,
    /// Periodic Steady State
    Pss,
    /// Stability
    Stb,
    /// Temperature Sweep
    TempSweep,
}

impl AnalysisType {
    /// Get the tab index for this analysis type
    pub fn tab_index(self) -> usize {
        match self {
            Self::Op => 0,
            Self::Transient => 1,
            Self::Ac => 2,
            Self::Dc => 3,
            Self::Noise => 4,
            Self::PoleZero => 5,
            Self::Sensitivity => 6,
            Self::MonteCarlo => 7,
            Self::Pss => 8,
            Self::Stb => 9,
            Self::TempSweep => 10,
        }
    }

    /// Get analysis type from tab index
    pub fn from_tab_index(index: usize) -> Self {
        match index {
            0 => Self::Op,
            1 => Self::Transient,
            2 => Self::Ac,
            3 => Self::Dc,
            4 => Self::Noise,
            5 => Self::PoleZero,
            6 => Self::Sensitivity,
            7 => Self::MonteCarlo,
            8 => Self::Pss,
            9 => Self::Stb,
            10 => Self::TempSweep,
            _ => Self::Op, // Default fallback
        }
    }

    /// Get SPICE command keyword
    pub fn spice_keyword(self) -> &'static str {
        match self {
            Self::Op => ".op",
            Self::Transient => ".tran",
            Self::Ac => ".ac",
            Self::Dc => ".dc",
            Self::Noise => ".noise",
            Self::PoleZero => ".pz",
            Self::Sensitivity => ".sens",
            Self::MonteCarlo => ".mc",
            Self::Pss => ".pss",
            Self::Stb => ".stb",
            Self::TempSweep => ".temp",
        }
    }

    /// Get display name
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Op => "Operating Point",
            Self::Transient => "Transient",
            Self::Ac => "AC Analysis",
            Self::Dc => "DC Sweep",
            Self::Noise => "Noise",
            Self::PoleZero => "Pole-Zero",
            Self::Sensitivity => "Sensitivity",
            Self::MonteCarlo => "Monte Carlo",
            Self::Pss => "PSS",
            Self::Stb => "Stability",
            Self::TempSweep => "Temperature",
        }
    }

    /// All analysis types in order
    pub const ALL: [AnalysisType; 11] = [
        Self::Op,
        Self::Transient,
        Self::Ac,
        Self::Dc,
        Self::Noise,
        Self::PoleZero,
        Self::Sensitivity,
        Self::MonteCarlo,
        Self::Pss,
        Self::Stb,
        Self::TempSweep,
    ];
}

// =============================================================================
// Transient Analysis State
// =============================================================================

/// Transient (time-domain) analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientState {
    /// Stop time (e.g., "1ms", "10us")
    pub stop: String,
    /// Time step (e.g., "1ns")
    pub step: String,
    /// Start time (e.g., "0")
    pub start: String,
    /// Maximum step size (optional)
    pub max_step: String,
    /// Use initial conditions (UIC)
    pub uic: bool,
}

impl Default for TransientState {
    fn default() -> Self {
        Self {
            stop: "1ms".to_string(),
            step: "1us".to_string(),
            start: "0".to_string(),
            max_step: String::new(),
            uic: false,
        }
    }
}

impl TransientState {
    /// Generate SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(".tran {} {}", self.step, self.stop);
        if !self.start.is_empty() && self.start != "0" {
            cmd.push_str(&format!(" {}", self.start));
        }
        if !self.max_step.is_empty() {
            cmd.push_str(&format!(" {}", self.max_step));
        }
        if self.uic {
            cmd.push_str(" uic");
        }
        cmd
    }
}

// =============================================================================
// AC Analysis State
// =============================================================================

/// AC small-signal analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcState {
    /// Start frequency (e.g., "1Hz", "1k")
    pub f_start: String,
    /// Stop frequency (e.g., "1GHz", "10Meg")
    pub f_stop: String,
    /// Number of points per decade/octave or total for linear
    pub points: String,
    /// Sweep type
    pub sweep_type: AcSweepType,
}

impl Default for AcState {
    fn default() -> Self {
        Self {
            f_start: "1".to_string(),
            f_stop: "1G".to_string(),
            points: "10".to_string(),
            sweep_type: AcSweepType::Decade,
        }
    }
}

impl AcState {
    /// Generate SPICE command string
    pub fn to_spice_string(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.points,
            self.f_start,
            self.f_stop
        )
    }
}

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AcSweepType {
    #[default]
    Decade,
    Octave,
    Linear,
}

impl AcSweepType {
    /// SPICE keyword
    pub fn spice_keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    /// Display name
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    /// All sweep types
    pub const ALL: [AcSweepType; 3] = [Self::Decade, Self::Octave, Self::Linear];
}

// =============================================================================
// DC Sweep Analysis State
// =============================================================================

/// DC sweep analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcState {
    /// Primary source name (e.g., "V1")
    pub source: String,
    /// Start value (e.g., "-5")
    pub start: String,
    /// Stop value (e.g., "5")
    pub stop: String,
    /// Step value (e.g., "0.1")
    pub step: String,
    /// Nested sweep enabled
    pub nested: bool,
    /// Secondary source name
    pub source2: String,
    /// Secondary start value
    pub start2: String,
    /// Secondary stop value
    pub stop2: String,
    /// Secondary step value
    pub step2: String,
}

impl Default for DcState {
    fn default() -> Self {
        Self {
            source: "V1".to_string(),
            start: "0".to_string(),
            stop: "5".to_string(),
            step: "0.1".to_string(),
            nested: false,
            source2: String::new(),
            start2: String::new(),
            stop2: String::new(),
            step2: String::new(),
        }
    }
}

impl DcState {
    /// Generate SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(
            ".dc {} {} {} {}",
            self.source, self.start, self.stop, self.step
        );
        if self.nested && !self.source2.is_empty() {
            cmd.push_str(&format!(
                " {} {} {} {}",
                self.source2, self.start2, self.stop2, self.step2
            ));
        }
        cmd
    }
}

// =============================================================================
// Noise Analysis State
// =============================================================================

/// Noise analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseState {
    /// Output node
    pub output: String,
    /// Reference node (optional, defaults to ground)
    pub reference: String,
    /// Input source name
    pub input: String,
    /// Start frequency
    pub f_start: String,
    /// Stop frequency
    pub f_stop: String,
    /// Points per decade
    pub points: String,
}

impl Default for NoiseState {
    fn default() -> Self {
        Self {
            output: "out".to_string(),
            reference: String::new(),
            input: "V1".to_string(),
            f_start: "1".to_string(),
            f_stop: "1G".to_string(),
            points: "10".to_string(),
        }
    }
}

// =============================================================================
// Pole-Zero Analysis State
// =============================================================================

/// Pole-zero analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoleZeroState {
    /// Input node
    pub input: String,
    /// Output node
    pub output: String,
    /// Analysis type
    pub pz_type: PzType,
}

impl Default for PoleZeroState {
    fn default() -> Self {
        Self {
            input: "in".to_string(),
            output: "out".to_string(),
            pz_type: PzType::Both,
        }
    }
}

/// Pole-zero analysis type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PzType {
    #[default]
    Both,
    PolesOnly,
    ZerosOnly,
}

// =============================================================================
// Sensitivity Analysis State
// =============================================================================

/// Sensitivity analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityState {
    /// Output variable
    pub output: String,
    /// Analysis type (DC or AC)
    pub sens_type: SensType,
}

impl Default for SensitivityState {
    fn default() -> Self {
        Self {
            output: "V(out)".to_string(),
            sens_type: SensType::Dc,
        }
    }
}

/// Sensitivity analysis type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SensType {
    #[default]
    Dc,
    Ac,
}

// =============================================================================
// Monte Carlo Analysis State
// =============================================================================

/// Monte Carlo analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloState {
    /// Number of runs
    pub runs: String,
    /// Random seed (0 = auto)
    pub seed: String,
    /// Variation type
    pub variation: McVariation,
    /// Underlying analysis type
    pub analysis: AnalysisType,
}

impl Default for MonteCarloState {
    fn default() -> Self {
        Self {
            runs: "100".to_string(),
            seed: "0".to_string(),
            variation: McVariation::Gaussian,
            analysis: AnalysisType::Transient,
        }
    }
}

/// Monte Carlo variation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum McVariation {
    Uniform,
    #[default]
    Gaussian,
}

// =============================================================================
// PSS (Periodic Steady State) Analysis State
// =============================================================================

/// PSS analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PssState {
    /// Fundamental frequency
    pub fundamental: String,
    /// Number of harmonics
    pub harmonics: String,
    /// Oscillator mode
    pub osc_mode: bool,
    /// Maximum iterations
    pub max_iter: String,
}

impl Default for PssState {
    fn default() -> Self {
        Self {
            fundamental: "1MHz".to_string(),
            harmonics: "10".to_string(),
            osc_mode: false,
            max_iter: "100".to_string(),
        }
    }
}

// =============================================================================
// STB (Stability) Analysis State
// =============================================================================

/// Stability analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StbState {
    /// Probe source
    pub probe: String,
    /// Start frequency
    pub f_start: String,
    /// Stop frequency
    pub f_stop: String,
}

impl Default for StbState {
    fn default() -> Self {
        Self {
            probe: "V1".to_string(),
            f_start: "1".to_string(),
            f_stop: "1G".to_string(),
        }
    }
}

// =============================================================================
// Temperature Sweep State
// =============================================================================

/// Temperature sweep parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempSweepState {
    /// Start temperature (°C)
    pub start: String,
    /// Stop temperature (°C)
    pub stop: String,
    /// Temperature step (°C)
    pub step: String,
    /// Underlying analysis type
    pub analysis: AnalysisType,
}

impl Default for TempSweepState {
    fn default() -> Self {
        Self {
            start: "-40".to_string(),
            stop: "125".to_string(),
            step: "25".to_string(),
            analysis: AnalysisType::Transient,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Analysis Type Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_analysis_type_tab_index_roundtrip() {
        for analysis in AnalysisType::ALL {
            let index = analysis.tab_index();
            let recovered = AnalysisType::from_tab_index(index);
            assert_eq!(
                analysis, recovered,
                "Tab index roundtrip failed for {:?}",
                analysis
            );
        }
    }

    #[test]
    fn test_analysis_type_spice_keywords() {
        assert_eq!(AnalysisType::Op.spice_keyword(), ".op");
        assert_eq!(AnalysisType::Transient.spice_keyword(), ".tran");
        assert_eq!(AnalysisType::Ac.spice_keyword(), ".ac");
        assert_eq!(AnalysisType::Dc.spice_keyword(), ".dc");
        assert_eq!(AnalysisType::Noise.spice_keyword(), ".noise");
    }

    #[test]
    fn test_analysis_type_display_names() {
        assert_eq!(AnalysisType::Op.display_name(), "Operating Point");
        assert_eq!(AnalysisType::Transient.display_name(), "Transient");
        assert_eq!(AnalysisType::Ac.display_name(), "AC Analysis");
    }

    #[test]
    fn test_analysis_type_from_invalid_index_defaults_to_op() {
        assert_eq!(AnalysisType::from_tab_index(999), AnalysisType::Op);
    }

    // -------------------------------------------------------------------------
    // Transient State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_transient_state_default() {
        let state = TransientState::default();
        assert_eq!(state.stop, "1ms");
        assert_eq!(state.step, "1us");
        assert_eq!(state.start, "0");
        assert!(!state.uic);
    }

    #[test]
    fn test_transient_state_spice_string_basic() {
        let state = TransientState {
            stop: "1ms".to_string(),
            step: "1us".to_string(),
            start: "0".to_string(),
            max_step: String::new(),
            uic: false,
        };
        assert_eq!(state.to_spice_string(), ".tran 1us 1ms");
    }

    #[test]
    fn test_transient_state_spice_string_with_start() {
        let state = TransientState {
            stop: "10ms".to_string(),
            step: "1us".to_string(),
            start: "1ms".to_string(),
            max_step: String::new(),
            uic: false,
        };
        assert_eq!(state.to_spice_string(), ".tran 1us 10ms 1ms");
    }

    #[test]
    fn test_transient_state_spice_string_with_max_step() {
        let state = TransientState {
            stop: "1ms".to_string(),
            step: "1us".to_string(),
            start: "0".to_string(),
            max_step: "10ns".to_string(),
            uic: false,
        };
        assert_eq!(state.to_spice_string(), ".tran 1us 1ms 10ns");
    }

    #[test]
    fn test_transient_state_spice_string_with_uic() {
        let state = TransientState {
            stop: "1ms".to_string(),
            step: "1us".to_string(),
            start: "0".to_string(),
            max_step: String::new(),
            uic: true,
        };
        assert_eq!(state.to_spice_string(), ".tran 1us 1ms uic");
    }

    // -------------------------------------------------------------------------
    // AC State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_ac_state_default() {
        let state = AcState::default();
        assert_eq!(state.f_start, "1");
        assert_eq!(state.f_stop, "1G");
        assert_eq!(state.points, "10");
        assert_eq!(state.sweep_type, AcSweepType::Decade);
    }

    #[test]
    fn test_ac_state_spice_string() {
        let state = AcState {
            f_start: "1".to_string(),
            f_stop: "1G".to_string(),
            points: "10".to_string(),
            sweep_type: AcSweepType::Decade,
        };
        assert_eq!(state.to_spice_string(), ".ac dec 10 1 1G");
    }

    #[test]
    fn test_ac_sweep_type_spice_keywords() {
        assert_eq!(AcSweepType::Decade.spice_keyword(), "dec");
        assert_eq!(AcSweepType::Octave.spice_keyword(), "oct");
        assert_eq!(AcSweepType::Linear.spice_keyword(), "lin");
    }

    // -------------------------------------------------------------------------
    // DC State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_dc_state_default() {
        let state = DcState::default();
        assert_eq!(state.source, "V1");
        assert_eq!(state.start, "0");
        assert_eq!(state.stop, "5");
        assert_eq!(state.step, "0.1");
        assert!(!state.nested);
    }

    #[test]
    fn test_dc_state_spice_string_basic() {
        let state = DcState::default();
        assert_eq!(state.to_spice_string(), ".dc V1 0 5 0.1");
    }

    #[test]
    fn test_dc_state_spice_string_nested() {
        let state = DcState {
            source: "V1".to_string(),
            start: "0".to_string(),
            stop: "5".to_string(),
            step: "0.1".to_string(),
            nested: true,
            source2: "V2".to_string(),
            start2: "-2".to_string(),
            stop2: "2".to_string(),
            step2: "0.5".to_string(),
        };
        assert_eq!(state.to_spice_string(), ".dc V1 0 5 0.1 V2 -2 2 0.5");
    }

    // -------------------------------------------------------------------------
    // Analysis Dialog State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_analysis_dialog_state_default() {
        let state = AnalysisDialogState::default();
        assert_eq!(state.active_tab, 0);
        assert_eq!(state.current_analysis(), AnalysisType::Op);
    }

    #[test]
    fn test_analysis_dialog_state_set_analysis() {
        let mut state = AnalysisDialogState::default();
        state.set_analysis(AnalysisType::Transient);
        assert_eq!(state.current_analysis(), AnalysisType::Transient);
        assert_eq!(state.active_tab, 1);
    }
}
