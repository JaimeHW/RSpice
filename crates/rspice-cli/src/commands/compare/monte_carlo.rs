//! Trial and campaign identities remain exact even with loose numeric tolerances.

use super::{WaveformData, evidence::true_indicator, strip_outer_call};
use crate::commands::report_identity::decode_exact;
use std::collections::HashSet;

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
    problems
}
