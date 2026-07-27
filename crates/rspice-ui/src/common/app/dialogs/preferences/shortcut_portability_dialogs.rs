//! Exact, transactional shortcut import/export workflows from the Preferences mockup.
//!
//! This module owns the retained dialog drafts and produces typed effects for
//! source selection, persistence and publication. It never mutates the live
//! shortcut library itself: the application owner must durably complete an
//! effect and report its result before the dialog advances.

use egui::{Context, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::common::shortcut_artifacts::{
    DecodedShortcutArtifact, DetectedShortcutArtifact, ImportBindingClass,
    PreparedShortcutArtifact, ReadyShortcutArtifactSource, ShortcutArtifactExportOutcome,
    ShortcutArtifactFormat, ShortcutConflictPolicy, ShortcutExportRequest, ShortcutExportScope,
    ShortcutImportOptions, ShortcutImportPlan, ShortcutImportReceipt, ShortcutMergePolicy,
    VscodeImportReport, build_shortcut_reference_model, plan_shortcut_import,
    prepare_shortcut_artifact, shortcut_library_digest,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, DialogTransactionTone, select};
use crate::workbench::ShortcutPreferences;
use crate::workbench::commands::{Command, CommandPlatform, ShortcutContext};
use crate::workbench::shortcuts::ShortcutProfileLibrary;

use super::preferences_shell::{preference_switch, right_aligned};

pub(crate) const IMPORT_TITLE: &str = "Import keyboard shortcut map";
pub(crate) const IMPORT_EYEBROW: &str = "PREFERENCES \u{00b7} VERSIONED COMMAND BINDINGS";
pub(crate) const IMPORT_PRIMARY: &str = "Import compatible bindings";
pub(crate) const EXPORT_TITLE: &str = "Export keyboard shortcut map";
pub(crate) const EXPORT_EYEBROW: &str = "PREFERENCES \u{00b7} PORTABLE COMMAND BINDINGS";
pub(crate) const EXPORT_PRIMARY: &str = "Export shortcut profile";

const IMPORT_CLASSES: [ImportBindingClass; 4] = [
    ImportBindingClass::Global,
    ImportBindingClass::Schematic,
    ImportBindingClass::Results,
    ImportBindingClass::Simulation,
];
const MERGE_POLICIES: [ShortcutMergePolicy; 3] = [
    ShortcutMergePolicy::MergeNonConflicting,
    ShortcutMergePolicy::ReplaceCurrentUserBindings,
    ShortcutMergePolicy::ImportNamedPreset,
];
const CONFLICT_POLICIES: [ShortcutConflictPolicy; 3] = [
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
const IMPORT_SOURCE_PLACEHOLDER: &str = "~/Downloads/rspice-shortcuts.json";

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
    #[allow(
        dead_code,
        reason = "constructed only by the retained receipt undo owner; the mockup dialog intentionally has no invented rollback control"
    )]
    RollbackImport(Box<ShortcutImportReceipt>),
    PublishExport(Box<PreparedShortcutArtifact>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ImportPending {
    #[default]
    None,
    Source,
    Commit,
    Rollback,
}

/// Retained, isolated import review. No field contains a source path or source
/// bytes; the decoded artifact retains only the normalized file name and
/// content digest required by the import receipt.
#[derive(Debug, Clone)]
pub(crate) struct ShortcutImportDialogState {
    open: bool,
    artifact: Option<DecodedShortcutArtifact>,
    vscode_report: Option<VscodeImportReport>,
    options: ShortcutImportOptions,
    plan: Option<ShortcutImportPlan>,
    plan_dirty: bool,
    pending: ImportPending,
    cancel_requested: bool,
    discard_confirmation: bool,
    error: Option<String>,
    last_receipt: Option<ShortcutImportReceipt>,
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

    fn request_idle_cancel(&mut self) {
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
    ) {
        if self.pending != ImportPending::Source || self.cancel_requested {
            self.close();
            return;
        }
        self.pending = ImportPending::None;
        self.accept_detected(base, source.into_detected());
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

    fn accept_detected(
        &mut self,
        base: &ShortcutProfileLibrary,
        detected: DetectedShortcutArtifact,
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
        self.refresh_plan(base);
    }

    fn refresh_plan(&mut self, base: &ShortcutProfileLibrary) {
        let Some(artifact) = self.artifact.as_ref() else {
            self.plan = None;
            self.plan_dirty = false;
            return;
        };
        match plan_shortcut_import(base, artifact, &self.options) {
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

    fn synchronize_plan(&mut self, base: &ShortcutProfileLibrary) {
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
            self.refresh_plan(base);
        }
    }

    /// Start the receipt's guarded rollback. The live library still remains
    /// untouched until the caller durably completes the returned action.
    #[allow(
        dead_code,
        reason = "called by the contextual receipt undo owner, not rendered as an extra mockup dialog control"
    )]
    pub(crate) fn begin_rollback(&mut self) -> Result<ShortcutPortabilityAction, &'static str> {
        if self.pending != ImportPending::None {
            return Err("a shortcut portability operation is already in progress");
        }
        let Some(receipt) = self.last_receipt.clone() else {
            return Err("there is no retained shortcut import receipt to roll back");
        };
        self.pending = ImportPending::Rollback;
        Ok(ShortcutPortabilityAction::RollbackImport(Box::new(receipt)))
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

    pub(crate) fn complete_rollback(&mut self, result: Result<(), String>) {
        if self.pending != ImportPending::Rollback {
            return;
        }
        self.pending = ImportPending::None;
        match result {
            Ok(()) => {
                self.last_receipt = None;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }
}

/// Retained export choices and publication state.
#[derive(Debug, Clone)]
pub(crate) struct ShortcutExportDialogState {
    open: bool,
    format: ShortcutArtifactFormat,
    scope: ShortcutExportScope,
    include_platform_mappings: bool,
    pending: bool,
    discard_confirmation: bool,
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

    fn request_idle_cancel(&mut self) {
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
            Ok(
                ShortcutArtifactExportOutcome::Published { .. }
                | ShortcutArtifactExportOutcome::DownloadStarted { .. },
            ) => self.close(),
            Err(error) => self.error = Some(error),
        }
    }
}

/// The mutually-exclusive Preferences portability workflows.
#[derive(Debug, Clone, Default)]
pub(crate) struct ShortcutPortabilityDialogsState {
    import: ShortcutImportDialogState,
    export: ShortcutExportDialogState,
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
                ImportPending::Source | ImportPending::Commit | ImportPending::Rollback => {
                    ShortcutPortabilityAction::None
                }
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
    ) {
        self.import.accept_source(base, source);
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

    #[allow(
        dead_code,
        reason = "called by the contextual receipt undo owner, not rendered as an extra mockup dialog control"
    )]
    pub(crate) fn begin_rollback(&mut self) -> Result<ShortcutPortabilityAction, &'static str> {
        self.import.begin_rollback()
    }

    pub(crate) fn complete_rollback(&mut self, result: Result<(), String>) {
        self.import.complete_rollback(result);
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
    ) -> ShortcutPortabilityAction {
        if self.import.is_open() {
            render_import_dialog(ctx, &mut self.import, library)
        } else if self.export.is_open() {
            render_export_dialog(ctx, &mut self.export, library.active(), environment)
        } else {
            ShortcutPortabilityAction::None
        }
    }
}

