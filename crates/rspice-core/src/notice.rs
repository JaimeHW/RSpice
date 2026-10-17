//! What a run raised and went on to complete with.
//!
//! The engine says these things in its log, as prose, for whoever is watching
//! the run. A notice is the same event in a shape a reader can act on: the
//! element, the parameter, the value asked for and the value used. The host
//! that runs an analysis installs a [`NoticeCollector`] on the solving thread;
//! where none is installed, recording a notice costs one thread-local read.
//!
//! Notices are deduplicated as they are recorded. A shared model card is
//! clamped once per bind, but a sweep or a Monte Carlo run binds it once per
//! point, and the reader needs the clamp once with the points it occurred at,
//! not ten thousand times.

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::ContinuationAid;

/// The sweep points one notice lists by name. Past this many, it counts.
const MAX_NAMED_POINTS: usize = 8;

/// One advisory a run raised.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EngineNotice {
    /// A parameter outside the range its model can evaluate, replaced by a
    /// value it can.
    ParameterReplaced(ParameterReplacement),
    /// An operating-point convergence aid reached a solution.
    ConvergenceAid(ContinuationAid),
    /// The transient stepper was held at its minimum timestep by an element's
    /// scheduled activations.
    TimestepFloor(TimestepFloor),
}

/// A parameter the model replaced, and why.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParameterReplacement {
    /// The model family that made the replacement, as it names itself: `BSIM4`.
    pub family: String,
    /// The model card the element was bound to, as the deck names it.
    pub model: Option<String>,
    /// The element being bound when the replacement was made, as the engine
    /// names it (`XAMP.M1`). For a [`ParameterScope::Model`] replacement it
    /// is the first element bound to the card, which is where the card is
    /// evaluated; every other element of the card shares the value.
    pub element: Option<String>,
    pub scope: ParameterScope,
    /// The parameter, lowercase, as the model card spells it.
    pub parameter: String,
    /// The value the card or the instance gave, after temperature and size
    /// scaling where the model applies them first.
    pub requested: f64,
    /// The value the model evaluates instead.
    pub applied: f64,
    /// The bound `requested` crossed, where the model states one.
    pub limit: Option<f64>,
}

/// Whose value a replaced parameter is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ParameterScope {
    /// The model card's, shared by every element bound to it.
    Model,
    /// One element's, after its own geometry and instance parameters.
    Instance,
}

/// The transient stepper held at its minimum timestep.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimestepFloor {
    /// The element whose activations held the step, as the engine names it,
    /// where the schedule names one.
    pub element: Option<String>,
    /// The minimum timestep, in seconds.
    pub floor: f64,
    /// When the step was first reported held there, in seconds.
    pub time: f64,
}

impl EngineNotice {
    /// What makes two notices the same notice: the kind, the object it is
    /// about and, for a replacement, the parameter. A model-card replacement
    /// is about the card, so the element that happened to bind it first does
    /// not split it.
    fn same_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParameterReplaced(a), Self::ParameterReplaced(b)) => {
                a.family == b.family
                    && a.scope == b.scope
                    && a.parameter == b.parameter
                    && match a.scope {
                        ParameterScope::Model => a.model == b.model,
                        ParameterScope::Instance => a.element == b.element,
                    }
            }
            (Self::ConvergenceAid(a), Self::ConvergenceAid(b)) => a == b,
            (Self::TimestepFloor(a), Self::TimestepFloor(b)) => a.element == b.element,
            _ => false,
        }
    }
}

/// One notice, as first raised, and every time the run raised it again.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordedNotice {
    pub notice: EngineNotice,
    /// How many times the run raised it.
    pub occurrences: usize,
    /// The sweep points it was raised at, in the order first raised, where
    /// the run names its points. At most a handful are named.
    pub points: Vec<String>,
    /// Further distinct points it was raised at, beyond those named.
    pub unnamed_points: usize,
}

impl RecordedNotice {
    fn count_at(&mut self, point: Option<&str>) {
        self.occurrences = self.occurrences.saturating_add(1);
        let Some(point) = point else {
            return;
        };
        if self.points.iter().any(|named| named == point) {
            return;
        }
        if self.points.len() < MAX_NAMED_POINTS {
            self.points.push(point.to_owned());
        } else {
            self.unnamed_points = self.unnamed_points.saturating_add(1);
        }
    }
}

/// Where one run's notices are recorded.
///
/// Shared, because a run may solve on more than one thread: a Monte Carlo run
/// evaluates its trials on workers, and each worker installs the run's
/// collector before it binds anything.
#[derive(Debug, Clone, Default)]
pub struct NoticeCollector(Arc<Mutex<Vec<RecordedNotice>>>);

impl NoticeCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record on this thread until the guard is dropped.
    #[must_use]
    pub fn install(&self) -> CollectorGuard {
        let previous = ACTIVE
            .try_with(|active| active.borrow_mut().replace(self.clone()))
            .unwrap_or(None);
        CollectorGuard {
            previous,
            _thread_bound: std::marker::PhantomData,
        }
    }

    /// The collector recording on this thread, for a thread the run spawns.
    pub fn current() -> Option<Self> {
        ACTIVE
            .try_with(|active| active.borrow().clone())
            .ok()
            .flatten()
    }

    /// Everything recorded so far, in the order first raised, leaving the
    /// collector empty.
    pub fn take(&self) -> Vec<RecordedNotice> {
        std::mem::take(&mut *self.lock())
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<RecordedNotice>> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn record(&self, notice: EngineNotice, point: Option<&str>) {
        let mut ledger = self.lock();
        if let Some(recorded) = ledger
            .iter_mut()
            .find(|recorded| recorded.notice.same_as(&notice))
        {
            recorded.count_at(point);
            return;
        }
        let mut recorded = RecordedNotice {
            notice,
            occurrences: 0,
            points: Vec::new(),
            unnamed_points: 0,
        };
        recorded.count_at(point);
        ledger.push(recorded);
    }
}

