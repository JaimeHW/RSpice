//! XSPICE controlled waveform oscillator code models.
//!
//! Implements the official `sine`, `square`, and `triangle` analog oscillator
//! blocks from the public-domain XSPICE code-model catalog.

use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec,
    PortType,
};
use std::sync::{Arc, OnceLock};

const MIN_FREQUENCY: Value = 1.0e-16;
const FREQUENCY_TABLE_RESOURCE: &str = "xspice.waveform.frequency_table";
const PHASE_STATE: usize = 0;
const SQUARE_TIME1_STATE: usize = 1;
const SQUARE_TIME2_STATE: usize = 2;
const SQUARE_TIME3_STATE: usize = 3;
const SQUARE_TIME4_STATE: usize = 4;
const TRIANGLE_TIME1_STATE: usize = 1;
const TRIANGLE_TIME2_STATE: usize = 2;
const TRIANGLE_START_STATE: usize = 3;

#[derive(Debug, Default)]
pub struct SineOscillator;

#[derive(Debug, Default)]
pub struct SquareOscillator;

#[derive(Debug, Default)]
pub struct TriangleOscillator;

#[derive(Debug, Clone, Copy)]
struct ControlPoint {
    control: Value,
    frequency: Value,
}

#[derive(Debug, Clone)]
struct FrequencyTableData {
    points: Vec<ControlPoint>,
    strictly_increasing_control: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct FrequencyTableSignature {
    controls: Vec<Value>,
    frequencies: Vec<Value>,
}

#[derive(Debug, Clone)]
struct FrequencyTableResource {
    signature: FrequencyTableSignature,
    result: CmResult<Option<Arc<FrequencyTableData>>>,
}

fn invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn waveform_ports() -> &'static [PortSpec] {
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
                    PortType::VoltageName,
                ],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Frequency-control input".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::Out,
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
                description: "Waveform output".to_string(),
            },
        ]
    })
}

fn oscillator_base_params() -> Vec<ParamSpec> {
    vec![
        ParamSpec::real_vector("cntl_array", vec![0.0, 1.0])
            .with_vector_min_len(2)
            .with_description("Control-input lookup points"),
        ParamSpec::real_vector("freq_array", vec![1.0e3, 2.0e3])
            .with_vector_min_len(2)
            .with_description("Frequency lookup points"),
        ParamSpec::real("out_low", -1.0).with_description("Low output level"),
        ParamSpec::real("out_high", 1.0).with_description("High output level"),
    ]
}

fn sine_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(oscillator_base_params)
}

fn square_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        let mut params = oscillator_base_params();
        params.push(
            ParamSpec::real("duty_cycle", 0.5)
                .with_description("Fraction of each cycle spent high, clamped to official limits"),
        );
        params.push(
            ParamSpec::real("rise_time", 1.0e-9).with_description("Low-to-high transition time"),
        );
        params.push(
            ParamSpec::real("fall_time", 1.0e-9).with_description("High-to-low transition time"),
        );
        params
    })
}

fn triangle_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        let mut params = oscillator_base_params();
        params.push(
            ParamSpec::real("duty_cycle", 0.5).with_description(
                "Fraction of each cycle spent rising, clamped to official limits",
            ),
        );
        params
    })
}

fn controlled_frequency_table(ctx: &CmContext) -> CmResult<FrequencyTableData> {
    controlled_frequency_table_optional_uncached(ctx)?
        .map(frequency_table_data)
        .ok_or_else(|| {
            invalid_param(
                "cntl_array/freq_array",
                "cntl_array length must match freq_array length",
            )
        })
}

fn frequency_table_signature(ctx: &CmContext) -> FrequencyTableSignature {
    FrequencyTableSignature {
        controls: ctx.real_vector_param("cntl_array").unwrap_or(&[]).to_vec(),
        frequencies: ctx.real_vector_param("freq_array").unwrap_or(&[]).to_vec(),
    }
}

fn frequency_table_signature_matches(ctx: &CmContext, signature: &FrequencyTableSignature) -> bool {
    ctx.real_vector_param("cntl_array").unwrap_or(&[]) == signature.controls.as_slice()
        && ctx.real_vector_param("freq_array").unwrap_or(&[]) == signature.frequencies.as_slice()
}

