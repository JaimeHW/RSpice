use std::collections::HashMap;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, SpPort};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::{SimulationError, SpecExecutionOptions};

pub(super) fn run_frequency_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
) -> Result<SimulationResult, SimulationError> {
    match spec {
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } => run_sparameter(
            netlist,
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        ),
        AnalysisSpec::Tf => run_tf(netlist, options),
        AnalysisSpec::Pac => run_pac(netlist, options),
        AnalysisSpec::Pxf => run_pxf(netlist, options),
        AnalysisSpec::Pnoise => run_pnoise(netlist, options),
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
        } => run_stb(
            netlist,
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
        ),
        AnalysisSpec::Pstb => run_pstb(netlist, options),
        _ => unreachable!("non-frequency spec routed to frequency runner"),
    }
}

fn run_sparameter(
    netlist: &str,
    start_freq: f64,
    stop_freq: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
    z0: f64,
    ports: Vec<SpPort>,
) -> Result<SimulationResult, SimulationError> {
    let sweep = match sweep {
        FrequencySweep::Decade => svc_runner::SParameterSweep::Decade,
        FrequencySweep::Octave => svc_runner::SParameterSweep::Octave,
        FrequencySweep::Linear => svc_runner::SParameterSweep::Linear,
    };
    let cfg = svc_runner::SParameterRunConfig {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        z0,
        ports: ports
            .into_iter()
            .map(|port| svc_runner::SParameterPort {
                node_pos: port.node_pos,
                node_neg: port.node_neg,
                z0: port.z0,
            })
            .collect(),
    };
    let data = svc_runner::run_sparameter_analysis(netlist, &cfg)
        .map_err(SimulationError::InvalidConfig)?;
    let mut waveforms = HashMap::new();
    for row in 0..data.num_ports {
        for col in 0..data.num_ports {
            let name = if data.num_ports <= 9 {
                format!("S{}{}", row + 1, col + 1)
            } else {
                format!("S{}_{}", row + 1, col + 1)
            };
            let trace = &data.s[row][col];
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(
                    name,
                    data.frequencies.clone(),
                    trace.iter().map(|value| value.re).collect(),
                    trace.iter().map(|value| value.im).collect(),
                ),
            );
        }
    }

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_tf(
    netlist: &str,
    options: SpecExecutionOptions,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(tf_cfg) = options.tf {
        svc_runner::run_tf_analysis_with_config(netlist, &tf_cfg)
            .map_err(SimulationError::InvalidConfig)?
    } else {
        svc_runner::run_tf_analysis(netlist).map_err(SimulationError::InvalidConfig)?
    };

    let mut waveforms = HashMap::new();
    let transfer_name = format!("H({}/{})", data.output_label, data.input_source);
    waveforms.insert(
        transfer_name.clone(),
        WaveformData::new_complex(
            transfer_name,
            data.frequencies.clone(),
            data.transfer.iter().map(|value| value.re).collect(),
            data.transfer.iter().map(|value| value.im).collect(),
        ),
    );
    insert_group_delay(&mut waveforms, data.group_delay);

    if let Some(zin) = data.input_impedance {
        let zin_name = format!("Zin({})", data.input_source);
        waveforms.insert(
            zin_name.clone(),
            WaveformData::new_complex(
                zin_name,
                data.frequencies.clone(),
                zin.iter().map(|value| value.re).collect(),
                zin.iter().map(|value| value.im).collect(),
            ),
        );
    }

    if let Some(zout) = data.output_impedance {
        let zout_name = format!("Zout({})", data.output_label);
        waveforms.insert(
            zout_name.clone(),
            WaveformData::new_complex(
                zout_name,
                data.frequencies.clone(),
                zout.iter().map(|value| value.re).collect(),
                zout.iter().map(|value| value.im).collect(),
            ),
        );
    }

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_pac(
    netlist: &str,
    options: SpecExecutionOptions,
) -> Result<SimulationResult, SimulationError> {
    let pac_cfg = options.pac.ok_or_else(|| {
        SimulationError::InvalidConfig(
            "PAC analysis requires explicit PAC execution options".to_string(),
        )
    })?;
    let data =
        svc_runner::run_pac_analysis(netlist, &pac_cfg).map_err(SimulationError::InvalidConfig)?;

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms: spectra_to_complex_waveforms(data.spectra),
        measurements: Vec::new(),
    })
}

