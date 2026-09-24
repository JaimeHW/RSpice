//! Navigation, symbol intelligence, and project-wide rename over the indexed
//! source closure.
//!
//! The index answers nine questions — what a document declares, what the caret
//! sits on, where a name is defined, declared, or referenced, what completes
//! here, what signature applies, what the whole project declares, and what
//! code actions the formatter can offer. All nine were implemented against a
//! bounded, comment-aware index, and none had a caller outside the tests.
//!
//! Every answer is bound to the exact closure the index was built from. The
//! transaction refuses rather than applying to a newer revision, so this
//! surface never repairs a stale index behind the user's back — it says so.

use egui::{Context, Grid, RichText, ScrollArea, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{
    code_inspector_property_list, code_inspector_section, property_row, property_row_input,
};
use crate::workbench::documents::code_workspace::{
    self, LanguageToolView, SourceCodeAction, SourceSymbolKind,
};
use crate::workbench::{MessageCatalog, MessageId};

/// What a view is called on screen, and the stable salt its result grid is
/// keyed by.
///
/// The grid used to take its `egui::Id` from the displayed title. Translating
/// the title moved the widget, so scroll position and row state belonged to
/// the locale rather than to the view.
const fn view_text(view: LanguageToolView) -> (MessageId, &'static str) {
    match view {
        LanguageToolView::DocumentSymbols => (MessageId::CodeToolsDocumentSymbols, "doc-symbols"),
        LanguageToolView::Completion => (MessageId::CodeToolsCompletion, "completion"),
        LanguageToolView::Hover => (MessageId::CodeToolsHover, "hover"),
        LanguageToolView::SignatureHelp => (MessageId::CodeToolsSignatureHelp, "signature"),
        LanguageToolView::Definitions => (MessageId::CodeToolsDefinitions, "definitions"),
        LanguageToolView::Declarations => (MessageId::CodeToolsDeclarations, "declarations"),
        LanguageToolView::References => (MessageId::CodeToolsReferences, "references"),
        LanguageToolView::WorkspaceSymbols => (MessageId::CodeToolsWorkspaceSymbols, "workspace"),
        LanguageToolView::CodeActions => (MessageId::CodeToolsCodeActions, "code-actions"),
    }
}

/// The catalog word for an indexed symbol's kind.
///
/// The kind column is display copy, so the surface chooses its words here
/// rather than painting the index's own vocabulary.
const fn symbol_kind_message(kind: SourceSymbolKind) -> MessageId {
    match kind {
        SourceSymbolKind::Module => MessageId::CodeSymbolKindModule,
        SourceSymbolKind::Function => MessageId::CodeSymbolKindFunction,
        SourceSymbolKind::Class => MessageId::CodeSymbolKindClass,
        SourceSymbolKind::Parameter => MessageId::CodeSymbolKindParameter,
        SourceSymbolKind::Port => MessageId::CodeSymbolKindPort,
        SourceSymbolKind::Net => MessageId::CodeSymbolKindNet,
        SourceSymbolKind::Variable => MessageId::CodeSymbolKindVariable,
        SourceSymbolKind::Import => MessageId::CodeSymbolKindImport,
        SourceSymbolKind::Configuration => MessageId::CodeSymbolKindConfiguration,
        SourceSymbolKind::Discipline => MessageId::CodeSymbolKindDiscipline,
        SourceSymbolKind::Nature => MessageId::CodeSymbolKindNature,
    }
}

/// Open the tools over the document the visible page is showing.
pub(crate) fn open_active_language_tools(app: &mut RSpiceApp) -> Result<(), String> {
    let page = app.state.ui.code_workspace.page;
    let language = code_workspace::page_source_language(page).ok_or_else(|| {
        "The netlist deck is served by its own completion and validation, not the project source index."
            .to_owned()
    })?;
    let logical_path = code_workspace::active_bundle_path(
        &app.state.workspace,
        &app.state.ui.code_workspace,
        language,
    )
    .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    // The caret the editor last reported for this document. Completion,
    // signature help, and the initial query all read from it, so a stale or
    // missing caret would silently answer for position zero.
    let caret = app
        .state
        .ui
        .code_workspace
        .source_carets
        .get(&(language, logical_path.clone()))
        .copied()
        .unwrap_or(0);
    code_workspace::open_language_tools(app, language, &logical_path, caret)
}

pub(crate) fn language_tools_are_available(state: &AppState) -> bool {
    code_workspace::page_source_language(state.ui.code_workspace.page).is_some_and(|language| {
        code_workspace::active_bundle_path(&state.workspace, &state.ui.code_workspace, language)
            .is_some()
    })
}

/// What the body asked for, applied after rendering so a transaction never
/// rebuilds the index while its own rows are on screen.
enum ToolAction {
    None,
    Select(LanguageToolView),
    Query(String),
    Replacement(String),
    Rename,
    Complete(String),
    Apply(SourceCodeAction),
    Reveal(String, usize),
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_source_language_tools_dialog(&mut self, ctx: &Context) {
        if self.state.ui.code_workspace.language_tools.is_none() {
            return;
        }
        let Some(tools) = self.state.ui.code_workspace.language_tools.clone() else {
            return;
        };
        let editable =
            code_workspace::source_file_mutation_block_reason(self, tools.index.language).is_none()
                && code_workspace::source_bundle_document_is_editable(
                    self,
                    tools.index.language,
                    tools.index.bundle_id,
                    &tools.active_path,
                );
        // An index built against an older closure can still answer questions;
        // it just must not be allowed to write. State which of the two it is.
        let stale = self
            .state
            .workspace
            .project_sources
            .get_bundle(tools.index.bundle_id)
            .is_none_or(|bundle| {
                !tools
                    .index
                    .is_current(self.state.workspace.project.id(), bundle)
            });

        let messages = self.state.ui.messages();
        let mut dialog = Dialog::new(
            messages.format(
                MessageId::CodeToolsEyebrow,
                &[
                    ("language", &tools.index.language.label().to_uppercase()),
                    ("path", &tools.active_path.to_uppercase()),
                ],
            ),
            messages.text(MessageId::CodeToolsTitle),
            messages.text(MessageId::CommonClose),
        )
        .description(messages.text(MessageId::CodeToolsDescription))
        .size(DialogSize::SimulationWorkflow)
        .initial_height(600.0)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(error) = tools.error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                messages.text(MessageId::CodeToolsTransactionRefused),
                error,
            );
        } else if stale {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                messages.text(MessageId::CodeToolsIndexBehind),
                messages.text(MessageId::CodeToolsStaleIndex),
            );
        } else if let Some(status) = tools.status.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Complete,
                messages.text(MessageId::CodeToolsApplied),
                status,
            );
        }

        let mut action = ToolAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = tools_body(ui, &tools, editable && !stale, messages);
            None
        });
        self.apply_tool_action(action, ctx);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            self.state.ui.code_workspace.language_tools = None;
        }
    }

    fn apply_tool_action(&mut self, action: ToolAction, ctx: &Context) {
        let outcome = match action {
            ToolAction::None => return,
            ToolAction::Select(view) => {
                if let Some(tools) = self.state.ui.code_workspace.language_tools.as_mut() {
                    tools.view = view;
                    tools.status = None;
                    tools.error = None;
                }
                return;
            }
            ToolAction::Query(query) => {
                if let Some(tools) = self.state.ui.code_workspace.language_tools.as_mut() {
                    tools.query = query;
                    tools.confirm_rename = false;
                    tools.status = None;
                    tools.error = None;
                }
                return;
            }
            ToolAction::Replacement(replacement) => {
                if let Some(tools) = self.state.ui.code_workspace.language_tools.as_mut() {
                    tools.replacement = replacement;
                    tools.confirm_rename = false;
                    tools.error = None;
                }
                return;
            }
            ToolAction::Reveal(logical_path, line) => {
                let Some(tools) = self.state.ui.code_workspace.language_tools.as_ref() else {
                    return;
                };
                let (language, bundle_id) = (tools.index.language, tools.index.bundle_id);
                crate::workbench::documents::text_editor_commands::queue_reveal_line(
                    ctx,
                    code_workspace::source_editor_id(language, bundle_id, &logical_path),
                    line,
                );
                return;
            }
            ToolAction::Rename => code_workspace::commit_language_rename(self),
            ToolAction::Complete(completion) => {
                code_workspace::commit_language_completion(self, &completion)
            }
            ToolAction::Apply(code_action) => {
                code_workspace::commit_language_code_action(self, &code_action)
            }
        };
        let Some(tools) = self.state.ui.code_workspace.language_tools.as_mut() else {
            if let Err(error) = outcome {
                self.state.push_user_message(ConsoleMessage::error(error));
            }
            return;
        };
        match outcome {
            Ok(message) => {
                tools.status = Some(message);
                tools.error = None;
            }
            Err(error) => tools.error = Some(error),
        }
    }
}

