//! The settings each Preferences page holds.
//!
//! Every row here reaches a preference the running application reads. A row
//! whose value nothing consumes is a control that does nothing, which is worse
//! than an absent one: it tells a reader the behaviour is configurable and
//! then does not configure it. Where this file is thinner than the preference
//! store is wide, that is why — the store keeps keys for domains that have no
//! runtime owner yet, and they stay off the page until one exists.

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
    PreferenceCategory, control_width, preference_row, preference_switch, section_heading,
    segmented, value_column,
};
use super::{AppState, PreferencePageActions};

/// Width of a numeric field, which is sized to the digits it holds rather
/// than to the control column the selects share.
const NUMERIC_FIELD_WIDTH: f32 = 88.0;

pub(super) fn render(
    ui: &mut Ui,
    category: PreferenceCategory,
    state: &mut AppState,
    actions: &mut PreferencePageActions,
) {
    match category {
        PreferenceCategory::Appearance => appearance(ui, state),
        PreferenceCategory::Accessibility => accessibility(ui, state),
        PreferenceCategory::Workspace => workspace(ui, state, actions),
        PreferenceCategory::Shortcuts => {
            super::shortcut_preferences::render_page(ui, state, actions);
        }
        PreferenceCategory::Units => units(ui, state),
        PreferenceCategory::Schematic => schematic(ui, state),
        PreferenceCategory::Projects => projects(ui, state, actions),
        PreferenceCategory::Results => results(ui, state),
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
    preference_row(ui, title, detail, |ui| {
        value_column(ui, |ui| {
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
    preference_row(ui, title, detail, |ui| {
        value_column(ui, |ui| {
            let mut value = state.ui.preferences.scalar(key);
            // A field holding at most two digits, at the width two digits
            // need. Stretched to the column a select uses, it reads as an
            // empty text box with a stray number floating in the middle.
            let response = ui.add_sized(
                [
                    NUMERIC_FIELD_WIDTH.min(control_width(ui)),
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

/// The ceiling on concurrent solver workers for one run.
///
/// Its own row rather than a `scalar_row` for two reasons: the help line has
/// to state how many cores this machine actually has, and zero is not a
/// quantity here but the word "All", so the field prints it rather than the
/// digit. The engine narrows whatever is asked for to the parallelism the
/// host reports, which is why a value above the detected count is accepted
/// instead of refused — a session carried to a smaller machine stays valid.
fn solver_threads_row(ui: &mut Ui, state: &mut AppState) {
    let detected = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let help = format!(
        "Concurrent workers for one run. {detected} logical core{} on this machine.",
        if detected == 1 { "" } else { "s" }
    );
    preference_row(ui, "Solver threads", &help, |ui| {
        value_column(ui, |ui| {
            let mut value = state
                .ui
                .preferences
                .scalar(ScalarPreference::LocalParallelSlots);
            let response = ui.add_sized(
                [
                    NUMERIC_FIELD_WIDTH.min(control_width(ui)),
                    Tokens::get(ui.ctx()).metrics.ctl_h,
                ],
                egui::DragValue::new(&mut value)
                    .range(0..=u32::try_from(detected).unwrap_or(u32::MAX))
                    .speed(1.0)
                    .custom_formatter(|value, _| {
                        if value < 1.0 {
                            "All".to_owned()
                        } else {
                            format!("{value:.0}")
                        }
                    }),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::DragValue, true, "Solver threads")
            });
            if response.changed() {
                state
                    .ui
                    .preferences
                    .set_scalar(ScalarPreference::LocalParallelSlots, value)
                    .expect("the detected core count is inside the preference domain");
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
    preference_row(ui, title, detail, |ui| {
        value_column(ui, |ui| {
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
    preference_row(ui, title, detail, |ui| {
        value_column(ui, |ui| {
            let active = state.ui.preferences.choice(key);
            let labels = options
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            let id = format!("preferences.choice.{key:?}");
            if let Some(selected) =
                select(ui, &id, title, options[active], &labels, control_width(ui))
            {
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
    preference_row(ui, "Color mode", "", |ui| {
        value_column(ui, |ui| {
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
    });
    preference_row(ui, "Density", "", |ui| {
        value_column(ui, |ui| {
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
    });
    // The zoom factor the shell applies on every frame. It has been a stored
    // preference with no control since the dialog was written, so the only way
    // to change it was to edit the session file.
    choice_row(
        ui,
        state,
        ChoicePreference::InterfaceScale,
        "Interface scale",
        "",
        &["100%", "110%", "125%", "150%"],
    );
    preference_row(
        ui,
        "Engineering canvas theme",
        "Schematic, layout, waveform and field-view backgrounds, as one palette.",
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
            value_column(ui, |ui| {
                if let Some(selected) = select(
                    ui,
                    "preferences.appearance.canvas-theme",
                    "Engineering canvas theme",
                    &labels[active],
                    &labels,
                    control_width(ui),
                ) {
                    state.ui.theme.canvas_theme = themes[selected];
                }
            });
        },
    );
    preference_row(
        ui,
        "Canvas contrast",
        "Separation between symbols, wires and the grid.",
        |ui| {
            value_column(ui, |ui| {
                // A slider sizes itself from `spacing.slider_width` and
                // ignores the size `add_sized` asks for, so it has to be told
                // the control column's width rather than given it. Told that,
                // its track ends where the selects above and below it end.
                let width = control_width(ui);
                ui.spacing_mut().slider_width = width;
                let response = ui.add_sized(
                    [width, Tokens::get(ui.ctx()).metrics.ctl_h],
                    egui::Slider::new(&mut state.ui.theme.canvas_contrast, 0..=100)
                        .show_value(false),
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Slider, true, "Canvas contrast")
                });
            });
        },
    );
    preference_row(
        ui,
        "Color-safe traces",
        "Distinguish traces by line style and marker as well as color.",
        |ui| {
            value_column(ui, |ui| {
                preference_switch(
                    ui,
                    "result-document.settingspagecontent.enable-color-safe-traces.425c2b40",
                    "Enable color-safe traces",
                    &mut state.ui.theme.colorblind_traces,
                );
            });
        },
    );
}

fn workspace(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    // No static help on this row: the line under the control says what the
    // selected profile actually covers, which is the same question answered
    // concretely. Two explanations of one setting is one too many.
    preference_row(ui, "Engineering profile", "", |ui| {
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
            control_width(ui),
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
            control_width(ui),
        );
    });
    if state.ui.preferences.workspace().is_none() {
        return;
    }

    section_heading(ui, "Layout");
    preference_row(
        ui,
        "Workspace preset",
        "Which panes a workspace opens with.",
        |ui| {
            value_column(ui, |ui| {
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
                    state.workbench.set_workspace_preset(preset);
                    state.workbench.apply_workspace_preset(preset);
                }
            });
        },
    );
    preference_row(
        ui,
        "Console on launch",
        "Errors and explicit Problem navigation always open it.",
        |ui| {
            value_column(ui, |ui| {
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
    // A "Dock sizes" row used to sit here reading back the navigator,
    // inspector and console widths in pixels. The reader sets those by
    // dragging the splitter and sees the result as they drag; a second,
    // non-interactive account of the same three numbers decides nothing.
    if action_row(
        ui,
        "Per-workspace layouts",
        "Each workspace keeps its own docks and console state.",
        "workspace-layouts",
        "Manage layouts…",
    ) {
        actions.open_workspace_layout_manager = true;
    }

    section_heading(ui, "Background work");
    solver_threads_row(ui, state);
    preference_row(
        ui,
        "Task attention",
        "Whether a finished or failed background run may take the foreground.",
        |ui| {
            value_column(ui, |ui| {
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
                    "Task attention",
                    options[active],
                    &option_labels,
                    control_width(ui),
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

fn units(ui: &mut Ui, state: &mut AppState) {
    preference_row(
        ui,
        "Unit system",
        "Display only. A project keeps the units its values were stored in.",
        |ui| {
            value_column(ui, |ui| {
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
        "How values typed into property and analysis fields are read. Netlists always follow SPICE.",
        &[
            "Strict · 10Meg, 10m; M only before a unit",
            "SPICE · not case-sensitive, 10M = 10m",
        ],
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
    // A "Locale, date, time and numeric format" row used to sit here whose
    // whole value column was the static sentence "System locale · portable
    // engineering files". There is no locale setting behind it.
    section_heading(ui, "Angles, coordinates and input");
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

fn schematic(ui: &mut Ui, state: &mut AppState) {
    preference_row(
        ui,
        "Grid and snap",
        "A project may declare a stricter increment.",
        |ui| {
            value_column(ui, |ui| {
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
    preference_row(
        ui,
        "Cursor",
        "A full-screen crosshair follows the mouse over the canvas.",
        |ui| {
            value_column(ui, |ui| {
                let mut selected = state
                    .ui
                    .preferences
                    .choice(ChoicePreference::SchematicCursor);
                if segmented(
                    ui,
                    "preferences.schematic.cursor",
                    &["Tool cursors", "Full-screen crosshair"],
                    &mut selected,
                ) {
                    state
                        .ui
                        .preferences
                        .set_choice(ChoicePreference::SchematicCursor, selected)
                        .expect("schematic-cursor segments define the complete domain");
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
        "Live checking",
        "Check open pins, floating nets, wiring and names as each edit settles.",
        "Enable live checking",
    );
    section_heading(ui, "Editing and hierarchy");
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
    toggle_row(
        ui,
        state,
        TogglePreference::HierarchicalEditInPlace,
        "Edit-in-place hierarchy",
        "Descending opens the child in its occurrence; off opens the master on its own.",
        "Enable hierarchical edit in place",
    );
    choice_row(
        ui,
        state,
        ChoicePreference::NetNamingPolicy,
        "Net name characters",
        "Net names are not case-sensitive: Out and out are one net.",
        &[
            "ASCII letters, digits, _.$:/![]<>-",
            "Any printable character but spaces",
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

fn results(ui: &mut Ui, state: &mut AppState) {
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
    section_heading(ui, "Data and export");
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
        "Exports keep full stored precision. Native bundles reopen in RSpice.",
        &[
            "CSV",
            "Touchstone where compatible",
            "TSV",
            "RSpice Result Bundle",
            "RSpice Dataset Bundle",
            "VCD · event timelines",
            "NumPy .npy · one array, column order stated",
            "NumPy .npz · one named array per signal",
            "HDF5 dataset · one analysis, names and units carried",
            "MATLAB v5 · column vectors, complex for AC",
        ],
    );
}

/// What a new project starts from, and what saving one does.
///
/// Three settings that used to be two pages: Simulation defaults held the
/// solver preset alone under two paragraphs explaining which numerical options
/// it does *not* own, and Files, storage and recovery held the other two under
/// a section heading each. They answer one question between them.
fn projects(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    // One file holding every preference and the shortcut library in use.
    preference_row(
        ui,
        "Settings file",
        "Move your preferences and shortcuts to another machine.",
        |ui| {
            value_column(ui, |ui| {
                let phone = ui.ctx().content_rect().width() <= 560.0;
                let width = ui.available_width();
                let height = if phone {
                    44.0
                } else {
                    Tokens::get(ui.ctx()).metrics.ctl_h
                };
                ui.allocate_ui_with_layout(
                    egui::vec2(width, height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let button = |ui: &mut Ui, id: &'static str, label: &'static str| {
                            ui.push_id(id, |ui| {
                                Button::new(label).min_height(height).show(ui).clicked()
                            })
                            .inner
                        };
                        if button(ui, "preferences.projects.settings-import", "Import\u{2026}") {
                            actions.import_settings_file = true;
                        }
                        if button(ui, "preferences.projects.settings-export", "Export\u{2026}") {
                            actions.export_settings_file = true;
                        }
                        // A browser keeps the file in its own storage, which
                        // no file manager can open.
                        #[cfg(not(target_arch = "wasm32"))]
                        if button(ui, "preferences.projects.settings-show", "Show file") {
                            actions.show_settings_file = true;
                        }
                    },
                );
            });
        },
    );
    choice_row(
        ui,
        state,
        ChoicePreference::DefaultSolverPreset,
        "Default solver preset",
        "The preset a new project starts from. Each plan may override it.",
        &["Balanced", "Fast", "Accurate", "Robust"],
    );
    preference_row(
        ui,
        "Autosave interval",
        "Checkpoints open editable documents.",
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
            value_column(ui, |ui| {
                if let Some(selected) = select(
                    ui,
                    "preferences.projects.autosave",
                    "Autosave interval",
                    &labels[active],
                    &labels,
                    control_width(ui),
                ) {
                    state.ui.autosave_minutes = values[selected];
                }
            });
        },
    );
    toggle_row(
        ui,
        state,
        TogglePreference::RunDesignChecksOnSave,
        "Design checks on save",
        "Findings are reported after the save completes. A save is never refused by them.",
        "Run design checks on save",
    );
}

fn accessibility(ui: &mut Ui, state: &mut AppState) {
    preference_row(
        ui,
        "Reduced motion",
        "Disables non-essential transitions and animated progress.",
        |ui| {
            value_column(ui, |ui| {
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
    preference_row(
        ui,
        "Canvas keyboard navigation",
        "Traverse symbols, nets, markers, traces and annotations without a pointer.",
        |ui| {
            value_column(ui, |ui| {
                let mut value = state
                    .ui
                    .preferences
                    .toggle(TogglePreference::CanvasKeyboardNavigation);
                if preference_switch(
                    ui,
                    "preferences.accessibility.canvas-keyboard-navigation",
                    "Enable canvas keyboard navigation",
                    &mut value,
                ) {
                    state
                        .ui
                        .preferences
                        .set_toggle(TogglePreference::CanvasKeyboardNavigation, value);
                }
            });
        },
    );
    #[cfg(target_arch = "wasm32")]
    preference_row(
        ui,
        "Speak control changes (browser)",
        "Uses the browser speech fallback while a semantic accessibility tree is unavailable.",
        |ui| {
            value_column(ui, |ui| {
                let mut value = state.ui.browser_spoken_feedback;
                if preference_switch(
                    ui,
                    "preferences.accessibility.browser-spoken-feedback",
                    "Speak control changes (browser)",
                    &mut value,
                ) {
                    state.ui.browser_spoken_feedback = value;
                    crate::workbench::browser::accessibility::set_spoken_feedback(value);
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

/// A line derived from the control above it, wrapped to that control's own
/// width so the two share a right edge.
fn setting_detail(ui: &mut Ui, text: &str, width: f32) {
    let t = Tokens::get(ui.ctx());
    ui.scope(|ui| {
        ui.set_max_width(width);
        ui.add(
            egui::Label::new(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
    });
}

/// Put every setting on `category`'s page back to what RSpice ships.
///
/// Page-scoped rather than global: a reader who has made one page unreadable
/// wants that page back, not a dialog that also discards their unit system and
/// their key bindings. The lists below are the settings each page shows, so a
/// row added to a page without being added here is a row the footer's control
/// silently skips — which the test at the bottom of this file checks for.
pub(super) fn restore_defaults(category: PreferenceCategory, state: &mut AppState) {
    let preferences = &mut state.ui.preferences;
    match category {
        PreferenceCategory::Appearance => {
            state.ui.theme = crate::ui::Theme::default();
            preferences.restore_defaults(&[ChoicePreference::InterfaceScale], &[], &[]);
        }
        PreferenceCategory::Accessibility => preferences.restore_defaults(
            &[ChoicePreference::MinimumTouchTarget],
            &[
                TogglePreference::ReducedMotion,
                TogglePreference::CanvasKeyboardNavigation,
            ],
            &[],
        ),
        PreferenceCategory::Workspace => {
            state.workbench.engineering_profile = crate::workbench::EngineeringProfile::default();
            preferences.restore_defaults(&[], &[], &[ScalarPreference::LocalParallelSlots]);
            if let Ok(workspace) = preferences.workspace_mut() {
                *workspace = WorkspacePreferences::default();
                let preset = workspace.preset();
                let console = workspace.console_on_launch();
                state.workbench.set_workspace_preset(preset);
                state.workbench.apply_workspace_preset(preset);
                state
                    .workbench
                    .apply_console_launch_behavior(console.is_open());
            }
        }
        PreferenceCategory::Shortcuts => {
            // The bindings and the three binding policies are one profile, and
            // replacing it has to go through the persist-before-live owner that
            // the editor and the policy rows already use.
        }
        PreferenceCategory::Units => preferences.restore_defaults(
            &[
                ChoicePreference::UnitSystem,
                ChoicePreference::EngineeringSuffixes,
                ChoicePreference::FrequencyDisplay,
                ChoicePreference::TemperatureDisplay,
                ChoicePreference::CopiedValueFormat,
                ChoicePreference::AngleDisplay,
                ChoicePreference::LayoutCoordinateDisplay,
                ChoicePreference::TimeFrequencyInput,
                ChoicePreference::DecimalSeparatorInput,
            ],
            &[],
            &[],
        ),
        PreferenceCategory::Schematic => preferences.restore_defaults(
            &[
                ChoicePreference::SchematicGrid,
                ChoicePreference::SchematicCursor,
                ChoicePreference::OperatingPointAnnotation,
                ChoicePreference::WireJunctionBehavior,
                ChoicePreference::SelectionCrossingPolicy,
                ChoicePreference::NetNamingPolicy,
                ChoicePreference::PropertyCommitPolicy,
            ],
            &[
                TogglePreference::CrossProbeBehavior,
                TogglePreference::IncrementalConnectivityChecks,
                TogglePreference::HierarchicalEditInPlace,
            ],
            &[],
        ),
        PreferenceCategory::Projects => {
            state.ui.autosave_minutes = super::DEFAULT_AUTOSAVE_MINUTES;
            preferences.restore_defaults(
                &[ChoicePreference::DefaultSolverPreset],
                &[TogglePreference::RunDesignChecksOnSave],
                &[],
            );
        }
        PreferenceCategory::Results => preferences.restore_defaults(
            &[
                ChoicePreference::CursorInterpolation,
                ChoicePreference::ComplexNumberDisplay,
                ChoicePreference::LargeDatasetDisplay,
                ChoicePreference::EngineeringExport,
            ],
            &[],
            &[ScalarPreference::DisplayedSignificantDigits],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;

    /// The page source, with this test module cut off the end.
    fn production_source() -> &'static str {
        include_str!("preference_pages.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests")
    }

    /// The same source with whole-line comments dropped.
    ///
    /// The scans below look for things the pages must not *do*, and several
    /// of this file's comments name a deleted row in order to say why it is
    /// gone. Reading the prose as if it were code makes an explanation of a
    /// fix look exactly like the defect it explains.
    fn production_code() -> String {
        production_source()
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The pages, and the reset that has to cover them, as two source halves.
    fn pages_and_restore() -> (String, String) {
        let code = production_code();
        let (pages, restore) = code
            .split_once("pub(super) fn restore_defaults(")
            .expect("restore_defaults follows the pages");
        (pages.to_owned(), restore.to_owned())
    }

    /// Every preference key named in `source`.
    fn named_keys(source: &str) -> Vec<String> {
        let mut keys = Vec::new();
        for enum_name in [
            "ChoicePreference::",
            "TogglePreference::",
            "ScalarPreference::",
        ] {
            for fragment in source.split(enum_name).skip(1) {
                let variant: String = fragment
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect();
                if !variant.is_empty() {
                    keys.push(format!("{enum_name}{variant}"));
                }
            }
        }
        keys
    }

    /// A control is a promise that the application reads the value behind it.
    ///
    /// These keys are in the preference store because an earlier
    /// implementation wrote them, and nothing in the running application reads
    /// any of them: this build has no remote execution service, no credential
    /// store and no extension loader, and the solver options named here are
    /// owned by each plan rather than by a profile default. Putting one back
    /// on a page means first giving it a consumer.
    #[test]
    fn pages_expose_only_preferences_the_application_reads() {
        let production = production_code();
        for unsupported in [
            "ChoicePreference::ConvergenceFailurePolicy",
            "ChoicePreference::CheckpointPolicy",
            "ChoicePreference::EngineSelection",
            "ChoicePreference::RandomSeedPolicy",
            "ChoicePreference::IncrementalResultReuse",
            "ChoicePreference::LicenseWaitBehavior",
            "ChoicePreference::LiveResultStreaming",
            "ChoicePreference::DiagnosticArtifact",
            "ChoicePreference::FamilyTraceLabeling",
            "ChoicePreference::DefaultAxisPolicy",
            "ChoicePreference::MeasurementEvaluation",
            "ChoicePreference::DefaultComputeTarget",
            "ChoicePreference::QueuePriority",
            "ChoicePreference::RuntimeEnvironment",
            "ChoicePreference::TransferPolicy",
            "ChoicePreference::PreemptionTimeout",
            "ChoicePreference::UnavailableTarget",
            "ChoicePreference::CertificateTrust",
            "ChoicePreference::NetworkProxy",
            "ChoicePreference::LocalProjectEncryption",
            "ChoicePreference::AutomationSandbox",
            "ChoicePreference::DataResidency",
            "ChoicePreference::ExtensionPolicy",
            "ChoicePreference::ExtensionUpdates",
            "TogglePreference::RestorePlotDocuments",
            "TogglePreference::SynchronizeResultFamilyCrossProbes",
            "TogglePreference::AutomaticDiagnostics",
            "TogglePreference::PythonAutomationApi",
            "ScalarPreference::ParallelTaskCeiling",
        ] {
            assert!(
                !production.contains(unsupported),
                "a preference with no runtime consumer reached a page: {unsupported}"
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

    /// The interface scale is the counter-example to the test above: the shell
    /// applies it to the context's zoom factor on every frame, and until it
    /// reached Appearance the only way to change it was to edit the stored
    /// session by hand.
    #[test]
    fn the_zoom_factor_the_shell_applies_is_reachable_from_appearance() {
        let appearance = production_source()
            .split("fn appearance(")
            .nth(1)
            .and_then(|source| source.split("\nfn ").next())
            .expect("Appearance page source");
        assert!(appearance.contains("ChoicePreference::InterfaceScale"));
        assert!(
            (crate::workbench::UserPreferences::default().interface_scale() - 1.0).abs()
                < f32::EPSILON,
            "the first option on the row has to be the scale RSpice ships"
        );
    }

    /// The solver's worker ceiling is the other counter-example: the engine
    /// has always taken one, the command line has always exposed it as
    /// `--jobs`, and the desktop had no control for it at all. The stored key
    /// existed and was *printed* in the Automation preview, which is worse
    /// than absent — a number shown to a reader that nothing honoured.
    #[test]
    fn the_worker_ceiling_the_engine_takes_is_reachable_from_workspace() {
        let workspace = production_source()
            .split("fn solver_threads_row(")
            .nth(1)
            .and_then(|source| source.split("\nfn ").next())
            .expect("solver threads row source");
        assert!(workspace.contains("ScalarPreference::LocalParallelSlots"));
        assert!(
            production_code().contains("solver_threads_row(ui, state)"),
            "the row has to be on a page, not merely defined"
        );
        assert_eq!(
            crate::workbench::UserPreferences::default().solver_worker_limit(),
            None,
            "shipped default leaves the engine's own ceiling in place"
        );
    }

    /// Zero on that row is the word "All", not the number none.
    ///
    /// A ceiling of zero workers would refuse to solve anything, so the
    /// sentinel has to read as what it means wherever it is shown.
    #[test]
    fn the_worker_row_spells_its_sentinel_rather_than_printing_zero() {
        let production = production_code();
        assert!(production.contains("custom_formatter"));
        assert!(production.contains("\"All\".to_owned()"));
    }

    /// Rows the pages used to carry that set nothing.
    ///
    /// Each was a label whose value column could not be operated: two dock
    /// widths read back from a layout the reader sets by dragging, a static
    /// sentence about the system locale, and a count of the host's logical
    /// processors.
    #[test]
    fn pages_carry_no_read_only_readout_dressed_as_a_setting() {
        let production = production_code();
        for readout in [
            "\"Dock sizes\"",
            "navigator {:.0} px",
            "Locale, date, time and numeric format",
            "System locale",
            "Local processor availability",
            "logical threads detected",
        ] {
            assert!(
                !production.contains(readout),
                "a read-only readout is presented as a setting: {readout}"
            );
        }
    }

    /// Restore defaults is only as good as its coverage: a row added to a page
    /// and not to [`restore_defaults`] is a setting the footer's control
    /// silently skips, with nothing on screen to say which.
    #[test]
    fn restore_defaults_covers_every_preference_the_pages_expose() {
        let (pages, restore) = pages_and_restore();
        let restored = named_keys(&restore);
        for key in named_keys(&pages) {
            assert!(
                restored.contains(&key),
                "{key} is on a page but Restore defaults does not reset it"
            );
        }
    }

    /// And the reset has to reach the stored value in every domain the store
    /// keeps a setting in: the plain key maps, the typed Units and Results
    /// domains, the workspace domain, and the two settings that live on the
    /// theme and the session rather than in preferences at all.
    #[test]
    fn restoring_a_page_clears_every_override_behind_it() {
        let shipped = serde_json::to_value(crate::workbench::UserPreferences::default())
            .expect("default preferences serialize");
        for category in PreferenceCategory::ALL {
            let mut app = RSpiceApp::test_instance();
            if !override_every_setting_on(category, &mut app.state) {
                continue;
            }
            assert_ne!(
                serde_json::to_value(&app.state.ui.preferences).expect("preferences serialize"),
                shipped,
                "{} set no stored override to restore",
                category.label()
            );
            restore_defaults(category, &mut app.state);
            assert_eq!(
                serde_json::to_value(&app.state.ui.preferences).expect("preferences serialize"),
                shipped,
                "{} left a stored override behind",
                category.label()
            );
            assert_eq!(
                app.state.ui.theme,
                crate::ui::Theme::default(),
                "{} left the theme changed",
                category.label()
            );
            assert_eq!(
                app.state.ui.autosave_minutes,
                super::super::DEFAULT_AUTOSAVE_MINUTES,
                "{} left the autosave interval changed",
                category.label()
            );
        }
    }

    /// Move every setting shown on `category` off its default. Returns `false`
    /// for a page that stores nothing in `UserPreferences`.
    fn override_every_setting_on(category: PreferenceCategory, state: &mut AppState) -> bool {
        let (_, restore) = pages_and_restore();
        let arm = restore
            .split(&format!("PreferenceCategory::{category:?} =>"))
            .nth(1)
            .and_then(|source| source.split("PreferenceCategory::").next())
            .unwrap_or_default();
        let keys = named_keys(arm);
        let mut stored = false;
        for key in ChoicePreference::ALL {
            if !keys.contains(&format!("ChoicePreference::{key:?}")) {
                continue;
            }
            state
                .ui
                .preferences
                .set_choice(key, 1)
                .expect("every exposed choice has a second option");
            stored = true;
        }
        for key in TogglePreference::ALL {
            if !keys.contains(&format!("TogglePreference::{key:?}")) {
                continue;
            }
            let flipped = !state.ui.preferences.toggle(key);
            state.ui.preferences.set_toggle(key, flipped);
            stored = true;
        }
        if keys.contains(&"ScalarPreference::DisplayedSignificantDigits".to_owned()) {
            state
                .ui
                .preferences
                .set_scalar(ScalarPreference::DisplayedSignificantDigits, 12)
                .expect("twelve significant digits is inside the domain");
            stored = true;
        }
        if category == PreferenceCategory::Appearance {
            state.ui.theme.mode = crate::ui::Mode::Light;
            state.ui.theme.canvas_contrast = 12;
        }
        if category == PreferenceCategory::Projects {
            state.ui.autosave_minutes = 10;
        }
        if category == PreferenceCategory::Workspace
            && let Ok(workspace) = state.ui.preferences.workspace_mut()
        {
            workspace.set_console_on_launch(
                ConsoleLaunchBehavior::from_index(1).expect("the console may launch open"),
            );
            stored = true;
        }
        stored
    }

    /// The hierarchy rows read in the order a reader builds them up: what a
    /// drag selects, then what descending into a child does, then what the
    /// nets it connects may be called.
    #[test]
    fn the_schematic_page_orders_its_hierarchy_rows() {
        let schematic = production_source()
            .split("fn schematic(")
            .nth(1)
            .and_then(|source| source.split("\nfn ").next())
            .expect("Schematic page source");
        let selection = schematic
            .find("\"Selection crossing policy\"")
            .expect("selection policy row");
        let edit_in_place = schematic
            .find("TogglePreference::HierarchicalEditInPlace")
            .expect("hierarchical edit-in-place row");
        let net_naming = schematic
            .find("\"Net name characters\"")
            .expect("net name characters row");
        assert!(selection < edit_in_place && edit_in_place < net_naming);
    }
}
