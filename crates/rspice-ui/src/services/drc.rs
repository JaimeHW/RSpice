//! Design Rule Checker (DRC/ERC)
//!
//! Professional-grade design and electrical rule checking for RSpice schematics.
//! Follows patterns from Cadence DRC and Mentor Calibre.

mod checker;
mod extraction;
mod input;
mod net;
mod types;

pub use self::checker::DrcConfig;
pub use self::extraction::{run_drc_check, run_drc_check_with_hierarchy_and_config};
pub use self::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
