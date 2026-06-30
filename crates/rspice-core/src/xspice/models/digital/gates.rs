use super::*;
use crate::Value;
use crate::xspice::EvaluationPhase;

//=============================================================================
// Basic Gates
//=============================================================================

const INITIAL_GATE_STATE: i64 = i64::MIN;
const OFFICIAL_DIGITAL_DELAY_MIN: Value = 1.0e-12;

fn gate_delay(ctx: &CmContext, new_state: i64, prev_state: i64, rise: Value, fall: Value) -> Value {
    if ctx.time == 0.0 || prev_state == INITIAL_GATE_STATE {
        0.0
    } else if new_state == 1 {
        rise
    } else if new_state == 0 {
        fall
    } else if prev_state == 0 {
        rise
    } else {
        fall
    }
}

fn official_digital_delay(ctx: &CmContext, name: &str) -> Value {
    ctx.param(name).max(OFFICIAL_DIGITAL_DELAY_MIN)
}

fn gate_commits_state(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

fn gate_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if gate_commits_state(ctx) {
        ctx.set_int_state(index, value);
    }
}

fn gate_set_state(ctx: &mut CmContext, index: usize, value: Value) {
    if gate_commits_state(ctx) {
        ctx.set_state(index, value);
    }
}

fn gate_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("rise_delay", 1e-9)
                .with_description("Rise propagation delay, clamped to official lower limit"),
            ParamSpec::real("fall_delay", 1e-9)
                .with_description("Fall propagation delay, clamped to official lower limit"),
            ParamSpec::real("input_load", 1.0e-12),
            ParamSpec::string("family", ""),
            ParamSpec::boolean("inertial_delay", false),
        ]
    })
}

fn digital_logic_code(value: DigitalValue) -> i64 {
    if value.state.is_high() {
        1
    } else if value.state.is_low() {
        0
    } else {
        -1
    }
}

fn strong_value_from_logic_code(code: i64) -> DigitalValue {
    match code {
        0 => DigitalValue::zero(),
        1 => DigitalValue::one(),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
    }
}

fn digital_strength_code(strength: DigitalStrength) -> i64 {
    match strength {
        DigitalStrength::Undetermined => 0,
        DigitalStrength::HighZ => 1,
        DigitalStrength::Resistive => 2,
        DigitalStrength::Strong => 3,
    }
}

fn digital_strength_from_code(code: i64) -> DigitalStrength {
    match code {
        1 => DigitalStrength::HighZ,
        2 => DigitalStrength::Resistive,
        3 => DigitalStrength::Strong,
        _ => DigitalStrength::Undetermined,
    }
}

fn set_basic_gate_output(
    ctx: &mut CmContext,
    name: &str,
    value: DigitalValue,
    previous: DigitalValue,
    delay: Value,
    rise: Value,
    fall: Value,
) {
    set_gate_output_with_unknown_delays(ctx, name, value, previous, delay, Some((rise, fall)));
}

fn set_gate_output_with_unknown_delays(
    ctx: &mut CmContext,
    name: &str,
    value: DigitalValue,
    previous: DigitalValue,
    delay: Value,
    unknown_transition_delays: Option<(Value, Value)>,
) {
    if ctx.param("inertial_delay") > 0.5 {
        ctx.set_output_digital_inertial(name, value, delay, previous, unknown_transition_delays);
    } else {
        ctx.set_output_digital(name, value, delay);
    }
}

fn tristate_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("delay", 1.0e-9)
                .with_description("Output delay, clamped to official lower limit"),
            ParamSpec::real("input_load", 1.0e-12),
            ParamSpec::real("enable_load", 1.0e-12),
            ParamSpec::boolean("inertial_delay", false),
            ParamSpec::string("family", ""),
        ]
    })
}

fn tristate_output(input: DigitalValue, enable: DigitalValue) -> DigitalValue {
    let strength = match enable.state.logic_level() {
        Some(false) => DigitalStrength::HighZ,
        Some(true) => DigitalStrength::Strong,
        None => DigitalStrength::Undetermined,
    };
    DigitalValue::new(input.state, strength)
}

