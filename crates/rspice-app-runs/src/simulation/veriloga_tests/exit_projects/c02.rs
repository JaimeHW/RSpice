//! C2, the charge-pump PLL (`tests/mixed-corpus/c02-pll`, variant `mx`): the
//! VCO as a Verilog-A view, the divide-by-20 and phase-frequency detector as
//! a Verilog view, the XSPICE divide-by-2 placed from the shelf, the pump
//! and third-order loop filter as a schematic cell over the BSIM3 cards of
//! `pll-models.cir`, and the reference, the bridges and the plotting D/A on
//! the top sheet.

use super::*;
use crate::state::PortDirectionType;

/// The pump and loop filter of `net/loopf_mx.cir` as their own schematic
/// cell, behind the ports u1n, d1 and vout. The deck's `.ic` holds the
/// filter's three capacitors at 2.5 V from the start; drawn, each capacitor
/// states that initial voltage itself.
fn loop_filter(drawing: &mut Drawing) -> CellViewRef {
    drawing.port("u1n", PortDirectionType::InputAnalog);
    drawing.port("d1", PortDirectionType::InputAnalog);
    drawing.port("vout", PortDirectionType::OutputAnalog);
    let (resistor, capacitor) = (ComponentType::Resistor, ComponentType::Capacitor);
    drawing.part(
        ComponentType::VoltageSource,
        "vdd",
        "{vcc}",
        "",
        &["dd", "0"],
    );
    drawing.part(ComponentType::VoltageSource, "vss", "0", "", &["ss", "0"]);
    let n = "w=12u l=0.35u as=24p ad=24p ps=28u pd=28u";
    drawing.part(
        ComponentType::Nmos,
        "mnd",
        "N1",
        n,
        &["dra", "d1", "ss", "ss"],
    );
    let p = "w=24u l=0.35u as=48p ad=48p ps=52u pd=52u";
    drawing.part(
        ComponentType::Pmos,
        "mpd",
        "P1",
        p,
        &["dra", "u1n", "dd", "dd"],
    );
    drawing.part(resistor, "r1", "{r1v}", "", &["dra", "int1"]);
    drawing.part(resistor, "r2", "200", "", &["int1", "c1"]);
    drawing.part(capacitor, "c1", "{c1v}", "ic=2.5", &["c1", "0"]);
    drawing.part(capacitor, "c2", "5n", "ic=2.5", &["int1", "0"]);
    drawing.part(resistor, "r3", "50", "", &["int1", "vout"]);
    drawing.part(capacitor, "c3", "0.5n", "ic=2.5", &["vout", "0"]);
    drawing.ground();
    drawing.master("loopf_mx")
}

/// The project as an engineer draws it, its transient running to `stop`
/// from the initial conditions.
pub(super) fn project(stop: &str) -> RunTestHost {
    let mut drawing = Drawing::new("pll");
    drawing.model_library("c02-pll/net/pll-models.cir");
    let filter = loop_filter(&mut drawing);
    let vco = drawing.source_view("vco_pwl", ViewType::VerilogA, "c02-pll/net/vco_pwl.va");
    let pfd = drawing.source_view("div20_pfd", ViewType::Verilog, "c02-pll/net/div20_pfd.v");

    // The 10 MHz reference, and the bridges onto the event side at 0.5 V.
    let reference = "v1=0 v2=3.3 td=10n tr=1n tf=1n pw=50n per=100n";
    drawing.part(
        ComponentType::VoltageSourcePulse,
        "vref",
        "",
        reference,
        &["ref", "0"],
    );
    let adc = "in_low=0.5 in_high=0.5";
    let bridge = ComponentType::XspiceAdcBridge;
    drawing.part(bridge, "abridgeref", "", adc, &["ref", "d_ref"]);
    drawing.part(bridge, "abridgevco", "", adc, &["vco_out", "d_vco"]);
    // The D/A the deck plots the divider and the reference through.
    let dac = "out_low=0 out_high=1 out_undef=0.5 input_load=5e-12 t_rise=1e-10 t_fall=1e-10";
    let plot = ComponentType::XspiceDacBridge;
    drawing.part(plot, "abridgew1", "", dac, &["d_divout", "s1"]);
    drawing.part(plot, "abridgew2", "", dac, &["d_ref", "s2"]);
    drawing.ground();

    drawing.instance(&vco, "xvco", 0, &[("cont", "cont"), ("out", "vco_out")]);
    let divide = "div_factor=2 high_cycles=1 i_count=0 rise_delay=1e-10 fall_delay=1e-10";
    drawing.shelf_xspice(
        "rspice.xspice.d_fdiv",
        "adiv2",
        divide,
        1,
        &[("freq_in", "d_vco"), ("freq_out", "d_div2")],
    );
    drawing.instance(
        &pfd,
        "xpfd",
        2,
        &[
            ("clkin", "d_div2"),
            ("ref", "d_ref"),
            ("up_n", "up_n"),
            ("dn", "dn"),
            ("divo", "d_divout"),
        ],
    );
    drawing.instance(
        &filter,
        "xlf",
        3,
        &[("u1n", "up_n"), ("d1", "dn"), ("vout", "cont")],
    );

    drawing.variable("vcc", "3.3");
    drawing.variable("r1v", "300");
    drawing.variable("c1v", "10n");
    drawing.transient_from("0.1n", stop, true);
    let tdiv = ".MEAS TRAN tdiv TRIG D(d_divout) VAL=0.5 RISE=1 TD=20u \
                TARG D(d_divout) VAL=0.5 RISE=50 TD=20u";
    drawing.measure("tdiv", tdiv);
    drawing.measure("fdiv", ".MEAS TRAN fdiv PARAM='tdiv/49'");
    drawing.finish()
}

