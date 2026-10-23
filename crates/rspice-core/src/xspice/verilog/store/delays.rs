//! Delays on drivers and nets (IEEE 1364-2005 sections 6.1.3 and 7.14): the
//! store's half.
//!
//! A delayed driver's value does not reach its contribution when its process
//! writes it. The store holds it back as a [`DelayRequest`] for the host,
//! whose pending slot for that driver applies the inertial rule and, once the
//! delay has passed, hands the value back through
//! [`DigitalSignalStore::apply_drive`]. A net delay is the same stage on the
//! value the net's drivers resolve to, handed back through
//! [`DigitalSignalStore::apply_net_value`].

use super::*;
use rspice_veriloga::canonical_ir::digital::{DigitalDriveStrength, DigitalWriteSelect};

/// Rise, fall and turn-off delays in the plan's ticks.
pub(crate) type TransitionTicks = [u64; 3];

/// The delays by contribution slot and by net, and the destinations of module
/// paths (IEEE 1364-2005 section 14), whose delays the host chooses. Empty
/// for a plan with none.
#[derive(Debug, Default, Clone)]
pub(super) struct DelayTables {
    drivers: Vec<Option<TransitionTicks>>,
    nets: Vec<Option<TransitionTicks>>,
    paths: Vec<bool>,
}

/// The delay tables and the values held back since the host last took them.
#[derive(Debug, Clone, Default)]
pub(super) struct StoreDelays {
    tables: Arc<DelayTables>,
    requests: Vec<DelayRequest>,
}

/// A value the store held back for the host's pending slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DelayRequest {
    /// A delayed driver's new value.
    Drive {
        driver: DigitalDriverId,
        value: FourStateValue,
    },
    /// What a delayed net's drivers now resolve to.
    Net {
        signal: DigitalSignalId,
        value: FourStateValue,
    },
}

impl StoreDelays {
    /// The plan's delays at its own precision. A delay that precision cannot
    /// count is past any run's end, and is treated as one.
    pub(super) fn new(plan: &CanonicalDigitalPlan, spans: &[DriverSpan]) -> Self {
        if plan.delays.is_empty() {
            return Self::default();
        }
        let precision = plan.timing.precision_exponent;
        let ticks = |delay: rspice_veriloga::canonical_ir::digital::DigitalTransitionDelay| {
            delay.ticks_at(precision).unwrap_or([u64::MAX; 3])
        };
        let mut tables = DelayTables {
            drivers: vec![None; spans.last().map_or(0, |span| span.start + span.count)],
            nets: vec![None; plan.signals.len()],
            paths: vec![false; plan.signals.len()],
        };
        for path in &plan.delays.paths {
            if let Some(staged) = tables.paths.get_mut(usize::from(path.destination)) {
                *staged = true;
            }
        }
        for delayed in &plan.delays.drivers {
            if let Some(span) = spans.get(usize::from(delayed.driver.signal))
                && (delayed.driver.index as usize) < span.count
            {
                tables.drivers[span.start + delayed.driver.index as usize] =
                    Some(ticks(delayed.delay));
            }
        }
        for delayed in &plan.delays.nets {
            if let Some(net) = tables.nets.get_mut(usize::from(delayed.signal)) {
                *net = Some(ticks(delayed.delay));
            }
        }
        Self {
            tables: Arc::new(tables),
            requests: Vec::new(),
        }
    }

    fn driver(&self, slot: usize) -> Option<TransitionTicks> {
        self.tables.drivers.get(slot).copied().flatten()
    }

    fn net(&self, signal: DigitalSignalId) -> Option<TransitionTicks> {
        self.tables.nets.get(usize::from(signal)).copied().flatten()
    }

    fn path(&self, signal: DigitalSignalId) -> bool {
        self.tables
            .paths
            .get(usize::from(signal))
            .copied()
            .unwrap_or(false)
    }
}

/// The delay of one bit's change (section 7.14, table 7-14): to 1 the rise
/// delay, to 0 the fall, to `z` the turn-off, and to `x` the least of the
/// three; no change, none.
fn bit_delay(from: FourStateBit, to: FourStateBit, [rise, fall, turn_off]: TransitionTicks) -> u64 {
    if from == to {
        return 0;
    }
    match to {
        FourStateBit::One => rise,
        FourStateBit::Zero => fall,
        FourStateBit::HighImpedance => turn_off,
        FourStateBit::Unknown => rise.min(fall).min(turn_off),
    }
}

