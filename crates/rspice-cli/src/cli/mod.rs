//! CLI Module - Command-line interface components
//!
//! This module contains all CLI-related types and logic:
//! - `args` - Argument definitions for subcommands
//! - `config` - Configuration file handling
//! - `error` - Error types and exit codes

pub mod args;
pub mod config;
pub mod error;

pub use args::*;
pub use config::Config;
pub use error::{CliError, ExitCode};
