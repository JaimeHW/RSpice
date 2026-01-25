//! Preferences Dialog
//!
//! Comprehensive settings dialog with tabbed interface for configuring
//! all aspects of RSpice behavior. Follows commercial EDA patterns.

use dioxus::prelude::*;

use super::resizable_dialog::ResizableDialog;
use crate::state::preferences::{GridStyle, InterpolationMode, Preferences, ThemePreference};
use crate::theme::Theme;

//=============================================================================
// Preferences Dialog Component
//=============================================================================

/// Props for the preferences dialog
#[derive(Props, Clone, PartialEq)]
pub struct PreferencesDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Handler for closing the dialog
    pub on_close: EventHandler<()>,
    /// Handler for saving preferences
    pub on_save: EventHandler<Preferences>,
    /// Current preferences
    pub preferences: Preferences,
}

/// Currently selected preferences tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreferencesTab {
    #[default]
    General,
    Schematic,
    Waveform,
    Simulation,
    Files,
}

impl PreferencesTab {
    fn display_name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Schematic => "Schematic",
            Self::Waveform => "Waveform",
            Self::Simulation => "Simulation",
            Self::Files => "Files",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::General => "⚙️",
            Self::Schematic => "📐",
            Self::Waveform => "📈",
            Self::Simulation => "▶️",
            Self::Files => "📁",
        }
    }

    fn all() -> &'static [PreferencesTab] {
        &[
            Self::General,
            Self::Schematic,
            Self::Waveform,
            Self::Simulation,
            Self::Files,
        ]
    }
}

/// Preferences dialog component
#[component]
pub fn PreferencesDialog(props: PreferencesDialogProps) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let mut active_tab = use_signal(|| PreferencesTab::General);
    let mut prefs = use_signal(|| props.preferences.clone());
    let mut has_changes = use_signal(|| false);

    rsx! {
        ResizableDialog {
            title: "Preferences".to_string(),
            is_open: props.visible,
            on_close: move |_| {
                if !has_changes() {
                    props.on_close.call(());
                }
            },
            default_width: 800.0,
            default_height: 600.0,
            min_width: 600.0,
            min_height: 450.0,
            max_width: 1200.0,
            max_height: 900.0,

            // Body with sidebar
            div {
                style: "display: flex; flex: 1; overflow: hidden;",

                // Tab sidebar
                div {
                    style: format!(
                        "
                        width: 180px;
                        background: {};
                        border-right: 1px solid {};
                        padding: 12px;
                        ",
                        theme.read().bg_tertiary(),
                        theme.read().border_subtle()
                    ),

                    for tab in PreferencesTab::all() {
                        button {
                            style: format!(
                                "
                                width: 100%;
                                padding: 10px 12px;
                                margin-bottom: 4px;
                                border: none;
                                border-radius: 6px;
                                background: {};
                                color: {};
                                font-size: 13px;
                                text-align: left;
                                cursor: pointer;
                                display: flex;
                                align-items: center;
                                gap: 8px;
                                transition: all 0.15s ease;
                                ",
                                if active_tab() == *tab {
                                    theme.read().accent_primary()
                                } else {
                                    "transparent"
                                },
                                if active_tab() == *tab {
                                    "white"
                                } else {
                                    theme.read().text_secondary()
                                }
                            ),
                            onclick: {
                                let tab = *tab;
                                move |_| active_tab.set(tab)
                            },
                            span { "{tab.icon()}" }
                            span { "{tab.display_name()}" }
                        }
                    }
                }

                // Tab content
                div {
                    style: "flex: 1; padding: 24px; overflow-y: auto;",

                    match active_tab() {
                        PreferencesTab::General => rsx! {
                            GeneralTab {
                                prefs: prefs(),
                                on_change: move |new_prefs| {
                                    prefs.set(new_prefs);
                                    has_changes.set(true);
                                }
                            }
                        },
                        PreferencesTab::Schematic => rsx! {
                            SchematicTab {
                                prefs: prefs(),
                                on_change: move |new_prefs| {
                                    prefs.set(new_prefs);
                                    has_changes.set(true);
                                }
                            }
                        },
                        PreferencesTab::Waveform => rsx! {
                            WaveformTab {
                                prefs: prefs(),
                                on_change: move |new_prefs| {
                                    prefs.set(new_prefs);
                                    has_changes.set(true);
                                }
                            }
                        },
                        PreferencesTab::Simulation => rsx! {
                            SimulationTab {
                                prefs: prefs(),
                                on_change: move |new_prefs| {
                                    prefs.set(new_prefs);
                                    has_changes.set(true);
                                }
                            }
                        },
                        PreferencesTab::Files => rsx! {
                            FilesTab {
                                prefs: prefs(),
                                on_change: move |new_prefs| {
                                    prefs.set(new_prefs);
                                    has_changes.set(true);
                                }
                            }
                        },
                    }
                }
            }

            // Footer
            div {
                style: format!(
                    "
                    display: flex;
                    justify-content: flex-end;
                    gap: 12px;
                    padding: 16px 24px;
                    border-top: 1px solid {};
                    ",
                    theme.read().border_subtle()
                ),

                button {
                    style: format!(
                        "
                        padding: 10px 24px;
                        background: {};
                        border: none;
                        border-radius: 6px;
                        color: {};
                        font-size: 14px;
                        cursor: pointer;
                        ",
                        theme.read().surface(),
                        theme.read().text_primary()
                    ),
                    onclick: move |_| prefs.set(Preferences::default()),
                    "Reset to Defaults"
                }

                button {
                    style: format!(
                        "
                        padding: 10px 24px;
                        background: {};
                        border: none;
                        border-radius: 6px;
                        color: {};
                        font-size: 14px;
                        cursor: pointer;
                        ",
                        theme.read().surface(),
                        theme.read().text_primary()
                    ),
                    onclick: move |_| props.on_close.call(()),
                    "Cancel"
                }

                button {
                    style: format!(
                        "
                        padding: 10px 24px;
                        background: {};
                        border: none;
                        border-radius: 6px;
                        color: white;
                        font-size: 14px;
                        font-weight: 500;
                        cursor: pointer;
                        ",
                        theme.read().accent_primary()
                    ),
                    onclick: move |_| {
                        props.on_save.call(prefs());
                        has_changes.set(false);
                    },
                    "Save"
                }
            }
        }
    }
}

