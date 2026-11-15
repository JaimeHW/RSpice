use crate::abort_signal::NoAbort;
use crate::analysis::PssConfig;
use crate::analysis::pss::PssResult;
use crate::{Complex64, Engine, Netlist, SimulationConfig, Value};

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
/// second period: PSS hands it to event shooting, which refuses it by name
/// with the fundamental that would close it.
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
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
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
/// PAC around the switched-capacitor twin's event orbit, against the
/// sampled-data transfer: with the sample at t_s = 480.556 ns, the update
/// at t_u = 520.556 ns, a = C1/(C1 + C2) and H_d(ω) = a/(1 − (1 − a)e^{−jωT}),
/// Y_m = H_d·e^{jω(t_s − t_u)}·e^{−jmω₀t_u}·(1 − e^{−jωT})/(j(ω + mω₀)T).
/// The Ron·C time constants (about 1 ns) the closed form neglects move the
/// phase by at most 0.04° at 100 kHz; the allowance is 0.5 % and 0.5°.
#[test]
fn event_pac_meets_the_sampled_data_transfer_of_the_switched_capacitor_twin() {
    use crate::analysis::{PacConfig, PacSweepType};
    use num_complex::Complex64;
    let deck = SC_PULSE.replace(
        "sin(0.5 0.2 1meg 0 0 -83)",
        "sin(0.5 0.2 1meg 0 0 -83) ac 1",
    );
    let netlist = Netlist::parse(&deck).expect("the filter parses");
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1.0e6).with_tstab(10.0e-6),
            &NoAbort,
        )
        .expect("event shooting solves the switched-capacitor orbit");
    let config = PacConfig::new()
        .with_sweep(1.0e3, 1.0e5, 1)
        .with_sweep_type(PacSweepType::Decade)
        .with_sidebands(-1, 1)
        .with_input_source("vin")
        .with_output_node("out");
    let pac = engine
        .run_pac_from_pss_with_abort(&netlist, config, &point, &NoAbort)
        .expect("PAC runs on the event orbit");
    let result = &pac.result;
    let out = result.node_index("out").expect("v(out) is solved");
    let period = 1.0e-6;
    let omega_0 = std::f64::consts::TAU / period;
    let (sample, update) = (480.0e-9 + 1.0e-9 / 1.8, 520.0e-9 + 1.0e-9 / 1.8);
    let share = 0.1;
    assert_eq!(result.frequencies.len(), 3, "{:?}", result.frequencies);
    for (index, &frequency) in result.frequencies.iter().enumerate() {
        let omega = std::f64::consts::TAU * frequency;
        let z_inverse = Complex64::from_polar(1.0, -omega * period);
        for m in [-1, 0, 1] {
            let shifted = omega + f64::from(m) * omega_0;
            let expected = share / (1.0 - (1.0 - share) * z_inverse)
                * Complex64::from_polar(1.0, omega * (sample - update))
                * Complex64::from_polar(1.0, -f64::from(m) * omega_0 * update)
                * (1.0 - z_inverse)
                / (Complex64::i() * shifted * period);
            let got = result
                .get_sideband_data(index, m)
                .expect("the sideband is published")
                .node_voltages[out];
            let magnitude = got.norm() / expected.norm() - 1.0;
            let phase = (got / expected).arg().to_degrees();
            println!(
                "pac {frequency:e} Hz sb {m:+}: {got:.6e} against {expected:.6e} \
                 ({magnitude:+.3e}, {phase:+.3}°)"
            );
            assert!(
                magnitude.abs() < 5.0e-3 && phase.abs() < 0.5,
                "sideband {m} at {frequency:e} Hz: {got} against {expected}"
            );
            let converted = result
                .conversion_matrix
                .get(index, m, 0)
                .expect("the conversion matrix holds the input at sideband 0");
            assert!((converted - got).norm() <= 1.0e-12 * got.norm().max(1.0e-30));
        }
    }
}

