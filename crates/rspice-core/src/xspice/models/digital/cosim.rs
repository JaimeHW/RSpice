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

const RESOURCE_RUNTIME: &str = "d_cosim.runtime";
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

fn official_cosim_delay(ctx: &CmContext) -> Value {
    let delay = ctx.param_or("delay", 1.0e-9);
    if delay.is_finite() {
        delay.max(COSIM_DELAY_MIN)
    } else {
        COSIM_DELAY_MIN
    }
}

fn official_cosim_queue_size(ctx: &CmContext) -> usize {
    let queue_size = ctx.param_or("queue_size", 128.0).round();
    if queue_size.is_finite() {
        (queue_size as i64).max(COSIM_QUEUE_SIZE_MIN) as usize
    } else {
        COSIM_QUEUE_SIZE_MIN as usize
    }
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

fn sized_digital_vector(ctx: &CmContext, name: &str, width: usize) -> Vec<DigitalValue> {
    let values = ctx.input_digital_vector_values(name).unwrap_or(&[]);
    (0..width)
        .map(|index| values.get(index).copied().unwrap_or_default())
        .collect()
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
) -> Vec<(usize, DigitalCosimInputEvent)> {
    let mut events = Vec::new();

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
            && digital_value_code(value) != ctx.int_state(STATE_PREV_INPUT_START + state_index)
        {
            events.push((state_index, DigitalCosimInputEvent { time, index, value }));
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
            && digital_value_code(value) != ctx.int_state(STATE_PREV_INPUT_START + state_index)
        {
            events.push((
                state_index,
                DigitalCosimInputEvent {
                    time,
                    index: unified_index,
                    value,
                },
            ));
        }
    }

    events.sort_by(|(_, a), (_, b)| {
        a.time
            .partial_cmp(&b.time)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.index.cmp(&b.index))
    });
    events
}

fn zero_vector(width: usize) -> Vec<DigitalValue> {
    vec![DigitalValue::zero(); width]
}

fn effective_output_delay(ctx: &CmContext, vtime: Value) -> Value {
    let delay = official_cosim_delay(ctx) - (ctx.time - vtime);
    if delay <= 0.0 || !delay.is_finite() {
        COSIM_DELAY_MIN
    } else {
        delay
    }
}

fn set_initial_outputs(ctx: &mut CmContext, delay: Value) {
    let output_width = ctx.port_width("d_out");
    let inout_width = ctx.port_width("d_inout");
    if output_width > 0 {
        ctx.set_output_digital_vector("d_out", zero_vector(output_width), delay);
    }
    if inout_width > 0 {
        ctx.set_output_digital_vector("d_inout", zero_vector(inout_width), delay);
    }
}

fn normalize_runtime_outputs(
    ctx: &CmContext,
    port_name: &str,
    values: &[DigitalValue],
    width: usize,
) -> Vec<DigitalValue> {
    let mut normalized = ctx.output_digital_vector(port_name);
    normalized.resize(width, DigitalValue::zero());
    for (index, value) in values.iter().copied().take(width).enumerate() {
        normalized[index] = value;
    }
    normalized
}

fn schedule_changed_outputs(
    ctx: &mut CmContext,
    port_name: &str,
    values: &[DigitalValue],
    delay: Value,
) {
    let mut previous = ctx.output_digital_vector(port_name);
    previous.resize(values.len(), DigitalValue::zero());

    for (index, value) in values.iter().copied().enumerate() {
        if previous[index] != value {
            ctx.set_output_digital_vector_element(port_name, index, value, delay);
        }
    }
}