fn render_import_dialog(
    ctx: &Context,
    state: &mut ShortcutImportDialogState,
    base: &ShortcutProfileLibrary,
) -> ShortcutPortabilityAction {
    state.synchronize_plan(base);
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
        ImportPending::Rollback => false,
    };
    let mut dialog = Dialog::new(IMPORT_EYEBROW, IMPORT_TITLE, IMPORT_PRIMARY)
        .description(
            "Import a versioned shortcut profile after reviewing compatibility, conflicts, and protected bindings.",
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
                "Selecting shortcut source",
                "Waiting for the platform file picker to complete or cancel.",
            ),
            ImportPending::Commit => dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Publishing shortcut profile",
                "The live profile changes only after durable publication succeeds.",
            ),
            ImportPending::Rollback => dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Restoring prior shortcut profile",
                "The retained predecessor is being validated and published.",
            ),
            ImportPending::None => dialog,
        };
    }
    let choice = dialog.show(ctx, |ui| {
        render_latest_import_receipt(ui, state);
        if render_import_body(ui, state, base) {
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
            ImportPending::Source | ImportPending::Commit | ImportPending::Rollback => {}
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
    let response = egui::Frame::NONE
        .fill(translucent(t.color.ok, 18))
        .stroke(Stroke::new(1.0, t.color.ok))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(format!("Latest receipt \u{00b7} {}", receipt.source_name()))
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.ok),
            );
            ui.label(
                egui::RichText::new(format!(
                    "Import completed and the prior shortcut profile was retained \u{00b7} {}",
                    receipt.id()
                ))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label("Latest shortcut import receipt");
        });
}

