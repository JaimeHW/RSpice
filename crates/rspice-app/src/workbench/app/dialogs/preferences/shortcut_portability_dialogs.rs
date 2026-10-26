//! Exact, transactional shortcut import/export workflows from the Preferences mockup.
//!
//! This module owns the retained dialog drafts and produces typed effects for
//! source selection, persistence and publication. It never mutates the live
//! shortcut library itself: the application owner must durably complete an
//! effect and report its result before the dialog advances.

use egui::{Context, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::state::command_catalog::{CommandPlatform, ShortcutContext};
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogSize, DialogTransactionTone, Problem, select,
};
use crate::workbench::app::dialogs::review_primitives::problem_note;
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::shortcuts::artifacts::{
    DecodedShortcutArtifact, DetectedShortcutArtifact, ImportBindingClass,
    PreparedShortcutArtifact, ReadyShortcutArtifactSource, ShortcutArtifactExportOutcome,
    ShortcutArtifactFormat, ShortcutConflictPolicy, ShortcutExportRequest, ShortcutExportScope,
    ShortcutImportOptions, ShortcutImportPlan, ShortcutImportReceipt, ShortcutMergePolicy,
    VscodeImportReport, build_shortcut_reference_model, plan_shortcut_import,
    prepare_shortcut_artifact, shortcut_library_digest,
};

use super::preferences_shell::{control_width, preference_switch, value_column};

pub(crate) const IMPORT_TITLE: &str = "Import keyboard shortcut map";
pub(crate) const IMPORT_PRIMARY: &str = "Import compatible bindings";
pub(crate) const EXPORT_TITLE: &str = "Export keyboard shortcut map";
pub(crate) const EXPORT_PRIMARY: &str = "Export shortcut profile";

pub const IMPORT_CLASSES: [ImportBindingClass; 4] = [
    ImportBindingClass::Global,
    ImportBindingClass::Schematic,
    ImportBindingClass::Results,
    ImportBindingClass::Simulation,
];
/// How an import combines with the current bindings. Importing into a named
/// preset is not offered: nothing in RSpice lists, switches to or deletes a
/// named preset, so one written there could never be used.
pub const MERGE_POLICIES: [ShortcutMergePolicy; 2] = [
    ShortcutMergePolicy::MergeNonConflicting,
    ShortcutMergePolicy::ReplaceCurrentUserBindings,
];
pub const CONFLICT_POLICIES: [ShortcutConflictPolicy; 3] = [
    ShortcutConflictPolicy::KeepCurrentAndReport,
    ShortcutConflictPolicy::UseImportedBinding,
    ShortcutConflictPolicy::LeaveBothUnbound,
];
const EXPORT_SCOPES: [ShortcutExportScope; 3] = [
    ShortcutExportScope::UserOverrides,
    ShortcutExportScope::CompleteResolved,
    ShortcutExportScope::CurrentWorkspace,
];

// Exact non-commercial workflow grammar from the mockup stylesheet.
const WORKFLOW_ROW_MIN_HEIGHT: f32 = 54.0;
const WORKFLOW_ROW_LABEL_FR: f32 = 0.38;
const WORKFLOW_ROW_VALUE_FR: f32 = 1.0;
const WORKFLOW_ROW_COLUMN_GAP: f32 = 12.0;
const WORKFLOW_ROW_HORIZONTAL_PADDING: i8 = 12;
const WORKFLOW_ROW_VERTICAL_PADDING: i8 = 10;
const WORKFLOW_ROW_STACK_BREAKPOINT: f32 = 560.0;
const WORKFLOW_TABLE_COLLAPSE_WIDTH: f32 = 660.0;
pub const IMPORT_SOURCE_PLACEHOLDER: &str = "~/Downloads/rspice-shortcuts.json";

/// Runtime context required to project an export accurately.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ShortcutExportEnvironment<'a> {
    pub(crate) runtime_platform: CommandPlatform,
    pub(crate) operating_system: egui::os::OperatingSystem,
    pub(crate) current_contexts: &'a [ShortcutContext],
}

