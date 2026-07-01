use crate::Value;
use crate::xspice::EvaluationPhase;

use super::*;

//=============================================================================
// Flip-Flops and Latches
//=============================================================================

const OFFICIAL_SEQUENTIAL_DELAY_MIN: Value = 1.0e-12;
const OFFICIAL_IC_MIN: i64 = 0;
const OFFICIAL_IC_MAX: i64 = 2;

fn official_sequential_delay(ctx: &CmContext, name: &str) -> Value {
    ctx.param(name).max(OFFICIAL_SEQUENTIAL_DELAY_MIN)
}

fn sequential_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

fn official_min_integer_param(ctx: &CmContext, name: &str, min: i64) -> i64 {
    (ctx.param(name).round() as i64).max(min)
}

fn input_code(value: Option<DigitalValue>) -> i64 {
    value.map(logic_code).unwrap_or(0)
}

fn logic_code(value: DigitalValue) -> i64 {
    match value.state.logic_level() {
        Some(false) => 0,
        Some(true) => 1,
        None => -1,
    }
}

fn output_value(code: i64) -> DigitalValue {
    match code {
        0 => DigitalValue::zero(),
        1 => DigitalValue::one(),
        _ => DigitalValue::unknown(),
    }
}

fn ic_code(ctx: &CmContext) -> i64 {
    match (ctx.param("ic").round() as i64).clamp(OFFICIAL_IC_MIN, OFFICIAL_IC_MAX) {
        0 => 0,
        1 => 1,
        _ => -1,
    }
}

fn invert_code(code: i64) -> i64 {
    match code {
        0 => 1,
        1 => 0,
        _ => -1,
    }
}

fn sr_result(set_input: i64, reset_input: i64, old_output: i64) -> i64 {
    match set_input {
        0 => match reset_input {
            0 => old_output,
            1 => 0,
            _ => -1,
        },
        1 => match reset_input {
            0 => 1,
            1 => -1,
            _ => -1,
        },
        _ => -1,
    }
}

fn transition_delays(ctx: &CmContext, old: i64, new: i64, base_delay: Value) -> (Value, Value) {
    let rise_delay = official_sequential_delay(ctx, "rise_delay");
    let fall_delay = official_sequential_delay(ctx, "fall_delay");
    match new {
        0 => (base_delay + fall_delay, base_delay + rise_delay),
        1 => (base_delay + rise_delay, base_delay + fall_delay),
        _ if old == 0 => (base_delay + rise_delay, base_delay + fall_delay),
        _ => (base_delay + fall_delay, base_delay + rise_delay),
    }
}

fn drive_outputs(ctx: &mut CmContext, output: i64, out_delay: Value, nout_delay: Value) {
    if ctx.port_width("out") > 0 {
        ctx.set_output_digital("out", output_value(output), out_delay);
    }
    if ctx.port_width("Nout") > 0 {
        ctx.set_output_digital("Nout", output_value(invert_code(output)), nout_delay);
    }
}

fn initial_async_output(ic: i64, set: i64, reset: i64) -> i64 {
    let mut output = ic;
    if set == 1 && reset == 0 {
        output = 1;
    }
    if set == 0 && reset == 1 {
        output = 0;
    }
    if set == 1 && reset == 1 {
        output = -1;
    }
    output
}

fn set_edge_output(old: i64, set: i64, reset: i64) -> i64 {
    match set {
        1 if reset != 1 => 1,
        1 => -1,
        0 | -1 if reset == 1 => 0,
        _ => old,
    }
}

fn reset_edge_output(old: i64, set: i64, reset: i64) -> i64 {
    match reset {
        1 if set != 1 => 0,
        1 => -1,
        0 | -1 if set == 1 => 1,
        _ => old,
    }
}

const EDGE_PREV_CLK: usize = 0;
const EDGE_PREV_SET: usize = 1;
const EDGE_PREV_RESET: usize = 2;
const EDGE_PREV_OUT: usize = 3;
const EDGE_STATE_COUNT: usize = 4;

fn edge_parameters(input_load: &'static str) -> Vec<ParamSpec> {
    vec![
        ParamSpec::real("clk_delay", 1e-9)
            .with_description("Clock propagation delay, clamped to official lower limit"),
        ParamSpec::real("set_delay", 1e-9)
            .with_description("Set propagation delay, clamped to official lower limit"),
        ParamSpec::real("reset_delay", 1e-9)
            .with_description("Reset propagation delay, clamped to official lower limit"),
        ParamSpec::integer("ic", 0)
            .with_description("Initial output state, clamped to official range"),
        ParamSpec::real("rise_delay", 1e-9)
            .with_description("Rise propagation delay, clamped to official lower limit"),
        ParamSpec::real("fall_delay", 1e-9)
            .with_description("Fall propagation delay, clamped to official lower limit"),
        ParamSpec::real(input_load, 1e-12),
        ParamSpec::real("clk_load", 1e-12),
        ParamSpec::real("set_load", 1e-12),
        ParamSpec::real("reset_load", 1e-12),
    ]
}

