//! The repeating `EXP` source: `EXP(V1 V2 TD1 TAU1 TD2 TAU2 PERIOD [N [BURST]])`.
//!
//! QSPICE and LTspice both read values past SPICE's six and mean different
//! things by them, so a deck says which it was written for (`.options
//! rspice_exp_repeat=`, QSPICE's reading by default):
//!
//! - QSPICE counts `TD2` from `TD1` and picks each repeated pulse up on the
//!   first pulse's curve, at the level that pulse had reached when the repeat
//!   began, so a train that overlaps its own tail jumps at each repeat.
//! - LTspice keeps SPICE's absolute `TD2` and adds each pulse to the tails of
//!   the ones before it.

use crate::Value;

/// Past this many time constants an exponential has settled to within 1e-18.
const SETTLED_TIME_CONSTANTS: Value = 41.5;

/// Whose meaning a repeating `EXP` carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ExpTrainDialect {
    #[default]
    Qspice,
    Ltspice,
}

/// A repeating `EXP` waveform. [`ExpTrain::new`] settles what its values mean.
#[derive(Debug, Clone)]
pub struct ExpTrain {
    pub v1: Value,
    pub v2: Value,
    pub td1: Value,
    pub tau1: Value,
    pub td2: Value,
    pub tau2: Value,
    /// Spacing of the pulses in a train.
    pub period: Value,
    /// Pulses per train, never zero; `None` repeats for ever.
    pulses: Option<u64>,
    /// Spacing of the trains.
    burst: Option<Value>,
    dialect: ExpTrainDialect,
    /// How far along the first pulse's curve a repeated pulse starts, and
    /// how much further the first pulse of a later train does (QSPICE).
    pulse_shift: Value,
    burst_shift: Value,
}

impl ExpTrain {
    /// `timing` is `[TD1, TAU1, TD2, TAU2, PERIOD]`. A count or spacing that
    /// repeats nothing leaves a single pulse, as the simulator named does;
    /// what QSPICE refuses or answers with NaN for is refused here by name.
    pub(crate) fn new(
        dialect: ExpTrainDialect,
        [v1, v2]: [Value; 2],
        [td1, tau1, td2, tau2, period]: [Value; 5],
        pulses: Option<u64>,
        burst: Option<Value>,
    ) -> Result<Self, String> {
        let mut train = Self {
            v1,
            v2,
            td1,
            tau1,
            td2,
            tau2,
            period,
            pulses,
            burst: burst.filter(|burst| *burst > 0.0),
            dialect,
            pulse_shift: 0.0,
            burst_shift: 0.0,
        };
        match dialect {
            ExpTrainDialect::Qspice => train.settle_qspice()?,
            ExpTrainDialect::Ltspice => train.settle_ltspice(),
        }
        Ok(train)
    }

    fn settle_ltspice(&mut self) {
        if self.period <= 0.0 || self.pulses == Some(0) {
            self.period = 0.0;
            self.pulses = Some(1);
            self.burst = None;
        }
    }

