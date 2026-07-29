//! Symbol Library - Commercial-grade SVG symbol management for schematic components
//!
//! This module provides SVG path parsing, symbol lookup, rendering, rotation
//! transforms, and pin position definitions for wire attachment.

#![allow(clippy::too_many_arguments)]

mod error;
mod library;
mod parser;
mod render;
mod types;

pub use self::library::SymbolLibrary;
pub use self::render::{draw_baked, draw_symbol};
pub use self::types::PathCommand;