impl DigitalSignalStore {
    /// The values held back since the last call, in the order they were.
    pub(crate) fn take_delay_requests(&mut self) -> Vec<DelayRequest> {
        std::mem::take(&mut self.delays.requests)
    }

    /// Whether `signal` has a net delay or is a module path's destination.
    pub(crate) fn delays_net(&self, signal: DigitalSignalId) -> bool {
        self.delays.net(signal).is_some() || self.delays.path(signal)
    }

    /// Whether `signal` is a module path's destination, whose changes wait
    /// for the delay the host chooses.
    pub(crate) fn path_destination(&self, signal: DigitalSignalId) -> bool {
        self.delays.path(signal)
    }

    /// Run every module path destination undelayed (`.options
    /// hdl_path_delays=no`), before the design starts.
    pub(crate) fn drop_path_delays(&mut self) {
        let staged: Vec<usize> = (0..self.delays.tables.paths.len())
            .filter(|index| self.delays.tables.paths[*index])
            .collect();
        if staged.is_empty() {
            return;
        }
        Arc::make_mut(&mut self.delays.tables).paths.fill(false);
        // A destination started `x` only for its stage's sake; it resolves
        // its drivers as any net does.
        for index in staged {
            if self.spans[index].count > 0 {
                self.settle(DigitalSignalId::from(index), 0);
            }
        }
        // What that changed is the initial state, not an event.
        self.delays.requests.clear();
        self.transitions.clear();
        self.take_charge_changes();
    }

    /// Stage the module path destinations `other` stages, for a store
    /// restored in its place: [`Self::drop_path_delays`] holds for the run.
    pub(super) fn inherit_path_stages(&mut self, other: &Self) {
        self.delays.tables = Arc::clone(&other.delays.tables);
    }

    /// Whether a value is held back that the host has not taken yet, which
    /// no checkpoint may leave behind.
    pub(crate) fn has_delay_requests(&self) -> bool {
        !self.delays.requests.is_empty()
    }

    /// Hold a delayed driver's value back for the host; `false` for a driver
    /// with no delay, whose value the caller applies.
    pub(super) fn hold_drive(&mut self, slot: usize, drive: &DigitalDrive) -> bool {
        if self.delays.driver(slot).is_none() {
            return false;
        }
        self.delays.requests.push(DelayRequest::Drive {
            driver: drive.driver,
            value: drive.value.clone(),
        });
        true
    }

    /// Hold a delayed net's resolved value back for the host; `false` for a
    /// net with no delay, which the caller publishes.
    pub(super) fn hold_net(&mut self, signal: DigitalSignalId, value: &FourStateValue) -> bool {
        if !self.delays_net(signal) {
            return false;
        }
        self.delays.requests.push(DelayRequest::Net {
            signal,
            value: value.clone(),
        });
        true
    }

    /// What a request's driver or net has now: the value a delayed driver
    /// contributes, or a delayed net's published value.
    pub(crate) fn delayed_current(&self, request: &DelayRequest) -> Option<FourStateValue> {
        match request {
            DelayRequest::Drive { driver, .. } => {
                let slot = self.contribution_slot(*driver)?;
                match &self.contributions[slot].value {
                    ContributionValue::FourState(value) => value.clone(),
                    ContributionValue::Real(_) => None,
                }
            }
            DelayRequest::Net { signal, .. } => self.values.get(usize::from(*signal)).cloned(),
        }
    }

    /// How long `request`'s driver or net takes to go from `from` to `to`
    /// (section 7.14), or `None` when the two drive the same thing, which is
    /// no transition at all.
    ///
    /// A vector takes the longest of its bits' delays, as Icarus Verilog
    /// does. A gate's change is its output's, read at a strong drive so that
    /// `L` and `H` stay apart; a bidirectional switch's is its control's.
    pub(crate) fn transition_ticks(
        &self,
        request: &DelayRequest,
        from: &FourStateValue,
        to: &FourStateValue,
    ) -> Option<u64> {
        let (delays, from, to) = match request {
            DelayRequest::Drive { driver, .. } => {
                let slot = self.contribution_slot(*driver)?;
                (
                    self.delays.driver(slot)?,
                    self.delayed_output(slot, from),
                    self.delayed_output(slot, to),
                )
            }
            DelayRequest::Net { signal, .. } => (
                self.delays.net(*signal)?,
                bits_of(from).map(StrengthBit::strong).collect(),
                bits_of(to).map(StrengthBit::strong).collect(),
            ),
        };
        (from != to).then(|| {
            from.iter()
                .zip(&to)
                .map(|(from, to)| bit_delay(from.value(), to.value(), delays))
                .max()
                .unwrap_or(0)
        })
    }

