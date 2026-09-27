//! Simulation execution services shared by application and worker hosts.
//!
//! PDK runtime validation compiles exact signed source closures and executes
//! bounded callbacks before its caller may publish an installation.

pub mod netlist_sources;
pub mod compilation;
pub mod model_import;
pub mod model_sources;
pub mod pdk;
pub mod project_veriloga;
pub mod veriloga;
