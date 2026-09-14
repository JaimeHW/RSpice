//! Bounded impulse preparation before the joint acceptance barrier.

use super::*;
use crate::{CurrentImpulsePoint, CurrentImpulseTrace};

enum Addition {
    Append {
        trace: usize,
        point: CurrentImpulsePoint,
    },
    New(CurrentImpulseTrace),
}

/// Exclusive ownership keeps the prepared target unchanged until acceptance.
/// Dropping this value publishes nothing; committing cannot allocate or fail.
#[must_use]
pub(super) struct Prepared<'a> {
    target: &'a mut Vec<CurrentImpulseTrace>,
    additions: Vec<(usize, Addition)>,
    added_values: usize,
}

fn failure(message: impl std::fmt::Display) -> SimulationError {
    SimulationError::Circuit(format!("physical current impulse observation: {message}"))
}

pub(super) fn prepare<'a>(
    result: &'a mut TransientResult,
    time: Value,
    solved_branches: usize,
    impulses: impl Iterator<Item = (usize, Value)>,
    retained_values: usize,
    limits: &crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Prepared<'a>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !time.is_finite() || time < 0.0 || result.time.last().is_none_or(|previous| time < *previous)
    {
        return Err(failure("invalid accepted event time"));
    }
    let target = result
        .current_impulses
        .as_mut()
        .ok_or_else(|| failure("physical history is unavailable"))?;
    let mut additions: Vec<(usize, Addition)> = Vec::new();
    let mut added_values = 0usize;
    let mut new_traces = 0usize;
    for (coordinate, charge_coulombs) in impulses {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let ordinal = coordinate
            .checked_sub(result.num_nodes)
            .ok_or_else(|| failure("impulse coordinate is not a branch"))?;
        if ordinal >= solved_branches {
            return Err(failure("impulse coordinate is not a solved branch"));
        }
        let branch = result
            .branch_names
            .get(ordinal)
            .filter(|name| !name.trim().is_empty())
            .ok_or_else(|| failure("impulse branch has no result identity"))?;
        if !charge_coulombs.is_finite() {
            return Err(failure("nonfinite charge"));
        }
        if charge_coulombs == 0.0 {
            continue;
        }
        if additions.iter().any(|(owner, _)| {
            *owner == ordinal || result.branch_names[*owner].eq_ignore_ascii_case(branch)
        }) {
            return Err(failure("duplicate branch impulse in one accepted event"));
        }
        let existing = target
            .iter()
            .position(|trace| trace.branch_name.eq_ignore_ascii_case(branch));
        if existing.is_some_and(|index| {
            target[index]
                .points
                .last()
                .is_some_and(|point| point.time >= time)
        }) {
            return Err(failure("impulse would replay or precede an accepted event"));
        }
        let cost = 2usize.saturating_add(if existing.is_none() {
            branch.len().div_ceil(std::mem::size_of::<Value>())
        } else {
            0
        });
        added_values = added_values.saturating_add(cost);
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            retained_values.saturating_add(added_values),
            limits.max_result_values,
        )?;
        additions.try_reserve(1).map_err(failure)?;
        let point = CurrentImpulsePoint {
            time,
            charge_coulombs,
        };
        let addition = if let Some(trace) = existing {
            target[trace].points.try_reserve(1).map_err(failure)?;
            Addition::Append { trace, point }
        } else {
            let mut branch_name = String::new();
            branch_name
                .try_reserve_exact(branch.len())
                .map_err(failure)?;
            branch_name.push_str(branch);
            let mut points = Vec::new();
            points.try_reserve_exact(1).map_err(failure)?;
            points.push(point);
            new_traces += 1;
            Addition::New(CurrentImpulseTrace {
                branch_name,
                points,
            })
        };
        additions.push((ordinal, addition));
    }
    target.try_reserve(new_traces).map_err(failure)?;
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(Prepared {
        target,
        additions,
        added_values,
    })
}

