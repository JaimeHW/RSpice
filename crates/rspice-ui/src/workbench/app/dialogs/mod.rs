pub(in crate::workbench::app) mod check_and_save;
pub(in crate::workbench::app) mod check_and_save_validation;
pub(in crate::workbench::app) mod configuration_sets;
pub(in crate::workbench::app) mod confirmation;
pub(in crate::workbench::app) mod confirmation_state;
pub(in crate::workbench::app) mod connectivity_manager;
pub(in crate::workbench::app) mod create_model_bound_symbol;
pub(in crate::workbench::app) mod design_management;
pub(in crate::workbench::app) mod design_review_comments;
pub(in crate::workbench::app) mod engineering_table;
pub(in crate::workbench::app) mod hardcopy;
pub(in crate::workbench::app) mod help;
pub(in crate::workbench::app) mod hierarchy;
pub(in crate::workbench::app) mod library;
pub(in crate::workbench::app) mod license;
pub(in crate::workbench::app) mod object_properties;
pub(in crate::workbench::app) mod operation_primitives;
pub(in crate::workbench::app) mod pdk_workflow;
pub(in crate::workbench::app) mod placement;
pub(in crate::workbench::app) mod preferences;
pub(in crate::workbench::app) mod project_revision_history;
pub(in crate::workbench::app) mod replace_instance;
pub(in crate::workbench::app) mod review_primitives;
pub(in crate::workbench::app) mod schematic_command;
pub(in crate::workbench::app) mod selection;
pub(in crate::workbench::app) mod state;
pub(in crate::workbench::app) mod symbol_definition;
pub(in crate::workbench::app) mod view_operations;
pub(in crate::workbench::app) mod visibility_options;
pub(in crate::workbench::app) mod window_session;

use egui::Context;

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    pub(in crate::workbench::app) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}
