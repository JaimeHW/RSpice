use std::collections::HashSet;

#[cfg(test)]
use crate::properties::engineering::parse_engineering_value;
#[cfg(test)]
use std::collections::HashMap;

pub(super) fn changed_lines_against_baseline(current: &str, baseline: &str) -> HashSet<usize> {
    let current_lines: Vec<&str> = current.lines().collect();
    let baseline_lines: Vec<&str> = baseline.lines().collect();
    let max_len = current_lines.len().max(baseline_lines.len());
    let mut changed = HashSet::new();

    for idx in 0..max_len {
        if current_lines.get(idx) != baseline_lines.get(idx) {
            changed.insert(idx);
        }
    }

    changed
}

#[cfg(test)]
fn param_values(buffer: &str) -> HashMap<String, f64> {
    let mut values = HashMap::new();
    for line in buffer.lines() {
        let Some(assignments) = super::tuner::scan_assignments(line) else {
            continue;
        };
        for (name, start, end) in assignments {
            let raw = &line[start..end];
            if raw.starts_with('{') {
                continue;
            }
            if let Ok(value) = parse_engineering_value(raw) {
                values.insert(name.to_ascii_lowercase(), value);
            }
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_lines_marks_replaced_and_added_lines() {
        let changed = changed_lines_against_baseline("a\nB\nc\nd\n", "a\nb\nc\n");

        assert!(changed.contains(&1));
        assert!(changed.contains(&3));
        assert_eq!(changed.len(), 2);
    }

    #[test]
    fn changed_lines_is_empty_for_identical_snapshots() {
        assert!(changed_lines_against_baseline("a\nb\n", "a\nb\n").is_empty());
    }

    #[test]
    fn param_values_parse_numeric_assignments_case_insensitively() {
        let values = param_values(".param Itail=20u cl = 2p expr={w*2}\n");
        assert!((values["itail"] - 20e-6).abs() < 1e-15);
        assert!((values["cl"] - 2e-12).abs() < 1e-21);
        assert!(!values.contains_key("expr"));
    }
}
