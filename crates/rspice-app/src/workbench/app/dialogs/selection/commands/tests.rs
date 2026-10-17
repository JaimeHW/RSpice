//! What the four immediate edit commands do, and what they say afterwards.
//!
//! The assertions are the document after the command, the number of undo
//! entries it cost, and the status-bar prompt — because with no dialog in
//! front of them the prompt is the whole report. None of it is logged: an
//! edit is answered, and undo is its record.

use super::*;

use crate::state::{Bus, BusDeclaration, Component, ComponentType, NetLabel, SchematicProbe, Wire};

fn last_prompt(state: &AppState) -> String {
    state
        .workbench
        .prompt
        .as_ref()
        .expect("the command answers on the status bar")
        .text
        .clone()
}

fn probe(id: u64, position: Point, reference: &str, expression: Option<&str>) -> SchematicProbe {
    SchematicProbe::new(id, position, reference, expression.map(ToOwned::to_owned)).expect("probe")
}

fn state_with_two_resistors() -> AppState {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    state.schematic.document_mut_for_test().wires.clear();
    let mut first = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    first.name = "R1".to_owned();
    let mut second = Component::new(42, ComponentType::Resistor, Point::new(80, 30));
    second.name = "R2".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(first);
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(second);
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
}

#[test]
fn delete_removes_the_selection_immediately_as_one_undo_entry() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);

    assert!(state.delete_schematic_selection());

    assert_eq!(state.schematic.document().components.len(), 1);
    assert_eq!(state.schematic.undo_description(), Some("delete selection"));
    // The current topology resolver includes the two unlabelled terminal nets.
    assert_eq!(
        last_prompt(&state),
        "Deleted 1 object. Nets affected: net1, net2."
    );
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().components.len(), 2);
    assert!(!state.schematic.can_undo());
}

#[test]
fn delete_promotes_a_wire_handle_to_its_whole_conductor() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    let wire = Wire::new(
        17,
        vec![Point::new(0, 0), Point::new(20, 0), Point::new(20, 20)],
    );
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(wire.clone());
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_wire_segment(wire.id, 1);

    assert!(state.delete_schematic_selection());

    assert!(state.schematic.document().wires.is_empty());
    assert_eq!(
        last_prompt(&state),
        "Deleted 1 object. Nets affected: net1."
    );
    assert_eq!(state.schematic.undo_description(), Some("delete selection"));
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().wires, vec![wire]);
}

#[test]
fn a_stale_wire_handle_deletes_nothing_and_says_so() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(17, vec![Point::new(0, 0), Point::new(20, 0)]));
    state.sync_active_schematic_to_workspace();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_wire_vertex(17, 3);

    assert!(!state.delete_schematic_selection());

    assert_eq!(state.schematic.document().wires.len(), 1);
    assert_eq!(last_prompt(&state), "Select something first.");
}

#[test]
fn delete_names_the_nets_it_took_away() {
    let mut state = state_with_two_resistors();
    let bus = Bus::segment(
        5,
        Point::new(0, 60),
        Point::new(40, 60),
        Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
    )
    .unwrap();
    state.schematic.document_mut_for_test().buses.push(bus);
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state.schematic.session.editor.selection.select_only_bus(5);

    assert!(state.delete_schematic_selection());

    assert_eq!(
        last_prompt(&state),
        "Deleted 1 object. Nets affected: DATA[3:0]."
    );
}

#[test]
fn delete_counts_the_records_that_still_reference_what_went() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(70, Point::new(20, 30), "vout"));
    state.schematic.document_mut_for_test().probes.push(probe(
        9,
        Point::new(200, 200),
        "P1",
        Some("v(vout)"),
    ));
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(70);

    assert!(state.delete_schematic_selection());

    let reported = last_prompt(&state);
    assert!(reported.starts_with("Deleted 1 object."), "{reported}");
    assert!(
        reported.ends_with("Still referenced by 1 probe."),
        "{reported}"
    );
}

#[test]
fn the_impact_sentence_is_plural_correct_and_omits_empty_classes() {
    assert_eq!(DeleteDependencyImpact::default().detail(), None);
    assert_eq!(
        DeleteDependencyImpact {
            nets: vec!["vout".to_owned(), "n7".to_owned()],
            saved_outputs: 2,
            probes: 1,
            ..DeleteDependencyImpact::default()
        }
        .detail()
        .as_deref(),
        Some("Nets affected: vout, n7. Still referenced by 2 saved outputs and 1 probe.")
    );
    assert_eq!(
        DeleteDependencyImpact {
            saved_outputs: 1,
            probes: 2,
            specifications: 1,
            ..DeleteDependencyImpact::default()
        }
        .detail()
        .as_deref(),
        Some("Still referenced by 1 saved output, 2 probes and 1 specification.")
    );
    assert_eq!(
        name_list(&(1..=7).map(|n| format!("n{n}")).collect::<Vec<_>>()),
        "n1, n2, n3, n4, n5 and 2 more"
    );
}

