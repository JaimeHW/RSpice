//! Placing a net label, drawn through a whole application.

use crate::state::CrossSheetPortDirection;
use crate::state::NetLabel;
use crate::state::NetLabelKind;
use crate::state::Point;
use crate::state::Tool;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::placement::net_label::tests::{
    apply_commit, key_event, render, validate_draft,
};
use crate::workbench::app::dialogs::placement::net_label::*;
use crate::workbench::app::open_net_label_placement_in;
use crate::workbench::app::place_sequenced_net_label_in;
use crate::workbench::app_state::AppState;
use egui::Context;

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

fn open_at(app: &mut RSpiceApp, anchor: Point) {
    app.state.model.schematic.session.editor.tool = Tool::Label;
    let dialog_modal_open = app.state.dialog_modal_open();
    assert!(open_net_label_placement_in(
        &mut app.state.design.net_label_placement,
        &mut app.state.model,
        &mut app.state.workbench,
        dialog_modal_open,
        anchor
    ));
}

fn open_connector_at(app: &mut RSpiceApp, anchor: Point) {
    app.state.model.schematic.session.editor.tool = Tool::OffSheetConnector;
    let dialog_modal_open = app.state.dialog_modal_open();
    assert!(open_net_label_placement_in(
        &mut app.state.design.net_label_placement,
        &mut app.state.model,
        &mut app.state.workbench,
        dialog_modal_open,
        anchor
    ));
}

#[test]
fn opening_input_reaches_the_name_field_for_both_placement_kinds() {
    for connector in [false, true] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        if connector {
            open_connector_at(&mut app, Point::origin());
        } else {
            open_at(&mut app, Point::origin());
        }
        // The entry asks for the caret once its field exists, so the
        // frame that builds it is the frame before the one that can be
        // typed into.
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render(&mut app, ctx);
        });
        let _ = ctx.run_ui(
            dialog_input(vec![egui::Event::Paste("BUSY".into())]),
            |ctx| {
                render(&mut app, ctx);
            },
        );
        assert_eq!(app.state.design.net_label_placement.name, "BUSY");
        assert!(app.state.model.schematic.document().net_labels.is_empty());
    }
}

#[test]
fn anchor_capture_is_non_mutating_and_valid_commit_is_one_undo_unit() {
    let mut app = RSpiceApp::test_instance();
    let topology_before = app.state.model.schematic.topology_version();
    open_at(&mut app, Point::new(40, -20));
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());
    assert_eq!(
        app.state.model.schematic.topology_version(),
        topology_before
    );

    app.state.design.net_label_placement.name = "DATA[7]".to_owned();
    let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
        panic!("valid typed label draft");
    };
    let id = apply_commit(&mut app.state, *commit).expect("placement");
    assert_eq!(
        app.state.model.schematic.document().net_labels,
        vec![NetLabel::new(id, Point::new(40, -20), "DATA[7]")]
    );
    assert!(app.state.model.schematic.can_undo());
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Label);
    assert!(app.state.model.schematic.undo());
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn invalid_name_cannot_produce_a_commit_or_history() {
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::origin());
    app.state.design.net_label_placement.name = "bell\u{7}node".to_owned();
    assert!(matches!(
        validate_draft(&app.state),
        DraftValidation::Invalid(_)
    ));
    // One bad name in a list refuses the whole entry, naming it.
    app.state.design.net_label_placement.name = "a DATA[7 c".to_owned();
    match validate_draft(&app.state) {
        DraftValidation::Invalid(message) => assert!(message.starts_with("DATA[7:")),
        other => panic!("a list with a bad name was accepted: {other:?}"),
    }
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());

    app.state.design.net_label_placement.name = "DATA[7".to_owned();
    assert!(matches!(
        validate_draft(&app.state),
        DraftValidation::Invalid(_)
    ));
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());
}

/// Place what the entry names at `anchor`.
fn commit_entry(state: &mut AppState, anchor: Point, entry: &str) {
    state.model.schematic.session.editor.tool = Tool::Label;
    let dialog_modal_open = state.dialog_modal_open();
    assert!(open_net_label_placement_in(
        &mut state.design.net_label_placement,
        &mut state.model,
        &mut state.workbench,
        dialog_modal_open,
        anchor
    ));
    state.design.net_label_placement.name = entry.to_owned();
    let DraftValidation::Valid(commit) = validate_draft(state) else {
        panic!("{entry} is a valid entry");
    };
    apply_commit(state, *commit).expect("placement");
    state.design.net_label_placement.close();
}