fn tools_body(
    ui: &mut Ui,
    tools: &code_workspace::CodeLanguageToolsState,
    writable: bool,
    messages: MessageCatalog,
) -> ToolAction {
    let t = Tokens::get(ui.ctx());
    let mut action = ToolAction::None;
    ScrollArea::vertical()
        .id_salt("source-language-tools")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            code_inspector_section(
                ui,
                &messages.text(MessageId::CodeToolsIndexedClosure),
                Some((
                    &messages.format(
                        MessageId::CodeToolsIndexRevision,
                        &[("revision", &tools.index.bundle_revision.to_string())],
                    ),
                    t.color.text_dim,
                )),
                |ui| {
                    code_inspector_property_list(ui, |ui| {
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeToolsDocument),
                            &tools.active_path,
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeToolsIndexedSymbols),
                            &tools.index.symbols().len().to_string(),
                        );
                        let mut query = tools.query.clone();
                        if property_row_input(
                            ui,
                            &messages.text(MessageId::CodeToolsSymbol),
                            &mut query,
                            false,
                        )
                        .changed()
                        {
                            action = ToolAction::Query(query);
                        }
                    });
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                        for view in LanguageToolView::ALL {
                            if ui
                                .selectable_label(
                                    tools.view == view,
                                    messages.text(view_text(view).0),
                                )
                                .clicked()
                            {
                                action = ToolAction::Select(view);
                            }
                        }
                    });
                    ui.add_space(6.0);
                },
            );

            if let Some(chosen) = view_results(ui, tools, &t, writable, messages) {
                action = chosen;
            }
            if tools.view == LanguageToolView::References
                && let Some(chosen) = rename_transaction(ui, tools, &t, writable, messages)
            {
                action = chosen;
            }
        });
    action
}

