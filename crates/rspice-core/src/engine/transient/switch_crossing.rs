//! Where an ideal switch changes inside a step of a locked segment.
//!
//! A segment from a live image (an event-shooting traversal) runs on a
//! locked mesh, and an ideal switch whose control crosses its threshold
//! inside one of its steps is cut there, so the change is dated where it
//! happens and the period map moves it with the state. Every candidate of
//! the cut solves the same step from the same accepted point to a different
//! end, so the step's path is a function of its end time and the crossing is
//! the root of the controls along it. [`SwitchCrossingSearch`] brackets that
//! root between the latest end on each side and takes the next end where the
//! controls' chord crosses, until the two sides are within the step's
//! resolution. Only the end that dates the change is accepted.
//!
//! A cut once accepted every candidate that fell short of the crossing, each
//! closer than the last: steps a solver floor long, whose trapezoid turned
//! roundoff into the derivative history the next step integrated, so the
//! period map moved by 5e-8 V between iterates 1e-5 V apart and its
//! linearization could not be certified.

use crate::Value;
use crate::circuit::CircuitData;

/// How many ends a search may try before it keeps the side past the
/// crossing as it stands.
const SEARCH_ENDS: usize = 64;

/// The controls of every ideal switch at one end of a step, voltage switches
/// first, in circuit order; a switch that is not ideal reads NaN.
fn controls(circuit: &CircuitData, solution: &[Value], candidate: bool) -> Vec<Value> {
    circuit
        .vswitches
        .iter()
        .map(|switch| switch.crossing_control(solution, candidate))
        .chain(
            circuit
                .iswitches
                .iter()
                .map(|switch| switch.crossing_control(solution, candidate)),
        )
        .map(|control| control.unwrap_or(Value::NAN))
        .collect()
}

/// The earliest fraction of the chord from `from` to `to` at which an ideal
/// switch's control crosses the threshold that changes its accepted state.
/// `from(i)` and `to(i)` give the `i`th switch's control in [`controls`]
/// order.
fn earliest_crossing(
    circuit: &CircuitData,
    from: impl Fn(usize) -> Value,
    to: impl Fn(usize) -> Value,
) -> Option<Value> {
    let voltage = circuit
        .vswitches
        .iter()
        .enumerate()
        .filter_map(|(index, switch)| switch.crossing_fraction(from(index), to(index)));
    let offset = circuit.vswitches.len();
    let current = circuit
        .iswitches
        .iter()
        .enumerate()
        .filter_map(|(index, switch)| {
            switch.crossing_fraction(from(offset + index), to(offset + index))
        });
    voltage.chain(current).reduce(Value::min)
}

/// One end of the bracket: its time and the switches' controls there.
struct End {
    time: Value,
    controls: Vec<Value>,
}

/// The bracketed search for an ideal switch's crossing inside the step from
/// the accepted point `from`.
pub(super) struct SwitchCrossingSearch {
    from: Value,
    /// The latest end short of the crossing; the accepted point until one is
    /// found.
    before: Option<End>,
    /// The latest end past it.
    after: End,
    /// Whether the side kept last was `after`.
    kept_after: bool,
    /// The end the next attempt solves to.
    target: Value,
    /// How close the two sides must come.
    resolution: Value,
    ends: usize,
}

/// What a candidate of a locked step leaves the search to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SwitchCrossingStep {
    /// The candidate dates every change it makes: accept it.
    Accept,
    /// Solve the step again to [`SwitchCrossingSearch::target`].
    Retry,
}

impl SwitchCrossingSearch {
    /// The end the next attempt from `t` solves to, while a search from `t`
    /// runs.
    pub(super) fn target(search: Option<&Self>, t: Value) -> Option<Value> {
        search
            .filter(|search| search.from == t && search.target > t)
            .map(|search| search.target)
    }

    /// The next estimate between `before` (or the accepted point) and
    /// `after` at chord fraction `fraction`: halfway toward the side kept
    /// last when the same side is kept twice running, so the chord cannot
    /// stall against an end far from the crossing.
    fn estimate(lower: Value, upper: Value, fraction: Value, toward: Option<Value>) -> Value {
        let chord = lower + fraction * (upper - lower);
        toward.map_or(chord, |stale| 0.5 * (chord + stale))
    }

