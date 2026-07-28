//! SPICE Library Parser
//!
//! Commercial-grade parser for SPICE model library files (.lib, .scs).
//! Supports Cadence Spectre syntax with sections, corners, and includes.
//!
//! # Features
//!
//! - Parse .lib and .scs model files
//! - Section/corner extraction (tt, ff, ss, sf, fs)
//! - Nested .include/.lib directive resolution
//! - Model parameter extraction
//! - Comment and continuation line handling
//! - Error reporting with line numbers

mod lexer;
mod parsed;
mod parser;
mod types;

pub use parsed::ParsedLibrary;
