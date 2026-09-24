//! What a retained `.lib` include offers, and which of it the deck takes.
//!
//! A foundry corner file declares several sections and a deck binds exactly
//! one of them by name. The name is written on the deck's own `.lib` card; the
//! set of names lives in the library's bytes. Neither surface can state the
//! pair alone, so this module joins them: it enumerates the sections a
//! retained dependency's exact bytes declare — keyed by the digest the closure
//! already recorded for those bytes, so a corner file is parsed once rather
//! than once per frame — and it rewrites the card's section token through the
//! document engine's ordinary replacement transaction, which is what makes the
//! change undoable and revalidated instead of a string splice.
//!
//! The bytes read here are the ones the closure retained. Re-opening the host
//! file would let the navigator describe a library that validation never saw.

use rspice_core::library::LibSectionSummary;

use crate::workbench::AppState;
use crate::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, active_netlist_source_is_editable, replace_owned_sources_atomically,
};

/// Bring the retained-section cache level with the active closure.
///
/// Called before the include rows are projected. Enumerating costs a library
/// parse, so an entry survives until the bytes behind its locator change; an
/// entry whose locator left the closure is dropped, which is what bounds the
/// map to the dependencies the document actually has.
pub(super) fn refresh(state: &mut AppState) {
    let Some(document) = super::active_canonical_netlist_document(state) else {
        state.ui.code_workspace.include_lib_sections.clear();
        return;
    };
    let cache = &state.ui.code_workspace.include_lib_sections;
    let mut live = std::collections::BTreeSet::new();
    let mut computed = Vec::new();
    for dependency in document.dependencies() {
        let Some(digest) = dependency.resolution().content_digest() else {
            continue;
        };
        let locator = dependency.requested_locator();
        live.insert(locator.to_owned());
        if cache
            .get(locator)
            .is_some_and(|(cached, _)| *cached == digest)
        {
            continue;
        }
        let Some(bytes) = dependency.source_bytes() else {
            continue;
        };
        let sections: std::sync::Arc<[LibSectionSummary]> =
            rspice_core::library::enumerate_lib_sections(bytes).into();
        computed.push((locator.to_owned(), (digest, sections)));
    }
    if computed.is_empty() && live.len() == cache.len() {
        return;
    }
    let cache = &mut state.ui.code_workspace.include_lib_sections;
    cache.retain(|locator, _| live.contains(locator));
    cache.extend(computed);
}

/// The sections the retained bytes behind `locator` declare, in declaration
/// order. A locator with no retained bytes, or one whose library declares
/// none, offers nothing.
pub(super) fn declared(state: &AppState, locator: &str) -> Vec<LibSectionSummary> {
    state
        .ui
        .code_workspace
        .include_lib_sections
        .get(locator)
        .map_or_else(Vec::new, |(_, sections)| sections.to_vec())
}

/// The section a card selects, read from the card itself.
///
/// A one-argument `.lib` opens a section rather than selecting one, and a
/// plain `.include` selects nothing; both answer `None`.
pub(super) fn selected(card: &str) -> Option<String> {
    rspice_core::netlist::parse_lib_directive(card).and_then(|(_, section)| section)
}

/// Point the `.lib` card at `line` of the visible source at `section`.
///
/// The card is rewritten in the authored source through the same transaction
/// an editor replacement uses, so the change is undoable, dirties the
/// document, and revalidates the closure. Only the section token moves: the
/// locator, its quoting, and anything after it are left exactly as written.
pub(super) fn use_section(
    state: &mut AppState,
    line: usize,
    section: &str,
) -> Result<String, String> {
    if !active_netlist_source_is_editable(state) {
        return Err(
            "This netlist source is read-only; its .lib section cannot be changed.".to_owned(),
        );
    }
    let source = state.simulation.netlist_content.clone();
    let span = section_token_span(&source, line)
        .ok_or_else(|| format!("Line {line} does not write a `.lib \"file\" section` card."))?;
    if source[span.clone()].trim_matches(['"', '\'']) == section {
        return Err(format!("The card already selects section {section}."));
    }
    let mut replacement = source.clone();
    replacement.replace_range(span, section);
    let edit = if let Some(identity) = state.ui.netlist.active_dependency_identity.as_deref() {
        OwnedNetlistReplacement::dependency(identity, &source, replacement, 1)
    } else {
        OwnedNetlistReplacement::root(&source, replacement, 1)
    };
    replace_owned_sources_atomically(state, vec![edit])?;
    Ok(format!("Selected .lib section {section} on line {line}."))
}