/// A gain an `@(initial_step)` block sets is state neither native shooting
/// nor harmonic balance carries: both evaluated the module with the gain at
/// its declared zero and returned a flat orbit, 0 V where the transient
/// swings 1.69 V. Both now refuse the module by name, and PSS hands it to
/// event shooting, whose orbit meets the closed form
/// 2 mA/V · 1 V · |1 kΩ ∥ 100 pF| at 1 MHz = 1.69347 V.
#[cfg(feature = "veriloga")]
#[test]
fn a_gain_an_initial_step_sets_is_carried_by_event_shooting_alone() {
    let model = write_module(
        "va_initial_gain",
        "`include \"disciplines.vams\"\n\
         module va_initial_gain(inp, out);\n\
         input inp;\n\
         output out;\n\
         electrical inp, out;\n\
         real gain;\n\
         analog begin\n\
         @(initial_step) gain = 2.0;\n\
         I(out) <+ -gain * V(inp) * 1m;\n\
         end\n\
         endmodule\n",
    );
    let deck = format!(
        "va initial gain\n\
         vin inp 0 sin(0 1 1meg)\n\
         XG inp out va_initial_gain\n\
         rl out 0 1k\n\
         cl out 0 100p\n\
         .va \"{model}\" va_initial_gain\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    for (analysis, error) in [
        (
            "native shooting",
            engine
                .run_pss_with_continuation_state_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
                .map(|_| ())
                .expect_err("native shooting does not carry the gain"),
        ),
        (
            "harmonic balance",
            engine
                .run_hb(
                    &netlist,
                    crate::analysis::HbConfig::new(1.0e6).with_harmonics(4),
                )
                .map(|_| ())
                .expect_err("harmonic balance does not carry the gain"),
        ),
    ] {
        let text = error.to_string();
        assert!(
            text.contains("analysis.periodic.veriloga")
                && text.contains("'XG'")
                && text.contains("`gain` is written by an event"),
            "{analysis}: {text}"
        );
    }
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
        .expect("event shooting carries the gain");
    assert!(point.is_event_orbit(), "the gain is event-shot");
    let peak = waveform(&point.analysis().result, "out")
        .iter()
        .fold(0.0_f64, |peak, value| peak.max(value.abs()));
    let omega_rc = 2.0 * std::f64::consts::PI * 1.0e6 * 1.0e3 * 100.0e-12;
    let expected = 2.0e-3 * 1.0e3 / (1.0 + omega_rc * omega_rc).sqrt();
    eprintln!("initial-step gain: peak {peak} against {expected}");
    assert!(
        (peak - expected).abs() <= 1.0e-3 * expected,
        "peak {peak} against {expected}"
    );
}

/// One-sided PNOISE of `point` at v(out): sampled at `phase` degrees, or the
/// time-averaged sideband zero.
fn event_pnoise(
    netlist: &Netlist,
    point: &crate::engine::PssOperatingPoint,
    offsets: &[Value],
    phase: Option<Value>,
) -> crate::engine::PnoiseAnalysisResult {
    let sampling =
        phase.map(|phase_degrees| crate::engine::PeriodicNoiseSampling::Phase { phase_degrees });
    let request = crate::engine::PeriodicNoiseRequest {
        sampling: sampling.as_ref(),
        offsets,
        output_node: "out",
        output_ref: None,
        input_source: None,
        max_sideband: 7,
        sidebands: crate::engine::PeriodicNoiseSidebands::default(),
    };
    Engine::new(SimulationConfig::default())
        .run_pnoise_from_pss_request_with_abort(netlist, &request, point, &NoAbort)
        .expect("PNOISE runs on the event orbit")
}

const BOLTZMANN_KELVIN: Value = 1.380_649e-23 * 300.15;

/// An RC whose resistor is a switch held closed is a time-invariant filter
/// the event carrier takes: its output is an Ornstein-Uhlenbeck process of
/// variance kT/C. Time-averaged, its noise is 4kT·R/(1 + (ωτ)²); sampled
/// once a period, the samples are AR(1) with pole e^{−T/τ}, whose one-sided
/// spectrum on [0, f₀/2] is 2T·(kT/C)·(1 − a²)/|1 − a·e^{−jωT}|². Every
/// sideband folds into the sampled spectrum; a window of them would not.
#[test]
fn event_pnoise_of_a_held_switch_rc_is_the_ornstein_uhlenbeck_closed_form() {
    let netlist = Netlist::parse(
        "held switch rc\n\
         vdrv in 0 dc 0 sin(0 0.1 1meg)\n\
         vctl ctl 0 dc 1.8\n\
         s1 in out ctl 0 swc\n\
         c1 out 0 1n\n\
         .model swc sw vt=0.9 vh=0.1 ron=1k roff=1e12\n\
         .end\n",
    )
    .expect("the deck parses");
    let point = Engine::new(SimulationConfig::default())
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort)
        .expect("event shooting solves the held RC");
    assert!(point.is_event_orbit(), "a switch circuit is event-shot");
    let (period, tau): (Value, Value) = (1.0e-6, 1.0e3 * 1.0e-9);
    let offsets = [1.0e3, 1.0e5, 4.0e5];
    let averaged = event_pnoise(&netlist, &point, &offsets, None);
    let sampled = event_pnoise(&netlist, &point, &offsets, Some(90.0));
    let pole = (-period / tau).exp();
    for (index, &frequency) in offsets.iter().enumerate() {
        let omega = 2.0 * std::f64::consts::PI * frequency;
        let expected_averaged = 4.0 * BOLTZMANN_KELVIN * 1.0e3 / (1.0 + (omega * tau).powi(2));
        let expected_sampled = 2.0 * period * BOLTZMANN_KELVIN / 1.0e-9 * (1.0 - pole * pole)
            / (Complex64::new(1.0, 0.0) - Complex64::from_polar(pole, -omega * period)).norm_sqr();
        let (a, s) = (averaged.output_noise[index], sampled.output_noise[index]);
        eprintln!(
            "held RC at {frequency:e} Hz: averaged {a:e} against {expected_averaged:e}, \
             sampled {s:e} against {expected_sampled:e}"
        );
        assert!(
            (a / expected_averaged - 1.0).abs() <= 1.0e-2,
            "averaged {a:e}"
        );
        assert!(
            (s / expected_sampled - 1.0).abs() <= 1.0e-2,
            "sampled {s:e}"
        );
    }
    assert_eq!(
        averaged.contributors.len(),
        1,
        "{:?}",
        averaged.contributors
    );
}

