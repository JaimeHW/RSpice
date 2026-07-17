use super::*;
use std::collections::HashMap;
use std::sync::Arc;

impl SimulationController {
    pub(super) fn convert_to_analysis_result_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata_owned(sim_result, analysis_type, &label)
    }

    pub(super) fn convert_to_analysis_result_with_metadata_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;

        match sim_result {
            SimulationResult::DcOp(dc_result) => {
                let node_voltages = dc_result
                    .node_voltages
                    .into_iter()
                    .map(|(name, value)| OperatingPointValue {
                        name: format!("V({})", name),
                        value,
                        unit: "V".to_string(),
                    })
                    .collect();
                let branch_currents = dc_result
                    .branch_currents
                    .into_iter()
                    .map(|(name, value)| OperatingPointValue {
                        name: format!("I({})", name),
                        value,
                        unit: "A".to_string(),
                    })
                    .collect();

                let state_dc_op = DcOpResult {
                    node_voltages,
                    branch_currents,
                    power_dissipation: Vec::new(),
                };
                let mut result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_dc_op(state_dc_op);
                if let Some(report) = dc_result.device_report {
                    result = result.with_device_op(report);
                }
                result
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => {
                let mut result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_noise_waveforms_owned(
                        frequencies,
                        output_noise,
                        input_noise,
                        contributors,
                    ));
                if let Some(summary) = summary {
                    result = result.with_noise_summary(summary);
                }
                result
            }

            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_sorted_waveforms_with_shared_x_owned(
                    time,
                    waveforms,
                    |name, waveform| (name, waveform.y_values),
                ))
                .with_measurements(measurements),

            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms))
                .with_measurements(measurements),

            SimulationResult::DcSweep {
                sweep_values,
                waveforms,
                measurements,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(sweep_values, waveforms))
                .with_measurements(measurements),

            SimulationResult::PoleZero { poles, zeros, gain } => {
                let payload = AnalysisResultPayload::PoleZero {
                    poles: poles
                        .into_iter()
                        .map(|(real, imaginary)| ComplexResultValue { real, imaginary })
                        .collect(),
                    zeros: zeros
                        .into_iter()
                        .map(|(real, imaginary)| ComplexResultValue { real, imaginary })
                        .collect(),
                    gain,
                };
                self.analysis_result_with_validated_payload(analysis_type, label, payload)
            }

            SimulationResult::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            } => {
                let result_mode = match (ac_mode, frequency_hz) {
                    (false, None) => SensitivityResultMode::Dc,
                    (true, Some(frequency_hz)) => SensitivityResultMode::Ac { frequency_hz },
                    (false, Some(_)) => {
                        return AnalysisResult::failed(
                            1,
                            analysis_type,
                            label.to_string(),
                            "Sensitivity result contract supplied an AC frequency for DC mode",
                        );
                    }
                    (true, None) => {
                        return AnalysisResult::failed(
                            1,
                            analysis_type,
                            label.to_string(),
                            "Sensitivity result contract omitted its AC frequency",
                        );
                    }
                };

                let mut parameters: Vec<_> = sensitivities.keys().cloned().collect();
                parameters.sort();
                if parameters.len() != normalized.len()
                    || parameters
                        .iter()
                        .any(|parameter| !normalized.contains_key(parameter))
                {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        "Sensitivity result contract has misaligned raw and normalized parameters",
                    );
                }
                let rows = parameters
                    .into_iter()
                    .map(|parameter| SensitivityResultRow {
                        raw: sensitivities[&parameter],
                        normalized: normalized[&parameter],
                        parameter,
                    })
                    .collect();
                let payload = AnalysisResultPayload::Sensitivity {
                    output,
                    result_mode,
                    rows,
                };
                self.analysis_result_with_validated_payload(analysis_type, label, payload)
            }

            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => {
                let (waveforms, variables) = self.build_monte_carlo_payload_owned(variables);
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(waveforms)
                    .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
                        seed,
                        runs_requested,
                        runs_completed,
                        failures: num_failures,
                        all_converged,
                        variables,
                    })
            }

            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => {
                let retained_sweep_values = sweep_values.clone();
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(
                        self.build_waveforms_with_shared_x_owned(sweep_values, waveforms),
                    )
                    .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
                        target,
                        sweep_values: retained_sweep_values,
                        failed_points: num_failures,
                    })
            }

            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
            } => {
                let retained_x_values = x_values.clone();
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_waveforms_with_shared_x_owned(x_values, waveforms))
                    .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                        x_values: retained_x_values,
                        x_label,
                        x_unit,
                        temperatures_c,
                        corner_labels,
                        failed_corners: num_failures,
                    })
            }

            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => {
                let retained_years = years.clone();
                let retained_waveforms = self.build_waveforms_with_shared_x_owned(years, waveforms);
                let mut devices = Vec::with_capacity(device_results.len());
                for mut device in device_results {
                    let mut checkpoints = Vec::with_capacity(retained_years.len());
                    for years in &retained_years {
                        let checkpoint_label = format!("{years}y");
                        let Some(shift) = device.shifts.remove(&checkpoint_label) else {
                            return AnalysisResult::failed(
                                1,
                                analysis_type,
                                label.to_string(),
                                format!(
                                    "Invalid retained reliability payload: device '{}' is missing checkpoint {years} years",
                                    device.device_id
                                ),
                            );
                        };
                        checkpoints.push(ReliabilityCheckpointEvidence {
                            years: *years,
                            shift: ReliabilityShiftEvidence {
                                threshold_voltage_shift_v: shift.vth_shift,
                                mobility_shift: shift.mobility_shift,
                                drain_source_resistance_shift: shift.rds_shift,
                            },
                        });
                    }
                    if !device.shifts.is_empty() {
                        return AnalysisResult::failed(
                            1,
                            analysis_type,
                            label.to_string(),
                            format!(
                                "Invalid retained reliability payload: device '{}' has checkpoints outside the retained lifetime axis",
                                device.device_id
                            ),
                        );
                    }
                    devices.push(ReliabilityDeviceEvidence {
                        device_id: device.device_id,
                        stress: ReliabilityStressEvidence {
                            average_gate_stress_v: device.stress.avg_vgs_stress,
                            average_drain_stress_v: device.stress.avg_vds_stress,
                            average_temperature_k: device.stress.avg_temp,
                            duration_s: device.stress.duration,
                        },
                        checkpoints,
                    });
                }
                devices.sort_by(|left, right| left.device_id.cmp(&right.device_id));
                let payload = AnalysisResultPayload::Reliability { devices };
                if let Err(error) = payload.validate_for(analysis_type) {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        format!("Invalid retained reliability payload: {error}"),
                    );
                }
                let result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(retained_waveforms)
                    .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                        years: retained_years,
                    })
                    .with_result_payload(payload);
                match result.validate_retained_evidence() {
                    Ok(()) => result,
                    Err(error) => AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        format!("Invalid retained reliability payload: {error}"),
                    ),
                }
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => {
                let retained_iterations = iterations.clone();
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_waveforms_with_shared_x_owned(iterations, waveforms))
                    .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                        iterations: retained_iterations,
                        best_cost,
                        best_variables: best_variables.into_iter().collect(),
                        converged,
                    })
            }

            SimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => {
                let retained_time = time.clone();
                let retained_waveforms = self.build_waveforms_with_shared_x_owned(time, waveforms);
                let mut violations = violations
                    .into_iter()
                    .map(|violation| SoaViolationEvidence {
                        device_id: violation.device_id,
                        parameter: retain_soa_parameter(violation.parameter),
                        limit_value: violation.limit_value,
                        actual_value: violation.actual_value,
                        time_s: violation.time,
                        severity: match violation.severity {
                            crate::services::safety::ViolationSeverity::Warning => {
                                SoaViolationSeverityEvidence::Warning
                            }
                            crate::services::safety::ViolationSeverity::Violation => {
                                SoaViolationSeverityEvidence::Violation
                            }
                            crate::services::safety::ViolationSeverity::Critical => {
                                SoaViolationSeverityEvidence::Critical
                            }
                        },
                    })
                    .collect::<Vec<_>>();
                violations.sort_by(|left, right| {
                    left.device_id
                        .cmp(&right.device_id)
                        .then_with(|| left.time_s.total_cmp(&right.time_s))
                        .then_with(|| left.parameter.cmp(&right.parameter))
                        .then_with(|| left.severity.cmp(&right.severity))
                        .then_with(|| left.limit_value.total_cmp(&right.limit_value))
                        .then_with(|| left.actual_value.total_cmp(&right.actual_value))
                });
                let mut evaluations = evaluations
                    .into_iter()
                    .map(|evaluation| SoaEvaluationEvidence {
                        device_id: evaluation.device_id,
                        parameter: retain_soa_parameter(evaluation.parameter),
                        limit_value: evaluation.limit_value,
                        worst_actual_value: evaluation.worst_actual_value,
                        worst_time_s: evaluation.worst_time,
                        sample_count: evaluation.sample_count,
                        unit: evaluation.unit,
                        description: evaluation.description,
                        verdict: match evaluation.verdict {
                            crate::services::safety::SoARuleVerdict::Pass => {
                                SoaRuleVerdictEvidence::Pass
                            }
                            crate::services::safety::SoARuleVerdict::Warning => {
                                SoaRuleVerdictEvidence::Warning
                            }
                            crate::services::safety::SoARuleVerdict::Violation => {
                                SoaRuleVerdictEvidence::Violation
                            }
                            crate::services::safety::SoARuleVerdict::Critical => {
                                SoaRuleVerdictEvidence::Critical
                            }
                        },
                    })
                    .collect::<Vec<_>>();
                evaluations.sort_by(|left, right| {
                    left.device_id
                        .cmp(&right.device_id)
                        .then_with(|| left.parameter.cmp(&right.parameter))
                });
                let payload = AnalysisResultPayload::Soa {
                    evaluations,
                    violations,
                };
                if let Err(error) = payload.validate_for(analysis_type) {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        format!("Invalid retained SOA payload: {error}"),
                    );
                }
                let result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(retained_waveforms)
                    .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                        time: retained_time,
                    })
                    .with_result_payload(payload);
                match result.validate_retained_evidence() {
                    Ok(()) => result,
                    Err(error) => AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        format!("Invalid retained SOA payload: {error}"),
                    ),
                }
            }

            SimulationResult::MeasurementsOnly { measurements } => {
                let payload = AnalysisResultPayload::ScalarMeasurements {
                    values: measurements.into_iter().collect(),
                };
                self.analysis_result_with_validated_payload(analysis_type, label, payload)
            }
        }
    }

    fn analysis_result_with_validated_payload(
        &self,
        analysis_type: AnalysisType,
        label: &str,
        payload: AnalysisResultPayload,
    ) -> AnalysisResult {
        match payload.validate_for(analysis_type) {
            Ok(()) => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_result_payload(payload),
            Err(error) => AnalysisResult::failed(
                1,
                analysis_type,
                label.to_string(),
                format!("Invalid retained analysis payload: {error}"),
            ),
        }
    }

    fn build_waveforms_with_shared_x_owned(
        &self,
        x_values: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
    ) -> Vec<crate::state::WaveformData> {
        self.build_sorted_waveforms_with_shared_x_owned(x_values, waveforms, |name, waveform| {
            (name, waveform.y_values)
        })
    }

    fn build_noise_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
    ) -> Vec<crate::state::WaveformData> {
        let shared_freqs = Arc::new(frequencies);
        let freq_len = shared_freqs.len();
        let mut results = Vec::new();

        if Self::samples_match_shared_axis(&output_noise, freq_len) {
            results.push(crate::state::WaveformData::new(
                "onoise".to_string(),
                Arc::clone(&shared_freqs),
                output_noise,
                Self::color_for_index(results.len()),
            ));
        }

        if let Some(inoise) = input_noise
            && Self::samples_match_shared_axis(&inoise, freq_len)
        {
            results.push(crate::state::WaveformData::new(
                "inoise".to_string(),
                Arc::clone(&shared_freqs),
                inoise,
                Self::color_for_index(results.len()),
            ));
        }

        let mut contributors: Vec<_> = contributors.into_iter().collect();
        contributors.sort_by(|a, b| a.0.cmp(&b.0));
        for (source, values) in contributors {
            if !Self::samples_match_shared_axis(&values, freq_len) {
                continue;
            }
            results.push(crate::state::WaveformData::new(
                format!("noise({})", source),
                Arc::clone(&shared_freqs),
                values,
                Self::color_for_index(results.len()),
            ));
        }

        results
    }

    fn build_monte_carlo_payload_owned(
        &self,
        variables: Vec<crate::simulation::results::MonteCarloVariableResult>,
    ) -> (
        Vec<crate::state::WaveformData>,
        Vec<MonteCarloVariableMetadata>,
    ) {
        let mut waveforms = Vec::with_capacity(variables.len());
        let mut metadata = Vec::with_capacity(variables.len());
        for variable in variables {
            let crate::simulation::results::MonteCarloVariableResult {
                name,
                samples,
                mean,
                std_dev,
                min,
                max,
                histogram,
                bin_edges,
            } = variable;
            if !histogram.is_empty() && bin_edges.len() == histogram.len().saturating_add(1) {
                let x: Vec<f64> = bin_edges
                    .windows(2)
                    .map(|window| (window[0] + window[1]) * 0.5)
                    .collect();
                let y: Vec<f64> = histogram.into_iter().map(|count| count as f64).collect();
                waveforms.push(crate::state::WaveformData::new(
                    format!("hist({name})"),
                    x,
                    y,
                    Self::color_for_index(waveforms.len()),
                ));
            }
            metadata.push(MonteCarloVariableMetadata {
                name,
                samples,
                mean,
                std_dev,
                min,
                max,
            });
        }
        (waveforms, metadata)
    }

    fn build_ac_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
    ) -> Vec<crate::state::WaveformData> {
        let shared_freqs = Arc::new(frequencies);
        let freq_len = shared_freqs.len();
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            let real = waveform.y_values;
            if !Self::samples_match_shared_axis(&real, freq_len) {
                continue;
            }
            match waveform.y_imag {
                Some(imag) => {
                    if !Self::samples_match_shared_axis(&imag, freq_len) {
                        continue;
                    }
                    let magnitude_values: Vec<f64> = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| (r * r + i * i).sqrt())
                        .collect();
                    let phase = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| i.atan2(*r).to_degrees())
                        .collect::<Vec<_>>();

                    let magnitude = crate::state::WaveformData::new(
                        format!("|{}|", name),
                        Arc::clone(&shared_freqs),
                        magnitude_values,
                        Self::color_for_index(results.len()),
                    )
                    .with_complex_components(name.clone(), real, imag);
                    results.push(magnitude);

                    results.push(crate::state::WaveformData::new(
                        format!("phase({})", name),
                        Arc::clone(&shared_freqs),
                        phase,
                        Self::color_for_index(results.len()),
                    ));
                }
                None => {
                    results.push(crate::state::WaveformData::new(
                        format!("|{}|", name),
                        Arc::clone(&shared_freqs),
                        real,
                        Self::color_for_index(results.len()),
                    ));
                }
            }
        }

        results
    }

    fn build_sorted_waveforms_with_shared_x_owned<F>(
        &self,
        x_values: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
        value_mapper: F,
    ) -> Vec<crate::state::WaveformData>
    where
        F: Fn(String, crate::simulation::WaveformData) -> (String, Vec<f64>),
    {
        let shared_x = Arc::new(x_values);
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            let (display_name, y_values) = value_mapper(name, waveform);
            if !Self::samples_match_shared_axis(&y_values, shared_x.len()) {
                continue;
            }
            results.push(crate::state::WaveformData::new(
                display_name,
                Arc::clone(&shared_x),
                y_values,
                Self::color_for_index(results.len()),
            ));
        }

        results
    }

    fn samples_match_shared_axis(samples: &[f64], axis_len: usize) -> bool {
        axis_len > 0 && samples.len() == axis_len
    }

    /// Get color for waveform trace by index
    pub(super) fn color_for_index(idx: usize) -> String {
        const COLORS: &[&str] = &[
            "#3B82F6", // Blue
            "#10B981", // Green
            "#F97316", // Orange
            "#8B5CF6", // Purple
            "#EC4899", // Pink
            "#EAB308", // Yellow
            "#14B8A6", // Teal
            "#EF4444", // Red
        ];
        COLORS[idx % COLORS.len()].to_string()
    }
}

fn retain_soa_parameter(parameter: crate::services::safety::SoAParameter) -> SoaParameterEvidence {
    match parameter {
        crate::services::safety::SoAParameter::Vgs => SoaParameterEvidence::GateSourceVoltage,
        crate::services::safety::SoAParameter::Vds => SoaParameterEvidence::DrainSourceVoltage,
        crate::services::safety::SoAParameter::Vgd => SoaParameterEvidence::GateDrainVoltage,
        crate::services::safety::SoAParameter::Vbe => SoaParameterEvidence::BaseEmitterVoltage,
        crate::services::safety::SoAParameter::Vce => SoaParameterEvidence::CollectorEmitterVoltage,
        crate::services::safety::SoAParameter::Vbc => SoaParameterEvidence::BaseCollectorVoltage,
        crate::services::safety::SoAParameter::Id => SoaParameterEvidence::DrainCurrent,
        crate::services::safety::SoAParameter::Ic => SoaParameterEvidence::CollectorCurrent,
        crate::services::safety::SoAParameter::Pdiss => SoaParameterEvidence::PowerDissipation,
        crate::services::safety::SoAParameter::Temp => SoaParameterEvidence::Temperature,
    }
}
