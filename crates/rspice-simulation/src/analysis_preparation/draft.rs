//! Lower an authored draft using borrowed design, setup, and run evidence.

use super::AnalysisInputs;
use rspice_simulation_contract::analysis_spec::{AnalysisSpec, SpPort};
use rspice_simulation_contract::config::FrequencySweep;
use rspice_simulation_contract::spice_value::parse_spice_value_checked;

pub fn analysis_draft_spec<'a, R, A, W>(
    state: &AnalysisInputs<'a, R, A>,
    draft: &rspice_simulation_contract::analysis_draft::AnalysisDraft,
) -> Result<AnalysisSpec, String>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;

    if let Some(error) = draft.manifest_configuration_error() {
        return Err(error);
    }
    let spec = match draft {
        AnalysisDraft::OperatingPoint(draft) => {
            crate::analysis_preparation::build_op_spec(state, draft)?
        }
        AnalysisDraft::Transient(draft) => AnalysisSpec::Transient {
            stop_time: parse_spice_value_checked(&draft.stop)
                .map_err(|e| format!("invalid stop time: {}", e))?,
            step_time: parse_spice_value_checked(&draft.step)
                .map_err(|e| format!("invalid step time: {}", e))?,
            start_time: parse_spice_value_checked(&draft.start)
                .map_err(|e| format!("invalid start time: {}", e))?,
            max_timestep: crate::analysis_preparation::parse_optional_spice_value(&draft.max_step)
                .map_err(|e| format!("invalid max step: {}", e))?,
            uic: draft.uic,
        },
        AnalysisDraft::Ac(draft) => AnalysisSpec::Ac {
            start_freq: parse_spice_value_checked(&draft.fstart)
                .map_err(|e| format!("invalid start frequency: {}", e))?,
            stop_freq: parse_spice_value_checked(&draft.fstop)
                .map_err(|e| format!("invalid stop frequency: {}", e))?,
            points_per_unit: crate::analysis_preparation::parse_positive_points(
                &draft.points,
                "ac_points",
            )?,
            sweep: crate::analysis_preparation::map_frequency_sweep(draft.sweep),
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
        AnalysisDraft::Pss(draft) => crate::analysis_preparation::build_pss_spec(draft)?,
        AnalysisDraft::HarmonicBalance(draft) => {
            crate::analysis_preparation::build_harmonic_balance_spec(draft)?
        }
        AnalysisDraft::Stb(draft) => crate::analysis_preparation::build_stb_spec(draft)?,
        AnalysisDraft::SParameter(draft) => {
            crate::analysis_preparation::build_sp_spec(state.schematic, draft)?
        }
        AnalysisDraft::Envelope(draft) => crate::analysis_preparation::build_envelope_spec(draft)?,
        AnalysisDraft::Fourier(draft) => crate::analysis_preparation::build_fourier_spec(draft)?,
        AnalysisDraft::Optimization(draft) => crate::analysis_preparation::build_optimization_spec(
            state.sim_setup,
            state.plan_payloads,
            draft,
        )?,
        AnalysisDraft::Soa(draft) => crate::analysis_preparation::build_soa_spec(draft)?,
        AnalysisDraft::PoleZero(draft) => crate::analysis_preparation::build_pole_zero_spec(draft)?,
        AnalysisDraft::Sensitivity(draft) => {
            crate::analysis_preparation::build_sensitivity_spec(draft)?
        }
        AnalysisDraft::TransferFunction(draft) => {
            crate::analysis_preparation::build_tf_spec(draft)?
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
            draft
                .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
                .map_err(|error| format!("invalid corner settings: {error}"))?;
            AnalysisSpec::Corner
        }
        AnalysisDraft::Disto(draft) => AnalysisSpec::Disto {
            start_freq: parse_spice_value_checked(&draft.sweep.fstart)
                .map_err(|e| format!("invalid DISTO start frequency: {e}"))?,
            stop_freq: parse_spice_value_checked(&draft.sweep.fstop)
                .map_err(|e| format!("invalid DISTO stop frequency: {e}"))?,
            points_per_unit: crate::analysis_preparation::parse_positive_points(
                &draft.sweep.points,
                "disto_points",
            )?,
            sweep: crate::analysis_preparation::map_frequency_sweep(draft.sweep.sweep),
            f2_over_f1: crate::analysis_preparation::parse_optional_spice_value(&draft.f2_over_f1)
                .map_err(|e| format!("invalid DISTO f2/f1 ratio: {e}"))?,
        },
        AnalysisDraft::Noise(draft) => {
            let mut config = draft.to_config()?;
            config.temperature_kelvin = state.sim_setup.reference_pvt.temperature_celsius + 273.15;
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
                table_options: rspice_simulation_contract::config::AcDataTableOptions {
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
            contributor_limit: parse_usize(&draft.contributor_limit, "DCMATCH contributor limit")?,
            include_process: draft.include_process,
            include_mismatch: draft.include_mismatch,
            normalized_contributions: draft.normalized_contributions,
            contribution_threshold:
                rspice_simulation_contract::drafts::dc_mismatch_share_threshold(
                    &draft.share_threshold,
                )?,
        },
    };
    spec.validate()?;
    Ok(spec)
}

fn parse_si(text: &str, field: &str) -> Result<f64, String> {
    rspice_simulation_contract::options::parse_si_value(text)
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
    draft: &rspice_simulation_contract::drafts::FrequencySweepDraft,
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
    ports: &[rspice_simulation_contract::periodic_network_draft::NetworkPortDraft],
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
