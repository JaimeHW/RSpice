//! Runtime-backed Preferences page content from the approved workbench mockup.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, select};
use crate::ui::{Density, EngineeringCanvasTheme, Mode};
use crate::workbench::{
    BackgroundTaskAttention, ChoicePreference, ConsoleLaunchBehavior, ScalarPreference,
    TogglePreference, WorkspacePreferences, WorkspacePreset,
};

use super::preferences_shell::{
    PreferenceCategory, actionable_scope_strip, informational_scope_strip, page_heading,
    preference_switch, right_aligned, section_label, segmented, setting_row,
};
use super::{AppState, PreferencePageActions};

pub(super) fn render(
    ui: &mut Ui,
    category: PreferenceCategory,
    state: &mut AppState,
    actions: &mut PreferencePageActions,
) {
    match category {
        PreferenceCategory::Appearance => appearance(ui, state, actions),
        PreferenceCategory::Workspace => workspace(ui, state, actions),
        PreferenceCategory::Units => units(ui, state, actions),
        PreferenceCategory::Schematic => schematic(ui, state, actions),
        PreferenceCategory::Simulation => simulation(ui, state, actions),
        PreferenceCategory::Results => results(ui, state, actions),
        PreferenceCategory::Files => files(ui, state, actions),
        PreferenceCategory::Compute => compute(ui, state, actions),
        PreferenceCategory::Security => security(ui, state, actions),
        PreferenceCategory::Accessibility => accessibility(ui, state, actions),
        PreferenceCategory::Shortcuts => {
            super::shortcut_preferences::render_page(ui, state, actions);
        }
        PreferenceCategory::Integrations => integrations(ui, state, actions),
    }
}

fn resolved_scope_strip(
    ui: &mut Ui,
    actions: &mut PreferencePageActions,
    scope: &str,
    detail: &str,
) {
    if actionable_scope_strip(ui, scope, detail, "View resolved policy\u{2026}") {
        actions.open_resolved_preference_policy = true;
    }
}

fn toggle_row(
    ui: &mut Ui,
    state: &mut AppState,
    key: TogglePreference,
    title: &'static str,
    detail: &'static str,
    accessible_label: &'static str,
) {
    setting_row(ui, title, detail, |ui| {
        right_aligned(ui, |ui| {
            let mut value = state.ui.preferences.toggle(key);
            if preference_switch(ui, key.stable_id(), accessible_label, &mut value) {
                state.ui.preferences.set_toggle(key, value);
            }
        });
    });
}

fn scalar_row(
    ui: &mut Ui,
    state: &mut AppState,
    key: ScalarPreference,
    title: &'static str,
    detail: &'static str,
    range: std::ops::RangeInclusive<u32>,
) {
    setting_row(ui, title, detail, |ui| {
        right_aligned(ui, |ui| {
            let mut value = state.ui.preferences.scalar(key);
            let response = ui.add_sized(
                [
                    ui.available_width().min(360.0),
                    Tokens::get(ui.ctx()).metrics.ctl_h,
                ],
                egui::DragValue::new(&mut value).range(range).speed(1.0),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::DragValue, true, title)
            });
            if response.changed() {
                state
                    .ui
                    .preferences
                    .set_scalar(key, value)
                    .expect("numeric control constrains the complete preference domain");
            }
        });
    });
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
                ui.available_width().min(360.0),
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

