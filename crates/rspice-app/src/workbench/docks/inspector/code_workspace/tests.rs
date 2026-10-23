//! A diagnostic's place is stated one-based, and a deck's provenance speaks
//! only when the deck needs attention.

use super::*;

#[test]
fn netlist_diagnostic_locations_are_one_based_and_exact() {
    let mut diagnostic = crate::state::netlist_diagnostics::Diagnostic::current(
        "rspice.test",
        "TEST-ADVISORY",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
        "Maximum transient step is implicit",
    );
    diagnostic.source_line = Some(127);
    diagnostic.line = Some(127);
    diagnostic.column = Some(8);

    assert_eq!(diagnostic_location(&diagnostic), "line 128 · column 9");
}

/// A deck that matches the schematic says nothing at all: the three states
/// left are the ones a designer has to act on.
#[test]
fn generated_provenance_speaks_only_when_the_deck_is_not_the_schematic() {
    let mut state = AppState::default();
    state.model.netlist_artifacts.generated_source = "generated\n.end\n".to_owned();
    let input = crate::product::ContentDigest::from_bytes([0x31; 32]);
    state.model.netlist_artifacts.generated_input_digest = Some(input);
    state
        .model
        .netlist_artifacts
        .current_generation_input_digest = Some(input);
    assert_eq!(generated_state(&state), None);

    state
        .model
        .netlist_artifacts
        .current_generation_input_digest =
        Some(crate::product::ContentDigest::from_bytes([0x32; 32]));
    assert_eq!(
        generated_state(&state),
        Some(("Older than the schematic", Problem::Warning))
    );

    state.model.netlist_artifacts.generated_source.clear();
    assert_eq!(
        generated_state(&state),
        Some(("Not generated yet", Problem::Warning))
    );
}

#[test]
fn owned_provenance_requires_exact_saved_and_validated_bytes() {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content = "owned\n.end\n".to_owned();
    let project_revision = state.model.workspace.content.project.revision().get();
    let digest = crate::state::content_digest(&state.model.simulation.source.netlist_content);
    state.code.netlist.externally_saved_content_digest = Some(digest);
    state.model.netlist_artifacts.validation = Some(crate::state::NetlistValidationReceipt {
        visible_content_digest: digest,
        executable_source_digest: digest,
        prepared_snapshot_digest: digest,
        project_revision,
        task_count: 1,
        advisory_count: 0,
    });

    // Saved and validated is the ordinary state, and so is a dirty project —
    // the document tab's marker owns that one.
    assert_eq!(owned_source_state(&state, digest), None);
    state.model.workspace.content.netlist_source_dirty = true;
    assert_eq!(owned_source_state(&state, digest), None);

    // Edited since the validation receipt was taken: the receipt no longer
    // describes these bytes, and the panel says so.
    state
        .model
        .simulation
        .source
        .netlist_content
        .push_str("* edit\n");
    let edited = crate::state::content_digest(&state.model.simulation.source.netlist_content);
    assert_eq!(owned_source_state(&state, edited), Some("Not checked yet"));
}