    fn settle_qspice(&mut self) -> Result<(), String> {
        if self.tau1 <= 0.0 || self.tau2 <= 0.0 {
            return Err("EXP: a repeating pulse needs positive TAU1 and TAU2".to_owned());
        }
        if self.period <= 0.0 || self.period < self.td2 {
            return Err("EXP: the repeat delay TD3 must be positive and at least TD2".to_owned());
        }
        self.pulses = self.pulses.filter(|pulses| *pulses > 0);
        if let Some(burst) = self.burst {
            let Some(pulses) = self.pulses else {
                return Err("EXP: TPERIOD repeats a counted train, so it needs N1".to_owned());
            };
            if (pulses - 1) as Value * self.period >= burst {
                return Err("EXP: all N1 pulses must start within TPERIOD".to_owned());
            }
        }
        self.pulse_shift = self.shift_to_level(self.period);
        if let (Some(burst), Some(pulses)) = (self.burst, self.pulses) {
            let elapsed = burst - (pulses - 1) as Value * self.period;
            self.burst_shift = self.shift_to_level(elapsed + self.pulse_shift);
        }
        if !(self.pulse_shift.is_finite() && self.burst_shift.is_finite()) {
            return Err(
                "EXP: the pulse reaches V2 where a repeat begins, leaving it \
                        no level to resume from"
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// Where on the rise a pulse reaches the level the first pulse had `s`
    /// after it began. QSPICE applies the fall term here at any `s`, even
    /// ahead of `TD2`, unlike on the curve itself.
    fn shift_to_level(&self, s: Value) -> Value {
        let rise = -(-s / self.tau1).exp_m1();
        let fall = -(-(s - self.td2) / self.tau2).exp_m1();
        -self.tau1 * (1.0 - (rise - fall)).ln()
    }

    /// The earliest time this source can differ from `V1`.
    #[doc(hidden)]
    pub fn first_edge(&self) -> Value {
        match self.dialect {
            ExpTrainDialect::Qspice => self.td1,
            ExpTrainDialect::Ltspice => self.td1.min(self.td2),
        }
    }

    /// Scale the levels, as a source multiplier does.
    pub(crate) fn scale_levels(&mut self, factor: Value) {
        self.v1 *= factor;
        self.v2 *= factor;
    }

    /// Value and slope at `time`; `left` asks for the limit from the left.
    #[doc(hidden)]
    pub fn value_and_slope(&self, time: Value, left: bool) -> (Value, Value) {
        let (shape, slope) = match self.dialect {
            ExpTrainDialect::Qspice => self.qspice_shape(time, left),
            ExpTrainDialect::Ltspice => self.ltspice_shape(time, left),
        };
        let span = self.v2 - self.v1;
        (self.v1 + span * shape, span * slope)
    }

    fn repeat_shift(&self, train: u64, index: u64) -> Value {
        let pulse = if index >= 1 { self.pulse_shift } else { 0.0 };
        let burst = if train >= 1 { self.burst_shift } else { 0.0 };
        pulse + burst
    }

    fn qspice_shape(&self, time: Value, left: bool) -> (Value, Value) {
        if time < self.td1 || (left && time == self.td1) {
            return (0.0, 0.0);
        }
        let train = self
            .burst
            .map_or(0, |burst| latest_index(time, self.td1, burst, left));
        let train_start = self.td1 + train as Value * self.burst.unwrap_or(0.0);
        let mut index = latest_index(time, train_start, self.period, left);
        if let Some(pulses) = self.pulses {
            index = index.min(pulses - 1);
        }
        let s = time - (train_start + index as Value * self.period);
        self.qspice_curve(s + self.repeat_shift(train, index))
    }

    /// The first pulse's curve. A repeat that picks up below `V1`, on a tail
    /// that undershot, holds `V1` until its shifted time reaches zero.
    fn qspice_curve(&self, s: Value) -> (Value, Value) {
        if s < 0.0 {
            return (0.0, 0.0);
        }
        let mut value = -(-s / self.tau1).exp_m1();
        let mut slope = (-s / self.tau1).exp() / self.tau1;
        if s > self.td2 {
            let fall = -(s - self.td2) / self.tau2;
            value += fall.exp_m1();
            slope -= fall.exp() / self.tau2;
        }
        (value, slope)
    }

    fn ltspice_shape(&self, time: Value, left: bool) -> (Value, Value) {
        let first = self.first_edge();
        let spacing = self.burst.unwrap_or(0.0);
        let latest_train = self
            .burst
            .map_or(0, |burst| latest_index(time, first, burst, false));
        let trains_are_disjoint = self
            .pulses
            .is_some_and(|pulses| pulses as Value * self.period <= spacing);
        let (mut shape, mut slope) = (0.0, 0.0);
        'trains: for train in (0..=latest_train).rev() {
            let train_start = train as Value * spacing;
            let mut latest = latest_index(time, first + train_start, self.period, false);
            if let Some(pulses) = self.pulses {
                latest = latest.min(pulses - 1);
            }
            // Newest first: a settled pulse adds nothing, nor does any older.
            for index in (0..=latest).rev() {
                let start = train_start + index as Value * self.period;
                let (rise, rise_slope, rise_settled) =
                    onset(time - (self.td1 + start), self.tau1, left);
                let (fall, fall_slope, fall_settled) =
                    onset(time - (self.td2 + start), self.tau2, left);
                shape += rise - fall;
                slope += rise_slope - fall_slope;
                if rise_settled && fall_settled {
                    if index == latest && trains_are_disjoint {
                        break 'trains;
                    }
                    break;
                }
            }
        }
        (shape, slope)
    }

    /// Every rise and fall edge up to `stop`, in pulse order.
    pub fn edges_until(&self, stop: Value) -> impl Iterator<Item = Value> + '_ {
        let first = self.first_edge();
        let spacing = self.burst.unwrap_or(0.0);
        let trains = (stop >= first).then(|| {
            self.burst
                .map_or(0, |burst| latest_index(stop, first, burst, false))
        });
        trains
            .into_iter()
            .flat_map(|trains| 0..=trains)
            .flat_map(move |train| {
                let train_offset = train as Value * spacing;
                let last = self.pulses.map_or_else(
                    || latest_index(stop, first + train_offset, self.period, false),
                    |pulses| pulses - 1,
                );
                (0..=last)
                    .take_while(move |index| {
                        first + train_offset + *index as Value * self.period <= stop
                    })
                    .flat_map(move |index| self.pulse_edges(train, index))
            })
    }

    fn pulse_edges(&self, train: u64, index: u64) -> [Value; 2] {
        let train_offset = train as Value * self.burst.unwrap_or(0.0);
        match self.dialect {
            ExpTrainDialect::Qspice => {
                let start = self.td1 + train_offset + index as Value * self.period;
                let fall = start + self.td2 - self.repeat_shift(train, index);
                [start, fall.max(start)]
            }
            ExpTrainDialect::Ltspice => {
                let offset = train_offset + index as Value * self.period;
                [self.td1 + offset, self.td2 + offset]
            }
        }
    }
}

/// Value, slope and whether settled, `x` after one exponential onset.
fn onset(x: Value, tau: Value, left: bool) -> (Value, Value, bool) {
    if x < 0.0 || (x == 0.0 && left) {
        return (0.0, 0.0, false);
    }
    if tau <= 0.0 {
        return (1.0, 0.0, true);
    }
    let ratio = -x / tau;
    (
        -ratio.exp_m1(),
        ratio.exp() / tau,
        -ratio > SETTLED_TIME_CONSTANTS,
    )
}

/// The last index whose start `base + index * period` has been reached at
/// `time`. The division can round across a boundary, so the answer is settled
/// on the same sum the breakpoints land on.
fn latest_index(time: Value, base: Value, period: Value, left: bool) -> u64 {
    if period <= 0.0 || time <= base {
        return 0;
    }
    let reached = |index: u64| {
        let boundary = base + index as Value * period;
        time > boundary || (time == boundary && !left)
    };
    let index = ((time - base) / period).floor() as u64;
    if reached(index + 1) {
        index + 1
    } else if index > 0 && !reached(index) {
        index - 1
    } else {
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const US: Value = 1e-6;

    fn train(
        dialect: ExpTrainDialect,
        timing: [Value; 5],
        pulses: Option<u64>,
        burst: Option<Value>,
    ) -> Result<ExpTrain, String> {
        ExpTrain::new(
            dialect,
            [0.0, 1.0],
            timing.map(|time| time * US),
            pulses,
            burst.map(|burst| burst * US),
        )
    }

    fn qspice(timing: [Value; 5], pulses: Option<u64>, burst: Option<Value>) -> ExpTrain {
        train(ExpTrainDialect::Qspice, timing, pulses, burst).expect("QSPICE runs this source")
    }

    fn ltspice(timing: [Value; 5], pulses: Option<u64>, burst: Option<Value>) -> ExpTrain {
        train(ExpTrainDialect::Ltspice, timing, pulses, burst).expect("LTspice runs this source")
    }

    fn assert_samples(train: &ExpTrain, samples: &[(Value, Value)]) {
        for &(time, expected) in samples {
            let value = train.value_and_slope(time * US, false).0;
            assert!(
                (value - expected).abs() < 5e-5,
                "{time} us: {value} vs {expected}"
            );
        }
    }

    // QSPICE64 build Oct 27 2026: a repeat jumps down to where the first pulse
    // stood, from the end of the pulse before it.
    #[test]
    fn a_repeat_resumes_on_the_first_pulse_curve_and_jumps() {
        let train = qspice([1.0, 1.0, 5.0, 2.0, 7.0], None, None);
        assert_samples(
            &train,
            &[(14.9, 0.307066), (14.99, 0.293581), (15.01, 0.373266)],
        );
        let repeat = train.edges_until(30.0 * US).nth(4).expect("third pulse");
        let left = train.value_and_slope(repeat, true).0;
        assert!((left - 0.292120).abs() < 5e-5, "{left}");
        let right = train.value_and_slope(repeat, false).0;
        assert!((right - 0.366968).abs() < 5e-5, "{right}");
    }

    // QSPICE64 build Oct 27 2026: a tail that undershot leaves the repeat at
    // V1 for a while.
    #[test]
    fn a_repeat_from_an_undershoot_holds_v1_first() {
        let train = qspice([1.0, 10.0, 5.0, 1.0, 20.0], None, None);
        assert_samples(
            &train,
            &[
                (10.0, -0.388254),
                (21.5, 0.0),
                (22.0, 0.0),
                (23.0, 0.070466),
                (25.0, 0.238962),
            ],
        );
    }

    // QSPICE64 build Oct 27 2026: the level a later train resumes from is
    // read one repeat shift past the end of the last pulse, with the fall term
    // applied even ahead of TD2.
    #[test]
    fn a_later_train_resumes_from_the_shifted_end_level() {
        let one_pulse = qspice([1.0, 1.0, 5.0, 20.0, 7.0], Some(1), Some(20.0));
        assert_samples(
            &one_pulse,
            &[(21.01, 0.425925), (22.0, 0.786687), (24.0, 0.971131)],
        );
        let early_fall = qspice([1.0, 1.0, 3.0, 20.0, 7.0], Some(2), Some(8.0));
        assert_samples(&early_fall, &[(9.01, 0.948470), (12.0, 0.860030)]);
    }

    // QSPICE64 build Oct 27 2026 stops on the first three and returns NaN for
    // the rest.
    #[test]
    fn a_repeat_qspice_cannot_run_is_refused() {
        for (timing, pulses, burst, why) in [
            ([1.0, 1.0, 20.0, 2.0, 7.0], None, None, "at least TD2"),
            ([1.0, 1.0, 5.0, 2.0, 0.0], None, None, "at least TD2"),
            (
                [1.0, 1.0, 5.0, 2.0, 7.0],
                Some(10),
                Some(50.0),
                "within TPERIOD",
            ),
            ([1.0, 0.0, 5.0, 2.0, 7.0], None, None, "positive TAU1"),
            ([1.0, 1.0, 5.0, 0.0, 7.0], None, None, "positive TAU1"),
            ([1.0, 1.0, 5.0, 2.0, 7.0], Some(0), Some(50.0), "needs N1"),
            ([1.0, 1.0, 5.0, 2.0, 7.0], Some(8), Some(50.0), "no level"),
        ] {
            let refusal = train(ExpTrainDialect::Qspice, timing, pulses, burst)
                .expect_err("QSPICE does not run this source");
            assert!(refusal.contains(why), "{refusal}");
        }
    }

    // LTspice 26.1.1: TD2 is absolute, so a fall scheduled ahead of its rise
    // dips below V1, once for every repeat.
    #[test]
    fn an_early_fall_dips_below_v1_at_every_repeat() {
        let train = ltspice([5.0, 1.0, 2.0, 2.0, 12.0], None, None);
        assert_samples(
            &train,
            &[
                (3.0, -0.393467),
                (5.0, -0.776870),
                (15.0, -0.392009),
                (27.0, -0.392006),
            ],
        );
        assert!((train.first_edge() - 2.0 * US).abs() < 1e-18);
    }

    // LTspice 26.1.1: no spacing leaves one pulse, whatever else the card says.
    #[test]
    fn a_zero_period_is_one_pulse_even_with_a_burst() {
        let train = ltspice([1.0, 1.0, 5.0, 2.0, 0.0], Some(3), Some(20.0));
        assert_samples(
            &train,
            &[(6.0, 0.599795), (9.0, 0.135000), (21.0, 0.000335)],
        );
    }

    #[test]
    fn a_published_edge_lands_on_the_boundary_the_waveform_compares_against() {
        let train = qspice([1.0, 1.0, 5.0, 2.0, 7.0], None, None);
        let starts: Vec<_> = train.edges_until(30.0 * US).step_by(2).collect();
        assert_eq!(starts.len(), 5);
        for &start in &starts[2..] {
            let left = train.value_and_slope(start, true).0;
            let right = train.value_and_slope(start, false).0;
            assert!(right - left > 0.07, "{start}: {left} to {right}");
        }
    }
}
