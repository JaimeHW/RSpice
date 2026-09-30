//! Lower authored analysis drafts into exact engine-facing specifications.

use super::helpers::map_ac_sweep;
use rspice_simulation_contract::analysis_spec::{
    AnalysisSpec, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort,
};
use rspice_simulation_contract::config::FrequencySweep;
use rspice_simulation_contract::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    NoiseSweepType, PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use rspice_simulation_contract::setup_state::SimulationSetup;

pub fn analysis_spec_to_config(spec: &AnalysisSpec) -> Result<AnalysisConfig, String> {
    match spec {
        AnalysisSpec::LegacyDcOp => Ok(AnalysisConfig::dc_op()),
        AnalysisSpec::DcOp {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            previous_state,
            violation_devices,
            violation_source_content_digest,
            run_point,
        } => Ok(AnalysisConfig::DcOp(
            rspice_simulation_contract::config::OpConfig {
                temperature_mode: *temperature_mode,
                temperature_celsius: *temperature_celsius,
                initial_guess: *initial_guess,
                node_initialization: *node_initialization,
                homotopy: *homotopy,
                annotation: *annotation,
                device_detail: *device_detail,
                save_device_op: *save_device_op,
                accuracy: *accuracy,
                selected_devices: selected_devices.clone(),
                previous_state: previous_state.clone(),
                violation_devices: violation_devices.clone(),
                violation_source_content_digest: *violation_source_content_digest,
                run_point: run_point.clone(),
            },
        )),
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
            hysteresis,
            modes,
        } => Ok(AnalysisConfig::DcSweep(DcSweepConfig {
            source: source_name.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            source2: source2.clone(),
            start2: *start2,
            stop2: *stop2,
            step2: *step2,
            hysteresis: *hysteresis,
            modes: modes.clone(),
        })),
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => Ok(AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: *start_freq,
            stop_freq: *stop_freq,
            num_points: *points_per_unit,
            sweep_type: map_ac_sweep(*sweep),
        })),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => Ok(AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: *start_time,
            max_timestep: *max_timestep,
            uic: *uic,
        })),
        AnalysisSpec::Noise {
            output_node,
            reference_node,
            input_source,
            start_freq,
            stop_freq,
            points_per_decade,
            sweep,
            explicit_frequencies,
            data_table_name,
            contribution_detail,
            integration_mode,
            temperature,
        } => Ok(AnalysisConfig::Noise(NoiseAnalysisConfig {
            output_node: output_node.clone(),
            reference_node: reference_node.clone(),
            input_source: input_source.clone(),
            sweep_type: match sweep {
                NoiseSweepType::Decade | NoiseSweepType::ExplicitFrequencyList => {
                    AcSweepType::Decade
                }
                NoiseSweepType::Octave => AcSweepType::Octave,
                NoiseSweepType::Linear => AcSweepType::Linear,
                NoiseSweepType::Unsupported(index) => {
                    return Err(format!(
                        "noise sweep mode {index} is outside the supported schema"
                    ));
                }
            },
            num_points: *points_per_decade,
            start_freq: *start_freq,
            stop_freq: *stop_freq,
            explicit_frequencies: explicit_frequencies.clone(),
            data_table_name: data_table_name.clone(),
            contribution_detail: *contribution_detail,
            integration_mode: *integration_mode,
            temperature_kelvin: *temperature,
        })),
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => {
            let analysis_type = match analysis_type.trim().to_ascii_uppercase().as_str() {
                "PZ" => PzAnalysisType::PoleZero,
                "POL" => PzAnalysisType::PolesOnly,
                "ZER" => PzAnalysisType::ZerosOnly,
                other => {
                    return Err(format!(
                        "invalid pole-zero analysis type '{}': expected PZ, POL, or ZER",
                        other
                    ));
                }
            };
            let transfer_type = transfer_type.trim().to_ascii_uppercase();
            if transfer_type != "VOL" && transfer_type != "CUR" {
                return Err(format!(
                    "invalid pole-zero transfer type '{}': expected VOL or CUR",
                    transfer_type
                ));
            }
            Ok(AnalysisConfig::PoleZero(PoleZeroConfig {
                input_node: input_node.clone(),
                input_ref: input_ref.clone(),
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                transfer_type,
                analysis_type,
            }))
        }
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
            filter,
            sweep,
        } => Ok(AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: output_var.clone(),
            ac_mode: *ac_mode,
            frequency: *frequency,
            filter: filter.clone(),
            sweep: sweep.map(rspice_simulation_contract::config::SensitivitySweep::from_spec),
        })),
        _ => Err(format!(
            "{} runs through the spec-driven simulation path and cannot be converted to a legacy analysis config",
            spec.run_type().display_name()
        )),
    }
}

