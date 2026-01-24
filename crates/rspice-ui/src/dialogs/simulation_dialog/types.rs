//! Types and Props for Simulation Dialog
//!
//! Contains the AnalysisTab enum and dialog props.

use dioxus::prelude::*;

use crate::state::simulation_command::SimulationConfig;

// =============================================================================
// Analysis Tab
// =============================================================================

/// Which analysis tab is currently active
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnalysisTab {
    #[default]
    Transient,
    Ac,
    DcSweep,
    Op,
    Noise,
    MonteCarlo,
    PoleZero,
    Sensitivity,
    SParam,
    // Advanced analysis types
    Pss,
    Pac,
    HarmonicBalance,
    Stb,
    Envelope,
    MultiRate,
    Corner,
    Transfer,
    Fourier,
    Parametric,
}

impl AnalysisTab {
    /// Get the display label for this tab
    pub fn label(&self) -> &'static str {
        match self {
            AnalysisTab::Transient => "Transient",
            AnalysisTab::Ac => "AC Analysis",
            AnalysisTab::DcSweep => "DC Sweep",
            AnalysisTab::Op => "Operating Point",
            AnalysisTab::Noise => "Noise",
            AnalysisTab::MonteCarlo => "Monte Carlo",
            AnalysisTab::PoleZero => "Pole-Zero",
            AnalysisTab::Sensitivity => "Sensitivity",
            AnalysisTab::SParam => "S-Param",
            AnalysisTab::Pss => "PSS",
            AnalysisTab::Pac => "PAC",
            AnalysisTab::HarmonicBalance => "Harmonic Balance",
            AnalysisTab::Stb => "STB (Stability)",
            AnalysisTab::Envelope => "Envelope",
            AnalysisTab::MultiRate => "Multi-Rate",
            AnalysisTab::Corner => "Corner",
            AnalysisTab::Transfer => "Transfer Func",
            AnalysisTab::Fourier => "Fourier/THD",
            AnalysisTab::Parametric => "Parametric",
        }
    }

    /// Get all analysis tabs in order
    pub fn all() -> &'static [AnalysisTab] {
        &[
            AnalysisTab::Transient,
            AnalysisTab::Ac,
            AnalysisTab::DcSweep,
            AnalysisTab::Op,
            AnalysisTab::Noise,
            AnalysisTab::MonteCarlo,
            AnalysisTab::PoleZero,
            AnalysisTab::Sensitivity,
            AnalysisTab::SParam,
            AnalysisTab::Pss,
            AnalysisTab::Pac,
            AnalysisTab::HarmonicBalance,
            AnalysisTab::Stb,
            AnalysisTab::Envelope,
            AnalysisTab::MultiRate,
            AnalysisTab::Corner,
            AnalysisTab::Transfer,
            AnalysisTab::Fourier,
            AnalysisTab::Parametric,
        ]
    }

    /// Check if this is a frequency-domain analysis
    pub fn is_frequency_domain(&self) -> bool {
        matches!(
            self,
            AnalysisTab::Ac
                | AnalysisTab::Noise
                | AnalysisTab::SParam
                | AnalysisTab::Pac
                | AnalysisTab::HarmonicBalance
                | AnalysisTab::Stb
        )
    }

    /// Check if this is a time-domain analysis
    pub fn is_time_domain(&self) -> bool {
        matches!(
            self,
            AnalysisTab::Transient | AnalysisTab::Envelope | AnalysisTab::MultiRate
        )
    }

    /// Check if this is a periodic analysis (requires PSS first)
    pub fn is_periodic_analysis(&self) -> bool {
        matches!(self, AnalysisTab::Pss | AnalysisTab::Pac)
    }
}

// =============================================================================
// Dialog Props
// =============================================================================

/// Props for the SimulationDialog component
#[derive(Props, Clone, PartialEq)]
pub struct SimulationDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current simulation configuration
    pub config: SimulationConfig,
    /// Called when OK is clicked with the new configuration
    pub on_confirm: EventHandler<SimulationConfig>,
    /// Called when Cancel is clicked
    pub on_cancel: EventHandler<()>,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_tab_label() {
        assert_eq!(AnalysisTab::Transient.label(), "Transient");
        assert_eq!(AnalysisTab::Ac.label(), "AC Analysis");
        assert_eq!(AnalysisTab::DcSweep.label(), "DC Sweep");
        assert_eq!(AnalysisTab::Op.label(), "Operating Point");
        // Test new tabs
        assert_eq!(AnalysisTab::Pss.label(), "PSS");
        assert_eq!(AnalysisTab::Pac.label(), "PAC");
        assert_eq!(AnalysisTab::HarmonicBalance.label(), "Harmonic Balance");
        assert_eq!(AnalysisTab::Stb.label(), "STB (Stability)");
        assert_eq!(AnalysisTab::Corner.label(), "Corner");
    }

    #[test]
    fn test_analysis_tab_all() {
        let all = AnalysisTab::all();
        assert_eq!(all.len(), 19);
        assert_eq!(all[0], AnalysisTab::Transient);
        assert_eq!(all[8], AnalysisTab::SParam);
        assert_eq!(all[9], AnalysisTab::Pss);
        assert_eq!(all[18], AnalysisTab::Parametric);
    }

    #[test]
    fn test_analysis_tab_is_frequency_domain() {
        assert!(AnalysisTab::Ac.is_frequency_domain());
        assert!(AnalysisTab::Noise.is_frequency_domain());
        assert!(AnalysisTab::SParam.is_frequency_domain());
        assert!(AnalysisTab::Pac.is_frequency_domain());
        assert!(AnalysisTab::HarmonicBalance.is_frequency_domain());
        assert!(AnalysisTab::Stb.is_frequency_domain());
        assert!(!AnalysisTab::Transient.is_frequency_domain());
        assert!(!AnalysisTab::DcSweep.is_frequency_domain());
    }

    #[test]
    fn test_analysis_tab_is_time_domain() {
        assert!(AnalysisTab::Transient.is_time_domain());
        assert!(AnalysisTab::Envelope.is_time_domain());
        assert!(AnalysisTab::MultiRate.is_time_domain());
        assert!(!AnalysisTab::Ac.is_time_domain());
    }

    #[test]
    fn test_analysis_tab_is_periodic_analysis() {
        assert!(AnalysisTab::Pss.is_periodic_analysis());
        assert!(AnalysisTab::Pac.is_periodic_analysis());
        assert!(!AnalysisTab::Transient.is_periodic_analysis());
        assert!(!AnalysisTab::Ac.is_periodic_analysis());
    }

    #[test]
    fn test_analysis_tab_default() {
        assert_eq!(AnalysisTab::default(), AnalysisTab::Transient);
    }
}
