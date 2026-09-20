//! Measurements over a result.
//!
//! Derived scalars — rise time, overshoot, bandwidth, gain margin — computed
//! from the stored traces rather than requested from the engine, so a
//! measurement can be added without re-running.

use super::*;

impl SimulationResult {
    /// Get a single scalar measurement by name without allocating a map.
    pub fn measurement(&self, name: &str) -> Option<f64> {
        let key = name.trim();
        if key.is_empty() {
            return None;
        }

        match self {
            SimulationResult::DcOp(op) => measurement_from_dc_op(op, key),
            SimulationResult::DcSweep {
                waveforms,
                measurements,
                ..
            }
            | SimulationResult::Transient {
                waveforms,
                measurements,
                ..
            }
            | SimulationResult::Ac {
                waveforms,
                measurements,
                ..
            }
            | SimulationResult::HarmonicBalance {
                waveforms,
                measurements,
                ..
            } => measurement_result_by_name(measurements, key)
                .or_else(|| waveform_last_value_by_name(waveforms, key)),
            SimulationResult::Qpss {
                operating_point, ..
            } => match key {
                "qpss.iterations" => Some(operating_point.iterations() as f64),
                "qpss.normalized_residual" => Some(operating_point.normalized_residual()),
                _ => None,
            },
            SimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                detect_subharmonics,
                modes,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                waveforms,
                ..
            } => {
                if key.eq_ignore_ascii_case("pstb.period") {
                    Some(*period)
                } else if key.eq_ignore_ascii_case("pstb.fundamental_frequency") {
                    Some(*fundamental_frequency)
                } else if key.eq_ignore_ascii_case("pstb.stability_threshold") {
                    Some(*stability_threshold)
                } else if key.eq_ignore_ascii_case("pstb.detect_subharmonics") {
                    Some(if *detect_subharmonics { 1.0 } else { 0.0 })
                } else if key.eq_ignore_ascii_case("pstb.mode_count") {
                    Some(modes.len() as f64)
                } else if key.eq_ignore_ascii_case("pstb.unstable_mode_count") {
                    Some(*num_unstable as f64)
                } else if key.eq_ignore_ascii_case("pstb.max_multiplier_magnitude") {
                    Some(*max_multiplier_magnitude)
                } else if key.eq_ignore_ascii_case("pstb.min_stability_margin_db") {
                    *min_stability_margin_db
                } else {
                    waveform_last_value_by_name(waveforms, key)
                }
            }
            SimulationResult::Parametric { waveforms, .. }
            | SimulationResult::Corner { waveforms, .. }
            | SimulationResult::Reliability { waveforms, .. }
            | SimulationResult::Optimization { waveforms, .. }
            | SimulationResult::Soa { waveforms, .. } => {
                waveform_last_value_by_name(waveforms, key)
            }
            SimulationResult::Noise {
                output_noise,
                input_noise,
                contributors,
                measurements,
                ..
            } => {
                if let Some(value) = measurement_result_by_name(measurements, key) {
                    return Some(value);
                }
                if key.eq_ignore_ascii_case("output_noise")
                    || key.eq_ignore_ascii_case("onoise_total")
                {
                    return output_noise.last().copied();
                }
                if key.eq_ignore_ascii_case("input_noise")
                    || key.eq_ignore_ascii_case("inoise_total")
                {
                    return input_noise.as_ref().and_then(|vals| vals.last().copied());
                }
                contributors.get(key).and_then(|vals| vals.last().copied())
            }
            SimulationResult::PoleZero {
                poles, zeros, gain, ..
            } => {
                if key.eq_ignore_ascii_case("gain") {
                    return *gain;
                }
                if key.eq_ignore_ascii_case("num_poles") {
                    return Some(poles.len() as f64);
                }
                if key.eq_ignore_ascii_case("num_zeros") {
                    return Some(zeros.len() as f64);
                }
                None
            }
            // A study of one point answers `<vector>` and
            // `normalized:<vector>`. A swept study answers neither: there is
            // no "the" derivative across a band, and picking one frequency
            // for the reader would be inventing the question.
            SimulationResult::SensitivityStudy { evidence } => {
                if evidence.is_swept() {
                    return None;
                }
                let (name, normalized) = key
                    .strip_prefix("normalized:")
                    .map_or((key, false), |name| (name, true));
                let row = sensitivity_study_row(evidence, name)?;
                let column = if normalized {
                    &row.normalized
                } else {
                    &row.raw
                };
                column.first().copied().and_then(|value| value.value())
            }
            // The core result document's own scalar names, plus a `dcmatch.`
            // spelling: bounding a quoted sigma is why an engineer runs this
            // analysis, so every one of the five answers a specification.
            SimulationResult::DcMismatch { evidence } => dc_mismatch_scalars(evidence)
                .into_iter()
                .find(|(name, _)| {
                    key.eq_ignore_ascii_case(name)
                        || key.eq_ignore_ascii_case(&format!("dcmatch.{name}"))
                })
                .map(|(_, value)| value)
                .filter(|value| value.is_finite()),
            SimulationResult::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => {
                if key.eq_ignore_ascii_case("gain")
                    || key.eq_ignore_ascii_case("transfer_gain")
                    || key.eq_ignore_ascii_case("tf.gain")
                {
                    gain.as_ref().and_then(tf_scalar_finite)
                } else if key.eq_ignore_ascii_case("input_resistance")
                    || key.eq_ignore_ascii_case("rin")
                    || key.eq_ignore_ascii_case("tf.input_resistance")
                {
                    input_resistance.as_ref().and_then(tf_scalar_finite)
                } else if key.eq_ignore_ascii_case("output_resistance")
                    || key.eq_ignore_ascii_case("rout")
                    || key.eq_ignore_ascii_case("tf.output_resistance")
                {
                    output_resistance.as_ref().and_then(tf_scalar_finite)
                } else {
                    None
                }
            }
            SimulationResult::MonteCarlo { variables, .. } => {
                if let Some(var) = variables.iter().find(|var| var.name == key) {
                    return Some(var.mean);
                }

                parse_wrapped_identifier(key, "mean")
                    .and_then(|inner| variables.iter().find(|var| var.name == inner))
                    .map(|var| var.mean)
            }
            SimulationResult::MeasurementsOnly { measurements } => measurements.get(key).copied(),
            // The figures a recorded spectrum reports are analysis-native
            // evidence on its payload, not measurements of a waveform.
            SimulationResult::Fft { .. } => None,
        }
    }

    /// Get all measurements associated with this result
    #[cfg(test)]
    pub fn measurements(&self) -> HashMap<String, f64> {
        match self {
            SimulationResult::DcOp(op) => {
                let mut out = HashMap::with_capacity(
                    op.node_voltages.len().saturating_mul(2) + op.branch_currents.len(),
                );
                for (node, value) in &op.node_voltages {
                    out.insert(node.clone(), *value);
                    out.insert(format!("V({})", node), *value);
                }
                for (branch, value) in &op.branch_currents {
                    out.insert(branch.clone(), *value);
                    if !branch.starts_with("I(") {
                        out.insert(format!("I({})", branch), *value);
                    }
                }
                out
            }
            SimulationResult::DcSweep { waveforms, .. }
            | SimulationResult::Transient { waveforms, .. }
            | SimulationResult::Ac { waveforms, .. }
            | SimulationResult::HarmonicBalance { waveforms, .. }
            | SimulationResult::Parametric { waveforms, .. }
            | SimulationResult::Corner { waveforms, .. }
            | SimulationResult::Reliability { waveforms, .. }
            | SimulationResult::Optimization { waveforms, .. }
            | SimulationResult::Soa { waveforms, .. } => waveforms
                .iter()
                .filter_map(|(name, wf)| {
                    wf.y_values
                        .last()
                        .copied()
                        .map(|value| (name.clone(), value))
                })
                .collect(),
            SimulationResult::Qpss {
                operating_point, ..
            } => HashMap::from([
                (
                    "qpss.iterations".into(),
                    operating_point.iterations() as f64,
                ),
                (
                    "qpss.normalized_residual".into(),
                    operating_point.normalized_residual(),
                ),
            ]),
            SimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                detect_subharmonics,
                modes,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                waveforms,
                ..
            } => {
                let mut values = waveforms
                    .iter()
                    .filter_map(|(name, waveform)| {
                        waveform
                            .y_values
                            .last()
                            .copied()
                            .map(|value| (name.clone(), value))
                    })
                    .collect::<HashMap<_, _>>();
                values.insert("pstb.period".to_owned(), *period);
                values.insert(
                    "pstb.fundamental_frequency".to_owned(),
                    *fundamental_frequency,
                );
                values.insert("pstb.stability_threshold".to_owned(), *stability_threshold);
                values.insert(
                    "pstb.detect_subharmonics".to_owned(),
                    if *detect_subharmonics { 1.0 } else { 0.0 },
                );
                values.insert("pstb.mode_count".to_owned(), modes.len() as f64);
                values.insert("pstb.unstable_mode_count".to_owned(), *num_unstable as f64);
                values.insert(
                    "pstb.max_multiplier_magnitude".to_owned(),
                    *max_multiplier_magnitude,
                );
                if let Some(margin) = min_stability_margin_db {
                    values.insert("pstb.min_stability_margin_db".to_owned(), *margin);
                }
                values
            }
            SimulationResult::Noise {
                output_noise,
                input_noise,
                contributors,
                ..
            } => {
                let mut out = HashMap::new();
                if let Some(v) = output_noise.last().copied() {
                    out.insert("output_noise".to_string(), v);
                    out.insert("onoise_total".to_string(), v);
                }
                if let Some(v) = input_noise.as_ref().and_then(|vals| vals.last().copied()) {
                    out.insert("input_noise".to_string(), v);
                    out.insert("inoise_total".to_string(), v);
                }
                for (name, vals) in contributors {
                    if let Some(v) = vals.last().copied() {
                        out.insert(name.clone(), v);
                    }
                }
                out
            }
            SimulationResult::PoleZero {
                poles, zeros, gain, ..
            } => {
                let mut values = HashMap::from([
                    ("num_poles".to_string(), poles.len() as f64),
                    ("num_zeros".to_string(), zeros.len() as f64),
                ]);
                if let Some(gain) = gain {
                    values.insert("gain".to_string(), *gain);
                }
                values
            }
            SimulationResult::SensitivityStudy { evidence } => {
                if evidence.is_swept() {
                    return HashMap::new();
                }
                evidence
                    .rows
                    .iter()
                    .flat_map(|row| {
                        let raw = row
                            .raw
                            .first()
                            .copied()
                            .and_then(|value| value.value())
                            .map(|value| (row.parameter.clone(), value));
                        let normalized = row
                            .normalized
                            .first()
                            .copied()
                            .and_then(|value| value.value())
                            .map(|value| (format!("normalized:{}", row.parameter), value));
                        [raw, normalized]
                    })
                    .flatten()
                    .collect()
            }
            SimulationResult::DcMismatch { evidence } => dc_mismatch_scalars(evidence)
                .into_iter()
                .filter(|(_, value)| value.is_finite())
                .flat_map(|(name, value)| {
                    [(name.to_owned(), value), (format!("dcmatch.{name}"), value)]
                })
                .collect(),
            SimulationResult::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => {
                let mut out = HashMap::new();
                if let Some(value) = gain.as_ref().and_then(tf_scalar_finite) {
                    out.insert("gain".to_owned(), value);
                    out.insert("tf.gain".to_owned(), value);
                }
                if let Some(value) = input_resistance.as_ref().and_then(tf_scalar_finite) {
                    out.insert("input_resistance".to_owned(), value);
                    out.insert("tf.input_resistance".to_owned(), value);
                }
                if let Some(value) = output_resistance.as_ref().and_then(tf_scalar_finite) {
                    out.insert("output_resistance".to_owned(), value);
                    out.insert("tf.output_resistance".to_owned(), value);
                }
                out
            }
            SimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
                .map(|var| (var.name.clone(), var.mean))
                .collect(),
            SimulationResult::MeasurementsOnly { measurements } => measurements.clone(),
            SimulationResult::Fft { .. } => HashMap::new(),
        }
    }
}

