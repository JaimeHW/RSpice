//! Preferences — the dedicated settings shell specified by the workbench
//! mockup.
//!
//! Every exposed setting is live, persisted, and consumed by the runtime.
//! Mockup categories whose underlying product behavior is not implemented are
//! intentionally withheld rather than presented as inert controls.

mod preferences_shell;

use egui::{Context, Id};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, select};
use crate::ui::{Density, EngineeringCanvasTheme, Mode};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use self::preferences_shell::{
    PreferenceCategory, page_heading, preference_switch, right_aligned, scope_strip, section_label,
    segmented, setting_row,
};
use super::{AppState, ConsoleMessage, RSpiceApp};

const CATEGORY_STATE_ID: &str = "rspice.preferences.active-category";
const SETTINGS_RANGE_WIDTH: f32 = 150.0;
const ENGINEERING_PROFILE_HELP: &str = "Sets the discipline context reported by capability discovery and the availability matrix. Deep links and project documents remain available.";
const CAPABILITY_MATRIX_HELP: &str = "See installed, licensed, preview and profile-hidden capabilities without duplicating their settings.";

#[derive(Debug, Default)]
struct PreferencePageActions {
    open_capability_matrix: bool,
    updated_profile_label: Option<&'static str>,
}

impl RSpiceApp {
    pub(super) fn render_preferences_dialog(&mut self, ctx: &Context) {
        let category_id = Id::new(CATEGORY_STATE_ID);
        let current_surface = self.state.workbench.current_route().surface_id();
        let route_owned = current_surface == SurfaceId::Preferences;
        let retained_under_capability_manager = self.state.dialogs.preferences_open
            && current_surface == SurfaceId::FeatureAvailability;
        if !route_owned && !retained_under_capability_manager {
            // `preferences_open` is only a retained-under-child marker. It is
            // never an alternate route owner, so browser traversal cannot
            // leave an orphaned Preferences modal over an unrelated task.
            self.state.dialogs.preferences_open = false;
            ctx.data_mut(|data| {
                data.remove_temp::<PreferenceCategory>(category_id);
            });
            preferences_shell::unmount(ctx);
            return;
        }

        let mut category = ctx
            .data_mut(|data| data.get_temp::<PreferenceCategory>(category_id))
            .unwrap_or_default();
        let mut actions = PreferencePageActions::default();
        // Preferences intentionally remains mounted below the two workflows
        // it can open. The lower surface must not consume Escape before the
        // later-rendered top modal gets a chance to close itself.
        let nested_modal_open =
            self.state.dialogs.license_dialog.open || retained_under_capability_manager;
        let shell_response = {
            let state = &mut self.state;
            preferences_shell::show(ctx, &mut category, !nested_modal_open, |ui, category| {
                render_preference_page(ui, category, state, &mut actions);
            })
        };
        if shell_response.close_requested {
            ctx.data_mut(|data| {
                data.remove_temp::<PreferenceCategory>(category_id);
            });
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
        if shell_response.license_activation_requested {
            // Preferences remains beneath the implemented license-activation
            // workflow, preserving the stacked-dialog return behavior.
            self.open_license_dialog();
        }
        if actions.open_capability_matrix {
            // Route overlays render after application dialogs, so this opens
            // above Preferences and returns to the exact category on close.
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

    // Direct deep links have no in-application predecessor. Replace the
    // manager route with the retained primary workspace projection so Close
    // updates the address without creating a Preferences/back loop.
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

fn render_preference_page(
    ui: &mut egui::Ui,
    category: PreferenceCategory,
    state: &mut AppState,
    actions: &mut PreferencePageActions,
) {
    match category {
        PreferenceCategory::Appearance => render_appearance(ui, state),
        PreferenceCategory::Workspace => render_workspace(ui, state, actions),
        PreferenceCategory::Files => render_files(ui, state),
    }
}

fn render_appearance(ui: &mut egui::Ui, state: &mut AppState) {
    scope_strip(ui, "User profile", "portable across desktop and web");
    page_heading(
        ui,
        "Appearance",
        "Shared across desktop and web. Touch target sizing follows the active platform.",
    );

    setting_row(
        ui,
        "Color mode",
        "Choose the workspace surface mode.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = Mode::ALL
                    .iter()
                    .position(|mode| *mode == state.ui.theme.mode)
                    .unwrap_or_default();
                if segmented(
                    ui,
                    "preferences.appearance.color-mode",
                    &["Dark", "Light", "System"],
                    &mut selected,
                ) {
                    state.ui.theme.mode = Mode::ALL[selected];
                }
            });
        },
    );

    setting_row(
        ui,
        "Density",
        "Compact is optimized for engineering workstations.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = usize::from(state.ui.theme.density == Density::Relaxed);
                if segmented(
                    ui,
                    "preferences.appearance.density",
                    &["Compact", "Comfortable"],
                    &mut selected,
                ) {
                    state.ui.theme.density = if selected == 0 {
                        Density::Compact
                    } else {
                        Density::Relaxed
                    };
                }
            });
        },
    );

    setting_row(
        ui,
        "Color-safe traces",
        "Use color, line style and marker redundancy.",
        |ui| {
            right_aligned(ui, |ui| {
                preference_switch(
                    ui,
                    "result-document.settingspagecontent.enable-color-safe-traces.425c2b40",
                    "Enable color-safe traces",
                    &mut state.ui.theme.colorblind_traces,
                );
            });
        },
    );

    setting_row(
        ui,
        "Canvas contrast",
        "Increase symbol and grid separation.",
        |ui| {
            right_aligned(ui, |ui| {
                // The mockup keeps range controls at 150 px on every
                // breakpoint; the phone row stacks, but does not stretch the
                // explicitly sized range input.
                let width = SETTINGS_RANGE_WIDTH.min(ui.available_width());
                ui.push_id(
                    "user-preferences.settingspagecontent.canvas-contrast.9171c299",
                    |ui| {
                        let response = ui.add_sized(
                            [width, Tokens::get(ui.ctx()).metrics.ctl_h],
                            egui::Slider::new(&mut state.ui.theme.canvas_contrast, 0..=100)
                                .show_value(false),
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Slider,
                                true,
                                "Canvas contrast",
                            )
                        });
                    },
                );
            });
        },
    );

    section_label(ui, "Visual system");
    setting_row(
        ui,
        "Engineering canvas theme",
        "Set schematic, layout, waveform and field-view backgrounds as one reviewed palette.",
        |ui| {
            let themes = EngineeringCanvasTheme::ALL;
            let labels = themes
                .iter()
                .map(|theme| theme.label().to_owned())
                .collect::<Vec<_>>();
            let active = themes
                .iter()
                .position(|theme| *theme == state.ui.theme.canvas_theme)
                .unwrap_or_default();
            right_aligned(ui, |ui| {
                let width = ui.available_width().min(360.0);
                if let Some(selected) = select(
                    ui,
                    "user-preferences.settingsextendedcontent.select-control.0e145472",
                    "Engineering canvas theme",
                    &labels[active],
                    &labels,
                    width,
                ) {
                    state.ui.theme.canvas_theme = themes[selected];
                }
            });
        },
    );
}

