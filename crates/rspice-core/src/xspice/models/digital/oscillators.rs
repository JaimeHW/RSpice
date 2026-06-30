use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase, PortDirection};
use std::sync::{Arc, OnceLock};

#[derive(Debug, Default)]
pub struct DigitalOscillator;

#[derive(Debug, Default)]
pub struct DigitalPwmOscillator;

#[derive(Debug, Default)]
pub struct NumericallyControlledOscillator;

const MIN_FREQUENCY: Value = 1.0e-16;
const CONTROL_TABLE_RESOURCE: &str = "xspice.digital_oscillator.control_table";
const FACTOR1: Value = 0.75;
const FACTOR2: Value = 0.8;
const TIME_EPSILON: Value = 1.0e-18;
const NCO_BASE_FREQUENCY: Value = 8.17578;
const NCO_SEMITONE_RATIO: Value = 1.059_463_094;

const OSC_LAST_TIME: usize = 0;
const OSC_LAST_STATE: usize = 0;
const OSC_INITIALIZED: usize = 1;
const NCO_NEXT_TIME: usize = 0;
const NCO_OUTPUT_STATE: usize = 0;

#[derive(Debug, Clone, Copy)]
struct ControlTablePoint {
    control: Value,
    value: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct ControlTableSignature {
    model: &'static str,
    control_name: &'static str,
    value_name: &'static str,
    controls_present: bool,
    values_present: bool,
    controls: Vec<Value>,
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct ControlTableResource {
    signature: ControlTableSignature,
    result: CmResult<Option<Arc<Vec<ControlTablePoint>>>>,
}

fn digital_oscillator_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "cntl_in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                ],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Control input".to_string(),
            },
            PortSpec::output("out", PortType::Digital).with_description("Digital output"),
        ]
    })
}

fn nco_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec::vector_input("in", PortType::Digital)
                .with_vector_len_range(7, 7)
                .with_description("7-bit MIDI program input"),
            PortSpec::output("out", PortType::Digital).with_description("Oscillator output"),
        ]
    })
}

fn d_osc_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("cntl_array", vec![0.0, 1.0])
                .with_vector_min_len(2)
                .with_description("Control-input lookup points"),
            ParamSpec::real_vector("freq_array", vec![1.0e6, 2.0e6])
                .with_vector_min_len(2)
                .with_description("Frequency lookup points"),
            ParamSpec::real("duty_cycle", 0.5)
                .with_description("Fraction of each cycle spent high, clamped to official limits"),
            ParamSpec::real("init_phase", 0.0)
                .with_description("Initial output phase in degrees, clamped to official limits"),
            ParamSpec::real("rise_delay", 1.0e-9)
                .with_description("Unused compatibility delay, accepted outside official limits"),
            ParamSpec::real("fall_delay", 1.0e-9)
                .with_description("Unused compatibility delay, accepted outside official limits"),
        ]
    })
}

fn d_pwm_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("cntl_array", vec![-1.0, 1.0])
                .with_vector_min_len(2)
                .with_description("Control-input lookup points"),
            ParamSpec::real_vector("dc_array", vec![0.0, 1.0])
                .with_vector_min_len(2)
                .with_description("Duty-cycle lookup points"),
            ParamSpec::real("frequency", 1.0e6)
                .with_description("Oscillator frequency, clamped to official lower limit"),
            ParamSpec::real("init_phase", 0.0)
                .with_description("Initial output phase in degrees, clamped to official limits"),
            ParamSpec::real("rise_delay", 1.0e-9)
                .with_description("Unused compatibility delay, accepted outside official limits"),
            ParamSpec::real("fall_delay", 1.0e-9)
                .with_description("Unused compatibility delay, accepted outside official limits"),
        ]
    })
}

fn nco_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("delay", 1.0e-9)
                .with_min(1.0e-15)
                .with_description("Output event delay"),
            ParamSpec::real("mult_factor", 1.0)
                .with_min(1.0e-9)
                .with_description("Frequency multiplier"),
        ]
    })
}

fn oscillator_error(model: &str, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("{model}: {}", message.into()))
}

fn sanitize_d_osc_frequency(frequency: Value) -> Value {
    if frequency <= 0.0 {
        MIN_FREQUENCY
    } else {
        frequency
    }
}

