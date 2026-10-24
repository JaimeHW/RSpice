//! Timing checks (IEEE 1364-2005 section 15).
//!
//! Each check keeps the tick of its last reference and data events. A
//! round's events are taken in three passes, so two at one tick meet in one
//! order whatever order the round drained them in: each reference event
//! first checks the data's last event, then every event records its tick,
//! then each data event checks the reference's last event. OVI Verilog LRM
//! 1.0 B.9 has `$setup` and `$recovery` check before they record a
//! simultaneous event, and `$hold` after.
//!
//! With `rel` a data event's tick less its reference event's, a check of a
//! stability window is violated when:
//!
//! - `$setup`: `-limit < rel < 0`; `$hold`: `0 <= rel < limit`;
//! - `$removal`: `-limit < rel <= 0`; `$recovery`: `0 < rel < limit`;
//! - `$setuphold` and `$recrem`: `rel` falls in the union of their pair's
//!   windows. A negative limit narrows the window from its side, and the
//!   closed end lies where the delayed signals of section 15.8 meet.
//!
//! The other checks:
//!
//! - `$skew` is violated when `rel > limit`.
//! - `$timeskew` and `$fullskew` are violated when their timecheck event
//!   follows their timestamp event by more than the limit, reported when
//!   the limit elapses or, event-based, at the late event (section 15.3;
//!   see `skew_step`).
//! - `$period` is violated when a reference edge follows the last by less
//!   than the limit.
//! - `$width` is violated when the opposite edge follows the reference edge
//!   by more than the threshold and less than the limit.
//! - `$nochange` is violated when a data event falls strictly inside
//!   `(lead - start, trail + end)`, `lead` the reference edge and `trail`
//!   its opposite.
//!
//! Pending the IEEE text: `$removal`'s closed end is inferred by symmetry
//! (it complements `$recovery` as `$hold` complements `$setup`), and
//! `$nochange`'s open window is an interpretation.
//!
//! An event counts on its edges of the least significant bit the terminal
//! names, or on any change of the bits it names when it names no edge: the
//! whole signal, or a bit- or part-select's bits (A.7.5.3). It counts only while
//! its `&&&` condition holds (section 15.6).
//!
//! `$setuphold` and `$recrem` take a timestamp and a timecheck condition
//! (sections 15.2.3, 15.2.6 and 15.5.2). Of their pair of checks, the data
//! event is the timestamp event of `$setup` and `$removal` and the reference
//! event their timecheck event; `$hold` and `$recovery` take them the other
//! way round. Here that is: an event records its tick only while the
//! timestamp condition holds, and checks the other event's last tick only
//! while the timecheck condition does. Like `&&&`, each is read when the
//! event occurs.
//!
//! A violation writes a warning record and toggles the check's notifier
//! (section 15.5): `x` to 0, 0 to 1, 1 to 0, and `z` stays `z`.

use super::*;
use rspice_veriloga::canonical_ir::digital::{
    DigitalCheckCondition, DigitalCheckEvent, DigitalTimingCheckKind as Kind,
};
use rspice_veriloga::four_state::FourStateBit;
use serde::{Deserialize, Serialize};

/// One check, its limits in the plan's ticks.
#[derive(Debug)]
struct Check {
    kind: Kind,
    reference: DigitalCheckEvent,
    /// The data event: for `$width` the reference's opposite edge; `None`
    /// for `$period`.
    data: Option<DigitalCheckEvent>,
    /// `$nochange`'s reference's opposite edge, which ends its window.
    trailing: Option<DigitalCheckEvent>,
    /// The stability window of `$setup`, `$hold`, `$setuphold`,
    /// `$removal`, `$recovery` and `$recrem`.
    window: Option<Window>,
    limits: [i64; 2],
    /// `$setuphold`'s and `$recrem`'s timestamp and timecheck conditions.
    timestamp: Option<DigitalCheckCondition>,
    timecheck: Option<DigitalCheckCondition>,
    notifier: Option<DigitalSignalId>,
    /// `$timeskew`'s and `$fullskew`'s flags.
    event_based: bool,
    remain_active: bool,
    label: String,
}

