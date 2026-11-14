//! Design library membership dialogs.
//!
//! Every dialog here changes what the project's library catalog contains —
//! a library, a cell, or a view — rather than the contents of one design, so
//! each is bound to the exact catalog revision it opened against and refuses
//! to commit against a catalog that moved underneath it.

pub(crate) use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device;
pub(in crate::workbench) use rspice_app_code::workbench::app::dialogs::library::veriloga_import;
#[cfg(test)]
pub(crate) use rspice_app_project::workbench::app::dialogs::library::LIBRARY_CATALOG_STALE_MESSAGE;
