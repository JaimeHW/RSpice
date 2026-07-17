//! Design Rule Checker (DRC/ERC)
//!
//! Professional-grade design and electrical rule checking for RSpice schematics.
//! Follows patterns from Cadence DRC and Mentor Calibre.

mod checker;
mod extraction;
mod input;
mod net;
mod types;

pub use self::checker::{DrcChecker, DrcConfig};
pub use self::extraction::{
    extract_drc_data, extract_drc_data_with_hierarchy,
    extract_drc_data_with_hierarchy_and_junctions, extract_drc_data_with_junctions, run_drc_check,
    run_drc_check_with_config, run_drc_check_with_hierarchy,
    run_drc_check_with_hierarchy_and_config,
};
pub use self::input::{ComponentInfo, JunctionInfo, NetLabelInfo, PinInfo, WireInfo};
pub use self::types::{
    DrcLocation, DrcResult, DrcSeverity, DrcSummary, DrcViolation, DrcViolationType,
};
