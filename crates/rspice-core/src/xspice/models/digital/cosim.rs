use crate::Value;
use crate::xspice::external::{
    DigitalCosimInputEvent, DigitalCosimRuntime, DigitalCosimSpec, DigitalCosimStep,
    start_digital_cosim_runtime,
};
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue,
    EvaluationPhase, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::{Arc, Mutex};

/// External irreversible digital co-simulation code model.
#[derive(Debug, Default)]
pub struct DigitalCosim;

type DigitalCosimRuntimeResource = Mutex<Box<dyn DigitalCosimRuntime>>;
type DigitalCosimInputScratchResource = DigitalCosimInputScratch;

const RESOURCE_RUNTIME: &str = "d_cosim.runtime";
const RESOURCE_INPUT_SCRATCH: &str = "d_cosim.input_scratch";
const STATE_TIME_ZERO_INITIALIZED: usize = 0;
const STATE_PREV_INPUT_START: usize = 1;
const COSIM_NOT_INITIALIZED: i64 = 0;
const COSIM_INPUTS_INITIALIZED: i64 = 1;
const COSIM_STARTUP_STEP_DONE: i64 = 2;
const EVENT_TIME_EPSILON: Value = 1.0e-18;
const COSIM_DELAY_MIN: Value = 1.0e-12;
const COSIM_QUEUE_SIZE_MIN: i64 = 1;

fn d_cosim_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_cosim: {}", message.into()))
}

#[derive(Default)]
struct DigitalCosimInputScratch {
    inputs: Vec<DigitalValue>,
    inouts: Vec<DigitalValue>,
    input_events: Vec<DigitalCosimInputEvent>,
    results: Vec<DigitalCosimStep>,
    output_changes: Vec<(usize, DigitalValue)>,
}

impl DigitalCosimInputScratch {
    fn with_capacities(
        input_count: usize,
        inout_count: usize,
        queue_size: usize,
        output_change_count: usize,
    ) -> CmResult<Self> {
        let mut scratch = Self::default();
        scratch
            .inputs
            .try_reserve_exact(input_count)
            .map_err(|err| {
                d_cosim_error(format!(
                    "unable to reserve {input_count} input values: {err}"
                ))
            })?;
        scratch
            .inouts
            .try_reserve_exact(inout_count)
            .map_err(|err| {
                d_cosim_error(format!(
                    "unable to reserve {inout_count} inout values: {err}"
                ))
            })?;
        scratch
            .input_events
            .try_reserve_exact(queue_size)
            .map_err(|err| {
                d_cosim_error(format!(
                    "unable to reserve {queue_size} input events: {err}"
                ))
            })?;
        scratch
            .results
            .try_reserve_exact(2)
            .map_err(|err| d_cosim_error(format!("unable to reserve step results: {err}")))?;
        scratch
            .output_changes
            .try_reserve_exact(output_change_count)
            .map_err(|err| {
                d_cosim_error(format!(
                    "unable to reserve {output_change_count} output changes: {err}"
                ))
            })?;
        Ok(scratch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DigitalCosimInputLayout {
    connected_input_count: usize,
    previous_input_start: usize,
    previous_inout_start: usize,
    int_state_count: usize,
}

fn digital_cosim_input_layout(
    input_count: usize,
    inout_count: usize,
) -> CmResult<DigitalCosimInputLayout> {
    let connected_input_count = input_count.checked_add(inout_count).ok_or_else(|| {
        d_cosim_error(format!(
            "input layout {input_count} inputs + {inout_count} inouts is too large"
        ))
    })?;
    let previous_input_start = STATE_PREV_INPUT_START;
    let previous_inout_start = previous_input_start
        .checked_add(input_count)
        .ok_or_else(|| {
            d_cosim_error(format!(
                "input layout {input_count} inputs + {inout_count} inouts is too large"
            ))
        })?;
    let int_state_count = previous_input_start
        .checked_add(connected_input_count)
        .ok_or_else(|| {
            d_cosim_error(format!(
                "input layout {input_count} inputs + {inout_count} inouts is too large"
            ))
        })?;

    Ok(DigitalCosimInputLayout {
        connected_input_count,
        previous_input_start,
        previous_inout_start,
        int_state_count,
    })
}

fn with_input_scratch<R>(
    ctx: &mut CmContext,
    f: impl FnOnce(&mut CmContext, &mut DigitalCosimInputScratch) -> CmResult<R>,
) -> CmResult<R> {
    let mut scratch = {
        let scratch = ctx
            .resource_mut::<DigitalCosimInputScratchResource>(RESOURCE_INPUT_SCRATCH)
            .ok_or_else(|| {
                d_cosim_error("input scratch is not initialized or is not uniquely owned")
            })?;
        std::mem::take(scratch)
    };
    let result = f(ctx, &mut scratch);
    let restore = ctx
        .resource_mut::<DigitalCosimInputScratchResource>(RESOURCE_INPUT_SCRATCH)
        .ok_or_else(|| d_cosim_error("input scratch is not initialized or is not uniquely owned"))
        .map(|slot| {
            *slot = scratch;
        });

    match (result, restore) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(err), Ok(())) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn official_cosim_delay(ctx: &CmContext) -> CmResult<Value> {
    let delay = ctx.param_or("delay", 1.0e-9);
    if !delay.is_finite() {
        return Err(CmError::InvalidParameter {
            name: "delay".to_string(),
            message: format!("value must be finite, got {delay}"),
        });
    }
    Ok(delay.max(COSIM_DELAY_MIN))
}

fn official_cosim_queue_size(ctx: &CmContext) -> CmResult<usize> {
    let queue_size = ctx.param_or("queue_size", 128.0).round();
    if !queue_size.is_finite() {
        return Err(CmError::InvalidParameter {
            name: "queue_size".to_string(),
            message: format!("value must be finite, got {queue_size}"),
        });
    }
    Ok((queue_size as i64).max(COSIM_QUEUE_SIZE_MIN) as usize)
}

fn official_cosim_integer(ctx: &CmContext, name: &str, default: Value) -> CmResult<i64> {
    let value = ctx.param_or(name, default).round();
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("integer value must be finite, got {value}"),
        });
    }
    Ok(value as i64)
}