/// A fully typed side effect. The caller must report completion through the
/// corresponding `complete_*` method; an emitted effect is never a success
/// claim by itself.
#[derive(Debug, Default)]
pub(crate) enum ShortcutPortabilityAction {
    #[default]
    None,
    SelectImportSource,
    CancelImportSource,
    CommitImport(Box<ShortcutImportPlan>),
    CancelImportCommit,
    PublishExport(Box<PreparedShortcutArtifact>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ImportPending {
    #[default]
    None,
    Source,
    Commit,
}

/// Retained, isolated import review. No field contains a source path or source
/// bytes; the decoded artifact retains only the normalized file name and
/// content digest required by the import receipt.
#[derive(Debug, Clone)]
pub(crate) struct ShortcutImportDialogState {
    pub open: bool,
    pub artifact: Option<DecodedShortcutArtifact>,
    pub vscode_report: Option<VscodeImportReport>,
    pub(crate) options: ShortcutImportOptions,
    pub(crate) plan: Option<ShortcutImportPlan>,
    pub(crate) plan_dirty: bool,
    pub pending: ImportPending,
    pub cancel_requested: bool,
    pub discard_confirmation: bool,
    pub error: Option<String>,
    pub last_receipt: Option<ShortcutImportReceipt>,
}

impl Default for ShortcutImportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact: None,
            vscode_report: None,
            options: ShortcutImportOptions::default(),
            plan: None,
            plan_dirty: false,
            pending: ImportPending::None,
            cancel_requested: false,
            discard_confirmation: false,
            error: None,
            last_receipt: None,
        }
    }
}

