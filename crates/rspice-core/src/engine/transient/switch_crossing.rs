//! Where an ideal switch changes inside a step of a locked segment.
//!
//! A segment from a live image (an event-shooting traversal) runs on a
//! locked mesh, and an ideal switch whose control crosses its threshold
//! inside one of its steps is cut there, so the change is dated where it
//! happens and the period map moves it with the state. Every candidate of
//! the cut solves the same step from the same accepted point to a different
//! end, so the step's path is a function of its end time and the crossing is
//! the root of the controls along it. [`SwitchCrossings`] brackets that root
//! between the latest end on each side and takes the next end where the
//! controls' chord crosses, until the two sides are within the step's
//! resolution.
//!
//! The step is accepted at the end short of the crossing, where the switch
//! still holds its state, and the switch changes at the start of the next
//! step, which restarts integration as a breakpoint does: at order one, over
//! a tenth of the step the mesh took there, after which the step to the
//! mesh's target resumes the order held at the landing. No step then
//! averages the slopes on both sides of the change: a step that ended just
//! past it took the closed slope at its end for the open one it followed,
//! and its trapezoid carried half of the change's saltation into the period
//! map.
//!
//! The order-one step keeps its length as the crossing moves. Its error grows
//! with its length, and one that ran on to the mesh's target, shorter as the
//! crossing came later, carried that change into the period map's
//! derivative, at nodes the switch never touches: 0.4% of an RC's decay on
//! its own control. Where a tenth of the step does not fit twice before the
//! target, the order-one step takes half of what is left, so its end moves
//! with the crossing by half and the map stays continuous; where even that
//! half would be a hundredth of the step, it takes all of it.
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

/// The first step from a landing is this fraction of the step the mesh took
/// across the crossing, as ngspice's `dctran.c` takes the first step after a
/// breakpoint at a tenth of the step it saved.
const RESTART_FRACTION: Value = 0.1;

/// The `index`th ideal switch's control, voltage switches first, in circuit
/// order: at a candidate as the crossing reads it, else as the solution
/// holds it. A switch that is not ideal reads NaN, which crosses nothing.
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

fn switch_count(circuit: &CircuitData) -> usize {
    circuit.vswitches.len() + circuit.iswitches.len()
}

/// The fraction of the chord from `from` to `to` at which the `index`th
/// ideal switch's control crosses the threshold that changes its accepted
/// state.
fn crossing(circuit: &CircuitData, index: usize, from: Value, to: Value) -> Option<Value> {
    match index.checked_sub(circuit.vswitches.len()) {
        None => circuit.vswitches[index].crossing_fraction(from, to),
        Some(index) => circuit.iswitches[index].crossing_fraction(from, to),
    }
}

/// The earliest such fraction over every switch.
fn earliest_crossing(
    circuit: &CircuitData,
    from: impl Fn(usize) -> Value,
    to: impl Fn(usize) -> Value,
) -> Option<Value> {
    (0..switch_count(circuit))
        .filter_map(|index| crossing(circuit, index, from(index), to(index)))
        .reduce(Value::min)
}

/// One end of the bracket: its time and the switches' controls there.
struct End {
    time: Value,
    controls: Vec<Value>,
}

/// The bracketed search for a crossing inside the step from the accepted
/// point `from`.
struct Search {
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

/// The accepted point a crossing was landed short of.
struct Landing {
    time: Value,
    /// The accepted point the step into it left.
    start: Value,
    /// Which switches, in [`switch_control`] order, change at the start of
    /// the step from it. The search leaves them out of that step: their
    /// crossing is its start.
    crossed: Vec<bool>,
    /// The end of the order-one step from it, when that step is cut short
    /// of its target, and how far that end moves when the landing does.
    restart: Option<(Value, Value)>,
    /// The integration order the step after that one resumes.
    order: Option<u8>,
}

/// The crossings of one locked segment: the search inside the step from the
/// latest accepted point, and the crossing that point was landed short of.
#[derive(Default)]
pub(super) struct SwitchCrossings {
    search: Option<Search>,
    landing: Option<Landing>,
}

/// What a candidate of a locked step leaves the search to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SwitchCrossingStep {
    /// The candidate changes no switch inside it, or ends a resolution short
    /// of a change: accept it.
    Accept,
    /// Solve the step again to [`SwitchCrossings::target`].
    Retry,
}

/// The next estimate between `lower` and `upper` at chord fraction
/// `fraction`: halfway toward the side kept last when the same side is kept
/// twice running, so the chord cannot stall against an end far from the
/// crossing. Kept a resolution inside both sides.
fn estimate(
    (lower, upper): (Value, Value),
    fraction: Option<Value>,
    toward: Option<Value>,
    resolution: Value,
) -> Value {
    let chord = fraction.map_or(0.5 * (lower + upper), |fraction| {
        lower + fraction * (upper - lower)
    });
    let estimate = toward.map_or(chord, |stale| 0.5 * (chord + stale));
    estimate.clamp(lower + resolution, upper - resolution)
}

impl SwitchCrossings {
    /// The end the next attempt from `t` solves to, while a search from `t`
    /// runs.
    pub(super) fn target(&self, t: Value) -> Option<Value> {
        self.search
            .as_ref()
            .filter(|search| search.from == t && search.target > t)
            .map(|search| search.target)
    }

    /// Whether the point accepted at `t` was landed short of a crossing, so
    /// the step from it restarts integration.
    pub(super) fn landed_at(&self, t: Value) -> bool {
        self.landing
            .as_ref()
            .is_some_and(|landing| landing.time == t)
    }

