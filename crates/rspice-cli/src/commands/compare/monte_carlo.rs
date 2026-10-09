//! Trial and campaign identities remain exact even with loose numeric tolerances.

use super::{WaveformData, evidence::true_indicator, strip_outer_call};
use crate::commands::report_identity::decode_exact;
use std::collections::{HashMap, HashSet};

pub(super) fn is_population(data: &WaveformData) -> bool {
    data.variables.iter().any(|name| {
        name.to_ascii_lowercase()
            .starts_with("mc:identity:trial_coordinates(")
    }) || data
        .variables
        .first()
        .is_some_and(|name| name.eq_ignore_ascii_case("trial_index"))
}

pub(super) fn problems(data: &WaveformData) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut problems = Vec::new();
    let columns: HashMap<_, _> = data
        .variables
        .iter()
        .enumerate()
        .map(|(column, name)| (name.trim().to_ascii_lowercase(), column))
        .collect();
    let mut exact_values = HashMap::new();
    if is_population(data) {
        let coordinates = &data.values[0];
        if coordinates.is_empty()
            || (0..coordinates.len()).any(|row| {
                data.sample(0, row)
                    .is_none_or(|value| value < 0.0 || value.fract() != 0.0)
            })
            || coordinates.windows(2).any(|pair| pair[0] >= pair[1])
        {
            problems.push(
                "Monte Carlo trial coordinates must be increasing nonnegative integers".into(),
            );
        }
    }
    for (column, name) in data.variables.iter().enumerate() {
        let component = strip_outer_call(name, "Re").or_else(|| strip_outer_call(name, "Im"));
        let normalized = component.unwrap_or(name).trim().to_ascii_lowercase();
        let Some(claim) = normalized.strip_prefix("mc:identity:") else {
            continue;
        };
        let identity = claim.split_once('(').and_then(|(kind, body)| {
            let body = body.strip_suffix(')')?;
            let fields: Vec<_> = body.split(',').collect();
            match (kind, fields.as_slice()) {
                ("trial_coordinates", ["recorded" | "unknown" | "empty" | "unretained"]) => {
                    Some("trial_coordinates".into())
                }
                ("variable", [name]) => {
                    let name = decode_exact(name)?;
                    (!name.trim().is_empty()).then(|| format!("variable:{name}"))
                }
                ("unit", [name, value]) => {
                    let name = decode_exact(name)?;
                    decode_exact(value)?;
                    (!name.trim().is_empty()).then(|| format!("unit:{}", name.to_ascii_lowercase()))
                }
                ("text" | "integer" | "count" | "boolean", [name, value]) => {
                    let name = decode_exact(name)?;
                    if name.trim().is_empty() {
                        return None;
                    }
                    let valid = match kind {
                        "text" => decode_exact(value).is_some(),
                        "integer" => value
                            .parse::<i64>()
                            .is_ok_and(|number| number.to_string() == *value),
                        "count" => value
                            .parse::<u64>()
                            .is_ok_and(|number| number.to_string() == *value),
                        "boolean" => matches!(*value, "true" | "false"),
                        _ => false,
                    };
                    if valid && kind != "text" {
                        let value = if kind == "boolean" {
                            i128::from(*value == "true")
                        } else {
                            value.parse::<i128>().ok()?
                        };
                        let kind = match kind {
                            "count" => "count",
                            "integer" => "integer",
                            _ => "boolean",
                        };
                        exact_values.insert(name.to_ascii_lowercase(), (kind, value));
                    }
                    valid.then(|| format!("scalar:{}", name.to_ascii_lowercase()))
                }
                _ => None,
            }
        });
        if component.is_some()
            || !true_indicator(data, column)
            || identity.is_none_or(|identity| !seen.insert(identity))
        {
            problems.push(format!(
                "'{name}': invalid or conflicting Monte Carlo identity"
            ));
        } else if matches!(
            claim,
            "trial_coordinates(unretained)" | "trial_coordinates(empty)"
        ) {
            problems.push("Monte Carlo report has no retained successful trial population".into());
        } else if (claim == "trial_coordinates(recorded)"
            && !data.variables[0].eq_ignore_ascii_case("trial_index"))
            || (claim == "trial_coordinates(unknown)"
                && !data.variables[0].eq_ignore_ascii_case("sample_index"))
        {
            problems.push("Monte Carlo trial provenance disagrees with the coordinate axis".into());
        }
    }
    for (name, (_, exact)) in &exact_values {
        if columns.contains_key(&format!("re({name})"))
            || columns.contains_key(&format!("im({name})"))
        {
            problems.push(format!(
                "'{name}': integer and boolean Monte Carlo metadata cannot have complex columns"
            ));
        }
        if let Some(&column) = columns.get(name) {
            let numeric = *exact as f64;
            if numeric as i128 != *exact
                || (0..data.values[column].len())
                    .any(|row| data.sample(column, row) != Some(numeric))
            {
                problems.push(format!(
                    "'{name}': numeric values contradict the exact Monte Carlo metadata"
                ));
            }
        }
    }
    let count = |name: &str| {
        exact_values
            .get(name)
            .and_then(|&(kind, value)| (kind == "count").then_some(value))
    };
    let completed = count("completed_runs");
    let failed = count("failed_runs");
    let successful = count("successful_runs");
    if completed
        .zip(failed)
        .is_some_and(|(all, failed)| failed > all)
        || completed
            .zip(successful)
            .is_some_and(|(all, successful)| successful > all)
        || completed
            .zip(failed)
            .zip(successful)
            .is_some_and(|((all, failed), successful)| all - failed != successful)
        || (is_population(data)
            && !data.variables[0].eq_ignore_ascii_case("report")
            && successful.is_some_and(|successful| successful != data.values[0].len() as i128))
    {
        problems.push("Monte Carlo run counters contradict the retained population".into());
    }
    if let Some(failed) = failed {
        for (name, expected) in [
            ("all_converged", failed == 0),
            ("mean_confidence_conditional_on_success", failed != 0),
        ] {
            if exact_values
                .get(name)
                .is_some_and(|&(kind, value)| kind != "boolean" || value != i128::from(expected))
            {
                problems.push(format!(
                    "'{name}': Monte Carlo declaration contradicts the failed run count"
                ));
            }
        }
    }
    problems
}
