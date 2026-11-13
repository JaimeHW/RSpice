//! The Netlist page's toolbar drawn through the application's context.

use crate::workbench::RSpiceApp;
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::surfaces::netlist::fixtures::retain_generated;
use crate::workbench::surfaces::netlist::toolbar::{CodeToolbarLayout, code_toolbar};
use egui::vec2;

/// A netlist-first project: an owned deck and no generated primary. This is
/// the shape whose action set is widest, because the third full-set button
/// is "Manage source document" rather than "Return to primary".
fn netlist_first_app() -> RSpiceApp {
    const DECK: &str = "toolbar fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    app.state.model.workspace.content.netlist_source = Some(DECK.to_owned());
    app.state.model.simulation.source.netlist_content = DECK.to_owned();
    app.state.code.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    app
}

/// A schematic-first project: a generated primary is retained and active.
fn schematic_first_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    retain_generated(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "generated\nR1 out 0 1k\n.op\n.end\n",
    );
    app.state.model.simulation.source.netlist_content =
        app.state.model.netlist_artifacts.generated_source.clone();
    app.state.code.netlist.active_document = ActiveNetlistDocument::Generated;
    app.state.code.netlist.active_document_initialized = true;
    app
}

/// Build one application, paint the toolbar over it, and read the layout
/// back. The application is built and dropped here so a caller never holds
/// two of them at once.
fn toolbar_layout(build: fn() -> RSpiceApp, width: f32) -> CodeToolbarLayout {
    let mut app = build();
    let mut captured = None;
    crate::ui::raster::render(vec2(width, 120.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                captured = Some(code_toolbar(
                    ui,
                    &mut crate::workbench::features::cx_with_foreign_answers(&mut app, |_| true),
                ));
            });
    });
    captured.expect("the toolbar paints on every pass")
}

/// The action group must never reach the status chips.
///
/// This replaces a set of frozen width tallies. They were hand-tuned, drifted
/// as controls were added, and by the time the run control arrived the group
/// over-ran its reservation by more than a hundred points and printed on top
/// of the status — which no assertion about a constant could catch. The
/// reservation is now measured from the labels, and this reads back where
/// the two groups actually landed.
#[test]
fn the_action_group_never_reaches_the_status_chips() {
    // One application at a time: two live `AppState`s on a test thread's
    // stack is an overflow, not a fixture.
    for width in [1000.0, 1600.0, 2560.0] {
        for (shape, build) in [
            ("netlist-first", netlist_first_app as fn() -> RSpiceApp),
            ("schematic-first", schematic_first_app as fn() -> RSpiceApp),
        ] {
            let layout = toolbar_layout(build, width);
            assert!(
                layout.actions.left() >= layout.status.right(),
                "{shape} at {width}: actions start at {} but the status chips end at {}",
                layout.actions.left(),
                layout.status.right()
            );
            assert!(
                layout.actions.right() <= layout.content.right() + 0.5,
                "{shape} at {width}: actions overflow the toolbar content rect"
            );
        }
    }
}