impl ShortcutImportDialogState {
    #[must_use]
    pub(crate) const fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn open(&mut self) {
        let last_receipt = self.last_receipt.take();
        *self = Self {
            open: true,
            last_receipt,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        let last_receipt = self.last_receipt.take();
        *self = Self {
            last_receipt,
            ..Self::default()
        };
    }

    fn has_unsaved_changes(&self) -> bool {
        self.artifact.is_some()
            || self.vscode_report.is_some()
            || self.options != ShortcutImportOptions::default()
    }

    pub fn request_idle_cancel(&mut self) {
        debug_assert_eq!(self.pending, ImportPending::None);
        if self.has_unsaved_changes() && !self.discard_confirmation {
            self.discard_confirmation = true;
        } else {
            self.close();
        }
    }

    /// Accept the completed source picker result and prepare a fresh plan.
    /// VS Code sources are fail-closed unless their adapter report is lossless.
    pub(crate) fn accept_source(
        &mut self,
        base: &ShortcutProfileLibrary,
        source: ReadyShortcutArtifactSource,
        registry: &Registry,
    ) {
        if self.pending != ImportPending::Source || self.cancel_requested {
            self.close();
            return;
        }
        self.pending = ImportPending::None;
        self.accept_detected(base, source.into_detected(), registry);
    }

    pub(crate) fn source_cancelled(&mut self) {
        if self.pending != ImportPending::Source {
            return;
        }
        self.pending = ImportPending::None;
        if self.cancel_requested {
            self.close();
        }
    }

    pub(crate) fn source_failed(&mut self, error: impl Into<String>) {
        if self.pending != ImportPending::Source {
            return;
        }
        self.pending = ImportPending::None;
        if self.cancel_requested {
            self.close();
        } else {
            self.error = Some(error.into());
        }
    }

    pub(crate) fn accept_detected(
        &mut self,
        base: &ShortcutProfileLibrary,
        detected: DetectedShortcutArtifact,
        registry: &Registry,
    ) {
        // A protected acknowledgement belongs to exactly one reviewed source.
        // Replacing the source must never carry consent to a different digest.
        self.options.protected_confirmations.clear();
        let (artifact, report) = match detected {
            DetectedShortcutArtifact::RSpice(native) => (native.into_artifact(), None),
            DetectedShortcutArtifact::Vscode(adaptation) => {
                let (artifact, report) = adaptation.into_parts();
                if !report.is_importable() {
                    self.artifact = None;
                    self.plan = None;
                    self.vscode_report = Some(report.clone());
                    self.error = Some(vscode_blocking_message(&report));
                    return;
                }
                (artifact, Some(report))
            }
        };
        self.artifact = Some(artifact);
        self.vscode_report = report;
        self.discard_confirmation = false;
        self.error = None;
        self.plan_dirty = true;
        self.refresh_plan(base, registry);
    }

    pub fn refresh_plan(&mut self, base: &ShortcutProfileLibrary, registry: &Registry) {
        let Some(artifact) = self.artifact.as_ref() else {
            self.plan = None;
            self.plan_dirty = false;
            return;
        };
        match plan_shortcut_import(base, artifact, &self.options, registry) {
            Ok(plan) => {
                self.plan = Some(plan);
                self.error = None;
            }
            Err(error) => {
                self.plan = None;
                self.error = Some(error.to_string());
            }
        }
        self.plan_dirty = false;
    }

    pub(crate) fn synchronize_plan(&mut self, base: &ShortcutProfileLibrary, registry: &Registry) {
        if self.pending != ImportPending::None || self.artifact.is_none() {
            return;
        }
        let stale = match (&self.plan, shortcut_library_digest(base)) {
            (Some(plan), Ok(digest)) => {
                plan.base_revision() != base.revision() || plan.base_digest() != digest
            }
            (None, _) => self.error.is_none(),
            (_, Err(error)) => {
                self.plan = None;
                self.error = Some(error.to_string());
                false
            }
        };
        if self.plan_dirty || stale {
            self.refresh_plan(base, registry);
        }
    }

    pub(crate) fn complete_commit(&mut self, result: Result<ShortcutImportReceipt, String>) {
        if self.pending != ImportPending::Commit {
            return;
        }
        self.pending = ImportPending::None;
        self.cancel_requested = false;
        match result {
            Ok(receipt) => {
                self.last_receipt = Some(receipt);
                self.open = false;
                self.artifact = None;
                self.vscode_report = None;
                self.plan = None;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// Report that a pending commit was cancelled before publication.
    pub(crate) fn commit_cancelled(&mut self, result: Result<(), String>) {
        if self.pending != ImportPending::Commit || !self.cancel_requested {
            return;
        }
        match result {
            Ok(()) => {
                self.cancel_requested = false;
                self.pending = ImportPending::None;
                self.close();
            }
            Err(error) => {
                // Publication may already have crossed its commit boundary.
                // Keep owning the operation so its eventual completion can
                // still install the receipt and close the review correctly.
                // The retained cancellation flag also prevents route-loss
                // cleanup from dispatching duplicate cancellation effects.
                self.error = Some(error);
            }
        }
    }
}

/// Retained export choices and publication state.
#[derive(Debug, Clone)]
pub(crate) struct ShortcutExportDialogState {
    open: bool,
    format: ShortcutArtifactFormat,
    pub scope: ShortcutExportScope,
    include_platform_mappings: bool,
    pub pending: bool,
    pub discard_confirmation: bool,
    error: Option<String>,
}

impl Default for ShortcutExportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            format: ShortcutArtifactFormat::Json,
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: true,
            pending: false,
            discard_confirmation: false,
            error: None,
        }
    }
}

impl ShortcutExportDialogState {
    #[must_use]
    pub(crate) const fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn open(&mut self) {
        *self = Self {
            open: true,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    fn has_unsaved_changes(&self) -> bool {
        let defaults = Self::default();
        self.format != defaults.format
            || self.scope != defaults.scope
            || self.include_platform_mappings != defaults.include_platform_mappings
    }

    pub fn request_idle_cancel(&mut self) {
        debug_assert!(!self.pending);
        if self.has_unsaved_changes() && !self.discard_confirmation {
            self.discard_confirmation = true;
        } else {
            self.close();
        }
    }

    pub(crate) fn complete_publication(
        &mut self,
        result: Result<ShortcutArtifactExportOutcome, String>,
    ) {
        if !self.pending {
            return;
        }
        self.pending = false;
        match result {
            Ok(ShortcutArtifactExportOutcome::Cancelled) => self.error = None,
            #[cfg(any(test, not(target_arch = "wasm32")))]
            Ok(ShortcutArtifactExportOutcome::Published { .. }) => self.close(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Ok(ShortcutArtifactExportOutcome::DownloadStarted { .. }) => self.close(),
            Err(error) => self.error = Some(error),
        }
    }
}

/// The mutually-exclusive Preferences portability workflows.
#[derive(Debug, Clone, Default)]
pub(crate) struct ShortcutPortabilityDialogsState {
    pub import: ShortcutImportDialogState,
    pub export: ShortcutExportDialogState,
}

impl ShortcutPortabilityDialogsState {
    #[must_use]
    pub(crate) const fn application_modal_open(&self) -> bool {
        self.import.is_open() || self.export.is_open()
    }

    pub(crate) fn open_import(&mut self) {
        if self.application_modal_open() {
            return;
        }
        self.import.open();
    }

    pub(crate) fn open_export(&mut self) {
        if self.application_modal_open() {
            return;
        }
        self.export.open();
    }

    /// Release the retained modal when canonical navigation removes its
    /// Preferences owner. Asynchronous source selection and durable import
    /// publication retain ownership until their exact cancellation or
    /// completion result arrives; idle dialogs close immediately.
    pub(crate) fn request_route_close(&mut self) -> ShortcutPortabilityAction {
        if self.import.is_open() {
            return match self.import.pending {
                ImportPending::None => {
                    self.import.close();
                    ShortcutPortabilityAction::None
                }
                ImportPending::Source if !self.import.cancel_requested => {
                    self.import.cancel_requested = true;
                    ShortcutPortabilityAction::CancelImportSource
                }
                ImportPending::Commit if !self.import.cancel_requested => {
                    self.import.cancel_requested = true;
                    ShortcutPortabilityAction::CancelImportCommit
                }
                ImportPending::Source | ImportPending::Commit => ShortcutPortabilityAction::None,
            };
        }
        if self.export.is_open() && !self.export.pending {
            self.export.close();
        }
        ShortcutPortabilityAction::None
    }

    pub(crate) fn accept_import_source(
        &mut self,
        base: &ShortcutProfileLibrary,
        source: ReadyShortcutArtifactSource,
        registry: &Registry,
    ) {
        self.import.accept_source(base, source, registry);
    }

    pub(crate) fn import_source_cancelled(&mut self) {
        self.import.source_cancelled();
    }

    pub(crate) fn import_source_failed(&mut self, error: impl Into<String>) {
        self.import.source_failed(error);
    }

    pub(crate) fn complete_import(&mut self, result: Result<ShortcutImportReceipt, String>) {
        self.import.complete_commit(result);
    }

    pub(crate) fn import_commit_cancelled(&mut self, result: Result<(), String>) {
        self.import.commit_cancelled(result);
    }

    pub(crate) fn complete_export(
        &mut self,
        result: Result<ShortcutArtifactExportOutcome, String>,
    ) {
        self.export.complete_publication(result);
    }

    pub(crate) fn render(
        &mut self,
        ctx: &Context,
        library: &ShortcutProfileLibrary,
        environment: ShortcutExportEnvironment<'_>,
        registry: &Registry,
    ) -> ShortcutPortabilityAction {
        if self.import.is_open() {
            render_import_dialog(ctx, &mut self.import, library, registry)
        } else if self.export.is_open() {
            render_export_dialog(
                ctx,
                &mut self.export,
                library.active(),
                environment,
                registry,
            )
        } else {
            ShortcutPortabilityAction::None
        }
    }
}

fn render_import_dialog(
    ctx: &Context,
    state: &mut ShortcutImportDialogState,
    base: &ShortcutProfileLibrary,
    registry: &Registry,
) -> ShortcutPortabilityAction {
    state.synchronize_plan(base, registry);
    let can_apply = state.pending == ImportPending::None
        && state
            .plan
            .as_ref()
            .is_some_and(ShortcutImportPlan::can_apply);
    let mut action = ShortcutPortabilityAction::None;
    let cancel_label = if state.discard_confirmation {
        "Discard changes"
    } else {
        "Cancel"
    };
    let transaction_error = state.error.clone();
    let cancel_enabled = match state.pending {
        ImportPending::None => true,
        ImportPending::Source | ImportPending::Commit => !state.cancel_requested,
    };
    let mut dialog = Dialog::new(IMPORT_TITLE, IMPORT_PRIMARY)
        .description(
            "Bring in key bindings from a shortcut file after checking them for conflicts.",
        )
        .size(DialogSize::Transaction)
        .primary_enabled(can_apply)
        .ghost(cancel_label)
        .ghost_enabled(cancel_enabled)
        .flush_body();
    if state.discard_confirmation {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Shortcut import changes are not saved",
            "Choose Discard changes to close without importing them, or continue editing.",
        );
    } else if let Some(error) = transaction_error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Shortcut import requires attention",
            error,
        );
    } else {
        dialog = match state.pending {
            ImportPending::Source => dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Choosing a shortcut file",
                "Waiting for the file picker.",
            ),
            ImportPending::Commit => dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Saving shortcuts\u{2026}",
                "Your current shortcuts stay in effect until the import is saved.",
            ),
            ImportPending::None => dialog,
        };
    }
    let choice = dialog.show(ctx, |ui| {
        render_latest_import_receipt(ui, state);
        if render_import_body(ui, state, base, registry) {
            state.pending = ImportPending::Source;
            state.cancel_requested = false;
            state.discard_confirmation = false;
            state.error = None;
            action = ShortcutPortabilityAction::SelectImportSource;
        }
    });

    match choice {
        DialogChoice::Primary if can_apply => {
            if let Some(plan) = state.plan.clone() {
                state.pending = ImportPending::Commit;
                state.cancel_requested = false;
                action = ShortcutPortabilityAction::CommitImport(Box::new(plan));
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => match state.pending {
            ImportPending::Source if !state.cancel_requested => {
                state.cancel_requested = true;
                action = ShortcutPortabilityAction::CancelImportSource;
            }
            ImportPending::Commit if !state.cancel_requested => {
                state.cancel_requested = true;
                action = ShortcutPortabilityAction::CancelImportCommit;
            }
            ImportPending::None => state.request_idle_cancel(),
            ImportPending::Source | ImportPending::Commit => {}
        },
        DialogChoice::None | DialogChoice::Secondary | DialogChoice::Primary => {}
    }
    action
}

fn render_latest_import_receipt(ui: &mut Ui, state: &ShortcutImportDialogState) {
    let Some(receipt) = state.last_receipt.as_ref() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    // What was imported, and from where. That it completed, and the id it
    // completed under, are not facts the reader can use.
    let response = ui.label(
        egui::RichText::new(format!("Imported from {}", receipt.source_name()))
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label("Latest shortcut import");
    });
}