fn label_names(app: &RSpiceApp) -> Vec<String> {
    app.state
        .model
        .schematic
        .document()
        .net_labels
        .iter()
        .map(|label| label.name.clone())
        .collect()
}

fn next_name(app: &RSpiceApp) -> Option<String> {
    app.state
        .model
        .schematic
        .session
        .editor
        .pending_net_label_sequence
        .as_ref()
        .and_then(|sequence| sequence.next_name())
}

/// After `D0` the tool stays armed with `D1`: each click places the
/// next number without the entry, as its own undo step (audit G-6).
#[test]
fn a_numbered_label_keeps_the_tool_armed_with_the_next_name() {
    let mut app = RSpiceApp::test_instance();
    commit_entry(&mut app.state, Point::new(0, 0), "D0");
    assert_eq!(next_name(&app).as_deref(), Some("D1"));

    let dialog_modal_open = app.state.dialog_modal_open();
    assert!(place_sequenced_net_label_in(
        &mut app.state.model,
        &mut app.state.workbench,
        dialog_modal_open,
        Point::new(0, 20)
    ));
    let dialog_modal_open = app.state.dialog_modal_open();
    assert!(place_sequenced_net_label_in(
        &mut app.state.model,
        &mut app.state.workbench,
        dialog_modal_open,
        Point::new(0, 40)
    ));
    assert!(!app.state.design.net_label_placement.open);
    assert_eq!(label_names(&app), ["D0", "D1", "D2"]);
    assert_eq!(next_name(&app).as_deref(), Some("D3"));
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Label);

    assert!(app.state.model.schematic.undo());
    assert_eq!(label_names(&app), ["D0", "D1"]);
    // Esc ends the tool and the sequence with it.
    app.state.model.schematic.cancel_interaction_step();
    assert_eq!(next_name(&app), None);
}

/// A list or a bus range places one name per click, then the next click
/// asks for a name again; a range typed onto a bus names the bus.
#[test]
fn a_list_or_a_bus_range_places_one_name_per_click() {
    for (entry, expected) in [
        ("a b c", vec!["a", "b", "c"]),
        ("D<0:2>", vec!["D<0>", "D<1>", "D<2>"]),
    ] {
        let mut app = RSpiceApp::test_instance();
        commit_entry(&mut app.state, Point::new(0, 0), entry);
        let mut y = 0;
        while next_name(&app).is_some() {
            y += 20;
            let dialog_modal_open = app.state.dialog_modal_open();
            assert!(place_sequenced_net_label_in(
                &mut app.state.model,
                &mut app.state.workbench,
                dialog_modal_open,
                Point::new(0, y)
            ));
        }
        assert_eq!(label_names(&app), expected);
        let dialog_modal_open = app.state.dialog_modal_open();
        assert!(!place_sequenced_net_label_in(
            &mut app.state.model,
            &mut app.state.workbench,
            dialog_modal_open,
            Point::new(0, 200)
        ));
    }

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(
            crate::state::Bus::segment(90, Point::new(-50, 0), Point::new(50, 0), None)
                .expect("bus"),
        );
    commit_entry(&mut app.state, Point::new(0, 0), "D<0:2>");
    assert_eq!(label_names(&app), ["D<0:2>"]);
    assert_eq!(next_name(&app), None);
}

/// Leaving the entry leaves the document alone and keeps the tool armed,
/// so the next click names the next net.
#[test]
fn closing_the_entry_leaves_schematic_and_history_untouched() {
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::new(10, 20));
    app.state.design.net_label_placement.name = "sense".to_owned();
    app.state.design.net_label_placement.close();
    assert!(!app.state.design.net_label_placement.open);
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Label);
}

#[test]
fn read_only_and_stale_authority_fail_closed() {
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::origin());
    app.state.design.net_label_placement.name = "sense".to_owned();
    app.state.model.schematic.session.read_only = true;
    assert!(matches!(
        validate_draft(&app.state),
        DraftValidation::Invalid(_)
    ));
    app.state.model.schematic.session.read_only = false;
    let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
        panic!("authority should recover after read-only mode is cleared");
    };
    app.state.model.schematic.bump_topology_version();
    assert!(matches!(
        validate_draft(&app.state),
        DraftValidation::Invalid(_)
    ));
    assert!(apply_commit(&mut app.state, *commit).is_err());
    assert!(app.state.model.schematic.document().net_labels.is_empty());
    assert!(!app.state.model.schematic.can_undo());
}

