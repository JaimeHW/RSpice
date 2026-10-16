//! The connect-rule converters against closed forms, and against the
//! converters they replace at the defaults.

use super::*;
use crate::xspice::context::{AnalogValue, InputValue};
use crate::xspice::{AnalysisType, CallType};

const BOLTZMANN_4KT: Value = 4.0 * 1.380_649e-23 * 300.15;

/// Instance parameters given explicitly.
type Params<'a> = &'a [(&'a str, Value)];

/// A converter context with every parameter at its declared default, then
/// `params` set as if given explicitly.
fn context(model: &dyn CodeModel, width: usize, params: Params) -> CmContext {
    let mut ctx = CmContext::new();
    for port in model.ports() {
        let port_width = if port.is_vector { width } else { 1 };
        ctx.set_port_width(&port.name, port_width);
        if port.direction == PortDirection::Out {
            ctx.init_output_vector(&port.name, port.default_type, port_width);
        }
    }
    for param in model.parameters() {
        ctx.set_param(&param.name, param.default);
    }
    for (name, value) in params {
        ctx.set_param(name, *value);
        ctx.mark_param_provided(name);
    }
    ctx.analysis = AnalysisType::Transient;
    ctx.call_type = CallType::TransientAnalysis;
    ctx.timestep = 1.0e-9;
    model.init(&mut ctx).expect("the converter initializes");
    ctx
}

fn analog(values: &[Value]) -> InputValue {
    InputValue::AnalogVector(values.iter().copied().map(AnalogValue::new).collect())
}

fn set_supply(ctx: &mut CmContext, supply: Option<Value>) {
    if let Some(supply) = supply {
        ctx.set_input("sup", InputValue::Analog(AnalogValue::new(supply)));
    }
}

/// A published event: element, value, and the instant it takes effect.
type Published = (usize, DigitalValue, Value);

/// Evaluate an A/D at `time` as an accepted point and advance its state.
fn adc_step(
    model: &dyn CodeModel,
    ctx: &mut CmContext,
    port: &str,
    time: Value,
    inputs: &[Value],
    supply: Option<Value>,
) -> Vec<Published> {
    ctx.time = time;
    ctx.set_input(port, analog(inputs));
    set_supply(ctx, supply);
    ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
    model.evaluate(ctx).expect("the converter evaluates");
    let events = ctx
        .take_pending_events()
        .into_iter()
        .map(|event| (event.start_index, event.values[0], time + event.delay))
        .collect();
    ctx.advance_state();
    events
}

/// The value each declared threshold surface compares at `time`, less the
/// surface: a sign change between two instants is a crossing between them.
fn surfaces(
    model: &dyn CodeModel,
    ctx: &mut CmContext,
    port: &str,
    time: Value,
    input: &dyn Fn(Value) -> Value,
    supply: Option<&dyn Fn(Value) -> Value>,
) -> Vec<Value> {
    ctx.set_input(port, analog(&[input(time)]));
    set_supply(ctx, supply.map(|supply| supply(time)));
    let thresholds = model
        .analog_input_thresholds(ctx)
        .expect("the converter declares thresholds");
    let mut ordinals = std::collections::BTreeMap::new();
    let mut values = Vec::new();
    for threshold in thresholds {
        let ordinal = ordinals
            .entry((threshold.port.clone(), threshold.element))
            .or_insert(0usize);
        let raw = if threshold.port == "sup" {
            supply.map_or(0.0, |supply| supply(time))
        } else {
            input(time)
        };
        let value =
            model.analog_threshold_input_at(ctx, &threshold.port, threshold.element, *ordinal, raw);
        *ordinal += 1;
        values.push(value - threshold.low);
        values.push(value - threshold.high);
    }
    values
}

/// A one-element A/D driven over `[0, end]` by a stepper that, like the
/// mixed trial, lands on every declared threshold crossing (by bisection to
/// 1e-16 s) and on every requested breakpoint. Returns what it published.
fn run_adc(
    model: &dyn CodeModel,
    ctx: &mut CmContext,
    port: &str,
    input: &dyn Fn(Value) -> Value,
    supply: Option<&dyn Fn(Value) -> Value>,
    end: Value,
    step: Value,
) -> Vec<Published> {
    let mut published = adc_step(model, ctx, port, 0.0, &[input(0.0)], supply.map(|s| s(0.0)));
    let mut breakpoints: Vec<Value> = ctx.take_requested_breakpoints();
    let mut time = 0.0;
    while time < end {
        let mut next = (time + step).min(end);
        if let Some(&breakpoint) = breakpoints
            .iter()
            .filter(|&&b| b > time)
            .min_by(|a, b| a.total_cmp(b))
        {
            next = next.min(breakpoint);
        }
        let before = surfaces(model, ctx, port, time, input, supply);
        let after = surfaces(model, ctx, port, next, input, supply);
        let mut crossed = |t: Value| {
            let at = surfaces(model, ctx, port, t, input, supply);
            before
                .iter()
                .zip(&at)
                .any(|(a, b)| (*a < 0.0) != (*b < 0.0))
        };
        if before
            .iter()
            .zip(&after)
            .any(|(a, b)| (*a < 0.0) != (*b < 0.0))
        {
            let (mut low, mut high) = (time, next);
            while high - low > 1.0e-16 {
                let middle = 0.5 * (low + high);
                if crossed(middle) {
                    high = middle;
                } else {
                    low = middle;
                }
            }
            next = high;
        }
        published.extend(adc_step(
            model,
            ctx,
            port,
            next,
            &[input(next)],
            supply.map(|s| s(next)),
        ));
        breakpoints.extend(ctx.take_requested_breakpoints());
        time = next;
    }
    published
}

fn states(published: &[Published]) -> Vec<(DigitalState, Value)> {
    published
        .iter()
        .map(|(_, value, time)| (value.state, *time))
        .collect()
}

fn assert_times(published: &[Published], expected: &[(DigitalState, Value)], tolerance: Value) {
    let got = states(published);
    assert_eq!(got.len(), expected.len(), "published {got:?}");
    for ((state, time), (want_state, want_time)) in got.iter().zip(expected) {
        assert_eq!(state, want_state, "published {got:?}");
        assert!(
            (time - want_time).abs() <= tolerance,
            "{state:?} at {time:e}, expected {want_time:e} (published {got:?})"
        );
    }
}

/// A band of 1 V to 2 V with no timeout holds: a triangle 0 → 3 → 0 V at
/// 1 V/µs reads 1 where it rises through 2 V and 0 where it falls through
/// 1 V, and nothing where it enters the band.
#[test]
fn a_band_holds_its_decision_between_the_thresholds() {
    let model = ConnectAdc { tracking: false };
    let mut ctx = context(&model, 1, &[("vtlo", 1.0), ("vthi", 2.0)]);
    let triangle = |t: Value| {
        let t = t * 1.0e6;
        if t <= 3.0 { t } else { 6.0 - t }
    };
    let published = run_adc(&model, &mut ctx, "in", &triangle, None, 6.0e-6, 0.13e-6);
    assert_times(
        &published,
        &[
            (DigitalState::Zero, 0.0),
            (DigitalState::One, 2.0e-6),
            (DigitalState::Zero, 5.0e-6),
        ],
        1.0e-12,
    );
}

/// An input held inside the band reads X `txdel` after it entered, at that
/// instant exactly; with `txdel = 0` at the entry itself.
#[test]
fn an_input_held_in_the_band_reads_x_after_the_timeout() {
    let step = |t: Value| if t < 1.5e-6 { 0.0 } else { 1.5 };
    for (timeout, x_at) in [(100.0e-9, 1.6e-6), (0.0, 1.5e-6)] {
        let model = ConnectAdc { tracking: false };
        let mut ctx = context(
            &model,
            1,
            &[("vtlo", 1.0), ("vthi", 2.0), ("txdel", timeout)],
        );
        let published = run_adc(&model, &mut ctx, "in", &step, None, 3.0e-6, 0.13e-6);
        assert_times(
            &published,
            &[(DigitalState::Zero, 0.0), (DigitalState::Unknown, x_at)],
            1.0e-12,
        );
    }
}

/// Leaving the band before the deadline makes it stale: no X follows.
#[test]
fn leaving_the_band_before_the_deadline_publishes_no_x() {
    let model = ConnectAdc { tracking: false };
    let mut ctx = context(
        &model,
        1,
        &[("vtlo", 1.0), ("vthi", 2.0), ("txdel", 100.0e-9)],
    );
    let pulse = |t: Value| {
        if (1.5e-6..1.55e-6).contains(&t) {
            1.5
        } else {
            0.0
        }
    };
    let published = run_adc(&model, &mut ctx, "in", &pulse, None, 3.0e-6, 0.13e-6);
    assert_times(&published, &[(DigitalState::Zero, 0.0)], 0.0);
}

/// A deterministic pseudo-random sequence in `[0, 1)`.
fn random_trace(count: usize, mut seed: u64) -> Vec<Value> {
    (0..count)
        .map(|_| {
            seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (seed >> 11) as Value / (1u64 << 53) as Value
        })
        .collect()
}

/// Two A/Ds stepped through one input trace publish the same events and
/// hold the same state at every point, and declare the same thresholds.
fn assert_adcs_identical(
    ours: (&dyn CodeModel, &mut CmContext),
    theirs: (&dyn CodeModel, &mut CmContext),
    inputs: &[Value],
    supplies: Option<&[Value]>,
) {
    let (ours, our_ctx) = ours;
    let (theirs, their_ctx) = theirs;
    for (step, input) in inputs.iter().enumerate() {
        let time = step as Value * 37.0e-9;
        let supply = supplies.map(|supplies| supplies[step]);
        let a = adc_step(ours, our_ctx, "in", time, &[*input], supply);
        let b = adc_step(theirs, their_ctx, "in", time, &[*input], supply);
        assert_eq!(a, b, "events at step {step}");
        assert_eq!(
            our_ctx.int_state(0),
            their_ctx.int_state(0),
            "state at step {step}"
        );
        assert_eq!(
            our_ctx.state(0).to_bits(),
            their_ctx.state(0).to_bits(),
            "compared quantity at step {step}"
        );
        let a = ours
            .analog_input_thresholds(our_ctx)
            .expect("ours declares");
        let b = theirs
            .analog_input_thresholds(their_ctx)
            .expect("theirs declares");
        assert_eq!(a.len(), b.len());
        for (a, b) in a.iter().zip(&b) {
            assert_eq!(
                (
                    &a.port,
                    a.element,
                    a.low.to_bits(),
                    a.high.to_bits(),
                    a.behavior
                ),
                (
                    &b.port,
                    b.element,
                    b.low.to_bits(),
                    b.high.to_bits(),
                    b.behavior
                )
            );
        }
        assert_eq!(
            ours.analog_threshold_input(our_ctx, "in", 0, *input)
                .to_bits(),
            theirs
                .analog_threshold_input(their_ctx, "in", 0, *input)
                .to_bits()
        );
    }
}

/// `txdel = 0` is `adc_bridge`, bit for bit, on a 50-point random trace.
#[test]
fn a_zero_timeout_is_adc_bridge() {
    let inputs: Vec<_> = random_trace(50, 7).into_iter().map(|u| 3.0 * u).collect();
    let ours = ConnectAdc { tracking: false };
    let mut our_ctx = context(
        &ours,
        1,
        &[
            ("vtlo", 1.0),
            ("vthi", 2.0),
            ("txdel", 0.0),
            ("tdrise", 1.0e-9),
            ("tdfall", 2.0e-9),
        ],
    );
    let mut their_ctx = context(
        &AdcBridge,
        1,
        &[
            ("in_low", 1.0),
            ("in_high", 2.0),
            ("rise_delay", 1.0e-9),
            ("fall_delay", 2.0e-9),
        ],
    );
    assert_adcs_identical(
        (&ours, &mut our_ctx),
        (&AdcBridge, &mut their_ctx),
        &inputs,
        None,
    );
}

/// `txdel = inf` is the HDL host's built-in A/D, bit for bit, static and
/// tracking a live supply, on 50-point random traces.
#[cfg(feature = "veriloga")]
#[test]
fn an_infinite_timeout_is_the_built_in_converter() {
    use crate::xspice::models::MixedAdcBridge;
    let inputs: Vec<_> = random_trace(50, 11).into_iter().map(|u| 3.3 * u).collect();
    // Hit the equal-threshold rule's exact tie too.
    let mut tied = inputs.clone();
    tied[20] = 1.65;
    tied[21] = 1.65;
    let ours = ConnectAdc { tracking: false };
    let theirs = MixedAdcBridge { tracking: false };
    for (low, high) in [(1.0, 2.0), (1.65, 1.65)] {
        let mut our_ctx = context(
            &ours,
            1,
            &[("vtlo", low), ("vthi", high), ("tdrise", 1.0e-9)],
        );
        let mut their_ctx = context(
            &theirs,
            1,
            &[("in_low", low), ("in_high", high), ("rise_delay", 1.0e-9)],
        );
        assert_adcs_identical(
            (&ours, &mut our_ctx),
            (&theirs, &mut their_ctx),
            &tied,
            None,
        );
    }

    let supplies: Vec<_> = random_trace(50, 13)
        .into_iter()
        .map(|u| 2.0 + 1.3 * u)
        .collect();
    let ours = ConnectAdc { tracking: true };
    let theirs = MixedAdcBridge { tracking: true };
    let mut our_ctx = context(&ours, 1, &[]);
    let mut their_ctx = context(&theirs, 1, &[("in_low", 0.5), ("in_high", 0.5)]);
    assert_adcs_identical(
        (&ours, &mut our_ctx),
        (&theirs, &mut their_ctx),
        &inputs,
        Some(&supplies),
    );
}

/// A band whose edges both track the supply: with u held at 1.5 V and the
/// supply ramped 3 → 2 V over 1 µs, the band (0.4, 0.6)·V(sup) passes up
/// through u where 0.6·V(sup) = 1.5 V, at V(sup) = 2.5 V, t = 0.5 µs.
#[test]
fn a_tracking_band_reads_its_crossing_where_the_supply_moves_an_edge_through_the_input() {
    let model = ConnectAdc { tracking: true };
    let mut ctx = context(&model, 1, &[]);
    // Explicit thresholds are absolute volts; the tracked fractions are the
    // values a threshold carries when it is not given explicitly.
    ctx.set_param("vtlo", 0.4);
    ctx.set_param("vthi", 0.6);
    let thresholds = model
        .analog_input_thresholds(&ctx)
        .expect("declares thresholds");
    assert_eq!(thresholds.len(), 2, "one zero threshold per tracked edge");
    let held = |_: Value| 1.5;
    let supply = |t: Value| 3.0 - (t * 1.0e6).min(1.0);
    let published = run_adc(
        &model,
        &mut ctx,
        "in",
        &held,
        Some(&supply),
        1.0e-6,
        0.07e-6,
    );
    assert_times(
        &published,
        &[(DigitalState::Unknown, 0.0), (DigitalState::One, 0.5e-6)],
        2.0e-12,
    );
}

/// Below `vsupmin` a tracking A/D reads X at the localized supply crossing,
/// and recovers when the supply does.
#[test]
fn an_unpowered_converter_reads_x() {
    let model = ConnectAdc { tracking: true };
    let mut ctx = context(&model, 1, &[("vsupmin", 1.0)]);
    let low = |_: Value| 0.0;
    let supply = |t: Value| 3.0 - 3.0 * (t * 1.0e6).min(1.0) + 3.0 * (t * 1.0e6 - 1.0).max(0.0);
    let published = run_adc(&model, &mut ctx, "in", &low, Some(&supply), 2.0e-6, 0.09e-6);
    // V(sup) = 1 V at 2/3 µs falling and 4/3 µs rising.
    assert_times(
        &published,
        &[
            (DigitalState::Zero, 0.0),
            (DigitalState::Unknown, 2.0e-6 / 3.0),
            (DigitalState::Zero, 4.0e-6 / 3.0),
        ],
        2.0e-12,
    );
}

//=============================================================================
// D/A
//=============================================================================

/// Evaluate a D/A at `time` with `bits` on its input (each changed at
/// `event`) as an accepted point, and advance its state. Returns each
/// element's Norton pair `(-G·level, G)`.
fn dac_step(
    model: &dyn CodeModel,
    ctx: &mut CmContext,
    time: Value,
    bits: &[DigitalValue],
    event: Option<Value>,
    supply: Option<Value>,
) -> Vec<(Value, Value)> {
    let pairs = dac_evaluate(model, ctx, time, bits, event, supply);
    ctx.advance_state();
    pairs
}

/// [`dac_step`] without accepting the point.
fn dac_evaluate(
    model: &dyn CodeModel,
    ctx: &mut CmContext,
    time: Value,
    bits: &[DigitalValue],
    event: Option<Value>,
    supply: Option<Value>,
) -> Vec<(Value, Value)> {
    ctx.time = time;
    ctx.set_input("in", InputValue::DigitalVector(bits.to_vec()));
    ctx.set_input_digital_vector_event_times("in", vec![event; bits.len()]);
    set_supply(ctx, supply);
    ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
    model.evaluate(ctx).expect("the converter evaluates");
    (0..bits.len())
        .map(|index| {
            (
                ctx.output_vector_value("out", index),
                ctx.partial_vector_value("out", index),
            )
        })
        .collect()
}

/// The Thevenin level a Norton pair stands for.
fn level((value, conductance): (Value, Value)) -> Value {
    -value / conductance
}

/// A D/A driving `cap` from 0 V through each edge: the instants the node
/// crosses `fraction` of the swing after a rise to 1 at 10 ns and a fall to
/// 0 at 300 ns. The node is integrated exactly between points, the drive
/// at each point's end held over the step (the edge point is the left
/// limit, so the edge acts from the first point after it).
fn rc_crossings(model: &dyn CodeModel, ctx: &mut CmContext, cap: Value) -> (Value, Value) {
    let step = 2.0e-12;
    let mut node = 0.0;
    let (mut rise, mut fall) = (None, None);
    let mut time = 0.0;
    let vhi = 3.3;
    while time < 600.0e-9 {
        let (bit, event) = if time < 10.0e-9 {
            (DigitalValue::zero(), None)
        } else if time < 300.0e-9 {
            (DigitalValue::one(), Some(10.0e-9))
        } else {
            (DigitalValue::zero(), Some(300.0e-9))
        };
        let pair = dac_step(model, ctx, time, &[bit], event, None)[0];
        let source = level(pair);
        let before = node;
        node = source + (node - source) * (-pair.1 * step / cap).exp();
        let interpolate = |threshold: Value| time + step * (threshold - before) / (node - before);
        let high = 0.632 * vhi;
        let low = 0.368 * vhi;
        if rise.is_none() && before < high && node >= high {
            rise = Some(interpolate(high) - 10.0e-9);
        }
        if fall.is_none() && time >= 300.0e-9 && before > low && node <= low {
            fall = Some(interpolate(low) - 300.0e-9);
        }
        time += step;
    }
    (rise.expect("the node rises"), fall.expect("the node falls"))
}

/// Per-state resistance: `rhi` 10 Ω and `rlo` 5 Ω into 1 nF give τ = 10 ns
/// for the rise and 5 ns for the fall; an X behind `rx` 50 Ω at `vx` 1.65 V
/// into 1 kΩ sits at 1.65·1000/1050.
#[test]
fn each_state_drives_through_its_own_resistance() {
    let model = ConnectDac { tracking: false };
    let mut ctx = context(&model, 1, &[("vhi", 3.3), ("rhi", 10.0), ("rlo", 5.0)]);
    let (rise, fall) = rc_crossings(&model, &mut ctx, 1.0e-9);
    // The 63.2% point of an exponential is ln(1/0.368) τ = 0.99967 τ.
    assert!(
        (rise - 10.0e-9).abs() <= 1.0e-3 * 10.0e-9,
        "rise τ {rise:e}"
    );
    assert!((fall - 5.0e-9).abs() <= 1.0e-3 * 5.0e-9, "fall τ {fall:e}");

    let mut ctx = context(&model, 1, &[("vhi", 3.3), ("vx", 1.65), ("rx", 50.0)]);
    ctx.analysis = AnalysisType::DcOp;
    let (value, g) = dac_step(
        &model,
        &mut ctx,
        0.0,
        &[DigitalValue::unknown()],
        None,
        None,
    )[0];
    // G·(V − Vs) + V/R = 0 with the stamp's I = value + G·V.
    let node = -value / (g + 1.0 / 1000.0);
    assert!(
        (node - 1.65 * 1000.0 / 1050.0).abs() <= 1.0e-12,
        "X node {node}"
    );
}

/// `transition()` shaping: with 10 ns edges, a fall 5 ns into a rise starts
/// from half the swing and still takes the full 10 ns, ending at 25 ns, not
/// at 20 ns as a constant slope would.
#[test]
fn an_interrupted_change_restarts_from_the_present_level_and_takes_its_full_time() {
    let model = ConnectDac { tracking: false };
    let mut ctx = context(
        &model,
        1,
        &[("vhi", 3.3), ("trise", 10.0e-9), ("tfall", 10.0e-9)],
    );
    let zero = DigitalValue::zero();
    let one = DigitalValue::one();
    let at = |ctx: &mut CmContext, time, bit, event| {
        level(dac_step(&model, ctx, time, &[bit], event, None)[0])
    };
    assert_eq!(at(&mut ctx, 0.0, zero, None), 0.0);
    assert_eq!(at(&mut ctx, 10.0e-9, one, Some(10.0e-9)), 0.0);
    assert_breakpoints(&mut ctx, &[20.0e-9]);
    let mid = at(&mut ctx, 15.0e-9, zero, Some(15.0e-9));
    assert!((mid - 1.65).abs() < 1.0e-12, "the fall starts from {mid}");
    assert_breakpoints(&mut ctx, &[25.0e-9]);
    let quarter = at(&mut ctx, 20.0e-9, zero, Some(15.0e-9));
    assert!((quarter - 0.825).abs() < 1.0e-12, "halfway down: {quarter}");
    let end = at(&mut ctx, 25.0e-9, zero, Some(15.0e-9));
    assert!(end.abs() < 1.0e-12, "down at 25 ns: {end}");
}

fn assert_breakpoints(ctx: &mut CmContext, expected: &[Value]) {
    let got = ctx.take_requested_breakpoints();
    assert_eq!(got.len(), expected.len(), "breakpoints {got:?}");
    for (got, expected) in got.iter().zip(expected) {
        assert!(
            (got - expected).abs() <= 1.0e-21,
            "breakpoint {got:e}, expected {expected:e}"
        );
    }
}

/// `tdelay` holds the old drive until the change starts, and asks the
/// stepper to land on the start.
#[test]
fn a_delayed_change_starts_tdelay_after_its_event() {
    let model = ConnectDac { tracking: false };
    let mut ctx = context(
        &model,
        1,
        &[("vhi", 3.3), ("trise", 10.0e-9), ("tdelay", 3.0e-9)],
    );
    let one = DigitalValue::one();
    let at = |ctx: &mut CmContext, time, bit, event| {
        level(dac_step(&model, ctx, time, &[bit], event, None)[0])
    };
    let start = 10.0e-9 + 3.0e-9;
    assert_eq!(at(&mut ctx, 0.0, DigitalValue::zero(), None), 0.0);
    assert_eq!(at(&mut ctx, 10.0e-9, one, Some(10.0e-9)), 0.0);
    assert_breakpoints(&mut ctx, &[start]);
    assert_eq!(at(&mut ctx, 12.0e-9, one, Some(10.0e-9)), 0.0);
    assert_eq!(
        at(&mut ctx, start, one, Some(10.0e-9)),
        0.0,
        "left limit at the start"
    );
    assert_breakpoints(&mut ctx, &[start, start + 10.0e-9]);
    let early = at(&mut ctx, start + 1.0e-9, one, Some(10.0e-9));
    assert!(
        (early - 0.33).abs() < 1.0e-12,
        "a tenth of the way: {early}"
    );
}

/// The node several driver elements tie together: `Σ G·V / Σ G`, with an
/// optional resistor to a rail.
fn tied_node(pairs: &[(Value, Value)], pull: Option<(Value, Value)>) -> Value {
    let (mut current, mut conductance) = (0.0, 0.0);
    for (value, g) in pairs {
        current -= value;
        conductance += g;
    }
    if let Some((rail, resistance)) = pull {
        current += rail / resistance;
        conductance += 1.0 / resistance;
    }
    current / conductance
}

/// Driver-sensitive conversion: each driver is its own Thevenin branch.
/// A resistive 1 (`rpull` 10 kΩ) against a strong 0 (20 Ω) sits at
/// 3.3·20/10020; a weak 1 at 3.3·(1/100k)/(1/100k + 1/20); a strong 1
/// against a strong 0 with a 2.2 kΩ pull-up at 3.3·(1/20 + 1/2200)/(2/20 +
/// 1/2200).
#[test]
fn each_driver_converts_at_its_own_strength() {
    let model = ConnectDac { tracking: false };
    let mut ctx = context(&model, 2, &[("vhi", 3.3)]);
    ctx.analysis = AnalysisType::DcOp;
    let resistive_one = DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive);
    let pairs = dac_step(
        &model,
        &mut ctx,
        0.0,
        &[resistive_one, DigitalValue::zero()],
        None,
        None,
    );
    let node = tied_node(&pairs, None);
    assert!((node - 3.3 * 20.0 / 10_020.0).abs() <= 1.0e-6, "{node}");

    let pairs = dac_step(
        &model,
        &mut ctx,
        0.0,
        &[DigitalValue::one(), DigitalValue::zero()],
        None,
        None,
    );
    let node = tied_node(&pairs, Some((3.3, 2200.0)));
    let expected = 3.3 * (1.0 / 20.0 + 1.0 / 2200.0) / (2.0 / 20.0 + 1.0 / 2200.0);
    assert!((node - expected).abs() <= 1.0e-9, "{node} vs {expected}");
    assert!((expected - 1.6575).abs() < 1.0e-4);

    // Weak and supply strength reach the converter from HDL drive strengths.
    let law = DacLaw::from_params(&ctx, model.name(), false).expect("the law resolves");
    let weak = conductance(law.resistance(DriveState::One, ConverterStrength::Weak));
    let strong = conductance(law.resistance(DriveState::Zero, ConverterStrength::Strong));
    let node = 3.3 * weak / (weak + strong);
    assert!((node - 0.660e-3).abs() <= 1.0e-6, "{node}");
    use ConverterStrength as S;
    let table = [
        (DriveState::Zero, S::Supply, 2.0),
        (DriveState::One, S::Supply, 2.0),
        (DriveState::Unknown, S::Supply, 20.0),
        (DriveState::Unknown, S::Strong, 20.0),
        (DriveState::Unknown, S::Pull, 1.0e4),
        (DriveState::Unknown, S::Weak, 1.0e5),
        (DriveState::One, S::HighZ, Value::INFINITY),
        (DriveState::HighZ, S::Strong, Value::INFINITY),
        (DriveState::Zero, S::Undetermined, 20.0),
    ];
    for (state, strength, resistance) in table {
        assert_eq!(
            law.resistance(state, strength),
            resistance,
            "{state:?} {strength:?}"
        );
    }
}

/// The small-signal stamp of a held 1 is `G = 1/rhi`; tracking the supply
/// adds `−G` from it (`vhi` at its default, the whole supply) and `−0` for an
/// explicit `vhi`; Z behind `rz = inf` stamps nothing; the thermal noise is
/// `4kT/R`.
#[test]
fn the_held_stamp_is_the_drive_resistance_and_its_supply_partial() {
    let model = ConnectDac { tracking: true };
    for (explicit, partial) in [(false, -0.1), (true, 0.0)] {
        let params: &[(&str, Value)] = if explicit {
            &[("vhi", 3.3), ("rhi", 10.0)]
        } else {
            &[("rhi", 10.0)]
        };
        let mut ctx = context(&model, 1, params);
        ctx.analysis = AnalysisType::Ac;
        let (value, g) = dac_step(
            &model,
            &mut ctx,
            0.0,
            &[DigitalValue::one()],
            None,
            Some(3.3),
        )[0];
        assert_eq!(g, 0.1);
        assert!((value + 0.1 * 3.3).abs() < 1.0e-15);
        let partials = model.output_vector_input_partials(&ctx, "out", 0);
        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].0, "sup");
        assert_eq!(partials[0].1, partial);
        let noise = model.converter_noise_conductances(&ctx);
        assert_eq!(noise.len(), 1);
        let density = BOLTZMANN_4KT * noise[0].1;
        assert!((density - BOLTZMANN_4KT / 10.0).abs() <= 1.0e-12 * density);

        let (_, g) = dac_step(
            &model,
            &mut ctx,
            0.0,
            &[DigitalValue::high_z()],
            None,
            Some(3.3),
        )[0];
        assert_eq!(g, 0.0);
        assert!(
            model
                .output_vector_input_partials(&ctx, "out", 0)
                .is_empty()
        );
        assert!(model.converter_noise_conductances(&ctx).is_empty());
    }
}