    /// Advance the search with the candidate `candidate` of the step from
    /// the accepted point (`t`, `accepted`) to `step_time`.
    ///
    /// A candidate that carries a control across its threshold relative to
    /// the latest end short of it becomes the end past the crossing; one that
    /// does not becomes the end short of it, when the search chose its end.
    /// The search ends on the end past the crossing once the sides are within
    /// `resolution`, so the step that is accepted is the one the switch
    /// changes at the end of, within the resolution of its crossing.
    pub(super) fn advance(
        search: &mut Option<Self>,
        circuit: &CircuitData,
        (t, accepted): (Value, &[Value]),
        (step_time, candidate): (Value, &[Value]),
        resolution: Value,
    ) -> SwitchCrossingStep {
        let current = search.take().filter(|search| search.from == t);
        let before = current.as_ref().and_then(|search| search.before.as_ref());
        let lower = before.map_or(t, |end| end.time);
        let from = |index: usize| match before {
            Some(end) => end.controls[index],
            None => switch_control(circuit, accepted, index, false),
        };
        let crossed = earliest_crossing(circuit, from, |index| {
            switch_control(circuit, candidate, index, true)
        });
        if let Some(fraction) = crossed {
            let (resolution, ends, repeated) = current.as_ref().map_or(
                (
                    resolution.max(16.0 * Value::EPSILON * step_time.abs()),
                    0,
                    false,
                ),
                |search| (search.resolution, search.ends, search.kept_after),
            );
            if step_time - lower <= resolution || ends >= SEARCH_ENDS {
                return SwitchCrossingStep::Accept;
            }
            let target = Self::estimate(lower, step_time, fraction, repeated.then_some(lower))
                .max(lower + resolution);
            // A crossing within the resolution of the step's end is dated
            // there already.
            if step_time - target <= resolution {
                return SwitchCrossingStep::Accept;
            }
            *search = Some(Self {
                from: t,
                before: current.and_then(|search| search.before),
                after: End {
                    time: step_time,
                    controls: controls(circuit, candidate, true),
                },
                kept_after: true,
                target,
                resolution,
                ends: ends + 1,
            });
            return SwitchCrossingStep::Retry;
        }
        // A candidate short of the crossing that the search did not ask for,
        // or the end past it solved again and no longer past it, is the step.
        let Some(mut current) = current.filter(|search| {
            search.target == step_time && step_time < search.after.time && search.ends < SEARCH_ENDS
        }) else {
            return SwitchCrossingStep::Accept;
        };
        let before = End {
            time: step_time,
            controls: controls(circuit, candidate, true),
        };
        let upper = current.after.time;
        let fraction = earliest_crossing(
            circuit,
            |index| before.controls[index],
            |index| current.after.controls[index],
        );
        current.target = match fraction {
            Some(fraction) if upper - step_time > current.resolution => {
                let repeated = !current.kept_after;
                let target = Self::estimate(step_time, upper, fraction, repeated.then_some(upper))
                    .min(upper - current.resolution);
                if target > step_time { target } else { upper }
            }
            // The sides have met: solve the end past the crossing again and
            // accept it.
            _ => upper,
        };
        current.before = Some(before);
        current.kept_after = false;
        current.ends += 1;
        *search = Some(current);
        SwitchCrossingStep::Retry
    }
}

/// The `index`th ideal switch's control in [`controls`] order: at a
/// candidate as the crossing reads it, else as the solution holds it.
fn switch_control(
    circuit: &CircuitData,
    solution: &[Value],
    index: usize,
    candidate: bool,
) -> Value {
    let control = match index.checked_sub(circuit.vswitches.len()) {
        None => circuit.vswitches[index].crossing_control(solution, candidate),
        Some(index) => circuit.iswitches[index].crossing_control(solution, candidate),
    };
    control.unwrap_or(Value::NAN)
}
