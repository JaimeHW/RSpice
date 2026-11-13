//! Revision-bound find and replace across a project source bundle.
//!
//! The search itself, its comment masking, its bounded streaming, and its
//! atomic replacement were all implemented and reachable only from tests. This
//! is the surface for them.
//!
//! Two limits are stated rather than hidden. The bundle streams up to
//! [`SOURCE_SEARCH_STREAM_LIMIT`] matches and displays the first
//! [`SOURCE_SEARCH_RESULT_LIMIT`]; a replace that would exceed the stream
//! limit is refused instead of applying part of itself.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, ScrollArea, Ui};

use crate::state::ProjectSourceLanguage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::design_system::inspector::muted_inspector_copy;
use crate::workbench::documents::code_workspace::{
    self, CodeSourceSearchResults, CodeSourceSearchScope, CodeSourceSearchState,
    CodeWorkspaceRuntimeState, SOURCE_SEARCH_RESULT_LIMIT,
};
use crate::workbench::documents::netlist_document::NetlistDocumentState;
use crate::workbench::{MessageCatalog, MessageId};

/// The scopes a bundle search can run in, in the order the dialog offers them.
///
/// `Selection` is offered only when the editor actually captured one; the
/// search refuses it otherwise, and an option that always fails is worse than
/// no option.
///
/// A scope names itself through the catalog rather than through the state
/// type: the picker writes back by matching the label it displayed, so a
/// translated label has to be the one the round-trip is built on.
const SCOPES: [(CodeSourceSearchScope, MessageId); 4] = [
    (
        CodeSourceSearchScope::Selection,
        MessageId::CodeSearchSelection,
    ),
    (
        CodeSourceSearchScope::CurrentDocument,
        MessageId::CodeSearchCurrentDocument,
    ),
    (
        CodeSourceSearchScope::OpenDocuments,
        MessageId::CodeSearchOpenDocuments,
    ),
    (
        CodeSourceSearchScope::ActiveLanguageProject,
        MessageId::CodeSearchActiveLanguage,
    ),
];

fn scope_label(messages: MessageCatalog, scope: CodeSourceSearchScope) -> String {
    messages.text(
        SCOPES
            .iter()
            .find(|(candidate, _)| *candidate == scope)
            .map_or(MessageId::CodeSearchActiveLanguage, |(_, id)| *id),
    )
}

/// The height of one match in the results list.
const RESULT_ROW_HEIGHT: f32 = 24.0;
/// How tall the results list grows before it scrolls.
const RESULT_LIST_MAX_HEIGHT: f32 = 168.0;
/// Below this width the labels stack over their fields.
const COMPACT_FIELDS_WIDTH: f32 = 360.0;
/// The label column of every kit form, Go to line's among them: its
/// fields start 100 pt in, past this column and the 8 pt gap after it.
pub(crate) const LABEL_COLUMN: f32 = 92.0;
pub(crate) const LABEL_GAP: f32 = 8.0;

