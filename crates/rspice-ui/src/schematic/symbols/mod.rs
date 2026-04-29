//! Symbol Library - Commercial-grade SVG symbol management for schematic components
//!
//! This module provides SVG path parsing, symbol lookup, rendering, rotation
//! transforms, and pin position definitions for wire attachment.

#![allow(clippy::too_many_arguments)]

mod error;
mod library;
mod parser;
mod pins;
mod render;
mod types;

pub use self::error::SymbolError;
pub use self::library::SymbolLibrary;
pub use self::parser::parse_svg;
pub use self::render::draw_symbol;
pub use self::types::{PathCommand, PinDirection, Symbol, SymbolPath, SymbolPin};
