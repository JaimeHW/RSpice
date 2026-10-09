//! Validate categorical STB claims independently of numeric tolerances.

use super::WaveformData;

pub(super) fn problems(data: &WaveformData) -> Vec<String> {
    let fields: [(&str, &[&str]); 3] = [
        ("completed", &["true", "false"]),
        (
            "circuit_stability",
            &["stable", "unstable", "indeterminate"],
        ),
        (
            "circuit_poles",
            &[
                "not_computed",
                "unsupported",
                "numerical_failure",
                "resource_limit",
                "qualified",
                "approximate",
            ],
        ),
    ];
    let mut seen = [false; 3];
    let mut problems = Vec::new();
    for (column, name) in data.variables.iter().enumerate() {
        if !name
            .trim()
            .get(..4)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("stb:"))
        {
            continue;
        }
        let normalized = name.trim().to_ascii_lowercase();
        let Some((field, state)) = normalized[4..].split_once('(') else {
            continue;
        };
        let Some((index, (_, states))) = fields
            .iter()
            .enumerate()
            .find(|(_, (key, _))| *key == field)
        else {
            continue;
        };
        let state = state.strip_suffix(')');
        let valid = !seen[index]
            && state.is_some_and(|state| states.contains(&state))
            && !data.values[column].is_empty()
            && data.units[column].as_deref().is_none_or(|unit| unit == "1")
            && super::quantity_type(&data.variable_types[column])
                .as_deref()
                .is_none_or(|unit| unit == "1")
            && (0..data.values[column].len()).all(|row| data.sample(column, row) == Some(1.0));
        seen[index] = true;
        if !valid {
            problems.push(format!(
                "'{name}': invalid or conflicting STB evidence indicator"
            ));
        } else if field == "completed" && state == Some("false") {
            problems.push("STB loop-gain analysis did not complete".into());
        }
    }
    problems
}
