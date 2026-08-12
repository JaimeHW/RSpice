//! Ownership, persistence, history, and file operations for one project-owned
//! source document.
//!
//! Every operation here is a revision-bound transaction the Code Workspace
//! already implements. This module is the surface that reaches them. Before it
//! existed the whole lifecycle — new, rename, move, duplicate, delete, import,
//! role reassignment, and revision restore — was written, tested, and
//! unreachable, and the "Create source workspace" buttons on the empty
//! Verilog-A and Automation pages opened a transaction that nothing ever drew.
//!
//! A draft is never applied by closing the dialog. Committing is an explicit
//! act, because the operations here rewrite a source graph.

use egui::{Context, Grid, RichText, ScrollArea, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::{ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{
    StatusMark, code_inspector_property_list, code_inspector_section, property_row,
    property_row_input, property_row_status, section_header,
};
use crate::workbench::documents::code_workspace::{self, CodeSourceFileAction};

const TITLE: &str = "Manage source document";
const WORKSPACE_TITLE: &str = "Create source workspace";

/// Bring the lifecycle dialog up for the page that is showing.
///
/// The netlist page is deliberately refused: its deck is a generated artifact
/// or an owned file, never a project source bundle, and it has its own
/// ownership transaction.
pub(crate) fn open_source_document_dialog(state: &mut AppState) -> Result<(), String> {
    let page = state.ui.code_workspace.page;
    let language = code_workspace::page_source_language(page)
        .ok_or_else(|| "The netlist deck is managed from its own ownership dialog.".to_owned())?;
    let owner = ProjectSourceOwner::code_workspace(language);
    if state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .is_none()
    {
        return Err(format!(
            "This project owns no {} source workspace yet.",
            language.label()
        ));
    }
    state.ui.code_workspace.source_document_dialog = true;
    Ok(())
}

/// Whether the lifecycle dialog can be opened for the visible page.
pub(crate) fn source_document_dialog_is_available(state: &AppState) -> bool {
    code_workspace::page_source_language(state.ui.code_workspace.page).is_some_and(|language| {
        let owner = ProjectSourceOwner::code_workspace(language);
        state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .is_some()
    })
}

/// What the dialog states about the visible document.
///
/// Read once per frame into owned values so applying a transaction cannot
/// invalidate a borrow the body is still rendering from, and so every row is
/// measured from the same closure.
struct DocumentFacts {
    language: ProjectSourceLanguage,
    logical_path: String,
    leaf: String,
    identity: String,
    role: String,
    is_entry: bool,
    is_environment_lock: bool,
    editable: bool,
    is_root: bool,
    shape: String,
    line_endings: &'static str,
    bundle: String,
    validation_current: bool,
    imports: Vec<String>,
    imported_by: Vec<String>,
    history: Vec<RevisionRow>,
    block_reason: Option<&'static str>,
}

struct RevisionRow {
    revision: u64,
    digest: String,
    files: usize,
    current: bool,
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
}

fn leaf_name(logical_path: &str) -> String {
    logical_path
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(logical_path)
        .to_owned()
}

/// Line endings as they actually are in the retained bytes, not as a policy
/// the writer intends to apply. A mixed document is worth saying out loud
/// because a whole-file rewrite silently normalizes it.
fn line_ending_summary(content: &str) -> &'static str {
    let carriage_return_line_feed = content.matches("\r\n").count();
    let line_feed = content.matches('\n').count() - carriage_return_line_feed;
    match (carriage_return_line_feed, line_feed) {
        (0, 0) => "none \u{00b7} single line",
        (0, _) => "LF",
        (_, 0) => "CRLF",
        _ => "mixed LF and CRLF",
    }
}

fn document_facts(app: &RSpiceApp) -> Option<DocumentFacts> {
    let language = code_workspace::page_source_language(app.state.ui.code_workspace.page)?;
    let logical_path = code_workspace::active_bundle_path(
        &app.state.workspace,
        &app.state.ui.code_workspace,
        language,
    )?;
    let owner = ProjectSourceOwner::code_workspace(language);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)?;
    let content = bundle.file_content(&logical_path).unwrap_or_default();
    let identity = bundle.document_id(&logical_path).map_or_else(
        || "not retained".to_owned(),
        |id| {
            let revision = bundle
                .document_revision(&logical_path)
                .map_or_else(|| "-".to_owned(), |revision| revision.get().to_string());
            format!(
                "{} \u{00b7} rev {revision}",
                id.as_uuid()
                    .simple()
                    .to_string()
                    .chars()
                    .take(12)
                    .collect::<String>()
            )
        },
    );
    let history = bundle
        .revision_history()
        .iter()
        .map(|snapshot| RevisionRow {
            revision: snapshot.revision().get(),
            digest: short_digest(snapshot.closure_digest()),
            // A closure is its root plus the files beside it, and `files()`
            // holds only the latter.
            files: snapshot.files().len() + 1,
            current: snapshot.closure_digest() == bundle.closure_digest(),
        })
        .collect();
    Some(DocumentFacts {
        language,
        leaf: leaf_name(&logical_path),
        identity,
        role: role_label(bundle.role_for_path(&logical_path)).to_owned(),
        is_entry: bundle.role_for_path(&logical_path)
            == Some(crate::state::ProjectSourceRole::AutomationEntry),
        is_environment_lock: bundle.role_for_path(&logical_path)
            == Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
        editable: code_workspace::source_bundle_document_is_editable(
            app,
            language,
            bundle.id(),
            &logical_path,
        ),
        is_root: crate::state::project_source_paths_equal(
            &logical_path,
            bundle.root().logical_path(),
        ),
        shape: format!(
            "{} lines \u{00b7} {} bytes",
            content.lines().count(),
            content.len()
        ),
        line_endings: line_ending_summary(content),
        bundle: format!(
            "revision {} \u{00b7} {}",
            bundle.revision().get(),
            short_digest(bundle.closure_digest())
        ),
        validation_current: bundle.validation_is_current(),
        imports: dependency_paths(bundle, &logical_path, true),
        imported_by: dependency_paths(bundle, &logical_path, false),
        history,
        block_reason: code_workspace::source_file_mutation_block_reason(app, language),
        logical_path,
    })
}

