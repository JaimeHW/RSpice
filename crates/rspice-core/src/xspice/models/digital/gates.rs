use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};

//=============================================================================
// Basic Gates
//=============================================================================

const INITIAL_GATE_STATE: i64 = i64::MIN;
const OFFICIAL_DIGITAL_DELAY_MIN: Value = 1.0e-12;
const DEFAULT_TRANSPORT_DELAY: i64 = 0;
const DEFAULT_INERTIAL_DELAY: i64 = 1;
const OVERRIDE_TRANSPORT_DELAY: i64 = 2;
const OVERRIDE_INERTIAL_DELAY: i64 = 3;

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

fn official_digital_delay(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("digital gate delay must be finite, got {value}"),
        });
    }
    Ok(value.max(OFFICIAL_DIGITAL_DELAY_MIN))
}

fn gate_boolean_param(ctx: &CmContext, name: &str) -> CmResult<bool> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("digital gate boolean must be finite, got {value}"),
        });
    }
    Ok(value > 0.5)
}

fn gate_inertial_delay_enabled(ctx: &CmContext) -> CmResult<bool> {
    let param_enabled = gate_boolean_param(ctx, "inertial_delay")?;
    Ok(match ctx.digital_delay_type() {
        Some(OVERRIDE_TRANSPORT_DELAY) => false,
        Some(OVERRIDE_INERTIAL_DELAY) => true,
        Some(DEFAULT_TRANSPORT_DELAY) => ctx.param_was_provided("inertial_delay") && param_enabled,
        Some(DEFAULT_INERTIAL_DELAY) => !ctx.param_was_provided("inertial_delay") || param_enabled,
        Some(value) => {
            return Err(CmError::InvalidParameter {
                name: "digital_delay_type".to_string(),
                message: format!(
                    "digital_delay_type must be 0, 1, 2, or 3 for XSPICE gates, got {value}"
                ),
            });
        }
        None => param_enabled,
    })
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

fn init_basic_gate_state(ctx: &mut CmContext) -> CmResult<()> {
    gate_boolean_param(ctx, "inertial_delay")?;
    ctx.allocate_int_states(1);
    ctx.set_int_state(0, INITIAL_GATE_STATE);
    Ok(())
}

fn latest_gate_input_event_time(ctx: &CmContext) -> Option<Value> {
    let scalar = ctx.input_digital_event_time("in");
    let vector = ctx.input_digital_vector_values("in").and_then(|values| {
        (0..values.len())
            .filter_map(|index| ctx.input_digital_vector_event_time("in", index))
            .max_by(|a, b| a.total_cmp(b))
    });
    scalar
        .into_iter()
        .chain(vector)
        .filter(|time| time.is_finite() && *time <= ctx.time)
        .max_by(|a, b| a.total_cmp(b))
}

fn gate_event_rebased_delay(ctx: &CmContext, delay: Value) -> Value {
    if ctx.time == 0.0 {
        return delay;
    }
    let Some(event_time) = latest_gate_input_event_time(ctx) else {
        return delay;
    };
    event_time + delay - ctx.time
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

fn inverter_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("rise_delay", 1e-9)
                .with_description("Rise propagation delay, clamped to official lower limit"),
            ParamSpec::real("fall_delay", 1e-9)
                .with_description("Fall propagation delay, clamped to official lower limit"),
            ParamSpec::boolean("inertial_delay", false),
            ParamSpec::string("family", ""),
            ParamSpec::real("input_load", 1.0e-12),
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

fn base_digital_state(value: DigitalValue) -> DigitalState {
    match value.state.logic_level() {
        Some(false) => DigitalState::Zero,
        Some(true) => DigitalState::One,
        None => DigitalState::Unknown,
    }
}

fn inverted_base_digital_state(value: DigitalValue) -> DigitalState {
    match value.state.logic_level() {
        Some(false) => DigitalState::One,
        Some(true) => DigitalState::Zero,
        None => DigitalState::Unknown,
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
) -> CmResult<()> {
    set_gate_output_with_unknown_delays(
        ctx,
        name,
        value,
        previous,
        gate_event_rebased_delay(ctx, delay),
        Some((rise, fall)),
    )
}

fn set_gate_output_with_unknown_delays(
    ctx: &mut CmContext,
    name: &str,
    value: DigitalValue,
    previous: DigitalValue,
    delay: Value,
    unknown_transition_delays: Option<(Value, Value)>,
) -> CmResult<()> {
    if gate_inertial_delay_enabled(ctx)? {
        ctx.set_output_digital_inertial(name, value, delay, previous, unknown_transition_delays);
    } else {
        ctx.set_output_digital(name, value, delay);
    }
    Ok(())
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
    let state = base_digital_state(input);
    let strength = match enable.state.logic_level() {
        Some(false) => DigitalStrength::HighZ,
        Some(true) => DigitalStrength::Strong,
        None => DigitalStrength::Undetermined,
    };
    DigitalValue::new(state, strength)
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

struct TristatePendingEvents {
    items: [Option<(Value, DigitalValue)>; 2],
    len: usize,
}

impl Default for TristatePendingEvents {
    fn default() -> Self {
        Self {
            items: [None; 2],
            len: 0,
        }
    }
}

impl TristatePendingEvents {
    fn push(&mut self, delay: Value, value: DigitalValue) {
        debug_assert!(
            self.len < self.items.len(),
            "d_tristate can queue at most two intermediate events"
        );
        if let Some(slot) = self.items.get_mut(self.len) {
            *slot = Some((delay, value));
            self.len += 1;
        }
    }
}

fn tristate_push_revert(
    events: &mut TristatePendingEvents,
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
    events.push(delay, tristate_value_from_codes(state, strength));
}

fn set_tristate_output(
    ctx: &mut CmContext,
    value: DigitalValue,
    delay: Value,
    out_state: i64,
    out_strength: i64,
) -> CmResult<()> {
    let mut state = digital_logic_code(value);
    let mut strength = digital_strength_code(value.strength);

    if state == out_state && strength == out_strength {
        return Ok(());
    }

    if !gate_inertial_delay_enabled(ctx)? || !ctx.is_transient() {
        ctx.set_output_digital("out", value, delay);
        gate_set_int_state(ctx, 0, state);
        gate_set_int_state(ctx, 1, strength);
        return Ok(());
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
    let mut events = TristatePendingEvents::default();

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
                events.push((second_time - time) / 2.0, reversion);
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
                events.push(
                    (first_time - time) / 2.0,
                    tristate_value_from_codes(state_prev, strength_prev),
                );
                output_delay = second_time - time;
            }
            TristateInertialControl::Revert => {
                state = state_prev;
                strength = strength_prev;
                output_delay = (first_time - time) / 2.0;
            }
            TristateInertialControl::Both => {
                let reversion = tristate_value_from_codes(state_prev, strength_prev);
                events.push((first_time - time) / 2.0, reversion);
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
                events.push((first_time - time) / 2.0, reversion);
                let restoration = if d_first {
                    tristate_value_from_codes(state_prev, out_strength)
                } else {
                    tristate_value_from_codes(out_state, strength_prev)
                };
                events.push(second_time - time, restoration);
            }
            TristateInertialControl::Revert | TristateInertialControl::Both => {
                events.push(
                    (first_time - time) / 2.0,
                    tristate_value_from_codes(state_prev, strength_prev),
                );
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
    for index in 0..events.len {
        if let Some((event_delay, event_value)) = events.items[index] {
            ctx.set_output_digital("out", event_value, event_delay);
        }
    }
    ctx.set_output_digital(
        "out",
        tristate_value_from_codes(state, strength),
        output_delay,
    );
    gate_set_int_state(ctx, 0, state);
    gate_set_int_state(ctx, 1, strength);
    Ok(())
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

            fn can_skip_unchanged_event_inputs(&self) -> bool {
                true
            }

            fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
                init_basic_gate_state(ctx)
            }

            fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
                let rise = official_digital_delay(ctx, "rise_delay")?;
                let fall = official_digital_delay(ctx, "fall_delay")?;
                let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
                let prev = ctx.int_state(0);

                let result: DigitalState = $op(inputs);
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
                    )?;
                }
                gate_set_int_state(ctx, 0, new_state);
                Ok(())
            }
        }
    };
}

