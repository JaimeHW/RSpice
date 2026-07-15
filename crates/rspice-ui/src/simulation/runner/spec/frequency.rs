use std::collections::HashMap;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, SpPort};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::{SimulationError, SpecExecutionOptions};

pub(super) fn run_frequency_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
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
            SParameterRequest {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            },
            abort,
        ),
        AnalysisSpec::Tf => run_tf(netlist, options, abort),
        AnalysisSpec::Pac => run_pac(netlist, options, abort),
        AnalysisSpec::Pxf => run_pxf(netlist, options, abort),
        AnalysisSpec::Pnoise => run_pnoise(netlist, options, abort),
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
        } => run_stb(
            netlist,
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
            abort,
        ),
        AnalysisSpec::Pstb => run_pstb(netlist, options, abort),
        other => Err(super::misrouted_spec_error("frequency", &other)),
    }
}

struct SParameterRequest {
    start_freq: f64,
    stop_freq: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
    z0: f64,
    ports: Vec<SpPort>,
}

fn run_sparameter(
    netlist: &str,
    request: SParameterRequest,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let sweep = match request.sweep {
        FrequencySweep::Decade => svc_runner::SParameterSweep::Decade,
        FrequencySweep::Octave => svc_runner::SParameterSweep::Octave,
        FrequencySweep::Linear => svc_runner::SParameterSweep::Linear,
    };
    let mut configured_ports = Vec::with_capacity(request.ports.len());
    for port in request.ports {
        super::ensure_not_aborted(abort)?;
        configured_ports.push(svc_runner::SParameterPort {
            node_pos: port.node_pos,
            node_neg: port.node_neg,
            z0: port.z0,
        });
    }
    let cfg = svc_runner::SParameterRunConfig {
        start_freq: request.start_freq,
        stop_freq: request.stop_freq,
        points_per_unit: request.points_per_unit,
        sweep,
        z0: request.z0,
        ports: configured_ports,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_sparameter_analysis_with_abort(netlist, &cfg, abort)
    })?;
    let mut waveforms = HashMap::new();
    for row in 0..data.num_ports {
        super::ensure_not_aborted(abort)?;
        for col in 0..data.num_ports {
            super::ensure_not_aborted(abort)?;
            let name = if data.num_ports <= 9 {
                format!("S{}{}", row + 1, col + 1)
            } else {
                format!("S{}_{}", row + 1, col + 1)
            };
            let trace = &data.s[row][col];
            let waveform = complex_waveform(name.clone(), &data.frequencies, trace, abort)?;
            waveforms.insert(name, waveform);
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
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(tf_cfg) = options.tf {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_tf_analysis_with_config_and_abort(netlist, &tf_cfg, abort)
        })?
    } else {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_tf_analysis_with_abort(netlist, abort)
        })?
    };

    let mut waveforms = HashMap::new();
    let transfer_name = format!("H({}/{})", data.output_label, data.input_source);
    let transfer = complex_waveform(
        transfer_name.clone(),
        &data.frequencies,
        &data.transfer,
        abort,
    )?;
    waveforms.insert(transfer_name, transfer);
    insert_group_delay(&mut waveforms, data.group_delay, abort)?;

    if let Some(zin) = data.input_impedance {
        let zin_name = format!("Zin({})", data.input_source);
        let waveform = complex_waveform(zin_name.clone(), &data.frequencies, &zin, abort)?;
        waveforms.insert(zin_name, waveform);
    }

    if let Some(zout) = data.output_impedance {
        let zout_name = format!("Zout({})", data.output_label);
        let waveform = complex_waveform(zout_name.clone(), &data.frequencies, &zout, abort)?;
        waveforms.insert(zout_name, waveform);
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
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pac_cfg = required_execution_option(
        options.pac,
        "PAC analysis requires explicit PAC execution options",
        abort,
    )?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pac_analysis_with_abort(netlist, &pac_cfg, abort)
    })?;

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms: spectra_to_complex_waveforms(data.spectra, abort)?,
        measurements: Vec::new(),
    })
}

