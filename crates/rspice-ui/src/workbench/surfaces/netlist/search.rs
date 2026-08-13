//! Find and replace across the netlist document set.
//!
//! A scope names a set of documents rather than one buffer, so a match always
//! carries the document it came from and whether that document is editable.
//! Read-only generated source is searchable and never replaceable.

use egui::Ui;

use crate::diagnostics::ConsoleMessage;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::{AppState, MessageCatalog, MessageId, RSpiceApp};

use super::ownership::open_owned_source;

/// How many matches the surface will hold at once.
///
/// A result list is read, not exhausted: a query that matches a hundred
/// thousand times in a flat deck is a query to narrow, and materializing every
/// hit to say so costs more than the search. The count reported alongside the
/// rows states plainly that it is a first page.
const NETLIST_FIND_RESULT_LIMIT: usize = 500;

#[derive(Debug, Clone, Copy)]
enum FindWindowAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

/// One searchable document. The source is borrowed from application state:
/// the closure a project-references search covers is the whole include graph,
/// and copying it per frame copies the design.
#[derive(Debug, Clone)]
struct NetlistSearchDocument<'a> {
    active_document: ActiveNetlistDocument,
    dependency_identity: Option<&'a str>,
    editable: bool,
    label: String,
    source: &'a str,
}

#[derive(Debug, Clone)]
struct NetlistSearchMatch {
    /// Index into the scanned document list, so a match is the size of a
    /// match rather than the size of the document it was found in.
    document: usize,
    found: crate::state::FindMatch,
    /// The card the match sits on, captured while the document was walked.
    card: String,
}

/// The match an action names, held independently of the scan it came from.
struct FindTarget {
    index: usize,
    active_document: ActiveNetlistDocument,
    dependency_identity: Option<String>,
    editable: bool,
    /// The document's bytes, present only for a replacement.
    source: Option<String>,
    line: usize,
    caret_byte: usize,
}

/// Matches for one query across one scope, bounded in count.
struct NetlistSearchResults {
    matches: Vec<NetlistSearchMatch>,
    /// Whether the deck holds more matches than the surface retained.
    truncated: bool,
    error: Option<crate::state::FindError>,
}

fn plan_all_owned_replacements(
    documents: &[NetlistSearchDocument<'_>],
    query: &str,
    replacement: &str,
    options: crate::state::FindOptions,
) -> Result<
    Vec<crate::workbench::documents::netlist_document::OwnedNetlistReplacement>,
    crate::state::FindError,
> {
    use crate::state::{ReplaceScope, replace_in_source};
    use crate::workbench::documents::netlist_document::OwnedNetlistReplacement;

    let mut edits = Vec::new();
    for document in documents.iter().filter(|document| document.editable) {
        let outcome = replace_in_source(
            document.source,
            query,
            replacement,
            options,
            ReplaceScope::All,
        )?;
        let replacement_count = outcome.replacement_count();
        let replaced_source = outcome.into_source();
        if replacement_count == 0 || replaced_source == document.source {
            continue;
        }
        edits.push(if let Some(identity) = document.dependency_identity {
            OwnedNetlistReplacement::dependency(
                identity,
                document.source,
                replaced_source,
                replacement_count,
            )
        } else {
            OwnedNetlistReplacement::root(document.source, replaced_source, replacement_count)
        });
    }
    Ok(edits)
}

fn netlist_search_documents(
    state: &AppState,
    scope: crate::workbench::documents::netlist_document::NetlistFindScope,
) -> Vec<NetlistSearchDocument<'_>> {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let generated = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::Generated,
        dependency_identity: None,
        editable: false,
        label: "generated.sp".to_owned(),
        source: state.ui.netlist.generated_source.as_str(),
    };
    let owned = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::OwnedSource,
        dependency_identity: None,
        editable: true,
        label: state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        source: state
            .workspace
            .netlist_source
            .as_deref()
            .unwrap_or_default(),
    };

    match scope {
        NetlistFindScope::CurrentDocument => {
            if let Some(dependency) =
                crate::workbench::documents::netlist_document::active_dependency(state)
            {
                vec![NetlistSearchDocument {
                    active_document: state
                        .ui
                        .netlist
                        .active_dependency_root
                        .unwrap_or(state.ui.netlist.active_document),
                    dependency_identity: Some(dependency.locator().logical_identity()),
                    editable:
                        crate::workbench::documents::netlist_document::active_dependency_is_owned(
                            state,
                        ),
                    label: dependency.locator().display_name().to_owned(),
                    source: dependency.source().unwrap_or_default(),
                }]
            } else {
                vec![match state.ui.netlist.active_document {
                    ActiveNetlistDocument::Generated => generated(),
                    ActiveNetlistDocument::OwnedSource => owned(),
                    ActiveNetlistDocument::GeneratedDiff => NetlistSearchDocument {
                        active_document: ActiveNetlistDocument::GeneratedDiff,
                        dependency_identity: None,
                        editable: false,
                        label: "generated.diff".to_owned(),
                        source: state.ui.netlist.generated_diff_source.as_str(),
                    },
                }]
            }
        }
        NetlistFindScope::AllOwnedSources => {
            let mut documents = state
                .workspace
                .netlist_source
                .as_ref()
                .map(|_| vec![owned()])
                .unwrap_or_default();
            documents.extend(dependency_search_documents(
                state,
                ActiveNetlistDocument::OwnedSource,
                true,
            ));
            documents
        }
        NetlistFindScope::ProjectReferences => {
            let mut documents = Vec::new();
            if !state.ui.netlist.generated_source.is_empty() {
                documents.push(generated());
                documents.extend(dependency_search_documents(
                    state,
                    ActiveNetlistDocument::Generated,
                    false,
                ));
            }
            if state.workspace.netlist_source.is_some() {
                documents.push(owned());
                documents.extend(dependency_search_documents(
                    state,
                    ActiveNetlistDocument::OwnedSource,
                    false,
                ));
            }
            documents
        }
    }
}

