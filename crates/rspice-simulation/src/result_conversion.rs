//! Conversion of engine completions into exact retained analysis evidence.
//!
//! The host supplies its completion clock and decorates the returned waveforms.
//! Source samples, units, result payloads and their validation stay here.

use crate::results::SimulationResult;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::floquet::PstbFloquetModeEvidence;
use rspice_results::operating_point::DcOpResult;
use rspice_results::operating_point::OperatingPointValue;
use rspice_results::simulation_values::ComplexResultValue;
use rspice_results::soa_evidence::SoaEvaluationEvidence;
use rspice_results::soa_evidence::SoaRuleVerdictEvidence;
use rspice_results::soa_evidence::SoaViolationEvidence;
use rspice_results::soa_evidence::SoaViolationSeverityEvidence;
use rspice_results::transfer_function::TransferFunctionAccuracyEvidence;
use rspice_results::transfer_function::TransferFunctionNormalizationEvidence;
use rspice_results::transfer_function::TransferFunctionQuantityEvidence;
use rspice_results::transfer_function::TransferFunctionScalarEvidence;
#[cfg(test)]
use std::collections::HashMap;

mod floquet;
mod operating_point;
mod periodic_noise;
mod recorded_fft;
mod sensitivity;
mod soa;
mod waveforms;
use floquet::{
    retain_floquet_evidence, retain_floquet_orbit_kind, retain_floquet_verdict,
    retain_pss_floquet_payload, retain_pstb_classification,
};
use operating_point::operating_point_payload;
pub use periodic_noise::retain_periodic_noise_metadata;
pub use recorded_fft::incomplete_history_notice;
use soa::retain_soa_parameter;

/// Retain one completed engine result without selecting or mutating an app document.
pub fn convert(
    result: SimulationResult,
    analysis_type: AnalysisType,
    label: &str,
    now: impl Fn() -> f64,
) -> AnalysisResult {
    ResultConversion { now }.convert(result, analysis_type, label)
}

struct ResultConversion<Clock> {
    now: Clock,
}

