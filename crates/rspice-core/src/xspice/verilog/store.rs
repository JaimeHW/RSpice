//! The live half of a running digital design: one value per signal, one
//! contribution per driver, and the resolution between them.
//!
//! This is the [`DigitalEnvironment`] the canonical-IR process interpreter
//! runs against. The interpreter owns the *plan* — widths, bodies, driver
//! identities — and this owns everything that changes while a simulation runs.
//!
//! # Why a contribution per driver rather than a value per net
//!
//! IEEE 1364-2005 section 7.9 makes a net's value a function of every driver
//! on it, not of the last one to write. Two `assign` statements on one net, or
//! two instances driving one net through collapsed output ports, are two
//! contributions that must both be visible to the resolution at all times: a
//! store that wrote a drive into the net would resolve by last-write-wins and
//! would do so silently, which is the exact failure
//! [`DigitalEnvironment::drive_signal`] exists to prevent.
//!
//! So [`DigitalSignalStore`] sizes its contribution array from
//! [`CanonicalDigitalPlan::drivers`] at construction, before any process has
//! run, and every contribution starts `z` — section 7.9's value for a driver
//! that is not driving. A single-driver net therefore already resolves
//! correctly before its `assign` has ever evaluated, and keeps resolving
//! correctly when a second driver appears.
//!
//! # The resolution
//!
//! [`RESOLUTION_TABLE`] is IEEE 1364-2005 table 4-1, the `wire`/`tri` column,
//! transcribed rather than derived. `z` is its identity, which is what lets one
//! fold serve both the single- and multi-driver cases without a special case
//! that could disagree with the general one.
//!
//! Resolution is per bit, not per net: `assign bus[3:0] = ...` on an eight-bit
//! net drives four of its bits and contributes `z` to the other four, so two
//! partial drivers of one net compose rather than fight.
//!
//! # The other resolution
//!
//! A real net (Verilog-AMS LRM 2.4 section 3.7) resolves here too, beside
//! table 4-1 and for the same reason: which value a net takes from several
//! drivers is a rule over the whole net, and the whole net is what this owns.
//! What it resolves *with* is the compiler's, recorded on the signal as
//! [`DigitalRealResolution`] — the net-type keyword its author wrote.
//!
//! The rules differ from table 4-1's in one structural way, and it is worth
//! naming. Table 4-1 has an identity (`z`), so a `wire`'s fold starts from
//! all-`z` and a driver that has not run contributes nothing. A real fold has
//! no such value: section 3.7 says "if no driver is connected to a `wreal`
//! net, its value shall be zero (0.0)" and "unlike other digital nets which
//! have an initial value of `z`, `wreal` nets shall have an initial value of
//! zero". Zero is not an identity for `min`, for `max`, or for the average, so
//! the fold is over the contributions that exist rather than seeded with one —
//! and a net with no contributions at all answers `0.0` by that clause
//! directly, not by folding an empty set.
//!
//! A driver that has not run yet contributes `0.0`, which is the same clause
//! read the other way: a `wreal` "shall not store its value", so before a
//! driver has produced one there is nothing but the initial value to have.
//!
//! `wreal` itself never reaches the fold with more than one contribution:
//! section 6.5.3 permits one driver of a real-valued net, and the front end
//! refuses a second before a plan is built. The four resolved spellings are the
//! ones with something to combine.
//!
//! # What is *not* resolved
//!
//! A `reg`. IEEE 1364-2005 section 4.2.2 makes a variable a store with exactly
//! one writer at a time and no notion of a driver, so
//! [`DigitalEnvironment::write_signal`] stores what it is given. The plan says
//! which is which in [`DigitalSignal::procedurally_assignable`], and the store
//! refuses the mixed case rather than picking one reading — see
//! [`DigitalSignalStore::force`].
//!
//! # Transitions
//!
//! Every value change is recorded as a [`SignalTransition`] carrying the
//! previous value, because that is what IEEE 1364-2005 table 5-2's edge
//! classification needs and the interpreter deliberately does not report. The
//! host drains the queue after each write; a rewrite of the value a signal
//! already holds produces no transition, which is what stops a level-sensitive
//! `@*` process from re-triggering itself forever.

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalDriverId, DigitalRealResolution, DigitalSchedulingRegion,
    DigitalSignal, DigitalSignalKind, DigitalWriteSelect,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalDeferredUpdate, DigitalDrive, DigitalEnvironment, DigitalRealDrive,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;

/// IEEE 1364-2005 table 4-1: the value one `wire` bit takes from two drivers.
///
/// Rows are the accumulated value, columns the incoming contribution, both in
/// the `0 1 x z` order [`bit_index`] fixes. Transcribed from the standard as a
/// table for the same reason section 4.1's operator tables are tables: it can
/// be read against the document, whereas a chain of conditions cannot.
///
/// Three properties the fold depends on, all visible by eye and all pinned by
/// [`tests::the_resolution_table_matches_the_standard`]:
///
/// * `z` is the identity — the `z` row and the `z` column reproduce the other
///   operand, so an undriven contribution never changes an answer;
/// * `x` is absorbing — every entry in the `x` row and the `x` column is `x`;
/// * `0` against `1` is `x`, which is bus contention.
pub(crate) const RESOLUTION_TABLE: [[FourStateBit; 4]; 4] = {
    use FourStateBit::{HighImpedance as Z, One as I, Unknown as X, Zero as O};
    [
        //          0  1  x  z
        /* 0 */ [O, X, X, O],
        /* 1 */ [X, I, X, I],
        /* x */ [X, X, X, X],
        /* z */ [O, I, X, Z],
    ]
};

