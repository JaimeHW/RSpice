//! Simulation setup — the typed analysis-configuration model behind the
//! Simulate view.
//!
//! Every analysis owns exactly one state struct: draft string buffers plus
//! a `to_config()` parse/validate step. The Simulate view edits these
//! structs and the controller consumes the very same structs when it
//! builds the run plan, so what you see is what runs. `enabled` is the run
//! set; run order is ascending analysis index.

use std::collections::HashSet;

/// `.tran` draft. SI suffixes allowed; "auto" max step defers to the
/// engine's LTE control.
#[derive(Debug, Clone)]
pub struct TranSetup {
    /// Stop time.
    pub stop: String,
    /// Suggested step time.
    pub step: String,
    /// Output start time.
    pub start: String,
    /// Max timestep, or "auto".
    pub max_step: String,
    /// Skip the DC operating point and use initial conditions.
    pub uic: bool,
}

impl Default for TranSetup {
    fn default() -> Self {
        Self {
            stop: "1m".to_owned(),
            step: "10n".to_owned(),
            start: "0".to_owned(),
            max_step: "auto".to_owned(),
            uic: false,
        }
    }
}

/// `.ac` draft — DISTO rides on the same sweep.
#[derive(Debug, Clone)]
pub struct AcSetup {
    /// Start frequency.
    pub fstart: String,
    /// Stop frequency.
    pub fstop: String,
    /// Points per decade/octave, or total when linear.
    pub points: String,
    /// 0 = decade, 1 = octave, 2 = linear.
    pub sweep: usize,
}

impl Default for AcSetup {
    fn default() -> Self {
        Self {
            fstart: "1".to_owned(),
            fstop: "1G".to_owned(),
            points: "101".to_owned(),
            sweep: 0,
        }
    }
}

/// `.dc` draft with the optional nested secondary sweep.
#[derive(Debug, Clone)]
pub struct DcSetup {
    /// Swept source name.
    pub source: String,
    /// Sweep start value.
    pub start: String,
    /// Sweep stop value.
    pub stop: String,
    /// Sweep step.
    pub step: String,
    /// Nested secondary sweep enabled.
    pub nested: bool,
    /// Secondary source name.
    pub source2: String,
    /// Secondary start.
    pub start2: String,
    /// Secondary stop.
    pub stop2: String,
    /// Secondary step.
    pub step2: String,
}

impl Default for DcSetup {
    fn default() -> Self {
        Self {
            source: "V1".to_owned(),
            start: "0".to_owned(),
            stop: "5".to_owned(),
            step: "0.01".to_owned(),
            nested: false,
            source2: "V2".to_owned(),
            start2: "0".to_owned(),
            stop2: "3.3".to_owned(),
            step2: "0.1".to_owned(),
        }
    }
}

/// `.noise` draft.
#[derive(Debug, Clone)]
pub struct NoiseSetup {
    /// Output node.
    pub output: String,
    /// Reference node (0 = ground).
    pub reference: String,
    /// Input source.
    pub input: String,
    /// Start frequency.
    pub fstart: String,
    /// Stop frequency.
    pub fstop: String,
}

impl Default for NoiseSetup {
    fn default() -> Self {
        Self {
            output: "out".to_owned(),
            reference: "0".to_owned(),
            input: "V1".to_owned(),
            fstart: "1".to_owned(),
            fstop: "100Meg".to_owned(),
        }
    }
}

/// All analysis configuration plus the engine options, in one place.
#[derive(Debug, Clone, Default)]
pub struct SimSetupState {
    /// Enabled analysis indices; run order is ascending index.
    pub enabled: HashSet<usize>,
    /// Transient sweep.
    pub tran: TranSetup,
    /// AC sweep.
    pub ac: AcSetup,
    /// DISTO secondary tone ratio f2/f1 (empty = single-tone HD).
    pub disto_f2_over_f1: String,
    /// DC transfer sweep.
    pub dc: DcSetup,
    /// Noise analysis.
    pub noise: NoiseSetup,
    /// DC operating point.
    pub op: crate::simulation::dialog::op::OpDialogState,
    /// Pole-zero extraction.
    pub pz: crate::simulation::dialog::pz::PzDialogState,
    /// Sensitivity.
    pub sens: crate::simulation::dialog::sens::SensDialogState,
    /// Monte Carlo.
    pub mc: crate::simulation::dialog::mc::McDialogState,
    /// Periodic steady state.
    pub pss: crate::simulation::dialog::pss::PssDialogState,
    /// Loop stability.
    pub stb: crate::simulation::dialog::stb::StbDialogState,
    /// Temperature sweep.
    pub temp: crate::simulation::dialog::temp::TempDialogState,
    /// Harmonic balance.
    pub hb: crate::simulation::dialog::hb::HbDialogState,
    /// S-parameters.
    pub sp: crate::simulation::dialog::sp::SpDialogState,
    /// Periodic AC.
    pub pac: crate::simulation::dialog::pac::PacDialogState,
    /// Periodic noise.
    pub pnoise: crate::simulation::dialog::pnoise::PnoiseDialogState,
    /// Periodic transfer.
    pub pxf: crate::simulation::dialog::pxf::PxfDialogState,
    /// Periodic stability.
    pub pstb: crate::simulation::dialog::pstb::PstbDialogState,
    /// Transfer function.
    pub xf: crate::simulation::dialog::xf::XfDialogState,
    /// Process corners.
    pub corner: crate::simulation::dialog::corner::CornerDialogState,
    /// Envelope transient.
    pub envelope: crate::simulation::dialog::envelope::EnvelopeDialogState,
    /// Fourier.
    pub fourier: crate::simulation::dialog::fourier::FourierDialogState,
    /// Reliability / aging.
    pub reliability: crate::simulation::dialog::reliability::ReliabilityDialogState,
    /// Optimization.
    pub optimization: crate::simulation::dialog::optimization::OptimizationDialogState,
    /// Safe operating area.
    pub soa: crate::simulation::dialog::soa::SoaDialogState,
    /// Effective engine options (validated).
    pub options: crate::simulation::dialog::SimulationOptions,
    /// Draft buffers for the options dialog.
    pub options_draft: crate::simulation::dialog::OptionsDialogState,
    /// Parse/validation errors from the last options apply.
    pub options_errors: Vec<String>,
    /// Options dialog open.
    pub options_open: bool,
    /// Add-analysis picker open.
    pub picker_open: bool,
    /// Picker search query.
    pub picker_query: String,
}

impl SimSetupState {
    /// Fill defaults into any analysis state still at its blank
    /// `Default` (each guards on its own `initialized` flag).
    pub fn ensure_initialized(&mut self) {
        self.op.ensure_initialized();
        self.pz.ensure_initialized();
        self.sens.ensure_initialized();
        self.mc.ensure_initialized();
        self.pss.ensure_initialized();
        self.stb.ensure_initialized();
        self.temp.ensure_initialized();
        self.hb.ensure_initialized();
        self.sp.ensure_initialized();
        self.pac.ensure_initialized();
        self.pnoise.ensure_initialized();
        self.pxf.ensure_initialized();
        self.pstb.ensure_initialized();
        self.xf.ensure_initialized();
        self.corner.ensure_initialized();
        self.envelope.ensure_initialized();
        self.fourier.ensure_initialized();
        self.reliability.ensure_initialized();
        self.optimization.ensure_initialized();
        self.soa.ensure_initialized();
    }
}