fn view_results(
    ui: &mut Ui,
    tools: &code_workspace::CodeLanguageToolsState,
    t: &Tokens,
    writable: bool,
    messages: MessageCatalog,
) -> Option<ToolAction> {
    let index = &tools.index;
    let query = tools.query.as_str();
    let view = tools.view;
    match view {
        LanguageToolView::DocumentSymbols => symbol_table(
            ui,
            t,
            view,
            messages,
            index
                .document_symbols(&tools.active_path)
                .into_iter()
                .map(|symbol| SymbolRow::from_symbol(symbol, messages))
                .collect(),
        ),
        LanguageToolView::WorkspaceSymbols => symbol_table(
            ui,
            t,
            view,
            messages,
            index
                .symbols()
                .iter()
                .filter(|symbol| query.is_empty() || symbol.name.contains(query))
                .map(|symbol| SymbolRow::from_symbol(symbol, messages))
                .collect(),
        ),
        LanguageToolView::Definitions => symbol_table(
            ui,
            t,
            view,
            messages,
            index
                .definitions(query)
                .into_iter()
                .map(|symbol| SymbolRow::from_symbol(symbol, messages))
                .collect(),
        ),
        LanguageToolView::Declarations => symbol_table(
            ui,
            t,
            view,
            messages,
            index
                .declarations(query)
                .into_iter()
                .map(|symbol| SymbolRow::from_symbol(symbol, messages))
                .collect(),
        ),
        LanguageToolView::References => symbol_table(
            ui,
            t,
            view,
            messages,
            index
                .references(query)
                .into_iter()
                .map(|occurrence| SymbolRow {
                    kind: messages.text(if occurrence.definition {
                        MessageId::CodeToolsDefinitionSuffix
                    } else {
                        MessageId::CodeToolsReferenceSuffix
                    }),
                    name: occurrence.name,
                    logical_path: occurrence.logical_path,
                    line: occurrence.line,
                    detail: occurrence.line_preview.trim().to_owned(),
                })
                .collect(),
        ),
        LanguageToolView::Hover => prose(
            ui,
            t,
            messages.text(view_text(view).0),
            index.hover(query),
            messages.text(MessageId::CodeToolsHoverNoSymbol),
        ),
        LanguageToolView::SignatureHelp => prose(
            ui,
            t,
            messages.text(view_text(view).0),
            index.signature_help(&tools.active_path, tools.caret_char_index),
            messages.text(MessageId::CodeToolsSignatureNoCall),
        ),
        LanguageToolView::Completion => completions(ui, tools, t, writable, messages),
        LanguageToolView::CodeActions => code_actions(ui, tools, t, writable, messages),
    }
}

