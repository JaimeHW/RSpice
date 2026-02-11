use super::*;

impl RunExecutor {
    pub(super) fn execute_analysis(
        spec: &AnalysisSpec,
        netlist: &str,
    ) -> Result<MappedResult, String> {
        use crate::services::simulation_runner;

        match spec {
            AnalysisSpec::DcOp => {
                let sim_result = simulation_runner::run_simulation(netlist);
                if sim_result.success {
                    // Build operating point data from DC result
                    let op_data = sim_result.dc_op.map(|voltages| {
                        let mut op = crate::simulation::result_mapper::OperatingPointMap::default();
                        // Convert Vec<(String, Value)> to HashMap<String, f64>
                        op.node_voltages = voltages
                            .into_iter()
                            .map(|(name, val)| (name, val.into()))
                            .collect();
                        op
                    });
                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::DcOp,
                        status: ResultStatus::Success,
                        op_data,
                        ..Default::default()
                    })
                } else {
                    Err(sim_result
                        .error
                        .unwrap_or_else(|| "DC OP failed".to_string()))
                }
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
            } => match simulation_runner::run_transient_analysis(netlist, *stop_time, *step_time) {
                Ok(data) => {
                    let time: Vec<f64> = data.time.into_iter().collect();
                    let waveforms = data
                        .voltages
                        .into_iter()
                        .map(|(name, values)| {
                            MappedWaveform::time_domain(
                                name,
                                time.clone(),
                                values.into_iter().collect(),
                            )
                        })
                        .collect();

                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Transient,
                        status: ResultStatus::Success,
                        waveforms,
                        ..Default::default()
                    })
                }
                Err(e) => Err(e),
            },
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => {
                let ac_result = simulation_runner::run_ac_analysis(
                    netlist,
                    *start_freq,
                    *stop_freq,
                    *points_per_unit,
                    sweep.runner_keyword(),
                );
                match ac_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let waveforms = data
                            .responses
                            .into_iter()
                            .map(|(name, values)| {
                                let real: Vec<f64> = values.iter().map(|v| v.re).collect();
                                let imag: Vec<f64> = values.iter().map(|v| v.im).collect();
                                MappedWaveform::complex_ac(name, freq.clone(), real, imag)
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Ac,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => {
                let sweep = match sweep {
                    crate::simulation::multi_run::FrequencySweep::Decade => {
                        simulation_runner::DistoFrequencySweep::Decade
                    }
                    crate::simulation::multi_run::FrequencySweep::Octave => {
                        simulation_runner::DistoFrequencySweep::Octave
                    }
                    crate::simulation::multi_run::FrequencySweep::Linear => {
                        simulation_runner::DistoFrequencySweep::Linear
                    }
                };
                let cfg = simulation_runner::DistoRunConfig {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points_per_unit,
                    sweep,
                    f2_over_f1: *f2_over_f1,
                    allow_linearized_fallback: false,
                };
                match simulation_runner::run_disto_analysis(netlist, &cfg) {
                    Ok(data) => {
                        let frequencies = data.frequencies;
                        let warnings = data.warnings;
                        let traces = data.traces;

                        let mut waveforms = Vec::new();
                        let mut measurements = Vec::new();

                        for trace in traces {
                            let signal_name = trace.name.clone();
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} Gain(dB)", signal_name),
                                frequencies.clone(),
                                trace.fundamental_gain_db,
                                "Gain",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} HD2(dBc)", signal_name),
                                frequencies.clone(),
                                trace.hd2_db,
                                "HD2",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} HD3(dBc)", signal_name),
                                frequencies.clone(),
                                trace.hd3_db,
                                "HD3",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} THD(%)", signal_name),
                                frequencies.clone(),
                                trace.thd_percent.clone(),
                                "THD",
                            ));
                            if let Some(imd2) = trace.imd2_db {
                                waveforms.push(MappedWaveform::frequency_domain(
                                    format!("{} IMD2(dBc)", signal_name),
                                    frequencies.clone(),
                                    imd2,
                                    "IMD2",
                                ));
                            }
                            if let Some(imd3) = trace.imd3_db {
                                waveforms.push(MappedWaveform::frequency_domain(
                                    format!("{} IMD3(dBc)", signal_name),
                                    frequencies.clone(),
                                    imd3,
                                    "IMD3",
                                ));
                            }

                            if let Some(max_thd) =
                                trace.thd_percent.iter().copied().reduce(f64::max)
                            {
                                measurements.push(MappedMeasurement {
                                    name: format!("max_thd_percent({})", signal_name),
                                    meas_type: MeasurementType::Custom,
                                    value: max_thd,
                                    unit: "%".to_string(),
                                    signal: signal_name,
                                    status: MeasurementStatus::Success,
                                });
                            }
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Disto,
                            status: if warnings.is_empty() {
                                ResultStatus::Success
                            } else {
                                ResultStatus::Warning
                            },
                            waveforms,
                            measurements,
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => {
                let bridge = crate::simulation::engine_bridge::EngineBridge::new();
                let dc_cfg = AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source_name.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2: source2.clone(),
                    start2: *start2,
                    stop2: *stop2,
                    step2: *step2,
                });

                let sim_result = bridge.run(&dc_cfg, netlist).map_err(|e| e.to_string())?;
                if let crate::simulation::results::SimulationResult::DcSweep {
                    sweep_values,
                    waveforms,
                    ..
                } = sim_result
                {
                    let waveforms = waveforms
                        .into_iter()
                        .map(|(name, wf)| {
                            MappedWaveform::time_domain(
                                name,
                                sweep_values.clone(),
                                wf.y_values.into_iter().collect(),
                            )
                        })
                        .collect();

                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::DcSweep,
                        status: ResultStatus::Success,
                        waveforms,
                        ..Default::default()
                    })
                } else {
                    Err("engine bridge returned unexpected result type for DC sweep".to_string())
                }
            }
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => {
                let noise_result = simulation_runner::run_noise_analysis(
                    netlist,
                    output_node,
                    *start_freq,
                    *stop_freq,
                    *points_per_decade,
                    *temperature,
                );
                match noise_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let output_noise: Vec<f64> = data.output_noise.into_iter().collect();
                        let waveforms = vec![MappedWaveform::frequency_domain(
                            "V(onoise)",
                            freq,
                            output_noise,
                            "Noise Density",
                        )];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Noise,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => {
                let pz_result = simulation_runner::run_pole_zero_analysis(
                    netlist,
                    input_node,
                    input_ref,
                    output_node,
                    output_ref,
                    transfer_type,
                    analysis_type,
                );
                match pz_result {
                    Ok(data) => {
                        let dc_gain_unit = if transfer_type.eq_ignore_ascii_case("CUR") {
                            "V/A"
                        } else {
                            "V/V"
                        };

                        let poles_real: Vec<f64> = data.poles.iter().map(|(re, _)| *re).collect();
                        let poles_imag: Vec<f64> = data.poles.iter().map(|(_, im)| *im).collect();
                        let zeros_real: Vec<f64> = data.zeros.iter().map(|(re, _)| *re).collect();
                        let zeros_imag: Vec<f64> = data.zeros.iter().map(|(_, im)| *im).collect();

                        let mut waveforms = Vec::new();
                        if !poles_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Poles".to_string(),
                                x: poles_real,
                                y: poles_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }
                        if !zeros_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Zeros".to_string(),
                                x: zeros_real,
                                y: zeros_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }

                        let measurements = vec![
                            MappedMeasurement {
                                name: "dc_gain".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.gain,
                                unit: dc_gain_unit.to_string(),
                                signal: "transfer".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "pole_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.poles.len() as f64,
                                unit: "count".to_string(),
                                signal: "poles".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "zero_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.zeros.len() as f64,
                                unit: "count".to_string(),
                                signal: "zeros".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::PoleZero,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => {
                let raw_unit = sensitivity_raw_unit(output_var);

                let sens_result = simulation_runner::run_sensitivity_analysis(
                    netlist, output_var, *ac_mode, *frequency,
                );
                match sens_result {
                    Ok(data) => {
                        let mut measurements: Vec<MappedMeasurement> = data
                            .sensitivities
                            .iter()
                            .map(|(name, raw, _)| MappedMeasurement {
                                name: format!("d({})/d({})", data.output_var, name),
                                meas_type: MeasurementType::Custom,
                                value: *raw,
                                unit: raw_unit.to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            })
                            .collect();

                        measurements.extend(data.sensitivities.iter().map(
                            |(name, _, normalized)| MappedMeasurement {
                                name: format!("norm({})", name),
                                meas_type: MeasurementType::Custom,
                                value: *normalized,
                                unit: "ratio".to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            },
                        ));

                        // Keep most significant normalized sensitivities first for UI consumption.
                        measurements.sort_by(|a, b| {
                            b.value
                                .abs()
                                .partial_cmp(&a.value.abs())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Sensitivity,
                            status: ResultStatus::Success,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => {
                let pss_result = simulation_runner::run_pss_analysis(
                    netlist,
                    *fundamental_freq,
                    *num_harmonics,
                    *tolerance,
                );
                match pss_result {
                    Ok(data) => {
                        let time: Vec<f64> = data.time.into_iter().collect();
                        let waveforms = data
                            .waveforms
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(
                                    name,
                                    time.clone(),
                                    values.into_iter().collect(),
                                )
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pss,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => {
                let hb_tones: Vec<simulation_runner::HbToneRunConfig> = tones
                    .iter()
                    .map(|tone| simulation_runner::HbToneRunConfig {
                        frequency: tone.frequency,
                        harmonics: tone.harmonics,
                        source: tone.source.clone(),
                        name: tone.name.clone(),
                    })
                    .collect();
                let hb_cfg = simulation_runner::HbRunConfig {
                    tones: hb_tones,
                    reltol: *reltol,
                    abstol: *abstol,
                    max_iterations: *max_iterations,
                    damping: *damping,
                    oversample: *oversample,
                    max_mixing_order: *max_mixing_order,
                    use_krylov: *use_krylov,
                    gmres_restart: *gmres_restart,
                    source_stepping: *source_stepping,
                    verbose: *verbose,
                };
                let hb_result = simulation_runner::run_hb_analysis(netlist, &hb_cfg);
                match hb_result {
                    Ok(data) => {
                        let waveforms = data
                            .spectra
                            .into_iter()
                            .flat_map(|(name, spectrum)| {
                                let x: Vec<f64> = spectrum.iter().map(|(f, _, _)| *f).collect();
                                let magnitude: Vec<f64> =
                                    spectrum.iter().map(|(_, m, _)| *m).collect();
                                let phase: Vec<f64> =
                                    spectrum.iter().map(|(_, _, phase)| *phase).collect();
                                vec![
                                    MappedWaveform::frequency_domain(
                                        format!("{} Magnitude", name),
                                        x.clone(),
                                        magnitude,
                                        "Magnitude",
                                    ),
                                    MappedWaveform::frequency_domain(
                                        format!("{} Phase", name),
                                        x,
                                        phase,
                                        "Phase",
                                    ),
                                ]
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::HarmonicBalance,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            } => {
                let sweep = match sweep {
                    crate::simulation::multi_run::FrequencySweep::Decade => {
                        simulation_runner::SParameterSweep::Decade
                    }
                    crate::simulation::multi_run::FrequencySweep::Octave => {
                        simulation_runner::SParameterSweep::Octave
                    }
                    crate::simulation::multi_run::FrequencySweep::Linear => {
                        simulation_runner::SParameterSweep::Linear
                    }
                };
                let cfg = simulation_runner::SParameterRunConfig {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points_per_unit,
                    sweep,
                    z0: *z0,
                    ports: ports
                        .iter()
                        .map(|port| simulation_runner::SParameterPort {
                            node_pos: port.node_pos.clone(),
                            node_neg: port.node_neg.clone(),
                            z0: port.z0,
                        })
                        .collect(),
                };
                let sp_result = simulation_runner::run_sparameter_analysis(netlist, &cfg);
                match sp_result {
                    Ok(data) => {
                        let mut waveforms = Vec::with_capacity(data.num_ports * data.num_ports);
                        for row in 0..data.num_ports {
                            for col in 0..data.num_ports {
                                let name = if data.num_ports <= 9 {
                                    format!("S{}{}", row + 1, col + 1)
                                } else {
                                    format!("S{}_{}", row + 1, col + 1)
                                };
                                let trace = &data.s[row][col];
                                waveforms.push(MappedWaveform::complex_ac(
                                    name,
                                    data.frequencies.clone(),
                                    trace.iter().map(|v| v.re).collect(),
                                    trace.iter().map(|v| v.im).collect(),
                                ));
                            }
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::SParameter,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => {
                let cfg = simulation_runner::EnvelopeRunConfig {
                    fundamental_freq: *fundamental_freq,
                    stop_time: *stop_time,
                    num_harmonics: *num_harmonics,
                    max_step: *max_step,
                };
                let env_result = simulation_runner::run_envelope_analysis(netlist, &cfg);
                match env_result {
                    Ok(data) => {
                        let waveforms = data
                            .waveforms
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(name, data.time.clone(), values)
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Envelope,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            } => {
                let cfg = simulation_runner::FourierRunConfig {
                    fundamental_freq: *fundamental_freq,
                    num_harmonics: *num_harmonics,
                    output_node: output_node.clone(),
                    output_ref: (!output_ref.trim().is_empty()).then_some(output_ref.clone()),
                    start_time: *start_time,
                    stop_time: *stop_time,
                };
                let fourier_result = simulation_runner::run_fourier_analysis(netlist, &cfg);
                match fourier_result {
                    Ok(data) => {
                        let waveforms = vec![
                            MappedWaveform::complex_ac(
                                format!("{} Spectrum", data.output_label),
                                data.frequencies.clone(),
                                data.response.iter().map(|v| v.re).collect(),
                                data.response.iter().map(|v| v.im).collect(),
                            ),
                            MappedWaveform::frequency_domain(
                                "THD(%)",
                                vec![*fundamental_freq],
                                vec![data.thd_percent],
                                "THD",
                            ),
                            MappedWaveform::frequency_domain(
                                "DC",
                                vec![0.0],
                                vec![data.dc_component],
                                "DC",
                            ),
                        ];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Fourier,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::MonteCarlo => {
                let mc_result = simulation_runner::run_monte_carlo_analysis(netlist);
                match mc_result {
                    Ok(data) => {
                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "runs_requested".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.runs_requested as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "runs_completed".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.runs_completed as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "runs_failed".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.num_failures as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];

                        let variable_unit = |name: &str| {
                            if name.starts_with("V(") {
                                "V"
                            } else if name.starts_with("I(") {
                                "A"
                            } else {
                                "unit"
                            }
                        };

                        measurements.extend(data.variables.iter().flat_map(|var| {
                            let unit = variable_unit(&var.name).to_string();
                            [
                                MappedMeasurement {
                                    name: format!("mean({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.mean,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("stddev({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.std_dev,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("min({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.min,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("max({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.max,
                                    unit,
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                            ]
                        }));

                        let waveforms = data
                            .variables
                            .iter()
                            .filter_map(|var| {
                                if var.histogram.is_empty() || var.bin_edges.len() < 2 {
                                    return None;
                                }
                                let x: Vec<f64> = var
                                    .bin_edges
                                    .windows(2)
                                    .map(|window| (window[0] + window[1]) * 0.5)
                                    .collect();
                                let y: Vec<f64> =
                                    var.histogram.iter().map(|count| *count as f64).collect();
                                Some(MappedWaveform {
                                    name: format!("hist({})", var.name),
                                    x,
                                    y,
                                    x_label: "Value".to_string(),
                                    y_label: "Count".to_string(),
                                    y_unit: "count".to_string(),
                                    ..Default::default()
                                })
                            })
                            .collect();

                        let mut warnings = Vec::new();
                        if data.num_failures > 0 {
                            warnings.push(format!(
                                "Monte Carlo converged on {}/{} runs ({} failed)",
                                data.runs_completed, data.runs_requested, data.num_failures
                            ));
                        }
                        if !data.all_converged && data.num_failures == 0 {
                            warnings.push(
                                "Monte Carlo reported non-convergence despite zero explicit failures"
                                    .to_string(),
                            );
                        }

                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::MonteCarlo,
                            status,
                            waveforms,
                            measurements,
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Parametric => {
                let param_result = simulation_runner::run_parametric_analysis(netlist);
                match param_result {
                    Ok(data) => {
                        let sweep_values = data.sweep_values;
                        let waveforms = data
                            .voltages
                            .into_iter()
                            .map(|(name, values)| MappedWaveform {
                                name,
                                x: sweep_values.clone(),
                                y: values.into_iter().collect(),
                                x_label: data.target.clone(),
                                y_label: "Voltage".to_string(),
                                y_unit: "V".to_string(),
                                ..Default::default()
                            })
                            .collect();

                        let warnings = if data.num_failures > 0 {
                            vec![format!(
                                "Parametric sweep completed with {} failed points",
                                data.num_failures
                            )]
                        } else {
                            Vec::new()
                        };

                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Parametric,
                            status,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "sweep_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_points as f64,
                                    unit: "count".to_string(),
                                    signal: "parametric".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "failed_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_failures as f64,
                                    unit: "count".to_string(),
                                    signal: "parametric".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Corner => {
                let corner_result = simulation_runner::run_corner_analysis(netlist);
                match corner_result {
                    Ok(data) => {
                        let x_values = data.x_values;
                        let x_label = data.x_label;
                        let x_unit = data.x_unit;
                        let waveforms = data
                            .voltages
                            .into_iter()
                            .map(|(name, values)| MappedWaveform {
                                name,
                                x: x_values.clone(),
                                y: values.into_iter().collect(),
                                x_label: x_label.clone(),
                                x_unit: x_unit.clone(),
                                y_label: "Voltage".to_string(),
                                y_unit: "V".to_string(),
                                ..Default::default()
                            })
                            .collect();

                        let warnings = if data.num_failures > 0 {
                            vec![format!(
                                "Corner sweep completed with {} failed corners",
                                data.num_failures
                            )]
                        } else {
                            Vec::new()
                        };
                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Corner,
                            status,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "corner_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_points as f64,
                                    unit: "count".to_string(),
                                    signal: "corner".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "failed_corners".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_failures as f64,
                                    unit: "count".to_string(),
                                    signal: "corner".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => {
                let cfg = simulation_runner::ReliabilityRunConfig {
                    target_years: target_years.clone(),
                    enable_hci: *enable_hci,
                    enable_nbti: *enable_nbti,
                    enable_em: *enable_em,
                    min_stress_voltage: *min_stress_voltage,
                };
                let reliability_result =
                    simulation_runner::run_reliability_analysis_with_config(netlist, &cfg);
                match reliability_result {
                    Ok(data) => {
                        let mut waveforms = Vec::new();
                        for device in &data.device_results {
                            let mut x_years = Vec::with_capacity(data.years.len());
                            let mut vth = Vec::with_capacity(data.years.len());
                            let mut mobility = Vec::with_capacity(data.years.len());
                            let mut rds = Vec::with_capacity(data.years.len());

                            for years in &data.years {
                                let key = format!("{}y", years);
                                let shift = device.shifts.get(&key).cloned().unwrap_or_default();
                                x_years.push(*years);
                                vth.push(shift.vth_shift);
                                mobility.push(shift.mobility_shift);
                                rds.push(shift.rds_shift);
                            }

                            waveforms.push(MappedWaveform {
                                name: format!("DVTH({})", device.device_id),
                                x: x_years.clone(),
                                y: vth,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Vth".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "V".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                            waveforms.push(MappedWaveform {
                                name: format!("DMU({})", device.device_id),
                                x: x_years.clone(),
                                y: mobility,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Mobility".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "ratio".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                            waveforms.push(MappedWaveform {
                                name: format!("DRDS({})", device.device_id),
                                x: x_years,
                                y: rds,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Rds".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "ratio".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Reliability,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "devices_analyzed".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.device_results.len() as f64,
                                    unit: "count".to_string(),
                                    signal: "reliability".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "lifetime_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.years.len() as f64,
                                    unit: "count".to_string(),
                                    signal: "reliability".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            } => {
                let cfg = simulation_runner::OptimizationRunConfig {
                    variables: variables
                        .iter()
                        .map(|var| simulation_runner::OptimizationVariable {
                            name: var.name.clone(),
                            min: var.min,
                            max: var.max,
                            initial: var.initial,
                        })
                        .collect(),
                    objective_node: objective_node.clone(),
                    objective_ref: objective_ref.clone(),
                    goal: match goal {
                        crate::simulation::multi_run::OptimizationGoal::Minimize => {
                            simulation_runner::OptimizationGoalMode::Minimize
                        }
                        crate::simulation::multi_run::OptimizationGoal::Maximize => {
                            simulation_runner::OptimizationGoalMode::Maximize
                        }
                        crate::simulation::multi_run::OptimizationGoal::Target => {
                            simulation_runner::OptimizationGoalMode::Target
                        }
                    },
                    target: *target,
                    algorithm: match algorithm {
                        crate::simulation::multi_run::OptimizationAlgorithm::GradientDescent => {
                            simulation_runner::OptimizationAlgorithmMode::GradientDescent
                        }
                        crate::simulation::multi_run::OptimizationAlgorithm::PatternSearch => {
                            simulation_runner::OptimizationAlgorithmMode::PatternSearch
                        }
                        crate::simulation::multi_run::OptimizationAlgorithm::SimulatedAnnealing => {
                            simulation_runner::OptimizationAlgorithmMode::SimulatedAnnealing
                        }
                    },
                    max_iterations: *max_iterations,
                    cost_tolerance: *cost_tolerance,
                    fd_step: *fd_step,
                    initial_step: *initial_step,
                    min_step: *min_step,
                };

                match simulation_runner::run_optimization_analysis_with_config(netlist, &cfg) {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform {
                            name: "Optimization Cost".to_string(),
                            x: data.iterations.clone(),
                            y: data.costs.clone(),
                            x_label: "Iteration".to_string(),
                            y_label: "Cost".to_string(),
                            x_unit: "iter".to_string(),
                            y_unit: "cost".to_string(),
                            is_complex: false,
                            y_imag: None,
                        }];
                        for (name, values) in &data.variable_traces {
                            waveforms.push(MappedWaveform {
                                name: format!("Var({})", name),
                                x: data.iterations.clone(),
                                y: values.clone(),
                                x_label: "Iteration".to_string(),
                                y_label: "Value".to_string(),
                                x_unit: "iter".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "best_cost".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.best_cost,
                                unit: "cost".to_string(),
                                signal: "optimization".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "converged".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.converged { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "optimization".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];
                        for (name, value) in &data.best_variables {
                            measurements.push(MappedMeasurement {
                                name: format!("best_{}", name),
                                meas_type: MeasurementType::Custom,
                                value: *value,
                                unit: "".to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Optimization,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => {
                let cfg = simulation_runner::SoaRunConfig {
                    stop_time: *stop_time,
                    step_time: *step_time,
                    check_vgs_max: *check_vgs_max,
                    max_vgs: *max_vgs,
                    check_vds_max: *check_vds_max,
                    max_vds: *max_vds,
                    check_vbe_max: *check_vbe_max,
                    max_vbe: *max_vbe,
                    check_vce_max: *check_vce_max,
                    max_vce: *max_vce,
                };
                match simulation_runner::run_soa_analysis_with_config(netlist, &cfg) {
                    Ok(data) => {
                        let mut measurements = vec![MappedMeasurement {
                            name: "num_violations".to_string(),
                            meas_type: MeasurementType::Custom,
                            value: data.violations.len() as f64,
                            unit: "count".to_string(),
                            signal: "soa".to_string(),
                            status: MeasurementStatus::Success,
                        }];
                        if let Some(first) = data.violations.first() {
                            measurements.push(MappedMeasurement {
                                name: "first_violation_time".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: first.time,
                                unit: "s".to_string(),
                                signal: "soa".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Soa,
                            status: if data.violations.is_empty() {
                                ResultStatus::Success
                            } else {
                                ResultStatus::Warning
                            },
                            waveforms: vec![MappedWaveform {
                                name: "SOA Violation Count".to_string(),
                                x: data.time,
                                y: data.violation_count,
                                x_label: "Time".to_string(),
                                y_label: "Violation Count".to_string(),
                                x_unit: "s".to_string(),
                                y_unit: "count".to_string(),
                                is_complex: false,
                                y_imag: None,
                            }],
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Tf => {
                let tf_result = simulation_runner::run_tf_analysis(netlist);
                match tf_result {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform::complex_ac(
                            format!("H({}/{})", data.output_label, data.input_source),
                            data.frequencies.clone(),
                            data.transfer.iter().map(|value| value.re).collect(),
                            data.transfer.iter().map(|value| value.im).collect(),
                        )];

                        waveforms.push(MappedWaveform {
                            name: format!("|H({}/{})| dB", data.output_label, data.input_source),
                            x: data.frequencies.clone(),
                            y: data.magnitude_db.clone(),
                            x_label: "Frequency".to_string(),
                            y_label: "Magnitude".to_string(),
                            x_unit: "Hz".to_string(),
                            y_unit: "dB".to_string(),
                            is_complex: false,
                            y_imag: None,
                        });

                        waveforms.push(MappedWaveform {
                            name: format!("Phase(H({}/{}))", data.output_label, data.input_source),
                            x: data.frequencies.clone(),
                            y: data.phase_deg.clone(),
                            x_label: "Frequency".to_string(),
                            y_label: "Phase".to_string(),
                            x_unit: "Hz".to_string(),
                            y_unit: "deg".to_string(),
                            is_complex: false,
                            y_imag: None,
                        });

                        if let Some(group_delay) = data.group_delay {
                            let (x, y): (Vec<f64>, Vec<f64>) = group_delay.into_iter().unzip();
                            waveforms.push(MappedWaveform {
                                name: format!("GroupDelay({})", data.output_label),
                                x,
                                y,
                                x_label: "Frequency".to_string(),
                                y_label: "Group Delay".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        if let Some(zin) = data.input_impedance {
                            waveforms.push(MappedWaveform::complex_ac(
                                format!("Zin({})", data.input_source),
                                data.frequencies.clone(),
                                zin.iter().map(|value| value.re).collect(),
                                zin.iter().map(|value| value.im).collect(),
                            ));
                        }

                        if let Some(zout) = data.output_impedance {
                            waveforms.push(MappedWaveform::complex_ac(
                                format!("Zout({})", data.output_label),
                                data.frequencies.clone(),
                                zout.iter().map(|value| value.re).collect(),
                                zout.iter().map(|value| value.im).collect(),
                            ));
                        }

                        let mut measurements = Vec::new();
                        if let Some(dc_gain) = data.dc_gain {
                            measurements.push(MappedMeasurement {
                                name: "dc_gain".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: dc_gain,
                                unit: "V/V".to_string(),
                                signal: data.output_label.clone(),
                                status: MeasurementStatus::Success,
                            });
                            measurements.push(MappedMeasurement {
                                name: "dc_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: 20.0 * dc_gain.max(1e-30).log10(),
                                unit: "dB".to_string(),
                                signal: data.output_label.clone(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Tf,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pac => {
                let pac_result = simulation_runner::run_pac_analysis_auto(netlist);
                match pac_result {
                    Ok(data) => {
                        let waveforms = data
                            .spectra
                            .into_iter()
                            .map(|(name, spectrum)| {
                                let x: Vec<f64> = spectrum.iter().map(|(f, _, _)| *f).collect();
                                let y: Vec<f64> = spectrum.iter().map(|(_, m, _)| *m).collect();
                                MappedWaveform {
                                    name,
                                    x,
                                    y,
                                    x_label: "Frequency Offset".to_string(),
                                    y_label: "Magnitude".to_string(),
                                    x_unit: "Hz".to_string(),
                                    y_unit: "V".to_string(),
                                    is_complex: false,
                                    y_imag: None,
                                }
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pac,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements: vec![MappedMeasurement {
                                name: "num_sidebands".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.sidebands.len() as f64,
                                unit: "count".to_string(),
                                signal: "pac".to_string(),
                                status: MeasurementStatus::Success,
                            }],
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pxf => {
                let pxf_result = simulation_runner::run_pxf_analysis(netlist);
                match pxf_result {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform::complex_ac(
                            format!(
                                "H(sb{}->sb{}, {})",
                                data.input_sideband, data.output_sideband, data.output_label
                            ),
                            data.frequencies.clone(),
                            data.transfer.iter().map(|value| value.re).collect(),
                            data.transfer.iter().map(|value| value.im).collect(),
                        )];
                        if let Some(group_delay) = data.group_delay {
                            let (x, y): (Vec<f64>, Vec<f64>) = group_delay.into_iter().unzip();
                            waveforms.push(MappedWaveform {
                                name: "group_delay".to_string(),
                                x,
                                y,
                                x_label: "Frequency".to_string(),
                                y_label: "Group Delay".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "input_sideband".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.input_sideband as f64,
                                unit: "index".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "output_sideband".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.output_sideband as f64,
                                unit: "index".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "fundamental_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.fundamental_frequency,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];
                        if let Some(dc_gain) = data.dc_gain {
                            measurements.push(MappedMeasurement {
                                name: "dc_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: 20.0 * dc_gain.norm().max(1e-30).log10(),
                                unit: "dB".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some((peak_freq, peak_gain_db)) = data.peak_gain {
                            measurements.push(MappedMeasurement {
                                name: "peak_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: peak_gain_db,
                                unit: "dB".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                            measurements.push(MappedMeasurement {
                                name: "peak_gain_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: peak_freq,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some(bw) = data.bandwidth_3db {
                            measurements.push(MappedMeasurement {
                                name: "bandwidth_3db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: bw,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some(ugf) = data.unity_gain_freq {
                            measurements.push(MappedMeasurement {
                                name: "unity_gain_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: ugf,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        let status = if data.warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };
                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pxf,
                            status,
                            waveforms,
                            measurements,
                            warnings: data.warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pnoise => {
                let pnoise_result = simulation_runner::run_pnoise_analysis(netlist);
                match pnoise_result {
                    Ok(data) => Ok(Self::map_pnoise_data(data)),
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            } => {
                let stb_result = simulation_runner::run_stb_analysis(
                    netlist,
                    probe_node,
                    *start_freq,
                    *stop_freq,
                    *points_per_decade,
                );
                match stb_result {
                    Ok(data) => Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Stb,
                        status: ResultStatus::Success,
                        waveforms: vec![
                            MappedWaveform {
                                name: "Loop Gain (dB)".to_string(),
                                x: data.frequencies.clone(),
                                y: data.loop_gain_db,
                                x_label: "Frequency".to_string(),
                                y_label: "Loop Gain".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "dB".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Loop Phase (deg)".to_string(),
                                x: data.frequencies,
                                y: data.loop_phase_deg,
                                x_label: "Frequency".to_string(),
                                y_label: "Loop Phase".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "deg".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                        ],
                        measurements: vec![
                            MappedMeasurement {
                                name: "phase_margin".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.phase_margin,
                                unit: "deg".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "gain_margin".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.gain_margin,
                                unit: "dB".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "unity_gain_freq".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.unity_gain_freq,
                                unit: "Hz".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "phase_crossover_freq".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.phase_crossover_freq,
                                unit: "Hz".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "is_stable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.is_stable { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ],
                        ..Default::default()
                    }),
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pstb => {
                let pstb_result = simulation_runner::run_pstb_analysis(netlist);
                match pstb_result {
                    Ok(data) => Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Pstb,
                        status: ResultStatus::Success,
                        waveforms: vec![
                            MappedWaveform {
                                name: "Floquet |lambda|".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.multiplier_magnitude,
                                x_label: "Mode Index".to_string(),
                                y_label: "Multiplier Magnitude".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Stability Margin (dB)".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.stability_margin_db,
                                x_label: "Mode Index".to_string(),
                                y_label: "Stability Margin".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "dB".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Mode Damping (1/s)".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.mode_damping,
                                x_label: "Mode Index".to_string(),
                                y_label: "Damping".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "1/s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Probe Mode Participation".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.probe_mode_participation,
                                x_label: "Mode Index".to_string(),
                                y_label: "Participation".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                        ],
                        measurements: vec![
                            MappedMeasurement {
                                name: "dominant_multiplier".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_multiplier_magnitude,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "min_stability_margin_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.min_stability_margin_db,
                                unit: "dB".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "num_unstable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.num_unstable as f64,
                                unit: "count".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "is_stable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.is_stable { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_branch_ordinal".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_branch_ordinal as f64,
                                unit: "ordinal".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_index".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_index as f64,
                                unit: "index".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_self_transition".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_self_transition,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_column_norm".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_column_norm,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_row_norm".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_row_norm,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_persistence_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_persistence_db,
                                unit: "dB".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "dominant_probe_mode".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_probe_mode as f64,
                                unit: "index".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "dominant_probe_mode_participation".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_probe_mode_participation,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ],
                        ..Default::default()
                    }),
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub(super) fn map_pnoise_data(
        data: crate::services::simulation_runner::PnoiseData,
    ) -> MappedResult {
        let y_label = match data.reference {
            crate::services::simulation_runner::PnoiseReference::Phase => "Phase Noise",
            crate::services::simulation_runner::PnoiseReference::Input => "Input-Referred Noise",
            crate::services::simulation_runner::PnoiseReference::Output => "Output-Referred Noise",
        };
        let y_unit = match data.reference {
            crate::services::simulation_runner::PnoiseReference::Phase => "dBc/Hz",
            _ => "V^2/Hz",
        };
        let waveform_y =
            if data.reference == crate::services::simulation_runner::PnoiseReference::Input {
                data.input_noise
                    .clone()
                    .unwrap_or_else(|| data.output_noise.clone())
            } else {
                data.output_noise.clone()
            };

        let mut measurements = vec![
            MappedMeasurement {
                name: "carrier_frequency".to_string(),
                meas_type: MeasurementType::Custom,
                value: data.carrier_frequency,
                unit: "Hz".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            },
            MappedMeasurement {
                name: "sideband_factor".to_string(),
                meas_type: MeasurementType::Custom,
                value: data.sideband_factor as f64,
                unit: "x".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            },
        ];
        if let Some(total) = data.total_output_noise {
            measurements.push(MappedMeasurement {
                name: "integrated_noise".to_string(),
                meas_type: MeasurementType::Rms,
                value: total,
                unit: "Vrms".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            });
        }

        let status = if data.warnings.is_empty() {
            ResultStatus::Success
        } else {
            ResultStatus::Warning
        };

        MappedResult {
            analysis_type: MappedAnalysisType::Pnoise,
            status,
            waveforms: vec![MappedWaveform {
                name: "pnoise".to_string(),
                x: data.frequencies,
                y: waveform_y,
                x_label: "Frequency Offset".to_string(),
                y_label: y_label.to_string(),
                x_unit: "Hz".to_string(),
                y_unit: y_unit.to_string(),
                is_complex: false,
                y_imag: None,
            }],
            measurements,
            warnings: data.warnings,
            ..Default::default()
        }
    }
}
