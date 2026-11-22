//! Probe markers retained on the sheet: the expression and plan output each
//! one binds, the identity a repeated placement reuses, and the edit
//! authority that refuses one without mutating the document.

use super::*;
use crate::workbench::design::test_host::{DesignTestHost, canvas_parts};

/// A probe-tool click on the bare sheet probes nothing: it leaves no marker
/// and no undo step, and the status line says what to click.
#[test]
fn a_probe_click_on_the_bare_sheet_places_nothing_and_says_what_to_click() {
    let mut state = DesignTestHost::default();
    let symbol_context = schematic_symbol_context(&state);

    handle_probe_click(
        &mut canvas_parts(&mut state),
        Point::new(30, 40),
        &symbol_context,
    );

    assert!(state.model.schematic.document().probes.is_empty());
    assert!(!state.model.schematic.can_undo());
    assert_eq!(
        state.prompt_text(),
        Some("Nothing probed: click a wire, a pin or a part to probe it.")
    );
}

#[test]
fn probe_marker_rejects_read_only_and_replaced_view_identity_without_mutation() {
    let mut read_only = DesignTestHost::default();
    read_only.model.schematic.session.read_only = true;
    assert!(
        retain_probe_flag(
            &mut canvas_parts(&mut read_only),
            Point::origin(),
            None,
            None
        )
        .is_err()
    );
    assert!(read_only.model.schematic.document().probes.is_empty());
    assert!(!read_only.model.schematic.can_undo());

    let mut read_only_reference = DesignTestHost::default();
    read_only_reference
        .model
        .workspace
        .content
        .set_active_read_only_reference(true);
    assert!(
        retain_probe_flag(
            &mut canvas_parts(&mut read_only_reference),
            Point::origin(),
            None,
            None
        )
        .is_err()
    );
    assert!(
        read_only_reference
            .model
            .schematic
            .document()
            .probes
            .is_empty()
    );
    assert!(!read_only_reference.model.schematic.can_undo());

    let mut replaced = DesignTestHost::default();
    replaced.model.workspace.content.active_view.view = "symbol".to_owned();
    assert!(
        retain_probe_flag(
            &mut canvas_parts(&mut replaced),
            Point::origin(),
            None,
            None
        )
        .is_err()
    );
    assert!(replaced.model.schematic.document().probes.is_empty());
    assert!(!replaced.model.schematic.can_undo());
}

#[test]
fn bound_probe_marker_retains_the_exact_source_expression() {
    let mut state = DesignTestHost::default();
    retain_probe_flag(
        &mut canvas_parts(&mut state),
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("bound marker");

    assert_eq!(
        state.model.schematic.document().probes[0].reference,
        "V(OUT)"
    );
    assert_eq!(
        state.model.schematic.document().probes[0]
            .source_expression
            .as_deref(),
        Some("V(OUT)")
    );
}

#[test]
fn probe_placed_while_descended_names_the_occurrence() {
    let mut state = DesignTestHost::default();
    state.model.workspace.descend_into(
        "X1".to_owned(),
        crate::state::CellViewRef::new("user", "amp", "schematic"),
        ViewType::Schematic,
    );

    let spelling =
        probe_spelling_for(&state.model, "n1", "V(n1)").expect("an ASCII occurrence spells");

    assert_eq!(spelling.display(), "V(/X1/n1)");
    assert_eq!(spelling.engine(), "V(x1.n1)");
    assert_eq!(
        probe_spelling_for(&state.model, "V(x1.n1)", "V(x1.n1)")
            .expect("an exact expression is not re-scoped")
            .display(),
        "V(x1.n1)",
        "an expression that already names a node must be used as written"
    );
}

#[test]
fn an_occurrence_probe_saves_the_display_name_and_requests_the_engine_node() {
    let mut state = DesignTestHost::default();
    let spelling = OccurrenceProbeSpelling::for_leaf(
        &crate::state::InstancePath::parse("/X1").expect("one descent"),
        'V',
        "n1",
    )
    .expect("an ASCII occurrence spells");

    assert!(matches!(
        request_probe_signal(&mut canvas_parts(&mut state), &spelling),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));

    let outputs = saved_outputs(&state);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name, "V(/X1/n1)");
    assert_eq!(
        outputs[0].source_expression, "V(x1.n1)",
        "the plan must request the node the engine solved, not the one drawn"
    );

    retain_probe_flag(
        &mut canvas_parts(&mut state),
        Point::new(10, 20),
        Some(&spelling),
        None,
    )
    .expect("marker retains");
    assert_eq!(
        state.model.schematic.document().probes[0].reference,
        "V(/X1/n1)"
    );
    assert_eq!(
        state.model.schematic.document().probes[0]
            .source_expression
            .as_deref(),
        Some("V(x1.n1)")
    );
}

