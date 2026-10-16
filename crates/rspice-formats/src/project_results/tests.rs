use super::*;

/// The reader's accept set is the engine's own label vocabulary, so nothing the
/// engine can name is refused on write. This is the property that failed: the
/// reader carried a hand-copied subset, and every family that had outgrown it
/// wrote a project the reader would not take back.
#[test]
fn the_project_reader_accepts_every_label_the_engine_can_emit() {
    for label in rspice_core::circuit::OP_LABELS {
        require_static_label(label.as_str(), "device_op label")
            .unwrap_or_else(|error| panic!("{error}"));
    }
}

/// One prepared-task provenance record as a project writes it: a small-signal
/// result depending on `parent`, with `state` attached when it has one.
fn state_provenance(
    parent: AnalysisInstanceId,
    state: Option<AnalysisResultTransientState>,
) -> Result<AnalysisResultProvenance, String> {
    AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x2a; 32]),
        vec![parent],
    )?
    .with_transient_state(state)
}

/// A result linearized around a transient state keeps its parent and its
/// time through the project file, bit for bit.
#[test]
fn a_transient_state_survives_the_project_file() {
    let parent = AnalysisInstanceId::new();
    let state = AnalysisResultTransientState {
        parent,
        time_s: 5.5e-4,
    };
    let provenance = state_provenance(parent, Some(state)).expect("the provenance builds");
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
        .expect("the provenance writes");
    assert!(text.contains("\"transient_state\""), "{text}");
    let read: ProjectAnalysisResultProvenance =
        serde_json::from_str(&text).expect("the provenance reads");
    read.validate().expect("the read provenance validates");
    let restored = read.into_provenance().expect("the provenance restores");
    assert_eq!(restored, provenance);
    assert_eq!(restored.transient_state(), Some(&state));
}

/// A project written before results named a transient state reads as it
/// always did: every result in it was linearized at the operating point, and
/// a result that names no state writes no field.
#[test]
fn a_project_written_before_transient_states_still_reads() {
    let provenance = state_provenance(AnalysisInstanceId::new(), None).expect("builds");
    let text = serde_json::to_string(&ProjectAnalysisResultProvenance::from(&provenance))
        .expect("the provenance writes");
    assert!(!text.contains("transient_state"), "{text}");
    let read: ProjectAnalysisResultProvenance =
        serde_json::from_str(&text).expect("an older record reads");
    read.validate().expect("an older record validates");
    let restored = read.into_provenance().expect("an older record restores");
    assert_eq!(restored.transient_state(), None);
    assert_eq!(restored, provenance);
}

/// A state whose parent is not among the result's dependencies, or whose
/// time is not a time, is refused rather than persisted.
#[test]
fn a_transient_state_names_one_of_its_dependencies_and_a_finite_time() {
    let parent = AnalysisInstanceId::new();
    let stranger = AnalysisResultTransientState {
        parent: AnalysisInstanceId::new(),
        time_s: 1e-6,
    };
    let error = state_provenance(parent, Some(stranger)).expect_err("a stranger parent");
    assert!(error.contains("not one of its dependencies"), "{error}");
    for time_s in [f64::NAN, f64::INFINITY, -1e-9] {
        let error = state_provenance(
            parent,
            Some(AnalysisResultTransientState { parent, time_s }),
        )
        .expect_err("not a time");
        assert!(error.contains("not a finite time"), "{time_s}: {error}");
    }
}

/// A run's log is read back from the project as it was written, and a run
/// with nothing in its log writes no log at all, so a project from before
/// runs kept one is the same file.
#[test]
fn a_runs_log_survives_the_project_file_and_an_empty_one_is_not_written() {
    use rspice_app_types::diagnostics::{LogSeverity, LogSource};
    use rspice_app_types::run_log::RunLogLine;

    let mut run = SimulationRun::<AnalysisResult>::new(
        4,
        1_760_613_845.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("a fresh run takes provenance");
    let persist = |run: &SimulationRun| {
        ProjectSimulationRun::from_run::<_, RetainedWaveform>(run, |_| {
            unreachable!("the run retained no analyses")
        })
    };
    let written = serde_json::to_value(persist(&run)).expect("serializes");
    assert!(written.get("log").is_none(), "{written}");

    run.log.push(RunLogLine {
        unix_ms: 1_760_613_846_020,
        severity: LogSeverity::Warning,
        source: LogSource::Hdl,
        message: "tb.v:14: $error: output stuck low".to_owned(),
    });
    let text = serde_json::to_string(&persist(&run)).expect("serializes");
    let read: ProjectSimulationRun = serde_json::from_str(&text).expect("deserializes");
    assert_eq!(read.into_run().expect("restores").log, run.log);
}
