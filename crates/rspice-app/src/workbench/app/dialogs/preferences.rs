//! Preferences — canonical mockup shell with durable, validated settings.

pub(crate) mod preference_pages;
pub(crate) mod preferences_shell;
pub(crate) mod shortcut_portability_dialogs;
pub(crate) mod shortcut_preferences;
pub(crate) mod workspace_layout_manager;

use egui::{Context, Id};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::command_catalog::ShortcutContext;
use crate::workbench::shortcuts::artifacts::{
    ShortcutArtifactExportOutcome, ShortcutArtifactImportOutcome, ShortcutImportReceipt,
    apply_shortcut_import, export_shortcut_artifact,
};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use self::preferences_shell::PreferenceCategory;
use self::shortcut_portability_dialogs::{ShortcutExportEnvironment, ShortcutPortabilityAction};
use crate::workbench::app_state::session::preferences_file::{
    MAX_SETTINGS_FILE_BYTES, PreferencesFileParts, PreferencesFileRuntime, SETTINGS_FILE,
    stage_settings_import,
};
use crate::workbench::app_state::session::shortcut_library::{
    ShortcutLibraryParts, ShortcutLibraryPersistenceRuntime, ShortcutLibraryPublication,
    ShortcutLibraryPublicationContinuation,
};
use crate::workbench::app_state::session::shortcuts::ShortcutContextMatch;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::feature::CloudPort;
use crate::workbench::state::WorkbenchSession;
use rspice_app_services::workbench::slots::platform::trim_recent_files;

const CATEGORY_STATE_ID: &str = "rspice.preferences.active-category";
const SETTINGS_IMPORT_ID: &str = "rspice.preferences.settings-file.import";
const SETTINGS_EXPORT_ID: &str = "rspice.preferences.settings-file.export";
const PENDING_RESTORE_ID: &str = "rspice.preferences.pending-restore";

/// The page whose Restore defaults is waiting on the reader's answer.
pub(crate) fn pending_restore(ctx: &Context) -> Option<PreferenceCategory> {
    ctx.data(|data| data.get_temp::<PreferenceCategory>(Id::new(PENDING_RESTORE_ID)))
}

pub(crate) fn set_pending_restore(ctx: &Context, category: Option<PreferenceCategory>) {
    let id = Id::new(PENDING_RESTORE_ID);
    ctx.data_mut(|data| match category {
        Some(category) => {
            data.insert_temp(id, category);
        }
        None => {
            data.remove_temp::<PreferenceCategory>(id);
        }
    });
}

#[derive(Debug, Default)]
pub(crate) struct PreferencePageActions {
    pub(crate) open_shortcut_editor: bool,
    pub(crate) open_shortcut_import: bool,
    pub(crate) open_shortcut_export: bool,
    pub(crate) import_settings_file: bool,
    pub(crate) export_settings_file: bool,
    pub(crate) show_settings_file: bool,
    pub(crate) shortcut_policy_candidate: Option<crate::state::shortcuts::ShortcutPreferences>,
}

/// What Preferences and the windows it opens read and write: the settings
/// feature's dialogs and its stores, the model whose preferences they edit,
/// the session, the platform services' state, whose autosave interval and
/// recent files a page keeps, and what the cloud shows.
pub(crate) struct PreferencesDialogs<'a> {
    pub(crate) preferences_open: &'a mut bool,
    pub(crate) workspace_layout_manager:
        &'a mut workspace_layout_manager::WorkspaceLayoutManagerState,
    pub(crate) shortcut_portability:
        &'a mut shortcut_portability_dialogs::ShortcutPortabilityDialogsState,
    pub(crate) shortcut_editor: &'a mut shortcut_preferences::ShortcutEditorState,
    pub(crate) shortcut_policy_candidate:
        &'a mut Option<crate::state::shortcuts::ShortcutPreferences>,
    pub(crate) browser_spoken_feedback: &'a mut bool,
    pub(crate) shortcut_library_persistence: &'a mut ShortcutLibraryPersistenceRuntime,
    pub(crate) shortcut_library_publication_continuation:
        &'a mut Option<ShortcutLibraryPublicationContinuation>,
    pub(crate) preferences_file: &'a mut PreferencesFileRuntime,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) platform: &'a mut PlatformState,
    pub(crate) cloud: &'a dyn CloudPort,
}

