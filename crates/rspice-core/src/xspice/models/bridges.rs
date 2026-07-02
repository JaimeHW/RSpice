//! A/D, D/A, and bidirectional bridge Code Models

use super::analog::smooth_discontinuity;
use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue,
    EvaluationPhase, ParamSpec, PortDirection, PortSpec, PortType,
};

const OFFICIAL_BRIDGE_TIMING_MIN: Value = 1.0e-12;
const OFFICIAL_BRIDGE_RESISTANCE_MIN: Value = 1.0e-6;
const OFFICIAL_BRIDGE_CONTROL_MIN: i64 = 0;
const OFFICIAL_BRIDGE_CONTROL_MAX: i64 = 2;

fn official_bridge_timing(value: Value) -> Value {
    value.max(OFFICIAL_BRIDGE_TIMING_MIN)
}

fn official_bridge_resistance(value: Value) -> Value {
    value.max(OFFICIAL_BRIDGE_RESISTANCE_MIN)
}

fn official_bridge_control(value: Value) -> i64 {
    let value = value.round();
    if value.is_finite() {
        (value as i64).clamp(OFFICIAL_BRIDGE_CONTROL_MIN, OFFICIAL_BRIDGE_CONTROL_MAX)
    } else {
        OFFICIAL_BRIDGE_CONTROL_MIN
    }
}

/// Analog to digital converter bridge
#[derive(Debug, Default)]
pub struct AdcBridge;

impl CodeModel for AdcBridge {
    fn name(&self) -> &str {
        "adc_bridge"
    }

    fn description(&self) -> &str {
        "Analog to digital converter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec {
                    name: "in".to_string(),
                    direction: PortDirection::In,
                    default_type: PortType::Voltage,
                    allowed_types: vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::Current,
                        PortType::DifferentialCurrent,
                        PortType::VoltageName,
                    ],
                    is_vector: true,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Analog input vector".to_string(),
                },
                PortSpec::vector_output("out", PortType::Digital)
                    .with_description("Digital output vector"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_low", 0.1),
                ParamSpec::real("in_high", 0.9),
                ParamSpec::real("rise_delay", 1e-9)
                    .with_description("Rise delay, clamped to official lower limit"),
                ParamSpec::real("fall_delay", 1e-9)
                    .with_description("Fall delay, clamped to official lower limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, "adc_bridge")?;
        ctx.allocate_int_states(width);
        for index in 0..width {
            ctx.set_int_state(index, -1);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, "adc_bridge")?;
        let in_low = ctx.param("in_low");
        let in_high = ctx.param("in_high");
        let rise_delay = official_bridge_timing(ctx.param("rise_delay"));
        let fall_delay = official_bridge_timing(ctx.param("fall_delay"));
        let commit_outputs = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;

        for index in 0..width {
            let v_in = analog_vector_input_value(ctx, "in", index);
            let prev = ctx.int_state(index);

            let new_state = if v_in <= in_low {
                0
            } else if v_in >= in_high {
                1
            } else {
                -1
            };

            if commit_outputs && new_state != prev {
                let val = match new_state {
                    0 => DigitalValue::zero(),
                    1 => DigitalValue::one(),
                    _ => DigitalValue::unknown(),
                };
                let delay = if ctx.time == 0.0 {
                    0.0
                } else {
                    match new_state {
                        1 => rise_delay,
                        -1 if prev == 0 => rise_delay,
                        _ => fall_delay,
                    }
                };
                ctx.set_output_digital_vector_element("out", index, val, delay);
            }
            if commit_outputs {
                ctx.set_int_state(index, new_state);
            }
        }
        Ok(())
    }
}

/// Digital to analog converter bridge
#[derive(Debug, Default)]
pub struct DacBridge;

/// Bidirectional digital/analog bridge.
#[derive(Debug, Default)]
pub struct BidiBridge;

const BIDI_STATE_SVOC_BASE: usize = 0;
const BIDI_STATE_CURRENT_BASE_OFFSET: usize = 1;
const BIDI_STATE_DRIVE_STATE_BASE_OFFSET: usize = 2;
const BIDI_STATE_DRIVE_STRENGTH_BASE_OFFSET: usize = 3;
const BIDI_INT_OUTPUT_STATE_BASE: usize = 0;
const BIDI_INT_OUTPUT_STRENGTH_BASE_OFFSET: usize = 1;
const BIDI_UNINITIALIZED_STATE_CODE: i64 = 99;
const BIDI_OFF_LOW: Value = 0.7;
const BIDI_OFF_HIGH: Value = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BidiDirection {
    Dac,
    Adc,
    Bidirectional,
}

#[derive(Debug, Clone, Copy)]
struct BidiParams {
    direction: i64,
    input_load: Value,
    strength: i64,
    smooth: i64,
    in_low: Value,
    in_high: Value,
    out_low: Value,
    out_high: Value,
    drive_low: Value,
    drive_high: Value,
    r_stl: Value,
    r_sth: Value,
    r_low: Value,
    r_high: Value,
    t_rise: Value,
    t_fall: Value,
    rise_delay: Value,
    fall_delay: Value,
}

#[derive(Debug, Clone, Copy)]
struct BidiAnalogDrive {
    current: Value,
    partial: Value,
    svoc: Value,
}

#[derive(Debug, Clone, Copy)]
struct BidiAnalogSegment {
    drive: DigitalValue,
    interval: Value,
}

fn node_row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

