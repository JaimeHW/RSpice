//! C6, the digitally controlled buck converter (`tests/mixed-corpus/c06-buck`,
//! variant `vsup`): the controller as a Verilog-AMS view; the power stage,
//! the gates and the sagging drive supply on the top sheet; the body diode's
//! card as a project model; and the gate drives' converter put on the `vdrv`
//! net on the Connect rules page, where the deck selects the `gate_drive`
//! block its source declares.

use super::*;

/// The project as an engineer draws it, its transient running to `stop`.
pub(super) fn project(stop: &str) -> RunTestHost {
    let mut drawing = Drawing::new("buck");
    let control = drawing.source_view(
        "buck_ctrl",
        ViewType::VerilogAms,
        "c06-buck/net/buck_ctrl_vsup.vams",
    );
    drawing.project_model("buck", "dbody", "D", &[("is", 1e-12), ("rs", 10e-3)]);

    // The power stage of net/stage.cir: ideal switches with a low-side body
    // diode, the inductor and its DCR, the output capacitor and its ESR, and
    // a 2 ohm load stepping to 1 ohm at 500 us.
    let source = ComponentType::VoltageSource;
    let pwl = ComponentType::VoltageSourcePwl;
    let switch = ComponentType::VSwitch;
    let resistor = ComponentType::Resistor;
    let power = "vt=2.5 vh=0.1 ron=20m roff=1e9";
    drawing.part(source, "vin", "5", "", &["vin", "0"]);
    drawing.part(switch, "shs", "", power, &["vin", "sw", "ghs", "0"]);
    drawing.part(switch, "sls", "", power, &["sw", "0", "gls", "0"]);
    drawing.part(ComponentType::Diode, "dls", "dbody", "", &["0", "sw"]);
    drawing.part(ComponentType::Inductor, "lbuck", "{lv}", "", &["sw", "lx"]);
    drawing.part(resistor, "rdcr", "20m", "", &["lx", "vout"]);
    drawing.part(
        ComponentType::Capacitor,
        "cout",
        "{cv}",
        "",
        &["vout", "ce"],
    );
    drawing.part(resistor, "resr", "10m", "", &["ce", "0"]);
    drawing.part(resistor, "rload", "2", "", &["vout", "0"]);
    let step = "vt=0.5 vh=0.1 ron=1m roff=1e9";
    drawing.part(switch, "sload", "", step, &["vout", "lo2", "gstep", "0"]);
    drawing.part(resistor, "rload2", "2", "", &["lo2", "0"]);
    drawing.part(pwl, "vstep", "0 0 500u 0 500.01u 1", "", &["gstep", "0"]);
    // The gate drives' supply, sagging from 5 V to 3.5 V and recovering,
    // and the two 1 nF gates.
    let sag = "0 5 1.0m 5 1.1m 3.5 1.3m 3.5 1.4m 5";
    drawing.part(pwl, "vdrv", sag, "", &["vdrv", "0"]);
    drawing.part(ComponentType::Capacitor, "cghs", "1n", "", &["ghs", "0"]);
    drawing.part(ComponentType::Capacitor, "cgls", "1n", "", &["gls", "0"]);
    drawing.ground();

    drawing.instance(
        &control,
        "xctrl",
        0,
        &[("fb", "vout"), ("hs", "ghs"), ("ls", "gls")],
    );

    drawing.variable("lv", "22u");
    drawing.variable("cv", "4.7u");
    drawing.variable("vcc", "5");
    // The Connect rules page: the run's own rules, the d2a converter
    // supplied from the vdrv net and driving through 10 and 5 ohms, as the
    // source's gate_drive block states.
    drawing.run_connect_rules(&[
        ("d2a", ConnectRuleColumn::Supply, "vdrv"),
        ("d2a", ConnectRuleColumn::DriveHigh, "10"),
        ("d2a", ConnectRuleColumn::DriveLow, "5"),
    ]);
    drawing.transient("50n", stop);
    drawing.finish()
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
        .unwrap_or_else(|| panic!("no V({node}) in the run"));
    (&wave.x, &wave.y)
}

/// The trace's mean over `window`, its samples joined by straight lines,
/// as the corpus measures a window mean.
fn mean(run: &SimulationRun, node: &str, window: (f64, f64)) -> f64 {
    let (x, y) = trace(run, node);
    let at = |time: f64| {
        let index = x.partition_point(|t| *t < time).clamp(1, x.len() - 1);
        let (t0, t1) = (x[index - 1], x[index]);
        y[index - 1] + (y[index] - y[index - 1]) * (time - t0) / (t1 - t0)
    };
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

/// When the trace first crosses `level` inside `window`, rising or falling,
/// its samples joined by straight lines.
fn crossing(run: &SimulationRun, node: &str, level: f64, rising: bool, window: (f64, f64)) -> f64 {
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
            crosses.then(|| x[index - 1] + (x[index] - x[index - 1]) * before / (before - after))
        })
        .unwrap_or_else(|| panic!("V({node}) never crosses {level} in {window:?}"))
}

/// The regulated output: the controller's target code over its 512 codes
/// per volt (the corpus's analytic oracle).
const VSS: f64 = 922.0 / 512.0;