fn d_cosim_inout_port() -> PortSpec {
    PortSpec {
        name: "d_inout".to_string(),
        direction: PortDirection::InOut,
        default_type: PortType::Digital,
        allowed_types: vec![PortType::Digital],
        is_vector: true,
        null_allowed: true,
        vector_min_len: Some(0),
        vector_max_len: None,
        description: "Digital bidirectional vector".to_string(),
    }
}

fn fill_sized_digital_vector(
    ctx: &CmContext,
    name: &str,
    width: usize,
    output: &mut Vec<DigitalValue>,
) -> CmResult<()> {
    if output.capacity() < width {
        let additional = width - output.capacity();
        output.try_reserve_exact(additional).map_err(|err| {
            d_cosim_error(format!(
                "unable to reserve {width} {name} vector value(s): {err}"
            ))
        })?;
    }

    let values = ctx.input_digital_vector_values(name).unwrap_or(&[]);
    output.clear();
    for index in 0..width {
        output.push(values.get(index).copied().unwrap_or_default());
    }
    Ok(())
}

fn event_is_new(event_time: Value, previous_time: Value) -> bool {
    event_time.is_finite() && event_time > previous_time + EVENT_TIME_EPSILON
}

fn digital_state_code(state: DigitalState) -> i64 {
    match state {
        DigitalState::Zero => 0,
        DigitalState::One => 1,
        DigitalState::Unknown => 2,
        DigitalState::ZeroR => 3,
        DigitalState::OneR => 4,
        DigitalState::UnknownR => 5,
        DigitalState::ZeroZ => 6,
        DigitalState::OneZ => 7,
        DigitalState::UnknownZ => 8,
        DigitalState::HighZ => 9,
    }
}

fn digital_strength_code(strength: DigitalStrength) -> i64 {
    match strength {
        DigitalStrength::Strong => 0,
        DigitalStrength::Resistive => 1,
        DigitalStrength::HighZ => 2,
        DigitalStrength::Undetermined => 3,
    }
}

fn digital_value_code(value: DigitalValue) -> i64 {
    digital_state_code(value.state) * 4 + digital_strength_code(value.strength)
}

fn input_event_allowed(input_event_limit: Option<usize>, unified_index: usize) -> bool {
    input_event_limit.is_none_or(|limit| unified_index < limit)
}

