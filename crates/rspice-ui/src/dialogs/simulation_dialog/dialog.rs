//! Main Simulation Dialog Component
//!
//! A tabbed dialog for configuring simulation analyses.

use dioxus::prelude::*;

use super::super::resizable_dialog::ResizeEdge;
use super::form_components::{format_value, SidebarButton};
use super::tabs::{
    AcTab, CornerTab, DcSweepTab, EnvelopeTab, FourierTab, HarmonicBalanceTab, MonteCarloTab,
    MultiRateTab, NoiseTab, OpTab, PacTab, ParametricTab, PoleZeroTab, PssTab, SParamTab,
    SensitivityTab, StbTab, TransferTab, TransientTab,
};
use super::types::{AnalysisTab, SimulationDialogProps};
use crate::state::simulation_command::{
    parse_spice_value, AcConfig, AcSweepType, CornerConfig, DcSweepConfig, EnvelopeConfig,
    FourierConfig, HbConfig, HbToneSpec, McDistribution, ModulationType, MonteCarloConfig,
    MultiRateConfig, NoiseConfig, OpConfig, PacConfig, ParametricConfig, ParametricStepType,
    PoleZeroConfig, ProcessCorner, PssConfig, PzTransferType, SParamConfig, SensitivityConfig,
    SimulationConfig, StbConfig, StbSweepType, TransferConfig, TransientConfig,
};
use crate::theme::Theme;

