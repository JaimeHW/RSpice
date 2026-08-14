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
use crate::workbench::{MessageCatalog, MessageId};

/// Bring the lifecycle dialog up for the page that is showing.
///
pub(crate) fn open_source_document_dialog(state: &mut AppState) -> Result<(), String> {
    let page = state.ui.code_workspace.page;
    if page == code_workspace::CodeWorkspacePage::Netlist {
        if !state.project_lifecycle.project_open {
            return Err("Open a project before managing its top decks.".to_owned());
        }
        state.ui.code_workspace.source_document_dialog = true;
        return Ok(());
    }
    let language = code_workspace::page_source_language(page)
        .ok_or_else(|| "The visible Code workspace page has no source lifecycle.".to_owned())?;
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
    if state.ui.code_workspace.page == code_workspace::CodeWorkspacePage::Netlist {
        return state.project_lifecycle.project_open;
    }
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
    line_endings: MessageId,
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

struct NetlistLifecycleFacts {
    deck_id: Option<uuid::Uuid>,
    logical_path: String,
    identity: String,
    shape: String,
    line_endings: MessageId,
    published_origin: String,
    history_count: usize,
    decks: Vec<(uuid::Uuid, String)>,
    operations_blocked: Option<String>,
    include_identity: Option<String>,
    include_owned: bool,
}

fn netlist_lifecycle_facts(
    app: &RSpiceApp,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleFacts> {
    let descriptor = app.state.workspace.netlist_descriptor.as_ref();
    let document = app.state.workspace.netlist_document.as_ref();
    if descriptor.is_some() != document.is_some() {
        return None;
    }
    let mut decks = Vec::with_capacity(
        app.state.workspace.retained_netlist_decks.len() + usize::from(descriptor.is_some()),
    );
    if let Some(descriptor) = descriptor {
        decks.push((descriptor.deck_id, descriptor.artifact_name.clone()));
    }
    decks.extend(
        app.state
            .workspace
            .retained_netlist_decks
            .iter()
            .map(|deck| {
                (
                    deck.descriptor.deck_id,
                    deck.descriptor.artifact_name.clone(),
                )
            }),
    );
    decks.sort_by(|left, right| {
        left.1
            .to_ascii_lowercase()
            .cmp(&right.1.to_ascii_lowercase())
    });
    let identity = match (descriptor, document) {
        (Some(descriptor), Some(document)) => format!(
            "{} · doc {} · rev {}",
            descriptor
                .deck_id
                .simple()
                .to_string()
                .chars()
                .take(12)
                .collect::<String>(),
            document
                .id()
                .as_uuid()
                .simple()
                .to_string()
                .chars()
                .take(12)
                .collect::<String>(),
            document.revision().get(),
        ),
        (None, None) => "No project-owned top deck has been created.".to_owned(),
        _ => return None,
    };
    let active_dependency =
        crate::workbench::documents::netlist_document::active_dependency(&app.state);
    let published_origin = active_dependency.map_or_else(
        || {
            app.state
                .workspace
                .netlist_source_path
                .as_ref()
                .map_or_else(
                    || messages.text(MessageId::NetlistNotPublished),
                    |path| path.display().to_string(),
                )
        },
        |dependency| {
            dependency
                .locator()
                .native_origin()
                .map_or_else(|| "Retained inside the project".to_owned(), str::to_owned)
        },
    );
    let include_owned = active_dependency.is_some()
        && crate::workbench::documents::netlist_document::active_dependency_is_owned(&app.state);
    let root_path = descriptor.map_or_else(
        || "No top deck".to_owned(),
        |descriptor| descriptor.artifact_name.clone(),
    );
    Some(NetlistLifecycleFacts {
        deck_id: descriptor.map(|descriptor| descriptor.deck_id),
        logical_path: active_dependency.map_or_else(
            || root_path.clone(),
            |dependency| dependency.locator().display_name().to_owned(),
        ),
        identity: active_dependency.map_or(identity, |dependency| {
            format!(
                "{} · included by {root_path}",
                dependency.locator().logical_identity()
            )
        }),
        shape: active_dependency.map_or_else(
            || {
                document.map_or_else(
                    || "No authored source".to_owned(),
                    |document| {
                        messages.format(
                            MessageId::CodeSourceContentShape,
                            &[
                                ("lines", &document.source().lines().count().to_string()),
                                ("bytes", &document.source().len().to_string()),
                            ],
                        )
                    },
                )
            },
            |dependency| {
                dependency.source().map_or_else(
                    || "Unresolved include".to_owned(),
                    |source| {
                        messages.format(
                            MessageId::CodeSourceContentShape,
                            &[
                                ("lines", &source.lines().count().to_string()),
                                ("bytes", &source.len().to_string()),
                            ],
                        )
                    },
                )
            },
        ),
        line_endings: active_dependency
            .and_then(crate::state::DependencyMetadata::source)
            .map_or_else(
                || {
                    document.map_or(MessageId::CodeSourceLineEndingsNone, |document| {
                        line_ending_summary(document.source())
                    })
                },
                line_ending_summary,
            ),
        published_origin,
        history_count: descriptor.map_or(0, |descriptor| descriptor.revision_history.len()),
        decks,
        operations_blocked: active_dependency.and_then(|_| {
            (!include_owned).then(|| {
                "Copy this include into the project before changing its document ownership."
                    .to_owned()
            })
        }),
        include_identity: active_dependency
            .map(|dependency| dependency.locator().logical_identity().to_owned()),
        include_owned,
    })
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
fn line_ending_summary(content: &str) -> MessageId {
    let carriage_return_line_feed = content.matches("\r\n").count();
    let line_feed = content.matches('\n').count() - carriage_return_line_feed;
    match (carriage_return_line_feed, line_feed) {
        (0, 0) => MessageId::CodeSourceLineEndingsNone,
        (0, _) => MessageId::CodeSourceLineEndingsLf,
        (_, 0) => MessageId::CodeSourceLineEndingsCrLf,
        _ => MessageId::CodeSourceLineEndingsMixed,
    }
}

fn document_facts(app: &RSpiceApp, messages: MessageCatalog) -> Option<DocumentFacts> {
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
        || messages.text(MessageId::CodeSourceIdentityNotRetained),
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
        role: role_label(messages, bundle.role_for_path(&logical_path)),
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
        shape: messages.format(
            MessageId::CodeSourceContentShape,
            &[
                ("lines", &content.lines().count().to_string()),
                ("bytes", &content.len().to_string()),
            ],
        ),
        line_endings: line_ending_summary(content),
        bundle: messages.format(
            MessageId::CodeSourceAgainstRevision,
            &[
                ("revision", &bundle.revision().get().to_string()),
                ("digest", &short_digest(bundle.closure_digest())),
            ],
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

enum NetlistLifecycleAction {
    None,
    Begin(CodeSourceFileAction),
    Commit,
    Cancel,
    SelectDeck(uuid::Uuid),
    OpenHistory,
    CopyIncludeOwnership,
    ReleaseIncludeOwnership,
    BeginInclude(CodeSourceFileAction),
    CommitInclude,
    CancelInclude,
    SaveSourceAs,
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
///
/// The picker resolves a selection by matching the label it drew, so the label
/// has to come from the same locale the row was painted in.
fn role_label(messages: MessageCatalog, role: Option<crate::state::ProjectSourceRole>) -> String {
    messages.text(
        role.map_or(MessageId::CodeSourceRoleOrdinary, |role| match role {
            crate::state::ProjectSourceRole::VerilogABuildProfile => {
                MessageId::CodeSourceRoleBuildProfile
            }
            crate::state::ProjectSourceRole::AutomationEntry => {
                MessageId::CodeSourceRoleAutomationEntry
            }
            crate::state::ProjectSourceRole::AutomationRunPlan => MessageId::CodeSourceRoleRunPlan,
            crate::state::ProjectSourceRole::AutomationEnvironmentLock => {
                MessageId::CodeSourceRoleEnvironmentLock
            }
            crate::state::ProjectSourceRole::AutomationPermissionManifest => {
                MessageId::CodeSourceRolePermissionManifest
            }
        }),
    )
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
        if self.state.ui.code_workspace.page == code_workspace::CodeWorkspacePage::Netlist {
            self.render_netlist_source_document_dialog(ctx);
            return;
        }
        let messages = self.state.ui.messages();
        let Some(facts) = document_facts(self, messages) else {
            // The bundle or the page went away underneath an open dialog.
            self.state.ui.code_workspace.source_document_dialog = false;
            return;
        };

        let mut dialog = Dialog::new(
            messages.format(
                MessageId::CodeSourceLifecycleEyebrow,
                &[("document", &facts.leaf.to_uppercase())],
            ),
            messages.text(MessageId::CodeSourceLifecycleTitle),
            messages.text(MessageId::CommonClose),
        )
        .description(messages.format(
            MessageId::CodeSourceLifecycleDescription,
            &[("document", &facts.leaf)],
        ))
        .size(DialogSize::SimulationWorkflow)
        .initial_height(600.0)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(reason) = facts.block_reason {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                messages.text(MessageId::CodeSourceOperationsHeld),
                reason,
            );
        } else if !facts.editable {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                messages.text(MessageId::CodeSourceReadOnlyDocument),
                messages.text(MessageId::CodeSourceReadOnlyDocumentDetail),
            );
        }

        let mut action = LifecycleAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = lifecycle_body(ui, &mut self.state, &facts, messages);
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

    fn render_netlist_source_document_dialog(&mut self, ctx: &Context) {
        let messages = self.state.ui.messages();
        let Some(facts) = netlist_lifecycle_facts(self, messages) else {
            self.state.ui.code_workspace.source_document_dialog = false;
            self.state.ui.netlist.lifecycle_dialog.transaction = None;
            self.state.ui.netlist.lifecycle_dialog.include_transaction = None;
            return;
        };
        let mut dialog = Dialog::new(
            messages.format(
                MessageId::CodeSourceLifecycleEyebrow,
                &[("document", &facts.logical_path.to_uppercase())],
            ),
            messages.text(MessageId::CodeSourceLifecycleTitle),
            messages.text(MessageId::CommonClose),
        )
        .description(messages.format(
            MessageId::CodeSourceLifecycleDescription,
            &[("document", &facts.logical_path)],
        ))
        .size(DialogSize::SimulationWorkflow)
        .initial_height(600.0)
        .primary_on_enter(false)
        .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(reason) = facts.operations_blocked.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                messages.text(MessageId::CodeSourceOperationsHeld),
                reason,
            );
        }

        let mut action = NetlistLifecycleAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = netlist_lifecycle_body(ui, &mut self.state, &facts, messages);
            None
        });
        match action {
            NetlistLifecycleAction::None => {}
            NetlistLifecycleAction::Begin(file_action) => {
                if let Err(error) =
                    crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
                        &mut self.state,
                        file_action,
                    )
                {
                    self.state.push_user_message(ConsoleMessage::error(error));
                }
            }
            NetlistLifecycleAction::Commit => {
                match crate::workbench::documents::netlist_document::commit_netlist_lifecycle_action(
                    &mut self.state,
                ) {
                    Ok(message) => self.state.push_user_message(ConsoleMessage::info(message)),
                    Err(error) => {
                        if let Some(transaction) =
                            self.state.ui.netlist.lifecycle_dialog.transaction.as_mut()
                        {
                            transaction.error = Some(error);
                        } else {
                            self.state.push_user_message(ConsoleMessage::error(error));
                        }
                    }
                }
            }
            NetlistLifecycleAction::Cancel => {
                self.state.ui.netlist.lifecycle_dialog.transaction = None;
            }
            NetlistLifecycleAction::SelectDeck(deck_id) => {
                if let Err(error) =
                    crate::workbench::documents::netlist_document::select_retained_top_deck(
                        &mut self.state,
                        deck_id,
                    )
                {
                    self.state.push_user_message(ConsoleMessage::error(error));
                }
            }
            NetlistLifecycleAction::OpenHistory => {
                self.state.ui.netlist.comparison_dialog.open = true;
                self.state
                    .ui
                    .netlist
                    .comparison_dialog
                    .selected_history_index = facts.history_count.saturating_sub(1);
                self.state.ui.code_workspace.source_document_dialog = false;
            }
            NetlistLifecycleAction::CopyIncludeOwnership => {
                match crate::workbench::documents::netlist_document::copy_active_dependency_to_project(
                    &mut self.state,
                ) {
                    Ok(_) => self.state.push_user_message(ConsoleMessage::info(
                        messages.text(MessageId::NetlistCopySucceeded),
                    )),
                    Err(error) => self.state.push_user_message(ConsoleMessage::error(error)),
                }
            }
            NetlistLifecycleAction::ReleaseIncludeOwnership => {
                match crate::workbench::documents::netlist_document::release_active_dependency_from_project(
                    &mut self.state,
                ) {
                    Ok(()) => self.state.push_user_message(ConsoleMessage::info(
                        messages.text(MessageId::NetlistReleaseSucceeded),
                    )),
                    Err(error) => self.state.push_user_message(ConsoleMessage::error(error)),
                }
            }
            NetlistLifecycleAction::BeginInclude(file_action) => {
                if let Err(error) = crate::workbench::documents::netlist_document::begin_owned_include_lifecycle_action(
                    &mut self.state,
                    file_action,
                ) {
                    self.state.push_user_message(ConsoleMessage::error(error));
                }
            }
            NetlistLifecycleAction::CommitInclude => {
                match crate::workbench::documents::netlist_document::commit_owned_include_lifecycle_action(
                    &mut self.state,
                ) {
                    Ok(message) => self.state.push_user_message(ConsoleMessage::info(message)),
                    Err(error) => {
                        if let Some(transaction) = self
                            .state
                            .ui
                            .netlist
                            .lifecycle_dialog
                            .include_transaction
                            .as_mut()
                        {
                            transaction.error = Some(error);
                        }
                    }
                }
            }
            NetlistLifecycleAction::CancelInclude => {
                self.state.ui.netlist.lifecycle_dialog.include_transaction = None;
            }
            NetlistLifecycleAction::SaveSourceAs => {
                crate::workbench::documents::netlist_document::open_netlist_save_dialog(
                    &mut self.state,
                    true,
                );
                self.state.ui.code_workspace.source_document_dialog = false;
            }
        }
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            self.state.ui.code_workspace.source_document_dialog = false;
            self.state.ui.netlist.lifecycle_dialog.transaction = None;
            self.state.ui.netlist.lifecycle_dialog.include_transaction = None;
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
        let messages = self.state.ui.messages();
        let mut dialog = Dialog::new(
            messages.format(
                MessageId::CodeSourceWorkspaceEyebrow,
                &[("language", &draft.language.label().to_uppercase())],
            ),
            messages.text(MessageId::CodeSourceWorkspaceTitle),
            messages.text(MessageId::CodeSourceWorkspacePrimary),
        )
        .description(messages.format(
            MessageId::CodeSourceWorkspaceDescription,
            &[("language", draft.language.label())],
        ))
        .size(DialogSize::Transaction)
        .ghost(messages.text(MessageId::CommonCancel));
        if let Some(error) = draft.error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                messages.text(MessageId::CodeSourceWorkspaceRefused),
                error,
            );
        }

        let mut edited = draft.clone();
        let choice = dialog.show(ctx, |ui| {
            code_inspector_section(
                ui,
                &messages.text(MessageId::CodeSourceWorkspaceClosure),
                None,
                |ui| {
                    code_inspector_property_list(ui, |ui| {
                        property_row_input(
                            ui,
                            &messages.text(MessageId::CodeSourceWorkspaceRootPath),
                            &mut edited.root_path,
                            false,
                        );
                        if automation {
                            property_row_input(
                                ui,
                                &messages.text(MessageId::CodeSourceWorkspaceRunPlanPath),
                                &mut edited.run_plan_path,
                                false,
                            );
                            property_row_input(
                                ui,
                                &messages.text(MessageId::CodeSourceWorkspaceEnvironmentLockPath),
                                &mut edited.environment_lock_path,
                                false,
                            );
                            property_row_input(
                                ui,
                                &messages.text(MessageId::CodeSourceWorkspacePermissionPath),
                                &mut edited.permission_manifest_path,
                                false,
                            );
                        } else {
                            property_row_input(
                                ui,
                                &messages.text(MessageId::CodeSourceWorkspaceBuildProfilePath),
                                &mut edited.build_profile_path,
                                false,
                            );
                        }
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeSourceWorkspaceBoundDeck),
                            &edited.netlist_source_path,
                        );
                    });
                },
            );
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

