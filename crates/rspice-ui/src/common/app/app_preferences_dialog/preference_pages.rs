//! Runtime-backed Preferences page content.
//!
//! The mockup contains additional settings and categories. A control is
//! exposed here only when an existing RSpice subsystem consumes it; storage
//! by itself is not treated as implementation.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, select};
use crate::ui::{Density, EngineeringCanvasTheme, Mode};
use crate::workbench::{ChoicePreference, TogglePreference};

use super::preferences_shell::{
    PreferenceCategory, page_heading, preference_switch, right_aligned, scope_strip, section_label,
    segmented, setting_row,
};
use super::{AppState, PreferencePageActions};

pub(super) fn render(
    ui: &mut Ui,
    category: PreferenceCategory,
    state: &mut AppState,
    actions: &mut PreferencePageActions,
) {
    match category {
        PreferenceCategory::Appearance => appearance(ui, state),
        PreferenceCategory::Workspace => workspace(ui, state, actions),
        PreferenceCategory::Files => files(ui, state),
        PreferenceCategory::Accessibility => accessibility(ui, state),
        PreferenceCategory::Shortcuts => {
            super::shortcut_preferences::render_page(ui, state, actions);
        }
    }
}

fn action_row(
    ui: &mut Ui,
    title: &'static str,
    detail: &'static str,
    action_id: &'static str,
    label: &'static str,
) -> bool {
    let mut clicked = false;
    setting_row(ui, title, detail, |ui| {
        right_aligned(ui, |ui| {
            clicked = ui
                .push_id(action_id, |ui| Button::new(label).show(ui).clicked())
                .inner;
        });
    });
    clicked
}

fn choice_row(
    ui: &mut Ui,
    state: &mut AppState,
    key: ChoicePreference,
    title: &'static str,
    detail: &'static str,
    options: &'static [&'static str],
) {
    setting_row(ui, title, detail, |ui| {
        right_aligned(ui, |ui| {
            let active = state.ui.preferences.choice(key);
            let labels = options
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            let id = format!("preferences.choice.{key:?}");
            if let Some(selected) = select(
                ui,
                &id,
                title,
                options[active],
                &labels,
                ui.available_width().min(390.0),
            ) {
                state
                    .ui
                    .preferences
                    .set_choice(key, selected)
                    .expect("page descriptors constrain every preference choice");
            }
        });
    });
}

fn appearance(ui: &mut Ui, state: &mut AppState) {
    scope_strip(
        ui,
        "User profile",
        "saved in this application profile on the current device",
    );
    page_heading(
        ui,
        "Appearance",
        "Applied to this application profile. Touch target sizing follows the active platform.",
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
                let response = ui.add_sized(
                    [
                        ui.available_width().min(150.0),
                        Tokens::get(ui.ctx()).metrics.ctl_h,
                    ],
                    egui::Slider::new(&mut state.ui.theme.canvas_contrast, 0..=100)
                        .show_value(false),
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Slider, true, "Canvas contrast")
                });
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
                if let Some(selected) = select(
                    ui,
                    "preferences.appearance.canvas-theme",
                    "Engineering canvas theme",
                    &labels[active],
                    &labels,
                    ui.available_width().min(360.0),
                ) {
                    state.ui.theme.canvas_theme = themes[selected];
                }
            });
        },
    );
}

fn workspace(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    scope_strip(
        ui,
        "Device local",
        "dock geometry and restored documents stay on this device",
    );
    section_label(ui, "Product scope");
    setting_row(
        ui,
        "Engineering profile",
        super::ENGINEERING_PROFILE_HELP,
        |ui| {
            let profiles = crate::workbench::EngineeringProfile::ALL;
            let labels = profiles
                .iter()
                .map(|profile| profile.label().to_owned())
                .collect::<Vec<_>>();
            let active = profiles
                .iter()
                .position(|profile| *profile == state.workbench.engineering_profile)
                .unwrap_or_default();
            if let Some(selected) = select(
                ui,
                "preferences.workspace.engineering-profile",
                "Engineering profile",
                profiles[active].label(),
                &labels,
                ui.available_width().min(440.0),
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
                super::engineering_profile_detail(state.workbench.engineering_profile),
            );
        },
    );
    if action_row(
        ui,
        "Feature ownership and availability",
        super::CAPABILITY_MATRIX_HELP,
        "feature-availability",
        "Review matrix…",
    ) {
        actions.open_capability_matrix = true;
    }
    page_heading(
        ui,
        "Workspace layout",
        "Saved per device while project documents remain portable.",
    );
}

fn files(ui: &mut Ui, state: &mut AppState) {
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
            let normalized = super::normalize_autosave_minutes(state.ui.autosave_minutes);
            let active = values
                .iter()
                .position(|minutes| *minutes == normalized)
                .unwrap_or_default();
            right_aligned(ui, |ui| {
                if let Some(selected) = select(
                    ui,
                    "preferences.files.autosave",
                    "Autosave interval",
                    &labels[active],
                    &labels,
                    ui.available_width().min(360.0),
                ) {
                    state.ui.autosave_minutes = values[selected];
                }
            });
        },
    );
}

fn accessibility(ui: &mut Ui, state: &mut AppState) {
    scope_strip(
        ui,
        "User profile",
        "saved on this device; capability may increase minimum touch targets",
    );
    page_heading(
        ui,
        "Accessibility",
        "Keyboard, screen-reader, vision and motor-access settings apply across every workspace.",
    );
    choice_row(
        ui,
        state,
        ChoicePreference::InterfaceScale,
        "Interface scale",
        "Independent of plot and schematic zoom.",
        &["100% · system default", "110%", "125%", "150%"],
    );
    setting_row(
        ui,
        "Reduced motion",
        "Disables non-essential transitions and animated progress.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut value = state.ui.preferences.toggle(TogglePreference::ReducedMotion);
                if preference_switch(
                    ui,
                    "preferences.accessibility.reduced-motion",
                    "Reduce interface motion",
                    &mut value,
                ) {
                    state
                        .ui
                        .preferences
                        .set_toggle(TogglePreference::ReducedMotion, value);
                }
            });
        },
    );
    choice_row(
        ui,
        state,
        ChoicePreference::MinimumTouchTarget,
        "Minimum touch target",
        "Applied automatically on coarse pointers.",
        &["44 px · WCAG recommended", "48 px"],
    );
}

fn setting_detail(ui: &mut Ui, text: &str) {
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
