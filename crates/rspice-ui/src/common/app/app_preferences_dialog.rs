//! Preferences — canonical mockup shell with durable, validated settings.

mod managed_preference_policy;
mod preference_pages;
mod preferences_shell;
pub(crate) mod shortcut_portability_dialogs;
mod shortcut_preferences;
pub(crate) mod workspace_layout_manager;

use egui::{Context, Id};

use crate::common::shortcut_artifacts::{
    ShortcutArtifactExportOutcome, ShortcutArtifactImportOutcome, ShortcutImportReceipt,
    apply_shortcut_import, export_shortcut_artifact, rollback_shortcut_import,
};
use crate::workbench::commands::ShortcutContext;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use self::preferences_shell::PreferenceCategory;
use self::shortcut_portability_dialogs::{ShortcutExportEnvironment, ShortcutPortabilityAction};
use super::app_shortcut_library_persistence::{
    ShortcutLibraryPublication, ShortcutLibraryPublicationContinuation,
};
use super::{AppState, ConsoleMessage, RSpiceApp};

const CATEGORY_STATE_ID: &str = "rspice.preferences.active-category";
pub(super) const ENGINEERING_PROFILE_HELP: &str = "Sets the discipline context reported by capability discovery and the availability matrix. Deep links and project documents remain available.";
pub(super) const CAPABILITY_MATRIX_HELP: &str = "See installed, licensed, preview and profile-hidden capabilities without duplicating their settings.";

#[derive(Debug, Default)]
pub(super) struct PreferencePageActions {
    pub(super) open_capability_matrix: bool,
    pub(super) open_resolved_preference_policy: bool,
    pub(super) open_workspace_layout_manager: bool,
    pub(super) open_shortcut_editor: bool,
    pub(super) open_shortcut_import: bool,
    pub(super) open_shortcut_export: bool,
    pub(super) open_resolved_shortcut_policy: bool,
    pub(super) shortcut_policy_candidate: Option<crate::workbench::ShortcutPreferences>,
    pub(super) updated_profile_label: Option<&'static str>,
}

impl RSpiceApp {
    pub(super) fn render_preferences_dialog(&mut self, ctx: &Context) {
        self.poll_shortcut_portability_source();
        self.finish_shortcut_library_publication();
        let category_id = Id::new(CATEGORY_STATE_ID);
        let current_surface = self.state.workbench.current_route().surface_id();
        let route_owned = current_surface == SurfaceId::Preferences;
        let retained_under_child_manager = self.state.dialogs.preferences_open
            && matches!(
                current_surface,
                SurfaceId::FeatureAvailability | SurfaceId::AccountOrganization
            );
        if !route_owned && !retained_under_child_manager {
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
            self.state.dialogs.managed_preference_policy_open = false;
            self.state.dialogs.workspace_layout_manager.close();
            ctx.data_mut(|data| data.remove_temp::<PreferenceCategory>(category_id));
            preferences_shell::unmount(ctx);
            return;
        }

        let mut category = ctx
            .data_mut(|data| data.get_temp::<PreferenceCategory>(category_id))
            .unwrap_or_default();
        let mut actions = PreferencePageActions::default();
        // The retained parent does not consume Escape while one of its real
        // child executors is open above it.
        let nested_modal_open = self.state.dialogs.license_dialog.open
            || self.state.dialogs.shortcut_editor.open
            || self
                .state
                .dialogs
                .shortcut_portability
                .application_modal_open()
            || self.state.dialogs.managed_preference_policy_open
            || self.state.dialogs.workspace_layout_manager.open
            || retained_under_child_manager;
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
            self.state.push_console_message(ConsoleMessage::info(
                engineering_profile_updated_message(label),
            ));
        }
        if shell_response.close_requested {
            self.state.dialogs.preferences_open = false;
            if route_owned {
                close_preferences(&mut self.state);
            }
        }
        if shell_response.account_organization_requested {
            crate::workbench::commands::Command::AccountOrganization.execute(self);
            self.state.dialogs.preferences_open =
                self.state.workbench.current_route().surface_id() == SurfaceId::AccountOrganization;
        }
        self.execute_preference_page_actions(ctx, actions);
        shortcut_preferences::render_editor(ctx, &mut self.state);
        managed_preference_policy::render(ctx, &mut self.state, category);
        workspace_layout_manager::render(ctx, &mut self.state);
        self.render_shortcut_portability_dialogs(ctx);
    }

    fn execute_preference_page_actions(&mut self, ctx: &Context, actions: PreferencePageActions) {
        if actions.open_capability_matrix {
            crate::workbench::commands::Command::FeatureAvailability.execute(self);
            self.state.dialogs.preferences_open =
                self.state.workbench.current_route().surface_id() == SurfaceId::FeatureAvailability;
        }
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
        if actions.open_resolved_preference_policy || actions.open_resolved_shortcut_policy {
            self.state.dialogs.managed_preference_policy_open = true;
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
            self.state.push_user_message(ConsoleMessage::error(format!(
                "Shortcut policy was not saved: {error}"
            )));
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
            runtime_platform: crate::common::app::runtime_command_platform(ctx),
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
                match crate::common::shortcut_artifacts::import_shortcut_artifact_source() {
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
                    crate::common::shortcut_artifacts::start_browser_shortcut_artifact_import()
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
                        crate::common::shortcut_artifacts::cancel_browser_shortcut_artifact_import(
                        );
                    match outcome {
                        crate::common::shortcut_artifacts::BrowserShortcutArtifactImportCancelOutcome::Cancelled
                        | crate::common::shortcut_artifacts::BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased => {
                            self.state
                                .dialogs
                                .shortcut_portability
                                .import_source_cancelled();
                        }
                        crate::common::shortcut_artifacts::BrowserShortcutArtifactImportCancelOutcome::StillOwned => {}
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
                    self.state.push_console_message(ConsoleMessage::info(
                        "Shortcut profile export was handed to the selected platform destination.",
                    ));
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
            crate::common::shortcut_artifacts::poll_browser_shortcut_artifact_import()
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
                    self.state
                        .push_console_message(ConsoleMessage::error(format!(
                            "Shortcut import source could not be opened: {message}"
                        )));
                }
            }
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
                let message = "Canonical shortcut publication completed without its retained workflow owner. The stored state is authoritative; the interrupted workflow was recovered and no further publication will start until this completion is acknowledged.";
                log::error!("{message}");
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
                self.state.push_user_message(ConsoleMessage::error(message));
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
                        self.push_console_message(ConsoleMessage::info(
                            "Keyboard shortcut profile saved after complete collision and reserved-binding validation.",
                        ));
                    }
                    Err(error) => {
                        self.dialogs.shortcut_editor.error_summary =
                            Some(format!("Shortcut profile was not saved: {error}"));
                        self.push_console_message(ConsoleMessage::error(format!(
                            "Shortcut profile was not saved: {error}"
                        )));
                    }
                }
            }
            ShortcutLibraryPublicationContinuation::Policy => {
                self.dialogs.shortcut_policy_candidate = None;
                match result {
                    Ok(()) => self.push_console_message(ConsoleMessage::info(
                        "Shortcut binding policy saved to the canonical user profile.",
                    )),
                    Err(error) => self.push_user_message(ConsoleMessage::error(format!(
                        "Shortcut policy was not saved: {error}"
                    ))),
                }
            }
            ShortcutLibraryPublicationContinuation::Import(receipt) => {
                if result.is_ok() {
                    self.push_console_message(ConsoleMessage::info(format!(
                        "Imported shortcut profile '{}' \u{00b7} receipt {}.",
                        receipt.source_name(),
                        receipt.id()
                    )));
                } else if let Err(error) = &result {
                    self.push_user_message(ConsoleMessage::error(format!(
                        "Shortcut profile import was not saved: {error}"
                    )));
                }
                self.dialogs
                    .shortcut_portability
                    .complete_import(result.map(|()| *receipt));
            }
            ShortcutLibraryPublicationContinuation::Rollback => {
                if result.is_ok() {
                    self.push_console_message(ConsoleMessage::info(
                        "Shortcut import was rolled back through its retained receipt.",
                    ));
                } else if let Err(error) = &result {
                    self.push_user_message(ConsoleMessage::error(format!(
                        "Shortcut import rollback was not saved: {error}"
                    )));
                }
                self.dialogs.shortcut_portability.complete_rollback(result);
            }
        }
    }
}

