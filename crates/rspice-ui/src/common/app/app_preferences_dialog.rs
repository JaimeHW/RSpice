//! Preferences — canonical mockup shell with durable, validated settings.

mod preference_pages;
mod preferences_shell;

use egui::{Context, Id};

use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use self::preferences_shell::PreferenceCategory;
use super::{AppState, ConsoleMessage, RSpiceApp};

const CATEGORY_STATE_ID: &str = "rspice.preferences.active-category";
pub(super) const ENGINEERING_PROFILE_HELP: &str = "Sets the discipline context reported by capability discovery and the availability matrix. Deep links and project documents remain available.";
pub(super) const CAPABILITY_MATRIX_HELP: &str = "See installed, licensed, preview and profile-hidden capabilities without duplicating their settings.";

#[derive(Debug, Default)]
pub(super) struct PreferencePageActions {
    pub(super) open_capability_matrix: bool,
    pub(super) updated_profile_label: Option<&'static str>,
}

impl RSpiceApp {
    pub(super) fn render_preferences_dialog(&mut self, ctx: &Context) {
        let category_id = Id::new(CATEGORY_STATE_ID);
        let current_surface = self.state.workbench.current_route().surface_id();
        let route_owned = current_surface == SurfaceId::Preferences;
        let retained_under_child_manager = self.state.dialogs.preferences_open
            && matches!(
                current_surface,
                SurfaceId::FeatureAvailability | SurfaceId::AccountOrganization
            );
        if !route_owned && !retained_under_child_manager {
            self.state.dialogs.preferences_open = false;
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
        let nested_modal_open =
            self.state.dialogs.license_dialog.open || retained_under_child_manager;
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
        self.execute_preference_page_actions(actions);
    }

    fn execute_preference_page_actions(&mut self, actions: PreferencePageActions) {
        if actions.open_capability_matrix {
            crate::workbench::commands::Command::FeatureAvailability.execute(self);
            self.state.dialogs.preferences_open =
                self.state.workbench.current_route().surface_id() == SurfaceId::FeatureAvailability;
        }
    }
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
        app.execute_preference_page_actions(PreferencePageActions {
            open_capability_matrix: true,
            ..PreferencePageActions::default()
        });
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::FeatureAvailability
        );
        assert!(app.state.dialogs.preferences_open);
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