/// The five DC mismatch scalars, under the names the core result document
/// gives them.
///
/// One list, read by both the single-name lookup and the whole map, so a
/// specification that resolves by name and a report that lists what is
/// available cannot offer different sets.
fn dc_mismatch_scalars(evidence: &crate::state::DcMismatchEvidence) -> [(&'static str, f64); 5] {
    [
        ("nominal_value", evidence.nominal_value),
        ("sigma_total", evidence.sigma_total),
        ("sigma_mismatch", evidence.sigma_mismatch),
        ("sigma_process", evidence.sigma_process),
        ("quoted_sigma", evidence.quoted_sigma()),
    ]
}

/// The study row a measurement name asks for.
///
/// The engine's vector name is the answer, matched case-insensitively. A bare
/// name is also accepted for a design parameter: a specification saved when
/// the Studio named its rows `GAIN` still resolves now that the engine names
/// the same quantity `PARAM:GAIN`, and the alias is only consulted when no
/// vector name matched exactly, so it can never shadow a device called
/// `GAIN`.
fn sensitivity_study_row<'a>(
    evidence: &'a crate::state::SensitivityStudyEvidence,
    name: &str,
) -> Option<&'a crate::state::SensitivityStudyRow> {
    evidence
        .rows
        .iter()
        .find(|row| row.parameter.eq_ignore_ascii_case(name))
        .or_else(|| {
            evidence.rows.iter().find(|row| {
                row.parameter
                    .strip_prefix("PARAM:")
                    .is_some_and(|parameter| parameter.eq_ignore_ascii_case(name))
            })
        })
}