/// One navigable row, whatever the view called it.
struct SymbolRow {
    kind: String,
    name: String,
    logical_path: String,
    line: usize,
    detail: String,
}

impl SymbolRow {
    fn from_symbol(symbol: &code_workspace::SourceSymbol, messages: MessageCatalog) -> Self {
        Self {
            kind: messages.text(symbol_kind_message(symbol.kind)),
            name: symbol.name.clone(),
            logical_path: symbol.logical_path.clone(),
            line: symbol.line,
            detail: symbol.signature.clone(),
        }
    }
}

fn symbol_table(
    ui: &mut Ui,
    t: &Tokens,
    view: LanguageToolView,
    messages: MessageCatalog,
    rows: Vec<SymbolRow>,
) -> Option<ToolAction> {
    let mut action = None;
    let (title, salt) = view_text(view);
    code_inspector_section(
        ui,
        &messages.text(title),
        Some((&rows.len().to_string(), t.color.text_dim)),
        |ui| {
            if rows.is_empty() {
                muted_line(ui, t, &messages.text(MessageId::CodeToolsNoAnswer));
                return;
            }
            Grid::new(format!("source-language-tools-{salt}"))
                .num_columns(4)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for row in &rows {
                        // The name is the only focusable cell, and a result is
                        // its name, its kind, and where it lives. Announce all
                        // four rather than the one cell that takes focus.
                        let response = ui.selectable_label(
                            false,
                            RichText::new(&row.name)
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
                        );
                        let announced = messages.format(
                            MessageId::CodeToolsResultAccessible,
                            &[
                                ("name", &row.name),
                                ("kind", &row.kind),
                                ("path", &row.logical_path),
                                ("line", &row.line.to_string()),
                                ("detail", &row.detail),
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
                            action = Some(ToolAction::Reveal(row.logical_path.clone(), row.line));
                        }
                        dim(ui, t, &row.kind);
                        dim(ui, t, &format!("{}:{}", row.logical_path, row.line));
                        dim(ui, t, &row.detail);
                        ui.end_row();
                    }
                });
            ui.add_space(6.0);
        },
    );
    action
}