fn lifecycle_body(
    ui: &mut Ui,
    state: &mut AppState,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> LifecycleAction {
    let t = Tokens::get(ui.ctx());
    let mut action = LifecycleAction::None;
    ScrollArea::vertical()
        .id_salt("source-document-lifecycle")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(chosen) = current_document(ui, facts, t.color.ok, t.color.accent, messages)
            {
                action = chosen;
            }
            operations(ui, facts, &mut action, messages);
            if let Some(chosen) = operation_review(ui, state, messages) {
                action = chosen;
            }
            includes(ui, facts, messages);
            if let Some(chosen) = revision_history(ui, state, facts, messages) {
                action = chosen;
            }
        });
    action
}

fn netlist_lifecycle_body(
    ui: &mut Ui,
    state: &mut AppState,
    facts: &NetlistLifecycleFacts,
    messages: MessageCatalog,
) -> NetlistLifecycleAction {
    let t = Tokens::get(ui.ctx());
    let mut action = NetlistLifecycleAction::None;
    ScrollArea::vertical()
        .id_salt("netlist-document-lifecycle")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            code_inspector_section(
                ui,
                &messages.text(MessageId::CodeSourceCurrentDocument),
                Some((
                    &messages.text(MessageId::CodeSourceOwnedEditable),
                    t.color.ok,
                )),
                |ui| {
                    code_inspector_property_list(ui, |ui| {
                        if facts.include_identity.is_some() {
                            property_row(
                                ui,
                                &messages.text(MessageId::NetlistLogicalIdentity),
                                facts.include_identity.as_deref().unwrap_or_default(),
                            );
                        } else if facts.decks.is_empty() {
                            property_row(
                                ui,
                                &messages.text(MessageId::NetlistTopDeck),
                                &facts.logical_path,
                            );
                        } else {
                            let choices = facts
                                .decks
                                .iter()
                                .map(|(_, path)| (path.clone(), path.clone()))
                                .collect::<Vec<_>>();
                            let mut selected = facts.logical_path.clone();
                            if crate::workbench::design_system::property_row_combo(
                                ui,
                                &messages.text(MessageId::NetlistTopDeck),
                                "netlist-top-deck-selector",
                                &mut selected,
                                &choices,
                                facts.operations_blocked.is_none()
                                    && state.ui.netlist.lifecycle_dialog.transaction.is_none(),
                            ) && let Some((deck_id, _)) =
                                facts.decks.iter().find(|(_, path)| *path == selected)
                                && Some(*deck_id) != facts.deck_id
                            {
                                action = NetlistLifecycleAction::SelectDeck(*deck_id);
                            }
                        }
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeSourcePath),
                            &facts.logical_path,
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeSourceIdentity),
                            &facts.identity,
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeSourceContent),
                            &facts.shape,
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::CodeSourceLineEndings),
                            &messages.text(facts.line_endings),
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::NetlistPublishedOrigin),
                            &facts.published_origin,
                        );
                        property_row(
                            ui,
                            &messages.text(MessageId::NetlistTopDeckCatalog),
                            &messages.format(
                                if facts.decks.len() == 1 {
                                    MessageId::CodeSourceDocumentSingular
                                } else {
                                    MessageId::CodeSourceDocuments
                                },
                                &[("count", &facts.decks.len().to_string())],
                            ),
                        );
                    });
                },
            );
            code_inspector_section(
                ui,
                &messages.text(MessageId::CodeSourceFileOperations),
                None,
                |ui| {
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                        let transaction_open =
                            state.ui.netlist.lifecycle_dialog.transaction.is_some()
                                || state
                                    .ui
                                    .netlist
                                    .lifecycle_dialog
                                    .include_transaction
                                    .is_some();
                        if facts.include_identity.is_some() {
                            if Button::new(&messages.text(MessageId::NetlistCopyProject))
                                .enabled(!facts.include_owned && !transaction_open)
                                .show(ui)
                                .clicked()
                            {
                                action = NetlistLifecycleAction::CopyIncludeOwnership;
                            }
                            for (label, candidate) in [
                                (MessageId::CodeSourceRename, CodeSourceFileAction::Rename),
                                (MessageId::CodeSourceMove, CodeSourceFileAction::Move),
                            ] {
                                let response = Button::new(&messages.text(label))
                                    .enabled(facts.include_owned && !transaction_open)
                                    .show(ui);
                                if response.clicked() {
                                    action = NetlistLifecycleAction::BeginInclude(candidate);
                                }
                            }
                            let response = Button::new(
                                &messages.text(MessageId::NetlistReleaseProjectOwnership),
                            )
                            .enabled(facts.include_owned && !transaction_open)
                            .show(ui);
                            if response.clicked() {
                                action = NetlistLifecycleAction::ReleaseIncludeOwnership;
                            }
                            if !facts.include_owned {
                                response.on_disabled_hover_text(
                                    facts.operations_blocked.as_deref().unwrap_or(
                                        "This include is already a read-only referenced source.",
                                    ),
                                );
                            }
                            ui.label(messages.text(MessageId::NetlistReleaseOwnershipDescription));
                        } else {
                            if Button::new(&messages.text(if cfg!(target_arch = "wasm32") {
                                MessageId::NetlistDownloadSourceCopy
                            } else {
                                MessageId::NetlistSaveSourceAs
                            }))
                            .enabled(facts.deck_id.is_some() && !transaction_open)
                            .show(ui)
                            .clicked()
                            {
                                action = NetlistLifecycleAction::SaveSourceAs;
                            }
                            for (label, candidate) in FILE_OPERATIONS {
                                let enabled = facts.operations_blocked.is_none()
                                    && !transaction_open
                                    && (facts.deck_id.is_some()
                                        || candidate == CodeSourceFileAction::New);
                                let response = Button::new(&messages.text(label))
                                    .enabled(enabled)
                                    .destructive(candidate == CodeSourceFileAction::Delete)
                                    .show(ui);
                                if response.clicked() {
                                    action = NetlistLifecycleAction::Begin(candidate);
                                }
                                if let Some(reason) = facts.operations_blocked.as_deref() {
                                    response.on_disabled_hover_text(reason);
                                }
                            }
                            if Button::new(&messages.text(MessageId::CodeSourceRevisionHistory))
                                .enabled(
                                    facts.history_count > 0
                                        && state.ui.netlist.lifecycle_dialog.transaction.is_none(),
                                )
                                .show(ui)
                                .clicked()
                            {
                                action = NetlistLifecycleAction::OpenHistory;
                            }
                        }
                    });
                    ui.add_space(6.0);
                },
            );
            if let Some(chosen) = netlist_operation_review(ui, state, messages) {
                action = chosen;
            }
            if let Some(chosen) = netlist_include_operation_review(ui, state, messages) {
                action = chosen;
            }
        });
    action
}

