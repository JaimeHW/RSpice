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
pub(crate) mod confirmation;
pub(in crate::workbench) mod confirmation_state;
pub(in crate::workbench) mod connectivity_manager;
pub(in crate::workbench) mod create_model_bound_symbol;
pub(in crate::workbench) mod delete_sheet;
pub(in crate::workbench) mod design_management;
pub(in crate::workbench) mod design_review_comments;
pub(in crate::workbench) mod drawing_sheet_defaults;
pub(in crate::workbench) mod drawing_sheet_overflow;
pub(in crate::workbench) mod drawing_sheet_presets;
pub(in crate::workbench) mod drawing_sheet_preview;
pub(in crate::workbench) mod drawing_sheet_support;
pub(in crate::workbench) mod engineering_table;
pub(in crate::workbench) mod find_replace;
pub(in crate::workbench) mod grid_snap_routing;
pub(crate) mod hardcopy;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod hierarchy;
pub(in crate::workbench) mod library;
pub(in crate::workbench) mod license;
pub(in crate::workbench) mod live_session;
pub(in crate::workbench) mod object_properties;
pub(in crate::workbench) mod operation_primitives;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod pdk_workflow;
pub(in crate::workbench) mod placement;
pub(in crate::workbench) mod plan_removal;
pub(crate) mod preferences;
pub(in crate::workbench) mod project;
pub(in crate::workbench) mod project_revision_history;
pub(in crate::workbench) mod property_dialog;
pub(in crate::workbench) mod publish_web;
pub(in crate::workbench) mod replace_instance;
pub(in crate::workbench) mod review_primitives;
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
pub(crate) mod window_session;

use egui::Context;

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    pub(in crate::workbench) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.models.model_browser_state,
            &self.state.model.model_library_manager,
        );
    }
}
