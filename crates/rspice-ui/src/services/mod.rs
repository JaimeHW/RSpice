//! Services Module
//!
//! Backend services for the RSpice UI application.
//! Contains specialized processing and analysis utilities.

pub mod analysis_results;
pub mod corner_comparison;
pub mod drc;
pub mod pdf_export;
pub mod performance_summary;
pub mod safety;
pub mod simulation_runner;
pub mod yield_manager;

// Re-export main types for convenient access
pub use analysis_results::{
    AnalysisResult, CornerData, DcOpData, FourierData, MonteCarloData, PacData, ParametricData,
    PoleZeroData, SensitivityData, TransferData,
};
pub use corner_comparison::{ComparisonRow, Corner, CornerComparisonView, CornerResult};
pub use pdf_export::{PageSize, PdfExportConfig, SvgExporter, TitleBlock};
pub use performance_summary::{MetricStatus, MetricType, PerformanceMetric, PerformanceSummary};
pub use safety::{SoAManager, SoAViolation, ViolationSeverity};
pub use simulation_runner::{
    AcData, DcSweepData, HbData, NoiseData, PssData, PstbData, SimulationResult, SimulationStats,
    StbData, TransientData, run_ac_analysis, run_dc_sweep, run_hb_analysis, run_noise_analysis,
    run_pole_zero_analysis, run_pss_analysis, run_pstb_analysis, run_sensitivity_analysis,
    run_simulation, run_stb_analysis,
};
pub use yield_manager::{
    DistributionStats, SpecLimitType, YieldAnalysisManager, YieldResult, YieldSpec,
};
