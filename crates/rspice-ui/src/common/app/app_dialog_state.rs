//! App Dialog State
//!
//! Large dialog/form state payload used by `AppState`.

use super::ConfirmationDialogState;

/// Dialog visibility state

#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// Simulation setup dialog
    pub simulation_dialog: bool,
    /// Simulation options dialog
    pub simulation_options: bool,
    /// About dialog
    pub about: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// New Cell creation dialog
    pub new_cell_dialog: bool,
    /// New Cell name input
    pub new_cell_name: String,
    /// New Cell target library
    pub new_cell_library: String,
    /// New Cell description
    pub new_cell_description: String,
    /// New Cell view types to create
    pub new_cell_create_schematic: bool,
    /// Create symbol view for new cell
    pub new_cell_create_symbol: bool,
    /// Create testbench view for new cell
    pub new_cell_create_testbench: bool,
    /// New Cell validation error message
    pub new_cell_error: Option<String>,
    /// New View creation dialog
    pub new_view_dialog: bool,
    /// New View target library
    pub new_view_library: String,
    /// New View target cell
    pub new_view_cell: String,
    /// New View name input
    pub new_view_name: String,
    /// New View type selection
    pub new_view_type: crate::state::ViewType,
    /// New View validation error message
    pub new_view_error: Option<String>,
    /// Active simulation tab (0=OP, 1=Tran, 2=AC, 3=DC, 4=Noise, 5=PZ, 6=Sens, 7=MC, 8=PSS, 9=STB, 10=Temp, 24=DISTO)
    pub sim_active_tab: usize,
    /// Set of enabled analysis indices
    pub enabled_analyses: std::collections::HashSet<usize>,
    // --- Transient Analysis ---
    /// Transient stop time
    pub tran_stop: String,
    /// Transient step time
    pub tran_step: String,
    /// Transient start time
    pub tran_start: String,
    /// Transient max step
    pub tran_maxstep: String,
    /// Use initial conditions
    pub tran_uic: bool,
    // --- AC Analysis ---
    /// AC start frequency
    pub ac_fstart: String,
    /// AC stop frequency
    pub ac_fstop: String,
    /// AC points per decade
    pub ac_points: String,
    /// AC sweep type (0=decade, 1=octave, 2=linear)
    pub ac_sweep_type: usize,
    /// DISTO secondary tone ratio (f2/f1), optional
    pub disto_f2_over_f1: String,
    // --- DC Analysis ---
    /// DC source name
    pub dc_source: String,
    /// DC start value
    pub dc_start: String,
    /// DC stop value
    pub dc_stop: String,
    /// DC step value
    pub dc_step: String,
    /// DC nested sweep enabled
    pub dc_nested: bool,
    /// DC source 2 name
    pub dc_source2: String,
    /// DC start2 value
    pub dc_start2: String,
    /// DC stop2 value
    pub dc_stop2: String,
    /// DC step2 value
    pub dc_step2: String,
    // --- Noise Analysis ---
    /// Noise output node
    pub noise_output: String,
    /// Noise reference node
    pub noise_ref: String,
    /// Noise input source
    pub noise_input: String,
    /// Noise frequency start
    pub noise_fstart: String,
    /// Noise frequency stop
    pub noise_fstop: String,
    // --- Pole-Zero Analysis ---
    /// PZ input node
    pub pz_input: String,
    /// PZ output node
    pub pz_output: String,
    /// PZ type (0=both, 1=poles only, 2=zeros only)
    pub pz_type: usize,
    // --- Sensitivity Analysis ---
    /// Sensitivity output variable
    pub sens_output: String,
    /// Sensitivity type (0=DC, 1=AC)
    pub sens_type: usize,
    // --- Monte Carlo Analysis ---
    /// Number of MC runs
    pub mc_runs: String,
    /// MC seed (0=random)
    pub mc_seed: String,
    /// MC variation type (0=uniform, 1=gaussian)
    pub mc_variation: usize,
    /// MC analysis type (0=tran, 1=ac, 2=dc)
    pub mc_analysis: usize,
    // --- PSS (Periodic Steady State) Analysis ---
    /// PSS fundamental frequency
    pub pss_fund: String,
    /// PSS number of harmonics
    pub pss_harmonics: String,
    /// PSS oscillator mode
    pub pss_oscmode: bool,
    /// PSS max iterations
    pub pss_maxiter: String,
    // --- STB (Stability) Analysis ---
    /// STB probe source
    pub stb_probe: String,
    /// STB start frequency
    pub stb_fstart: String,
    /// STB stop frequency
    pub stb_fstop: String,
    // --- Temperature Sweep ---
    /// Temperature start (°C)
    pub temp_start: String,
    /// Temperature stop (°C)
    pub temp_stop: String,
    /// Temperature step (°C)
    pub temp_step: String,
    /// Temp sweep analysis type
    pub temp_analysis: usize,
    // --- Harmonic Balance (HB) ---
    pub hb_state: crate::simulation::dialog::hb::HbDialogState,
    // --- S-Parameter ---
    pub sp_state: crate::simulation::dialog::sp::SpDialogState,
    // --- PAC (Periodic AC) ---
    pub pac_state: crate::simulation::dialog::pac::PacDialogState,
    // --- PNoise (Periodic Noise) ---
    pub pnoise_state: crate::simulation::dialog::pnoise::PnoiseDialogState,
    // --- PXF (Periodic Transfer) ---
    pub pxf_state: crate::simulation::dialog::pxf::PxfDialogState,
    // --- PSTB (Periodic Stability) ---
    pub pstb_state: crate::simulation::dialog::pstb::PstbDialogState,
    // --- XF (Transfer Function) ---
    pub xf_state: crate::simulation::dialog::xf::XfDialogState,
    // --- Corner Analysis ---
    pub corner_state: crate::simulation::dialog::corner::CornerDialogState,
    // --- Envelope Transient ---
    pub envelope_state: crate::simulation::dialog::envelope::EnvelopeDialogState,
    // --- Fourier ---
    pub fourier_state: crate::simulation::dialog::fourier::FourierDialogState,
    // --- Reliability ---
    pub reliability_state: crate::simulation::dialog::reliability::ReliabilityDialogState,
    // --- Optimization ---
    pub optimization_state: crate::simulation::dialog::optimization::OptimizationDialogState,
    // --- Safety (SOA) ---
    pub soa_state: crate::simulation::dialog::soa::SoaDialogState,
    // --- DC Operating Point ---
    pub op_state: crate::simulation::dialog::op::OpDialogState,
    // --- Pole-Zero ---
    pub pz_state: crate::simulation::dialog::pz::PzDialogState,
    // --- Sensitivity ---
    pub sens_state: crate::simulation::dialog::sens::SensDialogState,
    // --- Monte Carlo ---
    pub mc_state: crate::simulation::dialog::mc::McDialogState,
    // --- PSS ---
    pub pss_state: crate::simulation::dialog::pss::PssDialogState,
    // --- STB ---
    pub stb_state: crate::simulation::dialog::stb::StbDialogState,
    // --- Temperature Sweep ---
    pub temp_state: crate::simulation::dialog::temp::TempDialogState,
    // --- Simulation Options ---
    /// Effective simulation options used for engine configuration.
    pub simulation_options_config: crate::simulation::dialog::SimulationOptions,
    /// Editable UI buffers for the simulation options dialog.
    pub simulation_options_state: crate::simulation::dialog::OptionsDialogState,
    /// Validation/parse errors shown in the simulation options dialog.
    pub simulation_options_errors: Vec<String>,

    /// Starting position of selection drag (grid coords)
    pub drag_start: Option<(i32, i32)>,
    /// Last drag position for computing delta (grid coords)
    pub last_drag_pos: Option<(i32, i32)>,

    // =========================================================================
    // Commercial-Grade Tool Dialogs
    // =========================================================================

    // --- DRC/ERC ---
    /// DRC results (cached from last run; surfaced by the schematic view)
    pub drc_results: Option<crate::services::drc::DrcResult>,

    // --- Waveform Calculator Dialog ---
    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,

    // --- Verilog-A Model Loading ---
    /// Verilog-A model loading dialog state
    pub veriloga_dialog: crate::panels::VerilogALoadDialogState,

    // --- Runtime Interaction State ---
    /// Runtime interaction state for drag, hover, etc.
    pub interaction: super::InteractionState,

    // --- Save Confirmation Dialog ---
    /// State for save confirmation modal (unsaved changes warning)
    pub confirmation_dialog: ConfirmationDialogState,
}
