//! Design rule configuration, input mapping, and complete electrical reports.

mod checker;
mod extraction;
mod input;
mod net;
mod rule_deck;
mod rules;
mod source_loops;
mod types;
mod waivers;

pub use checker::{DrcChecker, DrcConfig};
pub use extraction::{
    CheckReach, CheckReads, append_unused_cellviews, check_reads, extract_checked_design,
    run_check_of_cellview, run_check_with_hierarchy,
};
pub use input::{BrokenReference, ComponentInfo, ParameterRangeIssue, PinInfo};
pub use rule_deck::{DESIGN_RULE_DECK_SCHEMA_VERSION, DeckEditRefusal, DeckIssue, DesignRuleDeck};
pub use rules::{DrcFloor, DrcRule};
pub use types::{
    DrcLocation, DrcResult, DrcRuleCategory, DrcSeverity, DrcSummary, DrcViolation,
    DrcViolationType,
};
pub use waivers::{
    CheckWaivers, DRC_WAIVER_SCHEMA_VERSION, DrcWaiver, DrcWaivers, WaiverEditRefusal,
    location_key, waiver_site,
};
