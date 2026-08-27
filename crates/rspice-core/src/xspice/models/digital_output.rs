//! Xyce-compatible analog output companion for the PSpice/Xyce U TFF.
//!
//! Xyce's `DIG` model is an event-driven logic primitive with an analog
//! interface.  Each output is connected to both the low and high power rails
//! through state-dependent resistors and has one capacitor to each rail.  The
//! ordinary XSPICE `d_tff` model intentionally exposes digital outputs and is
//! therefore followed by an ideal DAC bridge; that bridge cannot represent
//! the finite output impedance used by Xyce.  `XyceDTff` keeps the sequential
//! state in this model and stamps the physical two-rail output directly.
//!
//! The model is deliberately independent of the netlist frontend.  A
//! frontend may lower a six-terminal U-device to the ports in the order
//! `dpwr dgnd t clk q qbar`, with the first four ports as inputs and the last
//! two as analog in/out conductance ports.

use crate::xspice::{
    AnalogTransition, CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec,
    PortDirection, PortSpec, PortType,
};
use crate::{Complex64, Value};

const MODEL_NAME: &str = "xyce_d_tff";
const RESISTANCE_MIN: Value = 1.0e-12;
const TIME_MIN: Value = 0.0;
const CAPACITANCE_MIN: Value = 0.0;

// Floating state layout.  The state array is checkpoint-serializable and is
// intentionally limited to accepted logical/analog history; no host objects
// are retained by this model.
const STATE_Q: usize = 0;
const STATE_CLK: usize = 1;
const STATE_LAST_CLK_EVENT: usize = 2;
const STATE_Q_TRANSITION_START: usize = 3;
const STATE_Q_TRANSITION_FROM: usize = 4;
const STATE_PENDING_Q: usize = 5;
const STATE_PENDING_TIME: usize = 6;
const STATE_Q_PREV_LOW_VOLTAGE: usize = 7;
const STATE_Q_PREV_HIGH_VOLTAGE: usize = 8;
const STATE_QBAR_PREV_LOW_VOLTAGE: usize = 9;
const STATE_QBAR_PREV_HIGH_VOLTAGE: usize = 10;
const STATE_T: usize = 11;
const STATE_CLK_VOLTAGE: usize = 12;
const STATE_T_VOLTAGE: usize = 13;
const STATE_T_TRANSITION_TIME: usize = 14;
const STATE_COUNT: usize = 15;

const Q_UNKNOWN: Value = -1.0;
const Q_LOW: Value = 0.0;
const Q_HIGH: Value = 1.0;
const Q_PENDING_NONE: Value = -1.0;

#[derive(Debug, Clone, Copy)]
struct DigParams {
    clo: Value,
    chi: Value,
    cload: Value,
    rload: Value,
    s0_rlo: Value,
    s0_rhi: Value,
    s0_tsw: Value,
    s0_vlo: Value,
    s0_vhi: Value,
    s1_rlo: Value,
    s1_rhi: Value,
    s1_tsw: Value,
    s1_vlo: Value,
    s1_vhi: Value,
    delay: Value,
}

#[derive(Debug, Clone, Copy)]
struct OutputResistances {
    low: Value,
    high: Value,
}

fn finite_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("{MODEL_NAME} parameter must be finite, got {value}"),
        })
    }
}

fn nonnegative_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = finite_param(ctx, name)?;
    if value < TIME_MIN {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("{MODEL_NAME} parameter must be non-negative, got {value}"),
        });
    }
    Ok(value)
}

fn capacitance_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = finite_param(ctx, name)?;
    if value < CAPACITANCE_MIN {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("{MODEL_NAME} capacitance must be non-negative, got {value}"),
        });
    }
    Ok(value)
}

fn resistance_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = finite_param(ctx, name)?;
    if value <= 0.0 {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("{MODEL_NAME} resistance must be positive, got {value}"),
        });
    }
    Ok(value.max(RESISTANCE_MIN))
}

fn dig_params(ctx: &CmContext) -> CmResult<DigParams> {
    let params = DigParams {
        clo: capacitance_param(ctx, "clo")?,
        chi: capacitance_param(ctx, "chi")?,
        cload: capacitance_param(ctx, "cload")?,
        rload: resistance_param(ctx, "rload")?,
        s0_rlo: resistance_param(ctx, "s0rlo")?,
        s0_rhi: resistance_param(ctx, "s0rhi")?,
        s0_tsw: nonnegative_param(ctx, "s0tsw")?,
        s0_vlo: finite_param(ctx, "s0vlo")?,
        s0_vhi: finite_param(ctx, "s0vhi")?,
        s1_rlo: resistance_param(ctx, "s1rlo")?,
        s1_rhi: resistance_param(ctx, "s1rhi")?,
        s1_tsw: nonnegative_param(ctx, "s1tsw")?,
        s1_vlo: finite_param(ctx, "s1vlo")?,
        s1_vhi: finite_param(ctx, "s1vhi")?,
        delay: nonnegative_param(ctx, "delay")?,
    };
    Ok(params)
}

fn node_row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

/// Stamp one passive conductance between arbitrary differential terminal
/// pairs.  The signs describe `V(a)-V(b)`, where each pair is itself
/// differential.  Ground terminals are omitted from the matrix.
fn stamp_between(ctx: &mut CmContext, a: (usize, usize), b: (usize, usize), conductance: Value) {
    if conductance == 0.0 || !conductance.is_finite() {
        return;
    }

    let terminals = [(a.0, 1.0), (a.1, -1.0), (b.0, -1.0), (b.1, 1.0)];
    for (row_node, row_sign) in terminals {
        let Some(row) = node_row(row_node) else {
            continue;
        };
        for (col_node, col_sign) in terminals {
            let Some(col) = node_row(col_node) else {
                continue;
            };
            ctx.stamp_conductance(row, col, conductance * row_sign * col_sign);
        }
    }
}

fn stamp_between_rhs(ctx: &mut CmContext, a: (usize, usize), b: (usize, usize), current: Value) {
    if !current.is_finite() {
        return;
    }
    let terminals = [(a.0, 1.0), (a.1, -1.0), (b.0, -1.0), (b.1, 1.0)];
    for (node, sign) in terminals {
        if let Some(row) = node_row(node) {
            ctx.stamp_rhs(row, -sign * current);
        }
    }
}

fn queue_between_static(
    ctx: &mut CmContext,
    a: (usize, usize),
    b: (usize, usize),
    conductance: Value,
) {
    if ctx.evaluation_phase() != EvaluationPhase::AcceptedStep {
        return;
    }
    if conductance == 0.0 || !conductance.is_finite() {
        return;
    }
    let terminals = [(a.0, 1.0), (a.1, -1.0), (b.0, -1.0), (b.1, 1.0)];
    for (row_node, row_sign) in terminals {
        let Some(row) = node_row(row_node) else {
            continue;
        };
        for (col_node, col_sign) in terminals {
            let Some(col) = node_row(col_node) else {
                continue;
            };
            ctx.stamp_static_conductance(row, col, conductance * row_sign * col_sign);
        }
    }
}

fn queue_between_rhs_static(
    ctx: &mut CmContext,
    a: (usize, usize),
    b: (usize, usize),
    current: Value,
) {
    if ctx.evaluation_phase() != EvaluationPhase::AcceptedStep {
        return;
    }
    if !current.is_finite() {
        return;
    }
    let terminals = [(a.0, 1.0), (a.1, -1.0), (b.0, -1.0), (b.1, 1.0)];
    for (node, sign) in terminals {
        if let Some(row) = node_row(node) {
            ctx.stamp_static_rhs(row, -sign * current);
        }
    }
}

fn q_state(value: Value) -> Option<bool> {
    if (value - Q_HIGH).abs() <= 0.25 {
        Some(true)
    } else if value.abs() <= 0.25 {
        Some(false)
    } else {
        None
    }
}

fn state_resistances(state: Option<bool>, params: DigParams) -> OutputResistances {
    match state {
        Some(false) => OutputResistances {
            low: params.s0_rlo,
            high: params.s0_rhi,
        },
        Some(true) => OutputResistances {
            low: params.s1_rlo,
            high: params.s1_rhi,
        },
        None => OutputResistances {
            // Xyce's X state drives both rails.  The arithmetic mean of the
            // state resistances is the continuous midpoint used during an
            // unresolved output transition.
            low: 0.5 * (params.s0_rlo + params.s1_rlo),
            high: 0.5 * (params.s0_rhi + params.s1_rhi),
        },
    }
}

fn transition_duration(target: bool, params: DigParams) -> Value {
    if target { params.s1_tsw } else { params.s0_tsw }
}

fn interpolated_resistances(
    state: Option<bool>,
    from: Option<bool>,
    transition_start: Value,
    time: Value,
    params: DigParams,
) -> OutputResistances {
    let Some(target) = state else {
        return state_resistances(None, params);
    };
    let Some(from) = from else {
        return state_resistances(Some(target), params);
    };
    if from == target {
        return state_resistances(Some(target), params);
    }

    let duration = transition_duration(target, params);
    let progress = if duration <= 0.0 || transition_start <= 0.0 {
        1.0
    } else {
        ((time - transition_start) / duration).clamp(0.0, 1.0)
    };
    let from_r = state_resistances(Some(from), params);
    let to_r = state_resistances(Some(target), params);
    OutputResistances {
        low: from_r.low + progress * (to_r.low - from_r.low),
        high: from_r.high + progress * (to_r.high - from_r.high),
    }
}

fn transition_time_for_state(target: bool, start: Value, params: DigParams) -> Option<Value> {
    let duration = transition_duration(target, params);
    (start > 0.0 && duration > 0.0 && start.is_finite())
        .then_some(start + duration)
        .filter(|time| time.is_finite())
}

fn transition_time_at_threshold(
    ctx: &CmContext,
    previous_voltage: Value,
    voltage: Value,
    threshold: Value,
) -> Value {
    let timestep = ctx.timestep;
    let denominator = voltage - previous_voltage;
    if !timestep.is_finite()
        || timestep <= 0.0
        || !previous_voltage.is_finite()
        || !voltage.is_finite()
        || !threshold.is_finite()
        || denominator.abs() <= Value::EPSILON
    {
        return ctx.time;
    }
    let delta = timestep * (voltage - threshold) / denominator;
    let transition_time = ctx.time - delta;
    if transition_time.is_finite() {
        transition_time.clamp(ctx.time_prev.min(ctx.time), ctx.time_prev.max(ctx.time))
    } else {
        ctx.time
    }
}

fn q_ports() -> &'static [PortSpec] {
    use std::sync::OnceLock;
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec::input("dpwr", PortType::Voltage)
                .with_description("Positive digital power rail"),
            PortSpec::input("dgnd", PortType::Voltage)
                .with_description("Digital ground/reference rail"),
            PortSpec::input("t", PortType::Voltage)
                .with_description("Analog toggle input referenced to DGND"),
            PortSpec::input("clk", PortType::Voltage)
                .with_description("Analog rising-edge clock input referenced to DGND"),
            PortSpec {
                name: "q".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Finite-impedance Q output".to_string(),
            },
            PortSpec {
                name: "qbar".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Finite-impedance complemented Q output".to_string(),
            },
        ]
    })
}

fn q_parameters() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("clo", 1.0e-6),
            ParamSpec::real("chi", 1.0e-6),
            ParamSpec::real("cload", 1.0e-6),
            ParamSpec::real("rload", 1000.0),
            ParamSpec::real("s0rlo", 100.0),
            ParamSpec::real("s0rhi", 100.0),
            ParamSpec::real("s0tsw", 1.0e-8),
            ParamSpec::real("s0vlo", -1.5),
            ParamSpec::real("s0vhi", 1.7),
            ParamSpec::real("s1rlo", 100.0),
            ParamSpec::real("s1rhi", 100.0),
            ParamSpec::real("s1tsw", 1.0e-8),
            ParamSpec::real("s1vlo", 0.9),
            ParamSpec::real("s1vhi", 7.0),
            ParamSpec::real("delay", 1.0e-8),
        ]
    })
}

/// Xyce-compatible finite-output T flip-flop.
#[derive(Debug, Default)]
pub struct XyceDTff;