fn netlist_operation_review(
    ui: &mut Ui,
    state: &mut AppState,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let transaction = state.ui.netlist.lifecycle_dialog.transaction.as_mut()?;
    let (title, commit) = match transaction.action {
        CodeSourceFileAction::New => (
            MessageId::CodeSourceCreateTitle,
            MessageId::CodeSourceCreatePrimary,
        ),
        CodeSourceFileAction::Rename => (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        ),
        CodeSourceFileAction::Move => (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        ),
        CodeSourceFileAction::Duplicate => (
            MessageId::CodeSourceDuplicateTitle,
            MessageId::CodeSourceDuplicatePrimary,
        ),
        CodeSourceFileAction::Delete => (
            MessageId::CodeSourceDeleteTitle,
            MessageId::CodeSourceDeletePrimary,
        ),
    };
    let deleting = transaction.action == CodeSourceFileAction::Delete;
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(title),
        Some((
            &messages.text(MessageId::CodeSourceOpenTransaction),
            t.color.accent,
        )),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                if deleting {
                    property_row(
                        ui,
                        &messages.text(MessageId::CodeSourceRemoving),
                        &transaction.source_path,
                    );
                } else {
                    property_row_input(
                        ui,
                        &messages.text(MessageId::CodeSourceLogicalPath),
                        &mut transaction.proposed_path,
                        transaction.error.is_some(),
                    );
                }
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceAgainst),
                    &match (transaction.document_revision, transaction.content_digest) {
                        (Some(revision), Some(digest)) => format!(
                            "doc rev {} · {}",
                            revision.get(),
                            digest.to_string().chars().take(12).collect::<String>()
                        ),
                        _ => "empty project netlist workspace".to_owned(),
                    },
                );
            });
            if let Some(error) = transaction.error.as_deref() {
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
                if Button::new(&messages.text(commit))
                    .accent()
                    .destructive(deleting)
                    .enabled(deleting || !transaction.proposed_path.trim().is_empty())
                    .show(ui)
                    .clicked()
                {
                    action = Some(NetlistLifecycleAction::Commit);
                }
                if Button::new(&messages.text(MessageId::CommonCancel))
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    action = Some(NetlistLifecycleAction::Cancel);
                }
                if deleting {
                    ui.label(
                        RichText::new(messages.text(MessageId::CodeSourceDeleteWarning))
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

fn netlist_include_operation_review(
    ui: &mut Ui,
    state: &mut AppState,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let transaction = state
        .ui
        .netlist
        .lifecycle_dialog
        .include_transaction
        .as_mut()?;
    let (title, commit) = if transaction.action == CodeSourceFileAction::Rename {
        (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        )
    } else {
        (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        )
    };
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(title),
        Some((
            &messages.text(MessageId::CodeSourceOpenTransaction),
            t.color.accent,
        )),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::NetlistRequestedAs),
                    &transaction.logical_identity,
                );
                property_row_input(
                    ui,
                    &messages.text(MessageId::CodeSourceLogicalPath),
                    &mut transaction.proposed_identity,
                    transaction.error.is_some(),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceAgainst),
                    &format!("root rev {}", transaction.root_revision.get()),
                );
            });
            if let Some(error) = transaction.error.as_deref() {
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
                if Button::new(&messages.text(commit))
                    .accent()
                    .enabled(!transaction.proposed_identity.trim().is_empty())
                    .show(ui)
                    .clicked()
                {
                    action = Some(NetlistLifecycleAction::CommitInclude);
                }
                if Button::new(&messages.text(MessageId::CommonCancel))
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    action = Some(NetlistLifecycleAction::CancelInclude);
                }
            });
            ui.add_space(6.0);
        },
    );
    action
}

