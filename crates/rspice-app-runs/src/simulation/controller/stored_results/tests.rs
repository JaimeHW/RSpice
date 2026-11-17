//! A run's results store as the controller reports it: a transient its volume stopped keeps its
//! rows and says where it stopped, and a stored column a waveform could not read is a finding of
//! the run that wrote it.

use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;
use rspice_app_state::workbench::app_state::diagnostic_store::run_notices::{
    COLUMN_UNREADABLE_CODE, STORAGE_FULL_CODE,
};
use rspice_results::waveform::ColumnReadFailure;
use rspice_simulation::results::{SampledWaveform, TransientStoreReceipt, TransientStoreStop};

use super::*;
use crate::simulation::controller::test_host::RunTestHost;

const SEGMENT: &str = "/tmp/rspice-results/7-1-0/0.rsr";

/// A run of one transient, running, with the controller bound to it as a prepared task.
fn running(controller: &mut SimulationController) -> (RunTestHost, u64) {
    let mut state = RunTestHost::default();
    let run_sequence = state.model.simulation.start_run().id;
    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence_mut(run_sequence)
        .expect("the test run is retained");
    run.mark_running()
        .expect("the test run enters running state");
    state.model.simulation.execution.active_execution = run.execution_identity();
    controller.current_run_id = Some(run_sequence);
    controller.current_analysis_label = Some("Transient".to_owned());
    controller.current_provenance = Some(
        AnalysisResultProvenance::new(
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x5b; 32]),
            Vec::new(),
        )
        .expect("synthetic prepared-task provenance is valid"),
    );
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    (state, run_sequence)
}

/// A transient that wrote three rows to its store, stopped as `stopped` says.
fn stored_transient(stopped: Option<TransientStoreStop>) -> crate::simulation::SimulationResult {
    let mut waveforms = std::collections::HashMap::new();
    waveforms.insert(
        "V(out)".to_owned(),
        SampledWaveform {
            name: "V(out)".to_owned(),
            x_values: Vec::new().into(),
            y_values: vec![0.0, 0.5, 1.0].into(),
            y_unit: "V".to_owned(),
            is_complex: false,
            y_imag: None,
        },
    );
    crate::simulation::SimulationResult::Transient {
        spectra: Vec::new(),
        checks: Default::default(),
        hdl_output: Default::default(),
        time: vec![0.0, 1.0e-3, 2.31e-3].into(),
        waveforms,
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
        store: Some(TransientStoreReceipt {
            path: SEGMENT.into(),
            rows: 3,
            bytes: 4096,
            digest: None,
            stopped,
        }),
    }
}

#[test]
fn a_transient_its_results_storage_stopped_keeps_its_rows_and_says_where_it_stopped() {
    let mut controller = SimulationController::new();
    let (mut state, run_sequence) = running(&mut controller);
    let export_io = super::super::test_execution::MockExportWorkflowIo::default();
    let stop = TransientStoreStop {
        at: Some(2.31e-3),
        free: Some(84 * 1024 * 1024),
    };

    controller.accept_completion(&mut state, &export_io, Ok(stored_transient(Some(stop))));

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is kept");
    // The rows it kept are its result, drawn as any other.
    assert_eq!(
        run.lifecycle,
        crate::state::SimulationRunLifecycle::Completed
    );
    assert!(run.analyses[0].success && run.analyses[0].has_data());
    let stop = run.results_storage_stop().expect("the run says it stopped");
    assert_eq!(
        (stop.at, stop.rows, stop.bytes, stop.folder.as_str()),
        (Some(2.31e-3), 3, 4096, "/tmp/rspice-results/7-1-0")
    );
    let message = "Stopped at t = 2.31ms: the volume holding /tmp/rspice-results/7-1-0 filled; \
                   3 points kept (4.00 KiB), 84.00 MiB free";
    let logged: Vec<_> = run
        .log
        .lines()
        .iter()
        .filter(|line| line.severity == LogSeverity::Error)
        .map(|line| line.message.as_str())
        .collect();
    assert_eq!(logged, [format!("Transient: {message}")]);
    assert_eq!(
        state.model.simulation.execution.status,
        "Stopped: results storage full"
    );

    let records = state.model.diagnostics.published_run_records(run_sequence);
    let full: Vec<_> = records
        .iter()
        .filter(|record| record.canonical.code.as_ref() == STORAGE_FULL_CODE)
        .map(|record| (record.level, record.message.as_str()))
        .collect();
    assert_eq!(full, [(DiagnosticLevel::Error, message)]);
}

#[test]
fn a_transient_its_store_kept_whole_says_nothing_of_its_store() {
    let mut controller = SimulationController::new();
    let (mut state, run_sequence) = running(&mut controller);
    let export_io = super::super::test_execution::MockExportWorkflowIo::default();

    controller.accept_completion(&mut state, &export_io, Ok(stored_transient(None)));

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is kept");
    assert!(run.results_storage_stop().is_none());
    assert_eq!(run.log.count(LogSeverity::Error), 0);
    assert_eq!(state.model.simulation.execution.status, "Complete");
    assert!(
        state
            .model
            .diagnostics
            .published_run_records(run_sequence)
            .is_empty()
    );
}

#[test]
fn a_stored_column_that_cannot_be_read_is_a_finding_of_the_run_that_wrote_it() {
    let mut controller = SimulationController::new();
    let (mut state, run_sequence) = running(&mut controller);
    let export_io = super::super::test_execution::MockExportWorkflowIo::default();
    controller.accept_completion(&mut state, &export_io, Ok(stored_transient(None)));
    let failure = |store: &str| ColumnReadFailure {
        column: "V(out)".to_owned(),
        store: store.to_owned(),
        cause: "chunk 4's checksum does not match".to_owned(),
    };

    controller.report_column_failures(
        &mut state,
        &[failure(SEGMENT), failure("/tmp/rspice-results/7-1-0/9.rsr")],
    );

    let records = state.model.diagnostics.published_run_records(run_sequence);
    let unreadable: Vec<_> = records
        .iter()
        .filter(|record| record.canonical.code.as_ref() == COLUMN_UNREADABLE_CODE)
        .map(|record| {
            (
                record.level,
                record.subject.object.as_str(),
                record.message.as_str(),
            )
        })
        .collect();
    assert_eq!(
        unreadable,
        [(
            DiagnosticLevel::Error,
            "V(out)",
            "The stored samples of V(out) could not be read, so its trace shows none there: \
             chunk 4's checksum does not match"
        )]
    );
    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is kept");
    assert_eq!(run.log.count(LogSeverity::Error), 1);
    // A store no kept run wrote is said in the session's log alone.
    assert!(state.model.log_buffer.entries().any(|entry| entry.message
        == "Results: the stored samples of 'V(out)' could not be read from \
            /tmp/rspice-results/7-1-0/9.rsr: chunk 4's checksum does not match"));
}
