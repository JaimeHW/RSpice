//! Integrated Viewer State
//!
//! Manages the active specialized viewer and provides unified access
//! to all viewer states for commercial-grade integration.

use super::bode::state::BodePlotState;
use super::cross_probe::CrossProbeState;
use super::eye_diagram::state::EyeDiagramState;
use super::fft::state::FftState;
use super::histogram::state::HistogramState;
use super::nyquist::state::NyquistState;
use super::pole_zero::state::PoleZeroState;
use super::simulation_dialog::ac::AcConfig;
use super::simulation_dialog::corner::CornerConfig;
use super::simulation_dialog::dc::DcConfig;
use super::simulation_dialog::envelope::EnvelopeConfig;
use super::simulation_dialog::fourier::FourierConfig;
use super::simulation_dialog::framework::SimulationDialog;
use super::simulation_dialog::hb::HbConfig;
use super::simulation_dialog::noise::NoiseConfig;
use super::simulation_dialog::pac::PacConfig;
use super::simulation_dialog::pnoise::PnoiseConfig;
use super::simulation_dialog::pstb::PstbConfig;
use super::simulation_dialog::pxf::PxfConfig;
use super::simulation_dialog::sp::SpConfig;
use super::simulation_dialog::transient::TransientConfig;
use super::simulation_dialog::xf::XfConfig;
use super::smith_chart::state::SmithChartState;

// =============================================================================
// Active Viewer
// =============================================================================

/// Currently active viewer in the waveform panel
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActiveViewer {
    /// Standard waveform viewer
    #[default]
    Waveform,
    /// Smith chart (RF/S-parameters)
    SmithChart,
    /// Eye diagram (signal integrity)
    EyeDiagram,
    /// Histogram (statistical)
    Histogram,
    /// Bode plot (frequency response)
    BodePlot,
    /// Nyquist plot (stability)
    Nyquist,
    /// FFT spectrum
    Fft,
    /// Pole-zero map
    PoleZero,
}

impl ActiveViewer {
    /// Display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Waveform => "Waveform",
            Self::SmithChart => "Smith Chart",
            Self::EyeDiagram => "Eye Diagram",
            Self::Histogram => "Histogram",
            Self::BodePlot => "Bode Plot",
            Self::Nyquist => "Nyquist",
            Self::Fft => "FFT Spectrum",
            Self::PoleZero => "Pole-Zero",
        }
    }

    /// All viewer types
    pub fn all() -> &'static [ActiveViewer] {
        &[
            Self::Waveform,
            Self::SmithChart,
            Self::EyeDiagram,
            Self::Histogram,
            Self::BodePlot,
            Self::Nyquist,
            Self::Fft,
            Self::PoleZero,
        ]
    }

    /// Does this viewer need AC analysis data?
    pub fn needs_ac_data(&self) -> bool {
        matches!(
            self,
            Self::SmithChart | Self::BodePlot | Self::Nyquist | Self::PoleZero
        )
    }

    /// Does this viewer need transient data?
    pub fn needs_transient_data(&self) -> bool {
        matches!(
            self,
            Self::Waveform | Self::EyeDiagram | Self::Histogram | Self::Fft
        )
    }
}

// =============================================================================
// Viewer State Container
// =============================================================================

/// Container for all viewer states
#[derive(Default)]
pub struct ViewerStates {
    /// Currently active viewer
    pub active: ActiveViewer,
    /// Smith chart state
    pub smith_chart: SmithChartState,
    /// Eye diagram state
    pub eye_diagram: EyeDiagramState,
    /// Histogram state
    pub histogram: HistogramState,
    /// Bode plot state
    pub bode: BodePlotState,
    /// Nyquist plot state
    pub nyquist: NyquistState,
    /// FFT state
    pub fft: FftState,
    /// Pole-zero state
    pub pole_zero: PoleZeroState,
}

impl ViewerStates {
    /// Create new viewer states
    pub fn new() -> Self {
        Self::default()
    }

    /// Switch to a different viewer
    pub fn switch_to(&mut self, viewer: ActiveViewer) {
        self.active = viewer;
    }

    /// Get active viewer name
    pub fn active_name(&self) -> &'static str {
        self.active.name()
    }
}

// =============================================================================
// Analysis Tab Index
// =============================================================================

/// Analysis tab indices for commercial simulator organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum AnalysisTab {
    Transient = 0,
    Ac = 1,
    Dc = 2,
    Noise = 3,
    HarmonicBalance = 4,
    SParameter = 5,
    PeriodicAc = 6,
    PeriodicNoise = 7,
    PeriodicXf = 8,
    PeriodicStability = 9,
    TransferFunction = 10,
    Corner = 11,
    Envelope = 12,
    Fourier = 13,
}

