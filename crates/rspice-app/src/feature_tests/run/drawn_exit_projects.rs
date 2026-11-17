//! The exit projects drawn as an engineer draws them, through the whole
//! application: New project; each HDL module written into its cell's view in
//! the code editor and compiled, which gives the cell its symbol; every part
//! placed from the shelf and set in the inspector, every pin's net named by
//! a label or a ground symbol; the transient set on the Analyses page; and
//! Run. What the run gives is held to the corpus deck, run through the
//! manual-deck route.

use std::path::PathBuf;

use rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at_within;
use rspice_app_state::state::{AnalysisResultPayload, SimulationRun};
use rspice_results::run::SimulationRunLifecycle;

use crate::feature_tests::engineer::{Engineer, RUN_LIMIT, Scratch, kept, pin};

/// A corpus file, by its path under `tests/mixed-corpus`.
fn corpus(relative: &str) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(relative)
}

fn corpus_text(relative: &str) -> String {
    std::fs::read_to_string(corpus(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The value the run measured as `name`, a deck's `.MEASURE`.
fn measured(run: &SimulationRun, name: &str) -> f64 {
    let result = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.measurements)
        .find(|result| result.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no measurement {name}"));
    result
        .value
        .unwrap_or_else(|| panic!("{name} measured nothing: {result:?}"))
}

/// Run a corpus deck through the manual-deck route with its transient cut
/// to `stop`.
fn deck_route(relative: &str, step: &str, stop: &str) -> SimulationRun {
    let mut source = String::new();
    for line in corpus_text(relative).lines() {
        if line.trim_start().to_ascii_lowercase().starts_with(".tran ") {
            source.push_str(&format!(".tran {step} {stop}\n"));
        } else {
            source.push_str(line);
            source.push('\n');
        }
    }
    run_manual_batch_at_within(
        &source,
        &corpus(relative),
        SimulationRunLifecycle::Completed,
        RUN_LIMIT,
    )
}

/// The committed events of the digital net `name`: `(time, value code)`.
fn events(run: &SimulationRun, name: &str) -> Vec<(f64, u8)> {
    run.analyses
        .iter()
        .filter_map(|analysis| match analysis.result_payload.as_ref() {
            Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) => {
                Some(digital_traces)
            }
            _ => None,
        })
        .flatten()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no D({name}) in the run"))
        .points
        .iter()
        .map(|point| (point.time_s, point.value_code))
        .collect()
}

/// A digital net's value at `time`: its last event at or before it.
fn level_at(events: &[(f64, u8)], time: f64) -> u8 {
    events
        .iter()
        .rev()
        .find(|(at, _)| *at <= time)
        .map_or(u8::MAX, |(_, code)| *code)
}

/// An analog net's sample held at `time`.
fn held_at(run: &SimulationRun, name: &str, time: f64) -> f64 {
    let wave = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| {
            wave.name.eq_ignore_ascii_case(name)
                || wave.name.eq_ignore_ascii_case(&format!("V({name})"))
        })
        .unwrap_or_else(|| panic!("no {name} in the run"));
    let index = wave
        .x
        .iter()
        .rposition(|t| *t <= time)
        .expect("a sample at or before the time");
    wave.y[index]
}

/// An analog trace's samples, by its node.
fn trace<'a>(run: &'a SimulationRun, node: &str) -> (&'a [f64], &'a [f64]) {
    let wave = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| {
            wave.name.eq_ignore_ascii_case(node)
                || wave.name.eq_ignore_ascii_case(&format!("V({node})"))
        })
        .unwrap_or_else(|| {
            panic!(
                "no V({node}) in the run: {:?}",
                run.analyses
                    .iter()
                    .flat_map(|analysis| &analysis.waveforms)
                    .map(|wave| &wave.name)
                    .collect::<Vec<_>>()
            )
        });
    (&wave.x, &wave.y)
}

/// The trace's value at `time`, its samples joined by straight lines.
fn value_at(run: &SimulationRun, node: &str, time: f64) -> f64 {
    let (x, y) = trace(run, node);
    let index = x.partition_point(|t| *t < time).clamp(1, x.len() - 1);
    let (t0, t1) = (x[index - 1], x[index]);
    y[index - 1] + (y[index] - y[index - 1]) * (time - t0) / (t1 - t0)
}

/// The trace's mean over `window`, its samples joined by straight lines,
/// as the corpus measures a window mean.
fn mean(run: &SimulationRun, node: &str, window: (f64, f64)) -> f64 {
    let (x, y) = trace(run, node);
    let at = |time: f64| value_at(run, node, time);
    let mut points = vec![(window.0, at(window.0))];
    points.extend(
        x.iter()
            .zip(y)
            .filter(|(t, _)| **t > window.0 && **t < window.1)
            .map(|(t, v)| (*t, *v)),
    );
    points.push((window.1, at(window.1)));
    let area: f64 = points
        .windows(2)
        .map(|pair| (pair[1].0 - pair[0].0) * (pair[0].1 + pair[1].1) / 2.0)
        .sum();
    area / (window.1 - window.0)
}

/// C4, the 8-bit SAR ADC (`tests/mixed-corpus/c04-sar8`, variant `ports`):
/// the successive-approximation register as a Verilog view, the comparator
/// as a Verilog-A view, and the capacitor DAC, its switches and their
/// drives drawn on the top sheet.
mod c04 {
    use super::*;

    /// The codes of the first conversions of the corpus staircase.
    const CODES: [u32; 2] = [0, 1];

    /// The deck's `.MEASURE tconv` as the calculator writes it: samp's first
    /// fall to eoc's first rise.
    const TCONV: &str = r#"delay(D(samp), D(eoc), 0.5, "falling", 1, 0.5, "rising", 1)"#;

    /// tconv's tolerance in the corpus (`c04-sar8/tolerance.toml`): each
    /// edge held to the register's 1 ps precision, two roundings.
    const TCONV_ABS: f64 = 2e-12;