fn collect_input_events(
    ctx: &CmContext,
    inputs: &[DigitalValue],
    inouts: &[DigitalValue],
    input_event_limit: Option<usize>,
    layout: DigitalCosimInputLayout,
    events: &mut Vec<DigitalCosimInputEvent>,
) {
    events.clear();
    for (index, value) in inputs.iter().copied().enumerate() {
        if !input_event_allowed(input_event_limit, index) {
            continue;
        }
        let Some(time) = ctx.input_digital_vector_event_time("d_in", index) else {
            continue;
        };
        let state_index = index;
        if time <= ctx.time + EVENT_TIME_EPSILON
            && event_is_new(time, ctx.state(state_index))
            && digital_value_code(value) != ctx.int_state(layout.previous_input_start + state_index)
        {
            events.push(DigitalCosimInputEvent { time, index, value });
        }
    }

    let offset = inputs.len();
    for (index, value) in inouts.iter().copied().enumerate() {
        let unified_index = offset + index;
        if !input_event_allowed(input_event_limit, unified_index) {
            continue;
        }
        let Some(time) = ctx.input_digital_vector_event_time("d_inout", index) else {
            continue;
        };
        let state_index = unified_index;
        if time <= ctx.time + EVENT_TIME_EPSILON
            && event_is_new(time, ctx.state(state_index))
            && digital_value_code(value) != ctx.int_state(layout.previous_input_start + state_index)
        {
            events.push(DigitalCosimInputEvent {
                time,
                index: unified_index,
                value,
            });
        }
    }

    events.sort_by(|a, b| {
        a.time
            .partial_cmp(&b.time)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.index.cmp(&b.index))
    });
}

fn effective_output_delay(ctx: &CmContext, vtime: Value) -> CmResult<Value> {
    let delay = official_cosim_delay(ctx)? - (ctx.time - vtime);
    if delay <= 0.0 || !delay.is_finite() {
        Ok(COSIM_DELAY_MIN)
    } else {
        Ok(delay)
    }
}

fn set_initial_outputs(ctx: &mut CmContext, delay: Value) {
    let output_width = ctx.port_width("d_out");
    let inout_width = ctx.port_width("d_inout");
    if output_width > 0 {
        ctx.set_output_digital_vector_from_context_fn("d_out", output_width, delay, |_, _| {
            DigitalValue::zero()
        });
    }
    if inout_width > 0 {
        ctx.set_output_digital_vector_from_context_fn("d_inout", inout_width, delay, |_, _| {
            DigitalValue::zero()
        });
    }
}

fn schedule_normalized_runtime_outputs(
    ctx: &mut CmContext,
    port_name: &str,
    runtime_values: &[DigitalValue],
    width: usize,
    delay: Value,
    changes: &mut Vec<(usize, DigitalValue)>,
) {
    changes.clear();
    let previous_values = ctx.output_digital_vector_values(port_name);
    for index in 0..width {
        let previous = previous_values
            .and_then(|values| values.get(index).copied())
            .unwrap_or_else(DigitalValue::zero);
        let value = runtime_values.get(index).copied().unwrap_or(previous);
        if previous != value {
            changes.push((index, value));
        }
    }
    for (index, value) in changes.iter().copied() {
        ctx.set_output_digital_vector_element(port_name, index, value, delay);
    }
}

fn apply_cosim_step(
    ctx: &mut CmContext,
    result: DigitalCosimStep,
    output_changes: &mut Vec<(usize, DigitalValue)>,
) -> CmResult<()> {
    let output_width = ctx.port_width("d_out");
    let inout_width = ctx.port_width("d_inout");
    let delay = effective_output_delay(ctx, result.vtime)?;

    if output_width > 0 {
        schedule_normalized_runtime_outputs(
            ctx,
            "d_out",
            &result.outputs,
            output_width,
            delay,
            output_changes,
        );
    }
    if inout_width > 0 {
        schedule_normalized_runtime_outputs(
            ctx,
            "d_inout",
            &result.inouts,
            inout_width,
            delay,
            output_changes,
        );
    }
    Ok(())
}

fn apply_cosim_steps(
    ctx: &mut CmContext,
    results: &mut Vec<DigitalCosimStep>,
    output_changes: &mut Vec<(usize, DigitalValue)>,
) -> CmResult<()> {
    for result in results.drain(..) {
        apply_cosim_step(ctx, result, output_changes)?;
    }
    Ok(())
}

impl CodeModel for DigitalCosim {
    fn name(&self) -> &str {
        "d_cosim"
    }

