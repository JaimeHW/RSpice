//! Rules the document engines' session shares across its owners.
//!
//! Each feature keeps its own interaction session in its slot
//! ([`crate::workbench::slots`]) and the workbench keeps the shell's; this
//! module holds only what more than one of them reads: the autosave interval
//! Preferences offers, and the symbol editor's tool vocabulary.

pub(crate) use crate::services::session::normalize_autosave_minutes;
pub use rspice_schematic_editor::symbol_editor::session::{SymbolGridSpacing, SymbolTool};
