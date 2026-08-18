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
    app.state.simulation.netlist_content = "deck\n.end\n".to_owned();

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

/// Navigating to the Verilog-A page is not compiling it. The stage's Compile
/// button dispatches this command, so an execute that only switched pages made
/// the button a no-op on the page it lives on.
#[test]
fn compiling_veriloga_requests_the_compile_and_not_only_the_page() {
    let mut app = RSpiceApp::test_instance();
    Command::CompileVerilogA.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        app.state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA
    );
    assert!(app.state.ui.code_workspace.veriloga.compile_requested);
}