impl Check {
    /// A `$timeskew` or `$fullskew` that reports when its limit elapses.
    fn timed(&self) -> bool {
        matches!(self.kind, Kind::TimeSkew | Kind::FullSkew) && !self.event_based
    }

    /// The tick at which an active timer-based skew check that `state`
    /// stamped is violated: the first at which the time since the stamp
    /// exceeds the limit its other event must meet.
    fn deadline(&self, state: &CheckState) -> Option<u64> {
        if !self.timed() || !state.active {
            return None;
        }
        let [after_reference, after_data] = self.limits;
        let (stamp, limit) = match (state.reference, state.data) {
            (Some(stamp), _) => (stamp, after_reference),
            (None, Some(stamp)) => (stamp, after_data),
            (None, None) => return None,
        };
        Some(
            stamp
                .saturating_add(u64::try_from(limit).unwrap_or(0))
                .saturating_add(1),
        )
    }
}

/// A stability window over `rel`, a data event's tick less its reference
/// event's: open on `(-before, after)`, and closed at `junction` when
/// `closed`.
#[derive(Debug, Clone, Copy)]
struct Window {
    before: i64,
    after: i64,
    junction: i64,
    closed: bool,
}

impl Window {
    /// The window of a check with `limits`, its delayed reference and data
    /// delayed by `delays`.
    fn of(kind: Kind, [first, second]: [i64; 2], [reference, data]: [i64; 2]) -> Option<Self> {
        // Which side of the pair closes the window at the junction: `$hold`
        // counts a simultaneous event, `$setup` does not; `$removal` counts
        // one, `$recovery` does not.
        let (before, after, after_closes) = match kind {
            Kind::Setup => (first, 0, true),
            Kind::Hold => (0, first, true),
            Kind::SetupHold => (first, second, true),
            Kind::Removal => (first, 0, false),
            Kind::Recovery => (0, first, false),
            Kind::RecRem => (second, first, false),
            _ => return None,
        };
        // Sections 15.5.1 and 15.8: the delayed signals' delays leave both
        // limits non-negative between them, and the two delayed events meet
        // at `junction`. A delay another check sharing the signal sets can
        // move it inside the window, which then is open at both ends.
        let junction = reference.saturating_sub(data);
        let closed = if after_closes {
            after.saturating_sub(junction) > 0
        } else {
            before.saturating_add(junction) > 0
        };
        Some(Self {
            before,
            after,
            junction,
            closed,
        })
    }

    fn holds(self, rel: i64) -> bool {
        (self.before.saturating_neg() < rel && rel < self.after)
            || (self.closed && rel == self.junction)
    }
}

#[derive(Debug)]
struct CheckTables {
    checks: Vec<Check>,
    by_signal: BTreeMap<DigitalSignalId, Vec<usize>>,
    /// The timer-based `$timeskew` and `$fullskew` checks, by index.
    timed: Vec<usize>,
}

/// One check's last event ticks, as the host image's `specify` section
/// stores them.
///
/// A `$timeskew` or `$fullskew` keeps only its timestamp event: in
/// `reference` or, a `$fullskew` whose data event came first, in `data`;
/// `active` while it waits for the other event.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CheckState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reference: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data: Option<u64>,
    /// `$nochange`'s last opposite reference edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trailing: Option<u64>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    active: bool,
}

/// The plan's timing checks and each one's last event ticks.
#[derive(Debug, Clone)]
pub(super) struct TimingChecks {
    tables: Arc<CheckTables>,
    states: Vec<CheckState>,
}

/// A violation: the warning text and the notifier to toggle.
pub(super) struct Violation {
    pub(super) message: String,
    pub(super) notifier: Option<DigitalSignalId>,
}

/// The passes over one round's events.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pass {
    /// Reference events check the data's last event.
    Reference,
    /// Every event records its tick.
    Stamp,
    /// Data events check the reference's last event.
    Data,
}

