//! Commands the code workspace's pages own, one visible page at a time.
//!
//! These pages share a workspace but not a document, so a command belongs to
//! the page in front of the user: a netlist route is not offered while a
//! sibling page is showing, Find opens the window the visible page owns, and
//! compiling Verilog-A requests the compile rather than merely revealing the
//! page that would report it.

use super::*;

/// Every one of these opens a window drawn only by the netlist page. Offered
/// from a sibling page they set a dialog open with nothing on screen, and it
/// then appeared unprompted when the user navigated back.
#[test]
fn netlist_document_commands_are_not_offered_from_the_sibling_code_pages() {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;
    app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    app.state.simulation.source.netlist_content = "deck\n.end\n".to_owned();

    assert!(Command::ValidateCodeDocument.is_enabled(&app));

    for page in [CodeWorkspacePage::VerilogA, CodeWorkspacePage::Automation] {
        app.state.ui.code_workspace.page = page;
        assert!(
            !Command::ValidateCodeDocument.is_enabled(&app),
            "{page:?} does not own the netlist deck"
        );
        assert!(
            !Command::CompareGeneratedRevisions.is_enabled(&app),
            "{page:?} does not draw the revision comparison"
        );
    }
}

/// Find is the exception to the rule above: it is page-scoped rather than
/// netlist-only. Each page has a find window of its own -- the deck's on the
/// netlist page, the bundle search on the two language pages -- so the command
/// dispatches per page instead of being withheld from two of the three. It was
/// netlist-only for exactly as long as the bundle search had no surface.
#[test]
fn find_opens_the_window_the_visible_code_page_owns() {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;

    app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    assert!(Command::FindCodeDocument.is_enabled(&app));
    Command::FindCodeDocument.execute(&mut app);
    assert!(app.state.ui.netlist.find.open, "the deck find opened");
    assert!(
        app.state.ui.code_workspace.source_search.is_none(),
        "the deck is not a project source bundle"
    );

    app.state.ui.netlist.find.open = false;
    app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
    assert!(Command::FindCodeDocument.is_enabled(&app));
    Command::FindCodeDocument.execute(&mut app);
    assert!(
        app.state.ui.code_workspace.source_search.is_some(),
        "the bundle search opened"
    );
    assert!(
        !app.state.ui.netlist.find.open,
        "and it did not also open the deck's find window"
    );
}

/// Navigating to the Verilog-A editor is not compiling it. The stage's Compile
/// button dispatches this command, so an execute that only switched pages made
/// the button a no-op on the page it lives on.
///
/// A Verilog-A cell view is edited in Design, so Compile — offered from the
/// Models toolbar here — takes the reader to that document and requests the
/// compile of its bundle. The Netlist page the command used to open is left
/// as it was.
#[test]
fn compiling_veriloga_requests_the_compile_and_not_only_the_page() {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let mut app = RSpiceApp::test_instance();
    let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
    let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    let mut cell = crate::state::Cell::new("gain");
    cell.add_view(view);
    let mut library = crate::state::Library::new("behavioral");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);
    app.state
        .workspace
        .content
        .project_sources
        .insert_bundle(
            crate::state::ProjectSourceBundle::try_new(
                crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                crate::state::ProjectSourceLanguage::VerilogA,
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
    let page_before = app.state.ui.code_workspace.page;

    assert!(
        Command::CompileVerilogA.is_enabled(&app),
        "the Models toolbar's Compile reaches the Verilog-A document in front"
    );
    Command::CompileVerilogA.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Design
    );
    assert_eq!(app.state.workspace.content.active_view, reference);
    assert_eq!(app.state.ui.code_workspace.page, page_before);
    assert_ne!(page_before, CodeWorkspacePage::VerilogA);
    assert!(app.state.ui.code_workspace.veriloga.compile_requested);
}