fn render_workspace(ui: &mut egui::Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    scope_strip(
        ui,
        "Device local",
        "dock geometry and restored documents stay on this device",
    );
    section_label(ui, "Product scope");
    setting_row(ui, "Engineering profile", ENGINEERING_PROFILE_HELP, |ui| {
        let profiles = crate::workbench::EngineeringProfile::ALL;
        let labels = profiles
            .iter()
            .map(|profile| profile.label().to_owned())
            .collect::<Vec<_>>();
        let active = profiles
            .iter()
            .position(|profile| *profile == state.workbench.engineering_profile)
            .unwrap_or_default();
        let width = ui.available_width().min(440.0);
        if let Some(selected) = select(
            ui,
            "user-preferences.settingsprioritycontent.engineering-profile.47d1f11a",
            "Engineering profile",
            profiles[active].label(),
            &labels,
            width,
        ) {
            let profile = profiles[selected];
            if profile != state.workbench.engineering_profile {
                state.workbench.engineering_profile = profile;
                actions.updated_profile_label = Some(profile.label());
            }
        }
        ui.add_space(4.0);
        setting_detail(
            ui,
            engineering_profile_detail(state.workbench.engineering_profile),
        );
    });

    setting_row(
        ui,
        "Feature ownership and availability",
        CAPABILITY_MATRIX_HELP,
        |ui| {
            right_aligned(ui, |ui| {
                let phone = ui.ctx().content_rect().width() <= 560.0;
                let clicked = ui
                    .push_id("feature-availability", |ui| {
                        Button::new("Review matrix…")
                            .min_width(if phone { ui.available_width() } else { 0.0 })
                            .show(ui)
                            .clicked()
                    })
                    .inner;
                if clicked {
                    actions.open_capability_matrix = true;
                }
            });
        },
    );
}

fn render_files(ui: &mut egui::Ui, state: &mut AppState) {
    scope_strip(
        ui,
        "User + project",
        "locations are device local; retention policy is project portable",
    );
    page_heading(
        ui,
        "Files, storage and recovery",
        "Transactional writes, content-addressed results and explicit retention.",
    );
    section_label(ui, "Recovery");
    setting_row(
        ui,
        "Autosave interval",
        "Checkpoint active editable documents.",
        |ui| {
            let values = [5_u8, 2, 10];
            let labels = ["5 minutes", "2 minutes", "10 minutes"]
                .map(str::to_owned)
                .to_vec();
            let normalized = normalize_autosave_minutes(state.ui.autosave_minutes);
            let active = values
                .iter()
                .position(|minutes| *minutes == normalized)
                .unwrap_or_default();
            right_aligned(ui, |ui| {
                let width = ui.available_width().min(360.0);
                if let Some(selected) = select(
                    ui,
                    "user-preferences.settingspagecontent.select-control.1874449c",
                    "Autosave interval",
                    &labels[active],
                    &labels,
                    width,
                ) {
                    state.ui.autosave_minutes = values[selected];
                }
            });
        },
    );
}

fn setting_detail(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
}

fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}