/// Which of a check's events one transition is, and whether its timestamp
/// and timecheck conditions hold.
#[derive(Clone, Copy)]
struct Events {
    reference: bool,
    data: bool,
    trailing: bool,
    stamps: bool,
    checks: bool,
}

impl TimingChecks {
    /// The plan's checks; `None` for a plan with none.
    pub(super) fn for_plan(plan: &CanonicalDigitalPlan) -> Option<Self> {
        if plan.delays.checks.is_empty() {
            return None;
        }
        let precision = plan.timing.precision_exponent;
        let mut tables = CheckTables {
            checks: Vec::with_capacity(plan.delays.checks.len()),
            by_signal: BTreeMap::new(),
            timed: Vec::new(),
        };
        for (index, check) in plan.delays.checks.iter().enumerate() {
            let scale = u32::try_from(i32::from(check.exponent) - i32::from(precision))
                .ok()
                .and_then(|finer| 10_i64.checked_pow(finer))
                .unwrap_or(i64::MAX);
            let opposite = |event: DigitalCheckEvent| DigitalCheckEvent {
                edges: event.edges.map(reversed),
                ..event
            };
            let data = match check.kind {
                Kind::Width => Some(opposite(check.reference)),
                Kind::Period => None,
                _ => check.data,
            };
            for signal in
                std::iter::once(check.reference.signal).chain(data.map(|data| data.signal))
            {
                let entry = tables.by_signal.entry(signal).or_default();
                if entry.last() != Some(&index) {
                    entry.push(index);
                }
            }
            let limits = check.limits.map(|limit| limit.saturating_mul(scale));
            let delays = check.delays.map(|delay| delay.saturating_mul(scale));
            tables.checks.push(Check {
                kind: check.kind,
                reference: check.reference,
                data,
                trailing: (check.kind == Kind::NoChange).then(|| opposite(check.reference)),
                window: Window::of(check.kind, limits, delays),
                limits,
                timestamp: check.timestamp_condition,
                timecheck: check.timecheck_condition,
                notifier: check.notifier,
                event_based: check.event_based,
                remain_active: check.remain_active,
                label: check.label.clone(),
            });
            if tables.checks[index].timed() {
                tables.timed.push(index);
            }
        }
        Some(Self {
            states: vec![CheckState::default(); tables.checks.len()],
            tables: Arc::new(tables),
        })
    }

    /// The wakeup of a plan's timer-based `$timeskew` and `$fullskew`
    /// checks; `None` for a plan with none, which interns no wakeup.
    pub(super) fn timer_for_plan(
        plan: &CanonicalDigitalPlan,
        scheduler: &mut EventScheduler,
    ) -> Option<TargetId> {
        plan.delays
            .checks
            .iter()
            .any(|check| {
                matches!(check.kind, Kind::TimeSkew | Kind::FullSkew) && !check.event_based
            })
            .then(|| {
                scheduler.intern_target(EventTarget {
                    node_id: usize::MAX,
                    port_name: "timing check limits".to_string(),
                    driver_index: 0,
                    instance: "specify".to_string(),
                })
            })
    }

    /// The warnings a run starts with: each check whose negative limit the
    /// plan took as 0 (sections 15.5.1 and 15.8).
    pub(super) fn start_warnings(plan: &CanonicalDigitalPlan) -> Vec<String> {
        let mut warnings = Vec::new();
        for check in &plan.delays.checks {
            if check.limit_zeroed {
                warnings.push(format!(
                    "{}: its limits sum to no more than 0, so its negative limit is taken as 0 \
                     (IEEE 1364-2005 section 15.8)",
                    check.label
                ));
            }
            if check.limits_inconsistent {
                warnings.push(format!(
                    "{}: no delays of its delayed signals satisfy its limits and those of the \
                     checks sharing them, so the smallest negative limit among them is taken as \
                     0 (IEEE 1364-2005 section 15.5.1)",
                    check.label
                ));
            }
        }
        warnings
    }

