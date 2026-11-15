//! Find and replace drawn and applied through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::find_replace::tests::part;
use rspice_app_design::workbench::app::dialogs::find_replace::*;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::state::Point;
use rspice_app_state::state::{LibraryCellInstance, NetLabel, SchematicState};

/// Each option row is one line, its checks in the columns the scope's
/// segments stand in, and every row of the form is one pitch from the
/// next.
#[test]
fn the_options_stand_on_one_grid() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.design.find_replace.open = true;
    let mut render = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui.ctx()),
        )
    };
    let _ = render();
    let nodes = render()
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    let rect = |label: &str| {
        let bounds = nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| {
                node.label() == Some(label)
                    && matches!(
                        node.role(),
                        egui::accesskit::Role::TextInput
                            | egui::accesskit::Role::CheckBox
                            | egui::accesskit::Role::RadioButton
                    )
            })
            .and_then(|node| node.bounds())
            .unwrap_or_else(|| panic!("{label} is not published with its bounds"));
        egui::Rect::from_min_max(
            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
        )
    };
    let rows = [
        vec!["Find"],
        vec!["Replace"],
        vec!["Instance names", "Net labels", "Values"],
        SCOPES.to_vec(),
        vec!["Whole word", "Match case", "Regular expression"],
    ];
    let centres: Vec<f32> = rows
        .iter()
        .map(|row| {
            let centre = rect(row[0]).center().y;
            for item in row {
                assert!(
                    (rect(item).center().y - centre).abs() <= 0.5,
                    "{item} is not on the line {} is on",
                    row[0]
                );
            }
            centre
        })
        .collect();
    let pitch = centres[1] - centres[0];
    for pair in centres.windows(2) {
        assert!(
            (pair[1] - pair[0] - pitch).abs() <= 0.5,
            "the rows step {centres:?}, not one pitch"
        );
    }
    for column in 0..3 {
        let left = rect(SCOPES[column]).left();
        for row in [&rows[2], &rows[4]] {
            assert!(
                (rect(row[column]).left() - left).abs() <= 0.5,
                "{} does not start where {} does",
                row[column],
                SCOPES[column]
            );
        }
    }
    let (find, scope) = (rect("Find"), rect(SCOPES[2]));
    assert!(
        (scope.right() - find.right()).abs() <= 0.5,
        "the scope does not span the field column"
    );
}

