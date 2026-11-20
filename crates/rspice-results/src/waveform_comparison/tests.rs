use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use super::*;
use crate::run::ExecutionTarget;
use crate::waveform::{ColumnSource, StoredWindow, WindowLead};
use rspice_vocabulary::product::{ContentDigest, DatasetId};

fn source(number: u64, analysis_id: u64, coordinates: Vec<f64>, values: Vec<f64>) -> SimulationRun {
    let mut run = SimulationRun::new(number, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(
        AnalysisResult::new(analysis_id, AnalysisType::Transient, "TRAN", 0.0)
            .with_waveforms(vec![RetainedWaveform::new("V(out)", coordinates, values)]),
    );
    run
}

fn side(run: &SimulationRun) -> ComparisonSide<'_, RetainedWaveform> {
    ComparisonSide {
        binding: DatasetBinding::new(run.dataset_id, ContentDigest::from_bytes([7; 32])),
        analysis: &run.analyses[0],
    }
}

fn options(alignment: ComparisonAlignmentDraft) -> ComparisonOptions<'static> {
    ComparisonOptions {
        alignment,
        alignment_signal: "V(out)",
        threshold: 0.0,
        maximum_lag_samples: 2,
        absolute_tolerance: 0.0,
        relative_tolerance: 0.0,
    }
}

fn compare(
    candidate: &SimulationRun,
    baseline: &SimulationRun,
    signals: &[&str],
    options: ComparisonOptions<'_>,
) -> Result<ComparisonOutcome, String> {
    let signals: Vec<String> = signals.iter().map(|name| (*name).to_owned()).collect();
    execute_comparison(
        side(candidate),
        side(baseline),
        &signals,
        options,
        &mut |_, _| true,
    )
}

#[test]
fn comparison_records_threshold_and_cross_correlation_alignment_parameters() {
    let threshold_candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![-1.0, 1.0, 3.0]);
    let threshold_baseline = source(2, 29, vec![10.0, 11.0, 12.0], vec![-2.0, 2.0, 4.0]);
    let threshold_receipt = compare(
        &threshold_candidate,
        &threshold_baseline,
        &["V(out)"],
        options(ComparisonAlignmentDraft::FirstThresholdCrossing),
    )
    .expect("threshold alignment must execute")
    .receipt;
    assert!(matches!(
        threshold_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::FirstThresholdCrossing {
            signal_key,
            threshold: 0.0,
            baseline_crossing: 10.5,
            candidate_crossing: 0.5,
        } if signal_key == "signal:0"
    ));
    assert_eq!(
        threshold_receipt.policy.execution.resampling,
        ComparisonResamplingPolicy::BaselineOntoCandidateGrid
    );

    let correlation_candidate = source(
        1,
        17,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        vec![0.0, 0.0, 1.0, 0.0, -1.0, 0.0],
    );
    let correlation_baseline = source(
        2,
        29,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
    );
    let correlation_receipt = compare(
        &correlation_candidate,
        &correlation_baseline,
        &["V(out)"],
        options(ComparisonAlignmentDraft::CrossCorrelation),
    )
    .expect("correlation alignment must execute")
    .receipt;
    assert!(matches!(
        correlation_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::CrossCorrelation {
            selected_lag_samples: 1,
            sample_interval: 1.0,
            baseline_shift: 1.0,
            ..
        }
    ));
}

#[test]
fn interpolation_and_threshold_crossings_stay_finite_across_the_binary64_range() {
    assert_eq!(
        interpolate_monotone(&[-1e308, 1e308], &[0.0, 2.0], 0.0),
        Ok(1.0)
    );
    assert_eq!(
        interpolate_monotone(&[0.0, 1.0], &[-1e308, 1e308], 0.5),
        Ok(0.0)
    );
    assert_eq!(
        first_threshold_crossing(&[0.0, 1.0], &[-1e308, 1e308], 0.0),
        Ok(0.5)
    );
    assert_eq!(
        first_threshold_crossing(&[-1e308, 1e308], &[-1.0, 1.0], 0.0),
        Ok(0.0)
    );
}

