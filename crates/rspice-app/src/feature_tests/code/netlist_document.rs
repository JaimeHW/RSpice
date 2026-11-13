//! The netlist document over the whole session: a first top deck authored
//! with no synthetic generated baseline, and a run's deck snapshot beside
//! the working deck.

use crate::workbench::AppState;
use crate::workbench::documents::CodeSourceFileAction;
use crate::workbench::documents::netlist_document::tests::seed_owned_dependency;
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, active_netlist_source_is_editable_in, begin_netlist_lifecycle_action_in,
    close_run_deck_snapshot, commit_netlist_lifecycle_action, open_run_deck_snapshot_in,
};

#[test]
fn first_top_deck_is_authored_without_a_synthetic_generated_baseline() {
    let mut state = AppState::default();
    state.model.design_checks.drc_results = Some(rspice_design::drc::DrcResult::new());
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();
    assert!(state.model.workspace.content.netlist_descriptor.is_none());
    assert!(state.model.workspace.content.netlist_document.is_none());

    begin_netlist_lifecycle_action_in(
        &state.model,
        &mut state.code.netlist,
        CodeSourceFileAction::New,
    )
    .unwrap();
    let transaction = state
        .code
        .netlist
        .lifecycle_dialog
        .transaction
        .as_ref()
        .unwrap();
    assert!(transaction.deck_id.is_none());
    assert!(transaction.document_id.is_none());
    commit_netlist_lifecycle_action(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
    )
    .unwrap();

    assert!(state.is_netlist_first_without_schematic());
    assert!(state.model.design_checks.drc_results.is_none());
    assert_eq!(state.model.design_checks.drc_checked_version, 0);

    let document = state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(document.source(), "* New RSpice top deck\n.end\n");
    assert!(document.generated_artifact().is_none());
    assert!(document.provenance().generated().is_none());
    assert_eq!(
        document.ownership(),
        crate::state::DocumentOwnership::Editable
    );
    assert!(
        state
            .model
            .workspace
            .content
            .retained_netlist_decks
            .is_empty()
    );
    state
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

/// The deck a run consumed is not the deck in the editor: opening the snapshot
/// must project the sealed bytes, and the working source must survive intact.
#[test]
fn run_deck_snapshot_projects_the_sealed_deck_and_leaves_the_working_deck_alone() {
    const RAN: &str = "ran deck\nR1 out 0 1k\n.op\n.end\n";
    const EDITED: &str = "ran deck\nR1 out 0 2k\n.op\n.end\n";

    let mut state = AppState::default();
    seed_owned_dependency(&mut state.model, &mut state.code.netlist);
    state.model.workspace.content.netlist_source = Some(EDITED.to_owned());
    state.model.simulation.source.netlist_content = EDITED.to_owned();
    state.code.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.code.netlist.active_document_initialized = true;
    state.runs.manual_run.last_run_buffer = Some(RAN.to_owned());
    state.runs.manual_run.last_run_id = Some(3);

    assert!(open_run_deck_snapshot_in(
        &mut state.model,
        &mut state.code.netlist,
        &state.runs
    ));
    assert_eq!(
        state.code.netlist.active_document,
        ActiveNetlistDocument::RunSnapshot
    );
    assert_eq!(state.model.simulation.source.netlist_content, RAN);
    assert!(!active_netlist_source_is_editable_in(
        &state.model,
        &state.code.netlist
    ));
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some(EDITED)
    );
    assert!(state.manual_deck_run_block_reason().is_some());

    assert!(close_run_deck_snapshot(
        &mut state.model,
        &mut state.code.netlist
    ));
    assert_eq!(
        state.code.netlist.active_document,
        ActiveNetlistDocument::OwnedSource
    );
    assert_eq!(state.model.simulation.source.netlist_content, EDITED);
}
