//! XSPICE extra event-driven code models.

use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec,
    PortType,
};

const MIN_EVENT_DELAY: Value = 1.0e-15;

fn digital_to_real_value(ctx: &CmContext) -> CmResult<Value> {
    let zero = finite_event_param(ctx, "d_to_real", "zero", 0.0)?;
    let one = finite_event_param(ctx, "d_to_real", "one", 1.0)?;
    let midpoint = 0.5 * (zero + one);
    let input = ctx.input_digital("in").unwrap_or_default();
    let enabled = ctx
        .input_digital("enable")
        .map(|value| value.state.is_high())
        .unwrap_or(true);

    Ok(if !enabled {
        0.0
    } else if input.state.is_low() {
        zero
    } else if input.state.is_high() {
        one
    } else {
        midpoint
    })
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

fn finite_event_param(ctx: &CmContext, model: &str, name: &str, default: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if !value.is_finite() {
        return Err(CmError::EvaluationError(format!(
            "{model}: {name} must be finite, got {value}"
        )));
    }
    Ok(value)
}

fn limited_event_delay(
    ctx: &CmContext,
    model: &str,
    name: &str,
    default: Value,
) -> CmResult<Value> {
    Ok(finite_event_param(ctx, model, name, default)?.max(MIN_EVENT_DELAY))
}

fn finite_event_delay(ctx: &CmContext, model: &str, name: &str, default: Value) -> CmResult<Value> {
    finite_event_param(ctx, model, name, default)
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
        // ngspice's real_to_v model applies gain to the ramp delta only.
        start_value + (target - start_value) * fraction * gain
    }
}

fn real_to_v_completion_time(
    start_time: Value,
    start_value: Value,
    target: Value,
    transition_time: Value,
) -> Option<Value> {
    if (target - start_value).abs() <= 1.0e-12
        || !(start_time.is_finite()
            && start_value.is_finite()
            && target.is_finite()
            && transition_time.is_finite()
            && transition_time > 0.0)
    {
        return None;
    }

    Some(start_time + transition_time)
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
            limited_event_delay(ctx, "d_to_real", "delay", 1.0e-9)?
        } else {
            0.0
        };
        set_real_output_when_changed(ctx, "out", digital_to_real_value(ctx)?, delay);
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

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let ic = finite_event_param(ctx, "real_gain", "ic", 0.0)?;
        if ctx.is_dc() {
            set_real_output_when_changed(ctx, "out", ic, 0.0);
            return Ok(());
        }

        let delay = finite_event_delay(ctx, "real_gain", "delay", 1.0e-9)?;
        let gain = finite_event_param(ctx, "real_gain", "gain", 1.0)?;
        let in_offset = finite_event_param(ctx, "real_gain", "in_offset", 0.0)?;
        let out_offset = finite_event_param(ctx, "real_gain", "out_offset", 0.0)?;
        let value = gain * (ctx.input_real("in").unwrap_or(0.0) + in_offset) + out_offset;
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
                limited_event_delay(ctx, "real_delay", "delay", 1.0e-9)?,
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

/// Ngspice-compatible alias used by real-valued auto-bridge decks.
#[derive(Debug, Default)]
pub struct RealToVoltageAlias;

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
        let gain = finite_event_param(ctx, "real_to_v", "gain", 1.0)?;
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

        let transition_time = limited_event_delay(ctx, "real_to_v", "transition_time", 1.0e-9)?;
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
            if let Some(completion_time) = real_to_v_completion_time(
                next_start_time,
                next_start_value,
                next_target,
                transition_time,
            ) && completion_time > ctx.time + 1.0e-18
            {
                ctx.request_breakpoint(completion_time);
            }
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

impl CodeModel for RealToVoltageAlias {
    fn name(&self) -> &str {
        "r_to_v"
    }

    fn description(&self) -> &str {
        RealToVoltage.description()
    }

