//! Monte Carlo statistics: the yield judge's numbers and the gate's count.

use super::*;
use crate::population::wilson_interval;
use crate::specification::MonteCarloSpecificationGate;
use crate::verification::{Acceptance, JointYield};
use crate::yield_analysis::judge_monte_carlo;

/// The statistics a row states are the yield judge's, number for number:
/// the trial that did not converge is a failing trial with no value.
#[test]
fn yield_statistics_are_the_yield_judges() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let trials = monte_carlo(
        None,
        4_000,
        &[(
            "gain",
            &[Some(42.0), Some(39.0), None, Some(45.0), Some(41.0)],
        )],
    );
    let judged = judge_monte_carlo(&trials, &specifications);
    let run = history(vec![trials]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    let statistics = sheet.rows[0]
        .statistics
        .as_ref()
        .expect("the row has trials");
    let judged = &judged[0];
    assert_eq!(
        format!("{:?}", statistics.distribution),
        format!("{:?}", judged.stats)
    );
    assert_eq!(statistics.samples(), judged.samples);
    let trail = statistics
        .trials
        .iter()
        .map(|trial| trial.passing)
        .collect::<Vec<_>>();
    assert_eq!(trail, judged.trail);
    let count = statistics.yield_count.expect("a limited row has a yield");
    assert_eq!(
        (count.passing, count.total),
        (judged.pass_count, judged.total_runs)
    );
    assert_eq!(count.percent(), judged.yield_percent);
    assert_eq!(count.interval, wilson_interval(3, 5));
    let failing = statistics
        .failing_trials()
        .map(|trial| (trial.index(), trial.seed(), trial.value))
        .collect::<Vec<_>>();
    assert_eq!(
        failing,
        [
            (Some(1), Some(4_001), Some(39.0)),
            (Some(2), Some(4_002), None)
        ]
    );
    assert!(
        statistics.per_point.is_empty(),
        "the trials ran at one point"
    );
}

/// A row's yield is the count the acceptance gate divides: the gate blocks
/// on it exactly when the row's percentage is under the gate's.
#[test]
fn a_rows_yield_is_the_count_the_gate_divides() {
    let gain = entry("gain", Some(40.0), None, "dB");
    let specifications = vec![governed(&gain, |_| {})];
    let run = history(vec![monte_carlo(
        None,
        11,
        &[(
            "gain",
            &[Some(42.0), Some(39.0), None, Some(45.0), Some(41.0)],
        )],
    )]);
    for (percent, blocked) in [(80.0, true), (50.0, false)] {
        let policy = SpecificationPolicy {
            monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent },
            ..SpecificationPolicy::default()
        };
        let sheet = build(&specifications, &policy, &run, None);
        assert_rows_agree(&sheet, &specifications, &policy, &run);

        let count = sheet.rows[0]
            .statistics
            .as_ref()
            .and_then(|statistics| statistics.yield_count)
            .expect("a limited row has a yield");
        assert_eq!((count.passing, count.total), (3, 5));
        assert_eq!(
            sheet.summary.acceptance,
            Some(Acceptance {
                blocked,
                blocking: if blocked { vec![0] } else { Vec::new() },
            }),
            "a {percent} % gate"
        );
    }
}

#[test]
fn joint_yield_counts_the_trials_that_pass_every_requirement() {
    let specifications = vec![
        limit("gain", Some(40.0), None, "dB"),
        limit("pm", Some(45.0), None, "deg"),
    ];
    let policy = SpecificationPolicy::default();
    let trials = monte_carlo(
        None,
        1,
        &[
            ("gain", &[Some(42.0), Some(39.0), Some(44.0), Some(45.0)]),
            ("pm", &[Some(50.0), Some(50.0), Some(44.0), Some(50.0)]),
        ],
    );
    let run = history(vec![trials.clone()]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);
    assert_eq!(
        sheet.joint_yield,
        Some(JointYield::Counted {
            passing: 2,
            total: 4,
            specifications: 2,
        })
    );

    // A second analysis measured gain alone, so gain was judged over trials
    // pm never saw: no trial can be said to pass both.
    let run = history(vec![
        trials,
        monte_carlo(None, 9, &[("gain", &[Some(43.0)])]),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);
    assert_eq!(sheet.joint_yield, Some(JointYield::DifferentTrials));
}

/// Trials that ran at two points are counted together for the row and
/// apart for each point.
#[test]
fn monte_carlo_at_two_points_gives_statistics_per_point() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let run = history(vec![
        monte_carlo(
            Some(at("SS", 1.62, 125.0, false)),
            100,
            &[("gain", &[Some(38.0), Some(41.0)])],
        ),
        monte_carlo(
            Some(at("TT", 1.8, 27.0, true)),
            200,
            &[("gain", &[Some(44.0), Some(43.0), Some(39.0)])],
        ),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    let row = &sheet.rows[0];
    let statistics = row.statistics.as_ref().expect("the row has trials");
    let count = statistics.yield_count.expect("a limited row has a yield");
    assert_eq!((count.passing, count.total), (3, 5));
    let per_point = statistics
        .per_point
        .iter()
        .map(|point| {
            let count = point.yield_count.expect("a limited row has a yield");
            (
                point.point,
                point.trials,
                count.passing,
                point.distribution.count,
            )
        })
        .collect::<Vec<_>>();
    // The nominal point heads the sheet, so its trials come first.
    assert_eq!(per_point, [(Some(0), 3, 2, 3), (Some(1), 2, 1, 2)]);
    assert_eq!(row.cells[0].evidence_count, 3);
    assert_eq!(row.cells[1].evidence_count, 2);
    assert_eq!(row.cells[1].value, Some(38.0));
}
