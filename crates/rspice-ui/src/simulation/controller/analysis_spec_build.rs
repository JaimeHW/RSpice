//! Assembling the analysis specification for a run.
//!
//! Resolves one row of the configured run set into a fully-determined
//! [`AnalysisSpec`]: the analysis itself, its sweep, its outputs, and the
//! manifest entry that records what was executed.

use super::*;

impl SimulationController {
    pub(super) fn build_manifest_preview_spec(
        &self,
        state: &AppState,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<Option<AnalysisSpec>, String> {
        use crate::simulation::plan::AnalysisDraft;

        if let Some(error) = draft.manifest_configuration_error() {
            return Err(error);
        }
        let spec = match draft {
            AnalysisDraft::Disto(draft) => AnalysisSpec::Disto {
                start_freq: parse_spice_value_checked(&draft.sweep.fstart)
                    .map_err(|e| format!("invalid DISTO start frequency: {e}"))?,
                stop_freq: parse_spice_value_checked(&draft.sweep.fstop)
                    .map_err(|e| format!("invalid DISTO stop frequency: {e}"))?,
                points_per_unit: Self::parse_positive_points(&draft.sweep.points, "disto_points")?,
                sweep: Self::map_frequency_sweep(draft.sweep.sweep),
                f2_over_f1: Self::parse_optional_spice_value(&draft.f2_over_f1)
                    .map_err(|e| format!("invalid DISTO f2/f1 ratio: {e}"))?,
            },
            AnalysisDraft::Noise(draft) => {
                let mut config = draft.to_config()?;
                config.temperature_kelvin =
                    state.sim_setup.reference_pvt.temperature_celsius + 273.15;
                config.validate().map_err(|errors| errors.join("; "))?;
                AnalysisSpec::Noise {
                    output_node: config.output_node,
                    reference_node: config.reference_node,
                    input_source: config.input_source,
                    start_freq: config.start_freq,
                    stop_freq: config.stop_freq,
                    points_per_decade: config.num_points,
                    sweep: draft.sweep,
                    explicit_frequencies: config.explicit_frequencies,
                    data_table_name: None,
                    contribution_detail: config.contribution_detail,
                    integration_mode: config.integration_mode,
                    temperature: config.temperature_kelvin,
                }
            }
            AnalysisDraft::Qpss(draft) => draft.to_spec()?,
            AnalysisDraft::Hbsp(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Hbsp {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    ports: parse_manifest_ports(&draft.ports)?,
                    max_sideband: parse_usize(&draft.max_sideband, "HBSP max sideband")?,
                    reltol: parse_positive_value(&draft.reltol, "HBSP relative tolerance")?,
                    abstol: parse_positive_value(&draft.abstol, "HBSP absolute tolerance")?,
                    mixed_mode: draft.mixed_mode,
                    noise_parameters: draft.noise_parameters,
                    noise_reference: draft.noise_reference()?,
                }
            }
            AnalysisDraft::Hbnoise(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Hbnoise {
                    input_sideband: draft.sidebands()?.0,
                    output_sideband: draft.sidebands()?.1,
                    noise_reference: draft.noise_reference()?,
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    output_node: draft.output_node.trim().to_owned(),
                    output_ref: draft.output_ref.trim().to_owned(),
                    input_source: draft.input_source.trim().to_owned(),
                    max_sideband: parse_usize(&draft.max_sideband, "HBNOISE max sideband")?,
                    integrated_noise: draft.integrated_noise,
                    noise_figure: draft.noise_figure,
                    contributor_ranking: draft.contributor_ranking,
                }
            }
            AnalysisDraft::Psp(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Psp {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    ports: parse_manifest_ports(&draft.ports)?,
                    max_sideband: parse_usize(&draft.max_sideband, "PSP max sideband")?,
                    reltol: parse_positive_value(&draft.reltol, "PSP relative tolerance")?,
                    abstol: parse_positive_value(&draft.abstol, "PSP absolute tolerance")?,
                    mixed_mode: draft.mixed_mode,
                    noise_parameters: draft.noise_parameters,
                    noise_reference: draft.noise_reference()?,
                }
            }
            AnalysisDraft::Qpac(draft) => draft.to_spec()?,
            AnalysisDraft::Qpnoise(draft) => draft.to_spec()?,
            AnalysisDraft::Qpxf(draft) => draft.to_spec()?,
            AnalysisDraft::TransientNoise(draft) => AnalysisSpec::TransientNoise {
                stop_time: parse_si(&draft.stop_time, "TNOISE stop time")?,
                step_time: parse_si(&draft.step_time, "TNOISE step time")?,
                start_time: parse_si(&draft.start_time, "TNOISE start time")?,
                max_timestep: parse_si(&draft.max_step, "TNOISE max step")?,
                seed: draft
                    .seed
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| "TNOISE seed must be an unsigned integer".to_owned())?,
                noise_fmax: parse_si(&draft.noise_fmax, "TNOISE maximum noise frequency")?,
                // Empty is not a missing value: it is the run asking the
                // engine for its own `1/tstop` floor, which is the widest
                // band the window can represent.
                noise_fmin: if draft.noise_fmin.trim().is_empty() {
                    None
                } else {
                    Some(parse_si(
                        &draft.noise_fmin,
                        "TNOISE minimum noise frequency",
                    )?)
                },
                scale: parse_si(&draft.scale, "TNOISE noise scale")?,
                uic: draft.use_initial_conditions,
            },
            AnalysisDraft::AcData(draft) => {
                let config = draft.to_config()?;
                AnalysisSpec::AcData {
                    table_name: config.table_name,
                    frequencies: config.frequencies,
                }
            }
            AnalysisDraft::Fft(draft) => AnalysisSpec::Fft {
                request: draft.to_request()?,
            },
            AnalysisDraft::DcMismatch(draft) => AnalysisSpec::DcMismatch {
                moment_options: draft.moment_options()?,
                output_expression: draft.output_expression.trim().to_owned(),
                sigma_multiplier: parse_si(&draft.sigma_multiplier, "DCMATCH sigma multiplier")?,
                contributor_limit: parse_usize(
                    &draft.contributor_limit,
                    "DCMATCH contributor limit",
                )?,
                include_process: draft.include_process,
                include_mismatch: draft.include_mismatch,
                normalized_contributions: draft.normalized_contributions,
                contribution_threshold: crate::simulation::plan::dc_mismatch_share_threshold(
                    &draft.share_threshold,
                )?,
            },
            AnalysisDraft::Reliability(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                let config = draft
                    .to_config()
                    .map_err(|error| format!("invalid reliability settings: {error}"))?;
                AnalysisSpec::Reliability {
                    study: config.study,
                    target_years: config.target_years,
                    enable_hci: config.enable_hci,
                    enable_nbti: config.enable_nbti,
                    enable_em: config.enable_em,
                    min_stress_voltage: config.min_stress_voltage,
                }
            }
            _ => return Ok(None),
        };
        spec.validate()?;
        Ok(Some(spec))
    }