    fn ports(&self) -> &[PortSpec] {
        RealToVoltage.ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        RealToVoltage.parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        RealToVoltage.init(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        RealToVoltage.evaluate(ctx)
    }
}

/// Analog input observed into a real-valued event.
///
/// The mirror of [`RealToVoltage`], and the direction the XSPICE estate did not
/// have: every other analog-observing bridge produces four-state values.
///
/// # The sampling ruling
///
/// **`v_to_real` samples its analog input at every accepted analog step and
/// publishes the sample as a real event with no delay. It requests no
/// breakpoints.**
///
/// The estate's other analog-observing bridges — `adc_bridge`, `bidi_bridge`,
/// and the Xyce `DIG` family — observe by *threshold crossing*, and
/// `models::digital_output`'s `input_transition_time` interpolates that
/// crossing inside the accepted step so the transition is dated at a real
/// instant of the analog solution. That is D5 clause 5, and none of it
/// transfers to a real-valued observer. A crossing is an event because a
/// threshold turns a continuous quantity into a discrete one, and the moment it
/// changes is well defined. A real net carries the quantity *itself*, whose
/// next change is not an event and has no time to interpolate for.
///
/// So there is no crossing to date, and the two limits of sampling are stated
/// rather than hidden:
///
/// * The observed value is a **staircase at the analog step sequence**. The
///   event world's view of an analog node is as fine as the accepted steps and
///   no finer.
/// * The model **names no future time**, because it has none to name. It
///   therefore requests no breakpoints and cannot move the accepted step
///   sequence by observing, which is what keeps D5 clause 6 — pure-analog
///   inertness — true of a deck it appears in.
///
/// [`RealToVoltage`] is deliberately the contrast: the same boundary crossed
/// the other way, and it *does* know a future time — its own ramp completion —
/// and asks for it.
///
/// # Not an ngspice model
///
/// ngspice 46 ships `d_to_real`, `real_gain`, `real_delay` and `real_to_v`, and
/// no observer at all — a real event value can be produced from bits and
/// consumed by the matrix, but an analog node cannot be read into one. This
/// fills that gap, so it takes RSpice's own reading rather than a foreign one,
/// and `OFFICIAL_NGSPICE_46_XSPICE_MODELS` deliberately does not list it.
///
/// One consequence of not being ngspice's is worth naming, because the sibling
/// model is a trap: `real_to_v`'s `gain` scales *the ramp delta only*, which is
/// ngspice's behaviour and is pinned as such. This model's `gain` is a plain
/// multiplication of the sampled value, because there is no ramp for it to
/// apply to and no foreign behaviour to match.
/// `pub(crate)` where its siblings are `pub`, and deliberately: the registry
/// erases it to `Arc<dyn CodeModel>` at registration and a deck names it by
/// string, so nothing outside this crate has a use for the type itself.
#[derive(Debug, Default)]
pub(crate) struct VoltageToReal;

impl CodeModel for VoltageToReal {
    fn name(&self) -> &str {
        "v_to_real"
    }