fn stamp_pair_conductance(ctx: &mut CmContext, pair: (usize, usize), conductance: Value) {
    if conductance == 0.0 || !conductance.is_finite() {
        return;
    }
    if let Some(row) = node_row(pair.0) {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let Some(row) = node_row(pair.1) {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let (Some(pos), Some(neg)) = (node_row(pair.0), node_row(pair.1)) {
        ctx.stamp_conductance(pos, neg, -conductance);
        ctx.stamp_conductance(neg, pos, -conductance);
    }
}

fn stamp_pair_current_rhs(ctx: &mut CmContext, pair: (usize, usize), equivalent: Value) {
    if !equivalent.is_finite() {
        return;
    }
    if let Some(row) = node_row(pair.0) {
        ctx.stamp_rhs(row, -equivalent);
    }
    if let Some(row) = node_row(pair.1) {
        ctx.stamp_rhs(row, equivalent);
    }
}

fn digital_state_code(state: DigitalState) -> i64 {
    if state.is_low() {
        0
    } else if state.is_high() {
        1
    } else if state.is_high_z() {
        3
    } else {
        2
    }
}

fn digital_state_from_code(code: i64) -> DigitalState {
    match code {
        0 => DigitalState::Zero,
        1 => DigitalState::One,
        3 => DigitalState::HighZ,
        _ => DigitalState::Unknown,
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

fn digital_strength_from_code(code: i64) -> DigitalStrength {
    match code {
        0 => DigitalStrength::Strong,
        1 => DigitalStrength::Resistive,
        2 => DigitalStrength::HighZ,
        _ => DigitalStrength::Undetermined,
    }
}

fn digital_strength_from_param(code: i64) -> DigitalStrength {
    match code {
        0 => DigitalStrength::Strong,
        1 => DigitalStrength::Resistive,
        2 => DigitalStrength::HighZ,
        _ => DigitalStrength::Undetermined,
    }
}

fn digital_value_with_strength(state: DigitalState, strength: DigitalStrength) -> DigitalValue {
    match strength {
        DigitalStrength::Strong => match state {
            DigitalState::Zero => DigitalValue::zero(),
            DigitalState::One => DigitalValue::one(),
            DigitalState::HighZ => DigitalValue::high_z(),
            _ => DigitalValue::unknown(),
        },
        DigitalStrength::Resistive => match state {
            DigitalState::Zero => DigitalValue::new(DigitalState::ZeroR, strength),
            DigitalState::One => DigitalValue::new(DigitalState::OneR, strength),
            DigitalState::HighZ => DigitalValue::high_z(),
            _ => DigitalValue::new(DigitalState::UnknownR, strength),
        },
        DigitalStrength::HighZ => match state {
            DigitalState::Zero => DigitalValue::new(DigitalState::ZeroZ, strength),
            DigitalState::One => DigitalValue::new(DigitalState::OneZ, strength),
            DigitalState::HighZ => DigitalValue::high_z(),
            _ => DigitalValue::new(DigitalState::UnknownZ, strength),
        },
        DigitalStrength::Undetermined => {
            let state = match state {
                DigitalState::Zero => DigitalState::Zero,
                DigitalState::One => DigitalState::One,
                DigitalState::HighZ => DigitalState::HighZ,
                _ => DigitalState::Unknown,
            };
            DigitalValue::new(state, strength)
        }
    }
}

fn bidi_ports() -> &'static [PortSpec] {
    use std::sync::OnceLock;
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "a".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: true,
                null_allowed: false,
                vector_min_len: Some(1),
                vector_max_len: None,
                description: "Analog in/out".to_string(),
            },
            PortSpec {
                name: "d".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Digital,
                allowed_types: vec![PortType::Digital],
                is_vector: true,
                null_allowed: false,
                vector_min_len: Some(1),
                vector_max_len: None,
                description: "Digital in/out".to_string(),
            },
            PortSpec {
                name: "dir".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Digital,
                allowed_types: vec![PortType::Digital],
                is_vector: true,
                null_allowed: true,
                vector_min_len: None,
                vector_max_len: None,
                description: "Optional direction control".to_string(),
            },
        ]
    })
}

fn bidi_parameters() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::integer("direction", 2)
                .with_description("Forced bridge direction, clamped to official range"),
            ParamSpec::real("input_load", 1.0e-12),
            ParamSpec::integer("strength", 0)
                .with_description("Output strength, clamped to official range"),
            ParamSpec::integer("smooth", 0)
                .with_description("Smoothing level, clamped to official range"),
            ParamSpec::real("in_low", 0.1),
            ParamSpec::real("in_high", 0.9),
            ParamSpec::real("out_low", 0.0),
            ParamSpec::real("out_high", 3.3),
            ParamSpec::real("drive_low", 0.02),
            ParamSpec::real("drive_high", 0.02),
            ParamSpec::real("r_stl", 20.0)
                .with_description("Low taper resistance, clamped to official lower limit"),
            ParamSpec::real("r_sth", 20.0)
                .with_description("High taper resistance, clamped to official lower limit"),
            ParamSpec::real("r_low", 10_000.0)
                .with_description("Drive resistor to ground, clamped to official lower limit"),
            ParamSpec::real("r_high", 10_000.0)
                .with_description("Drive resistor to out_high, clamped to official lower limit"),
            ParamSpec::real("t_rise", 1.0e-9)
                .with_description("Analog rise time, clamped to official lower limit"),
            ParamSpec::real("t_fall", 1.0e-9)
                .with_description("Analog fall time, clamped to official lower limit"),
            ParamSpec::real("rise_delay", 1.0e-9)
                .with_description("Digital rise delay, clamped to official lower limit"),
            ParamSpec::real("fall_delay", 1.0e-9)
                .with_description("Digital fall delay, clamped to official lower limit"),
        ]
    })
}

fn bidi_params(ctx: &CmContext) -> BidiParams {
    BidiParams {
        direction: official_bridge_control(ctx.param("direction")),
        input_load: ctx.param("input_load"),
        strength: official_bridge_control(ctx.param("strength")),
        smooth: official_bridge_control(ctx.param("smooth")),
        in_low: ctx.param("in_low"),
        in_high: ctx.param("in_high"),
        out_low: ctx.param("out_low"),
        out_high: ctx.param("out_high"),
        drive_low: ctx.param("drive_low"),
        drive_high: ctx.param("drive_high"),
        r_stl: official_bridge_resistance(ctx.param("r_stl")),
        r_sth: official_bridge_resistance(ctx.param("r_sth")),
        r_low: official_bridge_resistance(ctx.param("r_low")),
        r_high: official_bridge_resistance(ctx.param("r_high")),
        t_rise: official_bridge_timing(ctx.param("t_rise")),
        t_fall: official_bridge_timing(ctx.param("t_fall")),
        rise_delay: official_bridge_timing(ctx.param("rise_delay")),
        fall_delay: official_bridge_timing(ctx.param("fall_delay")),
    }
}

fn bidi_state_base(width: usize) -> usize {
    BIDI_STATE_CURRENT_BASE_OFFSET * width
}

fn bidi_drive_state_base(width: usize) -> usize {
    BIDI_STATE_DRIVE_STATE_BASE_OFFSET * width
}

fn bidi_drive_strength_base(width: usize) -> usize {
    BIDI_STATE_DRIVE_STRENGTH_BASE_OFFSET * width
}

fn bidi_int_strength_base(width: usize) -> usize {
    BIDI_INT_OUTPUT_STRENGTH_BASE_OFFSET * width
}

fn bidi_direction(params: BidiParams, dir: &[DigitalValue], index: usize) -> BidiDirection {
    match params.direction {
        0 => BidiDirection::Dac,
        1 => BidiDirection::Adc,
        _ => match dir.get(index).and_then(DigitalValue::to_bool) {
            Some(false) => BidiDirection::Dac,
            Some(true) => BidiDirection::Adc,
            None => BidiDirection::Bidirectional,
        },
    }
}

fn bidi_default_effective_direction(
    input: DigitalValue,
    own_state: DigitalState,
    own_strength: DigitalStrength,
) -> BidiDirection {
    if matches!(
        input.strength,
        DigitalStrength::HighZ | DigitalStrength::Undetermined
    ) {
        return BidiDirection::Adc;
    }

    if own_strength == DigitalStrength::HighZ {
        return BidiDirection::Dac;
    }

    if input.state.logic_level() == own_state.logic_level() && input.strength == own_strength {
        BidiDirection::Adc
    } else {
        BidiDirection::Dac
    }
}

