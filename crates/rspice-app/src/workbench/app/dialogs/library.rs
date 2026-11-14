//! Design library membership dialogs.
//!
//! Every dialog here changes what the project's library catalog contains —
//! a library, a cell, or a view — rather than the contents of one design, so
//! each is bound to the exact catalog revision it opened against and refuses
//! to commit against a catalog that moved underneath it.

pub(crate) use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device;
mod shared;
pub(in crate::workbench) use rspice_app_code::workbench::app::dialogs::library::veriloga_import;
pub(crate) use shared::LIBRARY_CATALOG_STALE_MESSAGE;
pub(in crate::workbench) use shared::validate_lcv_name;