fn transient_events_payload(
    events: crate::results::TransientEventHistory,
) -> Option<AnalysisResultPayload> {
    if events.is_empty() {
        return None;
    }
    let mut digital_traces = events
        .digital
        .into_iter()
        .map(|trace| rspice_results::events::DigitalEventTraceEvidence {
            node_name: trace.node_name,
            points: trace
                .points
                .into_iter()
                .map(|point| rspice_results::events::DigitalEventPointEvidence {
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
        .map(|trace| rspice_results::events::RealEventTraceEvidence {
            node_name: trace.node_name,
            points: trace
                .points
                .into_iter()
                .map(|point| rspice_results::events::RealEventPointEvidence {
                    time_s: point.time_s,
                    value: point.value,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    real_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
    // Declaration order, not name order: a bus states which conductor is the
    // most significant bit, and sorting the members would restate the word.
    // The table itself is ordered by name for the same digest reason the
    // traces are.
    let mut digital_buses = events.digital_buses;
    digital_buses.sort_by(|left, right| left.name.cmp(&right.name));
    let payload = AnalysisResultPayload::TransientEvents {
        current_impulses: events.current_impulses,
        digital_traces,
        real_traces,
        digital_buses,
    };
    // A history the validator would reject is retained as nothing at all: a
    // viewer must never have to decide whether its own evidence is usable.
    payload
        .validate_for(rspice_results::analysis_type::AnalysisType::Transient)
        .is_ok()
        .then_some(payload)
}

impl<Clock: Fn() -> f64> ResultConversion<Clock> {
    fn convert(
        &self,
        sim_result: crate::results::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::results::SimulationResult;

        let convergence = sim_result.transient_convergence().cloned();
        if let Some(quality) = &convergence
            && let Err(error) = quality.validate()
        {
            return AnalysisResult::failed(1, analysis_type, label, error, (self.now)());
        }
        let mut retained = match sim_result {
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
                let mut result =
                    AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                        .with_dc_op(state_dc_op)
                        .with_result_payload(op_payload);
                if let Some(report) = dc_result.device_report {
                    result = result.with_device_op(report);
                }
                result
            }

            SimulationResult::Noise {
                output_unit,
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => {
                let mut result =
                    AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                        .with_waveforms(self.build_noise_waveforms_owned(
                            frequencies,
                            output_noise,
                            input_noise,
                            contributors,
                            summary.as_ref().and_then(|summary| summary.input_quantity),
                        ))
                        .with_measurements(measurements);
                if matches!(analysis_type, AnalysisType::Noise | AnalysisType::Hbnoise)
                    || summary
                        .as_ref()
                        .is_some_and(|summary| summary.conversion.is_some())
                {
                    for wave in &mut result.waveforms {
                        if wave.name == "onoise" || wave.name.starts_with("noise(") {
                            wave.unit = Some("V²/Hz".into());
                        }
                    }
                }
                if let Some(unit) = output_unit.as_ref().and_then(|unit| unit.symbol()) {
                    for wave in &mut result.waveforms {
                        if wave.name == "onoise" || wave.name.starts_with("noise(") {
                            wave.unit = Some(unit.to_owned());
                        }
                    }
                }
                if let Some(summary) = summary {
                    if let Some(figure) = &summary.noise_figure {
                        result.waveforms.push(
                            rspice_results::waveform::RetainedWaveform::new(
                                "Noise figure (SSB)",
                                figure.frequencies.clone(),
                                figure.decibels.clone(),
                            )
                            .with_unit("dB"),
                        );
                    }
                    result = result.with_noise_summary(summary);
                }
                result
            }

            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                events,
                periodic_state,
                convergence: _,
                // A recorded spectrum is its own analysis's result, published
                // by the FFT task that bound this transient's trajectory. The
                // transient's own retained result and digest are unchanged.
                spectra: _,
            } => {
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_time_waveforms_owned(time, waveforms))
                    .with_measurements(measurements);
                let event_payload = transient_events_payload(events);
                match (periodic_state, event_payload) {
                    (Some(_), Some(_)) => AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        "A PSS result cannot retain both Floquet and transient-event payloads",
                        (self.now)(),
                    ),
                    (Some(operating_point), None) => {
                        let payload = match retain_pss_floquet_payload(&operating_point) {
                            Ok(payload) => payload,
                            Err(error) => {
                                return AnalysisResult::failed(
                                    1,
                                    analysis_type,
                                    label.to_string(),
                                    format!("Invalid retained PSS Floquet payload: {error}"),
                                    (self.now)(),
                                );
                            }
                        };
                        self.attach_validated_payload(result, analysis_type, label, payload)
                    }
                    (None, Some(payload)) => result.with_result_payload(payload),
                    (None, None) => result,
                }
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
                convergence: _,
            } => {
                let mut result =
                    AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                        .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms))
                        .with_measurements(measurements);
                result.family_metadata = reference_impedances_ohm.map(|reference_impedances_ohm| {
                    AnalysisResultFamilyMetadata::SParameter {
                        reference_impedances_ohm,
                        noise_reference_temperature_kelvin,
                    }
                });
                result
            }

            SimulationResult::Pstb {
                period,
                fundamental_frequency,
                modes,
                floquet_evidence,
                orbit_kind,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => {
                let payload = (|| -> Result<AnalysisResultPayload, String> {
                    Ok(AnalysisResultPayload::Pstb {
                        period_s: Some(period),
                        fundamental_frequency_hz: Some(fundamental_frequency),
                        stability_threshold: Some(stability_threshold),
                        probe_instance: Some(probe_instance),
                        detect_subharmonics: Some(detect_subharmonics),
                        modes: modes
                            .into_iter()
                            .map(|mode| {
                                Ok(PstbFloquetModeEvidence {
                                    multiplier: ComplexResultValue {
                                        real: mode.multiplier.0,
                                        imaginary: mode.multiplier.1,
                                    },
                                    exponent: ComplexResultValue {
                                        real: mode.exponent.0,
                                        imaginary: mode.exponent.1,
                                    },
                                    probe_participation: mode.probe_participation,
                                    is_unstable: mode.is_unstable,
                                    is_trivial: mode.is_trivial,
                                    subharmonic_order: mode
                                        .subharmonic_order
                                        .map(u64::try_from)
                                        .transpose()
                                        .map_err(|_| {
                                            "PSTB subharmonic order does not fit durable storage"
                                        })?,
                                })
                            })
                            .collect::<Result<Vec<_>, String>>()?,
                        floquet_evidence: retain_floquet_evidence(&floquet_evidence)?,
                        orbit_kind: retain_floquet_orbit_kind(orbit_kind)?,
                        trivial_multiplier_index: trivial_multiplier_index
                            .map(u64::try_from)
                            .transpose()
                            .map_err(
                                |_| "PSTB trivial Floquet index does not fit durable storage",
                            )?,
                        stability_verdict: retain_floquet_verdict(stability_verdict)?,
                        stability_classification: retain_pstb_classification(
                            stability_classification,
                        )?,
                        min_stability_margin_db,
                        max_multiplier_magnitude: Some(max_multiplier_magnitude),
                        num_unstable: Some(
                            u64::try_from(num_unstable)
                                .map_err(|_| "PSTB unstable count does not fit durable storage")?,
                        ),
                        subharmonics: subharmonics
                            .into_iter()
                            .map(u64::try_from)
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| "PSTB subharmonic order does not fit durable storage")?,
                        converged: Some(converged),
                        iterations: Some(
                            u64::try_from(iterations)
                                .map_err(|_| "PSTB iteration count does not fit durable storage")?,
                        ),
                    })
                })();
                let payload = match payload {
                    Ok(payload) => payload,
                    Err(error) => {
                        return AnalysisResult::failed(
                            1,
                            analysis_type,
                            label.to_string(),
                            format!("Invalid retained PSTB payload: {error}"),
                            (self.now)(),
                        );
                    }
                };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_ac_waveforms_owned(mode_indices, waveforms));
                self.attach_validated_payload(result, analysis_type, label, payload)
            }

            SimulationResult::Stb {
                response,
                measurements,
            } => {
                let waveforms = match AnalysisResultPayload::stb_waveforms(&response) {
                    Ok(waveforms) => waveforms,
                    Err(error) => {
                        return AnalysisResult::failed(
                            1,
                            analysis_type,
                            label.to_string(),
                            error,
                            (self.now)(),
                        );
                    }
                };
                let payload = AnalysisResultPayload::Stb { response };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(waveforms)
                    .with_measurements(measurements);
                self.attach_validated_payload(result, analysis_type, label, payload)
            }
            SimulationResult::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => {
                let payload = AnalysisResultPayload::Qpnoise { response };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms));
                self.attach_validated_payload(result, analysis_type, label, payload)
            }
            SimulationResult::Qpxf {
                frequencies,
                waveforms,
                response,
            } => {
                let payload = AnalysisResultPayload::Qpxf { response };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms));
                self.attach_validated_payload(result, analysis_type, label, payload)
            }
            SimulationResult::Qpac {
                frequencies,
                waveforms,
                response,
            } => {
                let payload = AnalysisResultPayload::Qpac { response };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms));
                self.attach_validated_payload(result, analysis_type, label, payload)
            }
            SimulationResult::Qpss {
                frequencies,
                waveforms,
                operating_point,
                ..
            } => {
                let payload = AnalysisResultPayload::Qpss { operating_point };
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms));
                self.attach_validated_payload(result, analysis_type, label, payload)
            }

            SimulationResult::HarmonicBalance {
                frequencies,
                waveforms,
                measurements,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms))
                .with_measurements(measurements),

            SimulationResult::DcSweep {
                evidence,
                sweep_values,
                sweep_var,
                waveforms,
                measurements,
                ..
            } => {
                if evidence
                    .as_ref()
                    .is_some_and(|evidence| !evidence.source.eq_ignore_ascii_case(&sweep_var))
                {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        "DC primary source disagrees with its curve evidence",
                        (self.now)(),
                    );
                }
                if let Some(evidence) = &evidence
                    && let Err(error) = evidence.validate_traces(
                        &sweep_values,
                        waveforms
                            .values()
                            .map(|trace| rspice_results::dc_sweep::DcTraceView {
                                name: &trace.name,
                                unit: Some(&trace.y_unit),
                                x: &trace.x_values,
                                sample_count: trace.y_values.len(),
                                complex: trace.is_complex || trace.y_imag.is_some(),
                            }),
                    )
                {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        error,
                        (self.now)(),
                    );
                }
                let mut result =
                    AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                        .with_waveforms(
                            self.build_waveforms_with_shared_x_owned(sweep_values, waveforms),
                        )
                        .with_measurements(measurements);
                result.result_payload =
                    evidence.map(|evidence| AnalysisResultPayload::DcSweep { evidence });
                if let Err(error) = result.validate_retained_evidence() {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        error,
                        (self.now)(),
                    );
                }
                result
            }

            SimulationResult::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => {
                let payload = AnalysisResultPayload::PoleZero {
                    poles: poles
                        .into_iter()
                        .map(|(real, imaginary)| ComplexResultValue { real, imaginary })
                        .collect(),
                    zeros: zeros
                        .into_iter()
                        .map(|(real, imaginary)| ComplexResultValue { real, imaginary })
                        .collect(),
                    pole_evidence,
                    zero_evidence,
                    gain,
                };
                self.analysis_result_with_validated_payload(analysis_type, label, payload)
            }

            SimulationResult::SensitivityStudy { evidence } => {
                sensitivity::analysis_result(analysis_type, label, evidence, &self.now)
            }

            SimulationResult::DcMismatch { evidence } => self
                .analysis_result_with_validated_payload(
                    analysis_type,
                    label,
                    AnalysisResultPayload::DcMismatch { evidence },
                ),

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
                    crate::results::TransferFunctionQuantity::Voltage => {
                        TransferFunctionQuantityEvidence::Voltage
                    }
                    crate::results::TransferFunctionQuantity::Current => {
                        TransferFunctionQuantityEvidence::Current
                    }
                };
                let scalar = |value| match value {
                    crate::results::TransferFunctionScalar::Finite(value) => {
                        TransferFunctionScalarEvidence::Finite(value)
                    }
                    crate::results::TransferFunctionScalar::PositiveInfinity => {
                        TransferFunctionScalarEvidence::PositiveInfinity
                    }
                    crate::results::TransferFunctionScalar::NegativeInfinity => {
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
                        rspice_simulation_contract::analysis_spec::TfNormalization::None => {
                            TransferFunctionNormalizationEvidence::None
                        }
                        rspice_simulation_contract::analysis_spec::TfNormalization::RelativeToNominal => {
                            TransferFunctionNormalizationEvidence::RelativeToNominal
                        }
                        rspice_simulation_contract::analysis_spec::TfNormalization::PerSourceUnit => {
                            TransferFunctionNormalizationEvidence::PerSourceUnit
                        }
                    },
                    accuracy: match accuracy {
                        rspice_simulation_contract::analysis_spec::TfAccuracy::Fast => {
                            TransferFunctionAccuracyEvidence::Fast
                        }
                        rspice_simulation_contract::analysis_spec::TfAccuracy::Balanced => {
                            TransferFunctionAccuracyEvidence::Balanced
                        }
                        rspice_simulation_contract::analysis_spec::TfAccuracy::Accurate => {
                            TransferFunctionAccuracyEvidence::Accurate
                        }
                        rspice_simulation_contract::analysis_spec::TfAccuracy::Robust => {
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
                member_measurements,
            } => {
                let (waveforms, variables) = self.build_monte_carlo_payload_owned(variables);
                AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(waveforms)
                    .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
                        seed,
                        runs_requested,
                        runs_completed,
                        failures: num_failures,
                        all_converged,
                        variables,
                        member_measurements,
                    })
            }

            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => {
                let retained_sweep_values = sweep_values.clone();
                AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(
                        self.build_waveforms_with_shared_x_owned(sweep_values, waveforms),
                    )
                    .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
                        target,
                        sweep_values: retained_sweep_values,
                        failed_points: num_failures,
                        member_measurements,
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
                member_measurements,
            } => {
                let retained_x_values = x_values.clone();
                AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_waveforms_with_shared_x_owned(x_values, waveforms))
                    .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                        x_values: retained_x_values,
                        x_label,
                        x_unit,
                        temperatures_c,
                        corner_labels,
                        failed_corners: num_failures,
                        member_measurements,
                    })
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                best_objectives,
                best_constraints,
                converged,
            } => {
                let retained_iterations = iterations.clone();
                AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                    .with_waveforms(self.build_waveforms_with_shared_x_owned(iterations, waveforms))
                    .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                        iterations: retained_iterations,
                        best_cost,
                        best_variables: best_variables.into_iter().collect(),
                        best_objectives,
                        best_constraints,
                        converged,
                    })
            }

            SimulationResult::Soa {
                source_history,
                time,
                waveforms,
                violations,
                evaluations,
                convergence: _,
            } => {
                let retained_time = source_history
                    .as_ref()
                    .map_or_else(|| time.clone(), |source| source.time.clone());
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
                            rspice_results::safety::ViolationSeverity::Warning => {
                                SoaViolationSeverityEvidence::Warning
                            }
                            rspice_results::safety::ViolationSeverity::Violation => {
                                SoaViolationSeverityEvidence::Violation
                            }
                            rspice_results::safety::ViolationSeverity::Critical => {
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
                        duration: evaluation.duration,
                        thresholds: evaluation.thresholds,
                        envelope: evaluation.envelope,
                        derating: evaluation.derating,
                        device_id: evaluation.device_id,
                        parameter: retain_soa_parameter(evaluation.parameter),
                        limit_value: evaluation.limit_value,
                        worst_actual_value: evaluation.worst_actual_value,
                        worst_time_s: evaluation.worst_time,
                        sample_count: evaluation.sample_count,
                        unit: evaluation.unit,
                        description: evaluation.description,
                        verdict: match evaluation.verdict {
                            rspice_results::safety::SoARuleVerdict::Pass => {
                                SoaRuleVerdictEvidence::Pass
                            }
                            rspice_results::safety::SoARuleVerdict::Warning => {
                                SoaRuleVerdictEvidence::Warning
                            }
                            rspice_results::safety::SoARuleVerdict::Violation => {
                                SoaRuleVerdictEvidence::Violation
                            }
                            rspice_results::safety::SoARuleVerdict::Critical => {
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
                    source_history,
                    evaluations,
                    violations,
                };
                if let Err(error) = payload.validate_for(analysis_type) {
                    return AnalysisResult::failed(
                        1,
                        analysis_type,
                        label.to_string(),
                        format!("Invalid retained SOA payload: {error}"),
                        (self.now)(),
                    );
                }
                let result = AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
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
                        (self.now)(),
                    ),
                }
            }

            SimulationResult::Fft { spectrum, .. } => recorded_fft::analysis_result(
                analysis_type,
                label,
                &spectrum,
                |x, w| self.build_ac_waveforms_owned(x, w),
                &self.now,
            ),
            SimulationResult::MeasurementsOnly { measurements } => {
                let payload = AnalysisResultPayload::ScalarMeasurements {
                    values: measurements.into_iter().collect(),
                };
                self.analysis_result_with_validated_payload(analysis_type, label, payload)
            }
        };
        retained.convergence = convergence;
        if retained.success {
            retained.retain_native_scalar_units();
        }
        retained
    }

    fn analysis_result_with_validated_payload(
        &self,
        analysis_type: AnalysisType,
        label: &str,
        payload: AnalysisResultPayload,
    ) -> AnalysisResult {
        match payload.validate_for(analysis_type) {
            Ok(()) => AnalysisResult::new(1, analysis_type, label.to_string(), (self.now)())
                .with_result_payload(payload),
            Err(error) => AnalysisResult::failed(
                1,
                analysis_type,
                label.to_string(),
                format!("Invalid retained analysis payload: {error}"),
                (self.now)(),
            ),
        }
    }

    fn attach_validated_payload(
        &self,
        result: AnalysisResult,
        analysis_type: AnalysisType,
        label: &str,
        payload: AnalysisResultPayload,
    ) -> AnalysisResult {
        if let Err(error) = payload.validate_for(analysis_type) {
            return AnalysisResult::failed(
                1,
                analysis_type,
                label.to_string(),
                format!("Invalid retained analysis payload: {error}"),
                (self.now)(),
            );
        }
        let result = result.with_result_payload(payload);
        match result.validate_retained_evidence() {
            Ok(()) => result,
            Err(error) => AnalysisResult::failed(
                1,
                analysis_type,
                label.to_string(),
                format!("Invalid retained analysis payload: {error}"),
                (self.now)(),
            ),
        }
    }
}