/// A field's label as the kit's forms state theirs: the secondary size in
/// the dimmed ink, so the values beside it carry the weight.
fn field_label(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

/// What the body asked for, applied after rendering.
enum FindAction {
    None,
    Replace,
    Reveal(String, usize),
}

/// The footer's one status line: why the search failed, what it needs, or
/// what it found.
fn find_status(
    messages: MessageCatalog,
    search: &CodeSourceSearchState,
    failure: Option<&str>,
    blocked: Option<&str>,
    found: &CodeSourceSearchResults,
    replaceable: bool,
) -> (String, DialogHintTone) {
    if let Some(error) = failure {
        return (error.to_owned(), DialogHintTone::Error);
    }
    if search.query.is_empty() {
        // Before anything is found, say why there is no Replace when the
        // bundle's files are held.
        return match blocked {
            Some(reason) => (reason.to_owned(), DialogHintTone::Warn),
            None => (
                messages.text(MessageId::NetlistFindEnterTextHint),
                DialogHintTone::Neutral,
            ),
        };
    }
    if found.total_matches == 0 {
        return (
            messages.text(MessageId::CodeSearchNoMatches),
            DialogHintTone::Neutral,
        );
    }
    let count = found.total_matches.to_string();
    let text = if replaceable && found.read_only_matches > 0 {
        // Replace all skips these, so the count it acts on is not the total.
        messages.format(
            MessageId::CodeSearchMatchesReadOnly,
            &[
                ("count", &count),
                ("read_only", &found.read_only_matches.to_string()),
            ],
        )
    } else {
        messages.format(
            if found.total_matches == 1 {
                MessageId::CodeSearchMatchSingular
            } else {
                MessageId::CodeSearchMatches
            },
            &[("count", &count)],
        )
    };
    (text, DialogHintTone::Neutral)
}

/// Draw Find in source while a search is open, over the Code workspace's
/// runtime state, the netlist editor's state, the model and the platform's
/// state. Returns `true` when the reader asked to replace every match, which
/// writes the bundle past these parts: the search stays open for it.
pub(in crate::workbench) fn render_source_find_dialog(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    model: &Model,
    platform: &PlatformState,
    messages: MessageCatalog,
    ctx: &Context,
) -> bool {
    if code.source_search.is_none() {
        crate::ui::input::InputScope::discard(ctx, code_workspace::source_search_query_id());
        return false;
    }
    let _opening_input =
        crate::ui::input::InputScope::enter(ctx, code_workspace::source_search_query_id());
    // Read the search once, run it once, and render from that. Re-running
    // per part would let two parts of one dialog state different counts.
    let Some(search) = code.source_search.clone() else {
        return false;
    };
    let results = code_workspace::source_search_results_in(model, code, netlist, platform, &search);
    let blocked = code_workspace::source_file_mutation_block_reason_in(
        model,
        code,
        platform,
        search.language,
    );
    let replaceable = blocked.is_none()
        && code_workspace::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
            search.language,
            search.bundle_id,
            &search.active_path,
        );

    let failure = search
        .error
        .clone()
        .or_else(|| results.as_ref().err().cloned());
    let found = results.unwrap_or_default();
    let (status, tone) = find_status(
        messages,
        &search,
        failure.as_deref(),
        blocked,
        &found,
        replaceable,
    );
    // A document nothing may be written to is searched, never replaced
    // in: it has no Replace field and no Replace all, and Close is all
    // its footer offers.
    let (title, primary) = if replaceable {
        (
            MessageId::NetlistFindTitle,
            MessageId::NetlistFindReplaceAll,
        )
    } else {
        (MessageId::CodeSearchTitle, MessageId::CommonClose)
    };
    let mut dialog = Dialog::new(messages.text(title), messages.text(primary))
        .description(messages.text(MessageId::CodeSearchDescription))
        .size(DialogSize::Transaction)
        // The fields end where the footer's buttons do, not a bar's width in.
        .overlay_scrollbar()
        // Enter in the Find field searches as it types; it must never
        // replace every match.
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::Control(
            code_workspace::source_search_query_id(),
        ))
        .hint(status)
        .hint_tone(tone);
    if replaceable {
        dialog = dialog
            .primary_enabled(
                failure.is_none() && !search.query.is_empty() && found.editable_matches > 0,
            )
            .ghost(messages.text(MessageId::CommonClose));
    }

    let mut action = FindAction::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        action = find_body(ui, code, messages, &found, replaceable);
        Some(code_workspace::source_search_query_id())
    });
    let close = match choice {
        DialogChoice::Primary if replaceable => {
            action = FindAction::Replace;
            false
        }
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => true,
        DialogChoice::Secondary | DialogChoice::None => false,
    };
    let replace = apply_find_action(code, action, ctx);
    if close {
        code.source_search = None;
    }
    replace
}

/// Apply what the body asked for over the Code workspace's runtime state.
/// Returns `true` for Replace all, which the shell carries out.
fn apply_find_action(
    code: &mut CodeWorkspaceRuntimeState,
    action: FindAction,
    ctx: &Context,
) -> bool {
    match action {
        FindAction::None => false,
        FindAction::Replace => true,
        // A result row is a document plus an exact line, so following one
        // has to select that document before revealing the line in it.
        FindAction::Reveal(logical_path, line) => {
            let Some(search) = code.source_search.as_ref() else {
                return false;
            };
            let (language, bundle_id) = (search.language, search.bundle_id);
            match language {
                ProjectSourceLanguage::VerilogA => {
                    code.veriloga.selected_file = Some(code_workspace::VerilogAFileSelection {
                        bundle_id,
                        logical_path: logical_path.clone(),
                    });
                }
                ProjectSourceLanguage::RSpiceAutomation => {
                    code.automation.selected_file = Some(logical_path.clone());
                }
            }
            crate::workbench::documents::text_editor_commands::queue_reveal_line(
                ctx,
                code_workspace::source_editor_id(language, bundle_id, &logical_path),
                line,
            );
            if let Some(search) = code.source_search.as_mut() {
                search.active_path = logical_path;
            }
            false
        }
    }
}