    pub(super) fn build_analysis_spec_for_index(
        &self,
        state: &AppState,
        idx: usize,
    ) -> Result<AnalysisSpec, String> {
        match idx {
            0 => {
                let mut config = state.sim_setup.op.to_config()?;
                config.selected_devices = state
                    .schematic
                    .components
                    .iter()
                    .filter(|component| state.schematic.selection.has_component(component.id))
                    .map(|component| component.name.clone())
                    .collect();
                config.selected_devices.sort();
                config.selected_devices.dedup();
                if config.initial_guess.uses_previous_state() {
                    config.previous_state = state.simulation.newest_retained_op_state(
                        state.workspace.project.revision(),
                        config.initial_guess
                            == crate::simulation::dialog::OpInitialGuess::PreviousCompatible,
                    );
                }
                if matches!(
                    config.device_detail,
                    crate::simulation::dialog::OpDeviceDetail::SelectedAndViolations
                        | crate::simulation::dialog::OpDeviceDetail::ViolationsOnly
                ) && let Some((source_digest, devices)) = state
                    .simulation
                    .active_soa_violation_context(state.workspace.project.revision())
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
            1 => Ok(AnalysisSpec::Transient {
                stop_time: parse_spice_value_checked(&state.sim_setup.tran.stop)
                    .map_err(|e| format!("invalid stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.sim_setup.tran.step)
                    .map_err(|e| format!("invalid step time: {}", e))?,
                start_time: parse_spice_value_checked(&state.sim_setup.tran.start)
                    .map_err(|e| format!("invalid start time: {}", e))?,
                max_timestep: Self::parse_optional_spice_value(&state.sim_setup.tran.max_step)
                    .map_err(|e| format!("invalid max step: {}", e))?,
                uic: state.sim_setup.tran.uic,
            }),
            2 => Ok(AnalysisSpec::Ac {
                start_freq: parse_spice_value_checked(&state.sim_setup.ac.fstart)
                    .map_err(|e| format!("invalid start frequency: {}", e))?,
                stop_freq: parse_spice_value_checked(&state.sim_setup.ac.fstop)
                    .map_err(|e| format!("invalid stop frequency: {}", e))?,
                points_per_unit: Self::parse_positive_points(
                    &state.sim_setup.ac.points,
                    "ac_points",
                )?,
                sweep: Self::map_frequency_sweep(state.sim_setup.ac.sweep),
            }),
            24 => self.build_disto_spec(state),
            3 => {
                let config = state.sim_setup.dc.to_config()?;
                Ok(AnalysisSpec::DcSweep {
                    source_name: config.source,
                    start: config.start,
                    stop: config.stop,
                    step: config.step,
                    source2: config.source2,
                    start2: config.start2,
                    stop2: config.stop2,
                    step2: config.step2,
                    hysteresis: config.hysteresis,
                    modes: config.modes,
                })
            }
            4 => match self.build_manifest_preview_spec(
                state,
                &state
                    .sim_setup
                    .legacy_analysis_draft(crate::simulation::plan::AnalysisKind::Noise),
            )? {
                Some(spec @ AnalysisSpec::Noise { .. }) => Ok(spec),
                _ => Err("noise draft did not produce an exact noise spec".to_owned()),
            },
            5 => self.build_pole_zero_spec(state),
            6 => self.build_sensitivity_spec(state),
            7 => self.build_monte_carlo_spec(state),
            8 => self.build_pss_spec(state),
            9 => self.build_stb_spec(state),
            10 => self.build_temperature_sweep_spec(state),
            11 => self.build_harmonic_balance_spec(state),
            12 => self.build_sp_spec(state),
            13 => self.build_pac_spec(state),
            14 => self.build_pnoise_spec(state),
            15 => self.build_pxf_spec(state),
            16 => self.build_pstb_spec(state),
            17 => self.build_tf_spec(state),
            18 => self.build_corner_sweep_spec(state),
            19 => self.build_envelope_spec(state),
            20 => self.build_fourier_spec(state),
            21 => self.build_reliability_spec(state),
            22 => self.build_optimization_spec(state),
            23 => self.build_soa_spec(state),
            _ => Err(format!(
                "analysis index {idx} is outside the canonical Simulation Studio catalog"
            )),
        }
    }

    pub(super) fn analysis_spec_to_config(
        &self,
        _state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<AnalysisConfig, String> {
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
            } => Ok(AnalysisConfig::DcOp(crate::simulation::dialog::OpConfig {
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
            })),
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
                sweep_type: Self::map_ac_sweep(*sweep),
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
                sweep: sweep.map(crate::simulation::config::SensitivitySweep::from_spec),
            })),
            _ => Err(format!(
                "{} runs through the spec-driven simulation path and cannot be converted to a legacy analysis config",
                spec.run_type().display_name()
            )),
        }
    }

    pub(super) fn build_monte_carlo_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut mc_state = state.sim_setup.mc.clone();
        mc_state.ensure_initialized();
        let mc_cfg = mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;
        Ok(AnalysisSpec::MonteCarlo {
            variation_source: mc_cfg.variation_source,
            params: mc_cfg.params,
        })
    }

    pub(super) fn build_temperature_sweep_spec(
        &self,
        state: &AppState,
    ) -> Result<AnalysisSpec, String> {
        let mut temp_state = state.sim_setup.temp.clone();
        temp_state.ensure_initialized();
        temp_state
            .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
        Ok(AnalysisSpec::Parametric)
    }

    pub(super) fn build_corner_sweep_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut corner_state = state.sim_setup.corner.clone();
        corner_state.ensure_initialized();
        corner_state
            .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
            .map_err(|e| format!("invalid corner settings: {}", e))?;
        Ok(AnalysisSpec::Corner)
    }

    pub(super) fn build_pss_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pss_state = state.sim_setup.pss.clone();
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

    pub(super) fn build_stb_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut stb_state = state.sim_setup.stb.clone();
        stb_state.ensure_initialized();
        let stb_cfg = stb_state
            .to_config()
            .map_err(|e| format!("invalid STB settings: {}", e))?;
        Ok(AnalysisSpec::Stb {
            probe_node: stb_cfg.probe_source,
            start_freq: stb_cfg.start_freq,
            stop_freq: stb_cfg.stop_freq,
            sweep: match stb_cfg.sweep_type {
                crate::simulation::dialog::stb::StbSweepType::Decade => FrequencySweep::Decade,
                crate::simulation::dialog::stb::StbSweepType::Octave => FrequencySweep::Octave,
                crate::simulation::dialog::stb::StbSweepType::Linear => FrequencySweep::Linear,
            },
            points_per_decade: stb_cfg.num_points as usize,
            compute_nyquist: stb_cfg.compute_nyquist,
        })
    }

    pub(super) fn build_harmonic_balance_spec(
        &self,
        state: &AppState,
    ) -> Result<AnalysisSpec, String> {
        let mut hb_state = state.sim_setup.hb.clone();
        hb_state.ensure_initialized();
        let hb_cfg = hb_state
            .to_config()
            .map_err(|e| format!("invalid harmonic balance settings: {}", e))?;
        let mut tones = Vec::with_capacity(1 + hb_cfg.additional_tones.len());
        // The primary tone's label. Fixed, because nothing authors one: the
        // form has no name row and the additional tones number themselves
        // from this.
        let mut primary_tone =
            HbToneSpec::new(hb_cfg.fundamental_freq, hb_cfg.num_harmonics as usize)
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
                crate::simulation::dialog::hb::HbSolverType::Krylov
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
    pub(super) fn build_sp_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut sp_state = state.sim_setup.sp.clone();
        sp_state.ensure_initialized();
        let placed = crate::simulation::placed_sources::placed_rf_ports(&state.schematic, None);
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
                crate::simulation::dialog::sp::SpSweepType::Decade => FrequencySweep::Decade,
                crate::simulation::dialog::sp::SpSweepType::Octave => FrequencySweep::Octave,
                crate::simulation::dialog::sp::SpSweepType::Linear => FrequencySweep::Linear,
            },
            z0: sp_cfg.z0,
            ports,
            do_noise: sp_cfg.do_noise,
        })
    }

    pub(super) fn build_envelope_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut envelope_state = state.sim_setup.envelope.clone();
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

    pub(super) fn build_fourier_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut fourier_state = state.sim_setup.fourier.clone();
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

    pub(super) fn build_reliability_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut reliability_state = state.sim_setup.reliability.clone();
        reliability_state.ensure_initialized();
        let reliability_cfg = reliability_state
            .to_config()
            .map_err(|e| format!("invalid reliability settings: {}", e))?;
        Ok(AnalysisSpec::Reliability {
            study: reliability_cfg.study,
            target_years: reliability_cfg.target_years,
            enable_hci: reliability_cfg.enable_hci,
            enable_nbti: reliability_cfg.enable_nbti,
            enable_em: reliability_cfg.enable_em,
            min_stress_voltage: reliability_cfg.min_stress_voltage,
        })
    }

    pub(super) fn build_optimization_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut optimization_state = state.sim_setup.optimization.clone();
        optimization_state.ensure_initialized();
        let cfg = optimization_state
            .to_config()
            .map_err(|e| format!("invalid optimization settings: {}", e))?;

        Self::reject_optimization_of_fixed_design_variables(state, &cfg.variables)?;

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
                crate::simulation::dialog::optimization::OptimizationGoalMode::Minimize => {
                    OptimizationGoal::Minimize
                }
                crate::simulation::dialog::optimization::OptimizationGoalMode::Maximize => {
                    OptimizationGoal::Maximize
                }
                crate::simulation::dialog::optimization::OptimizationGoalMode::Target => {
                    OptimizationGoal::Target
                }
            },
            target: cfg.target_value,
            algorithm: match cfg.algorithm {
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::GradientDescent => {
                    OptimizationAlgorithm::GradientDescent
                }
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::PatternSearch => {
                    OptimizationAlgorithm::PatternSearch
                }
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::SimulatedAnnealing => {
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
        state: &AppState,
        variables: &[crate::simulation::dialog::optimization::OptimizationVariableConfig],
    ) -> Result<(), String> {
        let Some(payload) = state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .map(|plan| plan.id())
            .and_then(|plan_id| state.workspace.plan_data(plan_id))
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
                    == crate::state::DesignVariableSweepEligibility::FixedParameter
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

    pub(super) fn build_soa_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut soa_state = state.sim_setup.soa.clone();
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

    pub(super) fn build_pac_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pac_state = state.sim_setup.pac.clone();
        pac_state.ensure_initialized();
        pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;
        Ok(AnalysisSpec::Pac)
    }

    pub(super) fn build_pnoise_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pnoise_state = state.sim_setup.pnoise.clone();
        pnoise_state.ensure_initialized();
        pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;
        Ok(AnalysisSpec::Pnoise)
    }

    pub(super) fn build_pxf_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pxf_state = state.sim_setup.pxf.clone();
        pxf_state.ensure_initialized();
        pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;
        Ok(AnalysisSpec::Pxf)
    }

    pub(super) fn build_pstb_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pstb_state = state.sim_setup.pstb.clone();
        pstb_state.ensure_initialized();
        pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;
        Ok(AnalysisSpec::Pstb)
    }

    pub(super) fn build_tf_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut xf_state = state.sim_setup.xf.clone();
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
                crate::simulation::dialog::XfNormalization::None => {
                    crate::simulation::multi_run::TfNormalization::None
                }
                crate::simulation::dialog::XfNormalization::RelativeToNominal => {
                    crate::simulation::multi_run::TfNormalization::RelativeToNominal
                }
                crate::simulation::dialog::XfNormalization::PerSourceUnit => {
                    crate::simulation::multi_run::TfNormalization::PerSourceUnit
                }
            },
            // The form's tier and the spec's tier are the one shared type, so
            // there is no translation left to get wrong.
            accuracy: config.accuracy,
        })
    }

    pub(super) fn build_disto_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        Ok(AnalysisSpec::Disto {
            start_freq: parse_spice_value_checked(&state.sim_setup.ac.fstart)
                .map_err(|e| format!("invalid DISTO start frequency: {}", e))?,
            stop_freq: parse_spice_value_checked(&state.sim_setup.ac.fstop)
                .map_err(|e| format!("invalid DISTO stop frequency: {}", e))?,
            points_per_unit: Self::parse_positive_points(&state.sim_setup.ac.points, "ac_points")?,
            sweep: Self::map_frequency_sweep(state.sim_setup.ac.sweep),
            f2_over_f1: Self::parse_optional_spice_value(&state.sim_setup.disto_f2_over_f1)
                .map_err(|e| format!("invalid DISTO f2/f1 ratio: {}", e))?,
        })
    }

    pub(super) fn build_pole_zero_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pz_state = state.sim_setup.pz.clone();
        pz_state.ensure_initialized();
        let pz_cfg = pz_state
            .to_config()
            .map_err(|e| format!("invalid pole-zero settings: {}", e))?;

        let analysis_type = match pz_cfg.analysis_type {
            crate::simulation::dialog::pz::PzAnalysisType::PolesAndZeros => {
                PzAnalysisType::PoleZero
            }
            crate::simulation::dialog::pz::PzAnalysisType::PolesOnly => PzAnalysisType::PolesOnly,
            crate::simulation::dialog::pz::PzAnalysisType::ZerosOnly => PzAnalysisType::ZerosOnly,
        };

        let transfer_type = match pz_cfg.transfer_type {
            crate::simulation::dialog::pz::PzTransferType::Voltage => "VOL",
            crate::simulation::dialog::pz::PzTransferType::Current => "CUR",
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

    pub(super) fn build_sensitivity_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut sens_state = state.sim_setup.sens.clone();
        sens_state.ensure_initialized();
        let sens_cfg = sens_state
            .to_config()
            .map_err(|e| format!("invalid sensitivity settings: {}", e))?;

        let ac_mode = matches!(
            sens_cfg.sens_type,
            crate::simulation::dialog::sens::SensType::Ac
        );

        Ok(AnalysisSpec::Sensitivity {
            output_var: sens_cfg.output_expr,
            ac_mode,
            frequency: ac_mode.then_some(sens_cfg.ac_freq),
            filter: sens_cfg.filter,
            sweep: sens_cfg
                .sweep
                .map(crate::simulation::config::SensitivitySweep::to_spec),
        })
    }
}

