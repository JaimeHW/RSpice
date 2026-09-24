//! Design Rule Checker (DRC/ERC)
//!
//! Professional-grade design and electrical rule checking for RSpice schematics.
//! Follows patterns from Cadence DRC and Mentor Calibre.
//!
//! The checks read the design's one connectivity extraction — the same pass the
//! netlister emits from — so a finding names the node the engine will see. That
//! pass is named once, here, and the rest of the module reaches it through this
//! import rather than deriving connectivity of its own.

use crate::simulation::netlist_gen;

mod checker;
mod extraction;
mod input;
mod net;
mod types;

pub use self::checker::DrcConfig;
pub use self::extraction::run_drc_check_with_hierarchy_and_config;
pub use self::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
