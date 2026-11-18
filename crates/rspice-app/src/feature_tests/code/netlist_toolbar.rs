//! The Netlist page's toolbar drawn through the application's context.

use crate::workbench::RSpiceApp;
use egui::vec2;
use rspice_app_code::workbench::documents::netlist_document::NetlistTab;
use rspice_app_code::workbench::surfaces::netlist::fixtures::retain_generated;
use rspice_app_code::workbench::surfaces::netlist::toolbar::{CodeToolbarLayout, code_toolbar};

/// A netlist-first project: a deck and no generated netlist.
fn netlist_first_app() -> RSpiceApp {
    const DECK: &str = "toolbar fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = rspice_app_state::workbench::workspace::Workspace::Netlist;
    app.state.seed_test_deck(DECK.to_owned());
    app
}

/// A schematic-first project: a generated primary is retained and active.
fn schematic_first_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = rspice_app_state::workbench::workspace::Workspace::Netlist;
    retain_generated(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "generated\nR1 out 0 1k\n.op\n.end\n",
    );
    app.state.model.simulation.source.netlist_content =
        app.state.model.netlist_artifacts.generated_source.clone();
    app.state.code.netlist.front = NetlistTab::Generated;
    app.state.code.netlist.active_document_initialized = true;
    app
}

/// Build one application, paint the toolbar over it, and read the layout
/// back. The application is built and dropped here so a caller never holds
/// two of them at once.
fn toolbar_layout(build: fn() -> RSpiceApp, width: f32) -> CodeToolbarLayout {
    let mut app = build();
    let mut captured = None;
    rspice_app_workbench::ui::raster::render(vec2(width, 120.0), |ui, background| {
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

/// Where the frame put each button it labelled, by its label.
fn buttons(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
        .filter_map(|(_, node)| {
            let bounds = node.bounds()?;
            Some((
                node.label()?.to_owned(),
                egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ),
            ))
        })
        .collect()
}

/// The application with `deck` open on the Netlist workspace at 1024 by
/// 768, the console closed, drawn until its check has run.
fn deck_at_1024(deck: &str) -> (egui::Context, RSpiceApp, egui::FullOutput) {
    use crate::workbench::app::veriloga_design_tests::{frame, open_project_app, themed_context};
    let mut app = open_project_app();
    app.state.seed_test_deck(deck);
    app.state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Netlist);
    app.state.workbench.console_visible = false;
    let ctx = themed_context();
    let mut output = frame(&ctx, &mut app, AT_1024);
    for _ in 0..3 {
        output = frame(&ctx, &mut app, AT_1024);
    }
    (ctx, app, output)
}

const AT_1024: egui::Vec2 = egui::vec2(1024.0, 768.0);

/// At 1024 the inspector is a drawer, so the header's count of the deck's
/// errors is where the check's problems are read from: pressed, it lists
/// them on the Problems page, whose rows go to their lines.
#[test]
fn the_error_count_lists_the_errors_on_the_problems_page() {
    use crate::workbench::app::veriloga_design_tests::frame_with;
    let (ctx, mut app, output) =
        deck_at_1024("amp\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.op\n.end\n");
    let shown = buttons(&output);
    let (_, count) = shown
        .iter()
        .find(|(label, _)| label.ends_with(" error") || label.ends_with(" errors"))
        .unwrap_or_else(|| panic!("the header counts the error: {shown:?}"));
    let at = count.center();
    frame_with(
        &ctx,
        &mut app,
        AT_1024,
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame_with(
        &ctx,
        &mut app,
        AT_1024,
        vec![egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );

    assert!(app.state.workbench.console_visible);
    assert_eq!(
        app.state.workbench.console_page,
        rspice_app_workbench::workbench::state::workbench_session::ConsolePage::Problems
    );
}

/// The header lays out by the shell's layout class, as every workspace
/// does: at 1024 the window is a desktop one, and a deck's warnings are
/// counted beside its chip where they fit.
#[test]
fn at_1024_the_header_counts_the_warnings() {
    let (_, _, output) = deck_at_1024("broken\nR1 out 0\n.op\n.end\n");
    let shown = buttons(&output);
    assert!(
        shown
            .iter()
            .any(|(label, _)| label.ends_with(" warning") || label.ends_with(" warnings")),
        "the header counts the warning: {shown:?}"
    );
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