fn and_op(inputs: &[DigitalValue]) -> DigitalState {
    let mut unknown = false;
    for input in inputs {
        match input.state.logic_level() {
            Some(false) => return DigitalState::Zero,
            Some(true) => {}
            None => unknown = true,
        }
    }

    if unknown {
        DigitalState::Unknown
    } else {
        DigitalState::One
    }
}

fn or_op(inputs: &[DigitalValue]) -> DigitalState {
    let mut unknown = false;
    for input in inputs {
        match input.state.logic_level() {
            Some(true) => return DigitalState::One,
            Some(false) => {}
            None => unknown = true,
        }
    }

    if unknown {
        DigitalState::Unknown
    } else {
        DigitalState::Zero
    }
}

fn xor_op(inputs: &[DigitalValue]) -> DigitalState {
    let mut parity = false;
    for input in inputs {
        let Some(bit) = input.state.logic_level() else {
            return DigitalState::Unknown;
        };
        parity ^= bit;
    }

    if parity {
        DigitalState::One
    } else {
        DigitalState::Zero
    }
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay")?;
        let fall = official_digital_delay(ctx, "fall_delay")?;
        let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
        let prev = ctx.int_state(0);

        let result = and_op(inputs).invert();
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
            )?;
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay")?;
        let fall = official_digital_delay(ctx, "fall_delay")?;
        let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
        let prev = ctx.int_state(0);
        let result = or_op(inputs).invert();
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
            )?;
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay")?;
        let fall = official_digital_delay(ctx, "fall_delay")?;
        let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
        let prev = ctx.int_state(0);
        let result = xor_op(inputs).invert();
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
            )?;
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
        inverter_params()
    }

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay")?;
        let fall = official_digital_delay(ctx, "fall_delay")?;
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = inverted_base_digital_state(input);
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
            )?;
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
        gate_params()
    }

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = official_digital_delay(ctx, "rise_delay")?;
        let fall = official_digital_delay(ctx, "fall_delay")?;
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = base_digital_state(input);
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
            )?;
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        gate_boolean_param(ctx, "inertial_delay")?;
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
        let delay = official_digital_delay(ctx, "delay")?;
        let input = ctx.input_digital("in").unwrap_or_default();
        let enable = ctx.input_digital("enable").unwrap_or_default();

        let result = tristate_output(input, enable);
        let prev_state = ctx.int_state(0);
        let prev_strength = ctx.int_state(1);

        set_tristate_output(ctx, result, delay, prev_state, prev_strength)?;
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
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