/// The byte range of the section token on a `.lib "file" section` card.
///
/// The card is identified by the engine's own library-directive parser and the
/// token located by the outline's tokenizer, so no third reading of SPICE
/// decides what a section token is. A quoted token is replaced including its
/// quotes: a section name that needed them would not be a section name.
fn section_token_span(source: &str, line: usize) -> Option<std::ops::Range<usize>> {
    let line_start = line_start_offset(source, line)?;
    let card = source[line_start..].lines().next()?;
    rspice_core::netlist::parse_lib_directive(card)?.1?;
    let (_, column) = crate::state::netlist_document::card_tokens_with_columns(card)
        .into_iter()
        .nth(2)?;
    let start = card
        .char_indices()
        .nth(column.checked_sub(1)?)
        .map(|(offset, _)| offset)?;
    let quote = card[start..].chars().next()?;
    let end = if quote == '"' || quote == '\'' {
        let text = start + quote.len_utf8();
        text + card[text..].find(quote)? + quote.len_utf8()
    } else {
        card[start..]
            .find(|ch: char| ch.is_whitespace() || ch == ';' || ch == '$')
            .map_or(card.len(), |offset| start + offset)
    };
    Some(line_start + start..line_start + end)
}

/// Byte offset of a one-based line, counted the way the outline counts lines.
fn line_start_offset(source: &str, line: usize) -> Option<usize> {
    let mut offset = 0usize;
    for (index, text) in source.split_inclusive('\n').enumerate() {
        if index + 1 == line {
            return Some(offset);
        }
        offset += text.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ObjectRevision;
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
        NetlistDocument, NetlistDocumentId, SourceLocator,
    };
    use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, undo_netlist_edit};

    const ROOT: &str =
        "corner deck\n.lib \"corners.lib\" tt\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n";
    const CORNERS: &str = "* corners\n\
         .lib tt\n\
         .model nch NMOS (LEVEL=1)\n\
         .endl tt\n\
         .lib ff\n\
         .model nch NMOS (LEVEL=1)\n\
         .endl ff\n";
    const LOCATOR: &str = "corners.lib";

    fn corner_deck_state(root: &str) -> AppState {
        let locator = SourceLocator::try_new(LOCATOR, "corners.lib").unwrap();
        let dependency = DependencyMetadata::unresolved_direct_to(0, LOCATOR, locator)
            .unwrap()
            .resolve_utf8(CORNERS.as_bytes().to_vec())
            .unwrap();
        let provenance = GeneratedProvenance::try_new(
            "rspice-lib-section-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                crate::state::content_digest("lib-section-input"),
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
        let generated =
            NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
        let owned = generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
            .unwrap();

        let mut state = AppState::default();
        state.workspace.netlist_source = Some(root.to_owned());
        state.workspace.netlist_document = Some(owned.clone());
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            deck_id: uuid::Uuid::new_v4(),
            artifact_name: "owned.cir".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
            compatibility_reviewed: true,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
        state.ui.netlist.generated_source = root.to_owned();
        state.ui.netlist.generated_document = Some(generated);
        state.ui.netlist.owned_document = Some(owned);
        state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        state.ui.netlist.active_document_initialized = true;
        state.simulation.netlist_content = root.to_owned();
        state.workspace.validate_simulation_configuration().unwrap();
        state
    }

    /// The navigator asks what every row offers on each frame. Enumerating a
    /// corner file costs a library parse, so the answer must survive until the
    /// bytes it describes change.
    #[test]
    fn sections_are_enumerated_from_retained_bytes_once_per_revision() {
        let mut state = corner_deck_state(ROOT);
        refresh(&mut state);

        assert_eq!(
            declared(&state, LOCATOR)
                .iter()
                .map(|section| section.name.clone())
                .collect::<Vec<_>>(),
            ["tt", "ff"],
            "the row's alternatives come from the bytes the closure retained"
        );
        let first = state.ui.code_workspace.include_lib_sections[LOCATOR]
            .1
            .clone();
        refresh(&mut state);
        assert!(
            std::sync::Arc::ptr_eq(
                &first,
                &state.ui.code_workspace.include_lib_sections[LOCATOR].1
            ),
            "unchanged bytes must not be parsed a second time"
        );

        // A locator that leaves the closure leaves the cache with it.
        state.ui.netlist.owned_document = None;
        state.workspace.netlist_document = None;
        refresh(&mut state);
        assert!(state.ui.code_workspace.include_lib_sections.is_empty());
    }

    #[test]
    fn choosing_a_section_rewrites_only_the_section_token_and_can_be_undone() {
        let mut state = corner_deck_state(ROOT);

        use_section(&mut state, 2, "ff").expect("the owned root is editable");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("corner deck\n.lib \"corners.lib\" ff\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n"),
            "the locator, its quotes, and every other card stay exactly as \
             written"
        );
        assert!(
            state.workspace.netlist_source_dirty,
            "a rewritten card is an unsaved change"
        );

        undo_netlist_edit(&mut state).expect("the rewrite is one journal entry");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(ROOT),
            "undo must restore the card the deck was authored with"
        );
    }

    #[test]
    fn a_section_rewrite_states_why_it_cannot_run() {
        let mut state = corner_deck_state(ROOT);
        assert_eq!(
            use_section(&mut state, 2, "tt").unwrap_err(),
            "The card already selects section tt.",
            "re-selecting the bound section is a no-op, not an edit"
        );
        assert_eq!(
            use_section(&mut state, 3, "ff").unwrap_err(),
            "Line 3 does not write a `.lib \"file\" section` card.",
            "only a library card has a section token to move"
        );

        state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
        assert_eq!(
            use_section(&mut state, 2, "ff").unwrap_err(),
            "This netlist source is read-only; its .lib section cannot be changed.",
            "a generated document is reviewed, not authored"
        );
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(ROOT),
            "a refused rewrite changes nothing"
        );
    }

    /// The section token is found the way the outline finds tokens, so an
    /// unquoted locator, a quoted section, and a trailing comment all land on
    /// the same range.
    #[test]
    fn the_section_token_is_located_exactly_on_every_card_shape() {
        for (card, expected) in [
            (".lib \"corners.lib\" tt", "tt"),
            (".lib corners.lib tt ; keep", "tt"),
            ("  .LIB 'corners.lib'   'tt'", "'tt'"),
        ] {
            let source = format!("deck\n{card}\n.end\n");
            let span = section_token_span(&source, 2)
                .unwrap_or_else(|| panic!("a section token on {card:?}"));
            assert_eq!(&source[span], expected, "{card:?}");
        }
        assert!(section_token_span("deck\n.include corners.lib\n", 2).is_none());
        assert!(section_token_span("deck\n.lib tt\n", 2).is_none());
        assert!(section_token_span("deck\n.lib \"corners.lib\" tt\n", 9).is_none());
    }

    #[test]
    fn a_card_names_the_section_it_binds_and_nothing_else_does() {
        assert_eq!(selected(".lib \"corners.lib\" tt"), Some("tt".to_owned()));
        assert_eq!(selected(".include corners.lib"), None);
        assert_eq!(selected(".lib tt"), None, "this card opens a section");
    }
}
