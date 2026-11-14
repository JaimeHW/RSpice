use crate::abort_signal::NoAbort;
use crate::analysis::PssConfig;
use crate::analysis::pss::PssResult;
use crate::{Engine, Netlist, SimulationConfig, Value};

fn waveform<'a>(result: &'a PssResult, node: &str) -> &'a [Value] {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} in {:?}", result.node_names));
    &result.waveforms[index].values
}

/// The value at `time`, linearly interpolated on the orbit's own mesh.
fn sample(result: &PssResult, node: &str, time: Value) -> Value {
    let values = waveform(result, node);
    let index = result
        .time
        .partition_point(|t| *t <= time)
        .clamp(1, result.time.len() - 1);
    let (t0, t1) = (result.time[index - 1], result.time[index]);
    let (v0, v1) = (values[index - 1], values[index]);
    if t1 == t0 {
        v0
    } else {
        v0 + (v1 - v0) * (time - t0) / (t1 - t0)
    }
}

/// The trapezoidal mean of a node's waveform over the period.
fn mean(result: &PssResult, node: &str) -> Value {
    let values = waveform(result, node);
    let area: Value = result
        .time
        .windows(2)
        .zip(values.windows(2))
        .map(|(t, v)| 0.5 * (v[0] + v[1]) * (t[1] - t[0]))
        .sum();
    area / result.period
}

/// The switched-capacitor filter of the C11 analog-clock twin: two PULSE
/// clocks with nonoverlapping phases drive ideal hysteretic switches.
const SC_PULSE: &str = "sc pulse
.param vcc=1.8
vph1 ph1 0 pulse(0 {vcc} 20n 1n 1n 459n 1u)
vph2 ph2 0 pulse(0 {vcc} 520n 1n 1n 459n 1u)
s1 in a ph1 0 swc
c1 a 0 1p
s2 a out ph2 0 swc
c2 out 0 9p
.model swc sw vt=0.9 vh=0.1 ron=1k roff=1e12
vin in 0 dc 0.5 sin(0.5 0.2 1meg 0 0 -83)
.end
";