impl XyceDTff {
    fn set_state_if_committed(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn input_logic_state(
        voltage: Value,
        previous: Option<bool>,
        params: DigParams,
    ) -> Option<bool> {
        if !voltage.is_finite() {
            return None;
        }
        match previous {
            Some(false) if voltage > params.s0_vhi && voltage > params.s1_vlo => Some(true),
            Some(true) if voltage < params.s1_vlo && voltage < params.s0_vhi => Some(false),
            Some(previous) => Some(previous),
            None => Some(voltage >= params.s0_vhi),
        }
    }

    fn input_transition_time(
        ctx: &CmContext,
        previous_voltage: Value,
        voltage: Value,
        next_state: bool,
        params: DigParams,
    ) -> Value {
        let timestep = ctx.timestep;
        let denominator = voltage - previous_voltage;
        if !timestep.is_finite()
            || timestep <= 0.0
            || !previous_voltage.is_finite()
            || !voltage.is_finite()
            || denominator.abs() <= Value::EPSILON
        {
            return ctx.time;
        }

        // The Xyce DIG device linearly interpolates the input crossing within
        // the accepted solver step.  A rising transition must clear both
        // high thresholds; a falling transition must cross below both low
        // thresholds.
        let threshold = if next_state {
            params.s0_vhi.max(params.s1_vlo)
        } else {
            params.s1_vlo.min(params.s0_vhi)
        };
        let delta = timestep * (voltage - threshold) / denominator;
        let transition_time = ctx.time - delta;
        if transition_time.is_finite() {
            transition_time.clamp(ctx.time_prev.min(ctx.time), ctx.time_prev.max(ctx.time))
        } else {
            ctx.time
        }
    }

    fn sample_input(
        ctx: &mut CmContext,
        name: &str,
        state_index: usize,
        voltage_index: usize,
        transition_index: usize,
        params: DigParams,
    ) -> (Option<bool>, Value, bool) {
        let voltage = ctx.input(name) - ctx.input("dgnd");
        let previous_state = q_state(ctx.state_prev(state_index));
        let previous_voltage = ctx.state_prev(voltage_index);
        let mut state = Self::input_logic_state(voltage, previous_state, params);
        let mut transition_time = ctx.state_prev(transition_index);
        let changed = previous_state.is_some() && state.is_some() && previous_state != state;

        if ctx.is_dc() {
            state = Self::input_logic_state(voltage, None, params);
            transition_time = 0.0;
        } else if changed {
            transition_time = Self::input_transition_time(
                ctx,
                previous_voltage,
                voltage,
                state.expect("changed input has a decoded state"),
                params,
            );
        }

        if let Some(state) = state {
            Self::set_state_if_committed(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
        }
        if voltage.is_finite() {
            Self::set_state_if_committed(ctx, voltage_index, voltage);
        }
        if transition_time.is_finite() {
            Self::set_state_if_committed(ctx, transition_index, transition_time);
        }

        (state, transition_time, changed)
    }

    fn update_logic_state(
        &self,
        ctx: &mut CmContext,
        params: DigParams,
    ) -> (Option<bool>, Option<bool>, Value, Option<bool>) {
        let (t, t_transition_time, _) = Self::sample_input(
            ctx,
            "t",
            STATE_T,
            STATE_T_VOLTAGE,
            STATE_T_TRANSITION_TIME,
            params,
        );
        let (clk, clk_transition_time, clk_changed) = Self::sample_input(
            ctx,
            "clk",
            STATE_CLK,
            STATE_CLK_VOLTAGE,
            STATE_LAST_CLK_EVENT,
            params,
        );
        let mut q = q_state(ctx.state_prev(STATE_Q));
        if q.is_none() {
            q = t;
        }
        let old_clk = q_state(ctx.state_prev(STATE_CLK));
        let mut transition_start = ctx.state_prev(STATE_Q_TRANSITION_START);
        let mut transition_from = q_state(ctx.state_prev(STATE_Q_TRANSITION_FROM));
        if q.is_some() && transition_from.is_none() {
            transition_start = 0.0;
            transition_from = q;
        }
        let mut pending_q = q_state(ctx.state_prev(STATE_PENDING_Q));

        if ctx.is_dc() {
            q = t;
            transition_start = 0.0;
            transition_from = q;
            pending_q = None;
        } else if clk_changed && old_clk == Some(false) && clk == Some(true) && t == Some(true) {
            let input_transition_time = clk_transition_time.max(t_transition_time);
            let target = !q.unwrap_or(false);
            let pending_time = input_transition_time + params.delay;
            if pending_time <= ctx.time + 1.0e-18 {
                transition_from = q;
                q = Some(target);
                transition_start = pending_time;
                pending_q = None;
            } else {
                pending_q = Some(target);
                Self::set_state_if_committed(ctx, STATE_PENDING_TIME, pending_time);
                ctx.request_breakpoint(pending_time);
            }
        }

        if let Some(target) = pending_q {
            let pending_time = ctx.state_prev(STATE_PENDING_TIME);
            if pending_time.is_finite() && pending_time <= ctx.time + 1.0e-18 {
                transition_from = q;
                q = Some(target);
                transition_start = pending_time;
                pending_q = None;
                Self::set_state_if_committed(ctx, STATE_PENDING_TIME, f64::NAN);
            }
        }

        if let Some(q) = q {
            Self::set_state_if_committed(ctx, STATE_Q, if q { Q_HIGH } else { Q_LOW });
            Self::set_state_if_committed(
                ctx,
                STATE_Q_TRANSITION_FROM,
                transition_from.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
            );
        }
        Self::set_state_if_committed(
            ctx,
            STATE_CLK,
            clk.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::set_state_if_committed(ctx, STATE_Q_TRANSITION_START, transition_start);
        Self::set_state_if_committed(
            ctx,
            STATE_PENDING_Q,
            pending_q.map_or(Q_PENDING_NONE, |value| if value { Q_HIGH } else { Q_LOW }),
        );

        (q, q.map(|value| !value), transition_start, transition_from)
    }

    fn stamp_output(
        ctx: &mut CmContext,
        port_name: &str,
        state: Option<bool>,
        previous_low_voltage_state: usize,
        previous_high_voltage_state: usize,
        params: DigParams,
        transition_start: Value,
        transition_from: Option<bool>,
    ) {
        let output_pair = ctx.port_node_pair(port_name).unwrap_or((0, 0));
        let dpwr_pair = ctx.port_node_pair("dpwr").unwrap_or((0, 0));
        let dgnd_pair = ctx.port_node_pair("dgnd").unwrap_or((0, 0));
        let resistances =
            interpolated_resistances(state, transition_from, transition_start, ctx.time, params);
        let g_low = 1.0 / resistances.low;
        let g_high = 1.0 / resistances.high;
        if ctx.is_ac() {
            // AC uses output_input_ac_partials below; direct real-valued
            // stamps are intentionally not queued in a complex solve.
            ctx.set_output_with_partial(port_name, 0.0, g_low + g_high);
            return;
        }

        stamp_between(ctx, output_pair, dgnd_pair, g_low);
        stamp_between(ctx, output_pair, dpwr_pair, g_high);

        if ctx.is_transient() && ctx.time == 0.0 {
            // Capture the converged t=0 operating-point voltage as the
            // capacitor history.  The first transient Newton trial may have
            // no positive timestep yet; deriving history from that trial's
            // unknown would make the companion source depend on the Newton
            // iterate and defeat its linearization.
            let current_voltage = ctx.input(port_name);
            let rail_low = ctx.input("dgnd");
            let rail_high = ctx.input("dpwr");
            if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                ctx.set_initial_state(previous_low_voltage_state, current_voltage - rail_low);
                ctx.set_initial_state(previous_high_voltage_state, current_voltage - rail_high);
            }
        }

        if ctx.is_transient() && ctx.timestep.is_finite() && ctx.timestep > 0.0 {
            let capacitance_low = params.clo;
            let capacitance_high = params.chi;
            let current_voltage = ctx.input(port_name);
            let rail_low = ctx.input("dgnd");
            let rail_high = ctx.input("dpwr");
            let previous_low_voltage = ctx.state_prev(previous_low_voltage_state);
            let previous_high_voltage = ctx.state_prev(previous_high_voltage_state);
            let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
            let history_low = if !previous_low_voltage.is_finite() || initial_point {
                current_voltage - rail_low
            } else {
                previous_low_voltage
            };
            let history_high = if !previous_high_voltage.is_finite() || initial_point {
                current_voltage - rail_high
            } else {
                previous_high_voltage
            };
            let dt = ctx.timestep;
            let g_cap_low = capacitance_low / dt;
            let g_cap_high = capacitance_high / dt;
            stamp_between(ctx, output_pair, dgnd_pair, g_cap_low);
            stamp_between_rhs(ctx, output_pair, dgnd_pair, -g_cap_low * history_low);
            stamp_between(ctx, output_pair, dpwr_pair, g_cap_high);
            stamp_between_rhs(ctx, output_pair, dpwr_pair, -g_cap_high * history_high);
            Self::set_state_if_committed(
                ctx,
                previous_low_voltage_state,
                current_voltage - rail_low,
            );
            Self::set_state_if_committed(
                ctx,
                previous_high_voltage_state,
                current_voltage - rail_high,
            );
        }

        ctx.set_output_with_partial(port_name, 0.0, 0.0);
    }
}

impl CodeModel for XyceDTff {
    fn name(&self) -> &str {
        MODEL_NAME
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output T flip-flop"
    }

    fn ports(&self) -> &[PortSpec] {
        q_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        q_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        ctx.allocate_states(STATE_COUNT);
        ctx.set_initial_state(STATE_Q, f64::NAN);
        ctx.set_initial_state(STATE_CLK, f64::NAN);
        ctx.set_initial_state(STATE_LAST_CLK_EVENT, 0.0);
        ctx.set_initial_state(STATE_Q_TRANSITION_START, 0.0);
        ctx.set_initial_state(STATE_Q_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(STATE_PENDING_Q, Q_PENDING_NONE);
        ctx.set_initial_state(STATE_PENDING_TIME, f64::NAN);
        ctx.set_initial_state(STATE_Q_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_Q_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_QBAR_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_QBAR_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_T, f64::NAN);
        ctx.set_initial_state(STATE_CLK_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_T_VOLTAGE, f64::NAN);
        ctx.set_initial_state(STATE_T_TRANSITION_TIME, 0.0);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (q, qbar, transition_start, transition_from) = self.update_logic_state(ctx, params);
        Self::stamp_output(
            ctx,
            "q",
            q,
            STATE_Q_PREV_LOW_VOLTAGE,
            STATE_Q_PREV_HIGH_VOLTAGE,
            params,
            transition_start,
            transition_from,
        );
        Self::stamp_output(
            ctx,
            "qbar",
            qbar,
            STATE_QBAR_PREV_LOW_VOLTAGE,
            STATE_QBAR_PREV_HIGH_VOLTAGE,
            params,
            transition_start,
            transition_from.map(|value| !value),
        );

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            if let Some(q) = q
                && let Some(end) = transition_time_for_state(q, transition_start, params)
                && end > ctx.time + 1.0e-18
            {
                ctx.request_breakpoint(end);
            }
        }
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("q") && !output_port.eq_ignore_ascii_case("qbar") {
            return Vec::new();
        }
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        let state = q_state(ctx.state_prev(STATE_Q));
        let from = q_state(ctx.state_prev(STATE_Q_TRANSITION_FROM));
        let resistance = interpolated_resistances(
            if output_port.eq_ignore_ascii_case("q") {
                state
            } else {
                state.map(|v| !v)
            },
            if output_port.eq_ignore_ascii_case("q") {
                from
            } else {
                from.map(|v| !v)
            },
            ctx.state_prev(STATE_Q_TRANSITION_START),
            ctx.time,
            params,
        );
        let g_low = 1.0 / resistance.low;
        let g_high = 1.0 / resistance.high;
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        let c_low = Complex64::new(0.0, omega * params.clo);
        let c_high = Complex64::new(0.0, omega * params.chi);
        vec![
            (
                output_port.to_string(),
                Complex64::new(g_low + g_high, 0.0) + c_low + c_high,
            ),
            ("dgnd".to_string(), -Complex64::new(g_low, 0.0) - c_low),
            ("dpwr".to_string(), -Complex64::new(g_high, 0.0) - c_high),
        ]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("q") || output_port.eq_ignore_ascii_case("qbar")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

//=============================================================================
// Xyce DIG combinational gates
//=============================================================================

#[derive(Debug, Clone, Copy)]
enum GateOperation {
    And,
    Buffer,
    Inverter,
    Nand,
    Nor,
    Or,
    Xnor,
    Xor,
}

/// Xyce-compatible analog-interface combinational gate.
///
/// Xyce's PSpice `U` gates are not ordinary XSPICE event gates: their inputs
/// are analog voltages, their output is a finite-impedance analog node, and
/// the referenced `DIG` card controls thresholds, propagation delay, rail
/// resistances, and charge storage.  This model is shared by the eight
/// truth-table operations while each registry entry retains its canonical
/// model name.
#[derive(Debug, Clone, Copy)]
pub struct XyceDGate {
    model_name: &'static str,
    operation: GateOperation,
}

impl XyceDGate {
    pub const fn and() -> Self {
        Self {
            model_name: "xyce_d_and",
            operation: GateOperation::And,
        }
    }

    pub const fn buffer() -> Self {
        Self {
            model_name: "xyce_d_buffer",
            operation: GateOperation::Buffer,
        }
    }

    pub const fn inverter() -> Self {
        Self {
            model_name: "xyce_d_inverter",
            operation: GateOperation::Inverter,
        }
    }

    pub const fn nand() -> Self {
        Self {
            model_name: "xyce_d_nand",
            operation: GateOperation::Nand,
        }
    }

    pub const fn nor() -> Self {
        Self {
            model_name: "xyce_d_nor",
            operation: GateOperation::Nor,
        }
    }

    pub const fn or() -> Self {
        Self {
            model_name: "xyce_d_or",
            operation: GateOperation::Or,
        }
    }

    pub const fn xnor() -> Self {
        Self {
            model_name: "xyce_d_xnor",
            operation: GateOperation::Xnor,
        }
    }

    pub const fn xor() -> Self {
        Self {
            model_name: "xyce_d_xor",
            operation: GateOperation::Xor,
        }
    }

    const Q_STATE: usize = 0;
    const Q_TRANSITION_START: usize = 1;
    const Q_TRANSITION_FROM: usize = 2;
    const PENDING_Q: usize = 3;
    const PENDING_TIME: usize = 4;
    const OUTPUT_PREV_LOW_VOLTAGE: usize = 5;
    const OUTPUT_PREV_HIGH_VOLTAGE: usize = 6;
    const OUTPUT_PREV_PREV_LOW_VOLTAGE: usize = 7;
    const OUTPUT_PREV_PREV_HIGH_VOLTAGE: usize = 8;
    const OUTPUT_PREV_LOW_CURRENT: usize = 9;
    const OUTPUT_PREV_HIGH_CURRENT: usize = 10;
    const INPUT_BASE: usize = 11;
    const INPUT_STRIDE: usize = 3;

    const INPUT_STATE: usize = 0;
    const INPUT_VOLTAGE: usize = 1;
    const INPUT_TRANSITION_TIME: usize = 2;

    fn input_state_index(index: usize, field: usize) -> usize {
        Self::INPUT_BASE + index * Self::INPUT_STRIDE + field
    }

    fn commit_state(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn state_to_value(state: Option<bool>) -> Value {
        state.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW })
    }

    fn logic(&self, inputs: &[Option<bool>]) -> Option<bool> {
        if inputs.is_empty() {
            return None;
        }
        if inputs.iter().any(Option::is_none) {
            return None;
        }
        let values = inputs.iter().map(|value| value.expect("checked above"));
        match self.operation {
            GateOperation::And => Some(values.clone().all(|value| value)),
            GateOperation::Buffer => inputs[0],
            GateOperation::Inverter => inputs[0].map(|value| !value),
            GateOperation::Nand => Some(!values.clone().all(|value| value)),
            GateOperation::Nor => Some(!values.clone().any(|value| value)),
            GateOperation::Or => Some(values.clone().any(|value| value)),
            GateOperation::Xor => Some(values.clone().fold(false, |acc, value| acc ^ value)),
            GateOperation::Xnor => Some(!values.fold(false, |acc, value| acc ^ value)),
        }
    }

    fn ports() -> &'static [PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("dpwr", PortType::Voltage)
                    .with_description("Positive digital power rail"),
                PortSpec::input("dgnd", PortType::Voltage)
                    .with_description("Digital ground/reference rail"),
                PortSpec::vector_input("in", PortType::Voltage)
                    .with_vector_min_len(1)
                    .with_description("Analog logic inputs referenced to DGND"),
                PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::InOut,
                    default_type: PortType::Conductance,
                    allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Finite-impedance analog gate output".to_string(),
                },
            ]
        })
    }

    fn sample_inputs(
        &self,
        ctx: &mut CmContext,
        params: DigParams,
    ) -> (Vec<Option<bool>>, Vec<Value>, Value) {
        let width = ctx.port_width("in");
        let dgnd = ctx.input("dgnd");
        let voltages: Vec<Value> = ctx
            .input_analog_vector_values("in")
            .map(|values| values.iter().map(|value| value.value - dgnd).collect())
            .unwrap_or_default();
        let mut states = Vec::with_capacity(width);
        let mut transition_times = Vec::with_capacity(width);
        let mut last_transition_time: Value = 0.0;

        for index in 0..width {
            let state_index = Self::input_state_index(index, Self::INPUT_STATE);
            let voltage_index = Self::input_state_index(index, Self::INPUT_VOLTAGE);
            let transition_index = Self::input_state_index(index, Self::INPUT_TRANSITION_TIME);
            let voltage = voltages.get(index).copied().unwrap_or(f64::NAN);
            let previous_state = q_state(ctx.state_prev(state_index));
            let previous_voltage = ctx.state_prev(voltage_index);
            let mut state = XyceDTff::input_logic_state(voltage, previous_state, params);
            let mut transition_time = ctx.state_prev(transition_index);

            if ctx.is_dc() {
                state = XyceDTff::input_logic_state(voltage, None, params);
                transition_time = 0.0;
            } else if previous_state.is_some() && previous_state != state {
                transition_time = XyceDTff::input_transition_time(
                    ctx,
                    previous_voltage,
                    voltage,
                    state.unwrap_or(false),
                    params,
                );
            }

            if let Some(state) = state {
                Self::commit_state(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
            }
            if voltage.is_finite() {
                Self::commit_state(ctx, voltage_index, voltage);
            }
            if transition_time.is_finite() {
                Self::commit_state(ctx, transition_index, transition_time);
            }

            states.push(state);
            transition_times.push(transition_time);
            if transition_time.is_finite() {
                last_transition_time = last_transition_time.max(transition_time);
            }
        }

        (states, transition_times, last_transition_time)
    }

    fn update_output(
        &self,
        ctx: &mut CmContext,
        desired: Option<bool>,
        last_input_transition_time: Value,
        params: DigParams,
    ) -> (Option<bool>, Value, Option<bool>) {
        let previous_q = q_state(ctx.state_prev(Self::Q_STATE));
        let mut q = previous_q;
        let mut transition_start = ctx.state_prev(Self::Q_TRANSITION_START);
        let mut transition_from = q_state(ctx.state_prev(Self::Q_TRANSITION_FROM));
        let mut pending_q = q_state(ctx.state_prev(Self::PENDING_Q));
        let mut pending_time = ctx.state_prev(Self::PENDING_TIME);

        if transition_from.is_none() && q.is_some() {
            transition_from = q;
            transition_start = 0.0;
        }

        // The transient engine can start with a fresh context after the DC
        // operating point.  Xyce carries a combinational gate's resolved
        // truth-table value into that first transient sample rather than
        // treating it as a delayed transition from X.
        if !ctx.is_dc() && q.is_none() && desired.is_some() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
        }

        if ctx.is_dc() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
            pending_q = None;
            pending_time = f64::NAN;
        } else {
            if let Some(target) = pending_q
                && pending_time.is_finite()
                && pending_time <= ctx.time + 1.0e-18
            {
                transition_from = q;
                q = Some(target);
                transition_start = pending_time;
            }

            match desired {
                Some(target) if Some(target) != q => {
                    let event_time = last_input_transition_time + params.delay;
                    if event_time <= ctx.time + 1.0e-18 {
                        transition_from = q;
                        q = Some(target);
                        transition_start = event_time;
                        pending_q = None;
                        pending_time = f64::NAN;
                    } else {
                        pending_q = Some(target);
                        pending_time = event_time;
                        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                            ctx.request_breakpoint(event_time);
                        }
                    }
                }
                Some(_) => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
                None => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
            }
        }

        Self::commit_state(ctx, Self::Q_STATE, Self::state_to_value(q));
        Self::commit_state(ctx, Self::Q_TRANSITION_START, transition_start);
        Self::commit_state(
            ctx,
            Self::Q_TRANSITION_FROM,
            Self::state_to_value(transition_from),
        );
        Self::commit_state(ctx, Self::PENDING_Q, Self::state_to_value(pending_q));
        Self::commit_state(ctx, Self::PENDING_TIME, pending_time);
        (q, transition_start, transition_from)
    }

    fn stamp_input_load(
        ctx: &mut CmContext,
        index: usize,
        voltage_state_index: usize,
        params: DigParams,
    ) {
        if ctx.is_ac() {
            return;
        }
        let input_pair = ctx.port_vector_node_pair("in", index).unwrap_or((0, 0));
        let dgnd_pair = ctx.port_node_pair("dgnd").unwrap_or((0, 0));
        stamp_between(ctx, input_pair, dgnd_pair, 1.0 / params.rload);

        if !ctx.is_transient() || !ctx.timestep.is_finite() || ctx.timestep <= 0.0 {
            return;
        }

        let current_voltage = ctx
            .input_analog_vector_values("in")
            .and_then(|values| values.get(index).copied())
            .map(|value| value.value)
            .unwrap_or(0.0);
        let dgnd_voltage = ctx.input("dgnd");
        let current_relative = current_voltage - dgnd_voltage;
        let previous_relative = ctx.state_prev(voltage_state_index);
        let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
        let history = if !previous_relative.is_finite() || initial_point {
            current_relative
        } else {
            previous_relative
        };
        let conductance = params.cload / ctx.timestep;
        stamp_between(ctx, input_pair, dgnd_pair, conductance);
        stamp_between_rhs(ctx, input_pair, dgnd_pair, -conductance * history);
    }

    fn gate_parameters() -> &'static [ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = q_parameters().to_vec();
            // Xyce's generic U-gates accept a scalar IC instance parameter
            // that seeds the output state during the DC operating point.  It
            // is intentionally separate from the DIG model card parameters.
            params.push(ParamSpec::real("ic", f64::NAN));
            params
        })
    }
}