#[test]
fn cut_copies_the_selection_and_then_removes_it() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);

    assert!(state.cut_schematic_selection());

    assert_eq!(state.schematic.document().components.len(), 1);
    assert_eq!(state.schematic.session.editor.clipboard.components.len(), 1);
    assert_eq!(
        state.schematic.session.editor.clipboard.components[0].name,
        "R1"
    );
    assert_eq!(last_prompt(&state), "Cut 1 object.");
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().components.len(), 2);
}

/// Duplicate landed everything two world units off its anchor, off every
/// grid pitch, and the copy it carried was measured from the selection's
/// unsnapped middle, so even a whole-grid offset left parts between grid
/// points (audit E-6).
#[test]
fn a_duplicate_lands_on_the_grid_one_step_off_its_anchor() {
    let mut state = state_with_two_resistors();
    let capacitor = Component::new(43, ComponentType::Capacitor, Point::new(40, 70));
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(capacitor);
    let selection = &mut state.schematic.session.editor.selection;
    for id in [41, 42, 43] {
        selection.select_component(id);
    }
    let grid = state.schematic.document().grid_size;
    let anchor = state.schematic_paste_anchor();

    assert!(state.duplicate_schematic_selection());

    let copies: Vec<Point> = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| ![41, 42, 43].contains(&component.id))
        .map(|component| component.pos)
        .collect();
    assert_eq!(copies.len(), 3);
    assert!(
        copies
            .iter()
            .all(|pos| pos.x % grid == 0 && pos.y % grid == 0),
        "a duplicate left the grid of {grid}: {copies:?}"
    );
    // The copy's grid-snapped middle (50, 40) lands one grid step past the
    // anchor, so the first resistor at (20, 30) lands 30 left and 10 up of it.
    assert!(copies.contains(&Point::new(anchor.x + grid - 30, anchor.y + grid - 10)));
}

#[test]
fn the_prompt_clauses_are_plural_correct() {
    assert_eq!(open_nets_clause(1), "1 net is now open");
    assert_eq!(open_nets_clause(3), "3 nets are now open");
    assert_eq!(named_net_connections(1), "1 named-net connection");
    assert_eq!(named_net_connections(2), "2 named-net connections");
    assert_eq!(object_count(1), "1 object");
    assert_eq!(object_count(4), "4 objects");
}

#[test]
fn duplicate_leaves_the_clipboard_byte_identical() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(state.copy_active_schematic_selection());
    let before =
        serde_json::to_string(&state.schematic.session.editor.clipboard).expect("clipboard");

    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(42);
    assert!(state.duplicate_schematic_selection_at(Point::new(140, 30)));

    assert_eq!(state.schematic.document().components.len(), 3);
    assert_eq!(
        serde_json::to_string(&state.schematic.session.editor.clipboard).expect("clipboard"),
        before,
        "Duplicate borrows the clipboard and must hand it back untouched"
    );
    assert_eq!(last_prompt(&state), "Duplicated 1 object.");
}

#[test]
fn duplicate_selects_what_it_made_and_costs_one_undo_entry() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);

    assert!(state.duplicate_schematic_selection_at(Point::new(140, 30)));

    let created = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id != 41 && component.id != 42)
        .expect("the duplicate exists");
    assert!(
        state
            .schematic
            .session
            .editor
            .selection
            .has_component(created.id)
    );
    assert!(!state.schematic.session.editor.selection.has_component(41));
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().components.len(), 2);
    assert!(!state.schematic.can_undo());
}

#[test]
fn duplicate_honours_the_persisted_external_net_preference() {
    for preference in [
        DuplicateExternalNets::LeaveUnconnected,
        DuplicateExternalNets::PreserveNamedNetAttachment,
    ] {
        let mut state = state_with_two_resistors();
        state.ui.duplicate_external_nets = preference;
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(70, Point::new(20, 30), "vout"));
        state.sync_active_schematic_to_workspace();
        state.schematic.init_undo_history();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(41);

        assert!(
            state.duplicate_schematic_selection_at(Point::new(140, 30)),
            "{preference:?} still duplicates"
        );
        assert_eq!(
            state.schematic.document().components.len(),
            3,
            "{preference:?}"
        );
        assert!(
            last_prompt(&state).starts_with("Duplicated 1 object"),
            "{preference:?}: {}",
            last_prompt(&state)
        );
    }
}