    /// Take a round's transitions at `tick`, and return the violations they
    /// make, in check order within each pass.
    pub(super) fn note(
        &mut self,
        store: &DigitalSignalStore,
        drained: &[SignalTransition],
        tick: u64,
        precision: i8,
    ) -> Vec<Violation> {
        // A timer due by now has elapsed before this round's events, so the
        // order the host takes the timer's wakeup and the round in does not
        // matter.
        let mut violations = self.expire(tick, precision);
        // The `$fullskew` checks whose reference event this round was a
        // timecheck event, which therefore stamps nothing.
        let mut timechecked = BTreeSet::new();
        for pass in [Pass::Reference, Pass::Stamp, Pass::Data] {
            for transition in drained {
                let Some(checks) = self.tables.by_signal.get(&transition.signal) else {
                    continue;
                };
                for &index in checks {
                    let check = &self.tables.checks[index];
                    if matches!(check.kind, Kind::TimeSkew | Kind::FullSkew) {
                        let events = Events {
                            reference: counts(&check.reference, transition, store),
                            data: check
                                .data
                                .as_ref()
                                .is_some_and(|event| counts(event, transition, store)),
                            trailing: false,
                            stamps: true,
                            checks: true,
                        };
                        if skew_step(
                            check,
                            &mut self.states[index],
                            events,
                            tick,
                            pass,
                            &mut timechecked,
                            index,
                        ) {
                            violations.push(Violation {
                                message: format!(
                                    "{} violated at {}",
                                    check.label,
                                    time_text(tick, precision)
                                ),
                                notifier: check.notifier,
                            });
                        }
                        continue;
                    }
                    let events = Events {
                        reference: counts(&check.reference, transition, store),
                        data: check
                            .data
                            .as_ref()
                            .is_some_and(|event| counts(event, transition, store)),
                        trailing: check
                            .trailing
                            .as_ref()
                            .is_some_and(|event| counts(event, transition, store)),
                        stamps: holds(check.timestamp.as_ref(), store),
                        checks: holds(check.timecheck.as_ref(), store),
                    };
                    if step(check, &mut self.states[index], events, tick, pass) {
                        violations.push(Violation {
                            message: format!(
                                "{} violated at {}",
                                check.label,
                                time_text(tick, precision)
                            ),
                            notifier: check.notifier,
                        });
                    }
                }
            }
        }
        violations
    }

    /// Each check's last event ticks, in plan order.
    pub(super) fn image(&self) -> &[CheckState] {
        &self.states
    }

    /// Install the event ticks a host image's `specify` section names, one
    /// state per check.
    pub(super) fn restore(&mut self, states: &[CheckState]) -> Result<(), String> {
        if states.len() != self.states.len() {
            return Err("host specify section names other than one state per timing check".into());
        }
        // Only a `$timeskew` or `$fullskew` is ever active, on one stamp.
        for (check, state) in self.tables.checks.iter().zip(states) {
            let skew = matches!(check.kind, Kind::TimeSkew | Kind::FullSkew);
            if (state.active && (!skew || state.reference.is_none() == state.data.is_none()))
                || (skew && state.reference.is_some() && state.data.is_some())
            {
                return Err("host specify section holds a skew state no check can reach".into());
            }
        }
        self.states.clone_from_slice(states);
        Ok(())
    }

    /// The earliest tick at which a timer-based `$timeskew` or `$fullskew`
    /// is violated, which the host keeps a wakeup at.
    pub(super) fn next_deadline(&self) -> Option<u64> {
        self.tables
            .timed
            .iter()
            .filter_map(|&index| self.tables.checks[index].deadline(&self.states[index]))
            .min()
    }

