/// Optional execution overrides for spec-driven analyses.
#[derive(Debug, Clone, Default)]
pub struct SpecExecutionOptions {
    pub study_base: Option<crate::study::StudyRunConfig>,
    pub tran_checkpoint: Option<crate::transient_checkpoint::TransientCheckpointRequest>,
    pub mc_checkpoint: Option<crate::monte_carlo_checkpoint::MonteCarloCheckpointRequest>,
    /// Histogram bins for the default all-node OP study. Configured bases carry their own.
    pub mc_histogram_bins: Option<usize>,
    pub mc_statistics: Option<rspice_simulation_contract::mc_statistics::McStatisticsConfig>,
    pub temp: Option<crate::sweeps::TempRunConfig>,
    /// Base analysis paired with a design-parameter `.STEP`. `None` retains
    /// the classic operating-point behavior for older prepared requests.
    pub parametric_base: Option<crate::sweeps::CornerBaseMode>,
    pub corner: Option<crate::sweeps::CornerRunConfig>,
    pub pac: Option<crate::periodic::PacRunConfig>,
    pub pxf: Option<crate::periodic::PxfRunConfig>,
    pub pnoise: Option<crate::periodic::PnoiseRunConfig>,
    pub pstb: Option<crate::periodic::PstbRunConfig>,
}
