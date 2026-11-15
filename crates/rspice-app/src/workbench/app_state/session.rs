//! Session lifecycle: autosave, serialization, and device-local persistence.

pub(in crate::workbench) mod preference_runtime;
pub(in crate::workbench) mod serialization;
pub(in crate::workbench) mod session_store;
pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts;
pub(in crate::workbench) mod state_init;
pub(crate) mod wire;
