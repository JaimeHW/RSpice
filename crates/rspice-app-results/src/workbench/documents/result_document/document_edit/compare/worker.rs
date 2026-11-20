//! Where a comparison runs. It reads every sample of the signals it compares,
//! so it runs as a Results job, on a thread of its own, while the form shows
//! how far it has got, and it keeps nothing once it answers: the samples it
//! read go with it. On the web, which has no threads, it runs where it is
//! asked.
//!
//! A [`Worker`] is the whole of the arrangement: the comparison goes in when
//! it starts, its progress and answer are the job's, and dropping it stops the
//! comparison.

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::waveform::RetainedWaveform;
use rspice_results::waveform_comparison::{
    ComparisonAlignmentDraft, ComparisonOptions, ComparisonOutcome, ComparisonSide,
    execute_comparison,
};

use crate::product::DatasetBinding;
use crate::workbench::results_job::{JobOutcome, JobState, JobStop, ResultsJob};

/// One run's side of a comparison: the waveforms it compares, sharing their
/// samples with the run.
pub(crate) struct Side {
    pub binding: DatasetBinding,
    pub analysis: AnalysisResult<RetainedWaveform>,
}

/// A comparison a worker runs, owned whole.
pub(crate) struct Job {
    pub candidate: Side,
    pub baseline: Side,
    pub signals: Vec<String>,
    pub alignment: ComparisonAlignmentDraft,
    pub alignment_signal: String,
    pub threshold: f64,
    pub maximum_lag_samples: u32,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

impl Side {
    fn compared(&self) -> ComparisonSide<'_, RetainedWaveform> {
        ComparisonSide {
            binding: self.binding,
            analysis: &self.analysis,
        }
    }
}

impl Job {
    fn run(
        &self,
        progress: &mut dyn FnMut(usize, usize) -> bool,
    ) -> Result<ComparisonOutcome, String> {
        execute_comparison(
            self.candidate.compared(),
            self.baseline.compared(),
            &self.signals,
            ComparisonOptions {
                alignment: self.alignment,
                alignment_signal: &self.alignment_signal,
                threshold: self.threshold,
                maximum_lag_samples: self.maximum_lag_samples,
                absolute_tolerance: self.absolute_tolerance,
                relative_tolerance: self.relative_tolerance,
            },
            progress,
        )
    }
}

/// What a comparison answers: its outcome, boxed so a job's value stays small,
/// or why it has none.
pub(crate) type Answer = Result<Box<ComparisonOutcome>, String>;

/// Where a running comparison has got.
pub(crate) enum Poll {
    /// Signals compared, of how many.
    Running(usize, usize),
    Done(Answer),
}

/// A comparison running as a Results job, or answered and not yet collected.
/// Dropping it stops the comparison.
#[derive(Debug)]
pub(crate) struct Worker {
    job: ResultsJob<Box<ComparisonOutcome>>,
    /// The signals it compares: its work before it has stated any.
    signals: usize,
}

impl Worker {
    /// Start `job`.
    pub(crate) fn start(job: Job) -> Self {
        let signals = job.signals.len();
        let total = signals as u64;
        let job = ResultsJob::start("results-comparison", move |context| {
            context.step("Comparing signals", total);
            let mut known = total;
            let mut progress = |done: usize, total: usize| {
                if total as u64 != known {
                    known = total as u64;
                    context.step("Comparing signals", known);
                }
                context.done(done as u64);
                !context.cancelled()
            };
            job.run(&mut progress)
                .map(Box::new)
                .map_err(JobStop::Failed)
        });
        Self { job, signals }
    }

    /// Where the comparison has got, or its answer once it has one.
    pub(crate) fn poll(&self) -> Poll {
        match self.job.state() {
            JobState::Running { done, total: 0, .. } => Poll::Running(done as usize, self.signals),
            JobState::Running { done, total, .. } => Poll::Running(done as usize, total as usize),
            JobState::Finished | JobState::Taken => Poll::Done(self.answer()),
        }
    }

    /// The answer of a comparison that ended.
    fn answer(&self) -> Answer {
        match self.job.take_outcome() {
            Some(JobOutcome::Done(outcome)) => Ok(outcome),
            Some(JobOutcome::Failed(reason)) => Err(reason),
            Some(JobOutcome::Cancelled) => Err("The comparison was stopped.".to_owned()),
            None => Err("The comparison ended without an answer.".to_owned()),
        }
    }

    /// Wait for the answer.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn wait(&self) -> Answer {
        while !self.job.wait(std::time::Duration::from_secs(60)) {}
        self.answer()
    }

    /// Let a comparison held by `hold_jobs_for_tests` work.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn release(&self) {
        self.job.release();
    }

    /// Ask the comparison to stop after the signal it is comparing.
    pub(crate) fn stop(&self) {
        self.job.cancel();
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}
