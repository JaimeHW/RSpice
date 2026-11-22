//! An eye triggered from a clock signal: every acquisition starts at one of
//! the clock's rising threshold crossings, as a sampling scope triggered
//! from a clock channel acquires it, so jitter the clock shares with the
//! data leaves the eye.
//!
//! The data is placed in the window by where its own crossings sit after
//! the clock's edges: half a unit interval in, so the opening lands at the
//! window's centre as it does for a recovered or an ideal clock, unless the
//! reader states the delay.

use std::f64::consts::TAU;

/// Fewest rising edges a clock triggers an eye from: two periods.
pub const MIN_CLOCK_EDGES: usize = 3;

/// The rising edges of a clock signal and the period they run at.
#[derive(Debug, Clone, PartialEq)]
pub struct ClockTrigger {
    /// Each rising crossing of the threshold, interpolated, in seconds.
    pub edges: Vec<f64>,
    /// The median spacing of the edges, in seconds: the unit interval the
    /// eye is drawn at.
    pub period: f64,
}

/// Why a clock signal triggers no eye.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockTriggerRejection {
    /// The clock crosses its threshold upward fewer than
    /// [`MIN_CLOCK_EDGES`] times.
    TooFewEdges(usize),
}

impl std::fmt::Display for ClockTriggerRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewEdges(count) => write!(
                f,
                "the clock crosses its threshold upward {count} times, fewer than the \
                 {MIN_CLOCK_EDGES} an eye is triggered from"
            ),
        }
    }
}

/// The rising crossings of `threshold` by `clock` sampled at `time`.
pub fn clock_trigger(
    time: &[f64],
    clock: &[f64],
    threshold: f64,
) -> Result<ClockTrigger, ClockTriggerRejection> {
    let n = time.len().min(clock.len());
    let mut edges = Vec::new();
    for i in 1..n {
        let (v0, v1) = (clock[i - 1], clock[i]);
        let (t0, t1) = (time[i - 1], time[i]);
        if !(v0.is_finite() && v1.is_finite() && t0.is_finite() && t1.is_finite()) {
            continue;
        }
        if v0 < threshold && v1 >= threshold && v1 > v0 {
            edges.push(t0 + (threshold - v0) / (v1 - v0) * (t1 - t0));
        }
    }
    if edges.len() < MIN_CLOCK_EDGES {
        return Err(ClockTriggerRejection::TooFewEdges(edges.len()));
    }
    let mut spacings: Vec<f64> = edges.windows(2).map(|pair| pair[1] - pair[0]).collect();
    spacings.sort_by(f64::total_cmp);
    let period = spacings[spacings.len() / 2];
    if !(period.is_finite() && period > 0.0) {
        return Err(ClockTriggerRejection::TooFewEdges(edges.len()));
    }
    Ok(ClockTrigger { edges, period })
}

/// The delay after each clock edge that puts the data's mean crossing half
/// a period into the window: the circular mean of every data crossing's
/// time after the edge before it, less half a period. `None` when the data
/// never crosses `level` between two edges.
pub fn centring_delay(
    trigger: &ClockTrigger,
    time: &[f64],
    signal: &[f64],
    level: f64,
) -> Option<f64> {
    let n = time.len().min(signal.len());
    let (mut cos_sum, mut sin_sum, mut count) = (0.0, 0.0, 0usize);
    let mut edge = 0usize;
    for i in 1..n {
        let (v0, v1) = (signal[i - 1], signal[i]);
        if !(v0.is_finite() && v1.is_finite()) || (v0 - level) * (v1 - level) > 0.0 || v0 == v1 {
            continue;
        }
        let crossing = time[i - 1] + (level - v0) / (v1 - v0) * (time[i] - time[i - 1]);
        while edge + 1 < trigger.edges.len() && trigger.edges[edge + 1] <= crossing {
            edge += 1;
        }
        let Some(&before) = trigger.edges.get(edge) else {
            break;
        };
        if crossing < before {
            continue;
        }
        let angle = TAU * ((crossing - before) / trigger.period).rem_euclid(1.0);
        cos_sum += angle.cos();
        sin_sum += angle.sin();
        count += 1;
    }
    if count == 0 {
        return None;
    }
    let phase = (trigger.period / TAU) * sin_sum.atan2(cos_sum);
    Some((phase - 0.5 * trigger.period).rem_euclid(trigger.period))
}

#[cfg(test)]
mod tests {
    use super::*;

    const UI: f64 = 1e-9;

    /// A 0-to-1.2 V clock at `period`, linear edges of a tenth of a period,
    /// each rising edge moved by `jitter(k)`.
    fn clock(period: f64, cycles: usize, jitter: impl Fn(usize) -> f64) -> (Vec<f64>, Vec<f64>) {
        let dt = period / 200.0;
        let edge = 0.1 * period;
        let end = period * cycles as f64;
        let samples = (end / dt) as usize;
        let mut time = Vec::with_capacity(samples);
        let mut value = Vec::with_capacity(samples);
        for i in 0..samples {
            let t = i as f64 * dt;
            let k = (t / period).floor();
            let start = k * period + 0.25 * period + jitter(k as usize);
            let rise = ((t - start) / edge).clamp(0.0, 1.0);
            let fall = ((t - start - 0.5 * period) / edge).clamp(0.0, 1.0);
            time.push(t);
            value.push(1.2 * (rise - fall));
        }
        (time, value)
    }

    #[test]
    fn a_clock_triggers_at_its_rising_threshold_crossings() {
        let (time, value) = clock(UI, 20, |_| 0.0);
        let trigger = clock_trigger(&time, &value, 0.6).expect("twenty edges");
        assert_eq!(trigger.edges.len(), 20);
        assert!(
            (trigger.period - UI).abs() < 1e-6 * UI,
            "{}",
            trigger.period
        );
        // Half way up a tenth-of-a-period edge that starts a quarter in.
        let first = 0.25 * UI + 0.05 * UI;
        assert!((trigger.edges[0] - first).abs() < 1e-6 * UI);
    }

    #[test]
    fn a_clock_with_too_few_edges_triggers_nothing() {
        let (time, value) = clock(UI, 2, |_| 0.0);
        assert_eq!(
            clock_trigger(&time, &value, 0.6),
            Err(ClockTriggerRejection::TooFewEdges(2))
        );
    }

    /// Data that crosses 0.3 UI after every clock edge is centred by a
    /// delay that puts that crossing half a period in: 0.8 UI after the
    /// edge, wrapped into the period.
    #[test]
    fn the_delay_puts_the_data_crossings_half_a_period_in() {
        let (time, clock_value) = clock(UI, 40, |_| 0.0);
        let trigger = clock_trigger(&time, &clock_value, 0.6).expect("forty edges");
        // Alternating bits at the clock's rate are a square wave of twice
        // its period, whose crossings sit 0.6 UI into every unit interval:
        // 0.3 UI after each clock edge.
        let (data_time, data) = clock(2.0 * UI, 20, |_| 0.0);
        let delay = centring_delay(&trigger, &data_time, &data, 0.6).expect("data crosses");
        assert!((delay - 0.8 * UI).abs() < 1e-3 * UI, "delay {delay:e}");
    }
}
