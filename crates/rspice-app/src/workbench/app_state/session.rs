//! Session lifecycle: autosave, serialization, and device-local persistence.

pub(in crate::workbench) mod preference_runtime;
pub(in crate::workbench) mod serialization;
pub(in crate::workbench) mod session_store;
pub(in crate::workbench) mod state_init;
pub(crate) mod wire;