/// Simulation setup dialog with tabbed interface
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

    // Resizable dialog state
    let mut dialog_size = use_signal(|| (560.0_f64, 650.0_f64)); // (width, height)
    let mut resize_edge: Signal<ResizeEdge> = use_signal(|| ResizeEdge::None);
    let mut resize_start_mouse = use_signal(|| (0.0_f64, 0.0_f64));
    let mut resize_start_size = use_signal(|| (0.0_f64, 0.0_f64));
    let mut resize_start_pos = use_signal(|| (0.0_f64, 0.0_f64));

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

    // ==========================================================================
    // PSS (Periodic Steady State) parameters
    // ==========================================================================
    let pss_enabled = use_signal(|| props.config.pss.is_some());
    let pss_fundamental = use_signal(|| {
        format_value(
            props
                .config
                .pss
                .as_ref()
                .map(|p| p.fundamental_freq)
                .unwrap_or(1e6),
        )
    });
    let pss_auto_detect = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| p.auto_detect_period)
            .unwrap_or(true)
    });
    let pss_harmonics = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| p.num_harmonics)
            .unwrap_or(10)
            .to_string()
    });
    let pss_stab_cycles = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| p.stabilization_cycles)
            .unwrap_or(3)
            .to_string()
    });
    let pss_tolerance = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| format!("{:e}", p.tolerance))
            .unwrap_or("1e-6".to_string())
    });
    let pss_max_iter = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| p.max_iterations)
            .unwrap_or(50)
            .to_string()
    });
    let pss_floquet = use_signal(|| {
        props
            .config
            .pss
            .as_ref()
            .map(|p| p.floquet_analysis)
            .unwrap_or(false)
    });

    // ==========================================================================
    // PAC (Periodic AC) parameters
    // ==========================================================================
    let pac_enabled = use_signal(|| props.config.pac.is_some());
    let pac_start = use_signal(|| {
        format_value(
            props
                .config
                .pac
                .as_ref()
                .map(|p| p.start_freq)
                .unwrap_or(1.0),
        )
    });
    let pac_stop = use_signal(|| {
        format_value(
            props
                .config
                .pac
                .as_ref()
                .map(|p| p.stop_freq)
                .unwrap_or(1e9),
        )
    });
    let pac_pts = use_signal(|| {
        props
            .config
            .pac
            .as_ref()
            .map(|p| p.points_per_decade)
            .unwrap_or(10)
            .to_string()
    });
    let pac_sidebands = use_signal(|| {
        props
            .config
            .pac
            .as_ref()
            .map(|p| p.max_sidebands)
            .unwrap_or(5)
            .to_string()
    });
    let pac_sweep_type = use_signal(|| {
        props
            .config
            .pac
            .as_ref()
            .map(|p| p.sweep_type)
            .unwrap_or_default()
    });

    // ==========================================================================
    // Harmonic Balance parameters
    // ==========================================================================
    let hb_enabled = use_signal(|| props.config.harmonic_balance.is_some());
    let hb_tone1_freq = use_signal(|| {
        format_value(
            props
                .config
                .harmonic_balance
                .as_ref()
                .and_then(|h| h.tones.first())
                .map(|t| t.frequency)
                .unwrap_or(1e9),
        )
    });
    let hb_tone1_harmonics = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .and_then(|h| h.tones.first())
            .map(|t| t.harmonics)
            .unwrap_or(7)
            .to_string()
    });
    let hb_tone1_source = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .and_then(|h| h.tones.first())
            .map(|t| t.source.clone())
            .unwrap_or("V1".to_string())
    });
    let hb_tone2_enabled = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| h.tones.len() > 1)
            .unwrap_or(false)
    });
    let hb_tone2_freq = use_signal(|| {
        format_value(
            props
                .config
                .harmonic_balance
                .as_ref()
                .and_then(|h| h.tones.get(1))
                .map(|t| t.frequency)
                .unwrap_or(1.001e9),
        )
    });
    let hb_tone2_harmonics = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .and_then(|h| h.tones.get(1))
            .map(|t| t.harmonics)
            .unwrap_or(3)
            .to_string()
    });
    let hb_tone2_source = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .and_then(|h| h.tones.get(1))
            .map(|t| t.source.clone())
            .unwrap_or("V2".to_string())
    });
    let hb_max_order = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| h.max_order)
            .unwrap_or(7)
            .to_string()
    });
    let hb_tolerance = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| format!("{:e}", h.tolerance))
            .unwrap_or("1e-6".to_string())
    });
    let hb_max_iter = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| h.max_iterations)
            .unwrap_or(200)
            .to_string()
    });
    let hb_krylov = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| h.use_krylov)
            .unwrap_or(true)
    });
    let hb_oversample = use_signal(|| {
        props
            .config
            .harmonic_balance
            .as_ref()
            .map(|h| h.oversample)
            .unwrap_or(4)
            .to_string()
    });

    // ==========================================================================
    // STB (Loop Stability) parameters
    // ==========================================================================
    let stb_enabled = use_signal(|| props.config.stb.is_some());
    let stb_probe_pos = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.probe_pos.clone())
            .unwrap_or("out".to_string())
    });
    let stb_probe_neg = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.probe_neg.clone())
            .unwrap_or("0".to_string())
    });
    let stb_start = use_signal(|| {
        format_value(
            props
                .config
                .stb
                .as_ref()
                .map(|s| s.start_freq)
                .unwrap_or(1.0),
        )
    });
    let stb_stop = use_signal(|| {
        format_value(
            props
                .config
                .stb
                .as_ref()
                .map(|s| s.stop_freq)
                .unwrap_or(100e6),
        )
    });
    let stb_pts = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.points_per_decade)
            .unwrap_or(20)
            .to_string()
    });
    let stb_pm_target = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.target_phase_margin)
            .unwrap_or(45.0)
            .to_string()
    });
    let stb_gm_target = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.target_gain_margin)
            .unwrap_or(10.0)
            .to_string()
    });
    let stb_nyquist = use_signal(|| {
        props
            .config
            .stb
            .as_ref()
            .map(|s| s.show_nyquist)
            .unwrap_or(false)
    });

    // ==========================================================================
    // Envelope Transient parameters
    // ==========================================================================
    let env_enabled = use_signal(|| props.config.envelope.is_some());
    let env_carrier = use_signal(|| {
        format_value(
            props
                .config
                .envelope
                .as_ref()
                .map(|e| e.carrier_freq)
                .unwrap_or(1e9),
        )
    });
    let env_mod_type = use_signal(|| {
        props
            .config
            .envelope
            .as_ref()
            .map(|e| e.modulation_type)
            .unwrap_or_default()
    });
    let env_bandwidth = use_signal(|| {
        format_value(
            props
                .config
                .envelope
                .as_ref()
                .map(|e| e.envelope_bandwidth)
                .unwrap_or(10e6),
        )
    });
    let env_stop = use_signal(|| {
        format_value(
            props
                .config
                .envelope
                .as_ref()
                .map(|e| e.stop_time)
                .unwrap_or(1e-6),
        )
    });
    let env_max_step = use_signal(|| {
        props
            .config
            .envelope
            .as_ref()
            .and_then(|e| e.max_step)
            .map(format_value)
            .unwrap_or_default()
    });
    let env_harmonics = use_signal(|| {
        props
            .config
            .envelope
            .as_ref()
            .map(|e| e.carrier_harmonics)
            .unwrap_or(3)
            .to_string()
    });

    // ==========================================================================
    // Multi-Rate parameters
    // ==========================================================================
    let mr_enabled = use_signal(|| props.config.multi_rate.is_some());
    let mr_auto = use_signal(|| {
        props
            .config
            .multi_rate
            .as_ref()
            .map(|m| m.auto_partition)
            .unwrap_or(true)
    });
    let mr_fast = use_signal(|| {
        format_value(
            props
                .config
                .multi_rate
                .as_ref()
                .map(|m| m.fast_rate)
                .unwrap_or(1e9),
        )
    });
    let mr_slow = use_signal(|| {
        format_value(
            props
                .config
                .multi_rate
                .as_ref()
                .map(|m| m.slow_rate)
                .unwrap_or(1e6),
        )
    });
    let mr_stop = use_signal(|| {
        format_value(
            props
                .config
                .multi_rate
                .as_ref()
                .map(|m| m.stop_time)
                .unwrap_or(1e-3),
        )
    });
    let mr_latency = use_signal(|| {
        format_value(
            props
                .config
                .multi_rate
                .as_ref()
                .map(|m| m.latency_tolerance)
                .unwrap_or(1e-12),
        )
    });

    // ==========================================================================
    // Corner (PVT) parameters
    // ==========================================================================
    let corner_enabled = use_signal(|| props.config.corner.is_some());
    let corner_process = use_signal(|| {
        props
            .config
            .corner
            .as_ref()
            .map(|c| c.process_corners.clone())
            .unwrap_or_default()
    });
    let corner_voltage = use_signal(|| {
        props
            .config
            .corner
            .as_ref()
            .map(|c| {
                c.voltage_corners
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or("90, 100, 110".to_string())
    });
    let corner_temp = use_signal(|| {
        props
            .config
            .corner
            .as_ref()
            .map(|c| {
                c.temperature_corners
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or("-40, 25, 125".to_string())
    });
    let corner_full_matrix = use_signal(|| {
        props
            .config
            .corner
            .as_ref()
            .map(|c| c.full_matrix)
            .unwrap_or(false)
    });

    // ==========================================================================
    // Transfer Function parameters
    // ==========================================================================
    let tf_enabled = use_signal(|| props.config.transfer.is_some());
    let tf_output = use_signal(|| {
        props
            .config
            .transfer
            .as_ref()
            .map(|t| t.output_var.clone())
            .unwrap_or("V(out)".to_string())
    });
    let tf_input = use_signal(|| {
        props
            .config
            .transfer
            .as_ref()
            .map(|t| t.input_source.clone())
            .unwrap_or("Vin".to_string())
    });

    // ==========================================================================
    // Fourier/THD parameters
    // ==========================================================================
    let fourier_enabled = use_signal(|| props.config.fourier.is_some());
    let fourier_freq = use_signal(|| {
        format_value(
            props
                .config
                .fourier
                .as_ref()
                .map(|f| f.fundamental_freq)
                .unwrap_or(1e3),
        )
    });
    let fourier_harmonics = use_signal(|| {
        props
            .config
            .fourier
            .as_ref()
            .map(|f| f.num_harmonics)
            .unwrap_or(10)
            .to_string()
    });
    let fourier_output = use_signal(|| {
        props
            .config
            .fourier
            .as_ref()
            .map(|f| f.output_var.clone())
            .unwrap_or("V(out)".to_string())
    });
    let fourier_thd = use_signal(|| {
        props
            .config
            .fourier
            .as_ref()
            .map(|f| f.calculate_thd)
            .unwrap_or(true)
    });

    // ==========================================================================
    // Parametric Sweep parameters
    // ==========================================================================
    let param_enabled = use_signal(|| props.config.parametric.is_some());
    let param_name = use_signal(|| {
        props
            .config
            .parametric
            .as_ref()
            .map(|p| p.param_name.clone())
            .unwrap_or("R1".to_string())
    });
    let param_start = use_signal(|| {
        format_value(
            props
                .config
                .parametric
                .as_ref()
                .map(|p| p.start_value)
                .unwrap_or(1e3),
        )
    });
    let param_stop = use_signal(|| {
        format_value(
            props
                .config
                .parametric
                .as_ref()
                .map(|p| p.stop_value)
                .unwrap_or(10e3),
        )
    });
    let param_step_type = use_signal(|| {
        props
            .config
            .parametric
            .as_ref()
            .map(|p| p.step_type)
            .unwrap_or_default()
    });
    let param_steps = use_signal(|| {
        props
            .config
            .parametric
            .as_ref()
            .map(|p| p.num_steps)
            .unwrap_or(10)
            .to_string()
    });
    let param_list = use_signal(|| {
        props
            .config
            .parametric
            .as_ref()
            .map(|p| {
                p.values
                    .iter()
                    .map(|v| format_value(*v))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
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

        // PSS
        if *pss_enabled.read() {
            config.pss = Some(PssConfig {
                fundamental_freq: parse_spice_value(&pss_fundamental.read()).unwrap_or(1e6),
                auto_detect_period: *pss_auto_detect.read(),
                num_harmonics: pss_harmonics.read().parse().unwrap_or(10),
                stabilization_cycles: pss_stab_cycles.read().parse().unwrap_or(3),
                tolerance: parse_spice_value(&pss_tolerance.read()).unwrap_or(1e-6),
                max_iterations: pss_max_iter.read().parse().unwrap_or(50),
                floquet_analysis: *pss_floquet.read(),
            });
        }

        // PAC
        if *pac_enabled.read() {
            config.pac = Some(PacConfig {
                start_freq: parse_spice_value(&pac_start.read()).unwrap_or(1.0),
                stop_freq: parse_spice_value(&pac_stop.read()).unwrap_or(1e9),
                points_per_decade: pac_pts.read().parse().unwrap_or(10),
                sweep_type: *pac_sweep_type.read(),
                sideband: 0, // Fundamental
                max_sidebands: pac_sidebands.read().parse().unwrap_or(5),
            });
        }

        // Harmonic Balance
        if *hb_enabled.read() {
            let mut tones = vec![HbToneSpec {
                frequency: parse_spice_value(&hb_tone1_freq.read()).unwrap_or(1e9),
                harmonics: hb_tone1_harmonics.read().parse().unwrap_or(7),
                source: hb_tone1_source.read().clone(),
            }];
            if *hb_tone2_enabled.read() {
                tones.push(HbToneSpec {
                    frequency: parse_spice_value(&hb_tone2_freq.read()).unwrap_or(1.001e9),
                    harmonics: hb_tone2_harmonics.read().parse().unwrap_or(3),
                    source: hb_tone2_source.read().clone(),
                });
            }
            config.harmonic_balance = Some(HbConfig {
                tones,
                max_order: hb_max_order.read().parse().unwrap_or(7),
                tolerance: parse_spice_value(&hb_tolerance.read()).unwrap_or(1e-6),
                max_iterations: hb_max_iter.read().parse().unwrap_or(200),
                use_krylov: *hb_krylov.read(),
                oversample: hb_oversample.read().parse().unwrap_or(4),
            });
        }

        // STB
        if *stb_enabled.read() {
            config.stb = Some(StbConfig {
                probe_pos: stb_probe_pos.read().clone(),
                probe_neg: stb_probe_neg.read().clone(),
                start_freq: parse_spice_value(&stb_start.read()).unwrap_or(1.0),
                stop_freq: parse_spice_value(&stb_stop.read()).unwrap_or(100e6),
                points_per_decade: stb_pts.read().parse().unwrap_or(20),
                sweep_type: StbSweepType::Frequency,
                target_phase_margin: stb_pm_target.read().parse().unwrap_or(45.0),
                target_gain_margin: stb_gm_target.read().parse().unwrap_or(10.0),
                show_nyquist: *stb_nyquist.read(),
            });
        }

        // Envelope
        if *env_enabled.read() {
            config.envelope = Some(EnvelopeConfig {
                carrier_freq: parse_spice_value(&env_carrier.read()).unwrap_or(1e9),
                modulation_type: *env_mod_type.read(),
                envelope_bandwidth: parse_spice_value(&env_bandwidth.read()).unwrap_or(10e6),
                stop_time: parse_spice_value(&env_stop.read()).unwrap_or(1e-6),
                max_step: parse_spice_value(&env_max_step.read()),
                carrier_harmonics: env_harmonics.read().parse().unwrap_or(3),
            });
        }

        // Multi-Rate
        if *mr_enabled.read() {
            config.multi_rate = Some(MultiRateConfig {
                auto_partition: *mr_auto.read(),
                fast_rate: parse_spice_value(&mr_fast.read()).unwrap_or(1e9),
                slow_rate: parse_spice_value(&mr_slow.read()).unwrap_or(1e6),
                stop_time: parse_spice_value(&mr_stop.read()).unwrap_or(1e-3),
                latency_tolerance: parse_spice_value(&mr_latency.read()).unwrap_or(1e-12),
            });
        }

        // Corner (PVT)
        if *corner_enabled.read() {
            // Parse voltage corners from comma-separated string
            let voltages: Vec<f64> = corner_voltage
                .read()
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            // Parse temperature corners from comma-separated string
            let temps: Vec<f64> = corner_temp
                .read()
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            config.corner = Some(CornerConfig {
                process_corners: corner_process.read().clone(),
                voltage_corners: if voltages.is_empty() {
                    vec![90.0, 100.0, 110.0]
                } else {
                    voltages
                },
                temperature_corners: if temps.is_empty() {
                    vec![-40.0, 25.0, 125.0]
                } else {
                    temps
                },
                full_matrix: *corner_full_matrix.read(),
            });
        }

        // Transfer Function
        if *tf_enabled.read() {
            config.transfer = Some(TransferConfig {
                output_var: tf_output.read().clone(),
                input_source: tf_input.read().clone(),
            });
        }

        // Fourier
        if *fourier_enabled.read() {
            config.fourier = Some(FourierConfig {
                fundamental_freq: parse_spice_value(&fourier_freq.read()).unwrap_or(1e3),
                num_harmonics: fourier_harmonics.read().parse().unwrap_or(10),
                output_var: fourier_output.read().clone(),
                calculate_thd: *fourier_thd.read(),
            });
        }

        // Parametric Sweep
        if *param_enabled.read() {
            let values: Vec<f64> = param_list
                .read()
                .split_whitespace()
                .filter_map(|s| parse_spice_value(s))
                .collect();
            config.parametric = Some(ParametricConfig {
                param_name: param_name.read().clone(),
                start_value: parse_spice_value(&param_start.read()).unwrap_or(1e3),
                stop_value: parse_spice_value(&param_stop.read()).unwrap_or(10e3),
                step_type: *param_step_type.read(),
                num_steps: param_steps.read().parse().unwrap_or(10),
                values,
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
    let (dialog_w, dialog_h) = *dialog_size.read();

    // Size constraints
    let min_w = 450.0_f64;
    let min_h = 400.0_f64;
    let max_w = 1200.0_f64;
    let max_h = 900.0_f64;

    // Read enabled states for sidebar buttons
    let is_tran_enabled = *transient_enabled.read();
    let is_ac_enabled = *ac_enabled.read();
    let is_dc_enabled = *dc_enabled.read();
    let is_op_enabled = *op_enabled.read();
    let is_noise_enabled = *noise_enabled.read();
    let is_mc_enabled = *mc_enabled.read();
    let is_pz_enabled = *pz_enabled.read();
    let is_sens_enabled = *sens_enabled.read();
    let is_sp_enabled = *sp_enabled.read();
    let is_pss_enabled = *pss_enabled.read();
    let is_pac_enabled = *pac_enabled.read();
    let is_hb_enabled = *hb_enabled.read();
    let is_stb_enabled = *stb_enabled.read();
    let is_env_enabled = *env_enabled.read();
    let is_mr_enabled = *mr_enabled.read();
    let is_corner_enabled = *corner_enabled.read();
    let is_tf_enabled = *tf_enabled.read();
    let is_fourier_enabled = *fourier_enabled.read();
    let is_param_enabled = *param_enabled.read();
    let current_tab = *active_tab.read();

    // Check if we're currently interacting (drag or resize)
    let is_interacting = *dragging.read() || *resize_edge.read() != ResizeEdge::None;

    rsx! {
        // Full-screen capture overlay (only during drag/resize to capture mouse everywhere)
        if is_interacting {
            div {
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9999; cursor: inherit;",
                onmousemove: move |e| {
                    let page = e.page_coordinates();

                    // Handle drag
                    if *dragging.read() {
                        let (ox, oy) = *drag_offset.read();
                        dialog_pos.set((page.x - ox, page.y - oy));
                        return;
                    }

                    // Handle resize
                    let edge = *resize_edge.read();
                    if edge != ResizeEdge::None {
                        let (start_mx, start_my) = *resize_start_mouse.read();
                        let (start_w, start_h) = *resize_start_size.read();
                        let (start_px, start_py) = *resize_start_pos.read();

                        let delta_x = page.x - start_mx;
                        let delta_y = page.y - start_my;

                        let mut new_w = start_w;
                        let mut new_h = start_h;
                        let mut new_px = start_px;
                        let mut new_py = start_py;

                        match edge {
                            ResizeEdge::Right => new_w = start_w + delta_x,
                            ResizeEdge::Left => {
                                new_w = start_w - delta_x;
                                new_px = start_px + delta_x;
                            }
                            ResizeEdge::Bottom => new_h = start_h + delta_y,
                            ResizeEdge::Top => {
                                new_h = start_h - delta_y;
                                new_py = start_py + delta_y;
                            }
                            ResizeEdge::BottomRight => {
                                new_w = start_w + delta_x;
                                new_h = start_h + delta_y;
                            }
                            ResizeEdge::BottomLeft => {
                                new_w = start_w - delta_x;
                                new_h = start_h + delta_y;
                                new_px = start_px + delta_x;
                            }
                            ResizeEdge::TopRight => {
                                new_w = start_w + delta_x;
                                new_h = start_h - delta_y;
                                new_py = start_py + delta_y;
                            }
                            ResizeEdge::TopLeft => {
                                new_w = start_w - delta_x;
                                new_h = start_h - delta_y;
                                new_px = start_px + delta_x;
                                new_py = start_py + delta_y;
                            }
                            ResizeEdge::None => {}
                        }

                        // Clamp size
                        let clamped_w = new_w.clamp(min_w, max_w);
                        let clamped_h = new_h.clamp(min_h, max_h);

                        // Adjust position if clamped
                        if edge.moves_left() && clamped_w != new_w {
                            new_px = start_px + (start_w - clamped_w);
                        }
                        if edge.moves_top() && clamped_h != new_h {
                            new_py = start_py + (start_h - clamped_h);
                        }

                        dialog_size.set((clamped_w, clamped_h));
                        dialog_pos.set((new_px, new_py));
                    }
                },
                onmouseup: move |_| {
                    dragging.set(false);
                    resize_edge.set(ResizeEdge::None);
                },
            }
        }

        // Positioned draggable/resizable dialog
        div {
            style: "position: fixed; left: {pos_x}px; top: {pos_y}px; z-index: 1000;",

            // Dialog container
            div {
                style: "position: relative; display: flex; flex-direction: column; background: {th.bg_secondary()}; border: 1px solid {th.border()}; border-radius: 8px; width: {dialog_w}px; height: {dialog_h}px; overflow: visible; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);",

                // =======================================================
                // RESIZE HANDLES
                // =======================================================

                // Top edge
                div {
                    style: "position: absolute; top: -4px; left: 14px; right: 14px; height: 8px; cursor: n-resize; z-index: 10;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::Top);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Bottom edge
                div {
                    style: "position: absolute; bottom: -4px; left: 14px; right: 14px; height: 8px; cursor: s-resize; z-index: 10;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::Bottom);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Left edge
                div {
                    style: "position: absolute; left: -4px; top: 14px; bottom: 14px; width: 8px; cursor: w-resize; z-index: 10;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::Left);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Right edge
                div {
                    style: "position: absolute; right: -4px; top: 14px; bottom: 14px; width: 8px; cursor: e-resize; z-index: 10;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::Right);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Top-left corner
                div {
                    style: "position: absolute; top: -4px; left: -4px; width: 18px; height: 18px; cursor: nw-resize; z-index: 11;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::TopLeft);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Top-right corner
                div {
                    style: "position: absolute; top: -4px; right: -4px; width: 18px; height: 18px; cursor: ne-resize; z-index: 11;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::TopRight);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Bottom-left corner
                div {
                    style: "position: absolute; bottom: -4px; left: -4px; width: 18px; height: 18px; cursor: sw-resize; z-index: 11;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::BottomLeft);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }
                // Bottom-right corner
                div {
                    style: "position: absolute; bottom: -4px; right: -4px; width: 18px; height: 18px; cursor: se-resize; z-index: 11;",
                    onmousedown: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        resize_edge.set(ResizeEdge::BottomRight);
                        resize_start_mouse.set((e.page_coordinates().x, e.page_coordinates().y));
                        resize_start_size.set((dialog_w, dialog_h));
                        resize_start_pos.set((pos_x, pos_y));
                    },
                }

                // Draggable header
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 10px 16px; border-bottom: 1px solid {th.border()}; background: {th.bg_tertiary()}; cursor: move; user-select: none; flex-shrink: 0;",
                    onmousedown: move |e| {
                        let page = e.page_coordinates();
                        let (px, py) = *dialog_pos.read();
                        drag_offset.set((page.x - px, page.y - py));
                        dragging.set(true);
                    },
                    h2 {
                        style: "margin: 0; font-size: 16px; font-weight: 600; color: {th.text_primary()}; pointer-events: none;",
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
                    style: "display: flex; flex: 1; min-height: 0; overflow: hidden;",

                    // Vertical sidebar navigation
                    div {
                        style: "width: 140px; background: {th.bg_primary()}; border-right: 1px solid {th.border()}; padding: 8px 0; overflow-y: auto;",

                        div {
                            style: "padding: 0 8px 8px 8px; font-size: 10px; font-weight: 600; color: {th.text_muted()}; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Analysis Types"
                        }

                        SidebarButton {
                            label: "Transient",
                            active: current_tab == AnalysisTab::Transient,
                            enabled: is_tran_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Transient),
                        }
                        SidebarButton {
                            label: "AC Analysis",
                            active: current_tab == AnalysisTab::Ac,
                            enabled: is_ac_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Ac),
                        }
                        SidebarButton {
                            label: "DC Sweep",
                            active: current_tab == AnalysisTab::DcSweep,
                            enabled: is_dc_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::DcSweep),
                        }
                        SidebarButton {
                            label: "Op Point",
                            active: current_tab == AnalysisTab::Op,
                            enabled: is_op_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Op),
                        }
                        SidebarButton {
                            label: "Noise",
                            active: current_tab == AnalysisTab::Noise,
                            enabled: is_noise_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Noise),
                        }
                        SidebarButton {
                            label: "Monte Carlo",
                            active: current_tab == AnalysisTab::MonteCarlo,
                            enabled: is_mc_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::MonteCarlo),
                        }
                        SidebarButton {
                            label: "Pole-Zero",
                            active: current_tab == AnalysisTab::PoleZero,
                            enabled: is_pz_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::PoleZero),
                        }
                        SidebarButton {
                            label: "Sensitivity",
                            active: current_tab == AnalysisTab::Sensitivity,
                            enabled: is_sens_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Sensitivity),
                        }
                        SidebarButton {
                            label: "S-Parameters",
                            active: current_tab == AnalysisTab::SParam,
                            enabled: is_sp_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::SParam),
                        }

                        // Divider for RF/Periodic section
                        div {
                            style: "padding: 12px 8px 8px 8px; font-size: 10px; font-weight: 600; color: {th.text_muted()}; text-transform: uppercase; letter-spacing: 0.5px; border-top: 1px solid {th.border()}; margin-top: 8px;",
                            "RF/Periodic"
                        }

                        SidebarButton {
                            label: "PSS",
                            active: current_tab == AnalysisTab::Pss,
                            enabled: is_pss_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Pss),
                        }
                        SidebarButton {
                            label: "PAC",
                            active: current_tab == AnalysisTab::Pac,
                            enabled: is_pac_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Pac),
                        }
                        SidebarButton {
                            label: "HB",
                            active: current_tab == AnalysisTab::HarmonicBalance,
                            enabled: is_hb_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::HarmonicBalance),
                        }
                        SidebarButton {
                            label: "STB",
                            active: current_tab == AnalysisTab::Stb,
                            enabled: is_stb_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Stb),
                        }
                        SidebarButton {
                            label: "Envelope",
                            active: current_tab == AnalysisTab::Envelope,
                            enabled: is_env_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Envelope),
                        }
                        SidebarButton {
                            label: "Multi-Rate",
                            active: current_tab == AnalysisTab::MultiRate,
                            enabled: is_mr_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::MultiRate),
                        }

                        // Divider for PVT/Misc section
                        div {
                            style: "padding: 12px 8px 8px 8px; font-size: 10px; font-weight: 600; color: {th.text_muted()}; text-transform: uppercase; letter-spacing: 0.5px; border-top: 1px solid {th.border()}; margin-top: 8px;",
                            "PVT/Misc"
                        }

                        SidebarButton {
                            label: "Corner",
                            active: current_tab == AnalysisTab::Corner,
                            enabled: is_corner_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Corner),
                        }
                        SidebarButton {
                            label: "Transfer",
                            active: current_tab == AnalysisTab::Transfer,
                            enabled: is_tf_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Transfer),
                        }
                        SidebarButton {
                            label: "Fourier",
                            active: current_tab == AnalysisTab::Fourier,
                            enabled: is_fourier_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Fourier),
                        }
                        SidebarButton {
                            label: "Parametric",
                            active: current_tab == AnalysisTab::Parametric,
                            enabled: is_param_enabled,
                            onclick: move |_| active_tab.set(AnalysisTab::Parametric),
                        }
                    }

                    // Tab content area
                    div {
                        style: "flex: 1; padding: 20px; overflow-y: auto;",

                        match current_tab {
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
                            // Advanced analysis tabs
                            AnalysisTab::Pss => rsx! {
                                PssTab {
                                    enabled: pss_enabled,
                                    fundamental_freq: pss_fundamental,
                                    auto_detect: pss_auto_detect,
                                    num_harmonics: pss_harmonics,
                                    stabilization_cycles: pss_stab_cycles,
                                    tolerance: pss_tolerance,
                                    max_iterations: pss_max_iter,
                                    floquet_analysis: pss_floquet,
                                }
                            },
                            AnalysisTab::Pac => rsx! {
                                PacTab {
                                    enabled: pac_enabled,
                                    start_freq: pac_start,
                                    stop_freq: pac_stop,
                                    points_per_decade: pac_pts,
                                    sweep_type: pac_sweep_type,
                                    max_sidebands: pac_sidebands,
                                }
                            },
                            AnalysisTab::HarmonicBalance => rsx! {
                                HarmonicBalanceTab {
                                    enabled: hb_enabled,
                                    tone1_freq: hb_tone1_freq,
                                    tone1_harmonics: hb_tone1_harmonics,
                                    tone1_source: hb_tone1_source,
                                    tone2_enabled: hb_tone2_enabled,
                                    tone2_freq: hb_tone2_freq,
                                    tone2_harmonics: hb_tone2_harmonics,
                                    tone2_source: hb_tone2_source,
                                    max_order: hb_max_order,
                                    tolerance: hb_tolerance,
                                    max_iterations: hb_max_iter,
                                    use_krylov: hb_krylov,
                                    oversample: hb_oversample,
                                }
                            },
                            AnalysisTab::Stb => rsx! {
                                StbTab {
                                    enabled: stb_enabled,
                                    probe_pos: stb_probe_pos,
                                    probe_neg: stb_probe_neg,
                                    start_freq: stb_start,
                                    stop_freq: stb_stop,
                                    points_per_decade: stb_pts,
                                    target_phase_margin: stb_pm_target,
                                    target_gain_margin: stb_gm_target,
                                    show_nyquist: stb_nyquist,
                                }
                            },
                            AnalysisTab::Envelope => rsx! {
                                EnvelopeTab {
                                    enabled: env_enabled,
                                    carrier_freq: env_carrier,
                                    modulation_type: env_mod_type,
                                    envelope_bandwidth: env_bandwidth,
                                    stop_time: env_stop,
                                    max_step: env_max_step,
                                    carrier_harmonics: env_harmonics,
                                }
                            },
                            AnalysisTab::MultiRate => rsx! {
                                MultiRateTab {
                                    enabled: mr_enabled,
                                    auto_partition: mr_auto,
                                    fast_rate: mr_fast,
                                    slow_rate: mr_slow,
                                    stop_time: mr_stop,
                                    latency_tolerance: mr_latency,
                                }
                            },
                            AnalysisTab::Corner => rsx! {
                                CornerTab {
                                    enabled: corner_enabled,
                                    process_corners: corner_process,
                                    voltage_corners: corner_voltage,
                                    temperature_corners: corner_temp,
                                    full_matrix: corner_full_matrix,
                                }
                            },
                            AnalysisTab::Transfer => rsx! {
                                TransferTab {
                                    enabled: tf_enabled,
                                    output_var: tf_output,
                                    input_source: tf_input,
                                }
                            },
                            AnalysisTab::Fourier => rsx! {
                                FourierTab {
                                    enabled: fourier_enabled,
                                    fundamental_freq: fourier_freq,
                                    num_harmonics: fourier_harmonics,
                                    output_var: fourier_output,
                                    calculate_thd: fourier_thd,
                                }
                            },
                            AnalysisTab::Parametric => rsx! {
                                ParametricTab {
                                    enabled: param_enabled,
                                    param_name: param_name,
                                    start_value: param_start,
                                    stop_value: param_stop,
                                    step_type: param_step_type,
                                    num_steps: param_steps,
                                    values_list: param_list,
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
                    style: "display: flex; justify-content: flex-end; gap: 10px; padding: 12px 16px; border-top: 1px solid {th.border()}; background: {th.bg_tertiary()};",
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
