//! Preferences — canonical mockup shell with durable, validated settings.

mod preference_pages;
mod preferences_shell;
#[cfg(test)]
mod raster;
pub(crate) mod shortcut_portability_dialogs;
mod shortcut_preferences;
pub(crate) mod workspace_layout_manager;

use egui::{Context, Id};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::commands::ShortcutContext;
use crate::workbench::shortcuts::artifacts::{
    ShortcutArtifactExportOutcome, ShortcutArtifactImportOutcome, ShortcutImportReceipt,
    apply_shortcut_import, export_shortcut_artifact, rollback_shortcut_import,
};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use self::preferences_shell::PreferenceCategory;
use self::shortcut_portability_dialogs::{ShortcutExportEnvironment, ShortcutPortabilityAction};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::preferences_file::{
    MAX_SETTINGS_FILE_BYTES, SETTINGS_FILE,
};
use crate::workbench::app_state::session::shortcut_library::{
    ShortcutLibraryPublication, ShortcutLibraryPublicationContinuation,
};

const CATEGORY_STATE_ID: &str = "rspice.preferences.active-category";
const SETTINGS_IMPORT_ID: &str = "rspice.preferences.settings-file.import";
const SETTINGS_EXPORT_ID: &str = "rspice.preferences.settings-file.export";

#[derive(Debug, Default)]
pub(in crate::workbench) struct PreferencePageActions {
    pub(in crate::workbench) open_workspace_layout_manager: bool,
    pub(in crate::workbench) open_shortcut_editor: bool,
    pub(in crate::workbench) open_shortcut_import: bool,
    pub(in crate::workbench) open_shortcut_export: bool,
    pub(in crate::workbench) import_settings_file: bool,
    pub(in crate::workbench) export_settings_file: bool,
    pub(in crate::workbench) show_settings_file: bool,
    pub(in crate::workbench) shortcut_policy_candidate:
        Option<crate::workbench::ShortcutPreferences>,
    pub(in crate::workbench) updated_profile_label: Option<&'static str>,
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_preferences_dialog(&mut self, ctx: &Context) {
        self.poll_shortcut_portability_source();
        self.poll_settings_file_transfer(ctx);
        self.finish_shortcut_library_publication();
        let category_id = Id::new(CATEGORY_STATE_ID);
        let current_surface = self.state.workbench.current_route().surface_id();
        let route_owned = current_surface == SurfaceId::Preferences;
        let standalone_layout_manager = self.state.dialogs.workspace_layout_manager.open
            && !self.state.dialogs.preferences_open;
        if standalone_layout_manager {
            workspace_layout_manager::render(ctx, &mut self.state);
            return;
        }
        if !route_owned {
            let portability_action = self
                .state
                .dialogs
                .shortcut_portability
                .request_route_close();
            self.execute_shortcut_portability_action(ctx, portability_action);
            self.state.dialogs.preferences_open = false;
            #[cfg(target_arch = "wasm32")]
            let editor_publication_pending = self.state.dialogs.shortcut_editor.persistence_pending;
            #[cfg(not(target_arch = "wasm32"))]
            // Native persistence is synchronous; a retained pending flag
            // cannot own an actual write across frames.
            let close_editor = true;
            #[cfg(target_arch = "wasm32")]
            let mut close_editor = !editor_publication_pending;
            #[cfg(target_arch = "wasm32")]
            if editor_publication_pending
                && matches!(
                    self.state.shortcut_library_publication_continuation,
                    Some(ShortcutLibraryPublicationContinuation::Editor)
                )
                && self.state.cancel_pending_shortcut_library_publication()
            {
                self.state.shortcut_library_publication_continuation = None;
                close_editor = true;
            }
            if close_editor {
                self.state.dialogs.shortcut_editor.close_and_discard();
            }
            self.state.dialogs.workspace_layout_manager.close();
            ctx.data_mut(|data| data.remove_temp::<PreferenceCategory>(category_id));
            preferences_shell::unmount(ctx);
            return;
        }

        let mut category = ctx
            .data_mut(|data| data.get_temp::<PreferenceCategory>(category_id))
            .unwrap_or_default();
        let mut actions = PreferencePageActions::default();
        // Preferences does not consume Escape while one of its own child
        // executors is open above it.
        let nested_modal_open = self.state.dialogs.license_dialog.open
            || self.state.dialogs.shortcut_editor.open
            || self
                .state
                .dialogs
                .shortcut_portability
                .application_modal_open()
            || self.state.dialogs.workspace_layout_manager.open;
        let shell_response = {
            let state = &mut self.state;
            preferences_shell::show(ctx, &mut category, !nested_modal_open, |ui, category| {
                preference_pages::render(ui, category, state, &mut actions);
            })
        };
        if shell_response.close_requested {
            ctx.data_mut(|data| data.remove_temp::<PreferenceCategory>(category_id));
        } else {
            ctx.data_mut(|data| data.insert_temp(category_id, category));
        }

        if let Some(label) = actions.updated_profile_label {
            self.state
                .prompt(engineering_profile_updated_message(label));
        }
        if shell_response.restore_defaults_requested {
            preference_pages::restore_defaults(category, &mut self.state);
        }
        if shell_response.close_requested {
            self.state.dialogs.preferences_open = false;
            if route_owned {
                close_preferences(&mut self.state);
            }
        }
        self.execute_preference_page_actions(ctx, actions);
        shortcut_preferences::render_editor(ctx, &mut self.state);
        workspace_layout_manager::render(ctx, &mut self.state);
        self.render_shortcut_portability_dialogs(ctx);
    }