fn tf_scalar_finite(value: &TransferFunctionScalar) -> Option<f64> {
    match value {
        TransferFunctionScalar::Finite(value) => Some(*value),
        TransferFunctionScalar::PositiveInfinity | TransferFunctionScalar::NegativeInfinity => None,
    }
}

fn measurement_result_by_name(
    measurements: &[rspice_core::MeasureResult],
    key: &str,
) -> Option<f64> {
    measurements
        .iter()
        .find(|measurement| measurement.name.eq_ignore_ascii_case(key))
        .and_then(|measurement| measurement.passed.then_some(measurement.value).flatten())
        .filter(|value| value.is_finite())
}

fn parse_wrapped_identifier<'a>(key: &'a str, prefix: &str) -> Option<&'a str> {
    if key.len() <= prefix.len() + 2 {
        return None;
    }
    if !key[..prefix.len()].eq_ignore_ascii_case(prefix) {
        return None;
    }
    if !key[prefix.len()..].starts_with('(') || !key.ends_with(')') {
        return None;
    }
    Some(&key[prefix.len() + 1..key.len() - 1])
}

fn measurement_from_dc_op(op: &DcOpResult, key: &str) -> Option<f64> {
    if let Some(v) = op.node_voltages.get(key).copied() {
        return Some(v);
    }
    if let Some(v) = op.branch_currents.get(key).copied() {
        return Some(v);
    }
    if let Some(node) = parse_wrapped_identifier(key, "V") {
        return op.node_voltages.get(node).copied();
    }
    if let Some(branch) = parse_wrapped_identifier(key, "I") {
        return op
            .branch_currents
            .get(branch)
            .copied()
            .or_else(|| op.branch_currents.get(key).copied());
    }
    None
}

