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

use egui::{Context, Grid, RichText, ScrollArea, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::ProjectSourceLanguage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::design_system::{
    code_inspector_property_list, code_inspector_section, property_row, property_row_combo,
};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceSearchResults, CodeSourceSearchScope, SOURCE_SEARCH_RESULT_LIMIT,
};
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

/// What the body asked for, applied after rendering.
enum FindAction {
    None,
    Replace,
    Reveal(String, usize),
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_source_find_dialog(&mut self, ctx: &Context) {
        if self.state.ui.code_workspace.source_search.is_none() {
            return;
        }
        // Read the search once, run it once, and render from that. Re-running
        // per section would let two parts of one dialog state different counts.
        let Some(search) = self.state.ui.code_workspace.source_search.clone() else {
            return;
        };
        let results = code_workspace::source_search_results(self, &search);
        let blocked = code_workspace::source_file_mutation_block_reason(self, search.language);
        let replaceable = blocked.is_none()
            && code_workspace::source_bundle_document_is_editable(
                self,
                search.language,
                search.bundle_id,
                &search.active_path,
            );

        let messages = self.state.ui.messages();
        let mut dialog = Dialog::new(
            messages.format(
                MessageId::CodeSearchEyebrow,
                &[("language", &search.language.label().to_uppercase())],
            ),
            messages.text(MessageId::CodeSearchTitle),
            messages.text(MessageId::CommonClose),
        )
        .description(messages.text(MessageId::CodeSearchDescription))
        .size(DialogSize::SimulationWorkflow)
        .initial_height(580.0)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl);
        let failure = search
            .error
            .clone()
            .or_else(|| results.as_ref().err().cloned());
        if let Some(error) = failure.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                messages.text(MessageId::CodeSearchCannotRun),
                error,
            );
        } else if let Some(reason) = blocked {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                messages.text(MessageId::CodeSearchReplacementHeld),
                reason,
            );
        }

        let found = results.unwrap_or_default();
        let mut action = FindAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = find_body(ui, &mut self.state, &found, replaceable);
            None
        });
        self.apply_find_action(action, ctx);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            self.state.ui.code_workspace.source_search = None;
        }
    }

    fn apply_find_action(&mut self, action: FindAction, ctx: &Context) {
        match action {
            FindAction::None => {}
            FindAction::Replace => {
                if let Err(error) = code_workspace::commit_source_search_replace(self) {
                    if let Some(search) = self.state.ui.code_workspace.source_search.as_mut() {
                        search.error = Some(error);
                    } else {
                        self.state.push_user_message(ConsoleMessage::error(error));
                    }
                }
            }
            // A result row is a document plus an exact line, so following one
            // has to select that document before revealing the line in it.
            FindAction::Reveal(logical_path, line) => {
                let Some(search) = self.state.ui.code_workspace.source_search.as_ref() else {
                    return;
                };
                let (language, bundle_id) = (search.language, search.bundle_id);
                match language {
                    ProjectSourceLanguage::VerilogA => {
                        self.state.ui.code_workspace.veriloga.selected_file =
                            Some(code_workspace::VerilogAFileSelection {
                                bundle_id,
                                logical_path: logical_path.clone(),
                            });
                    }
                    ProjectSourceLanguage::RSpiceAutomation => {
                        self.state.ui.code_workspace.automation.selected_file =
                            Some(logical_path.clone());
                    }
                }
                crate::workbench::documents::text_editor_commands::queue_reveal_line(
                    ctx,
                    code_workspace::source_editor_id(language, bundle_id, &logical_path),
                    line,
                );
                if let Some(search) = self.state.ui.code_workspace.source_search.as_mut() {
                    search.active_path = logical_path;
                }
            }
        }
    }
}

fn find_body(
    ui: &mut Ui,
    state: &mut crate::workbench::app_state::AppState,
    found: &CodeSourceSearchResults,
    replaceable: bool,
) -> FindAction {
    let t = Tokens::get(ui.ctx());
    let messages = state.ui.messages();
    let mut action = FindAction::None;
    ScrollArea::vertical()
        .id_salt("source-find")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let Some(search) = state.ui.code_workspace.source_search.as_mut() else {
                return;
            };
            let has_selection = search.selection_char_range.is_some();
            code_inspector_section(ui, &messages.text(MessageId::CodeSearchQuery), None, |ui| {
                code_inspector_property_list(ui, |ui| {
                    property_row(
                        ui,
                        &messages.text(MessageId::CodeSearchActiveDocument),
                        &search.active_path,
                    );
                    crate::workbench::design_system::property_row_input(
                        ui,
                        &messages.text(MessageId::CommonFind),
                        &mut search.query,
                        false,
                    );
                    let replace_with = messages.text(MessageId::CodeSearchReplaceWith);
                    if replaceable {
                        crate::workbench::design_system::property_row_input(
                            ui,
                            &replace_with,
                            &mut search.replacement,
                            false,
                        );
                    } else {
                        property_row(
                            ui,
                            &replace_with,
                            &messages.text(MessageId::CodeSearchReplaceUnavailable),
                        );
                    }
                    let options = SCOPES
                        .iter()
                        // Offering a scope the search will refuse is worse than
                        // not offering it, so Selection appears only when the
                        // editor captured one.
                        .filter(|(scope, _)| {
                            has_selection || *scope != CodeSourceSearchScope::Selection
                        })
                        .map(|(_, id)| (messages.text(*id), messages.text(*id)))
                        .collect::<Vec<_>>();
                    let mut chosen = scope_label(messages, search.scope);
                    if property_row_combo(
                        ui,
                        &messages.text(MessageId::CommonScope),
                        "source-find-scope",
                        &mut chosen,
                        &options,
                        true,
                    ) && let Some((scope, _)) =
                        SCOPES.iter().find(|(_, id)| messages.text(*id) == chosen)
                    {
                        search.scope = *scope;
                    }
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(12.0, 4.0);
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
                    .on_disabled_hover_text(
                        messages.text(MessageId::CodeSearchIncludeGeneratedHint),
                    );
                });
                ui.add_space(6.0);
            });

            if replaceable {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    if Button::new(&messages.text(MessageId::CodeSearchReplaceAllEligible))
                        .accent()
                        .enabled(!search.query.is_empty() && found.editable_matches > 0)
                        .show(ui)
                        .clicked()
                    {
                        action = FindAction::Replace;
                    }
                    if found.read_only_matches > 0 {
                        ui.label(
                            RichText::new(messages.format(
                                if found.read_only_matches == 1 {
                                    MessageId::CodeSearchReadOnlyMatchSingular
                                } else {
                                    MessageId::CodeSearchReadOnlyMatches
                                },
                                &[("count", &found.read_only_matches.to_string())],
                            ))
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.warn),
                        );
                    }
                });
                ui.add_space(4.0);
            }

            if let Some(reveal) = results_table(ui, found, &t, messages) {
                action = FindAction::Reveal(reveal.0, reveal.1);
            }
        });
    action
}