#[test]
fn ordinary_interpolation_and_crossings_keep_their_previous_bits() {
    // The arithmetic the comparison used before it shared the overflow-safe
    // kernel; wherever nothing overflows the results are bit-identical.
    let previous = |x: [f64; 2], y: [f64; 2], at: f64| {
        let fraction = (at - x[0]) / (x[1] - x[0]);
        y[0] + fraction * (y[1] - y[0])
    };
    assert_eq!(
        interpolate_monotone(&[0.0, 3.0], &[0.2, 5.0], 1.0),
        Ok(1.7999999999999998)
    );
    // A convex combination would put 1-ULP noise on a flat segment.
    for at in (1..1000).map(|step| f64::from(step) / 1000.0) {
        assert_eq!(interpolate_monotone(&[0.0, 1.0], &[0.1, 0.1], at), Ok(0.1));
    }
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut unit = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (state >> 11) as f64 / (1_u64 << 53) as f64
    };
    for _ in 0..20_000 {
        let x0 = unit() * 1e-6;
        let x1 = x0 + 1e-12 + unit() * 1e-7;
        let y0 = (unit() - 0.5) * 10.0;
        let y1 = if unit() < 0.25 {
            y0
        } else {
            y0 + (unit() - 0.5) * 10.0_f64.powi(-(unit() * 6.0) as i32)
        };
        let at = (x0 + unit() * (x1 - x0)).min(x1);
        assert_eq!(
            interpolate_monotone(&[x0, x1], &[y0, y1], at)
                .unwrap()
                .to_bits(),
            previous([x0, x1], [y0, y1], at).to_bits(),
            "x=[{x0:e}, {x1:e}] y=[{y0:e}, {y1:e}] at={at:e}"
        );
        if y0 == y1 {
            continue;
        }
        let threshold = (y0 + unit() * (y1 - y0)).clamp(y0.min(y1), y0.max(y1));
        let expected = if threshold.to_bits() == y0.to_bits() {
            x0
        } else {
            previous([y0, y1], [x0, x1], threshold)
        };
        assert_eq!(
            first_threshold_crossing(&[x0, x1], &[y0, y1], threshold)
                .unwrap()
                .to_bits(),
            expected.to_bits(),
            "x=[{x0:e}, {x1:e}] y=[{y0:e}, {y1:e}] threshold={threshold:e}"
        );
    }
}

