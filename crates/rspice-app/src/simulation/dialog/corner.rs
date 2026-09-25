//! App adapter for the portable Corner draft and instrumented run-set projection.

pub use rspice_simulation_contract::corner_config::{
    CornerBaseAnalysis, CornerConfig, CornerPointSpec,
};
pub use rspice_simulation_contract::corner_draft::CornerDialogState;

use crate::simulation::run_set::{ReferencePoint, RunSetCornerProjection, RunSetState};

/// Project the authored Corner draft through the app's measured run-set path.
pub fn to_config(
    draft: &CornerDialogState,
    run_set: &RunSetState,
    reference: ReferencePoint,
) -> Result<CornerConfig, String> {
    run_set.to_corner_config(draft.base_analysis(), reference)
}
