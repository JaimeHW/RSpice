//! The canvas context menu driven through a whole application.

use crate::schematic::view::context_menu::tests::select_a_veriloga_instance;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::state::Workspace;

/// The row names its shortcut, the Descend command, so the command must do
/// what the row does on a Verilog-A instance: it was disabled there, because
/// a module is not a schematic master.
#[test]
fn the_descend_shortcut_opens_a_veriloga_instances_view_as_its_row_does() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let (_, reference) = select_a_veriloga_instance(&mut app.state);
    assert!(app.command_key_enabled(DesignCommand::DescendHierarchyDirect.key()));
    assert!(
        !app.command_key_enabled(DesignCommand::DescendHierarchy.key()),
        "the descent dialog lists schematic masters only"
    );

    app.execute_command_key(DesignCommand::DescendHierarchyDirect.key());

    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
}
