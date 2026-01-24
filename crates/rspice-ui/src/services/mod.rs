//! Services Module
//!
//! Business logic and service modules for the RSpice UI application.
//! These are separated from UI state to improve testability and organization.
//!
//! ## Modules
//!
//! - `analysis_results` - Unified result types for all analysis types
//! - `cross_probing` - Cross-probe between schematic and waveform viewer
//! - `netlist_generator` - Generate SPICE netlists from schematic
//! - `pdf_export` - Export schematics to PDF format
//! - `simulation_runner` - Run simulations and manage results

pub mod analysis_results;
pub mod cross_probing;
pub mod netlist_generator;
pub mod pdf_export;
pub mod simulation_runner;

// Re-export main types for convenient access
// Types unique to analysis_results
pub use analysis_results::{
    AnalysisResult, CornerData, DcOpData, FourierData, MonteCarloData, PacData, ParametricData,
    PoleZeroData, SensitivityData, TransferData,
};
pub use cross_probing::CrossProbeManager;
pub use netlist_generator::{generate_netlist, NetlistResult};
pub use pdf_export::{PageSize, PdfExportConfig, SvgExporter, TitleBlock};
// Types from simulation_runner (including visualization-ready data structures)
pub use simulation_runner::{
    run_ac_analysis, run_dc_sweep, run_hb_analysis, run_noise_analysis, run_pss_analysis,
    run_simulation, run_stb_analysis, AcData, DcSweepData, HbData, NoiseData, PssData,
    SimulationResult, SimulationStats, StbData, TransientData,
};