fn bidi_adc_state(input: Value, previous: DigitalState, params: BidiParams) -> DigitalState {
    if params.in_high < params.in_low {
        if input > params.in_low {
            DigitalState::One
        } else if input < params.in_high {
            DigitalState::Zero
        } else {
            previous
        }
    } else if input < params.in_low {
        DigitalState::Zero
    } else if input > params.in_high {
        DigitalState::One
    } else {
        DigitalState::Unknown
    }
}

fn bidi_digital_delay(state: DigitalState, params: BidiParams) -> Value {
    if state.is_high() {
        params.rise_delay
    } else if state.is_low() {
        params.fall_delay
    } else {
        params.rise_delay.min(params.fall_delay)
    }
}

fn bidi_unknown_target_svoc(drive: DigitalValue, params: BidiParams) -> Value {
    match drive.strength {
        DigitalStrength::Strong => {
            if params.drive_high > params.drive_low {
                1.0
            } else {
                0.5
            }
        }
        DigitalStrength::Resistive | DigitalStrength::Undetermined => {
            params.r_low / (params.r_low + params.r_high)
        }
        DigitalStrength::HighZ => 0.5,
    }
}

fn bidi_target_svoc(drive: DigitalValue, params: BidiParams) -> Value {
    if drive.state.is_low() {
        0.0
    } else if drive.state.is_high() {
        1.0
    } else {
        bidi_unknown_target_svoc(drive, params)
    }
}

fn bidi_advance_svoc(prev: Value, drive: DigitalValue, dt: Value, params: BidiParams) -> Value {
    if drive.strength == DigitalStrength::HighZ {
        return 0.5;
    }

    let target = bidi_target_svoc(drive, params);
    if !(dt.is_finite() && dt > 0.0) {
        return prev;
    }
    if target > prev {
        (prev + dt / params.t_rise).min(target)
    } else if target < prev {
        (prev - dt / params.t_fall).max(target)
    } else {
        prev
    }
}

fn bidi_previous_drive(ctx: &CmContext, index: usize, width: usize) -> DigitalValue {
    let state_base = bidi_drive_state_base(width);
    let strength_base = bidi_drive_strength_base(width);
    let state = digital_state_from_code(ctx.state_prev(state_base + index).round() as i64);
    let strength = digital_strength_from_code(ctx.state_prev(strength_base + index).round() as i64);
    DigitalValue::new(state, strength)
}

fn bidi_analog_step(ctx: &CmContext) -> Value {
    let step_start = ctx.time_prev;
    let step_end = ctx.time;
    if step_end > step_start {
        step_end - step_start
    } else {
        ctx.timestep.max(0.0)
    }
}

fn bidi_analog_segments(
    ctx: &CmContext,
    index: usize,
    drive: DigitalValue,
    width: usize,
) -> Vec<BidiAnalogSegment> {
    let step = bidi_analog_step(ctx);
    let Some(event_time) = ctx.input_digital_vector_event_time("d", index) else {
        return vec![BidiAnalogSegment {
            drive,
            interval: step,
        }];
    };
    let previous_drive = bidi_previous_drive(ctx, index, width);
    let step_start = ctx.time_prev;
    let step_end = ctx.time;
    let iota = step.abs() * 1.0e-6;

    if step_end - event_time < iota {
        vec![BidiAnalogSegment {
            drive: previous_drive,
            interval: step,
        }]
    } else if event_time - step_start < iota {
        vec![BidiAnalogSegment {
            drive,
            interval: step,
        }]
    } else if event_time > step_start && event_time < step_end {
        vec![
            BidiAnalogSegment {
                drive: previous_drive,
                interval: (event_time - step_start).max(0.0),
            },
            BidiAnalogSegment {
                drive,
                interval: (step_end - event_time).max(0.0),
            },
        ]
    } else {
        vec![BidiAnalogSegment {
            drive,
            interval: step,
        }]
    }
}

fn bidi_current_target(
    voltage: Value,
    drive: DigitalValue,
    svoc: Value,
    params: BidiParams,
) -> (Value, Value, Value) {
    let mut scaled_voc = svoc.clamp(0.0, 1.0);
    if params.smooth > 0 {
        scaled_voc = smooth_discontinuity(scaled_voc, 0.0, 0.0, 1.0, 1.0).0;
    }
    let voc = params.out_low + (params.out_high - params.out_low) * scaled_voc;
    let mut max_high = params.drive_high * (scaled_voc - BIDI_OFF_HIGH) / (1.0 - BIDI_OFF_HIGH);
    if max_high < 0.0 {
        max_high = 0.0;
    }
    if params.smooth > 1 && params.drive_high > 0.0 {
        max_high = smooth_discontinuity(max_high, 0.0, 0.0, params.drive_high, params.drive_high).0;
    }
    let mut max_low = params.drive_low * (BIDI_OFF_LOW - scaled_voc) / BIDI_OFF_LOW;
    if max_low < 0.0 {
        max_low = 0.0;
    }
    if params.smooth > 1 && params.drive_low > 0.0 {
        max_low = smooth_discontinuity(max_low, 0.0, 0.0, params.drive_low, params.drive_low).0;
    }

    let (mut target, mut partial, range) = match drive.strength {
        DigitalStrength::Strong => {
            let g_low = 1.0 / params.r_stl;
            let g_high = 1.0 / params.r_sth;
            let (target, partial) = if drive.state.is_low() {
                ((voltage - voc) * g_low, g_low)
            } else if drive.state.is_high() {
                ((voltage - voc) * g_high, g_high)
            } else {
                let partial = g_low + g_high;
                ((voltage - voc) * partial, partial)
            };
            (target, partial, params.drive_high + params.drive_low)
        }
        DigitalStrength::Resistive | DigitalStrength::Undetermined => {
            let g_low = 1.0 / params.r_low;
            let g_high = 1.0 / params.r_high;
            let (target, partial) = if drive.state.is_low() {
                if voltage < voc {
                    (0.0, 0.0)
                } else {
                    ((voltage - voc) * g_low, g_low)
                }
            } else if drive.state.is_high() {
                if voltage > voc {
                    (0.0, 0.0)
                } else {
                    ((voltage - voc) * g_high, g_high)
                }
            } else if voltage < params.out_low {
                ((voltage - params.out_high) * g_high, g_high)
            } else if voltage >= params.out_high {
                ((voltage - params.out_low) * g_low, g_low)
            } else {
                let partial = g_low + g_high;
                ((voltage - voc) * partial, partial)
            };
            (
                target,
                partial,
                params.out_high / params.r_high + params.out_low / params.r_low,
            )
        }
        DigitalStrength::HighZ => (0.0, 0.0, params.drive_high.max(params.drive_low)),
    };

    if drive.strength == DigitalStrength::Strong {
        if target > max_low {
            target = max_low;
            partial = 0.0;
        } else if target < -max_high {
            target = -max_high;
            partial = 0.0;
        }
    }

    (target, partial, range)
}