fn sanitize_d_pwm_duty_cycle(duty: Value) -> Value {
    duty.clamp(0.01, 0.99)
}

fn control_table_signature(
    ctx: &CmContext,
    model: &'static str,
    control_name: &'static str,
    value_name: &'static str,
) -> ControlTableSignature {
    let controls = ctx.real_vector_param(control_name);
    let values = ctx.real_vector_param(value_name);
    ControlTableSignature {
        model,
        control_name,
        value_name,
        controls_present: controls.is_some(),
        values_present: values.is_some(),
        controls: controls.unwrap_or(&[]).to_vec(),
        values: values.unwrap_or(&[]).to_vec(),
    }
}

#[cfg(test)]
fn validate_table(
    ctx: &mut CmContext,
    model: &'static str,
    control_name: &'static str,
    value_name: &'static str,
    sanitize_value: fn(Value) -> Value,
) -> CmResult<Arc<Vec<ControlTablePoint>>> {
    validate_table_optional(ctx, model, control_name, value_name, sanitize_value)?.ok_or_else(
        || {
            oscillator_error(
                model,
                format!("badly-formed control table {control_name}/{value_name}"),
            )
        },
    )
}

fn validate_table_optional(
    ctx: &mut CmContext,
    model: &'static str,
    control_name: &'static str,
    value_name: &'static str,
    sanitize_value: fn(Value) -> Value,
) -> CmResult<Option<Arc<Vec<ControlTablePoint>>>> {
    let signature = control_table_signature(ctx, model, control_name, value_name);
    if let Some(resource) = ctx.resource::<ControlTableResource>(CONTROL_TABLE_RESOURCE)
        && resource.signature == signature
    {
        return resource.result.clone();
    }

    let result =
        validate_table_optional_uncached(ctx, model, control_name, value_name, sanitize_value)
            .map(|table| table.map(Arc::new));
    ctx.set_resource(
        CONTROL_TABLE_RESOURCE,
        Arc::new(ControlTableResource {
            signature,
            result: result.clone(),
        }),
    );
    result
}

fn validate_table_optional_uncached(
    ctx: &CmContext,
    model: &str,
    control_name: &str,
    value_name: &str,
    sanitize_value: fn(Value) -> Value,
) -> CmResult<Option<Vec<ControlTablePoint>>> {
    let controls = ctx
        .real_vector_param(control_name)
        .ok_or_else(|| CmError::MissingParameter(control_name.to_string()))?;
    let values = ctx
        .real_vector_param(value_name)
        .ok_or_else(|| CmError::MissingParameter(value_name.to_string()))?;

    if controls.len() < 2 || values.len() < 2 {
        return Err(oscillator_error(
            model,
            format!(
                "{control_name}/{value_name} require at least 2 points, got {}/{}",
                controls.len(),
                values.len()
            ),
        ));
    }
    if controls.len() != values.len() {
        return Ok(None);
    }

    for (index, (&control, &value)) in controls.iter().zip(values).enumerate() {
        if !control.is_finite() {
            return Err(oscillator_error(
                model,
                format!("{control_name}[{index}] must be finite, got {control}"),
            ));
        }
        if !value.is_finite() {
            return Err(oscillator_error(
                model,
                format!("{value_name}[{index}] must be finite, got {value}"),
            ));
        }
        if index > 0 && control <= controls[index - 1] {
            return Ok(None);
        }
    }

    Ok(Some(
        controls
            .iter()
            .zip(values)
            .map(|(&control, &value)| ControlTablePoint {
                control,
                value: sanitize_value(value),
            })
            .collect(),
    ))
}

fn interpolate_table(table: &[ControlTablePoint], control: Value) -> Value {
    let right_index = table
        .iter()
        .position(|point| point.control > control)
        .unwrap_or(table.len());
    let left = if right_index == 0 {
        0
    } else if right_index == table.len() {
        table.len() - 2
    } else {
        right_index - 1
    };
    let right = left + 1;
    let left_control = table[left].control;
    let right_control = table[right].control;
    let left_value = table[left].value;
    let right_value = table[right].value;
    let span = right_control - left_control;
    if span.abs() <= Value::EPSILON {
        return left_value;
    }
    left_value + (control - left_control) * ((right_value - left_value) / span)
}