fn current_document(
    ui: &mut Ui,
    facts: &DocumentFacts,
    ok: egui::Color32,
    accent: egui::Color32,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let ownership = if facts.editable {
        (messages.text(MessageId::CodeSourceOwnedEditable), ok)
    } else {
        (messages.text(MessageId::CodeSourceOwnedReadOnly), accent)
    };
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeSourceCurrentDocument),
        Some((&ownership.0, ownership.1)),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourcePath),
                    &facts.logical_path,
                );
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceIdentity),
                    &facts.identity,
                );
                if let Some(chosen) = role_control(ui, facts, messages) {
                    action = Some(chosen);
                }
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceContent),
                    &facts.shape,
                );
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceLineEndings),
                    &messages.text(facts.line_endings),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceBundle),
                    &facts.bundle,
                );
                property_row_status(
                    ui,
                    &messages.text(MessageId::CodeSourceValidation),
                    &messages.text(if facts.validation_current {
                        MessageId::CodeSourceValidationCurrent
                    } else {
                        MessageId::CodeSourceValidationStale
                    }),
                    if facts.validation_current { ok } else { accent },
                    if facts.validation_current {
                        StatusMark::Success
                    } else {
                        StatusMark::Warning
                    },
                );
            });
        },
    );
    action
}