/// Returns true when the source well was activated.
pub fn render_import_body(
    ui: &mut Ui,
    state: &mut ShortcutImportDialogState,
    base: &ShortcutProfileLibrary,
    registry: &Registry,
) -> bool {
    let controls_enabled = state.pending == ImportPending::None;
    let source_requested = workflow_row(
        ui,
        "Source",
        "An RSpice shortcut file or a VS Code keybindings file.",
        |ui| {
            value_column(ui, |ui| {
                source_well(
                    ui,
                    state
                        .artifact
                        .as_ref()
                        .map(DecodedShortcutArtifact::source_name),
                    controls_enabled,
                )
            })
        },
    );

    let mut options_changed = false;
    workflow_row(
        ui,
        "Current bindings",
        "Protected system and accessibility shortcuts always ask first.",
        |ui| {
            value_column(ui, |ui| {
                let labels = MERGE_POLICIES
                    .iter()
                    .map(|policy| policy.label().to_owned())
                    .collect::<Vec<_>>();
                let selected = ui
                    .add_enabled_ui(controls_enabled, |ui| {
                        select(
                            ui,
                            "preferences.shortcuts.import.merge-policy",
                            "Current bindings",
                            state.options.merge_policy.label(),
                            &labels,
                            control_width(ui),
                        )
                    })
                    .inner;
                if let Some(index) = selected {
                    state.options.merge_policy = MERGE_POLICIES[index];
                    options_changed = true;
                }
            });
        },
    );

    render_import_summary(ui, state.plan.as_ref(), state.options.conflict_policy);

    workflow_row(ui, "Conflict handling", "", |ui| {
        value_column(ui, |ui| {
            let labels = CONFLICT_POLICIES
                .iter()
                .map(|policy| policy.label().to_owned())
                .collect::<Vec<_>>();
            let selected = ui
                .add_enabled_ui(controls_enabled, |ui| {
                    select(
                        ui,
                        "preferences.shortcuts.import.conflict-policy",
                        "Conflict handling",
                        state.options.conflict_policy.label(),
                        &labels,
                        control_width(ui),
                    )
                })
                .inner;
            if let Some(index) = selected {
                state.options.conflict_policy = CONFLICT_POLICIES[index];
                options_changed = true;
            }
        });
    });

    let required = state
        .plan
        .as_ref()
        .map(|plan| {
            plan.required_protected_confirmations()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !required.is_empty() {
        workflow_row(
            ui,
            "Protected bindings",
            "Confirm each protected accessibility or operating-system binding explicitly.",
            |ui| {
                ui.vertical(|ui| {
                    for command_id in &required {
                        let label = registry
                            .resolve(command_id)
                            .map_or(command_id.as_str(), |command| registry.label(command));
                        let mut confirmed =
                            state.options.protected_confirmations.contains(command_id);
                        if ui
                            .add_enabled_ui(controls_enabled, |ui| {
                                ui.checkbox(&mut confirmed, format!("Confirm {label}"))
                            })
                            .inner
                            .changed()
                        {
                            if confirmed {
                                state
                                    .options
                                    .protected_confirmations
                                    .insert(command_id.clone());
                            } else {
                                state.options.protected_confirmations.remove(command_id);
                            }
                            options_changed = true;
                        }
                    }
                });
            },
        );
    }

    if let Some(report) = state.vscode_report.as_ref() {
        let message = format!(
            "Editor preset mapping v{}: {} imported, {} unmapped, {} rejected, {} conflicts.",
            report.mapping_version,
            report.imported_entries,
            report.unmapped_entries,
            report.rejected_entries,
            report.conflicting_entries
        );
        validation_message(ui, &message, report.has_errors());
    }
    if state.error.is_none()
        && let Some(artifact) = state.artifact.as_ref()
        && !artifact.warnings().is_empty()
    {
        validation_message(ui, &artifact.warnings().join(" "), false);
    }

    if options_changed {
        state.discard_confirmation = false;
        state.plan_dirty = true;
        state.refresh_plan(base, registry);
    }
    source_requested
}

fn render_import_summary(
    ui: &mut Ui,
    plan: Option<&ShortcutImportPlan>,
    conflict_policy: ShortcutConflictPolicy,
) {
    egui::ScrollArea::horizontal()
        .id_salt("shortcut-import-summary-scroll")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(WORKFLOW_TABLE_COLLAPSE_WIDTH);
            let table = ui
                .scope(|ui| {
                    summary_table_row(
                        ui,
                        true,
                        [
                            ("Binding class", TableCellTone::Default),
                            ("Imported", TableCellTone::Default),
                            ("Conflicts", TableCellTone::Default),
                            ("Policy", TableCellTone::Default),
                        ],
                    );
                    for class in IMPORT_CLASSES {
                        let summary = plan.and_then(|plan| {
                            plan.summaries()
                                .iter()
                                .find(|summary| summary.binding_class == Some(class))
                        });
                        let imported = summary.map_or(0, |summary| summary.imported);
                        let conflicts = summary.map_or(0, |summary| summary.conflicts);
                        let (policy, warning) = import_policy_label(conflicts, conflict_policy);
                        let imported = imported.to_string();
                        let conflicts = conflicts.to_string();
                        summary_table_row(
                            ui,
                            false,
                            [
                                (class.label(), TableCellTone::Default),
                                (&imported, TableCellTone::Default),
                                (&conflicts, TableCellTone::Default),
                                (
                                    policy,
                                    if warning {
                                        TableCellTone::Warning
                                    } else {
                                        TableCellTone::Default
                                    },
                                ),
                            ],
                        );
                    }
                })
                .response;
            ui.ctx().accesskit_node_builder(table.id, |node| {
                node.set_role(egui::accesskit::Role::Table);
                node.set_label("Shortcut import compatibility summary");
            });
        });
}