fn d_osc_period_from_table(table: &[ControlTablePoint], control: Value) -> Value {
    1.0 / interpolate_table(table, control)
}

#[cfg(test)]
fn d_osc_period(ctx: &mut CmContext) -> CmResult<Value> {
    let table = validate_table(
        ctx,
        "d_osc",
        "cntl_array",
        "freq_array",
        sanitize_d_osc_frequency,
    )?;
    Ok(d_osc_period_from_table(&table, ctx.input("cntl_in")))
}

fn d_pwm_duty_cycle_from_table(table: &[ControlTablePoint], control: Value) -> Value {
    sanitize_d_pwm_duty_cycle(interpolate_table(table, control))
}

#[cfg(test)]
fn d_pwm_duty_cycle(ctx: &mut CmContext) -> CmResult<Value> {
    let table = validate_table(
        ctx,
        "d_pwm",
        "cntl_array",
        "dc_array",
        sanitize_d_pwm_duty_cycle,
    )?;
    Ok(d_pwm_duty_cycle_from_table(&table, ctx.input("cntl_in")))
}

fn d_osc_duty_cycle(ctx: &CmContext) -> CmResult<Value> {
    let duty = ctx.param("duty_cycle");
    if !duty.is_finite() {
        return Err(oscillator_error(
            "d_osc",
            format!("duty_cycle must be finite, got {duty}"),
        ));
    }
    Ok(duty.clamp(1.0e-6, 0.999_999))
}

fn d_pwm_frequency(ctx: &CmContext) -> CmResult<Value> {
    let frequency = ctx.param("frequency");
    if !frequency.is_finite() {
        return Err(oscillator_error(
            "d_pwm",
            format!("frequency must be finite, got {frequency}"),
        ));
    }
    Ok(frequency.max(1.0e-6))
}

fn scalar_param_in_range(
    ctx: &CmContext,
    model: &str,
    name: &str,
    min: Value,
    max: Value,
) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() || value < min || value > max {
        return Err(oscillator_error(
            model,
            format!("{name} must be in [{min}, {max}], got {value}"),
        ));
    }
    Ok(value)
}

fn scalar_finite_param(ctx: &CmContext, model: &str, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(oscillator_error(
            model,
            format!("{name} must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn oscillator_set_state(ctx: &mut CmContext, index: usize, value: Value) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_state(index, value);
    }
}

fn oscillator_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

fn initialize_timing_state(ctx: &mut CmContext, period: Value, duty_cycle: Value) -> (Value, i64) {
    let mut phase = ctx.param("init_phase").clamp(-180.0, 360.0) / 360.0;
    if phase < 0.0 {
        phase += 1.0;
    }

    let last_time = period * (1.0 - duty_cycle - phase);
    let (last_time, last_state) = if last_time < 0.0 {
        (last_time, 1)
    } else {
        (-period * phase, 0)
    };
    oscillator_set_state(ctx, OSC_LAST_TIME, last_time);
    oscillator_set_int_state(ctx, OSC_LAST_STATE, last_state);
    oscillator_set_int_state(ctx, OSC_INITIALIZED, 1);
    (last_time, last_state)
}

fn state_to_value(state: i64) -> DigitalValue {
    if state == 0 {
        DigitalValue::zero()
    } else {
        DigitalValue::one()
    }
}

fn request_future_breakpoint(ctx: &mut CmContext, time: Value) {
    if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
        return;
    }
    if time.is_finite() && time > ctx.time + TIME_EPSILON {
        ctx.request_breakpoint(time);
    }
}

fn nco_param_min(ctx: &CmContext, name: &str, default: Value, min: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if !value.is_finite() || value < min {
        return Err(oscillator_error(
            "nco",
            format!("{name} must be finite and >= {min:e}, got {value}"),
        ));
    }
    Ok(value)
}

fn nco_note_index(ctx: &CmContext) -> usize {
    let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
    let mut index = 0usize;
    let mut scale_factor = 64usize;
    for bit in 0..7 {
        if inputs
            .get(bit)
            .is_some_and(|value| value.state == DigitalState::One)
        {
            index += scale_factor;
        }
        scale_factor /= 2;
    }
    index
}

fn nco_half_period(ctx: &CmContext) -> CmResult<Value> {
    let mult_factor = nco_param_min(ctx, "mult_factor", 1.0, 1.0e-9)?;
    let index = nco_note_index(ctx);
    let frequency = NCO_BASE_FREQUENCY * mult_factor * NCO_SEMITONE_RATIO.powi(index as i32);
    if !frequency.is_finite() || frequency <= 0.0 {
        return Err(oscillator_error(
            "nco",
            format!("computed frequency must be positive and finite, got {frequency:e}"),
        ));
    }
    Ok(1.0 / frequency)
}

fn nco_set_state(ctx: &mut CmContext, next_time: Value, output_state: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_state(NCO_NEXT_TIME, next_time);
        ctx.set_int_state(NCO_OUTPUT_STATE, output_state);
    }
}