fn evaluate_edge_model(
    ctx: &mut CmContext,
    next_on_rising_clock: fn(&CmContext, i64) -> i64,
) -> CmResult<()> {
    let clk = input_code(ctx.input_digital("clk"));
    let set = input_code(ctx.input_digital("set"));
    let reset = input_code(ctx.input_digital("reset"));

    let old_clk = ctx.int_state(EDGE_PREV_CLK);
    let old_set = ctx.int_state(EDGE_PREV_SET);
    let old_reset = ctx.int_state(EDGE_PREV_RESET);
    let old_output = ctx.int_state(EDGE_PREV_OUT);

    let output = if ctx.time == 0.0 {
        let initial = initial_async_output(ic_code(ctx), set, reset);
        drive_outputs(ctx, initial, 0.0, 0.0);
        initial
    } else {
        let (next, base_delay) = if set != old_set {
            (
                set_edge_output(old_output, set, reset),
                Some(official_sequential_delay(ctx, "set_delay")),
            )
        } else if reset != old_reset {
            (
                reset_edge_output(old_output, set, reset),
                Some(official_sequential_delay(ctx, "reset_delay")),
            )
        } else if clk != old_clk && reset != 1 && set != 1 {
            if clk == 1 {
                (
                    next_on_rising_clock(ctx, old_output),
                    Some(official_sequential_delay(ctx, "clk_delay")),
                )
            } else {
                (old_output, None)
            }
        } else {
            (old_output, None)
        };

        if next != old_output
            && let Some(base_delay) = base_delay
        {
            let (out_delay, nout_delay) = transition_delays(ctx, old_output, next, base_delay);
            drive_outputs(ctx, next, out_delay, nout_delay);
        }

        next
    };

    sequential_set_int_state(ctx, EDGE_PREV_CLK, clk);
    sequential_set_int_state(ctx, EDGE_PREV_SET, set);
    sequential_set_int_state(ctx, EDGE_PREV_RESET, reset);
    sequential_set_int_state(ctx, EDGE_PREV_OUT, output);
    Ok(())
}

fn dff_next_on_rising_clock(ctx: &CmContext, _old_output: i64) -> i64 {
    input_code(ctx.input_digital("data"))
}

fn jkff_next_on_rising_clock(ctx: &CmContext, old_output: i64) -> i64 {
    let j = input_code(ctx.input_digital("j"));
    let k = input_code(ctx.input_digital("k"));
    match j {
        0 => match k {
            0 => old_output,
            1 => 0,
            _ => -1,
        },
        1 => match k {
            0 => 1,
            1 => invert_code(old_output),
            _ => -1,
        },
        _ => -1,
    }
}

fn tff_next_on_rising_clock(ctx: &CmContext, old_output: i64) -> i64 {
    match input_code(ctx.input_digital("t")) {
        0 => old_output,
        1 => invert_code(old_output),
        _ => -1,
    }
}

fn srff_next_on_rising_clock(ctx: &CmContext, old_output: i64) -> i64 {
    sr_result(
        input_code(ctx.input_digital("s")),
        input_code(ctx.input_digital("r")),
        old_output,
    )
}

/// D Flip-Flop
#[derive(Debug, Default)]
pub struct DFlipFlop;