/// Below `vsupmin` a tracking D/A drives `vlo` through `rx`.
#[test]
fn an_unpowered_dac_drives_low_through_rx() {
    let model = ConnectDac { tracking: true };
    let mut ctx = context(&model, 1, &[("vsupmin", 1.0), ("rx", 50.0)]);
    ctx.analysis = AnalysisType::DcOp;
    let thresholds = model.analog_input_thresholds(&ctx).expect("declares");
    assert_eq!(thresholds.len(), 1);
    assert_eq!(
        (thresholds[0].port.as_str(), thresholds[0].low),
        ("sup", 1.0)
    );
    let (value, g) = dac_step(
        &model,
        &mut ctx,
        0.0,
        &[DigitalValue::one()],
        None,
        Some(0.5),
    )[0];
    assert_eq!((value, g), (0.0, 1.0 / 50.0));
}

/// A deterministic trace of strong 0, 1, X and Z.
#[cfg(feature = "veriloga")]
fn random_bits(count: usize, seed: u64) -> Vec<DigitalValue> {
    random_trace(count, seed)
        .into_iter()
        .map(|u| match (u * 4.0) as usize {
            0 => DigitalValue::zero(),
            1 => DigitalValue::one(),
            2 => DigitalValue::unknown(),
            _ => DigitalValue::high_z(),
        })
        .collect()
}