    /// The violations of every timer-based `$timeskew` and `$fullskew` whose
    /// limit has elapsed by `tick` without its timecheck event, each at the
    /// tick it was violated; each then goes dormant until its next timestamp
    /// event (IEEE 1364-2005 section 15.3).
    pub(super) fn expire(&mut self, tick: u64, precision: i8) -> Vec<Violation> {
        let mut violations = Vec::new();
        for &index in &self.tables.timed {
            let check = &self.tables.checks[index];
            let state = &mut self.states[index];
            let Some(deadline) = check.deadline(state).filter(|deadline| *deadline <= tick) else {
                continue;
            };
            state.active = false;
            violations.push(Violation {
                message: format!(
                    "{} violated at {}",
                    check.label,
                    time_text(deadline, precision)
                ),
                notifier: check.notifier,
            });
        }
        violations
    }
}

/// One `$timeskew` or `$fullskew` response to its events at `tick` in one
/// pass; whether it is violated (IEEE 1364-2005 section 15.3).
///
/// The timestamp event starts the check, which then waits, active, for its
/// timecheck event; the check is violated when the timecheck event follows
/// the timestamp event by more than the limit, so never by simultaneous
/// events. `$timeskew`'s reference event is its timestamp event and its
/// data event its timecheck event, under its one limit. `$fullskew` takes
/// either order: the first of the two is the timestamp event, and the data
/// event must follow the reference event within the first limit, the
/// reference event the data event within the second. A timestamp event
/// that comes again restarts the check.
///
/// By default the check is timer-based: the violation is reported as the
/// limit elapses ([`TimingChecks::expire`]), at the first tick past it, and
/// a timecheck event within the limit ends the check. With the event-based
/// flag the violation is reported at the late timecheck event, and the
/// check stays active through timecheck events within the limit; after a
/// violation it goes dormant until its next timestamp event unless the
/// remain-active flag is set.
///
/// The passes keep two events at one tick in one order: a `$fullskew`
/// reference event first checks a data timestamp, then events stamp, then
/// data events check a reference timestamp.
fn skew_step(
    check: &Check,
    state: &mut CheckState,
    events: Events,
    tick: u64,
    pass: Pass,
    timechecked: &mut BTreeSet<usize>,
    index: usize,
) -> bool {
    let full = check.kind == Kind::FullSkew;
    let [after_reference, after_data] = check.limits;
    let timecheck = |state: &mut CheckState, stamp: u64, limit: i64| {
        let elapsed = i64::try_from(tick.saturating_sub(stamp)).unwrap_or(i64::MAX);
        if elapsed > limit {
            state.active = check.event_based && check.remain_active;
            true
        } else {
            if !check.event_based {
                state.active = false;
            }
            false
        }
    };
    match pass {
        Pass::Reference if full && events.reference => {
            let Some(stamp) = state.data.filter(|_| state.active) else {
                return false;
            };
            timechecked.insert(index);
            timecheck(state, stamp, after_data)
        }
        Pass::Stamp => {
            if events.reference && !timechecked.contains(&index) {
                state.reference = Some(tick);
                state.data = None;
                state.active = true;
            }
            if full && events.data && !(state.active && state.reference.is_some()) {
                state.data = Some(tick);
                state.reference = None;
                state.active = true;
            }
            false
        }
        Pass::Data if events.data => {
            let Some(stamp) = state.reference.filter(|_| state.active) else {
                return false;
            };
            timecheck(state, stamp, after_reference)
        }
        Pass::Reference | Pass::Data => false,
    }
}

/// An edge mask's opposite: each transition in table 14-3's order (01, 10,
/// 0z, z1, 1z, z0, 0x, x1, 1x, x0, xz, zx) taken the other way, so
/// `posedge` becomes `negedge`.
fn reversed(edges: u16) -> u16 {
    const PARTNER: [u16; 12] = [1, 0, 5, 4, 3, 2, 9, 8, 7, 6, 11, 10];
    PARTNER
        .iter()
        .enumerate()
        .filter(|(index, _)| edges & (1 << index) != 0)
        .fold(0, |mask, (_, partner)| mask | (1 << partner))
}

