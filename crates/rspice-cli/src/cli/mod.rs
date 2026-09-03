//! CLI Module - Command-line interface components
//!
//! This module contains all CLI-related types and logic:
//! - `args` - Argument definitions for subcommands
//! - `config` - Configuration file handling
//! - `error` - Error types and exit codes

pub mod args;
pub mod config;
pub mod error;
/// Proves the README's published exit-code table is exactly what
/// [`error::exit_code_for`] produces, so automation branching on the
/// documented numbers cannot drift from the process it watches.
#[cfg(test)]
mod readme_exit_codes;

pub use args::*;
pub use config::Config;
pub(crate) use error::map_atomic_output_error;
pub use error::{CliError, ErrorDetails, FailureCategory};