/// At its defaults the D/A is the HDL host's built-in D/A, bit for bit,
/// static and tracking a live supply: the same Norton pair, breakpoints,
/// supply partial and noise at every point of a 50-point random trace.
#[cfg(feature = "veriloga")]
#[test]
fn the_default_dac_is_the_built_in_converter() {
    use crate::xspice::models::MixedDacBridge;
    let bits = random_bits(50, 17);
    let supplies: Vec<_> = random_trace(50, 19)
        .into_iter()
        .map(|u| 2.0 + 1.3 * u)
        .collect();
    for tracking in [false, true] {
        let ours = ConnectDac { tracking };
        let theirs = MixedDacBridge { tracking };
        let (our_params, their_params): (Params, Params) = if tracking {
            (
                &[],
                &[("out_low", 0.0), ("out_high", 1.0), ("out_undef", 0.5)],
            )
        } else {
            (
                &[("vhi", 3.3), ("vx", 1.65)],
                &[("out_low", 0.0), ("out_high", 3.3), ("out_undef", 1.65)],
            )
        };
        let mut our_ctx = context(&ours, 1, our_params);
        let mut their_ctx = context(&theirs, 1, their_params);
        let mut event = None;
        let mut previous = None;
        for (step, bit) in bits.iter().enumerate() {
            let time = step as Value * 41.0e-9;
            if previous != Some(*bit) {
                event = Some(time);
                previous = Some(*bit);
            }
            let supply = tracking.then(|| supplies[step]);
            // Two points on each event: the edge itself and one just after.
            for time in [time, time + 3.0e-12] {
                // Asked as the engine asks: the stamp, its partials and the
                // noise of one point, before that point is accepted.
                let a = dac_evaluate(&ours, &mut our_ctx, time, &[*bit], event, supply);
                let b = dac_evaluate(&theirs, &mut their_ctx, time, &[*bit], event, supply);
                assert_eq!(
                    (a[0].0.to_bits(), a[0].1.to_bits()),
                    (b[0].0.to_bits(), b[0].1.to_bits()),
                    "Norton pair at {time:e} for {bit:?}, tracking {tracking}"
                );
                assert_eq!(
                    our_ctx.take_requested_breakpoints(),
                    their_ctx.take_requested_breakpoints(),
                    "breakpoints at {time:e}"
                );
                let a = ours.output_vector_input_partials(&our_ctx, "out", 0);
                let b = theirs.output_vector_input_partials(&their_ctx, "out", 0);
                assert_eq!(a.len(), b.len(), "supply partials at {time:e}");
                for (a, b) in a.iter().zip(&b) {
                    assert_eq!(a.0, b.0);
                    assert_eq!(a.1, b.1, "supply partial at {time:e}");
                }
                // The built-in D/A lists noise by its input alone, so the two
                // agree everywhere but on a seizing edge's own left limit.
                if time > event.unwrap_or(0.0) {
                    assert_eq!(
                        ours.converter_noise_conductances(&our_ctx),
                        theirs.converter_noise_conductances(&their_ctx),
                        "noise at {time:e}"
                    );
                }
                our_ctx.advance_state();
                their_ctx.advance_state();
            }
        }
    }
}

