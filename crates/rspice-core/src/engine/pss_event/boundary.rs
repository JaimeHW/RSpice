//! The quiet boundary and the nominal mesh.
//!
//! The boundary `t0` lies in the last stabilization period, at the candidate
//! instant farthest from every event: no event within the guard `g` and no
//! `transition` or `slew` ramp in flight. The segment restarts the
//! integrator at order one at `t0`, so the period map depends on the image's
//! accepted state alone.
//!
//! The nominal mesh is the stabilization's accepted points over the window
//! `(t0, t0 + T]`, without the points the stabilization's state-dependent
//! events were landed on: a traversal locates its own events and cuts the
//! step it finds them in, so the old landings would only leave slivers
//! beside the new ones. The period boundary `nT` inside the window is
//! added, so the orbit has a sample at phase zero.

use super::basis::{Probe, ShootingBasis};
use crate::Value;

/// One event a run met: when, which discrete quantity changed, and whether
/// the change follows from the analog state (a crossing or a switch).
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ObservedEvent {
    pub(super) time: Value,
    /// Index of the quantity in the basis's discrete fingerprint.
    pub(super) quantity: usize,
    /// The value it changed to.
    pub(super) value: u64,
    pub(super) state_dependent: bool,
    /// The rate of the crossing expression at the event, when one crossed.
    pub(super) slope: Option<Value>,
}

/// The events between consecutive probes, in time order.
pub(super) fn events(basis: &ShootingBasis, probes: &[Probe]) -> Vec<ObservedEvent> {
    let mut events = Vec::new();
    let fingerprints: Vec<_> = probes
        .iter()
        .map(|probe| basis.fingerprint(probe))
        .collect();
    let crossing_start = basis.crossing_offset();
    for index in 1..probes.len() {
        let (before, after) = (&fingerprints[index - 1], &fingerprints[index]);
        for (quantity, (old, new)) in before.values.iter().zip(&after.values).enumerate() {
            if old == new {
                continue;
            }
            let crossing = quantity
                .checked_sub(crossing_start)
                .filter(|detector| *detector < probes[index].crossings.len());
            let slope = crossing.and_then(|detector| {
                let next = probes.get(index + 1)?;
                let (value, _) = next.crossings.get(detector)?;
                let (here, _) = probes[index].crossings.get(detector)?;
                let dt = next.time - probes[index].time;
                (dt > 0.0).then(|| (value - here) / dt)
            });
            events.push(ObservedEvent {
                time: probes[index].time,
                quantity,
                value: *new,
                state_dependent: basis.is_state_dependent(quantity),
                slope,
            });
        }
    }
    events
}

/// The boundary instant chosen among the candidates.
#[derive(Clone, Debug)]
pub(super) struct BoundaryChoice {
    /// Index of the chosen candidate.
    pub(super) candidate: usize,
    /// Its distance to the nearest event, in seconds.
    pub(super) distance: Value,
}

/// Why no candidate is quiet: the busiest reason any of them gave.
#[derive(Clone, Debug)]
pub(super) enum NoQuietBoundary {
    /// A ramp in flight at every candidate, in this instance.
    Ramping(String),
    /// An event within the guard of every candidate, on this quantity.
    Busy(usize),
}

/// Choose the quietest candidate.
///
/// `events` are the stabilization's events; an event `e` of the last period
/// also stands for `e + T` and `e - T`. A candidate is quiet when no event
/// and no ramp in flight lies within `guard` of it.
pub(super) fn choose(
    candidates: &[(Value, &Probe)],
    events: &[ObservedEvent],
    period: Value,
    guard: Value,
) -> Result<BoundaryChoice, NoQuietBoundary> {
    let mut best: Option<BoundaryChoice> = None;
    let mut ramping = None;
    let mut busiest = None;
    for (index, &(time, probe)) in candidates.iter().enumerate() {
        if let Some(instance) = &probe.ramping {
            ramping.get_or_insert_with(|| instance.clone());
            continue;
        }
        let mut distance = Value::INFINITY;
        let mut nearest = None;
        for event in events {
            for shift in [-period, 0.0, period] {
                let gap = (event.time + shift - time).abs();
                if gap < distance {
                    distance = gap;
                    nearest = Some(event.quantity);
                }
            }
        }
        if distance <= guard {
            busiest = busiest.or(nearest);
            continue;
        }
        if best.as_ref().is_none_or(|best| distance > best.distance) {
            best = Some(BoundaryChoice {
                candidate: index,
                distance,
            });
        }
    }
    best.ok_or(match (ramping, busiest) {
        (Some(instance), _) => NoQuietBoundary::Ramping(instance),
        (None, Some(quantity)) => NoQuietBoundary::Busy(quantity),
        (None, None) => NoQuietBoundary::Busy(0),
    })
}

