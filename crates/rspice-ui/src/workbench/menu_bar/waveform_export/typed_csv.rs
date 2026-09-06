//! The table an analysis whose result is a report rather than a waveform
//! publishes.
//!
//! An operating point, a sensitivity table, a periodic-orbit certificate, a
//! reliability checkpoint and their kin each retain evidence with its own
//! shape, and each spells that evidence as its own rows. Those spellings are
//! one concern — what a family's table looks like — and they live here
//! so that the routing in the parent reads as routing.
//!
//! The result is a [`PreparedTypedResultCsv`], which the parent publishes as
//! CSV or, delimiter-translated, as TSV.

use super::{PreparedTypedResultCsv, csv_text};

pub(super) fn prepare_typed_result_csv(
    analysis: &crate::state::AnalysisResult,
) -> Option<PreparedTypedResultCsv> {
    let payload = analysis.result_payload.as_ref()?;
    if !analysis.success || analysis.validate_retained_evidence().is_err() {
        return None;
    }

    use crate::state::{AnalysisResultPayload, SensitivityResultMode};
    match payload {
        AnalysisResultPayload::OperatingPoint {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            violation_devices,
            violation_source_content_digest,
            validated_startup_directives,
            mna_node_names,
            mna_branch_names,
            mna_solution,
            effective_source_content_digest,
            run_point_index,
            run_point_count,
            run_point_process,
            run_point_supply_voltage,
            run_point_nominal_supply_voltage,
        } => {
            let mut contents = String::from("field,value,unit\n");
            for (field, value, unit) in [
                (
                    "temperature_mode",
                    serialized_enum_name(temperature_mode),
                    "",
                ),
                (
                    "temperature_celsius",
                    format!("{temperature_celsius:.17e}"),
                    "degC",
                ),
                ("initial_guess", serialized_enum_name(initial_guess), ""),
                (
                    "node_initialization",
                    serialized_enum_name(node_initialization),
                    "",
                ),
                ("homotopy", serialized_enum_name(homotopy), ""),
                ("annotation", serialized_enum_name(annotation), ""),
                ("device_detail", serialized_enum_name(device_detail), ""),
                ("save_device_op", serialized_enum_name(save_device_op), ""),
                ("accuracy", serialized_enum_name(accuracy), ""),
                (
                    "validated_startup_directives",
                    validated_startup_directives.to_string(),
                    "count",
                ),
                ("selected_devices", selected_devices.join(";"), ""),
                ("violation_devices", violation_devices.join(";"), ""),
                (
                    "violation_source_content_digest",
                    violation_source_content_digest
                        .map_or_else(String::new, |digest| digest.to_string()),
                    "sha256",
                ),
                ("mna_nodes", mna_node_names.len().to_string(), "count"),
                ("mna_branches", mna_branch_names.len().to_string(), "count"),
                ("mna_values", mna_solution.len().to_string(), "count"),
                (
                    "effective_source_content_digest",
                    effective_source_content_digest
                        .map_or_else(String::new, |digest| digest.to_string()),
                    "sha256",
                ),
                ("run_point_index", run_point_index.to_string(), "zero_based"),
                ("run_point_count", run_point_count.to_string(), "count"),
                (
                    "run_point_process",
                    serialized_enum_name(run_point_process),
                    "",
                ),
                (
                    "run_point_supply_voltage",
                    run_point_supply_voltage
                        .map(|voltage| format!("{voltage:.17e}"))
                        .unwrap_or_default(),
                    "V",
                ),
                (
                    "run_point_nominal_supply_voltage",
                    run_point_nominal_supply_voltage
                        .map(|voltage| format!("{voltage:.17e}"))
                        .unwrap_or_default(),
                    "V",
                ),
            ] {
                contents.push_str(&format!(
                    "{},{},{}\n",
                    csv_text(field),
                    csv_text(&value),
                    csv_text(unit)
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "operating-point-contract.csv",
                contents,
                detail: "exact operating-point execution and retention contract".to_owned(),
            })
        }
        AnalysisResultPayload::PoleZero {
            poles,
            zeros,
            pole_evidence,
            zero_evidence,
            gain,
        } => {
            let mut contents =
                String::from("record,index,real_rad_per_s,imaginary_rad_per_s,value\n");
            let gain = gain
                .map(|gain| format!("{gain:.17e}"))
                .unwrap_or_else(|| "unavailable".to_owned());
            contents.push_str(&format!("gain,,,,{gain}\n"));
            append_pole_zero_evidence_csv(&mut contents, "pole", pole_evidence);
            append_pole_zero_evidence_csv(&mut contents, "zero", zero_evidence);
            for (kind, roots) in [("pole", poles), ("zero", zeros)] {
                for (index, root) in roots.iter().enumerate() {
                    contents.push_str(&format!(
                        "{kind},{index},{:.17e},{:.17e},\n",
                        root.real, root.imaginary
                    ));
                }
            }
            Some(PreparedTypedResultCsv {
                default_name: "pole-zero.csv",
                contents,
                detail: format!(
                    "{} poles ({}), {} zeros ({}), DC gain {}",
                    poles.len(),
                    pole_evidence.label(),
                    zeros.len(),
                    zero_evidence.label(),
                    if gain == "unavailable" {
                        "unavailable"
                    } else {
                        "retained"
                    }
                ),
            })
        }
        AnalysisResultPayload::PssFloquet {
            period_s,
            fundamental_frequency_hz,
            iterations,
            residual_norm,
            multipliers,
            floquet_evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
        } => {
            let mut contents = periodic_csv_header();
            for (field, value, unit) in [
                ("period_s", optional_f64_csv(*period_s), "s"),
                (
                    "fundamental_frequency_hz",
                    optional_f64_csv(*fundamental_frequency_hz),
                    "Hz",
                ),
                ("iterations", optional_u64_csv(*iterations), "count"),
                ("residual_norm", optional_f64_csv(*residual_norm), ""),
                (
                    "retained_multiplier_count",
                    multipliers.len().to_string(),
                    "count",
                ),
                (
                    "authenticated_complete_multiplier_count",
                    authenticated_floquet_count_csv(multipliers.len(), floquet_evidence),
                    "count",
                ),
                (
                    "floquet_evidence_json",
                    serde_json::to_string(floquet_evidence)
                        .expect("Floquet evidence is JSON-serializable"),
                    "",
                ),
                ("orbit_kind", serialized_enum_name(orbit_kind), ""),
                (
                    "trivial_multiplier_index",
                    optional_u64_csv(*trivial_multiplier_index),
                    "zero_based",
                ),
                (
                    "stability_verdict",
                    serialized_enum_name(stability_verdict),
                    "",
                ),
            ] {
                append_periodic_metadata_csv(&mut contents, field, &value, unit);
            }
            append_floquet_certificate_csv(&mut contents, floquet_evidence);
            for (index, multiplier) in multipliers.iter().enumerate() {
                append_periodic_csv_row(
                    &mut contents,
                    [
                        "multiplier".to_owned(),
                        index.to_string(),
                        format!("{:.17e}", multiplier.multiplier.real),
                        format!("{:.17e}", multiplier.multiplier.imaginary),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                    ],
                );
            }
            Some(PreparedTypedResultCsv {
                default_name: "pss-floquet-evidence.csv",
                contents,
                detail: format!(
                    "{} with retained qualification evidence",
                    floquet_export_count_label(
                        multipliers.len(),
                        floquet_evidence,
                        "PSS Floquet multipliers"
                    )
                ),
            })
        }
        AnalysisResultPayload::Pstb {
            period_s,
            fundamental_frequency_hz,
            stability_threshold,
            probe_instance,
            detect_subharmonics,
            modes,
            floquet_evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
            stability_classification,
            min_stability_margin_db,
            max_multiplier_magnitude,
            num_unstable,
            subharmonics,
            converged,
            iterations,
        } => {
            let mut contents = periodic_csv_header();
            for (field, value, unit) in [
                ("period_s", optional_f64_csv(*period_s), "s"),
                (
                    "fundamental_frequency_hz",
                    optional_f64_csv(*fundamental_frequency_hz),
                    "Hz",
                ),
                (
                    "stability_threshold",
                    optional_f64_csv(*stability_threshold),
                    "multiplier_magnitude",
                ),
                (
                    "probe_instance",
                    probe_instance.clone().unwrap_or_default(),
                    "",
                ),
                (
                    "detect_subharmonics",
                    optional_bool_csv(*detect_subharmonics),
                    "",
                ),
                ("retained_mode_count", modes.len().to_string(), "count"),
                (
                    "authenticated_complete_mode_count",
                    authenticated_floquet_count_csv(modes.len(), floquet_evidence),
                    "count",
                ),
                (
                    "floquet_evidence_json",
                    serde_json::to_string(floquet_evidence)
                        .expect("Floquet evidence is JSON-serializable"),
                    "",
                ),
                ("orbit_kind", serialized_enum_name(orbit_kind), ""),
                (
                    "trivial_multiplier_index",
                    optional_u64_csv(*trivial_multiplier_index),
                    "zero_based",
                ),
                (
                    "stability_verdict",
                    serialized_enum_name(stability_verdict),
                    "",
                ),
                (
                    "stability_classification",
                    serialized_enum_name(stability_classification),
                    "",
                ),
                (
                    "min_stability_margin_db",
                    optional_f64_csv(*min_stability_margin_db),
                    "dB",
                ),
                (
                    "max_multiplier_magnitude",
                    optional_f64_csv(*max_multiplier_magnitude),
                    "",
                ),
                ("num_unstable", optional_u64_csv(*num_unstable), "count"),
                (
                    "subharmonics",
                    subharmonics
                        .iter()
                        .map(u64::to_string)
                        .collect::<Vec<_>>()
                        .join(";"),
                    "orders",
                ),
                ("converged", optional_bool_csv(*converged), ""),
                ("iterations", optional_u64_csv(*iterations), "count"),
            ] {
                append_periodic_metadata_csv(&mut contents, field, &value, unit);
            }
            append_floquet_certificate_csv(&mut contents, floquet_evidence);
            for (index, mode) in modes.iter().enumerate() {
                append_periodic_csv_row(
                    &mut contents,
                    [
                        "mode".to_owned(),
                        index.to_string(),
                        format!("{:.17e}", mode.multiplier.real),
                        format!("{:.17e}", mode.multiplier.imaginary),
                        format!("{:.17e}", mode.exponent.real),
                        format!("{:.17e}", mode.exponent.imaginary),
                        format!("{:.17e}", mode.probe_participation),
                        mode.is_unstable.to_string(),
                        mode.is_trivial.to_string(),
                        mode.subharmonic_order
                            .map_or_else(String::new, |order| order.to_string()),
                        String::new(),
                        String::new(),
                        String::new(),
                    ],
                );
            }
            Some(PreparedTypedResultCsv {
                default_name: "pstb-floquet-evidence.csv",
                contents,
                detail: format!(
                    "{} with retained qualification and stability evidence",
                    floquet_export_count_label(modes.len(), floquet_evidence, "PSTB Floquet modes")
                ),
            })
        }
        AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        } => {
            let (mode, frequency) = match result_mode {
                SensitivityResultMode::Dc => ("dc", String::new()),
                SensitivityResultMode::Ac { frequency_hz } => {
                    ("ac", format!("{frequency_hz:.17e}"))
                }
            };
            let escaped_output = csv_text(output);
            let mut contents = String::from(
                "parameter,raw_sensitivity,normalized_sensitivity,output,mode,frequency_hz\n",
            );
            for row in rows {
                contents.push_str(&format!(
                    "{},{:.17e},{:.17e},{escaped_output},{mode},{frequency}\n",
                    csv_text(&row.parameter),
                    row.raw,
                    row.normalized,
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "sensitivity.csv",
                contents,
                detail: format!("{} exact sensitivity rows", rows.len()),
            })
        }
        AnalysisResultPayload::ScalarMeasurements { values } => {
            let mut contents = String::from("name,value\n");
            for (name, value) in values {
                contents.push_str(&format!("{},{value:.17e}\n", csv_text(name)));
            }
            Some(PreparedTypedResultCsv {
                default_name: "scalar-results.csv",
                contents,
                detail: format!("{} exact scalar values", values.len()),
            })
        }
        AnalysisResultPayload::TransferFunction {
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
            let normalization_label = match normalization {
                crate::state::TransferFunctionNormalizationEvidence::None => "disabled",
                crate::state::TransferFunctionNormalizationEvidence::RelativeToNominal => {
                    "relative_to_nominal"
                }
                crate::state::TransferFunctionNormalizationEvidence::PerSourceUnit => {
                    "per_source_unit"
                }
            };
            let accuracy_label = match accuracy {
                crate::state::TransferFunctionAccuracyEvidence::Fast => "fast",
                crate::state::TransferFunctionAccuracyEvidence::Balanced => "balanced",
                crate::state::TransferFunctionAccuracyEvidence::Accurate => "accurate",
                crate::state::TransferFunctionAccuracyEvidence::Robust => "robust",
            };
            let gain_unit = if matches!(
                normalization,
                crate::state::TransferFunctionNormalizationEvidence::RelativeToNominal
            ) {
                "1"
            } else {
                match (input_quantity, output_quantity) {
                    (
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                    ) => "V/V",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                        crate::state::TransferFunctionQuantityEvidence::Current,
                    ) => "A/V",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Current,
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                    ) => "V/A",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Current,
                        crate::state::TransferFunctionQuantityEvidence::Current,
                    ) => "A/A",
                }
            };
            let mut contents = String::from(
                "quantity,value,unit,input_source,output_expression,normalization,accuracy,solve_point\n",
            );
            let mut rows = 0usize;
            let mut push_scalar = |quantity: &str,
                                   value: &crate::state::TransferFunctionScalarEvidence,
                                   unit: &str| {
                let value = match value {
                    crate::state::TransferFunctionScalarEvidence::Finite(value) => {
                        format!("{value:.17e}")
                    }
                    crate::state::TransferFunctionScalarEvidence::PositiveInfinity => {
                        "+infinity".to_owned()
                    }
                    crate::state::TransferFunctionScalarEvidence::NegativeInfinity => {
                        "-infinity".to_owned()
                    }
                };
                contents.push_str(&format!(
                    "{quantity},{value},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(unit),
                    csv_text(input_source),
                    csv_text(output_expression),
                ));
                rows += 1;
            };
            if let Some(gain) = gain {
                push_scalar("transfer_gain", gain, gain_unit);
            }
            if let Some(input_resistance) = input_resistance {
                push_scalar("input_resistance", input_resistance, "ohm");
            }
            if let Some(output_resistance) = output_resistance {
                push_scalar("output_resistance", output_resistance, "ohm");
            }
            if let Some(value) = nominal_input {
                contents.push_str(&format!(
                    "nominal_input,{value:.17e},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(input_unit), csv_text(input_source), csv_text(output_expression)
                ));
                rows += 1;
            }
            if let Some(value) = nominal_output {
                contents.push_str(&format!(
                    "nominal_output,{value:.17e},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(output_unit), csv_text(input_source), csv_text(output_expression)
                ));
                rows += 1;
            }
            Some(PreparedTypedResultCsv {
                default_name: "transfer-function.csv",
                contents,
                detail: format!("{rows} exact transfer-function values"),
            })
        }
        AnalysisResultPayload::Reliability { devices } => {
            let mut contents = String::from(
                "device,lifetime_years,average_gate_stress_v,average_drain_stress_v,average_temperature_k,duration_s,threshold_voltage_shift_v,mobility_shift,drain_source_resistance_shift\n",
            );
            let row_count = devices
                .iter()
                .map(|device| device.checkpoints.len())
                .sum::<usize>();
            for device in devices {
                for checkpoint in &device.checkpoints {
                    let shift = &checkpoint.shift;
                    contents.push_str(&format!(
                        "{},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e}\n",
                        csv_text(&device.device_id),
                        checkpoint.years,
                        device.stress.average_gate_stress_v,
                        device.stress.average_drain_stress_v,
                        device.stress.average_temperature_k,
                        device.stress.duration_s,
                        shift.threshold_voltage_shift_v,
                        shift.mobility_shift,
                        shift.drain_source_resistance_shift,
                    ));
                }
            }
            Some(PreparedTypedResultCsv {
                default_name: "reliability-evidence.csv",
                contents,
                detail: format!(
                    "{} devices, {} exact lifetime-shift rows",
                    devices.len(),
                    row_count
                ),
            })
        }
        AnalysisResultPayload::Soa {
            evaluations,
            violations,
        } => {
            let mut contents = String::from(
                "record,device,parameter,limit_value,actual_value,time_s,sample_count,unit,description,verdict\n",
            );
            for evaluation in evaluations {
                contents.push_str(&format!(
                    "evaluation,{},{},{:.17e},{:.17e},{:.17e},{},{},{},{}\n",
                    csv_text(&evaluation.device_id),
                    soa_parameter_csv(evaluation.parameter),
                    evaluation.limit_value,
                    evaluation.worst_actual_value,
                    evaluation.worst_time_s,
                    evaluation.sample_count,
                    csv_text(&evaluation.unit),
                    csv_text(&evaluation.description),
                    soa_verdict_csv(evaluation.verdict),
                ));
            }
            for violation in violations {
                contents.push_str(&format!(
                    "event,{},{},{:.17e},{:.17e},{:.17e},,,,{}\n",
                    csv_text(&violation.device_id),
                    soa_parameter_csv(violation.parameter),
                    violation.limit_value,
                    violation.actual_value,
                    violation.time_s,
                    soa_violation_severity_csv(violation.severity),
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "soa-evidence.csv",
                contents,
                detail: format!(
                    "{} evaluated rules, {} warning/violation events",
                    evaluations.len(),
                    violations.len()
                ),
            })
        }
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            ..
        } => {
            let mut contents = String::from("node,domain,time_s,value_code,value\n");
            for trace in digital_traces {
                for point in &trace.points {
                    contents.push_str(&format!(
                        "{},digital,{:.17e},{},\n",
                        csv_text(&trace.node_name),
                        point.time_s,
                        point.value_code,
                    ));
                }
            }
            for trace in real_traces {
                for point in &trace.points {
                    contents.push_str(&format!(
                        "{},real,{:.17e},,{:.17e}\n",
                        csv_text(&trace.node_name),
                        point.time_s,
                        point.value,
                    ));
                }
            }
            let events: usize = digital_traces
                .iter()
                .map(|trace| trace.points.len())
                .chain(real_traces.iter().map(|trace| trace.points.len()))
                .sum();
            Some(PreparedTypedResultCsv {
                default_name: "event-history.csv",
                contents,
                detail: format!(
                    "{} event nodes, {events} committed events",
                    digital_traces.len() + real_traces.len()
                ),
            })
        }
    }
}

