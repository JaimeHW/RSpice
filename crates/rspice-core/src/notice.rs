//! What a run raised and went on to complete with.
//!
//! The engine says these things in its log, as prose, for whoever is watching
//! the run. A notice is the same event in a shape a reader can act on: the
//! element, the parameter, the value asked for and the value used. The host
//! that runs an analysis keeps a [`NoticeCollector`] in the run's
//! [`log_scope`](crate::log_scope), which the engine enters on every thread it
//! hands part of the run to; where no run collects, recording a notice costs
//! one thread-local read.
//!
//! Notices are deduplicated as they are recorded. A shared model card is
//! clamped once per bind, but a sweep or a Monte Carlo run binds it once per
//! point, and the reader needs the clamp once with the points it occurred at,
//! not ten thousand times. The point is the scope's, the label the run's log
//! lines carry: `sample 12`.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, hash_map};
use std::sync::{Arc, Mutex};

use crate::ContinuationAid;
use crate::log_scope::{self, LogScope, LogScopeGuard};

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

/// What makes two notices the same notice: the kind, the object it is about
/// and, for a replacement, the parameter. A model-card replacement is about
/// the card, so the element that happened to bind it first does not split it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NoticeKey {
    ParameterReplaced {
        family: String,
        scope: ParameterScope,
        parameter: String,
        /// The card of a model-scope replacement, the element of an
        /// instance-scope one.
        subject: Option<String>,
    },
    ConvergenceAid(ContinuationAid),
    TimestepFloor {
        element: Option<String>,
    },
}

