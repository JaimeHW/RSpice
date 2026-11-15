//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! Each feature keeps its dialogs' state in its slot
//! ([`crate::workbench::slots`]), and `AppState::dialog_modal_open` reads
//! them all so a single read answers whether any modal owns input this
//! frame; the dialogs themselves never inspect each other.

pub(crate) use rspice_app_design::workbench::app::dialogs::connectivity_manager;
pub(crate) use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol;
pub(crate) use rspice_app_design::workbench::app::dialogs::design_request;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::design_review_comments;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::engineering_table;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::find_replace;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::grid_snap_routing;
pub(crate) use rspice_app_project::workbench::app::dialogs::check_and_save_validation;
pub(in crate::workbench) mod help;
pub(crate) use rspice_app_design::workbench::app::dialogs::hierarchy;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::object_properties;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::placement;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::property_dialog;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::replace_instance;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::rule_setup;
pub(crate) use rspice_app_design::workbench::app::dialogs::selection;
pub(in crate::workbench) mod state;
pub(crate) use rspice_app_design::workbench::app::dialogs::stimulus_link;
pub(crate) use rspice_app_design::workbench::app::dialogs::symbol_definition;
#[cfg(test)]
pub(crate) use rspice_app_design::workbench::app::dialogs::visibility_options;
