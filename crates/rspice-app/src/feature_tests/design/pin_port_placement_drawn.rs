//! Placing a pin or port, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::placement::pin_port::tests::{
    self as form, dialog_input, key_event,
};
use rspice_app_design::workbench::app::dialogs::placement::pin_port::*;
use rspice_app_state::state::ComponentType;
use rspice_app_state::state::Tool;
use rspice_app_state::state::{Point, PortDirection, PortDiscipline, PortSignalType};

/// Draw the form as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    form::render(&mut app.state.design.pin_port, &mut app.state.model, ctx);
}

/// The batch the form holds, over what it reads.
fn draft(state: &AppState) -> Draft {
    form::draft(&state.design.pin_port, &state.model)
}

/// Every labelled node the form publishes, with its role and bounds, after a
/// few frames at the desktop size.
fn published(app: &mut RSpiceApp) -> Vec<(egui::accesskit::Role, String, egui::Rect)> {
    form::published(&mut app.state.design.pin_port, &mut app.state.model)
}

/// What the names declare, over the model they are declared in.
fn declaration_line(state: &AppState, names: &[String]) -> Option<String> {
    rspice_app_design::workbench::app::dialogs::placement::pin_port::declaration_line(
        &state.model,
        names,
    )
}

fn open_with(state: &mut AppState, names: &str) {
    open_create_pins(&mut state.design.pin_port, &state.model);
    state.design.pin_port.fields.names = names.to_owned();
}

/// One port already on the sheet, so duplicate refusals have something to
/// collide with.
fn place_port(state: &mut AppState, name: &str) {
    let id = state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Port,
        Point::origin(),
    );
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port exists")
        .value = name.to_owned();
}

#[test]
fn names_split_on_whitespace_and_each_is_validated_in_the_models_words() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "  INP   INN\tOUT  ");
    assert_eq!(
        draft(&app.state),
        Draft::Ready(vec!["INP".to_owned(), "INN".to_owned(), "OUT".to_owned()])
    );

    // A refusal is the model's sentence, prefixed with the name it is about
    // only when there is more than one name to be about.
    app.state.design.pin_port.fields.names = "DATA[3]".to_owned();
    let expected = app
        .state
        .model
        .schematic
        .validate_new_port_name("DATA[3]")
        .expect_err("one member of a bus is not a pin")
        .to_string();
    assert_eq!(draft(&app.state), Draft::Refused(expected.clone()));

    app.state.design.pin_port.fields.names = "OK DATA[3]".to_owned();
    assert_eq!(
        draft(&app.state),
        Draft::Refused(format!("DATA[3]: {expected}"))
    );
}

#[test]
fn a_name_listed_twice_is_refused_before_arming() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN OUT en");
    assert_eq!(
        draft(&app.state),
        Draft::Refused("en is listed twice.".to_owned())
    );

    let names = (0..=MAX_NAMES)
        .map(|index| format!("P{index}"))
        .collect::<Vec<_>>()
        .join(" ");
    app.state.design.pin_port.fields.names = names;
    assert_eq!(draft(&app.state), Draft::Refused(TOO_MANY.to_owned()));
}

#[test]
fn the_derived_line_states_pins_conductors_and_port_list_positions() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN");
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()).as_deref(),
        Some("1 pin \u{00b7} 1 conductor \u{00b7} port-list position 1")
    );
    assert_eq!(deck_bits_line(draft(&app.state).names()), None);

    app.state.design.pin_port.fields.names = "EN DATA[7:0] OUT".to_owned();
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()).as_deref(),
        Some("3 pins \u{00b7} 10 conductors \u{00b7} port-list positions 1 to 3")
    );
    // The deck bits are shown for the one case a reader cannot work out: a
    // single name that declares a range.
    assert_eq!(deck_bits_line(draft(&app.state).names()), None);

    app.state.design.pin_port.fields.names = "DATA[7:0]".to_owned();
    assert_eq!(
        deck_bits_line(draft(&app.state).names()).as_deref(),
        Some("DATA#7 DATA#6 DATA#5 DATA#4 DATA#3 DATA#2 DATA#1 DATA#0")
    );

    // An empty draft states nothing rather than stating zero.
    app.state.design.pin_port.fields.names.clear();
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()),
        None
    );
}