fn periodic_csv_header() -> String {
    "record,index,multiplier_real,multiplier_imaginary,exponent_real_per_s,exponent_imaginary_per_s,probe_participation,is_unstable,is_trivial,subharmonic_order,field,value,unit\n".to_owned()
}

fn append_periodic_csv_row(contents: &mut String, fields: [String; 13]) {
    contents.push_str(
        &fields
            .iter()
            .map(|field| csv_text(field))
            .collect::<Vec<_>>()
            .join(","),
    );
    contents.push('\n');
}

fn append_periodic_metadata_csv(contents: &mut String, field: &str, value: &str, unit: &str) {
    append_periodic_csv_row(
        contents,
        [
            "metadata".to_owned(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            field.to_owned(),
            value.to_owned(),
            unit.to_owned(),
        ],
    );
}

fn append_floquet_certificate_csv(
    contents: &mut String,
    evidence: &crate::state::FloquetSpectrumEvidence,
) {
    if let Some(certificate) = evidence.certificate() {
        for (field, value) in [
            (
                "certificate_problem_order",
                certificate.problem_order.to_string(),
            ),
            (
                "certificate_max_backward_error",
                format!("{:.17e}", certificate.max_backward_error),
            ),
            (
                "certificate_qualification_tolerance",
                format!("{:.17e}", certificate.qualification_tolerance),
            ),
        ] {
            append_periodic_metadata_csv(contents, field, &value, "");
        }
    }
}

fn authenticated_floquet_count_csv(
    count: usize,
    evidence: &crate::state::FloquetSpectrumEvidence,
) -> String {
    matches!(
        evidence,
        crate::state::FloquetSpectrumEvidence::Qualified { .. }
            | crate::state::FloquetSpectrumEvidence::NoDynamicModes
    )
    .then(|| count.to_string())
    .unwrap_or_default()
}

fn floquet_export_count_label(
    count: usize,
    evidence: &crate::state::FloquetSpectrumEvidence,
    noun: &str,
) -> String {
    if matches!(
        evidence,
        crate::state::FloquetSpectrumEvidence::Qualified { .. }
            | crate::state::FloquetSpectrumEvidence::NoDynamicModes
    ) {
        format!("{count} complete {noun}")
    } else {
        format!("{count} retained {noun}; completeness unavailable")
    }
}

fn optional_f64_csv(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value:.17e}"))
}