pub fn build_pss_spec(
    draft: &rspice_simulation_contract::pss_draft::PssDialogState,
) -> Result<AnalysisSpec, String> {
    let mut pss_state = draft.clone();
    pss_state.ensure_initialized();
    let pss_cfg = pss_state
        .to_config()
        .map_err(|e| format!("invalid PSS settings: {}", e))?;
    Ok(AnalysisSpec::Pss {
        // The editor builds shooting requests and nothing else. The
        // legacy harmonic-balance formulation is still a variant so a
        // sealed manifest that named it opens and refuses by name; no
        // control reaches it.
        method: PssMethod::Shooting,
        fundamental_freq: pss_cfg.fund_freq,
        tone_sources: pss_cfg.tone_sources,
        tstab_periods: pss_cfg.tstab_periods,
        points_per_period: pss_cfg.points_per_period,
        tolerance: pss_cfg.tolerance,
        oscillator_mode: pss_cfg.osc_mode,
        oscillator_node: pss_cfg.osc_mode.then(|| pss_cfg.osc_node.trim().to_owned()),
        num_harmonics: pss_cfg.num_harmonics,
        integration_method: pss_cfg.integration_method,
        tstab: pss_cfg.tstab,
        max_iterations: pss_cfg.max_iterations,
        abstol: pss_cfg.abstol,
        damping: pss_cfg.damping,
        max_period_change: pss_cfg.max_period_change,
        verbose: pss_cfg.verbose,
    })
}

pub fn build_stb_spec(
    draft: &rspice_simulation_contract::stb_draft::StbDialogState,
) -> Result<AnalysisSpec, String> {
    let mut stb_state = draft.clone();
    stb_state.ensure_initialized();
    let stb_cfg = stb_state
        .to_config()
        .map_err(|e| format!("invalid STB settings: {}", e))?;
    Ok(AnalysisSpec::Stb {
        probe_node: stb_cfg.probe_source,
        start_freq: stb_cfg.start_freq,
        stop_freq: stb_cfg.stop_freq,
        sweep: match stb_cfg.sweep_type {
            rspice_simulation_contract::stb_draft::StbSweepType::Decade => FrequencySweep::Decade,
            rspice_simulation_contract::stb_draft::StbSweepType::Octave => FrequencySweep::Octave,
            rspice_simulation_contract::stb_draft::StbSweepType::Linear => FrequencySweep::Linear,
        },
        points_per_decade: stb_cfg.num_points as usize,
        compute_nyquist: stb_cfg.compute_nyquist,
    })
}

pub fn build_harmonic_balance_spec(
    draft: &rspice_simulation_contract::hb_draft::HbDialogState,
) -> Result<AnalysisSpec, String> {
    let mut hb_state = draft.clone();
    hb_state.ensure_initialized();
    let hb_cfg = hb_state
        .to_config()
        .map_err(|e| format!("invalid harmonic balance settings: {}", e))?;
    let mut tones = Vec::with_capacity(1 + hb_cfg.additional_tones.len());
    // The primary tone's label. Fixed, because nothing authors one: the
    // form has no name row and the additional tones number themselves
    // from this.
    let mut primary_tone = HbToneSpec::new(hb_cfg.fundamental_freq, hb_cfg.num_harmonics as usize)
        .with_name("tone1".to_string());
    if let Some(source) = hb_cfg
        .fundamental_source
        .as_deref()
        .map(str::trim)
        .filter(|source| !source.is_empty())
    {
        primary_tone = primary_tone.with_source(source.to_string());
    }
    tones.push(primary_tone);
    for (idx, tone) in hb_cfg.additional_tones.iter().enumerate() {
        let label = if tone.name.trim().is_empty() {
            format!("tone{}", idx + 2)
        } else {
            tone.name.clone()
        };
        let mut tone_spec =
            HbToneSpec::new(tone.frequency, tone.harmonics as usize).with_name(label);
        if let Some(source) = tone
            .source
            .as_deref()
            .map(str::trim)
            .filter(|source| !source.is_empty())
        {
            tone_spec = tone_spec.with_source(source.to_string());
        }
        tones.push(tone_spec);
    }
    Ok(AnalysisSpec::HarmonicBalance {
        tones,
        reltol: hb_cfg.reltol,
        abstol: hb_cfg.abstol,
        max_iterations: hb_cfg.maxiter as usize,
        damping: hb_cfg.damping,
        min_damping: hb_cfg.min_damping,
        oversample: hb_cfg.oversample as usize,
        collocation_points: hb_cfg.collocation_points.map(|points| points as usize),
        max_mixing_order: hb_cfg.max_mixing_order as usize,
        use_krylov: matches!(
            hb_cfg.solver,
            rspice_simulation_contract::hb_draft::HbSolverType::Krylov
        ),
        gmres_restart: hb_cfg.gmres_restart as usize,
        source_stepping: hb_cfg.source_stepping,
        use_exact_jacobian: hb_cfg.use_exact_jacobian,
        verbose: hb_cfg.verbose,
    })
}

