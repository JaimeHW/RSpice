//! Whether Compare datasets is offered, as the toolbar asks it on every frame.

use rspice_results::waveform::column_work::ColumnWork;

use super::*;
use crate::state::SimulationRun;
use crate::workbench::documents::result_document::ResultsTestHost;
use crate::workbench::documents::result_document::test_fixtures::{
    CountedColumn, stored_transient,
};

/// Two retained runs of one transient, each kept in a results store, the
/// first shown in Results.
fn two_stored_runs(rows: usize) -> (ResultsTestHost, Vec<std::sync::Arc<CountedColumn>>) {
    let mut host = ResultsTestHost::default();
    let mut columns = Vec::new();
    let mut runs = Vec::new();
    for id in 1..=2 {
        let time: Vec<f64> = (0..rows).map(|row| row as f64 * 1.0e-9).collect();
        let out: Vec<f64> = (0..rows).map(|row| (row as f64 * 1.0e-3).sin()).collect();
        let (analysis, held) =
            stored_transient(time, vec![("V(out)".to_owned(), out)], Some([id as u8; 32]));
        let mut run = SimulationRun::new(id);
        run.add_analysis(analysis);
        runs.push(run);
        columns.extend(held);
    }
    host.model.simulation.retained.runs = runs.into();
    assert!(host.model.simulation.select_run(0));
    assert!(host.model.simulation.select_analysis(0));
    host.workbench.workspace = Workspace::Results;
    (host, columns)
}

/// Compare datasets is offered over a stored run that has a stored baseline,
/// and deciding so, frame after frame, reads none of either run's samples:
/// the toolbar used to read every common column of every comparable run on
/// every frame, and keep what it read for the runs' lives.
#[test]
fn compare_is_offered_without_reading_a_sample_of_either_run() {
    let (host, columns) = two_stored_runs(50_000);
    let before = ColumnWork::read();
    for _ in 0..10 {
        assert!(compare_availability(host.results_reads()).is_available());
    }
    let work = before.since();
    assert_eq!(
        (work.stored_views, work.stored_reads, work.memory_views),
        (0, 0, 0),
        "deciding Compare's availability read whole columns"
    );
    assert!(
        columns.iter().all(|column| column.rows_read() <= 2),
        "deciding Compare's availability read rows: {:?}",
        columns
            .iter()
            .map(|column| column.rows_read())
            .collect::<Vec<_>>()
    );
}

/// A baseline that holds none of the candidate's signals is no baseline,
/// stored or not.
#[test]
fn compare_is_not_offered_against_a_run_of_other_signals() {
    let (mut host, _) = two_stored_runs(1_000);
    let mut other = SimulationRun::new(3);
    other.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![crate::state::RetainedTrace::new(
                "V(elsewhere)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#fff",
            )]),
    );
    let first = host.model.simulation.retained.runs[0].clone();
    host.model.simulation.retained.runs = vec![first, other].into();
    assert!(host.model.simulation.select_run(0));
    assert!(host.model.simulation.select_analysis(0));
    assert!(!compare_availability(host.results_reads()).is_available());
}

/// Two retained runs of one transient with `signals` stored signals each, the
/// baseline's every signal 0.5 V below the result's, the first shown in
/// Results.
fn two_wide_stored_runs(
    signals: usize,
    rows: usize,
) -> (ResultsTestHost, Vec<std::sync::Arc<CountedColumn>>) {
    let mut host = ResultsTestHost::default();
    let mut columns = Vec::new();
    let mut runs = Vec::new();
    for id in 1..=2_u64 {
        let time: Vec<f64> = (0..rows).map(|row| row as f64 * 1.0e-9).collect();
        let offset = if id == 1 { 0.5 } else { 0.0 };
        let traces = (0..signals)
            .map(|signal| {
                let values = (0..rows)
                    .map(|row| (row as f64 * 1.0e-3 + signal as f64).sin() + offset)
                    .collect();
                (format!("V(n{signal})"), values)
            })
            .collect();
        let (analysis, held) = stored_transient(time, traces, Some([id as u8; 32]));
        let mut run = SimulationRun::new(id);
        run.add_analysis(analysis);
        runs.push(run);
        columns.extend(held);
    }
    host.model.simulation.retained.runs = runs.into();
    assert!(host.model.simulation.select_run(0));
    assert!(host.model.simulation.select_analysis(0));
    host.workbench.workspace = Workspace::Results;
    (host, columns)
}