fn bidi_advance_current(
    current: Value,
    target: Value,
    interval: Value,
    range: Value,
    params: BidiParams,
) -> Value {
    if !(interval.is_finite() && interval > 0.0 && range.is_finite() && range.abs() > 0.0) {
        return current;
    }

    let delta = target - current;
    let transition_time = (delta
        * if delta > 0.0 {
            params.t_rise
        } else {
            params.t_fall
        }
        / range)
        .abs();
    if transition_time > interval && transition_time.is_finite() {
        current + delta * interval / transition_time
    } else {
        target
    }
}

fn bidi_drive_current(
    ctx: &CmContext,
    index: usize,
    voltage: Value,
    drive: DigitalValue,
    params: BidiParams,
    width: usize,
) -> BidiAnalogDrive {
    if !ctx.is_transient() {
        let svoc = bidi_target_svoc(drive, params);
        let (current, partial, _) = bidi_current_target(voltage, drive, svoc, params);
        return BidiAnalogDrive {
            current,
            partial,
            svoc,
        };
    }

    let step = bidi_analog_step(ctx);
    let mut svoc = ctx.state_prev(BIDI_STATE_SVOC_BASE + index).clamp(0.0, 1.0);
    let mut current = ctx.state_prev(bidi_state_base(width) + index);
    if !current.is_finite() {
        current = 0.0;
    }
    let mut partial = 0.0;
    for segment in bidi_analog_segments(ctx, index, drive, width) {
        svoc = bidi_advance_svoc(svoc, segment.drive, step, params);
        let (target, segment_partial, range) =
            bidi_current_target(voltage, segment.drive, svoc, params);
        partial = segment_partial;
        current = bidi_advance_current(current, target, segment.interval, range, params);
    }

    BidiAnalogDrive {
        current,
        partial,
        svoc,
    }
}

fn dac_bridge_ramp_value(
    time: Value,
    start_time: Value,
    start_value: Value,
    target: Value,
    out_low: Value,
    out_high: Value,
    t_rise: Value,
    t_fall: Value,
) -> Value {
    let elapsed = (time - start_time).max(0.0);
    let span = (out_high - out_low).abs();
    if (target - start_value).abs() < 1e-12 || span <= 1e-12 {
        target
    } else {
        let duration =
            dac_bridge_transition_duration(start_value, target, out_low, out_high, t_rise, t_fall);
        let slope = span / duration;
        if !slope.is_finite() || slope <= 0.0 {
            return target;
        }
        let delta = slope * elapsed;
        if target > start_value {
            (start_value + delta).min(target)
        } else {
            (start_value - delta).max(target)
        }
    }
}

fn dac_bridge_transition_duration(
    start_value: Value,
    target: Value,
    out_low: Value,
    out_high: Value,
    t_rise: Value,
    t_fall: Value,
) -> Value {
    if (target - out_high).abs() <= 1e-12 {
        t_rise
    } else if (target - out_low).abs() <= 1e-12 {
        t_fall
    } else if target > start_value {
        t_rise
    } else {
        t_fall
    }
}

fn dac_bridge_completion_time(
    start_time: Value,
    start_value: Value,
    target: Value,
    out_low: Value,
    out_high: Value,
    t_rise: Value,
    t_fall: Value,
) -> Option<Value> {
    let span = (out_high - out_low).abs();
    if (target - start_value).abs() <= 1e-12 || span <= 1e-12 {
        return None;
    }

    let duration =
        dac_bridge_transition_duration(start_value, target, out_low, out_high, t_rise, t_fall);
    let slope = span / duration;
    (slope > 0.0 && slope.is_finite()).then_some(start_time + (target - start_value).abs() / slope)
}

fn bridge_vector_width(ctx: &CmContext, model_name: &str) -> CmResult<usize> {
    let in_width = ctx.port_width("in");
    let out_width = ctx.port_width("out");
    if in_width == 0 || out_width == 0 {
        return Err(CmError::InvalidPortConnection(format!(
            "{model_name} requires non-empty input and output vectors"
        )));
    }
    if in_width != out_width {
        return Err(CmError::InvalidPortConnection(format!(
            "{model_name} input vector width {in_width} does not match output vector width {out_width}"
        )));
    }
    Ok(in_width)
}

fn analog_vector_input_value(ctx: &CmContext, name: &str, index: usize) -> Value {
    ctx.input_analog_vector_values(name)
        .and_then(|inputs| inputs.get(index))
        .map(|input| input.value)
        .unwrap_or(0.0)
}

fn digital_vector_input_value(ctx: &CmContext, name: &str, index: usize) -> DigitalValue {
    ctx.input_digital_vector_values(name)
        .and_then(|inputs| inputs.get(index).copied())
        .unwrap_or_default()
}

fn dac_bridge_state_base(index: usize) -> usize {
    index * 4
}

fn dac_bridge_out_undef(ctx: &CmContext, out_low: Value, out_high: Value) -> Value {
    if ctx.param_was_provided("out_low")
        && ctx.param_was_provided("out_high")
        && !ctx.param_was_provided("out_undef")
    {
        out_low + (out_high - out_low) / 2.0
    } else {
        ctx.param("out_undef")
    }
}

impl CodeModel for DacBridge {
    fn name(&self) -> &str {
        "dac_bridge"
    }