/// Restores the collector a thread had before [`NoticeCollector::install`].
pub struct CollectorGuard {
    previous: Option<NoticeCollector>,
    // Dropping a guard restores thread-local state, so it must stay on its thread.
    _thread_bound: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl Drop for CollectorGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        let _ = ACTIVE.try_with(|active| *active.borrow_mut() = previous);
    }
}

/// The element being bound on this thread and the card it is bound to.
#[derive(Debug, Clone, Default)]
pub(crate) struct Binding {
    element: Option<String>,
    model: Option<String>,
}

thread_local! {
    static ACTIVE: RefCell<Option<NoticeCollector>> = const { RefCell::new(None) };
    static BINDING: RefCell<Binding> = RefCell::new(Binding::default());
    static POINT: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Restores the thread's previous binding or point when dropped.
pub(crate) struct ContextGuard<T: 'static> {
    key: &'static std::thread::LocalKey<RefCell<T>>,
    previous: Option<T>,
    _thread_bound: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl<T: 'static> Drop for ContextGuard<T> {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.take() {
            let _ = self.key.try_with(|cell| *cell.borrow_mut() = previous);
        }
    }
}

fn replace_context<T: 'static>(
    key: &'static std::thread::LocalKey<RefCell<T>>,
    value: T,
) -> ContextGuard<T> {
    let previous = key
        .try_with(|cell| std::mem::replace(&mut *cell.borrow_mut(), value))
        .ok();
    ContextGuard {
        key,
        previous,
        _thread_bound: std::marker::PhantomData,
    }
}

/// Name the element being bound, and its model card, for the replacements
/// its model makes while the guard lives.
pub(crate) fn binding(element: &str, model: Option<&str>) -> ContextGuard<Binding> {
    replace_context(
        &BINDING,
        Binding {
            element: Some(element.to_owned()),
            model: model.map(str::to_owned),
        },
    )
}

/// Name the sweep point being solved, for every notice raised while the
/// guard lives.
pub(crate) fn point(label: String) -> ContextGuard<Option<String>> {
    replace_context(&POINT, Some(label))
}

/// Record `notice` with the collector on this thread, if there is one.
pub(crate) fn record(notice: EngineNotice) {
    let Some(collector) = NoticeCollector::current() else {
        return;
    };
    let point = POINT
        .try_with(|point| point.borrow().clone())
        .ok()
        .flatten();
    collector.record(notice, point.as_deref());
}

/// Record that `family` replaced `parameter` of the element or card being
/// bound on this thread.
pub(crate) fn parameter_replaced(
    family: &str,
    scope: ParameterScope,
    parameter: &str,
    requested: f64,
    applied: f64,
    limit: Option<f64>,
) {
    if NoticeCollector::current().is_none() {
        return;
    }
    let binding = BINDING
        .try_with(|binding| binding.borrow().clone())
        .unwrap_or_default();
    record(EngineNotice::ParameterReplaced(ParameterReplacement {
        family: family.to_owned(),
        model: binding.model,
        element: binding.element,
        scope,
        parameter: parameter.to_ascii_lowercase(),
        requested,
        applied,
        limit,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clamp(parameter: &str) {
        parameter_replaced(
            "BSIM4",
            ParameterScope::Model,
            parameter,
            0.05,
            0.1,
            Some(0.1),
        );
    }

    /// A card clamped at every point of a sweep is one notice that names the
    /// points, and an element of the same card does not split it.
    #[test]
    fn a_repeated_notice_is_counted_with_the_points_it_was_raised_at() {
        let collector = NoticeCollector::new();
        let _installed = collector.install();
        for index in 0..12 {
            let _point = point(format!("trial {}", index + 1));
            let _binding = binding(if index == 0 { "M1" } else { "M2" }, Some("nch"));
            clamp("PBS");
        }
        {
            let _binding = binding("M1", Some("pch"));
            clamp("pbs");
        }

        let recorded = collector.take();
        assert_eq!(recorded.len(), 2, "{recorded:#?}");
        assert_eq!(recorded[0].occurrences, 12);
        assert_eq!(recorded[0].points.len(), MAX_NAMED_POINTS);
        assert_eq!(recorded[0].points[0], "trial 1");
        assert_eq!(recorded[0].unnamed_points, 12 - MAX_NAMED_POINTS);
        let EngineNotice::ParameterReplaced(first) = &recorded[0].notice else {
            panic!("a replacement")
        };
        assert_eq!(first.parameter, "pbs");
        assert_eq!(first.element.as_deref(), Some("M1"));
        assert_eq!(recorded[1].occurrences, 1);
        assert!(collector.take().is_empty(), "taking empties the collector");
    }

    /// A worker thread the run spawns records into the run's collector once
    /// it installs it, and the guard puts the thread back as it found it.
    #[test]
    fn a_worker_records_into_the_collector_it_installs() {
        let collector = NoticeCollector::new();
        let shared = {
            let _installed = collector.install();
            NoticeCollector::current().expect("installed on this thread")
        };
        assert!(NoticeCollector::current().is_none(), "the guard restores");
        std::thread::spawn(move || {
            let _installed = shared.install();
            record(EngineNotice::ConvergenceAid(ContinuationAid::GminStepping));
        })
        .join()
        .expect("the worker completes");
        assert_eq!(collector.take().len(), 1);
    }
}
