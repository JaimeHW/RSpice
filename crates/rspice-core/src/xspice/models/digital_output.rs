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
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec,
    PortType,
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
}