fn apply_cosim_step(ctx: &mut CmContext, result: DigitalCosimStep) -> CmResult<()> {
    let output_width = ctx.port_width("d_out");
    let inout_width = ctx.port_width("d_inout");
    let delay = effective_output_delay(ctx, result.vtime);

    if output_width > 0 {
        let outputs = normalize_runtime_outputs(ctx, "d_out", &result.outputs, output_width);
        schedule_changed_outputs(ctx, "d_out", &outputs, delay);
    }
    if inout_width > 0 {
        let inouts = normalize_runtime_outputs(ctx, "d_inout", &result.inouts, inout_width);
        schedule_changed_outputs(ctx, "d_inout", &inouts, delay);
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
        let input_state_count = ctx.port_width("d_in") + ctx.port_width("d_inout");
        ctx.allocate_int_states(STATE_PREV_INPUT_START + input_state_count);
        ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_NOT_INITIALIZED);
        ctx.allocate_states(input_state_count);
        for index in 0..input_state_count {
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
        let input_count = ctx.port_width("d_in");
        let output_count = ctx.port_width("d_out");
        let inout_count = ctx.port_width("d_inout");
        let connected_input_count = input_count + inout_count;
        let mut queue_size = official_cosim_queue_size(ctx);
        if connected_input_count > queue_size {
            queue_size = connected_input_count + 16;
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
            irreversible: ctx.param_or("irreversible", 1.0) as i64,
        };
        if spec.queue_size == 0 {
            return Err(CmError::InvalidParameter {
                name: "queue_size".to_string(),
                message: "must be at least 1".to_string(),
            });
        }

        let runtime = start_digital_cosim_runtime(&spec)?;
        let runtime: Arc<DigitalCosimRuntimeResource> = Arc::new(Mutex::new(runtime));
        ctx.set_resource(RESOURCE_RUNTIME, runtime);
        set_initial_outputs(ctx, official_cosim_delay(ctx));
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            return Ok(());
        }

        let input_width = ctx.port_width("d_in");
        let inout_width = ctx.port_width("d_inout");
        let inputs = sized_digital_vector(ctx, "d_in", input_width);
        let inouts = sized_digital_vector(ctx, "d_inout", inout_width);

        let runtime = ctx
            .resource::<DigitalCosimRuntimeResource>(RESOURCE_RUNTIME)
            .ok_or_else(|| d_cosim_error("runtime is not initialized"))?;
        let input_event_limit = {
            let runtime = runtime
                .lock()
                .map_err(|_| d_cosim_error("runtime lock is poisoned"))?;
            runtime.input_event_limit()
        };
        let input_events = collect_input_events(ctx, &inputs, &inouts, input_event_limit);
        let runtime_events: Vec<DigitalCosimInputEvent> = input_events
            .iter()
            .map(|(_, event)| event.clone())
            .collect();

        let mut runtime = runtime
            .lock()
            .map_err(|_| d_cosim_error("runtime lock is poisoned"))?;

        let mut results = Vec::new();
        if ctx.time == 0.0 {
            if ctx.int_state(STATE_TIME_ZERO_INITIALIZED) != COSIM_NOT_INITIALIZED {
                return Ok(());
            }
            ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_INPUTS_INITIALIZED);
            for index in 0..(input_width + inout_width) {
                ctx.set_state(index, 0.0);
            }
            for (index, value) in inputs.iter().copied().enumerate() {
                ctx.set_int_state(STATE_PREV_INPUT_START + index, digital_value_code(value));
            }
            for (index, value) in inouts.iter().copied().enumerate() {
                ctx.set_int_state(
                    STATE_PREV_INPUT_START + input_width + index,
                    digital_value_code(value),
                );
            }
            results.push(runtime.initialize(ctx.time, &inputs, &inouts)?);
        } else {
            if ctx.int_state(STATE_TIME_ZERO_INITIALIZED) == COSIM_INPUTS_INITIALIZED {
                results.push(runtime.startup_step(0.0)?);
                ctx.set_int_state(STATE_TIME_ZERO_INITIALIZED, COSIM_STARTUP_STEP_DONE);
            }
            results.push(runtime.step(ctx.time, &inputs, &inouts, &runtime_events)?);
        }
        drop(runtime);

        for result in results {
            apply_cosim_step(ctx, result)?;
        }
        for (state_index, event) in input_events {
            ctx.set_state(state_index, event.time);
            ctx.set_int_state(
                STATE_PREV_INPUT_START + state_index,
                digital_value_code(event.value),
            );
        }
        Ok(())
    }
}