/// An instance renamed by Replace all ends as the Rename form leaves
/// it: the probe and the saved output that name it follow it, and one
/// Undo takes the whole change back.
#[test]
fn replace_all_renames_an_instance_as_the_rename_form_does() {
    fn fixture() -> (RSpiceApp, rspice_app_state::product::SimulationPlanId) {
        let mut app = RSpiceApp::test_instance();
        let document = app.state.model.schematic.document_mut_for_test();
        document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
        document.wires.clear();
        document.probes = vec![
            rspice_app_state::state::SchematicProbe::new(
                30,
                Point::new(100, 40),
                "I(R1)",
                Some("I(R1)".to_owned()),
            )
            .expect("a current probe"),
        ];
        app.state.model.schematic.init_undo_history();
        let plan_id = rspice_app_state::product::SimulationPlanId::new();
        app.state
            .model
            .workspace
            .content
            .simulation_plan_payloads
            .push(rspice_app_state::state::SimulationPlanPayloadRecord {
                plan_id,
                payload: rspice_app_state::state::SimulationPlanPayload {
                    saved_outputs: vec![
                        rspice_app_state::state::SavedOutput::new(
                            rspice_app_state::state::SavedOutputKind::RawVoltageOrCurrent,
                            "load current",
                            "I(R1)",
                            rspice_app_state::state::SavedOutputCompatibility::OpTranAc,
                            rspice_app_state::state::SavedOutputPolicy::EveryAcceptedPoint,
                            rspice_app_state::state::SavedOutputPrecision::FullSourcePrecision,
                            rspice_app_state::state::SavedOutputStreaming::StoreOnly,
                        )
                        .expect("a raw current is a saved output"),
                    ],
                    ..Default::default()
                },
            });
        (app, plan_id)
    }
    fn outcome(
        app: &RSpiceApp,
        plan_id: rspice_app_state::product::SimulationPlanId,
    ) -> (Vec<String>, Vec<Option<String>>, String) {
        let document = app.state.model.schematic.document();
        (
            document
                .components
                .iter()
                .map(|component| component.name.clone())
                .collect(),
            document
                .probes
                .iter()
                .map(|probe| probe.source_expression.clone())
                .collect(),
            app.state
                .model
                .workspace
                .content
                .plan_data(plan_id)
                .expect("the plan this test owns")
                .saved_outputs[0]
                .source_expression
                .clone(),
        )
    }

    let (mut by_form, form_plan) = fixture();
    let expected = by_form.state.model.schematic.document().components[0].clone();
    assert_eq!(
        by_form
            .state
            .model
            .rename_component_transaction(&expected, "R10".to_owned()),
        Ok(true)
    );
    let (mut by_replace, replace_plan) = fixture();
    ask(&mut by_replace.state, "R1", "R10", FindScope::CellView);
    assert_eq!(replace_all(&mut by_replace.state), Ok((1, 1)));

    let renamed = (
        vec!["R10".to_owned(), "R2".to_owned()],
        vec![Some("I(R10)".to_owned())],
        "I(R10)".to_owned(),
    );
    assert_eq!(outcome(&by_form, form_plan), renamed, "the Rename form");
    assert_eq!(outcome(&by_replace, replace_plan), renamed, "Replace all");
    by_replace.execute_command_key(Command::Undo.key());
    assert_eq!(
        outcome(&by_replace, replace_plan),
        (
            vec!["R1".to_owned(), "R2".to_owned()],
            vec![Some("I(R1)".to_owned())],
            "I(R1)".to_owned(),
        ),
        "one Undo takes the rename and its references back"
    );
}

/// The matches, over what the form reads.
fn find_replace_matches(state: &AppState) -> Result<Vec<DocumentMatches>, String> {
    rspice_app_design::workbench::app::dialogs::find_replace::find_replace_matches(
        &state.design.find_replace,
        &state.model,
    )
}

/// Replace all, over the parts it reads and edits.
pub(crate) fn replace_all(state: &mut AppState) -> Result<(usize, usize), String> {
    let edit_in_progress = state.schematic_edit_in_progress();
    rspice_app_design::workbench::app::dialogs::find_replace::replace_all(
        &state.design.find_replace,
        edit_in_progress,
        &mut state.model,
        &mut state.workbench,
    )
}

/// A form nobody has typed into says what Replace all waits for, as a
/// fact: it refused nothing, so it carries no failure mark. A pattern
/// that does not parse is still a refusal.
#[test]
fn a_pristine_form_states_a_reason_not_a_refusal() {
    let mut state = AppState::default();
    state.design.find_replace.open = true;
    let found = find_replace_matches(&state);
    assert!(found.is_err(), "the query itself refuses an empty find");
    assert_eq!(
        find_replace_summary("", &found, 0),
        Ok(Some("Enter the text to find.".to_owned()))
    );
    state.design.find_replace.options.regex = true;
    ask(&mut state, "R(", "", FindScope::CellView);
    let found = find_replace_matches(&state);
    assert!(find_replace_summary("R(", &found, 0).is_err());
}

pub(crate) fn ask(state: &mut AppState, find: &str, replace: &str, scope: FindScope) {
    let form = &mut state.design.find_replace;
    form.find = find.to_owned();
    form.replace = replace.to_owned();
    form.scope = match scope {
        FindScope::Sheet => 0,
        FindScope::CellView => 1,
        FindScope::Hierarchy => 2,
    };
}

fn values(state: &AppState) -> Vec<String> {
    state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|component| component.value.clone())
        .collect()
}

