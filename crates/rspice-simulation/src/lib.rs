//! Simulation execution services shared by application and worker hosts.
//!
//! PDK runtime validation compiles exact signed source closures and executes
//! bounded callbacks before its caller may publish an installation.

pub mod analysis_preparation;
pub mod compilation;
pub mod error;
pub mod execution_identity;
pub mod execution_options;
pub mod live_transient;
pub mod measurement_references;
pub mod model_import;
pub mod model_sources;
pub mod monte_carlo_checkpoint;
pub mod netlist_gen;
pub mod netlist_preparation;
pub mod netlist_sources;
pub mod optimization;
pub mod pdk;
pub mod periodic;
pub mod prepared_dependency;
pub mod project_veriloga;
pub mod results;
pub mod sealed_source;
pub mod study;
pub mod sweeps;
pub mod veriloga;
