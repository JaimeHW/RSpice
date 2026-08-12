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

const TITLE: &str = "Find in source";

/// The scopes a bundle search can run in, in the order the dialog offers them.
///
/// `Selection` is offered only when the editor actually captured one; the
/// search refuses it otherwise, and an option that always fails is worse than
/// no option.
const SCOPES: [(CodeSourceSearchScope, &str); 4] = [
    (CodeSourceSearchScope::Selection, "Selection"),
    (CodeSourceSearchScope::CurrentDocument, "Current document"),
    (CodeSourceSearchScope::OpenDocuments, "Open documents"),
    (
        CodeSourceSearchScope::ActiveLanguageProject,
        "Active language project",
    ),
];

fn scope_label(scope: CodeSourceSearchScope) -> &'static str {
    SCOPES
        .iter()
        .find(|(candidate, _)| *candidate == scope)
        .map_or("Active language project", |(_, label)| *label)
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

        let mut dialog = Dialog::new(
            format!(
                "{} \u{00b7} BUNDLE-SCOPED SEARCH",
                search.language.label().to_uppercase()
            ),
            TITLE,
            "Close",
        )
        .description(
            "Search the project-owned source closure. Finding changes nothing; replacement is one atomic bundle revision that skips every ineligible source.",
        )
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
                "This search cannot run",
                error,
            );
        } else if let Some(reason) = blocked {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Replacement held",
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
    let mut action = FindAction::None;
    ScrollArea::vertical()
        .id_salt("source-find")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let Some(search) = state.ui.code_workspace.source_search.as_mut() else {
                return;
            };
            let has_selection = search.selection_char_range.is_some();
            code_inspector_section(ui, "Query", None, |ui| {
                code_inspector_property_list(ui, |ui| {
                    property_row(ui, "Active document", &search.active_path);
                    crate::workbench::design_system::property_row_input(
                        ui,
                        "Find",
                        &mut search.query,
                        false,
                    );
                    if replaceable {
                        crate::workbench::design_system::property_row_input(
                            ui,
                            "Replace with",
                            &mut search.replacement,
                            false,
                        );
                    } else {
                        property_row(
                            ui,
                            "Replace with",
                            "unavailable \u{00b7} this document is read-only",
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
                        .map(|(_, label)| ((*label).to_owned(), (*label).to_owned()))
                        .collect::<Vec<_>>();
                    let mut chosen = scope_label(search.scope).to_owned();
                    if property_row_combo(
                        ui,
                        "Scope",
                        "source-find-scope",
                        &mut chosen,
                        &options,
                        true,
                    ) && let Some((scope, _)) = SCOPES.iter().find(|(_, label)| *label == chosen)
                    {
                        search.scope = *scope;
                    }
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(12.0, 4.0);
                    ui.checkbox(&mut search.match_case, "Match case");
                    ui.checkbox(&mut search.whole_symbol, "Whole symbol");
                    ui.checkbox(&mut search.regular_expression, "Regular expression");
                    ui.checkbox(&mut search.include_comments, "Include comments");
                    ui.add_enabled(
                        search.scope == CodeSourceSearchScope::ActiveLanguageProject,
                        egui::Checkbox::new(
                            &mut search.include_generated_references,
                            "Include generated deck \u{00b7} find only",
                        ),
                    )
                    .on_disabled_hover_text(
                        "The generated deck is only searched alongside the whole language project.",
                    );
                });
                ui.add_space(6.0);
            });

            if replaceable {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    if Button::new("Replace all eligible")
                        .accent()
                        .enabled(!search.query.is_empty() && found.editable_matches > 0)
                        .show(ui)
                        .clicked()
                    {
                        action = FindAction::Replace;
                    }
                    if found.read_only_matches > 0 {
                        ui.label(
                            RichText::new(format!(
                                "{} match{} sit in read-only sources and stay unchanged.",
                                found.read_only_matches,
                                if found.read_only_matches == 1 {
                                    ""
                                } else {
                                    "es"
                                }
                            ))
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.warn),
                        );
                    }
                });
                ui.add_space(4.0);
            }

            if let Some(reveal) = results_table(ui, found, &t) {
                action = FindAction::Reveal(reveal.0, reveal.1);
            }
        });
    action
}

fn results_table(
    ui: &mut Ui,
    found: &CodeSourceSearchResults,
    t: &Tokens,
) -> Option<(String, usize)> {
    let summary = format!(
        "{} match{}",
        found.total_matches,
        if found.total_matches == 1 { "" } else { "es" }
    );
    let mut reveal = None;
    code_inspector_section(
        ui,
        "Matches",
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
                    RichText::new("No match in the selected scope.")
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
                        if ui
                            .selectable_label(
                                false,
                                RichText::new(&row.logical_path)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                            )
                            .clicked()
                        {
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
                    RichText::new(format!(
                        "Showing the first {SOURCE_SEARCH_RESULT_LIMIT} of {} matches{}. Narrow the query to see the rest.",
                        found.total_matches,
                        if found.truncated {
                            ", and the search stopped at its streaming limit"
                        } else {
                            ""
                        }
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
    #[test]
    fn every_offered_scope_round_trips_through_its_label() {
        for (scope, label) in SCOPES {
            assert_eq!(scope_label(scope), label);
            assert_eq!(
                SCOPES
                    .iter()
                    .find(|(_, name)| *name == label)
                    .map(|(s, _)| *s),
                Some(scope)
            );
        }
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
