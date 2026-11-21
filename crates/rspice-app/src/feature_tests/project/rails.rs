//! The Project rails drawn through the whole application: each rail lists what
//! the reader selects from.

use crate::workbench::RSpiceApp;
use egui::{Event, Modifiers};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{Cell, Library, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::docks::navigator::show;

use super::surface::painted_text;

/// One pass of the Project rail as the shell draws it.
fn rail_frame(ctx: &egui::Context, app: &mut RSpiceApp, events: Vec<Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, app);
            });
        },
    )
}

/// On the Overview, a library chosen on the rail opens the Library page on
/// it, and the rail's filter says that it filters libraries.
#[test]
fn a_library_on_the_overview_rail_opens_the_library_page_on_it() {
    let mut app = RSpiceApp::test_instance();
    let mut bench = Library::new("bench");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    bench.add_cell(cell);
    app.state.model.library_manager.add_library(bench);
    app.state.workbench.workspace = Workspace::Project;
    app.state.project.project_page = ProjectPage::Overview;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut settled = rail_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        settled = rail_frame(&ctx, &mut app, Vec::new());
    }
    let painted = painted_text(&settled);
    assert!(painted.contains("Filter libraries\u{2026}"), "{painted}");
    assert!(!painted.contains("cells, views"), "{painted}");
    let bounds = settled
        .platform_output
        .accesskit_update
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == "bench").then_some(node.bounds()?))
        .expect("the rail lists the library");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let _ = rail_frame(
        &ctx,
        &mut app,
        vec![
            Event::PointerMoved(at),
            Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::default(),
            },
            Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::default(),
            },
        ],
    );
    assert_eq!(app.state.project.project_page, ProjectPage::Library);
    assert_eq!(
        app.state.model.library_manager.selected_library.as_deref(),
        Some("bench")
    );
}

/// Whether the rail lists a library shown as `name`, once its layout has
/// settled, and whether the Overview's Cells table lists any cell of it.
fn rail_and_overview_list(app: &mut RSpiceApp, name: &str) -> (bool, bool) {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut settled = rail_frame(&ctx, app, Vec::new());
    for _ in 0..2 {
        settled = rail_frame(&ctx, app, Vec::new());
    }
    let on_rail = settled
        .platform_output
        .accesskit_update
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .any(|(_, node)| node.label() == Some(name));
    let snapshot =
        rspice_app_project::workbench::surfaces::project::overview::OverviewSnapshot::capture(
            &egui::Context::default(),
            &crate::workbench::features::cx_with_foreign_answers(app, |_| true),
        );
    let on_overview = snapshot.cell_rows().any(|row| row.library == name);
    (on_rail, on_overview)
}

/// A netlist-first project's starter library, holding nothing but the
/// pristine top cell its deck stands in for, is on neither the rail nor the
/// Overview; once the reader keeps a cell of their own in it, it is on both.
/// The rail and the Overview read one rule.
#[test]
fn a_netlist_first_rail_leaves_out_the_starter_library_as_the_overview_does() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck("V1 out 0 1\n.op\n.end\n".to_owned(), None, "front_end.sp")
    );
    app.state.workbench.workspace = Workspace::Project;
    app.state.project.project_page = ProjectPage::Overview;
    let root = app
        .state
        .model
        .workspace
        .content
        .project
        .root_library
        .clone();
    let mut starter = app
        .state
        .model
        .library_manager
        .get_library(&root)
        .cloned()
        .expect("a netlist-first project keeps its starter library");
    let name = starter.display_name().to_owned();
    assert_eq!(
        rail_and_overview_list(&mut app, &name),
        (false, false),
        "the starter library holds only the pristine top cell"
    );

    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    starter.add_cell(cell);
    app.state.model.library_manager.add_library(starter);
    assert_eq!(
        rail_and_overview_list(&mut app, &name),
        (true, true),
        "a cell of the reader's own is part of the design"
    );
}
