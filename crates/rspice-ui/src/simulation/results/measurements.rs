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
            SimulationResult::DcSweep { waveforms, .. }
            | SimulationResult::Transient { waveforms, .. }
            | SimulationResult::Ac { waveforms, .. }
            | SimulationResult::Parametric { waveforms, .. }
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
                ..
            } => {
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
            SimulationResult::PoleZero { poles, zeros, gain } => {
                if key.eq_ignore_ascii_case("gain") {
                    return Some(*gain);
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
            SimulationResult::PoleZero { poles, zeros, gain } => HashMap::from([
                ("gain".to_string(), *gain),
                ("num_poles".to_string(), poles.len() as f64),
                ("num_zeros".to_string(), zeros.len() as f64),
            ]),
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
            SimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
                .map(|var| (var.name.clone(), var.mean))
                .collect(),
            SimulationResult::MeasurementsOnly { measurements } => measurements.clone(),
        }
    }
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