fn controlled_frequency_table_optional_uncached(
    ctx: &CmContext,
) -> CmResult<Option<Vec<ControlPoint>>> {
    let controls = ctx
        .real_vector_param("cntl_array")
        .ok_or_else(|| CmError::MissingParameter("cntl_array".to_string()))?;
    let frequencies = ctx
        .real_vector_param("freq_array")
        .ok_or_else(|| CmError::MissingParameter("freq_array".to_string()))?;

    if controls.len() < 2 || frequencies.len() < 2 {
        return Err(invalid_param(
            "cntl_array/freq_array",
            format!(
                "cntl_array and freq_array require at least 2 points, got {}/{}",
                controls.len(),
                frequencies.len()
            ),
        ));
    }
    if controls.len() != frequencies.len() {
        return Ok(None);
    }

    let mut table: Vec<ControlPoint> = Vec::with_capacity(controls.len());
    for (idx, (&control, &frequency)) in controls.iter().zip(frequencies).enumerate() {
        if !control.is_finite() {
            return Err(invalid_param(
                "cntl_array",
                format!("point {idx} must be finite, got {control}"),
            ));
        }
        if !frequency.is_finite() {
            return Err(invalid_param(
                "freq_array",
                format!("point {idx} must be finite, got {frequency}"),
            ));
        }
        table.push(ControlPoint {
            control,
            frequency: frequency.max(MIN_FREQUENCY),
        });
    }

    Ok(Some(table))
}

fn frequency_table_data(points: Vec<ControlPoint>) -> FrequencyTableData {
    let strictly_increasing_control = points
        .windows(2)
        .all(|pair| pair[0].control < pair[1].control);
    FrequencyTableData {
        points,
        strictly_increasing_control,
    }
}

fn controlled_frequency_table_optional(
    ctx: &mut CmContext,
) -> CmResult<Option<Arc<FrequencyTableData>>> {
    if let Some(resource) = ctx.resource::<FrequencyTableResource>(FREQUENCY_TABLE_RESOURCE)
        && frequency_table_signature_matches(ctx, &resource.signature)
    {
        return resource.result.clone();
    }

    let signature = frequency_table_signature(ctx);
    let result = controlled_frequency_table_optional_uncached(ctx)
        .map(|table| table.map(frequency_table_data).map(Arc::new));
    ctx.set_resource(
        FREQUENCY_TABLE_RESOURCE,
        Arc::new(FrequencyTableResource {
            signature,
            result: result.clone(),
        }),
    );
    result
}

fn interpolate_frequency_linear_scan(table: &[ControlPoint], control: Value) -> Value {
    let first = table[0];
    let last = table[table.len() - 1];

    if control <= first.control {
        let raw = linear_interpolate(table[0], table[1], control);
        // ngspice clamps only the low-control extrapolation path here. The
        // high-control path is allowed to extrapolate negative frequency.
        raw.max(MIN_FREQUENCY)
    } else if control >= last.control {
        linear_interpolate(table[table.len() - 2], last, control)
    } else {
        let mut frequency = None;
        for window in table.windows(2) {
            let left = window[0];
            let right = window[1];
            if control >= left.control && control < right.control {
                frequency = Some(linear_interpolate(left, right, control));
            }
        }
        frequency.unwrap_or(last.frequency)
    }
}

fn interpolate_frequency(table: &FrequencyTableData, control: Value) -> Value {
    let points = table.points.as_slice();
    if !table.strictly_increasing_control {
        return interpolate_frequency_linear_scan(points, control);
    }

    let first = points[0];
    let last = points[points.len() - 1];
    if control <= first.control {
        let raw = linear_interpolate(points[0], points[1], control);
        return raw.max(MIN_FREQUENCY);
    }
    if control >= last.control {
        return linear_interpolate(points[points.len() - 2], last, control);
    }

    let upper_index = points.partition_point(|point| point.control <= control);
    linear_interpolate(points[upper_index - 1], points[upper_index], control)
}

fn linear_interpolate(left: ControlPoint, right: ControlPoint, control: Value) -> Value {
    let span = right.control - left.control;
    if span.abs() <= Value::EPSILON {
        return left.frequency;
    }
    let alpha = (control - left.control) / span;
    left.frequency + alpha * (right.frequency - left.frequency)
}

