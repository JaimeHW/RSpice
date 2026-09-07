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

/// The first accepted `$finish` request in one simulation run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelFinish {
    pub instance: String,
    pub model: String,
    /// Model-local analog site identity, not a source line number.
    pub site: u32,
    pub point: ModelFinishPoint,
    pub diagnostic_level: u8,
}

impl std::fmt::Display for ModelFinish {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "model '{}' instance '{}' requested $finish at site {} during ",
            self.model, self.instance, self.site
        )?;
        match self.point {
            ModelFinishPoint::Initialization => formatter.write_str("initialization"),
            ModelFinishPoint::OperatingPoint => formatter.write_str("the operating point"),
            ModelFinishPoint::Transient { time } => write!(formatter, "transient time {time}"),
            ModelFinishPoint::DcSweep { value } => write!(formatter, "DC sweep value {value}"),
            ModelFinishPoint::Frequency { frequency } => {
                write!(formatter, "frequency {frequency} Hz")
            }
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

    pub(crate) fn request_finish(&self, finish: ModelFinish) {
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
pub(crate) struct ModelRunSignal<'a> {
    parent: &'a dyn crate::AbortSignal,
    control: ModelRunControl,
}

impl<'a> ModelRunSignal<'a> {
    pub(crate) fn if_needed(parent: &'a dyn crate::AbortSignal) -> Option<Self> {
        parent.model_control().is_none().then(|| Self::new(parent))
    }

    pub(crate) fn new(parent: &'a dyn crate::AbortSignal) -> Self {
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