    /// Hold `order`, the integration order at the landing at `t`, for the
    /// step after the order-one step that leaves it.
    pub(super) fn hold_order(&mut self, t: Value, order: u8) {
        if let Some(landing) = self.landing.as_mut().filter(|landing| landing.time == t) {
            landing.order = Some(order);
        }
    }

    /// The order the step after the one from a landing at `from` resumes.
    pub(super) fn held_order(&self, from: Value) -> Option<u8> {
        self.landing
            .as_ref()
            .filter(|landing| landing.time == from)
            .and_then(|landing| landing.order)
    }

    /// The end of the order-one step from a landing at `t` toward `target`:
    /// a tenth of the step from the landing step's start to `target`, or half
    /// of what is left when that tenth does not fit twice, or `target` itself
    /// when that half would be shorter than a hundredth of the step, rather
    /// than leave a sliver of a trapezoid whose derivative history would be
    /// roundoff.
    pub(super) fn restart_target(&mut self, t: Value, target: Value) -> Option<Value> {
        let landing = self.landing.as_mut().filter(|landing| landing.time == t)?;
        let (restart, remainder) = (RESTART_FRACTION * (target - landing.start), target - t);
        landing.restart = if remainder >= 2.0 * restart {
            Some((t + restart, 1.0))
        } else if remainder >= 2.0 * RESTART_FRACTION * restart {
            Some((t + 0.5 * remainder, 0.5))
        } else {
            None
        };
        landing.restart.map(|(end, _)| end)
    }

    /// How far the end of the step from `from` to `to` moves when its start
    /// does: an order-one step cut short from a landing ends a fixed interval
    /// after it, or halfway to its target, and moves with it fully or by
    /// half. Every other end is where the mesh put it.
    pub(super) fn follows(&self, from: Value, to: Value) -> Value {
        self.landing
            .as_ref()
            .filter(|landing| landing.time == from)
            .and_then(|landing| landing.restart)
            .filter(|(end, _)| *end == to)
            .map_or(0.0, |(_, follows)| follows)
    }

    /// Advance the search with the candidate `candidate` of the step from
    /// the accepted point (`t`, `accepted`) to `step_time`.
    ///
    /// A candidate that carries a control across its threshold relative to
    /// the latest end short of it becomes the end past the crossing; one that
    /// does not becomes the end short of it, when the search chose its end.
    /// Once the two sides are within two resolutions, no end between them is
    /// a resolution from both, and the step is accepted at the side short of
    /// the crossing, solved again if the latest candidate was past it.
    pub(super) fn advance(
        &mut self,
        circuit: &CircuitData,
        (t, accepted): (Value, &[Value]),
        (step_time, candidate): (Value, &[Value]),
        resolution: Value,
    ) -> SwitchCrossingStep {
        let landed = self
            .landing
            .as_ref()
            .filter(|landing| landing.time == t)
            .map(|landing| landing.crossed.clone());
        let left_out = |index: usize| landed.as_ref().is_some_and(|crossed| crossed[index]);
        let read = |solution: &[Value], index: usize, at_candidate: bool| {
            if left_out(index) {
                Value::NAN
            } else {
                switch_control(circuit, solution, index, at_candidate)
            }
        };
        let current = self.search.take().filter(|search| search.from == t);
        let before = current.as_ref().and_then(|search| search.before.as_ref());
        let lower = before.map_or(t, |end| end.time);
        let from = |index: usize| match before {
            Some(end) => end.controls[index],
            None => read(accepted, index, false),
        };
        let to = |index: usize| read(candidate, index, true);
        let controls = || -> Vec<Value> { (0..switch_count(circuit)).map(to).collect() };
        if let Some(fraction) = earliest_crossing(circuit, from, to) {
            let (resolution, ends, repeated) = current.as_ref().map_or(
                (
                    resolution.max(16.0 * Value::EPSILON * step_time.abs()),
                    0,
                    false,
                ),
                |search| (search.resolution, search.ends, search.kept_after),
            );
            if ends >= SEARCH_ENDS {
                return SwitchCrossingStep::Accept;
            }
            let target = if step_time - lower < 2.0 * resolution {
                // The sides have met: solve the end short of the crossing
                // again and land there. A crossing within a resolution of
                // the accepted point is dated there already.
                match before {
                    Some(end) => end.time,
                    None => return SwitchCrossingStep::Accept,
                }
            } else {
                estimate(
                    (lower, step_time),
                    Some(fraction),
                    repeated.then_some(lower),
                    resolution,
                )
            };
            let after = End {
                time: step_time,
                controls: controls(),
            };
            self.search = Some(Search {
                from: t,
                before: current.and_then(|search| search.before),
                after,
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
            controls: controls(),
        };
        let upper = current.after.time;
        if upper - step_time < 2.0 * current.resolution {
            let crossed = (0..switch_count(circuit))
                .map(|index| {
                    crossing(
                        circuit,
                        index,
                        before.controls[index],
                        current.after.controls[index],
                    )
                    .is_some()
                })
                .collect();
            self.landing = Some(Landing {
                time: step_time,
                start: t,
                crossed,
                restart: None,
                order: None,
            });
            return SwitchCrossingStep::Accept;
        }
        let fraction = earliest_crossing(
            circuit,
            |index| before.controls[index],
            |index| current.after.controls[index],
        );
        current.target = estimate(
            (step_time, upper),
            fraction,
            (!current.kept_after).then_some(upper),
            current.resolution,
        );
        current.before = Some(before);
        current.kept_after = false;
        current.ends += 1;
        self.search = Some(current);
        SwitchCrossingStep::Retry
    }
}