fn evaluate_digital_oscillator(
    ctx: &mut CmContext,
    period: Value,
    duty_cycle: Value,
) -> CmResult<()> {
    if !ctx.is_transient() {
        return Ok(());
    }

    let initialized = if ctx.time == 0.0 || ctx.int_state(OSC_INITIALIZED) == 0 {
        let timing_state = initialize_timing_state(ctx, period, duty_cycle);
        ctx.set_output_digital("out", state_to_value(timing_state.1), 0.0);
        Some(timing_state)
    } else {
        None
    };

    let last_time = initialized
        .map(|(last_time, _)| last_time)
        .unwrap_or_else(|| ctx.state(OSC_LAST_TIME));
    if ctx.time + TIME_EPSILON < last_time {
        request_future_breakpoint(ctx, last_time);
        return Ok(());
    }

    let last_state = initialized
        .map(|(_, last_state)| last_state)
        .unwrap_or_else(|| ctx.int_state(OSC_LAST_STATE));
    let interval = if last_state == 0 {
        period * (1.0 - duty_cycle)
    } else {
        period * duty_cycle
    };
    if !interval.is_finite() || interval == 0.0 {
        return Err(oscillator_error(
            "digital oscillator",
            format!("computed transition interval must be finite and non-zero, got {interval}"),
        ));
    }

    let threshold = last_time + FACTOR1 * interval;
    if ctx.time + TIME_EPSILON < threshold {
        request_future_breakpoint(ctx, last_time + FACTOR2 * interval);
        return Ok(());
    }

    let mut transition_time = last_time + interval;
    if transition_time < ctx.time {
        transition_time = ctx.time;
    }

    let next_state = 1 - last_state;
    oscillator_set_state(ctx, OSC_LAST_TIME, transition_time);
    oscillator_set_int_state(ctx, OSC_LAST_STATE, next_state);
    ctx.set_output_digital(
        "out",
        state_to_value(next_state),
        (transition_time - ctx.time).max(0.0),
    );

    let next_interval = if next_state == 0 {
        period * (1.0 - duty_cycle)
    } else {
        period * duty_cycle
    };
    request_future_breakpoint(ctx, transition_time + FACTOR2 * next_interval);
    Ok(())
}

impl CodeModel for NumericallyControlledOscillator {
    fn name(&self) -> &str {
        "nco"
    }

    fn description(&self) -> &str {
        "MIDI numerically controlled oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        nco_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        nco_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        nco_param_min(ctx, "delay", 1.0e-9, 1.0e-15)?;
        nco_param_min(ctx, "mult_factor", 1.0, 1.0e-9)?;
        ctx.allocate_states(1);
        ctx.allocate_int_states(1);
        ctx.set_initial_state(NCO_NEXT_TIME, 0.0);
        ctx.set_int_state(NCO_OUTPUT_STATE, 0);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let half_period = nco_half_period(ctx)?;
        if !ctx.is_transient() || ctx.time == 0.0 {
            let output_state = ctx.int_state(NCO_OUTPUT_STATE);
            nco_set_state(ctx, half_period, output_state);
            request_future_breakpoint(ctx, half_period);
            ctx.set_output_digital("out", state_to_value(output_state), 0.0);
            return Ok(());
        }

        let next_time = ctx.state(NCO_NEXT_TIME);
        if ctx.time + TIME_EPSILON < next_time {
            request_future_breakpoint(ctx, next_time);
            return Ok(());
        }

        let output_state = 1 - ctx.int_state(NCO_OUTPUT_STATE);
        let following_time = ctx.time + half_period;
        nco_set_state(ctx, following_time, output_state);
        request_future_breakpoint(ctx, following_time);
        ctx.set_output_digital(
            "out",
            state_to_value(output_state),
            nco_param_min(ctx, "delay", 1.0e-9, 1.0e-15)?,
        );
        Ok(())
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        Ok(vec![nco_half_period(ctx)?])
    }
}