/// The recorded role, editable where the project can actually change it.
///
/// An Automation environment lock refuses graph mutation and the refusal tells
/// the user to reassign its role first. Without this control that instruction
/// could not be followed, so the lock was a document nothing could ever move,
/// rename, or delete.
fn role_control(
    ui: &mut Ui,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let row_label = messages.text(MessageId::CodeSourceRecordedRole);
    if facts.language != ProjectSourceLanguage::RSpiceAutomation || facts.is_entry {
        property_row(ui, &row_label, &facts.role);
        return None;
    }
    let options = ASSIGNABLE_AUTOMATION_ROLES
        .iter()
        .map(|role| (role_label(messages, *role), role_label(messages, *role)))
        .collect::<Vec<_>>();
    let mut selected = facts.role.clone();
    let changed = crate::workbench::design_system::property_row_combo(
        ui,
        &row_label,
        "source-document-role",
        &mut selected,
        &options,
        facts.block_reason.is_none(),
    );
    changed
        .then(|| {
            ASSIGNABLE_AUTOMATION_ROLES
                .iter()
                .find(|role| role_label(messages, **role) == selected)
                .map(|role| LifecycleAction::AssignRole(*role))
        })
        .flatten()
}

/// The file operations, in the order they are offered.
const FILE_OPERATIONS: [(MessageId, CodeSourceFileAction); 5] = [
    (MessageId::CodeSourceNewFile, CodeSourceFileAction::New),
    (MessageId::CodeSourceRename, CodeSourceFileAction::Rename),
    (MessageId::CodeSourceMove, CodeSourceFileAction::Move),
    (
        MessageId::CodeSourceDuplicate,
        CodeSourceFileAction::Duplicate,
    ),
    (MessageId::CodeSourceDelete, CodeSourceFileAction::Delete),
];

