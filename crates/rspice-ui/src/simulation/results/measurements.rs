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
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } => sensitivities
                .get(key)
                .copied()
                .or_else(|| normalized.get(key).copied()),
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
            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } => {
                let mut out = sensitivities.clone();
                for (name, value) in normalized {
                    out.insert(format!("normalized:{}", name), *value);
                }
                out
            }
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
        }
    }
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
}