/// Validate the form's port source and prepare its configured fallback.
/// Circuit elaboration resolves authored ports, including hierarchy, and
/// the resulting scattering data carries the references used by the solver.
pub fn build_sp_spec(
    schematic: &rspice_design::schematic::document::SchematicDocument,
    draft: &rspice_simulation_contract::sp_draft::SpDialogState,
) -> Result<AnalysisSpec, String> {
    let mut sp_state = draft.clone();
    sp_state.ensure_initialized();
    let ports = rspice_design::rf_ports::rf_ports(schematic);
    let placed: Vec<_> = ports
        .iter()
        .map(|port| rspice_simulation_contract::sp_draft::SpPlacedPort {
            reference: &port.reference,
            port_number: port.port_number,
            z0: &port.z0,
            nets: &port.nets,
        })
        .collect();
    let sp_cfg = sp_state
        .to_config(Some(&placed))
        .map_err(|e| format!("invalid S-parameter settings: {}", e))?;
    let ports = sp_cfg
        .ports
        .iter()
        .map(|port| SpPort {
            node_pos: port.node_pos.clone(),
            node_neg: port.node_neg.clone(),
            z0: port.z0,
        })
        .collect();
    Ok(AnalysisSpec::SParameter {
        start_freq: sp_cfg.start_freq,
        stop_freq: sp_cfg.stop_freq,
        points_per_unit: sp_cfg.num_points as usize,
        sweep: match sp_cfg.sweep_type {
            rspice_simulation_contract::sp_config::SpSweepType::Decade => FrequencySweep::Decade,
            rspice_simulation_contract::sp_config::SpSweepType::Octave => FrequencySweep::Octave,
            rspice_simulation_contract::sp_config::SpSweepType::Linear => FrequencySweep::Linear,
        },
        z0: sp_cfg.z0,
        ports,
        do_noise: sp_cfg.do_noise,
    })
}

pub fn build_envelope_spec(
    draft: &rspice_simulation_contract::envelope_draft::EnvelopeDialogState,
) -> Result<AnalysisSpec, String> {
    let mut envelope_state = draft.clone();
    envelope_state.ensure_initialized();
    let envelope_cfg = envelope_state
        .to_config()
        .map_err(|e| format!("invalid envelope settings: {}", e))?;
    let (fundamental_freq, additional_carrier_tones) = envelope_cfg
        .carrier_tones
        .split_first()
        .map(|(first, additional)| (*first, additional.to_vec()))
        .ok_or_else(|| "invalid envelope settings: carrier tone list is empty".to_owned())?;
    Ok(AnalysisSpec::Envelope {
        multirate: envelope_cfg.multirate,
        initialization: envelope_cfg.initialization,
        fundamental_freq,
        additional_carrier_tones,
        stop_time: envelope_cfg.stop_time,
        num_harmonics: envelope_cfg.harmonic_order as usize,
        envelope_step: Some(envelope_cfg.envelope_step),
        modulation_sources: envelope_cfg.modulation_sources,
        initial_periodic_solve: envelope_cfg.initial_periodic_solve,
        adaptive_mode: envelope_cfg.adaptive_mode,
        extraction_path: envelope_cfg.extraction_path,
    })
}