fn waveform_last_value_by_name(
    waveforms: &HashMap<String, WaveformData>,
    key: &str,
) -> Option<f64> {
    if let Some(v) = waveforms
        .get(key)
        .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    if let Some(inner) =
        parse_wrapped_identifier(key, "V").or_else(|| parse_wrapped_identifier(key, "I"))
        && let Some(v) = waveforms
            .get(inner)
            .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    let voltage_key = format!("V({})", key);
    if let Some(v) = waveforms
        .get(&voltage_key)
        .and_then(|wf| wf.y_values.last().copied())
    {
        return Some(v);
    }

    let current_key = format!("I({})", key);
    waveforms
        .get(&current_key)
        .and_then(|wf| wf.y_values.last().copied())
}

#[cfg(test)]
mod transfer_function_tests {
    use super::*;
    use crate::simulation::multi_run::{TfAccuracy, TfNormalization};

    fn result(
        gain: Option<TransferFunctionScalar>,
        input_resistance: Option<TransferFunctionScalar>,
        output_resistance: Option<TransferFunctionScalar>,
    ) -> SimulationResult {
        SimulationResult::TransferFunction {
            input_source: "VIN".to_owned(),
            output_expression: "V(out)".to_owned(),
            input_quantity: TransferFunctionQuantity::Voltage,
            output_quantity: TransferFunctionQuantity::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
            gain,
            input_resistance,
            output_resistance,
            nominal_input: None,
            nominal_output: None,
        }
    }