/// A summary cell's severity. A policy that resolves cleanly is an ordinary
/// cell; only one the reader has to look at carries a mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableCellTone {
    Default,
    Warning,
}

fn summary_table_row(ui: &mut Ui, heading: bool, cells: [(&str, TableCellTone); 4]) {
    let t = Tokens::get(ui.ctx());
    let height = if heading { 27.0 } else { t.metrics.row_h };
    let width = ui.available_width().max(WORKFLOW_TABLE_COLLAPSE_WIDTH);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            cells
                .iter()
                .map(|(text, _)| *text)
                .collect::<Vec<_>>()
                .join("; "),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    if heading {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, translucent(t.color.border, 191)),
    );

    let cell_width = rect.width() / cells.len() as f32;
    for (index, (text, tone)) in cells.into_iter().enumerate() {
        let left = rect.left() + index as f32 * cell_width;
        let right = if index + 1 == 4 {
            rect.right()
        } else {
            left + cell_width
        };
        let cell_rect = egui::Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        let content_rect = cell_rect.shrink2(vec2(8.0, 0.0));
        let mut cell_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        cell_ui.set_clip_rect(cell_rect.intersect(ui.clip_rect()));
        let color = if heading {
            t.color.text_faint
        } else {
            match tone {
                TableCellTone::Default => t.color.text_dim,
                TableCellTone::Warning => t.color.warn,
            }
        };
        let response = if heading {
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &text.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_0, FontWeight::Medium),
                    color,
                    extra_letter_spacing: 0.04 * tokens::FS_0,
                    ..Default::default()
                },
            );
            cell_ui.add_sized(
                content_rect.size(),
                egui::Label::new(job).truncate().selectable(true),
            )
        } else {
            cell_ui.add_sized(
                content_rect.size(),
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(color),
                )
                .truncate()
                .selectable(true),
            )
        };
        cell_ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(if heading {
                egui::accesskit::Role::ColumnHeader
            } else {
                egui::accesskit::Role::Cell
            });
        });
    }
}