fn output_levels(ctx: &CmContext) -> CmResult<(Value, Value)> {
    let low = ctx.param("out_low");
    let high = ctx.param("out_high");
    if !low.is_finite() {
        return Err(invalid_param(
            "out_low",
            format!("value must be finite, got {low}"),
        ));
    }
    if !high.is_finite() {
        return Err(invalid_param(
            "out_high",
            format!("value must be finite, got {high}"),
        ));
    }
    Ok((low, high))
}

fn duty_cycle(ctx: &CmContext) -> CmResult<Value> {
    let duty = ctx.param("duty_cycle");
    if !duty.is_finite() {
        return Err(invalid_param(
            "duty_cycle",
            format!("value must be finite, got {duty}"),
        ));
    }
    Ok(duty.clamp(1.0e-6, 0.999_999))
}

fn finite_time(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(invalid_param(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn waveform_commits_state(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

fn waveform_set_state(ctx: &mut CmContext, index: usize, value: Value) {
    if waveform_commits_state(ctx) {
        ctx.set_state(index, value);
    }
}

fn transient_phase(ctx: &mut CmContext, table: &FrequencyTableData) -> Value {
    let frequency = interpolate_frequency(table, ctx.input("cntl_in"));
    let dt = (ctx.time - ctx.time_prev).max(0.0);
    let phase = ctx.state_prev(PHASE_STATE) + frequency * dt;
    waveform_set_state(ctx, PHASE_STATE, phase);
    phase
}

fn phase_fraction(phase: Value) -> Value {
    phase - phase.floor()
}

fn c_truncated_fraction(phase: Value) -> Value {
    phase - phase.trunc()
}

fn request_absolute_breakpoint(ctx: &mut CmContext, time: Value) {
    if waveform_commits_state(ctx) && time.is_finite() && time >= ctx.time {
        ctx.request_breakpoint(time);
    }
}

fn request_next_phase_breakpoint(
    ctx: &mut CmContext,
    phase: Value,
    frequency: Value,
    target: Value,
) {
    if !frequency.is_finite() || frequency <= 0.0 || !target.is_finite() || target < 0.0 {
        return;
    }

    let mut target_phase = phase.floor() + target;
    while target_phase <= phase + 1.0e-15 {
        target_phase += 1.0;
    }
    let time = ctx.time + (target_phase - phase) / frequency;
    if waveform_commits_state(ctx) {
        ctx.request_breakpoint(time);
    }
}

fn initialize_oscillator(ctx: &mut CmContext) -> CmResult<()> {
    controlled_frequency_table_optional(ctx)?;
    output_levels(ctx)?;
    ctx.allocate_states(1);
    ctx.set_initial_state(PHASE_STATE, 0.0);
    Ok(())
}

fn initialize_square_oscillator(ctx: &mut CmContext) -> CmResult<()> {
    controlled_frequency_table_optional(ctx)?;
    output_levels(ctx)?;
    ctx.allocate_states(5);
    ctx.set_initial_state(PHASE_STATE, 0.0);
    ctx.set_initial_state(SQUARE_TIME1_STATE, -1.0);
    ctx.set_initial_state(SQUARE_TIME2_STATE, -1.0);
    ctx.set_initial_state(SQUARE_TIME3_STATE, -1.0);
    ctx.set_initial_state(SQUARE_TIME4_STATE, -1.0);
    Ok(())
}

fn initialize_triangle_oscillator(ctx: &mut CmContext) -> CmResult<()> {
    controlled_frequency_table_optional(ctx)?;
    output_levels(ctx)?;
    ctx.allocate_states(4);
    ctx.set_initial_state(PHASE_STATE, 0.0);
    ctx.set_initial_state(TRIANGLE_TIME1_STATE, -1.0);
    ctx.set_initial_state(TRIANGLE_TIME2_STATE, -1.0);
    ctx.set_initial_state(TRIANGLE_START_STATE, 0.0);
    Ok(())
}

impl CodeModel for SineOscillator {
    fn name(&self) -> &str {
        "sine"
    }

    fn description(&self) -> &str {
        "Controlled sine wave oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        waveform_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        sine_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        initialize_oscillator(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = controlled_frequency_table_optional(ctx)? else {
            return Ok(());
        };
        let (low, high) = output_levels(ctx)?;
        let center = 0.5 * (high + low);
        if ctx.is_dc() {
            waveform_set_state(ctx, PHASE_STATE, 0.0);
            ctx.set_output_with_partial("out", center, 0.0);
            return Ok(());
        }
        if ctx.is_ac() {
            ctx.set_output_with_partial("out", 0.0, 0.0);
            return Ok(());
        }

        let phase = transient_phase(ctx, &table);
        let peak = 0.5 * (high - low);
        let output = center + peak * (std::f64::consts::TAU * phase).sin();
        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}

impl CodeModel for SquareOscillator {
    fn name(&self) -> &str {
        "square"
    }

    fn description(&self) -> &str {
        "Controlled square wave oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        waveform_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        square_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        initialize_square_oscillator(ctx)?;
        duty_cycle(ctx)?;
        finite_time(ctx, "rise_time")?;
        finite_time(ctx, "fall_time")?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = controlled_frequency_table_optional(ctx)? else {
            return Ok(());
        };
        let (low, high) = output_levels(ctx)?;
        if ctx.is_dc() {
            waveform_set_state(ctx, PHASE_STATE, 0.0);
            waveform_set_state(ctx, SQUARE_TIME1_STATE, -1.0);
            waveform_set_state(ctx, SQUARE_TIME2_STATE, -1.0);
            waveform_set_state(ctx, SQUARE_TIME3_STATE, -1.0);
            waveform_set_state(ctx, SQUARE_TIME4_STATE, -1.0);
            ctx.set_output_with_partial("out", low, 0.0);
            return Ok(());
        }
        if ctx.is_ac() {
            ctx.set_output_with_partial("out", 0.0, 0.0);
            return Ok(());
        }

        let frequency = interpolate_frequency(&table, ctx.input("cntl_in"));
        let dt = (ctx.time - ctx.time_prev).max(0.0);
        let phase = ctx.state_prev(PHASE_STATE) + frequency * dt;
        waveform_set_state(ctx, PHASE_STATE, phase);

        let duty = duty_cycle(ctx)?;
        let rise_time = finite_time(ctx, "rise_time")?;
        let fall_time = finite_time(ctx, "fall_time")?;
        let amplitude = high - low;

        let output = if frequency < 0.0 {
            let mut dphase = c_truncated_fraction(ctx.state_prev(PHASE_STATE));
            if dphase > 1.0 - duty {
                dphase -= 1.0;
            }

            let rise_start_time = ctx.time_prev + ((1.0 - duty) - dphase) / frequency;
            let rise_end_time = rise_start_time + rise_time;
            let fall_start_time = ctx.time_prev + (1.0 - dphase) / frequency;
            let fall_end_time = fall_start_time + fall_time;

            if rise_start_time <= ctx.time && ctx.time <= rise_end_time {
                low + ((ctx.time - rise_start_time) / (rise_end_time - rise_start_time)) * amplitude
            } else if rise_end_time <= ctx.time && ctx.time <= fall_start_time {
                high
            } else if fall_start_time <= ctx.time && ctx.time <= fall_end_time {
                high + ((ctx.time - fall_start_time) / (fall_end_time - fall_start_time))
                    * (low - high)
            } else {
                low
            }
        } else {
            let mut time1 = ctx.state_prev(SQUARE_TIME1_STATE);
            let mut time2 = ctx.state_prev(SQUARE_TIME2_STATE);
            let mut time3 = ctx.state_prev(SQUARE_TIME3_STATE);
            let mut time4 = ctx.state_prev(SQUARE_TIME4_STATE);
            let mut dphase = c_truncated_fraction(ctx.state_prev(PHASE_STATE));

            let value = if time1 <= ctx.time && ctx.time <= time2 {
                time3 = ctx.time_prev + (1.0 - dphase) / frequency;
                time4 = time3 + fall_time;

                if ctx.time < time2 {
                    request_absolute_breakpoint(ctx, time2);
                }
                request_absolute_breakpoint(ctx, time3);
                request_absolute_breakpoint(ctx, time4);

                low + ((ctx.time - time1) / (time2 - time1)) * amplitude
            } else if time2 <= ctx.time && ctx.time <= time3 {
                time3 = ctx.time_prev + (1.0 - dphase) / frequency;
                time4 = time3 + fall_time;

                if ctx.time < time3 {
                    request_absolute_breakpoint(ctx, time3);
                }
                request_absolute_breakpoint(ctx, time4);

                high
            } else if time3 <= ctx.time && ctx.time <= time4 {
                if dphase > 1.0 - duty {
                    dphase -= 1.0;
                }

                time1 = ctx.time_prev + ((1.0 - duty) - dphase) / frequency;
                time2 = time1 + rise_time;

                if ctx.time < time4 {
                    request_absolute_breakpoint(ctx, time4);
                }
                request_absolute_breakpoint(ctx, time1);
                request_absolute_breakpoint(ctx, time2);

                high + ((ctx.time - time3) / (time4 - time3)) * (low - high)
            } else {
                if dphase > 1.0 - duty {
                    dphase -= 1.0;
                }

                time1 = ctx.time_prev + ((1.0 - duty) - dphase) / frequency;
                time2 = time1 + rise_time;

                if ctx.time < time1 || ctx.time_prev == 0.0 {
                    request_absolute_breakpoint(ctx, time1);
                }
                request_absolute_breakpoint(ctx, time2);

                low
            };

            waveform_set_state(ctx, SQUARE_TIME1_STATE, time1);
            waveform_set_state(ctx, SQUARE_TIME2_STATE, time2);
            waveform_set_state(ctx, SQUARE_TIME3_STATE, time3);
            waveform_set_state(ctx, SQUARE_TIME4_STATE, time4);
            value
        };

        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}

impl CodeModel for TriangleOscillator {
    fn name(&self) -> &str {
        "triangle"
    }

    fn description(&self) -> &str {
        "Controlled triangle wave oscillator"
    }

    fn ports(&self) -> &[PortSpec] {
        waveform_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        triangle_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        initialize_triangle_oscillator(ctx)?;
        duty_cycle(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = controlled_frequency_table_optional(ctx)? else {
            return Ok(());
        };
        let (low, high) = output_levels(ctx)?;
        if ctx.is_dc() {
            waveform_set_state(ctx, PHASE_STATE, 0.0);
            waveform_set_state(ctx, TRIANGLE_TIME1_STATE, -1.0);
            waveform_set_state(ctx, TRIANGLE_TIME2_STATE, -1.0);
            waveform_set_state(ctx, TRIANGLE_START_STATE, 0.0);
            ctx.set_output_with_partial("out", low, 0.0);
            return Ok(());
        }
        if ctx.is_ac() {
            ctx.set_output_with_partial("out", 0.0, 0.0);
            return Ok(());
        }

        let frequency = interpolate_frequency(&table, ctx.input("cntl_in"));
        let dt = (ctx.time - ctx.time_prev).max(0.0);
        let phase_prev = ctx.state_prev(PHASE_STATE);
        let phase = phase_prev + frequency * dt;
        waveform_set_state(ctx, PHASE_STATE, phase);
        let duty = duty_cycle(ctx)?;
        let amplitude = high - low;

        let output = if !frequency.is_finite() {
            low
        } else if frequency >= 0.0 {
            let frac = phase_fraction(phase);
            request_next_phase_breakpoint(ctx, phase, frequency, duty);
            request_next_phase_breakpoint(ctx, phase, frequency, 1.0);
            if frac <= duty {
                low + (frac / duty) * amplitude
            } else {
                high + ((frac - duty) / (1.0 - duty)) * (low - high)
            }
        } else {
            let mut time1 = ctx.state_prev(TRIANGLE_TIME1_STATE);
            let mut time2 = ctx.state_prev(TRIANGLE_TIME2_STATE);
            let mut t_start = ctx.state_prev(TRIANGLE_START_STATE);
            let mut dphase = c_truncated_fraction(phase_prev);

            let value = if time1 <= ctx.time && ctx.time <= time2 {
                time2 = ctx.time_prev + (1.0 - dphase) / frequency;

                if ctx.time < time2 {
                    request_absolute_breakpoint(ctx, time2);
                }

                t_start = time2;
                high - ((ctx.time - time1) / (time2 - time1)) * amplitude
            } else {
                if dphase > duty {
                    dphase -= 1.0;
                }

                time1 = ctx.time_prev + (duty - dphase) / frequency;
                time2 = ctx.time_prev + (1.0 - dphase) / frequency;

                if ctx.time < time1 || ctx.time_prev == 0.0 {
                    request_absolute_breakpoint(ctx, time1);
                }
                request_absolute_breakpoint(ctx, time2);

                low + ((ctx.time - t_start) / (time1 - t_start)) * amplitude
            };

            waveform_set_state(ctx, TRIANGLE_TIME1_STATE, time1);
            waveform_set_state(ctx, TRIANGLE_TIME2_STATE, time2);
            waveform_set_state(ctx, TRIANGLE_START_STATE, t_start);
            value
        };

        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::{AnalogValue, InputValue};
    use crate::xspice::{AnalysisType, CallType};

    fn oscillator_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("freq_array", vec![1.0e9, 1.0e9]);
        ctx.set_param("out_low", -1.0);
        ctx.set_param("out_high", 1.0);
        ctx.set_input("cntl_in", InputValue::Analog(AnalogValue::new(0.0)));
        ctx.init_output("out", PortType::Voltage);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time_prev = 0.0;
        ctx.time = 0.25e-9;
        ctx.timestep = 0.25e-9;
        ctx
    }

    fn mismatched_table_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("freq_array", vec![1.0e3, 2.0e3, 3.0e3]);
        ctx.init_output("out", PortType::Voltage);
        ctx
    }

    #[test]
    fn waveform_oscillators_ignore_mismatched_frequency_tables_like_ngspice() {
        let mut sine = mismatched_table_context();
        SineOscillator
            .init(&mut sine)
            .expect("ngspice sine reports mismatched tables but does not fail initialization");
        SineOscillator
            .evaluate(&mut sine)
            .expect("ngspice sine returns without fatal error on mismatched tables");
        assert_eq!(sine.output("out"), 0.0);

        let mut square = mismatched_table_context();
        SquareOscillator
            .init(&mut square)
            .expect("ngspice square reports mismatched tables but does not fail initialization");
        SquareOscillator
            .evaluate(&mut square)
            .expect("ngspice square returns without fatal error on mismatched tables");
        assert_eq!(square.output("out"), 0.0);

        let mut triangle = mismatched_table_context();
        TriangleOscillator
            .init(&mut triangle)
            .expect("ngspice triangle reports mismatched tables but does not fail initialization");
        TriangleOscillator
            .evaluate(&mut triangle)
            .expect("ngspice triangle returns without fatal error on mismatched tables");
        assert_eq!(triangle.output("out"), 0.0);
    }

    #[test]
    fn waveform_frequency_table_cache_reloads_when_params_change() {
        let mut ctx = oscillator_context();
        SineOscillator.init(&mut ctx).expect("sine initializes");

        let first = controlled_frequency_table_optional(&mut ctx)
            .expect("cached frequency table loads")
            .expect("frequency table is present");
        let second = controlled_frequency_table_optional(&mut ctx)
            .expect("cached frequency table reloads")
            .expect("frequency table is present");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged waveform frequency parameters should reuse the parsed table"
        );
        assert_eq!(interpolate_frequency(&first, 0.0), 1.0e9);

        ctx.set_real_vector_param("freq_array", vec![2.0e9, 2.0e9]);
        let updated = controlled_frequency_table_optional(&mut ctx)
            .expect("updated frequency table loads")
            .expect("frequency table is present");
        assert!(
            !Arc::ptr_eq(&first, &updated),
            "changed waveform frequency parameters must refresh the parsed table"
        );
        assert_eq!(interpolate_frequency(&updated, 0.0), 2.0e9);
    }

    #[test]
    fn sine_rollbackable_probe_does_not_commit_phase_state() {
        let mut ctx = oscillator_context();
        SineOscillator.init(&mut ctx).expect("sine initializes");

        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SineOscillator
            .evaluate(&mut ctx)
            .expect("probes sine oscillator");
        assert!(
            (ctx.output("out") - 1.0).abs() <= 1.0e-15,
            "rollbackable sine probe should still compute trial output"
        );
        assert_eq!(
            ctx.state(PHASE_STATE),
            0.0,
            "rollbackable sine probe must not commit phase"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SineOscillator
            .evaluate(&mut ctx)
            .expect("commits sine oscillator");
        assert!((ctx.state(PHASE_STATE) - 0.25).abs() <= 1.0e-15);
    }

    #[test]
    fn square_rollbackable_probe_does_not_commit_edge_timing_or_breakpoints() {
        let mut ctx = oscillator_context();
        ctx.set_param("duty_cycle", 0.5);
        ctx.set_param("rise_time", 0.1e-9);
        ctx.set_param("fall_time", 0.1e-9);
        SquareOscillator.init(&mut ctx).expect("square initializes");

        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SquareOscillator
            .evaluate(&mut ctx)
            .expect("probes square oscillator");
        assert!(
            (ctx.output("out") + 1.0).abs() <= 1.0e-15,
            "rollbackable square probe should still compute trial output"
        );
        assert_eq!(
            ctx.state(PHASE_STATE),
            0.0,
            "rollbackable square probe must not commit phase"
        );
        assert_eq!(
            ctx.state(SQUARE_TIME1_STATE),
            -1.0,
            "rollbackable square probe must not commit edge timing"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable square probe must not request edge breakpoints"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SquareOscillator
            .evaluate(&mut ctx)
            .expect("commits square oscillator");
        assert!((ctx.state(PHASE_STATE) - 0.25).abs() <= 1.0e-15);
        assert!((ctx.state(SQUARE_TIME1_STATE) - 0.5e-9).abs() <= 1.0e-21);
        assert!((ctx.state(SQUARE_TIME2_STATE) - 0.6e-9).abs() <= 1.0e-21);
        let breakpoints = ctx.take_requested_breakpoints();
        assert_eq!(breakpoints.len(), 2);
        assert!((breakpoints[0] - 0.5e-9).abs() <= 1.0e-21);
        assert!((breakpoints[1] - 0.6e-9).abs() <= 1.0e-21);
    }

    #[test]
    fn interpolate_frequency_uses_last_matching_segment_like_ngspice() {
        let table = frequency_table_data(vec![
            ControlPoint {
                control: 0.0,
                frequency: 0.0,
            },
            ControlPoint {
                control: 1.0,
                frequency: 100.0,
            },
            ControlPoint {
                control: 0.5,
                frequency: 50.0,
            },
            ControlPoint {
                control: 2.0,
                frequency: 300.0,
            },
        ]);

        let frequency = interpolate_frequency(&table, 0.75);

        assert!(
            (frequency - 91.666_666_666_666_66).abs() < 1.0e-12,
            "ngspice scans every ascending in-range segment and keeps the later match; got {frequency}"
        );
    }

    #[test]
    fn interpolate_frequency_uses_monotonic_brackets() {
        let table = frequency_table_data(vec![
            ControlPoint {
                control: 0.0,
                frequency: 100.0,
            },
            ControlPoint {
                control: 1.0,
                frequency: 200.0,
            },
            ControlPoint {
                control: 3.0,
                frequency: 600.0,
            },
        ]);

        assert!(table.strictly_increasing_control);
        assert!(
            (interpolate_frequency(&table, 2.0) - 400.0).abs() < 1.0e-12,
            "strictly increasing controls should interpolate from the binary-search bracket"
        );
        assert_eq!(
            interpolate_frequency(&table, 1.0),
            200.0,
            "exact interior controls should return the matching row"
        );
    }

    #[test]
    fn interpolate_frequency_clamps_entries_but_preserves_high_extrapolation_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("freq_array", vec![-1.0e3, 1.0e3]);

        let table = controlled_frequency_table(&ctx).expect("frequency table parses");

        assert!(
            (interpolate_frequency(&table, 0.5) - 500.0).abs() < 1.0e-9,
            "freq_array entries are constrained by the official parameter limits before interpolation"
        );

        let mut high_ctx = CmContext::new();
        high_ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        high_ctx.set_real_vector_param("freq_array", vec![1.0e9, MIN_FREQUENCY]);
        let high_table = controlled_frequency_table(&high_ctx).expect("frequency table parses");

        let high_extrapolated = interpolate_frequency(&high_table, 2.0);
        assert!(
            high_extrapolated < -9.9e8,
            "high-control extrapolation must preserve negative frequency"
        );
        assert_eq!(
            interpolate_frequency(&table, -1.0),
            MIN_FREQUENCY,
            "low-control nonpositive extrapolation clamps to ngspice's minimum frequency"
        );
    }
}