    fn execute_preference_page_actions(&mut self, ctx: &Context, actions: PreferencePageActions) {
        if actions.open_shortcut_editor {
            let profile = self.state.ui.preferences.shortcuts().clone();
            self.state.dialogs.shortcut_editor.open(&profile);
        }
        if actions.open_shortcut_import {
            self.state.dialogs.shortcut_portability.open_import();
        }
        if actions.open_shortcut_export {
            self.state.dialogs.shortcut_portability.open_export();
        }
        if actions.import_settings_file {
            self.begin_settings_import(ctx);
        }
        if actions.export_settings_file {
            self.begin_settings_export(ctx);
        }
        if actions.show_settings_file {
            self.state.show_preferences_file();
        }
        if actions.open_workspace_layout_manager {
            let preset = self
                .state
                .ui
                .preferences
                .workspace()
                .map(crate::workbench::WorkspacePreferences::preset)
                .unwrap_or_default();
            self.state.dialogs.workspace_layout_manager.open(preset);
        }
        if let Some(profile) = actions.shortcut_policy_candidate {
            self.begin_shortcut_policy_publication(ctx, profile);
        }
    }

    fn begin_shortcut_policy_publication(
        &mut self,
        ctx: &Context,
        profile: crate::workbench::ShortcutPreferences,
    ) {
        let mut candidate = self.state.ui.preferences.shortcut_profiles().clone();
        if let Err(error) = candidate.replace_active(profile.clone()) {
            self.state
                .prompt(format!("Shortcut policy was not saved: {error}"));
            return;
        }
        self.state.dialogs.shortcut_policy_candidate = Some(profile);
        self.publish_shortcut_candidate(
            ctx,
            &candidate,
            ShortcutLibraryPublicationContinuation::Policy,
        );
    }

