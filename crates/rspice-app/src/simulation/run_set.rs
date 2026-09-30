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
    RunSetReceiptStatus, RunSetState, RunSetStatus, RunSetValidation, forecast_point_count,
    format_bytes, format_duration_ms, modelled_cost_ms, nominal_point_key, parse_bytes,
    parse_parameter_source_authority, parse_source_value_authority, parse_supply_source_authority,
    participating_point_keys, point_key_label, retained,
};
#[cfg(test)]
pub use rspice_simulation_contract::run_set::{NETLIST_SUPPLY_SOURCE_PREFIX, RunSetComposition};

#[cfg(test)]
pub use rspice_simulation_contract::run_set::dispatch;
pub use rspice_simulation_contract::run_set::{
    compose, dispatch_for_plan, resolve, validate, validate_for_plan, validate_with_task_count,
};
