//! Simulation Dialog Component
//!
//! A professional tabbed dialog for configuring simulation analyses.
//! Replaces manual SPICE command editing with intuitive form inputs.

use dioxus::prelude::*;

use crate::state::simulation_command::{
    parse_spice_value, AcConfig, AcSweepType, DcSweepConfig, OpConfig, SimulationConfig,
    TransientConfig,
};
use crate::theme::Theme;

// =============================================================================
// Dialog State
// =============================================================================

/// Which analysis tab is currently active
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnalysisTab {
    #[default]
    Transient,
    Ac,
    DcSweep,
    Op,
}

impl AnalysisTab {
    pub fn label(&self) -> &'static str {
        match self {
            AnalysisTab::Transient => "Transient",
            AnalysisTab::Ac => "AC Analysis",
            AnalysisTab::DcSweep => "DC Sweep",
            AnalysisTab::Op => "Operating Point",
        }
    }
}

// =============================================================================
// Dialog Props
// =============================================================================

#[derive(Props, Clone, PartialEq)]
pub struct SimulationDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current simulation configuration
    pub config: SimulationConfig,
    /// Called when OK is clicked with the new configuration
    pub on_confirm: EventHandler<SimulationConfig>,
    /// Called when Cancel is clicked
    pub on_cancel: EventHandler<()>,
}

// =============================================================================
// Main Dialog Component
// =============================================================================