/// The include edges the bundle records for this document, in the requested
/// direction. These are persisted dependency records, never a re-parse of the
/// text, so the dialog cannot claim an edge the project does not hold.
fn dependency_paths(
    bundle: &ProjectSourceBundle,
    logical_path: &str,
    as_importer: bool,
) -> Vec<String> {
    bundle
        .dependencies()
        .iter()
        .filter_map(|dependency| {
            let (matched, other) = if as_importer {
                (dependency.importer(), dependency.imported())
            } else {
                (dependency.imported(), dependency.importer())
            };
            crate::state::project_source_paths_equal(matched, logical_path)
                .then(|| other.to_owned())
        })
        .collect()
}

/// What the body asked for, applied after rendering so a transaction never
/// mutates the project while its own rows are still on screen.
enum LifecycleAction {
    None,
    Begin(CodeSourceFileAction),
    Commit,
    Cancel,
    OpenHistory,
    SelectRevision(u64),
    Restore,
    CloseHistory,
    Import,
    AssignRole(Option<crate::state::ProjectSourceRole>),
}

/// Which section states a failure, chosen by the action that caused it.
#[derive(Clone, Copy)]
enum Report {
    FileTransaction,
    History,
    Console,
}

/// The roles an Automation document can be given here.
///
/// The entry role is the bundle's own identity, bound by the closure rather
/// than reassigned per file, so it is not offered. Everything else is a
/// project decision the workspace is allowed to change.
const ASSIGNABLE_AUTOMATION_ROLES: [Option<crate::state::ProjectSourceRole>; 4] = [
    None,
    Some(crate::state::ProjectSourceRole::AutomationRunPlan),
    Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
    Some(crate::state::ProjectSourceRole::AutomationPermissionManifest),
];

