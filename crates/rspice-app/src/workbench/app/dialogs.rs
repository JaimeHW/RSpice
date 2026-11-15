//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! Each feature keeps its dialogs' state in its slot
//! ([`crate::workbench::slots`]), and `AppState::dialog_modal_open` reads
//! them all so a single read answers whether any modal owns input this
//! frame; the dialogs themselves never inspect each other.

pub(crate) use rspice_app_project::workbench::app::dialogs::check_and_save_validation;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::confirmation;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::confirmation_state;
pub(crate) mod connectivity_manager;
pub(crate) mod create_model_bound_symbol;
pub(crate) mod design_request;
pub(crate) mod design_review_comments;
pub(crate) mod engineering_table;
pub(crate) mod find_replace;
pub(crate) mod grid_snap_routing;
pub(crate) use rspice_app_hardcopy::workbench::app::dialogs::hardcopy;
pub(in crate::workbench) mod help;
pub(crate) mod hierarchy;
pub(crate) use rspice_app_code::workbench::app::dialogs::library;
pub(crate) mod object_properties;
pub(crate) mod placement;
pub(crate) use rspice_app_project::workbench::app::dialogs::project;
pub(crate) mod property_dialog;
pub(crate) mod replace_instance;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::review_primitives;
pub(crate) mod rule_setup;
pub(crate) mod selection;
#[cfg(test)]
pub(crate) use rspice_app_code::workbench::app::dialogs::source_find;
pub(in crate::workbench) mod state;
pub(crate) mod stimulus_link;
pub(crate) mod symbol_definition;
#[cfg(test)]
pub(in crate::workbench) use rspice_app_code::workbench::app::dialogs::veriloga_build_settings;
pub(crate) mod visibility_options;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::window_session;
