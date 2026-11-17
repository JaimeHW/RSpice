//! Diffing a deck against its baseline.
//!
//! Reports which lines changed since the deck the last run executed, for the
//! change gutter in the netlist editor. The diff is the run controller's, so
//! the gutter after an edit and the gutter after a run mark the same lines.

use std::collections::HashSet;

pub(super) fn changed_lines_against_baseline(current: &str, baseline: &str) -> HashSet<usize> {
    crate::simulation::controller::SimulationController::changed_lines_against_baseline(
        current, baseline,
    )
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
    fn inserting_one_line_marks_that_line_alone() {
        let run = "deck\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.tran 1n 1u\n.end\n";
        let edited = "deck\nV1 in 0 1\nR2 in 0 10k\nR1 in out 1k\nC1 out 0 1n\n.tran 1n 1u\n.end\n";
        assert_eq!(
            changed_lines_against_baseline(edited, run),
            HashSet::from([2])
        );
    }

    #[test]
    fn deleting_a_line_marks_the_line_now_in_its_place() {
        let run = "deck\nV1 in 0 1\nR2 in 0 10k\nR1 in out 1k\n.end\n";
        let edited = "deck\nV1 in 0 1\nR1 in out 1k\n.end\n";
        assert_eq!(
            changed_lines_against_baseline(edited, run),
            HashSet::from([2])
        );
    }
}