/// Find, Replace when the document takes one, and Scope on the label grid,
/// the options under them, and the matches under a rule once there are any.
fn find_body(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
    found: &CodeSourceSearchResults,
    replaceable: bool,
) -> FindAction {
    let Some(search) = code.source_search.as_mut() else {
        return FindAction::None;
    };
    let has_selection = search.selection_char_range.is_some();
    let compact_fields = ui.ctx().content_rect().width() < COMPACT_FIELDS_WIDTH;
    // A field is a control's height, as tall as the Scope select beside it
    // and a finger's target in a touch shell.
    let field_size = |ui: &Ui| {
        egui::vec2(
            ui.available_width().max(180.0),
            Tokens::get(ui.ctx()).metrics.ctl_h,
        )
    };
    let find_field = |ui: &mut Ui, query: &mut String| {
        ui.add_sized(
            field_size(ui),
            egui::TextEdit::singleline(query)
                .id(code_workspace::source_search_query_id())
                .vertical_align(egui::Align::Center)
                .hint_text(messages.text(MessageId::NetlistFindExpressionHint)),
        );
    };
    let replace_field = |ui: &mut Ui, replacement: &mut String| {
        ui.add_sized(
            field_size(ui),
            egui::TextEdit::singleline(replacement).vertical_align(egui::Align::Center),
        );
    };
    if compact_fields {
        field_label(ui, &messages.text(MessageId::CommonFind));
        find_field(ui, &mut search.query);
        if replaceable {
            field_label(ui, &messages.text(MessageId::CommonReplace));
            replace_field(ui, &mut search.replacement);
        }
        field_label(ui, &messages.text(MessageId::CommonScope));
        scope_select(ui, search, has_selection, messages);
    } else {
        egui::Grid::new("source-find-fields")
            .num_columns(2)
            .spacing([LABEL_GAP, 8.0])
            .min_col_width(LABEL_COLUMN)
            .show(ui, |ui| {
                field_label(ui, &messages.text(MessageId::CommonFind));
                find_field(ui, &mut search.query);
                ui.end_row();
                if replaceable {
                    field_label(ui, &messages.text(MessageId::CommonReplace));
                    replace_field(ui, &mut search.replacement);
                    ui.end_row();
                }
                field_label(ui, &messages.text(MessageId::CommonScope));
                scope_select(ui, search, has_selection, messages);
                ui.end_row();
            });
    }
    ui.horizontal_wrapped(|ui| {
        ui.checkbox(
            &mut search.match_case,
            messages.text(MessageId::CodeSearchMatchCase),
        );
        ui.checkbox(
            &mut search.whole_symbol,
            messages.text(MessageId::CodeSearchWholeSymbol),
        );
        ui.checkbox(
            &mut search.regular_expression,
            messages.text(MessageId::CodeSearchRegularExpression),
        );
        ui.checkbox(
            &mut search.include_comments,
            messages.text(MessageId::CodeSearchIncludeComments),
        );
        ui.add_enabled(
            search.scope == CodeSourceSearchScope::ActiveLanguageProject,
            egui::Checkbox::new(
                &mut search.include_generated_references,
                messages.text(MessageId::CodeSearchIncludeGenerated),
            ),
        )
        .on_disabled_hover_text(messages.text(MessageId::CodeSearchIncludeGeneratedHint));
    });
    if found.matches.is_empty() {
        return FindAction::None;
    }
    ui.separator();
    match results_list(ui, found, messages) {
        Some((path, line)) => FindAction::Reveal(path, line),
        None => FindAction::None,
    }
}

/// The scope as the kit's select, in the order the search widens. Offering a
/// scope the search will refuse is worse than not offering it, so Selection
/// appears only when the editor captured one.
fn scope_select(
    ui: &mut Ui,
    search: &mut CodeSourceSearchState,
    has_selection: bool,
    messages: MessageCatalog,
) {
    let offered = SCOPES
        .iter()
        .filter(|(scope, _)| has_selection || *scope != CodeSourceSearchScope::Selection)
        .collect::<Vec<_>>();
    let labels = offered
        .iter()
        .map(|(_, id)| messages.text(*id))
        .collect::<Vec<_>>();
    if let Some(picked) = crate::ui::widgets::select(
        ui,
        "source-find-scope",
        &messages.text(MessageId::CommonScope),
        &scope_label(messages, search.scope),
        &labels,
        ui.available_width().max(180.0),
    ) && let Some((scope, _)) = offered.get(picked)
    {
        search.scope = *scope;
    }
}

