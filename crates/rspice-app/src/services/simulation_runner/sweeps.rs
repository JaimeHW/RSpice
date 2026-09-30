//! Parametric and corner sweep runners.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

mod corner;
mod mapping;
mod netlist_mutation;
mod parametric;
mod types;

pub(crate) use corner::materialize_corner_process_source;
pub(crate) use mapping::{map_corner_results, map_temperature_results};
pub(crate) use netlist_mutation::{apply_voltage_corner, infer_nominal_supply_voltage};
pub(crate) use types::{
    REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END, SweepPointResult,
};

pub use parametric::{
    run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
