//! Deterministic NRZ stimulus for the eye-diagram oracles.
//!
//! Every eye oracle in this module tree is checked against a closed-form
//! answer rather than a recorded one, so the stimulus has to be exactly
//! describable: a trapezoidal NRZ waveform whose 50 % crossings sit at times
//! the test chooses and whose 20–80 % edge is a stated constant. Both the
//! unit-interval estimator and the eye measurements read the same generator,
//! so a fold-alignment change cannot be "verified" against a signal that was
//! quietly regenerated to match it.

/// 20–80 % edge time of the generated trapezoid.
pub(super) const RISE_2080: f64 = 50e-12;
/// Sample step of the generated record.
pub(super) const DT: f64 = 10e-12;
/// Nominal unit interval of the generated patterns.
pub(super) const UI: f64 = 1e-9;

/// Full 0–100 % ramp width that yields [`RISE_2080`] between the 20 % and
/// 80 % levels of a linear edge.
pub(super) fn ramp_width(rise_2080: f64) -> f64 {
    rise_2080 / 0.6
}

/// A level change: the time of the 50 % crossing and the level held after it.
pub(super) type LevelEvent = (f64, f64);

/// Sample a trapezoidal waveform defined by its level changes.
///
/// Each event's linear ramp is centred on the event time and is
/// [`ramp_width`] wide, so the 50 % crossing lands exactly on the event time
/// and the measured 20–80 % edge is exactly `rise_2080`. Events must be
/// ascending and separated by more than one ramp width.
pub(super) fn trapezoid(
    initial: f64,
    events: &[LevelEvent],
    t_end: f64,
    dt: f64,
    rise_2080: f64,
) -> (Vec<f64>, Vec<f64>) {
    let width = ramp_width(rise_2080);
    let half = 0.5 * width;
    let count = ((t_end / dt).floor() as usize).saturating_add(1);
    let mut time = Vec::with_capacity(count);
    let mut signal = Vec::with_capacity(count);

    let mut cursor = 0usize;
    for index in 0..count {
        let t = index as f64 * dt;
        while cursor < events.len() && t >= events[cursor].0 + half {
            cursor += 1;
        }
        let held = if cursor == 0 {
            initial
        } else {
            events[cursor - 1].1
        };
        let value = match events.get(cursor) {
            Some(&(event_time, level)) if t > event_time - half => {
                let alpha = ((t - (event_time - half)) / width).clamp(0.0, 1.0);
                held + alpha * (level - held)
            }
            _ => held,
        };
        time.push(t);
        signal.push(value);
    }

    (time, signal)
}

/// Alternating 1010 clock: `bits` unit intervals, one transition per bit.
///
/// `jitter(n)` displaces bit `n`'s crossing; pass `|_| 0.0` for an ideal
/// clock.
pub(super) fn clock_events(
    bits: usize,
    t_start: f64,
    ui: f64,
    low: f64,
    high: f64,
    jitter: impl Fn(usize) -> f64,
) -> (f64, Vec<LevelEvent>) {
    let events = (0..bits)
        .map(|n| {
            let level = if n.is_multiple_of(2) { high } else { low };
            (t_start + n as f64 * ui + jitter(n), level)
        })
        .collect();
    // Bit 0 rises, so the record starts low.
    (low, events)
}

/// Alternating clock whose rising edges overshoot to `peak` and settle back
/// to `high` after `settle_delay`.
///
/// Asymmetric overshoot is what moves an eye's crossing point off mid-swing:
/// the rising family reaches the crossing level on a steeper ramp than the
/// falling family leaves it, so the two edge families intersect above the
/// half-amplitude point.
pub(super) fn overshooting_clock_events(
    bits: usize,
    t_start: f64,
    ui: f64,
    low: f64,
    high: f64,
    peak: f64,
    settle_delay: f64,
) -> (f64, Vec<LevelEvent>) {
    let mut events = Vec::with_capacity(bits + bits / 2);
    for n in 0..bits {
        let edge = t_start + n as f64 * ui;
        if n.is_multiple_of(2) {
            events.push((edge, peak));
            events.push((edge + settle_delay, high));
        } else {
            events.push((edge, low));
        }
    }
    (low, events)
}

/// NRZ level changes for a bit pattern, one bit per unit interval.
pub(super) fn nrz_events(
    bits: &[bool],
    t_start: f64,
    ui: f64,
    low: f64,
    high: f64,
) -> (f64, Vec<LevelEvent>) {
    let level = |bit: bool| if bit { high } else { low };
    let Some(&first) = bits.first() else {
        return (low, Vec::new());
    };
    let mut events = Vec::new();
    for n in 1..bits.len() {
        if bits[n] != bits[n - 1] {
            events.push((t_start + n as f64 * ui, level(bits[n])));
        }
    }
    (level(first), events)
}

/// Duty-cycle-distorted clock: the high phase takes `duty_high` of each
/// two-unit-interval clock cycle instead of half.
pub(super) fn dcd_clock_events(
    cycles: usize,
    t_start: f64,
    ui: f64,
    duty_high: f64,
    low: f64,
    high: f64,
) -> (f64, Vec<LevelEvent>) {
    let period = 2.0 * ui;
    let mut events = Vec::with_capacity(cycles * 2);
    for cycle in 0..cycles {
        let start = t_start + cycle as f64 * period;
        events.push((start, high));
        events.push((start + duty_high * period, low));
    }
    (low, events)
}

/// PRBS-7 (`x^7 + x^6 + 1`) bit sequence, seeded to the all-ones state.
pub(super) fn prbs7_bits(count: usize) -> Vec<bool> {
    let mut state: u8 = 0x7f;
    (0..count)
        .map(|_| {
            let bit = ((state >> 6) ^ (state >> 5)) & 1;
            state = ((state << 1) | bit) & 0x7f;
            bit == 1
        })
        .collect()
}

/// Level changes for a waveform that toggles at the union of two square
/// waves' edges — the classic two-incommensurate-clocks case, which has no
/// single bit period to find.
pub(super) fn beating_toggle_events(
    period_a: f64,
    period_b: f64,
    t_end: f64,
    low: f64,
    high: f64,
) -> (f64, Vec<LevelEvent>) {
    let mut times: Vec<f64> = Vec::new();
    for (period, offset) in [(period_a, 0.0), (period_b, 0.17e-9)] {
        let half = 0.5 * period;
        let mut t = offset + half;
        while t < t_end {
            times.push(t);
            t += half;
        }
    }
    times.sort_by(f64::total_cmp);
    let mut is_high = true;
    let events = times
        .into_iter()
        .map(|t| {
            is_high = !is_high;
            (t, if is_high { high } else { low })
        })
        .collect();
    (high, events)
}