impl Prepared<'_> {
    pub(super) fn commit(self) -> usize {
        for (_, addition) in self.additions {
            match addition {
                Addition::Append { trace, point } => self.target[trace].points.push(point),
                Addition::New(trace) => self.target.push(trace),
            }
        }
        self.added_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    fn result() -> TransientResult {
        TransientResult {
            time: vec![0.0],
            step_sizes: vec![0.0],
            num_nodes: 1,
            node_names: vec!["n".into()],
            voltages: vec![vec![0.0]],
            branch_names: vec!["Vdrive".into(), "Rzero".into(), "Cderived".into()],
            branch_currents: vec![vec![0.0]; 3],
            current_impulses: Some(Vec::new()),
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn impulse_preparation_drop_publishes_nothing_and_commit_preserves_smallest_charge() {
        let limits = crate::resource::ResourceLimits::default();
        let mut result = result();
        let pending = prepare(
            &mut result,
            0.0,
            2,
            [(1, -1e-12), (2, 0.0)].into_iter(),
            0,
            &limits,
            &NoAbort,
        )
        .unwrap();
        drop(pending);
        assert_eq!(result.current_impulses, Some(Vec::new()));
        assert_eq!(
            prepare(
                &mut result,
                0.0,
                2,
                [(1, -1e-12)].into_iter(),
                0,
                &limits,
                &NoAbort
            )
            .unwrap()
            .commit(),
            3
        );
        assert_eq!(
            prepare(
                &mut result,
                1.0,
                2,
                [(1, Value::from_bits(1)), (2, 1e-12)].into_iter(),
                3,
                &limits,
                &NoAbort
            )
            .unwrap()
            .commit(),
            5
        );
        let traces = result.current_impulses.unwrap();
        assert_eq!(traces[0].branch_name, "Vdrive");
        assert_eq!(traces[0].points[0].charge_coulombs, -1e-12);
        assert_eq!(traces[0].points[1].charge_coulombs.to_bits(), 1);
        assert_eq!(traces[1].branch_name, "Rzero");
        assert_eq!(traces[1].points[0].charge_coulombs, 1e-12);
    }

    #[test]
    fn impulse_preparation_refuses_budget_cancellation_and_bad_identity_without_publication() {
        let limits = crate::resource::ResourceLimits {
            max_result_values: 10,
            ..Default::default()
        };
        let mut output = result();
        assert!(matches!(
            prepare(
                &mut output,
                0.0,
                2,
                [(1, 1.0)].into_iter(),
                8,
                &limits,
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit(_))
        ));
        assert_eq!(output.current_impulses, Some(Vec::new()));
        for charges in [
            vec![(0, 1.0)],
            vec![(3, 1.0)],
            vec![(1, Value::NAN)],
            vec![(1, 1.0), (1, 2.0)],
        ] {
            assert!(
                prepare(
                    &mut output,
                    0.0,
                    2,
                    charges.into_iter(),
                    0,
                    &limits,
                    &NoAbort
                )
                .is_err()
            );
            assert_eq!(output.current_impulses, Some(Vec::new()));
        }
        let census = CountingAbort::new(usize::MAX);
        drop(
            prepare(
                &mut output,
                0.0,
                2,
                [(1, 1.0), (2, 2.0)].into_iter(),
                0,
                &limits,
                &census,
            )
            .unwrap(),
        );
        for poll in 0..census.count() {
            let abort = CountingAbort::new(poll);
            assert!(matches!(
                prepare(
                    &mut output,
                    0.0,
                    2,
                    [(1, 1.0), (2, 2.0)].into_iter(),
                    0,
                    &limits,
                    &abort
                ),
                Err(SimulationError::Aborted)
            ));
            assert_eq!(output.current_impulses, Some(Vec::new()));
            assert_eq!(abort.polls_after_abort(), 0);
        }
        output.branch_names[1] = "vDRIVE".into();
        assert!(
            prepare(
                &mut output,
                0.0,
                2,
                [(1, 1.0), (2, 2.0)].into_iter(),
                0,
                &limits,
                &NoAbort
            )
            .is_err()
        );
        assert_eq!(output.current_impulses, Some(Vec::new()));
    }

    #[test]
    fn impulse_preparation_rejects_seam_replay_and_keeps_zero_events_sparse() {
        let limits = crate::resource::ResourceLimits::default();
        let mut output = result();
        assert_eq!(
            prepare(
                &mut output,
                0.0,
                2,
                [(1, 0.0), (2, -0.0)].into_iter(),
                0,
                &limits,
                &NoAbort
            )
            .unwrap()
            .commit(),
            0
        );
        assert_eq!(output.current_impulses, Some(Vec::new()));
        prepare(
            &mut output,
            0.0,
            2,
            [(1, 1.0)].into_iter(),
            0,
            &limits,
            &NoAbort,
        )
        .unwrap()
        .commit();
        let before = output.current_impulses.clone();
        for time in [0.0, -1.0, Value::INFINITY, Value::NAN] {
            assert!(
                prepare(
                    &mut output,
                    time,
                    2,
                    [(1, 1.0)].into_iter(),
                    3,
                    &limits,
                    &NoAbort
                )
                .is_err()
            );
            assert_eq!(output.current_impulses, before);
        }
    }
}
