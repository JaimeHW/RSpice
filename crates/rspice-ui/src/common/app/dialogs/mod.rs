pub(in crate::common::app) mod check_and_save;
pub(in crate::common::app) mod check_and_save_validation;
pub(in crate::common::app) mod configuration_sets;
pub(in crate::common::app) mod confirmation;
pub(in crate::common::app) mod confirmation_state;
pub(in crate::common::app) mod connectivity_manager;
pub(in crate::common::app) mod create_model_bound_symbol;
pub(in crate::common::app) mod design_management;
pub(in crate::common::app) mod design_review_comments;
pub(in crate::common::app) mod engineering_table;
pub(in crate::common::app) mod hardcopy;
pub(in crate::common::app) mod help;
pub(in crate::common::app) mod hierarchy;
pub(in crate::common::app) mod library;
pub(in crate::common::app) mod license;
pub(in crate::common::app) mod object_properties;
pub(in crate::common::app) mod operation_primitives;
pub(in crate::common::app) mod pdk_workflow;
pub(in crate::common::app) mod placement;
pub(in crate::common::app) mod preferences;
pub(in crate::common::app) mod project_revision_history;
pub(in crate::common::app) mod replace_instance;
pub(in crate::common::app) mod review_primitives;
pub(in crate::common::app) mod schematic_command;
pub(in crate::common::app) mod selection;
pub(in crate::common::app) mod state;
pub(in crate::common::app) mod symbol_definition;
pub(in crate::common::app) mod view_operations;
pub(in crate::common::app) mod visibility_options;
pub(in crate::common::app) mod window_session;

use egui::Context;

use crate::common::app::RSpiceApp;

impl RSpiceApp {
    pub(in crate::common::app) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}