fn parse_si(text: &str, field: &str) -> Result<f64, String> {
    crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))
}

fn parse_positive_value(text: &str, field: &str) -> Result<f64, String> {
    let value = parse_si(text, field)?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("{field} must be finite and positive"));
    }
    Ok(value)
}

fn parse_usize(text: &str, field: &str) -> Result<usize, String> {
    text.trim()
        .parse::<usize>()
        .map_err(|_| format!("{field} must be a positive integer"))
}

fn parse_manifest_sweep(
    draft: &crate::simulation::plan::FrequencySweepDraft,
) -> Result<(f64, f64, usize, FrequencySweep), String> {
    let sweep = match draft.sweep {
        0 => FrequencySweep::Decade,
        1 => FrequencySweep::Octave,
        2 => FrequencySweep::Linear,
        _ => return Err("frequency sweep mode is outside the supported schema".to_owned()),
    };
    Ok((
        parse_si(&draft.start, "start frequency")?,
        parse_si(&draft.stop, "stop frequency")?,
        parse_usize(&draft.points, "sweep point count")?,
        sweep,
    ))
}

fn parse_manifest_ports(
    ports: &[crate::simulation::plan::NetworkPortDraft],
) -> Result<Vec<SpPort>, String> {
    ports
        .iter()
        .map(|port| {
            Ok(SpPort {
                node_pos: port.node_pos.trim().to_owned(),
                node_neg: port.node_neg.trim().to_owned(),
                z0: Some(parse_si(&port.z0, "port reference impedance")?),
            })
        })
        .collect()
}

