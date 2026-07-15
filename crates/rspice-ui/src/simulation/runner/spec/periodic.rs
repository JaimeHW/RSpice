use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

pub(super) fn run_periodic_spec(
    spec: AnalysisSpec,
    netlist: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
        } => run_pss(
            netlist,
            fundamental_freq,
            num_harmonics,
            tolerance,
            source_path,
            abort,
        ),
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
            run_harmonic_balance(netlist, &hb_cfg, source_path, abort)
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
        } => run_envelope(
            netlist,
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
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
        } => run_fourier(
            netlist,
            fundamental_freq,
            FourierRunRequest {
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            },
            source_path,
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
    fundamental_freq: f64,
    num_harmonics: usize,
    tolerance: f64,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pss_analysis_with_source_path_and_abort(
            netlist,
            fundamental_freq,
            num_harmonics,
            tolerance,
            source_path,
            abort,
        )
    })?;

    let time = data.time;
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
    })
}

fn run_harmonic_balance(
    netlist: &str,
    hb_cfg: &svc_runner::HbRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_hb_analysis_with_source_path_and_abort(netlist, hb_cfg, source_path, abort)
    })?;

    let waveforms = spectra_to_complex_waveforms(data.spectra, abort)?;
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
    fundamental_freq: f64,
    stop_time: f64,
    num_harmonics: usize,
    max_step: Option<f64>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let cfg = svc_runner::EnvelopeRunConfig {
        fundamental_freq,
        stop_time,
        num_harmonics,
        max_step,
    };
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
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, waveform_time, values),
        );
    }

    Ok(SimulationResult::Transient {
        time: data.time,
        waveforms,
        measurements: Vec::new(),
    })
}

struct FourierRunRequest {
    num_harmonics: usize,
    output_node: String,
    output_ref: String,
    start_time: f64,
    stop_time: f64,
}

fn run_fourier(
    netlist: &str,
    fundamental_freq: f64,
    request: FourierRunRequest,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let FourierRunRequest {
        num_harmonics,
        output_node,
        output_ref,
        start_time,
        stop_time,
    } = request;
    let output_ref = (!output_ref.trim().is_empty()).then_some(output_ref);
    let cfg = svc_runner::FourierRunConfig {
        fundamental_freq,
        num_harmonics,
        output_node,
        output_ref,
        start_time,
        stop_time,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_fourier_analysis_with_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;

    let mut real = Vec::with_capacity(data.response.len());
    let mut imaginary = Vec::with_capacity(data.response.len());
    for (value_idx, value) in data.response.iter().enumerate() {
        poll_periodically(abort, value_idx)?;
        real.push(value.re);
        imaginary.push(value.im);
    }
    let mut waveforms = HashMap::new();
    waveforms.insert(
        format!("{} Spectrum", data.output_label),
        WaveformData::new_complex(
            format!("{} Spectrum", data.output_label),
            clone_values_with_abort(&data.frequencies, abort)?,
            real,
            imaginary,
        ),
    );
    insert_scalar_waveform(
        &mut waveforms,
        "THD(%)".to_string(),
        vec![fundamental_freq],
        vec![data.thd_percent],
        "%",
        "Hz",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "DC".to_string(),
        vec![0.0],
        vec![data.dc_component],
        "V",
        "Hz",
    );

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
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
    x_unit: &str,
) {
    waveforms.insert(
        name.clone(),
        WaveformData {
            name,
            x_values,
            y_values,
            y_unit: y_unit.to_string(),
            x_unit: x_unit.to_string(),
            is_complex: false,
            y_imag: None,
        },
    );
}
