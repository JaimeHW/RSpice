//! Accepted crossing history and pure analog event evaluation.

use crate::Value;

/// Accepted history for one `cross`, `above`, or `last_crossing` site.
///
/// Generated models keep a separate speculative copy while Newton iterates.
/// Only this accepted image is serialized; a rejected endpoint therefore
/// cannot consume a crossing or change which side of zero the next trial sees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedCrossState {
    pub value: Value,
    pub time: Value,
    pub side: i8,
    pub last_event_time: Value,
    pub last_crossing_time: Value,
    pub initialized: bool,
}

/// Scalar representation used by one accepted-state checkpoint lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedCheckpointLaneType {
    NanForbiddenF64,
    I8AsF64,
    BoolAsF64,
}

impl GeneratedCheckpointLaneType {
    pub const fn identity_tag(self) -> &'static str {
        match self {
            Self::NanForbiddenF64 => "f64:nan-forbidden",
            Self::I8AsF64 => "i8-as-f64",
            Self::BoolAsF64 => "bool-as-f64",
        }
    }
}

/// One ordered lane in an accepted-state checkpoint schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratedCheckpointLaneDescriptor {
    pub name: &'static str,
    pub lane_type: GeneratedCheckpointLaneType,
}

impl GeneratedCrossState {
    pub const INITIAL: Self = Self {
        value: 0.0,
        time: 0.0,
        side: 0,
        last_event_time: Value::NEG_INFINITY,
        last_crossing_time: -1.0,
        initialized: false,
    };

    /// Ordered accepted-state schema shared by the runtime serializer and the
    /// offline generator's shape identity.
    pub const CHECKPOINT_SCHEMA: [GeneratedCheckpointLaneDescriptor; 6] = [
        GeneratedCheckpointLaneDescriptor {
            name: "value",
            lane_type: GeneratedCheckpointLaneType::NanForbiddenF64,
        },
        GeneratedCheckpointLaneDescriptor {
            name: "time",
            lane_type: GeneratedCheckpointLaneType::NanForbiddenF64,
        },
        GeneratedCheckpointLaneDescriptor {
            name: "side",
            lane_type: GeneratedCheckpointLaneType::I8AsF64,
        },
        GeneratedCheckpointLaneDescriptor {
            name: "last_event_time",
            lane_type: GeneratedCheckpointLaneType::NanForbiddenF64,
        },
        GeneratedCheckpointLaneDescriptor {
            name: "last_crossing_time",
            lane_type: GeneratedCheckpointLaneType::NanForbiddenF64,
        },
        GeneratedCheckpointLaneDescriptor {
            name: "initialized",
            lane_type: GeneratedCheckpointLaneType::BoolAsF64,
        },
    ];

    /// Scalar checkpoint lane count, derived from [`Self::CHECKPOINT_SCHEMA`].
    pub const CHECKPOINT_LANES: usize = Self::CHECKPOINT_SCHEMA.len();

    #[inline]
    pub fn append_checkpoint_lanes(self, values: &mut Vec<Value>) {
        values.extend_from_slice(&[
            self.value,
            self.time,
            Value::from(self.side),
            self.last_event_time,
            self.last_crossing_time,
            Value::from(u8::from(self.initialized)),
        ]);
    }

    pub fn from_checkpoint_lanes(values: &[Value]) -> Result<Self, String> {
        if values.len() != Self::CHECKPOINT_LANES {
            return Err(format!(
                "generated crossing checkpoint requires {} lanes, found {}",
                Self::CHECKPOINT_LANES,
                values.len()
            ));
        }
        let side = generated_checkpoint_integer("crossing side", values[2])?;
        let initialized = generated_checkpoint_integer("crossing initialized flag", values[5])?;
        if !(-1..=1).contains(&side) {
            return Err(format!(
                "generated crossing checkpoint side must be -1, 0, or 1, got {side}"
            ));
        }
        if !(0..=1).contains(&initialized) {
            return Err(format!(
                "generated crossing checkpoint initialized flag must be 0 or 1, got {initialized}"
            ));
        }
        let state = Self {
            value: values[0],
            time: values[1],
            side: side as i8,
            last_event_time: values[3],
            last_crossing_time: values[4],
            initialized: initialized != 0,
        };
        validate_generated_cross_state(state)?;
        Ok(state)
    }
}

