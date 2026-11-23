//! Commands the code documents own: Find opens the window of the document
//! in front of the user, and compiling Verilog-A requests the compile rather
//! than merely revealing the page that would report it.

use super::*;
use rspice_app_code::workbench::code::commands::SourceCommand;

/// The workbench's Find on the Netlist workspace asks the deck's editor for
/// its find bar.
#[test]
fn find_on_the_netlist_workspace_asks_for_the_deck_editors_find_bar() {
    use rspice_app_workbench::workbench::framework::command_registry::ids;
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;

    assert!(app.command_key_enabled(ids::FIND_IN_DESIGN));
    app.execute_command_key(ids::FIND_IN_DESIGN);
    assert!(
        app.state.code.netlist.find.open,
        "the deck's find bar is asked for"
    );
}

/// Navigating to the Verilog-A editor is not compiling it. The stage's Compile
/// button dispatches this command, so an execute that only switched pages made
/// the button a no-op on the page it lives on.
///
/// A Verilog-A cell view is edited in Design, so Compile — offered from the
/// Models toolbar here — takes the reader to that document and requests the
/// compile of its bundle.
#[test]
fn compiling_veriloga_requests_the_compile_and_not_only_the_page() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        !app.command_key_enabled(SourceCommand::CompileVerilogA.key()),
        "with no Verilog-A view open there is nothing to compile"
    );
    let reference = rspice_app_state::state::CellViewRef::new("behavioral", "gain", "veriloga");
    let mut view =
        rspice_app_state::state::View::new("veriloga", rspice_app_state::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    let mut cell = rspice_app_state::state::Cell::new("gain");
    cell.add_view(view);
    let mut library = rspice_app_state::state::Library::new("behavioral");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(
            rspice_app_state::state::ProjectSourceBundle::try_new(
                rspice_app_state::state::ProjectSourceOwner::cell_view(reference.clone()),
                "gain.va",
                "module gain(p, n); inout p, n; electrical p, n; endmodule\n",
                [],
                [],
            )
            .unwrap(),
        )
        .unwrap();
    app.state.open_workspace_view(reference.clone());
    app.state.workbench.activate(Workspace::Models);

    assert!(
        app.command_key_enabled(SourceCommand::CompileVerilogA.key()),
        "the Models toolbar's Compile reaches the Verilog-A document in front"
    );
    app.execute_command_key(SourceCommand::CompileVerilogA.key());

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::Design
    );
    assert_eq!(app.state.model.workspace.content.active_view, reference);
    let request = app
        .state
        .code
        .source_editing
        .veriloga
        .compile_request
        .as_ref()
        .expect("the click took the view's closure for the compile");
    assert_eq!(
        request.bundle().owner(),
        &rspice_app_state::state::ProjectSourceOwner::cell_view(reference)
    );
}

/// View > Zoom over a source editor sizes the editors' text a step at a
/// time, and Zoom 100% returns it to the size nobody chose.
#[test]
fn view_zoom_sizes_the_text_of_the_source_editor_in_front() {
    use rspice_app_workbench::workbench::framework::command_registry::ids;
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;
    app.state.workbench.active_source_editor = Some(
        rspice_app_code::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: egui::Id::new("zoomed-source"),
            editable: true,
            navigable: true,
            can_undo: false,
            can_redo: false,
            has_selection: false,
        },
    );
    let actual = app.state.model.preferences.editor_font_size();
    assert!(
        !app.command_key_enabled(ids::ZOOM_ONE_TO_ONE),
        "the text is at its actual size"
    );
    assert!(app.command_key_enabled(ids::ZOOM_IN));
    app.execute_command_key(ids::ZOOM_IN);
    assert!(app.state.model.preferences.editor_font_size() > actual);
    assert!(app.command_key_enabled(ids::ZOOM_ONE_TO_ONE));
    app.execute_command_key(ids::ZOOM_ONE_TO_ONE);
    assert_eq!(app.state.model.preferences.editor_font_size(), actual);
    app.execute_command_key(ids::ZOOM_OUT);
    assert!(app.state.model.preferences.editor_font_size() < actual);
    for _ in 0..16 {
        app.execute_command_key(ids::ZOOM_OUT);
    }
    assert!(
        !app.command_key_enabled(ids::ZOOM_OUT),
        "the smallest size goes no smaller"
    );
}

/// View > Word wrap switches every source editor's wrap, and back.
#[test]
fn view_word_wrap_switches_the_source_editors_wrap() {
    use rspice_app_state::workbench::preferences::TogglePreference;
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;
    app.state.workbench.active_source_editor = Some(
        rspice_app_code::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: egui::Id::new("wrapped-source"),
            editable: false,
            navigable: true,
            can_undo: false,
            can_redo: false,
            has_selection: false,
        },
    );
    let key = SourceCommand::ToggleWordWrap.key();
    let wraps = |app: &RSpiceApp| {
        app.state
            .model
            .preferences
            .toggle(TogglePreference::EditorWordWrap)
    };
    assert!(!wraps(&app), "lines run on until asked to wrap");
    assert!(
        app.command_key_enabled(key),
        "a read-only document wraps too"
    );
    app.execute_command_key(key);
    assert!(wraps(&app));
    app.execute_command_key(key);
    assert!(!wraps(&app));
}