fn run_pxf(
    netlist: &str,
    options: SpecExecutionOptions,
) -> Result<SimulationResult, SimulationError> {
    let pxf_cfg = options.pxf.ok_or_else(|| {
        SimulationError::InvalidConfig(
            "PXF analysis requires explicit PXF execution options".to_string(),
        )
    })?;
    let data = svc_runner::run_pxf_analysis_with_config(netlist, &pxf_cfg)
        .map_err(SimulationError::InvalidConfig)?;

    let mut waveforms = HashMap::new();
    let transfer_name = format!(
        "H(sb{}->sb{}, {})",
        data.input_sideband, data.output_sideband, data.output_label
    );
    waveforms.insert(
        transfer_name.clone(),
        WaveformData::new_complex(
            transfer_name,
            data.frequencies.clone(),
            data.transfer.iter().map(|value| value.re).collect(),
            data.transfer.iter().map(|value| value.im).collect(),
        ),
    );
    insert_group_delay(&mut waveforms, data.group_delay);

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_pnoise(
    netlist: &str,
    options: SpecExecutionOptions,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(pnoise_cfg) = options.pnoise {
        svc_runner::run_pnoise_analysis_with_config(netlist, &pnoise_cfg)
            .map_err(SimulationError::InvalidConfig)?
    } else {
        svc_runner::run_pnoise_analysis(netlist).map_err(SimulationError::InvalidConfig)?
    };

    let freq_len = data.frequencies.len().max(1);
    let contributors = data
        .contributors
        .into_iter()
        .map(|(name, percentage)| (name, vec![percentage; freq_len]))
        .collect();

    Ok(SimulationResult::Noise {
        frequencies: data.frequencies,
        output_noise: data.output_noise,
        input_noise: data.input_noise,
        contributors,
        // PNoise carries cyclostationary percentages, not band-integrated
        // mechanism contributions; no ranked summary for it (yet).
        summary: None,
    })
}

fn run_stb(
    netlist: &str,
    probe_node: String,
    start_freq: f64,
    stop_freq: f64,
    points_per_decade: usize,
) -> Result<SimulationResult, SimulationError> {
    let data = svc_runner::run_stb_analysis(
        netlist,
        &probe_node,
        start_freq,
        stop_freq,
        points_per_decade,
    )
    .map_err(SimulationError::InvalidConfig)?;

    let mut waveforms = HashMap::new();
    insert_scalar_waveform(
        &mut waveforms,
        "Loop Gain (dB)".to_string(),
        data.frequencies.clone(),
        data.loop_gain_db,
        "dB",
        "Hz",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Loop Phase (deg)".to_string(),
        data.frequencies.clone(),
        data.loop_phase_deg,
        "deg",
        "Hz",
    );

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_pstb(
    netlist: &str,
    options: SpecExecutionOptions,
) -> Result<SimulationResult, SimulationError> {
    let pstb_cfg = options.pstb.ok_or_else(|| {
        SimulationError::InvalidConfig(
            "PSTB analysis requires explicit PSTB execution options".to_string(),
        )
    })?;
    let data = svc_runner::run_pstb_analysis_with_config(netlist, &pstb_cfg)
        .map_err(SimulationError::InvalidConfig)?;

    let mut waveforms = HashMap::new();
    insert_scalar_waveform(
        &mut waveforms,
        "Floquet |lambda|".to_string(),
        data.mode_indices.clone(),
        data.multiplier_magnitude,
        "",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Stability Margin (dB)".to_string(),
        data.mode_indices.clone(),
        data.stability_margin_db,
        "dB",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Mode Damping (1/s)".to_string(),
        data.mode_indices.clone(),
        data.mode_damping,
        "1/s",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Probe Mode Participation".to_string(),
        data.mode_indices.clone(),
        data.probe_mode_participation,
        "",
        "mode",
    );

    Ok(SimulationResult::Ac {
        frequencies: data.mode_indices,
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

fn insert_group_delay(
    waveforms: &mut HashMap<String, WaveformData>,
    group_delay: Option<Vec<(f64, f64)>>,
) {
    if let Some(gd) = group_delay {
        let (freqs, delays): (Vec<f64>, Vec<f64>) = gd.into_iter().unzip();
        insert_scalar_waveform(
            waveforms,
            "group_delay".to_string(),
            freqs,
            delays,
            "s",
            "Hz",
        );
    }
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