/// Replace all changes every match in the cell view as one undo step
/// (audit G-7).
#[test]
fn replace_all_in_the_cell_view_is_one_undo_step() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "10k"), part(2, "R2", "10k")];
    document.wires.clear();
    state.model.schematic.init_undo_history();
    ask(&mut state, "10k", "22k", FindScope::CellView);

    assert_eq!(
        find_replace_matches(&state)
            .expect("query")
            .iter()
            .map(|document| document.matches.len())
            .sum::<usize>(),
        2
    );
    assert_eq!(replace_all(&mut state), Ok((2, 1)));
    assert_eq!(values(&state), ["22k", "22k"]);
    assert!(state.model.schematic.undo());
    assert_eq!(values(&state), ["10k", "10k"]);
    assert!(!state.model.schematic.can_undo());
}

/// The Sheet scope leaves another sheet of the same cell view alone.
#[test]
fn the_sheet_scope_leaves_the_other_sheets_alone() {
    let mut state = super::two_sheet_state(&[1], &[2]);
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
    document.wires.clear();
    state.model.schematic.init_undo_history();
    ask(&mut state, "1k", "2k", FindScope::Sheet);
    assert_eq!(replace_all(&mut state), Ok((1, 1)));
    assert_eq!(values(&state), ["2k", "1k"]);
    ask(&mut state, "1k", "2k", FindScope::CellView);
    assert_eq!(replace_all(&mut state), Ok((1, 1)));
    assert_eq!(values(&state), ["2k", "2k"]);
}

/// A replacement that would give two instances one name changes nothing.
#[test]
fn a_clash_anywhere_refuses_the_whole_replacement() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
    state.model.schematic.init_undo_history();
    let options = &mut state.design.find_replace.options;
    options.regex = true;
    ask(&mut state, r"R\d", "RA", FindScope::CellView);
    assert_eq!(
        replace_all(&mut state),
        Err("Two instances would both be named RA.".to_owned())
    );
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
    assert!(!state.model.schematic.can_undo());
}

/// Hierarchy below reaches the master an instance places, and the
/// master's change is its own undo step there.
#[test]
fn the_hierarchy_scope_reaches_the_cell_views_placed_below() {
    let mut state = AppState::default();
    let top = state.model.workspace.content.active_schematic_reference();
    let child = CellViewRef::new(&top.library, "amp", "schematic");
    let mut master = SchematicState::default();
    master.document_mut_for_test().net_labels = vec![NetLabel::new(5, Point::origin(), "vin")];
    master.init_undo_history();
    state
        .model
        .workspace
        .insert_schematic_editor(child.key(), master);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    state.model.schematic.document_mut_for_test().wires.clear();
    state.model.schematic.add_library_cell_component(
        Point::origin(),
        LibraryCellInstance::new(&top.library, "amp", "schematic"),
    );
    state.model.schematic.document_mut_for_test().net_labels =
        vec![NetLabel::new(9, Point::new(0, 50), "vin")];
    state.model.schematic.init_undo_history();

    ask(&mut state, "vin", "vin_a", FindScope::CellView);
    assert_eq!(find_replace_matches(&state).expect("query").len(), 1);
    ask(&mut state, "vin", "vin_a", FindScope::Hierarchy);
    let found = find_replace_matches(&state).expect("query");
    assert_eq!(
        found
            .iter()
            .map(|document| document.path.clone())
            .collect::<Vec<_>>(),
        [top.display_path(), child.display_path()]
    );
    assert_eq!(replace_all(&mut state), Ok((2, 2)));

    let mut stored = state
        .model
        .workspace
        .schematic_editor_mut(&child.key())
        .expect("child");
    assert_eq!(stored.editor.document().net_labels[0].name, "vin_a");
    assert!(stored.editor.undo(), "the master's change is its own step");
    assert_eq!(stored.editor.document().net_labels[0].name, "vin");
    drop(stored);
    assert_eq!(state.model.schematic.document().net_labels[0].name, "vin_a");
}