fn optional_u64_csv(value: Option<u64>) -> String {
    value.map_or_else(String::new, |value| value.to_string())
}

fn optional_bool_csv(value: Option<bool>) -> String {
    value.map_or_else(String::new, |value| value.to_string())
}

fn soa_parameter_csv(parameter: crate::state::SoaParameterEvidence) -> &'static str {
    use crate::state::SoaParameterEvidence;
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "vgs",
        SoaParameterEvidence::DrainSourceVoltage => "vds",
        SoaParameterEvidence::GateDrainVoltage => "vgd",
        SoaParameterEvidence::BaseEmitterVoltage => "vbe",
        SoaParameterEvidence::CollectorEmitterVoltage => "vce",
        SoaParameterEvidence::BaseCollectorVoltage => "vbc",
        SoaParameterEvidence::DrainCurrent => "id",
        SoaParameterEvidence::CollectorCurrent => "ic",
        SoaParameterEvidence::PowerDissipation => "pdiss",
        SoaParameterEvidence::Temperature => "temperature",
    }
}

fn soa_verdict_csv(verdict: crate::state::SoaRuleVerdictEvidence) -> &'static str {
    use crate::state::SoaRuleVerdictEvidence;
    match verdict {
        SoaRuleVerdictEvidence::Pass => "pass",
        SoaRuleVerdictEvidence::Warning => "warning",
        SoaRuleVerdictEvidence::Violation => "violation",
        SoaRuleVerdictEvidence::Critical => "critical",
    }
}