#[test]
fn enter_arms_the_whole_sequence_without_touching_the_document() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN OUT VDD");
    let authority = app.state.design.pin_port.authority.clone();
    app.state.design.pin_port.fields.signal_type = PortSignalType::Logic;
    app.state.design.pin_port.fields.discipline = PortDiscipline::Logic;

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.pin_port.open);
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(rspice_app_state::state::ComponentType::Port)
    );
    let sequence = app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_port_sequence
        .as_ref()
        .expect("the batch is armed");
    assert_eq!(
        sequence.names.iter().cloned().collect::<Vec<_>>(),
        ["INP", "INN", "OUT", "VDD"]
    );
    assert_eq!(sequence.total, 4);
    assert_eq!(sequence.direction, PortDirection::In);
    assert_eq!(sequence.signal_type, PortSignalType::Logic);
    assert_eq!(sequence.discipline, PortDiscipline::Logic);
    assert_eq!(sequence.authority, authority);
    assert!(app.state.design.pin_port.authority.is_none());

    assert!(app.state.model.schematic.document().components.is_empty());
    assert!(!app.state.model.schematic.session.is_dirty);
    assert!(!app.state.model.schematic.can_undo());
    // The canvas owns the keyboard, so R, M and Esc work before the pointer
    // has moved over it.
    assert_eq!(
        ctx.memory(|memory| memory.focused()),
        Some(egui::Id::new("rspice-schematic-canvas-interaction"))
    );
}

#[test]
fn reopening_while_armed_offers_the_remaining_names() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN OUT");
    app.state.design.pin_port.fields.signal_type = PortSignalType::Logic;
    app.state.design.pin_port.fields.discipline_touched = true;
    app.state.design.pin_port.fields.discipline = PortDiscipline::Wreal;

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });
    app.state
        .model
        .schematic
        .session
        .editor
        .pending_port_sequence
        .as_mut()
        .expect("armed")
        .advance();

    open_create_pins(&mut app.state.design.pin_port, &app.state.model);
    assert!(app.state.design.pin_port.open);
    assert_eq!(app.state.design.pin_port.fields.names, "INN OUT");
    assert_eq!(
        app.state.design.pin_port.fields.signal_type,
        PortSignalType::Logic
    );
    assert_eq!(
        app.state.design.pin_port.fields.discipline,
        PortDiscipline::Wreal
    );
}

/// The prefill suggests from the last batch and nothing else. It is empty the
/// first time, because a name invented before the reader has named anything is
/// sample data.
#[test]
fn the_prefill_is_empty_until_a_batch_has_been_armed() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_create_pins(&mut app.state.design.pin_port, &app.state.model);
    assert_eq!(app.state.design.pin_port.fields.names, "");

    app.state.design.pin_port.fields.names = "BIAS".to_owned();
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });
    app.state.model.schematic.cancel_tool();
    place_port(&mut app.state, "BIAS");

    open_create_pins(&mut app.state.design.pin_port, &app.state.model);
    assert_eq!(app.state.design.pin_port.fields.names, "BIAS_2");
}

#[test]
fn a_read_only_or_changed_document_blocks_the_form_and_says_so() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN");
    // Same-document edits are allowed; names and interface order are read live.
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    assert_eq!(draft(&app.state), Draft::Ready(vec!["EN".to_owned()]));
    app.state.model.schematic.session.read_only = true;
    assert_eq!(draft(&app.state), Draft::Blocked(READ_ONLY));

    app.state.model.schematic.session.read_only = false;
    assert!(matches!(draft(&app.state), Draft::Ready(_)));
    app.state.model.active_schematic_epoch = app.state.model.active_schematic_epoch.wrapping_add(1);
    assert_eq!(draft(&app.state), Draft::Blocked(DOCUMENT_CHANGED));
}

