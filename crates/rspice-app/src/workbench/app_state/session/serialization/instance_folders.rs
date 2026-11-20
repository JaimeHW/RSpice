//! Each running instance keeps the results its stored session reads in a folder of its own, so
//! instances never let go of each other's.

use super::tests::{relaunch, saved_session};
use super::*;
use rspice_app_services::io::result_segments::as_another_instance;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
    SimulationRunProvenance,
};

/// A session holding one finished run whose samples are in the results store.
fn session_with_a_run(level: f64) -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.mark_running().expect("a new run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("a running run completes");
    run.keep_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![level, level * 2.0, level * 3.0],
                "#00aaff",
            ),
        ]),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy fixture provenance seals");
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.retained.next_run_id = 1;
    state
}

/// Every sample of every run, as bits.
fn samples(state: &AppState) -> Vec<Vec<u64>> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .flat_map(|run| &run.analyses)
        .flat_map(|analysis| &analysis.waveforms)
        .flat_map(|waveform| [&waveform.x, &waveform.y])
        .map(|values| values.samples(..).map(f64::to_bits).collect())
        .collect()
}

#[test]
fn two_instances_keep_each_others_session_results() {
    let mut first = session_with_a_run(1.0);
    let first_samples = samples(&first);
    saved_session(&mut first);

    // A second instance stores its session and ends while the first still runs.
    let (second_session, second_samples) = as_another_instance(|| {
        let mut second = session_with_a_run(5.0);
        let read = samples(&second);
        (saved_session(&mut second), read)
    });
    // The first stores its session again, as it does when it closes.
    let first_session = saved_session(&mut first);
    drop(first);

    for (session, expected) in [
        (first_session, first_samples),
        (second_session, second_samples),
    ] {
        let read = as_another_instance(|| samples(&relaunch(session)));
        assert_eq!(read.len(), 2, "the session restores its run");
        assert!(read.iter().all(|values| values.len() == 3));
        assert_eq!(read, expected, "each session reads its own run's samples");
    }
}

/// Closing waits while the session's results go into its folder off the frame, and goes on once
/// they are there; the session it then stores reads them.
#[test]
fn closing_waits_until_the_session_results_are_in_its_folder() {
    use rspice_app_services::io::result_segments::{
        SessionResultsAtClose, place_by_copy_for_tests, session_results_at_close,
    };
    as_another_instance(|| {
        place_by_copy_for_tests(true);
        let mut state = session_with_a_run(2.0);
        assert!(
            state.hold_close_for_session_results(),
            "the results are not in the session's folder yet"
        );
        assert!(state.shell.closing_for_session_results);
        let until = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while session_results_at_close() != SessionResultsAtClose::Kept {
            assert!(std::time::Instant::now() < until, "the copy ends");
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        assert!(
            !state.hold_close_for_session_results(),
            "once they are there, closing goes on"
        );
        place_by_copy_for_tests(false);
        let expected = samples(&state);
        let session = saved_session(&mut state);
        drop(state);
        assert_eq!(
            as_another_instance(|| samples(&relaunch(session))),
            expected
        );
    });
}

/// Closing without waiting for the copy stores the session without its runs, and leaves
/// nothing of the copy.
#[test]
fn closing_without_waiting_stores_the_session_without_its_runs() {
    use rspice_app_services::io::result_segments::{
        close_without_session_results, hold_placements_for_tests, place_by_copy_for_tests,
    };
    as_another_instance(|| {
        place_by_copy_for_tests(true);
        hold_placements_for_tests(true);
        let mut state = session_with_a_run(3.0);
        assert!(state.hold_close_for_session_results());
        close_without_session_results();
        hold_placements_for_tests(false);
        place_by_copy_for_tests(false);
        let session = saved_session(&mut state);
        drop(state);
        assert!(as_another_instance(|| samples(&relaunch(session))).is_empty());
    });
}