    fn description(&self) -> &str {
        "Bridge to an irreversible digital co-simulation runtime"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("d_in", PortType::Digital)
                    .with_vector_min_len(0)
                    .nullable()
                    .with_description("Digital input vector"),
                PortSpec::vector_output("d_out", PortType::Digital)
                    .with_vector_min_len(0)
                    .nullable()
                    .with_description("Digital output vector"),
                d_cosim_inout_port(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("delay", 1.0e-9)
                    .with_description("Output delay time, clamped to official lower limit"),
                ParamSpec::string("simulation", "")
                    .required()
                    .with_description("Shared library path or provider simulation identifier"),
                ParamSpec::string_vector("lib_args", Vec::new())
                    .with_description("Arguments for the co-simulation library"),
                ParamSpec::string_vector("sim_args", Vec::new())
                    .with_description("Arguments for the co-simulation payload"),
                ParamSpec::integer("queue_size", 128)
                    .with_description("Input queue size, clamped to official lower limit"),
                ParamSpec::integer("irreversible", 1),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_count = ctx.port_width("d_in");
        let output_count = ctx.port_width("d_out");
        let inout_count = ctx.port_width("d_inout");
        let input_layout = digital_cosim_input_layout(input_count, inout_count)?;
        ctx.allocate_int_states(input_layout.int_state_count);
        ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_NOT_INITIALIZED);
        ctx.allocate_states(input_layout.connected_input_count);
        for index in 0..input_layout.connected_input_count {
            ctx.set_initial_state(index, -1.0);
        }

        let simulation = ctx
            .string_param("simulation")
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| CmError::InvalidParameter {
                name: "simulation".to_string(),
                message: "must not be empty".to_string(),
            })?
            .to_string();
        let mut queue_size = official_cosim_queue_size(ctx)?;
        if input_layout.connected_input_count > queue_size {
            queue_size = input_layout
                .connected_input_count
                .checked_add(16)
                .ok_or_else(|| d_cosim_error("input queue size is too large"))?;
        }
        let spec = DigitalCosimSpec {
            simulation,
            lib_args: ctx
                .string_vector_param("lib_args")
                .map(|values| values.to_vec())
                .unwrap_or_default(),
            sim_args: ctx
                .string_vector_param("sim_args")
                .map(|values| values.to_vec())
                .unwrap_or_default(),
            input_count,
            output_count,
            inout_count,
            queue_size,
            irreversible: official_cosim_integer(ctx, "irreversible", 1.0)?,
        };
        if spec.irreversible <= 0 {
            return Err(CmError::InvalidParameter {
                name: "irreversible".to_string(),
                message: "reversible d_cosim rollback is not implemented; use irreversible > 0"
                    .to_string(),
            });
        }
        if spec.queue_size == 0 {
            return Err(CmError::InvalidParameter {
                name: "queue_size".to_string(),
                message: "must be at least 1".to_string(),
            });
        }

