//! The run set: the space a simulation plan executes.
//!
//! A plan says *what* is analysed; the run set says *where* — which process
//! section, which supply, which temperature, and how those axes compose into
//! points. It is a transactional working state: edits move a revision and
//! leave receipts, a preview freezes a forecast that any later edit clears, and
//! nothing is dispatched against a space that has not validated.
//!
//! The model deliberately admits only dimension kinds the executor binds. A
//! dimension that rendered, validated and persisted without reaching the solver
//! would be indistinguishable from one that worked, which is the failure this
//! module is shaped to prevent.

mod corner_projection;
#[cfg(test)]
mod tests;

pub use corner_projection::RunSetCornerProjection;
#[cfg(test)]
pub use corner_projection::from_corner_config;

pub use rspice_simulation_contract::analysis_run_at::AnalysisRunAt;
pub use rspice_simulation_contract::run_set::{
    InvalidValuePolicy, ReferencePoint, RunSetAction, RunSetAdaptivePolicy, RunSetBudgets,
    RunSetCompositionMode, RunSetDimension, RunSetDimensionKind, RunSetForecast, RunSetPoint,
    RunSetReceiptStatus, RunSetState, RunSetStatus, RunSetTransaction, RunSetValidation,
    forecast_point_count, format_bytes, format_duration_ms, modelled_cost_ms, nominal_point_key,
    parse_bytes, parse_parameter_source_authority, parse_source_value_authority,
    parse_supply_source_authority, participating_point_keys, point_key_label, retained,
};
#[cfg(test)]
pub use rspice_simulation_contract::run_set::{NETLIST_SUPPLY_SOURCE_PREFIX, RunSetComposition};

/// Expand a run set while recording app test-only frame cost.
#[must_use]
pub fn compose(state: &RunSetState) -> Option<Vec<RunSetPoint<'_>>> {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::SpaceExpansion,
    );
    rspice_simulation_contract::run_set::compose(state)
}

/// Resolve executable points while recording app test-only frame cost.
#[must_use]
pub fn resolve(state: &RunSetState) -> Option<Vec<RunSetPoint<'_>>> {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::SpaceExpansion,
    );
    rspice_simulation_contract::run_set::resolve(state)
}

/// Validate a run set while recording app test-only frame cost.
#[must_use]
pub fn validate(state: &RunSetState, enabled_analysis_count: usize) -> RunSetValidation {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::RunSetValidation,
    );
    rspice_simulation_contract::run_set::validate(state, enabled_analysis_count)
}

/// Validate against a plan's exact analysis kinds and queue size.
#[must_use]
pub fn validate_for_plan(
    state: &RunSetState,
    enabled_analysis_kinds: &[crate::simulation::plan::AnalysisKind],
    exact_task_count: Option<usize>,
) -> RunSetValidation {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::RunSetValidation,
    );
    rspice_simulation_contract::run_set::validate_for_plan(
        state,
        enabled_analysis_kinds,
        exact_task_count,
    )
}

/// Validate with a supplied queue count.
#[must_use]
pub fn validate_with_task_count(
    state: &RunSetState,
    enabled_analysis_count: usize,
    exact_task_count: Option<usize>,
) -> RunSetValidation {
    #[cfg(test)]
    crate::simulation::cost_probe::record(
        crate::simulation::cost_probe::Derivation::RunSetValidation,
    );
    rspice_simulation_contract::run_set::validate_with_task_count(
        state,
        enabled_analysis_count,
        exact_task_count,
    )
}

/// Apply a plan-aware run-set transaction.
pub fn dispatch_for_plan(
    state: &mut RunSetState,
    action: RunSetAction,
    enabled_analysis_kinds: &[crate::simulation::plan::AnalysisKind],
    exact_task_count: Option<usize>,
    workload_error: Option<String>,
) -> RunSetTransaction {
    #[cfg(test)]
    if matches!(&action, RunSetAction::Preview) {
        crate::simulation::cost_probe::record(
            crate::simulation::cost_probe::Derivation::RunSetValidation,
        );
    }
    rspice_simulation_contract::run_set::dispatch_for_plan(
        state,
        action,
        enabled_analysis_kinds,
        exact_task_count,
        workload_error,
    )
}

#[cfg(test)]
pub fn dispatch(
    state: &mut RunSetState,
    action: RunSetAction,
    enabled_analysis_count: usize,
) -> RunSetTransaction {
    if matches!(&action, RunSetAction::Preview) {
        crate::simulation::cost_probe::record(
            crate::simulation::cost_probe::Derivation::RunSetValidation,
        );
    }
    rspice_simulation_contract::run_set::dispatch(state, action, enabled_analysis_count)
}