fn operations(
    ui: &mut Ui,
    facts: &DocumentFacts,
    action: &mut LifecycleAction,
    messages: MessageCatalog,
) {
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeSourceFileOperations),
        None,
        |ui| {
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                for (label, candidate) in FILE_OPERATIONS {
                    let refusal = refusal_reason(facts, candidate);
                    let response = Button::new(&messages.text(label))
                        .enabled(refusal.is_none())
                        .destructive(candidate == CodeSourceFileAction::Delete)
                        .show(ui);
                    if response.clicked() {
                        *action = LifecycleAction::Begin(candidate);
                    }
                    if let Some(reason) = refusal {
                        response.on_disabled_hover_text(reason.resolve(messages));
                    }
                }
                if Button::new(&messages.text(MessageId::CodeSourceRevisionHistory))
                    .show(ui)
                    .clicked()
                {
                    *action = LifecycleAction::OpenHistory;
                }
                if facts.language == ProjectSourceLanguage::RSpiceAutomation {
                    let response =
                        Button::new(&messages.text(MessageId::CodeSourceImportDependency))
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
        },
    );
}

/// Why an operation is unavailable, or `None` when it is.
///
/// These are the transaction's own rules restated, not a stricter house rule.
/// Content editability deliberately does not appear: a document can be
/// read-only to type into and still be renamed, moved, or deleted, and gating
/// on it would disable buttons the service would have honoured.
fn refusal_reason(facts: &DocumentFacts, candidate: CodeSourceFileAction) -> Option<Refusal> {
    if let Some(reason) = facts.block_reason {
        return Some(Refusal::Service(reason));
    }
    let rewrites_graph = matches!(
        candidate,
        CodeSourceFileAction::New
            | CodeSourceFileAction::Rename
            | CodeSourceFileAction::Move
            | CodeSourceFileAction::Delete
    );
    if rewrites_graph && facts.is_environment_lock {
        return Some(Refusal::Stated(
            MessageId::CodeSourceEnvironmentLockMutationBlocked,
        ));
    }
    // The root is the closure's entry document. Removing it would leave a
    // bundle with nothing to enter through, so the service refuses.
    (candidate == CodeSourceFileAction::Delete && facts.is_root)
        .then_some(Refusal::Stated(MessageId::CodeSourceRootDeleteBlocked))
}