/// The filter's orbit holds the sample its carrier peaks at: the fixed
/// point of y = 0.1·x_s + 0.9·y is x_s = 0.5 + 0.2/(1 + (ω₀·Ron·C1)²)
/// = 0.6999921 V, and v(a) tracks the carrier through Ron·C1 = 1 ns while
/// S1 is closed. Ten periods of transient from the 0.5 V operating point
/// leave v(out) 0.07 V short; the shooting closes it.
#[test]
fn event_shooting_holds_the_switched_capacitor_sample_on_its_orbit() {
    let netlist = Netlist::parse(SC_PULSE).expect("the filter parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(1.0e6).with_tstab(10.0e-6);
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
        .expect("event shooting solves the switched-capacitor orbit");
    assert!(point.is_event_orbit(), "a switched circuit is event-shot");
    let result = &point.analysis().result;
    let event = engine
        .run_event_pss(&netlist, &config, None, &NoAbort)
        .expect("the same orbit");
    eprintln!("certified: {:?}", event.certification);
    let held = mean(result, "out");
    let x_s = 0.5 + 0.2 / (1.0 + (2.0 * std::f64::consts::PI * 1.0e6 * 1.0e-9).powi(2));
    eprintln!(
        "held {held} against {x_s}: Newton {} corrections, {} samples",
        result.iterations,
        result.time.len()
    );
    assert!((held - x_s).abs() <= 5.0e-5, "held {held}, x_s {x_s}");
    let omega = 2.0 * std::f64::consts::PI * 1.0e6;
    let tau = 1.0e-9;
    let gain = 1.0 / (1.0 + (omega * tau).powi(2)).sqrt();
    let tracked =
        0.5 + 0.2 * gain * (omega * 3.0e-7 - 83.0_f64.to_radians() - (omega * tau).atan()).sin();
    for (time, expected) in [(3.0e-7, tracked), (7.0e-7, x_s)] {
        let got = sample(result, "a", time);
        assert!(
            (got - expected).abs() <= 5.0e-5,
            "v(a) at {time:e} s is {got}, the closed form {expected}"
        );
    }
}

/// A half-wave peak detector, a diode into an RC with RC equal to the
/// period, is a circuit both engines admit. Event shooting's orbit meets
/// the fixed-grid shooting orbit at every native mesh point to the two
/// discretizations' difference: T/1024 trapezoidal steps against the
/// transient's own mesh, bisected until it stops moving (1.05e-6 V measured
/// on the 1.33 V peak).
#[test]
fn event_shooting_meets_native_shooting_on_a_diode_rc() {
    let netlist = Netlist::parse(
        "diode rc
vin in 0 sin(0 2 1meg)
d1 in out dmod
r1 out 0 1k
c1 out 0 1n
.model dmod d is=1e-14
.end
",
    )
    .expect("the diode RC parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(1.0e6)
        .with_tstab(5.0e-6)
        .with_points_per_period(1024);
    let native = engine
        .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
        .expect("native PSS");
    assert!(!native.is_event_orbit());
    let event = engine
        .run_event_pss(&netlist, &config, None, &NoAbort)
        .expect("event PSS");
    let (native, event) = (&native.analysis().result, &event.analysis.result);
    let peak = waveform(native, "out")
        .iter()
        .fold(0.0_f64, |peak, value| peak.max(value.abs()));
    let worst = native
        .time
        .iter()
        .zip(waveform(native, "out"))
        .map(|(time, value)| (sample(event, "out", *time) - value).abs())
        .fold(0.0_f64, Value::max);
    eprintln!("diode RC: worst |event - native| = {worst:e} on a {peak} V peak");
    assert!(
        worst <= 1.0e-5,
        "worst |event - native| = {worst:e} against a {peak} V peak"
    );
}

/// An oscillator whose switch state native shooting cannot carry is refused
/// by name: event shooting solves driven orbits only.
#[test]
fn an_autonomous_switched_orbit_is_refused_by_name() {
    let netlist = Netlist::parse(SC_PULSE).expect("the filter parses");
    let engine = Engine::new(SimulationConfig::default());
    let error = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::autonomous().with_period_guess(1.0e-6),
            &NoAbort,
        )
        .expect_err("an autonomous event orbit is refused");
    assert!(
        error.to_string().contains("analysis.pss.event_autonomous"),
        "{error}"
    );
}

/// A Verilog-A module written where a deck can include it.
#[cfg(feature = "veriloga")]
fn write_module(name: &str, source: &str) -> String {
    let dir = std::env::temp_dir().join(format!("rspice-event-pss-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a module directory");
    let path = dir.join(format!("{name}.va"));
    std::fs::write(&path, source).expect("the module is written");
    path.to_string_lossy().replace('\\', "/")
}

/// A runtime Verilog-A RC whose charge native shooting integrates is a
/// circuit both engines admit; event shooting carries the module's `ddt`
/// state as a coordinate of its own and meets the native orbit to the two
/// discretizations' difference (6.2e-6 V measured on a 1 V drive).
#[cfg(feature = "veriloga")]
#[test]
fn event_shooting_meets_native_shooting_on_a_veriloga_rc() {
    let model = write_module(
        "va_rc",
        "`include \"disciplines.vams\"\n\
         module va_rc(p, n);\n\
         inout p, n;\n\
         electrical p, n, x;\n\
         analog begin\n\
         I(p, x) <+ V(p, x) / 500.0;\n\
         I(x, n) <+ ddt(100.0e-12 * V(x, n));\n\
         end\n\
         endmodule\n",
    );
    let deck = format!(
        "va rc\n\
         V1 in 0 DC 0 SIN(0.5 1 1meg)\n\
         R1 in a 1k\n\
         XVA a 0 va_rc\n\
         .va \"{model}\" va_rc\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(1.0e6).with_points_per_period(1024);
    let native = engine
        .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
        .expect("native shooting takes the charge-form module");
    assert!(!native.is_event_orbit());
    let event = engine
        .run_event_pss(&netlist, &config, None, &NoAbort)
        .expect("event shooting takes it too");
    assert!(
        event.basis.iter().any(|name| name.contains("XVA")),
        "the module's state is a coordinate: {:?}",
        event.basis
    );
    let (native, event) = (&native.analysis().result, &event.analysis.result);
    let worst = native
        .time
        .iter()
        .zip(waveform(native, "a"))
        .map(|(time, value)| (sample(event, "a", *time) - value).abs())
        .fold(0.0_f64, Value::max);
    eprintln!("Verilog-A RC: worst |event - native| = {worst:e}");
    assert!(worst <= 1.0e-5, "worst |event - native| = {worst:e}");
}

/// A Verilog-A sampler holds its input where its clock rises through
/// 0.9 V: `@(cross(V(clk) - 0.9, +1)) held = V(in)` and `V(out) <+ held`.
/// Native shooting refuses it three times over (the crossing, the retained
/// `held` and the voltage contribution); event shooting holds the sample
/// over the whole period: 0.5 + 0.2·sin(ω₀·100.5 ns), where the 1 ns ramp
/// from 100 ns crosses 0.9 V.
#[cfg(feature = "veriloga")]
#[test]
fn event_shooting_holds_a_veriloga_samplers_crossing() {
    let model = write_module(
        "va_sampler",
        "`include \"disciplines.vams\"\n\
         module va_sampler(in, clk, out);\n\
         input in, clk;\n\
         output out;\n\
         electrical in, clk, out;\n\
         real held;\n\
         analog begin\n\
         @(cross(V(clk) - 0.9, +1)) held = V(in);\n\
         V(out) <+ held;\n\
         end\n\
         endmodule\n",
    );
    let deck = format!(
        "va sampler\n\
         vclk clk 0 pulse(0 1.8 100n 1n 1n 498n 1u)\n\
         vin in 0 sin(0.5 0.2 1meg)\n\
         XS in clk out va_sampler\n\
         rl out 0 1k\n\
         .va \"{model}\" va_sampler\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
        .expect("event shooting solves the sampler");
    assert!(point.is_event_orbit());
    let expected = 0.5 + 0.2 * (2.0 * std::f64::consts::PI * 1.0e6 * 100.5e-9).sin();
    let result = &point.analysis().result;
    let worst = waveform(result, "out")
        .iter()
        .map(|value| (value - expected).abs())
        .fold(0.0_f64, Value::max);
    eprintln!("sampler: worst |out - {expected}| = {worst:e}");
    assert!(worst <= 1.0e-4, "worst |out - held| = {worst:e}");
}

/// A divider whose output toggles at every clock edge comes back only every
/// second period, and is refused by name with the fundamental that would
/// close it.
#[cfg(feature = "veriloga")]
#[test]
fn a_discrete_state_that_repeats_every_second_period_is_refused_by_name() {
    let model = write_module(
        "va_divider",
        "`include \"disciplines.vams\"\n\
         module va_divider(clk, out);\n\
         input clk;\n\
         output out;\n\
         electrical clk, out;\n\
         integer state;\n\
         analog begin\n\
         @(cross(V(clk) - 0.9, +1)) state = 1 - state;\n\
         V(out) <+ state;\n\
         end\n\
         endmodule\n",
    );
    let deck = format!(
        "va divider\n\
         vclk clk 0 pulse(0 1.8 100n 1n 1n 498n 1u)\n\
         XD clk out va_divider\n\
         rl out 0 1k\n\
         .va \"{model}\" va_divider\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_event_pss(&netlist, &PssConfig::new(1.0e6), None, &NoAbort)
        .expect_err("a divide-by-two orbit is not periodic at the clock");
    let text = error.to_string();
    assert!(
        text.contains("analysis.pss.discrete_state_not_periodic")
            && text.contains("every 2 periods"),
        "{text}"
    );
}

/// A module whose `absdelay` history no coordinate carries keeps native
/// shooting's refusal, which names the operator where the module wrote it.
#[cfg(feature = "veriloga")]
#[test]
fn a_delay_history_is_refused_as_opaque_state() {
    let model = write_module(
        "va_delay",
        "`include \"disciplines.vams\"\n\
         module va_delay(in, out);\n\
         input in;\n\
         output out;\n\
         electrical in, out;\n\
         analog V(out) <+ absdelay(V(in), 10n);\n\
         endmodule\n",
    );
    let deck = format!(
        "va delay\n\
         vin in 0 sin(0 1 1meg)\n\
         XD in out va_delay\n\
         rl out 0 1k\n\
         .va \"{model}\" va_delay\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
        .expect_err("a delay history is refused");
    let text = error.to_string();
    assert!(
        text.contains("analysis.pss.veriloga_operator") && text.contains("`absdelay` at"),
        "{text}"
    );
}