/// Professional simulation setup dialog with tabbed interface
#[component]
pub fn SimulationDialog(props: SimulationDialogProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Local state for editing (not committed until OK)
    let mut active_tab = use_signal(|| AnalysisTab::Transient);
    let transient_enabled = use_signal(|| props.config.transient.is_some());
    let ac_enabled = use_signal(|| props.config.ac.is_some());
    let dc_enabled = use_signal(|| props.config.dc_sweep.is_some());
    let op_enabled = use_signal(|| props.config.op.enabled);

    // Draggable dialog state
    let mut dialog_pos = use_signal(|| (100.0_f64, 100.0_f64)); // (x, y) in pixels
    let mut dragging = use_signal(|| false);
    let mut drag_offset = use_signal(|| (0.0_f64, 0.0_f64));

    // Form values - stored as strings for editing, parsed on confirm
    let tran_stop = use_signal(|| {
        format_value(
            props
                .config
                .transient
                .as_ref()
                .map(|t| t.stop_time)
                .unwrap_or(1e-3),
        )
    });
    let tran_step = use_signal(|| {
        format_value(
            props
                .config
                .transient
                .as_ref()
                .map(|t| t.time_step)
                .unwrap_or(1e-6),
        )
    });
    let tran_start = use_signal(|| {
        format_value(
            props
                .config
                .transient
                .as_ref()
                .map(|t| t.start_time)
                .unwrap_or(0.0),
        )
    });
    let tran_max_step = use_signal(|| {
        props
            .config
            .transient
            .as_ref()
            .and_then(|t| t.max_step)
            .map(format_value)
            .unwrap_or_default()
    });

    let ac_start = use_signal(|| {
        format_value(
            props
                .config
                .ac
                .as_ref()
                .map(|a| a.start_freq)
                .unwrap_or(1.0),
        )
    });
    let ac_stop =
        use_signal(|| format_value(props.config.ac.as_ref().map(|a| a.stop_freq).unwrap_or(1e6)));
    let ac_points = use_signal(|| {
        props
            .config
            .ac
            .as_ref()
            .map(|a| a.num_points)
            .unwrap_or(10)
            .to_string()
    });
    let ac_sweep = use_signal(|| {
        props
            .config
            .ac
            .as_ref()
            .map(|a| a.sweep_type)
            .unwrap_or_default()
    });

    let dc_source = use_signal(|| {
        props
            .config
            .dc_sweep
            .as_ref()
            .map(|d| d.source_name.clone())
            .unwrap_or_else(|| "V1".to_string())
    });
    let dc_start = use_signal(|| {
        format_value(
            props
                .config
                .dc_sweep
                .as_ref()
                .map(|d| d.start_value)
                .unwrap_or(0.0),
        )
    });
    let dc_stop = use_signal(|| {
        format_value(
            props
                .config
                .dc_sweep
                .as_ref()
                .map(|d| d.stop_value)
                .unwrap_or(5.0),
        )
    });
    let dc_step = use_signal(|| {
        format_value(
            props
                .config
                .dc_sweep
                .as_ref()
                .map(|d| d.step_value)
                .unwrap_or(0.1),
        )
    });

    if !props.visible {
        return rsx! {};
    }

    // Build config from form values
    let build_config = move || {
        let mut config = SimulationConfig::new();

        if *transient_enabled.read() {
            config.transient = Some(TransientConfig {
                stop_time: parse_spice_value(&tran_stop.read()).unwrap_or(1e-3),
                time_step: parse_spice_value(&tran_step.read()).unwrap_or(1e-6),
                start_time: parse_spice_value(&tran_start.read()).unwrap_or(0.0),
                max_step: parse_spice_value(&tran_max_step.read()),
                use_initial_conditions: false,
            });
        }

        if *ac_enabled.read() {
            config.ac = Some(AcConfig {
                start_freq: parse_spice_value(&ac_start.read()).unwrap_or(1.0),
                stop_freq: parse_spice_value(&ac_stop.read()).unwrap_or(1e6),
                num_points: ac_points.read().parse().unwrap_or(10),
                sweep_type: *ac_sweep.read(),
            });
        }

        if *dc_enabled.read() {
            config.dc_sweep = Some(DcSweepConfig {
                source_name: dc_source.read().clone(),
                start_value: parse_spice_value(&dc_start.read()).unwrap_or(0.0),
                stop_value: parse_spice_value(&dc_stop.read()).unwrap_or(5.0),
                step_value: parse_spice_value(&dc_step.read()).unwrap_or(0.1),
                source2: None,
            });
        }

        config.op = OpConfig {
            enabled: *op_enabled.read(),
        };
        config
    };

    // Pre-compute preview
    let preview_config = build_config();
    let preview_text = preview_config.to_spice_string();
    let preview_display = if preview_text.is_empty() {
        "(No analyses selected)".to_string()
    } else {
        preview_text
    };

    let (pos_x, pos_y) = *dialog_pos.read();

    rsx! {
        // Positioned draggable dialog (no modal backdrop - allows interaction with schematic)
        div {
            style: "position: fixed; left: {pos_x}px; top: {pos_y}px; z-index: 1000;",
            onmousemove: move |e| {
                if *dragging.read() {
                    let (ox, oy) = *drag_offset.read();
                    let page = e.page_coordinates();
                    dialog_pos.set((page.x - ox, page.y - oy));
                }
            },
            onmouseup: move |_| dragging.set(false),
            onmouseleave: move |_| dragging.set(false),

            // Dialog container
            div {
                style: "background: {th.bg_secondary()}; border: 1px solid {th.border()}; border-radius: 8px; width: 520px; max-height: 80vh; overflow: hidden; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);",

                // Draggable header
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 16px 20px; border-bottom: 1px solid {th.border()}; background: {th.bg_tertiary()}; cursor: move; user-select: none;",
                    onmousedown: move |e| {
                        let page = e.page_coordinates();
                        let (px, py) = *dialog_pos.read();
                        drag_offset.set((page.x - px, page.y - py));
                        dragging.set(true);
                    },
                    h2 {
                        style: "margin: 0; font-size: 16px; font-weight: 600; color: {th.text_primary()};",
                        "Edit Simulation Command"
                    }
                    button {
                        style: "background: none; border: none; color: {th.text_muted()}; font-size: 20px; cursor: pointer; padding: 4px; line-height: 1;",
                        onmousedown: move |e| e.stop_propagation(),
                        onclick: move |_| props.on_cancel.call(()),
                        "×"
                    }
                }

                // Tab bar
                div {
                    style: "display: flex; border-bottom: 1px solid {th.border()}; background: {th.bg_primary()};",
                    TabButton {
                        label: "Transient",
                        active: *active_tab.read() == AnalysisTab::Transient,
                        enabled: *transient_enabled.read(),
                        onclick: move |_| active_tab.set(AnalysisTab::Transient),
                    }
                    TabButton {
                        label: "AC Analysis",
                        active: *active_tab.read() == AnalysisTab::Ac,
                        enabled: *ac_enabled.read(),
                        onclick: move |_| active_tab.set(AnalysisTab::Ac),
                    }
                    TabButton {
                        label: "DC Sweep",
                        active: *active_tab.read() == AnalysisTab::DcSweep,
                        enabled: *dc_enabled.read(),
                        onclick: move |_| active_tab.set(AnalysisTab::DcSweep),
                    }
                    TabButton {
                        label: "Op Point",
                        active: *active_tab.read() == AnalysisTab::Op,
                        enabled: *op_enabled.read(),
                        onclick: move |_| active_tab.set(AnalysisTab::Op),
                    }
                }

                // Tab content
                div {
                    style: "padding: 20px; min-height: 280px;",

                    match *active_tab.read() {
                        AnalysisTab::Transient => rsx! {
                            TransientTab {
                                enabled: transient_enabled,
                                stop_time: tran_stop,
                                time_step: tran_step,
                                start_time: tran_start,
                                max_step: tran_max_step,
                            }
                        },
                        AnalysisTab::Ac => rsx! {
                            AcTab {
                                enabled: ac_enabled,
                                start_freq: ac_start,
                                stop_freq: ac_stop,
                                num_points: ac_points,
                                sweep_type: ac_sweep,
                            }
                        },
                        AnalysisTab::DcSweep => rsx! {
                            DcSweepTab {
                                enabled: dc_enabled,
                                source_name: dc_source,
                                start_value: dc_start,
                                stop_value: dc_stop,
                                step_value: dc_step,
                            }
                        },
                        AnalysisTab::Op => rsx! {
                            OpTab { enabled: op_enabled }
                        },
                    }
                }

                // Command preview
                div {
                    style: "padding: 12px 20px; background: {th.bg_primary()}; border-top: 1px solid {th.border()};",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 6px;",
                        "Generated SPICE Commands:"
                    }
                    pre {
                        style: "margin: 0; padding: 8px 12px; background: {th.bg_tertiary()}; border-radius: 4px; font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 12px; color: {th.accent_primary()}; overflow-x: auto; min-height: 24px;",
                        "{preview_display}"
                    }
                }

                // Footer with buttons
                div {
                    style: "display: flex; justify-content: flex-end; gap: 10px; padding: 16px 20px; border-top: 1px solid {th.border()}; background: {th.bg_tertiary()};",
                    button {
                        style: "padding: 8px 16px; background: transparent; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_secondary()}; font-size: 13px; cursor: pointer;",
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        style: "padding: 8px 20px; background: {th.accent_primary()}; border: none; border-radius: 4px; color: white; font-size: 13px; font-weight: 500; cursor: pointer;",
                        onclick: move |_| props.on_confirm.call(build_config()),
                        "OK"
                    }
                }
            }
        }
    }
}