//=============================================================================
// Bidirectional
//=============================================================================

/// A port driving 0 against a 2.2 kΩ pull-up reads its own V_OL =
/// 3.3·20/2220 through the receiving half, which publishes 0; released, the
/// pull-up takes the node to 3.3 V and the port reads 1.
#[test]
fn a_bidirectional_port_drives_and_reads_its_own_node() {
    let model = ConnectBidi { tracking: false };
    let mut ctx = context(&model, 1, &[("vhi", 3.3), ("vtlo", 1.65), ("vthi", 1.65)]);
    let mut published = Vec::new();
    for (time, bit) in [
        (0.0, DigitalValue::zero()),
        (10.0e-9, DigitalValue::high_z()),
    ] {
        ctx.time = time;
        ctx.set_input("in", InputValue::DigitalVector(vec![bit]));
        ctx.set_input_digital_vector_event_times("in", vec![Some(time)]);
        // Solve the node against the pull-up with the drive as stamped.
        let mut node = 3.3;
        for _ in 0..3 {
            ctx.set_input("sense", analog(&[node]));
            ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
            model.evaluate(&mut ctx).expect("evaluates");
            let pair = (
                ctx.output_vector_value("out", 0),
                ctx.partial_vector_value("out", 0),
            );
            node = tied_node(&[pair], Some((3.3, 2200.0)));
        }
        ctx.set_input("sense", analog(&[node]));
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        model.evaluate(&mut ctx).expect("evaluates");
        published.extend(
            ctx.take_pending_events()
                .into_iter()
                .map(|e| e.values[0].state),
        );
        if time == 0.0 {
            assert!((node - 3.3 * 20.0 / 2220.0).abs() <= 1.0e-12, "V_OL {node}");
        }
        ctx.advance_state();
        // The released level arrives at the point after the edge.
        if time > 0.0 {
            ctx.time = time + 1.0e-9;
            ctx.set_input("sense", analog(&[3.3]));
            model.evaluate(&mut ctx).expect("evaluates");
            assert_eq!(ctx.partial_vector_value("out", 0), 0.0, "released");
            published.extend(
                ctx.take_pending_events()
                    .into_iter()
                    .map(|e| e.values[0].state),
            );
            ctx.advance_state();
        }
    }
    assert_eq!(published, vec![DigitalState::Zero, DigitalState::One]);
    assert_eq!(model.converter_noise_conductances(&ctx), Vec::new());

    let tracking = ConnectBidi { tracking: true };
    let ctx = context(&tracking, 1, &[("vtlo", 1.0), ("vsupmin", 1.0)]);
    let thresholds = tracking.analog_input_thresholds(&ctx).expect("declares");
    let ports: Vec<_> = thresholds.iter().map(|t| t.port.as_str()).collect();
    assert_eq!(ports, ["sense", "sense", "sup"]);
    assert!(tracking.output_vector_input_partials(&ctx, "out", 0).len() <= 1);
    assert_eq!(tracking.analog_threshold_input(&ctx, "sup", 0, 2.0), 2.0);
}
