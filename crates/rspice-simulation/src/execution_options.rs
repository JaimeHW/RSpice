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
    /// The one member of the deck's own `.MC` study or `.STEP` sweep this
    /// transient solves. `None` solves the deck as it stands.
    pub member: Option<FamilyMember>,
    /// Resume this transient from the checkpoint the transient it depends on
    /// captured at this scheduled time, rather than solving from the start.
    /// That transient is the same request on the same circuit with this time
    /// in its checkpoint schedule; the image is the one it captured there.
    pub resume_at: Option<f64>,
}

/// One member of a statistical study or a parameter sweep, solved as an
/// ordinary transient with its own waveforms, events, `.MEAS` results and
/// checks.
///
/// The task's deck carries the `.MC` or `.STEP` card the member belongs to,
/// and the member is materialized from it exactly as the engine's own
/// Monte Carlo transient and `.STEP` transient materialize theirs: a trial is
/// drawn from the card's seed at its own index, so trial `k` of a run of one
/// draws what trial `k` of the whole study draws; a point is the sweep's own
/// coordinate. A study is then one task per member, each with its full
/// result, rather than one task reduced to a scalar per trial or per point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "member")]
pub enum FamilyMember {
    /// Trial `index` of the deck's `.MC` study, in the card's own numbering
    /// (its first trial included), varied as `variation_source` states.
    MonteCarloTrial {
        index: usize,
        variation_source: rspice_simulation_contract::mc_draft::McVariationSource,
    },
    /// Point `index` of the deck's `.STEP` sweep, in sweep order.
    StepPoint { index: usize },
}

impl FamilyMember {
    /// What a result of this member is called, `trial 3` or `point 0`.
    pub fn label(self) -> String {
        match self {
            Self::MonteCarloTrial { index, .. } => format!("trial {index}"),
            Self::StepPoint { index } => format!("point {index}"),
        }
    }
}