fn tristate_value_from_codes(state_code: i64, strength_code: i64) -> DigitalValue {
    let state = match state_code {
        0 => DigitalState::Zero,
        1 => DigitalState::One,
        _ => DigitalState::Unknown,
    };
    DigitalValue::new(state, digital_strength_from_code(strength_code))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TristateInertialControl {
    Idle,
    Normal,
    Same,
    Revert,
    Both,
}

fn tristate_direct_revert(
    d_first: bool,
    value: &mut i64,
    strength: &mut i64,
    output_delay: &mut Value,
    delay: Value,
    state_prev: i64,
    strength_prev: i64,
) {
    if d_first {
        *strength = strength_prev;
    } else {
        *value = state_prev;
    }
    *output_delay = delay;
}

fn tristate_push_revert(
    events: &mut Vec<(Value, DigitalValue)>,
    d_first: bool,
    delay: Value,
    state_prev: i64,
    strength_prev: i64,
    out_state: i64,
    out_strength: i64,
) {
    let (state, strength) = if d_first {
        (out_state, strength_prev)
    } else {
        (state_prev, out_strength)
    };
    events.push((delay, tristate_value_from_codes(state, strength)));
}

fn set_tristate_output(
    ctx: &mut CmContext,
    value: DigitalValue,
    delay: Value,
    out_state: i64,
    out_strength: i64,
) {
    let mut state = digital_logic_code(value);
    let mut strength = digital_strength_code(value.strength);

    if state == out_state && strength == out_strength {
        return;
    }

    if ctx.param("inertial_delay") <= 0.5 || !ctx.is_transient() {
        ctx.set_output_digital("out", value, delay);
        gate_set_int_state(ctx, 0, state);
        gate_set_int_state(ctx, 1, strength);
        return;
    }

    let time = ctx.time;
    let mut state_when = ctx.state(0);
    let mut strength_when = ctx.state(1);
    let mut state_prev = ctx.int_state(2);
    let mut strength_prev = ctx.int_state(3);
    let (first_time, second_time, mut d_first) = if state_when <= strength_when {
        (state_when, strength_when, true)
    } else {
        (strength_when, state_when, false)
    };
    let mut output_delay = delay;
    let mut events = Vec::new();

    let state_control = if state_when <= time {
        if state == out_state {
            state_prev = state;
            TristateInertialControl::Idle
        } else {
            state_prev = out_state;
            state_when = time + output_delay;
            TristateInertialControl::Normal
        }
    } else if state == out_state {
        TristateInertialControl::Same
    } else if state == state_prev {
        state_when = -1.0;
        TristateInertialControl::Revert
    } else {
        state_when = time + output_delay;
        TristateInertialControl::Both
    };

    let strength_control = if strength_when <= time {
        if strength == out_strength {
            strength_prev = strength;
            TristateInertialControl::Idle
        } else {
            strength_prev = out_strength;
            strength_when = time + output_delay;
            TristateInertialControl::Normal
        }
    } else if strength == out_strength {
        TristateInertialControl::Same
    } else if strength == strength_prev {
        strength_when = -1.0;
        TristateInertialControl::Revert
    } else {
        strength_when = time + output_delay;
        TristateInertialControl::Both
    };

    let (first_control, second_control) = if d_first {
        (state_control, strength_control)
    } else {
        (strength_control, state_control)
    };

    match first_control {
        TristateInertialControl::Idle => match second_control {
            TristateInertialControl::Revert => {
                tristate_direct_revert(
                    d_first,
                    &mut state,
                    &mut strength,
                    &mut output_delay,
                    (second_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                );
            }
            TristateInertialControl::Both => {
                tristate_push_revert(
                    &mut events,
                    d_first,
                    (second_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                    out_state,
                    out_strength,
                );
            }
            _ => {}
        },
        TristateInertialControl::Normal => match second_control {
            TristateInertialControl::Revert => {
                let reversion = if d_first {
                    strength = strength_prev;
                    tristate_value_from_codes(out_state, strength_prev)
                } else {
                    state = state_prev;
                    tristate_value_from_codes(state_prev, out_strength)
                };
                events.push(((second_time - time) / 2.0, reversion));
            }
            TristateInertialControl::Both => {
                tristate_push_revert(
                    &mut events,
                    d_first,
                    (second_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                    out_state,
                    out_strength,
                );
            }
            _ => {}
        },
        TristateInertialControl::Same => match second_control {
            TristateInertialControl::Revert => {
                tristate_direct_revert(
                    d_first,
                    &mut state,
                    &mut strength,
                    &mut output_delay,
                    (first_time + second_time) / 2.0 - time,
                    state_prev,
                    strength_prev,
                );
            }
            TristateInertialControl::Both => {
                tristate_push_revert(
                    &mut events,
                    d_first,
                    (first_time + second_time) / 2.0 - time,
                    state_prev,
                    strength_prev,
                    out_state,
                    out_strength,
                );
            }
            _ => {}
        },
        TristateInertialControl::Revert => match second_control {
            TristateInertialControl::Normal => {
                d_first = !d_first;
                tristate_push_revert(
                    &mut events,
                    d_first,
                    (first_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                    out_state,
                    out_strength,
                );
            }
            TristateInertialControl::Same => {
                events.push((
                    (first_time - time) / 2.0,
                    tristate_value_from_codes(state_prev, strength_prev),
                ));
                output_delay = second_time - time;
            }
            TristateInertialControl::Revert => {
                state = state_prev;
                strength = strength_prev;
                output_delay = (first_time - time) / 2.0;
            }
            TristateInertialControl::Both => {
                let reversion = tristate_value_from_codes(state_prev, strength_prev);
                events.push(((first_time - time) / 2.0, reversion));
                if d_first {
                    state = state_prev;
                } else {
                    strength = strength_prev;
                }
            }
            _ => {
                d_first = !d_first;
                tristate_direct_revert(
                    d_first,
                    &mut state,
                    &mut strength,
                    &mut output_delay,
                    (first_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                );
            }
        },
        TristateInertialControl::Both => match second_control {
            TristateInertialControl::Same => {
                let reversion = tristate_value_from_codes(state_prev, strength_prev);
                events.push(((first_time - time) / 2.0, reversion));
                let restoration = if d_first {
                    tristate_value_from_codes(state_prev, out_strength)
                } else {
                    tristate_value_from_codes(out_state, strength_prev)
                };
                events.push((second_time - time, restoration));
            }
            TristateInertialControl::Revert | TristateInertialControl::Both => {
                events.push((
                    (first_time - time) / 2.0,
                    tristate_value_from_codes(state_prev, strength_prev),
                ));
            }
            _ => {
                d_first = !d_first;
                tristate_push_revert(
                    &mut events,
                    d_first,
                    (first_time - time) / 2.0,
                    state_prev,
                    strength_prev,
                    out_state,
                    out_strength,
                );
            }
        },
    }

    gate_set_state(ctx, 0, state_when);
    gate_set_state(ctx, 1, strength_when);
    gate_set_int_state(ctx, 2, state_prev);
    gate_set_int_state(ctx, 3, strength_prev);
    for (event_delay, event_value) in events {
        ctx.set_output_digital("out", event_value, event_delay);
    }
    ctx.set_output_digital(
        "out",
        tristate_value_from_codes(state, strength),
        output_delay,
    );
    gate_set_int_state(ctx, 0, state);
    gate_set_int_state(ctx, 1, strength);
}

macro_rules! define_gate {
    ($name:ident, $spice_name:expr, $desc:expr, $op:expr) => {
        #[derive(Debug, Default)]
        pub struct $name;

        impl CodeModel for $name {
            fn name(&self) -> &str {
                $spice_name
            }
            fn description(&self) -> &str {
                $desc
            }

            fn ports(&self) -> &[PortSpec] {
                use std::sync::OnceLock;
                static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
                PORTS.get_or_init(|| {
                    vec![
                        PortSpec::vector_input("in", PortType::Digital).with_vector_min_len(2),
                        PortSpec::output("out", PortType::Digital),
                    ]
                })
            }

            fn parameters(&self) -> &[ParamSpec] {
                gate_params()
            }

            fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
                ctx.allocate_int_states(1);
                ctx.set_int_state(0, INITIAL_GATE_STATE);
                Ok(())
            }

            fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
                let rise = official_digital_delay(ctx, "rise_delay");
                let fall = official_digital_delay(ctx, "fall_delay");
                let inputs = ctx.input_digital_vector("in");
                let prev = ctx.int_state(0);

                let result: DigitalState = $op(&inputs);
                let new_state = if result.is_high() {
                    1
                } else if result.is_low() {
                    0
                } else {
                    -1
                };

                if new_state != prev {
                    let val = DigitalValue::new(result, DigitalStrength::Strong);
                    let delay = gate_delay(ctx, new_state, prev, rise, fall);
                    set_basic_gate_output(
                        ctx,
                        "out",
                        val,
                        strong_value_from_logic_code(prev),
                        delay,
                        rise,
                        fall,
                    );
                }
                gate_set_int_state(ctx, 0, new_state);
                Ok(())
            }
        }
    };
}

fn and_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::One, |a, b| a.and(&b.state))
}

fn or_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::Zero, |a, b| a.or(&b.state))
}