/// Where a bit sits in [`RESOLUTION_TABLE`].
///
/// The same order the canonical IR's own truth tables use;
/// [`tests::the_resolution_table_is_indexed_like_the_truth_tables`] pins that
/// it stays so, because a reordering of [`FourStateBit`] that permuted one set
/// of tables and not the other would be very hard to see.
const fn bit_index(bit: FourStateBit) -> usize {
    match bit {
        FourStateBit::Zero => 0,
        FourStateBit::One => 1,
        FourStateBit::Unknown => 2,
        FourStateBit::HighImpedance => 3,
    }
}

/// Combine two driver contributions on one bit.
pub(crate) const fn resolve_bit(left: FourStateBit, right: FourStateBit) -> FourStateBit {
    RESOLUTION_TABLE[bit_index(left)][bit_index(right)]
}

/// One observed change of one signal.
///
/// Carries the previous value because edge classification is a property of the
/// transition rather than of the new value, and because a level-sensitive term
/// asks whether anything moved at all.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SignalTransition {
    pub(crate) signal: DigitalSignalId,
    pub(crate) values: TransitionValues,
}

/// What a signal held before and after one change.
///
/// One transition type for both value domains rather than two queues, because
/// the *ordering* is what a queue is for: a four-state net and a real net that
/// both move in one delta must be delivered in the order they moved, and two
/// queues would have to be merged by a rule nothing states.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TransitionValues {
    FourState {
        previous: FourStateValue,
        next: FourStateValue,
    },
    /// Verilog-AMS LRM 2.4 section 3.7's real net. Both values ride along for
    /// the same reason the four-state pair does — a term asks whether anything
    /// moved — even though a real has no edge to classify from them.
    Real { previous: f64, next: f64 },
}

/// Why a store operation was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StoreError {
    /// A signal id the plan does not declare.
    UndeclaredSignal(DigitalSignalId),
    /// An external write to a net the design itself drives.
    ///
    /// A stimulus generator is a driver like any other, and a net that already
    /// has one would need it resolved against the design's rather than
    /// overwritten. Refusing is the fail-closed reading: the alternative
    /// silently discards whatever the design was contributing.
    ExternallyDrivenNetHasDrivers {
        signal: DigitalSignalId,
        name: String,
        drivers: usize,
    },
    /// A value offered at a width the signal was not declared with.
    WidthMismatch {
        signal: DigitalSignalId,
        name: String,
        declared: u32,
        offered: u32,
    },
    /// A four-state stimulus offered to a real net.
    ///
    /// Verilog-AMS LRM 2.4 section 3.7 converts between bits and a real with
    /// the explicit `$realtobits`/`$bitstoreal`; a stimulus harness that meant
    /// to drive a real should say a real.
    RealPortDrivenWithBits {
        signal: DigitalSignalId,
        name: String,
    },
    /// A real stimulus offered to a four-state net, the same mistake reversed.
    FourStatePortDrivenWithAReal {
        signal: DigitalSignalId,
        name: String,
    },
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndeclaredSignal(signal) => write!(
                f,
                "signal {} is not declared by this digital plan",
                usize::from(*signal)
            ),
            Self::ExternallyDrivenNetHasDrivers { name, drivers, .. } => write!(
                f,
                "`{name}` is driven by {drivers} driver(s) inside the design, so an external \
                 stimulus cannot write it; drive an undriven net instead"
            ),
            Self::WidthMismatch {
                name,
                declared,
                offered,
                ..
            } => write!(
                f,
                "`{name}` is declared {declared} bit(s) wide but was offered a {offered}-bit value"
            ),
            Self::RealPortDrivenWithBits { name, .. } => write!(
                f,
                "`{name}` is a real net and was offered a four-state value; Verilog-AMS LRM 2.4                  section 3.7 makes it carry a real, so drive it with one"
            ),
            Self::FourStatePortDrivenWithAReal { name, .. } => write!(
                f,
                "`{name}` is a four-state net and was offered a real value; drive it with a                  four-state spelling"
            ),
        }
    }
}

impl std::error::Error for StoreError {}

/// Which contributions belong to one net.
#[derive(Debug, Clone, Copy)]
struct DriverSpan {
    start: usize,
    count: usize,
}

/// One driver's contribution, kept exactly as the driver evaluated it.
#[derive(Debug, Clone)]
struct Contribution {
    /// Which bits of the net this driver covers, from the plan. Always
    /// [`DigitalWriteSelect::Whole`] for a real net, which has no bits to
    /// cover part of.
    select: DigitalWriteSelect,
    value: ContributionValue,
}

/// What one driver last produced, in the domain its net carries.
#[derive(Debug, Clone)]
enum ContributionValue {
    /// The driver's most recent output, at the select's width, or `None` while
    /// the driver has not run. Both read as `z` on every bit the driver
    /// covers; the distinction is kept because a `None` is evidence about the
    /// schedule and a stored `z` is evidence about the design.
    FourState(Option<FourStateValue>),
    /// The same for a real net, where `None` and a stored `0.0` likewise read
    /// alike — Verilog-AMS LRM 2.4 section 3.7 makes an undriven real net zero
    /// — and are kept apart for the same reason.
    Real(Option<f64>),
}

