//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! Each feature keeps its dialogs' state in its slot
//! ([`crate::workbench::slots`]), and `AppState::dialog_modal_open` reads
//! them all so a single read answers whether any modal owns input this
//! frame; the dialogs themselves never inspect each other.

pub(in crate::workbench) mod check_and_save;
pub(in crate::workbench) mod check_and_save_validation;
pub(in crate::workbench) mod configuration_sets;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::confirmation;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::confirmation_state;
pub(in crate::workbench) mod connectivity_manager;
pub(in crate::workbench) mod create_model_bound_symbol;
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::app::dialogs::delete_sheet;
pub(in crate::workbench) mod design_management;
pub(in crate::workbench) mod design_review_comments;
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
pub(in crate::workbench) mod engineering_table;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::fact_grid;
pub(in crate::workbench) mod find_replace;
pub(in crate::workbench) mod grid_snap_routing;
pub(crate) mod hardcopy;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod hierarchy;
pub(in crate::workbench) mod library;
pub(crate) mod license;
pub(in crate::workbench) mod live_session;
pub(in crate::workbench) mod object_properties;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::operation_primitives;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod pdk_workflow;
pub(in crate::workbench) mod placement;
pub(in crate::workbench) mod plan_removal;
pub(crate) mod preferences;
pub(in crate::workbench) mod project;
pub(in crate::workbench) mod project_revision_history;
pub(in crate::workbench) mod property_dialog;
pub(crate) mod publish_web;
pub(in crate::workbench) mod replace_instance;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::dialogs::review_primitives;
pub(in crate::workbench) mod rule_setup;
pub(in crate::workbench) mod schematic_command;
pub(in crate::workbench) mod selection;
pub(in crate::workbench) mod source_document;
pub(in crate::workbench) mod source_find;
pub(in crate::workbench) mod source_language_tools;
pub(in crate::workbench) mod state;
pub(in crate::workbench) mod stimulus_link;
pub(in crate::workbench) mod symbol_definition;
pub(in crate::workbench) mod unpublish_web;
pub(in crate::workbench) mod veriloga_build_settings;
pub(in crate::workbench) mod visibility_options;
pub(crate) use rspice_app_workbench::workbench::app::dialogs::window_session;