fn xor_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::Zero, |a, b| a.xor(&b.state))
}

define_gate!(DigitalAnd, "d_and", "AND gate", and_op);
define_gate!(DigitalOr, "d_or", "OR gate", or_op);
define_gate!(DigitalXor, "d_xor", "XOR gate", xor_op);

/// NAND gate
#[derive(Debug, Default)]
pub struct DigitalNand;

impl CodeModel for DigitalNand {
    fn name(&self) -> &str {
        "d_nand"
    }
    fn description(&self) -> &str {
        "NAND gate"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital).with_vector_min_len(2),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        gate_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay");
        let fall = official_digital_delay(ctx, "fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);

        let result = and_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };

        if new_state != prev {
            let val = DigitalValue::new(result, DigitalStrength::Strong);
            let delay = gate_delay(ctx, new_state, prev, rise, fall);
            set_basic_gate_output(
                ctx,
                "out",
                val,
                strong_value_from_logic_code(prev),
                delay,
                rise,
                fall,
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// NOR gate
#[derive(Debug, Default)]
pub struct DigitalNor;

impl CodeModel for DigitalNor {
    fn name(&self) -> &str {
        "d_nor"
    }
    fn description(&self) -> &str {
        "NOR gate"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalNand.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay");
        let fall = official_digital_delay(ctx, "fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);
        let result = or_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            set_basic_gate_output(
                ctx,
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                strong_value_from_logic_code(prev),
                gate_delay(ctx, new_state, prev, rise, fall),
                rise,
                fall,
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// XNOR gate
#[derive(Debug, Default)]
pub struct DigitalXnor;

impl CodeModel for DigitalXnor {
    fn name(&self) -> &str {
        "d_xnor"
    }
    fn description(&self) -> &str {
        "XNOR gate"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalNand.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay");
        let fall = official_digital_delay(ctx, "fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);
        let result = xor_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            set_basic_gate_output(
                ctx,
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                strong_value_from_logic_code(prev),
                gate_delay(ctx, new_state, prev, rise, fall),
                rise,
                fall,
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// Inverter
#[derive(Debug, Default)]
pub struct DigitalInverter;

impl CodeModel for DigitalInverter {
    fn name(&self) -> &str {
        "d_inverter"
    }
    fn description(&self) -> &str {
        "Digital inverter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay");
        let fall = official_digital_delay(ctx, "fall_delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = input.state.invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            set_basic_gate_output(
                ctx,
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                strong_value_from_logic_code(prev),
                gate_delay(ctx, new_state, prev, rise, fall),
                rise,
                fall,
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// Buffer
#[derive(Debug, Default)]
pub struct DigitalBuffer;

impl CodeModel for DigitalBuffer {
    fn name(&self) -> &str {
        "d_buffer"
    }
    fn description(&self) -> &str {
        "Digital buffer"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalInverter.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalInverter.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay");
        let fall = official_digital_delay(ctx, "fall_delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = input.state;
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            set_basic_gate_output(
                ctx,
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                strong_value_from_logic_code(prev),
                gate_delay(ctx, new_state, prev, rise, fall),
                rise,
                fall,
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// Tristate buffer
#[derive(Debug, Default)]
pub struct DigitalTristate;

impl CodeModel for DigitalTristate {
    fn name(&self) -> &str {
        "d_tristate"
    }
    fn description(&self) -> &str {
        "Tri-state buffer"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        tristate_params()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(2);
        ctx.set_initial_state(0, -1.0);
        ctx.set_initial_state(1, -1.0);
        ctx.allocate_int_states(4);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        ctx.set_int_state(1, INITIAL_GATE_STATE);
        ctx.set_int_state(2, INITIAL_GATE_STATE);
        ctx.set_int_state(3, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = official_digital_delay(ctx, "delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let enable = ctx.input_digital("enable").unwrap_or_default();

        let result = tristate_output(input, enable);
        let prev_state = ctx.int_state(0);
        let prev_strength = ctx.int_state(1);

        set_tristate_output(ctx, result, delay, prev_state, prev_strength);
        Ok(())
    }
}

fn digital_pull_resistor_parameters() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![ParamSpec::real("load", 1.0e-12).with_description("Output load capacitance")]
    })
}

/// Pull-up resistor
#[derive(Debug, Default)]
pub struct DigitalPullup;

impl CodeModel for DigitalPullup {
    fn name(&self) -> &str {
        "d_pullup"
    }
    fn description(&self) -> &str {
        "Pull-up resistor"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![PortSpec::output("out", PortType::Digital)])
    }
    fn parameters(&self) -> &[ParamSpec] {
        digital_pull_resistor_parameters()
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.set_output_digital(
            "out",
            DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
            0.0,
        );
        Ok(())
    }
}

/// Pull-down resistor
#[derive(Debug, Default)]
pub struct DigitalPulldown;

impl CodeModel for DigitalPulldown {
    fn name(&self) -> &str {
        "d_pulldown"
    }
    fn description(&self) -> &str {
        "Pull-down resistor"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalPullup.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        digital_pull_resistor_parameters()
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.set_output_digital(
            "out",
            DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
            0.0,
        );
        Ok(())
    }
}

//=============================================================================
// Open-output buffers
//=============================================================================

fn open_output_ports() -> &'static [PortSpec] {
    use std::sync::OnceLock;
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec::input("in", PortType::Digital),
            PortSpec::output("out", PortType::Digital),
        ]
    })
}

fn open_collector_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("open_delay", 1.0e-9)
                .with_description("Open propagation delay, clamped to official lower limit"),
            ParamSpec::real("fall_delay", 1.0e-9)
                .with_description("Fall propagation delay, clamped to official lower limit"),
            ParamSpec::real("input_load", 1.0e-12),
            ParamSpec::string("family", ""),
            ParamSpec::boolean("inertial_delay", false),
        ]
    })
}

fn open_emitter_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("rise_delay", 1.0e-9)
                .with_description("Rise propagation delay, clamped to official lower limit"),
            ParamSpec::real("open_delay", 1.0e-9)
                .with_description("Open propagation delay, clamped to official lower limit"),
            ParamSpec::real("input_load", 1.0e-12),
            ParamSpec::string("family", ""),
            ParamSpec::boolean("inertial_delay", false),
        ]
    })
}

fn open_collector_output(input: DigitalValue) -> DigitalValue {
    match input.state.logic_level() {
        Some(false) => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        Some(true) => DigitalValue::new(DigitalState::One, DigitalStrength::HighZ),
        None => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
    }
}

fn open_collector_output_from_code(code: i64) -> DigitalValue {
    match code {
        0 => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        1 => DigitalValue::new(DigitalState::One, DigitalStrength::HighZ),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
    }
}

fn open_emitter_output(input: DigitalValue) -> DigitalValue {
    match input.state.logic_level() {
        Some(false) => DigitalValue::new(DigitalState::Zero, DigitalStrength::HighZ),
        Some(true) => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        None => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
    }
}

fn open_emitter_output_from_code(code: i64) -> DigitalValue {
    match code {
        0 => DigitalValue::new(DigitalState::Zero, DigitalStrength::HighZ),
        1 => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
    }
}

fn open_collector_delay(ctx: &CmContext, new_state: i64, prev_state: i64) -> Value {
    match new_state {
        0 => official_digital_delay(ctx, "fall_delay"),
        1 => official_digital_delay(ctx, "open_delay"),
        _ if prev_state == 0 => official_digital_delay(ctx, "open_delay"),
        _ => official_digital_delay(ctx, "fall_delay"),
    }
}

fn open_emitter_delay(ctx: &CmContext, new_state: i64, prev_state: i64) -> Value {
    match new_state {
        0 => official_digital_delay(ctx, "open_delay"),
        1 => official_digital_delay(ctx, "rise_delay"),
        _ if prev_state == 0 => official_digital_delay(ctx, "rise_delay"),
        _ => official_digital_delay(ctx, "open_delay"),
    }
}

/// Open-collector buffer
#[derive(Debug, Default)]
pub struct DigitalOpenCollector;

impl CodeModel for DigitalOpenCollector {
    fn name(&self) -> &str {
        "d_open_c"
    }

    fn description(&self) -> &str {
        "Open-collector digital buffer"
    }

    fn ports(&self) -> &[PortSpec] {
        open_output_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        open_collector_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input_digital("in").unwrap_or_default();
        let output = open_collector_output(input);
        let new_state = digital_logic_code(output);
        let prev = ctx.int_state(0);

        if new_state != prev {
            let delay = open_collector_delay(ctx, new_state, prev);
            set_gate_output_with_unknown_delays(
                ctx,
                "out",
                output,
                open_collector_output_from_code(prev),
                delay,
                Some((
                    official_digital_delay(ctx, "open_delay"),
                    official_digital_delay(ctx, "fall_delay"),
                )),
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

/// Open-emitter buffer
#[derive(Debug, Default)]
pub struct DigitalOpenEmitter;

impl CodeModel for DigitalOpenEmitter {
    fn name(&self) -> &str {
        "d_open_e"
    }

    fn description(&self) -> &str {
        "Open-emitter digital buffer"
    }

    fn ports(&self) -> &[PortSpec] {
        open_output_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        open_emitter_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, INITIAL_GATE_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input_digital("in").unwrap_or_default();
        let output = open_emitter_output(input);
        let new_state = digital_logic_code(output);
        let prev = ctx.int_state(0);

        if new_state != prev {
            let delay = open_emitter_delay(ctx, new_state, prev);
            set_gate_output_with_unknown_delays(
                ctx,
                "out",
                output,
                open_emitter_output_from_code(prev),
                delay,
                Some((
                    official_digital_delay(ctx, "rise_delay"),
                    official_digital_delay(ctx, "open_delay"),
                )),
            );
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::InputValue;

    #[test]
    fn basic_gate_rollbackable_probe_does_not_commit_previous_output_state() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 2);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::zero()]),
        );
        DigitalAnd.init(&mut ctx).expect("d_and init");

        DigitalAnd.evaluate(&mut ctx).expect("d_and initial");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(0), 0);

        ctx.time = 1.0e-9;
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::one()]),
        );
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        DigitalAnd.evaluate(&mut ctx).expect("d_and rollback probe");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 1.0e-9 && event.values == vec![DigitalValue::one()]),
            "rollbackable gate probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(
            ctx.int_state(0),
            0,
            "rollbackable gate probe must not commit previous output state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        DigitalAnd
            .evaluate(&mut ctx)
            .expect("d_and direct after probe");
        assert_eq!(ctx.int_state(0), 1);
    }

    #[test]
    fn tristate_rollbackable_probe_does_not_commit_state_or_strength() {
        let mut ctx = CmContext::new();
        ctx.set_param("delay", 1.0e-9);
        ctx.set_input_digital("in", DigitalValue::one());
        ctx.set_input_digital("enable", DigitalValue::one());
        DigitalTristate.init(&mut ctx).expect("d_tristate init");

        DigitalTristate
            .evaluate(&mut ctx)
            .expect("d_tristate initial");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(0), 1);
        assert_eq!(
            ctx.int_state(1),
            digital_strength_code(DigitalStrength::Strong)
        );

        ctx.time = 1.0e-9;
        ctx.set_input_digital("enable", DigitalValue::zero());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        DigitalTristate
            .evaluate(&mut ctx)
            .expect("d_tristate rollback probe");
        let events = ctx.take_pending_events();
        let high_z_one = DigitalValue::new(DigitalState::One, DigitalStrength::HighZ);
        assert!(
            events
                .iter()
                .any(|event| event.delay == 1.0e-9 && event.values == vec![high_z_one]),
            "rollbackable tristate probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(ctx.int_state(0), 1);
        assert_eq!(
            ctx.int_state(1),
            digital_strength_code(DigitalStrength::Strong),
            "rollbackable tristate probe must not commit strength state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        DigitalTristate
            .evaluate(&mut ctx)
            .expect("d_tristate direct after probe");
        assert_eq!(ctx.int_state(0), 1);
        assert_eq!(
            ctx.int_state(1),
            digital_strength_code(DigitalStrength::HighZ)
        );
    }
}