fn engineering_profile_detail(profile: crate::workbench::EngineeringProfile) -> &'static str {
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
    use crate::workbench::BrowserHistoryEffect;

    fn preferences_route() -> SurfaceRoute {
        SurfaceRoute::surface(SurfaceId::Preferences)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn test_app() -> RSpiceApp {
        RSpiceApp {
            state: AppState::default(),
            first_frame: false,
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    #[test]
    fn close_returns_to_the_exact_source_route_and_requests_history_traversal() {
        let mut state = AppState::default();
        let source = state.workbench.current_route();
        state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::User)
            .expect("Preferences route is executable");
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
            .expect("Preferences deep link is executable");
        assert_eq!(state.workbench.current_route(), preferences_route());
        assert_eq!(state.workbench.previous_route(), None);

        close_preferences(&mut state);

        assert_eq!(state.workbench.current_route(), fallback);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(fallback))
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn menu_and_shortcut_entries_both_open_the_canonical_preferences_route() {
        use crate::workbench::commands::Command;

        let mut menu_app = test_app();
        Command::Preferences.execute(&mut menu_app);
        assert_eq!(
            menu_app.state.workbench.current_route(),
            preferences_route()
        );
        assert!(!menu_app.state.dialogs.preferences_open);

        let mut shortcut_app = test_app();
        shortcut_app.execute_shortcut_command(Command::Preferences);
        assert_eq!(
            shortcut_app.state.workbench.current_route(),
            preferences_route()
        );
        assert!(!shortcut_app.state.dialogs.preferences_open);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn canonical_route_renders_preferences_without_the_legacy_dialog_flag() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = test_app();
        app.state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::BrowserPop)
            .expect("Preferences deep link is executable");

        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| app.render_preferences_dialog(ctx),
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog && node.label() == Some("Preferences")
        }));
        assert!(!app.state.dialogs.preferences_open);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn stale_retention_marker_cannot_orphan_preferences_over_an_unrelated_route() {
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
    fn browser_navigation_away_unmounts_transient_category_state() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 800.0),
            )),
            ..Default::default()
        };
        let mut app = test_app();
        app.state
            .workbench
            .navigate(preferences_route(), RouteTransitionSource::BrowserPop)
            .expect("Preferences deep link is executable");
        let _ = ctx.run(input(), |ctx| app.render_preferences_dialog(ctx));
        ctx.data_mut(|data| {
            data.insert_temp(Id::new(CATEGORY_STATE_ID), PreferenceCategory::Workspace);
        });

        app.state
            .workbench
            .navigate(
                SurfaceRoute::surface(SurfaceId::Design),
                RouteTransitionSource::BrowserPop,
            )
            .expect("Design route is executable");
        let _ = ctx.run(input(), |ctx| app.render_preferences_dialog(ctx));

        assert!(ctx.data(|data| {
            data.get_temp::<PreferenceCategory>(Id::new(CATEGORY_STATE_ID))
                .is_none()
        }));
        assert!(!app.state.dialogs.preferences_open);
    }

    #[test]
    fn appearance_exposes_every_runtime_backed_mockup_control() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 680.0),
            )),
            ..Default::default()
        };
        let mut state = AppState::default();

        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| render_appearance(ui, &mut state));
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let has = |role, label| {
            nodes
                .iter()
                .any(|(_, node)| node.role() == role && node.label() == Some(label))
        };

        for label in ["Dark", "Light", "System", "Compact", "Comfortable"] {
            assert!(has(egui::accesskit::Role::RadioButton, label));
        }
        assert!(has(
            egui::accesskit::Role::CheckBox,
            "Enable color-safe traces"
        ));
        assert!(has(egui::accesskit::Role::Slider, "Canvas contrast"));
        assert!(has(
            egui::accesskit::Role::ComboBox,
            "Engineering canvas theme"
        ));
        assert!(has(egui::accesskit::Role::Heading, "Visual system"));
    }

    #[test]
    fn engineering_profile_update_message_describes_implemented_behavior() {
        assert_eq!(
            engineering_profile_updated_message("Power electronics"),
            "Engineering profile updated · Power electronics capability-availability context is active. Existing project documents remain available."
        );
    }

    #[test]
    fn all_domains_profile_detail_does_not_claim_unimplemented_navigation_filtering() {
        assert_eq!(
            engineering_profile_detail(crate::workbench::EngineeringProfile::All),
            "Capability discovery reports every installed or specified specialist workspace for evaluators, administrators and cross-domain teams."
        );
    }

    #[test]
    fn legacy_autosave_values_migrate_to_mockup_supported_intervals() {
        assert_eq!(normalize_autosave_minutes(0), 2);
        assert_eq!(normalize_autosave_minutes(1), 2);
        assert_eq!(normalize_autosave_minutes(5), 5);
        assert_eq!(normalize_autosave_minutes(15), 10);
    }

    #[test]
    fn range_control_width_matches_the_mockup_at_every_breakpoint() {
        assert_eq!(SETTINGS_RANGE_WIDTH, 150.0);
        assert_eq!(SETTINGS_RANGE_WIDTH.min(120.0), 120.0);
    }
}
