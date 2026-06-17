use std::collections::{HashMap, HashSet};

use crate::properties::engineering::parse_engineering_value;

pub(crate) fn changed_lines_since_baseline(
    current: &str,
    baseline: Option<&str>,
) -> HashSet<usize> {
    let Some(baseline) = baseline else {
        return HashSet::new();
    };
    let current_lines: Vec<&str> = current.split('\n').collect();
    let baseline_lines: Vec<&str> = baseline.split('\n').collect();
    let max_len = current_lines.len().max(baseline_lines.len());
    (0..max_len)
        .filter(|&idx| current_lines.get(idx) != baseline_lines.get(idx))
        .collect()
}

pub(crate) fn param_values(buffer: &str) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for line in buffer.lines() {
        if let Some(assignments) = super::tuner::scan_assignments_for_baseline(line) {
            for (name, start, end) in assignments {
                let raw = &line[start..end];
                if raw.starts_with('{') {
                    continue;
                }
                if let Ok(value) = parse_engineering_value(raw) {
                    out.insert(name.to_ascii_lowercase(), value);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_lines_compare_against_snapshot() {
        let baseline = "deck\n.param r=1k\nR1 a 0 {r}\n.op\n.end\n";
        let current = "deck\n.param r=2k\nR1 a 0 {r}\n.op\n.end\n";
        assert_eq!(
            changed_lines_since_baseline(current, Some(baseline)),
            HashSet::from([1])
        );
    }

    #[test]
    fn changed_lines_empty_when_no_baseline() {
        assert!(changed_lines_since_baseline("deck\n.op\n", None).is_empty());
    }

    #[test]
    fn param_values_parse_numeric_assignments_case_insensitively() {
        let values = param_values(".param Itail=20u cl = 2p expr={w*2}\n");
        assert!((values["itail"] - 20e-6).abs() < 1e-15);
        assert!((values["cl"] - 2e-12).abs() < 1e-21);
        assert!(!values.contains_key("expr"));
    }
}