    /// The staircase's PWL table, as the corpus states it.
    fn staircase() -> String {
        corpus_text("c04-sar8/net/vin_codes.cir")
            .lines()
            .filter(|line| !line.starts_with('*'))
            .map(|line| {
                line.trim_start_matches("vin vin 0 pwl(")
                    .trim_start_matches('+')
                    .trim_end_matches(')')
                    .trim()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn codes(run: &SimulationRun) -> Vec<u32> {
        let eoc = events(run, "eoc");
        let bits: Vec<_> = (0..8)
            .rev()
            .map(|k| events(run, &format!("b{k}")))
            .collect();
        eoc.windows(2)
            .filter(|pair| pair[0].1 == 0 && pair[1].1 == 1)
            .map(|pair| {
                bits.iter().fold(0, |code, bit| {
                    code << 1 | u32::from(level_at(bit, pair[1].0) == 1)
                })
            })
            .collect()
    }

    /// The SAR drawn from New project to its results: the codes of the
    /// first two conversions, the code probe's level, and the conversion
    /// time measured over the result, kept through a save and a reopen, are
    /// the deck's.
    #[test]
    fn c4_drawn_through_the_application_converts_as_its_deck() {
        let mut engineer = Engineer::new_project("sar");
        engineer.module("sar8", "Verilog", &corpus_text("c04-sar8/net/sar8.v"));
        engineer.module(
            "cmp_va",
            "Verilog-A",
            &corpus_text("c04-sar8/net/cmp_va.va"),
        );
        engineer.top_sheet();
        let e = &mut engineer;

        // Supplies, references and stimulus along the top.
        let source = |e: &mut Engineer, x, name: &str, value: &str, net: &str| {
            let pins = e.place(
                "voltage source",
                "Voltage Source",
                (x, 100),
                &[("Instance", name), ("Value", value)],
            );
            e.connect(&pins, &[net, "0"]);
        };
        source(e, 100, "vdd", "1.8", "vdd");
        source(e, 250, "vref", "1.0", "vref");
        source(e, 400, "vcref", "0", "cref");
        let pulse = |e: &mut Engineer, x, name: &str, levels: (&str, &str), td, pw, per, net| {
            let pins = e.place(
                "pulse",
                "Pulse Voltage Source",
                (x, 100),
                &[
                    ("Instance", name),
                    ("Value", levels.0),
                    ("Pulsed value (V2), V", levels.1),
                    ("Delay time (TD), s", td),
                    ("Rise time (TR), s", "1n"),
                    ("Fall time (TF), s", "1n"),
                    ("Pulse width (PW), s", pw),
                    ("Period (PER), s", per),
                ],
            );
            e.connect(&pins, &[net, "0"]);
        };
        pulse(e, 550, "vclk", ("0", "1.8"), "0", "49n", "100n", "clk");
        pulse(e, 700, "vstart", ("1.8", "0"), "50n", "898n", "1u", "start");
        let input = e.place(
            "pwl",
            "PWL Voltage Source",
            (850, 100),
            &[
                ("Instance", "vin"),
                ("Value", &format!("PWL({})", staircase())),
            ],
        );
        e.connect(&input, &["vin", "0"]);

        // The sampling switch grounds the top plate while samp is high; the
        // bottom plates' input switches follow samp through an RC delay.
        let switch = |e: &mut Engineer, at, name: &str, nets: &[&str]| {
            let pins = e.place(
                "V-Switch",
                "V-Switch (S)",
                at,
                &[
                    ("Instance", name),
                    ("Threshold voltage, V", "0.9"),
                    ("Hysteresis voltage, V", "0.05"),
                    ("On resistance, Ω", "50"),
                    ("Off resistance, Ω", "1e12"),
                ],
            );
            e.connect(&pins, nets);
        };
        switch(e, (1000, 100), "stop", &["top", "0", "samp", "0"]);
        let rsd = e.place(
            "resistor",
            "Resistor",
            (1150, 100),
            &[("Instance", "rsd"), ("Value", "1k")],
        );
        e.connect(&rsd, &["samp", "samp_d"]);
        let csd = e.place(
            "capacitor",
            "Capacitor",
            (1300, 100),
            &[("Instance", "csd"), ("Value", "1p")],
        );
        e.connect(&csd, &["samp_d", "0"]);

        // The capacitor DAC, a column per bit: its capacitor off the top
        // plate, the bottom plate's three switches, and their two drives.
        let behavioural = |e: &mut Engineer, at, name: &str, expression: &str, net: &str| {
            let pins = e.place(
                "behavioral",
                "Behavioral Source",
                at,
                &[("Instance", name), ("Value", expression)],
            );
            e.connect(&pins, &[net, "0"]);
        };
        let columns = [
            "20f", "40f", "80f", "160f", "320f", "640f", "1.28p", "2.56p",
        ];
        for (k, value) in columns.into_iter().enumerate() {
            let x = 100 + 160 * k as i32;
            let (plate, bit) = (format!("bt{k}"), format!("b{k}"));
            let (reference, ground) = (format!("cr{k}"), format!("cg{k}"));
            let cap = e.place(
                "capacitor",
                "Capacitor",
                (x, 300),
                &[("Instance", &format!("ctop{k}")), ("Value", value)],
            );
            e.connect(&cap, &["top", &plate]);
            switch(
                e,
                (x, 460),
                &format!("sv{k}"),
                &[&plate, "vin", "samp_d", "0"],
            );
            switch(
                e,
                (x, 620),
                &format!("sr{k}"),
                &[&plate, "vref", &reference, "0"],
            );
            switch(e, (x, 780), &format!("sg{k}"), &[&plate, "0", &ground, "0"]);
            behavioural(
                e,
                (x, 940),
                &format!("bcr{k}"),
                &format!("V = v({bit})*(1.8-v(samp_d))/1.8"),
                &reference,
            );
            behavioural(
                e,
                (x, 1100),
                &format!("bcg{k}"),
                &format!("V = (1.8-v({bit}))*(1.8-v(samp_d))/1.8"),
                &ground,
            );
        }
        let x = 100 + 160 * 8;
        let dummy = e.place(
            "capacitor",
            "Capacitor",
            (x, 300),
            &[("Instance", "ctopd"), ("Value", "20f")],
        );
        e.connect(&dummy, &["top", "btd"]);
        switch(e, (x, 460), "svd", &["btd", "vin", "samp_d", "0"]);
        switch(e, (x, 780), "sgd", &["btd", "0", "cgd", "0"]);
        behavioural(e, (x, 1100), "bcgd", "V = 1.8-v(samp_d)", "cgd");

        // The code probe.
        let code = (0..8)
            .map(|index| format!("(v(b{})>0.9 ? {} : 0)", 7 - index, 128 >> index))
            .collect::<Vec<_>>()
            .join(" + ");
        behavioural(e, (1450, 100), "bcode", &format!("V = {code}"), "code");

        // The register and the comparator.
        let register = e.place("sar8", "sar8", (1800, 500), &[("Instance", "xsar")]);
        for (port, at) in register.clone() {
            e.label(at, &port.to_ascii_lowercase());
        }
        let comparator = e.place("cmp_va", "cmp_va", (1800, 1000), &[("Instance", "xcmp")]);
        e.label(pin(&comparator, "inp"), "top");
        e.label(pin(&comparator, "inn"), "cref");
        e.label(pin(&comparator, "out"), "cmp");
        e.label(pin(&comparator, "vdd"), "vdd");
        e.ground(pin(&comparator, "gnd"));

        e.transient("1n", "2u");
        let designed = e.run();
        let deck = deck_route("c04-sar8/deck/ports.tran.cir", "1n", "2u");

        assert_eq!(
            codes(&designed),
            codes(&deck),
            "the codes the deck converts"
        );
        assert_eq!(codes(&designed), CODES, "the staircase's codes");
        for (k, expected) in CODES.iter().enumerate() {
            let time = 0.97e-6 + 1e-6 * k as f64;
            let (app, reference) = (
                held_at(&designed, "code", time),
                held_at(&deck, "code", time),
            );
            assert!(
                (app - reference).abs() <= 0.01,
                "at {time:e}: {app} against {reference}"
            );
            assert!(
                (app - f64::from(*expected)).abs() <= 0.01,
                "at {time:e}: {app}"
            );
        }

        // Nine 100 ns clock periods, measured as the deck measures them.
        let tconv = e.measure(TCONV);
        let reference = measured(&deck, "tconv");
        assert!(
            (tconv - reference).abs() <= TCONV_ABS,
            "tconv {tconv:e} against the deck's {reference:e}"
        );
        assert!((tconv - 9e-7).abs() <= TCONV_ABS, "tconv {tconv:e}");
        assert_eq!(kept(&e.reopened(), TCONV), tconv, "tconv reopens as kept");
    }
}

/// C6, the digitally controlled buck converter (`tests/mixed-corpus/c06-buck`,
/// variant `vsup`): the controller as a Verilog-AMS view; the body diode's
/// card added from a model file on Models & sections; the power stage, the
/// gates and the sagging drive supply on the top sheet; and the gate drives'
/// converter supplied from the `vdrv` net on the Connect rules page, as the
/// `gate_drive` block the deck selects states it.
mod c06 {
    use super::*;

    /// vss (O6): the regulated output, the controller's target code over its
    /// 512 codes per volt, as V(vout)'s mean over 400-500 us, to 6 mV.
    const VSS: f64 = 922.0 / 512.0;
    const VSS_WINDOW: (f64, f64) = (4e-4, 5e-4);
    const VSS_ABS: f64 = 6e-3;

    /// vss as the calculator writes it.
    const VSS_MEAN: &str = "avg(V(vout), 400u, 500u)";

    /// The run stops where vss's window does: the cell's other quantities lie
    /// past it, and the 1.6 ms the corpus runs costs a debug build minutes.
    const STOP: &str = "500u";

    /// tau_r and tau_f's tolerance in the corpus: 0.3 ns.
    const EDGE_ABS: f64 = 3e-10;

    /// The stage's body-diode card, as the corpus states it.
    fn body_diode() -> String {
        corpus_text("c06-buck/net/stage.cir")
            .lines()
            .find(|line| line.starts_with(".model dbody"))
            .expect("the stage's body-diode card")
            .to_owned()
    }

    /// When the trace first crosses `level` inside `window`, rising or
    /// falling, its samples joined by straight lines.
    fn crossing(
        run: &SimulationRun,
        node: &str,
        level: f64,
        rising: bool,
        window: (f64, f64),
    ) -> f64 {
        let (x, y) = trace(run, node);
        (1..x.len())
            .filter(|index| x[*index] >= window.0 && x[*index - 1] <= window.1)
            .find_map(|index| {
                let (before, after) = (y[index - 1] - level, y[index] - level);
                let crosses = if rising {
                    before < 0.0 && after >= 0.0
                } else {
                    before > 0.0 && after <= 0.0
                };
                crosses
                    .then(|| x[index - 1] + (x[index] - x[index - 1]) * before / (before - after))
            })
            .unwrap_or_else(|| panic!("V({node}) never crosses {level} in {window:?}"))
    }

    /// The buck drawn from New project to its results: the regulated output
    /// is the corpus's vss, and the calculator's measure of it is kept
    /// through a save and a reopen; the gates' drive is the deck's.
    #[test]
    fn c6_drawn_through_the_application_regulates_and_drives_its_gates_as_its_deck() {
        // The deck route needs nothing drawn: it runs while the engineer
        // draws.
        let deck = std::thread::spawn(|| deck_route("c06-buck/deck/vsup.tran.cir", "50n", STOP));
        let models = Scratch::new("stage-models.lib", &body_diode());
        let mut engineer = Engineer::new_project("buck");
        engineer.module(
            "buck_ctrl",
            "Verilog-AMS",
            &corpus_text("c06-buck/net/buck_ctrl_vsup.vams"),
        );
        let e = &mut engineer;
        assert_eq!(
            e.add_model_file(&models.0),
            Ok("stage-models".to_owned()),
            "the body diode's card binds"
        );
        e.variable("lv", "22u");
        e.variable("cv", "4.7u");
        e.variable("vcc", "5");
        // The run's own connect rules: the d2a converter supplied from the
        // vdrv net and driving through 10 and 5 ohms.
        e.run_connect_rules(
            "d2a",
            &[("Supply", "vdrv"), ("R high", "10"), ("R low", "5")],
        );
        e.top_sheet();

        let part = |e: &mut Engineer, query, row, at, fields: &[(&str, &str)], nets: &[&str]| {
            let pins = e.place(query, row, at, fields);
            e.connect(&pins, nets);
        };
        let switch = |e: &mut Engineer, at, name: &str, model: [&str; 4], nets: &[&str]| {
            part(
                e,
                "V-Switch",
                "V-Switch (S)",
                at,
                &[
                    ("Instance", name),
                    ("Threshold voltage, V", model[0]),
                    ("Hysteresis voltage, V", model[1]),
                    ("On resistance, Ω", model[2]),
                    ("Off resistance, Ω", model[3]),
                ],
                nets,
            );
        };
        let two = |e: &mut Engineer, query, row, at, name: &str, value: &str, nets: &[&str]| {
            part(
                e,
                query,
                row,
                at,
                &[("Instance", name), ("Value", value)],
                nets,
            );
        };
        let pwl = |e: &mut Engineer, at, name: &str, table: &str, net: &str| {
            let value = format!("PWL({table})");
            two(
                e,
                "pwl",
                "PWL Voltage Source",
                at,
                name,
                &value,
                &[net, "0"],
            );
        };

        // The power stage along the top, left to right.
        let power = ["2.5", "0.1", "20m", "1e9"];
        two(
            e,
            "voltage source",
            "Voltage Source",
            (100, 100),
            "vin",
            "5",
            &["vin", "0"],
        );
        switch(e, (250, 100), "shs", power, &["vin", "sw", "ghs", "0"]);
        switch(e, (400, 100), "sls", power, &["sw", "0", "gls", "0"]);
        two(
            e,
            "diode",
            "Diode",
            (550, 100),
            "dls",
            "dbody",
            &["0", "sw"],
        );
        two(
            e,
            "inductor",
            "Inductor",
            (700, 100),
            "lbuck",
            "{lv}",
            &["sw", "lx"],
        );
        two(
            e,
            "resistor",
            "Resistor",
            (850, 100),
            "rdcr",
            "20m",
            &["lx", "vout"],
        );
        two(
            e,
            "capacitor",
            "Capacitor",
            (1000, 100),
            "cout",
            "{cv}",
            &["vout", "ce"],
        );
        two(
            e,
            "resistor",
            "Resistor",
            (1150, 100),
            "resr",
            "10m",
            &["ce", "0"],
        );
        two(
            e,
            "resistor",
            "Resistor",
            (1300, 100),
            "rload",
            "2",
            &["vout", "0"],
        );
        // The load step: a switch, driven by its own step source, adds a
        // second 2 ohm load at 500 us.
        let step = ["0.5", "0.1", "1m", "1e9"];
        switch(e, (100, 300), "sload", step, &["vout", "lo2", "gstep", "0"]);
        two(
            e,
            "resistor",
            "Resistor",
            (250, 300),
            "rload2",
            "2",
            &["lo2", "0"],
        );
        pwl(e, (400, 300), "vstep", "0 0 500u 0 500.01u 1", "gstep");

        // The gate drive below: the drives' supply, sagging from 5 V to
        // 3.5 V and recovering, the two 1 nF gates, and the controller.
        pwl(
            e,
            (550, 300),
            "vdrv",
            "0 5 1.0m 5 1.1m 3.5 1.3m 3.5 1.4m 5",
            "vdrv",
        );
        two(
            e,
            "capacitor",
            "Capacitor",
            (700, 300),
            "cghs",
            "1n",
            &["ghs", "0"],
        );
        two(
            e,
            "capacitor",
            "Capacitor",
            (850, 300),
            "cgls",
            "1n",
            &["gls", "0"],
        );
        let controller = e.place(
            "buck_ctrl",
            "buck_ctrl",
            (1100, 500),
            &[("Instance", "xctrl")],
        );
        e.label(pin(&controller, "fb"), "vout");
        e.label(pin(&controller, "hs"), "ghs");
        e.label(pin(&controller, "ls"), "gls");

        e.transient("50n", STOP);
        let designed = e.run();
        let deck = deck.join().expect("the deck route");

        // vss: the corpus's own window, value and tolerance, over the result
        // and as the calculator measures it.
        let app = mean(&designed, "vout", VSS_WINDOW);
        assert!(
            (app - VSS).abs() <= VSS_ABS,
            "V(vout) over {VSS_WINDOW:?}: {app}, the deck's {}",
            mean(&deck, "vout", VSS_WINDOW)
        );
        let measured = e.measure(VSS_MEAN);
        assert!(
            (measured - VSS).abs() <= VSS_ABS,
            "{VSS_MEAN} {measured}, the deck's {}",
            mean(&deck, "vout", VSS_WINDOW)
        );

        // In for tau_r, tau_f and vgate at 900 us, past this run: the settled
        // period at 400 us, where vdrv is the same 5 V, held to the deck
        // route over the same span at their tolerances. Each gate's edge
        // through 1 - 1/e of its drive and down to 1/e of it, and the
        // high-side gate 14 ticks into the period, where it holds V(vdrv).
        // Their sagged twins at 1.2 ms have no stand-in before 500 us.
        let period = 400e-6;
        let window = (period, period + 2e-6);
        for (node, level, rising) in [("ghs", 3.160_602_8, true), ("gls", 1.839_397_2, false)] {
            let (app, reference) = (
                crossing(&designed, node, level, rising, window),
                crossing(&deck, node, level, rising, window),
            );
            assert!(
                (app - reference).abs() <= EDGE_ABS,
                "V({node}) through {level}: {app:e}, the deck's {reference:e}"
            );
        }
        let held = period + 14.0 * 31.25e-9;
        let (app, reference) = (held_at(&designed, "ghs", held), held_at(&deck, "ghs", held));
        assert!(
            (app - reference).abs() <= 1e-3 * reference.abs(),
            "V(ghs) at {held:e}: {app}, the deck's {reference}"
        );

        assert_eq!(
            kept(&e.reopened(), VSS_MEAN),
            measured,
            "vss reopens as kept"
        );
    }
}

/// C12, the LDO testbench (`tests/mixed-corpus/c12-ldo-tb`, variant `full`):
/// the C5 regulator drawn as a schematic cell of its own, with its pins and
/// the symbol made from it; its BSIM4 cards added from their model file on
/// Models & sections; the trim register as a Verilog view and the
/// testbench as a Verilog-AMS view; the supply and static load on the top
/// sheet; and the testbench's verdict measured by a `.MEAS` card the plan
/// defines on Verify.
mod c12 {
    use egui::accesskit::Role;

    use super::*;

    /// When the testbench raises its verdict, after sixteen trim steps of
    /// 100 us: ok (O6, 1 ns) and okt (O6, 1 ps).
    const VERDICT: f64 = 1.6e-3;

    /// okt as the deck measures it.
    const OKT: &str = ".MEAS TRAN okt WHEN D(ok)=0.5 RISE=1";

    /// vk.15 as the calculator writes it: V(meas)'s mean over the corpus's
    /// window for code 15.
    const VK15: &str = "avg(V(meas), 1.5992m, 1.5998m)";

    /// vk.<k>'s window in the corpus: the testbench's reading of code `k`,
    /// held on meas from 92.5 us after the code's latch.
    fn window(k: usize) -> (f64, f64) {
        let start = k as f64 * 100e-6;
        (start + 99.2e-6, start + 99.8e-6)
    }

    /// vk.<k>'s value (O5-O1, 1 mV): V(vout) at the code-`k` operating
    /// point, the code held (ngspice-46, `ref/c<k>.op.O5-O1.out`).
    fn oracle(k: usize) -> f64 {
        let text = corpus_text(&format!("c12-ldo-tb/ref/c{k}.op.O5-O1.out"));
        let mut rows = text.lines().filter(|line| !line.starts_with('#'));
        let header: Vec<_> = rows.next().expect("a header").split_whitespace().collect();
        let values: Vec<_> = rows.next().expect("a row").split_whitespace().collect();
        let column = header
            .iter()
            .position(|name| *name == "v(meas)")
            .expect("v(meas)");
        values[column].parse().expect("a number")
    }

    /// When the net `name` goes to 1.
    fn rises(run: &SimulationRun, name: &str) -> Vec<f64> {
        events(run, name)
            .into_iter()
            .filter(|(_, code)| *code == 1)
            .map(|(time, _)| time)
            .collect()
    }

    /// The regulator, drawn in its own cell: the reference and bias, the
    /// error amplifier, the pass device and the trimmed feedback ladder of
    /// `c05-ldo-trim/net/ldo_body.inc`, each net named by a label; its pins,
    /// which name the nets they stand for; and its symbol.
    fn regulator(e: &mut Engineer) {
        e.schematic_cell("ldo");
        let part = |e: &mut Engineer, query, row, at, fields: &[(&str, &str)], nets: &[&str]| {
            let pins = e.place(query, row, at, fields);
            e.connect(&pins, nets);
        };
        let two = |e: &mut Engineer, query, row, at, name: &str, value: &str, nets: &[&str]| {
            part(
                e,
                query,
                row,
                at,
                &[("Instance", name), ("Value", value)],
                nets,
            );
        };
        let mos = |e: &mut Engineer, row, at, name: &str, size: [&str; 3], nets: &[&str]| {
            let model = if row == "NMOS" { "NBSIM4" } else { "PBSIM4" };
            let mut fields = vec![
                ("Instance", name),
                ("Value", model),
                ("Width, m", size[0]),
                ("Length, m", size[1]),
            ];
            if size[2] != "1" {
                fields.push(("Multiplier", size[2]));
            }
            part(e, row, row, at, &fields, nets);
        };
        let switch = |e: &mut Engineer, at, name: &str, nets: &[&str]| {
            part(
                e,
                "V-Switch",
                "V-Switch (S)",
                at,
                &[
                    ("Instance", name),
                    ("Threshold voltage, V", "0.9"),
                    ("Hysteresis voltage, V", "0.1"),
                    ("On resistance, Ω", "10"),
                    ("Off resistance, Ω", "1e12"),
                ],
                nets,
            );
        };
        let source = ("voltage source", "Voltage Source");
        let (resistor, capacitor) = (("resistor", "Resistor"), ("capacitor", "Capacitor"));

        // The reference, the bias and the error amplifier along the top.
        two(
            e,
            source.0,
            source.1,
            (100, 150),
            "vref",
            "0.5",
            &["vr0", "0"],
        );
        two(
            e,
            resistor.0,
            resistor.1,
            (250, 150),
            "rref",
            "100k",
            &["vr0", "vref"],
        );
        two(
            e,
            capacitor.0,
            capacitor.1,
            (400, 150),
            "cref",
            "10p",
            &["vref", "0"],
        );
        two(
            e,
            "current source",
            "Current Source",
            (550, 150),
            "ibias",
            "10u",
            &["vin", "nb"],
        );
        mos(
            e,
            "NMOS",
            (700, 150),
            "mb",
            ["2u", "1u", "1"],
            &["nb", "nb", "0", "0"],
        );
        mos(
            e,
            "NMOS",
            (850, 150),
            "mt",
            ["4u", "1u", "1"],
            &["tail", "nb", "0", "0"],
        );
        mos(
            e,
            "NMOS",
            (1000, 150),
            "m1",
            ["8u", "1u", "1"],
            &["d1", "fb", "tail", "0"],
        );
        mos(
            e,
            "NMOS",
            (1150, 150),
            "m2",
            ["8u", "1u", "1"],
            &["og", "vref", "tail", "0"],
        );
        mos(
            e,
            "PMOS",
            (1300, 150),
            "m3",
            ["4u", "1u", "1"],
            &["d1", "d1", "vin", "vin"],
        );
        mos(
            e,
            "PMOS",
            (1450, 150),
            "m4",
            ["4u", "1u", "1"],
            &["og", "d1", "vin", "vin"],
        );
        // The loop probe and the pass device, then the feedback ladder and
        // its trim switches, below.
        two(
            e,
            source.0,
            source.1,
            (100, 400),
            "vprobe",
            "0",
            &["og", "og2"],
        );
        mos(
            e,
            "PMOS",
            (250, 400),
            "mp",
            ["10u", "0.2u", "60"],
            &["vout", "og2", "vin", "vin"],
        );
        two(
            e,
            resistor.0,
            resistor.1,
            (400, 400),
            "rtop",
            "100k",
            &["vout", "fb"],
        );
        two(
            e,
            resistor.0,
            resistor.1,
            (550, 400),
            "rb0",
            "60k",
            &["fb", "l1"],
        );
        let ladder = [
            ("rt3", "8k", "l1", "l2"),
            ("rt2", "4k", "l2", "l3"),
            ("rt1", "2k", "l3", "l4"),
            ("rt0", "1k", "l4", "0"),
        ];
        for (index, (name, value, top, bottom)) in ladder.into_iter().enumerate() {
            let at = (700 + 150 * index as i32, 400);
            two(e, resistor.0, resistor.1, at, name, value, &[top, bottom]);
        }
        for (index, (name, top, bottom, bit)) in [
            ("s3", "l1", "l2", "tr3"),
            ("s2", "l2", "l3", "tr2"),
            ("s1", "l3", "l4", "tr1"),
            ("s0", "l4", "0", "tr0"),
        ]
        .into_iter()
        .enumerate()
        {
            let at = (100 + 150 * index as i32, 650);
            switch(e, at, name, &[top, bottom, bit, "0"]);
        }
        // Its pins: the supply, the output and the four trim bits.
        e.pins("vin", "Power", &[(100, 850)]);
        e.pins("vout", "Output", &[(250, 850)]);
        e.pins(
            "tr3 tr2 tr1 tr0",
            "Input",
            &[(400, 850), (550, 850), (700, 850), (850, 850)],
        );
        e.symbol_from_schematic("ldo");
    }

    /// The testbench drawn from New project to its verdict: each trim code's
    /// reading is its operating point's, the verdict rises once at 1600 us
    /// and the plan's `.MEAS` of it says so, and the last code's reading,
    /// measured over the result, is kept through a save and a reopen.
    #[test]
    fn c12_drawn_through_the_application_reads_its_codes_and_passes_as_its_deck() {
        // The deck route needs nothing drawn: it runs while the engineer
        // draws.
        let deck =
            std::thread::spawn(|| deck_route("c12-ldo-tb/deck/full.tran.cir", "0.5u", "1.65m"));
        let mut engineer = Engineer::new_project("ldotb");
        let e = &mut engineer;
        assert_eq!(
            e.add_model_file(&corpus("c05-ldo-trim/net/models-bsim4.cir")),
            Ok("models-bsim4".to_owned()),
            "the BSIM4 cards bind"
        );
        regulator(e);
        e.module(
            "spi_trim",
            "Verilog",
            &corpus_text("c05-ldo-trim/net/spi_trim.v"),
        );
        e.module(
            "tb_ldo",
            "Verilog-AMS",
            &corpus_text("c12-ldo-tb/net/tb_ldo_full.vams"),
        );
        e.variable("vcc", "1.8");
        e.top_sheet();

        // The supply and C5's static load.
        let part = |e: &mut Engineer, query, row, at, name: &str, value: &str, nets: &[&str]| {
            let pins = e.place(query, row, at, &[("Instance", name), ("Value", value)]);
            e.connect(&pins, nets);
        };
        part(
            e,
            "voltage source",
            "Voltage Source",
            (100, 150),
            "vin",
            "1.8",
            &["vin", "0"],
        );
        part(
            e,
            "resistor",
            "Resistor",
            (250, 150),
            "rload",
            "100",
            &["vout", "0"],
        );
        part(
            e,
            "capacitor",
            "Capacitor",
            (400, 150),
            "cout",
            "1u",
            &["vout", "co"],
        );
        part(
            e,
            "resistor",
            "Resistor",
            (550, 150),
            "resr",
            "0.1",
            &["co", "0"],
        );
        // The regulator, its trim register and the testbench, each pin on
        // the net of its own name.
        for (cell, instance, at) in [
            ("ldo", "xldo", (800, 500)),
            ("spi_trim", "xtrim", (1200, 500)),
            ("tb_ldo", "xtb", (1600, 500)),
        ] {
            let pins = e.place(cell, cell, at, &[("Instance", instance)]);
            for (port, at) in &pins {
                e.label(*at, port);
            }
        }

        e.plan_measurement("okt", OKT);
        e.setup_page("Save, streaming & retention policy");
        e.press("Plan output selection mode", Role::ComboBox);
        e.press("Save all", Role::ListBoxOption);
        e.transient("0.5u", "1.65m");
        let designed = e.run();
        let deck = deck.join().expect("the deck route");

        // vk.<k>: each code's reading against its operating point.
        for k in 0..16 {
            let app = mean(&designed, "meas", window(k));
            assert!(
                (app - oracle(k)).abs() <= 1e-3,
                "code {k}: V(meas) {app} against {}, the deck's {}",
                oracle(k),
                mean(&deck, "meas", window(k))
            );
        }
        // ok: the verdict rises once, at 1600 us; okt: the plan's .MEAS of it.
        let rose = rises(&designed, "ok");
        assert!(
            rose.len() == 1 && (rose[0] - VERDICT).abs() <= 1e-9,
            "ok rises at {rose:?}, the deck's at {:?}",
            rises(&deck, "xtb.ok__event_0")
        );
        let okt = measured(&designed, "okt");
        assert!(
            (okt - VERDICT).abs() <= 1e-12,
            "okt {okt:e}, the deck's {:e}",
            measured(&deck, "okt")
        );

        // vk.15 as the calculator measures it over the result, kept through
        // a save and a reopen.
        let measured = e.measure(VK15);
        assert!(
            (measured - oracle(15)).abs() <= 1e-3,
            "{VK15} {measured} against {}",
            oracle(15)
        );
        assert_eq!(kept(&e.reopened(), VK15), measured, "vk.15 reopens as kept");
    }
}

/// C2, the charge-pump PLL (`tests/mixed-corpus/c02-pll`, variant `mx`): the
/// pump and third-order loop filter drawn as a schematic cell of its own
/// over the BSIM3 cards of `pll-models.cir`, added from their model file on
/// Models & sections; the VCO as a Verilog-A view and the divide-by-20 and
/// phase-frequency detector as a Verilog view; and the reference, the
/// bridges, the XSPICE divide-by-2 and the plotting D/As on the top sheet.
mod c02 {
    use egui::accesskit::Role;

    use super::*;

    /// The run stops at 2 us: the loop's lock and the divider's measured
    /// period come after 20 us, which a debug build takes many minutes to
    /// reach.
    const STOP: &str = "2u";

    /// V(cont)'s mean over the run's last half microsecond, as the
    /// calculator writes it.
    const PULLING_IN: &str = "avg(V(cont), 1.5u, 2u)";

    /// The pump and loop filter of `net/loopf_mx.cir`, behind the pins u1n,
    /// d1 and vout. The deck's `.ic` starts the filter at 2.5 V; drawn,
    /// each of its three capacitors states that initial voltage itself.
    fn loop_filter(e: &mut Engineer) {
        e.schematic_cell("loopf_mx");
        let part = |e: &mut Engineer, query, row, at, fields: &[(&str, &str)], nets: &[&str]| {
            let pins = e.place(query, row, at, fields);
            e.connect(&pins, nets);
        };
        let two = |e: &mut Engineer, query, row, at, name: &str, value: &str, nets: &[&str]| {
            part(
                e,
                query,
                row,
                at,
                &[("Instance", name), ("Value", value)],
                nets,
            );
        };
        let capacitor = |e: &mut Engineer, at, name: &str, value: &str, nets: &[&str]| {
            let fields = [
                ("Instance", name),
                ("Value", value),
                ("Initial voltage, V", "2.5"),
            ];
            part(e, "capacitor", "Capacitor", at, &fields, nets);
        };
        let mos = |e: &mut Engineer, row, at, name: &str, size: [&str; 4], nets: &[&str]| {
            let model = if row == "NMOS" { "N1" } else { "P1" };
            let fields = [
                ("Instance", name),
                ("Value", model),
                ("Width, m", size[0]),
                ("Length, m", "0.35u"),
                ("Source area, m\u{b2}", size[1]),
                ("Drain area, m\u{b2}", size[1]),
                ("Source perimeter, m", size[2]),
                ("Drain perimeter, m", size[3]),
            ];
            part(e, row, row, at, &fields, nets);
        };
        let source = ("voltage source", "Voltage Source");
        // The pump's rails and its two devices onto dra.
        two(
            e,
            source.0,
            source.1,
            (100, 150),
            "vdd",
            "{vcc}",
            &["dd", "0"],
        );
        two(e, source.0, source.1, (250, 150), "vss", "0", &["ss", "0"]);
        mos(
            e,
            "NMOS",
            (400, 150),
            "mnd",
            ["12u", "24p", "28u", "28u"],
            &["dra", "d1", "ss", "ss"],
        );
        mos(
            e,
            "PMOS",
            (550, 150),
            "mpd",
            ["24u", "48p", "52u", "52u"],
            &["dra", "u1n", "dd", "dd"],
        );
        // The third-order filter.
        two(
            e,
            "resistor",
            "Resistor",
            (100, 400),
            "r1",
            "{r1v}",
            &["dra", "int1"],
        );
        two(
            e,
            "resistor",
            "Resistor",
            (250, 400),
            "r2",
            "200",
            &["int1", "c1"],
        );
        capacitor(e, (400, 400), "c1", "{c1v}", &["c1", "0"]);
        capacitor(e, (550, 400), "c2", "5n", &["int1", "0"]);
        two(
            e,
            "resistor",
            "Resistor",
            (700, 400),
            "r3",
            "50",
            &["int1", "vout"],
        );
        capacitor(e, (850, 400), "c3", "0.5n", &["vout", "0"]);
        e.pins("u1n d1", "Input", &[(100, 650), (250, 650)]);
        e.pins("vout", "Output", &[(400, 650)]);
        e.symbol_from_schematic("loopf_mx");
    }

    /// The PLL drawn from New project as it pulls in: its control voltage
    /// and every divider edge are the deck's, and the control voltage's
    /// mean over the last half microsecond, measured over the result, is
    /// kept through a save and a reopen.
    #[test]
    fn c2_drawn_through_the_application_pulls_in_as_its_deck() {
        // The deck route needs nothing drawn: it runs while the engineer
        // draws.
        let deck = std::thread::spawn(|| {
            deck_route("c02-pll/deck/mx.tran.cir", "0.1n", &format!("{STOP} uic"))
        });
        let mut engineer = Engineer::new_project("pll");
        let e = &mut engineer;
        assert_eq!(
            e.add_model_file(&corpus("c02-pll/net/pll-models.cir")),
            Ok("pll-models".to_owned()),
            "the BSIM3 cards bind"
        );
        loop_filter(e);
        e.module(
            "vco_pwl",
            "Verilog-A",
            &corpus_text("c02-pll/net/vco_pwl.va"),
        );
        e.module(
            "div20_pfd",
            "Verilog",
            &corpus_text("c02-pll/net/div20_pfd.v"),
        );
        e.variable("vcc", "3.3");
        e.variable("r1v", "300");
        e.variable("c1v", "10n");
        e.top_sheet();

        // The 10 MHz reference and the bridges onto the event side at
        // 0.5 V; the divide-by-2 from the shelf; and the D/As the deck plots
        // the divider and the reference through.
        let part = |e: &mut Engineer, query, row, at, fields: &[(&str, &str)], nets: &[&str]| {
            let pins = e.place(query, row, at, fields);
            e.connect(&pins, nets);
        };
        part(
            e,
            "pulse",
            "Pulse Voltage Source",
            (100, 150),
            &[
                ("Instance", "vref"),
                ("Value", "0"),
                ("Pulsed value (V2), V", "{vcc}"),
                ("Delay time (TD), s", "10n"),
                ("Rise time (TR), s", "1n"),
                ("Fall time (TF), s", "1n"),
                ("Pulse width (PW), s", "50n"),
                ("Period (PER), s", "100n"),
            ],
            &["ref", "0"],
        );
        let bridge = |e: &mut Engineer, at, name: &str, nets: &[&str]| {
            let fields = [
                ("Instance", name),
                ("Input low threshold", "0.5"),
                ("Input high threshold", "0.5"),
            ];
            part(e, "adc bridge", "ADC Bridge", at, &fields, nets);
        };
        bridge(e, (250, 150), "abridgeref", &["ref", "d_ref"]);
        bridge(e, (400, 150), "abridgevco", &["vco_out", "d_vco"]);
        part(
            e,
            "frequency divider",
            "Frequency Divider",
            (550, 150),
            &[
                ("Instance", "adiv2"),
                ("div_factor", "2"),
                ("high_cycles", "1"),
                ("i_count", "0"),
                ("rise_delay", "1e-10"),
                ("fall_delay", "1e-10"),
            ],
            &["d_vco", "d_div2"],
        );
        for (at, name, nets) in [
            ((700, 150), "abridgew1", ["d_divout", "s1"]),
            ((850, 150), "abridgew2", ["d_ref", "s2"]),
        ] {
            let fields = [
                ("Instance", name),
                ("Output Low", "0"),
                ("Output High", "1"),
                ("Output undefined", "0.5"),
                ("Input load, F", "5e-12"),
                ("Rise Time, s", "1e-10"),
                ("Fall Time, s", "1e-10"),
            ];
            part(e, "dac bridge", "DAC Bridge", at, &fields, &nets);
        }
        // The detector, the loop filter and the VCO, each port on its net.
        for (cell, instance, at, nets) in [
            (
                "div20_pfd",
                "xpfd",
                (300, 500),
                &[
                    ("clkin", "d_div2"),
                    ("ref", "d_ref"),
                    ("up_n", "up_n"),
                    ("dn", "dn"),
                    ("divo", "d_divout"),
                ][..],
            ),
            (
                "loopf_mx",
                "xlf",
                (800, 500),
                &[("u1n", "up_n"), ("d1", "dn"), ("vout", "cont")][..],
            ),
            (
                "vco_pwl",
                "xvco",
                (1200, 500),
                &[("cont", "cont"), ("out", "vco_out")][..],
            ),
        ] {
            let pins = e.place(cell, cell, at, &[("Instance", instance)]);
            assert_eq!(pins.len(), nets.len(), "{cell}'s ports: {pins:?}");
            for (port, net) in nets {
                e.label(pin(&pins, port), net);
            }
        }

        e.setup_page("Save, streaming & retention policy");
        e.press("Plan output selection mode", Role::ComboBox);
        e.press("Save all", Role::ListBoxOption);
        e.transient("0.1n", STOP);
        e.initial_conditions();
        let designed = e.run();
        let deck = deck.join().expect("the deck route");

        // In for vlock (O2, 10 mV), the locked control voltage's mean over
        // 20-25 us, past this run: the control voltage as the loop pulls in,
        // every half microsecond, held to the deck route over the same span
        // within vlock's 10 mV.
        for step in 1..=4 {
            let time = f64::from(step) * 0.5e-6;
            let (app, reference) = (
                value_at(&designed, "cont", time),
                value_at(&deck, "cont", time),
            );
            assert!(
                (app - reference).abs() <= 1e-2,
                "V(cont) at {time:e}: {app}, the deck's {reference}"
            );
        }
        // In for fdiv (O6, 51 ps), the divider's locked period over 49
        // periods after 20 us, past this run: every edge of D(d_divout),
        // held to the deck route's within fdiv's 51 ps.
        let (app, reference) = (events(&designed, "d_divout"), events(&deck, "d_divout"));
        assert!(
            app.len() == reference.len()
                && app
                    .iter()
                    .zip(&reference)
                    .all(|((time, code), (deck_time, deck_code))| {
                        code == deck_code && (time - deck_time).abs() <= 5.1e-11
                    }),
            "D(d_divout) {app:?}, the deck's {reference:?}"
        );

        // The control voltage's mean over the last half microsecond, as the
        // calculator measures it, kept through a save and a reopen.
        let measured = e.measure(PULLING_IN);
        let reference = mean(&deck, "cont", (1.5e-6, 2e-6));
        assert!(
            (measured - reference).abs() <= 1e-2,
            "{PULLING_IN} {measured}, the deck's {reference}"
        );
        assert_eq!(
            kept(&e.reopened(), PULLING_IN),
            measured,
            "the mean reopens as kept"
        );
    }
}