#[test]
fn select_all_takes_every_class_the_filter_admits_including_annotations() {
    let mut base = AppState::default();
    base.schematic.document_mut_for_test().components.clear();
    base.schematic.document_mut_for_test().wires.clear();
    base.schematic
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    base.schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(20, 0)));
    base.schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(3, Point::new(20, 0), "vout"));
    base.schematic
        .document_mut_for_test()
        .probes
        .push(probe(4, Point::new(20, 0), "P1", None));
    base.sync_active_schematic_to_workspace();

    let mut state = base.clone();
    assert!(state.select_all_schematic_objects());
    assert!(state.schematic.session.editor.selection.has_component(1));
    assert!(state.schematic.session.editor.selection.has_wire(2));
    assert!(state.schematic.session.editor.selection.has_net_label(3));
    assert!(
        state.schematic.session.editor.selection.has_probe(4),
        "probes are annotations and Select all takes them"
    );
    assert_eq!(last_prompt(&state), "Selected 4 objects.");

    for (class, present) in [
        ("instances", 1_u64),
        ("wires", 2),
        ("labels", 3),
        ("annotations", 4),
    ] {
        let mut state = base.clone();
        state.ui.schematic_selection_filter = crate::state::SchematicSelectionFilter {
            instances: class == "instances",
            wires: class == "wires",
            labels: class == "labels",
            annotations: class == "annotations",
        };

        assert!(state.select_all_schematic_objects(), "{class}");
        assert_eq!(
            state.schematic.session.editor.selection.count(),
            1,
            "{class}"
        );
        assert_eq!(last_prompt(&state), "Selected 1 object.", "{class}");
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_component(present)
                || state.schematic.session.editor.selection.has_wire(present)
                || state
                    .schematic
                    .session
                    .editor
                    .selection
                    .has_net_label(present)
                || state.schematic.session.editor.selection.has_probe(present),
            "{class}"
        );
    }
}

#[test]
fn select_all_says_when_the_filter_admits_nothing() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    state.schematic.document_mut_for_test().wires.clear();
    state.sync_active_schematic_to_workspace();

    assert!(!state.select_all_schematic_objects());
    assert_eq!(last_prompt(&state), "Nothing matches the selection filter.");
}

#[test]
fn a_read_only_schematic_refuses_every_mutating_command_without_touching_it() {
    let mut state = state_with_two_resistors();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    state.schematic.session.read_only = true;
    state.sync_active_schematic_to_workspace();

    // Every place these are offered greys them on a read-only schematic, so
    // reaching one anyway is answered by nothing at all.
    let logged = state.log_buffer.len();
    for command in [
        AppState::delete_schematic_selection as fn(&mut AppState) -> bool,
        AppState::cut_schematic_selection,
        AppState::duplicate_schematic_selection,
    ] {
        assert!(!command(&mut state));
        assert!(state.workbench.prompt.is_none());
        assert_eq!(state.log_buffer.len(), logged);
    }
    assert_eq!(state.schematic.document().components.len(), 2);
    assert!(state.schematic.session.editor.clipboard.is_empty());
    assert!(
        state.select_all_schematic_objects(),
        "reading a read-only schematic is not an edit"
    );
}

#[test]
fn an_empty_selection_refuses_with_one_plain_line() {
    let mut state = state_with_two_resistors();

    for command in [
        AppState::delete_schematic_selection as fn(&mut AppState) -> bool,
        AppState::cut_schematic_selection,
        AppState::duplicate_schematic_selection,
    ] {
        assert!(!command(&mut state));
        assert_eq!(last_prompt(&state), "Select something first.");
    }
    assert_eq!(state.schematic.document().components.len(), 2);
}

/// The copy these commands ship is the prompt, so the words the review
/// surface used must not come back through it.
#[test]
fn no_shipped_copy_here_speaks_the_old_review_vocabulary() {
    const BANNED: [&str; 4] = ["transaction", "stable id", "undo record", "mockup"];

    let production = crate::source_guard::production_source(include_str!("../commands.rs"));
    let mut found = Vec::new();
    for (number, line) in production.lines().enumerate() {
        let lowered = line.to_ascii_lowercase();
        for banned in BANNED {
            if lowered.contains(banned) {
                found.push(format!("commands.rs:{}: {banned}", number + 1));
            }
        }
    }

    assert!(
        found.is_empty(),
        "these lines speak the vocabulary the review surface was deleted for:\n  {}",
        found.join("\n  ")
    );
}