#[cfg(test)]
mod convergence_conversion_tests {
    use super::*;

    #[test]
    fn transient_convergence_changes_retained_result_identity() {
        let convert = |force_accepted| {
            let time = vec![0.0, 1e-9];
            let mut convergence = rspice_core::diagnostics::ConvergenceQuality::default();
            if force_accepted {
                convergence.record_force_accept(1);
            }
            let convergence = Some(std::sync::Arc::new(
                rspice_results::convergence_quality::TransientConvergenceEvidence::capture(
                    convergence,
                    &time,
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap(),
            ));
            let result = crate::results::SimulationResult::Transient {
                spectra: Vec::new(),
                time: time.clone(),
                waveforms: HashMap::from([(
                    "V(out)".to_owned(),
                    crate::results::WaveformData::new_time_domain("V(out)", time, vec![0.0, 0.8]),
                )]),
                measurements: Vec::new(),
                periodic_state: None,
                convergence,
                events: Default::default(),
            };
            super::convert(result, AnalysisType::Transient, "Quality retention", || 0.0)
        };
        let clean = convert(false);
        let degraded = convert(true);
        assert!(clean.success && degraded.success);
        assert_ne!(
            clean
                .result_data_ref()
                .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT),
            degraded
                .result_data_ref()
                .digest(rspice_results::result_digest::ResultDigestEncoding::CURRENT),
            "identical samples with different solver quality are different retained evidence"
        );
        assert!(
            degraded.result_data_ref().retained_data_bytes()
                > clean.result_data_ref().retained_data_bytes(),
            "retained quality records must participate in resource accounting"
        );
    }
}

