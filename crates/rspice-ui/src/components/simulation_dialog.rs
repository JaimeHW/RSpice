//! Simulation Dialog Component
//!
//! A professional tabbed dialog for configuring simulation analyses.
//! Replaces manual SPICE command editing with intuitive form inputs.

use dioxus::prelude::*;

use crate::state::simulation_command::{
    parse_spice_value, AcConfig, AcSweepType, DcSweepConfig, McDistribution, MonteCarloConfig,
    NoiseConfig, OpConfig, PoleZeroConfig, PzTransferType, SParamConfig, SensitivityConfig,
    SimulationConfig, TransientConfig,
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
    Noise,
    MonteCarlo,
    PoleZero,
    Sensitivity,
    SParam,
}

impl AnalysisTab {
    pub fn label(&self) -> &'static str {
        match self {
            AnalysisTab::Transient => "Transient",
            AnalysisTab::Ac => "AC Analysis",
            AnalysisTab::DcSweep => "DC Sweep",
            AnalysisTab::Op => "Operating Point",
            AnalysisTab::Noise => "Noise",
            AnalysisTab::MonteCarlo => "Monte Carlo",
            AnalysisTab::PoleZero => "Pole-Zero",
            AnalysisTab::Sensitivity => "Sensitivity",
            AnalysisTab::SParam => "S-Param",
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
    let noise_enabled = use_signal(|| props.config.noise.is_some());
    let mc_enabled = use_signal(|| props.config.monte_carlo.is_some());
    let pz_enabled = use_signal(|| props.config.pole_zero.is_some());
    let sens_enabled = use_signal(|| props.config.sensitivity.is_some());
    let sp_enabled = use_signal(|| props.config.s_param.is_some());

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

    // Noise analysis parameters
    let noise_output = use_signal(|| {
        props
            .config
            .noise
            .as_ref()
            .map(|n| n.output_node.clone())
            .unwrap_or_else(|| "out".to_string())
    });
    let noise_ref = use_signal(|| {
        props
            .config
            .noise
            .as_ref()
            .map(|n| n.reference_node.clone())
            .unwrap_or_else(|| "0".to_string())
    });
    let noise_input = use_signal(|| {
        props
            .config
            .noise
            .as_ref()
            .map(|n| n.input_source.clone())
            .unwrap_or_else(|| "Vin".to_string())
    });
    let noise_start = use_signal(|| {
        format_value(
            props
                .config
                .noise
                .as_ref()
                .map(|n| n.start_freq)
                .unwrap_or(1.0),
        )
    });
    let noise_stop = use_signal(|| {
        format_value(
            props
                .config
                .noise
                .as_ref()
                .map(|n| n.stop_freq)
                .unwrap_or(1e6),
        )
    });
    let noise_pts = use_signal(|| {
        props
            .config
            .noise
            .as_ref()
            .map(|n| n.points_per_decade)
            .unwrap_or(10)
            .to_string()
    });

    // Monte Carlo parameters
    let mc_runs = use_signal(|| {
        props
            .config
            .monte_carlo
            .as_ref()
            .map(|m| m.num_runs)
            .unwrap_or(100)
            .to_string()
    });
    let mc_tolerance = use_signal(|| {
        props
            .config
            .monte_carlo
            .as_ref()
            .map(|m| m.default_tolerance)
            .unwrap_or(5.0)
            .to_string()
    });
    let mc_output = use_signal(|| {
        props
            .config
            .monte_carlo
            .as_ref()
            .map(|m| m.track_output.clone())
            .unwrap_or_else(|| "V(out)".to_string())
    });
    let mc_dist = use_signal(|| {
        props
            .config
            .monte_carlo
            .as_ref()
            .map(|m| m.distribution)
            .unwrap_or_default()
    });

    // Pole-Zero parameters
    let pz_in_pos = use_signal(|| {
        props
            .config
            .pole_zero
            .as_ref()
            .map(|p| p.input_pos.clone())
            .unwrap_or_else(|| "in".to_string())
    });
    let pz_in_neg = use_signal(|| {
        props
            .config
            .pole_zero
            .as_ref()
            .map(|p| p.input_neg.clone())
            .unwrap_or_else(|| "0".to_string())
    });
    let pz_out_pos = use_signal(|| {
        props
            .config
            .pole_zero
            .as_ref()
            .map(|p| p.output_pos.clone())
            .unwrap_or_else(|| "out".to_string())
    });
    let pz_out_neg = use_signal(|| {
        props
            .config
            .pole_zero
            .as_ref()
            .map(|p| p.output_neg.clone())
            .unwrap_or_else(|| "0".to_string())
    });

    // Sensitivity parameters
    let sens_output = use_signal(|| {
        props
            .config
            .sensitivity
            .as_ref()
            .map(|s| s.output_var.clone())
            .unwrap_or_else(|| "V(out)".to_string())
    });
    let sens_is_ac = use_signal(|| {
        props
            .config
            .sensitivity
            .as_ref()
            .map(|s| s.is_ac)
            .unwrap_or(false)
    });
    let sens_freq = use_signal(|| {
        format_value(
            props
                .config
                .sensitivity
                .as_ref()
                .map(|s| s.frequency)
                .unwrap_or(1e6),
        )
    });

    // S-Parameter parameters
    let sp_port1_pos = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.port1_pos.clone())
            .unwrap_or_else(|| "in".to_string())
    });
    let sp_port1_neg = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.port1_neg.clone())
            .unwrap_or_else(|| "0".to_string())
    });
    let sp_port2_pos = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.port2_pos.clone())
            .unwrap_or_else(|| "out".to_string())
    });
    let sp_port2_neg = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.port2_neg.clone())
            .unwrap_or_else(|| "0".to_string())
    });
    let sp_z0 = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.z0)
            .unwrap_or(50.0)
            .to_string()
    });
    let sp_start = use_signal(|| {
        format_value(
            props
                .config
                .s_param
                .as_ref()
                .map(|s| s.start_freq)
                .unwrap_or(1e6),
        )
    });
    let sp_stop = use_signal(|| {
        format_value(
            props
                .config
                .s_param
                .as_ref()
                .map(|s| s.stop_freq)
                .unwrap_or(10e9),
        )
    });
    let sp_pts = use_signal(|| {
        props
            .config
            .s_param
            .as_ref()
            .map(|s| s.points_per_decade)
            .unwrap_or(20)
            .to_string()
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

        if *noise_enabled.read() {
            config.noise = Some(NoiseConfig {
                output_node: noise_output.read().clone(),
                reference_node: noise_ref.read().clone(),
                input_source: noise_input.read().clone(),
                start_freq: parse_spice_value(&noise_start.read()).unwrap_or(1.0),
                stop_freq: parse_spice_value(&noise_stop.read()).unwrap_or(1e6),
                points_per_decade: noise_pts.read().parse().unwrap_or(10),
                sweep_type: AcSweepType::Decade,
            });
        }

        if *mc_enabled.read() {
            config.monte_carlo = Some(MonteCarloConfig {
                num_runs: mc_runs.read().parse().unwrap_or(100),
                seed: None,
                default_tolerance: mc_tolerance.read().parse().unwrap_or(5.0),
                distribution: *mc_dist.read(),
                run_transient: true,
                track_output: mc_output.read().clone(),
            });
        }

        if *pz_enabled.read() {
            config.pole_zero = Some(PoleZeroConfig {
                input_pos: pz_in_pos.read().clone(),
                input_neg: pz_in_neg.read().clone(),
                output_pos: pz_out_pos.read().clone(),
                output_neg: pz_out_neg.read().clone(),
                transfer_type: PzTransferType::Voltage,
            });
        }

        if *sens_enabled.read() {
            config.sensitivity = Some(SensitivityConfig {
                output_var: sens_output.read().clone(),
                is_ac: *sens_is_ac.read(),
                frequency: parse_spice_value(&sens_freq.read()).unwrap_or(1e6),
            });
        }

        if *sp_enabled.read() {
            config.s_param = Some(SParamConfig {
                port1_pos: sp_port1_pos.read().clone(),
                port1_neg: sp_port1_neg.read().clone(),
                port2_pos: sp_port2_pos.read().clone(),
                port2_neg: sp_port2_neg.read().clone(),
                z0: sp_z0.read().parse().unwrap_or(50.0),
                start_freq: parse_spice_value(&sp_start.read()).unwrap_or(1e6),
                stop_freq: parse_spice_value(&sp_stop.read()).unwrap_or(10e9),
                points_per_decade: sp_pts.read().parse().unwrap_or(20),
            });
        }

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

                // Main content area with vertical sidebar
                div {
                    style: "display: flex; min-height: 340px;",

                    // Vertical sidebar navigation
                    div {
                        style: "width: 140px; background: {th.bg_primary()}; border-right: 1px solid {th.border()}; padding: 8px 0;",

                        div {
                            style: "padding: 0 8px 8px 8px; font-size: 10px; font-weight: 600; color: {th.text_muted()}; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Analysis Types"
                        }

                        SidebarButton {
                            label: "Transient",
                            active: *active_tab.read() == AnalysisTab::Transient,
                            enabled: *transient_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::Transient),
                        }
                        SidebarButton {
                            label: "AC Analysis",
                            active: *active_tab.read() == AnalysisTab::Ac,
                            enabled: *ac_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::Ac),
                        }
                        SidebarButton {
                            label: "DC Sweep",
                            active: *active_tab.read() == AnalysisTab::DcSweep,
                            enabled: *dc_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::DcSweep),
                        }
                        SidebarButton {
                            label: "Op Point",
                            active: *active_tab.read() == AnalysisTab::Op,
                            enabled: *op_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::Op),
                        }
                        SidebarButton {
                            label: "Noise",
                            active: *active_tab.read() == AnalysisTab::Noise,
                            enabled: *noise_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::Noise),
                        }
                        SidebarButton {
                            label: "Monte Carlo",
                            active: *active_tab.read() == AnalysisTab::MonteCarlo,
                            enabled: *mc_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::MonteCarlo),
                        }
                        SidebarButton {
                            label: "Pole-Zero",
                            active: *active_tab.read() == AnalysisTab::PoleZero,
                            enabled: *pz_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::PoleZero),
                        }
                        SidebarButton {
                            label: "Sensitivity",
                            active: *active_tab.read() == AnalysisTab::Sensitivity,
                            enabled: *sens_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::Sensitivity),
                        }
                        SidebarButton {
                            label: "S-Parameters",
                            active: *active_tab.read() == AnalysisTab::SParam,
                            enabled: *sp_enabled.read(),
                            onclick: move |_| active_tab.set(AnalysisTab::SParam),
                        }
                    }

                    // Tab content area
                    div {
                        style: "flex: 1; padding: 20px; overflow-y: auto;",

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
                            AnalysisTab::Noise => rsx! {
                                NoiseTab {
                                    enabled: noise_enabled,
                                    output_node: noise_output,
                                    ref_node: noise_ref,
                                    input_source: noise_input,
                                    start_freq: noise_start,
                                    stop_freq: noise_stop,
                                    points: noise_pts,
                                }
                            },
                            AnalysisTab::MonteCarlo => rsx! {
                                MonteCarloTab {
                                    enabled: mc_enabled,
                                    num_runs: mc_runs,
                                    tolerance: mc_tolerance,
                                    output_var: mc_output,
                                    distribution: mc_dist,
                                }
                            },
                            AnalysisTab::PoleZero => rsx! {
                                PoleZeroTab {
                                    enabled: pz_enabled,
                                    input_pos: pz_in_pos,
                                    input_neg: pz_in_neg,
                                    output_pos: pz_out_pos,
                                    output_neg: pz_out_neg,
                                }
                            },
                            AnalysisTab::Sensitivity => rsx! {
                                SensitivityTab {
                                    enabled: sens_enabled,
                                    output_var: sens_output,
                                    is_ac: sens_is_ac,
                                    frequency: sens_freq,
                                }
                            },
                            AnalysisTab::SParam => rsx! {
                                SParamTab {
                                    enabled: sp_enabled,
                                    port1_pos: sp_port1_pos,
                                    port1_neg: sp_port1_neg,
                                    port2_pos: sp_port2_pos,
                                    port2_neg: sp_port2_neg,
                                    z0: sp_z0,
                                    start_freq: sp_start,
                                    stop_freq: sp_stop,
                                    points: sp_pts,
                                }
                            },
                        }
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