/// One spelling of a document's role, so the property row and the picker can
/// never disagree about which entry is currently selected.
fn role_label(role: Option<crate::state::ProjectSourceRole>) -> &'static str {
    role.map_or("ordinary source", crate::state::ProjectSourceRole::label)
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_source_document_dialog(&mut self, ctx: &Context) {
        if self
            .state
            .ui
            .code_workspace
            .source_workspace_dialog
            .is_some()
        {
            self.render_source_workspace_dialog(ctx);
            return;
        }
        if !self.state.ui.code_workspace.source_document_dialog {
            return;
        }
        let Some(facts) = document_facts(self) else {
            // The bundle or the page went away underneath an open dialog.
            self.state.ui.code_workspace.source_document_dialog = false;
            return;
        };

        let mut dialog = Dialog::new(
            format!("{} \u{00b7} FILE LIFECYCLE", facts.leaf.to_uppercase()),
            TITLE,
            "Close",
        )
        .description(format!(
            "Review ownership, persistence, includes, and revision history for {}, and run the file operations this project source graph allows.",
            facts.leaf
        ))
        .size(DialogSize::SimulationWorkflow)
        .initial_height(600.0)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(reason) = facts.block_reason {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "File operations are held",
                reason,
            );
        } else if !facts.editable {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Read-only document",
                "This document is not editable in the active project, so operations that rewrite it are unavailable.",
            );
        }

        let mut action = LifecycleAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = lifecycle_body(ui, &mut self.state, &facts);
            None
        });
        self.apply_lifecycle_action(action, &facts);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            self.state.ui.code_workspace.source_document_dialog = false;
            self.state.ui.code_workspace.source_file_dialog = None;
            self.state.ui.code_workspace.source_history = None;
        }
    }

    fn apply_lifecycle_action(&mut self, action: LifecycleAction, facts: &DocumentFacts) {
        // Where a failure is stated is part of the action, not a guess made
        // afterwards: a rejected rename belongs beside the path field, and a
        // rejected restore beside the revision list. Deciding by whichever
        // draft happens to be open would put one inside the other's section.
        let (outcome, report) = match action {
            LifecycleAction::None => return,
            LifecycleAction::Begin(file_action) => (
                code_workspace::open_source_file_dialog(
                    self,
                    facts.language,
                    file_action,
                    &facts.logical_path,
                    &facts.logical_path,
                ),
                Report::Console,
            ),
            LifecycleAction::Commit => (
                code_workspace::commit_source_file_dialog(self).map(|_| ()),
                Report::FileTransaction,
            ),
            LifecycleAction::Cancel => {
                self.state.ui.code_workspace.source_file_dialog = None;
                return;
            }
            LifecycleAction::OpenHistory => (
                code_workspace::open_source_history(self, facts.language, &facts.logical_path),
                Report::Console,
            ),
            LifecycleAction::SelectRevision(revision) => {
                if let Some(history) = self.state.ui.code_workspace.source_history.as_mut() {
                    history.selected_revision = Some(revision);
                    history.error = None;
                }
                return;
            }
            LifecycleAction::Restore => (
                code_workspace::commit_source_history_restore(self).map(|_| ()),
                Report::History,
            ),
            LifecycleAction::CloseHistory => {
                self.state.ui.code_workspace.source_history = None;
                return;
            }
            LifecycleAction::Import => (
                code_workspace::request_automation_source_import(self, &facts.logical_path),
                Report::Console,
            ),
            LifecycleAction::AssignRole(role) => (
                code_workspace::assign_automation_source_role(self, &facts.logical_path, role)
                    .map(|_| ()),
                Report::Console,
            ),
        };
        let Err(error) = outcome else {
            return;
        };
        let stated = match report {
            Report::FileTransaction => self
                .state
                .ui
                .code_workspace
                .source_file_dialog
                .as_mut()
                .map(|draft| draft.error = Some(error.clone())),
            Report::History => self
                .state
                .ui
                .code_workspace
                .source_history
                .as_mut()
                .map(|history| history.error = Some(error.clone())),
            Report::Console => None,
        };
        // Nothing owns the failure when the draft it belonged to was cleared
        // by the same call, so the console states it instead of losing it.
        if stated.is_none() {
            self.state.push_user_message(ConsoleMessage::error(error));
        }
    }

    /// The transaction that creates a bundle for a page that has none. It is
    /// opened straight from the empty state, so it renders whenever the draft
    /// exists rather than waiting for the lifecycle dialog to be asked for.
    fn render_source_workspace_dialog(&mut self, ctx: &Context) {
        let Some(draft) = self.state.ui.code_workspace.source_workspace_dialog.clone() else {
            return;
        };
        let automation = draft.language == ProjectSourceLanguage::RSpiceAutomation;
        let mut dialog = Dialog::new(
            format!(
                "{} \u{00b7} PROJECT SOURCES",
                draft.language.label().to_uppercase()
            ),
            WORKSPACE_TITLE,
            "Create workspace",
        )
        .description(format!(
            "Create the project-owned {} source closure. Every path is portable and logical; none of these names carries semantic authority on its own.",
            draft.language.label()
        ))
        .size(DialogSize::Transaction)
        .ghost("Cancel");
        if let Some(error) = draft.error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "The source workspace cannot be created",
                error,
            );
        }

        let mut edited = draft.clone();
        let choice = dialog.show(ctx, |ui| {
            code_inspector_section(ui, "Closure", None, |ui| {
                code_inspector_property_list(ui, |ui| {
                    property_row_input(ui, "Root document", &mut edited.root_path, false);
                    if automation {
                        property_row_input(ui, "Run plan", &mut edited.run_plan_path, false);
                        property_row_input(
                            ui,
                            "Environment lock",
                            &mut edited.environment_lock_path,
                            false,
                        );
                        property_row_input(
                            ui,
                            "Permission manifest",
                            &mut edited.permission_manifest_path,
                            false,
                        );
                    } else {
                        property_row_input(
                            ui,
                            "Build profile",
                            &mut edited.build_profile_path,
                            false,
                        );
                    }
                    property_row(ui, "Bound deck", &edited.netlist_source_path);
                });
            });
        });
        if let Some(state) = self
            .state
            .ui
            .code_workspace
            .source_workspace_dialog
            .as_mut()
        {
            state.root_path = edited.root_path;
            state.build_profile_path = edited.build_profile_path;
            state.run_plan_path = edited.run_plan_path;
            state.environment_lock_path = edited.environment_lock_path;
            state.permission_manifest_path = edited.permission_manifest_path;
        }
        match choice {
            DialogChoice::Primary => match code_workspace::commit_source_workspace_dialog(self) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                }
                Err(error) => {
                    if let Some(state) = self
                        .state
                        .ui
                        .code_workspace
                        .source_workspace_dialog
                        .as_mut()
                    {
                        state.error = Some(error);
                    }
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.ui.code_workspace.source_workspace_dialog = None;
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn lifecycle_body(ui: &mut Ui, state: &mut AppState, facts: &DocumentFacts) -> LifecycleAction {
    let t = Tokens::get(ui.ctx());
    let mut action = LifecycleAction::None;
    ScrollArea::vertical()
        .id_salt("source-document-lifecycle")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(chosen) = current_document(ui, facts, t.color.ok, t.color.accent) {
                action = chosen;
            }
            operations(ui, facts, &mut action);
            if let Some(chosen) = operation_review(ui, state) {
                action = chosen;
            }
            includes(ui, facts);
            if let Some(chosen) = revision_history(ui, state, facts) {
                action = chosen;
            }
        });
    action
}

