//! Find and replace across the netlist document set.
//!
//! A scope names a set of documents rather than one buffer, so a match always
//! carries the document it came from and whether that document is editable.
//! Read-only generated source is searchable and never replaceable.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::Ui;
use std::ops::Range;

use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize};
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::CodeRequest;
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, NetlistFindAction, NetlistFindStep, NetlistFindTarget,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::{MessageCatalog, MessageId};

/// How many matches the surface will hold at once.
///
/// A result list is read, not exhausted: a query that matches a hundred
/// thousand times in a flat deck is a query to narrow, and materializing every
/// hit to say so costs more than the search. The count reported alongside the
/// rows states plainly that it is a first page.
const NETLIST_FIND_RESULT_LIMIT: usize = 500;
const NETLIST_REPLACE_ALL_LIMIT: usize = 50_000;

/// One searchable document. The source is borrowed from application state:
/// the closure a project-references search covers is the whole include graph,
/// and copying it per frame copies the design.
#[derive(Debug, Clone)]
struct NetlistSearchDocument<'a> {
    active_document: ActiveNetlistDocument,
    /// `None` names the active top deck; a UUID names an inactive project deck.
    deck_id: Option<uuid::Uuid>,
    dependency_identity: Option<&'a str>,
    editable: bool,
    label: String,
    source: &'a str,
    search_byte_range: Option<Range<usize>>,
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

/// What a netlist search reads: the model, the netlist editor's state, the
/// include tabs open and the runs.
#[derive(Clone, Copy)]
pub(super) struct SearchSources<'a> {
    pub(super) model: &'a crate::workbench::app_state::Model,
    pub(super) netlist: &'a crate::workbench::documents::netlist_document::NetlistDocumentState,
    pub(super) open_documents:
        &'a std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    pub(super) runs: &'a rspice_app_runs::workbench::slots::runs::RunsState,
}

/// Matches for one query across one scope, bounded in count.
struct NetlistSearchResults {
    matches: Vec<NetlistSearchMatch>,
    /// Whether the deck holds more matches than the surface retained.
    truncated: bool,
    error: Option<crate::state::FindError>,
}