#[component]
fn NoiseTab(
    enabled: Signal<bool>,
    output_node: Signal<String>,
    ref_node: Signal<String>,
    input_source: Signal<String>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points: Signal<String>,
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
                    "Enable Noise Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Output Node",
                    help: "Node to measure output noise",
                    FormInput {
                        value: output_node.read().clone(),
                        placeholder: "out",
                        suffix: "",
                        onchange: move |v| output_node.set(v),
                    }
                }

                FormRow {
                    label: "Reference Node",
                    help: "Reference for output (usually 0)",
                    FormInput {
                        value: ref_node.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| ref_node.set(v),
                    }
                }

                FormRow {
                    label: "Input Source",
                    help: "Source for input-referred noise",
                    FormInput {
                        value: input_source.read().clone(),
                        placeholder: "Vin",
                        suffix: "",
                        onchange: move |v| input_source.set(v),
                    }
                }

                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of frequency sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of frequency sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| points.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn MonteCarloTab(
    enabled: Signal<bool>,
    num_runs: Signal<String>,
    tolerance: Signal<String>,
    output_var: Signal<String>,
    distribution: Signal<McDistribution>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_dist = *distribution.read();

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
                    "Enable Monte Carlo Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Number of Runs",
                    help: "Simulation iterations",
                    FormInput {
                        value: num_runs.read().clone(),
                        placeholder: "100",
                        suffix: "",
                        onchange: move |v| num_runs.set(v),
                    }
                }

                FormRow {
                    label: "Tolerance",
                    help: "Component variation range",
                    FormInput {
                        value: tolerance.read().clone(),
                        placeholder: "5",
                        suffix: "%",
                        onchange: move |v| tolerance.set(v),
                    }
                }

                FormRow {
                    label: "Distribution",
                    help: "Statistical distribution type",
                    select {
                        style: "padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px; width: 100%;",
                        value: if current_dist == McDistribution::Uniform { "uniform" } else { "gaussian" },
                        onchange: move |e: Event<FormData>| {
                            let val = e.value();
                            distribution.set(if val == "gaussian" { McDistribution::Gaussian } else { McDistribution::Uniform });
                        },
                        option { value: "uniform", "Uniform (±%)" }
                        option { value: "gaussian", "Gaussian (σ)" }
                    }
                }

                FormRow {
                    label: "Track Output",
                    help: "Variable to measure statistics",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn PoleZeroTab(
    enabled: Signal<bool>,
    input_pos: Signal<String>,
    input_neg: Signal<String>,
    output_pos: Signal<String>,
    output_neg: Signal<String>,
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
                    "Enable Pole-Zero Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                p {
                    style: "color: {th.text_muted()}; font-size: 12px; margin: 0 0 16px 0;",
                    "Finds poles and zeros of the transfer function for stability and frequency analysis."
                }

                FormRow {
                    label: "Input (+)",
                    help: "Positive input node",
                    FormInput {
                        value: input_pos.read().clone(),
                        placeholder: "in",
                        suffix: "",
                        onchange: move |v| input_pos.set(v),
                    }
                }

                FormRow {
                    label: "Input (−)",
                    help: "Negative input node (ref)",
                    FormInput {
                        value: input_neg.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| input_neg.set(v),
                    }
                }

                FormRow {
                    label: "Output (+)",
                    help: "Positive output node",
                    FormInput {
                        value: output_pos.read().clone(),
                        placeholder: "out",
                        suffix: "",
                        onchange: move |v| output_pos.set(v),
                    }
                }

                FormRow {
                    label: "Output (−)",
                    help: "Negative output node (ref)",
                    FormInput {
                        value: output_neg.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| output_neg.set(v),
                    }
                }
            }
        }
    }
}