fn appearance(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User profile",
        "portable across desktop and web",
    );
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
    resolved_scope_strip(
        ui,
        actions,
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
    if state.ui.preferences.workspace().is_none() {
        return;
    }

    section_label(ui, "Dock composition");
    setting_row(
        ui,
        "Workspace preset",
        "Engineering keeps contextual panes; Canvas focuses the active editor; Diagnostics opens the console.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = state
                    .ui
                    .preferences
                    .workspace()
                    .map(WorkspacePreferences::preset)
                    .unwrap_or_default()
                    .index();
                if segmented(
                    ui,
                    "preferences.workspace.preset",
                    &["Engineering", "Canvas", "Diagnostics"],
                    &mut selected,
                ) {
                    let preset = WorkspacePreset::from_index(selected)
                        .expect("workspace preset segments define the complete domain");
                    state
                        .ui
                        .preferences
                        .workspace_mut()
                        .expect("the compatible Workspace domain was checked above")
                        .set_preset(preset);
                    state.workbench.apply_workspace_preset(preset);
                }
            });
        },
    );
    setting_row(
        ui,
        "Console on launch",
        "Errors and explicit Problem navigation always open it.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = state
                    .ui
                    .preferences
                    .workspace()
                    .map(WorkspacePreferences::console_on_launch)
                    .unwrap_or_default()
                    .index();
                if segmented(
                    ui,
                    "preferences.workspace.console-on-launch",
                    &["Collapsed", "Open"],
                    &mut selected,
                ) {
                    let behavior = ConsoleLaunchBehavior::from_index(selected)
                        .expect("console launch segments define the complete domain");
                    state
                        .ui
                        .preferences
                        .workspace_mut()
                        .expect("the compatible Workspace domain was checked above")
                        .set_console_on_launch(behavior);
                    state
                        .workbench
                        .apply_console_launch_behavior(behavior.is_open());
                }
            });
        },
    );
    setting_row(
        ui,
        "Dock sizes",
        "Independent navigator, inspector and console sizes per workspace.",
        |ui| {
            right_aligned(ui, |ui| {
                let layout = state.workbench.workspace_layout(state.workbench.workspace);
                ui.vertical(|ui| {
                    ui.label(format!(
                        "navigator {:.0} px · inspector {:.0} px",
                        layout.navigator_width, layout.inspector_width
                    ));
                    ui.label(format!("console {:.0} px · saved", layout.console_height));
                });
            });
        },
    );

    section_label(ui, "Attention and navigation");
    if action_row(
        ui,
        "Remember per-workspace layouts",
        "Design, Simulation, Results and DRC retain independent docks and console state.",
        "workspace-layouts",
        "Manage layouts…",
    ) {
        actions.open_workspace_layout_manager = true;
    }

    section_label(ui, "Startup and windows");
    setting_row(
        ui,
        "Background task attention",
        "Bring the application forward only for explicit user-blocking decisions.",
        |ui| {
            right_aligned(ui, |ui| {
                let options = [
                    "Badge and notify; never steal focus",
                    "Notify on failure only",
                    "Silent",
                ];
                let option_labels = options.map(str::to_owned);
                let active = state
                    .ui
                    .preferences
                    .workspace()
                    .map(WorkspacePreferences::background_task_attention)
                    .unwrap_or_default()
                    .index();
                if let Some(selected) = select(
                    ui,
                    "preferences.workspace.background-task-attention",
                    "Background task attention",
                    options[active],
                    &option_labels,
                    ui.available_width().min(360.0),
                ) {
                    state
                        .ui
                        .preferences
                        .workspace_mut()
                        .expect("the compatible Workspace domain was checked above")
                        .set_background_task_attention(
                            BackgroundTaskAttention::from_index(selected)
                                .expect("background-attention options define the complete domain"),
                        );
                }
            });
        },
    );
}