fn run_pxf(
    netlist: &str,
    options: SpecExecutionOptions,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pxf_cfg = required_execution_option(
        options.pxf,
        "PXF analysis requires explicit PXF execution options",
        abort,
    )?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pxf_analysis_with_config_and_abort(netlist, &pxf_cfg, abort)
    })?;

    let mut waveforms = HashMap::new();
    let transfer_name = format!(
        "H(sb{}->sb{}, {})",
        data.input_sideband, data.output_sideband, data.output_label
    );
    let transfer = complex_waveform(
        transfer_name.clone(),
        &data.frequencies,
        &data.transfer,
        abort,
    )?;
    waveforms.insert(transfer_name, transfer);
    insert_group_delay(&mut waveforms, data.group_delay, abort)?;

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_pnoise(
    netlist: &str,
    options: SpecExecutionOptions,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(pnoise_cfg) = options.pnoise {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_pnoise_analysis_with_config_and_abort(netlist, &pnoise_cfg, abort)
        })?
    } else {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_pnoise_analysis_with_abort(netlist, abort)
        })?
    };

    let freq_len = data.frequencies.len().max(1);
    let mut contributors = HashMap::with_capacity(data.contributors.len());
    for (name, percentage) in data.contributors {
        super::ensure_not_aborted(abort)?;
        contributors.insert(name, vec![percentage; freq_len]);
    }

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
    sweep: FrequencySweep,
    points_per_decade: usize,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_stb_analysis_with_sweep_and_source_path_and_abort(
            netlist,
            &probe_node,
            start_freq,
            stop_freq,
            stb_sweep_type(sweep),
            points_per_decade,
            None,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
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

fn stb_sweep_type(sweep: FrequencySweep) -> rspice_core::analysis::advanced::stb::StbSweepType {
    match sweep {
        FrequencySweep::Decade => rspice_core::analysis::advanced::stb::StbSweepType::Decade,
        FrequencySweep::Octave => rspice_core::analysis::advanced::stb::StbSweepType::Octave,
        FrequencySweep::Linear => rspice_core::analysis::advanced::stb::StbSweepType::Linear,
    }
}

fn run_pstb(
    netlist: &str,
    options: SpecExecutionOptions,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pstb_cfg = required_execution_option(
        options.pstb,
        "PSTB analysis requires explicit PSTB execution options",
        abort,
    )?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pstb_analysis_with_config_and_abort(netlist, &pstb_cfg, abort)
    })?;

    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
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
    abort: &dyn AbortSignal,
) -> Result<HashMap<String, WaveformData>, SimulationError> {
    let mut waveforms = HashMap::new();
    for (name, spectrum) in spectra {
        super::ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(spectrum.len());
        let mut real = Vec::with_capacity(spectrum.len());
        let mut imaginary = Vec::with_capacity(spectrum.len());
        for (frequency, magnitude, phase_degrees) in spectrum {
            super::ensure_not_aborted(abort)?;
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

fn insert_group_delay(
    waveforms: &mut HashMap<String, WaveformData>,
    group_delay: Option<Vec<(f64, f64)>>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    if let Some(gd) = group_delay {
        let mut frequencies = Vec::with_capacity(gd.len());
        let mut delays = Vec::with_capacity(gd.len());
        for (frequency, delay) in gd {
            super::ensure_not_aborted(abort)?;
            frequencies.push(frequency);
            delays.push(delay);
        }
        insert_scalar_waveform(
            waveforms,
            "group_delay".to_string(),
            frequencies,
            delays,
            "s",
            "Hz",
        );
    }
    Ok(())
}

fn complex_waveform(
    name: String,
    frequencies: &[f64],
    values: &[num_complex::Complex64],
    abort: &dyn AbortSignal,
) -> Result<WaveformData, SimulationError> {
    let mut real = Vec::with_capacity(values.len());
    let mut imaginary = Vec::with_capacity(values.len());
    for value in values {
        super::ensure_not_aborted(abort)?;
        real.push(value.re);
        imaginary.push(value.im);
    }
    Ok(WaveformData::new_complex(
        name,
        frequencies.to_vec(),
        real,
        imaginary,
    ))
}

fn required_execution_option<T>(
    option: Option<T>,
    message: &str,
    abort: &dyn AbortSignal,
) -> Result<T, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let option = option.ok_or_else(|| SimulationError::InvalidConfig(message.to_string()));
    super::ensure_not_aborted(abort)?;
    option
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