impl EngineNotice {
    fn key(&self) -> NoticeKey {
        match self {
            Self::ParameterReplaced(replacement) => NoticeKey::ParameterReplaced {
                family: replacement.family.clone(),
                scope: replacement.scope,
                parameter: replacement.parameter.clone(),
                subject: match replacement.scope {
                    ParameterScope::Model => replacement.model.clone(),
                    ParameterScope::Instance => replacement.element.clone(),
                },
            },
            Self::ConvergenceAid(aid) => NoticeKey::ConvergenceAid(*aid),
            Self::TimestepFloor(floor) => NoticeKey::TimestepFloor {
                element: floor.element.clone(),
            },
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

/// One notice as the collector keeps it while the run raises it.
#[derive(Debug)]
struct Entry {
    recorded: RecordedNotice,
    /// The points past the named ones it was raised at, so a point that
    /// raises it twice, as a sample solved in two analyses does, is counted
    /// once.
    unnamed: HashSet<String>,
}

impl Entry {
    fn count_at(&mut self, point: Option<&str>) {
        let recorded = &mut self.recorded;
        recorded.occurrences = recorded.occurrences.saturating_add(1);
        let Some(point) = point else {
            return;
        };
        if recorded.points.iter().any(|named| named == point) {
            return;
        }
        if recorded.points.len() < MAX_NAMED_POINTS {
            recorded.points.push(point.to_owned());
        } else if self.unnamed.insert(point.to_owned()) {
            recorded.unnamed_points = self.unnamed.len();
        }
    }
}

/// The notices one run has raised so far.
#[derive(Debug, Default)]
struct Ledger {
    /// In the order first raised.
    entries: Vec<Entry>,
    /// Where each notice sits in `entries`, so raising it again costs a
    /// lookup, not a scan of every notice a large design has raised.
    index: HashMap<NoticeKey, usize>,
}

/// Where one run's notices are recorded.
///
/// Shared, because a run may solve on more than one thread: a Monte Carlo run
/// evaluates its samples on workers, and a parallel sweep its chunks, each
/// inside the run's log scope.
#[derive(Debug, Clone, Default)]
pub struct NoticeCollector(Arc<Mutex<Ledger>>);

impl NoticeCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record on this thread, and on every thread it hands work to, until
    /// the guard is dropped.
    ///
    /// Into the run this thread is executing; on a thread outside any run,
    /// into a scope of the collector's own. A host that keeps the run's log
    /// scope itself sets the collector there with [`LogScope::collecting`].
    #[must_use]
    pub fn install(&self) -> LogScopeGuard {
        let scope = log_scope::capture().unwrap_or_else(|| LogScope::new(Arc::new(())));
        log_scope::enter(Some(scope.collecting(self.clone())))
    }

    /// Everything recorded so far, in the order first raised, leaving the
    /// collector empty.
    pub fn take(&self) -> Vec<RecordedNotice> {
        std::mem::take(&mut *self.lock())
            .entries
            .into_iter()
            .map(|entry| entry.recorded)
            .collect()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Ledger> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn record(&self, notice: EngineNotice, point: Option<&str>) {
        let mut ledger = self.lock();
        let Ledger { entries, index } = &mut *ledger;
        match index.entry(notice.key()) {
            hash_map::Entry::Occupied(at) => entries[*at.get()].count_at(point),
            hash_map::Entry::Vacant(slot) => {
                slot.insert(entries.len());
                let mut entry = Entry {
                    recorded: RecordedNotice {
                        notice,
                        occurrences: 0,
                        points: Vec::new(),
                        unnamed_points: 0,
                    },
                    unnamed: HashSet::new(),
                };
                entry.count_at(point);
                entries.push(entry);
            }
        }
    }
}

/// The element being bound on this thread and the card it is bound to.
#[derive(Debug, Clone, Default)]
pub(crate) struct Binding {
    element: Option<String>,
    model: Option<String>,
}

thread_local! {
    static BINDING: RefCell<Binding> = RefCell::new(Binding::default());
}

/// Restores the thread's previous binding when dropped.
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

/// Whether the run on this thread records its notices.
fn collecting() -> bool {
    log_scope::with_current(|scope| scope.notices().is_some()).unwrap_or(false)
}

/// Record `notice` with the collector of the run on this thread, if it has
/// one, at the point of the run this thread is solving.
pub(crate) fn record(notice: EngineNotice) {
    let _ = log_scope::with_current(|scope| {
        if let Some(collector) = scope.notices() {
            collector.record(notice, scope.point());
        }
    });
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
    if !collecting() {
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
            let _point = log_scope::enter_point(|| format!("sample {}", index + 1));
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
        assert_eq!(recorded[0].points[0], "sample 1");
        assert_eq!(recorded[0].unnamed_points, 12 - MAX_NAMED_POINTS);
        let EngineNotice::ParameterReplaced(first) = &recorded[0].notice else {
            panic!("a replacement")
        };
        assert_eq!(first.parameter, "pbs");
        assert_eq!(first.element.as_deref(), Some("M1"));
        assert_eq!(recorded[1].occurrences, 1);
        assert!(recorded[1].points.is_empty(), "raised outside any point");
        assert!(collector.take().is_empty(), "taking empties the collector");
    }

    /// A point that raises a notice again — a sample solved in two analyses
    /// binds its card twice — is one point, named or not.
    #[test]
    fn a_point_that_raises_a_notice_twice_is_counted_once() {
        let collector = NoticeCollector::new();
        let _installed = collector.install();
        for _ in 0..2 {
            for index in 0..12 {
                let _point = log_scope::enter_point(|| format!("sample {}", index + 1));
                let _binding = binding("M1", Some("nch"));
                clamp("pbs");
            }
        }
        let recorded = collector.take();
        assert_eq!(recorded.len(), 1, "{recorded:#?}");
        assert_eq!(recorded[0].occurrences, 24);
        assert_eq!(recorded[0].points.len(), MAX_NAMED_POINTS);
        assert_eq!(recorded[0].unnamed_points, 12 - MAX_NAMED_POINTS);
    }

    /// A clamp on every element of a large design, raised again at a second
    /// point, is one notice per element, in the order first raised, each
    /// with both points, however many the collector already holds.
    #[test]
    fn many_notices_are_each_recorded_once_in_the_order_first_raised() {
        const ELEMENTS: usize = 20_000;
        let collector = NoticeCollector::new();
        let _installed = collector.install();
        for point in ["sample 1", "sample 2"] {
            let _point = log_scope::enter_point(|| point.to_owned());
            for element in 0..ELEMENTS {
                let _binding = binding(&format!("M{element}"), Some("nch"));
                parameter_replaced(
                    "BSIM4",
                    ParameterScope::Instance,
                    "pseff",
                    -1.0,
                    0.0,
                    Some(0.0),
                );
            }
        }

        let recorded = collector.take();
        assert_eq!(recorded.len(), ELEMENTS);
        for (element, notice) in recorded.iter().enumerate() {
            let EngineNotice::ParameterReplaced(replacement) = &notice.notice else {
                panic!("a replacement")
            };
            assert_eq!(
                replacement.element.as_deref(),
                Some(format!("M{element}").as_str())
            );
            assert_eq!(notice.occurrences, 2);
            assert_eq!(notice.points, ["sample 1", "sample 2"]);
        }
    }

    /// A worker the run hands work to records into the run's collector once
    /// it enters the run's scope, and the guard puts the thread back as it
    /// found it.
    #[test]
    fn a_worker_records_into_the_collector_of_the_scope_it_enters() {
        let collector = NoticeCollector::new();
        let captured = {
            let _installed = collector.install();
            log_scope::capture()
        };
        assert!(!collecting(), "the guard restores");
        std::thread::scope(|threads| {
            threads.spawn(|| {
                record(EngineNotice::ConvergenceAid(
                    ContinuationAid::SourceStepping,
                ));
                let _run = log_scope::enter(captured.clone());
                let _sample = log_scope::enter_point(|| "sample 3".to_owned());
                record(EngineNotice::ConvergenceAid(ContinuationAid::GminStepping));
            });
        });
        let recorded = collector.take();
        assert_eq!(
            recorded.len(),
            1,
            "only the line inside the run: {recorded:#?}"
        );
        assert_eq!(recorded[0].points, ["sample 3"]);
    }

    /// Installed on a thread already executing a run, the collector joins
    /// that run's scope rather than replacing it.
    #[test]
    fn installing_inside_a_run_keeps_the_run() {
        let _run = log_scope::enter(Some(LogScope::new(Arc::new("run"))));
        let collector = NoticeCollector::new();
        let _installed = collector.install();
        let owner =
            log_scope::capture().and_then(|scope| scope.owner().downcast_ref::<&str>().copied());
        assert_eq!(owner, Some("run"));
        record(EngineNotice::ConvergenceAid(ContinuationAid::GminStepping));
        assert_eq!(collector.take().len(), 1);
    }
}