    fn description(&self) -> &str {
        "Digital to analog converter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital)
                    .with_description("Digital input vector"),
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
                    is_vector: true,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Analog output vector".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("out_low", 0.0),
                ParamSpec::real("out_high", 1.0),
                ParamSpec::real("out_undef", 0.5),
                ParamSpec::real("input_load", 1.0e-12),
                ParamSpec::real("t_rise", 1e-9)
                    .with_description("Rise time, clamped to official lower limit"),
                ParamSpec::real("t_fall", 1e-9)
                    .with_description("Fall time, clamped to official lower limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, "dac_bridge")?;
        ctx.allocate_states(4 * width);
        let out_low = ctx.param("out_low");
        let out_high = ctx.param("out_high");
        let undef = dac_bridge_out_undef(ctx, out_low, out_high);
        for index in 0..width {
            let base = dac_bridge_state_base(index);
            ctx.set_initial_state(base, undef);
            ctx.set_initial_state(base + 1, undef);
            ctx.set_initial_state(base + 2, 0.0);
            ctx.set_initial_state(base + 3, undef);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, "dac_bridge")?;
        let out_low = ctx.param("out_low");
        let out_high = ctx.param("out_high");
        let out_undef = dac_bridge_out_undef(ctx, out_low, out_high);
        let t_rise = official_bridge_timing(ctx.param("t_rise"));
        let t_fall = official_bridge_timing(ctx.param("t_fall"));
        let commit_outputs = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;

        for index in 0..width {
            let d_in = digital_vector_input_value(ctx, "in", index);
            let v_target = if d_in.state.is_high() {
                out_high
            } else if d_in.state.is_low() {
                out_low
            } else {
                out_undef
            };
            let base = dac_bridge_state_base(index);

            if !ctx.is_transient() {
                ctx.set_state(base, v_target);
                ctx.set_state(base + 1, v_target);
                ctx.set_state(base + 2, ctx.time);
                ctx.set_state(base + 3, v_target);
                ctx.set_output_vector_element("out", index, v_target);
                continue;
            }

            let accepted_output = ctx.state_prev(base);
            let accepted_target = ctx.state_prev(base + 1);
            let accepted_start_time = ctx.state_prev(base + 2);
            let accepted_start_value = ctx.state_prev(base + 3);
            let first_transient_point = ctx.time <= ctx.timestep.max(0.0) + 1e-18
                && (accepted_output - out_undef).abs() <= 1e-12;

            let (transition_target, transition_start_time, transition_start_value) =
                if first_transient_point {
                    (v_target, ctx.time, v_target)
                } else if (v_target - accepted_target).abs() > 1e-12 {
                    let event_time = ctx
                        .input_digital_vector_event_time("in", index)
                        .unwrap_or(ctx.time);
                    let start_value = dac_bridge_ramp_value(
                        event_time,
                        accepted_start_time,
                        accepted_start_value,
                        accepted_target,
                        out_low,
                        out_high,
                        t_rise,
                        t_fall,
                    );
                    (v_target, event_time, start_value)
                } else {
                    (accepted_target, accepted_start_time, accepted_start_value)
                };

            let v_out = dac_bridge_ramp_value(
                ctx.time,
                transition_start_time,
                transition_start_value,
                transition_target,
                out_low,
                out_high,
                t_rise,
                t_fall,
            );

            let v_out = if first_transient_point {
                v_target
            } else {
                v_out
            };

            if let Some(completion_time) = dac_bridge_completion_time(
                transition_start_time,
                transition_start_value,
                transition_target,
                out_low,
                out_high,
                t_rise,
                t_fall,
            ) && commit_outputs
                && completion_time > ctx.time + 1.0e-18
            {
                ctx.request_breakpoint(completion_time);
            }

            if commit_outputs {
                ctx.set_state(base, v_out);
                ctx.set_state(base + 1, transition_target);
                ctx.set_state(base + 2, transition_start_time);
                ctx.set_state(base + 3, transition_start_value);
            }
            ctx.set_output_vector_element("out", index, v_out);
        }

        Ok(())
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

impl CodeModel for BidiBridge {
    fn name(&self) -> &str {
        "bidi_bridge"
    }

    fn description(&self) -> &str {
        "Bidirectional digital/analog node bridge"
    }

    fn ports(&self) -> &[PortSpec] {
        bidi_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        bidi_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = ctx.port_width("a");
        if width == 0 {
            return Err(CmError::InvalidPortConnection(
                "bidi_bridge requires at least one analog/digital bit".to_string(),
            ));
        }
        if ctx.port_width("d") != width {
            return Err(CmError::InvalidPortConnection(format!(
                "bidi_bridge analog vector width {} does not match digital vector width {}",
                width,
                ctx.port_width("d")
            )));
        }

        let params = bidi_params(ctx);
        let _input_load = params.input_load;

        ctx.allocate_states(width * 4);
        ctx.allocate_int_states(width * 2);
        let current_base = bidi_state_base(width);
        let drive_state_base = bidi_drive_state_base(width);
        let drive_strength_base = bidi_drive_strength_base(width);
        let strength_base = bidi_int_strength_base(width);
        for index in 0..width {
            ctx.set_initial_state(BIDI_STATE_SVOC_BASE + index, 0.5);
            ctx.set_initial_state(current_base + index, 0.0);
            ctx.set_initial_state(
                drive_state_base + index,
                digital_state_code(DigitalState::Unknown) as Value,
            );
            ctx.set_initial_state(
                drive_strength_base + index,
                digital_strength_code(DigitalStrength::HighZ) as Value,
            );
            ctx.set_int_state(
                BIDI_INT_OUTPUT_STATE_BASE + index,
                BIDI_UNINITIALIZED_STATE_CODE,
            );
            ctx.set_int_state(
                strength_base + index,
                digital_strength_code(DigitalStrength::HighZ),
            );
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = ctx.port_width("a");
        if ctx.port_width("d") != width {
            return Err(CmError::InvalidPortConnection(format!(
                "bidi_bridge analog vector width {} does not match digital vector width {}",
                width,
                ctx.port_width("d")
            )));
        }

        let params = bidi_params(ctx);
        let output_strength = digital_strength_from_param(params.strength);
        let current_base = bidi_state_base(width);
        let drive_state_base = bidi_drive_state_base(width);
        let drive_strength_base = bidi_drive_strength_base(width);
        let strength_base = bidi_int_strength_base(width);
        let commit_outputs = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;

        for index in 0..width {
            let voltage = analog_vector_input_value(ctx, "a", index);
            let direction_request = bidi_direction(
                params,
                ctx.input_digital_vector_values("dir").unwrap_or(&[]),
                index,
            );
            let old_state_code = ctx.int_state(BIDI_INT_OUTPUT_STATE_BASE + index);
            let old_state = if old_state_code == BIDI_UNINITIALIZED_STATE_CODE {
                DigitalState::Unknown
            } else {
                digital_state_from_code(old_state_code)
            };
            let old_strength = digital_strength_from_code(ctx.int_state(strength_base + index));
            let digital_input = digital_vector_input_value(ctx, "d", index);
            let direction = if direction_request == BidiDirection::Bidirectional {
                bidi_default_effective_direction(digital_input, old_state, old_strength)
            } else {
                direction_request
            };

            if direction == BidiDirection::Dac {
                if commit_outputs && old_strength != DigitalStrength::HighZ {
                    ctx.set_output_digital_vector_element(
                        "d",
                        index,
                        DigitalValue::high_z(),
                        params.rise_delay.max(params.fall_delay),
                    );
                    ctx.set_int_state(
                        BIDI_INT_OUTPUT_STATE_BASE + index,
                        digital_state_code(DigitalState::HighZ),
                    );
                    ctx.set_int_state(
                        strength_base + index,
                        digital_strength_code(DigitalStrength::HighZ),
                    );
                }
            } else {
                let new_state = bidi_adc_state(voltage, old_state, params);
                if commit_outputs && (new_state != old_state || old_strength != output_strength) {
                    ctx.set_output_digital_vector_element(
                        "d",
                        index,
                        digital_value_with_strength(new_state, output_strength),
                        bidi_digital_delay(new_state, params),
                    );
                    ctx.set_int_state(
                        BIDI_INT_OUTPUT_STATE_BASE + index,
                        digital_state_code(new_state),
                    );
                    ctx.set_int_state(
                        strength_base + index,
                        digital_strength_code(output_strength),
                    );
                }
            }

            let drive = match direction {
                BidiDirection::Adc => DigitalValue::high_z(),
                BidiDirection::Dac | BidiDirection::Bidirectional => digital_input,
            };
            let analog_drive = bidi_drive_current(ctx, index, voltage, drive, params, width);
            let pair = ctx.port_vector_node_pair("a", index).unwrap_or((0, 0));
            stamp_pair_conductance(ctx, pair, analog_drive.partial);
            if !ctx.is_ac() {
                let equivalent = analog_drive.current - analog_drive.partial * voltage;
                stamp_pair_current_rhs(ctx, pair, equivalent);
            }

            if commit_outputs {
                ctx.set_state(BIDI_STATE_SVOC_BASE + index, analog_drive.svoc);
                ctx.set_state(current_base + index, analog_drive.current);
                ctx.set_state(
                    drive_state_base + index,
                    digital_state_code(drive.state) as Value,
                );
                ctx.set_state(
                    drive_strength_base + index,
                    digital_strength_code(drive.strength) as Value,
                );
            }
            ctx.set_output_vector_element_with_partial(
                "a",
                index,
                analog_drive.current,
                analog_drive.partial,
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::{AnalogValue, InputValue};
    use crate::xspice::{AnalysisType, CallType, EvaluationPhase, ParamType};

    #[test]
    fn adc_bridge_metadata_matches_ngspice46_interface() {
        let ports = AdcBridge.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["in", "out"]
        );
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[0].default_type, PortType::Voltage);
        assert_eq!(
            ports[0].allowed_types,
            vec![
                PortType::Voltage,
                PortType::DifferentialVoltage,
                PortType::Current,
                PortType::DifferentialCurrent,
                PortType::VoltageName,
            ]
        );
        assert!(ports[0].is_vector);
        assert!(!ports[0].null_allowed);
        assert_eq!(ports[1].direction, PortDirection::Out);
        assert_eq!(ports[1].default_type, PortType::Digital);
        assert!(ports[1].is_vector);
        assert!(!ports[1].null_allowed);

        let params = AdcBridge.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type, param.default))
                .collect::<Vec<_>>(),
            vec![
                ("in_low", &ParamType::Real, 0.1),
                ("in_high", &ParamType::Real, 0.9),
                ("rise_delay", &ParamType::Real, 1.0e-9),
                ("fall_delay", &ParamType::Real, 1.0e-9),
            ]
        );
    }

    #[test]
    fn dac_bridge_metadata_matches_ngspice46_interface() {
        let ports = DacBridge.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["in", "out"]
        );
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[0].default_type, PortType::Digital);
        assert!(ports[0].is_vector);
        assert!(!ports[0].null_allowed);
        assert_eq!(ports[1].direction, PortDirection::Out);
        assert_eq!(ports[1].default_type, PortType::Voltage);
        assert_eq!(
            ports[1].allowed_types,
            vec![
                PortType::Voltage,
                PortType::DifferentialVoltage,
                PortType::Current,
                PortType::DifferentialCurrent,
            ]
        );
        assert!(ports[1].is_vector);
        assert!(!ports[1].null_allowed);

        let params = DacBridge.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type, param.default))
                .collect::<Vec<_>>(),
            vec![
                ("out_low", &ParamType::Real, 0.0),
                ("out_high", &ParamType::Real, 1.0),
                ("out_undef", &ParamType::Real, 0.5),
                ("input_load", &ParamType::Real, 1.0e-12),
                ("t_rise", &ParamType::Real, 1.0e-9),
                ("t_fall", &ParamType::Real, 1.0e-9),
            ]
        );
    }

    #[test]
    fn bidi_bridge_metadata_matches_ngspice46_interface() {
        let ports = BidiBridge.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "d", "dir"]
        );
        assert_eq!(ports[0].direction, PortDirection::InOut);
        assert_eq!(ports[0].default_type, PortType::Conductance);
        assert_eq!(
            ports[0].allowed_types,
            vec![PortType::Conductance, PortType::DifferentialConductance]
        );
        assert!(ports[0].is_vector);
        assert!(!ports[0].null_allowed);
        assert_eq!(ports[0].vector_min_len, Some(1));
        assert_eq!(ports[1].direction, PortDirection::InOut);
        assert_eq!(ports[1].default_type, PortType::Digital);
        assert!(ports[1].is_vector);
        assert!(!ports[1].null_allowed);
        assert_eq!(ports[1].vector_min_len, Some(1));
        assert_eq!(ports[2].direction, PortDirection::In);
        assert_eq!(ports[2].default_type, PortType::Digital);
        assert!(ports[2].is_vector);
        assert!(ports[2].null_allowed);
        assert_eq!(ports[2].vector_min_len, None);

        let params = BidiBridge.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type, param.default))
                .collect::<Vec<_>>(),
            vec![
                ("direction", &ParamType::Integer, 2.0),
                ("input_load", &ParamType::Real, 1.0e-12),
                ("strength", &ParamType::Integer, 0.0),
                ("smooth", &ParamType::Integer, 0.0),
                ("in_low", &ParamType::Real, 0.1),
                ("in_high", &ParamType::Real, 0.9),
                ("out_low", &ParamType::Real, 0.0),
                ("out_high", &ParamType::Real, 3.3),
                ("drive_low", &ParamType::Real, 0.02),
                ("drive_high", &ParamType::Real, 0.02),
                ("r_stl", &ParamType::Real, 20.0),
                ("r_sth", &ParamType::Real, 20.0),
                ("r_low", &ParamType::Real, 10_000.0),
                ("r_high", &ParamType::Real, 10_000.0),
                ("t_rise", &ParamType::Real, 1.0e-9),
                ("t_fall", &ParamType::Real, 1.0e-9),
                ("rise_delay", &ParamType::Real, 1.0e-9),
                ("fall_delay", &ParamType::Real, 1.0e-9),
            ]
        );
    }

    #[test]
    fn adc_bridge_does_not_commit_rollbackable_probe_crossing() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_port_width("out", 1);
        ctx.set_param("in_low", 0.1);
        ctx.set_param("in_high", 0.9);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.init_output_vector("out", PortType::Digital, 1);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.timestep = 1.0e-9;

        AdcBridge.init(&mut ctx).expect("adc_bridge initializes");

        ctx.time = 0.0;
        ctx.set_input("in", InputValue::AnalogVector(vec![AnalogValue::new(0.0)]));
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AdcBridge
            .evaluate(&mut ctx)
            .expect("records initial low state");
        ctx.take_pending_events();

        ctx.time = 0.5e-9;
        ctx.set_input("in", InputValue::AnalogVector(vec![AnalogValue::new(1.0)]));
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        AdcBridge
            .evaluate(&mut ctx)
            .expect("probes high threshold crossing");
        assert!(
            ctx.take_pending_events().is_empty(),
            "rollbackable adc_bridge probe must not queue a digital event"
        );
        assert_eq!(
            ctx.int_state(0),
            0,
            "rollbackable adc_bridge probe must not advance remembered threshold state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AdcBridge
            .evaluate(&mut ctx)
            .expect("commits accepted high crossing");
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1, "accepted crossing should emit one event");
        assert_eq!(events[0].start_index, 0);
        assert_eq!(events[0].values[0].state, DigitalState::One);
        assert_eq!(ctx.int_state(0), 1);
    }

    #[test]
    fn dac_bridge_rollbackable_probe_does_not_commit_ramp_state_or_breakpoints() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_port_width("out", 1);
        ctx.set_param("out_low", 0.0);
        ctx.set_param("out_high", 1.0);
        ctx.set_param("out_undef", 0.5);
        ctx.set_param("input_load", 1.0e-12);
        ctx.set_param("t_rise", 1.0e-9);
        ctx.set_param("t_fall", 1.0e-9);
        ctx.init_output_vector("out", PortType::Voltage, 1);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.timestep = 1.0e-9;

        DacBridge.init(&mut ctx).expect("dac_bridge initializes");

        ctx.time = 0.0;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        DacBridge
            .evaluate(&mut ctx)
            .expect("records initial low ramp state");
        ctx.advance_state();

        let base = dac_bridge_state_base(0);
        assert_eq!(ctx.state_prev(base), 0.0);
        assert_eq!(ctx.state_prev(base + 1), 0.0);
        assert_eq!(ctx.state_prev(base + 2), 0.0);
        assert_eq!(ctx.state_prev(base + 3), 0.0);

        ctx.time = 0.5e-9;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.set_input_digital_vector_event_times("in", vec![Some(0.0)]);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        DacBridge
            .evaluate(&mut ctx)
            .expect("probes rising analog ramp");
        assert!(
            (ctx.output_vector("out")[0] - 0.5).abs() <= 1.0e-15,
            "rollbackable dac_bridge probe should still compute trial ramp output"
        );
        assert_eq!(
            ctx.state(base),
            0.0,
            "rollbackable dac_bridge probe must not advance current output state"
        );
        assert_eq!(
            ctx.state(base + 1),
            0.0,
            "rollbackable dac_bridge probe must not advance target state"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable dac_bridge probe must not schedule ramp completion breakpoints"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        DacBridge
            .evaluate(&mut ctx)
            .expect("commits accepted rising ramp");
        assert!((ctx.state(base) - 0.5).abs() <= 1.0e-15);
        assert_eq!(ctx.state(base + 1), 1.0);
        assert_eq!(ctx.state(base + 2), 0.0);
        assert_eq!(ctx.state(base + 3), 0.0);
        assert_eq!(ctx.take_requested_breakpoints(), vec![1.0e-9]);
    }

    #[test]
    fn dac_bridge_ramps_inverted_output_levels() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_port_width("out", 1);
        ctx.set_param("out_low", 5.0);
        ctx.set_param("out_high", 0.0);
        ctx.set_param("out_undef", 2.5);
        ctx.set_param("input_load", 1.0e-12);
        ctx.set_param("t_rise", 1.0e-9);
        ctx.set_param("t_fall", 2.0e-9);
        ctx.init_output_vector("out", PortType::Voltage, 1);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.timestep = 1.0e-9;

        DacBridge.init(&mut ctx).expect("dac_bridge initializes");

        ctx.time = 0.0;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        DacBridge
            .evaluate(&mut ctx)
            .expect("records initial active-low zero level");
        ctx.advance_state();

        ctx.time = 0.25e-9;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.set_input_digital_vector_event_times("in", vec![Some(0.0)]);
        DacBridge
            .evaluate(&mut ctx)
            .expect("ramps toward inverted one level");

        assert!(
            (ctx.output_vector("out")[0] - 3.75).abs() <= 1.0e-15,
            "inverted dac_bridge levels should ramp instead of snapping"
        );
        assert_eq!(ctx.take_requested_breakpoints(), vec![1.0e-9]);
    }

    #[test]
    fn bidi_bridge_forced_adc_does_not_commit_rollbackable_probe_crossing() {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, 0.0);
        ctx.set_param("direction", 1.0);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(0.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::high_z()]));
        ctx.init_output_vector("d", PortType::Digital, 1);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.timestep = 1.0e-9;

        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        ctx.time = 0.0;
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        BidiBridge
            .evaluate(&mut ctx)
            .expect("records initial low state");
        ctx.take_pending_events();

        ctx.time = 0.5e-9;
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(1.0)]));
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        BidiBridge
            .evaluate(&mut ctx)
            .expect("probes high threshold crossing");
        assert!(
            ctx.take_pending_events().is_empty(),
            "rollbackable bidi_bridge ADC probe must not queue a digital event"
        );
        assert_eq!(
            ctx.int_state(BIDI_INT_OUTPUT_STATE_BASE),
            digital_state_code(DigitalState::Zero),
            "rollbackable bidi_bridge ADC probe must not advance remembered threshold state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        BidiBridge
            .evaluate(&mut ctx)
            .expect("commits accepted high crossing");
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1, "accepted crossing should emit one event");
        assert_eq!(events[0].start_index, 0);
        assert_eq!(events[0].values[0].state, DigitalState::One);
        assert_eq!(
            ctx.int_state(BIDI_INT_OUTPUT_STATE_BASE),
            digital_state_code(DigitalState::One)
        );
    }

    #[test]
    fn bidi_bridge_dac_rollbackable_probe_does_not_commit_analog_drive_state() {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, 0.0);
        ctx.set_param("direction", 0.0);
        ctx.set_param("drive_low", 100.0);
        ctx.set_param("drive_high", 100.0);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(2.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time_prev = 0.0;
        ctx.time = 0.5e-9;
        ctx.timestep = 1.0e-9;
        ctx.set_input_digital_vector_event_times("d", vec![Some(0.0)]);

        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        let width = 1;
        let current_base = bidi_state_base(width);
        let drive_state_base = bidi_drive_state_base(width);
        let drive_strength_base = bidi_drive_strength_base(width);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        BidiBridge
            .evaluate(&mut ctx)
            .expect("probes DAC analog drive");
        assert!(
            ctx.output_vector("a")[0].is_finite(),
            "rollbackable bidi_bridge probe should still compute analog trial current"
        );
        assert_eq!(
            ctx.state(BIDI_STATE_SVOC_BASE),
            0.5,
            "rollbackable bidi_bridge probe must not advance normalized drive voltage"
        );
        assert_eq!(
            ctx.state(current_base),
            0.0,
            "rollbackable bidi_bridge probe must not advance output current state"
        );
        assert_eq!(
            ctx.state(drive_state_base),
            digital_state_code(DigitalState::Unknown) as Value,
            "rollbackable bidi_bridge probe must not advance previous drive state"
        );
        assert_eq!(
            ctx.state(drive_strength_base),
            digital_strength_code(DigitalStrength::HighZ) as Value,
            "rollbackable bidi_bridge probe must not advance previous drive strength"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        BidiBridge
            .evaluate(&mut ctx)
            .expect("commits DAC analog drive");
        assert_eq!(ctx.state(BIDI_STATE_SVOC_BASE), 1.0);
        assert!(ctx.state(current_base).is_finite());
        assert_eq!(
            ctx.state(drive_state_base),
            digital_state_code(DigitalState::One) as Value
        );
        assert_eq!(
            ctx.state(drive_strength_base),
            digital_strength_code(DigitalStrength::Strong) as Value
        );
    }

    #[test]
    fn bidi_bridge_high_z_drive_resets_svoc_like_ngspice() {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, 0.0);
        ctx.set_param("direction", 0.0);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(0.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::high_z()]));
        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        ctx.set_initial_state(BIDI_STATE_SVOC_BASE, 1.0);
        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time_prev = 0.0;
        ctx.time = 0.1e-9;
        ctx.timestep = 0.1e-9;

        BidiBridge
            .evaluate(&mut ctx)
            .expect("bidi_bridge evaluates high-Z drive");

        assert_eq!(
            ctx.state(BIDI_STATE_SVOC_BASE),
            0.5,
            "ngspice resets bidi_bridge open-circuit voltage immediately for high-Z drive"
        );
    }

    fn set_bidi_default_params(ctx: &mut CmContext, smooth: Value) {
        for (name, value) in [
            ("direction", 0.0),
            ("input_load", 1.0e-12),
            ("strength", 0.0),
            ("smooth", smooth),
            ("in_low", 0.1),
            ("in_high", 0.9),
            ("out_low", 0.0),
            ("out_high", 3.3),
            ("drive_low", 0.02),
            ("drive_high", 0.02),
            ("r_stl", 20.0),
            ("r_sth", 20.0),
            ("r_low", 10_000.0),
            ("r_high", 10_000.0),
            ("t_rise", 1.0e-9),
            ("t_fall", 1.0e-9),
            ("rise_delay", 1.0e-9),
            ("fall_delay", 1.0e-9),
        ] {
            ctx.set_param(name, value);
        }
    }

    fn bidi_output_current_for_smooth(smooth: Value) -> Value {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, smooth);
        ctx.set_param("t_fall", 0.1e-9);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(0.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::one()]));
        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time = 0.35e-9;
        ctx.time_prev = 0.0;
        ctx.timestep = 1.0e-9;
        ctx.set_input_digital_vector_event_times("d", vec![Some(0.0)]);

        BidiBridge
            .evaluate(&mut ctx)
            .expect("bidi_bridge evaluates");
        ctx.output_vector("a")[0]
    }

    #[test]
    fn bidi_bridge_smooth_one_smooths_open_circuit_voltage_like_ngspice() {
        let raw_svoc = 0.85;
        let raw_expected = -0.02 * (raw_svoc - BIDI_OFF_HIGH) / (1.0 - BIDI_OFF_HIGH);
        let smooth_voc = -2.0 * raw_svoc * raw_svoc + 4.0 * raw_svoc - 1.0;
        let smooth_expected = -0.02 * (smooth_voc - BIDI_OFF_HIGH) / (1.0 - BIDI_OFF_HIGH);

        let raw_current = bidi_output_current_for_smooth(0.0);
        let smooth_current = bidi_output_current_for_smooth(1.0);

        assert!(
            (raw_current - raw_expected).abs() <= 1.0e-15,
            "unsmoothed strong-one current should match ngspice raw clamp, got {raw_current}"
        );
        assert!(
            (smooth_current - smooth_expected).abs() <= 1.0e-15,
            "smooth=1 should smooth normalized voc before current limiting, got {smooth_current}, expected {smooth_expected}"
        );
        assert!(
            smooth_current < raw_current,
            "smooth=1 should allow the stronger high-side clamp at svoc={raw_svoc}, raw={raw_current}, smooth={smooth_current}"
        );
    }

    #[test]
    fn bidi_bridge_dac_event_at_current_time_uses_previous_drive_like_ngspice() {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, 0.0);
        ctx.set_param("r_stl", 10.0);
        ctx.set_param("r_sth", 1000.0);
        ctx.set_param("drive_low", 100.0);
        ctx.set_param("drive_high", 100.0);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(2.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        let width = 1;
        ctx.set_initial_state(BIDI_STATE_SVOC_BASE, 0.5);
        ctx.set_initial_state(
            bidi_drive_state_base(width),
            digital_state_code(DigitalState::One) as Value,
        );
        ctx.set_initial_state(
            bidi_drive_strength_base(width),
            digital_strength_code(DigitalStrength::Strong) as Value,
        );

        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time_prev = 0.0;
        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;
        ctx.set_input_digital_vector_event_times("d", vec![Some(ctx.time)]);

        BidiBridge
            .evaluate(&mut ctx)
            .expect("bidi_bridge evaluates");

        let current = ctx.output_vector("a")[0];
        let expected = (2.0 - 3.3) / 1000.0;
        assert!(
            (ctx.state(BIDI_STATE_SVOC_BASE) - 1.0).abs() <= 1.0e-15,
            "event at T(0) should leave previous ONE drive active for the whole analog step"
        );
        assert!(
            (current - expected).abs() <= 1.0e-15,
            "ngspice uses the previous drive when the event is at T(0), got {current}, expected {expected}"
        );
    }

    #[test]
    fn bidi_bridge_mid_step_dac_event_applies_full_step_svoc_per_segment_like_ngspice() {
        let mut ctx = CmContext::new();
        set_bidi_default_params(&mut ctx, 0.0);
        ctx.set_param("r_stl", 10.0);
        ctx.set_param("r_sth", 1000.0);
        ctx.set_param("drive_low", 100.0);
        ctx.set_param("drive_high", 100.0);
        ctx.set_port_width("a", 1);
        ctx.set_port_width("d", 1);
        ctx.set_input("a", InputValue::AnalogVector(vec![AnalogValue::new(2.0)]));
        ctx.set_input("d", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        BidiBridge.init(&mut ctx).expect("bidi_bridge initializes");

        let width = 1;
        ctx.set_initial_state(BIDI_STATE_SVOC_BASE, 0.5);
        ctx.set_initial_state(
            bidi_drive_state_base(width),
            digital_state_code(DigitalState::One) as Value,
        );
        ctx.set_initial_state(
            bidi_drive_strength_base(width),
            digital_strength_code(DigitalStrength::Strong) as Value,
        );

        ctx.analysis = AnalysisType::Transient;
        ctx.call_type = CallType::TransientAnalysis;
        ctx.time_prev = 0.0;
        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;
        ctx.set_input_digital_vector_event_times("d", vec![Some(0.75e-9)]);

        BidiBridge
            .evaluate(&mut ctx)
            .expect("bidi_bridge evaluates");

        assert!(
            ctx.state(BIDI_STATE_SVOC_BASE).abs() <= 1.0e-15,
            "ngspice advances bidi_bridge svoc by the full analog step for each split segment; got {}",
            ctx.state(BIDI_STATE_SVOC_BASE)
        );
    }
}