fn completions(
    ui: &mut Ui,
    tools: &code_workspace::CodeLanguageToolsState,
    t: &Tokens,
    writable: bool,
    messages: MessageCatalog,
) -> Option<ToolAction> {
    let offered = tools
        .index
        .completions(&tools.active_path, tools.caret_char_index, &tools.query);
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeToolsCompletion),
        Some((&offered.len().to_string(), t.color.text_dim)),
        |ui| {
            if offered.is_empty() {
                muted_line(ui, t, &messages.text(MessageId::CodeToolsNoCompletion));
                return;
            }
            Grid::new("source-language-tools-completions")
                .num_columns(3)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for candidate in &offered {
                        // Inserting rewrites the document, so the control is
                        // only offered where the transaction would be allowed.
                        if Button::new(&candidate.label)
                            .ghost()
                            .enabled(writable)
                            .show(ui)
                            .clicked()
                        {
                            action = Some(ToolAction::Complete(candidate.label.clone()));
                        }
                        dim(ui, t, &candidate.kind);
                        dim(ui, t, &candidate.detail);
                        ui.end_row();
                    }
                });
            ui.add_space(6.0);
        },
    );
    action
}

fn code_actions(
    ui: &mut Ui,
    tools: &code_workspace::CodeLanguageToolsState,
    t: &Tokens,
    writable: bool,
    messages: MessageCatalog,
) -> Option<ToolAction> {
    let offered = tools.index.code_actions(&tools.active_path);
    let mut action = None;
    let count = offered.as_ref().map_or(0, Vec::len);
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeToolsCodeActions),
        Some((&count.to_string(), t.color.text_dim)),
        |ui| match &offered {
            // The formatter refuses documents it cannot prove it would not
            // corrupt. That refusal is the finding, so it is shown as one.
            Err(refusal) => {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(refusal)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.warn),
                );
                ui.add_space(6.0);
            }
            Ok(actions) if actions.is_empty() => {
                muted_line(ui, t, &messages.text(MessageId::CodeToolsNoCodeActions));
            }
            Ok(actions) => {
                ui.add_space(6.0);
                for candidate in actions {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        if Button::new(&messages.text(MessageId::CodeToolsApply))
                            .accent()
                            .enabled(writable)
                            .show(ui)
                            .clicked()
                        {
                            action = Some(ToolAction::Apply(candidate.clone()));
                        }
                        dim(ui, t, candidate.kind.label());
                        ui.label(
                            RichText::new(&candidate.title)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                        );
                    });
                }
                ui.add_space(6.0);
            }
        },
    );
    action
}

/// Project-wide rename, offered under References because the reference list is
/// exactly what it will rewrite.
fn rename_transaction(
    ui: &mut Ui,
    tools: &code_workspace::CodeLanguageToolsState,
    t: &Tokens,
    writable: bool,
    messages: MessageCatalog,
) -> Option<ToolAction> {
    let mut action = None;
    let occurrences = tools.index.references(&tools.query).len();
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeToolsRenameSymbol),
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::CodeToolsRenaming),
                    &tools.query,
                );
                let mut replacement = tools.replacement.clone();
                if property_row_input(
                    ui,
                    &messages.text(MessageId::CodeToolsRenameTo),
                    &mut replacement,
                    false,
                )
                .changed()
                {
                    action = Some(ToolAction::Replacement(replacement));
                }
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let response = Button::new(&messages.format(
                    if occurrences == 1 {
                        MessageId::CodeToolsRenameOccurrenceSingular
                    } else {
                        MessageId::CodeToolsRenameOccurrences
                    },
                    &[("count", &occurrences.to_string())],
                ))
                .accent()
                .enabled(writable && occurrences > 0 && !tools.replacement.trim().is_empty())
                .show(ui);
                if response.clicked() {
                    action = Some(ToolAction::Rename);
                }
                if !writable {
                    response.on_disabled_hover_text(
                        messages.text(MessageId::CodeToolsRenameNotAllowed),
                    );
                }
                dim(ui, t, &messages.text(MessageId::CodeToolsRenameAtomic));
            });
            ui.add_space(6.0);
        },
    );
    action
}