/// The C11 analog-clock twin's sampled output noise is the kT/C of its
/// charge sharing: the samples C2 holds after S2 opens are AR(1),
/// y_k = 0.9·y_{k−1} + e_k, with innovation variance 0.19·kT/C2, so the
/// one-sided spectrum of the sample sequence is
/// S_y(f) = 2T·0.19·(kT/C2)/|1 − 0.9·e^{−j2πfT}|². The
/// sample is read at 990 ns, in the hold; 0.556 ns before S2 opens the output
/// still carries the C1–C2 differential noise of the closed switch, which
/// adds a nearly white part the AR(1) form leaves out.
#[test]
fn event_pnoise_samples_the_switched_capacitor_kt_over_c() {
    let netlist = Netlist::parse(SC_PULSE).expect("the filter parses");
    let point = Engine::new(SimulationConfig::default())
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1.0e6).with_tstab(10.0e-6),
            &NoAbort,
        )
        .expect("event shooting solves the switched-capacitor orbit");
    let offsets = [100.0, 1.0e4, 1.0e5, 4.99e5];
    let noise = event_pnoise(&netlist, &point, &offsets, Some(356.4));
    let period = 1.0e-6;
    for (index, &frequency) in offsets.iter().enumerate() {
        let omega = 2.0 * std::f64::consts::PI * frequency;
        let expected = 2.0 * period * 0.19 * BOLTZMANN_KELVIN
            / 9.0e-12
            / (Complex64::new(1.0, 0.0) - Complex64::from_polar(0.9, -omega * period)).norm_sqr();
        let actual = noise.output_noise[index];
        eprintln!("C11 sampled at {frequency:e} Hz: {actual:e} against {expected:e}");
        assert!(
            (actual / expected - 1.0).abs() <= 0.01,
            "{actual:e} against {expected:e}"
        );
    }
    let evidence = noise.sampling.as_ref().expect("sampling evidence");
    assert!((evidence.output.phase_degrees - 356.4).abs() < 1.0e-9);
}

/// `Φ` by the variational recurrence and by forward differences at the
/// stabilized boundary of `deck`.
struct Monodromies {
    exact: Vec<Vec<Value>>,
    differenced: Vec<Vec<Value>>,
    /// `Φ` by central differences of the map, when asked for: their error is
    /// second order in the step where the forward differences' is first.
    central: Option<Vec<Vec<Value>>>,
    /// The seconds the recurrence and the differences took.
    seconds: (f64, f64),
    /// Where the period starts.
    start: Value,
    names: Vec<String>,
}

impl Monodromies {
    fn coordinate(&self, name: &str) -> usize {
        self.names
            .iter()
            .position(|candidate| {
                candidate
                    .trim_start_matches(super::EVENT_BASIS_PREFIX)
                    .eq_ignore_ascii_case(name)
            })
            .unwrap_or_else(|| panic!("{name} is a coordinate of {:?}", self.names))
    }
}

fn both_monodromies(deck: &str, config: &PssConfig) -> Monodromies {
    monodromies(deck, config, None)
}