impl CodeModel for XyceDGate {
    fn name(&self) -> &str {
        self.model_name
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output combinational gate"
    }

    fn ports(&self) -> &[PortSpec] {
        Self::ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        Self::gate_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        let width = ctx.port_width("in");
        if width == 0 {
            return Err(CmError::InvalidPortConnection(
                "Xyce DIG gate requires at least one input".to_string(),
            ));
        }
        ctx.allocate_states(Self::INPUT_BASE + width * Self::INPUT_STRIDE);
        ctx.set_initial_state(Self::Q_STATE, f64::NAN);
        ctx.set_initial_state(Self::Q_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::Q_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::PENDING_Q, Q_PENDING_NONE);
        ctx.set_initial_state(Self::PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_LOW_CURRENT, 0.0);
        ctx.set_initial_state(Self::OUTPUT_PREV_HIGH_CURRENT, 0.0);
        for index in 0..width {
            ctx.set_initial_state(Self::input_state_index(index, Self::INPUT_STATE), f64::NAN);
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                f64::NAN,
            );
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_TRANSITION_TIME),
                0.0,
            );
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (inputs, _transition_times, last_input_transition_time) =
            self.sample_inputs(ctx, params);
        let desired = self.logic(&inputs);
        let desired = if ctx.is_dc() {
            q_state(ctx.param("ic")).or(desired)
        } else {
            desired
        };
        let (output_state, transition_start, transition_from) =
            self.update_output(ctx, desired, last_input_transition_time, params);
        for index in 0..ctx.port_width("in") {
            Self::stamp_input_load(
                ctx,
                index,
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                params,
            );
        }
        XyceDTff::stamp_output(
            ctx,
            "out",
            output_state,
            Self::OUTPUT_PREV_LOW_VOLTAGE,
            Self::OUTPUT_PREV_HIGH_VOLTAGE,
            params,
            transition_start,
            transition_from,
        );

        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        let state = q_state(ctx.state_prev(Self::Q_STATE));
        let from = q_state(ctx.state_prev(Self::Q_TRANSITION_FROM));
        let resistance = interpolated_resistances(
            state,
            from,
            ctx.state_prev(Self::Q_TRANSITION_START),
            ctx.time,
            params,
        );
        let g_low = 1.0 / resistance.low;
        let g_high = 1.0 / resistance.high;
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        let c_low = Complex64::new(0.0, omega * params.clo);
        let c_high = Complex64::new(0.0, omega * params.chi);
        vec![
            (
                output_port.to_string(),
                Complex64::new(g_low + g_high, 0.0) + c_low + c_high,
            ),
            ("dgnd".to_string(), -Complex64::new(g_low, 0.0) - c_low),
            ("dpwr".to_string(), -Complex64::new(g_high, 0.0) - c_high),
        ]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

/// Xyce-compatible finite-output three-input full adder.
///
/// The native Xyce ADD device has three analog logic inputs and two
/// independently loaded analog outputs.  Its truth table is the canonical
/// full-adder relation: `sum = A xor B xor Cin` and `carry = majority(A, B,
/// Cin)`.  The two outputs share input history but retain independent delay
/// and finite-output state so each terminal follows the DIG card exactly.
#[derive(Debug, Default)]
pub struct XyceDAdd;

impl XyceDAdd {
    const SUM_Q: usize = 0;
    const SUM_TRANSITION_START: usize = 1;
    const SUM_TRANSITION_FROM: usize = 2;
    const SUM_PENDING_Q: usize = 3;
    const SUM_PENDING_TIME: usize = 4;
    const SUM_PREV_LOW_VOLTAGE: usize = 5;
    const SUM_PREV_HIGH_VOLTAGE: usize = 6;
    const CARRY_Q: usize = 7;
    const CARRY_TRANSITION_START: usize = 8;
    const CARRY_TRANSITION_FROM: usize = 9;
    const CARRY_PENDING_Q: usize = 10;
    const CARRY_PENDING_TIME: usize = 11;
    const CARRY_PREV_LOW_VOLTAGE: usize = 12;
    const CARRY_PREV_HIGH_VOLTAGE: usize = 13;
    const INPUT_BASE: usize = 14;
    const INPUT_STRIDE: usize = 3;

    const INPUT_STATE: usize = 0;
    const INPUT_VOLTAGE: usize = 1;
    const INPUT_TRANSITION_TIME: usize = 2;

    fn input_state_index(index: usize, field: usize) -> usize {
        Self::INPUT_BASE + index * Self::INPUT_STRIDE + field
    }

    fn commit_state(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn sample_inputs(
        ctx: &mut CmContext,
        params: DigParams,
        width: usize,
    ) -> (Vec<Option<bool>>, Value) {
        let dgnd = ctx.input("dgnd");
        let voltages: Vec<Value> = ctx
            .input_analog_vector_values("in")
            .map(|values| values.iter().map(|value| value.value - dgnd).collect())
            .unwrap_or_default();
        let mut states = Vec::with_capacity(width);
        let mut last_transition_time: Value = 0.0;

        for index in 0..width {
            let state_index = Self::input_state_index(index, Self::INPUT_STATE);
            let voltage_index = Self::input_state_index(index, Self::INPUT_VOLTAGE);
            let transition_index = Self::input_state_index(index, Self::INPUT_TRANSITION_TIME);
            let voltage = voltages.get(index).copied().unwrap_or(f64::NAN);
            let previous_state = q_state(ctx.state_prev(state_index));
            let previous_voltage = ctx.state_prev(voltage_index);
            let mut state = XyceDTff::input_logic_state(voltage, previous_state, params);
            let mut transition_time = ctx.state_prev(transition_index);

            if ctx.is_dc() {
                state = XyceDTff::input_logic_state(voltage, None, params);
                transition_time = 0.0;
            } else if previous_state.is_some() && previous_state != state {
                transition_time = XyceDTff::input_transition_time(
                    ctx,
                    previous_voltage,
                    voltage,
                    state.unwrap_or(false),
                    params,
                );
            }

            if let Some(state) = state {
                Self::commit_state(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
            }
            if voltage.is_finite() {
                Self::commit_state(ctx, voltage_index, voltage);
            }
            if transition_time.is_finite() {
                Self::commit_state(ctx, transition_index, transition_time);
            }

            states.push(state);
            if transition_time.is_finite() {
                last_transition_time = last_transition_time.max(transition_time);
            }
        }

        (states, last_transition_time)
    }

    fn update_output(
        ctx: &mut CmContext,
        desired: Option<bool>,
        last_input_transition_time: Value,
        params: DigParams,
        q_state_index: usize,
        transition_start_index: usize,
        transition_from_index: usize,
        pending_q_index: usize,
        pending_time_index: usize,
    ) -> (Option<bool>, Value, Option<bool>) {
        let previous_q = q_state(ctx.state_prev(q_state_index));
        let mut q = previous_q;
        let mut transition_start = ctx.state_prev(transition_start_index);
        let mut transition_from = q_state(ctx.state_prev(transition_from_index));
        let mut pending_q = q_state(ctx.state_prev(pending_q_index));
        let mut pending_time = ctx.state_prev(pending_time_index);

        if transition_from.is_none() && q.is_some() {
            transition_from = q;
            transition_start = 0.0;
        }

        if !ctx.is_dc() && q.is_none() && desired.is_some() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
        }

        if ctx.is_dc() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
            pending_q = None;
            pending_time = f64::NAN;
        } else {
            if let Some(target) = pending_q
                && pending_time.is_finite()
                && pending_time <= ctx.time + 1.0e-18
            {
                transition_from = q;
                q = Some(target);
                transition_start = pending_time;
            }

            match desired {
                Some(target) if Some(target) != q => {
                    let event_time = last_input_transition_time + params.delay;
                    if event_time <= ctx.time + 1.0e-18 {
                        transition_from = q;
                        q = Some(target);
                        transition_start = event_time;
                        pending_q = None;
                        pending_time = f64::NAN;
                    } else {
                        pending_q = Some(target);
                        pending_time = event_time;
                        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                            ctx.request_breakpoint(event_time);
                        }
                    }
                }
                Some(_) => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
                None => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
            }
        }

        Self::commit_state(
            ctx,
            q_state_index,
            q.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, transition_start_index, transition_start);
        Self::commit_state(
            ctx,
            transition_from_index,
            transition_from.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(
            ctx,
            pending_q_index,
            pending_q.map_or(Q_PENDING_NONE, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, pending_time_index, pending_time);
        (q, transition_start, transition_from)
    }

    fn stamp_input_load(
        ctx: &mut CmContext,
        index: usize,
        voltage_state_index: usize,
        params: DigParams,
    ) {
        if ctx.is_ac() {
            return;
        }
        let input_pair = ctx.port_vector_node_pair("in", index).unwrap_or((0, 0));
        let dgnd_pair = ctx.port_node_pair("dgnd").unwrap_or((0, 0));
        stamp_between(ctx, input_pair, dgnd_pair, 1.0 / params.rload);

        if !ctx.is_transient() || !ctx.timestep.is_finite() || ctx.timestep <= 0.0 {
            return;
        }

        let current_voltage = ctx
            .input_analog_vector_values("in")
            .and_then(|values| values.get(index).copied())
            .map(|value| value.value)
            .unwrap_or(0.0);
        let dgnd_voltage = ctx.input("dgnd");
        let current_relative = current_voltage - dgnd_voltage;
        let previous_relative = ctx.state_prev(voltage_state_index);
        let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
        let history = if !previous_relative.is_finite() || initial_point {
            current_relative
        } else {
            previous_relative
        };
        let conductance = params.cload / ctx.timestep;
        stamp_between(ctx, input_pair, dgnd_pair, conductance);
        stamp_between_rhs(ctx, input_pair, dgnd_pair, -conductance * history);
    }

    fn ports() -> &'static [PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            let output = |name: &str, description: &str| PortSpec {
                name: name.to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: description.to_string(),
            };
            vec![
                PortSpec::input("dpwr", PortType::Voltage)
                    .with_description("Positive digital power rail"),
                PortSpec::input("dgnd", PortType::Voltage)
                    .with_description("Digital ground/reference rail"),
                PortSpec::vector_input("in", PortType::Voltage)
                    .with_vector_min_len(3)
                    .with_vector_max_len(3)
                    .with_description("Three analog full-adder inputs referenced to DGND"),
                output("sum", "Finite-impedance sum output"),
                output("carry", "Finite-impedance carry output"),
            ]
        })
    }

    fn parameters() -> &'static [ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = q_parameters().to_vec();
            params.push(ParamSpec::real("ic_sum", f64::NAN));
            params.push(ParamSpec::real("ic_carry", f64::NAN));
            params
        })
    }

    fn initial_state(value: Value) -> Value {
        if (value - Q_HIGH).abs() <= 0.25 {
            Q_HIGH
        } else if value.abs() <= 0.25 {
            Q_LOW
        } else {
            Q_UNKNOWN
        }
    }
}

impl CodeModel for XyceDAdd {
    fn name(&self) -> &str {
        "xyce_d_add"
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output full adder"
    }