        let runtime = start_digital_cosim_runtime(&spec)?;
        let runtime: Arc<DigitalCosimRuntimeResource> = Arc::new(Mutex::new(runtime));
        ctx.set_resource(RESOURCE_RUNTIME, runtime);
        ctx.set_resource(
            RESOURCE_INPUT_SCRATCH,
            Arc::new(DigitalCosimInputScratch::with_capacities(
                input_count,
                inout_count,
                spec.queue_size,
                output_count.max(inout_count),
            )?),
        );
        set_initial_outputs(ctx, official_cosim_delay(ctx)?);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            return Ok(());
        }

        let input_width = ctx.port_width("d_in");
        let inout_width = ctx.port_width("d_inout");
        let input_layout = digital_cosim_input_layout(input_width, inout_width)?;
        with_input_scratch(ctx, |ctx, input_scratch| {
            let DigitalCosimInputScratch {
                inputs,
                inouts,
                input_events,
                results,
                output_changes,
            } = input_scratch;
            fill_sized_digital_vector(ctx, "d_in", input_width, inputs)?;
            fill_sized_digital_vector(ctx, "d_inout", inout_width, inouts)?;

            let runtime = ctx
                .resource::<DigitalCosimRuntimeResource>(RESOURCE_RUNTIME)
                .ok_or_else(|| d_cosim_error("runtime is not initialized"))?;
            let input_event_limit = {
                let runtime = runtime
                    .lock()
                    .map_err(|_| d_cosim_error("runtime lock is poisoned"))?;
                runtime.input_event_limit()
            };
            collect_input_events(
                ctx,
                inputs,
                inouts,
                input_event_limit,
                input_layout,
                input_events,
            );

            let mut runtime = runtime
                .lock()
                .map_err(|_| d_cosim_error("runtime lock is poisoned"))?;

            results.clear();
            if ctx.time == 0.0 {
                if ctx.int_state(STATE_TIME_ZERO_INITIALIZED) != COSIM_NOT_INITIALIZED {
                    return Ok(());
                }
                ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_INPUTS_INITIALIZED);
                for index in 0..input_layout.connected_input_count {
                    ctx.set_state(index, 0.0);
                }
                for (index, value) in inputs.iter().copied().enumerate() {
                    ctx.set_int_state(
                        input_layout.previous_input_start + index,
                        digital_value_code(value),
                    );
                }
                for (index, value) in inouts.iter().copied().enumerate() {
                    ctx.set_int_state(
                        input_layout.previous_inout_start + index,
                        digital_value_code(value),
                    );
                }
                results.push(runtime.initialize(ctx.time, inputs, inouts)?);
            } else {
                if ctx.int_state(STATE_TIME_ZERO_INITIALIZED) == COSIM_INPUTS_INITIALIZED {
                    results.push(runtime.startup_step(0.0)?);
                    ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_STARTUP_STEP_DONE);
                }
                results.push(runtime.step(ctx.time, inputs, inouts, input_events)?);
            }
            drop(runtime);
            for event in input_events.iter() {
                ctx.set_state(event.index, event.time);
                ctx.set_int_state(
                    input_layout.previous_input_start + event.index,
                    digital_value_code(event.value),
                );
            }
            apply_cosim_steps(ctx, results, output_changes)?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::ParamType;
    use crate::xspice::context::InputValue;

    #[test]
    fn d_cosim_metadata_matches_ngspice46_interface() {
        let ports = DigitalCosim.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["d_in", "d_out", "d_inout"]
        );
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[1].direction, PortDirection::Out);
        assert_eq!(ports[2].direction, PortDirection::InOut);
        for port in ports {
            assert_eq!(port.default_type, PortType::Digital);
            assert_eq!(port.allowed_types, vec![PortType::Digital]);
            assert!(port.is_vector);
            assert!(port.null_allowed);
            assert_eq!(port.vector_min_len, Some(0));
            assert_eq!(port.vector_max_len, None);
        }

        let params = DigitalCosim.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type))
                .collect::<Vec<_>>(),
            vec![
                ("delay", &ParamType::Real),
                ("simulation", &ParamType::String),
                ("lib_args", &ParamType::StringVector),
                ("sim_args", &ParamType::StringVector),
                ("queue_size", &ParamType::Integer),
                ("irreversible", &ParamType::Integer),
            ]
        );
        assert_eq!(params[0].default, 1.0e-9);
        assert!(params[1].required);
        assert_eq!(params[4].default, 128.0);
        assert_eq!(params[5].default, 1.0);
    }

    #[test]
    fn d_cosim_limit_helpers_follow_ngspice46_lower_bounds() {
        let mut ctx = CmContext::new();

        assert_eq!(official_cosim_delay(&ctx).unwrap(), 1.0e-9);
        assert_eq!(official_cosim_queue_size(&ctx).unwrap(), 128);

        ctx.set_param("delay", 0.0);
        ctx.set_param("queue_size", 0.0);
        assert_eq!(official_cosim_delay(&ctx).unwrap(), COSIM_DELAY_MIN);
        assert_eq!(
            official_cosim_queue_size(&ctx).unwrap(),
            COSIM_QUEUE_SIZE_MIN as usize
        );

        ctx.set_param("delay", Value::NAN);
        ctx.set_param("queue_size", Value::NAN);
        ctx.set_param("irreversible", Value::NAN);
        assert_invalid_param(official_cosim_delay(&ctx), "delay");
        assert_invalid_param(official_cosim_queue_size(&ctx), "queue_size");
        assert_invalid_param(
            official_cosim_integer(&ctx, "irreversible", 1.0),
            "irreversible",
        );
    }

    fn assert_invalid_param<T: std::fmt::Debug>(result: CmResult<T>, expected_name: &str) {
        match result {
            Err(CmError::InvalidParameter { name, .. }) => assert_eq!(name, expected_name),
            other => panic!("expected InvalidParameter for {expected_name}, got {other:?}"),
        }
    }

    #[test]
    fn d_cosim_sized_input_fill_reuses_existing_buffer() {
        let mut ctx = CmContext::new();
        ctx.set_input(
            "d_in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        let mut values = Vec::new();

        fill_sized_digital_vector(&ctx, "d_in", 2, &mut values).expect("fill d_in vector");
        assert_eq!(values, vec![DigitalValue::one(), DigitalValue::zero()]);
        let first_ptr = values.as_ptr();
        let first_capacity = values.capacity();

        ctx.set_input(
            "d_in",
            InputValue::DigitalVector(vec![DigitalValue::unknown()]),
        );
        fill_sized_digital_vector(&ctx, "d_in", 2, &mut values).expect("fill d_in vector");

        assert_eq!(
            values,
            vec![DigitalValue::unknown(), DigitalValue::default()]
        );
        assert_eq!(values.as_ptr(), first_ptr);
        assert_eq!(values.capacity(), first_capacity);
    }

    #[test]
    fn d_cosim_initial_outputs_are_streamed_vectors() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("d_out", 2);
        ctx.set_port_width("d_inout", 1);

        set_initial_outputs(&mut ctx, 3.0e-9);

        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].port_name, "d_out");
        assert_eq!(events[0].values, vec![DigitalValue::zero(); 2]);
        assert_eq!(events[0].delay, 3.0e-9);
        assert_eq!(events[1].port_name, "d_inout");
        assert_eq!(events[1].values, vec![DigitalValue::zero()]);
        assert_eq!(events[1].delay, 3.0e-9);
    }

    #[test]
    fn d_cosim_input_events_use_unified_indices_for_runtime_batch() {
        let mut ctx = CmContext::new();
        ctx.time = 2.0e-9;

        let inputs = [DigitalValue::one(), DigitalValue::zero()];
        let inouts = [DigitalValue::unknown()];
        let input_layout =
            digital_cosim_input_layout(inputs.len(), inouts.len()).expect("valid input layout");
        ctx.allocate_states(input_layout.connected_input_count);
        ctx.allocate_int_states(input_layout.int_state_count);
        for index in 0..input_layout.connected_input_count {
            ctx.set_int_state(input_layout.previous_input_start + index, -1);
        }
        ctx.set_input_digital_vector_event_times("d_in", vec![Some(1.5e-9), Some(1.0e-9)]);
        ctx.set_input_digital_vector_event_times("d_inout", vec![Some(1.0e-9)]);

        let mut events = Vec::new();
        collect_input_events(&ctx, &inputs, &inouts, None, input_layout, &mut events);

        assert_eq!(events.len(), 3);
        assert_eq!(
            events.iter().map(|event| event.index).collect::<Vec<_>>(),
            vec![1, 2, 0]
        );
        assert_eq!(
            events.iter().map(|event| event.value).collect::<Vec<_>>(),
            vec![
                DigitalValue::zero(),
                DigitalValue::unknown(),
                DigitalValue::one()
            ]
        );
        let first_ptr = events.as_ptr();
        let first_capacity = events.capacity();

        collect_input_events(&ctx, &inputs, &inouts, Some(2), input_layout, &mut events);
        assert_eq!(events.as_ptr(), first_ptr);
        assert_eq!(events.capacity(), first_capacity);
        assert_eq!(
            events.iter().map(|event| event.index).collect::<Vec<_>>(),
            vec![1, 0]
        );
    }

    #[test]
    fn d_cosim_partial_runtime_outputs_preserve_previous_values() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("d_out", 3);
        ctx.set_port_width("d_inout", 2);
        set_initial_outputs(&mut ctx, 0.0);
        ctx.take_pending_events();

        ctx.time = 1.0e-9;
        let vtime = ctx.time;
        let mut output_changes = Vec::new();
        apply_cosim_step(
            &mut ctx,
            DigitalCosimStep {
                vtime,
                outputs: vec![DigitalValue::one()],
                inouts: Vec::new(),
            },
            &mut output_changes,
        )
        .expect("partial cosim step applies");

        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port_name, "d_out");
        assert_eq!(events[0].start_index, 0);
        assert_eq!(events[0].values, vec![DigitalValue::one()]);
    }

    #[test]
    fn d_cosim_result_drain_reuses_existing_buffer() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("d_out", 1);
        let mut results = vec![DigitalCosimStep {
            vtime: 0.0,
            outputs: vec![DigitalValue::one()],
            inouts: Vec::new(),
        }];
        let first_ptr = results.as_ptr();
        let first_capacity = results.capacity();
        let mut output_changes = Vec::new();

        apply_cosim_steps(&mut ctx, &mut results, &mut output_changes)
            .expect("cosim result applies");

        assert!(results.is_empty());
        assert_eq!(results.as_ptr(), first_ptr);
        assert_eq!(results.capacity(), first_capacity);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![DigitalValue::one()]);
    }
}