/// [`both_monodromies`], with central differences of relative step
/// `central` beside them when given.
fn monodromies(deck: &str, config: &PssConfig, central: Option<Value>) -> Monodromies {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let prepared = engine
        .prepare_event_orbit(&netlist, None, config, None, &NoAbort)
        .expect("the orbit stabilizes");
    let orbit = prepared.orbit(&netlist, config, &NoAbort);
    let (image, mesh) = (&prepared.image, &prepared.mesh);
    let z = prepared
        .basis
        .read(image)
        .expect("the boundary's coordinates");
    let base = orbit.traverse(image, &z, mesh).expect("one period");
    eprintln!("{} events over the period", base.events.len());
    let clock = std::time::Instant::now();
    let exact = orbit
        .variational_jacobian(image, &z, mesh)
        .expect("the recurrence runs")
        .expect("the recurrence is exact here");
    let exact_seconds = clock.elapsed().as_secs_f64();
    let clock = std::time::Instant::now();
    let differenced = orbit
        .finite_difference_jacobian(image, &z, &base, mesh)
        .expect("the differences run");
    let central = central.map(|relative| {
        let n = z.len();
        let mut phi = vec![vec![0.0; n]; n];
        for column in 0..n {
            let step = relative * prepared.basis.unit(column).scale(z[column]);
            let end = |sign: Value| {
                let mut trial = z.clone();
                trial[column] += sign * step;
                let traversal = orbit.traverse(image, &trial, mesh).expect("one period");
                assert_eq!(
                    super::newton::signature(&traversal.events),
                    super::newton::signature(&base.events)
                );
                traversal.z_end
            };
            let (after, before) = (end(1.0), end(-1.0));
            for (row, values) in phi.iter_mut().enumerate() {
                values[column] = (after[row] - before[row]) / (2.0 * step);
            }
        }
        phi
    });
    Monodromies {
        exact,
        differenced,
        central,
        seconds: (exact_seconds, clock.elapsed().as_secs_f64()),
        start: image.time,
        names: prepared.basis.names().to_vec(),
    }
}

/// The largest entry difference of two square matrices, relative to the
/// largest entry of the second.
fn worst_relative(exact: &[Vec<Value>], differenced: &[Vec<Value>]) -> (Value, (usize, usize)) {
    let scale = differenced
        .iter()
        .flatten()
        .fold(0.0_f64, |m, v| m.max(v.abs()));
    let mut worst = (0.0, (0, 0));
    for (row, (a, b)) in exact.iter().zip(differenced).enumerate() {
        for (col, (x, y)) in a.iter().zip(b).enumerate() {
            let miss = (x - y).abs() / scale;
            if miss > worst.0 {
                worst = (miss, (row, col));
            }
        }
    }
    worst
}

/// The dominant multiplier of a monodromy.
fn dominant_multiplier(phi: &[Vec<Value>]) -> Complex64 {
    let (multipliers, _) = crate::analysis::pss::ShootingNewtonSolver::new(1.0e-9)
        .compute_floquet_spectrum_with_abort(phi, &NoAbort)
        .expect("the spectrum qualifies");
    multipliers
        .into_iter()
        .max_by(|a, b| a.norm().total_cmp(&b.norm()))
        .expect("a multiplier")
}

/// The C11 analog-clock twin's period map is affine with the switches held
/// to their clocks, so its forward differences are exact up to the
/// traversals' Newton tolerance, and the recurrence meets them there. Both
/// carry the charge-sharing multiplier C2/(C1 + C2) = 0.9 of the held
/// sample; C1's own memory decays through Ron·C1 = 1 ns over the 459 ns S1
/// is closed.
#[test]
fn the_recurrence_meets_the_differenced_period_map_of_the_switched_capacitor_twin() {
    let config = PssConfig::new(1.0e6).with_tstab(10.0e-6);
    let Monodromies {
        exact,
        differenced,
        seconds: (fast, slow),
        ..
    } = both_monodromies(SC_PULSE, &config);
    let (worst, at) = worst_relative(&exact, &differenced);
    let (exact_mu, differenced_mu) = (
        dominant_multiplier(&exact),
        dominant_multiplier(&differenced),
    );
    eprintln!(
        "C11 pulse: worst |exact - FD| = {worst:e} of the largest entry at {at:?}; \
         multipliers {exact_mu} and {differenced_mu}; {fast:.3} s against {slow:.3} s"
    );
    assert!(worst <= 1.0e-6, "worst relative miss {worst:e} at {at:?}");
    assert!(
        (exact_mu - Complex64::new(0.9, 0.0)).norm() <= 1.0e-6,
        "{exact_mu}"
    );
}