fn units(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User profile",
        "display-only; projects retain explicit stored units",
    );
    page_heading(
        ui,
        "Units and engineering notation",
        "Parsing is strict and unit-safe; display choices never change stored values.",
    );
    setting_row(
        ui,
        "Unit system",
        "Mixed engineering is recommended for circuit design.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = state.ui.preferences.choice(ChoicePreference::UnitSystem);
                if segmented(
                    ui,
                    "preferences.units.system",
                    &["Mixed", "SI", "Imperial layout"],
                    &mut selected,
                ) {
                    state
                        .ui
                        .preferences
                        .set_choice(ChoicePreference::UnitSystem, selected)
                        .expect("unit-system segments define the complete domain");
                }
            });
        },
    );
    choice_row(
        ui,
        state,
        ChoicePreference::EngineeringSuffixes,
        "Engineering suffixes",
        "Case-sensitive SPICE parsing with unambiguous meg and mil handling.",
        &["Strict RSpice · 10Meg, 10m", "Classic SPICE compatibility"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::FrequencyDisplay,
        "Frequency display",
        "Stored internally as hertz.",
        &["Hz · engineering prefixes", "rad/s"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::TemperatureDisplay,
        "Temperature display",
        "Display conversion is applied only after the stored kelvin value is resolved.",
        &["°C", "K", "°F"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::CopiedValueFormat,
        "Copied values",
        "Copy retains full precision plus an explicit unit.",
        &[
            "Engineering notation + unit",
            "Scientific notation + SI unit",
        ],
    );
    section_label(ui, "Locale and reporting");
    setting_row(
        ui,
        "Locale, date, time and numeric format",
        "Display and reporting preferences never change parsed or stored values.",
        |ui| {
            right_aligned(ui, |ui| {
                ui.label("System locale · portable engineering files");
            });
        },
    );
    section_label(ui, "Angles, coordinates and input");
    choice_row(
        ui,
        state,
        ChoicePreference::AngleDisplay,
        "Angle display",
        "Expression functions continue to require an explicit unit where ambiguous.",
        &["Degrees", "Radians"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::LayoutCoordinateDisplay,
        "Layout coordinate display",
        "The PDK database unit remains authoritative and unchanged.",
        &["µm with database-unit remainder", "nm", "Database units"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::TimeFrequencyInput,
        "Time and frequency input",
        "Reject ambiguous suffixes and show the normalized value before commit.",
        &["Strict units required", "Infer from field quantity"],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::DecimalSeparatorInput,
        "Decimal separator on input",
        "Netlists and automation files always use the portable period separator.",
        &["Locale-aware UI · portable files", "Period everywhere"],
    );
}

fn schematic(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User default",
        "each project may declare stricter design rules",
    );
    page_heading(
        ui,
        "Schematic editor",
        "Interaction, units, hierarchy and connectivity behavior.",
    );
    setting_row(
        ui,
        "Grid and snap",
        "Visible grid and placement increments.",
        |ui| {
            right_aligned(ui, |ui| {
                let mut selected = state.ui.preferences.choice(ChoicePreference::SchematicGrid);
                if segmented(
                    ui,
                    "preferences.schematic.grid",
                    &["50 mil", "25 mil", "Metric"],
                    &mut selected,
                ) {
                    state
                        .ui
                        .preferences
                        .set_choice(ChoicePreference::SchematicGrid, selected)
                        .expect("schematic-grid segments define the complete domain");
                }
            });
        },
    );
    choice_row(
        ui,
        state,
        ChoicePreference::OperatingPointAnnotation,
        "Operating-point annotation",
        "Default detail after a completed compatible run.",
        &["Voltages + selected currents", "Voltages only", "Hidden"],
    );
    toggle_row(
        ui,
        state,
        TogglePreference::CrossProbeBehavior,
        "Cross-probe behavior",
        "Highlight the matching net, device, netlist line and trace.",
        "Enable cross-probe behavior",
    );
    toggle_row(
        ui,
        state,
        TogglePreference::IncrementalConnectivityChecks,
        "Connectivity checks",
        "Refresh incrementally while editing.",
        "Enable incremental connectivity checks",
    );
    section_label(ui, "Editing and hierarchy");
    choice_row(
        ui,
        state,
        ChoicePreference::WireJunctionBehavior,
        "Wire and junction behavior",
        "Orthogonal routing previews connectivity before a wire is committed.",
        &[
            "Orthogonal · automatic explicit junctions",
            "Orthogonal · manual junctions",
            "Any-angle routing",
        ],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::SelectionCrossingPolicy,
        "Selection crossing policy",
        "Left-to-right encloses; right-to-left intersects, matching physical editors.",
        &[
            "Directional window selection",
            "Enclosed objects only",
            "Intersecting objects",
        ],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::NetNamingPolicy,
        "Net naming policy",
        "Case, global-net syntax, generated names and bus expansion are checked before netlisting.",
        &[
            "Strict project policy · case sensitive",
            "SPICE-compatible relaxed",
        ],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::PropertyCommitPolicy,
        "Property commit",
        "Validate typed values and dependencies before a multi-object edit is applied.",
        &[
            "Atomic · reject the complete invalid edit",
            "Apply valid fields and report failures",
        ],
    );
}

fn simulation(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User default + plan",
        "the retained preset initializes new projects; each plan then owns its exact numerical options",
    );
    page_heading(
        ui,
        "Simulation defaults",
        "Execution targets, numerical policy and failure handling.",
    );
    choice_row(
        ui,
        state,
        ChoicePreference::DefaultSolverPreset,
        "Default solver preset",
        "Individual plans may override this value.",
        &["Balanced", "Fast", "Accurate", "Robust"],
    );
}

fn results(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User default",
        "plot documents may override display behavior",
    );
    page_heading(
        ui,
        "Results and precision",
        "Plot behavior never changes stored engineering precision. Trace accessibility is owned by Appearance.",
    );
    scalar_row(
        ui,
        state,
        ScalarPreference::DisplayedSignificantDigits,
        "Displayed significant digits",
        "Copy retains full stored precision.",
        3..=17,
    );
    choice_row(
        ui,
        state,
        ChoicePreference::CursorInterpolation,
        "Cursor interpolation",
        "Method used between accepted solver points.",
        &[
            "Monotone cubic where valid",
            "Linear",
            "Nearest accepted point",
        ],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::ComplexNumberDisplay,
        "Complex-number display",
        "Viewer-specific axes may override this without changing the dataset.",
        &[
            "Magnitude / phase · degrees",
            "Real / imaginary",
            "Magnitude / phase · radians",
        ],
    );
    section_label(ui, "Axes, data and evaluation");
    choice_row(
        ui,
        state,
        ChoicePreference::LargeDatasetDisplay,
        "Large-dataset display",
        "Visual decimation preserves extrema, crossings and cursor source samples.",
        &[
            "Envelope + extrema-preserving decimation",
            "Uniform display sampling",
            "No display decimation",
        ],
    );
    choice_row(
        ui,
        state,
        ChoicePreference::EngineeringExport,
        "Default engineering export",
        "Exports retain full stored precision; Touchstone is offered only for compatible S-parameter datasets.",
        &["CSV", "Touchstone where compatible"],
    );
}

fn files(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
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

fn compute(ui: &mut Ui, _state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "Runtime capability",
        "this build has no registered remote execution targets",
    );
    page_heading(
        ui,
        "Remote compute",
        "Remote target and scheduling controls are shown only when backed by a configured execution service.",
    );
    setting_row(
        ui,
        "Local processor availability",
        "Host-reported hardware concurrency; this is capability information, not a remote scheduling policy.",
        |ui| {
            right_aligned(ui, |ui| {
                let threads =
                    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
                ui.label(format!("{threads} logical threads detected"));
            });
        },
    );
}

fn security(ui: &mut Ui, _state: &mut AppState, _actions: &mut PreferencePageActions) {
    informational_scope_strip(
        ui,
        "Runtime capability",
        "no credential vault, certificate store, proxy provider, or crash-report transport is registered",
    );
    page_heading(
        ui,
        "Security and privacy",
        "Configurable security controls are shown only when an enforcing runtime service owns them.",
    );
}

fn integrations(ui: &mut Ui, _state: &mut AppState, _actions: &mut PreferencePageActions) {
    informational_scope_strip(
        ui,
        "Runtime capability",
        "no integration registry, signed-extension loader, Python IPC host, or webhook delivery service is registered",
    );
    page_heading(
        ui,
        "Plug-ins and integrations",
        "Configurable integration controls are shown only when a provider supplies executable actions and retained state.",
    );
}

fn accessibility(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    resolved_scope_strip(
        ui,
        actions,
        "User profile",
        "saved on this device; capability may increase minimum touch targets",
    );
    page_heading(
        ui,
        "Accessibility",
        "Keyboard, screen-reader, vision and motor-access settings apply across every workspace.",
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

#[cfg(test)]
mod tests {
    #[test]
    fn pages_do_not_expose_persistence_only_or_fabricated_controls() {
        let source = include_str!("preference_pages.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests");
        for unsupported in [
            "ScalarPreference::LocalParallelSlots",
            "ChoicePreference::DefaultComputeTarget",
            "ScalarPreference::ParallelTaskCeiling",
            "ChoicePreference::CertificateTrust",
            "TogglePreference::AutomaticDiagnostics",
            "ChoicePreference::ExtensionPolicy",
            "TogglePreference::PythonAutomationApi",
            "ChoicePreference::InterfaceScale",
        ] {
            assert!(
                !production.contains(unsupported),
                "persistence-only control leaked into Preferences: {unsupported}"
            );
        }
        for fabricated in [
            "lab-hpc-west",
            "Operating-system protected vault",
            "United States · organization managed",
            "0 installed extensions",
            "Built-in sandbox only",
        ] {
            assert!(
                !production.contains(fabricated),
                "fabricated runtime fixture leaked into Preferences: {fabricated}"
            );
        }
    }

    #[test]
    fn simulation_page_exposes_only_the_runtime_consumed_mockup_default() {
        let source = include_str!("preference_pages.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests");
        let simulation = production
            .split("fn simulation(")
            .nth(1)
            .and_then(|source| source.split("fn results(").next())
            .expect("Simulation page source");

        assert!(simulation.contains("Default solver preset"));
        assert!(simulation.contains("ChoicePreference::DefaultSolverPreset"));
        for missing_backend in [
            "Local parallel slots",
            "Convergence failure",
            "Checkpoint policy",
        ] {
            assert!(
                !simulation.contains(missing_backend),
                "unsupported Simulation row was exposed: {missing_backend}"
            );
        }
    }

    #[test]
    fn unavailable_provider_pages_do_not_offer_empty_policy_actions() {
        let source = include_str!("preference_pages.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests");
        let security = production
            .split("fn security(")
            .nth(1)
            .and_then(|source| source.split("fn integrations(").next())
            .expect("Security page source");
        let integrations = production
            .split("fn integrations(")
            .nth(1)
            .and_then(|source| source.split("fn accessibility(").next())
            .expect("Integrations page source");

        assert!(security.contains("informational_scope_strip"));
        assert!(security.contains("credential vault"));
        assert!(!security.contains("resolved_scope_strip"));
        assert!(integrations.contains("informational_scope_strip"));
        assert!(integrations.contains("integration registry"));
        assert!(!integrations.contains("resolved_scope_strip"));
    }

    #[test]
    fn workspace_rows_follow_the_mockup_order_and_hide_missing_backends() {
        let source = include_str!("preference_pages.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests");
        let workspace = production
            .split("fn workspace(")
            .nth(1)
            .and_then(|source| source.split("fn units(").next())
            .expect("Workspace page source");

        let mut cursor = 0;
        for row in [
            "Product scope",
            "Engineering profile",
            "Feature ownership and availability",
            "Workspace layout",
            "Dock composition",
            "Workspace preset",
            "Console on launch",
            "Dock sizes",
            "Attention and navigation",
            "Remember per-workspace layouts",
            "Startup and windows",
            "Background task attention",
        ] {
            let offset = workspace[cursor..]
                .find(row)
                .unwrap_or_else(|| panic!("missing Workspace mockup row: {row}"));
            cursor += offset + row.len();
        }

        for missing_backend in [
            "Restore workspace documents",
            "Portable preferences",
            "Notifications and job alerts",
            "Startup destination",
            "Multi-window documents",
            "Per-monitor placement",
            "Phone window model",
            "Document tabs and session",
        ] {
            assert!(
                !workspace.contains(missing_backend),
                "unsupported Workspace row was exposed: {missing_backend}"
            );
        }
    }
}
