//! Measurements over a result.
//!
//! Derived scalars — rise time, overshoot, bandwidth, gain margin — computed
//! from the stored traces rather than requested from the engine, so a
//! measurement can be added without re-running.

use super::*;

impl SimulationResult {
    /// Resolve a study request without falling back from a failed `.MEAS` to
    /// a same-named signal. Waveform reduction must be explicitly requested.
    pub(crate) fn study_measurement(
        &self,
        request: &str,
    ) -> Option<crate::state::FamilyMeasurementEvidence> {
        let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
        if mode.eq_ignore_ascii_case("meas") {
            let measurements = match self {
                Self::DcSweep { measurements, .. }
                | Self::Transient { measurements, .. }
                | Self::Ac { measurements, .. }
                | Self::Noise { measurements, .. }
                | Self::HarmonicBalance { measurements, .. } => measurements,
                _ => return None,
            };
            let measurement = measurements
                .iter()
                .find(|measurement| measurement.name.eq_ignore_ascii_case(key))?;
            // A missed GOAL is still an observed sample. Discarding it would
            // bias the distribution toward passing trials. Evaluation failures
            // have no raw value, even when an output default is configured.
            measurement.raw_value.filter(|value| value.is_finite())?;
            return Some(crate::state::FamilyMeasurementEvidence {
                name: request.to_owned(),
                value: Some(measurement.value.filter(|value| value.is_finite())?),
                passed: measurement.passed,
                error: measurement.error.clone(),
            });
        }
        let value = if mode.eq_ignore_ascii_case("bin") {
            let (index, quantity, signal) = parse_study_bin(key).ok()?;
            let parts = match self {
                Self::Fft { spectrum, .. }
                    if spectrum.evidence.status.is_complete() && signal.is_none() =>
                {
                    Some((*spectrum.real.get(index)?, *spectrum.imaginary.get(index)?))
                }
                Self::Transient {
                    periodic_state: Some(point),
                    ..
                } => {
                    if index > point.config().num_harmonics {
                        return None;
                    }
                    let result = &point.analysis().result;
                    let signal = signal?;
                    let (name, names, waves) =
                        if let Some(name) = parse_wrapped_identifier(signal, "V") {
                            (name, &result.node_names, &result.waveforms)
                        } else {
                            (
                                parse_wrapped_identifier(signal, "I")?,
                                &result.branch_names,
                                &result.branch_waveforms,
                            )
                        };
                    let waveform = &waves[names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))?];
                    let harmonics = waveform.compute_harmonics(
                        &result.time,
                        1.0 / point.analysis().period,
                        index,
                    );
                    let harmonic = harmonics.get(index)?;
                    let phase = harmonic.phase.to_radians();
                    Some((
                        harmonic.magnitude * phase.cos(),
                        harmonic.magnitude * phase.sin(),
                    ))
                }
                Self::Ac { waveforms, .. }
                | Self::HarmonicBalance { waveforms, .. }
                | Self::Pstb { waveforms, .. } => {
                    let waveform = if let Some(signal) = signal {
                        named_value(waveforms, signal)?
                    } else {
                        let mut choices = waveforms.values().filter(|waveform| waveform.is_complex);
                        let first = choices.next()?;
                        if choices.next().is_some() {
                            return None;
                        }
                        first
                    };
                    Some((
                        *waveform.y_values.get(index)?,
                        if waveform.is_complex {
                            *waveform.y_imag.as_ref()?.get(index)?
                        } else if quantity.eq_ignore_ascii_case("real") {
                            0.0
                        } else {
                            return None;
                        },
                    ))
                }
                Self::Noise { .. } if quantity.eq_ignore_ascii_case("real") => {
                    Some((*self.noise_study_series(signal?)?.get(index)?, 0.0))
                }
                _ => None,
            };
            parts.map(
                |(real, imaginary)| match quantity.to_ascii_lowercase().as_str() {
                    "real" => real,
                    "imag" => imaginary,
                    "magnitude" => real.hypot(imaginary),
                    "phase" => imaginary.atan2(real).to_degrees(),
                    _ => unreachable!(),
                },
            )
        } else if mode.eq_ignore_ascii_case("scalar") {
            match self {
                Self::DcOp(_)
                | Self::PoleZero { .. }
                | Self::SensitivityStudy { .. }
                | Self::TransferFunction { .. }
                | Self::DcMismatch { .. }
                | Self::Pstb { .. } => self.measurement(key),
                Self::Fft { .. } => self.measurement(key),
                Self::Transient {
                    periodic_state: Some(point),
                    ..
                } => {
                    if key.eq_ignore_ascii_case("pss.period") {
                        Some(point.analysis().period)
                    } else if key.eq_ignore_ascii_case("pss.frequency") {
                        Some(1.0 / point.analysis().period)
                    } else if key.eq_ignore_ascii_case("pss.iterations") {
                        Some(point.analysis().iterations as f64)
                    } else {
                        None
                    }
                }

                Self::Noise {
                    summary: Some(summary),
                    ..
                } => {
                    if key.eq_ignore_ascii_case("noise.output_rms") {
                        summary.total_rms
                    } else if key.eq_ignore_ascii_case("noise.input_rms") {
                        summary.input_rms
                    } else {
                        None
                    }
                }
                Self::Ac { waveforms, .. } => named_value(waveforms, key)
                    .filter(|waveform| !waveform.is_complex && waveform.y_values.len() == 1)
                    .and_then(|waveform| waveform.y_values.first().copied()),
                _ => None,
            }
        } else if mode.eq_ignore_ascii_case("last") {
            match self {
                Self::DcSweep { waveforms, .. }
                | Self::Transient { waveforms, .. }
                | Self::Ac { waveforms, .. }
                | Self::HarmonicBalance { waveforms, .. }
                | Self::Pstb { waveforms, .. } => waveform_last_value_by_name(waveforms, key),
                Self::Noise { .. } => self.noise_study_series(key)?.last().copied(),
                _ => None,
            }
        } else {
            None
        };
        value.filter(|value| value.is_finite()).map(|value| {
            crate::state::FamilyMeasurementEvidence {
                name: request.to_owned(),
                value: Some(value),
                passed: true,
                error: None,
            }
        })
    }

    fn noise_study_series(&self, name: &str) -> Option<&[f64]> {
        let Self::Noise {
            output_noise,
            input_noise,
            contributors,
            summary,
            ..
        } = self
        else {
            return None;
        };
        if name.eq_ignore_ascii_case("output_noise") {
            Some(output_noise)
        } else if name.eq_ignore_ascii_case("input_noise") {
            input_noise.as_deref()
        } else if name.eq_ignore_ascii_case("noise_figure_db") {
            summary
                .as_ref()?
                .noise_figure
                .as_ref()
                .map(|figure| figure.decibels.as_slice())
        } else {
            named_value(contributors, name).map(Vec::as_slice)
        }
    }

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
            SimulationResult::Qpac { waveforms, .. }
            | SimulationResult::Qpxf { waveforms, .. }
            | SimulationResult::Qpnoise { waveforms, .. } => {
                waveform_last_value_by_name(waveforms, key)
            }
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
            | SimulationResult::ReliabilityMission { waveforms, .. }
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
            SimulationResult::Fft { spectrum, .. } => {
                if !spectrum.evidence.status.is_complete() {
                    return None;
                }
                let key = key.to_ascii_lowercase();
                if key == "fft.dc" {
                    return spectrum.real.first().copied();
                }
                let metrics = spectrum.evidence.metrics.as_ref()?;
                match key.as_str() {
                    "fft.fundamental_magnitude" => Some(metrics.fundamental_magnitude),
                    "fft.thd_ratio" => Some(metrics.thd_ratio),
                    "fft.thd_db" => Some(metrics.thd_db),
                    "fft.sndr_db" => Some(metrics.sndr_db),
                    "fft.enob_bits" => Some(metrics.enob_bits),
                    "fft.snr_db" => Some(metrics.snr_db),
                    "fft.sfdr_db" => Some(metrics.sfdr_db),
                    "fft.sfdr_spur_frequency_hz" => metrics.sfdr_spur_frequency_hz,
                    _ => None,
                }
                .filter(|value| value.is_finite())
            }
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
            | SimulationResult::Qpac { waveforms, .. }
            | SimulationResult::Qpxf { waveforms, .. }
            | SimulationResult::Qpnoise { waveforms, .. }
            | SimulationResult::HarmonicBalance { waveforms, .. }
            | SimulationResult::Parametric { waveforms, .. }
            | SimulationResult::Corner { waveforms, .. }
            | SimulationResult::ReliabilityMission { waveforms, .. }
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
    if !key.get(..prefix.len())?.eq_ignore_ascii_case(prefix) {
        return None;
    }
    if !key[prefix.len()..].starts_with('(') || !key.ends_with(')') {
        return None;
    }
    Some(&key[prefix.len() + 1..key.len() - 1])
}