fn soa_violation_severity_csv(
    severity: crate::state::SoaViolationSeverityEvidence,
) -> &'static str {
    use crate::state::SoaViolationSeverityEvidence;
    match severity {
        SoaViolationSeverityEvidence::Warning => "warning",
        SoaViolationSeverityEvidence::Violation => "violation",
        SoaViolationSeverityEvidence::Critical => "critical",
    }
}

fn append_pole_zero_evidence_csv(
    contents: &mut String,
    root_kind: &str,
    evidence: &crate::state::PoleZeroRootSetEvidence,
) {
    contents.push_str(&format!("{root_kind}_evidence,,,,{}\n", evidence.label()));
    if let Some(certificate) = evidence.certificate() {
        for (field, value) in [
            ("problem_order", certificate.problem_order.to_string()),
            ("infinite_count", certificate.infinite_count.to_string()),
            (
                "max_backward_error",
                format!("{:.17e}", certificate.max_backward_error),
            ),
            (
                "qualification_tolerance",
                format!("{:.17e}", certificate.qualification_tolerance),
            ),
        ] {
            contents.push_str(&format!("{root_kind}_{field},,,,{value}\n"));
        }
    }
}

fn serialized_enum_name<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("retained evidence enums are JSON-serializable")
        .trim_matches('"')
        .to_owned()
}
