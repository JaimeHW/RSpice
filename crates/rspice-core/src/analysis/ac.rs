//! AC Small-Signal Analysis

use crate::Complex64;
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::FrequencyGridError;
use crate::netlist::FreqVariation;

/// Endpoint tolerance scale for the sweep loop; ngspice's acan.c derives
/// it from CKTreltol (default 1e-3) so the final multiplicative step
/// still lands on fstop despite accumulated rounding.
const SWEEP_RELTOL: Value = 1e-3;

/// Generate AC sweep frequencies with ngspice-46 `acan.c` semantics.
///
/// * `Dec`: spans of at least a decade warp the step ratio so that
///   `floor(decades * n)` steps land exactly on `fstop`; shorter spans
///   anchor a pure `10^(1/n)` ratio at `fstart` and end at or below
///   `fstop`.
/// * `Oct`: always anchors a `2^(1/n)` ratio at `fstart` (never warped,
///   `fstop` is generally not hit exactly).
/// * `Lin`: `points` total spread evenly over the range; ngspice's
///   freqDelta degenerates to zero for `points <= 2`, emitting a single
///   point at `fstart`.
///
/// # Legacy compatibility
///
/// This infallible wrapper is deprecated in favor of
/// `try_ac_sweep_frequencies`. It preserves the historical empty-vector
/// sentinel for downstream compatibility, but first-party execution paths
/// must use the checked API so invalid input and resource failures cannot be
/// mistaken for an analysis with no data.
pub fn ac_sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
) -> Vec<Value> {
    try_ac_sweep_frequencies(variation, points, fstart, fstop).unwrap_or_default()
}

/// Generate AC sweep frequencies while preserving validation, capacity, and
/// allocation failures.
pub(crate) fn try_ac_sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
) -> Result<Vec<Value>, FrequencyGridError> {
    try_ac_sweep_frequencies_with_abort(variation, points, fstart, fstop, &NoAbort)
}

