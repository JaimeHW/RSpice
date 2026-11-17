use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use rspice_results_store::EventValue;

use super::*;
use crate::events::{DigitalEventPointEvidence, RealEventPointEvidence};

/// A stored real event signal in memory that counts the changes it hands out.
struct Signal {
    changes: Vec<EventChange>,
    handed: AtomicUsize,
    fail: Mutex<bool>,
}

impl Signal {
    fn real(points: &[(f64, f64)]) -> Arc<Self> {
        let changes = points
            .iter()
            .enumerate()
            .map(|(sequence, &(time, value))| EventChange {
                signal: 1,
                time,
                sequence: sequence as u32,
                value: EventValue::Real(value),
                strength: 0,
            })
            .collect();
        Arc::new(Self {
            changes,
            handed: AtomicUsize::new(0),
            fail: Mutex::new(false),
        })
    }

    fn check(&self) -> Result<(), String> {
        if *self.fail.lock().unwrap() {
            return Err("the frame's checksum does not match".to_owned());
        }
        Ok(())
    }
}

impl EventHistorySource for Signal {
    fn name(&self) -> &str {
        "e(points_test)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn count(&self) -> Result<u64, String> {
        self.check()?;
        Ok(self.changes.len() as u64)
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        self.check()?;
        Ok(self.changes.iter().filter(|change| change.time < t).count() as u64)
    }

    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String> {
        self.check()?;
        let span = EventSpan {
            before: self.changes.iter().rev().find(|c| c.time < t0).cloned(),
            within: self
                .changes
                .iter()
                .filter(|c| c.time >= t0 && c.time <= t1)
                .cloned()
                .collect(),
            after: self.changes.iter().find(|c| c.time > t1).cloned(),
        };
        let handed = span.within.len()
            + usize::from(span.before.is_some())
            + usize::from(span.after.is_some());
        self.handed.fetch_add(handed, Ordering::SeqCst);
        Ok(span)
    }
}

fn real(points: &[(f64, f64)]) -> Vec<RealEventPointEvidence> {
    points
        .iter()
        .map(|&(time_s, value)| RealEventPointEvidence { time_s, value })
        .collect()
}

#[test]
fn a_stored_window_reads_what_the_points_in_memory_give_and_keeps_nothing() {
    // Changes at repeated instants, as delta cycles commit them, and runs of changes that
    // keep the value.
    let raw = [
        (0.0, 1.0),
        (1.0, 2.0),
        (1.0, 3.0),
        (2.5, 3.0),
        (3.0, 3.0),
        (4.0, 5.0),
        (4.0, 6.0),
        (5.0, 6.0),
        (7.0, 7.0),
        (8.0, 7.0),
        (8.0, 7.0),
        (9.0, 1.0),
    ];
    let same = |a: &RealEventPointEvidence, b: &RealEventPointEvidence| a.value == b.value;
    let windows = [
        (-1.0, -0.5),
        (0.0, 0.0),
        (0.5, 0.75),
        (1.0, 1.0),
        (1.5, 4.0),
        (4.0, 3.0),
        (6.0, 9.0),
        (8.0, 9.0),
        (3.5, 3.75),
        (5.5, 6.0),
        (7.5, 8.5),
        (f64::NEG_INFINITY, f64::INFINITY),
    ];
    for from in [f64::NEG_INFINITY, 0.0, 1.0, 3.0, 8.0] {
        let signal = Signal::real(&raw);
        let kept: Vec<_> = raw.iter().copied().filter(|(t, _)| *t >= from).collect();
        let memory = EventPoints::from(real(&kept));
        let stored = EventPoints::<RealEventPointEvidence>::stored(signal.clone(), from).unwrap();
        assert_eq!(stored.len(), kept.len(), "from {from}");
        assert_eq!(signal.handed.load(Ordering::SeqCst), 0, "len reads nothing");
        for (t0, t1) in windows {
            assert_eq!(
                stored.around(t0, t1, same).unwrap(),
                memory.around(t0, t1, same).unwrap(),
                "from {from} over {t0} ..= {t1}"
            );
        }
        // The windows kept nothing: the whole history is read on first use, once.
        assert_eq!(*stored, *memory);
        let handed = signal.handed.load(Ordering::SeqCst);
        assert_eq!(stored.try_points().unwrap(), &*memory);
        assert_eq!(
            stored.around(1.5, 4.0, same).unwrap(),
            memory.around(1.5, 4.0, same).unwrap()
        );
        assert_eq!(signal.handed.load(Ordering::SeqCst), handed);
    }
}

#[test]
fn points_in_memory_around_a_window_hold_the_one_before_and_the_one_after() {
    let points = EventPoints::from(real(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)]));
    let times = |t0, t1| {
        points
            .around(t0, t1, |_, _| false)
            .unwrap()
            .iter()
            .map(|point| point.time_s)
            .collect::<Vec<_>>()
    };
    assert_eq!(times(2.5, 2.75), vec![2.0, 3.0]);
    assert_eq!(times(2.0, 3.0), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(times(0.0, 0.5), vec![1.0]);
    assert_eq!(times(5.0, 6.0), vec![4.0]);
    assert!(
        EventPoints::<RealEventPointEvidence>::default()
            .around(0.0, 1.0, |_, _| false)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn a_window_reaches_back_to_where_its_held_value_was_set_and_on_to_where_it_next_changes() {
    let points = EventPoints::from(real(&[
        (1.0, 1.0),
        (2.0, 2.0),
        (3.0, 2.0),
        (4.0, 2.0),
        (5.0, 2.0),
        (6.0, 3.0),
    ]));
    let same = |a: &RealEventPointEvidence, b: &RealEventPointEvidence| a.value == b.value;
    let times = |t0, t1| {
        points
            .around(t0, t1, same)
            .unwrap()
            .iter()
            .map(|point| point.time_s)
            .collect::<Vec<_>>()
    };
    // 2 is held from t = 2 into a window at 4.5, and next changes at 6.
    assert_eq!(times(4.5, 4.75), vec![2.0, 6.0]);
    assert_eq!(times(2.5, 3.0), vec![2.0, 3.0, 6.0]);
    assert_eq!(times(0.5, 1.5), vec![1.0, 2.0]);
}

#[test]
fn a_stored_history_maps_to_another_point_kind_and_an_unreadable_one_shows_no_points() {
    let signal = Signal::real(&[(0.0, 1.0), (1.0, 2.0)]);
    let stored = EventPoints::<RealEventPointEvidence>::stored(signal.clone(), 0.0).unwrap();
    let mapped = stored
        .clone()
        .map(|point| crate::simulation_values::RealEventPoint {
            time_s: point.time_s,
            value: point.value,
        });
    assert!(mapped.is_stored());
    assert_eq!(mapped.len(), 2);
    assert_eq!(mapped[1].value, 2.0);
    // A digital history cannot hold a real change: reading one fails, naming the node.
    let digital = EventPoints::<DigitalEventPointEvidence>::stored(signal.clone(), 0.0).unwrap();
    assert_eq!(digital.try_points().unwrap_err().column, "e(points_test)");
    assert_eq!((digital.len(), digital.iter().count()), (2, 0));

    *signal.fail.lock().unwrap() = true;
    let failed = stored.around(0.0, 1.0, |_, _| false).unwrap_err();
    assert_eq!(failed.column, "e(points_test)");
    assert!(stored.iter().next().is_none());
    assert_eq!(stored.try_points().unwrap_err(), failed);
}