/// Draw C6, save it, reopen it, run both, and hold the runs to each other
/// and to the corpus deck route to `stop`: the gate edges always, and the
/// regulated output and the sag's levels where the run reaches them.
fn exit_run(stop: f64) {
    let stop_text = format!("{}u", stop * 1e6);
    let built = project(&stop_text);
    let reopened = save_and_reopen(&built, "c06");
    let designed = design_route(built);
    let rerun = design_route(reopened);
    assert_eq!(
        digest(&designed),
        digest(&rerun),
        "the reopened project runs to the same result"
    );
    let deck = deck_route("c06-buck/deck/vsup.tran.cir", ("50n", &stop_text));

    // tau_r, tau_f (O6, 0.3 ns): each gate's edge through 1 - 1/e of its
    // drive and down to 1/e of it, in the first period after 40 us, at 5 V.
    let edges = [("ghs", 3.1606028, true), ("gls", 1.8393972, false)];
    let mut windows = vec![(40e-6, 42e-6)];
    if stop >= 1.6e-3 {
        windows.extend([(9e-4, 9.001e-4), (1.2e-3, 1.2001e-3)]);
    }
    for window in windows {
        let sagged = window.0 > 1.1e-3 && window.0 < 1.3e-3;
        for (node, level, rising) in edges {
            let level = if sagged { level * 3.5 / 5.0 } else { level };
            let (app, reference) = (
                crossing(&designed, node, level, rising, window),
                crossing(&deck, node, level, rising, window),
            );
            assert!(
                (app - reference).abs() <= 3e-10,
                "V({node}) through {level} in {window:?}: {app:e} against {reference:e}"
            );
            // The analytic edge: hs rises two 31.25 ns ticks into the period
            // and its gate takes rhi Cg = 10 ns; ls falls at the period's
            // start and its gate takes rlo Cg = 5 ns.
            let edge = if rising {
                window.0 + 2.0 * 31.25e-9 + 10e-9
            } else {
                window.0 + 5e-9
            };
            assert!(
                (app - edge).abs() <= 3e-10,
                "V({node}) through {level} in {window:?}: {app:e} against {edge:e}"
            );
        }
    }

    // vss, vss_step, vss_sag (O6, 6 mV): the regulated output before and
    // after the load step, and through the drive's sag.
    let mut windows = vec![];
    if stop >= 5e-4 {
        windows.push((4e-4, 5e-4));
    }
    if stop >= 1.6e-3 {
        windows.extend([(7.5e-4, 8e-4), (1.2e-3, 1.3e-3)]);
    }
    for window in windows {
        let (app, reference) = (mean(&designed, "vout", window), mean(&deck, "vout", window));
        assert!(
            (app - reference).abs() <= 6e-3,
            "V(vout) over {window:?}: {app} against {reference}"
        );
        assert!((app - VSS).abs() <= 6e-3, "V(vout) over {window:?}: {app}");
    }

    // vgate (O6, 0.1 %): the high-side gate settled on its drive, at 5 V,
    // 3.5 V through the sag and 5 V again.
    if stop >= 1.6e-3 {
        for (time, drive) in [(9.004375e-4, 5.0), (1.2004375e-3, 3.5), (1.5004375e-3, 5.0)] {
            let (app, reference) = (held_at(&designed, "ghs", time), held_at(&deck, "ghs", time));
            assert!(
                (app - reference).abs() <= 1e-3 * reference.abs(),
                "V(ghs) at {time:e}: {app} against {reference}"
            );
            assert!(
                (app - drive).abs() <= 1e-3 * drive,
                "V(ghs) at {time:e}: {app}"
            );
        }
    }
}

/// The gate drive's supply net, vdrv, stands only on its source, and the
/// d2a converters the Connect rules page supplies from it read it: the
/// design's check finds it connected, as it finds every other node.
#[test]
fn c6_drive_supply_the_connect_rules_read_is_connected() {
    let mut state = project("1u");
    state.sync_active_schematic_to_workspace();
    let checks = state.run_active_design_checks().expect("check the design");
    let floating = checks
        .violations()
        .iter()
        .filter(|finding| {
            finding.violation_type == rspice_design::drc::DrcViolationType::FloatingNode
        })
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>();
    assert!(floating.is_empty(), "{floating:?}");
}

/// C6 drawn in the app drives its gates from the sagging supply as the
/// corpus deck does, through the connect rules set on the Connect rules
/// page: each gate's edges, and the regulated output; and saved and
/// reopened, it runs to the same result. Half a millisecond reaches the
/// regulated output; the whole run through the sag is
/// [`c6_runs_through_the_drive_sag_as_its_deck`].
#[test]
fn c6_drawn_saved_and_reopened_drives_its_gates_as_its_deck() {
    exit_run(5e-4);
}

/// The whole 1.6 ms: the load step, the sag and the recovery.
#[test]
#[ignore = "the corpus's whole 1.6 ms run; run with --ignored for the exit evidence"]
fn c6_runs_through_the_drive_sag_as_its_deck() {
    exit_run(1.6e-3);
}
