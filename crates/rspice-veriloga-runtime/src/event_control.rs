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
    UnrepresentableRefinement,
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
            Self::UnrepresentableRefinement => {
                "crossing cannot be refined to a representable interior time"
            }
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

fn generated_strictly_interior_time(start: Value, estimate: Value, end: Value) -> Option<Value> {
    if !estimate.is_finite() || end <= start {
        return None;
    }
    let target = generated_next_time_after(estimate.max(start));
    (target > start && target < end).then_some(target)
}

fn generated_crossing_time(accepted: GeneratedCrossState, value: Value, time: Value) -> Value {
    let accepted_magnitude = accepted.value.abs();
    let candidate_magnitude = value.abs();
    let scale = accepted_magnitude.max(candidate_magnitude);
    if scale == 0.0 || !scale.is_finite() {
        return time;
    }
    let accepted_scaled = accepted_magnitude / scale;
    let candidate_scaled = candidate_magnitude / scale;
    let fraction = (accepted_scaled / (accepted_scaled + candidate_scaled)).clamp(0.0, 1.0);
    accepted.time + fraction * (time - accepted.time)
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

    let rising = accepted.side < 0 && value >= 0.0;
    let falling = accepted.side > 0 && value <= 0.0;
    let crossing_direction = if rising {
        1
    } else if falling {
        -1
    } else {
        0
    };
    let crossing_time = if crossing_direction != 0 {
        generated_crossing_time(accepted, value, time)
    } else {
        -1.0
    };
    let fired = events_enabled
        && enabled
        && crossing_direction != 0
        && (direction == 0 || direction == crossing_direction);
    let refinement_time = if fired && !same_time_static {
        let effective_time_tol =
            generated_event_effective_tolerance(time_tol, accepted.time.abs().max(time.abs()));
        let effective_expr_tol =
            generated_event_effective_tolerance(expr_tol, accepted.value.abs().max(value.abs()));
        let time_error = (time - crossing_time).max(0.0);
        if time_error > effective_time_tol || value.abs() > effective_expr_tol {
            Some(
                generated_strictly_interior_time(accepted.time, crossing_time, time)
                    .ok_or(GeneratedEventControlError::UnrepresentableRefinement)?,
            )
        } else {
            None
        }
    } else {
        None
    };

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
/// This matches the portable VM's endpoint and coalescing rules.
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
