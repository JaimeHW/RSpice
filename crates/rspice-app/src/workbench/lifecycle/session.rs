//! Rules the document engines' session shares across its owners.
//!
//! Each feature keeps its own interaction session in its slot
//! ([`crate::workbench::slots`]) and the workbench keeps the shell's; this
//! module holds only what more than one of them reads: the autosave interval
//! Preferences offers, and the symbol editor's tool vocabulary.

pub use rspice_schematic_editor::symbol_editor::session::{SymbolGridSpacing, SymbolTool};

/// Map any stored interval onto the 2, 5 and 10 minutes Preferences offers.
pub(crate) fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}