    fn ports(&self) -> &[PortSpec] {
        Self::ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        Self::parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        if ctx.port_width("in") != 3 {
            return Err(CmError::InvalidPortConnection(
                "Xyce DIG ADD requires exactly three inputs".to_string(),
            ));
        }
        ctx.allocate_states(Self::INPUT_BASE + 3 * Self::INPUT_STRIDE);
        for (index, param) in [(Self::SUM_Q, "ic_sum"), (Self::CARRY_Q, "ic_carry")] {
            ctx.set_initial_state(index, Self::initial_state(ctx.param(param)));
        }
        for index in [
            Self::SUM_TRANSITION_START,
            Self::SUM_TRANSITION_FROM,
            Self::SUM_PENDING_TIME,
            Self::CARRY_TRANSITION_START,
            Self::CARRY_TRANSITION_FROM,
            Self::CARRY_PENDING_TIME,
        ] {
            ctx.set_initial_state(
                index,
                if index == Self::SUM_TRANSITION_FROM || index == Self::CARRY_TRANSITION_FROM {
                    f64::NAN
                } else {
                    0.0
                },
            );
        }
        ctx.set_initial_state(Self::SUM_PENDING_Q, Q_PENDING_NONE);
        ctx.set_initial_state(Self::CARRY_PENDING_Q, Q_PENDING_NONE);
        for index in [
            Self::SUM_PREV_LOW_VOLTAGE,
            Self::SUM_PREV_HIGH_VOLTAGE,
            Self::CARRY_PREV_LOW_VOLTAGE,
            Self::CARRY_PREV_HIGH_VOLTAGE,
        ] {
            ctx.set_initial_state(index, f64::NAN);
        }
        for index in 0..3 {
            ctx.set_initial_state(Self::input_state_index(index, Self::INPUT_STATE), f64::NAN);
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                f64::NAN,
            );
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_TRANSITION_TIME),
                0.0,
            );
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (inputs, last_input_transition_time) = Self::sample_inputs(ctx, params, 3);
        let desired = match inputs.as_slice() {
            // The three symmetric product terms are the canonical full-adder
            // carry. Clippy's factored rewrites are equivalent but hide that,
            // so keep the textbook form.
            #[allow(clippy::nonminimal_bool)]
            [Some(a), Some(b), Some(c)] => {
                Some((a ^ b ^ c, (*a && *b) || (*a && *c) || (*b && *c)))
            }
            _ => None,
        };
        let (sum, sum_start, sum_from) = Self::update_output(
            ctx,
            desired.map(|value| value.0),
            last_input_transition_time,
            params,
            Self::SUM_Q,
            Self::SUM_TRANSITION_START,
            Self::SUM_TRANSITION_FROM,
            Self::SUM_PENDING_Q,
            Self::SUM_PENDING_TIME,
        );
        let (carry, carry_start, carry_from) = Self::update_output(
            ctx,
            desired.map(|value| value.1),
            last_input_transition_time,
            params,
            Self::CARRY_Q,
            Self::CARRY_TRANSITION_START,
            Self::CARRY_TRANSITION_FROM,
            Self::CARRY_PENDING_Q,
            Self::CARRY_PENDING_TIME,
        );

        for index in 0..3 {
            Self::stamp_input_load(
                ctx,
                index,
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                params,
            );
        }
        XyceDTff::stamp_output(
            ctx,
            "sum",
            sum,
            Self::SUM_PREV_LOW_VOLTAGE,
            Self::SUM_PREV_HIGH_VOLTAGE,
            params,
            sum_start,
            sum_from,
        );
        XyceDTff::stamp_output(
            ctx,
            "carry",
            carry,
            Self::CARRY_PREV_LOW_VOLTAGE,
            Self::CARRY_PREV_HIGH_VOLTAGE,
            params,
            carry_start,
            carry_from,
        );

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            for (state, start) in [(sum, sum_start), (carry, carry_start)] {
                if let Some(state) = state
                    && let Some(end) = transition_time_for_state(state, start, params)
                    && end > ctx.time + 1.0e-18
                {
                    ctx.request_breakpoint(end);
                }
            }
        }
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let (q_state_index, from_index, start_index) = if output_port.eq_ignore_ascii_case("sum") {
            (
                Self::SUM_Q,
                Self::SUM_TRANSITION_FROM,
                Self::SUM_TRANSITION_START,
            )
        } else if output_port.eq_ignore_ascii_case("carry") {
            (
                Self::CARRY_Q,
                Self::CARRY_TRANSITION_FROM,
                Self::CARRY_TRANSITION_START,
            )
        } else {
            return Vec::new();
        };
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        let state = q_state(ctx.state_prev(q_state_index));
        let from = q_state(ctx.state_prev(from_index));
        let resistance =
            interpolated_resistances(state, from, ctx.state_prev(start_index), ctx.time, params);
        let g_low = 1.0 / resistance.low;
        let g_high = 1.0 / resistance.high;
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        let c_low = Complex64::new(0.0, omega * params.clo);
        let c_high = Complex64::new(0.0, omega * params.chi);
        vec![
            (
                output_port.to_string(),
                Complex64::new(g_low + g_high, 0.0) + c_low + c_high,
            ),
            ("dgnd".to_string(), -Complex64::new(g_low, 0.0) - c_low),
            ("dpwr".to_string(), -Complex64::new(g_high, 0.0) - c_high),
        ]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("sum") || output_port.eq_ignore_ascii_case("carry")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

/// Xyce's legacy Y-device combinational gate interface.
///
/// Unlike the PSpice U-device form, legacy Y gates do not expose power-rail
/// terminals.  The DIG card supplies fixed VLO, VHI, and VREF values, so this
/// model stamps each output and input load against those ideal references.
#[derive(Debug, Clone, Copy)]
pub struct XyceDLegacyGate {
    model_name: &'static str,
    operation: GateOperation,
}

impl XyceDLegacyGate {
    pub const fn and() -> Self {
        Self {
            model_name: "xyce_legacy_d_and",
            operation: GateOperation::And,
        }
    }

    pub const fn inverter() -> Self {
        Self {
            model_name: "xyce_legacy_d_inverter",
            operation: GateOperation::Inverter,
        }
    }

    pub const fn nand() -> Self {
        Self {
            model_name: "xyce_legacy_d_nand",
            operation: GateOperation::Nand,
        }
    }

    pub const fn nor() -> Self {
        Self {
            model_name: "xyce_legacy_d_nor",
            operation: GateOperation::Nor,
        }
    }

    pub const fn or() -> Self {
        Self {
            model_name: "xyce_legacy_d_or",
            operation: GateOperation::Or,
        }
    }

    pub const fn xnor() -> Self {
        Self {
            model_name: "xyce_legacy_d_xnor",
            operation: GateOperation::Xnor,
        }
    }

    pub const fn xor() -> Self {
        Self {
            model_name: "xyce_legacy_d_xor",
            operation: GateOperation::Xor,
        }
    }

    const Q_STATE: usize = 0;
    const Q_TRANSITION_START: usize = 1;
    const Q_TRANSITION_FROM: usize = 2;
    const PENDING_Q: usize = 3;
    const PENDING_TIME: usize = 4;
    const OUTPUT_PREV_LOW_VOLTAGE: usize = 5;
    const OUTPUT_PREV_HIGH_VOLTAGE: usize = 6;
    const OUTPUT_PREV_PREV_LOW_VOLTAGE: usize = 7;
    const OUTPUT_PREV_PREV_HIGH_VOLTAGE: usize = 8;
    const OUTPUT_PREV_LOW_CURRENT: usize = 9;
    const OUTPUT_PREV_HIGH_CURRENT: usize = 10;
    const INPUT_BASE: usize = 11;
    const INPUT_STRIDE: usize = 3;

    const INPUT_STATE: usize = 0;
    const INPUT_VOLTAGE: usize = 1;
    const INPUT_TRANSITION_TIME: usize = 2;

    fn input_state_index(index: usize, field: usize) -> usize {
        Self::INPUT_BASE + index * Self::INPUT_STRIDE + field
    }

    fn commit_state(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn rails(ctx: &CmContext) -> CmResult<(Value, Value, Value)> {
        let vref = finite_param(ctx, "vref")?;
        let vlo = finite_param(ctx, "vlo")?;
        let vhi = finite_param(ctx, "vhi")?;
        Ok((vref, vlo, vhi))
    }

    fn authored_initial_state(ctx: &CmContext) -> Option<bool> {
        let ic = ctx.param("ic");
        if (ic - Q_HIGH).abs() <= 0.25 {
            Some(true)
        } else if ic.abs() <= 0.25 {
            Some(false)
        } else {
            None
        }
    }

    fn logic(&self, inputs: &[Option<bool>]) -> Option<bool> {
        XyceDGate {
            model_name: "xyce_legacy_logic",
            operation: self.operation,
        }
        .logic(inputs)
    }

    fn sample_inputs(
        &self,
        ctx: &mut CmContext,
        params: DigParams,
        width: usize,
        vref: Value,
    ) -> (Vec<Option<bool>>, Value) {
        let voltages: Vec<Value> = ctx
            .input_analog_vector_values("in")
            .map(|values| values.iter().map(|value| value.value - vref).collect())
            .unwrap_or_default();
        let mut states = Vec::with_capacity(width);
        let mut last_transition_time: Value = 0.0;

        for index in 0..width {
            let state_index = Self::input_state_index(index, Self::INPUT_STATE);
            let voltage_index = Self::input_state_index(index, Self::INPUT_VOLTAGE);
            let transition_index = Self::input_state_index(index, Self::INPUT_TRANSITION_TIME);
            let voltage = voltages.get(index).copied().unwrap_or(f64::NAN);
            let previous_state = q_state(ctx.state_prev(state_index));
            let previous_voltage = ctx.state_prev(voltage_index);
            let transition = ctx.input_analog_vector_transition("in", index);
            // Analog code-model inputs remain governed by the receiving
            // device's Schmitt thresholds.  Upstream transition metadata only
            // supplies the causal event time once that analog threshold is
            // actually crossed; it must not bypass the receiving device's
            // finite voltage trajectory.
            let mut state = XyceDTff::input_logic_state(voltage, previous_state, params);
            let mut transition_time = ctx.state_prev(transition_index);
            let changed = previous_state.is_some() && previous_state != state;
            if ctx.is_dc() {
                state = XyceDTff::input_logic_state(voltage, None, params);
                transition_time = 0.0;
            } else if changed {
                transition_time = transition
                    .map(|transition| transition.event_time)
                    .unwrap_or_else(|| {
                        let threshold = if state.unwrap_or(false) {
                            params.s0_vhi.max(params.s1_vlo)
                        } else {
                            params.s1_vlo.min(params.s0_vhi)
                        };
                        transition_time_at_threshold(ctx, previous_voltage, voltage, threshold)
                    });
            }

            if let Some(state) = state {
                Self::commit_state(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
            }
            if voltage.is_finite() {
                Self::commit_state(ctx, voltage_index, voltage);
            }
            if transition_time.is_finite() {
                Self::commit_state(ctx, transition_index, transition_time);
            }

            states.push(state);
            if transition_time.is_finite() {
                last_transition_time = last_transition_time.max(transition_time);
            }
        }
        (states, last_transition_time)
    }

    fn update_output(
        &self,
        ctx: &mut CmContext,
        desired: Option<bool>,
        last_input_transition_time: Value,
        params: DigParams,
    ) -> (Option<bool>, Value, Option<bool>) {
        let previous_q = q_state(ctx.state_prev(Self::Q_STATE));
        let mut q = previous_q;
        let mut transition_start = ctx.state_prev(Self::Q_TRANSITION_START);
        let mut transition_from = q_state(ctx.state_prev(Self::Q_TRANSITION_FROM));
        let mut pending_q = q_state(ctx.state_prev(Self::PENDING_Q));
        let mut pending_time = ctx.state_prev(Self::PENDING_TIME);

        if transition_from.is_none() && q.is_some() {
            transition_from = q;
            transition_start = 0.0;
        }
        if !ctx.is_dc() && q.is_none() && desired.is_some() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
        }

        if ctx.is_dc() {
            // Xyce evaluates the combinational truth table first and then
            // applies GateData::setIC while establishing the DC operating
            // point.  An authored IC therefore owns the initial output even
            // when it opposes the truth-table result.  The first transient
            // evaluation will reconcile that state after the model DELAY.
            q = Self::authored_initial_state(ctx).or(desired);
            transition_from = q;
            transition_start = 0.0;
            pending_q = None;
            pending_time = f64::NAN;
        } else {
            if let Some(target) = pending_q
                && pending_time.is_finite()
                && pending_time <= ctx.time + 1.0e-18
            {
                transition_from = q;
                q = Some(target);
                transition_start = pending_time;
            }
            match desired {
                Some(target) if Some(target) != q => {
                    let event_time = last_input_transition_time + params.delay;
                    if event_time <= ctx.time + 1.0e-18 {
                        transition_from = q;
                        q = Some(target);
                        transition_start = event_time;
                        pending_q = None;
                        pending_time = f64::NAN;
                    } else {
                        pending_q = Some(target);
                        pending_time = event_time;
                        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                            ctx.request_breakpoint(event_time);
                        }
                    }
                }
                Some(_) => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
                None => {
                    pending_q = None;
                    pending_time = f64::NAN;
                }
            }
        }

        Self::commit_state(
            ctx,
            Self::Q_STATE,
            q.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, Self::Q_TRANSITION_START, transition_start);
        Self::commit_state(
            ctx,
            Self::Q_TRANSITION_FROM,
            transition_from.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(
            ctx,
            Self::PENDING_Q,
            pending_q.map_or(Q_PENDING_NONE, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, Self::PENDING_TIME, pending_time);
        (q, transition_start, transition_from)
    }

    fn stamp_input_load(
        ctx: &mut CmContext,
        index: usize,
        voltage_state_index: usize,
        params: DigParams,
        vref: Value,
    ) {
        if ctx.is_ac() {
            return;
        }
        let input_pair = ctx.port_vector_node_pair("in", index).unwrap_or((0, 0));
        let ground = (0, 0);
        let g_load = 1.0 / params.rload;
        let static_scale = if ctx.xyce_one_step_order2()
            && ctx.evaluation_phase() != EvaluationPhase::AcceptedStep
        {
            0.5
        } else {
            1.0
        };
        stamp_between(ctx, input_pair, ground, static_scale * g_load);
        stamp_between_rhs(ctx, input_pair, ground, -static_scale * g_load * vref);
        queue_between_static(ctx, input_pair, ground, g_load);
        queue_between_rhs_static(ctx, input_pair, ground, -g_load * vref);

        if !ctx.is_transient() || !ctx.timestep.is_finite() || ctx.timestep <= 0.0 {
            return;
        }
        let current_voltage = ctx
            .input_analog_vector_values("in")
            .and_then(|values| values.get(index).copied())
            .map(|value| value.value)
            .unwrap_or(0.0);
        let previous_voltage = ctx.state_prev(voltage_state_index);
        let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
        let history = if !previous_voltage.is_finite() || initial_point {
            current_voltage
        } else {
            previous_voltage
        };
        let conductance = params.cload / ctx.timestep;
        stamp_between(ctx, input_pair, ground, conductance);
        stamp_between_rhs(ctx, input_pair, ground, -conductance * history);
    }

    fn stamp_output(
        ctx: &mut CmContext,
        port_name: &str,
        state: Option<bool>,
        previous_low_voltage_state: usize,
        previous_high_voltage_state: usize,
        previous_previous_low_voltage_state: usize,
        previous_previous_high_voltage_state: usize,
        previous_low_current_state: usize,
        previous_high_current_state: usize,
        params: DigParams,
        transition_start: Value,
        transition_from: Option<bool>,
        vlo: Value,
        vhi: Value,
    ) {
        let output_pair = ctx.port_node_pair(port_name).unwrap_or((0, 0));
        let ground = (0, 0);
        let resistances =
            interpolated_resistances(state, transition_from, transition_start, ctx.time, params);
        let g_low = 1.0 / resistances.low;
        let g_high = 1.0 / resistances.high;

        if ctx.is_ac() {
            ctx.set_output_with_partial(port_name, 0.0, g_low + g_high);
            return;
        }

        let static_scale = if ctx.is_transient()
            && ctx.xyce_one_step_order2()
            && ctx.evaluation_phase() != EvaluationPhase::AcceptedStep
        {
            0.5
        } else {
            1.0
        };
        stamp_between(ctx, output_pair, ground, static_scale * (g_low + g_high));
        stamp_between_rhs(
            ctx,
            output_pair,
            ground,
            -static_scale * (g_low * vlo + g_high * vhi),
        );
        queue_between_static(ctx, output_pair, ground, g_low + g_high);
        queue_between_rhs_static(ctx, output_pair, ground, -(g_low * vlo + g_high * vhi));
        if ctx.is_transient() && ctx.time == 0.0 {
            let current_voltage = ctx.input(port_name);
            if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                ctx.set_initial_state(previous_low_voltage_state, current_voltage);
                ctx.set_initial_state(previous_high_voltage_state, current_voltage);
                ctx.set_initial_state(previous_previous_low_voltage_state, current_voltage);
                ctx.set_initial_state(previous_previous_high_voltage_state, current_voltage);
                ctx.set_initial_state(previous_low_current_state, 0.0);
                ctx.set_initial_state(previous_high_current_state, 0.0);
            }
        }

        if ctx.is_transient() && ctx.timestep.is_finite() && ctx.timestep > 0.0 {
            let current_voltage = ctx.input(port_name);
            let previous_low_voltage = ctx.state_prev(previous_low_voltage_state);
            let previous_high_voltage = ctx.state_prev(previous_high_voltage_state);
            let previous_previous_low_voltage = ctx.state_prev(previous_previous_low_voltage_state);
            let previous_previous_high_voltage =
                ctx.state_prev(previous_previous_high_voltage_state);
            let previous_low_current = ctx.state_prev(previous_low_current_state);
            let previous_high_current = ctx.state_prev(previous_high_current_state);
            let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
            let history_low = if !previous_low_voltage.is_finite() || initial_point {
                current_voltage
            } else {
                previous_low_voltage
            };
            let history_high = if !previous_high_voltage.is_finite() || initial_point {
                current_voltage
            } else {
                previous_high_voltage
            };
            let older_history_low = if !previous_previous_low_voltage.is_finite() || initial_point {
                history_low
            } else {
                previous_previous_low_voltage
            };
            let older_history_high = if !previous_previous_high_voltage.is_finite() || initial_point
            {
                history_high
            } else {
                previous_previous_high_voltage
            };
            let history_current_low = if previous_low_current.is_finite() {
                previous_low_current
            } else {
                0.0
            };
            let history_current_high = if previous_high_current.is_finite() {
                previous_high_current
            } else {
                0.0
            };
            let dt = ctx.timestep;
            let coefficients = ctx.transient_companion_coefficients();
            let g_cap_low = coefficients.capacitor_geq(params.clo, dt);
            let g_cap_high = coefficients.capacitor_geq(params.chi, dt);
            let i_eq_low = coefficients.capacitor_ieq(
                params.clo,
                dt,
                history_low,
                older_history_low,
                history_current_low,
            );
            let i_eq_high = coefficients.capacitor_ieq(
                params.chi,
                dt,
                history_high,
                older_history_high,
                history_current_high,
            );
            stamp_between(ctx, output_pair, ground, g_cap_low + g_cap_high);
            stamp_between_rhs(ctx, output_pair, ground, -i_eq_low - i_eq_high);
            Self::commit_state(ctx, previous_low_voltage_state, current_voltage);
            Self::commit_state(ctx, previous_high_voltage_state, current_voltage);
            Self::commit_state(ctx, previous_previous_low_voltage_state, history_low);
            Self::commit_state(ctx, previous_previous_high_voltage_state, history_high);
            Self::commit_state(
                ctx,
                previous_low_current_state,
                g_cap_low * current_voltage - i_eq_low,
            );
            Self::commit_state(
                ctx,
                previous_high_current_state,
                g_cap_high * current_voltage - i_eq_high,
            );
        }
        ctx.set_output_with_partial(port_name, 0.0, 0.0);
    }

    fn ports() -> &'static [PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Voltage)
                    .with_vector_min_len(1)
                    .with_description("Analog legacy Y-device inputs referenced to VREF"),
                PortSpec {
                    name: "out".to_string(),
                    direction: PortDirection::InOut,
                    default_type: PortType::Conductance,
                    allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Finite-impedance analog legacy Y-device output".to_string(),
                },
            ]
        })
    }

    fn parameters() -> &'static [ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = q_parameters().to_vec();
            params.push(ParamSpec::real("vref", 0.0));
            params.push(ParamSpec::real("vlo", 0.0));
            params.push(ParamSpec::real("vhi", 0.0));
            params.push(ParamSpec::real("ic", f64::NAN));
            params
        })
    }
}