#[cfg(test)]
mod manifest_tests {
    use super::*;
    use crate::simulation::multi_run::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    };
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

    #[test]
    fn single_frequency_noise_stb_and_disto_drafts_reach_valid_worker_specs() {
        use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        for (index, noise_sweep) in [
            NoiseSweepType::Decade,
            NoiseSweepType::Octave,
            NoiseSweepType::Linear,
        ]
        .into_iter()
        .enumerate()
        {
            let noise = crate::simulation::plan::NoiseDraft {
                output: "out".into(),
                input: "VIN".into(),
                fstart: "1k".into(),
                fstop: "1k".into(),
                points: "1".into(),
                sweep: noise_sweep,
                ..Default::default()
            };
            noise.to_config().unwrap();
            let mut disto = crate::simulation::plan::DistoDraft::default();
            disto.sweep.fstart = "1k".into();
            disto.sweep.fstop = "1k".into();
            disto.sweep.points = "1".into();
            disto.sweep.sweep = index;

            state.sim_setup.stb.ensure_initialized();
            state.sim_setup.stb.start_freq = "1k".into();
            state.sim_setup.stb.stop_freq = "1k".into();
            state.sim_setup.stb.num_points = "1".into();
            state.sim_setup.stb.sweep_type_idx = index;

            let specs = [
                controller
                    .build_manifest_preview_spec(&state, &AnalysisDraft::Noise(noise))
                    .unwrap()
                    .unwrap(),
                controller
                    .build_manifest_preview_spec(&state, &AnalysisDraft::Disto(disto))
                    .unwrap()
                    .unwrap(),
                controller.build_stb_spec(&state).unwrap(),
            ];
            for spec in specs {
                spec.validate().unwrap();
                let packet = WorkerAnalysisSpec::try_from(&spec).unwrap();
                let encoded = serde_json::to_string(&packet).unwrap();
                let restored = AnalysisSpec::from(
                    serde_json::from_str::<WorkerAnalysisSpec>(&encoded).unwrap(),
                );
                restored.validate().unwrap();
                assert_eq!(
                    serde_json::to_value(&restored).unwrap(),
                    serde_json::to_value(&spec).unwrap()
                );

                for invalid in [0.0, 999.0, f64::NAN, f64::INFINITY] {
                    let mut invalid_spec = spec.clone();
                    match &mut invalid_spec {
                        AnalysisSpec::Noise { stop_freq, .. }
                        | AnalysisSpec::Stb { stop_freq, .. }
                        | AnalysisSpec::Disto { stop_freq, .. } => *stop_freq = invalid,
                        _ => unreachable!(),
                    }
                    assert!(invalid_spec.validate().is_err(), "{invalid_spec:?}");
                }
            }
        }
    }

    #[test]
    fn noise_manifest_freezes_every_selected_field_without_singleton_fallback() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.reference_pvt.temperature_celsius = 125.0;
        state.sim_setup.noise.output = "wrong_singleton".to_owned();
        state.sim_setup.ac.points = "999".to_owned();
        let mut draft = crate::simulation::plan::NoiseDraft::default();
        draft.output = "V(out,ref)".to_owned();
        draft.input = "VIN_EXACT".to_owned();
        draft.sweep = NoiseSweepType::ExplicitFrequencyList;
        draft.explicit_frequencies = "3, 7, 11".to_owned();
        draft.contribution_detail =
            crate::simulation::config::NoiseContributionDetail::AllContributors;
        draft.integration_mode = crate::simulation::config::NoiseIntegrationMode::OutputNoiseOnly;

        let spec = controller
            .build_manifest_preview_spec(&state, &AnalysisDraft::Noise(draft))
            .expect("exact noise draft parses")
            .expect("noise draft has an exact spec");
        assert!(matches!(
            spec,
            AnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                sweep: NoiseSweepType::ExplicitFrequencyList,
                explicit_frequencies: Some(frequencies),
                contribution_detail:
                    crate::simulation::config::NoiseContributionDetail::AllContributors,
                integration_mode:
                    crate::simulation::config::NoiseIntegrationMode::OutputNoiseOnly,
                temperature,
                ..
            } if output_node == "out"
                && reference_node == "ref"
                && input_source == "VIN_EXACT"
                && frequencies == vec![3.0, 7.0, 11.0]
                && (temperature - 398.15).abs() < 1.0e-12
        ));
    }

    #[test]
    fn disto_manifest_freezes_its_owned_sweep_without_singleton_fallback() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.ac.fstart = "900".to_owned();
        state.sim_setup.ac.fstop = "2k".to_owned();
        state.sim_setup.ac.points = "7".to_owned();
        state.sim_setup.disto_f2_over_f1 = "0.2".to_owned();

        let mut draft = crate::simulation::plan::DistoDraft::default();
        draft.sweep.fstart = "3k".to_owned();
        draft.sweep.fstop = "30k".to_owned();
        draft.sweep.points = "41".to_owned();
        draft.sweep.sweep = 2;
        draft.f2_over_f1 = "0.8".to_owned();

        let spec = controller
            .build_manifest_preview_spec(&state, &AnalysisDraft::Disto(draft))
            .expect("exact DISTO draft parses")
            .expect("DISTO draft has an exact spec");
        assert!(matches!(
            spec,
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::Linear,
                f2_over_f1: Some(ratio),
            } if start_freq == 3_000.0
                && stop_freq == 30_000.0
                && points_per_unit == 41
                && ratio == 0.8
        ));
    }

    /// The field this alignment is about, read end to end: what a reader types
    /// into the Simulate form's transient stop time is what the run is
    /// configured with.
    ///
    /// `1ns` used to be refused outright by the parser behind this field — a
    /// unit letter after a scale factor was an unsupported suffix — while `1A`
    /// was accepted as 1e-18 seconds, because that table still had atto. Both
    /// now read the way the engine reads them out of a deck: one nanosecond,
    /// and one second with `A` as a neutral unit designator.
    #[test]
    fn a_transient_stop_time_is_the_number_a_deck_would_read_from_the_same_text() {
        let controller = SimulationController::new();
        for (typed, expected) in [
            ("1ns", 1e-9),
            ("1A", 1.0),
            ("1", 1.0),
            ("2.5s", 2.5),
            ("1m", 1e-3),
            ("1mil", 25.4e-6),
        ] {
            let mut state = AppState::default();
            state.sim_setup.tran.stop = typed.to_owned();

            let spec = controller
                .build_analysis_spec_for_index(&state, 1)
                .unwrap_or_else(|error| panic!("a stop time of {typed} must be accepted: {error}"));
            let AnalysisSpec::Transient { stop_time, .. } = spec else {
                panic!("index 1 is the transient analysis");
            };
            assert!(
                (stop_time - expected).abs() <= expected * 1e-12,
                "a stop time typed {typed} reached the draft as {stop_time:e}, not {expected:e}"
            );
        }

        // The spellings no deck reader has stay refused, rather than reaching a
        // run at a decade the engine would not have agreed with.
        for typed in ["1micro", "1wat", "1k5"] {
            let mut state = AppState::default();
            state.sim_setup.tran.stop = typed.to_owned();
            assert!(
                controller.build_analysis_spec_for_index(&state, 1).is_err(),
                "a stop time of {typed} must not reach a run"
            );
        }
    }

    /// The three step sizes the search actually uses are authored, not
    /// hardcoded.
    ///
    /// The configuration, its bounds and the runner's copy were all in place;
    /// what was missing was any control, so every run used the literals in
    /// `OptimizationConfig::default`. The assertion is against the typed
    /// specification the runner dispatches on, which is what
    /// `runner::spec::device::run_optimization` copies field by field into
    /// `OptimizationRunConfig` — not against the numbers themselves, which
    /// would pass just as well if the form were still ignored.
    #[test]
    fn the_optimizer_step_sizes_are_authorable_and_reach_the_run() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.optimization.ensure_initialized();

        let defaults = state
            .sim_setup
            .optimization
            .to_config()
            .expect("the default optimization draft is runnable");

        state.sim_setup.optimization.fd_step = "2.5e-3".to_owned();
        state.sim_setup.optimization.initial_step = "0.25".to_owned();
        state.sim_setup.optimization.min_step = "1e-6".to_owned();

        let index = AnalysisKind::Optimization.legacy_index();
        let spec = controller
            .build_analysis_spec_for_index(&state, index)
            .expect("an authored optimization draft builds its spec");
        let AnalysisSpec::Optimization {
            fd_step,
            initial_step,
            min_step,
            ..
        } = spec
        else {
            panic!("the optimization index builds an optimization spec");
        };

        assert_eq!(fd_step, 2.5e-3);
        assert_eq!(initial_step, 0.25);
        assert_eq!(min_step, 1e-6);
        assert!(
            fd_step != defaults.fd_step
                && initial_step != defaults.initial_step
                && min_step != defaults.min_step,
            "the authored values must differ from the defaults, or this test cannot tell a \
             wired form from an ignored one"
        );

        // The engine's own bound, refused at the boundary rather than clamped:
        // a first step smaller than the smallest one describes no search.
        state.sim_setup.optimization.min_step = "0.5".to_owned();
        let error = controller
            .build_analysis_spec_for_index(&state, index)
            .expect_err("a smallest step above the first step is not a search");
        assert!(
            error.contains("min_step"),
            "the refusal must name the control it is about: {error}"
        );
    }

    #[test]
    fn sp_noise_dialog_reaches_the_typed_request() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.sp.port_source_idx = Some(1);
        for requested in [false, true] {
            state.sim_setup.sp.do_noise = requested;
            let spec = controller.build_sp_spec(&state).unwrap();
            assert!(
                matches!(spec, AnalysisSpec::SParameter { do_noise, .. } if do_noise == requested)
            );
        }
    }

    #[test]
    fn every_new_manifest_draft_builds_its_exact_typed_spec() {
        let controller = SimulationController::new();
        for kind in [
            AnalysisKind::Qpss,
            AnalysisKind::Hbsp,
            AnalysisKind::Hbnoise,
            AnalysisKind::Psp,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
            AnalysisKind::TransientNoise,
            AnalysisKind::DcMismatch,
            AnalysisKind::Fft,
        ] {
            let draft = AnalysisDraft::for_kind(kind);
            let spec = controller
                .build_manifest_preview_spec(&AppState::default(), &draft)
                .expect("default draft parses")
                .expect("manifest draft has a typed spec");
            assert!(matches!(
                (kind, &spec),
                (AnalysisKind::Qpss, AnalysisSpec::Qpss { .. })
                    | (AnalysisKind::Hbsp, AnalysisSpec::Hbsp { .. })
                    | (AnalysisKind::Hbnoise, AnalysisSpec::Hbnoise { .. })
                    | (AnalysisKind::Psp, AnalysisSpec::Psp { .. })
                    | (AnalysisKind::Qpac, AnalysisSpec::Qpac { .. })
                    | (AnalysisKind::Qpnoise, AnalysisSpec::Qpnoise { .. })
                    | (AnalysisKind::Qpxf, AnalysisSpec::Qpxf { .. })
                    | (
                        AnalysisKind::TransientNoise,
                        AnalysisSpec::TransientNoise { .. }
                    )
                    | (AnalysisKind::DcMismatch, AnalysisSpec::DcMismatch { .. })
                    | (AnalysisKind::Fft, AnalysisSpec::Fft { .. })
            ));
            assert!(spec.validate().is_ok());
        }
    }

    #[test]
    fn periodic_network_tolerances_reach_both_typed_specs() {
        let controller = SimulationController::new();
        for kind in [AnalysisKind::Hbsp, AnalysisKind::Psp] {
            let mut draft = AnalysisDraft::for_kind(kind);
            let (AnalysisDraft::Hbsp(network) | AnalysisDraft::Psp(network)) = &mut draft else {
                unreachable!("the loop only creates periodic network drafts")
            };
            network.reltol = "2.5e-6".into();
            network.abstol = "7e-13".into();
            let spec = controller
                .build_manifest_preview_spec(&AppState::default(), &draft)
                .expect("periodic network draft parses")
                .expect("periodic network draft has a typed spec");
            match spec {
                AnalysisSpec::Hbsp { reltol, abstol, .. }
                | AnalysisSpec::Psp { reltol, abstol, .. } => {
                    assert_eq!(reltol, 2.5e-6);
                    assert_eq!(abstol, 7e-13);
                }
                _ => unreachable!("the loop only creates periodic network specs"),
            }
        }
    }

    #[test]
    fn pss_draft_projects_every_owned_execution_field() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.pss.ensure_initialized();
        state.sim_setup.pss.fund_freq = "2.5Meg".to_owned();
        state.sim_setup.pss.tone_sources = "VIN_LO, VIN_MOD".to_owned();
        state.sim_setup.pss.tstab_periods = "37".to_owned();
        state.sim_setup.pss.points_per_period = "1024".to_owned();
        state.sim_setup.pss.tolerance = "2e-9".to_owned();
        state.sim_setup.pss.num_harmonics = "17".to_owned();
        state.sim_setup.pss.integration_method_idx = 0;
        // Driven: this draft names two tones, and an autonomous solve naming a
        // tone is a refused contradiction rather than a projectable draft. The
        // retained oscillator node is deliberately left set to prove the driven
        // projection drops it rather than carrying a node it will not use.
        state.sim_setup.pss.osc_mode = false;
        state.sim_setup.pss.osc_node = "osc_out".to_owned();

        let spec = controller.build_pss_spec(&state).expect("PSS spec builds");
        assert_eq!(
            spec,
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 2.5e6,
                tone_sources: vec!["VIN_LO".to_owned(), "VIN_MOD".to_owned()],
                tstab_periods: 37,
                points_per_period: 1024,
                tolerance: 2.0e-9,
                oscillator_mode: false,
                oscillator_node: None,
                num_harmonics: 17,
                integration_method: None,
                tstab: 0.0,
                max_iterations: 100,
                abstol: 1.0e-12,
                damping: 1.0,
                max_period_change: 0.1,
                verbose: false,
            }
        );
    }

    /// The other half of the same projection: an autonomous draft names no
    /// tone, and the oscillator node it does name reaches the spec.
    #[test]
    fn an_autonomous_pss_draft_projects_its_oscillator_node_and_no_tones() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.pss.ensure_initialized();
        state.sim_setup.pss.integration_method_idx = 0;
        state.sim_setup.pss.tone_sources.clear();
        state.sim_setup.pss.osc_mode = true;
        state.sim_setup.pss.osc_node = "osc_out".to_owned();

        let spec = controller.build_pss_spec(&state).expect("PSS spec builds");
        assert!(
            matches!(
                spec,
                AnalysisSpec::Pss {
                    oscillator_mode: true,
                    ref oscillator_node,
                    ref tone_sources,
                    ..
                } if oscillator_node.as_deref() == Some("osc_out") && tone_sources.is_empty()
            ),
            "{spec:?}"
        );
    }

    #[test]
    fn fourier_draft_projects_thd_and_normalization_controls() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.fourier.ensure_initialized();
        state.sim_setup.fourier.compute_thd = false;
        state.sim_setup.fourier.normalize = true;

        let spec = controller
            .build_fourier_spec(&state)
            .expect("Fourier spec builds");
        assert!(matches!(
            spec,
            AnalysisSpec::Fourier {
                compute_thd: false,
                normalize: true,
                ..
            }
        ));
    }

    #[test]
    fn envelope_draft_projects_every_mockup_owned_execution_field() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.envelope.ensure_initialized();
        state.sim_setup.envelope.carrier_tones = "1Meg, 2.5Meg".to_owned();
        state.sim_setup.envelope.stop_time = "10m".to_owned();
        state.sim_setup.envelope.envelope_step = "1u".to_owned();
        state.sim_setup.envelope.harmonic_order = "11".to_owned();
        state.sim_setup.envelope.modulation_sources = "VIN_AM, VCTRL".to_owned();
        state.sim_setup.envelope.initial_periodic_solve_idx = 1;
        state.sim_setup.envelope.adaptive_mode_idx = 2;

        let spec = controller
            .build_envelope_spec(&state)
            .expect("Envelope spec builds");
        assert_eq!(
            spec,
            AnalysisSpec::Envelope {
                initialization: Default::default(),
                fundamental_freq: 1.0e6,
                additional_carrier_tones: vec![2.5e6],
                stop_time: 10.0e-3,
                num_harmonics: 11,
                envelope_step: Some(1.0e-6),
                modulation_sources: vec!["VIN_AM".to_owned(), "VCTRL".to_owned()],
                initial_periodic_solve: EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
                adaptive_mode: EnvelopeAdaptiveMode::EventAlignedOnly,
                extraction_path: EnvelopeExtractionPath::Projection,
            }
        );
    }
}