fn current_document(
    ui: &mut Ui,
    facts: &DocumentFacts,
    ok: egui::Color32,
    accent: egui::Color32,
) -> Option<LifecycleAction> {
    let ownership = if facts.editable {
        ("project-owned \u{00b7} editable", ok)
    } else {
        ("project-owned \u{00b7} read-only", accent)
    };
    let mut action = None;
    code_inspector_section(ui, "Current document", Some(ownership), |ui| {
        code_inspector_property_list(ui, |ui| {
            property_row(ui, "Path", &facts.logical_path);
            property_row(ui, "Identity", &facts.identity);
            if let Some(chosen) = role_control(ui, facts) {
                action = Some(chosen);
            }
            property_row(ui, "Content", &facts.shape);
            property_row(ui, "Line endings", facts.line_endings);
            property_row(ui, "Bundle", &facts.bundle);
            property_row_status(
                ui,
                "Validation",
                if facts.validation_current {
                    "current for this closure"
                } else {
                    "stale \u{00b7} revalidate before use"
                },
                if facts.validation_current { ok } else { accent },
                if facts.validation_current {
                    StatusMark::Success
                } else {
                    StatusMark::Warning
                },
            );
        });
    });
    action
}

/// The recorded role, editable where the project can actually change it.
///
/// An Automation environment lock refuses graph mutation and the refusal tells
/// the user to reassign its role first. Without this control that instruction
/// could not be followed, so the lock was a document nothing could ever move,
/// rename, or delete.
fn role_control(ui: &mut Ui, facts: &DocumentFacts) -> Option<LifecycleAction> {
    if facts.language != ProjectSourceLanguage::RSpiceAutomation || facts.is_entry {
        property_row(ui, "Recorded role", &facts.role);
        return None;
    }
    let options = ASSIGNABLE_AUTOMATION_ROLES
        .iter()
        .map(|role| (role_label(*role).to_owned(), role_label(*role).to_owned()))
        .collect::<Vec<_>>();
    let mut selected = facts.role.clone();
    let changed = crate::workbench::design_system::property_row_combo(
        ui,
        "Recorded role",
        "source-document-role",
        &mut selected,
        &options,
        facts.block_reason.is_none(),
    );
    changed
        .then(|| {
            ASSIGNABLE_AUTOMATION_ROLES
                .iter()
                .find(|role| role_label(**role) == selected)
                .map(|role| LifecycleAction::AssignRole(*role))
        })
        .flatten()
}