pub fn build_fourier_spec(
    draft: &rspice_simulation_contract::fourier_draft::FourierDialogState,
) -> Result<AnalysisSpec, String> {
    let mut fourier_state = draft.clone();
    fourier_state.ensure_initialized();
    let fourier_cfg = fourier_state
        .to_config()
        .map_err(|e| format!("invalid Fourier settings: {}", e))?;
    Ok(AnalysisSpec::Fourier {
        fundamental_freq: fourier_cfg.fundamental_freq,
        num_harmonics: fourier_cfg.num_harmonics as usize,
        num_periods: fourier_cfg.num_periods as usize,
        output_node: fourier_cfg.output_node.clone(),
        output_ref: fourier_cfg.output_ref.clone(),
        additional_outputs: fourier_cfg.additional_outputs.clone(),
        start_time: fourier_cfg.start_time,
        stop_time: fourier_cfg.stop_time,
        compute_thd: fourier_cfg.compute_thd,
        normalize: fourier_cfg.normalize,
    })
}

pub fn build_optimization_spec(
    sim_setup: &SimulationSetup,
    plan_payloads: &[rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord],
    draft: &rspice_simulation_contract::optimization_draft::OptimizationDialogState,
) -> Result<AnalysisSpec, String> {
    let mut optimization_state = draft.clone();
    optimization_state.ensure_initialized();
    let cfg = optimization_state
        .to_config()
        .map_err(|e| format!("invalid optimization settings: {}", e))?;

    reject_optimization_of_fixed_design_variables(sim_setup, plan_payloads, &cfg.variables)?;

    Ok(AnalysisSpec::Optimization {
        search: cfg.search,
        variables: cfg
            .variables
            .into_iter()
            .map(|var| OptimizationVariable {
                name: var.name,
                min: var.min,
                max: var.max,
                initial: var.initial,
            })
            .collect(),
        objective_unit: cfg.objective_unit,
        objective_expression: cfg.objective_expression,
        objective_node: cfg.objective_node,
        objective_ref: cfg.objective_ref,
        goal: match cfg.goal_mode {
            rspice_simulation_contract::optimization_draft::OptimizationGoalMode::Minimize => {
                OptimizationGoal::Minimize
            }
            rspice_simulation_contract::optimization_draft::OptimizationGoalMode::Maximize => {
                OptimizationGoal::Maximize
            }
            rspice_simulation_contract::optimization_draft::OptimizationGoalMode::Target => {
                OptimizationGoal::Target
            }
        },
        target: cfg.target_value,
        algorithm: match cfg.algorithm {
            rspice_simulation_contract::optimization_draft::OptimizationAlgorithmMode::GradientDescent => {
                OptimizationAlgorithm::GradientDescent
            }
            rspice_simulation_contract::optimization_draft::OptimizationAlgorithmMode::PatternSearch => {
                OptimizationAlgorithm::PatternSearch
            }
            rspice_simulation_contract::optimization_draft::OptimizationAlgorithmMode::SimulatedAnnealing => {
                OptimizationAlgorithm::SimulatedAnnealing
            }
        },
        max_iterations: cfg.max_iterations,
        cost_tolerance: cfg.cost_tolerance,
        fd_step: cfg.fd_step,
        initial_step: cfg.initial_step,
        min_step: cfg.min_step,
    })
}

/// Refuse to optimize a variable its owner declared fixed.
///
/// The sweep role on the Variables page is the designer's statement about
/// what may move. An optimizer that quietly drove a variable marked
/// "Fixed parameter" would make that statement decorative, and would hand
/// back a design nobody agreed to. Names are matched case-insensitively,
/// the way every other design-variable reference is resolved.
fn reject_optimization_of_fixed_design_variables(
    sim_setup: &SimulationSetup,
    plan_payloads: &[rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord],
    variables: &[rspice_simulation_contract::optimization_draft::OptimizationVariableConfig],
) -> Result<(), String> {
    let Some(payload) = sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| {
            plan_payloads
                .iter()
                .find(|record| record.plan_id == plan_id)
                .map(|record| &record.payload)
        })
    else {
        return Ok(());
    };

    let mut fixed: Vec<&str> = Vec::new();
    for variable in variables {
        if let Some(declared) = payload
            .design_variables
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(&variable.name))
            && declared.sweep_eligibility
                == rspice_simulation_contract::design_variable::DesignVariableSweepEligibility::FixedParameter
        {
            fixed.push(declared.name.as_str());
        }
    }

    if fixed.is_empty() {
        return Ok(());
    }
    Err(format!(
        "optimization cannot vary {}: {} declared a fixed parameter on the Variables page. \
         Change the sweep role, or optimize a different variable.",
        if fixed.len() == 1 {
            "this design variable"
        } else {
            "these design variables"
        },
        fixed.join(", ")
    ))
}