fn import_policy_label(conflicts: usize, policy: ShortcutConflictPolicy) -> (&'static str, bool) {
    if conflicts == 0 {
        return ("ready", false);
    }
    match policy {
        ShortcutConflictPolicy::KeepCurrentAndReport => ("review", true),
        ShortcutConflictPolicy::UseImportedBinding => ("use imported", true),
        ShortcutConflictPolicy::LeaveBothUnbound => ("leave unbound", true),
    }
}

fn render_export_dialog(
    ctx: &Context,
    state: &mut ShortcutExportDialogState,
    profile: &ShortcutPreferences,
    environment: ShortcutExportEnvironment<'_>,
    registry: &Registry,
) -> ShortcutPortabilityAction {
    let mut action = ShortcutPortabilityAction::None;
    let cancel_label = if state.discard_confirmation {
        "Discard changes"
    } else {
        "Cancel"
    };
    let transaction_error = state.error.clone();
    let mut dialog = Dialog::new(EXPORT_TITLE, EXPORT_PRIMARY)
        .description("Share key bindings between machines or with your team.")
        .size(DialogSize::Transaction)
        .primary_enabled(!state.pending)
        .ghost(cancel_label)
        .ghost_enabled(!state.pending)
        .flush_body();
    if state.discard_confirmation {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Shortcut export choices are not saved",
            "Choose Discard changes to close without exporting them, or continue editing.",
        );
    } else if let Some(error) = transaction_error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Shortcut export failed",
            error,
        );
    } else if state.pending {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Progress,
            "Saving shortcut file\u{2026}",
            "Waiting for you to choose where to save it.",
        );
    }
    let choice = dialog.show(ctx, |ui| render_export_body(ui, state));

    match choice {
        DialogChoice::Primary if !state.pending => {
            let request = ShortcutExportRequest {
                scope: state.scope,
                include_platform_mappings: state.include_platform_mappings,
                runtime_platform: environment.runtime_platform,
                operating_system: environment.operating_system,
                current_contexts: environment.current_contexts.to_vec(),
            };
            let prepared = build_shortcut_reference_model(profile, &request, registry)
                .map_err(|error| error.to_string())
                .and_then(|model| {
                    prepare_shortcut_artifact(&model, state.format, None)
                        .map_err(|error| error.to_string())
                });
            match prepared {
                Ok(artifact) => {
                    state.pending = true;
                    state.error = None;
                    action = ShortcutPortabilityAction::PublishExport(Box::new(artifact));
                }
                Err(error) => state.error = Some(error.to_string()),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled if !state.pending => {
            state.request_idle_cancel();
        }
        DialogChoice::None
        | DialogChoice::Secondary
        | DialogChoice::Primary
        | DialogChoice::Ghost
        | DialogChoice::Cancelled => {}
    }
    action
}

fn render_export_body(ui: &mut Ui, state: &mut ShortcutExportDialogState) {
    let controls_enabled = !state.pending;
    workflow_row(
        ui,
        "Format",
        "JSON imports back into RSpice. Markdown and PDF are for reading.",
        |ui| {
            value_column(ui, |ui| {
                let labels = ShortcutArtifactFormat::ALL
                    .iter()
                    .map(|format| export_format_label(*format).to_owned())
                    .collect::<Vec<_>>();
                let selected = ui
                    .add_enabled_ui(controls_enabled, |ui| {
                        select(
                            ui,
                            "preferences.shortcuts.export.format",
                            "Format",
                            export_format_label(state.format),
                            &labels,
                            control_width(ui),
                        )
                    })
                    .inner;
                if let Some(index) = selected {
                    state.format = ShortcutArtifactFormat::ALL[index];
                    state.discard_confirmation = false;
                    state.error = None;
                }
            });
        },
    );
    workflow_row(
        ui,
        "Scope",
        "Your changes only, every binding, or the current workspace's bindings.",
        |ui| {
            value_column(ui, |ui| {
                let labels = EXPORT_SCOPES
                    .iter()
                    .map(|scope| export_scope_label(*scope).to_owned())
                    .collect::<Vec<_>>();
                let selected = ui
                    .add_enabled_ui(controls_enabled, |ui| {
                        select(
                            ui,
                            "preferences.shortcuts.export.scope",
                            "Scope",
                            export_scope_label(state.scope),
                            &labels,
                            control_width(ui),
                        )
                    })
                    .inner;
                if let Some(index) = selected {
                    state.scope = EXPORT_SCOPES[index];
                    state.discard_confirmation = false;
                    state.error = None;
                }
            });
        },
    );
    workflow_row(
        ui,
        "Platform mappings",
        "Keep the separate desktop and browser keys a command may have.",
        |ui| {
            value_column(ui, |ui| {
                let changed = ui
                    .add_enabled_ui(controls_enabled, |ui| {
                        preference_switch(
                        ui,
                        "user-preferences.workflowmarkup.include-platform-shortcut-alternatives.a433d210",
                        "Include platform shortcut alternatives",
                        &mut state.include_platform_mappings,
                        )
                    })
                    .inner;
                if changed {
                    state.discard_confirmation = false;
                    state.error = None;
                }
            });
        },
    );
    workflow_row(
        ui,
        "Privacy",
        "Only key bindings are written. No projects, paths, history or credentials.",
        |ui| {
            value_column(ui, |ui| {
                let t = Tokens::get(ui.ctx());
                ui.label(
                    egui::RichText::new("key bindings only")
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_dim),
                );
            });
        },
    );
}