fn plan_scoped_replacements(
    documents: &[NetlistSearchDocument<'_>],
    query: &str,
    replacement: &str,
    options: crate::state::FindOptions,
    include_comments: bool,
) -> Result<
    Vec<crate::workbench::documents::netlist_document::OwnedNetlistReplacement>,
    crate::state::FindError,
> {
    use crate::state::replace_source_ranges;
    use crate::workbench::documents::netlist_document::OwnedNetlistReplacement;

    let mut edits = Vec::new();
    let mut remaining = NETLIST_REPLACE_ALL_LIMIT;
    for document in documents.iter().filter(|document| document.editable) {
        let found = find_document_matches(document, query, options, remaining, include_comments)?;
        let ranges = found
            .matches()
            .iter()
            .map(crate::state::FindMatch::byte_range)
            .collect::<Vec<_>>();
        if found.truncated() {
            return Err(crate::state::FindError::ReplacementLimitExceeded {
                limit: NETLIST_REPLACE_ALL_LIMIT,
            });
        }
        remaining -= ranges.len();
        let outcome =
            replace_source_ranges(document.source, query, replacement, options, &ranges, false)?;
        let replacement_count = outcome.replacement_count();
        let replaced_source = outcome.into_source();
        if replacement_count == 0 || replaced_source == document.source {
            continue;
        }
        edits.push(match (document.deck_id, document.dependency_identity) {
            (Some(deck_id), Some(identity)) => OwnedNetlistReplacement::retained_dependency(
                deck_id,
                identity,
                document.source,
                replaced_source,
                replacement_count,
            ),
            (Some(deck_id), None) => OwnedNetlistReplacement::retained_root(
                deck_id,
                document.source,
                replaced_source,
                replacement_count,
            ),
            (None, Some(identity)) => OwnedNetlistReplacement::dependency(
                identity,
                document.source,
                replaced_source,
                replacement_count,
            ),
            (None, None) => {
                OwnedNetlistReplacement::root(document.source, replaced_source, replacement_count)
            }
        });
    }
    Ok(edits)
}

fn netlist_search_documents(
    sources: SearchSources<'_>,
    scope: crate::workbench::documents::netlist_document::NetlistFindScope,
) -> Vec<NetlistSearchDocument<'_>> {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let generated = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::Generated,
        deck_id: None,
        dependency_identity: None,
        editable: false,
        label: "generated.sp".to_owned(),
        source: sources.model.netlist_artifacts.generated_source.as_str(),
        search_byte_range: None,
    };
    let owned = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::OwnedSource,
        deck_id: None,
        dependency_identity: None,
        editable: true,
        label: sources
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                sources
                    .model
                    .workspace
                    .content
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        source: sources
            .model
            .workspace
            .content
            .netlist_source
            .as_deref()
            .unwrap_or_default(),
        search_byte_range: None,
    };

    let current = || {
        if let Some(dependency) =
            crate::workbench::documents::netlist_document::active_dependency_in(
                sources.model,
                sources.netlist,
            )
        {
            NetlistSearchDocument {
                active_document: sources
                    .netlist
                    .active_dependency_root
                    .unwrap_or(sources.netlist.active_document),
                deck_id: None,
                dependency_identity: Some(dependency.locator().logical_identity()),
                editable:
                    crate::workbench::documents::netlist_document::active_dependency_is_owned_in(
                        sources.model,
                        sources.netlist,
                    ),
                label: dependency.locator().display_name().to_owned(),
                source: dependency.source().unwrap_or_default(),
                search_byte_range: None,
            }
        } else {
            match sources.netlist.active_document {
                ActiveNetlistDocument::Generated => generated(),
                ActiveNetlistDocument::OwnedSource => owned(),
                ActiveNetlistDocument::GeneratedDiff => NetlistSearchDocument {
                    active_document: ActiveNetlistDocument::GeneratedDiff,
                    deck_id: None,
                    dependency_identity: None,
                    editable: false,
                    label: "generated.diff".to_owned(),
                    source: sources.netlist.generated_diff_source.as_str(),
                    search_byte_range: None,
                },
                ActiveNetlistDocument::RunSnapshot => NetlistSearchDocument {
                    active_document: ActiveNetlistDocument::RunSnapshot,
                    deck_id: None,
                    dependency_identity: None,
                    editable: false,
                    label: crate::workbench::documents::netlist_document::run_deck_snapshot_artifact_name_in(
                        sources.runs,
                    ),
                    source: sources
                        .runs.manual_run
                        .last_run_buffer
                        .as_deref()
                        .unwrap_or_default(),
                    search_byte_range: None,
                },
            }
        }
    };

    match scope {
        NetlistFindScope::CurrentDocument => vec![current()],
        NetlistFindScope::Selection => {
            let mut document = current();
            document.search_byte_range = sources.netlist.selected_byte_range.clone().or(Some(0..0));
            vec![document]
        }
        NetlistFindScope::OpenDocuments => open_netlist_search_documents(sources),
        NetlistFindScope::ActiveLanguageProject => {
            let mut documents = sources
                .model
                .workspace
                .content
                .netlist_source
                .as_ref()
                .map(|_| vec![owned()])
                .unwrap_or_default();
            documents.extend(dependency_search_documents(
                sources,
                ActiveNetlistDocument::OwnedSource,
                false,
            ));
            documents.extend(retained_deck_search_documents(sources));
            if sources.netlist.find.include_generated_references {
                documents.push(generated());
                documents.extend(dependency_search_documents(
                    sources,
                    ActiveNetlistDocument::Generated,
                    false,
                ));
            }
            documents
        }
        NetlistFindScope::ProjectReferences => {
            let mut documents = Vec::new();
            if sources.netlist.find.include_generated_references
                && !sources.model.netlist_artifacts.generated_source.is_empty()
            {
                documents.push(generated());
                documents.extend(dependency_search_documents(
                    sources,
                    ActiveNetlistDocument::Generated,
                    false,
                ));
            }
            if sources.model.workspace.content.netlist_source.is_some() {
                documents.extend(dependency_search_documents(
                    sources,
                    ActiveNetlistDocument::OwnedSource,
                    false,
                ));
            }
            documents.extend(
                retained_deck_search_documents(sources)
                    .into_iter()
                    .filter(|document| document.dependency_identity.is_some()),
            );
            documents
        }
    }
}