//=============================================================================
// Tab Components
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct TabProps {
    prefs: Preferences,
    on_change: EventHandler<Preferences>,
}

#[component]
fn GeneralTab(props: TabProps) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let prefs = props.prefs.clone();

    rsx! {
        div {
            SectionTitle { title: "Appearance" }

            PreferenceRow {
                label: "Theme",
                description: "Application color theme",
                SelectInput {
                    value: format!("{:?}", prefs.general.theme),
                    options: ThemePreference::all().iter().map(|t| (format!("{:?}", t), t.display_name().to_string())).collect(),
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val: String| {
                            prefs.general.theme = match val.as_str() {
                                "Dark" => ThemePreference::Dark,
                                "Light" => ThemePreference::Light,
                                _ => ThemePreference::System,
                            };
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Startup" }

            PreferenceRow {
                label: "Show startup dialog",
                description: "Show welcome dialog on launch",
                CheckboxInput {
                    checked: prefs.general.show_startup_dialog,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.general.show_startup_dialog = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Confirm before closing unsaved",
                description: "Prompt when closing files with unsaved changes",
                CheckboxInput {
                    checked: prefs.general.confirm_close_unsaved,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.general.confirm_close_unsaved = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Auto-save" }

            PreferenceRow {
                label: "Auto-save interval (seconds)",
                description: "0 to disable",
                NumberInput {
                    value: prefs.general.auto_save_interval as f64,
                    min: 0.0,
                    max: 600.0,
                    step: 30.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.general.auto_save_interval = val as u32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SchematicTab(props: TabProps) -> Element {
    let prefs = props.prefs.clone();

    rsx! {
        div {
            SectionTitle { title: "Grid" }

            PreferenceRow {
                label: "Grid size (pixels)",
                description: "Grid spacing for component placement",
                NumberInput {
                    value: prefs.schematic.grid_size as f64,
                    min: 5.0,
                    max: 50.0,
                    step: 5.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.grid_size = val as u32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Show grid",
                description: "Display grid lines in schematic",
                CheckboxInput {
                    checked: prefs.schematic.show_grid,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.show_grid = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Snap to grid",
                description: "Align components to grid positions",
                CheckboxInput {
                    checked: prefs.schematic.snap_to_grid,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.snap_to_grid = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Grid style",
                description: "Visual style of the grid",
                SelectInput {
                    value: format!("{:?}", prefs.schematic.grid_style),
                    options: GridStyle::all().iter().map(|s| (format!("{:?}", s), s.display_name().to_string())).collect(),
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val: String| {
                            prefs.schematic.grid_style = match val.as_str() {
                                "Lines" => GridStyle::Lines,
                                "Crosses" => GridStyle::Crosses,
                                _ => GridStyle::Dots,
                            };
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Display" }

            PreferenceRow {
                label: "Show component values",
                description: "Display value labels on components",
                CheckboxInput {
                    checked: prefs.schematic.show_values,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.show_values = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Show reference designators",
                description: "Display component names (R1, C1, etc.)",
                CheckboxInput {
                    checked: prefs.schematic.show_ref_des,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.show_ref_des = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Wires" }

            PreferenceRow {
                label: "Wire width (pixels)",
                description: "Thickness of wire lines",
                NumberInput {
                    value: prefs.schematic.wire_width as f64,
                    min: 1.0,
                    max: 5.0,
                    step: 0.5,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.wire_width = val as f32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Junction dot radius",
                description: "Size of wire junction indicators",
                NumberInput {
                    value: prefs.schematic.junction_radius as f64,
                    min: 2.0,
                    max: 8.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.schematic.junction_radius = val as f32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WaveformTab(props: TabProps) -> Element {
    let prefs = props.prefs.clone();

    rsx! {
        div {
            SectionTitle { title: "Traces" }

            PreferenceRow {
                label: "Trace width (pixels)",
                description: "Thickness of waveform lines",
                NumberInput {
                    value: prefs.waveform.trace_width as f64,
                    min: 0.5,
                    max: 5.0,
                    step: 0.5,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.waveform.trace_width = val as f32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Anti-aliasing",
                description: "Smooth waveform lines",
                CheckboxInput {
                    checked: prefs.waveform.anti_alias,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.waveform.anti_alias = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Interpolation",
                description: "How points are connected",
                SelectInput {
                    value: format!("{:?}", prefs.waveform.interpolation),
                    options: InterpolationMode::all().iter().map(|m| (format!("{:?}", m), m.display_name().to_string())).collect(),
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val: String| {
                            prefs.waveform.interpolation = match val.as_str() {
                                "None" => InterpolationMode::None,
                                "Spline" => InterpolationMode::Spline,
                                _ => InterpolationMode::Linear,
                            };
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Display" }

            PreferenceRow {
                label: "Show grid",
                description: "Display grid in waveform viewer",
                CheckboxInput {
                    checked: prefs.waveform.show_grid,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.waveform.show_grid = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Use SI prefixes",
                description: "Display values with μ, m, k, M, etc.",
                CheckboxInput {
                    checked: prefs.waveform.use_si_prefixes,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.waveform.use_si_prefixes = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Measurement precision",
                description: "Decimal places for measurements",
                NumberInput {
                    value: prefs.waveform.measurement_precision as f64,
                    min: 2.0,
                    max: 12.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.waveform.measurement_precision = val as u32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SimulationTab(props: TabProps) -> Element {
    let prefs = props.prefs.clone();

    rsx! {
        div {
            SectionTitle { title: "Defaults" }

            PreferenceRow {
                label: "Temperature (°C)",
                description: "Default simulation temperature",
                NumberInput {
                    value: prefs.simulation.default_temp_c,
                    min: -273.15,
                    max: 500.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.default_temp_c = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Max iterations",
                description: "Newton-Raphson iteration limit",
                NumberInput {
                    value: prefs.simulation.default_max_iter as f64,
                    min: 10.0,
                    max: 1000.0,
                    step: 10.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.default_max_iter = val as u32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Tolerances" }

            PreferenceRow {
                label: "Absolute tolerance",
                description: "Convergence threshold (abstol)",
                NumberInput {
                    value: prefs.simulation.default_abstol.log10(),
                    min: -15.0,
                    max: -6.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.default_abstol = 10f64.powf(val);
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Relative tolerance",
                description: "Convergence ratio (reltol)",
                NumberInput {
                    value: prefs.simulation.default_reltol.log10(),
                    min: -6.0,
                    max: -1.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.default_reltol = 10f64.powf(val);
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Options" }

            PreferenceRow {
                label: "Show progress",
                description: "Display simulation progress indicator",
                CheckboxInput {
                    checked: prefs.simulation.show_progress,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.show_progress = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Auto DC operating point",
                description: "Run DC OP before transient automatically",
                CheckboxInput {
                    checked: prefs.simulation.auto_dc_op,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.auto_dc_op = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Save all nodes",
                description: "Save voltages for all nodes, not just requested",
                CheckboxInput {
                    checked: prefs.simulation.save_all_nodes,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.simulation.save_all_nodes = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FilesTab(props: TabProps) -> Element {
    let prefs = props.prefs.clone();

    rsx! {
        div {
            SectionTitle { title: "Recent Files" }

            PreferenceRow {
                label: "Recent files count",
                description: "Number of recent files to remember",
                NumberInput {
                    value: prefs.files.recent_files_count as f64,
                    min: 1.0,
                    max: 25.0,
                    step: 1.0,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.files.recent_files_count = val as u32;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            SectionTitle { title: "Backup" }

            PreferenceRow {
                label: "Create backup on save",
                description: "Save backup of previous version",
                CheckboxInput {
                    checked: prefs.files.create_backup,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.files.create_backup = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }

            PreferenceRow {
                label: "Auto-save to temp",
                description: "Periodically save to temporary file",
                CheckboxInput {
                    checked: prefs.files.auto_save_temp,
                    on_change: {
                        let mut prefs = prefs.clone();
                        move |val| {
                            prefs.files.auto_save_temp = val;
                            props.on_change.call(prefs.clone());
                        }
                    }
                }
            }
        }
    }
}

//=============================================================================
// Input Components
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct SectionTitleProps {
    title: &'static str,
}

#[component]
fn SectionTitle(props: SectionTitleProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        h3 {
            style: format!(
                "
                font-size: 12px;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                color: {};
                margin: 24px 0 12px 0;
                padding-bottom: 8px;
                border-bottom: 1px solid {};
                ",
                theme.read().text_muted(),
                theme.read().border_subtle()
            ),
            "{props.title}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PreferenceRowProps {
    label: &'static str,
    description: &'static str,
    children: Element,
}

#[component]
fn PreferenceRow(props: PreferenceRowProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 12px 0;
            ",

            div {
                div {
                    style: format!("font-size: 14px; color: {};", theme.read().text_primary()),
                    "{props.label}"
                }
                div {
                    style: format!("font-size: 12px; color: {}; margin-top: 2px;", theme.read().text_muted()),
                    "{props.description}"
                }
            }

            div {
                style: "min-width: 150px; display: flex; justify-content: flex-end;",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CheckboxInputProps {
    checked: bool,
    on_change: EventHandler<bool>,
}

#[component]
fn CheckboxInput(props: CheckboxInputProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        button {
            style: format!(
                "
                width: 44px;
                height: 24px;
                border-radius: 12px;
                border: none;
                cursor: pointer;
                position: relative;
                transition: background 0.2s ease;
                background: {};
                ",
                if props.checked {
                    theme.read().accent_primary()
                } else {
                    theme.read().surface()
                }
            ),
            onclick: move |_| props.on_change.call(!props.checked),

            // Knob
            div {
                style: format!(
                    "
                    position: absolute;
                    top: 2px;
                    width: 20px;
                    height: 20px;
                    border-radius: 50%;
                    background: white;
                    transition: left 0.2s ease;
                    left: {};
                    ",
                    if props.checked { "22px" } else { "2px" }
                ),
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NumberInputProps {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    on_change: EventHandler<f64>,
}

#[component]
fn NumberInput(props: NumberInputProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        input {
            r#type: "number",
            value: "{props.value}",
            min: "{props.min}",
            max: "{props.max}",
            step: "{props.step}",
            style: format!(
                "
                width: 100px;
                padding: 6px 10px;
                border-radius: 6px;
                border: 1px solid {};
                background: {};
                color: {};
                font-size: 13px;
                font-family: {};
                ",
                theme.read().border(),
                theme.read().bg_tertiary(),
                theme.read().text_primary(),
                Theme::FONT_MONO
            ),
            oninput: move |e| {
                if let Ok(val) = e.value().parse::<f64>() {
                    props.on_change.call(val.clamp(props.min, props.max));
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SelectInputProps {
    value: String,
    options: Vec<(String, String)>,
    on_change: EventHandler<String>,
}

#[component]
fn SelectInput(props: SelectInputProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        select {
            value: "{props.value}",
            style: format!(
                "
                padding: 6px 10px;
                border-radius: 6px;
                border: 1px solid {};
                background: {};
                color: {};
                font-size: 13px;
                cursor: pointer;
                min-width: 120px;
                ",
                theme.read().border(),
                theme.read().bg_tertiary(),
                theme.read().text_primary()
            ),
            onchange: move |e| props.on_change.call(e.value()),

            for (value, label) in &props.options {
                option {
                    value: "{value}",
                    selected: *value == props.value,
                    "{label}"
                }
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_tab_all() {
        let all = PreferencesTab::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_preferences_tab_display_names() {
        for tab in PreferencesTab::all() {
            assert!(!tab.display_name().is_empty());
            assert!(!tab.icon().is_empty());
        }
    }
}