/// Whether `transition` is `event`, its condition holding.
fn counts(
    event: &DigitalCheckEvent,
    transition: &SignalTransition,
    store: &DigitalSignalStore,
) -> bool {
    if event.signal != transition.signal {
        return false;
    }
    let TransitionValues::FourState { previous, next } = &transition.values else {
        return false;
    };
    // A.7.5.3: a select's event is a change of its bits, and an edge of the
    // least significant of them.
    let low = event.bits.map_or(0, |bits| bits.low);
    let changed = match (event.edges, event.bits) {
        (None, None) => previous != next,
        (None, Some(bits)) => (bits.low..bits.low.saturating_add(bits.width))
            .any(|bit| previous.bit(bit) != next.bit(bit)),
        (Some(edges), _) => {
            let (from, to) = (previous.bit(low), next.bit(low));
            from != to && edges & (1 << super::paths::transition_index(from, to)) != 0
        }
    };
    changed && holds(event.condition.as_ref(), store)
}

/// Whether `condition` holds, or there is none. Section 15.6 and OVI Verilog
/// LRM 1.0 B.9.10: `x` or `z` on the conditioning signal enables the check
/// only through `==` or `!=`.
fn holds(condition: Option<&DigitalCheckCondition>, store: &DigitalSignalStore) -> bool {
    condition.is_none_or(|condition| {
        match store.value(condition.signal).map(|value| value.bit(0)) {
            Some(FourStateBit::Zero) => !condition.enabling,
            Some(FourStateBit::One) => condition.enabling,
            _ => condition.unknown_enables,
        }
    })
}

/// One check's response to its events at `tick` in one pass; whether it is
/// violated.
fn step(check: &Check, state: &mut CheckState, events: Events, tick: u64, pass: Pass) -> bool {
    let since = |earlier: Option<u64>| {
        earlier.map(|at| i64::try_from(tick.saturating_sub(at)).unwrap_or(i64::MAX))
    };
    let [limit, second] = check.limits;
    match pass {
        Pass::Reference if events.reference && events.checks => match check.kind {
            Kind::Period => since(state.reference).is_some_and(|gap| gap < limit),
            Kind::NoChange => since(state.data).is_some_and(|gap| gap < limit),
            _ => check
                .window
                .is_some_and(|window| since(state.data).is_some_and(|gap| window.holds(-gap))),
        },
        Pass::Stamp if events.stamps => {
            if events.reference {
                state.reference = Some(tick);
            }
            if events.data {
                state.data = Some(tick);
            }
            if events.trailing {
                state.trailing = Some(tick);
            }
            false
        }
        Pass::Data if events.data && events.checks => match check.kind {
            Kind::Skew => since(state.reference).is_some_and(|gap| gap > limit),
            Kind::Width => {
                since(state.reference.take()).is_some_and(|width| second < width && width < limit)
            }
            Kind::NoChange => {
                let Some(lead) = state.reference else {
                    return false;
                };
                if state.trailing.is_none_or(|trail| trail < lead) {
                    since(Some(lead)).is_some_and(|gap| gap > limit.saturating_neg())
                } else {
                    since(state.trailing).is_some_and(|gap| gap < second)
                }
            }
            _ => check
                .window
                .is_some_and(|window| since(state.reference).is_some_and(|gap| window.holds(gap))),
        },
        Pass::Reference | Pass::Stamp | Pass::Data => false,
    }
}

/// `tick` in the largest SI unit that counts it whole: `25 ns`.
fn time_text(tick: u64, precision: i8) -> String {
    if tick == 0 {
        return "0 s".into();
    }
    for (exponent, unit) in [
        (0_i8, "s"),
        (-3, "ms"),
        (-6, "us"),
        (-9, "ns"),
        (-12, "ps"),
        (-15, "fs"),
    ] {
        let scale = 10_u64.saturating_pow(u32::from(exponent.abs_diff(precision)));
        if exponent < precision {
            return format!("{} {unit}", tick.saturating_mul(scale));
        }
        if tick.is_multiple_of(scale) {
            return format!("{} {unit}", tick / scale);
        }
    }
    format!("{tick} fs")
}