impl CodeModel for XyceDLegacyGate {
    fn name(&self) -> &str {
        self.model_name
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output legacy Y combinational gate"
    }

    fn ports(&self) -> &[PortSpec] {
        Self::ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        Self::parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        Self::rails(ctx)?;
        let width = ctx.port_width("in");
        if width == 0 {
            return Err(CmError::InvalidPortConnection(
                "Xyce legacy DIG gate requires at least one input".to_string(),
            ));
        }
        ctx.allocate_states(Self::INPUT_BASE + width * Self::INPUT_STRIDE);
        let initial_q = Self::authored_initial_state(ctx)
            .map_or(Q_UNKNOWN, |state| if state { Q_HIGH } else { Q_LOW });
        ctx.set_initial_state(Self::Q_STATE, initial_q);
        ctx.set_initial_state(Self::Q_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::Q_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::PENDING_Q, Q_PENDING_NONE);
        ctx.set_initial_state(Self::PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_PREV_LOW_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_PREV_HIGH_VOLTAGE, f64::NAN);
        ctx.set_initial_state(Self::OUTPUT_PREV_LOW_CURRENT, 0.0);
        ctx.set_initial_state(Self::OUTPUT_PREV_HIGH_CURRENT, 0.0);
        for index in 0..width {
            ctx.set_initial_state(Self::input_state_index(index, Self::INPUT_STATE), f64::NAN);
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                f64::NAN,
            );
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_TRANSITION_TIME),
                0.0,
            );
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (vref, vlo, vhi) = Self::rails(ctx)?;
        let width = ctx.port_width("in");
        let (inputs, last_input_transition_time) = self.sample_inputs(ctx, params, width, vref);
        let desired = self.logic(&inputs);
        let (output_state, transition_start, transition_from) =
            self.update_output(ctx, desired, last_input_transition_time, params);
        for index in 0..width {
            Self::stamp_input_load(
                ctx,
                index,
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                params,
                vref,
            );
        }
        Self::stamp_output(
            ctx,
            "out",
            output_state,
            Self::OUTPUT_PREV_LOW_VOLTAGE,
            Self::OUTPUT_PREV_HIGH_VOLTAGE,
            Self::OUTPUT_PREV_PREV_LOW_VOLTAGE,
            Self::OUTPUT_PREV_PREV_HIGH_VOLTAGE,
            Self::OUTPUT_PREV_LOW_CURRENT,
            Self::OUTPUT_PREV_HIGH_CURRENT,
            params,
            transition_start,
            transition_from,
            vlo,
            vhi,
        );
        if let Some(state) = output_state {
            let event_time = if transition_start.is_finite() && transition_start > 0.0 {
                (transition_start - params.delay).max(0.0)
            } else {
                0.0
            };
            let nominal_end = transition_time_for_state(state, transition_start, params)
                .unwrap_or(transition_start);
            let transition_end = if transition_from.is_some()
                && transition_from != Some(state)
                && transition_start.is_finite()
                && transition_start > 0.0
            {
                let threshold = if state { params.s1_vlo } else { params.s0_vhi };
                let current_voltage = ctx.input("out");
                let previous_voltage = ctx.state_prev(Self::OUTPUT_PREV_LOW_VOLTAGE);
                let crossed = if state {
                    previous_voltage < threshold && current_voltage >= threshold
                } else {
                    previous_voltage > threshold && current_voltage <= threshold
                };
                if crossed {
                    transition_time_at_threshold(ctx, previous_voltage, current_voltage, threshold)
                } else {
                    nominal_end
                }
            } else {
                nominal_end
            };
            ctx.set_output_analog_transition(
                "out",
                AnalogTransition {
                    state,
                    event_time,
                    transition_start,
                    transition_end,
                },
            );
        }
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        let state = q_state(ctx.state_prev(Self::Q_STATE));
        let from = q_state(ctx.state_prev(Self::Q_TRANSITION_FROM));
        let resistance = interpolated_resistances(
            state,
            from,
            ctx.state_prev(Self::Q_TRANSITION_START),
            ctx.time,
            params,
        );
        let g_low = 1.0 / resistance.low;
        let g_high = 1.0 / resistance.high;
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        vec![(
            "out".to_string(),
            Complex64::new(g_low + g_high, omega * (params.clo + params.chi)),
        )]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

/// Xyce's legacy Y-device D flip-flop interface.
///
/// The Y DFF has four analog inputs (PREB, CLRB, CLK, and D) and two finite
/// impedance analog outputs (Q and QBAR).  Its DIG card supplies fixed rail
/// voltages because the deprecated Y syntax has no rail terminals.
#[derive(Debug, Default)]
pub struct XyceDLegacyDff;

impl XyceDLegacyDff {
    const Q_STATE: usize = 0;
    const Q_TRANSITION_START: usize = 1;
    const Q_TRANSITION_FROM: usize = 2;
    const Q_PENDING: usize = 3;
    const Q_PENDING_TIME: usize = 4;
    const Q_PENDING_START: usize = 5;
    const Q_PREV_LOW_VOLTAGE: usize = 6;
    const Q_PREV_HIGH_VOLTAGE: usize = 7;
    const Q_PREV_PREV_LOW_VOLTAGE: usize = 16;
    const Q_PREV_PREV_HIGH_VOLTAGE: usize = 17;
    const Q_PREV_LOW_CURRENT: usize = 18;
    const Q_PREV_HIGH_CURRENT: usize = 19;
    const QB_STATE: usize = 8;
    const QB_TRANSITION_START: usize = 9;
    const QB_TRANSITION_FROM: usize = 10;
    const QB_PENDING: usize = 11;
    const QB_PENDING_TIME: usize = 12;
    const QB_PENDING_START: usize = 13;
    const QB_PREV_LOW_VOLTAGE: usize = 14;
    const QB_PREV_HIGH_VOLTAGE: usize = 15;
    const QB_PREV_PREV_LOW_VOLTAGE: usize = 20;
    const QB_PREV_PREV_HIGH_VOLTAGE: usize = 21;
    const QB_PREV_LOW_CURRENT: usize = 22;
    const QB_PREV_HIGH_CURRENT: usize = 23;
    const INPUT_BASE: usize = 24;
    const INPUT_STRIDE: usize = 3;
    const INPUT_STATE: usize = 0;
    const INPUT_VOLTAGE: usize = 1;
    const INPUT_TRANSITION_TIME: usize = 2;
    const INPUT_COUNT: usize = 4;
    const TRANSIENT_INITIALIZED: usize = Self::INPUT_BASE + Self::INPUT_COUNT * Self::INPUT_STRIDE;

    fn input_state_index(index: usize, field: usize) -> usize {
        Self::INPUT_BASE + index * Self::INPUT_STRIDE + field
    }

