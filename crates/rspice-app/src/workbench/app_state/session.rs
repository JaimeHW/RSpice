//! Session lifecycle: autosave, serialization, and device-local persistence.

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_app_services::workbench::app_state::session::autosave;
pub(in crate::workbench) mod calculator;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod preference_runtime;
pub(crate) mod preferences_file;
pub(in crate::workbench) mod serialization;
pub(in crate::workbench) mod session_store;
pub(in crate::workbench) mod shortcut_library;
pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts;
pub(in crate::workbench) mod state_init;
pub(in crate::workbench) mod veriloga_library;
pub(in crate::workbench) mod wire;