#[component]
fn SensitivityTab(
    enabled: Signal<bool>,
    output_var: Signal<String>,
    is_ac: Signal<bool>,
    frequency: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();
    let is_ac_mode = *is_ac.read();
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
                    "Enable Sensitivity Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Output Variable",
                    help: "Variable to analyze sensitivity",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }

                FormRow {
                    label: "Analysis Type",
                    help: "DC or AC sensitivity",
                    div {
                        style: "display: flex; gap: 16px;",
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "radio",
                                name: "sens_type",
                                checked: !is_ac_mode,
                                onchange: move |_| is_ac.set(false),
                            }
                            span { style: "color: {th.text_primary()};", "DC" }
                        }
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "radio",
                                name: "sens_type",
                                checked: is_ac_mode,
                                onchange: move |_| is_ac.set(true),
                            }
                            span { style: "color: {th.text_primary()};", "AC" }
                        }
                    }
                }

                if is_ac_mode {
                    FormRow {
                        label: "Frequency",
                        help: "AC sensitivity frequency",
                        FormInput {
                            value: frequency.read().clone(),
                            placeholder: "1MEG",
                            suffix: "Hz",
                            onchange: move |v| frequency.set(v),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SParamTab(
    enabled: Signal<bool>,
    port1_pos: Signal<String>,
    port1_neg: Signal<String>,
    port2_pos: Signal<String>,
    port2_neg: Signal<String>,
    z0: Signal<String>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points: Signal<String>,
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
                    "Enable S-Parameter Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                p {
                    style: "color: {th.text_muted()}; font-size: 12px; margin: 0 0 16px 0;",
                    "RF/microwave scattering parameter analysis for S11, S21, S12, S22."
                }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 14px;",
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 1 (+)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port1_pos.read()}",
                            oninput: move |e| port1_pos.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 1 (−)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port1_neg.read()}",
                            oninput: move |e| port1_neg.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 2 (+)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port2_pos.read()}",
                            oninput: move |e| port2_pos.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 2 (−)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port2_neg.read()}",
                            oninput: move |e| port2_neg.set(e.value()),
                        }
                    }
                }

                FormRow {
                    label: "Reference Z₀",
                    help: "Characteristic impedance",
                    FormInput {
                        value: z0.read().clone(),
                        placeholder: "50",
                        suffix: "Ω",
                        onchange: move |v| z0.set(v),
                    }
                }

                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "10G",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points.read().clone(),
                        placeholder: "20",
                        suffix: "",
                        onchange: move |v| points.set(v),
                    }
                }
            }
        }
    }
}

// =============================================================================
// Navigation Components
// =============================================================================

#[derive(Props, Clone, PartialEq)]
pub struct SidebarButtonProps {
    label: &'static str,
    active: bool,
    enabled: bool,
    onclick: EventHandler<()>,
}

#[component]
fn SidebarButton(props: SidebarButtonProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let bg = if props.active {
        th.accent_primary()
    } else {
        "transparent"
    };
    let text_color = if props.active {
        "#ffffff"
    } else if props.enabled {
        th.accent_primary()
    } else {
        th.text_secondary()
    };
    let font_weight = if props.active { "600" } else { "400" };
    let indicator_color = th.accent_primary();

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 6px; width: 100%; padding: 8px 12px; margin: 2px 0; background: {bg}; border: none; border-radius: 4px; color: {text_color}; font-size: 12px; font-weight: {font_weight}; text-align: left; cursor: pointer; transition: background 0.15s;",
            onclick: move |_| props.onclick.call(()),
            if props.enabled && !props.active {
                span {
                    style: "font-size: 6px; color: {indicator_color};",
                    "●"
                }
            }
            "{props.label}"
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