/// The signal store and driver resolution for one compiled digital plan.
pub(crate) struct DigitalSignalStore {
    /// Current value of every four-state signal, at its declared width.
    ///
    /// A real net has a slot here too, of width zero, and it is never read:
    /// one array per signal id means a signal's value has exactly one place to
    /// live, and paying a zero-width value for a real net is cheaper than a
    /// second index from signal id to real slot that could disagree with this
    /// one.
    values: Vec<FourStateValue>,
    /// Current value of every real net (Verilog-AMS LRM 2.4 section 3.7), in
    /// the same signal space and read only for a signal the plan calls real.
    reals: Vec<f64>,
    /// What each signal carries, cached from the plan.
    kinds: Vec<DigitalSignalKind>,
    /// Declared width per signal, cached so a hot path does not walk the plan.
    widths: Vec<u32>,
    /// Whether each signal is a variable (`reg`) rather than a net.
    variables: Vec<bool>,
    /// Contributions, grouped by net. Indexed through [`Self::spans`].
    contributions: Vec<Contribution>,
    /// Each net's slice of [`Self::contributions`].
    spans: Vec<DriverSpan>,
    /// Nonblocking and otherwise deferred updates, in the order they were
    /// evaluated. The host partitions them by region when it drains.
    deferred: Vec<DigitalDeferredUpdate>,
    /// Value changes since the host last drained, oldest first.
    transitions: Vec<SignalTransition>,
}