fn operations(ui: &mut Ui, facts: &DocumentFacts, action: &mut LifecycleAction) {
    code_inspector_section(ui, "File operations", None, |ui| {
        ui.add_space(6.0);
        let candidates = [
            ("New source\u{2026}", CodeSourceFileAction::New),
            ("Rename\u{2026}", CodeSourceFileAction::Rename),
            ("Move\u{2026}", CodeSourceFileAction::Move),
            ("Duplicate\u{2026}", CodeSourceFileAction::Duplicate),
            ("Delete\u{2026}", CodeSourceFileAction::Delete),
        ];
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            for (label, candidate) in candidates {
                let refusal = refusal_reason(facts, candidate);
                let response = Button::new(label)
                    .enabled(refusal.is_none())
                    .destructive(candidate == CodeSourceFileAction::Delete)
                    .show(ui);
                if response.clicked() {
                    *action = LifecycleAction::Begin(candidate);
                }
                if let Some(reason) = refusal {
                    response.on_disabled_hover_text(reason);
                }
            }
            if Button::new("Revision history\u{2026}").show(ui).clicked() {
                *action = LifecycleAction::OpenHistory;
            }
            if facts.language == ProjectSourceLanguage::RSpiceAutomation {
                let response = Button::new("Import dependency\u{2026}")
                    .enabled(facts.block_reason.is_none())
                    .show(ui);
                if response.clicked() {
                    *action = LifecycleAction::Import;
                }
                if let Some(reason) = facts.block_reason {
                    response.on_disabled_hover_text(reason);
                }
            }
        });
        ui.add_space(6.0);
    });
}

/// Why an operation is unavailable, or `None` when it is.
///
/// These are the transaction's own rules restated, not a stricter house rule.
/// Content editability deliberately does not appear: a document can be
/// read-only to type into and still be renamed, moved, or deleted, and gating
/// on it would disable buttons the service would have honoured.
fn refusal_reason(facts: &DocumentFacts, candidate: CodeSourceFileAction) -> Option<&'static str> {
    if let Some(reason) = facts.block_reason {
        return Some(reason);
    }
    let rewrites_graph = matches!(
        candidate,
        CodeSourceFileAction::New
            | CodeSourceFileAction::Rename
            | CodeSourceFileAction::Move
            | CodeSourceFileAction::Delete
    );
    if rewrites_graph && facts.is_environment_lock {
        return Some(
            "The managed Automation environment lock is read-only. Reassign its role above, or duplicate it, before changing the source graph.",
        );
    }
    // The root is the closure's entry document. Removing it would leave a
    // bundle with nothing to enter through, so the service refuses.
    (candidate == CodeSourceFileAction::Delete && facts.is_root)
        .then_some("The bundle root cannot be deleted; rename or replace it instead.")
}

