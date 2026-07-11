use std::collections::HashMap;
use std::path::Path;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

pub(super) fn run_periodic_spec(
    spec: AnalysisSpec,
    netlist: &str,
    source_path: Option<&Path>,
) -> Result<SimulationResult, SimulationError> {
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
            let hb_tones = tones
                .into_iter()
                .map(|tone| svc_runner::HbToneRunConfig {
                    frequency: tone.frequency,
                    harmonics: tone.harmonics,
                    source: tone.source,
                    name: tone.name,
                })
                .collect();
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
            run_harmonic_balance(netlist, &hb_cfg, source_path)
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
        ),
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } => run_disto(
            netlist,
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
            source_path,
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
) -> Result<SimulationResult, SimulationError> {
    let data = svc_runner::run_pss_analysis_with_source_path(
        netlist,
        fundamental_freq,
        num_harmonics,
        tolerance,
        source_path,
    )
    .map_err(SimulationError::InvalidConfig)?;

    let time = data.time;
    let waveforms: HashMap<String, WaveformData> = data
        .waveforms
        .into_iter()
        .map(|(name, values)| {
            (
                name.clone(),
                WaveformData::new_time_domain(name, time.clone(), values),
            )
        })
        .collect();

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
) -> Result<SimulationResult, SimulationError> {
    let data = svc_runner::run_hb_analysis_with_source_path(netlist, hb_cfg, source_path)
        .map_err(SimulationError::InvalidConfig)?;

    let waveforms = spectra_to_complex_waveforms(data.spectra);
    let frequencies = waveforms
        .values()
        .next()
        .map(|wf| wf.x_values.clone())
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
) -> Result<SimulationResult, SimulationError> {
    let cfg = svc_runner::EnvelopeRunConfig {
        fundamental_freq,
        stop_time,
        num_harmonics,
        max_step,
    };
    let data = svc_runner::run_envelope_analysis_with_source_path(netlist, &cfg, source_path)
        .map_err(SimulationError::InvalidConfig)?;
    let waveforms: HashMap<String, WaveformData> = data
        .waveforms
        .into_iter()
        .map(|(name, values)| {
            (
                name.clone(),
                WaveformData::new_time_domain(name, data.time.clone(), values),
            )
        })
        .collect();

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
    let data = svc_runner::run_fourier_analysis_with_source_path(netlist, &cfg, source_path)
        .map_err(SimulationError::InvalidConfig)?;

    let mut waveforms = HashMap::new();
    waveforms.insert(
        format!("{} Spectrum", data.output_label),
        WaveformData::new_complex(
            format!("{} Spectrum", data.output_label),
            data.frequencies.clone(),
            data.response.iter().map(|value| value.re).collect(),
            data.response.iter().map(|value| value.im).collect(),
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

fn run_disto(
    netlist: &str,
    start_freq: f64,
    stop_freq: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
    f2_over_f1: Option<f64>,
    source_path: Option<&Path>,
) -> Result<SimulationResult, SimulationError> {
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
    let data = svc_runner::run_disto_analysis_with_source_path(netlist, &cfg, source_path)
        .map_err(SimulationError::InvalidConfig)?;
    let frequencies = data.frequencies;

    let mut waveforms = HashMap::new();
    for trace in data.traces {
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} Gain(dB)", trace.name),
            frequencies.clone(),
            trace.fundamental_gain_db,
            "dB",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} HD2(dBc)", trace.name),
            frequencies.clone(),
            trace.hd2_db,
            "dBc",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} HD3(dBc)", trace.name),
            frequencies.clone(),
            trace.hd3_db,
            "dBc",
            "Hz",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("{} THD(%)", trace.name),
            frequencies.clone(),
            trace.thd_percent,
            "%",
            "Hz",
        );
        if let Some(imd2) = trace.imd2_db {
            insert_scalar_waveform(
                &mut waveforms,
                format!("{} IMD2(dBc)", trace.name),
                frequencies.clone(),
                imd2,
                "dBc",
                "Hz",
            );
        }
        if let Some(imd3) = trace.imd3_db {
            insert_scalar_waveform(
                &mut waveforms,
                format!("{} IMD3(dBc)", trace.name),
                frequencies.clone(),
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
) -> HashMap<String, WaveformData> {
    spectra
        .into_iter()
        .map(|(name, spectrum)| {
            let freqs: Vec<f64> = spectrum.iter().map(|(freq, _, _)| *freq).collect();
            let real: Vec<f64> = spectrum
                .iter()
                .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().cos())
                .collect();
            let imag: Vec<f64> = spectrum
                .iter()
                .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().sin())
                .collect();
            (
                name.clone(),
                WaveformData::new_complex(name, freqs, real, imag),
            )
        })
        .collect()
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