    fn render_shortcut_portability_dialogs(&mut self, ctx: &Context) {
        let library = self.state.ui.preferences.shortcut_profiles().clone();
        let current_contexts = ShortcutContext::ALL
            .into_iter()
            .filter(|context| context.matches(self))
            .collect::<Vec<_>>();
        let environment = ShortcutExportEnvironment {
            runtime_platform: crate::workbench::app_state::runtime_command_platform(ctx),
            operating_system: ctx.os(),
            current_contexts: &current_contexts,
        };
        let action = self
            .state
            .dialogs
            .shortcut_portability
            .render(ctx, &library, environment);
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
                        let library = self.state.ui.preferences.shortcut_profiles().clone();
                        self.state
                            .dialogs
                            .shortcut_portability
                            .accept_import_source(&library, *source);
                    }
                    Ok(ShortcutArtifactImportOutcome::Cancelled) => self
                        .state
                        .dialogs
                        .shortcut_portability
                        .import_source_cancelled(),
                    Err(error) => self
                        .state
                        .dialogs
                        .shortcut_portability
                        .import_source_failed(error.to_string()),
                }
                #[cfg(target_arch = "wasm32")]
                if let Err(error) =
                    crate::workbench::shortcuts::artifacts::start_browser_shortcut_artifact_import()
                {
                    self.state
                        .dialogs
                        .shortcut_portability
                        .import_source_failed(error.to_string());
                }
            }
            ShortcutPortabilityAction::CancelImportSource => {
                #[cfg(not(target_arch = "wasm32"))]
                self.state
                    .dialogs
                    .shortcut_portability
                    .import_source_cancelled();
                #[cfg(target_arch = "wasm32")]
                {
                    let outcome =
                        crate::workbench::shortcuts::artifacts::cancel_browser_shortcut_artifact_import(
                        );
                    match outcome {
                        crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::Cancelled
                        | crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased => {
                            self.state
                                .dialogs
                                .shortcut_portability
                                .import_source_cancelled();
                        }
                        crate::workbench::shortcuts::artifacts::BrowserShortcutArtifactImportCancelOutcome::StillOwned => {}
                    }
                }
            }
            ShortcutPortabilityAction::CommitImport(plan) => {
                match stage_shortcut_import(self.state.ui.preferences.shortcut_profiles(), &plan) {
                    Ok((candidate, receipt)) => self.publish_shortcut_candidate(
                        ctx,
                        &candidate,
                        ShortcutLibraryPublicationContinuation::Import(Box::new(receipt)),
                    ),
                    Err(error) => self
                        .state
                        .dialogs
                        .shortcut_portability
                        .complete_import(Err(error)),
                }
            }
            ShortcutPortabilityAction::CancelImportCommit => {
                #[cfg(target_arch = "wasm32")]
                {
                    let cancelled = self.state.cancel_pending_shortcut_library_publication();
                    if cancelled {
                        self.state.shortcut_library_publication_continuation = None;
                        self.state
                            .dialogs
                            .shortcut_portability
                            .import_commit_cancelled(Ok(()));
                    } else {
                        self.state.dialogs.shortcut_portability.import_commit_cancelled(Err(
                            "The durable commit boundary was already reached; the completed import remains active."
                                .to_owned(),
                        ));
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                self.state
                    .dialogs
                    .shortcut_portability
                    .import_commit_cancelled(Err(
                        "Native shortcut publication completes synchronously and cannot be cancelled after commit."
                            .to_owned(),
                    ));
            }
            ShortcutPortabilityAction::RollbackImport(receipt) => {
                match stage_shortcut_rollback(
                    self.state.ui.preferences.shortcut_profiles(),
                    &receipt,
                ) {
                    Ok(candidate) => self.publish_shortcut_candidate(
                        ctx,
                        &candidate,
                        ShortcutLibraryPublicationContinuation::Rollback,
                    ),
                    Err(error) => self
                        .state
                        .dialogs
                        .shortcut_portability
                        .complete_rollback(Err(error)),
                }
            }
            ShortcutPortabilityAction::PublishExport(artifact) => {
                let result = export_shortcut_artifact(&artifact).map_err(|error| error.to_string());
                if result.as_ref().is_ok_and(|outcome| {
                    !matches!(outcome, ShortcutArtifactExportOutcome::Cancelled)
                }) {
                    self.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Exported the keyboard shortcut profile.",
                    );
                }
                self.state
                    .dialogs
                    .shortcut_portability
                    .complete_export(result);
            }
        }
    }

    fn publish_shortcut_candidate(
        &mut self,
        ctx: &Context,
        candidate: &crate::workbench::shortcuts::ShortcutProfileLibrary,
        continuation: ShortcutLibraryPublicationContinuation,
    ) {
        if self
            .state
            .shortcut_library_publication_continuation
            .is_some()
        {
            self.state.complete_shortcut_library_publication(
                continuation,
                Err("another shortcut-library publication is awaiting acknowledgement".to_owned()),
            );
            return;
        }
        match self
            .state
            .publish_shortcut_library_candidate(candidate, ctx)
        {
            Ok(ShortcutLibraryPublication::Published) => {
                self.state
                    .complete_shortcut_library_publication(continuation, Ok(()));
            }
            Ok(ShortcutLibraryPublication::Pending) => {
                debug_assert!(
                    self.state
                        .shortcut_library_publication_continuation
                        .is_none()
                );
                self.state.shortcut_library_publication_continuation = Some(continuation);
            }
            Err(error) => self
                .state
                .complete_shortcut_library_publication(continuation, Err(error)),
        }
    }

    fn poll_shortcut_portability_source(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(result) =
            crate::workbench::shortcuts::artifacts::poll_browser_shortcut_artifact_import()
        {
            match result {
                Ok(ShortcutArtifactImportOutcome::Ready(source)) => {
                    let library = self.state.ui.preferences.shortcut_profiles().clone();
                    self.state
                        .dialogs
                        .shortcut_portability
                        .accept_import_source(&library, *source);
                }
                Ok(ShortcutArtifactImportOutcome::Cancelled) => self
                    .state
                    .dialogs
                    .shortcut_portability
                    .import_source_cancelled(),
                Err(error) => {
                    let message = error.to_string();
                    self.state
                        .dialogs
                        .shortcut_portability
                        .import_source_failed(message.clone());
                    self.state.record(
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
            self.state.prompt(refusal);
        }
    }

    fn begin_settings_export(&mut self, ctx: &Context) {
        let started = self.state.settings_file_bytes().and_then(|bytes| {
            crate::io::file_exchange::save_file(
                ctx,
                Id::new(SETTINGS_EXPORT_ID),
                SETTINGS_FILE,
                SETTINGS_FILE.fallback_name.to_owned(),
                bytes,
            )
        });
        if let Err(refusal) = started {
            self.state
                .prompt(format!("Settings were not exported: {refusal}"));
        }
    }

    /// Collect a settings-file pick, whether or not Preferences is still
    /// open: a result left in its mailbox would reach the next pick instead.
    fn poll_settings_file_transfer(&mut self, ctx: &Context) {
        match crate::io::file_exchange::take_saved(ctx, Id::new(SETTINGS_EXPORT_ID)) {
            Some(Ok(Some(saved))) => {
                self.state
                    .prompt(format!("Settings exported to {}.", saved.name));
            }
            Some(Err(refusal)) => self.state.prompt(refusal),
            Some(Ok(None)) | None => {}
        }
        match crate::io::file_exchange::take_opened(ctx, Id::new(SETTINGS_IMPORT_ID)) {
            Some(Ok(Some(file))) => {
                match self
                    .state
                    .stage_settings_import(&file.name, file.text.as_bytes())
                {
                    Ok((shortcuts, staged)) => self.publish_shortcut_candidate(
                        ctx,
                        &shortcuts,
                        ShortcutLibraryPublicationContinuation::SettingsImport(Box::new(staged)),
                    ),
                    Err(reason) => self.state.prompt(format!(
                        "Settings were not imported from {}: {reason}. Nothing was changed.",
                        file.name
                    )),
                }
            }
            Some(Err(refusal)) => self.state.prompt(refusal),
            Some(Ok(None)) | None => {}
        }
    }

    fn finish_shortcut_library_publication(&mut self) {
        if self
            .state
            .shortcut_library_publication_continuation
            .is_none()
        {
            if self.state.shortcut_library_publication_completion_pending() {
                let _ = self.state.take_shortcut_library_publication_completion();
                let message = "Saving keyboard shortcuts finished after the dialog that started it had closed. The saved shortcuts are in effect; review them in Preferences.";
                self.state.dialogs.shortcut_editor.persistence_pending = false;
                self.state.dialogs.shortcut_editor.error_summary = Some(message.to_owned());
                self.state.dialogs.shortcut_policy_candidate = None;
                self.state
                    .dialogs
                    .shortcut_portability
                    .complete_import(Err(message.to_owned()));
                self.state
                    .dialogs
                    .shortcut_portability
                    .complete_rollback(Err(message.to_owned()));
                self.state
                    .record(LogSeverity::Error, LogSource::Studio, message);
            }
            return;
        }
        let Some(result) = self.state.take_shortcut_library_publication_completion() else {
            return;
        };
        let continuation = self
            .state
            .shortcut_library_publication_continuation
            .take()
            .expect("completion owner was checked above");
        self.state
            .complete_shortcut_library_publication(continuation, result);
    }
}

impl AppState {
    fn complete_shortcut_library_publication(
        &mut self,
        continuation: ShortcutLibraryPublicationContinuation,
        result: Result<(), String>,
    ) {
        match continuation {
            ShortcutLibraryPublicationContinuation::Editor => {
                self.dialogs.shortcut_editor.persistence_pending = false;
                match result {
                    Ok(()) => {
                        self.dialogs.shortcut_editor.close_and_discard();
                        self.prompt("Keyboard shortcuts saved.");
                    }
                    Err(error) => {
                        let message = format!("Shortcut profile was not saved: {error}");
                        self.dialogs.shortcut_editor.error_summary = Some(message.clone());
                        self.record(LogSeverity::Error, LogSource::Studio, message);
                    }
                }
            }
            ShortcutLibraryPublicationContinuation::Policy => {
                self.dialogs.shortcut_policy_candidate = None;
                match result {
                    Ok(()) => self.prompt("Shortcut policy saved."),
                    Err(error) => {
                        self.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!("Shortcut policy was not saved: {error}"),
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
                self.dialogs
                    .shortcut_portability
                    .complete_import(result.map(|()| *receipt));
            }
            ShortcutLibraryPublicationContinuation::Rollback => {
                if result.is_ok() {
                    self.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Undid the shortcut profile import.",
                    );
                } else if let Err(error) = &result {
                    self.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("Undoing the shortcut profile import was not saved: {error}"),
                    );
                }
                self.dialogs.shortcut_portability.complete_rollback(result);
            }
            ShortcutLibraryPublicationContinuation::SettingsImport(staged) => {
                self.finish_settings_import(*staged, result);
            }
        }
    }
}

fn stage_shortcut_import(
    base: &crate::workbench::shortcuts::ShortcutProfileLibrary,
    plan: &crate::workbench::shortcuts::artifacts::ShortcutImportPlan,
) -> Result<
    (
        crate::workbench::shortcuts::ShortcutProfileLibrary,
        ShortcutImportReceipt,
    ),
    String,
> {
    let mut staged = base.clone();
    let receipt =
        apply_shortcut_import(&mut staged, plan, |_| Ok(())).map_err(|error| error.to_string())?;
    Ok((staged, receipt))
}

fn stage_shortcut_rollback(
    base: &crate::workbench::shortcuts::ShortcutProfileLibrary,
    receipt: &ShortcutImportReceipt,
) -> Result<crate::workbench::shortcuts::ShortcutProfileLibrary, String> {
    let mut staged = base.clone();
    rollback_shortcut_import(&mut staged, receipt, |_| Ok(()))
        .map_err(|error| error.to_string())?;
    Ok(staged)
}

fn close_preferences(state: &mut AppState) {
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

/// The autosave interval a fresh session carries, and the one Restore
/// defaults puts the Projects page back to.
pub(in crate::workbench) const DEFAULT_AUTOSAVE_MINUTES: u8 = 5;

pub(in crate::workbench) fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}

pub(in crate::workbench) fn engineering_profile_detail(
    profile: crate::workbench::EngineeringProfile,
) -> &'static str {
    if profile == crate::workbench::EngineeringProfile::All {
        "Capability discovery reports every installed or specified specialist workspace for evaluators, administrators and cross-domain teams."
    } else {
        profile.detail()
    }
}

fn engineering_profile_updated_message(label: &str) -> String {
    format!("Engineering profile set to {label}. Open project documents are unchanged.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::{BrowserHistoryEffect, ChoicePreference, TogglePreference};

    fn preferences_route() -> SurfaceRoute {
        SurfaceRoute::surface(SurfaceId::Preferences)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn test_app() -> RSpiceApp {
        RSpiceApp::test_instance()
    }

    #[test]
    fn close_returns_to_the_exact_source_route_and_requests_history_traversal() {
        let mut state = AppState::default();
        let source = state.workbench.current_route();
        state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::User)
            .unwrap();
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Push(preferences_route()))
        );
        close_preferences(&mut state);
        assert_eq!(state.workbench.current_route(), source);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -1,
                destination: source,
            })
        );
    }

    #[test]
    fn direct_deep_link_close_replaces_with_the_retained_workspace() {
        let mut state = AppState::default();
        let fallback = SurfaceRoute::surface(SurfaceId::Design);
        state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::BrowserPop)
            .unwrap();
        close_preferences(&mut state);
        assert_eq!(state.workbench.current_route(), fallback);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(fallback))
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn menu_and_shortcut_entries_open_the_canonical_preferences_route() {
        use crate::workbench::commands::vocabulary::Command;
        let mut menu_app = test_app();
        Command::Preferences.execute(&mut menu_app);
        assert_eq!(
            menu_app.state.workbench.current_route(),
            preferences_route()
        );

        let mut shortcut_app = test_app();
        shortcut_app.execute_shortcut_command(Command::Preferences);
        assert_eq!(
            shortcut_app.state.workbench.current_route(),
            preferences_route()
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn stale_retention_marker_cannot_orphan_preferences() {
        let ctx = Context::default();
        let mut app = test_app();
        app.state.dialogs.preferences_open = true;
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            app.render_preferences_dialog(ctx)
        });
        assert!(output.platform_output.accesskit_update.is_none());
        assert!(!app.state.dialogs.preferences_open);
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::Design
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn route_loss_releases_idle_preferences_child_modals() {
        let ctx = Context::default();
        let mut app = test_app();
        app.state.dialogs.preferences_open = true;
        app.state.dialogs.shortcut_portability.open_import();
        app.state
            .dialogs
            .workspace_layout_manager
            .open(crate::workbench::WorkspacePreset::Engineering);

        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            app.render_preferences_dialog(ctx)
        });

        assert!(!app.state.dialogs.preferences_open);
        assert!(
            !app.state
                .dialogs
                .shortcut_portability
                .application_modal_open()
        );
        assert!(!app.state.dialogs.workspace_layout_manager.open);
        assert!(!app.state.dialogs.application_modal_open());
    }

    /// Every category reaches a page with something on it that can be
    /// operated.
    ///
    /// This replaces a check that each page painted a level-3 heading, which
    /// the three categories that held nothing else also passed: a heading and
    /// a sentence saying the page had no settings satisfied it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_category_renders_a_control_the_reader_can_operate() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        for category in PreferenceCategory::ALL {
            let mut state = AppState::default();
            let mut actions = PreferencePageActions::default();
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 1200.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        preference_pages::render(ui, category, &mut state, &mut actions);
                    });
                },
            );
            let nodes = output
                .platform_output
                .accesskit_update
                .expect("AccessKit update")
                .nodes;
            let controls = nodes
                .iter()
                .filter(|(_, node)| {
                    matches!(
                        node.role(),
                        egui::accesskit::Role::ComboBox
                            | egui::accesskit::Role::CheckBox
                            | egui::accesskit::Role::Button
                            | egui::accesskit::Role::Slider
                            | egui::accesskit::Role::SpinButton
                            | egui::accesskit::Role::RadioButton
                    )
                })
                .count();
            assert!(
                controls > 0,
                "{} renders no control at all",
                category.label()
            );
        }
    }

    #[test]
    fn complete_preference_model_round_trips_through_the_app_session() {
        let mut state = AppState::default();
        state
            .ui
            .preferences
            .set_choice(ChoicePreference::InterfaceScale, 2)
            .unwrap();
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::ReducedMotion, true);
        let json = serde_json::to_string(&state).unwrap();
        let restored: AppState = serde_json::from_str(&json).unwrap();
        assert_eq!(
            restored
                .ui
                .preferences
                .choice(ChoicePreference::InterfaceScale),
            2
        );
        assert!(
            restored
                .ui
                .preferences
                .toggle(TogglePreference::ReducedMotion)
        );
        assert_eq!(restored.ui.preferences.interface_scale(), 1.25);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_exposed_preferences_action_routes_to_a_real_executor() {
        let mut layout_app = test_app();
        layout_app.execute_preference_page_actions(
            &Context::default(),
            PreferencePageActions {
                open_workspace_layout_manager: true,
                ..PreferencePageActions::default()
            },
        );
        assert!(layout_app.state.dialogs.workspace_layout_manager.open);
        assert!(layout_app.state.dialogs.application_modal_open());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn shortcut_editor_action_opens_an_isolated_profile_draft() {
        let mut app = test_app();
        app.execute_preference_page_actions(
            &Context::default(),
            PreferencePageActions {
                open_shortcut_editor: true,
                ..PreferencePageActions::default()
            },
        );
        assert!(app.state.dialogs.shortcut_editor.open);
        assert_eq!(
            app.state.dialogs.shortcut_editor.draft.as_ref(),
            Some(app.state.ui.preferences.shortcuts())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_shortcut_preferences_action_has_a_retained_executor() {
        let ctx = Context::default();

        let mut import_app = test_app();
        import_app.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                open_shortcut_import: true,
                ..PreferencePageActions::default()
            },
        );
        assert!(
            import_app
                .state
                .dialogs
                .shortcut_portability
                .application_modal_open()
        );

        let mut export_app = test_app();
        export_app.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                open_shortcut_export: true,
                ..PreferencePageActions::default()
            },
        );
        assert!(
            export_app
                .state
                .dialogs
                .shortcut_portability
                .application_modal_open()
        );
    }

    /// Export… writes a settings file through the shared picker, and Import…
    /// on another installation puts its contents in force.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn settings_file_actions_move_settings_between_installations() {
        use crate::io::file_exchange::{ScriptedChoice, script_next_choice, take_pickers_opened};

        let ctx = Context::default();
        let folder =
            std::env::temp_dir().join(format!("rspice-settings-file-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).expect("temp folder");
        let path = folder.join("rspice-settings.json");
        let _ = take_pickers_opened();

        let mut source = test_app();
        source.state.ui.autosave_minutes = 10;
        script_next_choice(ScriptedChoice::Chose(path.clone()));
        source.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                export_settings_file: true,
                ..PreferencePageActions::default()
            },
        );
        source.poll_settings_file_transfer(&ctx);
        assert_eq!(
            source.state.prompt_text(),
            Some("Settings exported to rspice-settings.json.")
        );

        let mut target = test_app();
        target.state.enable_volatile_test_shortcut_persistence();
        script_next_choice(ScriptedChoice::Chose(path));
        target.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                import_settings_file: true,
                ..PreferencePageActions::default()
            },
        );
        target.poll_settings_file_transfer(&ctx);
        assert_eq!(target.state.ui.autosave_minutes, 10);
        assert_eq!(
            target.state.prompt_text(),
            Some("Settings imported from rspice-settings.json.")
        );
        assert_eq!(
            take_pickers_opened(),
            vec![SETTINGS_FILE.label, SETTINGS_FILE.label]
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn shortcut_import_and_export_actions_map_to_their_exact_dialogs() {
        fn rendered_dialog_title(actions: PreferencePageActions) -> String {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut app = test_app();
            app.execute_preference_page_actions(&ctx, actions);
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900.0, 720.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| app.render_shortcut_portability_dialogs(ctx),
            );
            output
                .platform_output
                .accesskit_update
                .expect("dialog AccessKit update")
                .nodes
                .iter()
                .find_map(|(_, node)| {
                    (node.role() == egui::accesskit::Role::Dialog)
                        .then(|| node.label().map(str::to_owned))
                        .flatten()
                })
                .expect("exact portability dialog title")
        }

        assert_eq!(
            rendered_dialog_title(PreferencePageActions {
                open_shortcut_import: true,
                ..PreferencePageActions::default()
            }),
            shortcut_portability_dialogs::IMPORT_TITLE
        );
        assert_eq!(
            rendered_dialog_title(PreferencePageActions {
                open_shortcut_export: true,
                ..PreferencePageActions::default()
            }),
            shortcut_portability_dialogs::EXPORT_TITLE
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn shortcut_policy_action_publishes_before_the_candidate_becomes_live() {
        let ctx = Context::default();
        let mut app = test_app();
        app.state.enable_volatile_test_shortcut_persistence();
        let before = app.state.ui.preferences.shortcuts().clone();
        let mut candidate = before.clone();
        candidate
            .policies_mut()
            .set_chord_timeout(crate::workbench::ChordTimeoutPolicy::ThreeSeconds);

        app.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                shortcut_policy_candidate: Some(candidate.clone()),
                ..PreferencePageActions::default()
            },
        );

        assert_ne!(before, candidate);
        assert_eq!(app.state.ui.preferences.shortcuts(), &candidate);
        assert_eq!(
            app.state
                .shortcut_library_persistence
                .persisted()
                .expect("volatile canonical shortcut snapshot")
                .library()
                .active(),
            &candidate
        );
        assert!(app.state.dialogs.shortcut_policy_candidate.is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn shortcut_policy_persistence_failure_keeps_live_state_unchanged_and_recovers_controls() {
        let ctx = Context::default();
        let mut app = test_app();
        let before = app.state.ui.preferences.shortcuts().clone();
        let mut candidate = before.clone();
        candidate
            .policies_mut()
            .set_chord_timeout(crate::workbench::ChordTimeoutPolicy::ThreeSeconds);
        app.state.shortcut_library_persistence =
            crate::workbench::app_state::session::shortcut_library::ShortcutLibraryPersistenceRuntime::Unavailable(
                "simulated canonical storage failure".to_owned(),
            );

        app.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                shortcut_policy_candidate: Some(candidate),
                ..PreferencePageActions::default()
            },
        );

        assert_eq!(app.state.ui.preferences.shortcuts(), &before);
        assert!(app.state.dialogs.shortcut_policy_candidate.is_none());
        assert!(
            app.state
                .shortcut_library_publication_continuation
                .is_none()
        );
    }

    #[test]
    fn legacy_autosave_values_migrate_to_mockup_supported_intervals() {
        assert_eq!(normalize_autosave_minutes(0), 2);
        assert_eq!(normalize_autosave_minutes(5), 5);
        assert_eq!(normalize_autosave_minutes(15), 10);
    }

    #[test]
    fn engineering_profile_copy_describes_implemented_behavior() {
        assert_eq!(
            engineering_profile_updated_message("Power electronics"),
            "Engineering profile set to Power electronics. Open project documents are unchanged."
        );
    }
}