/// Enter places the label; Escape leaves at once, because the entry owns
/// nothing there is anything to confirm the discard of.
#[test]
fn rendered_enter_commits_and_escape_closes_at_once() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::new(30, 50));
    app.state.design.net_label_placement.name = "sense_out".to_owned();

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        render(&mut app, ctx)
    });
    assert!(!app.state.design.net_label_placement.open);
    assert_eq!(
        app.state.model.schematic.document().net_labels[0].name,
        "sense_out"
    );
    assert!(app.state.model.schematic.can_undo());

    open_at(&mut app, Point::new(60, 50));
    app.state.design.net_label_placement.name = "discard_me".to_owned();
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| render(&mut app, ctx));
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        render(&mut app, ctx)
    });
    assert!(!app.state.design.net_label_placement.open);
    assert_eq!(app.state.model.schematic.document().net_labels.len(), 1);
}

/// The entry sits where the click did, not in the middle of the screen,
/// and stays there while the name is typed.
#[test]
fn the_entry_is_anchored_at_the_click_and_does_not_follow_the_pointer() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::new(30, 50));
    let click = egui::pos2(420.0, 260.0);
    let mut input = dialog_input(vec![egui::Event::PointerMoved(click)]);
    input.events.push(egui::Event::PointerMoved(click));
    let _ = ctx.run_ui(input, |ctx| render(&mut app, ctx));
    let anchored = app
        .state
        .design
        .net_label_placement
        .screen_anchor
        .expect("the entry took its place from the click");
    assert_eq!(anchored, click);

    let _ = ctx.run_ui(
        dialog_input(vec![egui::Event::PointerMoved(egui::pos2(900.0, 700.0))]),
        |ctx| render(&mut app, ctx),
    );
    assert_eq!(
        app.state.design.net_label_placement.screen_anchor,
        Some(anchored),
        "the card does not chase the pointer"
    );
}

#[test]
fn committed_label_is_synchronized_to_the_project_owned_cell_view() {
    let mut app = RSpiceApp::test_instance();
    open_at(&mut app, Point::new(80, 30));
    app.state.design.net_label_placement.name = "project_net".to_owned();
    let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
        panic!("valid label draft");
    };

    apply_commit(&mut app.state, *commit).expect("placement");

    let key = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let retained = app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&key)
        .expect("active schematic is retained by the project workspace");
    assert_eq!(retained.document().net_labels.len(), 1);
    assert_eq!(retained.document().net_labels[0].pos, Point::new(80, 30));
    assert_eq!(retained.document().net_labels[0].name, "project_net");
}

#[test]
fn the_armed_tool_decides_the_kind_and_publishes_it_with_the_name() {
    let mut app = RSpiceApp::test_instance();
    open_connector_at(&mut app, Point::new(60, -40));
    assert_eq!(
        app.state.design.net_label_placement.kind,
        NetLabelKind::OffSheet {
            direction: CrossSheetPortDirection::default()
        }
    );

    app.state.design.net_label_placement.name = "BIAS".to_owned();
    app.state.design.net_label_placement.kind = NetLabelKind::OffSheet {
        direction: CrossSheetPortDirection::Output,
    };
    let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
        panic!("valid connector draft");
    };
    let id = apply_commit(&mut app.state, *commit).expect("placement");

    assert_eq!(
        app.state.model.schematic.document().net_labels,
        vec![NetLabel::off_sheet(
            id,
            Point::new(60, -40),
            "BIAS",
            CrossSheetPortDirection::Output
        )]
    );
    assert_eq!(
        app.state.model.schematic.undo_description(),
        Some("place off-sheet connector"),
        "history must name what was placed"
    );
    assert!(app.state.model.schematic.undo());
    assert!(app.state.model.schematic.document().net_labels.is_empty());

    // The plain label tool still publishes a local label through the same
    // transaction. The primary action closes the dialog after a commit,
    // and no placement opens over a modal that is still up.
    app.state.design.net_label_placement.close();
    open_at(&mut app, Point::new(10, 10));
    assert_eq!(
        app.state.design.net_label_placement.kind,
        NetLabelKind::Local
    );
}
