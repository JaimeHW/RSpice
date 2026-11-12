//! Model-requested completion carried by a scoped execution signal.

use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

/// The public analysis point that accepted a model's finish request.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ModelFinishPoint {
    Initialization,
    OperatingPoint,
    Transient { time: f64 },
    DcSweep { value: f64 },
    Frequency { frequency: f64 },
}

/// The system task that ended a run: `$finish` or `$stop` (IEEE 1364-2005
/// section 17.4.1), or `$fatal` (IEEE 1800-2017 section 20.10). A batch run
/// treats `$stop` as `$finish`, as `vvp -n` does; `$fatal` ends it failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelFinishTask {
    Finish,
    Stop,
    Fatal,
}

impl ModelFinishTask {
    /// The task as its call is written.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Finish => "$finish",
            Self::Stop => "$stop",
            Self::Fatal => "$fatal",
        }
    }
}

/// The file and one-based line a finishing call was written on.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModelFinishSource {
    pub file: String,
    pub line: u32,
}

/// The first accepted `$finish`, `$stop` or `$fatal` request in one
/// simulation run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelFinish {
    pub instance: String,
    pub model: String,
    /// Model-local analog site identity, not a source line number.
    pub site: u32,
    pub point: ModelFinishPoint,
    pub diagnostic_level: u8,
    /// The task that ended the run.
    pub task: ModelFinishTask,
    /// Where the call was written, when the model knows.
    pub source: Option<ModelFinishSource>,
    /// `$fatal`'s formatted message; `None` for `$finish` and `$stop`.
    pub message: Option<String>,
}

impl ModelFinish {
    /// Whether the task that ended the run failed it: a `$fatal`.
    pub fn failed(&self) -> bool {
        self.task == ModelFinishTask::Fatal
    }
}

impl std::fmt::Display for ModelFinish {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "model '{}' instance '{}' requested {} ",
            self.model,
            self.instance,
            self.task.name()
        )?;
        match &self.source {
            Some(source) => write!(formatter, "at {}:{}", source.file, source.line)?,
            None => write!(formatter, "at site {}", self.site)?,
        }
        formatter.write_str(" during ")?;
        match self.point {
            ModelFinishPoint::Initialization => formatter.write_str("initialization")?,
            ModelFinishPoint::OperatingPoint => formatter.write_str("the operating point")?,
            ModelFinishPoint::Transient { time } => write!(formatter, "transient time {time}")?,
            ModelFinishPoint::DcSweep { value } => write!(formatter, "DC sweep value {value}")?,
            ModelFinishPoint::Frequency { frequency } => {
                write!(formatter, "frequency {frequency} Hz")?
            }
        }
        match &self.message {
            Some(message) => write!(formatter, ": {message}"),
            None => Ok(()),
        }
    }
}

/// Successful execution can end at the requested endpoint or at an accepted
/// model request. Initialization may finish before any numerical result exists.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SimulationOutcome<T> {
    Completed(T),
    Finished {
        result: Option<T>,
        finish: ModelFinish,
    },
}

/// A run's shared model control. Analysis workers borrow this through their
/// run signal; independently started runs never share it through an Engine.
#[derive(Debug, Default)]
pub struct ModelRunControl {
    finished: AtomicBool,
    finish: Mutex<Option<ModelFinish>>,
    /// The run's output sink has been told who finished it and when.
    reported: AtomicBool,
}

impl ModelRunControl {
    pub fn is_finished(&self) -> bool {
        self.finished.load(Ordering::Acquire)
    }

    pub fn finish(&self) -> Option<ModelFinish> {
        self.finish
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    /// Claim the one report of the accepted finish; later callers get `false`.
    #[doc(hidden)]
    pub fn claim_finish_report(&self) -> bool {
        self.is_finished() && !self.reported.swap(true, Ordering::AcqRel)
    }

    #[doc(hidden)]
    pub fn request_finish(&self, finish: ModelFinish) {
        let mut first = self
            .finish
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if first.is_none() {
            *first = Some(finish);
            self.finished.store(true, Ordering::Release);
        }
    }
}

/// A borrowed scope keeps model completion separate from the caller's abort
/// flag, while preserving every observation and cancellation hook.
#[doc(hidden)]
pub struct ModelRunSignal<'a> {
    parent: &'a dyn crate::AbortSignal,
    control: ModelRunControl,
}

impl<'a> ModelRunSignal<'a> {
    #[doc(hidden)]
    pub fn if_needed(parent: &'a dyn crate::AbortSignal) -> Option<Self> {
        parent.model_control().is_none().then(|| Self::new(parent))
    }

    #[doc(hidden)]
    pub fn new(parent: &'a dyn crate::AbortSignal) -> Self {
        Self {
            parent,
            control: ModelRunControl::default(),
        }
    }
}

impl crate::AbortSignal for ModelRunSignal<'_> {
    fn is_aborted(&self) -> bool {
        self.parent.is_aborted()
    }
    fn abort_reason(&self) -> crate::AbortReason {
        self.parent.abort_reason()
    }
    fn model_control(&self) -> Option<&ModelRunControl> {
        Some(&self.control)
    }
    fn observe_progress(&self, fraction: f64) {
        self.parent.observe_progress(fraction);
    }
    fn observe_transient_sample(&self, sample: crate::abort_signal::TransientSample<'_>) {
        self.parent.observe_transient_sample(sample);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AbortSignal, AtomicAbort, NoAbort};
    use std::sync::Arc;

    #[test]
    fn borrowed_boxed_and_shared_signals_keep_control_and_cancellation_separate() {
        let parent = AtomicAbort::new();
        let scoped = ModelRunSignal::new(&parent);
        let boxed: Box<dyn AbortSignal + '_> = Box::new(&scoped);
        let shared: Arc<dyn AbortSignal + '_> = Arc::new(boxed);
        let control = shared
            .model_control()
            .expect("all wrappers preserve the run scope");
        let finish = ModelFinish {
            instance: "X1".into(),
            model: "finish".into(),
            site: 7,
            point: ModelFinishPoint::Initialization,
            diagnostic_level: 0,
            task: ModelFinishTask::Finish,
            source: None,
            message: None,
        };
        control.request_finish(finish.clone());
        assert!(control.is_finished());
        assert_eq!(scoped.model_control().unwrap().finish(), Some(finish));
        assert!(!shared.is_aborted());
        assert!(!parent.is_aborted());
        parent.set();
        assert!(
            shared.is_aborted(),
            "user cancellation must still reach the scoped worker"
        );
    }

    #[test]
    fn independent_run_scopes_do_not_share_the_first_finish_request() {
        let first = ModelRunSignal::new(&NoAbort);
        let second = ModelRunSignal::new(&NoAbort);
        let request = ModelFinish {
            instance: "first".into(),
            model: "m".into(),
            site: 1,
            point: ModelFinishPoint::OperatingPoint,
            diagnostic_level: 1,
            task: ModelFinishTask::Finish,
            source: None,
            message: None,
        };
        first.control.request_finish(request.clone());
        first.control.request_finish(ModelFinish {
            instance: "later".into(),
            ..request.clone()
        });
        assert_eq!(first.control.finish(), Some(request));
        assert!(!second.control.is_finished());
        assert!(second.control.finish().is_none());
        assert!(
            ModelRunSignal::if_needed(&first).is_none(),
            "nested work must inherit its run"
        );
    }
}
