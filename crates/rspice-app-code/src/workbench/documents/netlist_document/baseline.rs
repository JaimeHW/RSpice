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

    /// The marks of a line diff over the lines between the whole lines,
    /// breaks and all, the two share at their start and at their end: what
    /// passing over those lines as bytes must mark.
    fn trimmed_diff(current: &str, baseline: &str) -> HashSet<usize> {
        let current_lines = current.split_inclusive('\n').collect::<Vec<_>>();
        let baseline_lines = baseline.split_inclusive('\n').collect::<Vec<_>>();
        let head = current_lines
            .iter()
            .zip(&baseline_lines)
            .take_while(|(current, baseline)| current == baseline)
            .count();
        let tail = current_lines[head..]
            .iter()
            .rev()
            .zip(baseline_lines[head..].iter().rev())
            .take_while(|(current, baseline)| current == baseline)
            .count();
        fn text(line: &str) -> &str {
            line.strip_suffix('\n')
                .map_or(line, |line| line.strip_suffix('\r').unwrap_or(line))
        }
        let current_middle = current_lines[head..current_lines.len() - tail]
            .iter()
            .copied()
            .map(text)
            .collect::<Vec<_>>();
        let baseline_middle = baseline_lines[head..baseline_lines.len() - tail]
            .iter()
            .copied()
            .map(text)
            .collect::<Vec<_>>();
        let last = current_lines.len().checked_sub(1);
        let mut changed = HashSet::new();
        for op in similar::capture_diff_slices(
            similar::Algorithm::Myers,
            &baseline_middle,
            &current_middle,
        ) {
            match op {
                similar::DiffOp::Equal { .. } => {}
                similar::DiffOp::Delete { new_index, .. } => {
                    changed.extend(last.map(|last| (head + new_index).min(last)));
                }
                similar::DiffOp::Insert {
                    new_index, new_len, ..
                }
                | similar::DiffOp::Replace {
                    new_index, new_len, ..
                } => changed.extend(head + new_index..head + new_index + new_len),
            }
        }
        changed
    }

    /// Lines repeated next to an edit, a last line with no break, CRLF
    /// against LF, an edit at either end and an emptied deck: each marks
    /// what a diff of the lines between the shared ones marks.
    #[test]
    fn passing_over_the_shared_lines_marks_what_a_diff_of_the_lines_between_marks() {
        let decks = [
            "",
            "a",
            "a\n",
            "a\nb\n",
            "a\nb",
            "a\na\n",
            "a\na\na\n",
            "x\na\n",
            "a\nb\nb\nc\n",
            "a\nb\nc\n",
            "a\nXb\n",
            "a\r\nb\n",
            "a\nbc\n",
            "deck\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.tran 1n 1u\n.end\n",
            "deck\nV1 in 0 1\nR2 in 0 10k\nR1 in out 1k\nC1 out 0 1n\n.tran 1n 1u\n.end\n",
            "deck\nV1 in 0 1\nR1 in out 1k\n.end\n",
            "deck\nV1 in 0 1\nR1 in out 2k\nC1 out 0 1n\n.tran 1n 1u\n.end",
            "* deck\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.tran 1n 1u\n.end\n",
        ];
        for current in decks {
            for baseline in decks {
                assert_eq!(
                    changed_lines_against_baseline(current, baseline),
                    trimmed_diff(current, baseline),
                    "{current:?} against {baseline:?}"
                );
            }
        }
        // A deck longer than a block of the comparison, edited inside it.
        let long = (0..2000)
            .map(|at| format!("R{at} n{at} 0 1k\n"))
            .collect::<String>();
        for edited in [
            long.replacen("R1000 ", "R1000x ", 1),
            long.replacen("R10 n10 0 1k\n", "", 1),
            format!("* first\n{long}"),
            format!("{long}.end\n"),
            long.replace("R1999 n1999 0 1k\n", ""),
        ] {
            assert_eq!(
                changed_lines_against_baseline(&edited, &long),
                trimmed_diff(&edited, &long)
            );
            assert_eq!(
                changed_lines_against_baseline(&long, &edited),
                trimmed_diff(&long, &edited)
            );
        }
    }

    /// A line edited into a copy of the line after it is the one marked,
    /// not the copy.
    #[test]
    fn an_edit_among_repeated_lines_marks_the_line_edited() {
        assert_eq!(
            changed_lines_against_baseline("a\na\n", "x\na\n"),
            HashSet::from([0])
        );
        assert_eq!(
            changed_lines_against_baseline("a\na\na\n", "a\nx\na\n"),
            HashSet::from([1])
        );
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