/// Cancellable, fallible AC sweep generation with ngspice-46 semantics.
pub fn try_ac_sweep_frequencies_with_abort(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, FrequencyGridError> {
    let (delta, sweep_limit) =
        checked_ac_sweep_parameters(variation, points, fstart, fstop, abort)?;
    let retained_capacity =
        checked_ac_retained_capacity(variation, points, fstart, sweep_limit, delta)?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(retained_capacity)
        .map_err(|_| FrequencyGridError::Allocation {
            requested: retained_capacity,
        })?;
    populate_ac_sweep(
        &mut frequencies,
        variation,
        fstart,
        sweep_limit,
        delta,
        retained_capacity,
        abort,
    )?;
    Ok(frequencies)
}

/// Generate a checked AC sweep without allocating or retaining more points
/// than `max_points`.
///
/// If repeated floating-point stepping proves the sweep exceeds the limit,
/// [`FrequencyGridError::LimitExceeded`] reports `max_points + 1` as the
/// first known retained count above the ceiling. This bounded dry run avoids
/// both oversized allocation and attacker-controlled unbounded preflight.
pub fn try_ac_sweep_frequencies_bounded_with_abort(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
    max_points: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, FrequencyGridError> {
    let (delta, sweep_limit) =
        checked_ac_sweep_parameters(variation, points, fstart, fstop, abort)?;
    let retained_count =
        bounded_ac_sweep_point_count(variation, fstart, sweep_limit, delta, max_points, abort)?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(retained_count)
        .map_err(|_| FrequencyGridError::Allocation {
            requested: retained_count,
        })?;
    populate_ac_sweep(
        &mut frequencies,
        variation,
        fstart,
        sweep_limit,
        delta,
        retained_count,
        abort,
    )?;
    Ok(frequencies)
}

fn checked_ac_sweep_parameters(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    fstop: Value,
    abort: &dyn AbortSignal,
) -> Result<(Value, Value), FrequencyGridError> {
    ensure_not_aborted(abort)?;
    if points == 0 {
        return Err(FrequencyGridError::EmptySweep);
    }
    let valid_start = fstart.is_finite()
        && match variation {
            FreqVariation::Dec | FreqVariation::Oct => fstart > 0.0,
            FreqVariation::Lin => fstart >= 0.0,
        };
    if !valid_start {
        return Err(FrequencyGridError::InvalidStartFrequency);
    }
    if !fstop.is_finite() || fstart > fstop {
        return Err(FrequencyGridError::InvalidStopFrequency);
    }

    let delta = match variation {
        FreqVariation::Dec => {
            if fstop / 10.0 < fstart {
                // Less than a decade apart: pure ratio anchored at fstart.
                if fstop == fstart {
                    1.0
                } else {
                    (std::f64::consts::LN_10 / points as Value).exp()
                }
            } else {
                // Warp the ratio so floor(decades*n) steps span the range.
                let num_steps = ((fstop / fstart).log10().abs() * points as Value).floor();
                ((fstop / fstart).ln() / num_steps).exp()
            }
        }
        FreqVariation::Oct => (std::f64::consts::LN_2 / points as Value).exp(),
        FreqVariation::Lin => {
            if points > 2 {
                (fstop - fstart) / (points - 1) as Value
            } else {
                0.0
            }
        }
    };

    let freq_tol = match variation {
        FreqVariation::Lin => delta * SWEEP_RELTOL,
        _ => delta * fstop * SWEEP_RELTOL,
    };
    // Adding the endpoint tolerance can overflow even though both authored
    // endpoints are finite. An infinite loop bound would admit the first
    // overflowed frequency forever (`inf <= inf`). The tolerance is only an
    // endpoint-rounding allowance, so falling back to the exact finite stop is
    // the only meaningful behavior when that addition overflows.
    let sweep_limit = match fstop + freq_tol {
        limit if limit.is_finite() => limit,
        _ => fstop,
    };

    Ok((delta, sweep_limit))
}

fn populate_ac_sweep(
    frequencies: &mut Vec<Value>,
    variation: FreqVariation,
    fstart: Value,
    sweep_limit: Value,
    delta: Value,
    retained_capacity: usize,
    abort: &dyn AbortSignal,
) -> Result<(), FrequencyGridError> {
    let mut freq = fstart;
    let mut index = 0_usize;
    while freq <= sweep_limit {
        if index.is_multiple_of(256) {
            ensure_not_aborted(abort)?;
        }
        if frequencies.len() == retained_capacity {
            return Err(FrequencyGridError::PointCountOverflow);
        }
        frequencies.push(freq);
        let next = match variation {
            FreqVariation::Lin => {
                if delta == 0.0 {
                    break;
                }
                freq + delta
            }
            _ => {
                if delta == 1.0 {
                    break;
                }
                freq * delta
            }
        };
        // A representable positive delta/ratio can still fail to advance a
        // large floating-point frequency, and the step after the largest
        // finite value can overflow. Neither result can add a valid sweep
        // point; accepting either would make this loop non-terminating.
        if !next.is_finite() || next <= freq {
            break;
        }
        freq = next;
        index = index
            .checked_add(1)
            .ok_or(FrequencyGridError::PointCountOverflow)?;
    }
    ensure_not_aborted(abort)?;
    Ok(())
}

fn bounded_ac_sweep_point_count(
    variation: FreqVariation,
    fstart: Value,
    sweep_limit: Value,
    delta: Value,
    max_points: usize,
    abort: &dyn AbortSignal,
) -> Result<usize, FrequencyGridError> {
    let mut retained_count = 0_usize;
    let mut freq = fstart;
    while freq <= sweep_limit {
        if retained_count.is_multiple_of(256) {
            ensure_not_aborted(abort)?;
        }
        if retained_count == max_points {
            let requested = max_points
                .checked_add(1)
                .ok_or(FrequencyGridError::PointCountOverflow)?;
            return Err(FrequencyGridError::LimitExceeded {
                requested,
                limit: max_points,
            });
        }
        retained_count = retained_count
            .checked_add(1)
            .ok_or(FrequencyGridError::PointCountOverflow)?;
        let next = match variation {
            FreqVariation::Lin => {
                if delta == 0.0 {
                    break;
                }
                freq + delta
            }
            FreqVariation::Dec | FreqVariation::Oct => {
                if delta == 1.0 {
                    break;
                }
                freq * delta
            }
        };
        if !next.is_finite() || next <= freq {
            break;
        }
        freq = next;
    }
    ensure_not_aborted(abort)?;
    Ok(retained_count)
}

fn checked_ac_retained_capacity(
    variation: FreqVariation,
    points: usize,
    fstart: Value,
    sweep_limit: Value,
    delta: Value,
) -> Result<usize, FrequencyGridError> {
    if delta == 0.0 || delta == 1.0 {
        return Ok(1);
    }
    let first_step = match variation {
        FreqVariation::Lin => fstart + delta,
        FreqVariation::Dec | FreqVariation::Oct => fstart * delta,
    };
    if !first_step.is_finite() || first_step <= fstart {
        return Ok(1);
    }
    let raw_steps = match variation {
        FreqVariation::Lin => (sweep_limit - fstart) / delta,
        FreqVariation::Dec | FreqVariation::Oct => (sweep_limit / fstart).ln() / delta.ln(),
    };
    // The loop includes step zero. One additional slot covers the final
    // floating-point endpoint comparison if the analytic quotient rounds
    // just below the integer reached by repeated stepping.
    let capacity = raw_steps.ceil() + 2.0;
    if !capacity.is_finite() || capacity >= usize::MAX as Value {
        return Err(FrequencyGridError::PointCountOverflow);
    }
    let capacity = capacity as usize;
    // Linear ngspice semantics cannot retain more than the authored point
    // count, but using the smaller checked bound avoids enormous speculative
    // reservations when endpoint rounding stops the loop early.
    Ok(match variation {
        FreqVariation::Lin => capacity.min(points.max(1)),
        FreqVariation::Dec | FreqVariation::Oct => capacity.max(1),
    })
}

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), FrequencyGridError> {
    if abort.is_aborted() {
        Err(FrequencyGridError::Aborted)
    } else {
        Ok(())
    }
}

/// AC analysis result at a single frequency
#[derive(Debug, Clone)]
pub struct AcResult {
    /// Frequency
    pub frequency: Value,
    /// Stable node names aligned with `voltages`
    pub node_names: Vec<String>,
    /// Stable branch names aligned with `currents`
    pub branch_names: Vec<String>,
    /// Complex node voltages
    pub voltages: Vec<Complex64>,
    /// Complex branch currents  
    pub currents: Vec<Complex64>,
}

impl AcResult {
    /// Get voltage magnitude at a node (1-indexed, consistent with SPICE)
    pub fn voltage_magnitude(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground is always 0V
        }
        self.voltages.get(node - 1).map(|v| v.norm()).unwrap_or(0.0)
    }

    /// Get voltage phase at a node (in radians, 1-indexed)
    pub fn voltage_phase(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground phase is 0
        }
        self.voltages.get(node - 1).map(|v| v.arg()).unwrap_or(0.0)
    }

    /// Get voltage in dB at a node
    pub fn voltage_db(&self, node: usize) -> Value {
        20.0 * self.voltage_magnitude(node).log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, ImmediateAbort};

    fn sweep(variation: FreqVariation, points: usize, start: Value, stop: Value) -> Vec<Value> {
        try_ac_sweep_frequencies(variation, points, start, stop).expect("valid AC sweep")
    }

    #[test]
    fn decade_sweep_warps_to_span_whole_decade_ranges() {
        // ngspice-46: floor(decades*n) steps land exactly on fstop.
        let f = sweep(FreqVariation::Dec, 10, 1.0, 1.0e6);
        assert_eq!(f.len(), 61);
        assert_eq!(f[0], 1.0);
        let last = *f.last().unwrap();
        assert!((last - 1.0e6).abs() <= 1.0e6 * 1e-9, "last={last}");

        // Non-integer decade count: floor, not ceil (ngspice emits 55
        // points for 1Hz..300kHz at 10/decade, not 56).
        let f = sweep(FreqVariation::Dec, 10, 1.0, 3.0e5);
        assert_eq!(f.len(), 55);
        let last = *f.last().unwrap();
        assert!((last - 3.0e5).abs() <= 3.0e5 * 1e-9, "last={last}");
    }

    #[test]
    fn decade_sweep_anchors_at_fstart_for_sub_decade_ranges() {
        // Span under a decade keeps the pure 10^(1/n) ratio: 100..300 at
        // 10/decade gives 100*10^(k/10) up to 251.19, fstop not hit.
        let f = sweep(FreqVariation::Dec, 10, 100.0, 300.0);
        assert_eq!(f.len(), 5);
        assert_eq!(f[0], 100.0);
        let last = *f.last().unwrap();
        assert!((last - 251.188_643_150_958).abs() < 1e-9, "last={last}");
    }

    #[test]
    fn octave_sweep_never_warps() {
        // 10..50 at 2/octave: 10, 14.1, 20, 28.3, 40 — next step exceeds
        // fstop, which is not included.
        let f = sweep(FreqVariation::Oct, 2, 10.0, 50.0);
        assert_eq!(f.len(), 5);
        let last = *f.last().unwrap();
        assert!((last - 40.0).abs() < 40.0 * 1e-12, "last={last}");
    }

    #[test]
    fn linear_sweep_matches_ngspice_point_counts() {
        let f = sweep(FreqVariation::Lin, 5, 1.0e3, 2.0e3);
        assert_eq!(f.len(), 5);
        assert_eq!(f[0], 1.0e3);
        assert!((f[4] - 2.0e3).abs() < 1e-6);

        // ngspice's freqDelta degenerates to 0 for n <= 2: single point.
        let f = sweep(FreqVariation::Lin, 2, 1.0e3, 2.0e3);
        assert_eq!(f, vec![1.0e3]);
        let f = sweep(FreqVariation::Lin, 1, 1.0e3, 1.0e3);
        assert_eq!(f, vec![1.0e3]);
    }

    #[test]
    fn degenerate_ranges_are_rejected_or_single_point() {
        assert_eq!(
            try_ac_sweep_frequencies(FreqVariation::Dec, 10, 0.0, 1.0e3),
            Err(FrequencyGridError::InvalidStartFrequency)
        );
        assert_eq!(
            try_ac_sweep_frequencies(FreqVariation::Dec, 10, 1.0e3, 1.0),
            Err(FrequencyGridError::InvalidStopFrequency)
        );
        assert_eq!(
            try_ac_sweep_frequencies(FreqVariation::Dec, 0, 1.0, 1.0e3),
            Err(FrequencyGridError::EmptySweep)
        );
        // Equal start/stop emits exactly one point for log sweeps.
        assert_eq!(sweep(FreqVariation::Dec, 10, 5.0e3, 5.0e3), vec![5.0e3]);
        assert_eq!(sweep(FreqVariation::Oct, 10, 5.0e3, 5.0e3), vec![5.0e3]);
    }

    #[test]
    fn extreme_linear_sweep_never_admits_an_infinite_frequency() {
        let frequencies = sweep(FreqVariation::Lin, 3, 0.0, Value::MAX);
        assert_eq!(frequencies, vec![0.0, Value::MAX / 2.0, Value::MAX]);
        assert!(frequencies.iter().all(|frequency| frequency.is_finite()));
    }

    #[test]
    fn linear_sweep_stops_when_rounding_prevents_forward_progress() {
        let start = 1.0e300;
        let stop = 1.000_000_000_000_001e300;
        assert_eq!(
            try_ac_sweep_frequencies(FreqVariation::Lin, usize::MAX, start, stop)
                .expect("non-advancing checked grid remains a valid single point"),
            vec![start]
        );
    }

    #[test]
    fn checked_sweep_reports_allocation_and_cancellation() {
        assert_eq!(
            try_ac_sweep_frequencies(FreqVariation::Lin, usize::MAX / 2, 0.0, 1.0),
            Err(FrequencyGridError::Allocation {
                requested: usize::MAX / 2
            })
        );
        assert_eq!(
            try_ac_sweep_frequencies_with_abort(
                FreqVariation::Dec,
                10,
                1.0,
                1.0e3,
                &ImmediateAbort,
            ),
            Err(FrequencyGridError::Aborted)
        );
        let abort = CountingAbort::new(1);
        assert_eq!(
            try_ac_sweep_frequencies_with_abort(FreqVariation::Lin, 600, 1.0, 2.0, &abort,),
            Err(FrequencyGridError::Aborted)
        );
    }

    #[test]
    fn bounded_sweep_preflights_exact_small_grids_and_stops_at_the_ceiling() {
        assert_eq!(
            try_ac_sweep_frequencies_bounded_with_abort(
                FreqVariation::Dec,
                10,
                1.0,
                1.0e3,
                31,
                &NoAbort,
            )
            .expect("31-point decade grid fits the bound")
            .len(),
            31
        );
        assert_eq!(
            try_ac_sweep_frequencies_bounded_with_abort(
                FreqVariation::Lin,
                usize::MAX / 2,
                0.0,
                1.0,
                10,
                &NoAbort,
            ),
            Err(FrequencyGridError::LimitExceeded {
                requested: 11,
                limit: 10,
            })
        );
        assert_eq!(
            try_ac_sweep_frequencies_bounded_with_abort(
                FreqVariation::Lin,
                3,
                0.0,
                1.0,
                0,
                &NoAbort,
            ),
            Err(FrequencyGridError::LimitExceeded {
                requested: 1,
                limit: 0,
            })
        );
    }
}