impl Default for GeneratedCrossState {
    fn default() -> Self {
        Self::INITIAL
    }
}

/// Result of evaluating one generated `cross` or `above` site against accepted
/// history. The candidate is committed only after the simulator accepts the
/// complete timepoint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedCrossEvaluation {
    pub fired: bool,
    pub candidate: GeneratedCrossState,
    pub refinement_time: Option<Value>,
}

/// Malformed numeric input or accepted history at a generated event site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEventControlError {
    NonFiniteExpression,
    /// A direction or enable operand was not finite.
    ///
    /// Split from `NonIntegerDirection`/`NonIntegerEnable` because the two
    /// are different facts: `1.5` is a property of the module's source and no
    /// other point will fix it, while NaN is a property of the iterate the
    /// solver offered. The interpreter and the native route have always split
    /// these (`event_integer_operand`, `record_classified`); the generated
    /// route answered "structural" for both.
    NonFiniteOperand,
    InvalidTime,
    InvalidTimeTolerance,
    InvalidExpressionTolerance,
    NonIntegerDirection,
    InvalidLastCrossingDirection,
    NonIntegerEnable,
    InvalidAcceptedState,
}

impl std::fmt::Display for GeneratedEventControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NonFiniteExpression => "event expression must be finite",
            Self::NonFiniteOperand => "event direction and enable operands must be finite",
            Self::InvalidTime => "event evaluation time must be finite and non-negative",
            Self::InvalidTimeTolerance => "event time tolerance must be finite and non-negative",
            Self::InvalidExpressionTolerance => {
                "event expression tolerance must be finite and non-negative"
            }
            Self::NonIntegerDirection => "cross direction must be a finite integer",
            Self::InvalidLastCrossingDirection => "last_crossing direction must be -1, 0, or 1",
            Self::NonIntegerEnable => "event enable must be a finite integer",
            Self::InvalidAcceptedState => "accepted crossing state is malformed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for GeneratedEventControlError {}

#[inline]
fn generated_event_integer(
    value: Value,
    error: GeneratedEventControlError,
) -> Result<i32, GeneratedEventControlError> {
    // A NaN or infinite operand is a property of the iterate, not of the
    // module, so it is raised as its own variant: the consumer rejects the
    // point instead of refusing the run, which is what the interpreter and
    // the native route already do for the same input.
    if !value.is_finite() {
        return Err(GeneratedEventControlError::NonFiniteOperand);
    }
    if value.fract() != 0.0 || value < i32::MIN as Value || value > i32::MAX as Value {
        return Err(error);
    }
    Ok(value as i32)
}

fn generated_checkpoint_integer(name: &str, value: Value) -> Result<i32, String> {
    if !value.is_finite()
        || value.fract() != 0.0
        || value < i32::MIN as Value
        || value > i32::MAX as Value
    {
        return Err(format!(
            "generated {name} checkpoint lane must be a finite integer, got {value}"
        ));
    }
    Ok(value as i32)
}

#[inline]
fn generated_event_side(value: Value) -> i8 {
    if value > 0.0 {
        1
    } else if value < 0.0 {
        -1
    } else {
        0
    }
}

pub fn validate_generated_cross_state(state: GeneratedCrossState) -> Result<(), String> {
    if !(-1..=1).contains(&state.side)
        || !state.value.is_finite()
        || !state.time.is_finite()
        || state.time < 0.0
        || !(state.last_event_time.is_finite() || state.last_event_time == Value::NEG_INFINITY)
        || !state.last_crossing_time.is_finite()
        || state.last_crossing_time < -1.0
    {
        return Err("generated accepted crossing state is malformed".to_string());
    }
    Ok(())
}

