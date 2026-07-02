//! XSPICE extra event-driven code models.

use crate::Value;
use crate::xspice::{
    CmContext, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec, PortType,
};

const MIN_EVENT_DELAY: Value = 1.0e-15;
const REAL_GAIN_STARTUP_TIME: usize = 0;
const REAL_GAIN_STARTUP_STATE: usize = 0;
const REAL_GAIN_STARTUP_HOLDING: i64 = 1;
const REAL_GAIN_STARTUP_ACTIVE: i64 = 2;

fn digital_to_real_value(ctx: &CmContext) -> Value {
    let zero = ctx.param_or("zero", 0.0);
    let one = ctx.param_or("one", 1.0);
    let midpoint = 0.5 * (zero + one);
    let input = ctx.input_digital("in").unwrap_or_default();
    let enabled = ctx
        .input_digital("enable")
        .map(|value| value.state.is_high())
        .unwrap_or(true);

    if !enabled {
        0.0
    } else if input.state.is_low() {
        zero
    } else if input.state.is_unknown() {
        midpoint
    } else {
        one
    }
}

fn transition_value(
    time: Value,
    start_time: Value,
    start_value: Value,
    target: Value,
    transition_time: Value,
) -> Value {
    if (target - start_value).abs() < 1.0e-12 {
        return target;
    }
    if transition_time <= 0.0 {
        return target;
    }

    let fraction = ((time - start_time) / transition_time).clamp(0.0, 1.0);
    start_value + (target - start_value) * fraction
}

fn limited_event_delay(ctx: &CmContext, name: &str, default: Value) -> Value {
    ctx.param_or(name, default).max(MIN_EVENT_DELAY)
}

fn commit_event_outputs(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

fn real_to_v_output(
    time: Value,
    start_time: Value,
    start_value: Value,
    target: Value,
    transition_time: Value,
    gain: Value,
) -> Value {
    if transition_time <= 0.0 {
        return target * gain;
    }

    let end_time = start_time + transition_time;
    if time <= start_time {
        start_value * gain
    } else if time >= end_time {
        target * gain
    } else {
        let fraction = (time - start_time) / transition_time;
        (start_value + (target - start_value) * fraction) * gain
    }
}

fn set_real_output_when_changed(ctx: &mut CmContext, name: &str, value: Value, delay: Value) {
    if !commit_event_outputs(ctx) {
        return;
    }

    let changed = ctx
        .output_real(name)
        .is_none_or(|previous| (previous - value).abs() > 1.0e-12);
    if changed {
        ctx.set_output_real(name, value, delay);
    }
}

fn real_to_v_ports() -> &'static [PortSpec] {
    use std::sync::OnceLock;
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec::input("in", PortType::Real),
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
                description: "Analog output".to_string(),
            },
        ]
    })
}

/// Digital-to-real event converter.
#[derive(Debug, Default)]
pub struct DigitalToReal;

impl CodeModel for DigitalToReal {
    fn name(&self) -> &str {
        "d_to_real"
    }

    fn description(&self) -> &str {
        "Digital to real-valued event converter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::input("enable", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Real),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("zero", 0.0),
                ParamSpec::real("one", 1.0),
                ParamSpec::real("delay", 1.0e-9)
                    .with_description("Output event delay, clamped to official lower limit"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = if ctx.time > 0.0 {
            limited_event_delay(ctx, "delay", 1.0e-9)
        } else {
            0.0
        };
        set_real_output_when_changed(ctx, "out", digital_to_real_value(ctx), delay);
        Ok(())
    }
}

/// Real-valued event gain block.
#[derive(Debug, Default)]
pub struct RealGain;

impl CodeModel for RealGain {
    fn name(&self) -> &str {
        "real_gain"
    }