    #[test]
    fn tf_measurement_aliases_resolve_exact_finite_scalars() {
        let tf = result(
            Some(TransferFunctionScalar::Finite(-0.25)),
            Some(TransferFunctionScalar::Finite(3_000.0)),
            Some(TransferFunctionScalar::Finite(750.0)),
        );

        for alias in ["gain", "transfer_gain", "tf.gain", "TF.GAIN"] {
            assert_eq!(tf.measurement(alias), Some(-0.25), "alias {alias}");
        }
        for alias in ["input_resistance", "rin", "tf.input_resistance"] {
            assert_eq!(tf.measurement(alias), Some(3_000.0), "alias {alias}");
        }
        for alias in ["output_resistance", "rout", "tf.output_resistance"] {
            assert_eq!(tf.measurement(alias), Some(750.0), "alias {alias}");
        }
        assert_eq!(tf.measurement("unknown"), None);

        let measurements = tf.measurements();
        assert_eq!(measurements["gain"], -0.25);
        assert_eq!(measurements["tf.gain"], -0.25);
        assert_eq!(measurements["input_resistance"], 3_000.0);
        assert_eq!(measurements["tf.input_resistance"], 3_000.0);
        assert_eq!(measurements["output_resistance"], 750.0);
        assert_eq!(measurements["tf.output_resistance"], 750.0);
    }

    #[test]
    fn infinite_tf_resistance_never_leaks_into_finite_measurement_apis() {
        let tf = result(
            Some(TransferFunctionScalar::Finite(1.0)),
            Some(TransferFunctionScalar::NegativeInfinity),
            Some(TransferFunctionScalar::PositiveInfinity),
        );

        assert_eq!(tf.measurement("gain"), Some(1.0));
        assert_eq!(tf.measurement("rin"), None);
        assert_eq!(tf.measurement("rout"), None);
        let measurements = tf.measurements();
        assert_eq!(measurements["gain"], 1.0);
        assert!(!measurements.contains_key("input_resistance"));
        assert!(!measurements.contains_key("output_resistance"));
    }
    use crate::state::{
        ComplexResultValue, SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow,
    };

    fn study_row(parameter: &str, raw: Vec<f64>, normalized: Vec<f64>) -> SensitivityStudyRow {
        SensitivityStudyRow {
            parameter: parameter.to_owned(),
            nominal_value: 1.0,
            raw: raw.into_iter().map(Into::into).collect(),
            normalized: normalized.into_iter().map(Into::into).collect(),
            phase: Vec::new(),
        }
    }