    fn commit_state(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn ports() -> &'static [PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            let output = |name: &str, description: &str| PortSpec {
                name: name.to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: description.to_string(),
            };
            vec![
                PortSpec::vector_input("in", PortType::Voltage)
                    .with_vector_min_len(Self::INPUT_COUNT)
                    .with_vector_max_len(Self::INPUT_COUNT)
                    .with_description("Analog PREB, CLRB, CLK, and D inputs referenced to VREF"),
                output("q", "Finite-impedance analog Q output"),
                output("qbar", "Finite-impedance analog complemented Q output"),
            ]
        })
    }

    fn parameters() -> &'static [ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = q_parameters().to_vec();
            params.push(ParamSpec::real("vref", 0.0));
            params.push(ParamSpec::real("vlo", 0.0));
            params.push(ParamSpec::real("vhi", 0.0));
            params.push(ParamSpec::real("ic1", f64::NAN));
            params.push(ParamSpec::real("ic2", f64::NAN));
            params
        })
    }

    fn initial_state(value: Value) -> Value {
        if (value - Q_HIGH).abs() <= 0.25 {
            Q_HIGH
        } else if value.abs() <= 0.25 {
            Q_LOW
        } else {
            Q_UNKNOWN
        }
    }

    fn sample_inputs(
        ctx: &mut CmContext,
        params: DigParams,
        vref: Value,
    ) -> (Vec<Option<bool>>, Value, bool, Option<bool>, bool, Value) {
        let voltages: Vec<Value> = ctx
            .input_analog_vector_values("in")
            .map(|values| values.iter().map(|value| value.value - vref).collect())
            .unwrap_or_default();
        let mut states = Vec::with_capacity(Self::INPUT_COUNT);
        let mut last_transition_time: Value = 0.0;
        let mut clock_changed = false;
        let mut clock_state = None;
        let mut deferred_until = f64::NAN;
        let transient_start =
            !ctx.is_dc() && !ctx.state_prev(Self::TRANSIENT_INITIALIZED).is_finite();

        for index in 0..Self::INPUT_COUNT {
            let state_index = Self::input_state_index(index, Self::INPUT_STATE);
            let voltage_index = Self::input_state_index(index, Self::INPUT_VOLTAGE);
            let transition_index = Self::input_state_index(index, Self::INPUT_TRANSITION_TIME);
            let voltage = voltages.get(index).copied().unwrap_or(f64::NAN);
            let previous_state = q_state(ctx.state_prev(state_index));
            let transition = ctx.input_analog_vector_transition("in", index);
            if let Some(transition) = transition
                && transition.transition_start.is_finite()
                && transition.transition_start > 0.0
                && transition.transition_end.is_finite()
            {
                deferred_until = if deferred_until.is_finite() {
                    deferred_until.min(transition.transition_end)
                } else {
                    transition.transition_end
                };
            }
            let mut state = transition
                .map(|transition| transition.state)
                .or_else(|| XyceDTff::input_logic_state(voltage, previous_state, params));
            let mut transition_time = ctx.state_prev(transition_index);
            let metadata_changed = transition
                .filter(|transition| {
                    previous_state.is_some() && previous_state != Some(transition.state)
                })
                .is_some();
            if metadata_changed {
                state = transition.map(|transition| transition.state);
                deferred_until = deferred_until.max(
                    transition
                        .map(|transition| transition.transition_end)
                        .unwrap_or(f64::NAN),
                );
            }
            let changed = previous_state.is_some() && previous_state != state;

            if ctx.is_dc() {
                state = XyceDTff::input_logic_state(voltage, None, params);
                transition_time = 0.0;
            } else if metadata_changed {
                transition_time = transition
                    .map(|transition| transition.event_time)
                    .unwrap_or(ctx.time);
            } else if changed {
                transition_time = XyceDTff::input_transition_time(
                    ctx,
                    ctx.state_prev(voltage_index),
                    voltage,
                    state.unwrap_or(false),
                    params,
                );
            }

            if index == 2 {
                clock_changed = changed;
                clock_state = state;
            }
            if let Some(state) = state {
                Self::commit_state(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
            }
            if voltage.is_finite() {
                Self::commit_state(ctx, voltage_index, voltage);
            }
            if transition_time.is_finite() {
                Self::commit_state(ctx, transition_index, transition_time);
                last_transition_time = last_transition_time.max(transition_time);
            }
            states.push(state);
        }

        if transient_start {
            Self::commit_state(ctx, Self::TRANSIENT_INITIALIZED, 1.0);
        }

        (
            states,
            last_transition_time,
            clock_changed,
            clock_state,
            transient_start,
            deferred_until,
        )
    }

    fn targets(
        ctx: &CmContext,
        inputs: &[Option<bool>],
        q: Option<bool>,
        qbar: Option<bool>,
        clock_changed: bool,
        clock_state: Option<bool>,
    ) -> (Option<bool>, Option<bool>) {
        let [Some(prebar), Some(clrbar), Some(_clock), Some(data)] = inputs else {
            return (None, None);
        };

        if ctx.is_dc() {
            return if *prebar && *clrbar {
                (Some(*data), Some(!*data))
            } else if *prebar && !*clrbar {
                (Some(false), Some(true))
            } else if !*prebar && *clrbar {
                (Some(true), Some(false))
            } else {
                (Some(true), Some(true))
            };
        }

        // Xyce gives a clock transition priority over asynchronous controls;
        // when the controls are asserted during an edge, the existing output
        // state is retained until the controls are released.
        if clock_changed {
            if *prebar && *clrbar && clock_state == Some(true) {
                return (Some(*data), Some(!*data));
            }
            return (q, qbar);
        }

        if *prebar && !*clrbar {
            (Some(false), Some(true))
        } else if !*prebar && *clrbar {
            (Some(true), Some(false))
        } else if !*prebar && !*clrbar {
            (Some(true), Some(true))
        } else {
            let qbar = qbar.or_else(|| q.map(|value| !value));
            if q.is_none() && qbar.is_none() {
                (Some(*data), Some(!*data))
            } else if q.is_some() && qbar == q {
                (q, q.map(|value| !value))
            } else {
                (q, qbar)
            }
        }
    }

    fn update_output(
        ctx: &mut CmContext,
        desired: Option<bool>,
        last_input_transition_time: Value,
        deferred_until: Value,
        event_delay: Value,
        transition_duration: Value,
        q_index: usize,
        transition_start_index: usize,
        transition_from_index: usize,
        pending_index: usize,
        pending_time_index: usize,
        pending_start_index: usize,
        reschedule_pending: bool,
    ) -> (Option<bool>, Value, Option<bool>) {
        let previous_q = q_state(ctx.state_prev(q_index));
        let mut q = previous_q;
        let mut transition_start = ctx.state_prev(transition_start_index);
        let mut transition_from = q_state(ctx.state_prev(transition_from_index));
        let mut pending_q = q_state(ctx.state_prev(pending_index));
        let mut pending_time = ctx.state_prev(pending_time_index);
        let mut pending_start = ctx.state_prev(pending_start_index);
        if pending_q.is_none() {
            let current_pending = q_state(ctx.state(pending_index));
            let current_time = ctx.state(pending_time_index);
            if current_pending.is_some() && current_time.is_finite() {
                pending_q = current_pending;
                pending_time = current_time;
                pending_start = ctx.state(pending_start_index);
            }
        }

        if transition_from.is_none() && q.is_some() {
            transition_from = q;
            transition_start = 0.0;
        }
        if !ctx.is_dc() && q.is_none() && desired.is_some() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
        }

        if ctx.is_dc() {
            q = desired;
            transition_from = q;
            transition_start = 0.0;
            pending_q = None;
            pending_time = f64::NAN;
            pending_start = f64::NAN;
        } else {
            let mut pending_applied = false;
            if reschedule_pending
                && let Some(pending_target) = pending_q
                && pending_time.is_finite()
                && desired == Some(pending_target)
            {
                // Xyce recomputes the output event from the latest input
                // transition on every trial.  A later input transition can
                // therefore move an already scheduled event forward while
                // the truth-table target remains unchanged.
                let event_time = last_input_transition_time + event_delay;
                if event_time.is_finite() && event_time > pending_time + 1.0e-18 {
                    pending_time = event_time;
                    pending_start = event_time;
                }
            }
            if let Some(pending_target) = pending_q
                && deferred_until.is_finite()
                && pending_time.is_finite()
                && pending_target == desired.unwrap_or(pending_target)
                && deferred_until < pending_time
            {
                pending_time = deferred_until;
                pending_start = (deferred_until - transition_duration).max(0.0);
            }
            if let Some(target) = pending_q
                && pending_time.is_finite()
                // Xyce's next-state vector can carry the scheduled output
                // transition through the exact breakpoint evaluation while
                // the current-state vector remains unchanged.  Activate the
                // finite-output transition at the scheduled event time.
                && pending_time <= ctx.time + 1.0e-18
                && (!deferred_until.is_finite()
                    || ctx.evaluation_phase() == EvaluationPhase::AcceptedStep)
            {
                transition_from = q;
                q = Some(target);
                transition_start = if pending_start.is_finite() && pending_start > 0.0 {
                    pending_start
                } else {
                    pending_time
                };
                pending_q = None;
                pending_time = f64::NAN;
                pending_start = f64::NAN;
                pending_applied = true;
            }

            // Re-issue a scheduled output event from every committed model
            // evaluation.  The transient engine intentionally discards
            // rollback-only breakpoint requests, so retaining the pending
            // state alone is insufficient to guarantee that the accepted
            // solution lands on the delayed DIG transition.
            if !pending_applied
                && ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
                && pending_q.is_some()
                && pending_time.is_finite()
                && pending_time > ctx.time + 1.0e-18
            {
                ctx.request_breakpoint(pending_time);
            }

            if !pending_applied {
                match desired {
                    Some(target) if Some(target) != q => {
                        let pending_active = pending_q == Some(target)
                            && pending_time.is_finite()
                            && pending_time >= ctx.time - 1.0e-18;
                        if !pending_active {
                            let event_time = last_input_transition_time + event_delay;
                            if deferred_until.is_finite() && event_time <= ctx.time + 1.0e-18 {
                                transition_from = q;
                                transition_start = event_time;
                                pending_q = Some(target);
                                pending_time = deferred_until.max(event_time);
                                pending_start = event_time;
                            } else if event_time <= ctx.time + 1.0e-18 {
                                transition_from = q;
                                q = Some(target);
                                transition_start = event_time;
                                pending_q = None;
                                pending_time = f64::NAN;
                                pending_start = f64::NAN;
                            } else {
                                pending_q = Some(target);
                                pending_time = event_time;
                                pending_start = event_time;
                            }
                        }
                    }
                    Some(_)
                        if !(pending_q.is_some()
                            && pending_time.is_finite()
                            && pending_time > ctx.time + 1.0e-18) =>
                    {
                        pending_q = None;
                        pending_time = f64::NAN;
                        pending_start = f64::NAN;
                    }
                    Some(_) => {}
                    None if !(pending_q.is_some()
                        && pending_time.is_finite()
                        && pending_time > ctx.time + 1.0e-18) =>
                    {
                        pending_q = None;
                        pending_time = f64::NAN;
                        pending_start = f64::NAN;
                    }
                    None => {}
                }
            }
        }

        Self::commit_state(
            ctx,
            q_index,
            q.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, transition_start_index, transition_start);
        Self::commit_state(
            ctx,
            transition_from_index,
            transition_from.map_or(Q_UNKNOWN, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(
            ctx,
            pending_index,
            pending_q.map_or(Q_PENDING_NONE, |value| if value { Q_HIGH } else { Q_LOW }),
        );
        Self::commit_state(ctx, pending_time_index, pending_time);
        Self::commit_state(ctx, pending_start_index, pending_start);
        (q, transition_start, transition_from)
    }

    fn output_ac_partials(
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
        state_index: usize,
        from_index: usize,
        start_index: usize,
        params: DigParams,
    ) -> Vec<(String, Complex64)> {
        let state = q_state(ctx.state_prev(state_index));
        let from = q_state(ctx.state_prev(from_index));
        let resistance =
            interpolated_resistances(state, from, ctx.state_prev(start_index), ctx.time, params);
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        vec![(
            output_port.to_string(),
            Complex64::new(
                1.0 / resistance.low + 1.0 / resistance.high,
                omega * (params.clo + params.chi),
            ),
        )]
    }
}

impl CodeModel for XyceDLegacyDff {
    fn name(&self) -> &str {
        "xyce_legacy_d_dff"
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output legacy Y D flip-flop"
    }

    fn ports(&self) -> &[PortSpec] {
        Self::ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        Self::parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        let (vref, _vlo, _vhi) = XyceDLegacyGate::rails(ctx)?;
        let _ = vref;
        if ctx.port_width("in") != Self::INPUT_COUNT {
            return Err(CmError::InvalidPortConnection(
                "Xyce legacy DIG DFF requires PREB, CLRB, CLK, and D inputs".to_string(),
            ));
        }
        ctx.allocate_states(Self::TRANSIENT_INITIALIZED + 1);
        ctx.set_initial_state(Self::Q_STATE, Self::initial_state(ctx.param("ic1")));
        ctx.set_initial_state(Self::Q_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::Q_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::Q_PENDING, Q_PENDING_NONE);
        ctx.set_initial_state(Self::Q_PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::Q_PENDING_START, f64::NAN);
        ctx.set_initial_state(Self::QB_STATE, Self::initial_state(ctx.param("ic2")));
        ctx.set_initial_state(Self::QB_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::QB_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::QB_PENDING, Q_PENDING_NONE);
        ctx.set_initial_state(Self::QB_PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::QB_PENDING_START, f64::NAN);
        for index in [
            Self::Q_PREV_LOW_VOLTAGE,
            Self::Q_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_LOW_VOLTAGE,
            Self::QB_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_PREV_LOW_VOLTAGE,
            Self::Q_PREV_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_PREV_LOW_VOLTAGE,
            Self::QB_PREV_PREV_HIGH_VOLTAGE,
        ] {
            ctx.set_initial_state(index, f64::NAN);
        }
        for index in [
            Self::Q_PREV_LOW_CURRENT,
            Self::Q_PREV_HIGH_CURRENT,
            Self::QB_PREV_LOW_CURRENT,
            Self::QB_PREV_HIGH_CURRENT,
        ] {
            ctx.set_initial_state(index, 0.0);
        }
        for index in 0..Self::INPUT_COUNT {
            ctx.set_initial_state(Self::input_state_index(index, Self::INPUT_STATE), f64::NAN);
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                f64::NAN,
            );
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_TRANSITION_TIME),
                0.0,
            );
        }
        ctx.set_initial_state(Self::TRANSIENT_INITIALIZED, f64::NAN);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (vref, vlo, vhi) = XyceDLegacyGate::rails(ctx)?;
        let (
            inputs,
            last_input_transition_time,
            clock_changed,
            clock_state,
            _transient_start,
            deferred_until,
        ) = Self::sample_inputs(ctx, params, vref);
        let previous_q = q_state(ctx.state_prev(Self::Q_STATE));
        let previous_qbar = q_state(ctx.state_prev(Self::QB_STATE));
        let (q_target, qbar_target) = Self::targets(
            ctx,
            &inputs,
            previous_q,
            previous_qbar,
            clock_changed,
            clock_state,
        );
        let (q, q_start, q_from) = Self::update_output(
            ctx,
            q_target,
            last_input_transition_time,
            deferred_until,
            params.delay,
            transition_duration(q_target.unwrap_or(false), params),
            Self::Q_STATE,
            Self::Q_TRANSITION_START,
            Self::Q_TRANSITION_FROM,
            Self::Q_PENDING,
            Self::Q_PENDING_TIME,
            Self::Q_PENDING_START,
            false,
        );
        let (qbar, qbar_start, qbar_from) = Self::update_output(
            ctx,
            qbar_target,
            last_input_transition_time,
            deferred_until,
            params.delay,
            transition_duration(qbar_target.unwrap_or(false), params),
            Self::QB_STATE,
            Self::QB_TRANSITION_START,
            Self::QB_TRANSITION_FROM,
            Self::QB_PENDING,
            Self::QB_PENDING_TIME,
            Self::QB_PENDING_START,
            false,
        );
        for index in 0..Self::INPUT_COUNT {
            XyceDLegacyGate::stamp_input_load(
                ctx,
                index,
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                params,
                vref,
            );
        }
        XyceDLegacyGate::stamp_output(
            ctx,
            "q",
            q,
            Self::Q_PREV_LOW_VOLTAGE,
            Self::Q_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_PREV_LOW_VOLTAGE,
            Self::Q_PREV_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_LOW_CURRENT,
            Self::Q_PREV_HIGH_CURRENT,
            params,
            q_start,
            q_from,
            vlo,
            vhi,
        );
        XyceDLegacyGate::stamp_output(
            ctx,
            "qbar",
            qbar,
            Self::QB_PREV_LOW_VOLTAGE,
            Self::QB_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_PREV_LOW_VOLTAGE,
            Self::QB_PREV_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_LOW_CURRENT,
            Self::QB_PREV_HIGH_CURRENT,
            params,
            qbar_start,
            qbar_from,
            vlo,
            vhi,
        );
        for (port_name, state, transition_start) in [("q", q, q_start), ("qbar", qbar, qbar_start)]
        {
            if let Some(state) = state {
                let event_time = if transition_start.is_finite() && transition_start > 0.0 {
                    (transition_start - params.delay).max(0.0)
                } else {
                    0.0
                };
                ctx.set_output_analog_transition(
                    port_name,
                    AnalogTransition {
                        state,
                        event_time,
                        transition_start,
                        transition_end: transition_time_for_state(state, transition_start, params)
                            .unwrap_or(transition_start),
                    },
                );
            }
        }
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let (state_index, from_index, start_index) = if output_port.eq_ignore_ascii_case("q") {
            (
                Self::Q_STATE,
                Self::Q_TRANSITION_FROM,
                Self::Q_TRANSITION_START,
            )
        } else if output_port.eq_ignore_ascii_case("qbar") {
            (
                Self::QB_STATE,
                Self::QB_TRANSITION_FROM,
                Self::QB_TRANSITION_START,
            )
        } else {
            return Vec::new();
        };
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        Self::output_ac_partials(
            ctx,
            output_port,
            frequency,
            state_index,
            from_index,
            start_index,
            params,
        )
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("q") || output_port.eq_ignore_ascii_case("qbar")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

/// Finite-output implementations of the PSpice U-device sequential models.
///
/// PSpice's `DFF`, `JKFF`, and `DLTCH` U-devices are lowered by the netlist
/// frontend to an analog-vector XSPICE instance.  The native XSPICE models
/// (`d_dff`, `d_jkff`, and `d_dlatch`) expose ideal digital outputs, which is
/// not the interface implemented by Xyce's DIG device.  This model keeps the
/// canonical Xyce truth tables while using the same two-rail finite-output
/// stamping, input loading, transition scheduling, and checkpoint layout as
/// the already-qualified Xyce TFF and legacy Y-device models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceDSequentialKind {
    Dff,
    Jkff,
    Dlatch,
}

impl XyceDSequentialKind {
    fn input_count(self) -> usize {
        match self {
            Self::Dff | Self::Dlatch => 4,
            Self::Jkff => 5,
        }
    }

    fn event_input(self) -> usize {
        match self {
            // PSpice U-device port order is data/clock/set/reset.
            Self::Dff => 1,
            // PSpice U-device port order is J/K/clock/set/reset.
            Self::Jkff => 2,
            // A latch's enable input is the event-producing control.
            Self::Dlatch => 1,
        }
    }

    fn model_name(self) -> &'static str {
        match self {
            Self::Dff => "xyce_d_dff",
            Self::Jkff => "xyce_d_jkff",
            Self::Dlatch => "xyce_d_dlatch",
        }
    }
}

/// Shared finite-output sequential XSPICE code model.
#[derive(Debug, Clone, Copy)]
pub struct XyceDSequential {
    kind: XyceDSequentialKind,
}

impl XyceDSequential {
    pub const fn dff() -> Self {
        Self {
            kind: XyceDSequentialKind::Dff,
        }
    }

    pub const fn jkff() -> Self {
        Self {
            kind: XyceDSequentialKind::Jkff,
        }
    }

    pub const fn dlatch() -> Self {
        Self {
            kind: XyceDSequentialKind::Dlatch,
        }
    }

    const Q_STATE: usize = 0;
    const Q_TRANSITION_START: usize = 1;
    const Q_TRANSITION_FROM: usize = 2;
    const Q_PENDING: usize = 3;
    const Q_PENDING_TIME: usize = 4;
    const Q_PENDING_START: usize = 5;
    const Q_PREV_LOW_VOLTAGE: usize = 6;
    const Q_PREV_HIGH_VOLTAGE: usize = 7;
    const Q_PREV_PREV_LOW_VOLTAGE: usize = 8;
    const Q_PREV_PREV_HIGH_VOLTAGE: usize = 9;
    const Q_PREV_LOW_CURRENT: usize = 10;
    const Q_PREV_HIGH_CURRENT: usize = 11;
    const QB_STATE: usize = 12;
    const QB_TRANSITION_START: usize = 13;
    const QB_TRANSITION_FROM: usize = 14;
    const QB_PENDING: usize = 15;
    const QB_PENDING_TIME: usize = 16;
    const QB_PENDING_START: usize = 17;
    const QB_PREV_LOW_VOLTAGE: usize = 18;
    const QB_PREV_HIGH_VOLTAGE: usize = 19;
    const QB_PREV_PREV_LOW_VOLTAGE: usize = 20;
    const QB_PREV_PREV_HIGH_VOLTAGE: usize = 21;
    const QB_PREV_LOW_CURRENT: usize = 22;
    const QB_PREV_HIGH_CURRENT: usize = 23;
    const INPUT_BASE: usize = 24;
    const INPUT_STRIDE: usize = 3;
    const INPUT_STATE: usize = 0;
    const INPUT_VOLTAGE: usize = 1;
    const INPUT_TRANSITION_TIME: usize = 2;
    const INPUT_CAPACITY: usize = 5;
    const TRANSIENT_INITIALIZED: usize =
        Self::INPUT_BASE + Self::INPUT_CAPACITY * Self::INPUT_STRIDE;
    const STATE_COUNT: usize = Self::TRANSIENT_INITIALIZED + 1;