fn open_collector_delay(ctx: &CmContext, new_state: i64, prev_state: i64) -> CmResult<Value> {
    let name = match new_state {
        0 => "fall_delay",
        1 => "open_delay",
        _ if prev_state == 0 => "open_delay",
        _ => "fall_delay",
    };
    official_digital_delay(ctx, name)
}

fn open_emitter_delay(ctx: &CmContext, new_state: i64, prev_state: i64) -> CmResult<Value> {
    let name = match new_state {
        0 => "open_delay",
        1 => "rise_delay",
        _ if prev_state == 0 => "rise_delay",
        _ => "open_delay",
    };
    official_digital_delay(ctx, name)
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input_digital("in").unwrap_or_default();
        let output = open_collector_output(input);
        let new_state = digital_logic_code(output);
        let prev = ctx.int_state(0);

        if new_state != prev {
            let delay = open_collector_delay(ctx, new_state, prev)?;
            set_gate_output_with_unknown_delays(
                ctx,
                "out",
                output,
                open_collector_output_from_code(prev),
                delay,
                Some((
                    official_digital_delay(ctx, "open_delay")?,
                    official_digital_delay(ctx, "fall_delay")?,
                )),
            )?;
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

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        init_basic_gate_state(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input_digital("in").unwrap_or_default();
        let output = open_emitter_output(input);
        let new_state = digital_logic_code(output);
        let prev = ctx.int_state(0);

        if new_state != prev {
            let delay = open_emitter_delay(ctx, new_state, prev)?;
            set_gate_output_with_unknown_delays(
                ctx,
                "out",
                output,
                open_emitter_output_from_code(prev),
                delay,
                Some((
                    official_digital_delay(ctx, "rise_delay")?,
                    official_digital_delay(ctx, "open_delay")?,
                )),
            )?;
        }
        gate_set_int_state(ctx, 0, new_state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::InputValue;
    use crate::xspice::{ParamType, PortDirection};

    fn param_summary(model: &dyn CodeModel) -> Vec<(&str, ParamType, Value, Option<&str>)> {
        model
            .parameters()
            .iter()
            .map(|param| {
                (
                    param.name.as_str(),
                    param.param_type,
                    param.default,
                    param.string_default.as_deref(),
                )
            })
            .collect()
    }

    fn assert_digital_ports(
        model: &dyn CodeModel,
        expected: &[(&str, PortDirection, bool, Option<usize>)],
    ) {
        let ports = model.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|(name, _, _, _)| *name)
                .collect::<Vec<_>>()
        );
        for (port, (_, direction, is_vector, min_len)) in ports.iter().zip(expected) {
            assert_eq!(port.direction, *direction, "{} direction", port.name);
            assert_eq!(
                port.default_type,
                PortType::Digital,
                "{} default type",
                port.name
            );
            assert_eq!(
                port.allowed_types,
                vec![PortType::Digital],
                "{} allowed types",
                port.name
            );
            assert_eq!(port.is_vector, *is_vector, "{} vector flag", port.name);
            assert!(!port.null_allowed, "{} nullability", port.name);
            assert_eq!(port.vector_min_len, *min_len, "{} min length", port.name);
            assert_eq!(port.vector_max_len, None, "{} max length", port.name);
        }
    }

    fn official_gate_params() -> Vec<(&'static str, ParamType, Value, Option<&'static str>)> {
        vec![
            ("rise_delay", ParamType::Real, 1.0e-9, None),
            ("fall_delay", ParamType::Real, 1.0e-9, None),
            ("input_load", ParamType::Real, 1.0e-12, None),
            ("family", ParamType::String, 0.0, Some("")),
            ("inertial_delay", ParamType::Boolean, 0.0, None),
        ]
    }

    #[test]
    fn n_input_gate_metadata_matches_ngspice46_interfaces() {
        for model in [
            &DigitalAnd as &dyn CodeModel,
            &DigitalNand,
            &DigitalOr,
            &DigitalNor,
            &DigitalXor,
            &DigitalXnor,
        ] {
            assert_digital_ports(
                model,
                &[
                    ("in", PortDirection::In, true, Some(2)),
                    ("out", PortDirection::Out, false, None),
                ],
            );
            assert_eq!(
                param_summary(model),
                official_gate_params(),
                "{}",
                model.name()
            );
        }
    }

    #[test]
    fn one_bit_gate_metadata_matches_ngspice46_interfaces() {
        assert_digital_ports(
            &DigitalInverter,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalInverter),
            vec![
                ("rise_delay", ParamType::Real, 1.0e-9, None),
                ("fall_delay", ParamType::Real, 1.0e-9, None),
                ("inertial_delay", ParamType::Boolean, 0.0, None),
                ("family", ParamType::String, 0.0, Some("")),
                ("input_load", ParamType::Real, 1.0e-12, None),
            ]
        );

        assert_digital_ports(
            &DigitalBuffer,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(param_summary(&DigitalBuffer), official_gate_params());
    }

    #[test]
    fn tristate_metadata_matches_ngspice46_interface() {
        assert_digital_ports(
            &DigitalTristate,
            &[
                ("in", PortDirection::In, false, None),
                ("enable", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalTristate),
            vec![
                ("delay", ParamType::Real, 1.0e-9, None),
                ("input_load", ParamType::Real, 1.0e-12, None),
                ("enable_load", ParamType::Real, 1.0e-12, None),
                ("inertial_delay", ParamType::Boolean, 0.0, None),
                ("family", ParamType::String, 0.0, Some("")),
            ]
        );
    }

    #[test]
    fn pull_resistor_metadata_matches_ngspice46_interfaces() {
        for model in [&DigitalPullup as &dyn CodeModel, &DigitalPulldown] {
            assert_digital_ports(model, &[("out", PortDirection::Out, false, None)]);
            assert_eq!(
                param_summary(model),
                vec![("load", ParamType::Real, 1.0e-12, None)],
                "{}",
                model.name()
            );
        }
    }

    #[test]
    fn open_output_metadata_matches_ngspice46_interfaces() {
        assert_digital_ports(
            &DigitalOpenCollector,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalOpenCollector),
            vec![
                ("open_delay", ParamType::Real, 1.0e-9, None),
                ("fall_delay", ParamType::Real, 1.0e-9, None),
                ("input_load", ParamType::Real, 1.0e-12, None),
                ("family", ParamType::String, 0.0, Some("")),
                ("inertial_delay", ParamType::Boolean, 0.0, None),
            ]
        );

        assert_digital_ports(
            &DigitalOpenEmitter,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalOpenEmitter),
            vec![
                ("rise_delay", ParamType::Real, 1.0e-9, None),
                ("open_delay", ParamType::Real, 1.0e-9, None),
                ("input_load", ParamType::Real, 1.0e-12, None),
                ("family", ParamType::String, 0.0, Some("")),
                ("inertial_delay", ParamType::Boolean, 0.0, None),
            ]
        );
    }

    fn logic_values(states: &[DigitalState]) -> Vec<DigitalValue> {
        states
            .iter()
            .copied()
            .map(|state| DigitalValue::new(state, DigitalStrength::Strong))
            .collect()
    }

    fn assert_invalid_param(result: CmResult<()>, expected_name: &str) {
        match result {
            Err(CmError::InvalidParameter { name, .. }) => assert_eq!(name, expected_name),
            other => panic!("expected InvalidParameter for {expected_name}, got {other:?}"),
        }
    }

    #[test]
    fn digital_gates_reject_nonfinite_delay_params() {
        let mut buffer_ctx = CmContext::new();
        buffer_ctx.set_param("rise_delay", f64::INFINITY);
        buffer_ctx.set_input_digital("in", DigitalValue::one());
        DigitalBuffer.init(&mut buffer_ctx).expect("d_buffer init");
        assert_invalid_param(DigitalBuffer.evaluate(&mut buffer_ctx), "rise_delay");

        let mut tristate_ctx = CmContext::new();
        tristate_ctx.set_param("delay", f64::NAN);
        tristate_ctx.set_input_digital("in", DigitalValue::one());
        tristate_ctx.set_input_digital("enable", DigitalValue::one());
        DigitalTristate
            .init(&mut tristate_ctx)
            .expect("d_tristate init");
        assert_invalid_param(DigitalTristate.evaluate(&mut tristate_ctx), "delay");

        let mut open_ctx = CmContext::new();
        open_ctx.set_param("open_delay", f64::INFINITY);
        open_ctx.set_input_digital("in", DigitalValue::one());
        DigitalOpenCollector
            .init(&mut open_ctx)
            .expect("d_open_c init");
        assert_invalid_param(DigitalOpenCollector.evaluate(&mut open_ctx), "open_delay");
    }

    #[test]
    fn digital_gates_reject_nonfinite_inertial_delay_param() {
        for model in [
            &DigitalBuffer as &dyn CodeModel,
            &DigitalAnd,
            &DigitalTristate,
            &DigitalOpenCollector,
            &DigitalOpenEmitter,
        ] {
            let mut ctx = CmContext::new();
            ctx.set_param("inertial_delay", f64::NAN);
            assert_invalid_param(model.init(&mut ctx), "inertial_delay");
        }
    }

    #[test]
    fn digital_delay_type_controls_gate_inertial_policy() {
        let ctx = CmContext::new();
        assert!(!gate_inertial_delay_enabled(&ctx).expect("default transport"));

        let mut default_inertial = CmContext::new();
        default_inertial.set_digital_delay_type(Some(DEFAULT_INERTIAL_DELAY));
        assert!(gate_inertial_delay_enabled(&default_inertial).expect("global default inertial"));

        default_inertial.set_param("inertial_delay", 0.0);
        default_inertial.mark_param_provided("inertial_delay");
        assert!(
            !gate_inertial_delay_enabled(&default_inertial).expect("explicit param wins default")
        );

        let mut override_transport = CmContext::new();
        override_transport.set_digital_delay_type(Some(OVERRIDE_TRANSPORT_DELAY));
        override_transport.set_param("inertial_delay", 1.0);
        override_transport.mark_param_provided("inertial_delay");
        assert!(
            !gate_inertial_delay_enabled(&override_transport).expect("override transport wins")
        );

        let mut override_inertial = CmContext::new();
        override_inertial.set_digital_delay_type(Some(OVERRIDE_INERTIAL_DELAY));
        override_inertial.set_param("inertial_delay", 0.0);
        override_inertial.mark_param_provided("inertial_delay");
        assert!(gate_inertial_delay_enabled(&override_inertial).expect("override inertial wins"));
    }

    #[test]
    fn basic_gate_fast_ops_match_digital_state_truth_tables() {
        let states = [
            DigitalState::Zero,
            DigitalState::One,
            DigitalState::Unknown,
            DigitalState::ZeroR,
            DigitalState::OneR,
            DigitalState::UnknownR,
            DigitalState::ZeroZ,
            DigitalState::OneZ,
            DigitalState::UnknownZ,
            DigitalState::HighZ,
        ];

        for first in states {
            for second in states {
                for third in states {
                    let state_inputs = [first, second, third];
                    let inputs = logic_values(&state_inputs);

                    let expected_and = state_inputs
                        .iter()
                        .fold(DigitalState::One, |acc, state| acc.and(state));
                    let expected_or = state_inputs
                        .iter()
                        .fold(DigitalState::Zero, |acc, state| acc.or(state));
                    let expected_xor = state_inputs
                        .iter()
                        .fold(DigitalState::Zero, |acc, state| acc.xor(state));

                    assert_eq!(and_op(&inputs), expected_and);
                    assert_eq!(or_op(&inputs), expected_or);
                    assert_eq!(xor_op(&inputs), expected_xor);
                }
            }
        }
    }

    #[test]
    fn basic_gate_fast_ops_preserve_empty_vector_identities() {
        assert_eq!(and_op(&[]), DigitalState::One);
        assert_eq!(or_op(&[]), DigitalState::Zero);
        assert_eq!(xor_op(&[]), DigitalState::Zero);
    }

    #[test]
    fn one_bit_gates_normalize_extended_input_state_like_ngspice() {
        let mut buffer_ctx = CmContext::new();
        buffer_ctx.set_input_digital(
            "in",
            DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        );
        DigitalBuffer.init(&mut buffer_ctx).expect("d_buffer init");
        DigitalBuffer
            .evaluate(&mut buffer_ctx)
            .expect("d_buffer evaluates");
        assert_eq!(
            buffer_ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::one())
        );

        let mut inverter_ctx = CmContext::new();
        inverter_ctx.set_input_digital(
            "in",
            DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ),
        );
        DigitalInverter
            .init(&mut inverter_ctx)
            .expect("d_inverter init");
        DigitalInverter
            .evaluate(&mut inverter_ctx)
            .expect("d_inverter evaluates");
        assert_eq!(
            inverter_ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::one())
        );

        let mut high_z_ctx = CmContext::new();
        high_z_ctx.set_input_digital("in", DigitalValue::high_z());
        DigitalBuffer.init(&mut high_z_ctx).expect("d_buffer init");
        DigitalBuffer
            .evaluate(&mut high_z_ctx)
            .expect("d_buffer evaluates");
        assert_eq!(
            high_z_ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::unknown())
        );
    }

    #[test]
    fn tristate_normalizes_input_state_and_uses_enable_strength_like_ngspice() {
        let mut enabled_ctx = CmContext::new();
        enabled_ctx.set_input_digital("in", DigitalValue::high_z());
        enabled_ctx.set_input_digital("enable", DigitalValue::one());
        DigitalTristate
            .init(&mut enabled_ctx)
            .expect("d_tristate init");
        DigitalTristate
            .evaluate(&mut enabled_ctx)
            .expect("d_tristate evaluates");
        assert_eq!(
            enabled_ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::new(
                DigitalState::Unknown,
                DigitalStrength::Strong
            ))
        );

        let mut disabled_ctx = CmContext::new();
        disabled_ctx.set_input_digital(
            "in",
            DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        );
        disabled_ctx.set_input_digital("enable", DigitalValue::zero());
        DigitalTristate
            .init(&mut disabled_ctx)
            .expect("d_tristate init");
        DigitalTristate
            .evaluate(&mut disabled_ctx)
            .expect("d_tristate evaluates");
        assert_eq!(
            disabled_ctx.output_digital_vector_value("out", 0),
            Some(DigitalValue::new(DigitalState::One, DigitalStrength::HighZ))
        );
    }

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