#[inline]
fn generated_event_effective_tolerance(requested: Value, scale: Value) -> Value {
    if requested > 0.0 {
        requested
    } else {
        (64.0 * Value::EPSILON * scale).max(Value::MIN_POSITIVE)
    }
}

#[inline]
fn generated_next_time_after(time: Value) -> Value {
    if time == 0.0 {
        Value::from_bits(1)
    } else {
        Value::from_bits(time.to_bits() + 1)
    }
}

/// [`generated_refinement_time`], landing `margin` past the estimate where
/// the interval leaves room for it.
fn generated_refinement_time_past(
    start: Value,
    estimate: Value,
    end: Value,
    margin: Value,
) -> Option<Value> {
    if estimate.is_finite() && end > start {
        let target = estimate.max(start) + margin;
        if target > start && target < end && target > estimate {
            return Some(target);
        }
    }
    generated_refinement_time(start, estimate, end)
}

/// Where an unconverged crossing in `(start, end)` is resolved next, or `None`
/// when it is resolved as finely as time can be written: the first float
/// after the root estimate; for an estimate in the last float interval, the
/// float just before `end`, which brackets it; for an interval with no float
/// inside, nothing, because the crossing is located at `end`.
fn generated_refinement_time(start: Value, estimate: Value, end: Value) -> Option<Value> {
    if !estimate.is_finite() || end <= start {
        return None;
    }
    let target = generated_next_time_after(estimate.max(start));
    if target > start && target < end {
        return Some(target);
    }
    let last = Value::from_bits(end.to_bits() - 1);
    (last > start).then_some(last)
}

fn generated_crossing_time(accepted: GeneratedCrossState, value: Value, time: Value) -> Value {
    generated_crossing_time_between(accepted.value, accepted.time, value, time)
}

/// The linear root estimate between the accepted sample and the candidate.
pub fn generated_crossing_time_between(
    accepted_value: Value,
    accepted_time: Value,
    value: Value,
    time: Value,
) -> Value {
    let accepted_magnitude = accepted_value.abs();
    let candidate_magnitude = value.abs();
    let scale = accepted_magnitude.max(candidate_magnitude);
    if scale == 0.0 || !scale.is_finite() {
        return time;
    }
    let accepted_scaled = accepted_magnitude / scale;
    let candidate_scaled = candidate_magnitude / scale;
    let fraction = (accepted_scaled / (accepted_scaled + candidate_scaled)).clamp(0.0, 1.0);
    accepted_time + fraction * (time - accepted_time)
}

/// The accepted detector state a candidate is judged against.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossingAccepted {
    pub side: i8,
    pub value: Value,
    pub time: Value,
}

/// What a `cross` or `above` site asks of a crossing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossingRule {
    /// +1 rising only, -1 falling only, 0 both.
    pub direction: i32,
    pub time_tol: Value,
    pub expr_tol: Value,
    pub enabled: bool,
    /// A DC or IC sweep point at the accepted time itself, where consecutive
    /// accepted values are compared with no root to refine.
    pub same_time_static: bool,
}

/// How a candidate stands against the accepted state: the direction of the
/// crossing it completes (0 for none), the crossing's estimated instant,
/// whether the event fires at the candidate, and where to retry when the
/// candidate does not yet resolve the crossing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossingDecision {
    pub direction: i32,
    pub time: Value,
    pub fired: bool,
    pub refinement_time: Option<Value>,
}