    /// What a delayed driver's value drives, bit by bit.
    fn delayed_output(&self, slot: usize, value: &FourStateValue) -> Vec<StrengthBit> {
        let enable = || value.bit(value.width().saturating_sub(1));
        match self.slot_gate(slot) {
            None => bits_of(value).map(StrengthBit::strong).collect(),
            Some(strength::SlotGate::Tristate) => vec![strength::tristate(
                value.bit(0),
                enable(),
                DigitalDriveStrength::STRONG,
            )],
            Some(strength::SlotGate::Mos { resistive, .. }) => vec![strength::mos(
                StrengthBit::strong(value.bit(0)),
                enable(),
                resistive,
            )],
            Some(strength::SlotGate::Tran { .. }) => vec![StrengthBit::strong(enable())],
        }
    }

    /// Give a delayed driver's value to its contribution, now that its delay
    /// has passed.
    pub(crate) fn apply_drive(&mut self, driver: DigitalDriverId, value: FourStateValue) {
        if let Some(slot) = self.contribution_slot(driver) {
            self.apply_slot_drive(slot, driver.signal, value);
        }
    }

    /// Publish a delayed net's value, now that its delay has passed.
    pub(crate) fn apply_net_value(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        if usize::from(signal) < self.values.len() {
            self.publish_settled(signal, value, 0);
        }
    }

    /// A delayed driver contributes `x` until its first value arrives, and a
    /// delayed net with a driver is `x` until its first resolved value does,
    /// as Icarus Verilog's delayed outputs start.
    pub(super) fn initialize_delays(&mut self) {
        if self.delays.tables.drivers.is_empty() && self.delays.tables.nets.is_empty() {
            return;
        }
        // A module path's destination is a delayed net for this purpose.
        let tables = Arc::clone(&self.delays.tables);
        let mut nets = BTreeSet::new();
        for (index, span) in self.spans.iter().enumerate() {
            let signal = DigitalSignalId::from(index);
            for slot in span.start..span.start + span.count {
                if tables.drivers[slot].is_none() {
                    continue;
                }
                let width = self.contribution_width(slot, signal)
                    + u32::from(self.slot_gate(slot).is_some());
                self.contributions[slot].value = ContributionValue::FourState(Some(
                    FourStateValue::splat(width.max(1), FourStateBit::Unknown),
                ));
                nets.insert(signal);
            }
        }
        for signal in nets {
            self.settle(signal, 0);
        }
        for (index, net) in tables.nets.iter().enumerate() {
            if (net.is_some() || tables.paths[index]) && self.spans[index].count > 0 {
                let width = self.widths[index];
                self.values[index] = FourStateValue::splat(width, FourStateBit::Unknown);
            }
        }
        // What starting this way changed is the initial state, not an event.
        self.delays.requests.clear();
        self.transitions.clear();
        self.take_charge_changes();
    }

    /// The width a driver of `signal` in `slot` writes.
    fn contribution_width(&self, slot: usize, signal: DigitalSignalId) -> u32 {
        let index = slot - self.spans[usize::from(signal)].start;
        self.plan
            .drivers_of(signal)
            .nth(index)
            .map_or(1, |driver| match driver.target.select {
                DigitalWriteSelect::Whole => self.widths[usize::from(signal)],
                DigitalWriteSelect::Bit(_) => 1,
                DigitalWriteSelect::Part { msb, lsb } => (msb.abs_diff(lsb) + 1) as u32,
            })
    }
}

fn bits_of(value: &FourStateValue) -> impl Iterator<Item = FourStateBit> + '_ {
    (0..value.width()).map(|offset| value.bit(offset))
}
