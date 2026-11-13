//! A press in the netlist page's find window, carried out once the page has
//! drawn.
//!
//! Going to a match opens the document it is in, and a replacement commits
//! through a deck transaction over the model, the netlist editor's state
//! and the include tabs, so the window stores the press and the shell
//! carries it out here, with the body the window ran.

use std::collections::HashSet;

use rspice_app_runs::workbench::slots::runs::RunsState;

use crate::state::replace_source_ranges;
use crate::workbench::MessageId;
use crate::workbench::app_state::Model;
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, NetlistDocumentState, NetlistFindAction, NetlistFindStep,
    open_owned_source_in,
};
use crate::workbench::state::{WorkbenchSession, WorkspaceDocumentId};

/// Carry out the find window's `step`: replace every match Replace all
/// planned, or go to, or replace, the match it names.
pub(crate) fn apply_netlist_find_step(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    workbench: &mut WorkbenchSession,
    step: NetlistFindStep,
) {
    let NetlistFindStep {
        action,
        options,
        target,
        replacements: scoped_replacements,
    } = step;
    let messages = workbench.messages();

    if let Some(replacements) = scoped_replacements {
        match crate::workbench::documents::netlist_document::replace_owned_sources_atomically(
            model,
            netlist,
            open_documents,
            replacements,
        ) {
            Ok(count) if count > 0 => {
                netlist.find.selected_match = 0;
                let count_text = count.to_string();
                workbench.prompt(messages.format(
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
            Err(error) => netlist.find.error = Some(error),
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
                        model,
                        netlist,
                        open_documents,
                        deck_id,
                    )
            {
                workbench.prompt(error);
                return;
            }
            match target.active_document {
                ActiveNetlistDocument::Generated => {
                    let _ =
                        crate::workbench::documents::netlist_document::open_generated_primary_in(
                            model, netlist,
                        );
                }
                ActiveNetlistDocument::OwnedSource => {
                    let _ = open_owned_source_in(model, netlist);
                }
                ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {}
            }
            if let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                        model,
                        netlist,
                        open_documents,
                        runs,
                        identity,
                    )
            {
                workbench.prompt(error);
                return;
            }
            netlist.requested_line = Some(target.line);
        }
        Some(NetlistFindAction::ReplaceNext) => {
            let Some(source) = target.source.as_deref().filter(|_| target.editable) else {
                return;
            };
            if target.deck_id.is_none()
                && target.active_document == ActiveNetlistDocument::OwnedSource
                && netlist.active_document != ActiveNetlistDocument::OwnedSource
            {
                let _ = open_owned_source_in(model, netlist);
            }
            if target.deck_id.is_none()
                && let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                        model,
                        netlist,
                        open_documents,
                        runs,
                        identity,
                    )
            {
                workbench.prompt(error);
                return;
            }
            match replace_source_ranges(
                source,
                &netlist.find.find,
                &netlist.find.replacement,
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
                            model,
                            netlist,
                            open_documents,
                            vec![edit],
                        ) {
                            Ok(_) => true,
                            Err(error) => {
                                netlist.find.error = Some(error);
                                false
                            }
                        }
                    };
                    if replaced {
                        if let Some(deck_id) = target.deck_id
                            && let Err(error) = crate::workbench::documents::netlist_document::select_retained_top_deck(
                                model,
                                netlist,
                                open_documents,
                                deck_id,
                            )
                        {
                            workbench.prompt(error);
                            return;
                        }
                        if let Some(identity) = target.dependency_identity.as_deref()
                            && let Err(error) = crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                                model,
                                netlist,
                                open_documents,
                                runs,
                                identity,
                            )
                        {
                            workbench.prompt(error);
                            return;
                        }
                        netlist.requested_line = Some(target.line);
                        netlist.find.selected_match = 0;
                        let count_text = count.to_string();
                        workbench.prompt(messages.format(
                            if count == 1 {
                                MessageId::NetlistFindReplacedSingular
                            } else {
                                MessageId::NetlistFindReplaced
                            },
                            &[("count", &count_text)],
                        ));
                    }
                }
                Err(error) => netlist.find.error = Some(error.to_string()),
            }
        }
        Some(NetlistFindAction::ReplaceAll) => {}
        None => {}
    }
}