/// The crossing rule every route shares.
///
/// "The event shall occur after the threshold crossing" (VAMS-LRM 2.4
/// 5.10.3.1), so a crossing is complete only at a candidate strictly on the
/// far side of zero: a candidate reading exactly zero is on the threshold,
/// not past it, and an event body there would read the crossing expression
/// as zero, so `@(cross(V(in) - th)) q = V(in) > th;` would never change
/// `q`. Such a candidate is an ordinary point on the near side.
///
/// A landed candidate resolves the crossing when it lies within `time_tol` of
/// the estimated root and within `expr_tol` of zero. Where the accepted point
/// itself already reads the crossing to within `expr_tol` — the previous
/// landing came to rest on the threshold, one representable instant short of
/// the flip — the root is located there, and the expression criterion, whose
/// default scales with the two values and so collapses with the accepted
/// one, says nothing more. The candidate resolves the crossing within
/// `time_tol` of that point, and the retry lands half the default window past
/// it, where any expression that changes faster than its own rounding over
/// that interval has flipped. A sweep point at the same time has no root to refine
/// and keeps the closed comparison.
pub fn decide_crossing(
    accepted: CrossingAccepted,
    value: Value,
    time: Value,
    rule: CrossingRule,
) -> CrossingDecision {
    let (rising, falling) = if rule.same_time_static {
        (
            accepted.side < 0 && value >= 0.0,
            accepted.side > 0 && value <= 0.0,
        )
    } else {
        (
            accepted.side < 0 && value > 0.0,
            accepted.side > 0 && value < 0.0,
        )
    };
    let direction = if rising {
        1
    } else if falling {
        -1
    } else {
        0
    };
    let crossing_time = if direction != 0 {
        generated_crossing_time_between(accepted.value, accepted.time, value, time)
    } else {
        -1.0
    };
    let fired =
        rule.enabled && direction != 0 && (rule.direction == 0 || rule.direction == direction);
    let refinement_time = if fired && !rule.same_time_static {
        let time_scale = accepted.time.abs().max(time.abs());
        let effective_time_tol = generated_event_effective_tolerance(rule.time_tol, time_scale);
        let effective_expr_tol = generated_event_effective_tolerance(
            rule.expr_tol,
            accepted.value.abs().max(value.abs()),
        );
        // Half the window at the root's own scale, inside any tighter
        // `time_tol`: the landing's tolerance, taken at a scale no smaller,
        // holds the margin together with the landing's rounding, so the
        // landing resolves the crossing rather than asking again. One
        // representable instant past a root can still read the expression on
        // the threshold, which is why the margin is not one ulp.
        let margin = |root: Value| {
            generated_event_effective_tolerance(rule.time_tol, root.abs())
                .min(generated_event_effective_tolerance(0.0, root.abs()))
                / 2.0
        };
        if accepted.value.abs() <= effective_expr_tol {
            let expression_resolved = rule.expr_tol == 0.0 || value.abs() <= effective_expr_tol;
            if time - accepted.time <= effective_time_tol && expression_resolved {
                None
            } else {
                generated_refinement_time_past(
                    accepted.time,
                    accepted.time,
                    time,
                    margin(accepted.time),
                )
            }
        } else {
            let time_error = (time - crossing_time).max(0.0);
            if time_error > effective_time_tol || value.abs() > effective_expr_tol {
                generated_refinement_time_past(
                    accepted.time,
                    crossing_time,
                    time,
                    margin(crossing_time),
                )
            } else {
                None
            }
        }
    } else {
        None
    };
    CrossingDecision {
        direction,
        time: crossing_time,
        fired,
        refinement_time,
    }
}

/// Interpolate the latest matching zero crossing without requesting refinement.
/// The returned candidate is independent of rejected or repeated evaluations.
pub fn evaluate_generated_last_crossing(
    accepted: GeneratedCrossState,
    value: Value,
    time: Value,
    direction: Value,
) -> Result<GeneratedCrossState, GeneratedEventControlError> {
    if !matches!(direction, -1.0 | 0.0 | 1.0) {
        return Err(GeneratedEventControlError::InvalidLastCrossingDirection);
    }
    if !value.is_finite() {
        return Err(GeneratedEventControlError::NonFiniteExpression);
    }
    if !time.is_finite() || time < 0.0 {
        return Err(GeneratedEventControlError::InvalidTime);
    }
    validate_generated_cross_state(accepted)
        .map_err(|_| GeneratedEventControlError::InvalidAcceptedState)?;
    if accepted.initialized && time <= accepted.time {
        return Ok(accepted);
    }
    let previous = if accepted.initialized {
        accepted
    } else {
        GeneratedCrossState::INITIAL
    };
    let mut candidate = GeneratedCrossState {
        value,
        time,
        side: generated_event_side(value),
        initialized: true,
        ..previous
    };
    if accepted.initialized {
        let rising = accepted.value < 0.0 && value >= 0.0;
        let falling = accepted.value > 0.0 && value <= 0.0;
        if (rising && direction >= 0.0) || (falling && direction <= 0.0) {
            candidate.last_crossing_time = generated_crossing_time(accepted, value, time);
        }
    }
    Ok(candidate)
}

