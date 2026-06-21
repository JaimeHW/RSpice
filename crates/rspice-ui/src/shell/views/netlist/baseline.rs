use std::collections::HashSet;

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
}
