//! Find and its Replace row, drawn and applied through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::find_replace::tests::part;
use rspice_app_design::workbench::app::dialogs::find_replace::*;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::state::Point;
use rspice_app_state::state::{LibraryCellInstance, NetLabel, SchematicState};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// An instance renamed by Replace ends as the Rename form leaves it: the probe and the saved output that name it follow it, and one
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
    ask(&mut by_replace.state, "R1", "R10");
    assert_eq!(replace_all(&mut by_replace.state), Ok((1, 1)));

    let renamed = (
        vec!["R10".to_owned(), "R2".to_owned()],
        vec![Some("I(R10)".to_owned())],
        "I(R10)".to_owned(),
    );
    assert_eq!(outcome(&by_form, form_plan), renamed, "the Rename form");
    assert_eq!(outcome(&by_replace, replace_plan), renamed, "Replace");
    by_replace.execute_command_key(ids::UNDO);
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

/// The text Find searches for: the Navigator's search box.
fn find_text(state: &AppState) -> String {
    state
        .workbench
        .navigator_trees
        .filter(Workspace::Design)
        .to_owned()
}

/// The hits, over what Find reads.
fn hits_of(state: &AppState) -> Result<Vec<DocumentMatches>, String> {
    find_hits(&find_text(state), &state.design.find_replace, &state.model)
}

/// Replace the ticked hits, over the parts it reads and edits.
pub(crate) fn replace_all(state: &mut AppState) -> Result<(usize, usize), String> {
    let edit_in_progress = state.schematic_edit_in_progress();
    let find = find_text(state);
    replace_ticked(
        &mut state.design.find_replace,
        &find,
        edit_in_progress,
        &mut state.model,
        &mut state.workbench,
    )
}

/// Type `find` in the search box and `replace` in the Replace row.
pub(crate) fn ask(state: &mut AppState, find: &str, replace: &str) {
    *state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Design) = find.to_owned();
    state.design.find_replace.replacing = true;
    state.design.find_replace.replace = replace.to_owned();
}

/// Replace states why it would be refused while the hits are listed, so
/// the press is never the first the reader hears of it: a pattern that
/// does not parse, a clash, an edit under way.
#[test]
fn replace_is_refused_beside_its_button_before_it_is_pressed() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
    state.model.schematic.init_undo_history();
    state.design.find_replace.options.regex = true;
    let refusal = |state: &AppState, edit_in_progress| {
        replace_refusal(
            &state.design.find_replace,
            &hits_of(state),
            edit_in_progress,
            &state.model,
        )
    };

    ask(&mut state, "R(", "");
    assert!(
        refusal(&state, false).is_some_and(|reason| reason.contains("regular expression")),
        "a pattern that does not parse"
    );
    ask(&mut state, r"R\d", "RA");
    assert_eq!(
        refusal(&state, false).as_deref(),
        Some("Two instances would both be named RA.")
    );
    ask(&mut state, r"\d+k", "2k");
    assert_eq!(refusal(&state, false), None);
    assert_eq!(
        refusal(&state, true).as_deref(),
        Some("Finish or cancel the edit in progress first.")
    );
}

/// Replace changes the hits that are ticked and leaves the unticked ones,
/// which is how every 10k becomes 12k but one.
#[test]
fn replace_acts_on_the_ticked_hits() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "10k"), part(2, "R2", "10k")];
    document.wires.clear();
    state.model.schematic.init_undo_history();
    ask(&mut state, "10k", "12k");
    let found = hits_of(&state).expect("query");
    let second = &found[0];
    let key = second.hit_key(&second.matches[1]);
    state.design.find_replace.toggle_tick(key);

    assert_eq!(replace_all(&mut state), Ok((1, 1)));
    assert_eq!(values(&state), ["12k", "10k"]);
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

/// Replace changes every ticked hit in the cell view as one undo step.
#[test]
fn replace_all_in_the_cell_view_is_one_undo_step() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "10k"), part(2, "R2", "10k")];
    document.wires.clear();
    state.model.schematic.init_undo_history();
    ask(&mut state, "10k", "22k");

    assert_eq!(
        hits_of(&state)
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

/// A replacement that would give two instances one name changes nothing.
#[test]
fn a_clash_anywhere_refuses_the_whole_replacement() {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
    state.model.schematic.init_undo_history();
    state.design.find_replace.options.regex = true;
    ask(&mut state, r"R\d", "RA");
    assert_eq!(
        replace_all(&mut state),
        Err("Two instances would both be named RA.".to_owned())
    );
    assert_eq!(state.model.schematic.document().components[0].name, "R1");
    assert!(!state.model.schematic.can_undo());
}

/// Find reaches the master an instance places, and a Replace that changes
/// both cell views is one undo step: one Undo takes back both.
#[test]
fn replace_across_the_hierarchy_is_one_undo_step() {
    let mut app = RSpiceApp::test_instance();
    let state = &mut app.state;
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
    state.sync_active_schematic_to_workspace();

    ask(state, "vin", "vin_a");
    let found = hits_of(state).expect("query");
    assert_eq!(
        found
            .iter()
            .map(|document| document.path.clone())
            .collect::<Vec<_>>(),
        [top.display_path(), child.display_path()]
    );
    assert_eq!(replace_all(state), Ok((2, 2)));
    let child_label = |app: &RSpiceApp| {
        app.state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&child.key())
            .expect("child")
            .document()
            .net_labels[0]
            .name
            .clone()
    };
    assert_eq!(child_label(&app), "vin_a");
    assert_eq!(
        app.state.model.schematic.document().net_labels[0].name,
        "vin_a"
    );

    app.execute_command_key(ids::UNDO);

    assert_eq!(
        child_label(&app),
        "vin",
        "the master's change is undone too"
    );
    assert_eq!(
        app.state.model.schematic.document().net_labels[0].name,
        "vin"
    );
}