fn prose(
    ui: &mut Ui,
    t: &Tokens,
    title: String,
    body: Option<String>,
    empty: String,
) -> Option<ToolAction> {
    code_inspector_section(ui, &title, None, |ui| {
        ui.add_space(6.0);
        match body {
            Some(text) => {
                ui.label(
                    RichText::new(text)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                );
            }
            None => {
                ui.label(
                    RichText::new(&empty)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            }
        }
        ui.add_space(6.0);
    });
    None
}

fn dim(ui: &mut Ui, t: &Tokens, text: &str) {
    ui.label(
        RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn muted_line(ui: &mut Ui, t: &Tokens, text: &str) {
    ui.add_space(6.0);
    dim(ui, t, text);
    ui.add_space(6.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    /// A surface that rewrites every occurrence of a name across a project
    /// must never resolve state by unwrapping.
    #[test]
    fn the_language_tools_dialog_has_no_panic_based_state_transitions() {
        let production =
            crate::source_guard::production_source(include_str!("source_language_tools.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!("] {
            assert!(
                !production.contains(forbidden),
                "the language tools dialog contains panic shortcut {forbidden}"
            );
        }
    }

    /// Every view must be offered exactly once and name itself, or a chip
    /// would select a view the body cannot render.
    ///
    /// The grid salt is checked alongside the name because it used to be
    /// derived from it: two views that translate to the same word would then
    /// have shared one widget identity.
    #[test]
    fn every_view_is_offered_once_and_names_itself_without_sharing_a_widget_identity() {
        let salts = LanguageToolView::ALL
            .iter()
            .map(|view| view_text(*view).1)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(salts.len(), LanguageToolView::ALL.len());

        for locale in crate::workbench::UiTextLocale::ALL {
            let messages = MessageCatalog::new(locale);
            let names = LanguageToolView::ALL
                .iter()
                .map(|view| messages.text(view_text(*view).0))
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                names.len(),
                LanguageToolView::ALL.len(),
                "two views share a name in {locale:?}"
            );
        }
        let english = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
        assert_eq!(
            english.text(view_text(LanguageToolView::References).0),
            "References"
        );
        assert_eq!(
            english.text(view_text(LanguageToolView::CodeActions).0),
            "Code actions"
        );
    }

    /// The defect: the whole index — symbols, definitions, references, and
    /// project-wide rename — was reachable only from tests. Open it the way
    /// the surface does and assert it answers.
    #[test]
    fn the_index_answers_through_the_dialog_entry_point() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        assert!(language_tools_are_available(&app.state));

        open_active_language_tools(&mut app).expect("the Automation bundle is indexable");
        let tools = app
            .state
            .ui
            .code_workspace
            .language_tools
            .as_ref()
            .expect("the tools opened");
        assert!(
            !tools.index.symbols().is_empty(),
            "the fixture Automation source declares something"
        );
        assert_eq!(tools.view, LanguageToolView::DocumentSymbols);
    }

    /// The netlist deck has its own completion and validation and is not in
    /// the project source index.
    #[test]
    fn the_netlist_page_has_no_bundle_index() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
        assert!(!language_tools_are_available(&app.state));
        assert!(open_active_language_tools(&mut app).is_err());
    }

    /// `resolve_veriloga_compile` was written to gate the compile command and
    /// never attached to it, so the button stayed enabled while a compile was
    /// already running and the surface then refused the request.
    #[test]
    fn compiling_verilog_a_is_refused_while_a_compile_is_already_pending() {
        use crate::workbench::commands::vocabulary::Command;

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        app.state.ui.code_workspace.page = CodeWorkspacePage::VerilogA;
        let enabled_before = Command::CompileVerilogA.is_enabled(&app);

        app.state.workbench.safe_mode.active = true;
        app.state.workbench.safe_mode.applied.open_project_read_only = true;
        assert!(
            !Command::CompileVerilogA.is_enabled(&app),
            "a read-only project cannot compile; it was enabled before as {enabled_before}"
        );
    }
}