    fn description(&self) -> &str {
        "Real-valued event gain"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Real),
                PortSpec::output("out", PortType::Real),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0),
                ParamSpec::real("gain", 1.0),
                ParamSpec::real("out_offset", 0.0),
                ParamSpec::real("delay", 1.0e-9),
                ParamSpec::real("ic", 0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(1);
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.is_dc() {
            set_real_output_when_changed(ctx, "out", ctx.param_or("ic", 0.0), 0.0);
            return Ok(());
        }

        let delay = ctx.param_or("delay", 1.0e-9);
        let startup_state = ctx.int_state(REAL_GAIN_STARTUP_STATE);
        if ctx.is_transient() && startup_state != REAL_GAIN_STARTUP_ACTIVE {
            if startup_state == 0 {
                set_real_output_when_changed(ctx, "out", ctx.param_or("ic", 0.0), 0.0);
                if delay > 0.0 && delay.is_finite() {
                    let startup_time = ctx.time + delay;
                    if commit_event_outputs(ctx) {
                        ctx.set_state(REAL_GAIN_STARTUP_TIME, startup_time);
                        ctx.set_int_state(REAL_GAIN_STARTUP_STATE, REAL_GAIN_STARTUP_HOLDING);
                        ctx.request_breakpoint(startup_time);
                    }
                    return Ok(());
                }
                if commit_event_outputs(ctx) {
                    ctx.set_int_state(REAL_GAIN_STARTUP_STATE, REAL_GAIN_STARTUP_ACTIVE);
                }
            } else {
                let startup_time = ctx.state(REAL_GAIN_STARTUP_TIME);
                if ctx.time < startup_time {
                    if commit_event_outputs(ctx) {
                        ctx.request_breakpoint(startup_time);
                    }
                    return Ok(());
                }
                if commit_event_outputs(ctx) {
                    ctx.set_int_state(REAL_GAIN_STARTUP_STATE, REAL_GAIN_STARTUP_ACTIVE);
                }
            }
        }

        let value = ctx.param_or("gain", 1.0)
            * (ctx.input_real("in").unwrap_or(0.0) + ctx.param_or("in_offset", 0.0))
            + ctx.param_or("out_offset", 0.0);
        set_real_output_when_changed(ctx, "out", value, delay);
        Ok(())
    }
}

/// Real-valued event delay sampled by a digital rising clock edge.
#[derive(Debug, Default)]
pub struct RealDelay;

impl CodeModel for RealDelay {
    fn name(&self) -> &str {
        "real_delay"
    }

    fn description(&self) -> &str {
        "Clocked real-valued event delay"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Real),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::output("out", PortType::Real),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("delay", 1.0e-9)
                    .with_description("Clock-to-output delay, clamped to official lower limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        let clk_is_one = ctx.input_digital("clk").unwrap_or_default().state.is_high();
        ctx.set_int_state(0, i64::from(clk_is_one));
        Ok(())
    }

    fn can_skip_unchanged_event_inputs(&self) -> bool {
        true
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if !ctx.is_transient() {
            return Ok(());
        }

        let clk_is_one = ctx.input_digital("clk").unwrap_or_default().state.is_high();
        let prev_clk_is_one = ctx.int_state(0) != 0;
        let commit_outputs = commit_event_outputs(ctx);
        if commit_outputs && clk_is_one && !prev_clk_is_one {
            ctx.set_output_real(
                "out",
                ctx.input_real("in").unwrap_or(0.0),
                limited_event_delay(ctx, "delay", 1.0e-9),
            );
        }
        if commit_outputs {
            ctx.set_int_state(0, i64::from(clk_is_one));
        }
        Ok(())
    }
}

/// Real-valued event to analog output converter.
#[derive(Debug, Default)]
pub struct RealToVoltage;

impl CodeModel for RealToVoltage {
    fn name(&self) -> &str {
        "real_to_v"
    }

    fn description(&self) -> &str {
        "Real-valued event to analog output converter"
    }