/// Find works where nothing can change: a hit in a read-only view is
/// listed and shown, and Replace says why it passes it by.
#[test]
fn find_lists_hits_in_a_read_only_view_and_replace_says_why_not() {
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().components = vec![part(1, "R1", "10k")];
    state.model.schematic.session.read_only = true;
    ask(&mut state, "10k", "12k");

    let found = hits_of(&state).expect("query");
    assert_eq!(found.len(), 1);
    assert!(found[0].read_only);
    assert_eq!(
        replace_refusal(&state.design.find_replace, &Ok(found), false, &state.model).as_deref(),
        Some("Every hit is in a read-only cell view.")
    );
}

/// Ctrl+F, then a name, then Enter: the part on another sheet is shown,
/// its sheet brought forward, the part selected and centred.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ctrl_f_then_a_name_then_enter_centres_the_part_on_another_sheet() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![part(1, "R1", "1k"), part(17, "R17", "10k")];
    rspice_app_design::schematic::view::sheet_visibility::seed_two_sheets(
        &mut app.state.model,
        &[1],
        &[17],
    );
    app.state.workbench.activate(Workspace::Design);
    app.execute_command_key(ids::FIND_IN_DESIGN);
    let mut panel = super::navigator::NavigatorPanel::open(app);

    panel.pass(vec![egui::Event::Text("R17".to_owned())]);
    panel.press(egui::Key::Enter);

    let model = &mut panel.app.state.model;
    assert_eq!(
        model.sheet_name(model.active_sheet_id().expect("a sheet is active")),
        "Sheet 2"
    );
    model.retain_selection_on_active_sheet();
    assert!(model.schematic.session.editor.selection.has_component(17));
    assert_eq!(
        model.schematic.session.editor.center_request,
        Some(Point::new(1700, 0))
    );
}

/// Ctrl+F opens Find with the Replace row folded, even after Find and
/// replace unfolded it.
#[test]
fn find_folds_the_replace_row_that_find_and_replace_unfolded() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);

    app.execute_command_key(
        rspice_app_design::workbench::design::commands::DesignCommand::FindReplace.key(),
    );
    assert!(app.state.design.find_replace.replacing);
    app.execute_command_key(ids::FIND_IN_DESIGN);
    assert!(!app.state.design.find_replace.replacing);
}

/// F3 shows the next hit of what the Navigator's search box finds and
/// Shift+F3 the one before, wrapping at the ends, from anywhere in the
/// design workspace rather than only with the caret in the box. With the
/// box empty both say what to type.
#[test]
fn find_next_and_previous_step_through_the_hits_of_the_search_box() {
    use rspice_app_design::workbench::design::commands::DesignCommand;

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.schematic.document_mut_for_test().components = vec![
        part(1, "R1", "10k"),
        part(2, "R2", "1k"),
        part(3, "R3", "10k"),
    ];
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);
    assert!(!app.command_key_enabled(DesignCommand::FindNext.key()));
    assert!(!app.command_key_enabled(DesignCommand::FindPrevious.key()));

    app.state.workbench.set_navigator_filter("10k");
    let selected = |app: &RSpiceApp| {
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component()
    };
    app.execute_command_key(DesignCommand::FindNext.key());
    assert_eq!(selected(&app), Some(1), "F3 shows the first hit");
    app.execute_command_key(DesignCommand::FindNext.key());
    assert_eq!(selected(&app), Some(3), "and then the next, past R2's 1k");
    app.execute_command_key(DesignCommand::FindNext.key());
    assert_eq!(selected(&app), Some(1), "wrapping at the end");
    app.execute_command_key(DesignCommand::FindPrevious.key());
    assert_eq!(selected(&app), Some(3), "Shift+F3 steps back");
    assert_eq!(
        app.state.model.schematic.session.editor.center_request,
        Some(Point::new(300, 0)),
        "and centres the hit it shows"
    );
}