fn open_draft(host: &mut ResultsTestHost) -> CompareDraft {
    open_compare(&mut host.results_parts());
    match host.results_parts().result_document_dialog.clone() {
        Some(ResultDocumentDialog::Compare(draft)) => draft,
        _ => panic!("Compare datasets opens its form"),
    }
}

/// Run the form's draft to its outcome.
fn compare_draft(host: &mut ResultsTestHost, draft: CompareDraft) -> CompareOutcome {
    let mut parts = host.results_parts();
    *parts.result_document_dialog = Some(ResultDocumentDialog::Compare(draft));
    commit_open_dialog(&mut parts);
    settle_comparison(&mut parts);
    match parts.result_document_dialog.clone() {
        Some(ResultDocumentDialog::Compare(draft)) => draft
            .outcome
            .unwrap_or_else(|| panic!("the comparison ran: {:?}", host.prompt_text())),
        _ => panic!("the form stays open on its outcome"),
    }
}

/// Compare datasets opens with the plotted signals chosen, and offers its
/// choices frame after frame without reading a sample of either run.
#[test]
fn compare_opens_on_the_plotted_signals_without_reading_a_sample() {
    let (mut host, columns) = two_wide_stored_runs(6, 10_000);
    for (index, waveform) in host.model.simulation.retained.runs[0].analyses[0]
        .waveforms
        .iter_mut()
        .enumerate()
    {
        waveform.visible = index % 2 == 0;
    }
    let mut draft = open_draft(&mut host);
    assert_eq!(draft.signals, ["V(n0)", "V(n2)", "V(n4)"]);
    for _ in 0..10 {
        let choices = choices(host.results_reads(), &mut draft);
        assert_eq!(choices.refusal, None);
        assert_eq!(choices.signals.len(), 6);
    }
    assert!(
        columns.iter().all(|column| column.rows_read() <= 2),
        "offering the comparison read rows: {:?}",
        columns
            .iter()
            .map(|column| column.rows_read())
            .collect::<Vec<_>>()
    );
    draft.signals.clear();
    assert_eq!(
        choices(host.results_reads(), &mut draft).refusal.as_deref(),
        Some("Choose at least one signal to compare.")
    );
}

/// Comparing three of twenty stored signals reads the axis and those three
/// of each run, and nothing of the other seventeen; once it answers no
/// comparison is held.
#[test]
fn a_comparison_of_three_chosen_signals_reads_only_those() {
    let rows = 40_000;
    let (mut host, columns) = two_wide_stored_runs(20, rows);
    let mut draft = open_draft(&mut host);
    draft.signals = ["V(n3)", "V(n7)", "V(n12)"].map(str::to_owned).to_vec();
    draft.absolute_tolerance = "1".to_owned();
    let outcome = compare_draft(&mut host, draft);
    assert_eq!(outcome.rows_compared, rows);
    assert_eq!(
        outcome
            .signals
            .iter()
            .map(|signal| signal.name.as_str())
            .collect::<Vec<_>>(),
        ["V(n12)", "V(n3)", "V(n7)"],
        "the outcome lists the signals by name"
    );
    assert!(
        outcome
            .signals
            .iter()
            .all(|signal| signal.verdict == Verdict::Pass)
    );
    // Each run's columns: the axis, then V(n0) to V(n19).
    for (run, held) in columns.chunks(21).enumerate() {
        for (index, column) in held.iter().enumerate() {
            let chosen = matches!(index, 0 | 4 | 8 | 13);
            if chosen {
                assert!(
                    column.rows_read() >= rows as u64,
                    "run {run} column {index} was not read"
                );
            } else {
                assert!(
                    column.rows_read() <= 2,
                    "run {run} column {index} was read: {} rows",
                    column.rows_read()
                );
            }
        }
    }
    assert!(
        !running(host.results_reads()),
        "the answered comparison is held"
    );
}

