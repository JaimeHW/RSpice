//! The settings feature observed through a whole application: Preferences
//! and the keyboard shortcut workflows, in the cases that read the product's
//! own commands and bindings or another feature's state.

mod preference_pages;
mod preferences;
mod preferences_file;
mod shortcut_editor;
mod shortcut_library_persistence;
mod shortcut_merge;
#[cfg(not(target_arch = "wasm32"))]
mod shortcut_pdf;
mod shortcut_portability;
mod shortcut_profile;
mod shortcut_profile_workflow;
mod shortcut_projection;
mod shortcut_schema;
mod shortcut_vscode;
