//! Progress belongs to the whole sensitivity study, including result projection.

use super::SimulationError;
use crate::abort_signal::{AbortReason, AbortSignal, ModelRunControl, TransientSample};
use std::sync::Mutex;

pub(super) struct StudyProgress<'a> {
    parent: &'a dyn AbortSignal,
    last: Mutex<f64>,
}

impl<'a> StudyProgress<'a> {
    pub(super) fn new(parent: &'a dyn AbortSignal) -> Result<Self, SimulationError> {
        if parent.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(Self {
            parent,
            last: Mutex::new(0.0),
        })
    }

    pub(super) fn stage(&self, start: f64, end: f64) -> StudyStage<'_, 'a> {
        debug_assert!(0.0 <= start && start <= end && end < 1.0);
        StudyStage {
            study: self,
            start,
            end,
        }
    }

    fn publish(&self, fraction: f64) {
        // Serialize delivery as well as the stored maximum: an atomic maximum
        // alone can deliver callbacks out of order when workers finish together.
        let mut last = self
            .last
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if fraction > *last {
            *last = fraction;
            self.parent.observe_progress(fraction);
        }
    }

    pub(super) fn report(&self, fraction: f64) -> Result<(), SimulationError> {
        if self.parent.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.publish(fraction);
        if self.parent.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(())
    }

    pub(super) fn complete<T>(&self, result: T) -> Result<T, SimulationError> {
        self.report(1.0)?;
        Ok(result)
    }
}

pub(super) struct StudyStage<'scope, 'parent> {
    study: &'scope StudyProgress<'parent>,
    start: f64,
    end: f64,
}

impl AbortSignal for StudyStage<'_, '_> {
    fn is_aborted(&self) -> bool {
        self.study.parent.is_aborted()
    }
    fn abort_reason(&self) -> AbortReason {
        self.study.parent.abort_reason()
    }
    fn model_control(&self) -> Option<&ModelRunControl> {
        self.study.parent.model_control()
    }
    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        self.study.parent.observe_transient_sample(sample);
    }
    fn observe_progress(&self, fraction: f64) {
        if fraction.is_finite() {
            self.study.publish(
                (self.start + fraction.clamp(0.0, 1.0) * (self.end - self.start)).min(self.end),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[derive(Default)]
    struct Observer {
        values: Mutex<Vec<f64>>,
        samples: AtomicUsize,
        stopped: AtomicBool,
        control: ModelRunControl,
    }
    impl AbortSignal for Observer {
        fn is_aborted(&self) -> bool {
            self.stopped.load(Ordering::Relaxed)
        }
        fn abort_reason(&self) -> AbortReason {
            AbortReason::TimeLimit
        }
        fn model_control(&self) -> Option<&ModelRunControl> {
            Some(&self.control)
        }
        fn observe_progress(&self, fraction: f64) {
            self.values.lock().unwrap().push(fraction);
        }
        fn observe_transient_sample(&self, sample: TransientSample<'_>) {
            assert_eq!(sample.time, &[0.5]);
            self.samples.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn nested_compiler_completions_preserve_outer_progress_and_signal_identity() {
        let observer = Observer::default();
        let progress = StudyProgress::new(&observer).unwrap();
        let stage = progress.stage(0.1, 0.4);
        assert!(std::ptr::eq(
            stage.model_control().unwrap(),
            &observer.control
        ));
        assert_eq!(stage.abort_reason(), AbortReason::TimeLimit);
        stage.observe_transient_sample(TransientSample {
            time: &[0.5],
            node_names: &[],
            node_voltages: &[],
            branch_names: &[],
            branch_currents: &[],
            current_impulses: None,
            digital_values: &[],
            digital_buses: &[],
            real_values: &[],
        });
        assert_eq!(observer.samples.load(Ordering::Relaxed), 1);
        for fraction in [0.0, 0.5, 1.0, 0.0, 0.25, f64::NAN, 1.0] {
            stage.observe_progress(fraction);
        }
        assert_eq!(*observer.values.lock().unwrap(), vec![0.1, 0.25, 0.4]);
        progress.complete(()).unwrap();
        assert_eq!(*observer.values.lock().unwrap(), vec![0.1, 0.25, 0.4, 1.0]);
        observer.stopped.store(true, Ordering::Relaxed);
        assert!(stage.is_aborted());
        assert!(matches!(
            progress.report(1.0),
            Err(SimulationError::Aborted)
        ));
    }
}