/// A smooth orbit: a diode with junction charge (CJO = 2 pF) charging an
/// RC through 2 kΩ, whose junction conductance changes over the period by
/// eleven decades. A segment evaluates the junction's charge at the iterate
/// it starts from, as it does a capacitor's, so the recurrence is the
/// derivative of the map. Forward differences carry the map's curvature over
/// their 1e-4 step, 4.2e-6 of the largest entry; central differences, whose
/// error is second order in their 1e-5 step, meet the recurrence to 1e-10.
#[test]
fn the_recurrence_meets_the_differenced_period_map_of_a_diode_with_junction_charge() {
    let deck = "diode rc
vin in 0 sin(0 2 1meg)
r0 in d 2k
d1 d out dmod
r1 out 0 10k
c1 out 0 1n
.model dmod d is=1e-14 cjo=2p
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let prepared = engine
        .prepare_event_orbit(&netlist, None, &config, None, &NoAbort)
        .expect("the orbit stabilizes");
    assert!(
        prepared
            .orbit(&netlist, &config, &NoAbort)
            .admits_variational(&prepared.image),
        "a junction charge is taken from the segment's start"
    );
    let monodromies = monodromies(deck, &config, Some(1.0e-5));
    let central = monodromies.central.as_deref().expect("central differences");
    let (forward, at) = worst_relative(&monodromies.exact, &monodromies.differenced);
    let (second_order, central_at) = worst_relative(&monodromies.exact, central);
    let (exact_mu, differenced_mu) = (
        dominant_multiplier(&monodromies.exact),
        dominant_multiplier(&monodromies.differenced),
    );
    eprintln!(
        "diode RC with junction charge: |exact - forward| = {forward:e} at {at:?}, \
         |exact - central| = {second_order:e} at {central_at:?}; multipliers {exact_mu} and \
         {differenced_mu}"
    );
    assert!(
        forward <= 1.0e-5,
        "worst relative miss {forward:e} at {at:?}"
    );
    assert!(
        second_order <= 1.0e-8,
        "worst relative miss {second_order:e} at {central_at:?}"
    );
    assert!((exact_mu - differenced_mu).norm() <= 1.0e-5 * differenced_mu.norm());
}

/// A BSIM4 common-source stage with the model's default gate and junction
/// charges, its sine on the gate and 100 pF on the drain: the segment
/// re-takes every one of the model's charges from the iterate it starts
/// from, so the recurrence covers it. The forward differences miss by 1.1e-4
/// of the largest entry, their own error; central differences meet the
/// recurrence to 1.5e-9.
#[test]
fn the_recurrence_meets_the_differenced_period_map_of_a_bsim4_stage_with_gate_charge() {
    let deck = "bsim4 stage
vdd vdd 0 dc 1.2
vin g 0 sin(0.7 0.3 1meg)
m1 d g 0 0 nmod w=1u l=0.1u
rd vdd d 5k
cl d 0 100p
.model nmod nmos level=54
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let monodromies = monodromies(deck, &config, Some(1.0e-5));
    let central = monodromies.central.as_deref().expect("central differences");
    let (forward, at) = worst_relative(&monodromies.exact, &monodromies.differenced);
    let (second_order, central_at) = worst_relative(&monodromies.exact, central);
    eprintln!(
        "BSIM4 stage: |exact - forward| = {forward:e} at {at:?}, |exact - central| = \
         {second_order:e} at {central_at:?}"
    );
    assert!(
        forward <= 1.0e-3,
        "worst relative miss {forward:e} at {at:?}"
    );
    assert!(
        second_order <= 1.0e-7,
        "worst relative miss {second_order:e} at {central_at:?}"
    );
}

