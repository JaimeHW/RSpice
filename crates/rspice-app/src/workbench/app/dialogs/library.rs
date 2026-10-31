//! Design library membership dialogs.
//!
//! Every dialog here changes what the project's library catalog contains —
//! a library, a cell, or a view — rather than the contents of one design, so
//! each is bound to the exact catalog revision it opened against and refuses
//! to commit against a catalog that moved underneath it.

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::workbench::app::RSpiceApp;

pub(in crate::workbench) mod new_veriloga_device;
pub(in crate::workbench) mod pending_deletions;
mod shared;
pub(in crate::workbench) use shared::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
mod veriloga_import;