    fn input_state_index(index: usize, field: usize) -> usize {
        Self::INPUT_BASE + index * Self::INPUT_STRIDE + field
    }

    fn ports(kind: XyceDSequentialKind) -> &'static [PortSpec] {
        use std::sync::OnceLock;

        fn output(name: &str, description: &str) -> PortSpec {
            PortSpec {
                name: name.to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::Conductance,
                allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: description.to_string(),
            }
        }

        match kind {
            XyceDSequentialKind::Dff => {
                static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
                PORTS
                    .get_or_init(|| {
                        vec![
                            PortSpec::input("dpwr", PortType::Voltage)
                                .with_description("Positive digital power rail"),
                            PortSpec::input("dgnd", PortType::Voltage)
                                .with_description("Digital ground/reference rail"),
                            PortSpec::vector_input("in", PortType::Voltage)
                                .with_vector_min_len(4)
                                .with_vector_max_len(4)
                                .with_description(
                                    "Analog D, rising-edge clock, PREB, and CLRB inputs",
                                ),
                            output("q", "Finite-impedance analog Q output"),
                            output("qbar", "Finite-impedance analog complemented Q output"),
                        ]
                    })
                    .as_slice()
            }
            XyceDSequentialKind::Jkff => {
                static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
                PORTS
                    .get_or_init(|| {
                        vec![
                            PortSpec::input("dpwr", PortType::Voltage)
                                .with_description("Positive digital power rail"),
                            PortSpec::input("dgnd", PortType::Voltage)
                                .with_description("Digital ground/reference rail"),
                            PortSpec::vector_input("in", PortType::Voltage)
                                .with_vector_min_len(5)
                                .with_vector_max_len(5)
                                .with_description(
                                    "Analog J, K, falling-edge clock, PREB, and CLRB inputs",
                                ),
                            output("q", "Finite-impedance analog Q output"),
                            output("qbar", "Finite-impedance analog complemented Q output"),
                        ]
                    })
                    .as_slice()
            }
            XyceDSequentialKind::Dlatch => {
                static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
                PORTS
                    .get_or_init(|| {
                        vec![
                            PortSpec::input("dpwr", PortType::Voltage)
                                .with_description("Positive digital power rail"),
                            PortSpec::input("dgnd", PortType::Voltage)
                                .with_description("Digital ground/reference rail"),
                            PortSpec::vector_input("in", PortType::Voltage)
                                .with_vector_min_len(4)
                                .with_vector_max_len(4)
                                .with_description(
                                    "Analog D, enable, PREB, and CLRB inputs for a level latch",
                                ),
                            output("q", "Finite-impedance analog Q output"),
                            output("qbar", "Finite-impedance analog complemented Q output"),
                        ]
                    })
                    .as_slice()
            }
        }
    }

    fn parameters() -> &'static [ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS
            .get_or_init(|| {
                let mut params = q_parameters().to_vec();
                params.push(ParamSpec::real("vref", 0.0));
                params.push(ParamSpec::real("vlo", 0.0));
                params.push(ParamSpec::real("vhi", 0.0));
                params.push(ParamSpec::real("ic1", f64::NAN));
                params.push(ParamSpec::real("ic2", f64::NAN));
                params
            })
            .as_slice()
    }

    fn initial_state(value: Value) -> Value {
        XyceDLegacyDff::initial_state(value)
    }

    fn commit_state(ctx: &mut CmContext, index: usize, value: Value) {
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(index, value);
        }
    }

    fn sample_inputs(
        &self,
        ctx: &mut CmContext,
        params: DigParams,
        vref: Value,
    ) -> (Vec<Option<bool>>, Value, bool, Option<bool>, Value) {
        let width = self.kind.input_count();
        let voltages: Vec<Value> = ctx
            .input_analog_vector_values("in")
            .map(|values| values.iter().map(|value| value.value - vref).collect())
            .unwrap_or_default();
        let mut states = Vec::with_capacity(width);
        let mut last_transition_time: Value = 0.0;
        let mut event_changed = false;
        let mut event_state = None;
        let mut deferred_until = f64::NAN;
        let transient_start =
            !ctx.is_dc() && !ctx.state_prev(Self::TRANSIENT_INITIALIZED).is_finite();

        for index in 0..width {
            let state_index = Self::input_state_index(index, Self::INPUT_STATE);
            let voltage_index = Self::input_state_index(index, Self::INPUT_VOLTAGE);
            let transition_index = Self::input_state_index(index, Self::INPUT_TRANSITION_TIME);
            let voltage = voltages.get(index).copied().unwrap_or(f64::NAN);
            let previous_state = q_state(ctx.state_prev(state_index));
            let transition = ctx.input_analog_vector_transition("in", index);
            if let Some(transition) = transition
                && transition.transition_start.is_finite()
                && transition.transition_start > 0.0
                && transition.transition_end.is_finite()
            {
                deferred_until = if deferred_until.is_finite() {
                    deferred_until.min(transition.transition_end)
                } else {
                    transition.transition_end
                };
            }

            // The U-device receives an analog voltage.  Resolve its logic
            // state from the solved voltage and Schmitt thresholds; upstream
            // transition metadata is used only to carry a causal end time,
            // never to override the receiver's analog decision.
            let mut state = XyceDTff::input_logic_state(voltage, previous_state, params);
            let mut transition_time = ctx.state_prev(transition_index);
            let metadata_changed = transition
                .filter(|transition| {
                    previous_state.is_some() && previous_state != Some(transition.state)
                })
                .is_some();
            let changed = previous_state.is_some() && previous_state != state;

            if metadata_changed && changed {
                deferred_until = deferred_until.max(
                    transition
                        .map(|transition| transition.transition_end)
                        .unwrap_or(f64::NAN),
                );
            }

            if ctx.is_dc() {
                state = XyceDTff::input_logic_state(voltage, None, params);
                transition_time = 0.0;
            } else if changed {
                // Xyce linearly interpolates the analog receiver's Schmitt
                // threshold crossing inside the accepted solver interval.
                transition_time = XyceDTff::input_transition_time(
                    ctx,
                    ctx.state_prev(voltage_index),
                    voltage,
                    state.unwrap_or(false),
                    params,
                );
            }

            if index == self.kind.event_input() {
                event_changed = changed;
                event_state = state;
            }
            if let Some(state) = state {
                Self::commit_state(ctx, state_index, if state { Q_HIGH } else { Q_LOW });
            }
            if voltage.is_finite() {
                Self::commit_state(ctx, voltage_index, voltage);
            }
            if transition_time.is_finite() {
                Self::commit_state(ctx, transition_index, transition_time);
                last_transition_time = last_transition_time.max(transition_time);
            }
            states.push(state);
        }

        if transient_start {
            Self::commit_state(ctx, Self::TRANSIENT_INITIALIZED, 1.0);
        }
        (
            states,
            last_transition_time,
            event_changed,
            event_state,
            deferred_until,
        )
    }

    fn async_targets(
        prebar: bool,
        clrbar: bool,
        q: Option<bool>,
        qbar: Option<bool>,
        fallback: Option<bool>,
    ) -> (Option<bool>, Option<bool>) {
        if prebar && !clrbar {
            return (Some(false), Some(true));
        }
        if !prebar && clrbar {
            return (Some(true), Some(false));
        }
        if !prebar && !clrbar {
            return (Some(true), Some(true));
        }

        let qbar = qbar.or_else(|| q.map(|value| !value));
        if q.is_none() && qbar.is_none() {
            (fallback, fallback.map(|value| !value))
        } else if q.is_some() && qbar == q {
            (q, q.map(|value| !value))
        } else {
            (q, qbar)
        }
    }

    fn targets(
        &self,
        ctx: &CmContext,
        inputs: &[Option<bool>],
        q: Option<bool>,
        qbar: Option<bool>,
        event_changed: bool,
        event_state: Option<bool>,
    ) -> (Option<bool>, Option<bool>) {
        match self.kind {
            XyceDSequentialKind::Dff => {
                let [Some(data), Some(_clock), Some(prebar), Some(clrbar)] = inputs else {
                    return (None, None);
                };
                if ctx.is_dc() {
                    return Self::async_targets(*prebar, *clrbar, None, None, Some(*data));
                }
                if event_changed {
                    if *prebar && *clrbar && event_state == Some(true) {
                        return (Some(*data), Some(!*data));
                    }
                    return (q, qbar);
                }
                Self::async_targets(*prebar, *clrbar, q, qbar, Some(*data))
            }
            XyceDSequentialKind::Jkff => {
                let [Some(j), Some(k), Some(_clock), Some(prebar), Some(clrbar)] = inputs else {
                    return (None, None);
                };
                if ctx.is_dc() {
                    return Self::async_targets(*prebar, *clrbar, None, None, Some(*j));
                }
                if event_changed {
                    if *prebar && *clrbar && event_state == Some(false) {
                        let next = match (*j, *k) {
                            (false, false) => q,
                            (false, true) => Some(false),
                            (true, false) => Some(true),
                            (true, true) => q.map(|value| !value),
                        };
                        return (next, next.map(|value| !value));
                    }
                    return (q, qbar);
                }
                Self::async_targets(*prebar, *clrbar, q, qbar, Some(*j))
            }
            XyceDSequentialKind::Dlatch => {
                let [Some(data), Some(enable), Some(prebar), Some(clrbar)] = inputs else {
                    return (None, None);
                };
                if ctx.is_dc() {
                    return Self::async_targets(*prebar, *clrbar, None, None, Some(*data));
                }
                if *prebar && *clrbar && *enable {
                    return (Some(*data), Some(!*data));
                }
                Self::async_targets(*prebar, *clrbar, q, qbar, Some(*data))
            }
        }
    }

    fn initialize(&self, ctx: &mut CmContext) -> CmResult<()> {
        dig_params(ctx)?;
        let (vref, _vlo, _vhi) = XyceDLegacyGate::rails(ctx)?;
        let _ = vref;
        if ctx.port_width("in") != self.kind.input_count() {
            return Err(CmError::InvalidPortConnection(format!(
                "{} requires {} analog sequential inputs",
                self.kind.model_name(),
                self.kind.input_count()
            )));
        }
        ctx.allocate_states(Self::STATE_COUNT);
        ctx.set_initial_state(Self::Q_STATE, Self::initial_state(ctx.param("ic1")));
        ctx.set_initial_state(Self::Q_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::Q_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::Q_PENDING, Q_PENDING_NONE);
        ctx.set_initial_state(Self::Q_PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::Q_PENDING_START, f64::NAN);
        ctx.set_initial_state(Self::QB_STATE, Self::initial_state(ctx.param("ic2")));
        ctx.set_initial_state(Self::QB_TRANSITION_START, 0.0);
        ctx.set_initial_state(Self::QB_TRANSITION_FROM, f64::NAN);
        ctx.set_initial_state(Self::QB_PENDING, Q_PENDING_NONE);
        ctx.set_initial_state(Self::QB_PENDING_TIME, f64::NAN);
        ctx.set_initial_state(Self::QB_PENDING_START, f64::NAN);
        for index in [
            Self::Q_PREV_LOW_VOLTAGE,
            Self::Q_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_PREV_LOW_VOLTAGE,
            Self::Q_PREV_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_LOW_VOLTAGE,
            Self::QB_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_PREV_LOW_VOLTAGE,
            Self::QB_PREV_PREV_HIGH_VOLTAGE,
        ] {
            ctx.set_initial_state(index, f64::NAN);
        }
        for index in [
            Self::Q_PREV_LOW_CURRENT,
            Self::Q_PREV_HIGH_CURRENT,
            Self::QB_PREV_LOW_CURRENT,
            Self::QB_PREV_HIGH_CURRENT,
        ] {
            ctx.set_initial_state(index, 0.0);
        }
        for index in 0..Self::INPUT_CAPACITY {
            ctx.set_initial_state(Self::input_state_index(index, Self::INPUT_STATE), f64::NAN);
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                f64::NAN,
            );
            ctx.set_initial_state(
                Self::input_state_index(index, Self::INPUT_TRANSITION_TIME),
                0.0,
            );
        }
        ctx.set_initial_state(Self::TRANSIENT_INITIALIZED, f64::NAN);
        Ok(())
    }

    fn stamp_output(
        ctx: &mut CmContext,
        port_name: &str,
        state: Option<bool>,
        previous_low_voltage_state: usize,
        previous_high_voltage_state: usize,
        previous_previous_low_voltage_state: usize,
        previous_previous_high_voltage_state: usize,
        previous_low_current_state: usize,
        previous_high_current_state: usize,
        params: DigParams,
        transition_start: Value,
        transition_from: Option<bool>,
    ) {
        let output_pair = ctx.port_node_pair(port_name).unwrap_or((0, 0));
        let low_pair = ctx.port_node_pair("dgnd").unwrap_or((0, 0));
        let high_pair = ctx.port_node_pair("dpwr").unwrap_or((0, 0));
        let resistances =
            interpolated_resistances(state, transition_from, transition_start, ctx.time, params);
        let g_low = 1.0 / resistances.low;
        let g_high = 1.0 / resistances.high;
        if ctx.is_ac() {
            ctx.set_output_with_partial(port_name, 0.0, g_low + g_high);
            return;
        }

        let static_scale = if ctx.is_transient()
            && ctx.xyce_one_step_order2()
            && ctx.evaluation_phase() != EvaluationPhase::AcceptedStep
        {
            0.5
        } else {
            1.0
        };
        stamp_between(ctx, output_pair, low_pair, static_scale * g_low);
        stamp_between(ctx, output_pair, high_pair, static_scale * g_high);
        queue_between_static(ctx, output_pair, low_pair, g_low);
        queue_between_static(ctx, output_pair, high_pair, g_high);

        let current_voltage = ctx.input(port_name);
        let low_rail = ctx.input("dgnd");
        let high_rail = ctx.input("dpwr");
        if ctx.is_transient() && ctx.time == 0.0 {
            if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                ctx.set_initial_state(previous_low_voltage_state, current_voltage - low_rail);
                ctx.set_initial_state(previous_high_voltage_state, current_voltage - high_rail);
                ctx.set_initial_state(
                    previous_previous_low_voltage_state,
                    current_voltage - low_rail,
                );
                ctx.set_initial_state(
                    previous_previous_high_voltage_state,
                    current_voltage - high_rail,
                );
                ctx.set_initial_state(previous_low_current_state, 0.0);
                ctx.set_initial_state(previous_high_current_state, 0.0);
            }
        }

        if ctx.is_transient() && ctx.timestep.is_finite() && ctx.timestep > 0.0 {
            let previous_low_voltage = ctx.state_prev(previous_low_voltage_state);
            let previous_high_voltage = ctx.state_prev(previous_high_voltage_state);
            let previous_previous_low_voltage = ctx.state_prev(previous_previous_low_voltage_state);
            let previous_previous_high_voltage =
                ctx.state_prev(previous_previous_high_voltage_state);
            let previous_low_current = ctx.state_prev(previous_low_current_state);
            let previous_high_current = ctx.state_prev(previous_high_current_state);
            let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
            let current_low_voltage = current_voltage - low_rail;
            let current_high_voltage = current_voltage - high_rail;
            let history_low = if !previous_low_voltage.is_finite() || initial_point {
                current_low_voltage
            } else {
                previous_low_voltage
            };
            let history_high = if !previous_high_voltage.is_finite() || initial_point {
                current_high_voltage
            } else {
                previous_high_voltage
            };
            let older_history_low = if !previous_previous_low_voltage.is_finite() || initial_point {
                history_low
            } else {
                previous_previous_low_voltage
            };
            let older_history_high = if !previous_previous_high_voltage.is_finite() || initial_point
            {
                history_high
            } else {
                previous_previous_high_voltage
            };
            let history_current_low = if previous_low_current.is_finite() {
                previous_low_current
            } else {
                0.0
            };
            let history_current_high = if previous_high_current.is_finite() {
                previous_high_current
            } else {
                0.0
            };
            let dt = ctx.timestep;
            let coefficients = ctx.transient_companion_coefficients();
            let g_cap_low = coefficients.capacitor_geq(params.clo, dt);
            let g_cap_high = coefficients.capacitor_geq(params.chi, dt);
            let i_eq_low = coefficients.capacitor_ieq(
                params.clo,
                dt,
                history_low,
                older_history_low,
                history_current_low,
            );
            let i_eq_high = coefficients.capacitor_ieq(
                params.chi,
                dt,
                history_high,
                older_history_high,
                history_current_high,
            );
            stamp_between(ctx, output_pair, low_pair, g_cap_low);
            stamp_between_rhs(ctx, output_pair, low_pair, -i_eq_low);
            stamp_between(ctx, output_pair, high_pair, g_cap_high);
            stamp_between_rhs(ctx, output_pair, high_pair, -i_eq_high);
            Self::commit_state(ctx, previous_low_voltage_state, current_low_voltage);
            Self::commit_state(ctx, previous_high_voltage_state, current_high_voltage);
            Self::commit_state(ctx, previous_previous_low_voltage_state, history_low);
            Self::commit_state(ctx, previous_previous_high_voltage_state, history_high);
            Self::commit_state(
                ctx,
                previous_low_current_state,
                g_cap_low * current_low_voltage - i_eq_low,
            );
            Self::commit_state(
                ctx,
                previous_high_current_state,
                g_cap_high * current_high_voltage - i_eq_high,
            );
        }
        ctx.set_output_with_partial(port_name, 0.0, 0.0);
    }

    fn stamp_input_load(
        ctx: &mut CmContext,
        index: usize,
        voltage_state_index: usize,
        params: DigParams,
    ) {
        if ctx.is_ac() {
            return;
        }

        let input_pair = ctx.port_vector_node_pair("in", index).unwrap_or((0, 0));
        let dgnd_pair = ctx.port_node_pair("dgnd").unwrap_or((0, 0));
        let static_scale = if ctx.xyce_one_step_order2()
            && ctx.evaluation_phase() != EvaluationPhase::AcceptedStep
        {
            0.5
        } else {
            1.0
        };
        let g_load = 1.0 / params.rload;
        stamp_between(ctx, input_pair, dgnd_pair, static_scale * g_load);
        queue_between_static(ctx, input_pair, dgnd_pair, g_load);

        if !ctx.is_transient() || !ctx.timestep.is_finite() || ctx.timestep <= 0.0 {
            return;
        }

        let current_voltage = ctx
            .input_analog_vector_values("in")
            .and_then(|values| values.get(index).copied())
            .map(|value| value.value)
            .unwrap_or(0.0);
        let dgnd_voltage = ctx.input("dgnd");
        let current_relative = current_voltage - dgnd_voltage;
        let previous_relative = ctx.state_prev(voltage_state_index);
        let initial_point = ctx.time == 0.0 && ctx.time_prev == 0.0;
        let history = if !previous_relative.is_finite() || initial_point {
            current_relative
        } else {
            previous_relative
        };
        let coefficients = ctx.transient_companion_coefficients();
        let conductance = coefficients.capacitor_geq(params.cload, ctx.timestep);
        let current = coefficients.capacitor_ieq(params.cload, ctx.timestep, history, history, 0.0);
        stamp_between(ctx, input_pair, dgnd_pair, conductance);
        stamp_between_rhs(ctx, input_pair, dgnd_pair, -current);
    }

    fn evaluate_kind(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = dig_params(ctx)?;
        let (vref, _vlo, _vhi) = XyceDLegacyGate::rails(ctx)?;
        let (inputs, last_input_transition_time, event_changed, event_state, deferred_until) =
            self.sample_inputs(ctx, params, vref);
        let previous_q = q_state(ctx.state_prev(Self::Q_STATE));
        let previous_qbar = q_state(ctx.state_prev(Self::QB_STATE));
        let (q_target, qbar_target) = self.targets(
            ctx,
            &inputs,
            previous_q,
            previous_qbar,
            event_changed,
            event_state,
        );
        let output_event_time = if self.kind == XyceDSequentialKind::Dlatch
            && inputs.get(1).copied().flatten() == Some(false)
            && q_target == previous_q
            && qbar_target == Some(false)
            && previous_qbar == Some(true)
        {
            // With the latch closed, Xyce resolves the complementary output
            // when an asynchronous control releases an unstable state.  That
            // transition is timestamped at the accepted solver landing point,
            // rather than at an earlier analog threshold interpolation.
            last_input_transition_time.max(ctx.time)
        } else {
            last_input_transition_time
        };
        let (q, q_start, q_from) = XyceDLegacyDff::update_output(
            ctx,
            q_target,
            output_event_time,
            deferred_until,
            params.delay,
            transition_duration(q_target.unwrap_or(false), params),
            Self::Q_STATE,
            Self::Q_TRANSITION_START,
            Self::Q_TRANSITION_FROM,
            Self::Q_PENDING,
            Self::Q_PENDING_TIME,
            Self::Q_PENDING_START,
            self.kind == XyceDSequentialKind::Jkff,
        );
        let (qbar, qbar_start, qbar_from) = XyceDLegacyDff::update_output(
            ctx,
            qbar_target,
            output_event_time,
            deferred_until,
            params.delay,
            transition_duration(qbar_target.unwrap_or(false), params),
            Self::QB_STATE,
            Self::QB_TRANSITION_START,
            Self::QB_TRANSITION_FROM,
            Self::QB_PENDING,
            Self::QB_PENDING_TIME,
            Self::QB_PENDING_START,
            self.kind == XyceDSequentialKind::Jkff,
        );
        for index in 0..self.kind.input_count() {
            Self::stamp_input_load(
                ctx,
                index,
                Self::input_state_index(index, Self::INPUT_VOLTAGE),
                params,
            );
        }
        Self::stamp_output(
            ctx,
            "q",
            q,
            Self::Q_PREV_LOW_VOLTAGE,
            Self::Q_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_PREV_LOW_VOLTAGE,
            Self::Q_PREV_PREV_HIGH_VOLTAGE,
            Self::Q_PREV_LOW_CURRENT,
            Self::Q_PREV_HIGH_CURRENT,
            params,
            q_start,
            q_from,
        );
        Self::stamp_output(
            ctx,
            "qbar",
            qbar,
            Self::QB_PREV_LOW_VOLTAGE,
            Self::QB_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_PREV_LOW_VOLTAGE,
            Self::QB_PREV_PREV_HIGH_VOLTAGE,
            Self::QB_PREV_LOW_CURRENT,
            Self::QB_PREV_HIGH_CURRENT,
            params,
            qbar_start,
            qbar_from,
        );
        for (port_name, state, transition_start) in [("q", q, q_start), ("qbar", qbar, qbar_start)]
        {
            if let Some(state) = state {
                let event_time = if transition_start.is_finite() && transition_start > 0.0 {
                    (transition_start - params.delay).max(0.0)
                } else {
                    0.0
                };
                ctx.set_output_analog_transition(
                    port_name,
                    AnalogTransition {
                        state,
                        event_time,
                        transition_start,
                        transition_end: transition_time_for_state(state, transition_start, params)
                            .unwrap_or(transition_start),
                    },
                );
            }
        }
        Ok(())
    }
}