/// A switch a state controls: the RC at `c` (R1·C1 = 300 ns) sets when S1
/// connects the RC at `x` to its 1 V supply through Ron, so both instants
/// move with v(c), and each saltation carries its move into x's charge.
///
/// The closed form, on the sine's steady state `v(c) = A·sin(ωt − φ)` with
/// `A = 1/√(1 + (ωR1C1)²)` and `φ = atan(ωR1C1)`: S1 closes where v(c) rises
/// through vt + vh = 0.25 and opens where it falls through vt − vh = 0.15.
/// Closed, x relaxes to `V∞ = R2/(Ron + R2)` through `τ_on = (Ron‖R2)·C2`;
/// open, to `R2/(Roff + R2)` through `τ_off = (Roff‖R2)·C2`. Then
///
/// - `Φ_cc = exp(−T/R1C1)`,
/// - `Φ_xx = exp(−T_on/τ_on − T_off/τ_off)`,
/// - `Φ_xc = Σ (ẋ_old − ẋ_new)(τ_i)·δτ_i/δc₀·exp(−∫ after τ_i)`, with
///   `δτ_i = −exp(−(τ_i − t₀)/R1C1)/v̇(c)(τ_i)` per unit of v(c) at `t₀`.
///
/// The recurrence is the discrete map's derivative: it meets the map's
/// central differences to 1e-5, and both meet the closed form to the
/// trapezoidal steps' error. The map's differences once carried half of each
/// jump (0.050359 against 0.068703): the step that dated a crossing ended
/// just past it, and its trapezoid averaged the open and closed slopes over
/// it. It now ends short of the crossing, and the step after it restarts at
/// order one over a tenth of the mesh's step. An order-one step run on to the
/// mesh's target instead moved its error with the crossing, and `Φ_cc`, which
/// no switch touches, fell 0.4% short (3.553614e-2).
#[test]
fn the_recurrence_carries_a_state_controlled_switchs_saltation() {
    let deck = "sampler
vin in 0 sin(0 1 1meg)
r1 in c 1k
c1 c 0 300p
vb b 0 dc 1
s1 b x c 0 swm
r2 x 0 2k
c2 x 0 300p
.model swm sw vt=0.2 vh=0.05 ron=1k roff=1e9
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let monodromies = monodromies(deck, &config, Some(1.0e-6));
    let (c, x) = (
        monodromies.coordinate("V(c)"),
        monodromies.coordinate("V(x)"),
    );
    let (period, omega) = (1.0e-6, 2.0 * std::f64::consts::PI * 1.0e6);
    let (r1c1, ron, r2, roff, c2) = (300.0e-9, 1.0e3, 2.0e3, 1.0e9, 300.0e-12);
    let amplitude = 1.0 / (1.0 + (omega * r1c1).powi(2)).sqrt();
    let lag = (omega * r1c1).atan();
    let start = monodromies.start;
    let after_start = |phase: Value| {
        let mut instant = (phase + lag) / omega;
        while instant < start {
            instant += period;
        }
        instant
    };
    let closes = after_start((0.25 / amplitude).asin());
    let opens = after_start(std::f64::consts::PI - (0.15 / amplitude).asin());
    assert!(closes < opens && opens < start + period);
    let rate = |instant: Value| amplitude * omega * (omega * instant - lag).cos();
    let (tau_on, settle_on) = (ron * r2 / (ron + r2) * c2, r2 / (ron + r2));
    let (tau_off, settle_off) = (roff * r2 / (roff + r2) * c2, r2 / (roff + r2));
    let (decay_on, decay_off) = (
        (-(opens - closes) / tau_on).exp(),
        (-(start + period - opens) / tau_off).exp(),
    );
    let wrapped_off = (-(period - (opens - closes)) / tau_off).exp();
    // v(x) where S1 closes, from v(x) = settle + (v - settle)·decay around
    // the period.
    let at_close = (settle_off + (settle_on - settle_on * decay_on - settle_off) * wrapped_off)
        / (1.0 - decay_on * wrapped_off);
    let at_open = settle_on + (at_close - settle_on) * decay_on;
    let delay = |instant: Value| -(-(instant - start) / r1c1).exp() / rate(instant);
    let closing = ((settle_off - at_close) / tau_off - (settle_on - at_close) / tau_on)
        * delay(closes)
        * decay_on
        * decay_off;
    let opening = ((settle_on - at_open) / tau_on - (settle_off - at_open) / tau_off)
        * delay(opens)
        * decay_off;
    let expected = [
        ((c, c), (-period / r1c1).exp()),
        ((x, x), decay_on * wrapped_off),
        ((x, c), closing + opening),
    ];
    let central = monodromies.central.as_ref().expect("central differences");
    for ((row, col), closed_form) in expected {
        let (exact, differenced) = (monodromies.exact[row][col], central[row][col]);
        eprintln!(
            "sampler Φ[{}][{}]: recurrence {exact:.6e}, central differences {differenced:.6e}, \
             forward differences {:.6e}, closed form {closed_form:.6e}",
            monodromies.names[row], monodromies.names[col], monodromies.differenced[row][col]
        );
        assert!(
            (exact - differenced).abs() <= 1.0e-5 * differenced.abs(),
            "Φ[{row}][{col}] = {exact:e} against the map's {differenced:e}"
        );
        for value in [exact, differenced] {
            assert!(
                (value - closed_form).abs() <= 1.0e-2 * closed_form.abs(),
                "Φ[{row}][{col}] = {value:e} against {closed_form:e}"
            );
        }
    }
}

