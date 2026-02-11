use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::super::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use super::super::engine_bridge::EngineBridge;
use super::super::multi_run::AnalysisSpec;
use super::super::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use super::{SimulationError, SpecExecutionOptions};

pub(super) fn run_spec_request(
    bridge: &EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    abort_flag: &Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use crate::services::simulation_runner as svc_runner;

    if abort_flag.load(Ordering::SeqCst) {
        return Err(SimulationError::Aborted);
    }

    if let Some(config) = analysis_config_from_spec(&spec) {
        return bridge.run_with_abort(&config, netlist, abort_flag);
    }

    match spec {
        AnalysisSpec::MonteCarlo => {
            let data = svc_runner::run_monte_carlo_analysis(netlist)
                .map_err(SimulationError::InvalidConfig)?;
            let variables = data
                .variables
                .into_iter()
                .map(|var| MonteCarloVariableResult {
                    name: var.name,
                    mean: var.mean,
                    std_dev: var.std_dev,
                    min: var.min,
                    max: var.max,
                    histogram: var.histogram,
                    bin_edges: var.bin_edges,
                })
                .collect();
            Ok(SimulationResult::MonteCarlo {
                runs_requested: data.runs_requested,
                runs_completed: data.runs_completed,
                num_failures: data.num_failures,
                all_converged: data.all_converged,
                variables,
            })
        }
        AnalysisSpec::Parametric => {
            let data = if let Some(temp_cfg) = options.temp {
                svc_runner::run_parametric_analysis_with_config(netlist, &temp_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_parametric_analysis(netlist)
                    .map_err(SimulationError::InvalidConfig)?
            };
            let sweep_values = data.sweep_values;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .voltages
                .into_iter()
                .map(|(name, values)| {
                    (
                        name.clone(),
                        WaveformData::new_time_domain(name, sweep_values.clone(), values),
                    )
                })
                .collect();
            Ok(SimulationResult::Parametric {
                target: data.target,
                sweep_values,
                waveforms,
                num_failures: data.num_failures,
            })
        }
        AnalysisSpec::Corner => {
            let data = if let Some(corner_cfg) = options.corner {
                svc_runner::run_corner_analysis_with_config(netlist, &corner_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_corner_analysis(netlist).map_err(SimulationError::InvalidConfig)?
            };
            let x_values = data.x_values;
            let x_label = data.x_label;
            let x_unit = data.x_unit;
            let temperatures_c = data.temperatures_c;
            let corner_labels = data.corner_labels;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .voltages
                .into_iter()
                .map(|(name, values)| {
                    let waveform = WaveformData {
                        name: name.clone(),
                        x_values: x_values.clone(),
                        y_values: values,
                        y_unit: "V".to_string(),
                        x_unit: x_unit.clone(),
                        is_complex: false,
                        y_imag: None,
                    };
                    (name.clone(), waveform)
                })
                .collect();
            Ok(SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures: data.num_failures,
            })
        }
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            let cfg = svc_runner::ReliabilityRunConfig {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            };
            let data = svc_runner::run_reliability_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            for device in &data.device_results {
                let mut years = Vec::with_capacity(data.years.len());
                let mut vth = Vec::with_capacity(data.years.len());
                let mut mobility = Vec::with_capacity(data.years.len());
                let mut rds = Vec::with_capacity(data.years.len());

                for years_key in &data.years {
                    let key = format!("{}y", years_key);
                    let shift = device.shifts.get(&key).cloned().unwrap_or_default();
                    years.push(*years_key);
                    vth.push(shift.vth_shift);
                    mobility.push(shift.mobility_shift);
                    rds.push(shift.rds_shift);
                }

                let vth_name = format!("DVTH({})", device.device_id);
                waveforms.insert(
                    vth_name.clone(),
                    WaveformData {
                        name: vth_name,
                        x_values: years.clone(),
                        y_values: vth,
                        y_unit: "V".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );

                let mobility_name = format!("DMU({})", device.device_id);
                waveforms.insert(
                    mobility_name.clone(),
                    WaveformData {
                        name: mobility_name,
                        x_values: years.clone(),
                        y_values: mobility,
                        y_unit: "ratio".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );

                let rds_name = format!("DRDS({})", device.device_id);
                waveforms.insert(
                    rds_name.clone(),
                    WaveformData {
                        name: rds_name,
                        x_values: years,
                        y_values: rds,
                        y_unit: "ratio".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            Ok(SimulationResult::Reliability {
                years: data.years,
                waveforms,
                device_results: data.device_results,
            })
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
            let cfg = svc_runner::OptimizationRunConfig {
                variables: variables
                    .into_iter()
                    .map(|var| svc_runner::OptimizationVariable {
                        name: var.name,
                        min: var.min,
                        max: var.max,
                        initial: var.initial,
                    })
                    .collect(),
                objective_node,
                objective_ref,
                goal: match goal {
                    crate::simulation::multi_run::OptimizationGoal::Minimize => {
                        svc_runner::OptimizationGoalMode::Minimize
                    }
                    crate::simulation::multi_run::OptimizationGoal::Maximize => {
                        svc_runner::OptimizationGoalMode::Maximize
                    }
                    crate::simulation::multi_run::OptimizationGoal::Target => {
                        svc_runner::OptimizationGoalMode::Target
                    }
                },
                target,
                algorithm: match algorithm {
                    crate::simulation::multi_run::OptimizationAlgorithm::GradientDescent => {
                        svc_runner::OptimizationAlgorithmMode::GradientDescent
                    }
                    crate::simulation::multi_run::OptimizationAlgorithm::PatternSearch => {
                        svc_runner::OptimizationAlgorithmMode::PatternSearch
                    }
                    crate::simulation::multi_run::OptimizationAlgorithm::SimulatedAnnealing => {
                        svc_runner::OptimizationAlgorithmMode::SimulatedAnnealing
                    }
                },
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            };

            let data = svc_runner::run_optimization_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "OPT_COST".to_string(),
                WaveformData {
                    name: "OPT_COST".to_string(),
                    x_values: data.iterations.clone(),
                    y_values: data.costs.clone(),
                    y_unit: "cost".to_string(),
                    x_unit: "iter".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            for (name, values) in &data.variable_traces {
                let wf_name = format!("OPT_{}", name);
                waveforms.insert(
                    wf_name.clone(),
                    WaveformData {
                        name: wf_name,
                        x_values: data.iterations.clone(),
                        y_values: values.clone(),
                        y_unit: "value".to_string(),
                        x_unit: "iter".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            Ok(SimulationResult::Optimization {
                iterations: data.iterations,
                waveforms,
                best_cost: data.best_cost,
                best_variables: data.best_variables,
                converged: data.converged,
            })
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
            let cfg = svc_runner::SoaRunConfig {
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
            };
            let data = svc_runner::run_soa_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "SOA_VIOLATION_COUNT".to_string(),
                WaveformData {
                    name: "SOA_VIOLATION_COUNT".to_string(),
                    x_values: data.time.clone(),
                    y_values: data.violation_count.clone(),
                    y_unit: "count".to_string(),
                    x_unit: "s".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            Ok(SimulationResult::Soa {
                time: data.time,
                waveforms,
                violations: data.violations,
            })
        }
        AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
        } => {
            let data =
                svc_runner::run_pss_analysis(netlist, fundamental_freq, num_harmonics, tolerance)
                    .map_err(SimulationError::InvalidConfig)?;

            let time = data.time;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .waveforms
                .into_iter()
                .map(|(name, values)| {
                    (
                        name.clone(),
                        WaveformData::new_time_domain(name, time.clone(), values),
                    )
                })
                .collect();

            Ok(SimulationResult::Transient { time, waveforms })
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
            let hb_tones: Vec<svc_runner::HbToneRunConfig> = tones
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
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            };
            let data = svc_runner::run_hb_analysis(netlist, &hb_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .spectra
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
                .collect();
            let frequencies = waveforms
                .values()
                .next()
                .map(|wf| wf.x_values.clone())
                .unwrap_or_default();

            Ok(SimulationResult::Ac {
                frequencies,
                waveforms,
            })
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
                    svc_runner::SParameterSweep::Decade
                }
                crate::simulation::multi_run::FrequencySweep::Octave => {
                    svc_runner::SParameterSweep::Octave
                }
                crate::simulation::multi_run::FrequencySweep::Linear => {
                    svc_runner::SParameterSweep::Linear
                }
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
            let mut waveforms = std::collections::HashMap::new();
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
            })
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
        } => {
            let cfg = svc_runner::EnvelopeRunConfig {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            };
            let data = svc_runner::run_envelope_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
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
            })
        }
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
        } => {
            let output_ref = (!output_ref.trim().is_empty()).then_some(output_ref);
            let cfg = svc_runner::FourierRunConfig {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            };
            let data = svc_runner::run_fourier_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                format!("{} Spectrum", data.output_label),
                WaveformData::new_complex(
                    format!("{} Spectrum", data.output_label),
                    data.frequencies.clone(),
                    data.response.iter().map(|value| value.re).collect(),
                    data.response.iter().map(|value| value.im).collect(),
                ),
            );
            waveforms.insert(
                "THD(%)".to_string(),
                WaveformData {
                    name: "THD(%)".to_string(),
                    x_values: vec![fundamental_freq],
                    y_values: vec![data.thd_percent],
                    y_unit: "%".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "DC".to_string(),
                WaveformData {
                    name: "DC".to_string(),
                    x_values: vec![0.0],
                    y_values: vec![data.dc_component],
                    y_unit: "V".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
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
                    svc_runner::DistoFrequencySweep::Decade
                }
                crate::simulation::multi_run::FrequencySweep::Octave => {
                    svc_runner::DistoFrequencySweep::Octave
                }
                crate::simulation::multi_run::FrequencySweep::Linear => {
                    svc_runner::DistoFrequencySweep::Linear
                }
            };
            let cfg = svc_runner::DistoRunConfig {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
                allow_linearized_fallback: false,
            };
            let data = svc_runner::run_disto_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let frequencies = data.frequencies;
            let traces = data.traces;

            let mut waveforms: std::collections::HashMap<String, WaveformData> =
                std::collections::HashMap::new();
            for trace in traces {
                waveforms.insert(
                    format!("{} Gain(dB)", trace.name),
                    WaveformData {
                        name: format!("{} Gain(dB)", trace.name),
                        x_values: frequencies.clone(),
                        y_values: trace.fundamental_gain_db,
                        y_unit: "dB".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
                waveforms.insert(
                    format!("{} HD2(dBc)", trace.name),
                    WaveformData {
                        name: format!("{} HD2(dBc)", trace.name),
                        x_values: frequencies.clone(),
                        y_values: trace.hd2_db,
                        y_unit: "dBc".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
                waveforms.insert(
                    format!("{} HD3(dBc)", trace.name),
                    WaveformData {
                        name: format!("{} HD3(dBc)", trace.name),
                        x_values: frequencies.clone(),
                        y_values: trace.hd3_db,
                        y_unit: "dBc".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
                waveforms.insert(
                    format!("{} THD(%)", trace.name),
                    WaveformData {
                        name: format!("{} THD(%)", trace.name),
                        x_values: frequencies.clone(),
                        y_values: trace.thd_percent,
                        y_unit: "%".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
                if let Some(imd2) = trace.imd2_db {
                    waveforms.insert(
                        format!("{} IMD2(dBc)", trace.name),
                        WaveformData {
                            name: format!("{} IMD2(dBc)", trace.name),
                            x_values: frequencies.clone(),
                            y_values: imd2,
                            y_unit: "dBc".to_string(),
                            x_unit: "Hz".to_string(),
                            is_complex: false,
                            y_imag: None,
                        },
                    );
                }
                if let Some(imd3) = trace.imd3_db {
                    waveforms.insert(
                        format!("{} IMD3(dBc)", trace.name),
                        WaveformData {
                            name: format!("{} IMD3(dBc)", trace.name),
                            x_values: frequencies.clone(),
                            y_values: imd3,
                            y_unit: "dBc".to_string(),
                            x_unit: "Hz".to_string(),
                            is_complex: false,
                            y_imag: None,
                        },
                    );
                }
            }

            Ok(SimulationResult::Ac {
                frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Tf => {
            let data = if let Some(tf_cfg) = options.tf {
                svc_runner::run_tf_analysis_with_config(netlist, &tf_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_tf_analysis(netlist).map_err(SimulationError::InvalidConfig)?
            };

            let mut waveforms: std::collections::HashMap<String, WaveformData> =
                std::collections::HashMap::new();
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

            if let Some(gd) = data.group_delay {
                let (freqs, delays): (Vec<f64>, Vec<f64>) = gd.into_iter().unzip();
                waveforms.insert(
                    "group_delay".to_string(),
                    WaveformData {
                        name: "group_delay".to_string(),
                        x_values: freqs,
                        y_values: delays,
                        y_unit: "s".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

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
            })
        }
        AnalysisSpec::Pac => {
            let pac_cfg = options.pac.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PAC analysis requires explicit PAC execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pac_analysis(netlist, &pac_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .spectra
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
                .collect();

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Pxf => {
            let pxf_cfg = options.pxf.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PXF analysis requires explicit PXF execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pxf_analysis_with_config(netlist, &pxf_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms: std::collections::HashMap<String, WaveformData> =
                std::collections::HashMap::new();
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

            if let Some(gd) = data.group_delay {
                let (freqs, delays): (Vec<f64>, Vec<f64>) = gd.into_iter().unzip();
                waveforms.insert(
                    "group_delay".to_string(),
                    WaveformData {
                        name: "group_delay".to_string(),
                        x_values: freqs,
                        y_values: delays,
                        y_unit: "s".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Pnoise => {
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
            })
        }
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
        } => {
            let data = svc_runner::run_stb_analysis(
                netlist,
                &probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            )
            .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "Loop Gain (dB)".to_string(),
                WaveformData {
                    name: "Loop Gain (dB)".to_string(),
                    x_values: data.frequencies.clone(),
                    y_values: data.loop_gain_db,
                    y_unit: "dB".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Loop Phase (deg)".to_string(),
                WaveformData {
                    name: "Loop Phase (deg)".to_string(),
                    x_values: data.frequencies.clone(),
                    y_values: data.loop_phase_deg,
                    y_unit: "deg".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Pstb => {
            let pstb_cfg = options.pstb.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PSTB analysis requires explicit PSTB execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pstb_analysis_with_config(netlist, &pstb_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "Floquet |lambda|".to_string(),
                WaveformData {
                    name: "Floquet |lambda|".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.multiplier_magnitude,
                    y_unit: "".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Stability Margin (dB)".to_string(),
                WaveformData {
                    name: "Stability Margin (dB)".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.stability_margin_db,
                    y_unit: "dB".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Mode Damping (1/s)".to_string(),
                WaveformData {
                    name: "Mode Damping (1/s)".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.mode_damping,
                    y_unit: "1/s".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Probe Mode Participation".to_string(),
                WaveformData {
                    name: "Probe Mode Participation".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.probe_mode_participation,
                    y_unit: "".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );

            Ok(SimulationResult::Ac {
                frequencies: data.mode_indices,
                waveforms,
            })
        }
        unsupported => Err(SimulationError::InvalidConfig(format!(
            "{:?} is not supported by SimulationRunner::start_spec",
            unsupported.run_type()
        ))),
    }
}

pub(super) fn analysis_config_from_spec(spec: &AnalysisSpec) -> Option<AnalysisConfig> {
    match spec {
        AnalysisSpec::DcOp => Some(AnalysisConfig::DcOp),
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => Some(AnalysisConfig::DcSweep(DcSweepConfig {
            source: source_name.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            source2: source2.clone(),
            start2: *start2,
            stop2: *stop2,
            step2: *step2,
        })),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
        } => Some(AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => Some(AnalysisConfig::Ac(AcAnalysisConfig {
            sweep_type: ac_sweep_type_from_spec(*sweep),
            num_points: *points_per_unit,
            start_freq: *start_freq,
            stop_freq: *stop_freq,
        })),
        AnalysisSpec::Noise {
            output_node,
            start_freq,
            stop_freq,
            points_per_decade,
            ..
        } => Some(AnalysisConfig::Noise(NoiseAnalysisConfig {
            output_node: output_node.clone(),
            reference_node: "0".to_string(),
            input_source: "V1".to_string(),
            sweep_type: AcSweepType::Decade,
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
        } => Some(AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: input_node.clone(),
            input_ref: input_ref.clone(),
            output_node: output_node.clone(),
            output_ref: output_ref.clone(),
            transfer_type: transfer_type.clone(),
            analysis_type: pz_analysis_type_from_spec(analysis_type),
        })),
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
        } => Some(AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: output_var.clone(),
            ac_mode: *ac_mode,
            frequency: *frequency,
        })),
        _ => None,
    }
}

#[inline]
fn ac_sweep_type_from_spec(sweep: crate::simulation::multi_run::FrequencySweep) -> AcSweepType {
    match sweep {
        crate::simulation::multi_run::FrequencySweep::Decade => AcSweepType::Decade,
        crate::simulation::multi_run::FrequencySweep::Octave => AcSweepType::Octave,
        crate::simulation::multi_run::FrequencySweep::Linear => AcSweepType::Linear,
    }
}

#[inline]
fn pz_analysis_type_from_spec(mode: &str) -> PzAnalysisType {
    if mode.eq_ignore_ascii_case("POL") {
        PzAnalysisType::PolesOnly
    } else if mode.eq_ignore_ascii_case("ZER") {
        PzAnalysisType::ZerosOnly
    } else {
        PzAnalysisType::PoleZero
    }
}
