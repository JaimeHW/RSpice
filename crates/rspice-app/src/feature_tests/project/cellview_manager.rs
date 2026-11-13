//! The library cellview manager through the whole application: both pages at
//! every supported width, the symbol form's actions on the selected symbol,
//! and a press on the manager drawn in the frame it was clicked in.

use egui::{Modifiers, Rect, vec2};

use crate::state::{Cell, CellViewRef, Library, View, ViewType};
use crate::workbench::framework::requests::OpenSymbolAuthoring;
use crate::workbench::project::workspace_state::LibraryCellviewPage;
use crate::workbench::surfaces::library_cellview_manager::tests::typed_symbol;
use crate::workbench::surfaces::library_cellview_manager::{resolve_symbol_reference, show};
use crate::workbench::{RSpiceApp, RouteTransitionSource, SurfaceId, SurfaceRoute};

fn app_with_typed_symbol() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let definition = typed_symbol();
    let mut view = View::new("symbol", ViewType::Symbol);
    crate::state::store_model_bound_symbol(&definition, &mut view).expect("typed symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("authoring");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("authoring", "amp", "symbol");
    app.state.project.library_cellview_page = LibraryCellviewPage::SymbolForm;
    app
}

fn render(app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, app));
        },
    )
}

fn access_nodes(output: &egui::FullOutput) -> Vec<&egui::accesskit::Node> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("accessibility tree")
        .nodes
        .iter()
        .map(|(_, node)| node)
        .collect()
}

#[test]
fn specialist_renders_both_pages_at_desktop_tablet_and_phone_widths() {
    for page in LibraryCellviewPage::ALL {
        for size in [vec2(1440.0, 900.0), vec2(820.0, 1180.0), vec2(390.0, 844.0)] {
            let mut app = app_with_typed_symbol();
            app.state.project.library_cellview_page = page;
            let output = render(&mut app, size);
            assert!(
                !output.shapes.is_empty(),
                "{} did not render at {}x{}",
                page.label(),
                size.x,
                size.y
            );
        }
    }
}

#[test]
fn symbol_page_exposes_live_preview_terminal_form_and_actions() {
    let mut app = app_with_typed_symbol();
    let output = render(&mut app, vec2(1280.0, 900.0));
    let nodes = access_nodes(&output);
    for label in [
        "Authored symbol preview",
        "Terminal contract",
        "Component form and inheritance",
        "Open symbol editor",
        "Edit component form\u{2026}",
        "Create symbol\u{2026}",
        "Import symbol\u{2026}",
    ] {
        assert!(
            nodes.iter().any(|node| node.label() == Some(label)),
            "missing accessible node {label}"
        );
    }
    assert!(
        output.shapes.len() > 20,
        "the live symbol projection is empty"
    );
}

#[test]
fn exact_selected_symbol_wins_over_an_unrelated_active_document() {
    let app = app_with_typed_symbol();
    assert_ne!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("authoring", "amp", "symbol")
    );
    assert_eq!(
        resolve_symbol_reference(&app),
        Some(CellViewRef::new("authoring", "amp", "symbol"))
    );
}

#[test]
fn explicit_form_action_targets_the_selected_symbol() {
    let mut app = app_with_typed_symbol();
    app.state
        .workbench
        .outbox
        .push(OpenSymbolAuthoring::ParameterForm(Some(CellViewRef::new(
            "authoring",
            "amp",
            "symbol",
        ))));
    app.after_contribution();
    assert_eq!(
        app.state
            .design
            .symbol_parameter_form
            .target
            .as_ref()
            .map(CellViewRef::display_path),
        Some("authoring/amp/symbol".to_owned())
    );
    assert_eq!(
        app.state
            .design
            .symbol_parameter_form
            .draft_form
            .as_ref()
            .map(|form| form.revision),
        Some(4)
    );
}

/// Press the manager's `button` on the Symbol & form page, in whole frames
/// over an open project: the application after the press, and whether the
/// frame that drew the press drew the dialog titled `title`.
fn press_in_the_manager(button: &str, title: &str) -> (RSpiceApp, bool) {
    let mut app = app_with_typed_symbol();
    app.state.open_project_session();
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::LibraryCellviewManager),
            RouteTransitionSource::User,
        )
        .expect("the manager's route is executable");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut frame = eframe::Frame::_new_kittest();
    let mut run = |app: &mut RSpiceApp, pass: u32, events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
                time: Some(f64::from(pass) * 0.1),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        )
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default()
    };
    // The fonts exist from the second frame on, and the page lays out
    // against them on the third.
    let mut nodes = Vec::new();
    for pass in 1..=3 {
        nodes = run(&mut app, pass, Vec::new());
    }
    let bounds = nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.label() == Some(button))
        .and_then(|node| node.bounds())
        .unwrap_or_else(|| panic!("the page draws {button:?}"));
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    };
    let pressed = run(
        &mut app,
        4,
        vec![egui::Event::PointerMoved(at), press(true), press(false)],
    );
    let drew = pressed.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(title)
    });
    (app, drew)
}

/// Edit component form… raises the form designer for the symbol the page
/// shows, and the shell opens it as the manager returns: the frame that
/// drew the press draws the dialog.
#[test]
fn edit_component_form_draws_the_form_designer_in_the_frame_that_pressed_it() {
    let (app, drew) = press_in_the_manager(
        "Edit component form\u{2026}",
        "Open component form designer",
    );
    assert!(
        drew,
        "the frame that drew the press draws the form designer"
    );
    assert_eq!(
        app.state.design.symbol_parameter_form.target,
        Some(CellViewRef::new("authoring", "amp", "symbol"))
    );
}

/// Import symbol… raises the importer bound to the symbol the page shows,
/// and the shell opens it as the manager returns: the frame that drew the
/// press draws the dialog.
#[test]
fn import_symbol_draws_the_importer_in_the_frame_that_pressed_it() {
    let (app, drew) = press_in_the_manager("Import symbol\u{2026}", "Import symbol definition");
    assert!(drew, "the frame that drew the press draws the importer");
    assert_eq!(
        app.state.design.symbol_import.binding_source,
        Some(CellViewRef::new("authoring", "amp", "symbol"))
    );
}

#[test]
fn legacy_symbol_discloses_missing_contract_and_disables_form_editing() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("legacy");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .library_manager
        .select_view("legacy", "amp", "symbol");
    app.state.project.library_cellview_page = LibraryCellviewPage::SymbolForm;

    let output = render(&mut app, vec2(1100.0, 760.0));
    let nodes = access_nodes(&output);
    assert!(
        nodes
            .iter()
            .any(|node| node.label() == Some("Typed contract required"))
    );
    assert!(
        nodes.iter().any(|node| {
            node.label() == Some("Edit component form\u{2026}") && node.is_disabled()
        })
    );
}

#[test]
fn safe_mode_disables_every_symbol_mutation_but_keeps_editor_handoff() {
    let mut app = app_with_typed_symbol();
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "{}".to_owned(),
    );

    let output = render(&mut app, vec2(1100.0, 760.0));
    let nodes = access_nodes(&output);
    for label in [
        "Edit component form\u{2026}",
        "Create symbol\u{2026}",
        "Import symbol\u{2026}",
    ] {
        assert!(
            nodes
                .iter()
                .any(|node| node.label() == Some(label) && node.is_disabled()),
            "{label} remained enabled in safe mode"
        );
    }
    assert!(
        nodes
            .iter()
            .any(|node| { node.label() == Some("Open symbol editor") && !node.is_disabled() })
    );
}
