//! Dispatch for periodic steady-state analyses and the small-signal
//! analyses that linearize about one.

use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::execution::{ResolvedExecutionDependencies, TransientTrajectoryArtifact};
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, PssMethod};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

pub(super) fn run_periodic_spec(
    spec: AnalysisSpec,
    netlist: &str,
    source_path: Option<&Path>,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::Pss {
            method,
            fundamental_freq,
            tone_sources,
            tstab_periods,
            points_per_period,
            tolerance,
            oscillator_mode,
            oscillator_node,
            num_harmonics,
        } => match method {
            PssMethod::Shooting => run_pss(
                netlist,
                svc_runner::PssRunConfig {
                    fundamental_freq,
                    tone_sources,
                    tstab_periods,
                    points_per_period,
                    num_harmonics,
                    tolerance,
                    oscillator_mode,
                    oscillator_node,
                },
                source_path,
                dependencies,
                abort,
            ),
            PssMethod::HarmonicBalance => Err(SimulationError::InvalidConfig(
                "legacy HB-PSS mode is not executable; use a Harmonic Balance analysis".to_owned(),
            )),
        },
        AnalysisSpec::PssSpectrum { num_harmonics } => {
            run_pss_spectrum(num_harmonics, dependencies, abort)
        }
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            oversample,
            collocation_points,
            max_mixing_order,
            use_krylov,
            gmres_restart,
            source_stepping,
            verbose,
        } => {
            let mut hb_tones = Vec::with_capacity(tones.len());
            for tone in tones {
                super::ensure_not_aborted(abort)?;
                hb_tones.push(svc_runner::HbToneRunConfig {
                    frequency: tone.frequency,
                    harmonics: tone.harmonics,
                    source: tone.source,
                    name: tone.name,
                });
            }
            let hb_cfg = svc_runner::HbRunConfig {
                tones: hb_tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            };
            run_harmonic_balance(netlist, &hb_cfg, true, source_path, abort)
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            additional_carrier_tones,
            stop_time,
            num_harmonics,
            envelope_step,
            modulation_sources,
            initial_periodic_solve,
            adaptive_mode,
            extraction_path,
        } => run_envelope(
            netlist,
            svc_runner::EnvelopeRunConfig {
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path,
            },
            source_path,
            abort,
        ),
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
            compute_thd,
            normalize,
        } => run_fourier(
            fundamental_freq,
            FourierRunRequest {
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            },
            dependencies.transient_trajectory().map_err(|error| {
                SimulationError::InvalidConfig(format!(
                    "Fourier dependency artifact is unavailable: {error}"
                ))
            })?,
            abort,
        ),
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } => run_disto(
            netlist,
            DistoRunRequest {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            },
            source_path,
            abort,
        ),
        other => Err(super::misrouted_spec_error("periodic", &other)),
    }
}

fn run_pss(
    netlist: &str,
    config: svc_runner::PssRunConfig,
    source_path: Option<&Path>,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let artifact = dependencies.dc_operating_point_seed().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "shooting PSS operating-point dependency is unavailable: {error}"
        ))
    })?;
    let actual_source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(netlist);
    if artifact.effective_source_content_digest() != actual_source_digest {
        return Err(SimulationError::InvalidConfig(format!(
            "shooting PSS source identity {} does not match its bound operating-point source {}",
            actual_source_digest,
            artifact.effective_source_content_digest()
        )));
    }
    let dc_seed = artifact.core_seed().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "shooting PSS operating-point seed is invalid: {error}"
        ))
    })?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pss_analysis_with_dc_seed_and_source_path_and_abort(
            netlist,
            &config,
            source_path,
            &dc_seed,
            artifact.temperature_celsius(),
            artifact.supply_voltage(),
            artifact.nominal_supply_voltage(),
            abort,
        )
    })?;

    let time = data.time;
    let periodic_state = data.operating_point;
    let mut waveforms = HashMap::with_capacity(data.waveforms.len());
    for (name, values) in data.waveforms {
        super::ensure_not_aborted(abort)?;
        let waveform_time = clone_values_with_abort(&time, abort)?;
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, waveform_time, values),
        );
    }

    Ok(SimulationResult::Transient {
        time,
        waveforms,
        measurements: Vec::new(),
        periodic_state: Some(periodic_state),
        // A shooting-PSS result is not produced by the transient driver, so
        // the driver's convergence metrics would not describe it.
        convergence: Default::default(),
    })
}

