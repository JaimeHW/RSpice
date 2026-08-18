//! Locating a schematic instance in the deck the project generates.
//!
//! The jump reads the generated artifact's source map backwards, so these pin
//! what a caller is told when the deck cannot answer, that the card the caret
//! lands on is the instance's own, and that the round trip back through the
//! netlist returns the instance it started from.

use super::*;
const DECK: &str = "show-in-netlist fixture\nV1 in 0 1\nR1 in out 1k\n+ tc1=0\n.op\n.end\n";

/// A project whose retained generated deck states two of its three instances,
/// with `R1` spread over a card and its continuation.
fn located_instance_state() -> (AppState, u64, u64, u64) {
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
        NetlistDocument, NetlistDocumentId,
    };

    let mut state = AppState::default();
    let origin = Point::new(20, 20);
    let source_id = state
        .schematic
        .add_component(ComponentType::VoltageSource, origin);
    let load_id = state
        .schematic
        .add_component(ComponentType::Resistor, origin + Point::new(20, 0));
    let unmapped_id = state
        .schematic
        .add_component(ComponentType::Capacitor, origin + Point::new(40, 0));
    for (id, name) in [(source_id, "V1"), (load_id, "R1"), (unmapped_id, "C9")] {
        if let Some(component) = state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
        {
            component.name = name.to_owned();
        }
    }

    let view = state.workspace.active_view.key();
    let cell = format!(
        "{}/{}",
        state.workspace.active_view.library, state.workspace.active_view.cell
    );
    let entry = |line: usize, instance: &str, id: u64| {
        GeneratedSourceMapEntry::try_new(
            line,
            cell.clone(),
            view.clone(),
            Some(instance.to_owned()),
            Some(GeneratedSourceMapEntry::component_identity_for(&view, id)),
        )
        .expect("mapping")
    };
    let digest = crate::state::content_digest("show-in-netlist-input");
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "rspice-show-in-netlist-test",
            GenerationInput::new(crate::product::ObjectRevision::INITIAL, digest),
        )
        .expect("provenance"),
        DECK.as_bytes().to_vec(),
        Vec::new(),
        vec![
            entry(2, "V1", source_id),
            entry(3, "R1", load_id),
            entry(4, "R1", load_id),
        ],
    )
    .expect("artifact");

    state.ui.netlist.generated_source = DECK.to_owned();
    state.ui.netlist.generated_document = Some(
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).expect("document"),
    );
    state.ui.netlist.generated_input_digest = Some(digest);
    state.ui.netlist.current_generation_input_digest = Some(digest);
    // The engineer is looking at their own deck, so the jump has a document
    // transition to make rather than a no-op reactivation to skip.
    state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    (state, source_id, load_id, unmapped_id)
}

fn last_user_message(state: &AppState) -> String {
    state
        .log_buffer
        .entries()
        .filter(|entry| entry.source == LogSource::User)
        .last()
        .map(|entry| entry.message.clone())
        .unwrap_or_default()
}

/// A block has to say which of the three things is missing. One generic
/// "unavailable" is what makes an engineer regenerate a deck that was never
/// the problem.
#[test]
fn locating_an_instance_reports_the_exact_reason_it_cannot_be_located() {
    let (mut state, source_id, load_id, unmapped_id) = located_instance_state();

    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("select one instance")
    );

    state.schematic.selection.select_only_component(source_id);
    assert_eq!(state.selected_instance_netlist_block(), None);

    // The instance's own card is the first of its lines, never the
    // continuation the map also carries.
    state.schematic.selection.select_only_component(load_id);
    state.show_selected_instance_in_netlist();
    assert_eq!(state.ui.netlist.requested_line, Some(3));

    state.schematic.selection.select_only_component(unmapped_id);
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("no netlist line for this instance")
    );

    state.schematic.selection.select_component(load_id);
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("select one instance")
    );

    state.schematic.selection.select_only_component(source_id);
    state.ui.netlist.generated_document = None;
    state.ui.netlist.generated_source.clear();
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("no generated netlist yet — open the Netlist workspace to generate one")
    );
}

/// One gesture, one transaction: the workspace, the document and the caret
/// move together. Any of the three left behind shows the engineer a deck that
/// is not the one their instance is in.
#[test]
fn showing_an_instance_opens_the_generated_primary_at_its_card() {
    let (mut state, _, load_id, _) = located_instance_state();
    state.schematic.selection.select_only_component(load_id);

    state.show_selected_instance_in_netlist();

    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        state.ui.netlist.active_document,
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
    );
    assert!(state.ui.netlist.active_dependency_identity.is_none());
    assert_eq!(state.simulation.netlist_content, DECK);
    // The outline and the source-mapping panel both read the active line, so
    // the caret request and the cursor have to agree about which card it is.
    assert_eq!(state.ui.netlist.requested_line, Some(3));
    assert_eq!(state.ui.netlist.cursor_line, 2);
    assert_eq!(
        last_user_message(&state),
        "Located instance R1 at generated line 3."
    );
}

/// A stale deck still resolves — it is what the project last generated — but
/// the announcement has to say the line came from one, because regeneration
/// can move the card out from under the caret.
#[test]
fn showing_an_instance_from_a_stale_deck_says_so_without_blocking_the_jump() {
    let (mut state, source_id, _, _) = located_instance_state();
    state.schematic.selection.select_only_component(source_id);
    state.ui.netlist.current_generation_input_digest =
        Some(crate::state::content_digest("edited-schematic"));

    assert_eq!(state.selected_instance_netlist_block(), None);
    state.show_selected_instance_in_netlist();

    assert_eq!(state.ui.netlist.requested_line, Some(2));
    assert_eq!(
        last_user_message(&state),
        "Located instance V1 at generated line 2 in a stale deck; regenerating it may move the card."
    );
}

/// Schematic to netlist and back has to land on the instance it started from.
/// The forward jump and the navigator's reveal read the same map, so a round
/// trip that moves the selection means the two disagree about the identity.
#[test]
fn the_round_trip_through_the_generated_deck_selects_the_same_instance() {
    let (mut state, _, load_id, _) = located_instance_state();
    state.schematic.selection.select_only_component(load_id);

    state.show_selected_instance_in_netlist();
    assert_eq!(state.schematic.selection.single_component(), Some(load_id));

    // Reading the selection back the way the navigator's source-mapping panel
    // does: the active line, its map entry, and the component it names.
    let active_line = state.ui.netlist.cursor_line.saturating_add(1);
    let revealed = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(crate::state::NetlistDocument::generated_artifact)
        .and_then(|artifact| artifact.source_map_entry(active_line))
        .and_then(crate::state::GeneratedSourceMapEntry::component_id)
        .expect("the active line names a component");
    assert_eq!(revealed, load_id);

    state.schematic.selection.clear();
    state.schematic.selection.select_component(revealed);
    assert_eq!(state.schematic.selection.single_component(), Some(load_id));
    assert_eq!(state.selected_instance_netlist_block(), None);
}