/// One row per match: its path, its position and the line it sits on, the
/// row a reader clicks to reveal it. Returns the match clicked.
fn results_list(
    ui: &mut Ui,
    found: &CodeSourceSearchResults,
    messages: MessageCatalog,
) -> Option<(String, usize)> {
    let t = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut reveal = None;
    ScrollArea::vertical()
        .id_salt("source-find-results")
        .auto_shrink([false, true])
        .max_height(RESULT_LIST_MAX_HEIGHT)
        .show_rows(ui, RESULT_ROW_HEIGHT, found.matches.len(), |ui, rows| {
            for index in rows {
                let Some(row) = found.matches.get(index) else {
                    continue;
                };
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), RESULT_ROW_HEIGHT),
                    egui::Sense::click(),
                );
                if response.hovered() {
                    ui.painter().rect_filled(rect, 3.0, t.color.bg_hover);
                }
                // A row takes the keyboard's focus too; inside the list's
                // clip, so the ring is drawn inside the row.
                theme::paint_focus_ring(ui, &response, rect);
                let text = format!(
                    "{}  {}:{}  {}",
                    row.logical_path,
                    row.line,
                    row.column,
                    row.line_preview.trim()
                );
                let galley = ui.painter().layout_no_wrap(
                    text,
                    font.clone(),
                    // A match in a read-only source is found, not replaced.
                    if row.editable {
                        t.color.text
                    } else {
                        t.color.text_dim
                    },
                );
                ui.painter().with_clip_rect(rect).galley(
                    egui::pos2(rect.left() + 6.0, rect.center().y - galley.size().y * 0.5),
                    galley,
                    t.color.text,
                );
                // The row is a match, not a file name: announce all of it.
                let announced = messages.format(
                    if row.editable {
                        MessageId::CodeSearchResultAccessible
                    } else {
                        MessageId::CodeSearchResultAccessibleReadOnly
                    },
                    &[
                        ("path", &row.logical_path),
                        ("line", &row.line.to_string()),
                        ("column", &row.column.to_string()),
                        ("preview", row.line_preview.trim()),
                    ],
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &announced)
                });
                if response
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    reveal = Some((row.logical_path.clone(), row.line));
                }
            }
        });
    // Silence about a cap reads as "this is everything". Say what was
    // dropped and why the number on screen is not the total.
    if found.display_truncated {
        muted_inspector_copy(
            ui,
            &messages.format(
                if found.truncated {
                    MessageId::CodeSearchShowingFirstStreamStopped
                } else {
                    MessageId::CodeSearchShowingFirst
                },
                &[
                    ("limit", &SOURCE_SEARCH_RESULT_LIMIT.to_string()),
                    ("count", &found.total_matches.to_string()),
                ],
            ),
        );
    }
    reveal
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::code::{CodeFeature, tests::harness};
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    /// Every scope the picker offers must round-trip through its own label, or
    /// choosing one would silently leave the search in the previous scope.
    ///
    /// The picker writes back by matching the label it drew, so the round-trip
    /// has to hold in whichever locale drew it — a scope that only resolves in
    /// English is a control that stops working when the text is translated.
    #[test]
    fn every_offered_scope_round_trips_through_its_label_in_every_locale() {
        for locale in crate::workbench::UiTextLocale::ALL {
            let messages = MessageCatalog::new(locale);
            let mut drawn = std::collections::BTreeSet::new();
            for (scope, id) in SCOPES {
                let label = messages.text(id);
                assert_eq!(scope_label(messages, scope), label);
                assert!(
                    drawn.insert(label.clone()),
                    "two scopes share the label {label:?} in {locale:?}"
                );
                assert_eq!(
                    SCOPES
                        .iter()
                        .find(|(_, name)| messages.text(*name) == label)
                        .map(|(candidate, _)| *candidate),
                    Some(scope)
                );
            }
        }
    }

    /// The defect: search, its results, and its atomic replacement were all
    /// reachable only from tests. Drive the real entry points.
    #[test]
    fn a_bundle_search_finds_and_replaces_through_the_dialog_entry_points() {
        let mut harness = harness();
        harness.state.code_workspace.page = CodeWorkspacePage::Automation;
        code_workspace::open_active_source_search(&mut CodeFeature::workspace_parts(
            &mut harness.cx(),
        ))
        .expect("the Automation page owns a searchable bundle");

        let search = harness
            .state
            .code_workspace
            .source_search
            .as_mut()
            .expect("the search opened");
        search.query = "import".to_owned();
        search.replacement = "import".to_owned();
        let search = search.clone();

        let found = code_workspace::source_search_results_in(
            &harness.model,
            &harness.state.code_workspace,
            &harness.state.netlist,
            &harness.platform,
            &search,
        )
        .expect("the exact closure is searchable");
        assert!(
            found.total_matches > 0,
            "the fixture Automation bundle imports something"
        );
        assert_eq!(
            found.matches.len().min(SOURCE_SEARCH_RESULT_LIMIT),
            found.matches.len(),
            "the display list never exceeds its own limit"
        );
    }
}