fn dependency_search_documents(
    state: &AppState,
    root: ActiveNetlistDocument,
    owned_only: bool,
) -> Vec<NetlistSearchDocument<'_>> {
    let document = match root {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff => None,
    };
    let owned = state.workspace.netlist_descriptor.as_ref();
    document
        .into_iter()
        .flat_map(crate::state::NetlistDocument::dependencies)
        .filter_map(|dependency| {
            let source = dependency.source()?;
            let identity = dependency.locator().logical_identity();
            let editable = root == ActiveNetlistDocument::OwnedSource
                && owned
                    .and_then(|value| value.owned_include(identity))
                    .is_some();
            (!owned_only || editable).then(|| NetlistSearchDocument {
                active_document: root,
                dependency_identity: Some(identity),
                editable,
                label: dependency.locator().display_name().to_owned(),
                source,
            })
        })
        .collect()
}

/// Scan every document in the scope, stopping once the surface is full.
///
/// The limit is shared across documents rather than applied per document, so
/// the surface holds the first `NETLIST_FIND_RESULT_LIMIT` matches in scope
/// order however they are distributed. Each document is walked once for the
/// card text: deriving it per row costs the document per row.
fn scan_documents(
    documents: &[NetlistSearchDocument<'_>],
    query: &str,
    options: crate::state::FindOptions,
) -> NetlistSearchResults {
    use crate::state::find_all_in_source_bounded;

    let mut results = NetlistSearchResults {
        matches: Vec::new(),
        truncated: false,
        error: None,
    };
    if query.is_empty() {
        return results;
    }
    for (index, document) in documents.iter().enumerate() {
        let remaining = NETLIST_FIND_RESULT_LIMIT.saturating_sub(results.matches.len());
        if remaining == 0 {
            // Something is left only if a later document holds a match at all.
            results.truncated = find_all_in_source_bounded(document.source, query, options, 1)
                .is_ok_and(|found| !found.matches().is_empty());
            if results.truncated {
                break;
            }
            continue;
        }
        let found = match find_all_in_source_bounded(document.source, query, options, remaining) {
            Ok(found) => found,
            Err(error) => {
                results.matches.clear();
                results.truncated = false;
                results.error = Some(error);
                return results;
            }
        };
        results.truncated |= found.truncated();
        let mut cards = document.source.lines();
        let mut walked = 0usize;
        let mut card = "";
        for one in found.matches() {
            while walked < one.line() {
                card = cards.next().unwrap_or_default();
                walked += 1;
            }
            results.matches.push(NetlistSearchMatch {
                document: index,
                found: one.clone(),
                card: card.trim().to_owned(),
            });
        }
    }
    results
}

pub(super) fn find_replace_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.find.open {
        return;
    }

    use crate::state::{FindDirection, FindOptions, ReplaceScope, replace_in_source};
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let owned = crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
        &app.state,
    );
    let messages = app.state.ui.messages();
    let mut find = app.state.ui.netlist.find.clone();
    let options = FindOptions {
        direction: FindDirection::Forward,
        match_case: find.match_case,
        whole_word: find.whole_symbol,
        regular_expression: find.regular_expression,
    };
    let documents = netlist_search_documents(&app.state, find.scope);
    let found = scan_documents(&documents, &find.find, options);
    find.error = found.error.as_ref().map(ToString::to_string);
    let truncated = found.truncated;
    let matches = found.matches;
    if matches.is_empty() {
        find.selected_match = 0;
    } else {
        find.selected_match = find.selected_match.min(matches.len() - 1);
    }

    let mut action = None;
    let has_matches = !matches.is_empty();
    let compact_fields = ctx.content_rect().width() < 360.0;
    let find_hint = if find.find.is_empty() {
        messages.text(MessageId::NetlistFindEnterTextHint)
    } else if find.error.is_some() {
        messages.text(MessageId::NetlistFindCorrectExpressionHint)
    } else {
        messages.text(MessageId::NetlistFindGeneratedImmutableHint)
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistFindEyebrow),
        messages.text(MessageId::NetlistFindTitle),
        messages.text(MessageId::NetlistFindNext),
    )
    .description(messages.text(MessageId::NetlistFindDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(has_matches && find.error.is_none())
    .ghost(messages.text(MessageId::CommonClose))
    .hint(find_hint)
    .show_with_initial_body_focus(ctx, |ui| {
        let mut find_control_id = None;
        if compact_fields {
            ui.label(messages.text(MessageId::CommonFind));
            let response = ui.add(
                egui::TextEdit::singleline(&mut find.find)
                    .desired_width(ui.available_width())
                    .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
            );
            find_control_id = Some(response.id);
            ui.label(messages.text(MessageId::CommonReplace));
            ui.add_enabled(
                (owned || find.scope == NetlistFindScope::AllOwnedSources)
                    && find.scope != NetlistFindScope::ProjectReferences,
                egui::TextEdit::singleline(&mut find.replacement)
                    .desired_width(ui.available_width()),
            );
            ui.label(messages.text(MessageId::CommonScope));
            find_scope_combo(
                ui,
                &mut find,
                app.state.workspace.netlist_source.is_some(),
                messages,
            );
        } else {
            egui::Grid::new("rspice.code.find-fields")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label(messages.text(MessageId::CommonFind));
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut find.find)
                            .desired_width(ui.available_width().max(180.0))
                            .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
                    );
                    find_control_id = Some(response.id);
                    ui.end_row();
                    ui.label(messages.text(MessageId::CommonReplace));
                    ui.add_enabled(
                        (owned || find.scope == NetlistFindScope::AllOwnedSources)
                            && find.scope != NetlistFindScope::ProjectReferences,
                        egui::TextEdit::singleline(&mut find.replacement)
                            .desired_width(ui.available_width().max(180.0)),
                    );
                    ui.end_row();
                    ui.label(messages.text(MessageId::CommonScope));
                    find_scope_combo(
                        ui,
                        &mut find,
                        app.state.workspace.netlist_source.is_some(),
                        messages,
                    );
                    ui.end_row();
                });
        }

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(
                &mut find.match_case,
                messages.text(MessageId::NetlistFindMatchCase),
            );
            ui.checkbox(
                &mut find.whole_symbol,
                messages.text(MessageId::NetlistFindWholeSymbol),
            );
            ui.checkbox(
                &mut find.regular_expression,
                messages.text(MessageId::NetlistFindRegularExpression),
            );
        });
        ui.separator();

        if let Some(error) = &find.error {
            ui.colored_label(Tokens::get(ctx).color.err, error);
        } else if find.find.is_empty() {
            ui.weak(messages.text(MessageId::NetlistFindEnterExactArtifact));
        } else {
            let count = matches.len().to_string();
            ui.label(messages.format(
                if truncated {
                    // The rows are a first page, and a count that did not say
                    // so would be read as the number of matches in the deck.
                    MessageId::NetlistFindFirstMatches
                } else if matches.len() == 1 {
                    MessageId::NetlistFindMatchSingular
                } else {
                    MessageId::NetlistFindMatches
                },
                &[("count", &count)],
            ));
        }

        let row_height = 24.0;
        egui::ScrollArea::vertical()
            .id_salt("rspice.code.find-results")
            .max_height(168.0)
            .show_rows(ui, row_height, matches.len(), |ui, rows| {
                let show_document =
                    documents.len() > 1 || find.scope != NetlistFindScope::CurrentDocument;
                for index in rows {
                    let Some(result) = matches.get(index) else {
                        continue;
                    };
                    let label = documents
                        .get(result.document)
                        .map_or("", |document| document.label.as_str());
                    let location = if show_document {
                        format!("{label}  {}:{}", result.found.line(), result.found.column())
                    } else {
                        format!("{}:{}", result.found.line(), result.found.column())
                    };
                    if ui
                        .add_sized(
                            [ui.available_width(), row_height],
                            egui::Button::selectable(
                                find.selected_match == index,
                                format!("{location}  {}", result.card),
                            ),
                        )
                        .clicked()
                    {
                        action = Some(FindWindowAction::Select(index));
                    }
                }
            });

        ui.separator();
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    has_matches,
                    egui::Button::new(messages.text(MessageId::NetlistFindPrevious)),
                )
                .clicked()
            {
                let next = if find.selected_match == 0 {
                    matches.len().saturating_sub(1)
                } else {
                    find.selected_match - 1
                };
                action = Some(FindWindowAction::Select(next));
            }
            let selected_owned = matches
                .get(find.selected_match)
                .and_then(|result| documents.get(result.document))
                .is_some_and(|document| document.editable);
            let replace_enabled = find.scope != NetlistFindScope::ProjectReferences
                && has_matches
                && selected_owned
                && find.error.is_none();
            if ui
                .add_enabled(
                    replace_enabled,
                    egui::Button::new(messages.text(MessageId::CommonReplace)),
                )
                .clicked()
            {
                action = Some(FindWindowAction::ReplaceNext);
            }
            if ui
                .add_enabled(
                    replace_enabled,
                    egui::Button::new(messages.text(MessageId::NetlistFindReplaceAll)),
                )
                .clicked()
            {
                action = Some(FindWindowAction::ReplaceAll);
            }
        });
        find_control_id
    });
    match choice {
        DialogChoice::Primary => {
            action = Some(FindWindowAction::Select(
                (find.selected_match + 1) % matches.len().max(1),
            ));
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => find.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }

    let all_owned_replacements = if matches!(action, Some(FindWindowAction::ReplaceAll))
        && find.scope == NetlistFindScope::AllOwnedSources
    {
        match plan_all_owned_replacements(&documents, &find.find, &find.replacement, options) {
            Ok(replacements) => Some(replacements),
            Err(error) => {
                find.error = Some(error.to_string());
                action = None;
                None
            }
        }
    } else {
        None
    };

    // The action is resolved against the borrowed scan before anything is
    // written, because opening a document replaces the very buffers these
    // matches were found in.
    let replacing = matches!(
        action,
        Some(FindWindowAction::ReplaceNext | FindWindowAction::ReplaceAll)
    );
    let target = match action {
        Some(FindWindowAction::Select(index)) => Some(index),
        Some(FindWindowAction::ReplaceNext | FindWindowAction::ReplaceAll) => {
            Some(find.selected_match)
        }
        None => None,
    }
    .and_then(|index| Some((index, matches.get(index)?)))
    .and_then(|(index, result)| {
        let document = documents.get(result.document)?;
        Some(FindTarget {
            index,
            active_document: document.active_document,
            dependency_identity: document.dependency_identity.map(str::to_owned),
            editable: document.editable,
            // Only a replacement needs the bytes, and a retained include can
            // be the size of a model library.
            source: replacing.then(|| document.source.to_owned()),
            line: result.found.line(),
            caret_byte: result.found.byte_range().start,
        })
    });
    if let (Some(FindWindowAction::Select(_)), Some(target)) = (action, target.as_ref()) {
        find.selected_match = target.index;
    }
    drop(matches);
    drop(documents);
    app.state.ui.netlist.find = find;

    if let Some(replacements) = all_owned_replacements {
        match crate::workbench::documents::netlist_document::replace_owned_sources_atomically(
            &mut app.state,
            replacements,
        ) {
            Ok(count) if count > 0 => {
                app.state.ui.netlist.find.selected_match = 0;
                let count_text = count.to_string();
                app.state
                    .push_user_message(ConsoleMessage::info(messages.format(
                        if count == 1 {
                            MessageId::NetlistFindReplacedSingular
                        } else {
                            MessageId::NetlistFindReplaced
                        },
                        &[("count", &count_text)],
                    )));
            }
            Ok(_) => {}
            Err(error) => {
                app.state.ui.netlist.find.error = Some(error.clone());
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        return;
    }

    let Some(target) = target else {
        return;
    };
    match action {
        Some(FindWindowAction::Select(_)) => {
            match target.active_document {
                ActiveNetlistDocument::Generated => {
                    let _ = crate::workbench::documents::netlist_document::open_generated_primary(
                        &mut app.state,
                    );
                }
                ActiveNetlistDocument::OwnedSource => {
                    let _ = open_owned_source(&mut app.state);
                }
                ActiveNetlistDocument::GeneratedDiff => {}
            }
            if let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency(
                        &mut app.state,
                        identity,
                    )
            {
                app.state.push_user_message(ConsoleMessage::error(error));
                return;
            }
            app.state.ui.netlist.requested_line = Some(target.line);
        }
        Some(FindWindowAction::ReplaceNext) | Some(FindWindowAction::ReplaceAll) => {
            let Some(source) = target.source.as_deref().filter(|_| target.editable) else {
                return;
            };
            if target.active_document == ActiveNetlistDocument::OwnedSource
                && app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
            {
                let _ = open_owned_source(&mut app.state);
            }
            if let Some(identity) = target.dependency_identity.as_deref()
                && let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency(
                        &mut app.state,
                        identity,
                    )
            {
                app.state.push_user_message(ConsoleMessage::error(error));
                return;
            }
            let scope = if matches!(action, Some(FindWindowAction::ReplaceAll)) {
                ReplaceScope::All
            } else {
                ReplaceScope::Next {
                    caret_byte: target.caret_byte,
                }
            };
            match replace_in_source(
                source,
                &app.state.ui.netlist.find.find,
                &app.state.ui.netlist.find.replacement,
                options,
                scope,
            ) {
                Ok(outcome) => {
                    let count = outcome.replacement_count();
                    let replacement = outcome.into_source();
                    let edit = if let Some(identity) = target.dependency_identity.as_deref() {
                        crate::workbench::documents::netlist_document::OwnedNetlistReplacement::dependency(
                            identity,
                            source,
                            replacement,
                            count,
                        )
                    } else {
                        crate::workbench::documents::netlist_document::OwnedNetlistReplacement::root(
                            source,
                            replacement,
                            count,
                        )
                    };
                    let replaced = if count == 0 {
                        false
                    } else {
                        match crate::workbench::documents::netlist_document::replace_owned_sources_atomically(
                            &mut app.state,
                            vec![edit],
                        ) {
                            Ok(_) => true,
                            Err(error) => {
                                app.state.ui.netlist.find.error = Some(error.clone());
                                app.state.push_user_message(ConsoleMessage::error(error));
                                false
                            }
                        }
                    };
                    if replaced {
                        app.state.ui.netlist.find.selected_match = 0;
                        let count_text = count.to_string();
                        app.state
                            .push_user_message(ConsoleMessage::info(messages.format(
                                if count == 1 {
                                    MessageId::NetlistFindReplacedSingular
                                } else {
                                    MessageId::NetlistFindReplaced
                                },
                                &[("count", &count_text)],
                            )));
                    }
                }
                Err(error) => app.state.ui.netlist.find.error = Some(error.to_string()),
            }
        }
        None => {}
    }
}

fn find_scope_combo(
    ui: &mut Ui,
    find: &mut crate::workbench::documents::netlist_document::NetlistFindState,
    has_owned_source: bool,
    messages: MessageCatalog,
) {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    egui::ComboBox::from_id_salt("rspice.code.find-scope")
        .selected_text(match find.scope {
            NetlistFindScope::CurrentDocument => {
                messages.text(MessageId::NetlistFindCurrentDocument)
            }
            NetlistFindScope::AllOwnedSources => {
                messages.text(MessageId::NetlistFindAllOwnedSources)
            }
            NetlistFindScope::ProjectReferences => {
                messages.text(MessageId::NetlistFindProjectReferences)
            }
        })
        .width(ui.available_width().max(180.0))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::CurrentDocument,
                messages.text(MessageId::NetlistFindCurrentDocument),
            );
            if has_owned_source {
                ui.selectable_value(
                    &mut find.scope,
                    NetlistFindScope::AllOwnedSources,
                    messages.text(MessageId::NetlistFindAllOwnedSources),
                );
            }
            ui.selectable_value(
                &mut find.scope,
                NetlistFindScope::ProjectReferences,
                messages.text(MessageId::NetlistFindProjectReferences),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_owned_replace_plans_root_and_every_editable_include() {
        let root = ".include \"models.lib\"\n.param gain=10\n.end\n";
        let include = ".param gain=20\n";
        let generated = ".param gain=30\n";
        let documents = vec![
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::OwnedSource,
                dependency_identity: None,
                editable: true,
                label: "top.cir".to_owned(),
                source: root,
            },
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::OwnedSource,
                dependency_identity: Some("models.lib"),
                editable: true,
                label: "models.lib".to_owned(),
                source: include,
            },
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::Generated,
                dependency_identity: None,
                editable: false,
                label: "generated.sp".to_owned(),
                source: generated,
            },
        ];

        let edits = plan_all_owned_replacements(
            &documents,
            "gain",
            "av",
            crate::state::FindOptions::default(),
        )
        .unwrap();

        assert_eq!(edits.len(), 2, "generated source remains find-only");
    }
}