#[cfg(test)]
mod waveform_unit_conversion_tests {
    use super::*;

    fn producer_waveform(
        name: &str,
        y_values: Vec<f64>,
        y_unit: &str,
    ) -> crate::results::WaveformData {
        crate::results::WaveformData {
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
        let sim_result = crate::results::SimulationResult::Transient {
            spectra: Vec::new(),
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

        let result = super::convert(sim_result, AnalysisType::Transient, "TRAN", || 0.0);

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
        let mut spectrum = crate::results::WaveformData::new_complex(
            "V(out) Spectrum",
            vec![1.0, 10.0],
            vec![1.0, 0.0],
            vec![0.0, 1.0],
        );
        spectrum.y_unit = "V".to_owned();
        let sim_result = crate::results::SimulationResult::Ac {
            convergence: None,
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: None,
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([("V(out) Spectrum".to_owned(), spectrum)]),
            measurements: Vec::new(),
        };

        let result = super::convert(sim_result, AnalysisType::Ac, "AC", || 0.0);

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
        let group_delay = crate::results::WaveformData::new_time_domain_in_unit(
            "group_delay",
            vec![1.0, 10.0],
            vec![2.0e-9, 3.0e-9],
            "s",
        );
        let sim_result = crate::results::SimulationResult::Ac {
            convergence: None,
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: None,
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([("group_delay".to_owned(), group_delay)]),
            measurements: Vec::new(),
        };

        let result = super::convert(sim_result, AnalysisType::Pxf, "PXF", || 0.0);

        assert_eq!(result.waveforms[0].name, "group_delay");
        assert_eq!(result.waveforms[0].unit.as_deref(), Some("s"));
    }

    #[test]
    fn a_frequency_companion_curve_keeps_its_own_exact_abscissa() {
        let group_delay = crate::results::WaveformData::new_time_domain_in_unit(
            "group_delay",
            vec![3.0, 30.0],
            vec![2.0e-9, 3.0e-9],
            "s",
        );
        let result = super::convert(
            crate::results::SimulationResult::Ac {
                convergence: None,
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: None,
                frequencies: vec![1.0, 10.0, 100.0],
                waveforms: HashMap::from([("group_delay".to_owned(), group_delay)]),
                measurements: Vec::new(),
            },
            AnalysisType::Pxf,
            "PXF",
            || 0.0,
        );

        assert_eq!(result.waveforms[0].x.as_slice(), &[3.0, 30.0]);
        assert_eq!(result.waveforms[0].y.as_slice(), &[2.0e-9, 3.0e-9]);
    }

    #[test]
    fn a_complex_time_domain_waveform_retains_both_components_and_phase() {
        let waveform = crate::results::WaveformData::new_complex_in_unit(
            "V(env)",
            vec![0.0, 1.0],
            vec![3.0, 0.0],
            vec![4.0, 1.0],
            "V",
        );
        let result = super::convert(
            crate::results::SimulationResult::Transient {
                spectra: Vec::new(),
                time: vec![0.0, 1.0],
                waveforms: HashMap::from([("V(env)".to_owned(), waveform)]),
                measurements: Vec::new(),
                periodic_state: None,
                convergence: Default::default(),
                events: Default::default(),
            },
            AnalysisType::Envelope,
            "Envelope",
            || 0.0,
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
        let sim_result = crate::results::SimulationResult::Ac {
            convergence: None,
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: None,
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([(
                "V(out)".to_owned(),
                crate::results::WaveformData::new_complex(
                    "V(out)",
                    vec![1.0, 10.0],
                    vec![1.0, 0.0],
                    vec![0.0, 1.0],
                ),
            )]),
            measurements: Vec::new(),
        };

        let result = super::convert(sim_result, AnalysisType::Ac, "AC", || 0.0);

        assert_eq!(retained_unit(&result, "|V(out)|"), None);
    }

    #[test]
    fn retained_noise_series_state_no_unit_so_phase_noise_is_never_called_a_psd() {
        let sim_result = crate::results::SimulationResult::Noise {
            output_unit: None,
            frequencies: vec![1.0e3, 1.0e6],
            output_noise: vec![-90.0, -130.0],
            input_noise: None,
            contributors: HashMap::new(),
            summary: None,
            measurements: Vec::new(),
        };

        let result = super::convert(sim_result, AnalysisType::Pnoise, "PNOISE", || 0.0);

        assert_eq!(retained_unit(&result, "onoise"), None);
    }
}