#[test]
fn a_blocked_form_never_arms_on_enter() {
    use rspice_app_state::state::{CellViewRef, SheetDefinition, SheetPortPolicy, ViewType};

    for change in [
        "document",
        "design",
        "occurrence",
        "sheet",
        "missing",
        "read-only",
        "safe-mode",
    ] {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let master = CellViewRef::new("work", "pin_child", "schematic");
        app.state.model.workspace.descend_into(
            "X1".to_owned(),
            master.clone(),
            ViewType::Schematic,
        );
        let first = app
            .state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        open_with(&mut app.state, "EN");
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
        match change {
            "document" => app.state.model.active_schematic_epoch += 1,
            "design" => app.state.model.design_execution_epoch += 1,
            "occurrence" => {
                app.state.model.workspace.ascend_one().unwrap();
                app.state.model.workspace.descend_into(
                    "X2".to_owned(),
                    master,
                    ViewType::Schematic,
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
                        SheetDefinition {
                            name: "Sheet 2".to_owned(),
                            port_policy: SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(2),
                        },
                        Some(first),
                    )
                    .unwrap();
                catalog.set_active(second).unwrap();
            }
            "missing" => app.state.design.pin_port.authority = None,
            "read-only" => app.state.model.schematic.session.read_only = true,
            "safe-mode" => app.state.model.safe_mode.activate(
                rspice_app_state::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "pin form test".to_owned(),
            ),
            _ => unreachable!(),
        }
        let expected = if matches!(change, "read-only" | "safe-mode") {
            READ_ONLY
        } else {
            DOCUMENT_CHANGED
        };
        assert_eq!(draft(&app.state), Draft::Blocked(expected), "{change}");
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render(&mut app, ctx)
        });

        assert!(app.state.design.pin_port.open);
        assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
        assert!(
            app.state
                .model
                .schematic
                .session
                .editor
                .pending_port_sequence
                .is_none()
        );
        assert!(app.state.model.schematic.document().components.is_empty());
        assert!(!app.state.model.schematic.can_undo());
        if !matches!(change, "read-only" | "safe-mode") {
            app.state.design.pin_port.close();
            open_with(&mut app.state, "EN");
            assert_eq!(draft(&app.state), Draft::Ready(vec!["EN".to_owned()]));
        }
    }
}

/// Escape closes at once. There is no draft worth a confirmation here: the
/// form owns nothing but four answers, and the command is one chord away.
#[test]
fn escape_closes_the_form_at_once() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN");

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render(&mut app, ctx)
    });

    assert!(!app.state.design.pin_port.open);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    assert!(app.state.model.schematic.document().components.is_empty());
}

/// A duplicate of a name already on the sheet is refused before arming, in the
/// model's words.
#[test]
fn a_name_already_on_the_sheet_is_refused_before_arming() {
    let mut app = RSpiceApp::test_instance();
    place_port(&mut app.state, "BIAS_EN");
    open_with(&mut app.state, "bias_en");
    let expected = app
        .state
        .model
        .schematic
        .validate_new_port_name("bias_en")
        .expect_err("a taken name is refused")
        .to_string();
    assert_eq!(draft(&app.state), Draft::Refused(expected));
}

// ============================================================================
// Fit
// ============================================================================

/// An empty form says what Place waits for, and both choices span the
/// control column the Discipline list spans, so no row stops short of its
/// neighbours.
#[test]
fn an_empty_form_says_what_place_waits_for_and_its_choices_fill_the_column() {
    use egui::accesskit::Role;
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "");
    let nodes = published(&mut app);
    assert!(
        nodes.iter().any(|(_, label, _)| label == NAMES_MISSING),
        "the disabled Place does not say why"
    );
    let control = |name: &str| {
        nodes
            .iter()
            .find(|(role, label, _)| label == name && *role != Role::Label)
            .map(|(_, _, rect)| *rect)
            .unwrap_or_else(|| panic!("{name} is published"))
    };
    let discipline = control("Discipline");
    for (first, last) in [("Input", "Supply"), ("Analog", "Power")] {
        let (first, last) = (control(first), control(last));
        assert!(
            (first.left() - discipline.left()).abs() < 1.0
                && (last.right() - discipline.right()).abs() < 1.0,
            "{first:?}..{last:?} against {discipline:?}"
        );
    }
}
