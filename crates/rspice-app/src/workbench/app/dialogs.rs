//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! Each feature keeps its dialogs' state in its slot
//! ([`crate::workbench::slots`]), and `AppState::dialog_modal_open` reads
//! them all so a single read answers whether any modal owns input this
//! frame; the dialogs themselves never inspect each other.

pub(crate) use rspice_app_project::workbench::app::dialogs::check_and_save;
pub(crate) use rspice_app_project::workbench::app::dialogs::check_and_save_validation;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::confirmation;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::confirmation_state;
pub(crate) mod connectivity_manager;
pub(crate) mod create_model_bound_symbol;
#[cfg(test)]
pub(crate) use rspice_app_project::workbench::app::dialogs::design_management;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::delete_sheet;
pub(in crate::workbench) mod design_request;
pub(crate) mod design_review_comments;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_defaults;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_overflow;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_presets;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_preview;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_support;
pub(crate) mod engineering_table;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::fact_grid;
pub(crate) mod find_replace;
pub(crate) mod grid_snap_routing;
pub(crate) use rspice_app_hardcopy::workbench::app::dialogs::hardcopy;
pub(in crate::workbench) mod help;
pub(crate) mod hierarchy;
#[cfg(test)]
pub(crate) use rspice_app_cloud::workbench::app::dialogs::license;
pub(crate) use rspice_app_code::workbench::app::dialogs::library;
pub(crate) mod object_properties;
pub(crate) mod placement;
pub(crate) use rspice_app_project::workbench::app::dialogs::project;
#[cfg(test)]
pub(crate) use rspice_app_settings::workbench::app::dialogs::preferences;
#[cfg(test)]
pub(in crate::workbench) use rspice_app_simulate::workbench::app::dialogs::plan_removal;
pub(in crate::workbench) mod property_dialog;
#[cfg(test)]
pub(crate) use rspice_app_cloud::workbench::app::dialogs::publish_web;
pub(crate) mod replace_instance;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::review_primitives;
pub(crate) mod rule_setup;
pub(crate) mod selection;
pub(crate) use rspice_app_code::workbench::app::dialogs::source_document;
#[cfg(test)]
pub(crate) use rspice_app_code::workbench::app::dialogs::source_find;
pub(crate) use rspice_app_code::workbench::app::dialogs::source_language_tools;
pub(in crate::workbench) mod state;
pub(crate) mod stimulus_link;
pub(crate) mod symbol_definition;
pub(in crate::workbench) use rspice_app_code::workbench::app::dialogs::veriloga_build_settings;
pub(crate) mod visibility_options;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::window_session;