/// Evaluate a generated Verilog-A `cross` site from accepted history.
pub fn evaluate_generated_cross(
    accepted: GeneratedCrossState,
    value: Value,
    time: Value,
    direction: Value,
    time_tol: Value,
    expr_tol: Value,
    enable: Value,
    transient: bool,
) -> Result<GeneratedCrossEvaluation, GeneratedEventControlError> {
    let direction =
        generated_event_integer(direction, GeneratedEventControlError::NonIntegerDirection)?;
    let enabled = generated_event_enable(enable)?;
    evaluate_generated_cross_impl(
        accepted, value, time, direction, time_tol, expr_tol, enabled, false, false, transient,
    )
}

/// Evaluate a generated Verilog-A `above` site from accepted history.
pub fn evaluate_generated_above(
    accepted: GeneratedCrossState,
    value: Value,
    time: Value,
    time_tol: Value,
    expr_tol: Value,
    enable: Value,
    static_analysis: bool,
) -> Result<GeneratedCrossEvaluation, GeneratedEventControlError> {
    let enabled = generated_event_enable(enable)?;
    evaluate_generated_cross_impl(
        accepted,
        value,
        time,
        1,
        time_tol,
        expr_tol,
        enabled,
        true,
        static_analysis,
        true,
    )
}

#[inline]
fn generated_event_enable(enable: Value) -> Result<bool, GeneratedEventControlError> {
    generated_event_integer(enable, GeneratedEventControlError::NonIntegerEnable)
        .map(|enable| enable != 0)
}

#[allow(clippy::too_many_arguments)]
fn evaluate_generated_cross_impl(
    accepted: GeneratedCrossState,
    value: Value,
    time: Value,
    direction: i32,
    time_tol: Value,
    expr_tol: Value,
    enabled: bool,
    initial_above: bool,
    allow_same_time_crossing: bool,
    events_enabled: bool,
) -> Result<GeneratedCrossEvaluation, GeneratedEventControlError> {
    if !value.is_finite() {
        return Err(GeneratedEventControlError::NonFiniteExpression);
    }
    if !time.is_finite() || time < 0.0 {
        return Err(GeneratedEventControlError::InvalidTime);
    }
    if !time_tol.is_finite() || time_tol < 0.0 {
        return Err(GeneratedEventControlError::InvalidTimeTolerance);
    }
    if !expr_tol.is_finite() || expr_tol < 0.0 {
        return Err(GeneratedEventControlError::InvalidExpressionTolerance);
    }
    validate_generated_cross_state(accepted)
        .map_err(|_| GeneratedEventControlError::InvalidAcceptedState)?;

    if !accepted.initialized {
        let fired = initial_above && enabled && events_enabled && value > 0.0;
        return Ok(GeneratedCrossEvaluation {
            fired,
            candidate: GeneratedCrossState {
                value,
                time,
                side: generated_event_side(value),
                last_event_time: if fired { time } else { Value::NEG_INFINITY },
                last_crossing_time: if fired { time } else { -1.0 },
                initialized: true,
            },
            refinement_time: None,
        });
    }

    let same_time_static = allow_same_time_crossing && time == accepted.time;
    if time < accepted.time || (time == accepted.time && !same_time_static) {
        return Ok(GeneratedCrossEvaluation {
            fired: false,
            candidate: accepted,
            refinement_time: None,
        });
    }

    let decision = decide_crossing(
        CrossingAccepted {
            side: accepted.side,
            value: accepted.value,
            time: accepted.time,
        },
        value,
        time,
        CrossingRule {
            direction,
            time_tol,
            expr_tol,
            enabled: events_enabled && enabled,
            same_time_static,
        },
    );
    let crossing_direction = decision.direction;
    let crossing_time = decision.time;
    let fired = decision.fired;
    let refinement_time = decision.refinement_time;

    let stable_side = generated_event_side(value);
    let side = if stable_side != 0 {
        stable_side
    } else if crossing_direction != 0 {
        crossing_direction as i8
    } else {
        accepted.side
    };
    Ok(GeneratedCrossEvaluation {
        fired,
        candidate: GeneratedCrossState {
            value,
            time,
            side,
            last_event_time: if fired {
                crossing_time
            } else {
                accepted.last_event_time
            },
            last_crossing_time: if fired {
                crossing_time
            } else {
                accepted.last_crossing_time
            },
            initialized: true,
        },
        refinement_time,
    })
}

