//! Portable Corner draft.

use crate::corner_config::{CornerBaseAnalysis, CornerConfig};
use crate::run_set::RunSetState;

// =============================================================================
// Dialog State
// =============================================================================

/// The corner analysis's authored state.
///
/// The run space is *not* here. It is declared once, by the plan — PVT, sweeps
/// & variation owns it — and this analysis reads that declaration. A corner
/// form that also stored a space would be a second owner of one fact, and two
/// owners of a run space eventually disagree about how many points run; the
/// form used to render the embedded copy while claiming to state the plan's,
/// which is that disagreement already happening silently.
///
/// What remains here is the only thing the corner analysis actually owns: which
/// base analysis is repeated at every declared point.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(from = "CornerDialogStateRepr")]
pub struct CornerDialogState {
    /// Base analysis type index.
    pub base_analysis_idx: usize,
    /// Exact saved analysis used at every PVT point.
    #[serde(default)]
    pub base_analysis: Option<rspice_vocabulary::product::AnalysisInstanceId>,
    /// Initialized flag.
    #[serde(skip)]
    pub initialized: bool,
}

/// Wire form of [`CornerDialogState`]: a decoded draft is initialized.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CornerDialogStateRepr {
    #[serde(default)]
    base_analysis_idx: usize,
    #[serde(default)]
    base_analysis: Option<rspice_vocabulary::product::AnalysisInstanceId>,
}

impl From<CornerDialogStateRepr> for CornerDialogState {
    fn from(repr: CornerDialogStateRepr) -> Self {
        Self {
            base_analysis_idx: repr.base_analysis_idx,
            base_analysis: repr.base_analysis,
            initialized: true,
        }
    }
}

impl CornerDialogState {
    /// Project this draft through the plan's declared run space.
    pub fn to_config(
        &self,
        run_set: &RunSetState,
        reference: crate::run_set::ReferencePoint,
    ) -> Result<CornerConfig, String> {
        run_set.to_corner_config(self.base_analysis(), reference)
    }

    /// Initialize from config.
    ///
    /// Only the base analysis is read: the space the configuration carries
    /// belongs to the plan, and adopting it here would re-create the second
    /// owner this type just stopped being.
    pub fn from_config(config: &CornerConfig) -> Self {
        Self {
            base_analysis_idx: match config.base_analysis {
                CornerBaseAnalysis::Transient => 0,
                CornerBaseAnalysis::Ac => 1,
                CornerBaseAnalysis::Dc => 2,
                CornerBaseAnalysis::Op => 3,
            },
            base_analysis: None,
            initialized: true,
        }
    }

    /// The base analysis executed at every point.
    pub fn base_analysis(&self) -> CornerBaseAnalysis {
        match self.base_analysis_idx {
            0 => CornerBaseAnalysis::Transient,
            1 => CornerBaseAnalysis::Ac,
            2 => CornerBaseAnalysis::Dc,
            _ => CornerBaseAnalysis::Op,
        }
    }

    /// Ensure initialized.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            let base = Self::from_config(&CornerConfig::commercial_pvt());
            self.base_analysis_idx = base.base_analysis_idx;
            self.initialized = true;
        }
    }
}
