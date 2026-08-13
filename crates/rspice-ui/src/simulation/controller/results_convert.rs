//! Converting engine output into the viewer result model.
//!
//! Engine results arrive as raw vectors keyed by node and sweep variable.
//! This maps them onto the typed analysis results the viewers read, carrying
//! units and sweep metadata across the boundary rather than re-deriving them
//! downstream.

use super::*;
use crate::state::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Retain a transient run's committed event schedule as result evidence.
///
/// Nodes are ordered by name so the payload — and therefore the result
/// digest derived from it — is identical for identical runs regardless of the
/// order the engine happened to register event nodes in.
fn transient_events_payload(
    events: crate::simulation::results::TransientEventHistory,
) -> Option<AnalysisResultPayload> {
    if events.is_empty() {
        return None;
    }
    let mut digital_traces = events
        .digital
        .into_iter()
        .map(|trace| crate::state::DigitalEventTraceEvidence {
            node_name: trace.node_name,
            points: trace
                .points
                .into_iter()
                .map(|point| crate::state::DigitalEventPointEvidence {
                    time_s: point.time_s,
                    value_code: point.value_code,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    digital_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
    let mut real_traces = events
        .real
        .into_iter()
        .map(|trace| crate::state::RealEventTraceEvidence {
            node_name: trace.node_name,
            points: trace
                .points
                .into_iter()
                .map(|point| crate::state::RealEventPointEvidence {
                    time_s: point.time_s,
                    value: point.value,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    real_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
    let payload = AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
    };
    // A history the validator would reject is retained as nothing at all: a
    // viewer must never have to decide whether its own evidence is usable.
    payload
        .validate_for(crate::state::AnalysisType::Transient)
        .is_ok()
        .then_some(payload)
}

/// Frequencies, output noise, optional input noise, and per-source
/// contributors, all in the same stable order.
type OrderedNoiseSeries = (
    Vec<f64>,
    Vec<f64>,
    Option<Vec<f64>>,
    HashMap<String, Vec<f64>>,
);

impl SimulationController {
    /// Bind the periodic-noise output quantity from the exact typed execution
    /// options to the immutable retained result. PNOISE uses the generic
    /// engine noise transport, whose `onoise` vector alone cannot distinguish
    /// V²/Hz or A²/Hz output PSD from dBc/Hz phase noise.
    pub(super) fn retain_periodic_noise_result_metadata(&self, result: &mut AnalysisResult) {
        if !matches!(
            result.analysis_type,
            AnalysisType::Pnoise | AnalysisType::Qpnoise
        ) {
            return;
        }
        let Some(config) = self
            .current_spec_options
            .as_ref()
            .and_then(|options| options.pnoise.as_ref())
        else {
            return;
        };
        let output_quantity = match config.noise_ref {
            crate::services::simulation_runner::PnoiseReference::Phase => {
                PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz
            }
            crate::services::simulation_runner::PnoiseReference::Output
            | crate::services::simulation_runner::PnoiseReference::Input => {
                PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity
            }
        };
        if output_quantity == PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz
            && let Some(waveform) = result
                .waveforms
                .iter_mut()
                .find(|waveform| waveform.name.eq_ignore_ascii_case("onoise"))
        {
            waveform.name = "phase_noise".to_owned();
        }
        result.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity,
            carrier_frequency_hz: Some(config.pss_fundamental_freq),
        });
    }

    /// Retain the exact power-wave reference impedances that qualify an
    /// SP/PSP/HBSP result. Declared RF ports win for the same reason they win
    /// in execution; a plain SP setup falls back to its explicitly configured
    /// ports and default impedance only when the deck declares none.
    pub(super) fn retain_sparameter_result_metadata(&self, result: &mut AnalysisResult) {
        if !matches!(
            result.analysis_type,
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
        ) {
            return;
        }
        let Some(reference_impedances_ohm) = self.sparameter_reference_impedances() else {
            return;
        };
        let metadata = AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
        };
        if metadata.validate_for(result.analysis_type).is_ok() {
            result.family_metadata = Some(metadata);
        }
    }

    fn sparameter_reference_impedances(&self) -> Option<Vec<f64>> {
        if let Some(netlist_text) = self.cached_netlist.as_deref()
            && let Ok(netlist) = rspice_core::Netlist::parse(netlist_text)
            && let Ok(ports) = rspice_core::analysis::s_param::collect_ports(&netlist)
        {
            return Some(ports.into_iter().map(|port| port.z0).collect());
        }

        match self.current_spec.as_ref()? {
            AnalysisSpec::SParameter { z0, ports, .. } if ports.len() >= 2 => {
                Some(ports.iter().map(|port| port.z0.unwrap_or(*z0)).collect())
            }
            AnalysisSpec::Psp { ports, .. } | AnalysisSpec::Hbsp { ports, .. }
                if ports.len() >= 2 && ports.iter().all(|port| port.z0.is_some()) =>
            {
                Some(ports.iter().filter_map(|port| port.z0).collect())
            }
            _ => None,
        }
    }

    pub(super) fn convert_to_analysis_result_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata_owned(sim_result, analysis_type, &label)
    }

    /// Reachable across `simulation` because the corner family is assembled
    /// from the point results after the batch and must be retained through the
    /// same conversion an engine-produced result goes through.
    pub(in crate::simulation) fn convert_to_analysis_result_with_metadata_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;

        match sim_result {
            SimulationResult::DcOp(dc_result) => {
                let op_payload = operating_point_payload(
                    &dc_result.configuration,
                    dc_result.validated_startup_directives,
                    dc_result.mna_node_names.clone(),
                    dc_result.mna_branch_names.clone(),
                    dc_result.mna_solution.clone(),
                );
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
                    .with_dc_op(state_dc_op)
                    .with_result_payload(op_payload);
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
                events,
                ..
            } => {
                let result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_time_waveforms_owned(time, waveforms))
                    .with_measurements(measurements);
                // Only a deck with event nodes carries a payload. Attaching an
                // empty one would make every transient claim event evidence it
                // does not have, and the payload validator rejects it anyway.
                match transient_events_payload(events) {
                    Some(payload) => result.with_result_payload(payload),
                    None => result,
                }
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms))
                .with_measurements(measurements),

            SimulationResult::HarmonicBalance {
                frequencies,
                waveforms,
                measurements,
                ..
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

            SimulationResult::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                normalization,
                accuracy,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
            } => {
                let quantity = |value| match value {
                    crate::simulation::results::TransferFunctionQuantity::Voltage => {
                        TransferFunctionQuantityEvidence::Voltage
                    }
                    crate::simulation::results::TransferFunctionQuantity::Current => {
                        TransferFunctionQuantityEvidence::Current
                    }
                };
                let scalar = |value| match value {
                    crate::simulation::results::TransferFunctionScalar::Finite(value) => {
                        TransferFunctionScalarEvidence::Finite(value)
                    }
                    crate::simulation::results::TransferFunctionScalar::PositiveInfinity => {
                        TransferFunctionScalarEvidence::PositiveInfinity
                    }
                    crate::simulation::results::TransferFunctionScalar::NegativeInfinity => {
                        TransferFunctionScalarEvidence::NegativeInfinity
                    }
                };
                let payload = AnalysisResultPayload::TransferFunction {
                    input_source,
                    output_expression,
                    input_quantity: quantity(input_quantity),
                    output_quantity: quantity(output_quantity),
                    input_unit,
                    output_unit,
                    normalization: match normalization {
                        crate::simulation::multi_run::TfNormalization::None => {
                            TransferFunctionNormalizationEvidence::None
                        }
                        crate::simulation::multi_run::TfNormalization::RelativeToNominal => {
                            TransferFunctionNormalizationEvidence::RelativeToNominal
                        }
                        crate::simulation::multi_run::TfNormalization::PerSourceUnit => {
                            TransferFunctionNormalizationEvidence::PerSourceUnit
                        }
                    },
                    accuracy: match accuracy {
                        crate::simulation::multi_run::TfAccuracy::Fast => {
                            TransferFunctionAccuracyEvidence::Fast
                        }
                        crate::simulation::multi_run::TfAccuracy::Balanced => {
                            TransferFunctionAccuracyEvidence::Balanced
                        }
                        crate::simulation::multi_run::TfAccuracy::Accurate => {
                            TransferFunctionAccuracyEvidence::Accurate
                        }
                        crate::simulation::multi_run::TfAccuracy::Robust => {
                            TransferFunctionAccuracyEvidence::Robust
                        }
                    },
                    gain: gain.map(scalar),
                    input_resistance: input_resistance.map(scalar),
                    output_resistance: output_resistance.map(scalar),
                    nominal_input,
                    nominal_output,
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

    fn build_time_waveforms_owned(
        &self,
        time: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
    ) -> Vec<crate::state::WaveformData> {
        let shared_time = Arc::new(time);
        let sample_count = shared_time.len();
        let mut waveforms = waveforms.into_iter().collect::<Vec<_>>();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));
        let mut results = Vec::new();

        for (name, waveform) in waveforms {
            let unit = waveform.y_unit;
            let real = waveform.y_values;
            if !Self::samples_match_shared_axis(&real, sample_count) {
                continue;
            }
            if let Some(imag) = waveform.y_imag {
                if !Self::samples_match_shared_axis(&imag, sample_count) {
                    continue;
                }
                let magnitude = real
                    .iter()
                    .zip(imag.iter())
                    .map(|(real, imag)| real.hypot(*imag))
                    .collect::<Vec<_>>();
                let phase = real
                    .iter()
                    .zip(imag.iter())
                    .map(|(real, imag)| imag.atan2(*real).to_degrees())
                    .collect::<Vec<_>>();
                results.push(
                    crate::state::WaveformData::new(
                        format!("|{name}|"),
                        Arc::clone(&shared_time),
                        magnitude,
                        Self::color_for_index(results.len()),
                    )
                    .with_unit(unit)
                    .with_complex_components(name.clone(), real, imag),
                );
                results.push(
                    crate::state::WaveformData::new(
                        format!("phase({name})"),
                        Arc::clone(&shared_time),
                        phase,
                        Self::color_for_index(results.len()),
                    )
                    .with_unit("°"),
                );
            } else {
                results.push(
                    crate::state::WaveformData::new(
                        name,
                        Arc::clone(&shared_time),
                        real,
                        Self::color_for_index(results.len()),
                    )
                    .with_unit(unit),
                );
            }
        }
        results
    }

    fn build_noise_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
    ) -> Vec<crate::state::WaveformData> {
        let (frequencies, output_noise, input_noise, contributors) =
            Self::order_noise_series_for_retention(
                frequencies,
                output_noise,
                input_noise,
                contributors,
            );
        let shared_freqs = Arc::new(frequencies);
        let freq_len = shared_freqs.len();
        let mut results = Vec::new();

        // These series carry no stated unit on purpose. The generic engine
        // noise transport is the same vector for a V²/Hz output PSD and for
        // dBc/Hz phase noise, and only the periodic-noise family metadata
        // knows which one a run produced. Claiming one here would make every
        // oscillator's phase noise read as a power spectral density.
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

    /// Normalize the retained plotting axis without changing authored DATA
    /// execution order. A single stable permutation is applied to the axis and
    /// every aligned series, preserving each physical frequency/value pair.
    fn order_noise_series_for_retention(
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
    ) -> OrderedNoiseSeries {
        if frequencies
            .windows(2)
            .all(|pair| pair[0].total_cmp(&pair[1]) != std::cmp::Ordering::Greater)
        {
            return (frequencies, output_noise, input_noise, contributors);
        }

        let mut order = (0..frequencies.len()).collect::<Vec<_>>();
        order.sort_by(|left, right| {
            frequencies[*left]
                .total_cmp(&frequencies[*right])
                .then_with(|| left.cmp(right))
        });
        let frequencies = Self::permute_noise_samples(frequencies, &order);
        let output_noise = Self::permute_noise_samples_if_aligned(output_noise, &order);
        let input_noise =
            input_noise.map(|samples| Self::permute_noise_samples_if_aligned(samples, &order));
        let contributors = contributors
            .into_iter()
            .map(|(name, samples)| {
                (
                    name,
                    Self::permute_noise_samples_if_aligned(samples, &order),
                )
            })
            .collect();

        (frequencies, output_noise, input_noise, contributors)
    }

    fn permute_noise_samples(samples: Vec<f64>, order: &[usize]) -> Vec<f64> {
        order.iter().map(|index| samples[*index]).collect()
    }

    fn permute_noise_samples_if_aligned(samples: Vec<f64>, order: &[usize]) -> Vec<f64> {
        if samples.len() == order.len() {
            Self::permute_noise_samples(samples, order)
        } else {
            samples
        }
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
            // A magnitude is the modulus of the source quantity, so it reads
            // in the source's own unit. The phase projection does not: it is
            // produced here, in degrees, and says so itself.
            let unit = waveform.y_unit;
            let waveform_x = waveform.x_values;
            let real = waveform.y_values;
            let x = if Self::samples_match_shared_axis(&real, waveform_x.len()) {
                Arc::new(waveform_x)
            } else if Self::samples_match_shared_axis(&real, freq_len) {
                Arc::clone(&shared_freqs)
            } else {
                continue;
            };
            match waveform.y_imag {
                Some(imag) => {
                    if !Self::samples_match_shared_axis(&imag, x.len()) {
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
                        Arc::clone(&x),
                        magnitude_values,
                        Self::color_for_index(results.len()),
                    )
                    .with_unit(unit)
                    .with_complex_components(name.clone(), real, imag);
                    results.push(magnitude);

                    results.push(
                        crate::state::WaveformData::new(
                            format!("phase({})", name),
                            Arc::clone(&x),
                            phase,
                            Self::color_for_index(results.len()),
                        )
                        .with_unit("°"),
                    );
                }
                None => {
                    // A real-valued frequency-domain quantity is not a
                    // complex magnitude. Preserve its producer name and unit
                    // so group delay, stability margin, and Floquet-mode
                    // metrics are never wrapped in |...| or converted to dB
                    // by the viewer.
                    results.push(
                        crate::state::WaveformData::new(
                            name,
                            x,
                            real,
                            Self::color_for_index(results.len()),
                        )
                        .with_unit(unit),
                    );
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
            // Read the stated unit before the mapper consumes the waveform:
            // it is the producer's, and nothing downstream can recover it
            // from the samples or the name.
            let unit = waveform.y_unit.clone();
            let (display_name, y_values) = value_mapper(name, waveform);
            if !Self::samples_match_shared_axis(&y_values, shared_x.len()) {
                continue;
            }
            results.push(
                crate::state::WaveformData::new(
                    display_name,
                    Arc::clone(&shared_x),
                    y_values,
                    Self::color_for_index(results.len()),
                )
                .with_unit(unit),
            );
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

fn operating_point_payload(
    config: &crate::simulation::dialog::OpConfig,
    validated_startup_directives: usize,
    mna_node_names: Vec<String>,
    mna_branch_names: Vec<String>,
    mna_solution: Vec<f64>,
) -> crate::state::AnalysisResultPayload {
    use crate::simulation::dialog::*;
    use crate::state::*;
    AnalysisResultPayload::OperatingPoint {
        temperature_mode: match config.temperature_mode {
            OpTemperatureMode::PvtRunSet => OperatingPointTemperatureEvidence::PvtRunSet,
            OpTemperatureMode::Nominal27C => OperatingPointTemperatureEvidence::Nominal27C,
            OpTemperatureMode::Explicit => OperatingPointTemperatureEvidence::Explicit,
            OpTemperatureMode::ActiveRunSetAxis => {
                OperatingPointTemperatureEvidence::ActiveRunSetAxis
            }
        },
        temperature_celsius: config.temperature_celsius,
        initial_guess: match config.initial_guess {
            OpInitialGuess::Automatic => OperatingPointInitialGuessEvidence::Automatic,
            OpInitialGuess::PreviousConverged => {
                OperatingPointInitialGuessEvidence::PreviousConverged
            }
            OpInitialGuess::UserNodeVoltages => {
                OperatingPointInitialGuessEvidence::UserNodeVoltages
            }
            OpInitialGuess::ZeroState => OperatingPointInitialGuessEvidence::ZeroState,
        },
        node_initialization: match config.node_initialization {
            OpNodeInitialization::UseIcAndNodeset => {
                OperatingPointNodeInitializationEvidence::UseIcAndNodeset
            }
            OpNodeInitialization::IgnoreIcAndNodeset => {
                OperatingPointNodeInitializationEvidence::IgnoreIcAndNodeset
            }
            OpNodeInitialization::ForceIcValues => {
                OperatingPointNodeInitializationEvidence::ForceIcValues
            }
            OpNodeInitialization::ValidateOnly => {
                OperatingPointNodeInitializationEvidence::ValidateOnly
            }
        },
        homotopy: match config.homotopy {
            OpHomotopy::Adaptive => OperatingPointHomotopyEvidence::Adaptive,
            OpHomotopy::SourceStepping => OperatingPointHomotopyEvidence::SourceStepping,
            OpHomotopy::GminStepping => OperatingPointHomotopyEvidence::GminStepping,
            OpHomotopy::PseudoTransient => OperatingPointHomotopyEvidence::PseudoTransient,
            OpHomotopy::None => OperatingPointHomotopyEvidence::None,
        },
        annotation: match config.annotation {
            OpAnnotation::VoltagesAndCurrents => {
                OperatingPointAnnotationEvidence::VoltagesAndCurrents
            }
            OpAnnotation::VoltagesOnly => OperatingPointAnnotationEvidence::VoltagesOnly,
            OpAnnotation::VoltagesAndDeviceOp => {
                OperatingPointAnnotationEvidence::VoltagesAndDeviceOp
            }
            OpAnnotation::None => OperatingPointAnnotationEvidence::None,
        },
        device_detail: match config.device_detail {
            OpDeviceDetail::SelectedAndViolations => {
                OperatingPointDeviceDetailEvidence::SelectedAndViolations
            }
            OpDeviceDetail::AllDevices => OperatingPointDeviceDetailEvidence::AllDevices,
            OpDeviceDetail::ViolationsOnly => OperatingPointDeviceDetailEvidence::ViolationsOnly,
            OpDeviceDetail::None => OperatingPointDeviceDetailEvidence::None,
        },
        save_device_op: match config.save_device_op {
            OpSaveDevice::Enabled => OperatingPointSaveDeviceEvidence::Enabled,
            OpSaveDevice::Disabled => OperatingPointSaveDeviceEvidence::Disabled,
            OpSaveDevice::FinalPointOnly => OperatingPointSaveDeviceEvidence::FinalPointOnly,
        },
        accuracy: match config.accuracy {
            OpAccuracy::Fast => OperatingPointAccuracyEvidence::Fast,
            OpAccuracy::Balanced => OperatingPointAccuracyEvidence::Balanced,
            OpAccuracy::Accurate => OperatingPointAccuracyEvidence::Accurate,
            OpAccuracy::Robust => OperatingPointAccuracyEvidence::Robust,
        },
        selected_devices: config.selected_devices.clone(),
        violation_devices: config.violation_devices.clone(),
        violation_source_content_digest: config.violation_source_content_digest,
        validated_startup_directives: u64::try_from(validated_startup_directives)
            .unwrap_or(u64::MAX),
        mna_node_names,
        mna_branch_names,
        mna_solution,
        effective_source_content_digest: None,
        run_point_index: u64::try_from(config.run_point.index).unwrap_or(u64::MAX),
        run_point_count: u64::try_from(config.run_point.count).unwrap_or(u64::MAX),
        run_point_process: match config.run_point.process {
            crate::simulation::dialog::corner::ProcessCorner::TT => {
                OperatingPointProcessEvidence::TT
            }
            crate::simulation::dialog::corner::ProcessCorner::SS => {
                OperatingPointProcessEvidence::SS
            }
            crate::simulation::dialog::corner::ProcessCorner::FF => {
                OperatingPointProcessEvidence::FF
            }
            crate::simulation::dialog::corner::ProcessCorner::SF => {
                OperatingPointProcessEvidence::SF
            }
            crate::simulation::dialog::corner::ProcessCorner::FS => {
                OperatingPointProcessEvidence::FS
            }
        },
        run_point_supply_voltage: config.run_point.supply_voltage,
        run_point_nominal_supply_voltage: config.run_point.nominal_supply_voltage,
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

#[cfg(test)]
mod operating_point_conversion_tests {
    use super::*;

    #[test]
    fn branch_current_is_wrapped_exactly_once_in_retained_results() {
        let sim_result = crate::simulation::SimulationResult::DcOp(Box::new(
            crate::simulation::results::DcOpResult {
                configuration: crate::simulation::dialog::OpConfig::default(),
                branch_currents: HashMap::from([("V1".to_owned(), -1.0e-3)]),
                ..Default::default()
            },
        ));
        let result = SimulationController::new()
            .convert_to_analysis_result_owned(sim_result, &AnalysisConfig::dc_op());
        let currents = &result.dc_op.as_ref().unwrap().branch_currents;
        assert_eq!(currents.len(), 1);
        assert_eq!(currents[0].name, "I(V1)");
    }
}

#[cfg(test)]
mod waveform_unit_conversion_tests {
    use super::*;

    fn producer_waveform(
        name: &str,
        y_values: Vec<f64>,
        y_unit: &str,
    ) -> crate::simulation::results::WaveformData {
        crate::simulation::results::WaveformData {
            name: name.to_owned(),
            x_values: Vec::new(),
            y_values,
            y_unit: y_unit.to_owned(),
            is_complex: false,
            y_imag: None,
        }
    }

    fn retained_unit(result: &AnalysisResult, name: &str) -> Option<String> {
        result
            .waveforms
            .iter()
            .find(|waveform| waveform.name == name)
            .unwrap_or_else(|| panic!("missing retained waveform {name}"))
            .unit
            .clone()
    }

    #[test]
    fn a_waveform_retains_the_unit_its_producer_measured_it_in() {
        let sim_result = crate::simulation::SimulationResult::Transient {
            time: vec![0.0, 1.0],
            waveforms: HashMap::from([
                (
                    "V(out)".to_owned(),
                    producer_waveform("V(out)", vec![0.0, 5.0], "V"),
                ),
                (
                    "I(V1)".to_owned(),
                    producer_waveform("I(V1)", vec![0.0, 1.0e-3], "A"),
                ),
                (
                    "SOA_VIOLATION_COUNT".to_owned(),
                    producer_waveform("SOA_VIOLATION_COUNT", vec![0.0, 2.0], "count"),
                ),
            ]),
            measurements: Vec::new(),
            periodic_state: None,
            convergence: Default::default(),
            events: Default::default(),
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Transient,
            "TRAN",
        );

        assert_eq!(retained_unit(&result, "V(out)").as_deref(), Some("V"));
        assert_eq!(retained_unit(&result, "I(V1)").as_deref(), Some("A"));
        // The one the name cannot supply, and the reason this is carried at
        // all: a violation count read as volts in the results browser.
        assert_eq!(
            retained_unit(&result, "SOA_VIOLATION_COUNT").as_deref(),
            Some("count")
        );
    }

    #[test]
    fn an_ac_magnitude_keeps_the_source_unit_while_phase_states_degrees() {
        let mut spectrum = crate::simulation::results::WaveformData::new_complex(
            "V(out) Spectrum",
            vec![1.0, 10.0],
            vec![1.0, 0.0],
            vec![0.0, 1.0],
        );
        spectrum.y_unit = "V".to_owned();
        let sim_result = crate::simulation::SimulationResult::Ac {
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([("V(out) Spectrum".to_owned(), spectrum)]),
            measurements: Vec::new(),
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Ac,
            "AC",
        );

        assert_eq!(
            retained_unit(&result, "|V(out) Spectrum|").as_deref(),
            Some("V")
        );
        assert_eq!(
            retained_unit(&result, "phase(V(out) Spectrum)").as_deref(),
            Some("°")
        );
    }

    #[test]
    fn a_real_frequency_quantity_keeps_its_name_and_unit_without_magnitude_wrapping() {
        let group_delay = crate::simulation::results::WaveformData::new_time_domain_in_unit(
            "group_delay",
            vec![1.0, 10.0],
            vec![2.0e-9, 3.0e-9],
            "s",
        );
        let sim_result = crate::simulation::SimulationResult::Ac {
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([("group_delay".to_owned(), group_delay)]),
            measurements: Vec::new(),
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Pxf,
            "PXF",
        );

        assert_eq!(result.waveforms[0].name, "group_delay");
        assert_eq!(result.waveforms[0].unit.as_deref(), Some("s"));
    }

    #[test]
    fn a_frequency_companion_curve_keeps_its_own_exact_abscissa() {
        let group_delay = crate::simulation::results::WaveformData::new_time_domain_in_unit(
            "group_delay",
            vec![3.0, 30.0],
            vec![2.0e-9, 3.0e-9],
            "s",
        );
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Ac {
                frequencies: vec![1.0, 10.0, 100.0],
                waveforms: HashMap::from([("group_delay".to_owned(), group_delay)]),
                measurements: Vec::new(),
            },
            AnalysisType::Pxf,
            "PXF",
        );

        assert_eq!(result.waveforms[0].x.as_slice(), &[3.0, 30.0]);
        assert_eq!(result.waveforms[0].y.as_slice(), &[2.0e-9, 3.0e-9]);
    }

    #[test]
    fn a_complex_time_domain_waveform_retains_both_components_and_phase() {
        let waveform = crate::simulation::results::WaveformData::new_complex_time_domain(
            "V(env)",
            vec![0.0, 1.0],
            vec![3.0, 0.0],
            vec![4.0, 1.0],
        );
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Transient {
                time: vec![0.0, 1.0],
                waveforms: HashMap::from([("V(env)".to_owned(), waveform)]),
                measurements: Vec::new(),
                periodic_state: None,
                convergence: Default::default(),
                events: Default::default(),
            },
            AnalysisType::Envelope,
            "Envelope",
        );

        assert_eq!(result.waveforms.len(), 2);
        assert_eq!(result.waveforms[0].name, "|V(env)|");
        assert_eq!(result.waveforms[0].y.as_slice(), &[5.0, 1.0]);
        let complex = result.waveforms[0].complex.as_ref().expect("retained I/Q");
        assert_eq!(complex.real.as_slice(), &[3.0, 0.0]);
        assert_eq!(complex.imag.as_slice(), &[4.0, 1.0]);
        assert_eq!(result.waveforms[1].name, "phase(V(env))");
        assert_eq!(result.waveforms[1].unit.as_deref(), Some("°"));
    }

    #[test]
    fn a_producer_that_states_no_unit_retains_nothing_rather_than_an_empty_one() {
        // AC's own complex waveforms leave the unit empty. Retaining "" would
        // read downstream as a real unit and stop the browser and the axes
        // falling back to the accessor in the name.
        let sim_result = crate::simulation::SimulationResult::Ac {
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([(
                "V(out)".to_owned(),
                crate::simulation::results::WaveformData::new_complex(
                    "V(out)",
                    vec![1.0, 10.0],
                    vec![1.0, 0.0],
                    vec![0.0, 1.0],
                ),
            )]),
            measurements: Vec::new(),
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Ac,
            "AC",
        );

        assert_eq!(retained_unit(&result, "|V(out)|"), None);
    }

    #[test]
    fn retained_noise_series_state_no_unit_so_phase_noise_is_never_called_a_psd() {
        let sim_result = crate::simulation::SimulationResult::Noise {
            frequencies: vec![1.0e3, 1.0e6],
            output_noise: vec![-90.0, -130.0],
            input_noise: None,
            contributors: HashMap::new(),
            summary: None,
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Pnoise,
            "PNOISE",
        );

        assert_eq!(retained_unit(&result, "onoise"), None);
    }
}

#[cfg(test)]
mod noise_conversion_tests {
    use super::*;

    #[test]
    fn descending_data_axis_is_retained_monotonically_with_every_series_paired() {
        let sim_result = crate::simulation::SimulationResult::Noise {
            frequencies: vec![10.0, 1.0, 10.0],
            output_noise: vec![100.0, 10.0, 101.0],
            input_noise: Some(vec![200.0, 20.0, 201.0]),
            contributors: HashMap::from([("R1:thermal".to_owned(), vec![300.0, 30.0, 301.0])]),
            summary: None,
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Noise,
            "NOISE DATA",
        );

        for waveform in &result.waveforms {
            assert_eq!(waveform.x.as_ref(), &[1.0, 10.0, 10.0]);
        }
        let waveform = |name: &str| {
            result
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)
                .unwrap_or_else(|| panic!("missing retained waveform {name}"))
        };
        assert_eq!(waveform("onoise").y.as_ref(), &[10.0, 100.0, 101.0]);
        assert_eq!(waveform("inoise").y.as_ref(), &[20.0, 200.0, 201.0]);
        assert_eq!(
            waveform("noise(R1:thermal)").y.as_ref(),
            &[30.0, 300.0, 301.0]
        );
    }

    #[test]
    fn descending_axis_with_misaligned_worker_series_fails_closed_without_panicking() {
        let sim_result = crate::simulation::SimulationResult::Noise {
            frequencies: vec![10.0, 1.0],
            output_noise: vec![100.0],
            input_noise: Some(vec![200.0]),
            contributors: HashMap::from([("R1:thermal".to_owned(), vec![300.0])]),
            summary: None,
        };

        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Noise,
            "NOISE DATA",
        );

        assert!(result.waveforms.is_empty());
    }

    fn periodic_noise_result(
        reference: crate::services::simulation_runner::PnoiseReference,
    ) -> AnalysisResult {
        let mut controller = SimulationController::new();
        let mut config = crate::services::simulation_runner::PnoiseRunConfig::default();
        config.noise_ref = reference;
        config.pss_fundamental_freq = 2.4e9;
        controller.current_spec_options = Some(SpecExecutionOptions {
            pnoise: Some(config),
            ..SpecExecutionOptions::default()
        });
        let sim_result = crate::simulation::SimulationResult::Noise {
            frequencies: vec![1.0e3, 1.0e6],
            output_noise: vec![-90.0, -130.0],
            input_noise: None,
            contributors: HashMap::new(),
            summary: None,
        };
        let mut result = controller.convert_to_analysis_result_with_metadata_owned(
            sim_result,
            AnalysisType::Pnoise,
            "PNOISE",
        );
        controller.retain_periodic_noise_result_metadata(&mut result);
        result
    }

    #[test]
    fn phase_reference_retains_exact_dbc_per_hz_quantity_and_carrier() {
        let result =
            periodic_noise_result(crate::services::simulation_runner::PnoiseReference::Phase);
        assert!(result.validate_retained_evidence().is_ok());
        assert_eq!(result.waveforms[0].name, "phase_noise");
        assert_eq!(
            result.family_metadata,
            Some(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
        );
    }

    #[test]
    fn output_reference_remains_psd_and_is_never_relabelled_as_phase_noise() {
        let result =
            periodic_noise_result(crate::services::simulation_runner::PnoiseReference::Output);
        assert!(result.validate_retained_evidence().is_ok());
        assert_eq!(result.waveforms[0].name, "onoise");
        assert_eq!(
            result.family_metadata,
            Some(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                carrier_frequency_hz: Some(2.4e9),
            })
        );
    }

    #[test]
    fn sparameter_result_retains_declared_nondefault_port_impedances() {
        let mut controller = SimulationController::new();
        controller.cached_netlist = Some(
            "P1 IN 0 PORT=1 Z0=75 AC 1\nR1 IN OUT 50\nP2 OUT 0 PORT=2 Z0=100\n.end\n".to_owned(),
        );
        controller.current_spec = Some(AnalysisSpec::SParameter {
            start_freq: 1.0e6,
            stop_freq: 1.0e9,
            points_per_unit: 10,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            z0: 50.0,
            ports: Vec::new(),
        });
        let mut result = AnalysisResult::new(1, AnalysisType::SParameter, "SP");

        controller.retain_sparameter_result_metadata(&mut result);

        assert_eq!(
            result.family_metadata,
            Some(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![75.0, 100.0],
            })
        );
        assert!(result.validate_retained_evidence().is_ok());
    }
}
