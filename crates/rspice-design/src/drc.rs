//! Design rule configuration, input mapping, and complete electrical reports.

mod checker;
mod extraction;
mod input;
mod net;
mod types;

pub use checker::{DrcChecker, DrcConfig};
pub use extraction::{extract_components, run_check};
pub use input::{ComponentInfo, ParameterRangeIssue, PinInfo};
pub use types::{DrcLocation, DrcResult, DrcSeverity, DrcSummary, DrcViolation, DrcViolationType};
