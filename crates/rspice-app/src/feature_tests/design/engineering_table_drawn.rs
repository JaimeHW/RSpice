//! The engineering table drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use rspice_app_design::workbench::app::dialogs::engineering_table::tests as dialog;
use rspice_app_design::workbench::app::dialogs::engineering_table::*;
use rspice_app_state::state::EngineeringTableView;
use rspice_app_state::state::EngineeringViewScope;
use rspice_app_state::state::engineering_table::active_schematic_dataset;

/// The engineering table over a whole application's parts, as its dialog
/// step lends them.
fn table(app: &mut RSpiceApp) -> EngineeringTable<'_> {
    let state = &mut app.state;
    dialog::table(
        &mut state.design.engineering_table,
        &mut state.design.engineering_table_views,
        &mut state.model,
        &mut state.design.symbol,
        &mut state.platform,
        &mut state.workbench,
        &state.design.property_registry,
    )
}

#[test]
fn project_scope_is_versioned_while_personal_scope_is_not() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let dataset = active_schematic_dataset(&app.state.model.schematic);
    app.state.design.engineering_table.open(
        EngineeringTableView::for_dataset(&dataset),
        dataset.source_revision,
    );
    app.state.design.engineering_table.save_name = "Personal review".to_owned();
    app.state.design.engineering_table.save_scope = EngineeringViewScope::Personal;
    table(&mut app).save_engineering_table_view(&dataset);
    assert!(!app.state.model.workspace.content.project_metadata_dirty);

    app.state.design.engineering_table.save_name = "Project review".to_owned();
    app.state.design.engineering_table.save_scope = EngineeringViewScope::Project;
    table(&mut app).save_engineering_table_view(&dataset);
    assert!(app.state.model.workspace.content.project_metadata_dirty);
}