impl CodeModel for DigitalOscillator {
    fn name(&self) -> &str {
        "d_osc"
    }

    fn description(&self) -> &str {
        "Controlled digital oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        digital_oscillator_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        d_osc_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let has_table = validate_table_optional(
            ctx,
            "d_osc",
            "cntl_array",
            "freq_array",
            sanitize_d_osc_frequency,
        )?
        .is_some();
        d_osc_duty_cycle(ctx)?;
        scalar_finite_param(ctx, "d_osc", "init_phase")?;
        scalar_finite_param(ctx, "d_osc", "rise_delay")?;
        scalar_finite_param(ctx, "d_osc", "fall_delay")?;
        if has_table {
            ctx.allocate_states(1);
            ctx.allocate_int_states(2);
            ctx.set_initial_state(OSC_LAST_TIME, 0.0);
            ctx.set_int_state(OSC_LAST_STATE, 0);
            ctx.set_int_state(OSC_INITIALIZED, 0);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = validate_table_optional(
            ctx,
            "d_osc",
            "cntl_array",
            "freq_array",
            sanitize_d_osc_frequency,
        )?
        else {
            return Ok(());
        };
        let period = d_osc_period_from_table(&table, ctx.input("cntl_in"));
        let duty_cycle = d_osc_duty_cycle(ctx)?;
        evaluate_digital_oscillator(ctx, period, duty_cycle)
    }
}

impl CodeModel for DigitalPwmOscillator {
    fn name(&self) -> &str {
        "d_pwm"
    }

    fn description(&self) -> &str {
        "Duty-cycle controlled digital oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        digital_oscillator_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        d_pwm_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let has_table = validate_table_optional(
            ctx,
            "d_pwm",
            "cntl_array",
            "dc_array",
            sanitize_d_pwm_duty_cycle,
        )?
        .is_some();
        d_pwm_frequency(ctx)?;
        scalar_finite_param(ctx, "d_pwm", "init_phase")?;
        scalar_finite_param(ctx, "d_pwm", "rise_delay")?;
        scalar_finite_param(ctx, "d_pwm", "fall_delay")?;
        if has_table {
            ctx.allocate_states(1);
            ctx.allocate_int_states(2);
            ctx.set_initial_state(OSC_LAST_TIME, 0.0);
            ctx.set_int_state(OSC_LAST_STATE, 0);
            ctx.set_int_state(OSC_INITIALIZED, 0);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = validate_table_optional(
            ctx,
            "d_pwm",
            "cntl_array",
            "dc_array",
            sanitize_d_pwm_duty_cycle,
        )?
        else {
            return Ok(());
        };
        let frequency = d_pwm_frequency(ctx)?;
        let duty_cycle = d_pwm_duty_cycle_from_table(&table, ctx.input("cntl_in"));
        evaluate_digital_oscillator(ctx, 1.0 / frequency, duty_cycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::AnalysisType;
    use crate::xspice::context::{InputValue, PendingDigitalEvent};

    #[test]
    fn digital_oscillators_ignore_bad_control_tables_like_ngspice() {
        let mut osc = CmContext::new();
        osc.analysis = AnalysisType::Transient;
        osc.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        osc.set_real_vector_param("freq_array", vec![1.0e6, 2.0e6, 3.0e6]);
        osc.init_output("out", PortType::Digital);
        DigitalOscillator
            .init(&mut osc)
            .expect("ngspice d_osc reports malformed control tables but does not fail init");
        DigitalOscillator
            .evaluate(&mut osc)
            .expect("ngspice d_osc returns without fatal error when the table is unavailable");

        let mut pwm = CmContext::new();
        pwm.analysis = AnalysisType::Transient;
        pwm.set_real_vector_param("cntl_array", vec![1.0, 0.0]);
        pwm.set_real_vector_param("dc_array", vec![0.25, 0.75]);
        pwm.set_param("frequency", 1.0e6);
        pwm.init_output("out", PortType::Digital);
        DigitalPwmOscillator
            .init(&mut pwm)
            .expect("ngspice d_pwm reports malformed control tables but does not fail init");
        DigitalPwmOscillator
            .evaluate(&mut pwm)
            .expect("ngspice d_pwm returns without fatal error when the table is unavailable");
    }

    #[test]
    fn d_pwm_table_clamps_entries_before_interpolation_and_result_afterwards() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![-1.0, 1.0]);
        ctx.set_real_vector_param("dc_array", vec![0.0, 1.0]);
        ctx.set_input_analog("cntl_in", 0.0);

        assert!((d_pwm_duty_cycle(&mut ctx).unwrap() - 0.5).abs() < 1.0e-15);
    }

