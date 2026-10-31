//! Session lifecycle: autosave, serialization, and device-local persistence.

pub(in crate::workbench) use rspice_app_models::workbench::app_state::session::pdk_settings;
pub(in crate::workbench) use rspice_app_results::workbench::app_state::session::calculator;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_app_services::workbench::app_state::session::autosave;
pub(in crate::workbench) mod preference_runtime;
pub(crate) use rspice_app_settings::workbench::app_state::session::preferences_file;
pub(in crate::workbench) mod serialization;
pub(in crate::workbench) mod session_store;
pub(crate) use rspice_app_settings::workbench::app_state::session::shortcut_library;
pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts;
pub(in crate::workbench) mod state_init;
pub(in crate::workbench) mod veriloga_library;
pub(in crate::workbench) mod wire;
