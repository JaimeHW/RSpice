//! Assembling the analysis specification for a run.
//!
//! Resolves one row of the configured run set into a fully-determined
//! [`AnalysisSpec`]: the analysis itself, its sweep, its outputs, and the
//! manifest entry that records what was executed.

use super::*;

pub(super) type AnalysisInputs<'a> = rspice_simulation::analysis_preparation::AnalysisInputs<
    'a,
    crate::state::SimulationRun,
    crate::state::AnalysisResult,
>;

pub(super) fn analysis_inputs(state: &AppState) -> AnalysisInputs<'_> {
    AnalysisInputs {
        sim_setup: &state.sim_setup,
        schematic: state.schematic.document(),
        selected_components: &state.schematic.session.selection.components,
        runs: &state.simulation.runs,
        active_run: state.simulation.active_run().map(|run| &run.data),
        project_revision: state.workspace.content.project.revision(),
        plan_payloads: &state.workspace.content.simulation_plan_payloads,
    }
}

impl SimulationController {
    pub(super) fn analysis_draft_spec(
        &self,
        state: &AnalysisInputs<'_>,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<AnalysisSpec, String> {
        use crate::simulation::plan::AnalysisDraft;

        if let Some(error) = draft.manifest_configuration_error() {
            return Err(error);
        }
        let spec = match draft {
            AnalysisDraft::OperatingPoint(draft) => {
                rspice_simulation::analysis_preparation::build_op_spec(state, draft)?
            }
            AnalysisDraft::Transient(draft) => AnalysisSpec::Transient {
                stop_time: parse_spice_value_checked(&draft.stop)
                    .map_err(|e| format!("invalid stop time: {}", e))?,
                step_time: parse_spice_value_checked(&draft.step)
                    .map_err(|e| format!("invalid step time: {}", e))?,
                start_time: parse_spice_value_checked(&draft.start)
                    .map_err(|e| format!("invalid start time: {}", e))?,
                max_timestep: rspice_simulation::analysis_preparation::parse_optional_spice_value(
                    &draft.max_step,
                )
                .map_err(|e| format!("invalid max step: {}", e))?,
                uic: draft.uic,
            },
            AnalysisDraft::Ac(draft) => AnalysisSpec::Ac {
                start_freq: parse_spice_value_checked(&draft.fstart)
                    .map_err(|e| format!("invalid start frequency: {}", e))?,
                stop_freq: parse_spice_value_checked(&draft.fstop)
                    .map_err(|e| format!("invalid stop frequency: {}", e))?,
                points_per_unit: rspice_simulation::analysis_preparation::parse_positive_points(
                    &draft.points,
                    "ac_points",
                )?,
                sweep: rspice_simulation::analysis_preparation::map_frequency_sweep(draft.sweep),
            },
            AnalysisDraft::DcSweep(draft) => {
                let config = draft.to_config()?;
                AnalysisSpec::DcSweep {
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
                }
            }
            AnalysisDraft::MonteCarlo(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                let config = draft
                    .to_config()
                    .map_err(|error| format!("invalid Monte Carlo settings: {error}"))?;
                AnalysisSpec::MonteCarlo {
                    variation_source: config.variation_source,
                    params: config.params,
                }
            }
            AnalysisDraft::Pss(draft) => {
                rspice_simulation::analysis_preparation::build_pss_spec(draft)?
            }
            AnalysisDraft::HarmonicBalance(draft) => {
                rspice_simulation::analysis_preparation::build_harmonic_balance_spec(draft)?
            }
            AnalysisDraft::Stb(draft) => {
                rspice_simulation::analysis_preparation::build_stb_spec(draft)?
            }
            AnalysisDraft::SParameter(draft) => {
                rspice_simulation::analysis_preparation::build_sp_spec(state.schematic, draft)?
            }
            AnalysisDraft::Envelope(draft) => {
                rspice_simulation::analysis_preparation::build_envelope_spec(draft)?
            }
            AnalysisDraft::Fourier(draft) => {
                rspice_simulation::analysis_preparation::build_fourier_spec(draft)?
            }
            AnalysisDraft::Optimization(draft) => {
                rspice_simulation::analysis_preparation::build_optimization_spec(
                    state.sim_setup,
                    state.plan_payloads,
                    draft,
                )?
            }
            AnalysisDraft::Soa(draft) => {
                rspice_simulation::analysis_preparation::build_soa_spec(draft)?
            }
            AnalysisDraft::PoleZero(draft) => {
                rspice_simulation::analysis_preparation::build_pole_zero_spec(draft)?
            }
            AnalysisDraft::Sensitivity(draft) => {
                rspice_simulation::analysis_preparation::build_sensitivity_spec(draft)?
            }
            AnalysisDraft::TransferFunction(draft) => {
                rspice_simulation::analysis_preparation::build_tf_spec(draft)?
            }
            AnalysisDraft::Pac(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                draft
                    .to_config()
                    .map_err(|error| format!("invalid PAC settings: {error}"))?;
                AnalysisSpec::Pac
            }
            AnalysisDraft::Pnoise(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                draft
                    .to_config()
                    .map_err(|error| format!("invalid PNOISE settings: {error}"))?;
                AnalysisSpec::Pnoise
            }
            AnalysisDraft::Pxf(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                draft
                    .to_config()
                    .map_err(|error| format!("invalid PXF settings: {error}"))?;
                AnalysisSpec::Pxf
            }
            AnalysisDraft::Pstb(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                draft
                    .to_config()
                    .map_err(|error| format!("invalid PSTB settings: {error}"))?;
                AnalysisSpec::Pstb
            }
            AnalysisDraft::Temperature(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                draft
                    .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
                    .map_err(|error| format!("invalid temperature sweep settings: {error}"))?;
                AnalysisSpec::Parametric
            }
            AnalysisDraft::Corner(draft) => {
                let mut draft = draft.clone();
                draft.ensure_initialized();
                crate::simulation::dialog::corner::to_config(
                    &draft,
                    &state.sim_setup.run_set,
                    state.sim_setup.reference_pvt,
                )
                .map_err(|error| format!("invalid corner settings: {error}"))?;
                AnalysisSpec::Corner
            }
            AnalysisDraft::Disto(draft) => AnalysisSpec::Disto {
                start_freq: parse_spice_value_checked(&draft.sweep.fstart)
                    .map_err(|e| format!("invalid DISTO start frequency: {e}"))?,
                stop_freq: parse_spice_value_checked(&draft.sweep.fstop)
                    .map_err(|e| format!("invalid DISTO stop frequency: {e}"))?,
                points_per_unit: rspice_simulation::analysis_preparation::parse_positive_points(
                    &draft.sweep.points,
                    "disto_points",
                )?,
                sweep: rspice_simulation::analysis_preparation::map_frequency_sweep(
                    draft.sweep.sweep,
                ),
                f2_over_f1: rspice_simulation::analysis_preparation::parse_optional_spice_value(
                    &draft.f2_over_f1,
                )
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
                seed: draft.parsed_seed()?,
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
                    table_options: crate::simulation::config::AcDataTableOptions {
                        from_netlist: !config.authored,
                        parameter_columns: config.parameter_columns,
                    },
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
        };
        spec.validate()?;
        Ok(spec)
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
                    .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Noise(noise))
                    .unwrap(),
                controller
                    .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Disto(disto))
                    .unwrap(),
                rspice_simulation::analysis_preparation::build_stb_spec(&state.sim_setup.stb)
                    .unwrap(),
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
            .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Noise(draft))
            .expect("exact noise draft parses");
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
            .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Disto(draft))
            .expect("exact DISTO draft parses");
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
            let mut draft = state.sim_setup.tran.clone();
            draft.stop = typed.to_owned();
            state.sim_setup.tran.stop = "stale singleton".to_owned();

            let spec = controller
                .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Transient(draft))
                .unwrap_or_else(|error| panic!("a stop time of {typed} must be accepted: {error}"));
            let AnalysisSpec::Transient { stop_time, .. } = spec else {
                panic!("the authored transient draft builds a transient analysis");
            };
            assert!(
                (stop_time - expected).abs() <= expected * 1e-12,
                "a stop time typed {typed} reached the draft as {stop_time:e}, not {expected:e}"
            );
        }

        // The spellings no deck reader has stay refused, rather than reaching a
        // run at a decade the engine would not have agreed with.
        for typed in ["1micro", "1wat", "1k5"] {
            let state = AppState::default();
            let mut draft = state.sim_setup.tran.clone();
            draft.stop = typed.to_owned();
            assert!(
                controller
                    .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Transient(draft))
                    .is_err(),
                "a stop time of {typed} must not reach a run"
            );
        }
    }

    #[test]
    fn ac_and_dc_specs_read_exact_authored_drafts() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        let mut ac = state.sim_setup.ac.clone();
        ac.fstart = "3k".to_owned();
        ac.fstop = "30k".to_owned();
        ac.points = "41".to_owned();
        ac.sweep = 2;
        state.sim_setup.ac.fstart = "stale singleton".to_owned();
        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::Ac(ac))
            .expect("the authored AC draft builds its spec");
        assert!(matches!(
            spec,
            AnalysisSpec::Ac {
                start_freq: 3_000.0,
                stop_freq: 30_000.0,
                points_per_unit: 41,
                sweep: FrequencySweep::Linear,
            }
        ));

        let mut dc = state.sim_setup.dc.clone();
        dc.source = "VEXACT".to_owned();
        dc.stop = "3".to_owned();
        state.sim_setup.dc.source = "stale singleton".to_owned();
        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &AnalysisDraft::DcSweep(dc))
            .expect("the authored DC draft builds its spec");
        assert!(matches!(
            spec,
            AnalysisSpec::DcSweep {
                source_name,
                stop: 3.0,
                ..
            } if source_name == "VEXACT"
        ));
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
        let draft = AnalysisDraft::Optimization(state.sim_setup.optimization.clone());
        state.sim_setup.optimization.fd_step = "not a number".to_owned();

        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &draft)
            .expect("an authored optimization draft builds its spec");
        let line = controller
            .analysis_spec_to_spice_line(&analysis_inputs(&state), &draft, &spec)
            .expect("the authored optimization draft builds its card");
        assert!(line.contains("fd=2.500000e-3"), "{line}");
        let AnalysisSpec::Optimization {
            fd_step,
            initial_step,
            min_step,
            ..
        } = spec
        else {
            panic!("the optimization draft builds an optimization spec");
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
        let mut invalid_draft = draft;
        let AnalysisDraft::Optimization(invalid) = &mut invalid_draft else {
            unreachable!()
        };
        invalid.min_step = "0.5".to_owned();
        let error = controller
            .analysis_draft_spec(&analysis_inputs(&state), &invalid_draft)
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
            let mut draft = state.sim_setup.sp.clone();
            draft.do_noise = requested;
            state.sim_setup.sp.do_noise = !requested;
            let draft = AnalysisDraft::SParameter(draft);
            let spec = controller
                .analysis_draft_spec(&analysis_inputs(&state), &draft)
                .unwrap();
            assert!(
                matches!(&spec, AnalysisSpec::SParameter { do_noise, .. } if *do_noise == requested)
            );
            let line = controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &draft, &spec)
                .unwrap();
            assert_eq!(line.split_whitespace().nth(5) == Some("1"), requested);
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
                .analysis_draft_spec(&analysis_inputs(&AppState::default()), &draft)
                .expect("default draft parses");
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
                .analysis_draft_spec(&analysis_inputs(&AppState::default()), &draft)
                .expect("periodic network draft parses");
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

        let spec = rspice_simulation::analysis_preparation::build_pss_spec(&state.sim_setup.pss)
            .expect("PSS spec builds");
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
        let mut state = AppState::default();
        state.sim_setup.pss.ensure_initialized();
        state.sim_setup.pss.integration_method_idx = 0;
        state.sim_setup.pss.tone_sources.clear();
        state.sim_setup.pss.osc_mode = true;
        state.sim_setup.pss.osc_node = "osc_out".to_owned();

        let spec = rspice_simulation::analysis_preparation::build_pss_spec(&state.sim_setup.pss)
            .expect("PSS spec builds");
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
    fn fourier_spec_and_command_use_frozen_draft() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.fourier.ensure_initialized();
        state.sim_setup.fourier.compute_thd = false;
        state.sim_setup.fourier.normalize = true;
        state.sim_setup.fourier.fundamental = "2k".into();
        state.sim_setup.fourier.stop_time = "5m".into();
        let draft = AnalysisDraft::Fourier(state.sim_setup.fourier.clone());
        state.sim_setup.fourier.fundamental = "3k".into();

        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &draft)
            .expect("Fourier spec builds");
        assert!(matches!(
            &spec,
            AnalysisSpec::Fourier {
                compute_thd: false,
                normalize: true,
                ..
            }
        ));
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &draft, &spec)
                .unwrap()
                .starts_with(".four 2000 ")
        );
    }

    #[test]
    fn envelope_spec_uses_frozen_draft_execution_fields() {
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
        let draft = AnalysisDraft::Envelope(Box::new(state.sim_setup.envelope.clone()));
        state.sim_setup.envelope.carrier_tones = "not a frequency".into();

        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &draft)
            .expect("Envelope spec builds");
        assert_eq!(
            spec,
            AnalysisSpec::Envelope {
                multirate: None,
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
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &draft, &spec)
                .unwrap()
                .starts_with(".envlp carriers=[1Meg,2.5Meg] ")
        );
    }

    #[test]
    fn soa_spec_and_command_use_frozen_draft() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.soa.ensure_initialized();
        state.sim_setup.soa.stop_time = "2m".into();
        state.sim_setup.soa.step_time = "2u".into();
        let draft = AnalysisDraft::Soa(state.sim_setup.soa.clone());
        state.sim_setup.soa.stop_time = "not a time".into();

        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &draft)
            .unwrap();
        assert!(matches!(
            &spec,
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                ..
            } if *stop_time == 2e-3 && *step_time == 2e-6
        ));
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &draft, &spec)
                .unwrap()
                .starts_with(".soa stop=0.002 step=0.000002 ")
        );
    }

    #[test]
    fn pole_zero_sensitivity_and_transfer_function_use_frozen_drafts() {
        let controller = SimulationController::new();
        let mut state = AppState::default();

        state.sim_setup.pz.ensure_initialized();
        state.sim_setup.pz.input_pos = "PZ_IN".into();
        let pz_draft = AnalysisDraft::PoleZero(state.sim_setup.pz.clone());
        state.sim_setup.pz.input_pos.clear();
        let pz_spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &pz_draft)
            .unwrap();
        assert!(matches!(
            &pz_spec,
            AnalysisSpec::PoleZero { input_node, .. } if input_node == "PZ_IN"
        ));
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &pz_draft, &pz_spec)
                .unwrap()
                .contains("PZ_IN")
        );

        state.sim_setup.sens.ensure_initialized();
        state.sim_setup.sens.output_expr = "V(SENS_OUT)".into();
        let sens_draft = AnalysisDraft::Sensitivity(state.sim_setup.sens.clone());
        state.sim_setup.sens.output_expr.clear();
        let sens_spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &sens_draft)
            .unwrap();
        assert!(matches!(
            &sens_spec,
            AnalysisSpec::Sensitivity { output_var, .. } if output_var == "V(SENS_OUT)"
        ));
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &sens_draft, &sens_spec)
                .unwrap()
                .contains("V(SENS_OUT)")
        );

        state.sim_setup.xf.ensure_initialized();
        state.sim_setup.xf.input_source = "VTF".into();
        state.sim_setup.xf.output_expression = "V(TF_OUT)".into();
        let tf_draft = AnalysisDraft::TransferFunction(state.sim_setup.xf.clone());
        state.sim_setup.xf.input_source.clear();
        let tf_spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &tf_draft)
            .unwrap();
        assert!(matches!(
            &tf_spec,
            AnalysisSpec::Tf { input_source, .. } if input_source == "VTF"
        ));
        assert!(
            controller
                .analysis_spec_to_spice_line(&analysis_inputs(&state), &tf_draft, &tf_spec)
                .unwrap()
                .contains("VTF")
        );
    }

    #[test]
    fn operating_point_uses_frozen_draft_and_live_reference_temperature() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.sim_setup.reference_pvt.temperature_celsius = 42.0;
        let mut authored = state.sim_setup.op.clone();
        authored.temperature_mode_idx = 0;
        authored.temperature = "unfinished edit".into();
        state.sim_setup.op.temperature_mode_idx = usize::MAX;
        let draft = AnalysisDraft::OperatingPoint(authored.clone());
        let spec = controller
            .analysis_draft_spec(&analysis_inputs(&state), &draft)
            .unwrap();
        assert!(matches!(
            spec,
            AnalysisSpec::DcOp {
                temperature_celsius: 42.0,
                ..
            }
        ));

        authored.temperature_mode_idx = 2;
        authored.temperature = "-15".into();
        let spec = controller
            .analysis_draft_spec(
                &analysis_inputs(&state),
                &AnalysisDraft::OperatingPoint(authored),
            )
            .unwrap();
        assert!(matches!(
            spec,
            AnalysisSpec::DcOp {
                temperature_celsius: -15.0,
                ..
            }
        ));
    }
}