/// The sampler's period map is smooth through its switch's crossings, and
/// its orbit certifies. The cut that dates a crossing inside a locked step
/// once accepted every candidate short of it, each a solver floor closer:
/// trapezoid steps that long turned roundoff into the derivative history, the
/// map moved by 5e-8 V between starts 1e-5 V apart, and the orbit's
/// linearization missed its directions by 1.76e-4. A bracketed search now
/// accepts only the end that dates the change, so the map's second
/// difference over those starts is the roundoff of its first.
#[test]
fn the_samplers_period_map_is_smooth_through_its_crossings() {
    let deck = "sampler
vin in 0 sin(0 1 1meg)
r1 in c 1k
c1 c 0 300p
vb b 0 dc 1
s1 b x c 0 swm
r2 x 0 2k
c2 x 0 300p
.model swm sw vt=0.2 vh=0.05 ron=1k roff=1e9
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
        .expect("the sampler's orbit certifies");
    assert!(point.is_event_orbit());
    let prepared = engine
        .prepare_event_orbit(&netlist, None, &config, None, &NoAbort)
        .expect("the orbit stabilizes");
    let orbit = prepared.orbit(&netlist, &config, &NoAbort);
    let (image, mesh) = (&prepared.image, &prepared.mesh);
    let z = prepared.basis.read(image).expect("coordinates");
    let names = prepared.basis.names();
    let coordinate = |name: &str| {
        names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} in {names:?}"))
    };
    let (c, x) = (coordinate("event:V(C)"), coordinate("event:V(X)"));
    let end = |offset: Value| {
        let mut trial = z.clone();
        trial[c] += offset;
        orbit
            .traverse(image, &trial, mesh)
            .expect("one period of the map")
            .z_end[x]
    };
    let base = end(0.0);
    let (first, second) = (end(1.0e-5) - base, end(2.0e-5) - base);
    assert!(
        (second - 2.0 * first).abs() <= 1.0e-4 * first.abs(),
        "the map's response to 1e-5 V and 2e-5 V at V(c): {first:e}, {second:e}"
    );
}

/// A switch across its own control node: `s1 a 0 a 0` closes where the RC
/// at `a` rises through VT + VH = 1.2 V and, closed, holds `a` inside its band
/// until it falls below VT − VH = 0.8 V. Its iterates once alternated between
/// the two states at the closing and no step converged, so no period of the
/// map traversed; one period now closes and opens the switch once each.
#[test]
fn a_switch_that_drives_its_own_control_traverses_its_period() {
    let deck = "switch across its own control
vin in 0 sin(0 2 1meg)
r1 in a 1k
c1 a 0 100p
s1 a 0 a 0 swm
.model swm sw vt=1 vh=0.2 ron=1k roff=1e9
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let prepared = engine
        .prepare_event_orbit(&netlist, None, &config, None, &NoAbort)
        .expect("the orbit stabilizes");
    let orbit = prepared.orbit(&netlist, &config, &NoAbort);
    let z = prepared.basis.read(&prepared.image).expect("coordinates");
    let traversal = orbit
        .traverse(&prepared.image, &z, &prepared.mesh)
        .expect("one period of the map");
    let changes: Vec<u64> = traversal.events.iter().map(|event| event.value).collect();
    assert_eq!(changes.len(), 2, "{:?}", traversal.events);
    assert_ne!(changes[0], changes[1]);
}

/// The self-controlled switch's orbit in closed form. Open, `C·v' = (vin −
/// v)/R1 − v/ROFF`; closed, `ROFF` is `RON`. Each state is linear, so from
/// `(t0, v0)` the node is the sine's particular response plus the decay of
/// its difference; the switch closes where `v` rises through 1.2 V and opens
/// where it falls through 0.8 V.
mod self_controlled_closed_form {
    use crate::Value;
    const R1: Value = 1.0e3;
    const C: Value = 100.0e-12;
    const OMEGA: Value = std::f64::consts::TAU * 1.0e6;
    const PERIOD: Value = 1.0e-6;

    fn decay_and_drive(closed: bool) -> (Value, Value) {
        let conductance = 1.0 / R1 + 1.0 / if closed { 1.0e3 } else { 1.0e9 };
        (conductance / C, 2.0 / (R1 * C))
    }

    fn particular(t: Value, closed: bool) -> Value {
        let (a, b) = decay_and_drive(closed);
        b / (a * a + OMEGA * OMEGA) * (a * (OMEGA * t).sin() - OMEGA * (OMEGA * t).cos())
    }

    fn at(t: Value, (t0, v0, closed): (Value, Value, bool)) -> Value {
        let (a, _) = decay_and_drive(closed);
        particular(t, closed) + (v0 - particular(t0, closed)) * (-a * (t - t0)).exp()
    }

    fn changes(v: Value, closed: bool) -> bool {
        if closed { v < 0.8 } else { v > 1.2 }
    }

