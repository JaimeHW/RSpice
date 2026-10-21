//! Probe markers retained on the sheet: the expression and plan output each
//! one binds, the identity a repeated placement reuses, and the edit
//! authority that refuses one without mutating the document.

use super::*;

#[test]
fn empty_space_probe_retains_one_unbound_marker_and_undo_removes_it() {
    let mut state = AppState::default();
    let position = Point::new(30, 40);

    let id =
        retain_probe_flag(&mut state, position, None, None).expect("editable active schematic");
    assert_eq!(state.schematic.document().probes.len(), 1);
    assert_eq!(state.schematic.document().probes[0].id, id);
    assert_eq!(state.schematic.document().probes[0].position, position);
    assert_eq!(
        state.schematic.document().probes[0].reference,
        format!("P{id}")
    );
    assert!(
        state.schematic.document().probes[0]
            .source_expression
            .is_none()
    );
    assert_eq!(
        state.schematic.undo_description(),
        Some("place schematic probe")
    );

    assert!(state.schematic.undo());
    assert!(state.schematic.document().probes.is_empty());
}

#[test]
fn probe_marker_rejects_read_only_and_replaced_view_identity_without_mutation() {
    let mut read_only = AppState::default();
    read_only.schematic.session.read_only = true;
    assert!(retain_probe_flag(&mut read_only, Point::origin(), None, None).is_err());
    assert!(read_only.schematic.document().probes.is_empty());
    assert!(!read_only.schematic.can_undo());

    let mut read_only_reference = AppState::default();
    read_only_reference
        .workspace
        .content
        .set_active_read_only_reference(true);
    assert!(retain_probe_flag(&mut read_only_reference, Point::origin(), None, None).is_err());
    assert!(read_only_reference.schematic.document().probes.is_empty());
    assert!(!read_only_reference.schematic.can_undo());

    let mut replaced = AppState::default();
    replaced.workspace.content.active_view.view = "symbol".to_owned();
    assert!(retain_probe_flag(&mut replaced, Point::origin(), None, None).is_err());
    assert!(replaced.schematic.document().probes.is_empty());
    assert!(!replaced.schematic.can_undo());
}

#[test]
fn bound_probe_marker_retains_the_exact_source_expression() {
    let mut state = AppState::default();
    retain_probe_flag(
        &mut state,
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("bound marker");

    assert_eq!(state.schematic.document().probes[0].reference, "V(OUT)");
    assert_eq!(
        state.schematic.document().probes[0]
            .source_expression
            .as_deref(),
        Some("V(OUT)")
    );
}

#[test]
fn probe_placed_while_descended_names_the_occurrence() {
    let mut state = AppState::default();
    state.workspace.descend_into(
        "X1".to_owned(),
        crate::state::CellViewRef::new("user", "amp", "schematic"),
        ViewType::Schematic,
    );

    let spelling = probe_spelling_for(&state, "n1", "V(n1)").expect("an ASCII occurrence spells");

    assert_eq!(spelling.display(), "V(/X1/n1)");
    assert_eq!(spelling.engine(), "V(x1.n1)");
    assert_eq!(
        probe_spelling_for(&state, "V(x1.n1)", "V(x1.n1)")
            .expect("an exact expression is not re-scoped")
            .display(),
        "V(x1.n1)",
        "an expression that already names a node must be used as written"
    );
}

#[test]
fn an_occurrence_probe_saves_the_display_name_and_requests_the_engine_node() {
    let mut state = AppState::default();
    let spelling = OccurrenceProbeSpelling::for_leaf(
        &crate::state::InstancePath::parse("/X1").expect("one descent"),
        'V',
        "n1",
    )
    .expect("an ASCII occurrence spells");

    assert!(matches!(
        request_probe_signal(&mut state, &spelling),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));

    let outputs = saved_outputs(&state);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name, "V(/X1/n1)");
    assert_eq!(
        outputs[0].source_expression, "V(x1.n1)",
        "the plan must request the node the engine solved, not the one drawn"
    );

    retain_probe_flag(&mut state, Point::new(10, 20), Some(&spelling), None)
        .expect("marker retains");
    assert_eq!(state.schematic.document().probes[0].reference, "V(/X1/n1)");
    assert_eq!(
        state.schematic.document().probes[0]
            .source_expression
            .as_deref(),
        Some("V(x1.n1)")
    );
}

#[test]
fn bound_probe_marker_retains_stable_plan_output_identity() {
    let mut state = AppState::default();
    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    let binding = current_probe_output_binding(&state, "V(OUT)").expect("output binding");

    retain_probe_flag(
        &mut state,
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        Some(binding),
    )
    .expect("bound marker");

    assert_eq!(
        state.schematic.document().probes[0].plan_id,
        Some(binding.0)
    );
    assert_eq!(
        state.schematic.document().probes[0].saved_output_id,
        Some(binding.1)
    );
    assert!(state.schematic.document().probes[0].enabled);
    assert!(state.schematic.document().probes[0].plot_on_materialization);
}

#[test]
fn equivalent_probe_marker_placement_reuses_identity_without_an_undo_step() {
    let mut state = AppState::default();
    let position = Point::new(10, 20);
    let id = retain_probe_flag(
        &mut state,
        position,
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("first marker");
    state.schematic.clear_undo_history();

    let repeated = retain_probe_flag(
        &mut state,
        position,
        Some(&OccurrenceProbeSpelling::verbatim(" v ( out ) ")),
        None,
    )
    .expect("existing marker");

    assert_eq!(repeated, id);
    assert_eq!(state.schematic.document().probes.len(), 1);
    assert_eq!(
        state.schematic.session.editor.selection.single_probe(),
        Some(id)
    );
    assert!(!state.schematic.can_undo());
}

#[test]
fn late_safe_mode_activation_rejects_probe_marker_without_mutation() {
    let mut state = AppState::default();
    state.workbench.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        String::new(),
    );

    assert!(retain_probe_flag(&mut state, Point::new(10, 20), None, None).is_err());
    assert!(state.schematic.document().probes.is_empty());
    assert!(!state.schematic.session.is_dirty);
    assert!(!state.schematic.can_undo());
}