fn open_netlist_search_documents(sources: SearchSources<'_>) -> Vec<NetlistSearchDocument<'_>> {
    use crate::workbench::state::WorkspaceDocumentId;

    let mut documents = Vec::new();
    if let Some(generated) = sources.model.netlist_artifacts.generated_document.as_ref() {
        documents.push(NetlistSearchDocument {
            active_document: ActiveNetlistDocument::Generated,
            deck_id: None,
            dependency_identity: None,
            editable: false,
            label: "generated.sp".to_owned(),
            source: sources.model.netlist_artifacts.generated_source.as_str(),
            search_byte_range: None,
        });
        if sources.netlist.find.include_generated_references {
            documents.extend(
                dependency_search_documents(sources, ActiveNetlistDocument::Generated, false)
                    .into_iter()
                    .filter(|dependency| {
                        sources
                            .open_documents
                            .contains(&WorkspaceDocumentId::NetlistDependency {
                                root: generated.id(),
                                logical_identity: dependency
                                    .dependency_identity
                                    .unwrap_or_default()
                                    .to_owned(),
                            })
                    }),
            );
        }
    }
    if let Some(owned) = sources.netlist.owned_document.as_ref() {
        documents.push(NetlistSearchDocument {
            active_document: ActiveNetlistDocument::OwnedSource,
            deck_id: None,
            dependency_identity: None,
            editable: true,
            label: sources
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_ref()
                .map(|descriptor| descriptor.artifact_name.clone())
                .unwrap_or_else(|| "owned-source.sp".to_owned()),
            source: sources
                .model
                .workspace
                .content
                .netlist_source
                .as_deref()
                .unwrap_or_default(),
            search_byte_range: None,
        });
        documents.extend(
            dependency_search_documents(sources, ActiveNetlistDocument::OwnedSource, false)
                .into_iter()
                .filter(|dependency| {
                    sources
                        .open_documents
                        .contains(&WorkspaceDocumentId::NetlistDependency {
                            root: owned.id(),
                            logical_identity: dependency
                                .dependency_identity
                                .unwrap_or_default()
                                .to_owned(),
                        })
                }),
        );
    }
    if !sources.netlist.generated_diff_source.is_empty() {
        documents.push(NetlistSearchDocument {
            active_document: ActiveNetlistDocument::GeneratedDiff,
            deck_id: None,
            dependency_identity: None,
            editable: false,
            label: "generated.diff".to_owned(),
            source: sources.netlist.generated_diff_source.as_str(),
            search_byte_range: None,
        });
    }
    documents
}

fn dependency_search_documents(
    sources: SearchSources<'_>,
    root: ActiveNetlistDocument,
    owned_only: bool,
) -> Vec<NetlistSearchDocument<'_>> {
    let document = match root {
        ActiveNetlistDocument::Generated => {
            sources.model.netlist_artifacts.generated_document.as_ref()
        }
        ActiveNetlistDocument::OwnedSource => sources.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    };
    let owned = sources.model.workspace.content.netlist_descriptor.as_ref();
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
                deck_id: None,
                dependency_identity: Some(identity),
                editable,
                label: dependency.locator().display_name().to_owned(),
                source,
                search_byte_range: None,
            })
        })
        .collect()
}