    fn description(&self) -> &str {
        "Analog input sampled into a real-valued event"
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
                    // The same set `gain` admits. A real event carries a
                    // number, not a unit, so a current is as observable as a
                    // voltage and nothing here has to know which it was.
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
                    description: "Analog input".to_string(),
                },
                PortSpec::output("out", PortType::Real),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            // `gain` and nothing else. Offsets and further scaling are
            // `real_gain`'s, which already exists and composes; duplicating its
            // parameters here would give one arithmetic two spellings.
            //
            // In particular there is no `delay`. A delayed sample would land at
            // a future time, which is a breakpoint request, which is exactly
            // what `V_TO_REAL_SAMPLING_RULING` says an observer must not make.
            vec![
                ParamSpec::real("gain", 1.0)
                    .with_description("Scale applied to the sampled analog value"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = finite_event_param(ctx, "v_to_real", "gain", 1.0)?;
        let sample = gain * ctx.input("in");
        // Zero delay: the sample *is* the node's value now, so dating it later
        // would report a value the analog side never held at that time.
        //
        // The change test is the real-event family's — `set_real_output_when_changed`
        // — rather than a rule invented here, and it is what stops the last few
        // Newton iterates at one timepoint from each publishing an event that
        // differs only in the far decimals. It is an *observer's* threshold and
        // is a different thing from the tolerance-free change test Verilog-AMS
        // LRM 2.4 section 3.7 requires inside a `wreal` store, which compares a
        // value the design computed rather than one a solver converged to.
        set_real_output_when_changed(ctx, "out", sample, 0.0);
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
    fn d_to_real_high_z_input_maps_to_unknown_midpoint_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_param("zero", -2.0);
        ctx.set_param("one", 4.0);
        ctx.set_input_digital("in", DigitalValue::high_z());

        DigitalToReal
            .evaluate(&mut ctx)
            .expect("d_to_real evaluates");

        assert_eq!(ctx.output_real("out"), Some(1.0));
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
    fn bounded_xtraevt_delays_reject_nonfinite_values() {
        let mut d_to_real = CmContext::new();
        d_to_real.analysis = AnalysisType::Transient;
        d_to_real.time = 1.0e-9;
        d_to_real.set_param("delay", f64::INFINITY);
        d_to_real.set_input_digital("in", DigitalValue::one());
        let err = DigitalToReal
            .evaluate(&mut d_to_real)
            .expect_err("d_to_real must reject nonfinite delay");
        assert!(
            err.to_string().contains("delay must be finite"),
            "unexpected d_to_real error: {err:?}"
        );
        assert_eq!(d_to_real.output_real("out"), None);

        let mut real_delay = CmContext::new();
        real_delay.analysis = AnalysisType::Transient;
        real_delay.set_param("delay", f64::INFINITY);
        real_delay.set_input_real("in", 7.0);
        real_delay.set_input_digital("clk", DigitalValue::zero());
        RealDelay.init(&mut real_delay).expect("real_delay init");
        real_delay.set_input_digital("clk", DigitalValue::one());
        let err = RealDelay
            .evaluate(&mut real_delay)
            .expect_err("real_delay must reject nonfinite delay");
        assert!(
            err.to_string().contains("delay must be finite"),
            "unexpected real_delay error: {err:?}"
        );
        assert_eq!(real_delay.output_real("out"), None);

        let mut real_to_v = CmContext::new();
        real_to_v.analysis = AnalysisType::Transient;
        real_to_v.set_param("transition_time", f64::INFINITY);
        real_to_v.set_input_real("in", 1.0);
        RealToVoltage.init(&mut real_to_v).expect("real_to_v init");
        let err = RealToVoltage
            .evaluate(&mut real_to_v)
            .expect_err("real_to_v must reject nonfinite transition_time");
        assert!(
            err.to_string().contains("transition_time must be finite"),
            "unexpected real_to_v error: {err:?}"
        );
        assert!(real_to_v.take_requested_breakpoints().is_empty());
    }

    #[test]
    fn real_gain_transient_outputs_immediately_without_startup_hold() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("delay", 1.0e-9);
        ctx.set_param("gain", 10.0);
        ctx.set_input_real("in", 2.0);
        RealGain.init(&mut ctx).expect("real_gain initializes");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);

        RealGain
            .evaluate(&mut ctx)
            .expect("rollbackable real_gain probe evaluates");

        assert_eq!(
            ctx.output_real("out"),
            None,
            "rollbackable probes must not enqueue real_gain events"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "real_gain should not use startup breakpoints"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        RealGain
            .evaluate(&mut ctx)
            .expect("accepted real_gain step evaluates");

        assert_eq!(ctx.output_real("out"), Some(20.0));
        let events = ctx.take_pending_real_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![20.0]);
        assert_eq!(events[0].delay, 1.0e-9);
        assert!(ctx.take_requested_breakpoints().is_empty());
    }

    #[test]
    fn real_gain_rejects_nonfinite_delay_before_queueing_outputs() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("delay", f64::NAN);
        ctx.set_input_real("in", 3.0);
        RealGain.init(&mut ctx).expect("real_gain initializes");