impl DigitalSignalStore {
    /// Build a store for one plan.
    ///
    /// Initial values follow IEEE 1364-2005 directly and are not a policy this
    /// store gets to choose: a variable holds `x` (section 4.2.2) and a net
    /// holds the resolution of its drivers, which before any driver has run is
    /// `z` on every bit (section 7.9). An undriven net is therefore `z` and
    /// stays `z` until something outside the design writes it.
    pub(crate) fn new(plan: &CanonicalDigitalPlan) -> Self {
        let count = plan.signals.len();
        let mut widths = Vec::with_capacity(count);
        let mut kinds = Vec::with_capacity(count);
        let mut variables = Vec::with_capacity(count);
        let mut values = Vec::with_capacity(count);
        for signal in &plan.signals {
            widths.push(signal.width);
            kinds.push(signal.kind);
            variables.push(signal.procedurally_assignable);
            values.push(FourStateValue::splat(
                signal.width,
                if signal.procedurally_assignable {
                    FourStateBit::Unknown
                } else {
                    FourStateBit::HighImpedance
                },
            ));
        }
        // Verilog-AMS LRM 2.4 section 3.7 states this as the net's own initial
        // value and not as a consequence of having no drivers: "unlike other
        // digital nets which have an initial value of `z`, wreal nets shall
        // have an initial value of zero". So it is written here, where the
        // four-state initial values are, rather than left to the fold.
        let reals = vec![0.0; count];

        // Grouped by net so resolution reads one contiguous slice, and sized
        // from the plan so a driver that has never run still occupies its slot.
        let mut contributions = Vec::with_capacity(plan.drivers.len());
        let mut spans = Vec::with_capacity(count);
        for index in 0..count {
            let signal = DigitalSignalId::from(index);
            let start = contributions.len();
            let real = plan
                .signal(signal)
                .is_some_and(|signal| signal.kind.is_real());
            for driver in plan.drivers_of(signal) {
                contributions.push(Contribution {
                    select: driver.target.select.clone(),
                    value: if real {
                        ContributionValue::Real(None)
                    } else {
                        ContributionValue::FourState(None)
                    },
                });
            }
            spans.push(DriverSpan {
                start,
                count: contributions.len() - start,
            });
        }

        Self {
            values,
            reals,
            kinds,
            widths,
            variables,
            contributions,
            spans,
            deferred: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// The value a signal holds right now.
    pub(crate) fn value(&self, signal: DigitalSignalId) -> Option<&FourStateValue> {
        self.values.get(usize::from(signal))
    }

    /// Write a signal from outside the design.
    ///
    /// This is how a stimulus generator drives a top-level input, and it is
    /// deliberately not the same operation as a driver's contribution: an
    /// external writer has no [`DigitalDriverId`], so its value cannot enter
    /// the resolution. A net the design drives is refused rather than
    /// overwritten.
    pub(crate) fn force(
        &mut self,
        signal: DigitalSignalId,
        value: FourStateValue,
        plan: &CanonicalDigitalPlan,
    ) -> Result<(), StoreError> {
        let index = usize::from(signal);
        let declared = *self
            .widths
            .get(index)
            .ok_or(StoreError::UndeclaredSignal(signal))?;
        let name = signal_name(plan, signal);
        if self.is_real(signal) {
            return Err(StoreError::RealPortDrivenWithBits { signal, name });
        }
        if value.width() != declared {
            return Err(StoreError::WidthMismatch {
                signal,
                name,
                declared,
                offered: value.width(),
            });
        }
        let drivers = self.spans[index].count;
        if drivers > 0 {
            return Err(StoreError::ExternallyDrivenNetHasDrivers {
                signal,
                name,
                drivers,
            });
        }
        self.publish(signal, value);
        Ok(())
    }

    /// Write a real net from outside the design.
    ///
    /// The real twin of [`Self::force`], and refused on the same ground: a net
    /// the design already drives would need the stimulus resolved against the
    /// design's contribution rather than overwritten, and there is no driver
    /// identity for a stimulus to occupy.
    pub(crate) fn force_real(
        &mut self,
        signal: DigitalSignalId,
        value: f64,
        plan: &CanonicalDigitalPlan,
    ) -> Result<(), StoreError> {
        let index = usize::from(signal);
        if index >= self.reals.len() {
            return Err(StoreError::UndeclaredSignal(signal));
        }
        let name = signal_name(plan, signal);
        if !self.is_real(signal) {
            return Err(StoreError::FourStatePortDrivenWithAReal { signal, name });
        }
        let drivers = self.spans[index].count;
        if drivers > 0 {
            return Err(StoreError::ExternallyDrivenNetHasDrivers {
                signal,
                name,
                drivers,
            });
        }
        self.publish_real(signal, value);
        Ok(())
    }

    /// Take every deferred update belonging to one region, oldest first.
    ///
    /// Partitioned rather than drained wholesale so a promotion moves exactly
    /// one region: a nonblocking update must not be applied in the same pass as
    /// an inactive-region one, or `#0` would stop meaning "after the active
    /// region and before the nonblocking one".
    pub(crate) fn take_deferred_in(
        &mut self,
        region: DigitalSchedulingRegion,
    ) -> Vec<DigitalDeferredUpdate> {
        if !self.deferred.iter().any(|update| update.region == region) {
            return Vec::new();
        }
        let mut due = Vec::new();
        let mut held = Vec::with_capacity(self.deferred.len());
        for update in std::mem::take(&mut self.deferred) {
            if update.region == region {
                due.push(update);
            } else {
                held.push(update);
            }
        }
        self.deferred = held;
        due
    }

    /// Value changes since the last drain, oldest first.
    pub(crate) fn take_transitions(&mut self) -> Vec<SignalTransition> {
        std::mem::take(&mut self.transitions)
    }

    /// The value a real net holds right now.
    pub(crate) fn real_value(&self, signal: DigitalSignalId) -> Option<f64> {
        self.reals.get(usize::from(signal)).copied()
    }

    /// Whether the plan calls this signal real.
    pub(crate) fn is_real(&self, signal: DigitalSignalId) -> bool {
        self.kinds
            .get(usize::from(signal))
            .is_some_and(|kind| kind.is_real())
    }

    /// Store a value and record the transition if it is one.
    fn publish(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        let index = usize::from(signal);
        if self.values[index] == value {
            return;
        }
        let previous = std::mem::replace(&mut self.values[index], value.clone());
        self.transitions.push(SignalTransition {
            signal,
            values: TransitionValues::FourState {
                previous,
                next: value,
            },
        });
    }

    /// Store a real value and record the transition if it is one.
    ///
    /// The change test is `!=` and nothing else. Verilog-AMS LRM 2.4 section
    /// 3.7's event on a real net is a change of value; a tolerance here would
    /// decide that some changes do not count, which is a rule the standard does
    /// not have and which no author could see being applied.
    fn publish_real(&mut self, signal: DigitalSignalId, value: f64) {
        let index = usize::from(signal);
        if self.reals[index] == value {
            return;
        }
        let previous = std::mem::replace(&mut self.reals[index], value);
        self.transitions.push(SignalTransition {
            signal,
            values: TransitionValues::Real {
                previous,
                next: value,
            },
        });
    }

    /// Fold every contribution of one net into its resolved value.
    ///
    /// Starts from all-`z` and folds in declaration order. `z` is
    /// [`RESOLUTION_TABLE`]'s identity, so the starting value contributes
    /// nothing and a net with one driver resolves to exactly that driver's
    /// output over the bits it covers.
    fn resolve(&self, signal: DigitalSignalId) -> FourStateValue {
        let index = usize::from(signal);
        let width = self.widths[index];
        let span = self.spans[index];
        let mut resolved = FourStateValue::splat(width, FourStateBit::HighImpedance);
        for contribution in &self.contributions[span.start..span.start + span.count] {
            let ContributionValue::FourState(Some(value)) = &contribution.value else {
                continue;
            };
            let low = match contribution.select {
                DigitalWriteSelect::Whole => 0,
                DigitalWriteSelect::Bit(position) => position,
                DigitalWriteSelect::Part { msb, lsb } => msb.min(lsb),
            };
            for offset in 0..value.width() {
                let position = low + i64::from(offset);
                if position < 0 || position >= i64::from(width) {
                    continue;
                }
                let position = position as u32;
                resolved.set_bit(
                    position,
                    resolve_bit(resolved.bit(position), value.bit(offset)),
                );
            }
        }
        resolved
    }

    /// Combine every contribution to one real net into its resolved value.
    ///
    /// Verilog-AMS LRM 2.4 section 3.7 for the empty and single cases, and the
    /// net's own declared resolution for the rest. Written as a fold over the
    /// contributions that exist rather than one seeded with an identity,
    /// because none of these four has one: `0.0` is an identity for `sum` and
    /// for nothing else, and seeding `min` with it would clamp every net that
    /// ought to answer a positive number.
    ///
    /// `Single` reaching here with more than one contribution cannot happen —
    /// section 6.5.3 permits one driver and the front end refuses a second —
    /// but it is still given the honest answer for what it has rather than a
    /// `debug_assert`: the first driver's contribution, which is what one
    /// driver means.
    fn resolve_real(&self, signal: DigitalSignalId) -> f64 {
        let index = usize::from(signal);
        let span = self.spans[index];
        let mut contributions = self.contributions[span.start..span.start + span.count]
            .iter()
            .map(|contribution| match contribution.value {
                // A driver that has not run has produced nothing, and section
                // 3.7 makes the value of a real net with nothing driving it
                // zero. Both readings of "not driving" agree on 0.0, which is
                // why the fold does not have to distinguish them.
                ContributionValue::Real(value) => value.unwrap_or(0.0),
                ContributionValue::FourState(_) => 0.0,
            })
            .peekable();

        // Section 3.7: "If no driver is connected to a wreal net, its value
        // shall be zero (0.0)." Answered from the clause rather than by folding
        // an empty sequence, so that the three resolutions with no identity do
        // not each need an answer for a case the standard already gives one.
        if contributions.peek().is_none() {
            return 0.0;
        }

        let resolution = match self.kinds[index] {
            DigitalSignalKind::Real(resolution) => resolution,
            DigitalSignalKind::FourState => return 0.0,
        };
        match resolution {
            DigitalRealResolution::Single => contributions.next().unwrap_or(0.0),
            DigitalRealResolution::Sum => contributions.sum(),
            DigitalRealResolution::Average => {
                let values: Vec<f64> = contributions.collect();
                // Divided by the number of drivers the net *has*, not by the
                // number that have run: a driver is a permanent property of the
                // design (IEEE 1364-2005 section 6.1), so an average that
                // changed denominator as the schedule progressed would report a
                // different number for the same circuit depending on when it
                // was asked.
                values.iter().sum::<f64>() / values.len() as f64
            }
            DigitalRealResolution::Minimum => contributions.fold(f64::INFINITY, f64::min),
            DigitalRealResolution::Maximum => contributions.fold(f64::NEG_INFINITY, f64::max),
        }
    }

    /// The contribution slot of one driver, if the plan declared it.
    fn contribution_slot(&self, driver: DigitalDriverId) -> Option<usize> {
        let span = self.spans.get(usize::from(driver.signal))?;
        let offset = driver.index as usize;
        (offset < span.count).then_some(span.start + offset)
    }
}

/// The plan's name for a signal, or a positional stand-in.
///
/// Only ever used to build a diagnostic, so an id the plan does not declare
/// produces a readable message rather than a second failure.
pub(crate) fn signal_name(plan: &CanonicalDigitalPlan, signal: DigitalSignalId) -> String {
    plan.signal(signal)
        .map(|declared: &DigitalSignal| declared.name.to_string())
        .unwrap_or_else(|| format!("signal#{}", usize::from(signal)))
}

impl DigitalEnvironment for DigitalSignalStore {
    fn read_signal(&self, signal: DigitalSignalId) -> Option<FourStateValue> {
        self.values.get(usize::from(signal)).cloned()
    }

    fn write_signal(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        // A procedural write to a net would be a semantic error the front end
        // rejects, so reaching here with one means the plan and the store
        // disagree; storing it anyway is the lesser evil to panicking inside an
        // interpreter, and `variables` records the fact for the host's audit.
        debug_assert!(
            self.variables
                .get(usize::from(signal))
                .copied()
                .unwrap_or(true),
            "procedural write to a net; the front end should have refused it"
        );
        if usize::from(signal) >= self.values.len() {
            return;
        }
        self.publish(signal, value);
    }

    fn defer_update(&mut self, update: DigitalDeferredUpdate) {
        self.deferred.push(update);
    }

    fn drive_signal(&mut self, drive: DigitalDrive) {
        let Some(slot) = self.contribution_slot(drive.driver) else {
            // A drive naming a driver the plan does not list. Dropping it is
            // the only thing that keeps the resolution honest — there is no
            // slot to put it in and folding it into the net directly would be
            // the last-write-wins bug.
            debug_assert!(false, "drive from a driver the plan does not declare");
            return;
        };
        self.contributions[slot].value = ContributionValue::FourState(Some(drive.value));
        let resolved = self.resolve(drive.driver.signal);
        self.publish(drive.driver.signal, resolved);
    }

    fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64> {
        self.reals.get(usize::from(signal)).copied()
    }

    fn drive_real_signal(&mut self, drive: DigitalRealDrive) {
        let Some(slot) = self.contribution_slot(drive.driver) else {
            debug_assert!(false, "drive from a driver the plan does not declare");
            return;
        };
        self.contributions[slot].value = ContributionValue::Real(Some(drive.value));
        let resolved = self.resolve_real(drive.driver.signal);
        self.publish_real(drive.driver.signal, resolved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FourStateBit::{HighImpedance as Z, One as I, Unknown as X, Zero as O};
    use rspice_veriloga::canonical_ir::diagnostic::SourceSpanRef;
    use rspice_veriloga::canonical_ir::digital::{DigitalDriver, DigitalWriteTarget};
    use rspice_veriloga::canonical_ir::ids::DigitalProcessId;

    fn span() -> SourceSpanRef {
        SourceSpanRef {
            source_file_id: 0,
            start: 0,
            end: 0,
        }
    }

    fn signal(index: usize, name: &str, width: u32, reg: bool) -> DigitalSignal {
        DigitalSignal {
            id: DigitalSignalId::from(index),
            name: name.into(),
            kind: DigitalSignalKind::FourState,
            width,
            bounds: (width > 1).then_some((i64::from(width) - 1, 0)),
            signed: false,
            procedurally_assignable: reg,
            span: span(),
        }
    }

    /// A real net with the resolution its net-type keyword named. Width zero,
    /// because Verilog-AMS LRM 2.4 section 3.7 gives one no bits.
    fn real_signal(index: usize, name: &str, resolution: DigitalRealResolution) -> DigitalSignal {
        DigitalSignal {
            id: DigitalSignalId::from(index),
            name: name.into(),
            kind: DigitalSignalKind::Real(resolution),
            width: 0,
            bounds: None,
            signed: false,
            procedurally_assignable: false,
            span: span(),
        }
    }

    fn driver(signal: usize, index: u32, select: DigitalWriteSelect) -> DigitalDriver {
        DigitalDriver {
            id: DigitalDriverId {
                signal: DigitalSignalId::from(signal),
                index,
            },
            target: DigitalWriteTarget {
                signal: DigitalSignalId::from(signal),
                select,
            },
            process: DigitalProcessId::from(0usize),
            span: span(),
        }
    }

    fn value(spelling: &str) -> FourStateValue {
        let bits: Vec<FourStateBit> = spelling
            .chars()
            .map(|character| match character {
                '0' => O,
                '1' => I,
                'x' => X,
                'z' => Z,
                other => panic!("not a four-state digit: {other}"),
            })
            .collect();
        FourStateValue::from_bits_msb_first(&bits)
    }

    /// Table 4-1, written out. A test that re-derived the table from a rule
    /// would agree with a wrong transcription of it.
    #[test]
    fn the_resolution_table_matches_the_standard() {
        // z is the identity in both directions.
        for bit in [O, I, X, Z] {
            assert_eq!(resolve_bit(Z, bit), bit);
            assert_eq!(resolve_bit(bit, Z), bit);
        }
        // x is absorbing in both directions.
        for bit in [O, I, X, Z] {
            assert_eq!(resolve_bit(X, bit), X);
            assert_eq!(resolve_bit(bit, X), X);
        }
        // Agreement survives, disagreement is contention.
        assert_eq!(resolve_bit(O, O), O);
        assert_eq!(resolve_bit(I, I), I);
        assert_eq!(resolve_bit(O, I), X);
        assert_eq!(resolve_bit(I, O), X);
    }

    /// The table is commutative, which is what makes a fold's answer
    /// independent of the order the drivers happen to be listed in.
    #[test]
    fn the_resolution_is_commutative() {
        for left in [O, I, X, Z] {
            for right in [O, I, X, Z] {
                assert_eq!(resolve_bit(left, right), resolve_bit(right, left));
            }
        }
    }

    #[test]
    fn the_resolution_table_is_indexed_like_the_truth_tables() {
        use rspice_veriloga::canonical_ir::digital_value::TABLE_ORDER;
        for (index, bit) in TABLE_ORDER.into_iter().enumerate() {
            assert_eq!(bit_index(bit), index);
        }
    }

    #[test]
    fn a_variable_starts_unknown_and_an_undriven_net_starts_high_impedance() {
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "q", 4, true), signal(1, "w", 4, false)],
            processes: Vec::new(),
            drivers: Vec::new(),
        };
        let store = DigitalSignalStore::new(&plan);
        assert_eq!(
            store
                .value(DigitalSignalId::from(0usize))
                .unwrap()
                .spelling(),
            "xxxx"
        );
        assert_eq!(
            store
                .value(DigitalSignalId::from(1usize))
                .unwrap()
                .spelling(),
            "zzzz"
        );
    }

    /// A net whose only driver has not run yet is `z`, and becomes that
    /// driver's value the moment it does. This is the property that lets a
    /// resolver be correct before the schedule has run anything.
    #[test]
    fn a_single_driver_net_resolves_to_that_driver() {
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "y", 1, false)],
            processes: Vec::new(),
            drivers: vec![driver(0, 0, DigitalWriteSelect::Whole)],
        };
        let mut store = DigitalSignalStore::new(&plan);
        assert_eq!(
            store
                .value(DigitalSignalId::from(0usize))
                .unwrap()
                .spelling(),
            "z"
        );

        store.drive_signal(DigitalDrive {
            driver: DigitalDriverId {
                signal: DigitalSignalId::from(0usize),
                index: 0,
            },
            target: DigitalWriteTarget {
                signal: DigitalSignalId::from(0usize),
                select: DigitalWriteSelect::Whole,
            },
            value: value("1"),
        });
        assert_eq!(
            store
                .value(DigitalSignalId::from(0usize))
                .unwrap()
                .spelling(),
            "1"
        );
        assert_eq!(store.take_transitions().len(), 1);
    }

    /// The tristate bus of the corpus's four-state case, in miniature: two
    /// conditional drivers on one net through all four driver states.
    #[test]
    fn two_drivers_resolve_per_the_standard_through_every_driver_state() {
        let net = DigitalSignalId::from(0usize);
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "bus", 1, false)],
            processes: Vec::new(),
            drivers: vec![
                driver(0, 0, DigitalWriteSelect::Whole),
                driver(0, 1, DigitalWriteSelect::Whole),
            ],
        };
        let mut store = DigitalSignalStore::new(&plan);
        let mut drive = |index: u32, spelling: &str| {
            store.drive_signal(DigitalDrive {
                driver: DigitalDriverId { signal: net, index },
                target: DigitalWriteTarget {
                    signal: net,
                    select: DigitalWriteSelect::Whole,
                },
                value: value(spelling),
            });
            store.value(net).unwrap().spelling()
        };

        assert_eq!(drive(0, "z"), "z", "neither driver on");
        assert_eq!(drive(1, "z"), "z");
        assert_eq!(drive(0, "1"), "1", "one driver high");
        assert_eq!(drive(0, "z"), "z");
        assert_eq!(drive(1, "0"), "0", "the other driver low");
        assert_eq!(
            drive(0, "1"),
            "x",
            "contention resolves to x, not to either"
        );
    }

    /// `assign bus[3:0] = ...` and `assign bus[7:4] = ...` compose rather than
    /// fight, because resolution is per bit.
    #[test]
    fn two_partial_drivers_of_one_net_cover_different_bits() {
        let net = DigitalSignalId::from(0usize);
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "bus", 8, false)],
            processes: Vec::new(),
            drivers: vec![
                driver(0, 0, DigitalWriteSelect::Part { msb: 3, lsb: 0 }),
                driver(0, 1, DigitalWriteSelect::Part { msb: 7, lsb: 4 }),
            ],
        };
        let mut store = DigitalSignalStore::new(&plan);
        store.drive_signal(DigitalDrive {
            driver: DigitalDriverId {
                signal: net,
                index: 0,
            },
            target: DigitalWriteTarget {
                signal: net,
                select: DigitalWriteSelect::Part { msb: 3, lsb: 0 },
            },
            value: value("1010"),
        });
        assert_eq!(store.value(net).unwrap().spelling(), "zzzz1010");
        store.drive_signal(DigitalDrive {
            driver: DigitalDriverId {
                signal: net,
                index: 1,
            },
            target: DigitalWriteTarget {
                signal: net,
                select: DigitalWriteSelect::Part { msb: 7, lsb: 4 },
            },
            value: value("0011"),
        });
        assert_eq!(store.value(net).unwrap().spelling(), "00111010");
    }

    /// A rewrite of the value a signal already holds is not an event. Without
    /// this a level-sensitive process feeding itself would never settle.
    #[test]
    fn storing_an_unchanged_value_produces_no_transition() {
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "q", 2, true)],
            processes: Vec::new(),
            drivers: Vec::new(),
        };
        let mut store = DigitalSignalStore::new(&plan);
        store.write_signal(DigitalSignalId::from(0usize), value("01"));
        assert_eq!(store.take_transitions().len(), 1);
        store.write_signal(DigitalSignalId::from(0usize), value("01"));
        assert!(store.take_transitions().is_empty());
    }

    #[test]
    fn forcing_a_net_the_design_drives_is_refused() {
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "y", 1, false), signal(1, "a", 1, false)],
            processes: Vec::new(),
            drivers: vec![driver(0, 0, DigitalWriteSelect::Whole)],
        };
        let mut store = DigitalSignalStore::new(&plan);
        let error = store
            .force(DigitalSignalId::from(0usize), value("1"), &plan)
            .expect_err("the design drives `y`");
        assert!(matches!(
            error,
            StoreError::ExternallyDrivenNetHasDrivers { .. }
        ));
        // An undriven net is the stimulus generator's to write.
        store
            .force(DigitalSignalId::from(1usize), value("1"), &plan)
            .expect("`a` has no driver inside the design");
    }

    #[test]
    fn forcing_at_the_wrong_width_is_refused() {
        let plan = CanonicalDigitalPlan {
            signals: vec![signal(0, "a", 4, false)],
            processes: Vec::new(),
            drivers: Vec::new(),
        };
        let mut store = DigitalSignalStore::new(&plan);
        let error = store
            .force(DigitalSignalId::from(0usize), value("01"), &plan)
            .expect_err("two bits is not four");
        assert!(matches!(error, StoreError::WidthMismatch { .. }));
    }

    // ------------------------------------------------------------------
    // Real nets (Verilog-AMS LRM 2.4 section 3.7)
    // ------------------------------------------------------------------

    fn real_plan(resolution: DigitalRealResolution, drivers: u32) -> CanonicalDigitalPlan {
        CanonicalDigitalPlan {
            signals: vec![real_signal(0, "bus", resolution)],
            processes: Vec::new(),
            drivers: (0..drivers)
                .map(|index| driver(0, index, DigitalWriteSelect::Whole))
                .collect(),
        }
    }

    fn drive_real(store: &mut DigitalSignalStore, index: u32, value: f64) -> f64 {
        store.drive_real_signal(DigitalRealDrive {
            driver: DigitalDriverId {
                signal: DigitalSignalId::from(0usize),
                index,
            },
            value,
        });
        store
            .real_value(DigitalSignalId::from(0usize))
            .expect("declared")
    }

    /// "If no driver is connected to a wreal net, its value shall be zero
    /// (0.0). Unlike other digital nets which have an initial value of `z`,
    /// wreal nets shall have an initial value of zero." — section 3.7, which
    /// is the whole of the undriven ruling and needs no fold to reach.
    #[test]
    fn an_undriven_real_net_is_zero_and_not_high_impedance() {
        let plan = real_plan(DigitalRealResolution::Single, 0);
        let store = DigitalSignalStore::new(&plan);
        assert_eq!(store.real_value(DigitalSignalId::from(0usize)), Some(0.0));

        // And so is a net whose one driver has not run yet, for the same
        // clause: a wreal "shall not store its value".
        let plan = real_plan(DigitalRealResolution::Single, 1);
        let store = DigitalSignalStore::new(&plan);
        assert_eq!(store.real_value(DigitalSignalId::from(0usize)), Some(0.0));
    }

    /// A net with one driver is that driver's value, before and after every
    /// resolution has anything to combine.
    #[test]
    fn a_single_driver_real_net_resolves_to_that_driver() {
        for resolution in [
            DigitalRealResolution::Single,
            DigitalRealResolution::Sum,
            DigitalRealResolution::Average,
            DigitalRealResolution::Minimum,
            DigitalRealResolution::Maximum,
        ] {
            let plan = real_plan(resolution, 1);
            let mut store = DigitalSignalStore::new(&plan);
            assert_eq!(drive_real(&mut store, 0, -2.5), -2.5, "{resolution:?}");
        }
    }

    /// Each resolved spelling, pinned against the arithmetic its keyword names.
    ///
    /// Two drivers holding 3.0 and -1.0: their sum is 2.0, their average 1.0,
    /// their least -1.0 and their greatest 3.0. Written out rather than derived
    /// from a shared helper, so a fold that changed would have to be changed
    /// here too.
    #[test]
    fn each_resolution_combines_its_drivers_as_its_keyword_says() {
        for (resolution, expected) in [
            (DigitalRealResolution::Sum, 2.0),
            (DigitalRealResolution::Average, 1.0),
            (DigitalRealResolution::Minimum, -1.0),
            (DigitalRealResolution::Maximum, 3.0),
        ] {
            let plan = real_plan(resolution, 2);
            let mut store = DigitalSignalStore::new(&plan);
            drive_real(&mut store, 0, 3.0);
            assert_eq!(drive_real(&mut store, 1, -1.0), expected, "{resolution:?}");
        }
    }

    /// A driver that has not run contributes the section 3.7 initial value, so
    /// `wrealmin` over one driver at 3.0 and one that has never evaluated is
    /// `0.0` and not `3.0`. Stated as a test because it is the case where "no
    /// identity to seed with" has a visible consequence.
    #[test]
    fn a_driver_that_has_not_run_contributes_the_initial_value() {
        let plan = real_plan(DigitalRealResolution::Minimum, 2);
        let mut store = DigitalSignalStore::new(&plan);
        assert_eq!(drive_real(&mut store, 0, 3.0), 0.0);

        let plan = real_plan(DigitalRealResolution::Average, 2);
        let mut store = DigitalSignalStore::new(&plan);
        assert_eq!(
            drive_real(&mut store, 0, 4.0),
            2.0,
            "divided by two, not one"
        );
    }

    /// A rewrite of the value a real net already holds is not an event, the
    /// same rule the four-state side has — and the test is exact, so a change
    /// of one unit in the last place *is* one.
    #[test]
    fn a_real_transition_is_recorded_on_an_exact_change() {
        let plan = real_plan(DigitalRealResolution::Single, 1);
        let mut store = DigitalSignalStore::new(&plan);
        drive_real(&mut store, 0, 1.0);
        assert_eq!(store.take_transitions().len(), 1);
        drive_real(&mut store, 0, 1.0);
        assert!(store.take_transitions().is_empty());
        drive_real(&mut store, 0, 1.0 + f64::EPSILON);
        assert_eq!(store.take_transitions().len(), 1, "no tolerance is applied");
    }

    /// The two domains do not substitute for one another at the stimulus
    /// boundary either.
    #[test]
    fn a_stimulus_in_the_wrong_domain_is_refused() {
        let plan = CanonicalDigitalPlan {
            signals: vec![
                real_signal(0, "level", DigitalRealResolution::Single),
                signal(1, "a", 1, false),
            ],
            processes: Vec::new(),
            drivers: Vec::new(),
        };
        let mut store = DigitalSignalStore::new(&plan);
        assert!(matches!(
            store
                .force(DigitalSignalId::from(0usize), value("1"), &plan)
                .expect_err("`level` carries a real"),
            StoreError::RealPortDrivenWithBits { .. }
        ));
        assert!(matches!(
            store
                .force_real(DigitalSignalId::from(1usize), 1.0, &plan)
                .expect_err("`a` carries bits"),
            StoreError::FourStatePortDrivenWithAReal { .. }
        ));
        store
            .force_real(DigitalSignalId::from(0usize), 1.5, &plan)
            .expect("an undriven real net is the stimulus generator's to write");
        assert_eq!(store.real_value(DigitalSignalId::from(0usize)), Some(1.5));
    }
}
