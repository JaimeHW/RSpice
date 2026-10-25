//! Session lifecycle: autosave, serialization, and device-local persistence.

pub(crate) mod autosave;
pub(in crate::workbench) mod calculator;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod preference_runtime;
pub(in crate::workbench) mod preferences_file;
pub(in crate::workbench) mod serialization;
pub(in crate::workbench) mod session_store;
pub(in crate::workbench) mod shortcut_library;
pub(crate) mod shortcuts;
pub(in crate::workbench) mod state_init;
pub(in crate::workbench) mod veriloga_library;
pub(in crate::workbench) mod wire;