/// The last whole multiple of the period at or before `end`, as the double
/// nearest the exact multiple. It is formed by dividing by the frequency, not
/// multiplying by the period, so it is the instant a source's repetition
/// lands on; `n * period` can fall an ulp short of it and leave a step of one
/// ulp.
pub(super) fn phase_zero_before(end: Value, period: Value) -> Value {
    (end / period).floor() / period.recip()
}

/// The nominal mesh over `(t0, t0 + period]`.
///
/// `accepted` are a run's accepted times, which must reach `t0 + period`;
/// those in the window are kept except the instants in `excluded` (the run's
/// state-dependent events). The window's end and the period boundary inside
/// the window are always points of the mesh.
pub(super) fn nominal_mesh(
    accepted: &[Value],
    t0: Value,
    period: Value,
    excluded: &[Value],
) -> Vec<Value> {
    let end = t0 + period;
    let spacing_floor = period * 1.0e-9;
    let kept = |time: Value| {
        excluded
            .iter()
            .all(|&event| (event - time).abs() > spacing_floor)
    };
    let mut mesh: Vec<Value> = accepted
        .iter()
        .copied()
        .filter(|&time| time > t0 && time < end && kept(time))
        .collect();
    // An accepted instant within the floor of the boundary is the boundary
    // as the circuit's own sources reach it.
    let multiple = phase_zero_before(end, period);
    let phase_zero = mesh
        .iter()
        .copied()
        .filter(|time| (time - multiple).abs() <= spacing_floor)
        .min_by(|a, b| (a - multiple).abs().total_cmp(&(b - multiple).abs()))
        .unwrap_or(multiple);
    if phase_zero > t0 && phase_zero < end {
        mesh.push(phase_zero);
    }
    mesh.push(end);
    mesh.sort_by(Value::total_cmp);
    let mut thinned: Vec<Value> = Vec::with_capacity(mesh.len());
    let mut previous = t0;
    for time in mesh {
        if time - previous <= spacing_floor {
            // The window's end and the period boundary are kept over a
            // point that sits on top of them.
            if (time == end || time == phase_zero)
                && let Some(last) = thinned.last_mut()
                && *last - t0 > spacing_floor
            {
                *last = time;
                previous = time;
            }
            continue;
        }
        thinned.push(time);
        previous = time;
    }
    if thinned.last() != Some(&end) {
        thinned.push(end);
    }
    thinned
}

/// `mesh` with every interval, including the first from `t0`, halved.
pub(super) fn bisect(mesh: &[Value], t0: Value) -> Vec<Value> {
    let mut refined = Vec::with_capacity(2 * mesh.len());
    let mut previous = t0;
    for &time in mesh {
        let middle = previous + 0.5 * (time - previous);
        if middle > previous && middle < time {
            refined.push(middle);
        }
        refined.push(time);
        previous = time;
    }
    refined
}

/// `mesh` moved by `shift` seconds.
pub(super) fn shifted(mesh: &[Value], shift: Value) -> Vec<Value> {
    mesh.iter().map(|time| time + shift).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_nominal_mesh_keeps_the_window_and_lands_on_phase_zero() {
        let accepted = [1.0, 1.25, 1.6, 1.75, 2.0 + 1.0e-12, 2.25, 2.5, 2.75];
        let mesh = nominal_mesh(&accepted, 1.6, 1.0, &[2.25]);
        assert_eq!(mesh, vec![1.75, 2.0 + 1.0e-12, 2.5, 2.6]);
        let mesh = nominal_mesh(&[1.6, 1.75, 2.25, 2.75], 1.6, 1.0, &[]);
        assert_eq!(mesh, vec![1.75, 2.0, 2.25, 2.6]);
    }

    #[test]
    fn bisection_halves_every_interval_from_the_boundary() {
        assert_eq!(bisect(&[1.0, 2.0], 0.0), vec![0.5, 1.0, 1.5, 2.0]);
    }

    #[test]
    fn the_quietest_candidate_is_the_one_farthest_from_every_event() {
        let probe = |time| Probe::quiet(time);
        let (a, b, c) = (probe(0.1), probe(0.5), probe(0.9));
        let candidates = [(0.1, &a), (0.5, &b), (0.9, &c)];
        let event = ObservedEvent {
            time: 0.0,
            quantity: 3,
            value: 1,
            state_dependent: false,
            slope: None,
        };
        let choice = choose(&candidates, std::slice::from_ref(&event), 1.0, 1e-3).expect("quiet");
        assert_eq!(choice.candidate, 1);
        let busy = choose(
            &candidates[..1],
            &[ObservedEvent { time: 0.1, ..event }],
            1.0,
            1e-3,
        );
        assert!(matches!(busy, Err(NoQuietBoundary::Busy(3))));
    }
}
