//! What the netlist document's cases seed: a generated deck that includes
//! one dependency, and an open deck of the same text with its include open
//! beside it. The document's own cases and the source feature's
//! whole-application tests share it.

// Test-only code compiled under test-support, outside the parent's cfg(test) exemption.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use super::{Model, NetlistDocumentState, NetlistTab};
use crate::product::ObjectRevision;
use crate::state::{
    DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
    NetlistDocumentId, SourceLocator,
};
use crate::workbench::documents::deck_files::{OpenDeck, OpenFile};

pub(crate) const ROOT: &str = "deck with an include\n.include \"models/resistor.inc\"\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n";

pub(crate) const ORIGINAL_INCLUDE: &str = ".param rmodel=1k\n";

pub(crate) const INCLUDE_IDENTITY: &str = "models/resistor.inc";

pub(crate) const INCLUDE_NATIVE_ORIGIN: &str = "C:/project/models/resistor.inc";

/// The open deck's file.
pub const DECK_PATH: &str = "C:/project/amp.cir";

/// The identity the open deck's include goes by: its file's path.
pub const DECK_INCLUDE: &str = INCLUDE_NATIVE_ORIGIN;

/// The fixture seeded into a session's model and netlist editor: a
/// generated deck that includes one dependency, and the project's deck of
/// the same text open in front, its include open beside it.
pub fn seed_deck_with_include(model: &mut Model, netlist: &mut NetlistDocumentState) {
    seed_deck(model, netlist, ROOT, ORIGINAL_INCLUDE);
}

/// The same, over a deck of `root`'s text that includes
/// [`INCLUDE_IDENTITY`] first, which reads `include`.
pub fn seed_deck(model: &mut Model, netlist: &mut NetlistDocumentState, root: &str, include: &str) {
    let locator = SourceLocator::try_new(INCLUDE_IDENTITY, "resistor.inc")
        .unwrap()
        .with_native_origin(INCLUDE_NATIVE_ORIGIN)
        .unwrap();
    let dependency = DependencyMetadata::unresolved_direct_to(0, INCLUDE_IDENTITY, locator)
        .unwrap()
        .resolve_utf8(include.as_bytes().to_vec())
        .unwrap();
    let provenance = GeneratedProvenance::try_new(
        "rspice-deck-include-test",
        GenerationInput::new(
            ObjectRevision::INITIAL,
            crate::state::content_digest("deck-include-input"),
        ),
    )
    .unwrap();
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        root.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
    model.netlist_artifacts.generated_source = root.to_owned();
    model.netlist_artifacts.generated_document = Some(generated);

    let mut deck = OpenDeck::in_memory(DECK_PATH, root);
    deck.includes
        .push(OpenFile::in_memory(INCLUDE_NATIVE_ORIGIN, include));
    model.workspace.content.decks = vec![crate::state::ProjectDeck {
        id: deck.id,
        source: crate::state::DeckSource::File {
            path: DECK_PATH.into(),
        },
    }];
    netlist.open_deck = Some(deck);
    netlist.front = NetlistTab::Deck(model.workspace.content.decks[0].id);
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content.set(root.to_owned());
    model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}