/// The locked control voltage: where the VCO's PWL table gives 400 MHz
/// (the corpus's analytic oracle).
const VLOCK: f64 = 1.943932;

/// Draw C2, save it, reopen it, run both, and hold the runs to each other
/// and to the corpus deck route to `stop`: the control voltage and the
/// divider's edges always, and the lock and its measurement where the run
/// reaches them.
fn exit_run(stop: f64) {
    let stop_text = format!("{}u", stop * 1e6);
    let built = project(&stop_text);
    let reopened = save_and_reopen(&built, "c02");
    let designed = design_route(built);
    let rerun = design_route(reopened);
    assert_eq!(
        digest(&designed),
        digest(&rerun),
        "the reopened project runs to the same result"
    );
    let deck = deck_route(
        "c02-pll/deck/mx.tran.cir",
        ("0.1n", &format!("{stop_text} uic")),
    );

    // The control voltage as the loop pulls in, every half microsecond:
    // within the corpus's 10 mV between the two simulators' locked means.
    let mut time = 0.5e-6;
    while time <= stop {
        let (app, reference) = (
            held_at(&designed, "cont", time),
            held_at(&deck, "cont", time),
        );
        assert!(
            (app - reference).abs() <= 1e-2,
            "V(cont) at {time:e}: {app} against {reference}"
        );
        time += 0.5e-6;
    }

    // The divider's edges (fdiv, 51 ps): each rise of D(d_divout) on the deck
    // route's.
    let (app, reference) = (events(&designed, "d_divout"), events(&deck, "d_divout"));
    assert_eq!(app.len(), reference.len(), "{app:?} against {reference:?}");
    for ((time, code), (deck_time, deck_code)) in app.iter().zip(&reference) {
        assert_eq!(code, deck_code, "{app:?} against {reference:?}");
        assert!(
            (time - deck_time).abs() <= 5.1e-11,
            "{app:?} against {reference:?}"
        );
    }

    if stop >= 25e-6 {
        // vlock (O6 15 mV, O2 10 mV): the locked control voltage's mean.
        let window = (20e-6, 25e-6);
        let mean = |run: &SimulationRun| {
            let samples = 500;
            (0..samples)
                .map(|k| {
                    let time = window.0 + (window.1 - window.0) * (k as f64 + 0.5) / samples as f64;
                    held_at(run, "cont", time)
                })
                .sum::<f64>()
                / samples as f64
        };
        let (app, reference) = (mean(&designed), mean(&deck));
        assert!((app - reference).abs() <= 1e-2, "{app} against {reference}");
        assert!((app - VLOCK).abs() <= 1.5e-2, "{app}");
        // fdiv.meas (O6, 51 ps): the plan's measurement of the locked period.
        let (app, reference) = (measured(&designed, "fdiv"), measured(&deck, "fdiv"));
        assert!(
            (app - reference).abs() <= 5.1e-11,
            "{app:e} against {reference:e}"
        );
        assert!((app - 1e-7).abs() <= 5.1e-11, "{app:e}");
    }
}

/// C2 drawn in the app pulls in as the corpus deck does: the control
/// voltage and every divider edge; and saved and reopened, it runs to the
/// same result. Two microseconds keep the unoptimized build quick; the lock
/// is [`c2_locks_as_its_deck`].
#[test]
fn c2_drawn_saved_and_reopened_pulls_in_as_its_deck() {
    exit_run(2e-6);
}

/// The whole 25 us: the lock and the divider's measured period.
#[test]
#[ignore = "the corpus's whole 25 us run; run with --ignored for the exit evidence"]
fn c2_locks_as_its_deck() {
    exit_run(25e-6);
}