    fn ports(&self) -> &[PortSpec] {
        real_to_v_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("gain", 1.0),
                ParamSpec::real("transition_time", 1.0e-9)
                    .with_description("Output transition time, clamped to official lower limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(4);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param_or("gain", 1.0);
        let target = ctx.input_real("in").unwrap_or(0.0);
        let target_output = target * gain;

        if !ctx.is_transient() {
            ctx.set_initial_state(0, target_output);
            ctx.set_initial_state(1, target);
            ctx.set_initial_state(2, ctx.time);
            ctx.set_initial_state(3, target);
            ctx.set_output("out", target_output);
            return Ok(());
        }

        let transition_time = limited_event_delay(ctx, "transition_time", 1.0e-9);
        let accepted_output = ctx.state_prev(0);
        let accepted_target = ctx.state_prev(1);
        let accepted_start_time = ctx.state_prev(2);
        let accepted_start_value = ctx.state_prev(3);
        let first_transient_point = ctx.time == 0.0;

        let (next_target, next_start_time, next_start_value) = if first_transient_point {
            (target, ctx.time, target)
        } else if (target - accepted_target).abs() > 1.0e-12 {
            let event_time = ctx.input_real_event_time("in").unwrap_or(ctx.time);
            let start_value = transition_value(
                event_time,
                accepted_start_time,
                accepted_start_value,
                accepted_target,
                transition_time,
            );
            (target, event_time, start_value)
        } else {
            (accepted_target, accepted_start_time, accepted_start_value)
        };

        let output = if first_transient_point {
            target_output
        } else {
            real_to_v_output(
                ctx.time,
                next_start_time,
                next_start_value,
                next_target,
                transition_time,
                gain,
            )
        };

        if commit_event_outputs(ctx) {
            ctx.set_state(0, output);
            ctx.set_state(1, next_target);
            ctx.set_state(2, next_start_time);
            ctx.set_state(3, next_start_value);
        }
        ctx.set_output(
            "out",
            if output.is_finite() {
                output
            } else {
                accepted_output
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{AnalysisType, DigitalValue};

    #[test]
    fn d_to_real_unknown_enable_disables_output_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_param("zero", -2.0);
        ctx.set_param("one", 5.0);
        ctx.set_input_digital("in", DigitalValue::one());
        ctx.set_input_digital("enable", DigitalValue::unknown());

        DigitalToReal
            .evaluate(&mut ctx)
            .expect("d_to_real evaluates");

        assert_eq!(
            ctx.output_real("out"),
            Some(0.0),
            "ngspice only enables d_to_real when enable is exactly ONE"
        );
    }

    #[test]
    fn real_delay_initial_high_clock_does_not_sample_without_edge_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_input_real("in", 7.0);
        ctx.set_input_digital("clk", DigitalValue::one());

        RealDelay.init(&mut ctx).expect("real_delay initializes");
        ctx.analysis = AnalysisType::Transient;
        RealDelay.evaluate(&mut ctx).expect("real_delay evaluates");

        assert_eq!(
            ctx.output_real("out"),
            None,
            "ngspice initializes real_delay's previous clock from INPUT_STATE(clk)"
        );
    }

    #[test]
    fn real_delay_does_not_commit_rollbackable_probe_edge() {
        let mut ctx = CmContext::new();
        ctx.set_input_real("in", 7.0);
        ctx.set_input_digital("clk", DigitalValue::zero());

        RealDelay.init(&mut ctx).expect("real_delay initializes");
        ctx.analysis = AnalysisType::Transient;
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);

        RealDelay
            .evaluate(&mut ctx)
            .expect("rollbackable real_delay probe evaluates");

        assert_eq!(
            ctx.output_real("out"),
            None,
            "rollbackable probes must not schedule real_delay real events"
        );
        assert_eq!(
            ctx.int_state(0),
            0,
            "rollbackable probes must not consume the rising clock edge"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        RealDelay
            .evaluate(&mut ctx)
            .expect("accepted real_delay step evaluates");
        assert_eq!(ctx.output_real("out"), Some(7.0));
        assert_eq!(ctx.int_state(0), 1);
    }

    #[test]
    fn real_gain_does_not_commit_rollbackable_probe_startup() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("delay", 1.0e-9);
        ctx.set_param("ic", 2.0);
        RealGain.init(&mut ctx).expect("real_gain initializes");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);

        RealGain
            .evaluate(&mut ctx)
            .expect("rollbackable real_gain probe evaluates");

        assert_eq!(
            ctx.output_real("out"),
            None,
            "rollbackable probes must not enqueue real_gain startup events"
        );
        assert_eq!(
            ctx.state(REAL_GAIN_STARTUP_TIME),
            0.0,
            "rollbackable probes must not commit startup time"
        );
        assert_eq!(
            ctx.int_state(REAL_GAIN_STARTUP_STATE),
            0,
            "rollbackable probes must not commit startup phase"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable probes must not leave startup breakpoints behind"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        RealGain
            .evaluate(&mut ctx)
            .expect("accepted real_gain step evaluates");

        assert_eq!(ctx.output_real("out"), Some(2.0));
        assert!((ctx.state(REAL_GAIN_STARTUP_TIME) - 1.0e-9).abs() < 1.0e-18);
        assert_eq!(
            ctx.int_state(REAL_GAIN_STARTUP_STATE),
            REAL_GAIN_STARTUP_HOLDING
        );
        assert_eq!(ctx.take_requested_breakpoints(), vec![1.0e-9]);
    }

    #[test]
    fn real_to_v_gain_scales_the_full_mid_transition_value() {
        let mut ctx = CmContext::new();
        ctx.set_param("gain", 2.0);
        ctx.set_param("transition_time", 1.0e-9);
        ctx.analysis = AnalysisType::Transient;

        RealToVoltage.init(&mut ctx).expect("real_to_v initializes");

        ctx.set_input_real("in", 1.0);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v evaluates initial transient point");
        ctx.advance_state();

        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;
        ctx.set_input_real("in", 3.0);
        ctx.set_input_real_event_time("in", ctx.time);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v starts transition");
        ctx.advance_state();

        ctx.time = 1.5e-9;
        ctx.timestep = 0.5e-9;
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v evaluates transition midpoint");

        let out = ctx.output("out");
        assert!(
            (out - 4.0).abs() < 1.0e-12,
            "real_to_v should interpolate the real value and then apply gain, got {out}"
        );
    }

    #[test]
    fn real_to_v_first_nonzero_event_starts_ramp_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_param("gain", 1.0);
        ctx.set_param("transition_time", 1.0e-9);
        ctx.analysis = AnalysisType::Transient;

        RealToVoltage.init(&mut ctx).expect("real_to_v initializes");

        ctx.set_input_real("in", 0.0);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v evaluates initial transient point");
        ctx.advance_state();

        ctx.time = 1.0e-12;
        ctx.timestep = 1.0e-12;
        ctx.set_input_real("in", 4.0);
        ctx.set_input_real_event_time("in", ctx.time);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v starts first nonzero transition");

        let out = ctx.output("out");
        assert!(
            out.abs() < 1.0e-12,
            "ngspice only jumps immediately at TIME == 0; first nonzero event should start at the old value, got {out}"
        );
    }

    #[test]
    fn real_to_v_does_not_commit_rollbackable_probe_transition_state() {
        let mut ctx = CmContext::new();
        ctx.set_param("gain", 1.0);
        ctx.set_param("transition_time", 1.0e-9);
        ctx.analysis = AnalysisType::Transient;

        RealToVoltage.init(&mut ctx).expect("real_to_v initializes");
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        ctx.set_input_real("in", 0.0);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("real_to_v evaluates initial transient point");
        ctx.advance_state();

        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;
        ctx.set_input_real("in", 4.0);
        ctx.set_input_real_event_time("in", ctx.time);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("rollbackable real_to_v probe evaluates");

        assert_eq!(
            ctx.state(1),
            0.0,
            "rollbackable probes must not commit a new real_to_v event target"
        );
        assert_eq!(
            ctx.state(2),
            0.0,
            "rollbackable probes must not commit a new real_to_v transition start time"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("accepted real_to_v step evaluates");

        assert_eq!(ctx.state(1), 4.0);
        assert_eq!(ctx.state(2), 1.0e-9);
    }
}