#[test]
fn bound_probe_marker_retains_stable_plan_output_identity() {
    let mut state = DesignTestHost::default();
    assert!(matches!(
        request_probe_signal(
            &mut canvas_parts(&mut state),
            &OccurrenceProbeSpelling::verbatim("V(OUT)")
        ),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    let binding =
        current_probe_output_binding(&canvas_parts(&mut state), "V(OUT)").expect("output binding");

    retain_probe_flag(
        &mut canvas_parts(&mut state),
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        Some(binding),
    )
    .expect("bound marker");

    assert_eq!(
        state.model.schematic.document().probes[0].plan_id,
        Some(binding.0)
    );
    assert_eq!(
        state.model.schematic.document().probes[0].saved_output_id,
        Some(binding.1)
    );
    assert!(state.model.schematic.document().probes[0].enabled);
    assert!(state.model.schematic.document().probes[0].plot_on_materialization);
}

#[test]
fn equivalent_probe_marker_placement_reuses_identity_without_an_undo_step() {
    let mut state = DesignTestHost::default();
    let position = Point::new(10, 20);
    let id = retain_probe_flag(
        &mut canvas_parts(&mut state),
        position,
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("first marker");
    state.model.schematic.clear_undo_history();

    let repeated = retain_probe_flag(
        &mut canvas_parts(&mut state),
        position,
        Some(&OccurrenceProbeSpelling::verbatim(" v ( out ) ")),
        None,
    )
    .expect("existing marker");

    assert_eq!(repeated, id);
    assert_eq!(state.model.schematic.document().probes.len(), 1);
    assert_eq!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_probe(),
        Some(id)
    );
    assert!(!state.model.schematic.can_undo());
}

#[test]
fn late_safe_mode_activation_rejects_probe_marker_without_mutation() {
    let mut state = DesignTestHost::default();
    state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        String::new(),
    );

    assert!(
        retain_probe_flag(
            &mut canvas_parts(&mut state),
            Point::new(10, 20),
            None,
            None
        )
        .is_err()
    );
    assert!(state.model.schematic.document().probes.is_empty());
    assert!(!state.model.schematic.session.is_dirty);
    assert!(!state.model.schematic.can_undo());
}

/// Wire `OUT` with a probe on it, and wire `IN` 100 below it.
fn two_net_host() -> (DesignTestHost, SchematicProbe) {
    let mut state = DesignTestHost::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.wires = vec![
        Wire::new(91, vec![Point::new(0, 20), Point::new(80, 20)]),
        Wire::new(93, vec![Point::new(0, 120), Point::new(80, 120)]),
    ];
    document.net_labels = vec![
        NetLabel::new(92, Point::new(70, 20), "OUT"),
        NetLabel::new(94, Point::new(70, 120), "IN"),
    ];
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    let symbol_context = schematic_symbol_context(&state);
    handle_probe_click(
        &mut canvas_parts(&mut state),
        Point::new(40, 20),
        &symbol_context,
    );
    let original = state.model.schematic.document().probes[0].clone();
    assert_eq!(original.reference, "V(OUT)");
    (state, original)
}

/// Copy what is selected and place the paste `offset` from where it was.
fn paste_selection_moved_by(state: &mut DesignTestHost, offset: Point) -> bool {
    state.design.schematic_clipboard = state.model.schematic.copy_selection();
    let origin = state.design.schematic_clipboard.origin;
    let anchor = Point::new(origin.x + offset.x, origin.y + offset.y);
    assert!(
        crate::workbench::app::paste_schematic_clipboard_at(
            state.design.floating_copy_parts(),
            &mut state.model,
            &mut state.workbench,
            anchor,
        )
        .expect("the copy floats")
    );
    canvas_parts(state).place_floating_paste()
}

/// A pasted probe measures the signal under its new place, not the one the
/// original measures: a note and the probe on `OUT`, pasted 100 below, land
/// a probe on `IN`, bound to that signal, in the paste's one undo step and
/// selected with the note.
#[test]
fn a_pasted_probe_binds_to_the_signal_under_its_new_place() {
    let (mut state, original) = two_net_host();
    let note =
        crate::state::DesignNote::new(77, Point::new(40, 0), DesignNoteKind::ReviewNote, "Output")
            .unwrap();
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(note);
    state.sync_active_schematic_to_workspace();
    state.model.schematic.clear_undo_history();
    let selection = &mut state.model.schematic.session.editor.selection;
    selection.clear();
    selection.select_design_note(77);
    selection.select_probe(original.id);

    assert!(paste_selection_moved_by(&mut state, Point::new(0, 100)));

    let document = state.model.schematic.document();
    assert_eq!(document.design_notes.len(), 2, "the note is pasted");
    assert_eq!(document.probes.len(), 2);
    let copy = document
        .probes
        .iter()
        .find(|probe| probe.id != original.id)
        .expect("the pasted probe")
        .clone();
    assert_eq!(copy.position, Point::new(40, 120));
    assert_eq!(copy.reference, "V(IN)");
    assert_eq!(
        copy.source_expression
            .as_deref()
            .map(normalized_probe_expression),
        Some("v(in)".to_owned())
    );
    let output = saved_outputs(&state)
        .iter()
        .find(|output| normalized_probe_expression(&output.source_expression) == "v(in)")
        .expect("the copy's signal is asked of the run")
        .id;
    assert_eq!(copy.saved_output_id, Some(output));
    let selection = &state.model.schematic.session.editor.selection;
    assert!(
        selection.probes.contains(&copy.id),
        "the copy's probe is selected"
    );
    assert_eq!(selection.design_notes.len(), 1, "with the pasted note");

    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().probes, vec![original]);
    assert_eq!(state.model.schematic.document().design_notes.len(), 1);
    assert!(
        !state.model.schematic.can_undo(),
        "the paste is one undo step"
    );
}

/// A probe pasted where no wire, pin or part is under it is not pasted, and
/// the status line says why.
#[test]
fn a_probe_pasted_on_no_signal_is_left_out() {
    let (mut state, original) = two_net_host();
    state.model.schematic.clear_undo_history();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_probe(original.id);

    assert!(!paste_selection_moved_by(&mut state, Point::new(0, 300)));

    assert_eq!(state.model.schematic.document().probes, vec![original]);
    assert!(!state.model.schematic.can_undo());
    assert_eq!(
        state.prompt_text(),
        Some("Nothing was pasted: a probe goes on a wire, a pin or a part.")
    );
}