pub fn build_soa_spec(
    draft: &rspice_simulation_contract::soa_draft::SoaDialogState,
) -> Result<AnalysisSpec, String> {
    let mut soa_state = draft.clone();
    soa_state.ensure_initialized();
    let cfg = soa_state
        .to_config()
        .map_err(|e| format!("invalid SOA settings: {}", e))?;
    Ok(AnalysisSpec::Soa {
        import_model_voltage_ratings: cfg.import_model_voltage_ratings,
        observation: cfg.observation,
        rules: cfg.rules,
        stop_time: cfg.stop_time,
        step_time: cfg.step_time,
        check_vgs_max: cfg.check_vgs_max,
        max_vgs: cfg.max_vgs,
        check_vds_max: cfg.check_vds_max,
        max_vds: cfg.max_vds,
        check_vbe_max: cfg.check_vbe_max,
        max_vbe: cfg.max_vbe,
        check_vce_max: cfg.check_vce_max,
        max_vce: cfg.max_vce,
    })
}

pub fn build_tf_spec(
    draft: &rspice_simulation_contract::xf_draft::XfDialogState,
) -> Result<AnalysisSpec, String> {
    let mut xf_state = draft.clone();
    xf_state.ensure_initialized();
    let config = xf_state
        .to_config()
        .map_err(|e| format!("invalid transfer-function settings: {}", e))?;
    Ok(AnalysisSpec::Tf {
        input_source: config.input_source,
        output_expression: config.output_expression,
        transfer_gain: config.transfer_gain,
        input_resistance: config.input_resistance,
        output_resistance: config.output_resistance,
        normalization: match config.normalization {
            rspice_simulation_contract::xf_draft::XfNormalization::None => {
                rspice_simulation_contract::analysis_spec::TfNormalization::None
            }
            rspice_simulation_contract::xf_draft::XfNormalization::RelativeToNominal => {
                rspice_simulation_contract::analysis_spec::TfNormalization::RelativeToNominal
            }
            rspice_simulation_contract::xf_draft::XfNormalization::PerSourceUnit => {
                rspice_simulation_contract::analysis_spec::TfNormalization::PerSourceUnit
            }
        },
        // The form's tier and the spec's tier are the one shared type, so
        // there is no translation left to get wrong.
        accuracy: config.accuracy,
    })
}

pub fn build_pole_zero_spec(
    draft: &rspice_simulation_contract::pz_draft::PzDialogState,
) -> Result<AnalysisSpec, String> {
    let mut pz_state = draft.clone();
    pz_state.ensure_initialized();
    let pz_cfg = pz_state
        .to_config()
        .map_err(|e| format!("invalid pole-zero settings: {}", e))?;

    let analysis_type = match pz_cfg.analysis_type {
        rspice_simulation_contract::pz_draft::PzAnalysisType::PolesAndZeros => {
            PzAnalysisType::PoleZero
        }
        rspice_simulation_contract::pz_draft::PzAnalysisType::PolesOnly => {
            PzAnalysisType::PolesOnly
        }
        rspice_simulation_contract::pz_draft::PzAnalysisType::ZerosOnly => {
            PzAnalysisType::ZerosOnly
        }
    };

    let transfer_type = match pz_cfg.transfer_type {
        rspice_simulation_contract::pz_draft::PzTransferType::Voltage => "VOL",
        rspice_simulation_contract::pz_draft::PzTransferType::Current => "CUR",
    };

    Ok(AnalysisSpec::PoleZero {
        input_node: pz_cfg.input_pos,
        input_ref: pz_cfg.input_neg,
        output_node: pz_cfg.output_pos,
        output_ref: pz_cfg.output_neg,
        transfer_type: transfer_type.to_string(),
        analysis_type: match analysis_type {
            PzAnalysisType::PoleZero => "PZ".to_string(),
            PzAnalysisType::PolesOnly => "POL".to_string(),
            PzAnalysisType::ZerosOnly => "ZER".to_string(),
        },
    })
}