#[test]
fn comparison_fails_closed_for_nonmonotonic_source_coordinates() {
    let candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
    let baseline = source(2, 29, vec![0.0, 1.0, 0.5], vec![0.0, 1.0, 2.0]);
    let error = compare(
        &candidate,
        &baseline,
        &["V(out)"],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .expect_err("nonmonotonic immutable data must never be resampled");
    assert!(error.contains("nonmonotonic"), "{error}");
}

#[test]
fn comparison_requires_compatible_import_coordinates_and_retains_their_units() {
    use crate::result_import::{ResultImportCoordinate, ResultImportFormat, ResultImportSource};
    let mut candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
    let mut baseline = source(2, 29, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
    for run in [&mut candidate, &mut baseline] {
        run.analyses[0].analysis_type = AnalysisType::DcSweep;
        run.analyses[0].import_source = Some(ResultImportSource {
            source_name: "source.raw".into(),
            format: ResultImportFormat::SpiceRaw,
            coordinate: Some(ResultImportCoordinate {
                name: "bias".into(),
                unit: Some("A".into()),
            }),
            family_member: None,
        });
    }
    let outcome = compare(
        &candidate,
        &baseline,
        &["V(out)"],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .unwrap();
    assert_eq!(outcome.coordinate_unit.as_deref(), Some("A"));
    for unit in [Some("V"), None, Some("1")] {
        baseline.analyses[0]
            .import_source
            .as_mut()
            .unwrap()
            .coordinate
            .as_mut()
            .unwrap()
            .unit = unit.map(str::to_owned);
        let error = compare(
            &candidate,
            &baseline,
            &["V(out)"],
            options(ComparisonAlignmentDraft::AbsoluteXAxis),
        )
        .expect_err("incompatible units");
        assert!(error.contains("sweep different quantities"), "{error}");
        let error = comparison_signal_names(&candidate.analyses[0], &baseline.analyses[0])
            .expect_err("the signal list refuses them too");
        assert!(error.contains("sweep different quantities"), "{error}");
    }
}

#[test]
fn comparison_rejects_signal_unit_mismatches_before_any_alignment() {
    let mut candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![-1.0, 0.0, 1.0]);
    candidate.analyses[0].waveforms[0].unit = Some("V".into());
    for unit in [Some("mV"), Some("A"), Some("1"), None] {
        let mut baseline = source(2, 29, vec![0.0, 1.0, 2.0], vec![-1.0, 0.0, 1.0]);
        baseline.analyses[0].waveforms[0].unit = unit.map(str::to_owned);
        for alignment in [
            ComparisonAlignmentDraft::AbsoluteXAxis,
            ComparisonAlignmentDraft::FirstThresholdCrossing,
            ComparisonAlignmentDraft::CrossCorrelation,
        ] {
            let error = compare(&candidate, &baseline, &["V(out)"], options(alignment))
                .expect_err("identical numbers in different units are not identical quantities");
            assert!(
                error.contains("V(out)") && error.contains("in the same unit"),
                "{error}"
            );
        }
    }
}

#[test]
fn comparison_keeps_each_signals_declared_unit() {
    for unit in [Some("V"), Some("mV"), Some("arbitrary"), None] {
        let mut candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
        let mut baseline = source(2, 29, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
        for run in [&mut candidate, &mut baseline] {
            run.analyses[0].waveforms[0].unit = unit.map(str::to_owned);
        }
        let outcome = compare(
            &candidate,
            &baseline,
            &["V(out)"],
            options(ComparisonAlignmentDraft::AbsoluteXAxis),
        )
        .unwrap();
        assert_eq!(outcome.units, vec![unit.map(str::to_owned)]);
        assert_eq!(outcome.signals, vec!["V(out)".to_owned()]);
    }
}

/// A run of `signals` signals over `rows` points: signal k is k·sin(t).
fn wide_run(number: u64, rows: usize, signals: usize, offset: f64) -> SimulationRun {
    let time: Arc<Vec<f64>> = Arc::new((0..rows).map(|row| row as f64 * 1.0e-9).collect());
    let mut run = SimulationRun::new(number, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(
            (0..signals)
                .map(|signal| {
                    let values: Vec<f64> = time
                        .iter()
                        .map(|t| signal as f64 * (t * 1.0e6).sin() + offset)
                        .collect();
                    RetainedWaveform::new(format!("V(n{signal})"), Arc::clone(&time), values)
                })
                .collect(),
        ),
    );
    run
}

/// Two twenty-signal runs of 40,000 points compare whole, and so does a run
/// past the million rows a table of copied samples was refused at: a
/// comparison reads the runs, it copies nothing into a table.
#[test]
fn long_and_wide_runs_compare_without_a_size_refusal() {
    let candidate = wide_run(1, 40_000, 20, 0.0);
    let baseline = wide_run(2, 40_000, 20, 0.0);
    let signals = comparison_signal_names(&candidate.analyses[0], &baseline.analyses[0]).unwrap();
    assert_eq!(signals.len(), 20);
    let names: Vec<&str> = signals.iter().map(String::as_str).collect();
    let outcome = compare(
        &candidate,
        &baseline,
        &names,
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .expect("two ordinary runs compare");
    assert_eq!(outcome.receipt.rows_compared, 40_000);
    assert_eq!(outcome.receipt.signals.len(), 20);
    assert_eq!(outcome.receipt.disposition, ComparisonDisposition::Passed);

    let candidate = wide_run(1, 1_000_001, 1, 0.0);
    let baseline = wide_run(2, 1_000_001, 1, 0.0);
    let outcome = compare(
        &candidate,
        &baseline,
        &["V(n0)"],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .expect("a run past a million points compares");
    assert_eq!(outcome.receipt.rows_compared, 1_000_001);
}

/// A column of a results store in memory, counting the rows read of it.
struct Counted {
    name: String,
    values: Vec<f64>,
    rows_read: AtomicU64,
}

impl ColumnSource for Counted {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.rows_read.fetch_add(out.len() as u64, Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }

    fn ascending(&self) -> bool {
        self.name == "time"
    }
}

/// A transient whose axis and `signals` signals a results store keeps; the
/// columns come back, axis first, to count what is read of them.
fn stored_run(number: u64, rows: usize, signals: usize) -> (SimulationRun, Vec<Arc<Counted>>) {
    let column = |name: String, values: Vec<f64>| {
        Arc::new(Counted {
            name,
            values,
            rows_read: AtomicU64::new(0),
        })
    };
    let window = StoredWindow {
        first: 0,
        rows: rows as u64,
        lead: WindowLead::None,
    };
    let stored = |column: &Arc<Counted>| {
        WaveformValues::stored(column.clone(), window).expect("the window fits the column")
    };
    let axis = column(
        "time".to_owned(),
        (0..rows).map(|row| row as f64 * 1.0e-9).collect(),
    );
    let time = stored(&axis);
    let mut columns = vec![axis];
    let mut waveforms = Vec::new();
    for signal in 0..signals {
        let name = format!("V(n{signal})");
        let values = (0..rows).map(|row| (row * (signal + 1)) as f64).collect();
        let held = column(name.clone(), values);
        waveforms.push(RetainedWaveform::new(name, time.clone(), stored(&held)));
        columns.push(held);
    }
    let mut run = SimulationRun::new(number, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(waveforms),
    );
    (run, columns)
}

/// Comparing three chosen signals of two stored twenty-signal runs reads the
/// axes and those three columns of each, once each, and keeps none of them:
/// a second comparison reads them all again.
#[test]
fn a_comparison_of_chosen_signals_reads_only_those_and_keeps_nothing() {
    let rows = 4_000;
    let (candidate, candidate_columns) = stored_run(1, rows, 20);
    let (baseline, baseline_columns) = stored_run(2, rows, 20);
    let listed = comparison_signal_names(&candidate.analyses[0], &baseline.analyses[0]).unwrap();
    assert_eq!(listed.len(), 20);
    let read = |columns: &[Arc<Counted>]| {
        columns
            .iter()
            .map(|column| column.rows_read.load(Ordering::SeqCst))
            .collect::<Vec<_>>()
    };
    assert!(
        read(&candidate_columns)
            .iter()
            .chain(&read(&baseline_columns))
            .all(|rows| *rows == 0),
        "listing the signals read samples"
    );
    let chosen = ["V(n3)", "V(n7)", "V(n12)"];
    for pass in 1..=2_u64 {
        compare(
            &candidate,
            &baseline,
            &chosen,
            options(ComparisonAlignmentDraft::AbsoluteXAxis),
        )
        .expect("the stored runs compare");
        for columns in [&candidate_columns, &baseline_columns] {
            for (index, rows_read) in read(columns).into_iter().enumerate() {
                let expected = if index == 0 || [4, 8, 13].contains(&index) {
                    pass * rows as u64
                } else {
                    0
                };
                assert_eq!(
                    rows_read, expected,
                    "column {index} after comparison {pass}"
                );
            }
        }
    }
}

/// Every place a signal parts from its baseline by more than the tolerance
/// is found, on the candidate's own X axis, where it starts, ends and is
/// worst.
#[test]
fn the_places_a_signal_parts_from_its_baseline_are_found() {
    let x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let candidate = source(1, 17, x.clone(), vec![0.0, 0.5, 0.0, 0.0, -0.2, -0.4, 0.0]);
    let baseline = source(2, 29, x, vec![0.0; 7]);
    let outcome = compare(
        &candidate,
        &baseline,
        &["V(out)"],
        ComparisonOptions {
            absolute_tolerance: 0.1,
            ..options(ComparisonAlignmentDraft::AbsoluteXAxis)
        },
    )
    .unwrap();
    assert_eq!(
        outcome.places,
        vec![
            ComparisonPlace {
                signal: 0,
                first: 1.0,
                last: 1.0,
                worst_x: 1.0,
                worst_error: 0.5,
            },
            ComparisonPlace {
                signal: 0,
                first: 4.0,
                last: 5.0,
                worst_x: 5.0,
                worst_error: -0.4,
            },
        ]
    );
    assert_eq!(outcome.receipt.signals[0].failed_rows, 3);
}

#[test]
fn a_comparison_names_what_it_cannot_compare() {
    let candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
    let baseline = source(2, 29, vec![0.0, 1.0, 2.0], vec![0.0, f64::NAN, 2.0]);
    let error = compare(
        &candidate,
        &baseline,
        &["V(out)"],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .unwrap_err();
    assert_eq!(
        error,
        "V(out) has a missing sample in the baseline, so it cannot be compared."
    );
    let error = compare(
        &candidate,
        &baseline,
        &[],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .unwrap_err();
    assert_eq!(error, "Choose at least one signal to compare.");
    let mut same = candidate.clone();
    same.dataset_id = DatasetId::new();
    let mut stopped = 0;
    let error = execute_comparison(
        side(&candidate),
        side(&same),
        &["V(out)".to_owned()],
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
        &mut |done, total| {
            stopped = done;
            assert_eq!(total, 1);
            false
        },
    )
    .unwrap_err();
    assert_eq!(
        (error.as_str(), stopped),
        ("The comparison was stopped.", 1)
    );
}
