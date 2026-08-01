//! The netlist output format a user picked.
//!
//! Only the vocabulary lives here. Netlists are emitted by
//! `simulation::netlist_gen` for execution and by
//! `workbench::menu_bar::export_actions` for `Command::ExportNetlist`, and
//! each owns its own writing.

use serde::{Deserialize, Serialize};

/// Output netlist format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NetlistFormat {
    /// Cadence Spectre format
    #[default]
    Spectre,
    /// Standard SPICE format
    Spice,
    /// Synopsys HSPICE format
    Hspice,
    /// Xyce format
    Xyce,
}

impl NetlistFormat {
    /// File extension
    pub fn extension(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "scs",
            NetlistFormat::Spice => "spice",
            NetlistFormat::Hspice => "sp",
            NetlistFormat::Xyce => "cir",
        }
    }
}