fn retained_deck_search_documents(sources: SearchSources<'_>) -> Vec<NetlistSearchDocument<'_>> {
    let mut documents = Vec::new();
    for deck in &sources.model.workspace.content.retained_netlist_decks {
        let artifact = deck.descriptor.artifact_name.as_str();
        documents.push(NetlistSearchDocument {
            active_document: ActiveNetlistDocument::OwnedSource,
            deck_id: Some(deck.descriptor.deck_id),
            dependency_identity: None,
            editable: true,
            label: artifact.to_owned(),
            source: deck.document.source(),
            search_byte_range: None,
        });
        documents.extend(
            deck.document
                .dependencies()
                .iter()
                .filter_map(|dependency| {
                    let source = dependency.source()?;
                    let identity = dependency.locator().logical_identity();
                    Some(NetlistSearchDocument {
                        active_document: ActiveNetlistDocument::OwnedSource,
                        deck_id: Some(deck.descriptor.deck_id),
                        dependency_identity: Some(identity),
                        editable: deck.descriptor.owned_include(identity).is_some(),
                        label: format!("{artifact} › {}", dependency.locator().display_name()),
                        source,
                        search_byte_range: None,
                    })
                }),
        );
    }
    documents
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
    include_comments: bool,
) -> NetlistSearchResults {
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
            results.truncated =
                find_document_matches(document, query, options, 1, include_comments)
                    .is_ok_and(|found| !found.matches().is_empty());
            if results.truncated {
                break;
            }
            continue;
        }
        let found =
            match find_document_matches(document, query, options, remaining, include_comments) {
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

fn find_document_matches(
    document: &NetlistSearchDocument<'_>,
    query: &str,
    options: crate::state::FindOptions,
    limit: usize,
    include_comments: bool,
) -> Result<crate::state::BoundedFindMatches, crate::state::FindError> {
    let range = document
        .search_byte_range
        .clone()
        .unwrap_or(0..document.source.len());
    crate::state::find_all_in_source_range_bounded_filter(
        document.source,
        range,
        query,
        options,
        limit,
        |found| include_comments || !source_offset_is_comment(document.source, found.start),
    )
}

fn source_offset_is_comment(source: &str, offset: usize) -> bool {
    let line_start = source[..offset.min(source.len())]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let line_end = source[offset.min(source.len())..]
        .find('\n')
        .map_or(source.len(), |index| offset.min(source.len()) + index);
    let line = &source[line_start..line_end];
    let match_in_line = offset.saturating_sub(line_start);
    let trimmed = line.trim_start();
    if trimmed.starts_with('*') || trimmed.starts_with(';') || trimmed.starts_with("//") {
        return true;
    }

    let mut single_quote = false;
    let mut double_quote = false;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if index >= match_in_line {
            break;
        }
        if escaped {
            escaped = false;
            continue;
        }
        match character {
            '\\' if single_quote || double_quote => escaped = true,
            '\'' if !double_quote => single_quote = !single_quote,
            '"' if !single_quote => double_quote = !double_quote,
            ';' | '$' if !single_quote && !double_quote => return true,
            _ => {}
        }
    }
    if !single_quote
        && !double_quote
        && line
            .get(match_in_line..)
            .is_some_and(|tail| tail.starts_with(';') || tail.starts_with('$'))
    {
        return true;
    }
    false
}

pub(super) fn find_replace_window(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.find.open {
        return;
    }

    use crate::state::{FindDirection, FindOptions};
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let messages = cx.wb.messages();
    let mut find = cx.state.netlist.find.clone();
    let options = FindOptions {
        direction: FindDirection::Forward,
        match_case: find.match_case,
        whole_word: find.whole_symbol,
        regular_expression: find.regular_expression,
    };
    let documents = netlist_search_documents(
        SearchSources {
            model: cx.model,
            netlist: &cx.state.netlist,
            open_documents: &cx.state.netlist_open_documents,
            runs: cx.runs,
        },
        find.scope,
    );
    let found = scan_documents(&documents, &find.find, options, find.include_comments);
    find.error = if find.scope == NetlistFindScope::Selection
        && cx.state.netlist.selected_byte_range.is_none()
    {
        Some(messages.text(MessageId::NetlistFindSelectionRequired))
    } else {
        found.error.as_ref().map(ToString::to_string)
    };
    let truncated = found.truncated;
    let matches = found.matches;
    if matches.is_empty() {
        find.selected_match = 0;
    } else {
        find.selected_match = find.selected_match.min(matches.len() - 1);
    }

    let mut action = None;
    let has_matches = !matches.is_empty();
    let scope_replaceable = find.scope != NetlistFindScope::ProjectReferences
        && documents.iter().any(|document| document.editable);
    let compact_fields = ctx.content_rect().width() < 360.0;
    // One status line, in the footer beside the actions: why Find next is
    // off, or how many matches it walks.
    let (status, tone) = if let Some(error) = &find.error {
        (error.clone(), DialogHintTone::Error)
    } else if find.find.is_empty() {
        (
            messages.text(MessageId::NetlistFindEnterTextHint),
            DialogHintTone::Neutral,
        )
    } else {
        let count = matches.len().to_string();
        (
            messages.format(
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
            ),
            DialogHintTone::Neutral,
        )
    };
    // A scope with nothing editable in it, such as the generated deck, is
    // searched, never replaced in: it has no Replace field or actions.
    let title = if scope_replaceable {
        MessageId::NetlistFindTitle
    } else {
        MessageId::CodeSearchTitle
    };
    let choice = Dialog::new(
        messages.text(title),
        messages.text(MessageId::NetlistFindNext),
    )
    .description(messages.text(MessageId::NetlistFindDescription))
    .size(DialogSize::Transaction)
    // The fields end where the footer's buttons do, not a bar's width in.
    .overlay_scrollbar()
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(has_matches && find.error.is_none())
    .secondary(messages.text(MessageId::NetlistFindPrevious))
    .secondary_enabled(has_matches && find.error.is_none())
    .ghost(messages.text(MessageId::CommonClose))
    .hint(status)
    .hint_tone(tone)
    .show_with_initial_body_focus(ctx, |ui| {
        // A field is a control's height, as tall as the Scope select beside
        // it and a finger's target in a touch shell.
        let field_size = |ui: &Ui| {
            egui::vec2(
                ui.available_width().max(180.0),
                crate::ui::tokens::Tokens::get(ui.ctx()).metrics.ctl_h,
            )
        };
        let mut find_control_id = None;
        if compact_fields {
            ui.label(messages.text(MessageId::CommonFind));
            let response = ui.add_sized(
                field_size(ui),
                egui::TextEdit::singleline(&mut find.find)
                    .vertical_align(egui::Align::Center)
                    .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
            );
            find_control_id = Some(response.id);
            if scope_replaceable {
                ui.label(messages.text(MessageId::CommonReplace));
                ui.add_sized(
                    field_size(ui),
                    egui::TextEdit::singleline(&mut find.replacement)
                        .vertical_align(egui::Align::Center),
                );
            }
            ui.label(messages.text(MessageId::CommonScope));
            find_scope_select(ui, &mut find, messages);
            ui.horizontal_wrapped(|ui| find_options(ui, &mut find, messages));
        } else {
            egui::Grid::new("rspice.code.find-fields")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label(messages.text(MessageId::CommonFind));
                    let response = ui.add_sized(
                        field_size(ui),
                        egui::TextEdit::singleline(&mut find.find)
                            .vertical_align(egui::Align::Center)
                            .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
                    );
                    find_control_id = Some(response.id);
                    ui.end_row();
                    if scope_replaceable {
                        ui.label(messages.text(MessageId::CommonReplace));
                        ui.add_sized(
                            field_size(ui),
                            egui::TextEdit::singleline(&mut find.replacement)
                                .vertical_align(egui::Align::Center),
                        );
                        ui.end_row();
                    }
                    ui.label(messages.text(MessageId::CommonScope));
                    find_scope_select(ui, &mut find, messages);
                    ui.end_row();
                    // The options qualify the fields above them, so they
                    // start at the fields' edge, not under the labels.
                    ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover());
                    ui.horizontal_wrapped(|ui| find_options(ui, &mut find, messages));
                    ui.end_row();
                });
        }

        if !has_matches {
            return find_control_id;
        }
        ui.separator();
        let row_height = 24.0;
        egui::ScrollArea::vertical()
            .id_salt("rspice.code.find-results")
            .auto_shrink([false, true])
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
                        action = Some(NetlistFindAction::Select(index));
                    }
                }
            });

        if !scope_replaceable {
            return find_control_id;
        }
        ui.separator();
        ui.horizontal(|ui| {
            let selected_owned = matches
                .get(find.selected_match)
                .and_then(|result| documents.get(result.document))
                .is_some_and(|document| document.editable);
            let replace_enabled = find.scope != NetlistFindScope::ProjectReferences
                && has_matches
                && selected_owned
                && find.error.is_none();
            let replace_all_enabled = find.scope != NetlistFindScope::ProjectReferences
                && has_matches
                && scope_replaceable
                && find.error.is_none();
            if ui
                .add_enabled(
                    replace_enabled,
                    egui::Button::new(messages.text(MessageId::CommonReplace)),
                )
                .clicked()
            {
                action = Some(NetlistFindAction::ReplaceNext);
            }
            if ui
                .add_enabled(
                    replace_all_enabled,
                    egui::Button::new(messages.text(MessageId::NetlistFindReplaceAll)),
                )
                .clicked()
            {
                action = Some(NetlistFindAction::ReplaceAll);
            }
        });
        find_control_id
    });
    match choice {
        DialogChoice::Primary => {
            action = Some(NetlistFindAction::Select(
                (find.selected_match + 1) % matches.len().max(1),
            ));
        }
        DialogChoice::Secondary => {
            action = Some(NetlistFindAction::Select(if find.selected_match == 0 {
                matches.len().saturating_sub(1)
            } else {
                find.selected_match - 1
            }));
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => find.open = false,
        DialogChoice::None => {}
    }

    let scoped_replacements = if matches!(action, Some(NetlistFindAction::ReplaceAll)) {
        match plan_scoped_replacements(
            &documents,
            &find.find,
            &find.replacement,
            options,
            find.include_comments,
        ) {
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
        Some(NetlistFindAction::ReplaceNext | NetlistFindAction::ReplaceAll)
    );
    let target = match action {
        Some(NetlistFindAction::Select(index)) => Some(index),
        Some(NetlistFindAction::ReplaceNext | NetlistFindAction::ReplaceAll) => {
            Some(find.selected_match)
        }
        None => None,
    }
    .and_then(|index| Some((index, matches.get(index)?)))
    .and_then(|(index, result)| {
        let document = documents.get(result.document)?;
        Some(NetlistFindTarget {
            index,
            active_document: document.active_document,
            deck_id: document.deck_id,
            dependency_identity: document.dependency_identity.map(str::to_owned),
            editable: document.editable,
            // Only a replacement needs the bytes, and a retained include can
            // be the size of a model library.
            source: replacing.then(|| document.source.to_owned()),
            line: result.found.line(),
            byte_range: result.found.byte_range(),
        })
    });
    if let (Some(NetlistFindAction::Select(_)), Some(target)) = (action, target.as_ref()) {
        find.selected_match = target.index;
    }
    drop(matches);
    drop(documents);
    cx.state.netlist.find = find;
    // Going to a match opens its document, and a replacement commits through
    // a candidate copy of the whole state, so the shell carries the press out
    // once the page has drawn.
    if scoped_replacements.is_some() || target.is_some() {
        cx.state.netlist.find.pending = Some(NetlistFindStep {
            action,
            options,
            target,
            replacements: scoped_replacements,
        });
        cx.wb.outbox.push(CodeRequest::ApplyNetlistFind);
    }
}

/// The scope as the kit's select, in the order the search widens.
/// The search's options, in the order they narrow it.
fn find_options(
    ui: &mut Ui,
    find: &mut crate::workbench::documents::netlist_document::NetlistFindState,
    messages: MessageCatalog,
) {
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
    ui.checkbox(
        &mut find.include_comments,
        messages.text(MessageId::NetlistFindIncludeComments),
    );
    ui.checkbox(
        &mut find.include_generated_references,
        messages.text(MessageId::NetlistFindIncludeGeneratedReferences),
    );
}

fn find_scope_select(
    ui: &mut Ui,
    find: &mut crate::workbench::documents::netlist_document::NetlistFindState,
    messages: MessageCatalog,
) {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    const SCOPES: [(NetlistFindScope, MessageId); 5] = [
        (
            NetlistFindScope::CurrentDocument,
            MessageId::NetlistFindCurrentDocument,
        ),
        (NetlistFindScope::Selection, MessageId::NetlistFindSelection),
        (
            NetlistFindScope::OpenDocuments,
            MessageId::NetlistFindOpenDocuments,
        ),
        (
            NetlistFindScope::ActiveLanguageProject,
            MessageId::NetlistFindActiveLanguageProject,
        ),
        (
            NetlistFindScope::ProjectReferences,
            MessageId::NetlistFindProjectReferences,
        ),
    ];
    let labels = SCOPES.map(|(_, label)| messages.text(label));
    let current = SCOPES
        .iter()
        .position(|(scope, _)| *scope == find.scope)
        .unwrap_or_default();
    if let Some(picked) = crate::ui::widgets::select(
        ui,
        "rspice.code.find-scope",
        &messages.text(MessageId::CommonScope),
        &labels[current],
        &labels,
        ui.available_width().max(180.0),
    ) && let Some((scope, _)) = SCOPES.get(picked)
    {
        find.scope = *scope;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;

    #[test]
    fn all_owned_replace_plans_root_and_every_editable_include() {
        let root = ".include \"models.lib\"\n.param gain=10\n.end\n";
        let include = ".param gain=20\n";
        let generated = ".param gain=30\n";
        let documents = vec![
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::OwnedSource,
                deck_id: None,
                dependency_identity: None,
                editable: true,
                label: "top.cir".to_owned(),
                source: root,
                search_byte_range: None,
            },
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::OwnedSource,
                deck_id: None,
                dependency_identity: Some("models.lib"),
                editable: true,
                label: "models.lib".to_owned(),
                source: include,
                search_byte_range: None,
            },
            NetlistSearchDocument {
                active_document: ActiveNetlistDocument::Generated,
                deck_id: None,
                dependency_identity: None,
                editable: false,
                label: "generated.sp".to_owned(),
                source: generated,
                search_byte_range: None,
            },
        ];

        let edits = plan_scoped_replacements(
            &documents,
            "gain",
            "av",
            crate::state::FindOptions::default(),
            true,
        )
        .unwrap();

        assert_eq!(edits.len(), 2, "generated source remains find-only");
    }

    #[test]
    fn scoped_scan_honors_selection_and_comment_options() {
        let source = "* gain comment\n.param gain=10 gain=20 ; gain ignored\n";
        let selection_start = source.find("gain=10").unwrap();
        let selection_end = selection_start + "gain=10".len();
        let documents = vec![NetlistSearchDocument {
            active_document: ActiveNetlistDocument::OwnedSource,
            deck_id: None,
            dependency_identity: None,
            editable: true,
            label: "top.cir".to_owned(),
            source,
            search_byte_range: Some(selection_start..selection_end),
        }];

        let selected = scan_documents(
            &documents,
            "gain",
            crate::state::FindOptions::default(),
            false,
        );
        assert_eq!(selected.matches.len(), 1);

        let whole_document = [NetlistSearchDocument {
            search_byte_range: None,
            ..documents[0].clone()
        }];
        let without_comments = scan_documents(
            &whole_document,
            "gain",
            crate::state::FindOptions::default(),
            false,
        );
        assert_eq!(without_comments.matches.len(), 2);

        let with_comments = scan_documents(
            &whole_document,
            "gain",
            crate::state::FindOptions::default(),
            true,
        );
        assert_eq!(with_comments.matches.len(), 4);
    }

    /// What the find dialog names, by accessible label or value, after a few
    /// frames over `app`.
    fn find_dialog_names(app: &mut RSpiceApp) -> Vec<String> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ui| find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(app)),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        nodes
            .values()
            .flat_map(|node| [node.label(), node.value()])
            .flatten()
            .map(str::to_owned)
            .collect()
    }

    /// The generated deck is read-only, so finding in it offers no Replace
    /// field and no Replace actions, and the dialog is called Find; an owned
    /// deck is replaced in. The dialog used to show a disabled Replace field
    /// and disabled Replace buttons over the generated deck, and three status
    /// lines: a footer hint, a body line and the error.
    #[test]
    fn a_read_only_deck_is_found_in_but_offers_no_replace() {
        let mut app = RSpiceApp::test_instance();
        app.state.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        app.state.code.netlist.find.open = true;
        let names = find_dialog_names(&mut app);
        let says = |text: &str| names.iter().any(|name| name == text);
        assert!(says("Find in source"), "{names:?}");
        assert!(says("Find"), "the Find field is missing");
        assert!(!says("Replace"), "a read-only deck offers Replace");
        assert!(!says("Replace all"), "a read-only deck offers Replace all");
        assert!(
            !says("Enter text to search the current document."),
            "a second status line in the body"
        );

        crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
            &mut app.state,
            crate::workbench::documents::code_workspace::CodeSourceFileAction::New,
        )
        .unwrap();
        crate::workbench::documents::netlist_document::commit_netlist_lifecycle_action(
            &mut app.state,
        )
        .unwrap();
        let names = find_dialog_names(&mut app);
        let says = |text: &str| names.iter().any(|name| name == text);
        assert!(says("Find and replace in source"), "{names:?}");
        assert!(says("Replace"), "an owned deck offers no Replace");
    }

    /// The Find field is a control's height like the Scope select beside it,
    /// a finger's target in a touch shell. It was egui's default single line,
    /// shorter than the select at the desktop size and half its height on a
    /// tablet.
    #[test]
    fn the_find_field_is_as_tall_as_the_scope_select() {
        use egui::accesskit::Role;

        for coarse in [false, true] {
            let mut app = RSpiceApp::test_instance();
            app.state.code.code_workspace.page =
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
            app.state.code.netlist.find.open = true;
            let ctx = egui::Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            crate::ui::Theme::default().apply_responsive_metrics_with_target(
                &ctx,
                coarse.then_some(crate::ui::tokens::TOUCH_TARGET),
            );
            let mut nodes = std::collections::HashMap::new();
            for _ in 0..3 {
                let output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1024.0, 768.0),
                        )),
                        ..egui::RawInput::default()
                    },
                    |ui| {
                        find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(&mut app))
                    },
                );
                if let Some(update) = output.platform_output.accesskit_update {
                    nodes.extend(update.nodes);
                }
            }
            let height = |role: Role| {
                nodes
                    .values()
                    .find(|node| node.role() == role)
                    .and_then(|node| node.bounds())
                    .map(|bounds| bounds.y1 - bounds.y0)
                    .unwrap_or_else(|| panic!("no {role:?}"))
            };
            let (field, select) = (height(Role::TextInput), height(Role::ComboBox));
            assert!(
                (field - select).abs() <= 1.0,
                "touch {coarse}: the field is {field} tall and the select {select}"
            );
        }
    }

    /// The options start at the fields' left edge, under the Find field, not
    /// under the labels beside it.
    #[test]
    fn the_options_start_where_the_fields_do() {
        use egui::accesskit::Role;

        let mut app = RSpiceApp::test_instance();
        app.state.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        app.state.code.netlist.find.open = true;
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ui| find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(&mut app)),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let lefts = |role: Role| {
            nodes
                .values()
                .filter(|node| node.role() == role)
                .filter_map(|node| node.bounds())
                .map(|bounds| bounds.x0)
                .collect::<Vec<_>>()
        };
        let field = lefts(Role::TextInput)[0];
        let checks = lefts(Role::CheckBox);
        assert_eq!(checks.len(), 5, "{checks:?}");
        let first = checks.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(
            (first - field).abs() < 1.0,
            "the options start at {first}, the field at {field}"
        );
    }
}