/// Two runs of six samples whose V(out) parts by more than 100 mV from 1 ms
/// to 2 ms and at 5 ms, the first compared with the second.
fn parted_comparison() -> ResultsTestHost {
    let mut host = ResultsTestHost::default();
    let time = vec![0.0, 1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3, 5.0e-3];
    let mut runs = Vec::new();
    for (id, out) in [
        (1, vec![0.0, 1.0, 1.0, 0.0, 0.0, 2.0]),
        (2, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ] {
        let mut run = SimulationRun::new(id);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![crate::state::RetainedTrace::new(
                    "V(out)",
                    time.clone(),
                    out,
                    "#fff",
                )]),
        );
        runs.push(run);
    }
    host.model.simulation.retained.runs = runs.into();
    assert!(host.model.simulation.select_run(0));
    assert!(host.model.simulation.select_analysis(0));
    host.workbench.workspace = Workspace::Results;
    let mut draft = open_draft(&mut host);
    draft.absolute_tolerance = "100m".to_owned();
    let outcome = compare_draft(&mut host, draft);
    assert_eq!(outcome.signals[0].verdict, Verdict::Fail);
    host
}

/// The places the report lists.
fn reported_places(host: &mut ResultsTestHost) -> Vec<String> {
    let io =
        crate::workbench::documents::result_document::test_fixtures::MockExportWorkflowIo::default(
        );
    save_report_with_io(&mut host.results_parts(), &io);
    let files = io.text_files.borrow();
    let (_, text) = files.first().expect("the report is written");
    text.lines()
        .skip_while(|line| !line.starts_with("Where the runs part"))
        .skip(2)
        .map(str::to_owned)
        .collect()
}

/// The report states the comparison and lists the places the runs part, the
/// same places the comparison found.
#[test]
fn the_report_lists_every_place_the_runs_part() {
    let mut host = parted_comparison();
    let io =
        crate::workbench::documents::result_document::test_fixtures::MockExportWorkflowIo::default(
        );
    save_report_with_io(&mut host.results_parts(), &io);
    let files = io.text_files.borrow();
    let (path, text) = files.first().expect("the report is written");
    assert_eq!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("txt")
    );
    let places = text
        .lines()
        .skip_while(|line| !line.starts_with("Where the runs part"))
        .skip(2)
        .collect::<Vec<_>>();
    assert_eq!(
        places,
        [
            "V(out)  1 ms  2 ms  1 ms      1 V",
            "V(out)  5 ms  5 ms  5 ms      2 V",
        ],
        "{text}"
    );
    assert!(text.contains("Aligned by X value"), "{text}");
    assert!(text.contains("V(out)  3 of 6"), "{text}");
}

/// Next difference moves the stepping cursor to the first sample outside
/// the tolerance of each place the report lists, in order, and no further;
/// Previous difference steps back.
#[test]
fn next_difference_lands_on_each_place_the_report_lists() {
    use rspice_results::feature_search::Direction;

    let mut host = parted_comparison();
    let places = reported_places(&mut host);
    let tokens = crate::ui::tokens::Tokens::default();
    let mut step = |direction| step_difference(&mut host.results_parts(), &tokens, direction);
    let mut landed = Vec::new();
    while let Ok(x) = step(Direction::Next) {
        landed.push(x);
    }
    assert_eq!(landed, [1.0e-3, 5.0e-3]);
    let refusal = step(Direction::Next).expect_err("no place follows the last");
    assert!(refusal.contains("after the cursor"), "{refusal}");
    assert_eq!(step(Direction::Previous), Ok(1.0e-3));
    let session = &host.results.results.session;
    assert_eq!(session.cursors.get(session.stepping_cursor), Some(1.0e-3));
    // Each place the report lists starts where a step landed.
    assert_eq!(places.len(), landed.len(), "{places:?}");
    for (place, from) in places.iter().zip(["1 ms", "5 ms"]) {
        assert!(place.starts_with(&format!("V(out)  {from}")), "{place}");
    }
}

/// Plot difference draws the difference of each signal that parts from the
/// baseline on the result's strip, read through the pinned baseline, and
/// Next difference is offered.
#[test]
fn plot_difference_draws_each_parted_signal_less_its_baseline() {
    let mut host = parted_comparison();
    plot_difference(&mut host.results_parts()).expect("V(out) can be named");
    let simulation = &host.model.simulation;
    let run = simulation.active_run().expect("a run is shown");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let drawn: Vec<&str> = host.results.results.session.analysis_exprs[&key]
        .iter()
        .map(|expr| expr.text.as_str())
        .collect();
    assert_eq!(drawn, ["V(out) - V(out)@baseline"]);
    let baseline = simulation.retained.runs[1].dataset_id;
    assert_eq!(
        pinned_baseline(host.results_reads(), run.dataset_id),
        Some(baseline)
    );
    assert!(compared(host.results_reads()));
}