/// SPICE signal identities are case insensitive; an exact stored spelling
/// still takes precedence for result maps supplied by external readers.
fn named_value<'a, T>(values: &'a HashMap<String, T>, key: &str) -> Option<&'a T> {
    values.get(key).or_else(|| {
        values
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value)
    })
}

fn measurement_from_dc_op(op: &DcOpResult, key: &str) -> Option<f64> {
    if let Some(value) =
        named_value(&op.node_voltages, key).or_else(|| named_value(&op.branch_currents, key))
    {
        return Some(*value);
    }
    if let Some(node) = parse_wrapped_identifier(key, "V") {
        return named_value(&op.node_voltages, node).copied();
    }
    if let Some(branch) = parse_wrapped_identifier(key, "I") {
        return named_value(&op.branch_currents, branch)
            .or_else(|| named_value(&op.branch_currents, key))
            .copied();
    }
    None
}

fn waveform_last_value_by_name(
    waveforms: &HashMap<String, WaveformData>,
    key: &str,
) -> Option<f64> {
    let last = |name: &str| {
        named_value(waveforms, name)
            .and_then(|waveform| waveform.y_values.last())
            .copied()
    };
    last(key)
        .or_else(|| {
            parse_wrapped_identifier(key, "V")
                .or_else(|| parse_wrapped_identifier(key, "I"))
                .and_then(last)
        })
        .or_else(|| last(&format!("V({key})")))
        .or_else(|| last(&format!("I({key})")))
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

/// Explicit zero-based retained spectral bin, with no implicit complex reduction.
pub(crate) fn parse_study_bin(key: &str) -> Result<(usize, &str, Option<&str>), String> {
    let mut parts = key.splitn(3, ':');
    let index = parts
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .map_err(|_| "Spectral bin index must be a nonnegative integer")?;
    let quantity = parts.next().unwrap_or_default();
    if !["real", "imag", "magnitude", "phase"]
        .iter()
        .any(|name| quantity.eq_ignore_ascii_case(name))
    {
        return Err("Spectral quantity must be real, imag, magnitude, or phase (degrees)".into());
    }
    let signal = parts.next();
    if signal.is_some_and(|name| name.trim().is_empty()) {
        return Err("Spectral signal name is empty".into());
    }
    Ok((index, quantity, signal))
}