// =============================================================================
// Tab Button Component
// =============================================================================

#[component]
fn TabButton(
    label: &'static str,
    active: bool,
    enabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if active {
        th.bg_secondary().to_string()
    } else {
        "transparent".to_string()
    };
    let color = if enabled {
        th.text_primary()
    } else {
        th.text_muted()
    };
    let weight = if active { "600" } else { "400" };
    let border_color = if active {
        th.accent_primary().to_string()
    } else {
        "transparent".to_string()
    };
    let dot_color = th.accent_success();

    rsx! {
        button {
            style: "padding: 10px 16px; border: none; background: {bg}; color: {color}; font-size: 13px; font-weight: {weight}; cursor: pointer; border-bottom: 2px solid {border_color}; transition: all 0.15s ease;",
            onclick: move |e| onclick.call(e),
            "{label}"
            if enabled {
                span {
                    style: "margin-left: 6px; width: 8px; height: 8px; background: {dot_color}; border-radius: 50%; display: inline-block;",
                }
            }
        }
    }
}

// =============================================================================
// Tab Content Components
// =============================================================================

#[component]
fn TransientTab(
    enabled: Signal<bool>,
    stop_time: Signal<String>,
    time_step: Signal<String>,
    start_time: Signal<String>,
    max_step: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };

    rsx! {
        div {
            // Enable checkbox
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable Transient Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Stop Time",
                    help: "End time for simulation",
                    FormInput {
                        value: stop_time.read().clone(),
                        placeholder: "1m",
                        suffix: "s",
                        onchange: move |v| stop_time.set(v),
                    }
                }

                FormRow {
                    label: "Time Step",
                    help: "Suggested internal step",
                    FormInput {
                        value: time_step.read().clone(),
                        placeholder: "1u",
                        suffix: "s",
                        onchange: move |v| time_step.set(v),
                    }
                }

                FormRow {
                    label: "Start Saving At",
                    help: "Skip data before this time",
                    FormInput {
                        value: start_time.read().clone(),
                        placeholder: "0",
                        suffix: "s",
                        onchange: move |v| start_time.set(v),
                    }
                }

                FormRow {
                    label: "Maximum Step",
                    help: "Limit adaptive stepping (optional)",
                    FormInput {
                        value: max_step.read().clone(),
                        placeholder: "(auto)",
                        suffix: "s",
                        onchange: move |v| max_step.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn AcTab(
    enabled: Signal<bool>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    num_points: Signal<String>,
    sweep_type: Signal<AcSweepType>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_sweep = *sweep_type.read();

    let points_help = match current_sweep {
        AcSweepType::Decade => "Points per decade",
        AcSweepType::Octave => "Points per octave",
        AcSweepType::Linear => "Total number of points",
    };

    rsx! {
        div {
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable AC Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Sweep Type",
                    help: "Frequency spacing method",
                    select {
                        style: "width: 100%; padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px;",
                        onchange: move |e: Event<FormData>| {
                            let val = e.value();
                            let st = match val.as_str() {
                                "Octave" => AcSweepType::Octave,
                                "Linear" => AcSweepType::Linear,
                                _ => AcSweepType::Decade,
                            };
                            sweep_type.set(st);
                        },
                        option { value: "Decade", selected: current_sweep == AcSweepType::Decade, "Decade" }
                        option { value: "Octave", selected: current_sweep == AcSweepType::Octave, "Octave" }
                        option { value: "Linear", selected: current_sweep == AcSweepType::Linear, "Linear" }
                    }
                }

                FormRow {
                    label: "Points",
                    help: points_help,
                    FormInput {
                        value: num_points.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| num_points.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn DcSweepTab(
    enabled: Signal<bool>,
    source_name: Signal<String>,
    start_value: Signal<String>,
    stop_value: Signal<String>,
    step_value: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };

    rsx! {
        div {
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable DC Sweep Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Source Name",
                    help: "Voltage or current source to sweep",
                    FormInput {
                        value: source_name.read().clone(),
                        placeholder: "V1",
                        suffix: "",
                        onchange: move |v| source_name.set(v),
                    }
                }

                FormRow {
                    label: "Start Value",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_value.read().clone(),
                        placeholder: "0",
                        suffix: "V",
                        onchange: move |v| start_value.set(v),
                    }
                }

                FormRow {
                    label: "Stop Value",
                    help: "End of sweep",
                    FormInput {
                        value: stop_value.read().clone(),
                        placeholder: "5",
                        suffix: "V",
                        onchange: move |v| stop_value.set(v),
                    }
                }

                FormRow {
                    label: "Increment",
                    help: "Step size",
                    FormInput {
                        value: step_value.read().clone(),
                        placeholder: "0.1",
                        suffix: "V",
                        onchange: move |v| step_value.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn OpTab(enabled: Signal<bool>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();

    rsx! {
        div {
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 16px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable Operating Point Analysis"
                }
            }

            p {
                style: "color: {th.text_muted()}; font-size: 13px; line-height: 1.5; margin: 0;",
                "Operating point analysis computes the DC bias point of the circuit. "
                "This calculates all node voltages and branch currents with capacitors "
                "open and inductors shorted."
            }
        }
    }
}

// =============================================================================
// Form Helper Components
// =============================================================================

#[component]
fn FormRow(label: &'static str, help: &'static str, children: Element) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 140px 1fr; gap: 12px; align-items: center; margin-bottom: 14px;",
            div {
                label {
                    style: "font-size: 13px; font-weight: 500; color: {th.text_secondary()};",
                    "{label}"
                }
                div {
                    style: "font-size: 11px; color: {th.text_muted()}; margin-top: 2px;",
                    "{help}"
                }
            }
            {children}
        }
    }
}

#[component]
fn FormInput(
    value: String,
    placeholder: &'static str,
    suffix: &'static str,
    onchange: EventHandler<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            input {
                r#type: "text",
                style: "flex: 1; padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px; font-family: 'JetBrains Mono', 'Fira Code', monospace;",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| onchange.call(e.value()),
            }
            if !suffix.is_empty() {
                span {
                    style: "color: {th.text_muted()}; font-size: 12px; min-width: 20px;",
                    "{suffix}"
                }
            }
        }
    }
}

// =============================================================================
// Utilities
// =============================================================================

/// Format a value for display in form fields
fn format_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }

    let abs_val = v.abs();

    // Use engineering notation for convenient editing
    if abs_val >= 1e9 {
        format!("{}G", v / 1e9)
    } else if abs_val >= 1e6 {
        format!("{}MEG", v / 1e6)
    } else if abs_val >= 1e3 {
        format!("{}k", v / 1e3)
    } else if abs_val >= 1.0 {
        format!("{}", v)
    } else if abs_val >= 1e-3 {
        format!("{}m", v * 1e3)
    } else if abs_val >= 1e-6 {
        format!("{}u", v * 1e6)
    } else if abs_val >= 1e-9 {
        format!("{}n", v * 1e9)
    } else if abs_val >= 1e-12 {
        format!("{}p", v * 1e12)
    } else {
        format!("{:e}", v)
    }
}