impl PreferencesDialogs<'_> {
    /// Answer the gesture just made, on the status bar, as
    /// [`AppState::prompt`](crate::workbench::app_state::AppState::prompt) does.
    fn prompt(&mut self, text: impl Into<String>) {
        self.workbench.prompt(text);
    }

    /// Record an event in the Console, as
    /// [`AppState::record`](crate::workbench::app_state::AppState::record) does.
    fn record(&mut self, severity: LogSeverity, source: LogSource, text: impl Into<String>) -> u64 {
        self.model.record(severity, source, text)
    }

    /// What the shortcut library's persistence reads and writes.
    fn shortcut_library_parts(&mut self) -> ShortcutLibraryParts<'_> {
        ShortcutLibraryParts {
            persistence: &mut *self.shortcut_library_persistence,
            model: &mut *self.model,
            outbox: &mut self.workbench.outbox,
        }
    }

    /// What the preferences file reads and writes.
    fn preferences_file_parts(&mut self) -> PreferencesFileParts<'_> {
        PreferencesFileParts {
            file: &mut *self.preferences_file,
            browser_spoken_feedback: &mut *self.browser_spoken_feedback,
            model: &mut *self.model,
            workbench: &mut *self.workbench,
            platform: &mut *self.platform,
        }
    }

    /// Drop the recent projects beyond what the page now lists, as
    /// [`trim_recent_files`] does for the application state.
    fn trim_recent_files(&mut self) {
        trim_recent_files(self.model, self.platform);
    }
}