/// Returns true when the source well was activated.
fn render_import_body(
    ui: &mut Ui,
    state: &mut ShortcutImportDialogState,
    base: &ShortcutProfileLibrary,
) -> bool {
    let controls_enabled = state.pending == ImportPending::None;
    let source_requested = workflow_row(
        ui,
        "Source",
        "RSpice shortcut profile or supported editor preset.",
        |ui| {
            right_aligned(ui, |ui| {
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
        "Merge policy",
        "Protected accessibility and operating-system bindings remain explicit.",
        |ui| {
            right_aligned(ui, |ui| {
                let labels = MERGE_POLICIES
                    .iter()
                    .map(|policy| policy.label().to_owned())
                    .collect::<Vec<_>>();
                let selected = ui
                    .add_enabled_ui(controls_enabled, |ui| {
                        select(
                            ui,
                            "preferences.shortcuts.import.merge-policy",
                            "Merge policy",
                            state.options.merge_policy.label(),
                            &labels,
                            ui.available_width().min(360.0),
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

    if state.options.merge_policy == ShortcutMergePolicy::ImportNamedPreset {
        workflow_row(
            ui,
            "Preset name",
            "Create a named profile without changing the active shortcut map.",
            |ui| {
                right_aligned(ui, |ui| {
                    let width = ui.available_width().min(360.0);
                    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
                    let response = if controls_enabled {
                        ui.add_enabled_ui(true, |ui| {
                            ui.add_sized(
                                [width, height],
                                egui::TextEdit::singleline(
                                    state.options.preset_name.get_or_insert_with(String::new),
                                )
                                .hint_text("Preset name"),
                            )
                        })
                        .inner
                    } else {
                        // A disabled widget closure still executes. Render a
                        // detached display value so a malformed or restored
                        // pending state can never materialize `Some("")`.
                        let mut display = state.options.preset_name.clone().unwrap_or_default();
                        ui.add_enabled_ui(false, |ui| {
                            ui.add_sized(
                                [width, height],
                                egui::TextEdit::singleline(&mut display).hint_text("Preset name"),
                            )
                        })
                        .inner
                    };
                    if response.changed() {
                        options_changed = true;
                    }
                });
            },
        );
        let preset_exists = state
            .options
            .preset_name
            .as_deref()
            .is_some_and(|name| base.named_preset(name.trim()).is_some());
        if preset_exists {
            workflow_row(
                ui,
                "Existing preset",
                "Replacement remains explicit and is recorded by the import receipt.",
                |ui| {
                    let response = ui
                        .add_enabled_ui(controls_enabled, |ui| {
                            ui.checkbox(
                                &mut state.options.overwrite_existing_preset,
                                "Replace existing named preset",
                            )
                        })
                        .inner;
                    options_changed |= response.changed();
                },
            );
        } else if controls_enabled && state.options.overwrite_existing_preset {
            state.options.overwrite_existing_preset = false;
            options_changed = true;
        }
    } else if controls_enabled && state.options.preset_name.take().is_some() {
        state.options.overwrite_existing_preset = false;
        options_changed = true;
    }

    render_import_summary(ui, state.plan.as_ref(), state.options.conflict_policy);

    workflow_row(ui, "Conflict handling", "", |ui| {
        right_aligned(ui, |ui| {
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
                        ui.available_width().min(360.0),
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
                        let label = Command::from_stable_id(command_id)
                            .map_or(command_id.as_str(), |command| command.spec().label);
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
        state.refresh_plan(base);
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
                                        TableCellTone::Ok
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableCellTone {
    Default,
    Ok,
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
                TableCellTone::Ok => t.color.ok,
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
) -> ShortcutPortabilityAction {
    let mut action = ShortcutPortabilityAction::None;
    let cancel_label = if state.discard_confirmation {
        "Discard changes"
    } else {
        "Cancel"
    };
    let transaction_error = state.error.clone();
    let mut dialog = Dialog::new(EXPORT_EYEBROW, EXPORT_TITLE, EXPORT_PRIMARY)
        .description(
            "Export a portable shortcut profile with explicit scope, format, platform mappings, and privacy boundaries.",
        )
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
            "Publishing shortcut profile",
            "Waiting for the platform export destination to complete or cancel.",
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
            let prepared = build_shortcut_reference_model(profile, &request)
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
        "Versioned schema with stable command identifiers.",
        |ui| {
            right_aligned(ui, |ui| {
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
                            ui.available_width().min(360.0),
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
        "Defaults can be omitted for a smaller personal override.",
        |ui| {
            right_aligned(ui, |ui| {
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
                            ui.available_width().min(360.0),
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
        "Preserve explicit desktop and browser alternatives.",
        |ui| {
            right_aligned(ui, |ui| {
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
        "Projects, paths, recent commands, macros, credentials, and automation source are excluded.",
        |ui| {
            right_aligned(ui, |ui| {
                let t = Tokens::get(ui.ctx());
                ui.label(
                    egui::RichText::new("portable profile only")
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.ok),
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
    let width = ui.available_width().min(360.0);
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
        node.set_description("Choose an RSpice shortcut profile or supported editor preset");
    });
    if enabled {
        theme::paint_focus_ring(ui, &response, field.rect);
    }
    enabled && response.clicked()
}

fn validation_message(ui: &mut Ui, message: &str, error: bool) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::NONE
        .fill(translucent(
            if error { t.color.err } else { t.color.warn },
            20,
        ))
        .stroke(Stroke::new(
            1.0,
            if error { t.color.err } else { t.color.warn },
        ))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(if error { t.color.err } else { t.color.warn }),
                )
                .wrap(),
            );
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::shortcut_artifacts::{
        apply_shortcut_import, decode_shortcut_artifact_json, detect_shortcut_artifact,
        serialize_shortcut_reference_json,
    };
    use crate::workbench::shortcuts::{
        ProtectedShortcutPolicy, ShortcutBindingSlot, ShortcutSequence, ShortcutStroke,
    };

    fn exported_artifact(profile: &ShortcutPreferences) -> DecodedShortcutArtifact {
        let model = build_shortcut_reference_model(
            profile,
            &ShortcutExportRequest {
                scope: ShortcutExportScope::UserOverrides,
                include_platform_mappings: true,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: egui::os::OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap();
        decode_shortcut_artifact_json(
            "rspice-shortcuts.json",
            &serialize_shortcut_reference_json(&model).unwrap(),
        )
        .unwrap()
    }

    fn detected(profile: &ShortcutPreferences) -> DetectedShortcutArtifact {
        let model = build_shortcut_reference_model(
            profile,
            &ShortcutExportRequest::user_overrides(
                CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            ),
        )
        .unwrap();
        let json = serialize_shortcut_reference_json(&model).unwrap();
        detect_shortcut_artifact(
            "rspice-shortcuts.json",
            json.as_bytes(),
            crate::common::shortcut_artifacts::VscodeHostPlatform::Windows,
        )
        .unwrap()
    }

    fn open_with_artifact(
        state: &mut ShortcutImportDialogState,
        base: &ShortcutProfileLibrary,
        artifact: DecodedShortcutArtifact,
    ) {
        state.open();
        state.artifact = Some(artifact);
        state.plan_dirty = true;
        state.refresh_plan(base);
    }

    #[test]
    fn exact_mockup_contract_strings_and_option_order_are_stable() {
        assert_eq!(IMPORT_TITLE, "Import keyboard shortcut map");
        assert_eq!(
            IMPORT_EYEBROW,
            "PREFERENCES \u{00b7} VERSIONED COMMAND BINDINGS"
        );
        assert_eq!(IMPORT_PRIMARY, "Import compatible bindings");
        assert_eq!(EXPORT_TITLE, "Export keyboard shortcut map");
        assert_eq!(
            EXPORT_EYEBROW,
            "PREFERENCES \u{00b7} PORTABLE COMMAND BINDINGS"
        );
        assert_eq!(EXPORT_PRIMARY, "Export shortcut profile");
        assert_eq!(
            MERGE_POLICIES.map(ShortcutMergePolicy::label),
            [
                "Merge non-conflicting bindings",
                "Replace current user bindings",
                "Import into a named preset",
            ]
        );
        assert_eq!(
            CONFLICT_POLICIES.map(ShortcutConflictPolicy::label),
            [
                "Keep current and report",
                "Use imported binding",
                "Leave both unbound",
            ]
        );
        assert_eq!(
            IMPORT_CLASSES.map(ImportBindingClass::label),
            ["Global", "Schematic", "Results", "Simulation",]
        );
        assert_eq!(WORKFLOW_ROW_MIN_HEIGHT, 54.0);
        assert_eq!(WORKFLOW_ROW_LABEL_FR, 0.38);
        assert_eq!(WORKFLOW_ROW_VALUE_FR, 1.0);
        assert_eq!(WORKFLOW_ROW_COLUMN_GAP, 12.0);
        assert_eq!(WORKFLOW_ROW_HORIZONTAL_PADDING, 12);
        assert_eq!(WORKFLOW_ROW_VERTICAL_PADDING, 10);
        assert_eq!(WORKFLOW_ROW_STACK_BREAKPOINT, 560.0);
        assert_eq!(WORKFLOW_TABLE_COLLAPSE_WIDTH, 660.0);
        assert_eq!(
            IMPORT_SOURCE_PLACEHOLDER,
            "~/Downloads/rspice-shortcuts.json"
        );
    }

    #[test]
    fn changed_workflow_requires_a_second_explicit_discard() {
        let base = ShortcutProfileLibrary::default();
        let mut import = ShortcutImportDialogState::default();
        open_with_artifact(
            &mut import,
            &base,
            exported_artifact(&ShortcutPreferences::default()),
        );
        import.request_idle_cancel();
        assert!(import.is_open());
        assert!(import.discard_confirmation);
        import.request_idle_cancel();
        assert!(!import.is_open());

        let mut export = ShortcutExportDialogState::default();
        export.open();
        export.scope = ShortcutExportScope::CompleteResolved;
        export.request_idle_cancel();
        assert!(export.is_open());
        assert!(export.discard_confirmation);
        export.request_idle_cancel();
        assert!(!export.is_open());
    }

    #[test]
    fn unchanged_workflow_closes_on_the_first_cancel() {
        let mut import = ShortcutImportDialogState::default();
        import.open();
        import.request_idle_cancel();
        assert!(!import.is_open());

        let mut export = ShortcutExportDialogState::default();
        export.open();
        export.request_idle_cancel();
        assert!(!export.is_open());
    }

    #[test]
    fn source_review_is_isolated_until_the_typed_commit_effect_completes() {
        let base = ShortcutProfileLibrary::default();
        let mut imported = ShortcutPreferences::default();
        imported
            .set_binding(
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    egui::Key::F6,
                    false,
                    false,
                    false,
                ))),
            )
            .unwrap();
        let artifact = exported_artifact(&imported);
        let mut state = ShortcutImportDialogState::default();
        open_with_artifact(&mut state, &base, artifact);

        assert_eq!(base.active(), &ShortcutPreferences::default());
        assert!(
            state
                .plan
                .as_ref()
                .is_some_and(ShortcutImportPlan::can_apply)
        );

        state.pending = ImportPending::Commit;
        let plan = state.plan.clone().unwrap();
        let mut published = base.clone();
        let receipt = apply_shortcut_import(&mut published, &plan, |_| Ok(())).unwrap();
        state.complete_commit(Ok(receipt));

        assert!(!state.is_open());
        assert!(state.last_receipt.is_some());
        assert_ne!(published.active(), base.active());
    }

    #[test]
    fn stale_base_replans_before_commit() {
        let mut base = ShortcutProfileLibrary::default();
        let artifact = exported_artifact(&ShortcutPreferences::default());
        let mut state = ShortcutImportDialogState::default();
        open_with_artifact(&mut state, &base, artifact);
        let original_revision = state.plan.as_ref().unwrap().base_revision();
        base.edit_active(|profile| {
            profile.set_binding(
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                None,
            )
        })
        .unwrap()
        .unwrap();
        state.synchronize_plan(&base);
        assert_ne!(
            state.plan.as_ref().unwrap().base_revision(),
            original_revision
        );
        assert_eq!(
            state.plan.as_ref().unwrap().base_revision(),
            base.revision()
        );
    }

    #[test]
    fn protected_binding_requires_fresh_explicit_confirmation() {
        let mut imported = ShortcutPreferences::default();
        imported
            .policies_mut()
            .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
        imported
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Alternate,
                vec![
                    CommandPlatform::Browser,
                    CommandPlatform::Tablet,
                    CommandPlatform::Phone,
                ],
                None,
            )
            .unwrap();
        imported.acknowledge_protected_override(Command::Save);
        let artifact = exported_artifact(&imported);
        let base = ShortcutProfileLibrary::default();
        let mut state = ShortcutImportDialogState::default();
        open_with_artifact(&mut state, &base, artifact);
        state.options.conflict_policy = ShortcutConflictPolicy::UseImportedBinding;
        state.plan_dirty = true;
        state.synchronize_plan(&base);
        let command_id = Command::Save.stable_id().to_owned();
        assert!(
            state
                .plan
                .as_ref()
                .unwrap()
                .required_protected_confirmations()
                .contains(&command_id)
        );
        assert!(!state.plan.as_ref().unwrap().can_apply());

        state.options.protected_confirmations.insert(command_id);
        state.plan_dirty = true;
        state.synchronize_plan(&base);
        assert!(state.plan.as_ref().unwrap().can_apply());
    }

    #[test]
    fn replacing_source_clears_digest_specific_protected_consent() {
        let base = ShortcutProfileLibrary::default();
        let mut state = ShortcutImportDialogState::default();
        state.open();
        state
            .options
            .protected_confirmations
            .insert(Command::Save.stable_id().to_owned());

        state.accept_detected(&base, detected(&ShortcutPreferences::default()));

        assert!(state.options.protected_confirmations.is_empty());
    }

    #[test]
    fn failed_commit_cancellation_keeps_ownership_until_receipt_arrives() {
        let base = ShortcutProfileLibrary::default();
        let artifact = exported_artifact(&ShortcutPreferences::default());
        let plan =
            plan_shortcut_import(&base, &artifact, &ShortcutImportOptions::default()).unwrap();
        let mut published = base.clone();
        let receipt = apply_shortcut_import(&mut published, &plan, |_| Ok(())).unwrap();
        let receipt_id = receipt.id();
        let mut state = ShortcutImportDialogState {
            open: true,
            pending: ImportPending::Commit,
            cancel_requested: true,
            ..ShortcutImportDialogState::default()
        };

        state.commit_cancelled(Err("commit boundary reached".to_owned()));
        assert_eq!(state.pending, ImportPending::Commit);
        assert!(state.cancel_requested);
        assert!(state.is_open());
        state.complete_commit(Ok(receipt));
        assert!(!state.cancel_requested);
        assert!(!state.is_open());
        assert_eq!(state.last_receipt.as_ref().unwrap().id(), receipt_id);
    }

    #[test]
    fn named_preset_validation_never_falls_through_to_an_active_import() {
        let base = ShortcutProfileLibrary::default();
        let artifact = exported_artifact(&ShortcutPreferences::default());
        let mut state = ShortcutImportDialogState::default();
        open_with_artifact(&mut state, &base, artifact);
        state.options.merge_policy = ShortcutMergePolicy::ImportNamedPreset;
        state.options.preset_name = Some("   ".to_owned());
        state.plan_dirty = true;
        state.synchronize_plan(&base);
        assert!(state.plan.is_none());
        assert!(state.error.is_some());
    }

    #[test]
    fn rollback_effect_retains_the_exact_receipt_until_success() {
        let base = ShortcutProfileLibrary::default();
        let artifact = exported_artifact(&ShortcutPreferences::default());
        let plan =
            plan_shortcut_import(&base, &artifact, &ShortcutImportOptions::default()).unwrap();
        let mut published = base.clone();
        let receipt = apply_shortcut_import(&mut published, &plan, |_| Ok(())).unwrap();
        let receipt_id = receipt.id();
        let mut state = ShortcutImportDialogState {
            last_receipt: Some(receipt),
            ..ShortcutImportDialogState::default()
        };

        let action = state.begin_rollback().unwrap();
        assert!(matches!(
            action,
            ShortcutPortabilityAction::RollbackImport(receipt) if receipt.id() == receipt_id
        ));
        state.complete_rollback(Err("durable store unavailable".to_owned()));
        assert!(state.last_receipt.is_some());
        state.begin_rollback().unwrap();
        state.complete_rollback(Ok(()));
        assert!(state.last_receipt.is_none());
    }

    #[test]
    fn export_effect_contains_fully_prepared_private_artifact() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = ShortcutPortabilityDialogsState::default();
        state.open_export();
        let library = ShortcutProfileLibrary::default();
        let mut action = ShortcutPortabilityAction::None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let _ = state.render(
                    ctx,
                    &library,
                    ShortcutExportEnvironment {
                        runtime_platform: CommandPlatform::Desktop,
                        operating_system: egui::os::OperatingSystem::Windows,
                        current_contexts: &[ShortcutContext::EngineeringCanvas],
                    },
                );
            },
        );
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                events: vec![egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: Some(egui::Key::Enter),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            },
            |ctx| {
                action = state.render(
                    ctx,
                    &library,
                    ShortcutExportEnvironment {
                        runtime_platform: CommandPlatform::Desktop,
                        operating_system: egui::os::OperatingSystem::Windows,
                        current_contexts: &[ShortcutContext::EngineeringCanvas],
                    },
                );
            },
        );
        let ShortcutPortabilityAction::PublishExport(artifact) = action else {
            panic!("expected prepared export effect");
        };
        assert_eq!(artifact.format(), ShortcutArtifactFormat::Json);
        let text = std::str::from_utf8(artifact.bytes()).unwrap();
        assert!(!text.contains("source-path"));
        assert!(!text.contains("protected-override-acknowledgements"));
    }

    #[test]
    fn modal_accessibility_tree_exposes_exact_titles_and_summary_table() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let library = ShortcutProfileLibrary::default();
        let artifact = exported_artifact(&ShortcutPreferences::default());
        let mut state = ShortcutPortabilityDialogsState::default();
        state.open_import();
        open_with_artifact(&mut state.import, &library, artifact);
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let _ = state.render(
                    ctx,
                    &library,
                    ShortcutExportEnvironment {
                        runtime_platform: CommandPlatform::Desktop,
                        operating_system: egui::os::OperatingSystem::Windows,
                        current_contexts: &[],
                    },
                );
            },
        );
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some(IMPORT_TITLE)
                && node.is_modal()
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Table
                && node.label() == Some("Shortcut import compatibility summary")
        }));
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
                .count(),
            5
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
                .count(),
            4
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Cell)
                .count(),
            16
        );
        for label in ["Source", "Merge policy", "Conflict handling"] {
            assert!(nodes.iter().any(|(_, node)| node.label() == Some(label)));
        }
    }

    #[test]
    fn pending_import_controls_are_accessibly_disabled_and_source_keeps_picker_semantics() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let library = ShortcutProfileLibrary::default();
        let mut state = ShortcutPortabilityDialogsState::default();
        state.open_import();
        state.import.pending = ImportPending::Commit;
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let _ = state.render(
                    ctx,
                    &library,
                    ShortcutExportEnvironment {
                        runtime_platform: CommandPlatform::Desktop,
                        operating_system: egui::os::OperatingSystem::Windows,
                        current_contexts: &[],
                    },
                );
            },
        );
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        for label in ["Source", "Merge policy", "Conflict handling"] {
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| { node.label() == Some(label) && node.is_disabled() })
            );
        }
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Source")
                && node.value() == Some(IMPORT_SOURCE_PLACEHOLDER)
        }));
    }

    #[test]
    fn pending_render_is_structurally_immutable_even_for_incomplete_restored_state() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let library = ShortcutProfileLibrary::default();
        let mut state = ShortcutImportDialogState {
            open: true,
            options: ShortcutImportOptions {
                merge_policy: ShortcutMergePolicy::ImportNamedPreset,
                preset_name: None,
                ..ShortcutImportOptions::default()
            },
            pending: ImportPending::Commit,
            ..ShortcutImportDialogState::default()
        };
        let before = state.options.clone();
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = render_import_body(ui, &mut state, &library);
                });
            },
        );
        assert_eq!(state.options, before);
        assert_eq!(state.pending, ImportPending::Commit);
    }

    #[test]
    fn pending_export_controls_are_accessibly_disabled() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let library = ShortcutProfileLibrary::default();
        let mut state = ShortcutPortabilityDialogsState::default();
        state.open_export();
        state.export.pending = true;
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let _ = state.render(
                    ctx,
                    &library,
                    ShortcutExportEnvironment {
                        runtime_platform: CommandPlatform::Desktop,
                        operating_system: egui::os::OperatingSystem::Windows,
                        current_contexts: &[],
                    },
                );
            },
        );
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        for label in ["Format", "Scope", "Include platform shortcut alternatives"] {
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some(label) && node.is_disabled())
            );
        }
    }

    #[test]
    fn helper_artifact_is_canonical() {
        // Prevent the helper from silently drifting to a foreign adaptation.
        assert!(matches!(
            detected(&ShortcutPreferences::default()),
            DetectedShortcutArtifact::RSpice(_)
        ));
    }

    #[test]
    fn route_loss_closes_idle_dialogs_and_waits_for_exact_async_cancellation() {
        let mut dialogs = ShortcutPortabilityDialogsState::default();
        dialogs.open_import();
        assert!(matches!(
            dialogs.request_route_close(),
            ShortcutPortabilityAction::None
        ));
        assert!(!dialogs.application_modal_open());

        dialogs.open_import();
        dialogs.import.pending = ImportPending::Source;
        assert!(matches!(
            dialogs.request_route_close(),
            ShortcutPortabilityAction::CancelImportSource
        ));
        assert!(dialogs.import.cancel_requested);
        assert!(dialogs.application_modal_open());
        assert!(matches!(
            dialogs.request_route_close(),
            ShortcutPortabilityAction::None
        ));
        dialogs.import_source_cancelled();
        assert!(!dialogs.application_modal_open());

        dialogs.open_import();
        dialogs.import.pending = ImportPending::Commit;
        assert!(matches!(
            dialogs.request_route_close(),
            ShortcutPortabilityAction::CancelImportCommit
        ));
        assert!(dialogs.application_modal_open());
        dialogs.import_commit_cancelled(Ok(()));
        assert!(!dialogs.application_modal_open());

        dialogs.open_export();
        assert!(matches!(
            dialogs.request_route_close(),
            ShortcutPortabilityAction::None
        ));
        assert!(!dialogs.application_modal_open());
    }

    #[test]
    fn opening_a_second_portability_workflow_never_drops_the_current_owner() {
        let mut dialogs = ShortcutPortabilityDialogsState::default();
        dialogs.open_import();
        dialogs.import.pending = ImportPending::Source;
        dialogs.open_export();
        assert!(dialogs.import.is_open());
        assert!(!dialogs.export.is_open());

        dialogs.import.pending = ImportPending::None;
        dialogs.import.close();
        dialogs.open_export();
        dialogs.open_import();
        assert!(dialogs.export.is_open());
        assert!(!dialogs.import.is_open());
    }
}
