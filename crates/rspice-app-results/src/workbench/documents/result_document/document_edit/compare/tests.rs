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