impl PreferencesDialogs<'_> {
    /// Draw Preferences and the windows it opens for this frame.
    pub(crate) fn render(&mut self, ctx: &Context) {
        self.poll_shortcut_portability_source();
        self.poll_settings_file_transfer(ctx);
        self.finish_shortcut_library_publication();
        let category_id = Id::new(CATEGORY_STATE_ID);
        let current_surface = self.workbench.current_route().surface_id();
        let route_owned = current_surface == SurfaceId::Preferences;
        let standalone_layout_manager =
            self.workspace_layout_manager.open && !*self.preferences_open;
        if standalone_layout_manager {
            workspace_layout_manager::render(ctx, self);
            return;
        }
        if !route_owned {
            let portability_action = self.shortcut_portability.request_route_close();
            self.execute_shortcut_portability_action(ctx, portability_action);
            *self.preferences_open = false;
            #[cfg(target_arch = "wasm32")]
            let editor_publication_pending = self.shortcut_editor.persistence_pending;
            #[cfg(not(target_arch = "wasm32"))]
            // Native persistence is synchronous; a retained pending flag
            // cannot own an actual write across frames.
            let close_editor = true;
            #[cfg(target_arch = "wasm32")]
            let mut close_editor = !editor_publication_pending;
            #[cfg(target_arch = "wasm32")]
            if editor_publication_pending
                && matches!(
                    self.shortcut_library_publication_continuation,
                    Some(ShortcutLibraryPublicationContinuation::Editor)
                )
                && self
                    .shortcut_library_parts()
                    .cancel_pending_shortcut_library_publication()
            {
                *self.shortcut_library_publication_continuation = None;
                close_editor = true;
            }
            if close_editor {
                self.shortcut_editor.close_and_discard();
            }
            self.workspace_layout_manager.close();
            ctx.data_mut(|data| data.remove_temp::<PreferenceCategory>(category_id));
            set_pending_restore(ctx, None);
            preferences_shell::unmount(ctx);
            return;
        }

        let mut category = ctx
            .data_mut(|data| data.get_temp::<PreferenceCategory>(category_id))
            .unwrap_or_default();
        let mut actions = PreferencePageActions::default();
        // Preferences does not consume Escape while one of its own child
        // executors is open above it.
        let nested_modal_open = self.cloud.license_dialog_open()
            || self.shortcut_editor.open
            || self.shortcut_portability.application_modal_open()
            || self.workspace_layout_manager.open
            || pending_restore(ctx).is_some();
        let touch_target = self.model.preferences.minimum_touch_target();
        let shell_response = {
            let state = &mut *self;
            preferences_shell::show(
                ctx,
                &mut category,
                !nested_modal_open,
                touch_target,
                |ui, category| {
                    preference_pages::render(ui, category, state, &mut actions);
                },
            )
        };
        if shell_response.close_requested {
            ctx.data_mut(|data| data.remove_temp::<PreferenceCategory>(category_id));
        } else {
            ctx.data_mut(|data| data.insert_temp(category_id, category));
        }

        if shell_response.restore_defaults_requested {
            set_pending_restore(ctx, Some(category));
        }
        if shell_response.close_requested {
            *self.preferences_open = false;
            set_pending_restore(ctx, None);
            if route_owned {
                close_preferences(self);
            }
        }
        self.render_restore_defaults_confirmation(ctx, &mut actions);
        self.execute_preference_page_actions(ctx, actions);
        shortcut_preferences::render_editor(ctx, self);
        workspace_layout_manager::render(ctx, self);
        self.render_shortcut_portability_dialogs(ctx);
    }

    pub(crate) fn execute_preference_page_actions(
        &mut self,
        ctx: &Context,
        actions: PreferencePageActions,
    ) {
        if actions.open_shortcut_editor {
            let profile = self.model.preferences.shortcuts().clone();
            self.shortcut_editor.open(&profile);
        }
        if actions.open_shortcut_import {
            self.shortcut_portability.open_import();
        }
        if actions.open_shortcut_export {
            self.shortcut_portability.open_export();
        }
        if actions.import_settings_file {
            self.begin_settings_import(ctx);
        }
        if actions.export_settings_file {
            self.begin_settings_export(ctx);
        }
        if actions.show_settings_file {
            self.preferences_file_parts().show_preferences_file();
        }
        if let Some(profile) = actions.shortcut_policy_candidate {
            self.begin_shortcut_policy_publication(ctx, profile);
        }
    }

    /// Ask before the footer's Restore defaults discards a page's settings.
    ///
    /// The question names the page because the reset covers only that page:
    /// a reader who reads "restore defaults" as the whole dialog is told
    /// otherwise before anything changes.
    fn render_restore_defaults_confirmation(
        &mut self,
        ctx: &Context,
        actions: &mut PreferencePageActions,
    ) {
        use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

        let Some(category) = pending_restore(ctx) else {
            return;
        };
        let title = format!("Restore {} settings to their defaults?", category.label());
        let consequence = "Settings on the other pages are kept.";
        let choice = Dialog::new(&title, "Restore")
            .description(consequence)
            .size(DialogSize::Confirmation)
            .ghost("Cancel")
            .show(ctx, |ui| {
                let t = crate::ui::tokens::Tokens::get(ui.ctx());
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(consequence)
                            .font(crate::ui::theme::sans(
                                crate::ui::tokens::FS_2,
                                crate::ui::theme::FontWeight::Regular,
                            ))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            });
        match choice {
            DialogChoice::Primary => {
                set_pending_restore(ctx, None);
                preference_pages::restore_defaults(category, self, actions);
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => set_pending_restore(ctx, None),
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn begin_shortcut_policy_publication(
        &mut self,
        ctx: &Context,
        profile: crate::state::shortcuts::ShortcutPreferences,
    ) {
        let mut candidate = self.model.preferences.shortcut_profiles().clone();
        if let Err(error) = candidate.replace_active(profile.clone()) {
            self.prompt(format!("Shortcut settings were not saved: {error}"));
            return;
        }
        *self.shortcut_policy_candidate = Some(profile);
        self.publish_shortcut_candidate(
            ctx,
            &candidate,
            ShortcutLibraryPublicationContinuation::Policy,
        );
    }

    pub fn render_shortcut_portability_dialogs(&mut self, ctx: &Context) {
        let library = self.model.preferences.shortcut_profiles().clone();
        let current_contexts = ShortcutContext::ALL
            .into_iter()
            .filter(|context| context.matches(self.workbench, self.model))
            .collect::<Vec<_>>();
        let environment = ShortcutExportEnvironment {
            runtime_platform: crate::workbench::app_state::runtime_command_platform(ctx),
            operating_system: ctx.os(),
            current_contexts: &current_contexts,
        };
        let action = self.shortcut_portability.render(ctx, &library, environment);
        self.execute_shortcut_portability_action(ctx, action);
    }

    fn execute_shortcut_portability_action(
        &mut self,
        ctx: &Context,
        action: ShortcutPortabilityAction,
    ) {
        match action {
            ShortcutPortabilityAction::None => {}
            ShortcutPortabilityAction::SelectImportSource => {
                #[cfg(not(target_arch = "wasm32"))]
                match crate::workbench::shortcuts::artifacts::import_shortcut_artifact_source() {
                    Ok(ShortcutArtifactImportOutcome::Ready(source)) => {
                        let library = self.model.preferences.shortcut_profiles().clone();
                        self.shortcut_portability
                            .accept_import_source(&library, *source);
                    }
                    Ok(ShortcutArtifactImportOutcome::Cancelled) => {
                        self.shortcut_portability.import_source_cancelled()
                    }
                    Err(error) => self
                        .shortcut_portability
                        .import_source_failed(error.to_string()),
                }
                #[cfg(target_arch = "wasm32")]
                if let Err(error) =
                    crate::workbench::shortcuts::artifacts::start_browser_shortcut_artifact_import()
                {
                    self.shortcut_portability
                        .import_source_failed(error.to_string());
                }
            }
            ShortcutPortabilityAction::CancelImportSource => {
                #[cfg(not(target_arch = "wasm32"))]
                self.shortcut_portability.import_source_cancelled();
                #[cfg(target_arch = "wasm32")]
                {
                    let outcome =
                        crate::workbench::shortcuts::artifacts::cancel_browser_shortcut_artifact_import(
                        );
                    match outcome {
                        crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::Cancelled
                        | crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased => {
                            self
                                .shortcut_portability
                                .import_source_cancelled();
                        }
                        crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::StillOwned => {}
                    }
                }
            }
            ShortcutPortabilityAction::CommitImport(plan) => {
                match stage_shortcut_import(self.model.preferences.shortcut_profiles(), &plan) {
                    Ok((candidate, receipt)) => self.publish_shortcut_candidate(
                        ctx,
                        &candidate,
                        ShortcutLibraryPublicationContinuation::Import(Box::new(receipt)),
                    ),
                    Err(error) => self.shortcut_portability.complete_import(Err(error)),
                }
            }
            ShortcutPortabilityAction::CancelImportCommit => {
                #[cfg(target_arch = "wasm32")]
                {
                    let cancelled = self
                        .shortcut_library_parts()
                        .cancel_pending_shortcut_library_publication();
                    if cancelled {
                        *self.shortcut_library_publication_continuation = None;
                        self.shortcut_portability.import_commit_cancelled(Ok(()));
                    } else {
                        self.shortcut_portability.import_commit_cancelled(Err(
                            "The import had already been saved and is in effect.".to_owned(),
                        ));
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                self.shortcut_portability.import_commit_cancelled(Err(
                    "The import was already saved and can no longer be cancelled.".to_owned(),
                ));
            }
            ShortcutPortabilityAction::PublishExport(artifact) => {
                let result = export_shortcut_artifact(&artifact).map_err(|error| error.to_string());
                if result.as_ref().is_ok_and(|outcome| {
                    !matches!(outcome, ShortcutArtifactExportOutcome::Cancelled)
                }) {
                    self.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Exported the keyboard shortcut profile.",
                    );
                }
                self.shortcut_portability.complete_export(result);
            }
        }
    }

    fn publish_shortcut_candidate(
        &mut self,
        ctx: &Context,
        candidate: &crate::state::shortcuts::ShortcutProfileLibrary,
        continuation: ShortcutLibraryPublicationContinuation,
    ) {
        if self.shortcut_library_publication_continuation.is_some() {
            self.complete_shortcut_library_publication(
                continuation,
                Err("another shortcut-library publication is awaiting acknowledgement".to_owned()),
            );
            return;
        }
        let published = self
            .shortcut_library_parts()
            .publish_shortcut_library_candidate(candidate, ctx);
        match published {
            Ok(ShortcutLibraryPublication::Published) => {
                self.complete_shortcut_library_publication(continuation, Ok(()));
            }
            Ok(ShortcutLibraryPublication::Pending) => {
                debug_assert!(self.shortcut_library_publication_continuation.is_none());
                *self.shortcut_library_publication_continuation = Some(continuation);
            }
            Err(error) => self.complete_shortcut_library_publication(continuation, Err(error)),
        }
    }

    fn poll_shortcut_portability_source(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(result) =
            crate::workbench::shortcuts::artifacts::poll_browser_shortcut_artifact_import()
        {
            match result {
                Ok(ShortcutArtifactImportOutcome::Ready(source)) => {
                    let library = self.model.preferences.shortcut_profiles().clone();
                    self.shortcut_portability
                        .accept_import_source(&library, *source);
                }
                Ok(ShortcutArtifactImportOutcome::Cancelled) => {
                    self.shortcut_portability.import_source_cancelled()
                }
                Err(error) => {
                    let message = error.to_string();
                    self.shortcut_portability
                        .import_source_failed(message.clone());
                    self.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("Shortcut profile could not be opened for import: {message}"),
                    );
                }
            }
        }
    }

    fn begin_settings_import(&mut self, ctx: &Context) {
        if let Err(refusal) = crate::io::file_exchange::open_file(
            ctx,
            Id::new(SETTINGS_IMPORT_ID),
            SETTINGS_FILE,
            MAX_SETTINGS_FILE_BYTES,
        ) {
            self.prompt(refusal);
        }
    }

    fn begin_settings_export(&mut self, ctx: &Context) {
        let started = self
            .preferences_file_parts()
            .settings_file_bytes()
            .and_then(|bytes| {
                crate::io::file_exchange::save_file(
                    ctx,
                    Id::new(SETTINGS_EXPORT_ID),
                    SETTINGS_FILE,
                    SETTINGS_FILE.fallback_name.to_owned(),
                    bytes,
                )
            });
        if let Err(refusal) = started {
            self.prompt(format!("Settings were not exported: {refusal}"));
        }
    }

    /// Collect a settings-file pick, whether or not Preferences is still
    /// open: a result left in its mailbox would reach the next pick instead.
    pub fn poll_settings_file_transfer(&mut self, ctx: &Context) {
        match crate::io::file_exchange::take_saved(ctx, Id::new(SETTINGS_EXPORT_ID)) {
            Some(Ok(Some(saved))) => {
                self.prompt(format!("Settings exported to {}.", saved.name));
            }
            Some(Err(refusal)) => self.prompt(refusal),
            Some(Ok(None)) | None => {}
        }
        match crate::io::file_exchange::take_opened(ctx, Id::new(SETTINGS_IMPORT_ID)) {
            Some(Ok(Some(file))) => {
                match stage_settings_import(self.model, &file.name, file.text.as_bytes()) {
                    Ok((shortcuts, staged)) => self.publish_shortcut_candidate(
                        ctx,
                        &shortcuts,
                        ShortcutLibraryPublicationContinuation::SettingsImport(Box::new(staged)),
                    ),
                    Err(reason) => self.prompt(format!(
                        "Settings were not imported from {}: {reason}. Nothing was changed.",
                        file.name
                    )),
                }
            }
            Some(Err(refusal)) => self.prompt(refusal),
            Some(Ok(None)) | None => {}
        }
    }

    fn finish_shortcut_library_publication(&mut self) {
        if self.shortcut_library_publication_continuation.is_none() {
            if self
                .shortcut_library_parts()
                .shortcut_library_publication_completion_pending()
            {
                let _ = self
                    .shortcut_library_parts()
                    .take_shortcut_library_publication_completion();
                let message = "Saving keyboard shortcuts finished after the dialog that started it had closed. The saved shortcuts are in effect; review them in Preferences.";
                self.shortcut_editor.persistence_pending = false;
                self.shortcut_editor.error_summary = Some(message.to_owned());
                *self.shortcut_policy_candidate = None;
                self.shortcut_portability
                    .complete_import(Err(message.to_owned()));
                self.record(LogSeverity::Error, LogSource::Studio, message);
            }
            return;
        }
        let Some(result) = self
            .shortcut_library_parts()
            .take_shortcut_library_publication_completion()
        else {
            return;
        };
        let continuation = self
            .shortcut_library_publication_continuation
            .take()
            .expect("completion owner was checked above");
        self.complete_shortcut_library_publication(continuation, result);
    }

    fn complete_shortcut_library_publication(
        &mut self,
        continuation: ShortcutLibraryPublicationContinuation,
        result: Result<(), String>,
    ) {
        match continuation {
            ShortcutLibraryPublicationContinuation::Editor => {
                self.shortcut_editor.persistence_pending = false;
                match result {
                    Ok(()) => {
                        self.shortcut_editor.close_and_discard();
                        self.prompt("Keyboard shortcuts saved.");
                    }
                    Err(error) => {
                        let message = format!("Shortcut profile was not saved: {error}");
                        self.shortcut_editor.error_summary = Some(message.clone());
                        self.record(LogSeverity::Error, LogSource::Studio, message);
                    }
                }
            }
            ShortcutLibraryPublicationContinuation::Policy => {
                *self.shortcut_policy_candidate = None;
                match result {
                    Ok(()) => self.prompt("Shortcut settings saved."),
                    Err(error) => {
                        self.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!("Shortcut settings were not saved: {error}"),
                        );
                    }
                }
            }
            ShortcutLibraryPublicationContinuation::Import(receipt) => {
                if result.is_ok() {
                    self.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!("Imported shortcut profile '{}'.", receipt.source_name()),
                    );
                } else if let Err(error) = &result {
                    self.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("Shortcut profile import was not saved: {error}"),
                    );
                }
                self.shortcut_portability
                    .complete_import(result.map(|()| *receipt));
            }
            ShortcutLibraryPublicationContinuation::SettingsImport(staged) => {
                self.preferences_file_parts()
                    .finish_settings_import(*staged, result);
            }
        }
    }
}

fn stage_shortcut_import(
    base: &crate::state::shortcuts::ShortcutProfileLibrary,
    plan: &crate::workbench::shortcuts::artifacts::ShortcutImportPlan,
) -> Result<
    (
        crate::state::shortcuts::ShortcutProfileLibrary,
        ShortcutImportReceipt,
    ),
    String,
> {
    let mut staged = base.clone();
    let receipt =
        apply_shortcut_import(&mut staged, plan, |_| Ok(())).map_err(|error| error.to_string())?;
    Ok((staged, receipt))
}

pub fn close_preferences(state: &mut PreferencesDialogs<'_>) {
    if state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(state.workbench.workspace));
    if let Err(error) = state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        state.prompt(format!("Could not close Preferences: {error}"));
    }
}

pub fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}