impl CodeModel for XyceDSequential {
    fn name(&self) -> &str {
        self.kind.model_name()
    }

    fn description(&self) -> &str {
        "Xyce DIG-compatible finite-output PSpice sequential device"
    }

    fn ports(&self) -> &[PortSpec] {
        Self::ports(self.kind)
    }

    fn parameters(&self) -> &[ParamSpec] {
        Self::parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        self.initialize(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        self.evaluate_kind(ctx)
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let (state_index, from_index, start_index) = if output_port.eq_ignore_ascii_case("q") {
            (
                Self::Q_STATE,
                Self::Q_TRANSITION_FROM,
                Self::Q_TRANSITION_START,
            )
        } else if output_port.eq_ignore_ascii_case("qbar") {
            (
                Self::QB_STATE,
                Self::QB_TRANSITION_FROM,
                Self::QB_TRANSITION_START,
            )
        } else {
            return Vec::new();
        };
        let Ok(params) = dig_params(ctx) else {
            return Vec::new();
        };
        XyceDLegacyDff::output_ac_partials(
            ctx,
            output_port,
            frequency,
            state_index,
            from_index,
            start_index,
            params,
        )
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("q") || output_port.eq_ignore_ascii_case("qbar")
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> crate::xspice::XspiceCheckpointSupport {
        crate::xspice::XspiceCheckpointSupport::Serializable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::AnalysisType;

    fn context() -> CmContext {
        let mut ctx = CmContext::new();
        for spec in q_parameters() {
            ctx.set_param(&spec.name, spec.default);
        }
        ctx.set_port_node("dpwr", 4);
        ctx.set_port_node("dgnd", 0);
        ctx.set_port_node("q", 2);
        ctx.set_port_node("qbar", 3);
        ctx.set_input_analog("dpwr", 3.0);
        ctx.set_input_analog("dgnd", 0.0);
        ctx.set_input_analog("t", 3.0);
        ctx.set_input_analog("clk", 0.0);
        ctx
    }

    fn legacy_gate_context(ic: Value, inputs: &[Value]) -> CmContext {
        let mut ctx = CmContext::new();
        for spec in q_parameters() {
            ctx.set_param(&spec.name, spec.default);
        }
        ctx.set_param("vref", 0.0);
        ctx.set_param("vlo", 0.0);
        ctx.set_param("vhi", 3.0);
        ctx.set_param("ic", ic);
        ctx.set_port_vector_terminals(
            "in",
            (0..inputs.len()).map(|index| (index + 1, 0)).collect(),
        );
        ctx.set_port_node("out", inputs.len() + 1);
        ctx.set_input_analog_vector("in", inputs)
            .expect("set legacy gate analog inputs");
        ctx.set_input_analog("out", 0.0);
        ctx
    }

    #[test]
    fn metadata_matches_xyce_u_tff_terminal_order() {
        let ports = XyceDTff.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["dpwr", "dgnd", "t", "clk", "q", "qbar"]
        );
        assert_eq!(ports[4].direction, PortDirection::InOut);
        assert_eq!(ports[4].default_type, PortType::Conductance);
        assert_eq!(XyceDTff.name(), MODEL_NAME);
    }

    #[test]
    fn high_state_stamps_xyce_parallel_rail_resistances() {
        let mut ctx = context();
        ctx.set_param("s0rlo", 200.0);
        ctx.set_param("s0rhi", 200.0);
        ctx.set_param("s1rlo", 200.0);
        ctx.set_param("s1rhi", 5.0);
        ctx.set_param("clo", 0.0);
        ctx.set_param("chi", 0.0);
        ctx.analysis = AnalysisType::DcOp;
        XyceDTff.init(&mut ctx).expect("initialize xyce_d_tff");
        XyceDTff.evaluate(&mut ctx).expect("evaluate xyce_d_tff");
        let stamps = ctx.take_stamps();
        let output_diag = stamps
            .iter()
            .filter(|(row, col, _)| *row == 1 && *col == 1)
            .map(|(_, _, value)| *value)
            .sum::<Value>();
        let high_rail_diag = stamps
            .iter()
            .filter(|(row, col, _)| *row == 3 && *col == 3)
            .map(|(_, _, value)| *value)
            .sum::<Value>();
        assert!(
            (output_diag - 0.205).abs() < 1.0e-12,
            "output G={output_diag}"
        );
        assert!(
            (high_rail_diag - 0.205).abs() < 1.0e-12,
            "dpwr G={high_rail_diag}"
        );
    }

    #[test]
    fn rising_clock_with_toggle_schedules_delayed_state_transition() {
        let mut ctx = context();
        ctx.set_param("delay", 20.0e-9);
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.timestep = 0.0;
        XyceDTff.init(&mut ctx).expect("initialize xyce_d_tff");
        XyceDTff
            .evaluate(&mut ctx)
            .expect("evaluate initial xyce_d_tff");
        ctx.advance_state();
        ctx.time = 100.0e-9;
        ctx.set_input_analog("clk", 3.0);
        XyceDTff.evaluate(&mut ctx).expect("evaluate xyce_d_tff");
        assert!(ctx.take_requested_breakpoints().contains(&(120.0e-9)));
    }

    #[test]
    fn legacy_y_gate_dcop_applies_authored_ic_after_truth_table() {
        let gate = XyceDLegacyGate::nand();

        // NAND(high, high) is low, but Xyce GateData::setIC applies the
        // authored high state while establishing DCOP.
        let mut high_ic = legacy_gate_context(1.0, &[3.0, 3.0]);
        high_ic.analysis = AnalysisType::DcOp;
        gate.init(&mut high_ic).expect("initialize high-IC YNAND");
        gate.evaluate(&mut high_ic)
            .expect("evaluate high-IC YNAND DCOP");
        assert_eq!(q_state(high_ic.state(XyceDLegacyGate::Q_STATE)), Some(true));

        // NAND(low, low) is high; an authored low state has the same canonical
        // precedence. Without an IC, the ordinary truth result remains active.
        let mut low_ic = legacy_gate_context(0.0, &[0.0, 0.0]);
        low_ic.analysis = AnalysisType::DcOp;
        gate.init(&mut low_ic).expect("initialize low-IC YNAND");
        gate.evaluate(&mut low_ic)
            .expect("evaluate low-IC YNAND DCOP");
        assert_eq!(q_state(low_ic.state(XyceDLegacyGate::Q_STATE)), Some(false));

        let mut no_ic = legacy_gate_context(f64::NAN, &[0.0, 0.0]);
        no_ic.analysis = AnalysisType::DcOp;
        gate.init(&mut no_ic)
            .expect("initialize uninitialized YNAND");
        gate.evaluate(&mut no_ic)
            .expect("evaluate uninitialized YNAND DCOP");
        assert_eq!(q_state(no_ic.state(XyceDLegacyGate::Q_STATE)), Some(true));
    }

    #[test]
    fn legacy_y_gate_transient_reconciles_dcop_ic_after_model_delay() {
        let gate = XyceDLegacyGate::nand();
        let delay = 20.0e-9;
        let mut ctx = legacy_gate_context(1.0, &[3.0, 3.0]);
        ctx.set_param("delay", delay);
        ctx.analysis = AnalysisType::DcOp;
        gate.init(&mut ctx).expect("initialize high-IC YNAND");
        gate.evaluate(&mut ctx)
            .expect("evaluate high-IC YNAND DCOP");
        assert_eq!(q_state(ctx.state(XyceDLegacyGate::Q_STATE)), Some(true));

        ctx.advance_state();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.timestep = 0.0;
        gate.evaluate(&mut ctx)
            .expect("schedule post-DCOP truth reconciliation");
        assert_eq!(q_state(ctx.state(XyceDLegacyGate::Q_STATE)), Some(true));
        assert!(ctx.take_requested_breakpoints().contains(&delay));

        ctx.advance_state();
        ctx.time = delay;
        ctx.timestep = delay;
        gate.evaluate(&mut ctx)
            .expect("apply delayed YNAND truth-table state");
        assert_eq!(q_state(ctx.state(XyceDLegacyGate::Q_STATE)), Some(false));
    }
}
