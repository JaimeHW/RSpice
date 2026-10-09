//! Validate sensitivity identities and the reasons for missing derivative samples.

use super::{WaveformData, parse_variable_name, strip_outer_call};
use std::collections::{HashMap, HashSet};

pub(super) struct Evidence {
    statuses: Vec<Option<usize>>,
    markers: HashSet<usize>,
    pub problems: Vec<String>,
}

impl Evidence {
    pub fn new(data: &WaveformData) -> Self {
        let mut result = Self {
            statuses: Vec::new(),
            markers: HashSet::new(),
            problems: Vec::new(),
        };
        if !data.variables.iter().any(|name| {
            let name = name.to_ascii_lowercase();
            name.contains("sens:") || name.contains(":sensitivity_status")
        }) {
            return result;
        }
        result.statuses.resize(data.variables.len(), None);
        let indices: HashMap<_, _> = data
            .variables
            .iter()
            .enumerate()
            .map(|(index, name)| (parse_variable_name(name).key, index))
            .collect();
        let mut identities = HashSet::new();
        for (column, name) in data.variables.iter().enumerate() {
            let component = strip_outer_call(name, "Re").or_else(|| strip_outer_call(name, "Im"));
            let normalized = component.unwrap_or(name).trim().to_ascii_lowercase();
            if let Some(quantity) = normalized.strip_suffix(":sensitivity_status") {
                result.markers.insert(column);
                let metric = indices.get(&parse_variable_name(quantity).key).copied();
                let real = indices
                    .get(&parse_variable_name(&format!("Re({quantity})")).key)
                    .copied();
                let imag = indices
                    .get(&parse_variable_name(&format!("Im({quantity})")).key)
                    .copied();
                let metrics = match (metric, real, imag) {
                    (Some(metric), None, None) => vec![metric],
                    (None, Some(real), Some(imag)) => vec![real, imag],
                    _ => Vec::new(),
                };
                let valid = component.is_none()
                    && !metrics.is_empty()
                    && !data.values[column].is_empty()
                    && super::evidence::dimensionless(data, column)
                    && metrics
                        .iter()
                        .all(|&metric| result.statuses[metric].is_none())
                    && (0..data.values[column].len()).all(|row| {
                        let Some(code) = data.sample(column, row) else {
                            return false;
                        };
                        matches!(code, 0.0 | 1.0 | 2.0 | 3.0)
                            && metrics
                                .iter()
                                .all(|&metric| data.sample(metric, row).is_some() == (code == 0.0))
                    });
                if valid {
                    for metric in metrics {
                        result.statuses[metric] = Some(column);
                    }
                } else {
                    result.problems.push(format!(
                        "'{name}': invalid sensitivity availability evidence"
                    ));
                }
            } else if normalized.starts_with("sens:") {
                let identity = normalized
                    .strip_prefix("sens:output(")
                    .and_then(|value| value.strip_suffix(')'))
                    .filter(|value| !value.trim().is_empty())
                    .map(|_| "output".to_string())
                    .or_else(|| {
                        let state = normalized
                            .strip_prefix("sens:parameter(")?
                            .strip_suffix(')')?;
                        let fields = state
                            .split(',')
                            .map(decode_part)
                            .collect::<Option<Vec<_>>>()?;
                        let [vector, kind, element, _parameter] = fields.as_slice() else {
                            return None;
                        };
                        if vector.trim().is_empty()
                            || element.trim().is_empty()
                            || serde_json::from_value::<
                                rspice_core::execution::result_document::SensitivityElementTag,
                            >(serde_json::Value::String(
                                kind.clone(),
                            ))
                            .is_err()
                        {
                            return None;
                        }
                        Some(format!("parameter:{vector}"))
                    });
                if component.is_some()
                    || !super::evidence::true_indicator(data, column)
                    || identity.is_none_or(|identity| !identities.insert(identity))
                {
                    result.problems.push(format!(
                        "'{name}': invalid or conflicting sensitivity identity"
                    ));
                }
            }
        }
        result
    }

    pub fn is_status(&self, column: usize) -> bool {
        self.markers.contains(&column)
    }

    pub fn reason(&self, data: &WaveformData, column: usize, row: usize) -> Option<u8> {
        let status = self.statuses.get(column).copied().flatten()?;
        let code = data.sample(status, row)?;
        (code != 0.0).then_some(code as u8)
    }
}

fn decode_part(value: &str) -> Option<String> {
    let mut bytes = value.bytes();
    let mut decoded = Vec::with_capacity(value.len());
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            let high = char::from(bytes.next()?).to_digit(16)?;
            let low = char::from(bytes.next()?).to_digit(16)?;
            decoded.push((high * 16 + low) as u8);
        } else if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.') {
            decoded.push(byte);
        } else {
            return None;
        }
    }
    String::from_utf8(decoded).ok()
}