/// The open file transaction, drawn where the operation buttons are so the
/// proposed path and its failure reason stay together.
fn operation_review(ui: &mut Ui, state: &mut AppState) -> Option<LifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let draft = state.ui.code_workspace.source_file_dialog.as_mut()?;
    let (title, commit) = match draft.action {
        CodeSourceFileAction::New => ("New source document", "Create"),
        CodeSourceFileAction::Rename => ("Rename document", "Rename"),
        CodeSourceFileAction::Move => ("Move document", "Move"),
        CodeSourceFileAction::Duplicate => ("Duplicate document", "Duplicate"),
        CodeSourceFileAction::Delete => ("Delete document", "Delete"),
    };
    let deleting = draft.action == CodeSourceFileAction::Delete;
    let mut action = None;
    code_inspector_section(
        ui,
        title,
        Some(("open transaction", t.color.accent)),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                if deleting {
                    property_row(ui, "Removing", &draft.source_path);
                } else {
                    property_row_input(
                        ui,
                        "Logical path",
                        &mut draft.proposed_path,
                        draft.error.is_some(),
                    );
                }
                property_row(
                    ui,
                    "Against",
                    &format!(
                        "revision {} \u{00b7} {}",
                        draft.bundle_revision,
                        short_digest(draft.closure_digest)
                    ),
                );
            });
            if let Some(error) = draft.error.as_deref() {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.err),
                );
            }
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                if Button::new(commit)
                    .accent()
                    .destructive(deleting)
                    .enabled(deleting || !draft.proposed_path.trim().is_empty())
                    .show(ui)
                    .clicked()
                {
                    action = Some(LifecycleAction::Commit);
                }
                if Button::new("Cancel").ghost().show(ui).clicked() {
                    action = Some(LifecycleAction::Cancel);
                }
                if deleting {
                    ui.label(
                    RichText::new(
                        "Removing a document rewrites the source graph as a new bundle revision.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
                }
            });
            ui.add_space(6.0);
        },
    );
    action
}