impl CodeModel for DFlipFlop {
    fn name(&self) -> &str {
        "d_dff"
    }
    fn description(&self) -> &str {
        "Digital D-type flip-flop"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("data", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| edge_parameters("data_load"))
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(EDGE_STATE_COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        evaluate_edge_model(ctx, dff_next_on_rising_clock)
    }
}

/// JK Flip-Flop
#[derive(Debug, Default)]
pub struct JkFlipFlop;

impl CodeModel for JkFlipFlop {
    fn name(&self) -> &str {
        "d_jkff"
    }
    fn description(&self) -> &str {
        "Digital JK-type flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("j", PortType::Digital),
                PortSpec::input("k", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| edge_parameters("jk_load"))
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(EDGE_STATE_COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        evaluate_edge_model(ctx, jkff_next_on_rising_clock)
    }
}

/// T Flip-Flop
#[derive(Debug, Default)]
pub struct TFlipFlop;

impl CodeModel for TFlipFlop {
    fn name(&self) -> &str {
        "d_tff"
    }
    fn description(&self) -> &str {
        "Digital toggle flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("t", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| edge_parameters("t_load"))
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(EDGE_STATE_COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        evaluate_edge_model(ctx, tff_next_on_rising_clock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::PendingDigitalEvent;
    use crate::xspice::{ParamType, PortDirection};

    fn has_event(events: &[PendingDigitalEvent], value: DigitalValue, delay: Value) -> bool {
        events.iter().any(|event| {
            event.port_name == "out"
                && event.values == vec![value]
                && (event.delay - delay).abs() <= 1.0e-21
        })
    }

    fn param_summary(model: &dyn CodeModel) -> Vec<(&str, ParamType, Value)> {
        model
            .parameters()
            .iter()
            .map(|param| (param.name.as_str(), param.param_type, param.default))
            .collect()
    }

    fn assert_scalar_digital_ports(
        model: &dyn CodeModel,
        names: &[&str],
        directions: &[PortDirection],
        nullable: &[bool],
    ) {
        let ports = model.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            names
        );
        for ((port, direction), null_allowed) in ports.iter().zip(directions).zip(nullable) {
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
            assert!(
                !port.is_vector,
                "{} should be a scalar digital port",
                port.name
            );
            assert_eq!(
                port.null_allowed, *null_allowed,
                "{} nullability",
                port.name
            );
        }
    }

    fn assert_edge_model_metadata(model: &dyn CodeModel, names: &[&str], input_load: &str) {
        let mut directions = vec![PortDirection::In; names.len()];
        directions[names.len() - 2] = PortDirection::Out;
        directions[names.len() - 1] = PortDirection::Out;
        let mut nullable = vec![false; names.len()];
        nullable[names.len() - 4] = true;
        nullable[names.len() - 3] = true;
        nullable[names.len() - 2] = true;
        nullable[names.len() - 1] = true;
        assert_scalar_digital_ports(model, names, &directions, &nullable);

        assert_eq!(
            param_summary(model),
            vec![
                ("clk_delay", ParamType::Real, 1.0e-9),
                ("set_delay", ParamType::Real, 1.0e-9),
                ("reset_delay", ParamType::Real, 1.0e-9),
                ("ic", ParamType::Integer, 0.0),
                ("rise_delay", ParamType::Real, 1.0e-9),
                ("fall_delay", ParamType::Real, 1.0e-9),
                (input_load, ParamType::Real, 1.0e-12),
                ("clk_load", ParamType::Real, 1.0e-12),
                ("set_load", ParamType::Real, 1.0e-12),
                ("reset_load", ParamType::Real, 1.0e-12),
            ]
        );
    }

    #[test]
    fn edge_triggered_metadata_matches_ngspice46_interfaces() {
        assert_edge_model_metadata(
            &DFlipFlop,
            &["data", "clk", "set", "reset", "out", "Nout"],
            "data_load",
        );
        assert_edge_model_metadata(
            &TFlipFlop,
            &["t", "clk", "set", "reset", "out", "Nout"],
            "t_load",
        );
        assert_edge_model_metadata(
            &JkFlipFlop,
            &["j", "k", "clk", "set", "reset", "out", "Nout"],
            "jk_load",
        );
        assert_edge_model_metadata(
            &SrFlipFlop,
            &["s", "r", "clk", "set", "reset", "out", "Nout"],
            "sr_load",
        );
    }

    #[test]
    fn d_fdiv_metadata_matches_ngspice46_interface() {
        assert_scalar_digital_ports(
            &DigitalFrequencyDivider,
            &["freq_in", "freq_out"],
            &[PortDirection::In, PortDirection::Out],
            &[false, false],
        );
        assert_eq!(
            param_summary(&DigitalFrequencyDivider),
            vec![
                ("div_factor", ParamType::Integer, 2.0),
                ("high_cycles", ParamType::Integer, 1.0),
                ("i_count", ParamType::Integer, 0.0),
                ("rise_delay", ParamType::Real, 1.0e-9),
                ("fall_delay", ParamType::Real, 1.0e-9),
                ("freq_in_load", ParamType::Real, 1.0e-12),
            ]
        );
    }

    #[test]
    fn latch_metadata_matches_ngspice46_interfaces() {
        assert_scalar_digital_ports(
            &DLatch,
            &["data", "enable", "set", "reset", "out", "Nout"],
            &[
                PortDirection::In,
                PortDirection::In,
                PortDirection::In,
                PortDirection::In,
                PortDirection::Out,
                PortDirection::Out,
            ],
            &[false, false, true, true, true, true],
        );
        assert_eq!(
            param_summary(&DLatch),
            vec![
                ("data_delay", ParamType::Real, 1.0e-9),
                ("enable_delay", ParamType::Real, 1.0e-9),
                ("set_delay", ParamType::Real, 1.0e-9),
                ("reset_delay", ParamType::Real, 1.0e-9),
                ("ic", ParamType::Integer, 0.0),
                ("rise_delay", ParamType::Real, 1.0e-9),
                ("fall_delay", ParamType::Real, 1.0e-9),
                ("data_load", ParamType::Real, 1.0e-12),
                ("enable_load", ParamType::Real, 1.0e-12),
                ("set_load", ParamType::Real, 1.0e-12),
                ("reset_load", ParamType::Real, 1.0e-12),
            ]
        );

        assert_scalar_digital_ports(
            &SrLatch,
            &["s", "r", "enable", "set", "reset", "out", "Nout"],
            &[
                PortDirection::In,
                PortDirection::In,
                PortDirection::In,
                PortDirection::In,
                PortDirection::In,
                PortDirection::Out,
                PortDirection::Out,
            ],
            &[false, false, false, true, true, true, true],
        );
        assert_eq!(
            param_summary(&SrLatch),
            vec![
                ("sr_delay", ParamType::Real, 1.0e-9),
                ("enable_delay", ParamType::Real, 1.0e-9),
                ("set_delay", ParamType::Real, 1.0e-9),
                ("reset_delay", ParamType::Real, 1.0e-9),
                ("ic", ParamType::Integer, 0.0),
                ("rise_delay", ParamType::Real, 1.0e-9),
                ("fall_delay", ParamType::Real, 1.0e-9),
                ("sr_load", ParamType::Real, 1.0e-12),
                ("enable_load", ParamType::Real, 1.0e-12),
                ("set_load", ParamType::Real, 1.0e-12),
                ("reset_load", ParamType::Real, 1.0e-12),
            ]
        );
    }

    #[test]
    fn d_jkff_async_set_and_reset_update_output_state() {
        let model = JkFlipFlop;
        let mut ctx = CmContext::new();
        model.init(&mut ctx).expect("jkff init");

        ctx.set_input_digital("j", DigitalValue::zero());
        ctx.set_input_digital("k", DigitalValue::zero());
        ctx.set_input_digital("clk", DigitalValue::zero());
        ctx.set_input_digital("set", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.evaluate(&mut ctx).expect("set evaluates");
        assert_eq!(
            ctx.int_state(EDGE_PREV_OUT),
            1,
            "async set should drive the JK output state high"
        );

        ctx.set_input_digital("set", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::one());
        model.evaluate(&mut ctx).expect("reset evaluates");
        assert_eq!(
            ctx.int_state(EDGE_PREV_OUT),
            0,
            "async reset should drive the JK output state low"
        );
    }

    #[test]
    fn d_fdiv_high_initial_input_counts_first_transient_edge_like_ngspice() {
        let model = DigitalFrequencyDivider;
        let mut ctx = CmContext::new();
        ctx.set_param("div_factor", 2.0);
        ctx.set_param("high_cycles", 1.0);
        ctx.set_param("i_count", 0.0);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        model.init(&mut ctx).expect("fdiv init");

        ctx.set_input_digital("freq_in", DigitalValue::one());
        model.evaluate(&mut ctx).expect("initial fdiv evaluation");
        assert_eq!(
            ctx.output_digital_vector("freq_out"),
            vec![DigitalValue::zero()]
        );

        ctx.time = 1.0e-12;
        model
            .evaluate(&mut ctx)
            .expect("first transient fdiv evaluation");

        assert_eq!(
            ctx.output_digital_vector("freq_out"),
            vec![DigitalValue::one()],
            "ngspice leaves freq_in_old at ZERO during TIME=0, so a high initial input is counted on the first transient evaluation"
        );
    }

    #[test]
    fn d_dff_rollbackable_probe_does_not_commit_edge_state() {
        let model = DFlipFlop;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_port_width("Nout", 0);
        ctx.set_param("ic", 0.0);
        ctx.set_param("clk_delay", 1.0e-9);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_input_digital("data", DigitalValue::zero());
        ctx.set_input_digital("clk", DigitalValue::zero());
        ctx.set_input_digital("set", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.init(&mut ctx).expect("dff init");

        model.evaluate(&mut ctx).expect("dff initial");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(EDGE_PREV_OUT), 0);
        assert_eq!(ctx.int_state(EDGE_PREV_CLK), 0);

        ctx.time = 1.0e-9;
        ctx.set_input_digital("data", DigitalValue::one());
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        model.evaluate(&mut ctx).expect("dff rollback probe");
        let events = ctx.take_pending_events();
        assert!(
            has_event(&events, DigitalValue::one(), 2.0e-9),
            "rollbackable DFF probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(ctx.int_state(EDGE_PREV_OUT), 0);
        assert_eq!(ctx.int_state(EDGE_PREV_CLK), 0);

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        model.evaluate(&mut ctx).expect("dff direct after probe");
        assert_eq!(ctx.int_state(EDGE_PREV_OUT), 1);
        assert_eq!(ctx.int_state(EDGE_PREV_CLK), 1);
    }

    #[test]
    fn d_fdiv_rollbackable_probe_does_not_commit_count_or_edges() {
        let model = DigitalFrequencyDivider;
        let mut ctx = CmContext::new();
        ctx.set_param("div_factor", 2.0);
        ctx.set_param("high_cycles", 1.0);
        ctx.set_param("i_count", 0.0);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_input_digital("freq_in", DigitalValue::zero());
        model.init(&mut ctx).expect("fdiv init");

        model.evaluate(&mut ctx).expect("fdiv initial");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(FDIV_COUNT), 0);
        assert_eq!(ctx.int_state(FDIV_PREV_INPUT), 0);
        assert_eq!(ctx.int_state(FDIV_OUTPUT), 0);

        ctx.time = 1.0e-9;
        ctx.set_input_digital("freq_in", DigitalValue::one());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        model.evaluate(&mut ctx).expect("fdiv rollback probe");
        let events = ctx.take_pending_events();
        assert!(
            events.iter().any(|event| {
                event.port_name == "freq_out"
                    && event.values == vec![DigitalValue::one()]
                    && (event.delay - 1.0e-9).abs() <= 1.0e-21
            }),
            "rollbackable fdiv probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(ctx.int_state(FDIV_COUNT), 0);
        assert_eq!(ctx.int_state(FDIV_PREV_INPUT), 0);
        assert_eq!(ctx.int_state(FDIV_OUTPUT), 0);

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        model.evaluate(&mut ctx).expect("fdiv direct after probe");
        assert_eq!(ctx.int_state(FDIV_COUNT), 1);
        assert_eq!(ctx.int_state(FDIV_PREV_INPUT), 1);
        assert_eq!(ctx.int_state(FDIV_OUTPUT), 1);
    }

    #[test]
    fn d_dlatch_rollbackable_probe_does_not_commit_latch_state() {
        let model = DLatch;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_port_width("Nout", 0);
        ctx.set_param("ic", 0.0);
        ctx.set_param("enable_delay", 1.0e-9);
        ctx.set_param("data_delay", 1.0e-9);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_input_digital("data", DigitalValue::zero());
        ctx.set_input_digital("enable", DigitalValue::zero());
        ctx.set_input_digital("set", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.init(&mut ctx).expect("dlatch init");

        model.evaluate(&mut ctx).expect("dlatch initial");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(DLATCH_PREV_OUT), 0);
        assert_eq!(ctx.int_state(DLATCH_PREV_ENABLE), 0);

        ctx.time = 1.0e-9;
        ctx.set_input_digital("data", DigitalValue::one());
        ctx.set_input_digital("enable", DigitalValue::one());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        model.evaluate(&mut ctx).expect("dlatch rollback probe");
        let events = ctx.take_pending_events();
        assert!(
            has_event(&events, DigitalValue::one(), 2.0e-9),
            "rollbackable latch probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(ctx.int_state(DLATCH_PREV_OUT), 0);
        assert_eq!(ctx.int_state(DLATCH_PREV_ENABLE), 0);

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        model.evaluate(&mut ctx).expect("dlatch direct after probe");
        assert_eq!(ctx.int_state(DLATCH_PREV_OUT), 1);
        assert_eq!(ctx.int_state(DLATCH_PREV_ENABLE), 1);
    }
}

/// Frequency divider
#[derive(Debug, Default)]
pub struct DigitalFrequencyDivider;

const FDIV_PREV_INPUT: usize = 0;
const FDIV_COUNT: usize = 1;
const FDIV_OUTPUT: usize = 2;
const FDIV_INITIAL_OUTPUT: i64 = i64::MIN;

impl CodeModel for DigitalFrequencyDivider {
    fn name(&self) -> &str {
        "d_fdiv"
    }

    fn description(&self) -> &str {
        "Digital frequency divider"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("freq_in", PortType::Digital),
                PortSpec::output("freq_out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::integer("div_factor", 2)
                    .with_description("Division factor, clamped to official lower limit"),
                ParamSpec::integer("high_cycles", 1)
                    .with_description("High output cycles, clamped to official lower limit"),
                ParamSpec::integer("i_count", 0)
                    .with_description("Initial counter value, clamped to official lower limit"),
                ParamSpec::real("rise_delay", 1.0e-9)
                    .with_description("Rise propagation delay, clamped to official lower limit"),
                ParamSpec::real("fall_delay", 1.0e-9)
                    .with_description("Fall propagation delay, clamped to official lower limit"),
                ParamSpec::real("freq_in_load", 1.0e-12),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(3);
        ctx.set_int_state(FDIV_PREV_INPUT, 0);
        ctx.set_int_state(FDIV_COUNT, 0);
        ctx.set_int_state(FDIV_OUTPUT, FDIV_INITIAL_OUTPUT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let div_factor = official_min_integer_param(ctx, "div_factor", 1);
        let high_cycles = official_min_integer_param(ctx, "high_cycles", 1);
        let input = input_code(ctx.input_digital("freq_in"));
        let mut count = ctx.int_state(FDIV_COUNT);
        let mut output = ctx.int_state(FDIV_OUTPUT);

        if ctx.time == 0.0 || output == FDIV_INITIAL_OUTPUT {
            count = official_min_integer_param(ctx, "i_count", 0);
            output = if count >= div_factor || count < 0 {
                count = 0;
                0
            } else if count > 0 && count <= high_cycles {
                1
            } else {
                0
            };
            ctx.set_output_digital("freq_out", output_value(output), 0.0);
            sequential_set_int_state(ctx, FDIV_COUNT, count);
            sequential_set_int_state(ctx, FDIV_OUTPUT, output);
            if ctx.time == 0.0 {
                return Ok(());
            }
        }

        let previous_input = ctx.int_state(FDIV_PREV_INPUT);
        let mut next_output = output;
        let mut delay = None;

        if input != previous_input && input == 1 {
            count += 1;
            if count == div_factor + 1 || count == 1 {
                count = 1;
                next_output = 1;
                delay = Some(official_sequential_delay(ctx, "rise_delay"));
            } else if count == high_cycles + 1 {
                next_output = 0;
                delay = Some(official_sequential_delay(ctx, "fall_delay"));
            }
        }

        if next_output != output
            && let Some(delay) = delay
        {
            ctx.set_output_digital("freq_out", output_value(next_output), delay);
            output = next_output;
        }

        sequential_set_int_state(ctx, FDIV_PREV_INPUT, input);
        sequential_set_int_state(ctx, FDIV_COUNT, count);
        sequential_set_int_state(ctx, FDIV_OUTPUT, output);
        Ok(())
    }
}

/// SR Flip-Flop
#[derive(Debug, Default)]
pub struct SrFlipFlop;

impl CodeModel for SrFlipFlop {
    fn name(&self) -> &str {
        "d_srff"
    }
    fn description(&self) -> &str {
        "Digital set-reset flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("s", PortType::Digital),
                PortSpec::input("r", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| edge_parameters("sr_load"))
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(EDGE_STATE_COUNT);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        evaluate_edge_model(ctx, srff_next_on_rising_clock)
    }
}

/// D Latch
#[derive(Debug, Default)]
pub struct DLatch;

const DLATCH_PREV_DATA: usize = 0;
const DLATCH_PREV_ENABLE: usize = 1;
const DLATCH_PREV_SET: usize = 2;
const DLATCH_PREV_RESET: usize = 3;
const DLATCH_PREV_OUT: usize = 4;
const DLATCH_STATE_COUNT: usize = 5;

fn dlatch_initial_output(ic: i64, data: i64, enable: i64, set: i64, reset: i64) -> i64 {
    let mut output = ic;
    if enable == 1 {
        output = data;
    }
    if set == 1 && reset == 0 {
        output = 1;
    }
    if set == 0 && reset == 1 {
        output = 0;
    }
    if set == 1 && reset == 1 {
        output = -1;
    }
    output
}

fn dlatch_set_edge_output(old: i64, data: i64, enable: i64, set: i64, reset: i64) -> i64 {
    match set {
        1 if reset != 1 => 1,
        1 => -1,
        _ if reset == 1 => 0,
        _ if enable == 1 => data,
        _ => old,
    }
}

fn dlatch_reset_edge_output(old: i64, data: i64, enable: i64, set: i64, reset: i64) -> i64 {
    match reset {
        1 if set != 1 => 0,
        1 => -1,
        _ if set == 1 => 1,
        _ if enable == 1 => data,
        _ => old,
    }
}

impl CodeModel for DLatch {
    fn name(&self) -> &str {
        "d_dlatch"
    }
    fn description(&self) -> &str {
        "Digital D-type latch"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("data", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("data_delay", 1e-9)
                    .with_description("Data propagation delay, clamped to official lower limit"),
                ParamSpec::real("enable_delay", 1e-9)
                    .with_description("Enable propagation delay, clamped to official lower limit"),
                ParamSpec::real("set_delay", 1e-9)
                    .with_description("Set propagation delay, clamped to official lower limit"),
                ParamSpec::real("reset_delay", 1e-9)
                    .with_description("Reset propagation delay, clamped to official lower limit"),
                ParamSpec::integer("ic", 0)
                    .with_description("Initial output state, clamped to official range"),
                ParamSpec::real("rise_delay", 1e-9)
                    .with_description("Rise propagation delay, clamped to official lower limit"),
                ParamSpec::real("fall_delay", 1e-9)
                    .with_description("Fall propagation delay, clamped to official lower limit"),
                ParamSpec::real("data_load", 1e-12),
                ParamSpec::real("enable_load", 1e-12),
                ParamSpec::real("set_load", 1e-12),
                ParamSpec::real("reset_load", 1e-12),
            ]
        })
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(DLATCH_STATE_COUNT);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let data = input_code(ctx.input_digital("data"));
        let enable = input_code(ctx.input_digital("enable"));
        let set = input_code(ctx.input_digital("set"));
        let reset = input_code(ctx.input_digital("reset"));

        let old_data = ctx.int_state(DLATCH_PREV_DATA);
        let old_enable = ctx.int_state(DLATCH_PREV_ENABLE);
        let old_set = ctx.int_state(DLATCH_PREV_SET);
        let old_reset = ctx.int_state(DLATCH_PREV_RESET);
        let old_output = ctx.int_state(DLATCH_PREV_OUT);

        let output = if ctx.time == 0.0 {
            let initial = dlatch_initial_output(ic_code(ctx), data, enable, set, reset);
            drive_outputs(ctx, initial, 0.0, 0.0);
            initial
        } else {
            let (next, base_delay) = if set != old_set {
                (
                    dlatch_set_edge_output(old_output, data, enable, set, reset),
                    Some(official_sequential_delay(ctx, "set_delay")),
                )
            } else if reset != old_reset {
                (
                    dlatch_reset_edge_output(old_output, data, enable, set, reset),
                    Some(official_sequential_delay(ctx, "reset_delay")),
                )
            } else if enable != old_enable && reset != 1 && set != 1 {
                if enable == 1 {
                    (data, Some(official_sequential_delay(ctx, "enable_delay")))
                } else {
                    (old_output, None)
                }
            } else if data != old_data && reset != 1 && set != 1 {
                if enable == 1 {
                    (data, Some(official_sequential_delay(ctx, "data_delay")))
                } else {
                    (old_output, None)
                }
            } else {
                (old_output, None)
            };

            if next != old_output
                && let Some(base_delay) = base_delay
            {
                let (out_delay, nout_delay) = transition_delays(ctx, old_output, next, base_delay);
                drive_outputs(ctx, next, out_delay, nout_delay);
            }

            next
        };

        sequential_set_int_state(ctx, DLATCH_PREV_DATA, data);
        sequential_set_int_state(ctx, DLATCH_PREV_ENABLE, enable);
        sequential_set_int_state(ctx, DLATCH_PREV_SET, set);
        sequential_set_int_state(ctx, DLATCH_PREV_RESET, reset);
        sequential_set_int_state(ctx, DLATCH_PREV_OUT, output);
        Ok(())
    }
}

/// SR Latch
#[derive(Debug, Default)]
pub struct SrLatch;

const SRLATCH_PREV_S: usize = 0;
const SRLATCH_PREV_R: usize = 1;
const SRLATCH_PREV_ENABLE: usize = 2;
const SRLATCH_PREV_SET: usize = 3;
const SRLATCH_PREV_RESET: usize = 4;
const SRLATCH_PREV_OUT: usize = 5;
const SRLATCH_STATE_COUNT: usize = 6;

fn srlatch_initial_output(ic: i64, s: i64, r: i64, enable: i64, set: i64, reset: i64) -> i64 {
    let mut output = ic;
    if enable == 1 && s == 1 && r == 0 {
        output = 1;
    }
    if enable == 1 && s == 0 && r == 1 {
        output = 0;
    }
    if set == 1 && reset == 0 {
        output = 1;
    }
    if set == 0 && reset == 1 {
        output = 0;
    }
    if set == 1 && reset == 1 {
        output = -1;
    }
    output
}

fn srlatch_set_edge_output(old: i64, s: i64, r: i64, enable: i64, set: i64, reset: i64) -> i64 {
    match set {
        1 if reset != 1 => 1,
        1 => -1,
        _ if reset == 1 => 0,
        _ if enable == 1 => sr_result(s, r, old),
        _ => old,
    }
}

fn srlatch_reset_edge_output(old: i64, s: i64, r: i64, enable: i64, set: i64, reset: i64) -> i64 {
    match reset {
        1 if set != 1 => 0,
        1 => -1,
        _ if set == 1 => 1,
        _ if enable == 1 => sr_result(s, r, old),
        _ => old,
    }
}

impl CodeModel for SrLatch {
    fn name(&self) -> &str {
        "d_srlatch"
    }
    fn description(&self) -> &str {
        "Digital SR-type latch"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("s", PortType::Digital),
                PortSpec::input("r", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("out", PortType::Digital).nullable(),
                PortSpec::output("Nout", PortType::Digital).nullable(),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("sr_delay", 1e-9)
                    .with_description("S/R propagation delay, clamped to official lower limit"),
                ParamSpec::real("enable_delay", 1e-9)
                    .with_description("Enable propagation delay, clamped to official lower limit"),
                ParamSpec::real("set_delay", 1e-9)
                    .with_description("Set propagation delay, clamped to official lower limit"),
                ParamSpec::real("reset_delay", 1e-9)
                    .with_description("Reset propagation delay, clamped to official lower limit"),
                ParamSpec::integer("ic", 0)
                    .with_description("Initial output state, clamped to official range"),
                ParamSpec::real("rise_delay", 1e-9)
                    .with_description("Rise propagation delay, clamped to official lower limit"),
                ParamSpec::real("fall_delay", 1e-9)
                    .with_description("Fall propagation delay, clamped to official lower limit"),
                ParamSpec::real("sr_load", 1e-12),
                ParamSpec::real("enable_load", 1e-12),
                ParamSpec::real("set_load", 1e-12),
                ParamSpec::real("reset_load", 1e-12),
            ]
        })
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(SRLATCH_STATE_COUNT);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let s = input_code(ctx.input_digital("s"));
        let r = input_code(ctx.input_digital("r"));
        let enable = input_code(ctx.input_digital("enable"));
        let set = input_code(ctx.input_digital("set"));
        let reset = input_code(ctx.input_digital("reset"));

        let old_s = ctx.int_state(SRLATCH_PREV_S);
        let old_r = ctx.int_state(SRLATCH_PREV_R);
        let old_enable = ctx.int_state(SRLATCH_PREV_ENABLE);
        let old_set = ctx.int_state(SRLATCH_PREV_SET);
        let old_reset = ctx.int_state(SRLATCH_PREV_RESET);
        let old_output = ctx.int_state(SRLATCH_PREV_OUT);

        let output = if ctx.time == 0.0 {
            let initial = srlatch_initial_output(ic_code(ctx), s, r, enable, set, reset);
            drive_outputs(ctx, initial, 0.0, 0.0);
            initial
        } else {
            let (next, base_delay) = if set != old_set {
                (
                    srlatch_set_edge_output(old_output, s, r, enable, set, reset),
                    Some(official_sequential_delay(ctx, "set_delay")),
                )
            } else if reset != old_reset {
                (
                    srlatch_reset_edge_output(old_output, s, r, enable, set, reset),
                    Some(official_sequential_delay(ctx, "reset_delay")),
                )
            } else if enable != old_enable && reset != 1 && set != 1 {
                if enable == 1 {
                    (
                        sr_result(s, r, old_output),
                        Some(official_sequential_delay(ctx, "enable_delay")),
                    )
                } else {
                    (old_output, None)
                }
            } else if (s != old_s || r != old_r) && reset != 1 && set != 1 {
                if enable == 1 {
                    (
                        sr_result(s, r, old_output),
                        Some(official_sequential_delay(ctx, "sr_delay")),
                    )
                } else {
                    (old_output, None)
                }
            } else {
                (old_output, None)
            };

            if next != old_output
                && let Some(base_delay) = base_delay
            {
                let (out_delay, nout_delay) = transition_delays(ctx, old_output, next, base_delay);
                drive_outputs(ctx, next, out_delay, nout_delay);
            }

            next
        };

        sequential_set_int_state(ctx, SRLATCH_PREV_S, s);
        sequential_set_int_state(ctx, SRLATCH_PREV_R, r);
        sequential_set_int_state(ctx, SRLATCH_PREV_ENABLE, enable);
        sequential_set_int_state(ctx, SRLATCH_PREV_SET, set);
        sequential_set_int_state(ctx, SRLATCH_PREV_RESET, reset);
        sequential_set_int_state(ctx, SRLATCH_PREV_OUT, output);
        Ok(())
    }
}
