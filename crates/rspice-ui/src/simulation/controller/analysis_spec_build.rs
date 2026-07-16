use super::*;

impl SimulationController {
    pub(super) fn build_manifest_preview_spec(
        &self,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<Option<AnalysisSpec>, String> {
        use crate::simulation::plan::AnalysisDraft;

        if let Some(error) = draft.manifest_configuration_error() {
            return Err(error);
        }
        let spec = match draft {
            AnalysisDraft::Qpss(draft) => {
                let frequencies = parse_csv_si(&draft.tones, "QPSS tones")?;
                let harmonics = parse_csv_usize(&draft.harmonics, "QPSS harmonics")?;
                let tones = frequencies
                    .into_iter()
                    .zip(harmonics)
                    .enumerate()
                    .map(|(index, (frequency, harmonics))| {
                        HbToneSpec::new(frequency, harmonics)
                            .with_name(format!("tone{}", index + 1))
                    })
                    .collect();
                AnalysisSpec::Qpss {
                    tones,
                    max_iterations: parse_usize(&draft.max_iterations, "QPSS max iterations")?,
                    relative_tolerance: parse_si(
                        &draft.relative_tolerance,
                        "QPSS relative tolerance",
                    )?,
                    autonomous: draft.autonomous,
                    oscillator_node: (!draft.oscillator_node.trim().is_empty())
                        .then(|| draft.oscillator_node.trim().to_owned()),
                }
            }
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
                    mixed_mode: draft.mixed_mode,
                    noise_parameters: draft.noise_parameters,
                }
            }
            AnalysisDraft::Hbnoise(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Hbnoise {
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
                    mixed_mode: draft.mixed_mode,
                    noise_parameters: draft.noise_parameters,
                }
            }
            AnalysisDraft::Qpac(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Qpac {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    input_source: draft.input_source.trim().to_owned(),
                    output_node: draft.output_node.trim().to_owned(),
                    output_ref: draft.output_ref.trim().to_owned(),
                    input_lattice: parse_lattice_pair(&draft.input_lattice, "QPAC input lattice")?,
                    output_lattice: parse_lattice_pair(
                        &draft.output_lattice,
                        "QPAC output lattice",
                    )?,
                }
            }
            AnalysisDraft::Qpnoise(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                let (lattice_min, lattice_max) = parse_lattice_ranges(&draft.lattice_products)?;
                AnalysisSpec::Qpnoise {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    output_node: draft.output_node.trim().to_owned(),
                    output_ref: draft.output_ref.trim().to_owned(),
                    input_source: draft.input_source.trim().to_owned(),
                    lattice_min,
                    lattice_max,
                    integrated_noise: draft.integrated_noise,
                    contributor_ranking: draft.contributor_ranking,
                }
            }
            AnalysisDraft::Qpxf(draft) => {
                let (start_freq, stop_freq, points_per_unit, sweep) =
                    parse_manifest_sweep(&draft.sweep)?;
                AnalysisSpec::Qpxf {
                    start_freq,
                    stop_freq,
                    points_per_unit,
                    sweep,
                    input_source: draft.input_source.trim().to_owned(),
                    output_node: draft.output_node.trim().to_owned(),
                    output_ref: draft.output_ref.trim().to_owned(),
                    input_lattice: parse_lattice_pair(&draft.input_lattice, "QPXF input lattice")?,
                    output_lattice: parse_lattice_pair(
                        &draft.output_lattice,
                        "QPXF output lattice",
                    )?,
                    group_delay: draft.group_delay,
                }
            }
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
                scale: parse_si(&draft.scale, "TNOISE noise scale")?,
                uic: draft.use_initial_conditions,
            },
            AnalysisDraft::DcMismatch(draft) => AnalysisSpec::DcMismatch {
                output_expression: draft.output_expression.trim().to_owned(),
                sigma_multiplier: parse_si(&draft.sigma_multiplier, "DCMATCH sigma multiplier")?,
                contributor_limit: parse_usize(
                    &draft.contributor_limit,
                    "DCMATCH contributor limit",
                )?,
                include_process: draft.include_process,
                include_mismatch: draft.include_mismatch,
                normalized_contributions: draft.normalized_contributions,
            },
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
            0 => Ok(AnalysisSpec::DcOp),
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
                let (source2, start2, stop2, step2) = if state.sim_setup.dc.nested {
                    let source2 = state.sim_setup.dc.source2.trim();
                    if source2.is_empty() {
                        return Err(
                            "nested DC sweep requires a non-empty secondary sweep source"
                                .to_string(),
                        );
                    }
                    (
                        Some(source2.to_string()),
                        Some(
                            parse_spice_value_checked(&state.sim_setup.dc.start2)
                                .map_err(|e| format!("invalid secondary start value: {}", e))?,
                        ),
                        Some(
                            parse_spice_value_checked(&state.sim_setup.dc.stop2)
                                .map_err(|e| format!("invalid secondary stop value: {}", e))?,
                        ),
                        Some(
                            parse_spice_value_checked(&state.sim_setup.dc.step2)
                                .map_err(|e| format!("invalid secondary step value: {}", e))?,
                        ),
                    )
                } else {
                    (None, None, None, None)
                };
                Ok(AnalysisSpec::DcSweep {
                    source_name: state.sim_setup.dc.source.trim().to_string(),
                    start: parse_spice_value_checked(&state.sim_setup.dc.start)
                        .map_err(|e| format!("invalid start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.sim_setup.dc.stop)
                        .map_err(|e| format!("invalid stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.sim_setup.dc.step)
                        .map_err(|e| format!("invalid step value: {}", e))?,
                    source2,
                    start2,
                    stop2,
                    step2,
                })
            }
            4 => Ok(AnalysisSpec::Noise {
                output_node: state.sim_setup.noise.output.trim().to_string(),
                start_freq: parse_spice_value_checked(&state.sim_setup.noise.fstart)
                    .map_err(|e| format!("invalid start frequency: {}", e))?,
                stop_freq: parse_spice_value_checked(&state.sim_setup.noise.fstop)
                    .map_err(|e| format!("invalid stop frequency: {}", e))?,
                points_per_decade: Self::parse_positive_points(
                    &state.sim_setup.ac.points,
                    "ac_points",
                )?,
                temperature: 300.0,
            }),
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
            _ => Err(
                "analysis is not implemented in the current UI simulation controller".to_string(),
            ),
        }
    }

    pub(super) fn analysis_spec_to_config(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<AnalysisConfig, String> {
        match spec {
            AnalysisSpec::DcOp => Ok(AnalysisConfig::DcOp),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Ok(AnalysisConfig::DcSweep(DcSweepConfig {
                source: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
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
                start_freq,
                stop_freq,
                points_per_decade,
                ..
            } => Ok(AnalysisConfig::Noise(NoiseAnalysisConfig {
                output_node: output_node.clone(),
                reference_node: state.sim_setup.noise.reference.trim().to_string(),
                input_source: state.sim_setup.noise.input.trim().to_string(),
                sweep_type: Self::map_ac_sweep(Self::map_frequency_sweep(state.sim_setup.ac.sweep)),
                num_points: *points_per_decade,
                start_freq: *start_freq,
                stop_freq: *stop_freq,
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
            } => Ok(AnalysisConfig::Sensitivity(SensitivityConfig {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
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
        mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;
        Ok(AnalysisSpec::MonteCarlo)
    }

    pub(super) fn build_temperature_sweep_spec(
        &self,
        state: &AppState,
    ) -> Result<AnalysisSpec, String> {
        let mut temp_state = state.sim_setup.temp.clone();
        temp_state.ensure_initialized();
        temp_state
            .to_config()
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
        Ok(AnalysisSpec::Parametric)
    }

    pub(super) fn build_corner_sweep_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut corner_state = state.sim_setup.corner.clone();
        corner_state.ensure_initialized();
        corner_state
            .to_config()
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
            fundamental_freq: pss_cfg.fund_freq,
            num_harmonics: pss_cfg.num_harmonics as usize,
            tolerance: pss_cfg.stab_tol,
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
            sweep: FrequencySweep::Decade,
            points_per_decade: stb_cfg.points_per_decade as usize,
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
        let primary_name = if hb_cfg.fundamental_name.trim().is_empty() {
            "tone1".to_string()
        } else {
            hb_cfg.fundamental_name.trim().to_string()
        };
        let mut primary_tone =
            HbToneSpec::new(hb_cfg.fundamental_freq, hb_cfg.num_harmonics as usize)
                .with_name(primary_name);
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
            oversample: hb_cfg.oversample as usize,
            collocation_points: None,
            max_mixing_order: hb_cfg.max_mixing_order as usize,
            use_krylov: matches!(
                hb_cfg.solver,
                crate::simulation::dialog::hb::HbSolverType::Krylov
            ),
            gmres_restart: hb_cfg.gmres_restart as usize,
            source_stepping: hb_cfg.source_stepping,
            verbose: hb_cfg.verbose,
        })
    }

    pub(super) fn build_sp_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut sp_state = state.sim_setup.sp.clone();
        sp_state.ensure_initialized();
        let sp_cfg = sp_state
            .to_config()
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
        })
    }

    pub(super) fn build_envelope_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut envelope_state = state.sim_setup.envelope.clone();
        envelope_state.ensure_initialized();
        let envelope_cfg = envelope_state
            .to_config()
            .map_err(|e| format!("invalid envelope settings: {}", e))?;
        let max_step = (envelope_cfg.max_step > 0.0).then_some(envelope_cfg.max_step);
        Ok(AnalysisSpec::Envelope {
            fundamental_freq: envelope_cfg.fundamental_freq,
            stop_time: envelope_cfg.stop_time,
            num_harmonics: envelope_cfg.num_harmonics as usize,
            max_step,
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
            output_node: fourier_cfg.output_node.clone(),
            output_ref: fourier_cfg.output_ref.clone(),
            start_time: fourier_cfg.start_time,
            stop_time: fourier_cfg.stop_time,
        })
    }

    pub(super) fn build_reliability_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut reliability_state = state.sim_setup.reliability.clone();
        reliability_state.ensure_initialized();
        let reliability_cfg = reliability_state
            .to_config()
            .map_err(|e| format!("invalid reliability settings: {}", e))?;
        Ok(AnalysisSpec::Reliability {
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

        Ok(AnalysisSpec::Optimization {
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

    pub(super) fn build_soa_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut soa_state = state.sim_setup.soa.clone();
        soa_state.ensure_initialized();
        let cfg = soa_state
            .to_config()
            .map_err(|e| format!("invalid SOA settings: {}", e))?;
        Ok(AnalysisSpec::Soa {
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
        xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;
        Ok(AnalysisSpec::Tf)
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
        })
    }
}

fn parse_si(text: &str, field: &str) -> Result<f64, String> {
    crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))
}

fn parse_usize(text: &str, field: &str) -> Result<usize, String> {
    text.trim()
        .parse::<usize>()
        .map_err(|_| format!("{field} must be a positive integer"))
}

fn parse_csv_si(text: &str, field: &str) -> Result<Vec<f64>, String> {
    text.split(',')
        .map(|value| parse_si(value.trim(), field))
        .collect()
}

fn parse_csv_usize(text: &str, field: &str) -> Result<Vec<usize>, String> {
    text.split(',')
        .map(|value| parse_usize(value.trim(), field))
        .collect()
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

fn parse_lattice_pair(text: &str, field: &str) -> Result<[i32; 2], String> {
    let values = text
        .split(',')
        .map(|value| value.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("{field} must contain two integers"))?;
    values
        .try_into()
        .map_err(|_| format!("{field} must contain exactly two integers"))
}

fn parse_lattice_ranges(text: &str) -> Result<([i32; 2], [i32; 2]), String> {
    let ranges = text
        .split(',')
        .map(|range| {
            let bounds = range
                .split(':')
                .map(|value| value.trim().parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "QPNOISE lattice ranges require integers".to_owned())?;
            let [min, max]: [i32; 2] = bounds
                .try_into()
                .map_err(|_| "each QPNOISE lattice range requires min:max".to_owned())?;
            Ok((min, max))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let [(min0, max0), (min1, max1)]: [(i32, i32); 2] = ranges
        .try_into()
        .map_err(|_| "QPNOISE requires exactly two lattice ranges".to_owned())?;
    Ok(([min0, min1], [max0, max1]))
}

#[cfg(test)]
mod manifest_tests {
    use super::*;
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

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
        ] {
            let draft = AnalysisDraft::for_kind(kind);
            let spec = controller
                .build_manifest_preview_spec(&draft)
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
            ));
            assert!(spec.validate().is_ok());
        }
    }
}