/// Evaluate a generated timer and return its event level and next exact event.
///
/// A non-positive period denotes a one-shot timer. The event is level-true
/// throughout repeated Newton evaluations of the same candidate timepoint,
/// but is excluded once that timepoint becomes the lower endpoint of the next
/// step. `time_tol` is an event-coalescing tolerance, while a small
/// scale-aware floating-point tolerance protects endpoint comparisons. The
/// portable VM evaluates its timers through this function too.
pub fn evaluate_generated_timer(
    start_time: Value,
    period: Value,
    time_tol: Value,
    enable: Value,
    current_time: Value,
    timestep: Value,
) -> (bool, Option<Value>) {
    if !start_time.is_finite()
        || !period.is_finite()
        || !time_tol.is_finite()
        || !enable.is_finite()
        || !current_time.is_finite()
        || !timestep.is_finite()
        || enable == 0.0
        || period < 0.0
    {
        return (false, None);
    }

    let scale = start_time.abs().max(current_time.abs());
    let numeric_tol = (16.0 * Value::EPSILON * scale).max(Value::MIN_POSITIVE);
    let event_tol = time_tol.max(0.0).max(numeric_tol);
    let horizon = current_time + event_tol;
    let previous_time = current_time - timestep.max(0.0);
    let scheduled = if horizon + numeric_tol < start_time {
        None
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0);
        let candidate = start_time + cycles * period;
        candidate.is_finite().then_some(candidate)
    } else {
        Some(start_time)
    };
    let fired = scheduled.is_some_and(|event_time| {
        event_time <= horizon
            && if timestep > numeric_tol {
                event_time > previous_time + numeric_tol
            } else {
                (current_time - event_time).abs() <= event_tol
            }
    });
    let next_event = if horizon + numeric_tol < start_time {
        Some(start_time)
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0) + 1.0;
        let next = start_time + cycles * period;
        (next.is_finite() && next > current_time + numeric_tol).then_some(next)
    } else {
        None
    };
    (fired, next_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_crossing_interpolates_both_directions_from_accepted_history() {
        for (direction, rising, falling) in [(0.0, 0.5, 2.75), (1.0, 0.5, 0.5), (-1.0, -1.0, 2.75)]
        {
            let initial = evaluate_generated_last_crossing(
                GeneratedCrossState::INITIAL,
                -1.0,
                0.0,
                direction,
            )
            .unwrap();
            assert_eq!(initial.last_crossing_time, -1.0);
            let positive = evaluate_generated_last_crossing(initial, 3.0, 2.0, direction).unwrap();
            assert_eq!(positive.last_crossing_time, rising);
            assert_eq!(
                evaluate_generated_last_crossing(initial, 3.0, 2.0, direction).unwrap(),
                positive
            );
            assert_eq!(
                evaluate_generated_last_crossing(initial, -0.25, 1.0, direction)
                    .unwrap()
                    .last_crossing_time,
                -1.0
            );
            assert_eq!(
                evaluate_generated_last_crossing(positive, -1.0, 2.0, direction).unwrap(),
                positive
            );
            assert_eq!(
                evaluate_generated_last_crossing(positive, -1.0, 3.0, direction)
                    .unwrap()
                    .last_crossing_time,
                falling
            );
            assert_eq!(positive.last_event_time, Value::NEG_INFINITY);
        }
    }

    #[test]
    fn last_crossing_handles_extreme_finite_input_without_overflow() {
        let initial =
            evaluate_generated_last_crossing(GeneratedCrossState::INITIAL, -Value::MAX, 0.0, 0.0)
                .unwrap();
        let crossing =
            evaluate_generated_last_crossing(initial, Value::MAX, Value::MAX, 0.0).unwrap();
        assert_eq!(crossing.last_crossing_time, Value::MAX / 2.0);
    }

    /// The portable VM's refinement rule, on the generated route: the first
    /// float after the root estimate; for an estimate in the last float
    /// interval, the float just before the candidate, which brackets it; for
    /// an interval with no float inside, nothing, so the crossing fires at the
    /// candidate, located to the resolution of time.
    #[test]
    fn refinement_brackets_the_last_float_interval_and_then_locates_the_crossing() {
        let next = |time: Value| Value::from_bits(time.to_bits() + 1);
        let previous = |time: Value| Value::from_bits(time.to_bits() - 1);
        assert_eq!(generated_refinement_time(0.0, 0.25, 1.0), Some(next(0.25)));
        assert_eq!(
            generated_refinement_time(0.0, previous(1.0), 1.0),
            Some(previous(1.0)),
            "an estimate in the last float interval is bracketed by the float before the end"
        );
        assert_eq!(generated_refinement_time(previous(1.0), 1.0, 1.0), None);
        assert_eq!(generated_refinement_time(0.0, Value::NAN, 1.0), None);
        assert_eq!(generated_refinement_time(1.0, 0.5, 1.0), None);

        // The corpus PLL's VCO, `@(cross(ph - 0.5, 0))` with `ph` near 0.5:
        // the solver lands on the crossing, where the expression reads one
        // ulp of 0.5, far above a tolerance scaled by its own small value.
        let start = 6.202_420_706_862_774e-9;
        let end = 6.204_860_122_698_261e-9;
        let ulp_of_half = 0.5 * Value::EPSILON;
        let cross = |accepted, value, time| {
            evaluate_generated_cross(accepted, value, time, 0.0, 0.0, 0.0, 1.0, true).unwrap()
        };
        let initial = cross(
            GeneratedCrossState::INITIAL,
            -3.931_437_199_186_094e-4,
            start,
        );
        let bracketed = cross(initial.candidate, ulp_of_half, end);
        assert!(bracketed.fired);
        assert_eq!(bracketed.refinement_time, Some(previous(end)));

        let before = cross(GeneratedCrossState::INITIAL, -ulp_of_half, previous(end));
        let located = cross(before.candidate, ulp_of_half, end);
        assert!(located.fired);
        assert_eq!(located.refinement_time, None);
        assert!(
            (previous(end)..=end).contains(&located.candidate.last_crossing_time),
            "{:e}",
            located.candidate.last_crossing_time
        );

        // So is a jump between adjacent floats, whatever its height.
        let before = cross(GeneratedCrossState::INITIAL, -1.0, 1.0);
        let jump = cross(before.candidate, 1.0, next(1.0));
        assert!(jump.fired);
        assert_eq!(jump.refinement_time, None);
    }

    /// The crossing rule every route shares (VAMS-LRM 2.4 5.10.3.1: the
    /// event occurs after the crossing).
    #[test]
    fn a_crossing_completes_strictly_past_the_threshold_and_lands_near_the_root() {
        let rule = |direction, time_tol, expr_tol| CrossingRule {
            direction,
            time_tol,
            expr_tol,
            enabled: true,
            same_time_static: false,
        };
        let below = CrossingAccepted {
            side: -1,
            value: -1.0,
            time: 0.0,
        };

        // A candidate on the threshold has not crossed it.
        let on = decide_crossing(below, 0.0, 1.0, rule(0, 0.0, 0.0));
        assert_eq!(
            (on.direction, on.fired, on.refinement_time),
            (0, false, None)
        );

        // A whole step past the root lands half the default window past the
        // estimate, where the expression has left the threshold.
        let past = decide_crossing(below, 1.0, 1.0, rule(0, 0.0, 0.0));
        assert_eq!((past.direction, past.fired, past.time), (1, true, 0.5));
        assert_eq!(past.refinement_time, Some(0.5 + 16.0 * Value::EPSILON));

        // A tighter time_tol narrows that margin to half of itself.
        let tight = decide_crossing(below, 1.0, 1.0, rule(0, 1.0e-15, 0.0));
        assert_eq!(tight.refinement_time, Some(0.5 + 1.0e-15 / 2.0));

        // A candidate past the root within time_tol and expr_tol resolves
        // the crossing.
        let landed = decide_crossing(below, 1.0e-6, 0.5 + 1.0e-6, rule(0, 1.0e-3, 1.0e-3));
        assert!(landed.fired);
        assert_eq!(landed.refinement_time, None);

        // The direction filter selects the crossings that fire; the rest
        // still report their direction and ask for no landing.
        let above = CrossingAccepted {
            side: 1,
            value: 1.0,
            time: 0.0,
        };
        let falling = decide_crossing(above, -1.0, 1.0, rule(1, 0.0, 0.0));
        assert_eq!(
            (falling.direction, falling.fired, falling.refinement_time),
            (-1, false, None)
        );

        // A previous landing came to rest on the threshold: the root is
        // located there, and the next candidate is pulled back to within the
        // default window past it, where it fires.
        let resting = CrossingAccepted {
            side: -1,
            value: 0.0,
            time: 1.5,
        };
        let pulled = decide_crossing(resting, 0.5, 1.75, rule(0, 0.0, 0.0));
        assert!(pulled.fired);
        let landing = pulled
            .refinement_time
            .expect("a whole step past a located root");
        assert_eq!(landing, 1.5 + 64.0 * Value::EPSILON * 1.5 / 2.0);
        let fired = decide_crossing(resting, 1.0e-15, landing, rule(0, 0.0, 0.0));
        assert!(fired.fired);
        assert_eq!(fired.refinement_time, None);

        // A DC sweep point at the accepted time compares closed and has no
        // root to refine.
        let swept = decide_crossing(
            below,
            0.0,
            0.0,
            CrossingRule {
                same_time_static: true,
                ..rule(0, 0.0, 0.0)
            },
        );
        assert_eq!(
            (swept.direction, swept.fired, swept.refinement_time),
            (1, true, None)
        );
    }

    #[test]
    fn last_crossing_rejects_invalid_operands() {
        for direction in [
            -2.0,
            2.0,
            0.5,
            Value::NAN,
            Value::INFINITY,
            Value::NEG_INFINITY,
        ] {
            assert_eq!(
                evaluate_generated_last_crossing(GeneratedCrossState::INITIAL, 1.0, 0.0, direction),
                Err(GeneratedEventControlError::InvalidLastCrossingDirection)
            );
        }
        assert_eq!(
            evaluate_generated_last_crossing(GeneratedCrossState::INITIAL, Value::NAN, 0.0, 0.0),
            Err(GeneratedEventControlError::NonFiniteExpression)
        );
        assert_eq!(
            evaluate_generated_last_crossing(GeneratedCrossState::INITIAL, 1.0, -1.0, 0.0),
            Err(GeneratedEventControlError::InvalidTime)
        );
    }
}