impl AnalysisTab {
    /// All tab names for display
    pub fn all_names() -> &'static [&'static str] {
        &[
            "Transient",
            "AC",
            "DC",
            "Noise",
            "HB",
            "S-Param",
            "PAC",
            "PNoise",
            "PXF",
            "PSTB",
            "XF",
            "Corner",
            "Envelope",
            "Fourier",
        ]
    }

    /// Number of tabs
    pub fn count() -> usize {
        14
    }

    /// From index
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Transient),
            1 => Some(Self::Ac),
            2 => Some(Self::Dc),
            3 => Some(Self::Noise),
            4 => Some(Self::HarmonicBalance),
            5 => Some(Self::SParameter),
            6 => Some(Self::PeriodicAc),
            7 => Some(Self::PeriodicNoise),
            8 => Some(Self::PeriodicXf),
            9 => Some(Self::PeriodicStability),
            10 => Some(Self::TransferFunction),
            11 => Some(Self::Corner),
            12 => Some(Self::Envelope),
            13 => Some(Self::Fourier),
            _ => None,
        }
    }
}

// =============================================================================
// Simulation Config Container
// =============================================================================

/// Container for all simulation configurations
#[derive(Default)]
pub struct SimulationConfigs {
    // Core analyses
    pub transient: TransientConfig,
    pub ac: AcConfig,
    pub dc: DcConfig,
    pub noise: NoiseConfig,
    // Steady-state
    pub hb: HbConfig,
    // RF
    pub sp: SpConfig,
    // Periodic small-signal
    pub pac: PacConfig,
    pub pnoise: PnoiseConfig,
    pub pxf: PxfConfig,
    pub pstb: PstbConfig,
    // Transfer function
    pub xf: XfConfig,
    // Statistical
    pub corner: CornerConfig,
    // Post-processing
    pub envelope: EnvelopeConfig,
    pub fourier: FourierConfig,
    // Dialog state
    pub dialog: SimulationDialog,
    pub active_tab: usize,
}

impl SimulationConfigs {
    /// Create new with defaults
    pub fn new() -> Self {
        let mut dialog = SimulationDialog::new("Simulation Setup");
        for name in AnalysisTab::all_names() {
            dialog.add_tab(name);
        }
        Self {
            dialog,
            ..Default::default()
        }
    }

    /// Open simulation dialog
    pub fn open_dialog(&mut self) {
        self.dialog.open();
    }

    /// Get current analysis tab
    pub fn current_tab(&self) -> Option<AnalysisTab> {
        AnalysisTab::from_index(self.active_tab)
    }

    /// Validate current config
    pub fn validate_current(&self) -> Result<(), String> {
        match AnalysisTab::from_index(self.active_tab) {
            Some(AnalysisTab::Transient) => self.transient.validate(),
            Some(AnalysisTab::Ac) => self.ac.validate(),
            Some(AnalysisTab::Dc) => self.dc.validate(),
            Some(AnalysisTab::Noise) => self.noise.validate(),
            Some(AnalysisTab::HarmonicBalance) => self.hb.validate(),
            Some(AnalysisTab::SParameter) => self.sp.validate(),
            Some(AnalysisTab::PeriodicAc) => self.pac.validate(),
            Some(AnalysisTab::PeriodicNoise) => self.pnoise.validate(),
            Some(AnalysisTab::PeriodicXf) => self.pxf.validate(),
            Some(AnalysisTab::PeriodicStability) => self.pstb.validate(),
            Some(AnalysisTab::TransferFunction) => self.xf.validate(),
            Some(AnalysisTab::Corner) => self.corner.validate(),
            Some(AnalysisTab::Envelope) => self.envelope.validate(),
            Some(AnalysisTab::Fourier) => self.fourier.validate(),
            None => Ok(()),
        }
    }