    #[test]
    fn sensitivity_measurements_preserve_unavailability_and_normalized_identity() {
        use rspice_core::analysis::sensitivity::{SensitivityUnavailability, SensitivityValue};
        let result = SimulationResult::SensitivityStudy {
            evidence: std::sync::Arc::new(SensitivityStudyEvidence {
                output: "V(out)".to_owned(),
                filter: String::new(),
                basis: SensitivityBasisEvidence::Dc { output: 4.0 },
                rows: vec![
                    SensitivityStudyRow {
                        normalized: vec![SensitivityValue::unavailable(
                            SensitivityUnavailability::ZeroOutput,
                        )],
                        ..study_row("GAIN", vec![2.0], vec![0.5])
                    },
                    SensitivityStudyRow {
                        raw: vec![SensitivityValue::unavailable(
                            SensitivityUnavailability::NondifferentiableMagnitude,
                        )],
                        ..study_row("NULL", vec![0.0], vec![0.25])
                    },
                    study_row("ZERO", vec![0.0], vec![0.0]),
                ],
            }),
        };
        assert_eq!(result.measurement("ZERO"), Some(0.0));
        assert_eq!(result.measurement("normalized:GAIN"), None);
        assert_eq!(result.measurement("GAIN"), Some(2.0));
        assert_eq!(result.measurement("normalized:ZERO"), Some(0.0));
        assert_eq!(result.measurement("NULL"), None);
        let values = result.measurements();
        assert_eq!(values.len(), 4);
        for (name, value) in values {
            assert_eq!(result.measurement(&name), Some(value));
        }
    }

    /// A specification saved when the Studio named its rows `GAIN` still
    /// resolves now that the engine names the same quantity `PARAM:GAIN`.
    #[test]
    fn a_bare_parameter_name_still_answers_a_sensitivity_measurement() {
        let result = SimulationResult::SensitivityStudy {
            evidence: std::sync::Arc::new(SensitivityStudyEvidence {
                output: "V(out)".to_owned(),
                filter: "PARAM:*".to_owned(),
                basis: SensitivityBasisEvidence::Dc { output: 4.0 },
                rows: vec![study_row("PARAM:GAIN", vec![2.0], vec![0.5])],
            }),
        };
        assert_eq!(result.measurement("PARAM:GAIN"), Some(2.0));
        assert_eq!(result.measurement("GAIN"), Some(2.0));
        assert_eq!(result.measurement("normalized:GAIN"), Some(0.5));
        assert_eq!(result.measurement("gain"), Some(2.0));
        assert_eq!(result.measurement("SCALE"), None);
        // The map is keyed by the engine's names, never by the alias: two
        // keys for one row would double every listing that reads it.
        let values = result.measurements();
        assert_eq!(values.len(), 2);
        assert!(values.contains_key("PARAM:GAIN"));
        assert!(!values.contains_key("GAIN"));
    }

    /// A swept study answers no scalar measurement: there is no "the"
    /// derivative across a band, and choosing one frequency for the reader
    /// would be inventing the question.
    #[test]
    fn a_swept_sensitivity_answers_no_scalar_measurement() {
        let result = SimulationResult::SensitivityStudy {
            evidence: std::sync::Arc::new(SensitivityStudyEvidence {
                output: "V(out)".to_owned(),
                filter: String::new(),
                basis: SensitivityBasisEvidence::Ac {
                    frequencies_hz: vec![10.0, 100.0],
                    output: vec![
                        ComplexResultValue {
                            real: 1.0,
                            imaginary: 0.0,
                        },
                        ComplexResultValue {
                            real: 0.5,
                            imaginary: 0.0,
                        },
                    ],
                },
                rows: vec![SensitivityStudyRow {
                    phase: vec![0.0.into(), 0.0.into()],
                    ..study_row("R1", vec![2.0, 3.0], vec![0.5, 0.75])
                }],
            }),
        };
        assert_eq!(result.measurement("R1"), None);
        assert_eq!(result.measurement("normalized:R1"), None);
        assert!(result.measurements().is_empty());
    }
}