    /// The segment `start` is in, carried to its first change before `until`.
    fn change_before(start: (Value, Value, bool), until: Value) -> Option<Value> {
        let step = PERIOD / 2000.0;
        let mut t = start.0;
        while t < until {
            let next = (t + step).min(until);
            if changes(at(next, start), start.2) {
                let (mut lo, mut hi) = (t, next);
                while lo < hi && (lo + hi) / 2.0 > lo && (lo + hi) / 2.0 < hi {
                    let mid = (lo + hi) / 2.0;
                    if changes(at(mid, start), start.2) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                return Some(hi);
            }
            t = next;
        }
        None
    }

    /// The settled orbit: its segments' starts over one period from `from`.
    pub(super) struct Orbit {
        segments: Vec<(Value, Value, bool)>,
    }

    impl Orbit {
        pub(super) fn settled(from: Value) -> Self {
            let mut segment = (0.0, 0.0, false);
            let mut segments = Vec::new();
            let end = from + PERIOD;
            loop {
                match change_before(segment, end) {
                    Some(t) => {
                        segment = (t, if segment.2 { 0.8 } else { 1.2 }, !segment.2);
                        if t >= from - PERIOD {
                            segments.push(segment);
                        }
                    }
                    None => return Self { segments },
                }
            }
        }

        pub(super) fn voltage(&self, t: Value) -> Value {
            let segment = self
                .segments
                .iter()
                .rev()
                .find(|segment| segment.0 <= t)
                .expect("a segment starts before every time of the period");
            at(t, *segment)
        }

        /// The period map's multiplier: each state's decay over its stay,
        /// times each change's saltation, the ratio of the slopes after and
        /// before it.
        pub(super) fn multiplier(&self) -> Value {
            let (closes, opens) = self.change_phases();
            let closed_for = (opens - closes).rem_euclid(PERIOD);
            let (open_decay, _) = decay_and_drive(false);
            let (closed_decay, _) = decay_and_drive(true);
            let slope = |t: Value, v: Value, closed: bool| {
                let (a, b) = decay_and_drive(closed);
                -a * v + b * (OMEGA * t).sin()
            };
            let saltation = |t: Value, v: Value, from_closed: bool| {
                slope(t, v, !from_closed) / slope(t, v, from_closed)
            };
            (-closed_decay * closed_for - open_decay * (PERIOD - closed_for)).exp()
                * saltation(closes, 1.2, false)
                * saltation(opens, 0.8, true)
        }

        /// The changes' phases in the period, closing first.
        pub(super) fn change_phases(&self) -> (Value, Value) {
            let phase = |closed: bool| {
                self.segments
                    .iter()
                    .rev()
                    .find(|segment| segment.2 == closed)
                    .map(|segment| segment.0.rem_euclid(PERIOD))
                    .expect("both changes")
            };
            (phase(true), phase(false))
        }
    }
}

/// A switch across its own control node certifies its orbit. The node falls
/// through 0 V at 577.6 ns of the period, where the mesh certificate once
/// held it to VNTOL alone: the bisections' differences fell 4x each (5.2e-5,
/// 1.3e-5, 3.3e-6 V, the trapezoid's own error) and missed by 1.085x after
/// three. Each node is now held to RELTOL of its swing, as native shooting's
/// certificate holds it; the orbit meets the closed form at every point by
/// 1.7e-5 V against that 1.70e-3 V, its changes fall within 0.11 ns of the
/// closed form's 214.6497 and 393.3229 ns, and its multiplier is 5.6240e-6
/// against the closed form's 5.6274e-6 (the decays, and saltations -0.59815
/// and -1.23712).
#[test]
fn a_switch_that_drives_its_own_control_certifies_its_orbit() {
    let deck = "switch across its own control
vin in 0 sin(0 2 1meg)
r1 in a 1k
c1 a 0 100p
s1 a 0 a 0 swm
.model swm sw vt=1 vh=0.2 ron=1k roff=1e9
.end
";
    let config = PssConfig::new(1.0e6).with_tstab(5.0e-6);
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let event = engine
        .run_event_pss(&netlist, &config, None, &NoAbort)
        .expect("the orbit certifies");
    let result = &event.analysis.result;
    // The published orbit's time is absolute time modulo the period.
    let exact = self_controlled_closed_form::Orbit::settled(20.0e-6);
    let swing = waveform(result, "a")
        .iter()
        .fold(0.0, |peak: Value, value| peak.max(value.abs()));
    let tolerance = 1.0e-3 * swing + 1.0e-6;
    let worst = result
        .time
        .iter()
        .zip(waveform(result, "a"))
        .map(|(&t, &v)| (v - exact.voltage(20.0e-6 + t)).abs())
        .fold(0.0, Value::max);
    assert!(worst <= tolerance, "|V(a) - closed form| = {worst:e} V");
    let (closes, opens) = exact.change_phases();
    for phase in [closes, opens] {
        assert!(
            event
                .certification
                .events
                .iter()
                .any(|(_, published)| (published - phase).abs() <= 1.0e-9),
            "a change at {phase:e} s in {:?}",
            event.certification.events
        );
    }
    let multiplier = exact.multiplier();
    let dominant = result
        .floquet_multipliers
        .iter()
        .map(|value| value.norm())
        .fold(0.0, Value::max);
    assert!(
        (dominant / multiplier - 1.0).abs() < 1.0e-2,
        "multiplier {dominant:e} against {multiplier:e}"
    );
}