    /// Generate SPICE for current config
    pub fn current_spice(&self) -> String {
        match AnalysisTab::from_index(self.active_tab) {
            Some(AnalysisTab::Transient) => self.transient.to_spice(),
            Some(AnalysisTab::Ac) => self.ac.to_spice(),
            Some(AnalysisTab::Dc) => self.dc.to_spice(),
            Some(AnalysisTab::Noise) => self.noise.to_spice(),
            Some(AnalysisTab::HarmonicBalance) => self.hb.to_spice(),
            Some(AnalysisTab::SParameter) => self.sp.to_spice(),
            Some(AnalysisTab::PeriodicAc) => self.pac.to_spice(),
            Some(AnalysisTab::PeriodicNoise) => self.pnoise.to_spice(),
            Some(AnalysisTab::PeriodicXf) => self.pxf.to_spice(),
            Some(AnalysisTab::PeriodicStability) => self.pstb.to_spice(),
            Some(AnalysisTab::TransferFunction) => self.xf.to_spice(),
            Some(AnalysisTab::Corner) => String::new(), // Corner runs other analyses
            Some(AnalysisTab::Envelope) => self.envelope.to_spice(),
            Some(AnalysisTab::Fourier) => self.fourier.to_spice(),
            None => String::new(),
        }
    }
}

// =============================================================================
// Integrated State
// =============================================================================

/// Complete integrated viewer and simulation state
#[derive(Default)]
pub struct IntegratedViewerState {
    /// All viewer states
    pub viewers: ViewerStates,
    /// Cross-probe state
    pub cross_probe: CrossProbeState,
    /// Simulation configurations
    pub sim_configs: SimulationConfigs,
}

impl IntegratedViewerState {
    /// Create new integrated state
    pub fn new() -> Self {
        Self {
            viewers: ViewerStates::new(),
            cross_probe: CrossProbeState::new(),
            sim_configs: SimulationConfigs::new(),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_viewer_default() {
        let av = ActiveViewer::default();
        assert_eq!(av, ActiveViewer::Waveform);
    }

    #[test]
    fn test_active_viewer_all() {
        let all = ActiveViewer::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn test_active_viewer_names() {
        assert_eq!(ActiveViewer::SmithChart.name(), "Smith Chart");
        assert_eq!(ActiveViewer::EyeDiagram.name(), "Eye Diagram");
    }

    #[test]
    fn test_active_viewer_needs_ac() {
        assert!(ActiveViewer::SmithChart.needs_ac_data());
        assert!(ActiveViewer::BodePlot.needs_ac_data());
        assert!(!ActiveViewer::EyeDiagram.needs_ac_data());
    }

    #[test]
    fn test_active_viewer_needs_transient() {
        assert!(ActiveViewer::Waveform.needs_transient_data());
        assert!(ActiveViewer::Fft.needs_transient_data());
        assert!(!ActiveViewer::Nyquist.needs_transient_data());
    }

    #[test]
    fn test_viewer_states_new() {
        let vs = ViewerStates::new();
        assert_eq!(vs.active, ActiveViewer::Waveform);
    }

    #[test]
    fn test_viewer_states_switch() {
        let mut vs = ViewerStates::new();
        vs.switch_to(ActiveViewer::SmithChart);
        assert_eq!(vs.active, ActiveViewer::SmithChart);
    }

    #[test]
    fn test_analysis_tab_count() {
        assert_eq!(AnalysisTab::count(), 14);
        assert_eq!(AnalysisTab::all_names().len(), 14);
    }

    #[test]
    fn test_analysis_tab_from_index() {
        assert_eq!(AnalysisTab::from_index(0), Some(AnalysisTab::Transient));
        assert_eq!(
            AnalysisTab::from_index(4),
            Some(AnalysisTab::HarmonicBalance)
        );
        assert_eq!(AnalysisTab::from_index(100), None);
    }

    #[test]
    fn test_simulation_configs_new() {
        let sc = SimulationConfigs::new();
        assert_eq!(sc.dialog.tab_count(), 14);
    }

    #[test]
    fn test_simulation_configs_validate_all() {
        let mut sc = SimulationConfigs::new();
        for i in 0..14 {
            sc.active_tab = i;
            assert!(sc.validate_current().is_ok(), "Tab {} failed validation", i);
        }
    }

    #[test]
    fn test_simulation_configs_spice_transient() {
        let sc = SimulationConfigs::new();
        let spice = sc.current_spice();
        assert!(spice.starts_with(".tran"));
    }

    #[test]
    fn test_simulation_configs_spice_hb() {
        let mut sc = SimulationConfigs::new();
        sc.active_tab = AnalysisTab::HarmonicBalance as usize;
        let spice = sc.current_spice();
        assert!(spice.starts_with(".hb"));
    }

    #[test]
    fn test_simulation_configs_spice_sp() {
        let mut sc = SimulationConfigs::new();
        sc.active_tab = AnalysisTab::SParameter as usize;
        let spice = sc.current_spice();
        assert!(spice.starts_with(".sp"));
    }

    #[test]
    fn test_integrated_state_new() {
        let is = IntegratedViewerState::new();
        assert_eq!(is.viewers.active, ActiveViewer::Waveform);
    }
}