        let err = RealGain
            .evaluate(&mut ctx)
            .expect_err("nonfinite real_gain delay must fail evaluation");

        assert!(
            err.to_string().contains("delay must be finite"),
            "error should identify nonfinite delay, got {err:?}"
        );
        assert_eq!(
            ctx.output_real("out"),
            None,
            "real_gain must not queue outputs after rejecting delay"
        );
    }

    #[test]
    fn xtraevt_scalar_params_reject_nonfinite_values() {
        for name in ["zero", "one"] {
            let mut ctx = CmContext::new();
            ctx.set_param(name, f64::INFINITY);
            ctx.set_input_digital("in", DigitalValue::one());

            let err = DigitalToReal
                .evaluate(&mut ctx)
                .expect_err("d_to_real must reject nonfinite scalar params");

            assert!(
                err.to_string().contains(name),
                "d_to_real error should identify {name}, got {err:?}"
            );
            assert_eq!(ctx.output_real("out"), None);
        }

        for name in ["ic", "gain", "in_offset", "out_offset"] {
            let mut ctx = CmContext::new();
            ctx.analysis = AnalysisType::Transient;
            ctx.set_input_real("in", 3.0);
            RealGain.init(&mut ctx).expect("real_gain initializes");
            ctx.set_param(name, f64::NAN);

            let err = RealGain
                .evaluate(&mut ctx)
                .expect_err("real_gain must reject nonfinite scalar params");

            assert!(
                err.to_string().contains(name),
                "real_gain error should identify {name}, got {err:?}"
            );
            assert_eq!(ctx.output_real("out"), None);
        }

        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("gain", f64::INFINITY);
        ctx.set_input_real("in", 1.0);
        RealToVoltage.init(&mut ctx).expect("real_to_v initializes");

        let err = RealToVoltage
            .evaluate(&mut ctx)
            .expect_err("real_to_v must reject nonfinite gain");

        assert!(
            err.to_string().contains("gain"),
            "real_to_v error should identify gain, got {err:?}"
        );
        assert_eq!(ctx.output("out"), 0.0);
    }

    #[test]
    fn real_to_v_gain_scales_only_the_mid_transition_delta_like_ngspice() {
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
            (out - 3.0).abs() < 1.0e-12,
            "ngspice real_to_v applies gain only to the ramp delta term, got {out}"
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
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable probes must not schedule real_to_v completion breakpoints"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        RealToVoltage
            .evaluate(&mut ctx)
            .expect("accepted real_to_v step evaluates");

        assert_eq!(ctx.state(1), 4.0);
        assert_eq!(ctx.state(2), 1.0e-9);
        assert_eq!(ctx.take_requested_breakpoints(), vec![2.0e-9]);
    }

    // =======================================================================
    // v_to_real — the observer half of the boundary
    // =======================================================================

