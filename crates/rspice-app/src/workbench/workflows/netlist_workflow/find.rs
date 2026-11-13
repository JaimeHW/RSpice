//! A press in the netlist page's find window, carried out once the page has
//! drawn.
//!
//! Going to a match opens the document it is in, and a replacement commits
//! through a candidate copy of the whole state, so the window stores the
//! press and the shell carries it out here, with the body the window ran.

use crate::state::replace_source_ranges;
use crate::workbench::MessageId;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, NetlistFindAction, NetlistFindStep, open_owned_source_in,
};

/// Carry out the find window's `step`: replace every match Replace all
/// planned, or go to, or replace, the match it names.
pub(crate) fn apply_netlist_find_step(state: &mut AppState, step: NetlistFindStep) {
    let NetlistFindStep {
        action,
        options,
        target,
        replacements: scoped_replacements,
    } = step;
    let messages = state.workbench.messages();

    if let Some(replacements) = scoped_replacements {
        match crate::workbench::documents::netlist_document::replace_owned_sources_atomically(
            &mut state.model,
            &mut state.code.netlist,
            &mut state.code.netlist_open_documents,
            replacements,
        ) {
            Ok(count) if count > 0 => {
                state.code.netlist.find.selected_match = 0;
                let count_text = count.to_string();
                state.prompt(messages.format(
                    if count == 1 {
                        MessageId::NetlistFindReplacedSingular
                    } else {
                        MessageId::NetlistFindReplaced
                    },
                    &[("count", &count_text)],
                ));
            }
            Ok(_) => {}
            // The find window states the refusal in place.
            Err(error) => state.code.netlist.find.error = Some(error),
        }
        return;
    }

    let Some(target) = target else {
        return;
    };
    match action {
        Some(NetlistFindAction::Select(_)) => {
            if let Some(deck_id) = target.deck_id
                && let Err(error) =
                    crate::workbench::documents::netlist_document::select_retained_top_deck(
                        &mut state.model,
                        &mut state.code.netlist,
                        &mut state.code.netlist_open_documents,
                        deck_id,
                    )
            {
                state.prompt(error);
                return;
            }
            match target.active_document {
                ActiveNetlistDocument::Generated => {
                    let _ =
                        crate::workbench::documents::netlist_document::open_generated_primary_in(
                            &mut state.model,
                            &mut state.code.netlist,
                        );
                }
                ActiveNetlistDocument::OwnedSource => {
                    let _ = open_owned_source_in(&mut state.model, &mut state.code.netlist);
                }
                ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {}
            }
            if let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                        &mut state.model,
                        &mut state.code.netlist,
                        &mut state.code.netlist_open_documents,
                        &mut state.runs,
                        identity,
                    )
            {
                state.prompt(error);
                return;
            }
            state.code.netlist.requested_line = Some(target.line);
        }
        Some(NetlistFindAction::ReplaceNext) => {
            let Some(source) = target.source.as_deref().filter(|_| target.editable) else {
                return;
            };
            if target.deck_id.is_none()
                && target.active_document == ActiveNetlistDocument::OwnedSource
                && state.code.netlist.active_document != ActiveNetlistDocument::OwnedSource
            {
                let _ = open_owned_source_in(&mut state.model, &mut state.code.netlist);
            }
            if target.deck_id.is_none()
                && let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                        &mut state.model,
                        &mut state.code.netlist,
                        &mut state.code.netlist_open_documents,
                        &mut state.runs,
                        identity,
                    )
            {
                state.prompt(error);
                return;
            }
            match replace_source_ranges(
                source,
                &state.code.netlist.find.find,
                &state.code.netlist.find.replacement,
                options,
                std::slice::from_ref(&target.byte_range),
                false,
            ) {
                Ok(outcome) => {
                    let count = outcome.replacement_count();
                    let replacement = outcome.into_source();
                    let edit = match (target.deck_id, target.dependency_identity.as_deref()) {
                        (Some(deck_id), Some(identity)) => crate::workbench::documents::netlist_document::OwnedNetlistReplacement::retained_dependency(
                            deck_id,
                            identity,
                            source,
                            replacement,
                            count,
                        ),
                        (Some(deck_id), None) => crate::workbench::documents::netlist_document::OwnedNetlistReplacement::retained_root(
                            deck_id,
                            source,
                            replacement,
                            count,
                        ),
                        (None, Some(identity)) => crate::workbench::documents::netlist_document::OwnedNetlistReplacement::dependency(
                            identity,
                            source,
                            replacement,
                            count,
                        ),
                        (None, None) => crate::workbench::documents::netlist_document::OwnedNetlistReplacement::root(
                            source,
                            replacement,
                            count,
                        ),
                    };
                    let replaced = if count == 0 {
                        false
                    } else {
                        match crate::workbench::documents::netlist_document::replace_owned_sources_atomically(
                            &mut state.model,
                            &mut state.code.netlist,
                            &mut state.code.netlist_open_documents,
                            vec![edit],
                        ) {
                            Ok(_) => true,
                            Err(error) => {
                                state.code.netlist.find.error = Some(error);
                                false
                            }
                        }
                    };
                    if replaced {
                        if let Some(deck_id) = target.deck_id
                            && let Err(error) = crate::workbench::documents::netlist_document::select_retained_top_deck(
                                &mut state.model,
                                &mut state.code.netlist,
                                &mut state.code.netlist_open_documents,
                                deck_id,
                            )
                        {
                            state.prompt(error);
                            return;
                        }
                        if let Some(identity) = target.dependency_identity.as_deref()
                            && let Err(error) = crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                                &mut state.model,
                                &mut state.code.netlist,
                                &mut state.code.netlist_open_documents,
                                &mut state.runs,
                                identity,
                            )
                        {
                            state.prompt(error);
                            return;
                        }
                        state.code.netlist.requested_line = Some(target.line);
                        state.code.netlist.find.selected_match = 0;
                        let count_text = count.to_string();
                        state.prompt(messages.format(
                            if count == 1 {
                                MessageId::NetlistFindReplacedSingular
                            } else {
                                MessageId::NetlistFindReplaced
                            },
                            &[("count", &count_text)],
                        ));
                    }
                }
                Err(error) => state.code.netlist.find.error = Some(error.to_string()),
            }
        }
        Some(NetlistFindAction::ReplaceAll) => {}
        None => {}
    }
}