fn results_table(
    ui: &mut Ui,
    found: &CodeSourceSearchResults,
    t: &Tokens,
    messages: MessageCatalog,
) -> Option<(String, usize)> {
    let summary = messages.format(
        if found.total_matches == 1 {
            MessageId::CodeSearchMatchSingular
        } else {
            MessageId::CodeSearchMatches
        },
        &[("count", &found.total_matches.to_string())],
    );
    let mut reveal = None;
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeSearchMatchesHeading),
        Some((
            &summary,
            if found.total_matches == 0 {
                t.color.text_dim
            } else {
                t.color.ok
            },
        )),
        |ui| {
            if found.matches.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(messages.text(MessageId::CodeSearchNoMatches))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(6.0);
                return;
            }
            Grid::new("source-find-results")
                .num_columns(3)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for row in &found.matches {
                        // Only the path cell is focusable, and a match is its
                        // path, its position, and the line it sits on. Announce
                        // the row rather than the one cell that takes focus.
                        let response = ui.selectable_label(
                            false,
                            RichText::new(&row.logical_path)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
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
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::SelectableLabel,
                                true,
                                &announced,
                            )
                        });
                        if response.clicked() {
                            reveal = Some((row.logical_path.clone(), row.line));
                        }
                        ui.label(
                            RichText::new(format!("{}:{}", row.line, row.column))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(row.line_preview.trim())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(if row.editable {
                                    t.color.text
                                } else {
                                    t.color.text_dim
                                }),
                        );
                        ui.end_row();
                    }
                });
            // Silence about a cap reads as "this is everything". Say what was
            // dropped and why the number on screen is not the total.
            if found.display_truncated {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(messages.format(
                        if found.truncated {
                            MessageId::CodeSearchShowingFirstStreamStopped
                        } else {
                            MessageId::CodeSearchShowingFirst
                        },
                        &[
                            ("limit", &SOURCE_SEARCH_RESULT_LIMIT.to_string()),
                            ("count", &found.total_matches.to_string()),
                        ],
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.warn),
                );
            }
            ui.add_space(6.0);
        },
    );
    reveal
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    /// A dialog that rewrites source across a whole bundle must never resolve
    /// state by unwrapping.
    #[test]
    fn the_find_dialog_has_no_panic_based_state_transitions() {
        let production = crate::source_guard::production_source(include_str!("source_find.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!("] {
            assert!(
                !production.contains(forbidden),
                "the source-find dialog contains panic shortcut {forbidden}"
            );
        }
    }

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

    /// A result row is a path, a position, and the line it sits on, spread
    /// across three cells of which only the first can take focus. Assistive
    /// technology therefore heard a bare filename and nothing that identified
    /// which of its matches this was.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_result_row_announces_its_whole_match_and_not_just_the_focusable_cell() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        code_workspace::open_active_source_search(&mut app)
            .expect("the Automation page owns a searchable bundle");
        let Some(search) = app.state.ui.code_workspace.source_search.as_mut() else {
            panic!("the search opened");
        };
        search.query = "import".to_owned();
        let path = search.active_path.clone();

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_100.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| app.render_source_find_dialog(ctx),
        );
        let Some(update) = output.platform_output.accesskit_update else {
            panic!("the find dialog publishes an accessibility tree");
        };
        let labels = update
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();
        let Some(row) = labels
            .iter()
            .find(|label| label.starts_with(&path) && label.contains("line "))
        else {
            panic!("no result row announced its position; labels: {labels:?}");
        };
        assert!(
            row.contains("column ") && row.contains("import"),
            "a row announced its path and line but not its column or its source line: {row:?}"
        );
    }

    /// The defect: search, its results, and its atomic replacement were all
    /// reachable only from tests. Drive the real entry points.
    #[test]
    fn a_bundle_search_finds_and_replaces_through_the_dialog_entry_points() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        code_workspace::open_active_source_search(&mut app)
            .expect("the Automation page owns a searchable bundle");

        let search = app
            .state
            .ui
            .code_workspace
            .source_search
            .as_mut()
            .expect("the search opened");
        search.query = "import".to_owned();
        search.replacement = "import".to_owned();
        let search = search.clone();

        let found = code_workspace::source_search_results(&app, &search)
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