fn workflow_row<R>(
    ui: &mut Ui,
    title: &str,
    help: &str,
    add_value: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let stacked = ui.ctx().content_rect().width() <= WORKFLOW_ROW_STACK_BREAKPOINT;
    let content_min_height =
        (WORKFLOW_ROW_MIN_HEIGHT - f32::from(WORKFLOW_ROW_VERTICAL_PADDING) * 2.0).max(0.0);
    let response = egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(
            WORKFLOW_ROW_HORIZONTAL_PADDING,
            WORKFLOW_ROW_VERTICAL_PADDING,
        ))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if stacked {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = WORKFLOW_ROW_COLUMN_GAP;
                    workflow_row_copy(ui, title, help);
                    ui.scope(|ui| {
                        ui.set_width(ui.available_width());
                        add_value(ui)
                    })
                    .inner
                })
                .inner
            } else {
                let available = (ui.available_width() - WORKFLOW_ROW_COLUMN_GAP).max(0.0);
                let label_fraction =
                    WORKFLOW_ROW_LABEL_FR / (WORKFLOW_ROW_LABEL_FR + WORKFLOW_ROW_VALUE_FR);
                let label_width = (available * label_fraction).max(150.0).min(available);
                let value_width = (available - label_width).max(0.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = WORKFLOW_ROW_COLUMN_GAP;
                    ui.vertical(|ui| {
                        ui.set_width(label_width);
                        ui.set_min_height(content_min_height);
                        workflow_row_copy(ui, title, help);
                    });
                    ui.vertical(|ui| {
                        ui.set_width(value_width);
                        ui.set_min_height(content_min_height);
                        add_value(ui)
                    })
                    .inner
                })
                .inner
            }
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Group);
            node.set_label(title);
            if !help.is_empty() {
                node.set_description(help);
            }
        });
    response.inner
}

