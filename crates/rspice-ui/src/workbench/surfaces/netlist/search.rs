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

#[derive(Debug, Clone, Copy)]
enum FindWindowAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

#[derive(Debug, Clone)]
struct NetlistSearchDocument {
    active_document: ActiveNetlistDocument,
    dependency_identity: Option<String>,
    editable: bool,
    label: String,
    source: String,
}

#[derive(Debug, Clone)]
struct NetlistSearchMatch {
    document: NetlistSearchDocument,
    found: crate::state::FindMatch,
}

fn netlist_search_documents(
    state: &AppState,
    scope: crate::workbench::documents::netlist_document::NetlistFindScope,
) -> Vec<NetlistSearchDocument> {
    use crate::workbench::documents::netlist_document::NetlistFindScope;

    let generated = || NetlistSearchDocument {
        active_document: ActiveNetlistDocument::Generated,
        dependency_identity: None,
        editable: false,
        label: "generated.sp".to_owned(),
        source: state.ui.netlist.generated_source.clone(),
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
        source: state.workspace.netlist_source.clone().unwrap_or_default(),
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
                    dependency_identity: Some(dependency.locator().logical_identity().to_owned()),
                    editable:
                        crate::workbench::documents::netlist_document::active_dependency_is_owned(
                            state,
                        ),
                    label: dependency.locator().display_name().to_owned(),
                    source: dependency.source().unwrap_or_default().to_owned(),
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
                        source: state.ui.netlist.generated_diff_source.clone(),
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
) -> Vec<NetlistSearchDocument> {
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
                dependency_identity: Some(identity.to_owned()),
                editable,
                label: dependency.locator().display_name().to_owned(),
                source: source.to_owned(),
            })
        })
        .collect()
}

pub(super) fn find_replace_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.find.open {
        return;
    }

    use crate::state::{
        FindDirection, FindOptions, ReplaceScope, find_all_in_source, replace_in_source,
    };
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
    let matches: Result<Vec<NetlistSearchMatch>, crate::state::FindError> = if find.find.is_empty()
    {
        Ok(Vec::new())
    } else {
        documents
            .iter()
            .try_fold(Vec::<NetlistSearchMatch>::new(), |mut all, document| {
                all.extend(
                    find_all_in_source(&document.source, &find.find, options)?
                        .into_iter()
                        .map(|found| NetlistSearchMatch {
                            document: document.clone(),
                            found,
                        }),
                );
                Ok(all)
            })
    };
    find.error = matches.as_ref().err().map(ToString::to_string);
    let matches = matches.unwrap_or_default();
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
                if matches.len() == 1 {
                    MessageId::NetlistFindMatchSingular
                } else {
                    MessageId::NetlistFindMatches
                },
                &[("count", &count)],
            ));
        }

        egui::ScrollArea::vertical()
            .id_salt("rspice.code.find-results")
            .max_height(168.0)
            .show(ui, |ui| {
                let show_document =
                    documents.len() > 1 || find.scope != NetlistFindScope::CurrentDocument;
                for (index, result) in matches.iter().enumerate() {
                    let line_text = result
                        .document
                        .source
                        .lines()
                        .nth(result.found.line().saturating_sub(1))
                        .unwrap_or_default()
                        .trim();
                    let location = if show_document {
                        format!(
                            "{}  {}:{}",
                            result.document.label,
                            result.found.line(),
                            result.found.column()
                        )
                    } else {
                        format!("{}:{}", result.found.line(), result.found.column())
                    };
                    if ui
                        .add_sized(
                            [ui.available_width(), 24.0],
                            egui::Button::selectable(
                                find.selected_match == index,
                                format!("{location}  {line_text}"),
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
                .is_some_and(|result| result.document.editable);
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
    app.state.ui.netlist.find = find;

    match action {
        Some(FindWindowAction::Select(index)) => {
            if let Some(result) = matches.get(index) {
                app.state.ui.netlist.find.selected_match = index;
                match result.document.active_document {
                    ActiveNetlistDocument::Generated => {
                        let _ =
                            crate::workbench::documents::netlist_document::open_generated_primary(
                                &mut app.state,
                            );
                    }
                    ActiveNetlistDocument::OwnedSource => {
                        let _ = open_owned_source(&mut app.state);
                    }
                    ActiveNetlistDocument::GeneratedDiff => {}
                }
                if let Some(identity) = result.document.dependency_identity.as_deref()
                    && let Err(error) =
                        crate::workbench::documents::netlist_document::open_netlist_dependency(
                            &mut app.state,
                            identity,
                        )
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
                app.state.ui.netlist.requested_line = Some(result.found.line());
            }
        }
        Some(FindWindowAction::ReplaceNext) | Some(FindWindowAction::ReplaceAll) => {
            let selected = matches.get(app.state.ui.netlist.find.selected_match);
            let Some(selected) = selected.filter(|result| result.document.editable) else {
                return;
            };
            if selected.document.active_document == ActiveNetlistDocument::OwnedSource
                && app.state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
            {
                let _ = open_owned_source(&mut app.state);
            }
            if let Some(identity) = selected.document.dependency_identity.as_deref()
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
                    caret_byte: selected.found.byte_range().start,
                }
            };
            match replace_in_source(
                &selected.document.source,
                &app.state.ui.netlist.find.find,
                &app.state.ui.netlist.find.replacement,
                options,
                scope,
            ) {
                Ok(outcome) => {
                    let count = outcome.replacement_count();
                    let replacement = outcome.into_source();
                    let replaced = if selected.document.dependency_identity.is_some() {
                        crate::workbench::documents::netlist_document::replace_owned_dependency_source(
                            &mut app.state,
                            replacement,
                        )
                    } else {
                        crate::workbench::documents::netlist_document::replace_owned_source(
                            &mut app.state,
                            replacement,
                        )
                    };
                    if count > 0 && replaced {
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