fn stage_shortcut_import(
    base: &crate::workbench::shortcuts::ShortcutProfileLibrary,
    plan: &crate::common::shortcut_artifacts::ShortcutImportPlan,
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
        state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close Preferences: {error}"
        )));
    }
}

pub(super) fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}

pub(super) fn engineering_profile_detail(
    profile: crate::workbench::EngineeringProfile,
) -> &'static str {
    if profile == crate::workbench::EngineeringProfile::All {
        "Capability discovery reports every installed or specified specialist workspace for evaluators, administrators and cross-domain teams."
    } else {
        profile.detail()
    }
}

fn engineering_profile_updated_message(label: &str) -> String {
    format!(
        "Engineering profile updated · {label} capability-availability context is active. Existing project documents remain available."
    )
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
        use crate::workbench::commands::Command;
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
        let output = ctx.run(egui::RawInput::default(), |ctx| {
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
        app.state.dialogs.managed_preference_policy_open = true;
        app.state
            .dialogs
            .workspace_layout_manager
            .open(crate::workbench::WorkspacePreset::Engineering);

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            app.render_preferences_dialog(ctx)
        });

        assert!(!app.state.dialogs.preferences_open);
        assert!(
            !app.state
                .dialogs
                .shortcut_portability
                .application_modal_open()
        );
        assert!(!app.state.dialogs.managed_preference_policy_open);
        assert!(!app.state.dialogs.workspace_layout_manager.open);
        assert!(!app.state.dialogs.application_modal_open());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_runtime_backed_category_renders_owned_page_content() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        for category in PreferenceCategory::ALL {
            let mut state = AppState::default();
            let mut actions = PreferencePageActions::default();
            let output = ctx.run(
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
            assert!(
                nodes.iter().any(|(_, node)| {
                    node.role() == egui::accesskit::Role::Heading && node.level() == Some(3)
                }),
                "{} has no page heading",
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
        let mut app = test_app();
        app.state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::BrowserPop)
            .unwrap();
        app.execute_preference_page_actions(
            &Context::default(),
            PreferencePageActions {
                open_capability_matrix: true,
                ..PreferencePageActions::default()
            },
        );
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::FeatureAvailability
        );
        assert!(app.state.dialogs.preferences_open);

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

        let mut policy_app = test_app();
        policy_app.execute_preference_page_actions(
            &ctx,
            PreferencePageActions {
                open_resolved_shortcut_policy: true,
                ..PreferencePageActions::default()
            },
        );
        assert!(policy_app.state.dialogs.managed_preference_policy_open);
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
            let output = ctx.run(
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
            crate::common::app::app_shortcut_library_persistence::ShortcutLibraryPersistenceRuntime::Unavailable(
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
            "Engineering profile updated · Power electronics capability-availability context is active. Existing project documents remain available."
        );
    }
}
