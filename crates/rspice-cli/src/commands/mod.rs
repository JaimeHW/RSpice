//! Command Implementations
//!
//! This module contains the implementation logic for each CLI subcommand:
//! - `run` - Execute simulations
//! - `info` - Display netlist information
//! - `compile_va` - Compile Verilog-A models
//! - `check` - Validate netlists
//! - `convert` - Convert output formats
//! - `compare` - Golden file regression testing

pub mod check;
pub mod compare;
pub mod compile_va;
pub mod convert;
pub mod info;
pub mod run;
mod run_signals;

pub use check::execute as check;
pub use compile_va::execute as compile_va;
pub use convert::execute as convert;
pub use info::execute as info;
pub use run::execute as run;
