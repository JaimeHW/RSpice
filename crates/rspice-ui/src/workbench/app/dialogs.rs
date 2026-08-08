//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! `DialogState` in [`state`] aggregates the visibility flags so a single
//! read answers whether any modal owns input this frame; the dialogs
//! themselves never inspect each other.

pub(in crate::workbench) mod check_and_save;
pub(in crate::workbench) mod check_and_save_validation;
pub(in crate::workbench) mod configuration_sets;
pub(in crate::workbench) mod confirmation;
pub(in crate::workbench) mod confirmation_state;
pub(in crate::workbench) mod connectivity_manager;
pub(in crate::workbench) mod create_model_bound_symbol;
pub(in crate::workbench) mod design_management;
pub(in crate::workbench) mod design_review_comments;
pub(in crate::workbench) mod drawing_sheet_defaults;
pub(in crate::workbench) mod drawing_sheet_layers;
pub(in crate::workbench) mod drawing_sheet_overflow;
pub(in crate::workbench) mod drawing_sheet_presets;
pub(in crate::workbench) mod drawing_sheet_preview;
pub(in crate::workbench) mod drawing_sheet_setup;
pub(in crate::workbench) mod drawing_sheet_support;
pub(in crate::workbench) mod engineering_table;
pub(in crate::workbench) mod grid_snap_routing;
pub(in crate::workbench) mod hardcopy;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod hierarchy;
pub(in crate::workbench) mod library;
pub(in crate::workbench) mod license;
pub(in crate::workbench) mod object_properties;
pub(in crate::workbench) mod operation_primitives;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod pdk_workflow;
pub(in crate::workbench) mod placement;
pub(in crate::workbench) mod preferences;
pub(in crate::workbench) mod project_revision_history;
pub(in crate::workbench) mod property_dialog;
pub(in crate::workbench) mod publish_web;
pub(in crate::workbench) mod replace_instance;
pub(in crate::workbench) mod review_primitives;
pub(in crate::workbench) mod schematic_command;
pub(in crate::workbench) mod selection;
pub(in crate::workbench) mod state;
pub(in crate::workbench) mod symbol_definition;
pub(in crate::workbench) mod unpublish_web;
pub(in crate::workbench) mod view_operations;
pub(in crate::workbench) mod visibility_options;
pub(in crate::workbench) mod window_session;

use egui::Context;

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    pub(in crate::workbench) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}
