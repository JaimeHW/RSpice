//! Validate categorical analysis claims independently of numeric tolerances.

use super::WaveformData;

pub(super) fn dimensionless(data: &WaveformData, column: usize) -> bool {
    data.units[column].as_deref().is_none_or(|unit| unit == "1")
        && super::quantity_type(&data.variable_types[column])
            .as_deref()
            .is_none_or(|unit| unit == "1")
}

pub(super) fn true_indicator(data: &WaveformData, column: usize) -> bool {
    !data.values[column].is_empty()
        && dimensionless(data, column)
        && (0..data.values[column].len()).all(|row| data.sample(column, row) == Some(1.0))
}

pub(super) fn problems(data: &WaveformData) -> Vec<String> {
    let root_evidence: &[&str] = &[
        "not_requested",
        "legacy_unknown",
        "qualified_empty",
        "qualified",
        "approximate",
    ];
    let fields: [(&str, &[&str]); 9] = [
        ("stb:completed", &["true", "false"]),
        (
            "stb:circuit_stability",
            &["stable", "unstable", "indeterminate"],
        ),
        (
            "stb:circuit_poles",
            &[
                "not_computed",
                "unsupported",
                "numerical_failure",
                "resource_limit",
                "qualified",
                "approximate",
            ],
        ),
        ("pz:input", &[]),
        ("pz:output", &[]),
        ("pz:poles_evidence", root_evidence),
        ("pz:zeros_evidence", root_evidence),
        (
            "pz:poles_asymptotically_stable",
            &["true", "false", "unknown"],
        ),
        (
            "pz:zeros_asymptotically_stable",
            &["true", "false", "unknown"],
        ),
    ];
    let mut seen = [false; 9];
    let mut problems = Vec::new();
    for (column, name) in data.variables.iter().enumerate() {
        let name = name.trim();
        let component =
            super::strip_outer_call(name, "Re").or_else(|| super::strip_outer_call(name, "Im"));
        let claim = component.unwrap_or(name);
        if !claim
            .get(..4)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("stb:"))
            && !claim
                .get(..3)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("pz:"))
        {
            continue;
        }
        let normalized = claim.to_ascii_lowercase();
        let (field, state) = normalized
            .split_once('(')
            .map_or((normalized.as_str(), None), |(field, state)| {
                (field, state.strip_suffix(')'))
            });
        let Some((index, (_, states))) = fields
            .iter()
            .enumerate()
            .find(|(_, (key, _))| *key == field)
        else {
            continue;
        };
        let valid = component.is_none()
            && !seen[index]
            && state.is_some_and(|state| {
                if states.is_empty() {
                    !state.trim().is_empty()
                } else {
                    states.contains(&state)
                }
            })
            && true_indicator(data, column);
        seen[index] = true;
        if !valid {
            let family = if field.starts_with("stb:") {
                "STB"
            } else {
                "PZ"
            };
            problems.push(format!(
                "'{name}': invalid or conflicting {family} evidence indicator"
            ));
        } else if field == "stb:completed" && state == Some("false") {
            problems.push("STB loop-gain analysis did not complete".into());
        }
    }
    problems.extend(super::dc_match::problems(data));
    problems.extend(super::monte_carlo::problems(data));
    problems
}