pub fn build_sensitivity_spec(
    draft: &rspice_simulation_contract::sens_draft::SensDialogState,
) -> Result<AnalysisSpec, String> {
    let mut sens_state = draft.clone();
    sens_state.ensure_initialized();
    let sens_cfg = sens_state
        .to_config()
        .map_err(|e| format!("invalid sensitivity settings: {}", e))?;

    let ac_mode = matches!(
        sens_cfg.sens_type,
        rspice_simulation_contract::sens_draft::SensType::Ac
    );

    Ok(AnalysisSpec::Sensitivity {
        output_var: sens_cfg.output_expr,
        ac_mode,
        frequency: ac_mode.then_some(sens_cfg.ac_freq),
        filter: sens_cfg.filter,
        sweep: sens_cfg
            .sweep
            .map(rspice_simulation_contract::config::SensitivitySweep::to_spec),
    })
}

/// Borrowed inputs shared by draft lowering and frozen study preparation.
/// A projected instance replaces only `sim_setup`; circuit and evidence stay borrowed.
pub struct AnalysisInputs<'a, R, A> {
    pub sim_setup: &'a SimulationSetup,
    pub schematic: &'a rspice_design::schematic::document::SchematicDocument,
    pub selected_components: &'a std::collections::HashSet<u64>,
    pub runs: &'a [R],
    pub active_run: Option<&'a rspice_results::run::SimulationRun<A>>,
    pub project_revision: rspice_app_types::product::ObjectRevision,
    pub plan_payloads:
        &'a [rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord],
}

impl<R, A> Copy for AnalysisInputs<'_, R, A> {}
impl<R, A> Clone for AnalysisInputs<'_, R, A> {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn build_op_spec<'a, R, A, W>(
    state: &AnalysisInputs<'a, R, A>,
    draft: &rspice_simulation_contract::op_draft::OpDialogState,
) -> Result<AnalysisSpec, String>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    let mut op = draft.clone();
    if matches!(op.temperature_mode_idx, 0 | 3) {
        op.temperature = state
            .sim_setup
            .reference_pvt
            .temperature_celsius
            .to_string();
    }
    let mut config = op.to_config()?;
    config.selected_devices = state
        .schematic
        .components
        .iter()
        .filter(|component| state.selected_components.contains(&component.id))
        .map(|component| component.name.clone())
        .collect();
    config.selected_devices.sort();
    config.selected_devices.dedup();
    if config.initial_guess.uses_previous_state() {
        config.previous_state = rspice_results::run_history::newest_retained_op_state(
            state.runs.iter().map(AsRef::as_ref),
            state.project_revision,
            config.initial_guess
                == rspice_simulation_contract::config::OpInitialGuess::PreviousCompatible,
        );
    }
    if matches!(
        config.device_detail,
        rspice_simulation_contract::config::OpDeviceDetail::SelectedAndViolations
            | rspice_simulation_contract::config::OpDeviceDetail::ViolationsOnly
    ) && let Some((source_digest, devices)) = state
        .active_run
        .and_then(|run| run.soa_violation_context(state.project_revision))
    {
        config.violation_devices = devices;
        config.violation_source_content_digest = Some(source_digest);
    }
    config.validate_for_execution()?;
    Ok(AnalysisSpec::DcOp {
        temperature_mode: config.temperature_mode,
        temperature_celsius: config.temperature_celsius,
        initial_guess: config.initial_guess,
        node_initialization: config.node_initialization,
        homotopy: config.homotopy,
        annotation: config.annotation,
        device_detail: config.device_detail,
        save_device_op: config.save_device_op,
        accuracy: config.accuracy,
        selected_devices: config.selected_devices,
        previous_state: config.previous_state,
        violation_devices: config.violation_devices,
        violation_source_content_digest: config.violation_source_content_digest,
        run_point: config.run_point,
    })
}
