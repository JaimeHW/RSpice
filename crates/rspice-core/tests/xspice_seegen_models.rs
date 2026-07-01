//! Official XSPICE `seegen` single-event-effect current generator.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{
    AnalysisType, CodeModelRegistry, EvaluationPhase, PortConnection, XspiceInstance,
};

fn run_tran(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves")
}

fn transient_node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[idx]
}

fn value_at_time(times: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(times.len(), values.len(), "waveform lengths must match");
    let first_time = *times.first().expect("waveform has samples");
    let first_value = *values.first().expect("waveform has samples");
    if target <= first_time {
        return first_value;
    }

    for (time_pair, value_pair) in times.windows(2).zip(values.windows(2)) {
        let (t0, t1) = (time_pair[0], time_pair[1]);
        if target <= t1 {
            let (v0, v1) = (value_pair[0], value_pair[1]);
            let span = t1 - t0;
            if span.abs() <= f64::EPSILON {
                return v1;
            }
            let alpha = (target - t0) / span;
            return v0 + alpha * (v1 - v0);
        }
    }

    *values.last().expect("waveform has samples")
}

fn has_time_sample(times: &[f64], target: f64) -> bool {
    times.iter().any(|time| (*time - target).abs() <= 1.0e-18)
}

#[test]
fn seegen_rejects_output_vector_below_official_minimum_length_at_construction() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("seegen").expect("seegen is registered");

    let err = XspiceInstance::new(
        "asee_empty_out",
        model,
        vec![
            PortConnection::Null,
            PortConnection::Null,
            PortConnection::AnalogVector(Vec::new()),
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .expect_err("official seegen output vector lower bound must be enforced");

    assert!(
        err.to_string().contains("out") && err.to_string().contains("at least 1"),
        "seegen empty output vector should be rejected like ngspice, got {err}"
    );
}

#[test]
fn seegen_autonomous_pulse_drives_current_output_and_monitor() {
    let deck = "\
* XSPICE seegen autonomous single pulse
asee null mon %id[out 0] see
.model see seegen (tdelay=1n trise=0.5n tfall=2n inull=1m tperiod=10n)
rout out 0 1k
rmon mon 0 1meg
.end
";

    let result = run_tran(deck, 4.0e-9, 0.01e-9);
    let out = transient_node_series(&result, "out");
    let mon = transient_node_series(&result, "mon");

    let before = value_at_time(&result.time, out, 0.5e-9);
    let peak_time = 1.0e-9 + 2.0e-9 * 0.5e-9 * (0.5_f64 / 2.0).ln() / (0.5e-9 - 2.0e-9);
    let peak_current =
        1.0e-3 * ((-(peak_time - 1.0e-9) / 2.0e-9).exp() - (-(peak_time - 1.0e-9) / 0.5e-9).exp());
    let peak_out = value_at_time(&result.time, out, peak_time);
    let peak_mon = value_at_time(&result.time, mon, peak_time);

    assert!(
        before.abs() < 1.0e-6,
        "seegen output must be idle before tdelay, got {before}"
    );
    assert!(
        (peak_out + peak_current * 1.0e3).abs() < 3.0e-3,
        "positive seegen current should drive the %id node negative through 1 kohm: expected {}, got {peak_out}",
        -peak_current * 1.0e3
    );
    assert!(
        (peak_mon - peak_current).abs() < 3.0e-6,
        "monitor voltage should mirror generated current value, expected {peak_current}, got {peak_mon}"
    );
}

#[test]
fn seegenerator_alias_drives_current_output_like_seegen() {
    let deck = "\
* XSPICE official seegenerator alias
asee null mon %id[out 0] see
.model see seegenerator (tdelay=1n trise=0.5n tfall=2n inull=1m tperiod=10n)
rout out 0 1k
rmon mon 0 1meg
.end
";

    let result = run_tran(deck, 4.0e-9, 0.01e-9);
    let out = transient_node_series(&result, "out");
    let mon = transient_node_series(&result, "mon");
    let peak_time = 1.0e-9 + 2.0e-9 * 0.5e-9 * (0.5_f64 / 2.0).ln() / (0.5e-9 - 2.0e-9);
    let peak_current =
        1.0e-3 * ((-(peak_time - 1.0e-9) / 2.0e-9).exp() - (-(peak_time - 1.0e-9) / 0.5e-9).exp());
    let peak_out = value_at_time(&result.time, out, peak_time);
    let peak_mon = value_at_time(&result.time, mon, peak_time);

    assert!(
        (peak_out + peak_current * 1.0e3).abs() < 3.0e-3,
        "seegenerator alias should drive the same current output as seegen, got {peak_out}"
    );
    assert!(
        (peak_mon - peak_current).abs() < 3.0e-6,
        "seegenerator alias monitor should mirror generated current, got {peak_mon}"
    );
}

#[test]
fn seegen_accepts_unbounded_negative_time_constants_like_ngspice() {
    let deck = "\
* XSPICE seegen negative time constants oracle
asee null mon %id[out 0] see
.model see seegen (tdelay=1n trise=-0.5n tfall=-2n inull=1m tperiod=10n)
rout out 0 1k
rmon mon 0 1meg
.end
";

    let result = run_tran(deck, 4.0e-9, 0.1e-9);
    let out = transient_node_series(&result, "out");
    let mon = transient_node_series(&result, "mon");

    let before = value_at_time(&result.time, out, 0.5e-9);
    let target: f64 = 1.2e-9;
    let elapsed = target - 1.0e-9;
    let current = 1.0e-3 * ((-(elapsed / -2.0e-9_f64)).exp() - (-(elapsed / -0.5e-9_f64)).exp());
    let out_at_target = value_at_time(&result.time, out, target);
    let mon_at_target = value_at_time(&result.time, mon, target);

    assert!(
        before.abs() < 1.0e-6,
        "seegen output must stay idle before tdelay even with negative time constants, got {before}"
    );
    assert!(
        (out_at_target + current * 1.0e3).abs() < 1.0e-3,
        "negative seegen time constants should use ngspice raw exponential current; expected {}, got {out_at_target}",
        -current * 1.0e3
    );
    assert!(
        (mon_at_target - current).abs() < 1.0e-6,
        "seegen monitor should mirror ngspice raw current {current}, got {mon_at_target}"
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn seegen_clamps_particle_angle_to_official_limits_like_ngspice() {
    let deck = "\
* XSPICE seegen angle clamp oracle
aseelow null monlow %id[outlow 0] seelow
aseehigh null monhigh %id[outhigh 0] seehigh
.model seelow seegen (tdelay=1n trise=0.5n tfall=2n inull=0 let=1e-6 cdepth=1 angle=-1 tperiod=10n)
.model seehigh seegen (tdelay=1n trise=0.5n tfall=2n inull=0 let=1e-6 cdepth=1 angle=2 tperiod=10n)
routlow outlow 0 1k
routhigh outhigh 0 1
rmonlow monlow 0 1meg
rmonhigh monhigh 0 1meg
.end
";

    let result = run_tran(deck, 4.0e-9, 0.1e-9);
    let outlow = transient_node_series(&result, "outlow");
    let outhigh = transient_node_series(&result, "outhigh");

    let tdelay: f64 = 1.0e-9;
    let trise: f64 = 0.5e-9;
    let tfall: f64 = 2.0e-9;
    let peak_time = tdelay + tfall * trise * (trise / tfall).ln() / (trise - tfall);
    let pulse_shape = (-(peak_time - tdelay) / tfall).exp() - (-(peak_time - tdelay) / trise).exp();
    let unclamped_inull = 1.035e-14 * 1.0e-6 / (tfall - trise);
    let high_angle_inull = unclamped_inull / 1.57079_f64.cos();

    let low_peak = value_at_time(&result.time, outlow, peak_time);
    assert!(
        (low_peak + unclamped_inull * pulse_shape * 1.0e3).abs() < 1.0e-11,
        "seegen angle below zero should clamp to zero like ngspice, got {low_peak}"
    );

    let high_peak = value_at_time(&result.time, outhigh, peak_time);
    assert!(
        (high_peak + high_angle_inull * pulse_shape).abs() < 1.0e-9,
        "seegen angle above official max should clamp to 1.57079 like ngspice, got {high_peak}"
    );
}

#[test]
fn seegen_schedules_pulse_start_and_peak_breakpoints() {
    let deck = "\
* XSPICE seegen breakpoints
asee null null %id[out 0] see
.model see seegen (tdelay=1n trise=0.5n tfall=2n inull=1m tperiod=10n)
rout out 0 1k
.end
";

    let result = run_tran(deck, 4.0e-9, 4.0e-9);
    let peak_time = 1.0e-9 + 2.0e-9 * 0.5e-9 * (0.5_f64 / 2.0).ln() / (0.5e-9 - 2.0e-9);

    assert!(
        has_time_sample(&result.time, 1.0e-9) && has_time_sample(&result.time, peak_time),
        "seegen should schedule start and peak breakpoints; peak={peak_time:e}, samples={:?}",
        result.time
    );
}

#[test]
fn seegen_connected_control_starts_on_rising_threshold_crossing() {
    let deck = "\
* XSPICE seegen control-triggered pulse
vctrl ctrl 0 pwl(0 0 1n 0 1.05n 1 5n 1)
asee ctrl null %id[out 0] see
.model see seegen (ctrlthres=0.5 tdelay=0.2n trise=0.5n tfall=2n inull=1m tperiod=10n)
rout out 0 1k
.end
";

    let result = run_tran(deck, 4.0e-9, 0.025e-9);
    let out = transient_node_series(&result, "out");

    let before_trigger_delay = value_at_time(&result.time, out, 1.1e-9);
    let after_trigger_delay = value_at_time(&result.time, out, 1.6e-9);

    assert!(
        before_trigger_delay.abs() < 1.0e-5,
        "connected ctrl should suppress autonomous start until a rising crossing, got {before_trigger_delay}"
    );
    assert!(
        after_trigger_delay < -0.02,
        "rising ctrl crossing should start the seegen pulse after tdelay, got {after_trigger_delay}"
    );
}

#[test]
fn seegen_periodic_vector_outputs_advance_channels() {
    let deck = "\
* XSPICE seegen vector output sequence
asee null null %id[out0 0] %id[out1 0] see
.model see seegen (tdelay=0.5n trise=0.2n tfall=1n inull=1m tperiod=2n perlim=true)
r0 out0 0 1k
r1 out1 0 1k
.end
";

    let result = run_tran(deck, 4.0e-9, 0.05e-9);
    let out0 = transient_node_series(&result, "out0");
    let out1 = transient_node_series(&result, "out1");

    assert!(
        has_time_sample(&result.time, 2.3e-9),
        "seegen should schedule the vector channel-advance discontinuity; samples={:?}",
        result.time
    );

    let first_active = value_at_time(&result.time, out0, 1.0e-9);
    let second_idle_during_first = value_at_time(&result.time, out1, 1.0e-9);
    let first_idle_during_second = value_at_time(&result.time, out0, 3.0e-9);
    let second_active = value_at_time(&result.time, out1, 3.0e-9);

    assert!(
        first_active < -0.02 && second_idle_during_first.abs() < 1.0e-5,
        "first pulse should drive only the first vector output, got out0={first_active}, out1={second_idle_during_first}"
    );
    assert!(
        first_idle_during_second.abs() < 1.0e-5 && second_active < -0.02,
        "second pulse should advance to the second vector output, got out0={first_idle_during_second}, out1={second_active}"
    );
}

#[test]
fn seegen_channel_advance_outputs_previous_pulse_tail_like_ngspice() {
    let deck = "\
* XSPICE seegen channel advance ordering
asee null null %id[out0 0] %id[out1 0] see
.model see seegen (tdelay=0.5n trise=0.2n tfall=1n inull=1m tperiod=2n perlim=true)
r0 out0 0 1k
r1 out1 0 1k
.end
";

    let result = run_tran(deck, 2.35e-9, 0.01e-9);
    let out1 = transient_node_series(&result, "out1");
    let first_post_advance_sample = 2.301e-9;
    let just_after_advance = value_at_time(&result.time, out1, first_post_advance_sample);
    let elapsed = first_post_advance_sample - 0.5e-9;
    let previous_tail_current =
        1.0e-3 * (f64::exp(-(elapsed / 1.0e-9)) - f64::exp(-(elapsed / 0.2e-9)));
    let expected = -previous_tail_current * 1.0e3;

    assert!(
        (just_after_advance - expected).abs() < 5.0e-3,
        "ngspice computes the previous pulse tail before advancing output channel; expected {expected}, got {just_after_advance}"
    );
}

#[test]
fn seegen_perlim_exhaustion_does_not_force_last_output_to_zero_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("seegen").expect("seegen is registered");
    let mut instance = XspiceInstance::new(
        "asee_perlim_exhaustion",
        model,
        vec![
            PortConnection::Null,
            PortConnection::Analog(1),
            PortConnection::TypedAnalogVector(vec![
                rspice_core::xspice::AnalogInputConnection::CurrentOutput { pos: 2, neg: 0 },
                rspice_core::xspice::AnalogInputConnection::CurrentOutput { pos: 3, neg: 0 },
            ]),
        ],
        &[
            ("tdelay".to_string(), 0.5e-9),
            ("trise".to_string(), 0.2e-9),
            ("tfall".to_string(), 1.0e-9),
            ("inull".to_string(), 1.0e-3),
            ("tperiod".to_string(), 2.0e-9),
            ("perlim".to_string(), 1.0),
        ],
        &[],
        &[],
        &[],
    )
    .expect("seegen instance constructs");
    instance.init().expect("seegen init");

    for (time, timestep) in [(0.0, 0.0), (4.299e-9, 4.299e-9), (4.301e-9, 0.002e-9)] {
        instance
            .evaluate(
                time,
                timestep,
                AnalysisType::Transient,
                EvaluationPhase::AcceptedStep,
            )
            .expect("seegen evaluates");
        instance.advance_state();
    }

    let outputs = instance.output_vector("out");
    let out1_after = outputs[1];
    let mon_after = instance.output("mon");

    assert!(
        out1_after > 1.0e-5,
        "ngspice does not assign zero after perlim exhausts the output vector; got out[1]={out1_after}, outputs={outputs:?}"
    );
    assert!(
        mon_after > 1.0e-5,
        "ngspice leaves monitor on the final pulse tail after perlim exhaustion; got mon={mon_after}, outputs={outputs:?}"
    );
}

#[test]
fn seegen_zero_period_default_perlim_suppresses_nonzero_tail_like_ngspice() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("seegen").expect("seegen is registered");
    let mut instance = XspiceInstance::new(
        "asee_zero_period_perlim",
        model,
        vec![
            PortConnection::Null,
            PortConnection::Analog(1),
            PortConnection::TypedAnalogVector(vec![
                rspice_core::xspice::AnalogInputConnection::CurrentOutput { pos: 2, neg: 0 },
            ]),
        ],
        &[
            ("tdelay".to_string(), 0.0),
            ("trise".to_string(), 0.2e-9),
            ("tfall".to_string(), 1.0e-9),
            ("inull".to_string(), 1.0e-3),
            ("tperiod".to_string(), 0.0),
        ],
        &[],
        &[],
        &[],
    )
    .expect("seegen instance constructs");
    instance.init().expect("seegen init");

    for (time, timestep) in [(0.0, 0.0), (1.0e-12, 1.0e-12)] {
        instance
            .evaluate(
                time,
                timestep,
                AnalysisType::Transient,
                EvaluationPhase::AcceptedStep,
            )
            .expect("seegen evaluates");
        instance.advance_state();
    }

    let outputs = instance.output_vector("out");
    let mon = instance.output("mon");
    assert!(
        outputs[0].abs() < 1.0e-15,
        "ngspice advances the default perlim sequence before assigning a tperiod=0 tail; got outputs={outputs:?}"
    );
    assert!(
        mon.abs() < 1.0e-15,
        "monitor should remain at the previous zero output for tperiod=0 default perlim, got {mon}"
    );
}