/// The harmonic spectrum of a converged periodic steady state.
///
/// This reads the artifact the PSS task already produced rather than solving
/// the period again, exactly as Fourier reads a transient trajectory. That is
/// also why it is a task of its own: harmonics are indexed by frequency and
/// the periodic waveform by time, and one analysis carries one abscissa.
fn run_pss_spectrum(
    num_harmonics: usize,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let artifact = dependencies.periodic_state().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "PSS spectrum periodic-state dependency is unavailable: {error}"
        ))
    })?;
    let periodic = artifact.operating_point().analysis();
    let period = periodic.period;
    if !period.is_finite() || period <= 0.0 {
        return Err(SimulationError::InvalidConfig(
            "PSS spectrum source state has no valid period".to_owned(),
        ));
    }
    let fundamental = 1.0 / period;

    let node_names = &periodic.result.node_names;
    let mut waveforms = HashMap::new();
    let mut frequencies = Vec::new();
    for (index, waveform) in periodic.result.waveforms.iter().enumerate() {
        super::ensure_not_aborted(abort)?;
        let node_name = node_names
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("n{}", index + 1));
        if node_name == "0" || node_name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let harmonics = svc_runner::compute_fft_harmonics_with_abort(
            &waveform.values,
            fundamental,
            num_harmonics,
            abort,
        )
        .map_err(|error| SimulationError::SolverError(error.to_string()))?;
        if harmonics.is_empty() {
            continue;
        }
        // Every node resolves the same harmonic grid, so the first one that
        // produces a spectrum defines the shared abscissa.
        if frequencies.is_empty() {
            frequencies = harmonics.iter().map(|(freq, _, _)| *freq).collect();
        }
        let mut real = Vec::with_capacity(harmonics.len());
        let mut imaginary = Vec::with_capacity(harmonics.len());
        for (_, magnitude, phase_deg) in &harmonics {
            let radians = phase_deg.to_radians();
            real.push(magnitude * radians.cos());
            imaginary.push(magnitude * radians.sin());
        }
        let name = format!("V({node_name})");
        waveforms.insert(
            name.clone(),
            WaveformData::new_complex(
                name,
                harmonics.iter().map(|(freq, _, _)| *freq).collect(),
                real,
                imaginary,
            ),
        );
    }

    Ok(SimulationResult::Ac {
        frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_harmonic_balance(
    netlist: &str,
    hb_cfg: &svc_runner::HbRunConfig,
    retain_harmonics: bool,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_hb_analysis_with_source_path_and_abort(netlist, hb_cfg, source_path, abort)
    })?;

    let waveforms = if retain_harmonics {
        spectra_to_complex_waveforms(data.spectra, abort)?
    } else {
        let mut waveforms = HashMap::with_capacity(data.dc_voltages.len());
        for (index, (name, voltage)) in data.dc_voltages.into_iter().enumerate() {
            poll_periodically(abort, index)?;
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(name, vec![0.0], vec![voltage], vec![0.0]),
            );
        }
        waveforms
    };
    super::ensure_not_aborted(abort)?;
    let frequencies = waveforms
        .values()
        .next()
        .map(|wf| clone_values_with_abort(&wf.x_values, abort))
        .transpose()?
        .unwrap_or_default();

    Ok(SimulationResult::Ac {
        frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_envelope(
    netlist: &str,
    cfg: svc_runner::EnvelopeRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_envelope_analysis_with_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;
    let mut waveforms = HashMap::with_capacity(data.waveforms.len());
    for (name, values) in data.waveforms {
        super::ensure_not_aborted(abort)?;
        let waveform_time = clone_values_with_abort(&data.time, abort)?;
        let mut real = Vec::with_capacity(values.len());
        let mut imaginary = Vec::with_capacity(values.len());
        for (index, value) in values.into_iter().enumerate() {
            poll_periodically(abort, index)?;
            real.push(value.re);
            imaginary.push(value.im);
        }
        waveforms.insert(
            name.clone(),
            WaveformData::new_complex_time_domain(name, waveform_time, real, imaginary),
        );
    }

    Ok(SimulationResult::Transient {
        time: data.time,
        waveforms,
        measurements: Vec::new(),
        periodic_state: None,
        // As above: this waveform comes from the periodic solver.
        convergence: Default::default(),
    })
}

struct FourierRunRequest {
    num_harmonics: usize,
    output_node: String,
    output_ref: String,
    start_time: f64,
    stop_time: f64,
    compute_thd: bool,
    normalize: bool,
}

fn run_fourier(
    fundamental_freq: f64,
    request: FourierRunRequest,
    trajectory: &TransientTrajectoryArtifact,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let FourierRunRequest {
        num_harmonics,
        output_node,
        output_ref,
        start_time,
        stop_time,
        compute_thd,
        normalize,
    } = request;
    let output_unit = if normalize {
        "ratio"
    } else {
        fourier_output_unit(&output_node)
    };
    let output_ref = (!output_ref.trim().is_empty()).then_some(output_ref);
    let cfg = svc_runner::FourierRunConfig {
        fundamental_freq,
        num_harmonics,
        output_node,
        output_ref,
        start_time,
        stop_time,
        compute_thd,
        normalize,
    };
    cfg.validate().map_err(SimulationError::InvalidConfig)?;
    let data = fourier_from_transient_artifact(trajectory, &cfg, abort)?;

    let mut real = Vec::with_capacity(data.response.len());
    let mut imaginary = Vec::with_capacity(data.response.len());
    for (value_idx, value) in data.response.iter().enumerate() {
        poll_periodically(abort, value_idx)?;
        real.push(value.re);
        imaginary.push(value.im);
    }
    let mut waveforms = HashMap::new();
    let spectrum_name = format!("{} Spectrum", data.output_label);
    let mut spectrum = WaveformData::new_complex(
        spectrum_name.clone(),
        clone_values_with_abort(&data.frequencies, abort)?,
        real,
        imaginary,
    );
    spectrum.y_unit = output_unit.to_string();
    waveforms.insert(spectrum_name, spectrum);
    if let Some(thd_percent) = data.thd_percent {
        insert_scalar_waveform(
            &mut waveforms,
            "THD(%)".to_string(),
            vec![fundamental_freq],
            vec![thd_percent],
            "%",
            "Hz",
        );
    }
    insert_scalar_waveform(
        &mut waveforms,
        "DC".to_string(),
        vec![0.0],
        vec![data.dc_component],
        output_unit,
        "Hz",
    );

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn fourier_from_transient_artifact(
    trajectory: &TransientTrajectoryArtifact,
    config: &svc_runner::FourierRunConfig,
    abort: &dyn AbortSignal,
) -> Result<svc_runner::FourierData, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let node_values = trajectory.waveform(&config.output_node).ok_or_else(|| {
        SimulationError::InvalidConfig(format!(
            "Fourier output node '{}' is absent from bound transient artifact",
            config.output_node.trim()
        ))
    })?;
    let reference_values = config
        .output_ref
        .as_deref()
        .filter(|reference| {
            let reference = reference.trim();
            !reference.is_empty() && !reference.eq_ignore_ascii_case("0")
        })
        .map(|reference| {
            trajectory.waveform(reference).ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Fourier reference node '{}' is absent from bound transient artifact",
                    reference.trim()
                ))
            })
        })
        .transpose()?;

    let mut signal = Vec::with_capacity(trajectory.time().len());
    for (index, &value) in node_values.iter().enumerate() {
        poll_periodically(abort, index)?;
        signal.push(reference_values.map_or(value, |reference| value - reference[index]));
    }

    super::run_abort_aware_service(abort, || {
        svc_runner::run_fourier_from_signal_with_abort(trajectory.time(), &signal, config, abort)
    })
}