/// Why an operation is refused, and who owns the wording.
///
/// A rule this surface restates is catalog copy. A rule the source-graph
/// service already answered with is that service's sentence, passed through
/// unaltered so the two can never disagree about the reason.
#[derive(Clone, Copy)]
enum Refusal {
    Stated(MessageId),
    Service(&'static str),
}

impl Refusal {
    fn resolve(self, messages: MessageCatalog) -> String {
        match self {
            Self::Stated(id) => messages.text(id),
            Self::Service(reason) => reason.to_owned(),
        }
    }
}

/// The open file transaction, drawn where the operation buttons are so the
/// proposed path and its failure reason stay together.
fn operation_review(
    ui: &mut Ui,
    state: &mut AppState,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let draft = state.ui.code_workspace.source_file_dialog.as_mut()?;
    let (title, commit) = match draft.action {
        CodeSourceFileAction::New => (
            MessageId::CodeSourceCreateTitle,
            MessageId::CodeSourceCreatePrimary,
        ),
        CodeSourceFileAction::Rename => (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        ),
        CodeSourceFileAction::Move => (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        ),
        CodeSourceFileAction::Duplicate => (
            MessageId::CodeSourceDuplicateTitle,
            MessageId::CodeSourceDuplicatePrimary,
        ),
        CodeSourceFileAction::Delete => (
            MessageId::CodeSourceDeleteTitle,
            MessageId::CodeSourceDeletePrimary,
        ),
    };
    let deleting = draft.action == CodeSourceFileAction::Delete;
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(title),
        Some((
            &messages.text(MessageId::CodeSourceOpenTransaction),
            t.color.accent,
        )),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                if deleting {
                    property_row(
                        ui,
                        &messages.text(MessageId::CodeSourceRemoving),
                        &draft.source_path,
                    );
                } else {
                    property_row_input(
                        ui,
                        &messages.text(MessageId::CodeSourceLogicalPath),
                        &mut draft.proposed_path,
                        draft.error.is_some(),
                    );
                }
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceAgainst),
                    &messages.format(
                        MessageId::CodeSourceAgainstRevision,
                        &[
                            ("revision", &draft.bundle_revision.to_string()),
                            ("digest", &short_digest(draft.closure_digest)),
                        ],
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
                if Button::new(&messages.text(commit))
                    .accent()
                    .destructive(deleting)
                    .enabled(deleting || !draft.proposed_path.trim().is_empty())
                    .show(ui)
                    .clicked()
                {
                    action = Some(LifecycleAction::Commit);
                }
                if Button::new(&messages.text(MessageId::CommonCancel))
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    action = Some(LifecycleAction::Cancel);
                }
                if deleting {
                    ui.label(
                        RichText::new(messages.text(MessageId::CodeSourceDeleteWarning))
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

fn includes(ui: &mut Ui, facts: &DocumentFacts, messages: MessageCatalog) {
    let t = Tokens::get(ui.ctx());
    let total = facts.imports.len() + facts.imported_by.len();
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeSourceRecordedIncludes),
        Some((&total.to_string(), t.color.text_dim)),
        |ui| {
            if total == 0 {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(messages.text(MessageId::CodeSourceNoIncludes))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(6.0);
                return;
            }
            code_inspector_property_list(ui, |ui| {
                let imports = messages.text(MessageId::CodeSourceImports);
                let imported_by = messages.text(MessageId::CodeSourceImportedBy);
                for imported in &facts.imports {
                    property_row(ui, &imports, imported);
                }
                for importer in &facts.imported_by {
                    property_row(ui, &imported_by, importer);
                }
            });
        },
    );
}

fn revision_history(
    ui: &mut Ui,
    state: &mut AppState,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let history = state.ui.code_workspace.source_history.as_ref()?;
    let t = Tokens::get(ui.ctx());
    let selected = history.selected_revision;
    let error = history.error.clone();
    let mut action = None;
    code_inspector_section(
        ui,
        &messages.text(MessageId::CodeSourceRetainedRevisions),
        Some((&facts.history.len().to_string(), t.color.text_dim)),
        |ui| {
            if facts.history.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(messages.text(MessageId::CodeSourceNoRetainedRevision))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(6.0);
                return;
            }
            ui.add_space(4.0);
            section_header(
                ui,
                &messages.text(MessageId::CodeSourceRetainedClosures),
                Some(&messages.text(MessageId::CodeSourceNewestLast)),
            );
            Grid::new("source-document-revisions")
                .num_columns(4)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for row in &facts.history {
                        let documents = messages.format(
                            if row.files == 1 {
                                MessageId::CodeSourceDocumentSingular
                            } else {
                                MessageId::CodeSourceDocuments
                            },
                            &[("count", &row.files.to_string())],
                        );
                        // Restoring a closure is chosen from this row, and the
                        // row's digest, size, and currency live in cells that
                        // never take focus. The selectable cell says all of it.
                        let response = ui.selectable_label(
                            selected == Some(row.revision),
                            RichText::new(messages.format(
                                MessageId::CodeSourceRevision,
                                &[("revision", &row.revision.to_string())],
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
                        );
                        let announced = messages.format(
                            if row.current {
                                MessageId::CodeSourceRevisionAccessibleCurrent
                            } else {
                                MessageId::CodeSourceRevisionAccessible
                            },
                            &[
                                ("revision", &row.revision.to_string()),
                                ("digest", &row.digest),
                                ("documents", &documents),
                            ],
                        );
                        let chosen = selected == Some(row.revision);
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                true,
                                chosen,
                                &announced,
                            )
                        });
                        if response.clicked() {
                            action = Some(LifecycleAction::SelectRevision(row.revision));
                        }
                        ui.label(
                            RichText::new(&row.digest)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(&documents)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(if row.current {
                                messages.text(MessageId::CodeSourceCurrentClosure)
                            } else {
                                String::new()
                            })
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
                let response =
                    Button::new(&messages.text(MessageId::CodeSourceRestoreAsNewRevision))
                        .accent()
                        .enabled(selected.is_some() && facts.block_reason.is_none())
                        .show(ui);
                if response.clicked() {
                    action = Some(LifecycleAction::Restore);
                }
                if let Some(reason) = facts.block_reason {
                    response.on_disabled_hover_text(reason);
                }
                if Button::new(&messages.text(MessageId::CodeSourceCloseHistory))
                    .ghost()
                    .show(ui)
                    .clicked()
                {
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
    fn english() -> MessageCatalog {
        MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
    }

    #[test]
    fn line_endings_are_measured_rather_than_assumed() {
        let english = english();
        let summary = |content| english.text(line_ending_summary(content));
        assert_eq!(summary("a\nb\n"), "LF");
        assert_eq!(summary("a\r\nb\r\n"), "CRLF");
        assert_eq!(summary("a\r\nb\n"), "mixed LF and CRLF");
        assert_eq!(summary("single"), "none \u{00b7} single line");
    }

    /// The role picker resolves a selection by matching the label it drew, so
    /// every assignable role has to round-trip through its own text in every
    /// locale — including the one whose expansion rewrites every vowel.
    #[test]
    fn every_assignable_role_round_trips_through_its_label_in_every_locale() {
        for locale in crate::workbench::UiTextLocale::ALL {
            let messages = MessageCatalog::new(locale);
            let mut drawn = std::collections::BTreeSet::new();
            for role in ASSIGNABLE_AUTOMATION_ROLES {
                let label = role_label(messages, role);
                assert!(
                    drawn.insert(label.clone()),
                    "two roles share the label {label:?} in {locale:?}"
                );
                assert_eq!(
                    ASSIGNABLE_AUTOMATION_ROLES
                        .iter()
                        .find(|candidate| role_label(messages, **candidate) == label)
                        .copied(),
                    Some(role)
                );
            }
        }
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
        let facts = document_facts(&app, english()).expect("the page states a document");
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

        let before = document_facts(&app, english()).expect("the lock is a document");
        assert!(before.is_environment_lock);
        assert!(
            refusal_reason(&before, CodeSourceFileAction::Rename).is_some(),
            "the lock refuses graph mutation"
        );

        app.apply_lifecycle_action(LifecycleAction::AssignRole(None), &before);

        let after = document_facts(&app, english()).expect("the document survives the role change");
        assert!(!after.is_environment_lock, "the role was reassigned");
        assert!(
            refusal_reason(&after, CodeSourceFileAction::Rename).is_none(),
            "the refusal the service states is now followable"
        );
    }

    /// An empty project exposes the lifecycle manager so the first top deck
    /// can be created without a generated or imported predecessor.
    #[test]
    fn an_unowned_netlist_page_can_open_the_top_deck_lifecycle() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
        assert!(source_document_dialog_is_available(&app.state));
        open_source_document_dialog(&mut app.state).unwrap();
        assert!(app.state.ui.code_workspace.source_document_dialog);
        let facts = netlist_lifecycle_facts(&app, english()).unwrap();
        assert!(facts.deck_id.is_none());
        assert!(facts.decks.is_empty());
    }
}