fn includes(ui: &mut Ui, facts: &DocumentFacts) {
    let t = Tokens::get(ui.ctx());
    let total = facts.imports.len() + facts.imported_by.len();
    code_inspector_section(
        ui,
        "Recorded includes",
        Some((&total.to_string(), t.color.text_dim)),
        |ui| {
            if total == 0 {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(
                        "This document has no recorded include edge in either direction.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
                ui.add_space(6.0);
                return;
            }
            code_inspector_property_list(ui, |ui| {
                for imported in &facts.imports {
                    property_row(ui, "Imports", imported);
                }
                for importer in &facts.imported_by {
                    property_row(ui, "Imported by", importer);
                }
            });
        },
    );
}

fn revision_history(
    ui: &mut Ui,
    state: &mut AppState,
    facts: &DocumentFacts,
) -> Option<LifecycleAction> {
    let history = state.ui.code_workspace.source_history.as_ref()?;
    let t = Tokens::get(ui.ctx());
    let selected = history.selected_revision;
    let error = history.error.clone();
    let mut action = None;
    code_inspector_section(
        ui,
        "Retained bundle revisions",
        Some((&facts.history.len().to_string(), t.color.text_dim)),
        |ui| {
            if facts.history.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    RichText::new("This bundle has no retained revision to restore yet.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(6.0);
                return;
            }
            ui.add_space(4.0);
            section_header(ui, "Retained closures", Some("newest last"));
            Grid::new("source-document-revisions")
                .num_columns(4)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for row in &facts.history {
                        if ui
                            .selectable_label(
                                selected == Some(row.revision),
                                RichText::new(format!("revision {}", row.revision))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
                            )
                            .clicked()
                        {
                            action = Some(LifecycleAction::SelectRevision(row.revision));
                        }
                        ui.label(
                            RichText::new(&row.digest)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(format!("{} documents", row.files))
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(if row.current { "current closure" } else { "" })
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.ok),
                        );
                        ui.end_row();
                    }
                });
            if let Some(error) = error.as_deref() {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.err),
                );
            }
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let response = Button::new("Restore as new revision")
                    .accent()
                    .enabled(selected.is_some() && facts.block_reason.is_none())
                    .show(ui);
                if response.clicked() {
                    action = Some(LifecycleAction::Restore);
                }
                if let Some(reason) = facts.block_reason {
                    response.on_disabled_hover_text(reason);
                }
                if Button::new("Close history").ghost().show(ui).clicked() {
                    action = Some(LifecycleAction::CloseHistory);
                }
            });
            ui.add_space(6.0);
        },
    );
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    /// The panic-shortcut guard every lifecycle surface carries: a dialog that
    /// rewrites a source graph must never resolve state by unwrapping.
    #[test]
    fn the_lifecycle_dialog_has_no_panic_based_state_transitions() {
        let production = crate::source_guard::production_source(include_str!("source_document.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!("] {
            assert!(
                !production.contains(forbidden),
                "the source-document dialog contains panic shortcut {forbidden}"
            );
        }
    }

    /// Line endings are reported from the retained bytes. A whole-file rewrite
    /// normalizes a mixed document, so the dialog has to be able to say so.
    #[test]
    fn line_endings_are_measured_rather_than_assumed() {
        assert_eq!(line_ending_summary("a\nb\n"), "LF");
        assert_eq!(line_ending_summary("a\r\nb\r\n"), "CRLF");
        assert_eq!(line_ending_summary("a\r\nb\n"), "mixed LF and CRLF");
        assert_eq!(line_ending_summary("single"), "none \u{00b7} single line");
    }

    /// The defect this dialog exists for. Every file operation was reachable
    /// only from a test, so the whole lifecycle shipped unreachable. Assert a
    /// real transaction can be opened, edited, and committed through the same
    /// entry points the body uses.
    #[test]
    fn a_rename_transaction_runs_end_to_end_through_the_dialog_entry_points() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = ProjectSourceOwner::code_workspace(language);
        let original = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .map(|bundle| bundle.root().logical_path().to_owned())
            .unwrap_or_default();
        assert!(!original.is_empty(), "the fixture project owns a bundle");

        open_source_document_dialog(&mut app.state)
            .expect("the Automation page owns a source bundle");
        let facts = document_facts(&app).expect("the page states a document");
        assert_eq!(facts.logical_path, original);
        assert!(facts.is_root, "the bundle root is the default document");

        app.apply_lifecycle_action(LifecycleAction::Begin(CodeSourceFileAction::Rename), &facts);
        let draft = app
            .state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .expect("Rename opens a transaction");
        draft.proposed_path = "workflows/renamed_entry.py".to_owned();

        app.apply_lifecycle_action(LifecycleAction::Commit, &facts);
        assert!(
            app.state.ui.code_workspace.source_file_dialog.is_none(),
            "a committed transaction closes"
        );
        let renamed = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .map(|bundle| bundle.root().logical_path().to_owned())
            .unwrap_or_default();
        assert_eq!(renamed, "workflows/renamed_entry.py");
    }

    /// The environment lock refuses graph mutation and its refusal tells the
    /// user to reassign the role first. That instruction was impossible to
    /// follow, because no control could set a role. Assert the refusal, the
    /// reassignment, and that the refusal then lifts.
    #[test]
    fn reassigning_a_role_lifts_the_environment_lock_refusal() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = ProjectSourceOwner::code_workspace(language);
        let locked = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .and_then(|bundle| {
                bundle
                    .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
                    .next()
                    .map(str::to_owned)
            })
            .unwrap_or_default();
        assert!(!locked.is_empty(), "the fixture bundle pins an environment");
        app.state.ui.code_workspace.automation.selected_file = Some(locked.clone());

        let before = document_facts(&app).expect("the lock is a document");
        assert!(before.is_environment_lock);
        assert!(
            refusal_reason(&before, CodeSourceFileAction::Rename).is_some(),
            "the lock refuses graph mutation"
        );

        app.apply_lifecycle_action(LifecycleAction::AssignRole(None), &before);

        let after = document_facts(&app).expect("the document survives the role change");
        assert!(!after.is_environment_lock, "the role was reassigned");
        assert!(
            refusal_reason(&after, CodeSourceFileAction::Rename).is_none(),
            "the refusal the service states is now followable"
        );
    }

    /// The netlist deck is not a project source bundle, so the dialog refuses
    /// it rather than naming the deck after somebody else's closure.
    #[test]
    fn the_netlist_page_is_refused() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
        assert!(!source_document_dialog_is_available(&app.state));
        assert!(open_source_document_dialog(&mut app.state).is_err());
        assert!(!app.state.ui.code_workspace.source_document_dialog);
    }
}
