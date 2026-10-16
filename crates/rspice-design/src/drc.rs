//! Design rule configuration, input mapping, and complete electrical reports.

mod checker;
mod extraction;
mod input;
mod net;
mod rule_deck;
mod types;

pub use checker::{DrcChecker, DrcConfig};
pub use extraction::{extract_checked_design, run_check_with_hierarchy};
pub use input::{ComponentInfo, ParameterRangeIssue, PinInfo};
pub use rule_deck::{DESIGN_RULE_DECK_SCHEMA_VERSION, DesignRuleDeck};
pub use types::{
    DrcLocation, DrcResult, DrcRuleCategory, DrcSeverity, DrcSummary, DrcViolation,
    DrcViolationType,
};
