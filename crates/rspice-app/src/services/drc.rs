//! Application hierarchy adapter for design and electrical rule checks.
//!
//! Active symbol and binding lookups feed the shared connectivity pass;
//! rspice-design owns input mapping, configuration, rules, and reports.

mod extraction;

pub use self::extraction::run_drc_check_with_hierarchy_and_config;
pub use rspice_design::drc::DrcConfig;
pub use rspice_design::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