struct DistoRunRequest {
    start_freq: f64,
    stop_freq: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
    f2_over_f1: Option<f64>,
}

fn run_disto(
    netlist: &str,
    request: DistoRunRequest,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let DistoRunRequest {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        f2_over_f1,
    } = request;
    let sweep = match sweep {
        FrequencySweep::Decade => svc_runner::DistoFrequencySweep::Decade,
        FrequencySweep::Octave => svc_runner::DistoFrequencySweep::Octave,
        FrequencySweep::Linear => svc_runner::DistoFrequencySweep::Linear,
    };
    let cfg = svc_runner::DistoRunConfig {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        f2_over_f1,
        allow_linearized_fallback: false,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_disto_analysis_with_source_path_and_abort(netlist, &cfg, source_path, abort)
    })?;
    let frequencies = data.frequencies;

    let mut waveforms = HashMap::new();
    for trace in data.traces {
        super::ensure_not_aborted(abort)?;
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} Gain(dB)", trace.name),
            clone_values_with_abort(&frequencies, abort)?,
            trace.fundamental_gain_db,
            "dB",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} HD2(dBc)", trace.name),
            clone_values_with_abort(&frequencies, abort)?,
            trace.hd2_db,
            "dBc",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} HD3(dBc)", trace.name),
            clone_values_with_abort(&frequencies, abort)?,
            trace.hd3_db,
            "dBc",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} THD(%)", trace.name),
            clone_values_with_abort(&frequencies, abort)?,
            trace.thd_percent,
            "%",
            "Hz",
        );
        if let Some(imd2) = trace.imd2_db {
            insert_scalar_waveform(
                &mut waveforms,
                format!("{} IMD2(dBc)", trace.name),
                clone_values_with_abort(&frequencies, abort)?,
                imd2,
                "dBc",
                "Hz",
            );
        }
        if let Some(imd3) = trace.imd3_db {
            insert_scalar_waveform(
                &mut waveforms,
                format!("{} IMD3(dBc)", trace.name),
                clone_values_with_abort(&frequencies, abort)?,
                imd3,
                "dBc",
                "Hz",
            );
        }
    }

    Ok(SimulationResult::Ac {
        frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn spectra_to_complex_waveforms(
    spectra: impl IntoIterator<Item = (String, Vec<(f64, f64, f64)>)>,
    abort: &dyn AbortSignal,
) -> Result<HashMap<String, WaveformData>, SimulationError> {
    let mut waveforms = HashMap::new();
    for (name, spectrum) in spectra {
        super::ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(spectrum.len());
        let mut real = Vec::with_capacity(spectrum.len());
        let mut imaginary = Vec::with_capacity(spectrum.len());
        for (component_idx, (frequency, magnitude, phase_degrees)) in
            spectrum.into_iter().enumerate()
        {
            poll_periodically(abort, component_idx)?;
            let phase = phase_degrees.to_radians();
            frequencies.push(frequency);
            real.push(magnitude * phase.cos());
            imaginary.push(magnitude * phase.sin());
        }
        waveforms.insert(
            name.clone(),
            WaveformData::new_complex(name, frequencies, real, imaginary),
        );
    }
    Ok(waveforms)
}

fn clone_values_with_abort(
    values: &[f64],
    abort: &dyn AbortSignal,
) -> Result<Vec<f64>, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let mut cloned = Vec::with_capacity(values.len());
    for (value_idx, value) in values.iter().enumerate() {
        poll_periodically(abort, value_idx)?;
        cloned.push(*value);
    }
    super::ensure_not_aborted(abort)?;
    Ok(cloned)
}

#[inline]
fn poll_periodically(abort: &dyn AbortSignal, index: usize) -> Result<(), SimulationError> {
    const POLL_STRIDE: usize = 64;
    if index.is_multiple_of(POLL_STRIDE) {
        super::ensure_not_aborted(abort)?;
    }
    Ok(())
}

fn insert_scalar_waveform(
    waveforms: &mut HashMap<String, WaveformData>,
    name: String,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    y_unit: &str,
    _x_unit: &str,
) {
    waveforms.insert(
        name.clone(),
        WaveformData {
            name,
            x_values,
            y_values,
            y_unit: y_unit.to_string(),
            is_complex: false,
            y_imag: None,
        },
    );
}

fn fourier_output_unit(output_expression: &str) -> &'static str {
    if svc_runner::fourier_output_is_current(output_expression) {
        "A"
    } else {
        "V"
    }
}

#[cfg(test)]
mod tests {
    use super::fourier_output_unit;

    #[test]
    fn fourier_results_preserve_voltage_and_current_dimensions() {
        assert_eq!(fourier_output_unit("V(out)"), "V");
        assert_eq!(fourier_output_unit("  i(Rload)"), "A");
    }
}
