//! The job runner's contract: progress, a stated failure, and a cancel that
//! drops what the work made, on its own thread.

use std::time::Duration;

use super::*;

const PATIENCE: Duration = Duration::from_secs(30);

#[test]
fn a_job_states_its_step_and_progress_and_hands_back_its_value() {
    let (step_set, step_seen) = std::sync::mpsc::channel::<()>();
    let (go_on, wait_here) = std::sync::mpsc::channel::<()>();
    let job = ResultsJob::start("job-test", move |context| {
        context.step("Reading amp.raw", 400);
        context.done(100);
        step_set.send(()).expect("the test watches");
        wait_here.recv().expect("the test lets the job go on");
        context.done(400);
        Ok(42_u32)
    });
    step_seen.recv_timeout(PATIENCE).expect("the job starts");
    let state = job.state();
    assert_eq!(
        state,
        JobState::Running {
            step: "Reading amp.raw".to_owned(),
            done: 100,
            total: 400,
        }
    );
    assert_eq!(state.fraction(), Some(0.25));
    assert_eq!(job.take_outcome(), None, "a running job has no outcome");
    go_on.send(()).expect("the job waits");
    assert!(job.wait(PATIENCE));
    assert_eq!(job.state(), JobState::Finished);
    assert_eq!(job.take_outcome(), Some(JobOutcome::Done(42)));
    assert_eq!(job.state(), JobState::Taken);
    assert_eq!(job.take_outcome(), None, "an outcome is taken once");
}

#[test]
fn a_failed_job_states_why() {
    let job: ResultsJob<()> = ResultsJob::start("job-test", |_| {
        Err(JobStop::from("the file ends inside its header"))
    });
    assert!(job.wait(PATIENCE));
    assert_eq!(
        job.take_outcome(),
        Some(JobOutcome::Failed(
            "the file ends inside its header".to_owned()
        ))
    );
}

/// The work sees the cancel at its next check, and the job ends cancelled whatever the work
/// returned after it, so a value the reader cancelled is never handed back.
#[test]
fn a_cancelled_job_ends_cancelled_and_drops_its_value() {
    struct Dropped(std::sync::mpsc::Sender<()>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    let (dropped, was_dropped) = std::sync::mpsc::channel();
    let (started, has_started) = std::sync::mpsc::channel::<()>();
    let job = ResultsJob::start("job-test", move |context| {
        started.send(()).expect("the test watches");
        while !context.cancel_flag().load(Ordering::Acquire) {
            std::thread::sleep(Duration::from_millis(1));
        }
        // Work that ignores the cancel still has its value dropped.
        Ok(Dropped(dropped))
    });
    has_started.recv_timeout(PATIENCE).expect("the job starts");
    job.cancel();
    assert!(job.wait(PATIENCE));
    assert!(matches!(job.take_outcome(), Some(JobOutcome::Cancelled)));
    was_dropped
        .recv_timeout(PATIENCE)
        .expect("the cancelled value is dropped");
}

#[test]
fn a_held_job_waits_until_released_and_a_cancel_ends_it_unworked() {
    hold_jobs_for_tests(true);
    let worked = Arc::new(AtomicBool::new(false));
    let held = {
        let worked = Arc::clone(&worked);
        ResultsJob::start("job-test", move |_| {
            worked.store(true, Ordering::Release);
            Ok(())
        })
    };
    let cancelled = ResultsJob::<()>::start("job-test", |_| panic!("a cancelled job never works"));
    hold_jobs_for_tests(false);
    std::thread::sleep(Duration::from_millis(20));
    assert!(matches!(held.state(), JobState::Running { .. }));
    assert!(!worked.load(Ordering::Acquire));

    cancelled.cancel();
    assert!(cancelled.wait(PATIENCE));
    assert_eq!(cancelled.take_outcome(), Some(JobOutcome::Cancelled));

    held.release();
    assert!(held.wait(PATIENCE));
    assert!(worked.load(Ordering::Acquire));
    assert_eq!(held.take_outcome(), Some(JobOutcome::Done(())));
}
