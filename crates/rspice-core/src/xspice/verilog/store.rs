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
    CanonicalDigitalPlan, DigitalDriverId, DigitalSchedulingRegion, DigitalSignal,
    DigitalWriteSelect,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalDeferredUpdate, DigitalDrive, DigitalEnvironment,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignalTransition {
    pub(crate) signal: DigitalSignalId,
    pub(crate) previous: FourStateValue,
    pub(crate) next: FourStateValue,
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
    /// Which bits of the net this driver covers, from the plan.
    select: DigitalWriteSelect,
    /// The driver's most recent output, at the select's width, or `None` while
    /// the driver has not run. Both read as `z` on every bit the driver
    /// covers; the distinction is kept because a `None` is evidence about the
    /// schedule and a stored `z` is evidence about the design.
    value: Option<FourStateValue>,
}

/// The signal store and driver resolution for one compiled digital plan.
pub(crate) struct DigitalSignalStore {
    /// Current value of every signal, at its declared width.
    values: Vec<FourStateValue>,
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
        let mut variables = Vec::with_capacity(count);
        let mut values = Vec::with_capacity(count);
        for signal in &plan.signals {
            widths.push(signal.width);
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

        // Grouped by net so resolution reads one contiguous slice, and sized
        // from the plan so a driver that has never run still occupies its slot.
        let mut contributions = Vec::with_capacity(plan.drivers.len());
        let mut spans = Vec::with_capacity(count);
        for index in 0..count {
            let signal = DigitalSignalId::from(index);
            let start = contributions.len();
            for driver in plan.drivers_of(signal) {
                contributions.push(Contribution {
                    select: driver.target.select.clone(),
                    value: None,
                });
            }
            spans.push(DriverSpan {
                start,
                count: contributions.len() - start,
            });
        }

        Self {
            values,
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

    /// Store a value and record the transition if it is one.
    fn publish(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        let index = usize::from(signal);
        if self.values[index] == value {
            return;
        }
        let previous = std::mem::replace(&mut self.values[index], value.clone());
        self.transitions.push(SignalTransition {
            signal,
            previous,
            next: value,
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
            let Some(value) = &contribution.value else {
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
        self.contributions[slot].value = Some(drive.value);
        let resolved = self.resolve(drive.driver.signal);
        self.publish(drive.driver.signal, resolved);
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
            width,
            bounds: (width > 1).then_some((i64::from(width) - 1, 0)),
            signed: false,
            procedurally_assignable: reg,
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
}
