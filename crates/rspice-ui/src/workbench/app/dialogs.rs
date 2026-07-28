pub(in crate::workbench) mod check_and_save;
pub(in crate::workbench) mod check_and_save_validation;
pub(in crate::workbench) mod configuration_sets;
pub(in crate::workbench) mod confirmation;
pub(in crate::workbench) mod confirmation_state;
pub(in crate::workbench) mod connectivity_manager;
pub(in crate::workbench) mod create_model_bound_symbol;
pub(in crate::workbench) mod design_management;
pub(in crate::workbench) mod design_review_comments;
pub(in crate::workbench) mod engineering_table;
pub(in crate::workbench) mod hardcopy;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod hierarchy;
pub(in crate::workbench) mod library;
pub(in crate::workbench) mod license;
pub(in crate::workbench) mod object_properties;
pub(in crate::workbench) mod operation_primitives;
pub(in crate::workbench) mod pdk_workflow;
pub(in crate::workbench) mod placement;
pub(in crate::workbench) mod preferences;
pub(in crate::workbench) mod project_revision_history;
pub(in crate::workbench) mod replace_instance;
pub(in crate::workbench) mod review_primitives;
pub(in crate::workbench) mod schematic_command;
pub(in crate::workbench) mod selection;
pub(in crate::workbench) mod state;
pub(in crate::workbench) mod symbol_definition;
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