    #[test]
    fn digital_oscillator_control_table_cache_reloads_when_params_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("freq_array", vec![1.0e6, 2.0e6]);

        let first = validate_table(
            &mut ctx,
            "d_osc",
            "cntl_array",
            "freq_array",
            sanitize_d_osc_frequency,
        )
        .expect("control table loads");
        let second = validate_table(
            &mut ctx,
            "d_osc",
            "cntl_array",
            "freq_array",
            sanitize_d_osc_frequency,
        )
        .expect("control table reloads");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged digital oscillator table parameters should reuse the parsed table"
        );
        assert_eq!(first[1].value, 2.0e6);

        ctx.set_real_vector_param("freq_array", vec![1.0e6, 3.0e6]);
        let updated = validate_table(
            &mut ctx,
            "d_osc",
            "cntl_array",
            "freq_array",
            sanitize_d_osc_frequency,
        )
        .expect("updated control table loads");
        assert!(
            !Arc::ptr_eq(&first, &updated),
            "changed digital oscillator table parameters must refresh the parsed table"
        );
        assert_eq!(updated[1].value, 3.0e6);
    }

    fn nco_context_for_bits(bits: [DigitalValue; 7]) -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("delay", 2.0e-12);
        ctx.set_param("mult_factor", 1.0e9);
        ctx.set_input("in", InputValue::DigitalVector(bits.to_vec()));
        ctx.init_output("out", PortType::Digital);
        NumericallyControlledOscillator
            .init(&mut ctx)
            .expect("nco initializes");
        ctx
    }

    fn scalar_event(events: &[PendingDigitalEvent]) -> DigitalValue {
        assert_eq!(
            events.len(),
            1,
            "expected one digital event, got {events:?}"
        );
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].start_index, 0);
        assert_eq!(events[0].values.len(), 1);
        events[0].values[0]
    }

    #[test]
    fn nco_decodes_official_seven_bit_midi_input_order() {
        let mut ctx = nco_context_for_bits([
            DigitalValue::one(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
        ]);

        assert_eq!(nco_note_index(&ctx), 64);
        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("initial nco evaluation");
        let expected = 1.0 / (NCO_BASE_FREQUENCY * 1.0e9 * NCO_SEMITONE_RATIO.powi(64));
        assert!(
            (ctx.state(NCO_NEXT_TIME) - expected).abs() <= 1.0e-18,
            "nco should weight the first input bit as 64; expected {expected:e}, got {:e}",
            ctx.state(NCO_NEXT_TIME)
        );
        assert_eq!(
            scalar_event(&ctx.take_pending_events()),
            DigitalValue::zero()
        );
    }

    #[test]
    fn nco_counts_only_strong_one_inputs_like_official_example() {
        let mut ctx = nco_context_for_bits([
            DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
            DigitalValue::one(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
        ]);

        assert_eq!(nco_note_index(&ctx), 32);
        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("initial nco evaluation");
        let expected = 1.0 / (NCO_BASE_FREQUENCY * 1.0e9 * NCO_SEMITONE_RATIO.powi(32));
        assert!(
            (ctx.state(NCO_NEXT_TIME) - expected).abs() <= 1.0e-18,
            "resistive high must not be counted by the official INPUT_STATE == ONE check"
        );
    }

    #[test]
    fn nco_toggles_on_requested_times_with_output_delay() {
        let mut ctx = nco_context_for_bits([
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
        ]);

        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("initial nco evaluation");
        let first_toggle = ctx.state(NCO_NEXT_TIME);
        let _ = ctx.take_pending_events();

        ctx.time = first_toggle;
        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("first nco toggle");
        let events = ctx.take_pending_events();
        assert_eq!(scalar_event(&events), DigitalValue::one());
        assert_eq!(events[0].delay, 2.0e-12);
        assert_eq!(ctx.int_state(NCO_OUTPUT_STATE), 1);
        assert!((ctx.state(NCO_NEXT_TIME) - 2.0 * first_toggle).abs() <= 1.0e-18);
    }

    #[test]
    fn nco_rollbackable_probe_does_not_commit_timing_state() {
        let mut ctx = nco_context_for_bits([
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
            DigitalValue::zero(),
        ]);

        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("initial nco evaluation");
        let first_toggle = ctx.state(NCO_NEXT_TIME);
        let _ = ctx.take_pending_events();

        ctx.time = first_toggle;
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        NumericallyControlledOscillator
            .evaluate(&mut ctx)
            .expect("rollback nco toggle");
        assert_eq!(ctx.int_state(NCO_OUTPUT_STATE), 0);
        assert_eq!(ctx.state(NCO_NEXT_TIME), first_toggle);
        assert_eq!(
            scalar_event(&ctx.take_pending_events()),
            DigitalValue::one()
        );
    }

    #[test]
    fn d_osc_frequency_table_clamps_entries_before_interpolation_without_clamping_result() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("freq_array", vec![1.0e9, 1.0e-16]);
        ctx.set_input_analog("cntl_in", 2.0);

        let period = d_osc_period(&mut ctx).expect("negative high-side extrapolation is finite");

        assert!(
            (period + 1.0e-9).abs() < 1.0e-18,
            "ngspice clamps d_osc frequency table entries before interpolation but preserves the negative extrapolated result; got period {period:e}"
        );
    }

    #[test]
    fn digital_oscillator_accepts_negative_transition_intervals_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("init_phase", 0.0);
        ctx.allocate_states(1);
        ctx.allocate_int_states(2);

        evaluate_digital_oscillator(&mut ctx, -1.0e-9, 0.25)
            .expect("ngspice digital oscillator event path does not reject negative intervals");
    }

    #[test]
    fn initial_phase_selects_same_initial_state_as_official_model() {
        let mut ctx = CmContext::new();
        ctx.set_param("init_phase", 90.0);
        ctx.allocate_states(1);
        ctx.allocate_int_states(2);

        initialize_timing_state(&mut ctx, 1.0e-9, 0.25);

        assert_eq!(ctx.int_state(OSC_LAST_STATE), 0);
        assert!((ctx.state(OSC_LAST_TIME) + 0.25e-9).abs() < 1.0e-18);
    }

    #[test]
    fn digital_oscillator_rollbackable_probe_does_not_commit_phase_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("init_phase", 0.0);
        ctx.allocate_states(1);
        ctx.allocate_int_states(2);

        evaluate_digital_oscillator(&mut ctx, 1.0e-6, 0.5).expect("initial oscillator evaluation");
        let _ = ctx.take_pending_events();
        let _ = ctx.take_requested_breakpoints();
        assert_eq!(ctx.int_state(OSC_LAST_STATE), 0);
        assert_eq!(ctx.state(OSC_LAST_TIME), 0.0);

        ctx.time = 0.5e-6;
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        evaluate_digital_oscillator(&mut ctx, 1.0e-6, 0.5).expect("rollback oscillator transition");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 0.0 && event.values == vec![DigitalValue::one()]),
            "rollbackable oscillator probe should expose the trial transition, got {events:?}"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable oscillator probes must not leave future transition breakpoints behind"
        );
        assert_eq!(
            ctx.int_state(OSC_LAST_STATE),
            0,
            "rollbackable oscillator probe must not commit last state"
        );
        assert_eq!(
            ctx.state(OSC_LAST_TIME),
            0.0,
            "rollbackable oscillator probe must not commit last transition time"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        evaluate_digital_oscillator(&mut ctx, 1.0e-6, 0.5)
            .expect("direct oscillator transition after probe");
        assert_eq!(ctx.int_state(OSC_LAST_STATE), 1);
        assert!((ctx.state(OSC_LAST_TIME) - 0.5e-6).abs() < 1.0e-18);
    }
}