fn workflow_row_copy(ui: &mut Ui, title: &str, help: &str) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 3.0;
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
    );
    if !help.is_empty() {
        ui.add(
            egui::Label::new(
                egui::RichText::new(help)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            )
            .wrap(),
        );
    }
}

fn source_well(ui: &mut Ui, value: Option<&str>, enabled: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = control_width(ui);
    let mut display = value.unwrap_or_default().to_owned();
    let field = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized(
                [width, t.metrics.ctl_h],
                egui::TextEdit::singleline(&mut display)
                    .id_salt("preferences.shortcuts.import.source-display")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .hint_text(IMPORT_SOURCE_PLACEHOLDER)
                    .interactive(false),
            )
        })
        .inner;
    let response = ui.interact(
        field.rect,
        ui.make_persistent_id("preferences.shortcuts.import.source-picker"),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let response = if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, "Source"));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_value(value.unwrap_or(IMPORT_SOURCE_PLACEHOLDER));
        node.set_description("Choose an RSpice shortcut file or a VS Code keybindings file");
    });
    if enabled {
        theme::paint_focus_ring(ui, &response, field.rect);
    }
    enabled && response.clicked()
}

fn validation_message(ui: &mut Ui, message: &str, error: bool) {
    let response = problem_note(
        ui,
        if error {
            Problem::Failure
        } else {
            Problem::Warning
        },
        message,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if error {
            egui::accesskit::Role::Alert
        } else {
            egui::accesskit::Role::Status
        });
        node.set_label(message);
    });
}

fn translucent(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

const fn export_format_label(format: ShortcutArtifactFormat) -> &'static str {
    match format {
        ShortcutArtifactFormat::Json => "RSpice shortcut profile \u{00b7} JSON",
        ShortcutArtifactFormat::Markdown => "Human-readable reference \u{00b7} Markdown",
        ShortcutArtifactFormat::Pdf => "Printable reference \u{00b7} PDF",
    }
}

const fn export_scope_label(scope: ShortcutExportScope) -> &'static str {
    match scope {
        ShortcutExportScope::UserOverrides => "User overrides + platform exceptions",
        ShortcutExportScope::CompleteResolved => "Complete resolved shortcut map",
        ShortcutExportScope::CurrentWorkspace => "Current workspace context only",
    }
}

fn vscode_blocking_message(report: &VscodeImportReport) -> String {
    if report.imported_entries == 0 {
        "The editor preset contains no compatible RSpice command bindings.".to_owned()
    } else {
        format!(
            "The editor preset cannot be imported losslessly: {} unmapped, {} rejected, and {} conflicting entries require review at the source.",
            report.unmapped_entries, report.rejected_entries, report.conflicting_entries
        )
    }
}
