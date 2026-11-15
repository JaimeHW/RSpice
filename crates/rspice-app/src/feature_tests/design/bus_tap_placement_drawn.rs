//! Placing a bus tap, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::placement::bus_tap::tests::{
    self as form, key_event,
};
use rspice_app_design::workbench::app::dialogs::placement::bus_tap::*;
use rspice_app_state::state::Tool;
use rspice_app_state::state::{Bus, BusDeclaration, Point};

/// Draw the form as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    form::render(&mut app.state.design.bus_tap, &mut app.state.model, ctx);
}

fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    }
}

fn replace_text(value: &str) -> Vec<egui::Event> {
    vec![
        egui::Event::Key {
            key: egui::Key::A,
            physical_key: Some(egui::Key::A),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
        },
        egui::Event::Paste(value.to_owned()),
    ]
}

/// Draw one declared bus on the active sheet.
fn declare_bus(state: &mut AppState, declaration: &str) {
    let id = state.model.schematic.next_id();
    let declaration = BusDeclaration::parse(declaration).expect("declaration");
    state.model.schematic.document_mut_for_test().buses.push(
        Bus::segment(id, Point::new(0, 0), Point::new(40, 0), Some(declaration))
            .expect("bus segment"),
    );
}

#[test]
fn opening_input_reaches_the_bus_field() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);

    // The form asks for the caret once its rows exist, so the frame that
    // builds them is the frame before the one that can be typed into.
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(replace_text("DATA[31:0]")), |ctx| {
        render(&mut app, ctx)
    });

    assert_eq!(app.state.design.bus_tap.fields.bus, "DATA[31:0]");
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_bus_tap
            .is_none()
    );
}

/// The content bug this form shipped: it opened holding `DATA[15:0]` and
/// `DATA[7:0]` as values, so Enter on an untouched form placed a tap on a
/// bus the design had never declared.
#[test]
fn an_untouched_form_taps_nothing_and_prefills_only_the_one_declared_bus() {
    let mut app = RSpiceApp::test_instance();
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
    assert_eq!(app.state.design.bus_tap.fields.bus, "");
    assert_eq!(app.state.design.bus_tap.fields.slice, "");
    assert!(!app.state.design.bus_tap.fields.validate().can_commit());

    // One declared bus is the only answer, so the field starts there. The
    // slice is still the reader's, so the form still cannot commit.
    app.state.design.bus_tap.close();
    declare_bus(&mut app.state, "DATA[15:0]");
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
    assert_eq!(app.state.design.bus_tap.fields.bus, "DATA[15:0]");
    assert_eq!(app.state.design.bus_tap.fields.slice, "");
    assert!(!app.state.design.bus_tap.fields.validate().can_commit());

    // Two declarations are a choice, and a form does not choose for the
    // reader.
    app.state.design.bus_tap.close();
    declare_bus(&mut app.state, "ADDR[7:0]");
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
    assert_eq!(app.state.design.bus_tap.fields.bus, "");
}

/// Escape leaves at once. The draft owns nothing, so there is nothing to
/// confirm the discard of.
#[test]
fn escape_closes_the_form_at_once_and_places_nothing() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
    fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.bus_tap.open);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_bus_tap
            .is_none()
    );
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
}

#[test]
fn rendered_primary_revalidates_publishes_and_arms_the_bus_tap_tool() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
    fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.bus_tap.open);
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::BusTap);
    let pending = app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_bus_tap
        .as_ref()
        .expect("validated pending bus tap");
    assert_eq!(
        pending.configuration.bus_declaration.to_string(),
        "DATA[15:0]"
    );
    assert_eq!(pending.configuration.slice.to_string(), "DATA[7:0]");
    assert!(
        pending
            .authority
            .matches(&rspice_app_design::workbench::app::schematic::edit_authority::schematic_editor_request_source(
                &app.state
            ))
    );
}

#[test]
fn retained_dialog_cannot_arm_after_its_context_or_permission_changes() {
    for change in [
        "design",
        "buffer",
        "occurrence",
        "sheet",
        "content",
        "read-only",
        "safe-mode",
        "missing",
    ] {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let master = rspice_app_state::state::CellViewRef::new("work", "tap_parent", "schematic");
        app.state.model.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            rspice_app_state::state::ViewType::Schematic,
        );
        let first = app
            .state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        open_bus_tap(&mut app.state.design.bus_tap, &app.state.model);
        fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
        match change {
            "design" => app.state.model.design_execution_epoch += 1,
            "buffer" => app.state.model.active_schematic_epoch += 1,
            "occurrence" => {
                app.state.model.workspace.ascend_one().unwrap();
                app.state.model.workspace.descend_into(
                    "X2".to_owned(),
                    master.clone(),
                    rspice_app_state::state::ViewType::Schematic,
                );
            }
            "sheet" => {
                let catalog = app
                    .state
                    .model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&master.key())
                    .unwrap();
                let second = catalog
                    .create_sheet(
                        rspice_app_state::state::SheetDefinition {
                            name: "Sheet 2".to_owned(),
                            template: rspice_app_state::state::SheetTemplate::AnalogSchematic,
                            port_policy:
                                rspice_app_state::state::SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(2),
                        },
                        Some(first),
                    )
                    .unwrap();
                catalog.set_active(second).unwrap();
            }
            "content" => {
                app.state.model.schematic.add_component(
                    rspice_app_state::state::ComponentType::Resistor,
                    rspice_app_state::state::Point::new(40, 20),
                );
            }
            "read-only" => app.state.model.schematic.session.read_only = true,
            "safe-mode" => app.state.model.safe_mode.activate(
                rspice_app_state::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "bus-tap test".to_owned(),
            ),
            "missing" => app.state.design.bus_tap.source = None,
            _ => unreachable!(),
        }
        assert!(
            placement_source(&app.state.design.bus_tap, &app.state.model).is_err(),
            "{change}"
        );
        let content = app.state.model.schematic.content_version();
        let topology = app.state.model.schematic.topology_version();
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render(&mut app, ctx)
        });
        assert!(app.state.design.bus_tap.open, "{change}");
        assert_eq!(
            app.state.model.schematic.session.editor.tool,
            Tool::Select,
            "{change}"
        );
        assert!(
            app.state
                .model
                .schematic
                .session
                .editor
                .pending_bus_tap
                .is_none(),
            "{change}"
        );
        assert_eq!(
            app.state.model.schematic.content_version(),
            content,
            "{change}"
        );
        assert_eq!(
            app.state.model.schematic.topology_version(),
            topology,
            "{change}"
        );
    }
}

/// Type a complete tap into the open form.
pub(crate) fn fill(state: &mut AppState, bus: &str, slice: &str) {
    state.design.bus_tap.fields.bus = bus.to_owned();
    state.design.bus_tap.fields.slice = slice.to_owned();
}