    /// A context holding one analog sample, at a transient timepoint.
    fn observer_at(time: Value, sample: Value) -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = time;
        ctx.set_input_analog("in", sample);
        ctx
    }

    /// The sample is the analog value scaled by `gain`, and `gain` is a plain
    /// multiplication.
    ///
    /// Stated as a test because the sibling is a trap: `real_to_v`'s `gain`
    /// scales only the ramp delta, which is ngspice's behaviour and is pinned
    /// by `real_to_v_gain_scales_only_the_mid_transition_delta_like_ngspice`.
    /// This model has no ramp, is not ngspice's, and multiplies.
    #[test]
    fn v_to_real_publishes_the_gain_scaled_sample() {
        let mut ctx = observer_at(1.0e-9, 1.25);
        ctx.set_param("gain", 4.0);

        VoltageToReal.init(&mut ctx).expect("v_to_real initializes");
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("v_to_real evaluates");

        assert_eq!(ctx.output_real("out"), Some(5.0));
    }

    /// The observer names no future time, so it requests no breakpoints and
    /// publishes at zero delay.
    ///
    /// This is the sampling ruling on [`VoltageToReal`] in its machine-checked
    /// form, and it is what keeps D5 clause 6 — pure-analog inertness — true of
    /// a deck the observer appears in: a model that asked for a breakpoint
    /// would move the accepted step sequence merely by watching.
    #[test]
    fn v_to_real_requests_no_breakpoints_and_publishes_without_delay() {
        let mut ctx = observer_at(3.0e-9, 2.0);

        VoltageToReal.init(&mut ctx).expect("v_to_real initializes");
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("v_to_real evaluates");

        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "an observer of a continuous quantity has no future time to name"
        );
        let events = ctx.take_pending_real_events();
        assert_eq!(events.len(), 1, "one sample is one event");
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].values, vec![2.0]);
        assert_eq!(
            events[0].delay, 0.0,
            "the sample is the node's value now; dating it later would report a \
             value the analog side never held at that time"
        );
    }

    /// A sample that has not moved produces no event.
    ///
    /// The hold discipline: a real event value stands until the next event on
    /// its net, so republishing what the net already carries would restart a
    /// `real_to_v` ramp and cost a settle pass for nothing. The change test is
    /// the real-event family's shared one, not a rule this model invents.
    #[test]
    fn v_to_real_republishes_only_when_the_sample_moves() {
        let mut ctx = observer_at(1.0e-9, 2.0);

        VoltageToReal.init(&mut ctx).expect("v_to_real initializes");
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("v_to_real evaluates the first sample");
        assert_eq!(ctx.take_pending_real_events().len(), 1);

        // Same node voltage at a later timepoint: nothing to say.
        ctx.time = 2.0e-9;
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("v_to_real evaluates an unchanged sample");
        assert!(
            ctx.take_pending_real_events().is_empty(),
            "an unchanged sample must not republish"
        );

        // And a moved one does.
        ctx.time = 3.0e-9;
        ctx.set_input_analog("in", 2.5);
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("v_to_real evaluates a moved sample");
        let events = ctx.take_pending_real_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![2.5]);
    }

    /// A rollbackable probe observes without publishing.
    ///
    /// D5 clause 1: a timepoint the integrator ultimately rejects must leave
    /// the event world exactly as it found it. The observer is gated by the
    /// same `commit_event_outputs` check its siblings use, so a discarded
    /// trial's sample never becomes an event — and the accepted evaluation that
    /// follows still publishes, rather than being suppressed by a value the
    /// probe recorded.
    #[test]
    fn v_to_real_does_not_publish_from_a_rollbackable_probe() {
        let mut ctx = observer_at(1.0e-9, 7.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);

        VoltageToReal.init(&mut ctx).expect("v_to_real initializes");
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("rollbackable v_to_real probe evaluates");

        assert_eq!(
            ctx.output_real("out"),
            None,
            "rollbackable probes must not publish a sample"
        );
        assert!(ctx.take_pending_real_events().is_empty());

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        VoltageToReal
            .evaluate(&mut ctx)
            .expect("accepted v_to_real step evaluates");
        assert_eq!(ctx.output_real("out"), Some(7.0));
        assert_eq!(ctx.take_pending_real_events().len(), 1);
    }

    /// A nonfinite `gain` is refused by name rather than propagated into the
    /// event world, where a NaN would become one net's resolved value and
    /// spread through every driver summed onto it.
    #[test]
    fn v_to_real_rejects_a_nonfinite_gain() {
        let mut ctx = observer_at(1.0e-9, 1.0);
        ctx.set_param("gain", f64::NAN);

        VoltageToReal.init(&mut ctx).expect("v_to_real initializes");
        let err = VoltageToReal
            .evaluate(&mut ctx)
            .expect_err("v_to_real must reject a nonfinite gain");
        assert!(
            err.to_string().contains("v_to_real: gain must be finite"),
            "the refusal should name the model and the parameter, got {err:?}"
        );
        assert_eq!(ctx.output_real("out"), None);
        assert!(ctx.take_pending_real_events().is_empty());
    }
}
