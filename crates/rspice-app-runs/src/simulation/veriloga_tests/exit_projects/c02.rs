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
    use NetLabelSide::{Above, Right};
    let d = drawing;
    // The pump's two gates come in on the left; the filtered control
    // voltage leaves on the right.
    let u1n = d.port("u1n", PortDirectionType::InputAnalog, (-20, 200), HALF);
    let d1 = d.port("d1", PortDirectionType::InputAnalog, (-20, 320), HALF);
    let vout = d.port("vout", PortDirectionType::OutputAnalog, (760, 260), UPRIGHT);
    let (resistor, capacitor) = (ComponentType::Resistor, ComponentType::Capacitor);
    let source = ComponentType::VoltageSource;
    // The pump's rails.
    let vdd = d.part(source, "vdd", "{vcc}", "", (60, 100), UPRIGHT);
    d.earth(vdd[1]);
    d.wire(&[vdd[0], at(60, 60)]);
    d.name(at(60, 60), "dd", Right);
    let vss = d.part(source, "vss", "0", "", (60, 440), UPRIGHT);
    d.earth(vss[1]);
    // The pump: the PMOS sources charge from the upper rail, the NMOS sinks
    // it to the lower one, both onto dra.
    let n = "w=12u l=0.35u as=24p ad=24p ps=28u pd=28u";
    let mnd = d.part(ComponentType::Nmos, "mnd", "N1", n, (240, 320), UPRIGHT);
    let p = "w=24u l=0.35u as=48p ad=48p ps=52u pd=52u";
    let mpd = d.part(ComponentType::Pmos, "mpd", "P1", p, (240, 200), FLIPPED);
    d.wire(&[at(60, 60), at(260, 60)]);
    d.wire(&[at(260, 60), mpd[2]]);
    for (bulk, source) in [(mpd[3], mpd[2]), (mnd[3], mnd[2])] {
        d.wire(&[bulk, at(bulk.x + 20, bulk.y)]);
        d.wire(&[at(bulk.x + 20, bulk.y), at(source.x + 20, source.y)]);
        d.wire(&[at(source.x + 20, source.y), source]);
    }
    // The ports name the gates' nets.
    d.wire(&[u1n, mpd[1]]);
    d.wire(&[d1, mnd[1]]);
    d.wire(&[mpd[0], at(260, 260)]);
    d.wire(&[at(260, 260), mnd[0]]);
    d.wire(&[mnd[2], at(260, 400)]);
    d.wire(&[at(260, 400), at(60, 400)]);
    d.wire(&[at(60, 400), vss[0]]);
    d.name(at(60, 400), "ss", Right);
    // The third-order filter.
    let r1 = d.part(resistor, "r1", "{r1v}", "", (340, 260), UPRIGHT);
    d.wire(&[at(260, 260), at(290, 260)]);
    d.wire(&[at(290, 260), r1[0]]);
    d.name(at(290, 260), "dra", Above);
    let r2 = d.part(resistor, "r2", "200", "", (400, 340), TURNED);
    d.wire(&[r1[1], at(400, 260)]);
    d.wire(&[at(400, 260), r2[0]]);
    d.name(at(400, 260), "int1", Above);
    let c1 = d.part(capacitor, "c1", "{c1v}", "ic=2.5", (400, 420), TURNED);
    d.wire(&[r2[1], at(400, 380)]);
    d.wire(&[at(400, 380), c1[0]]);
    d.name(at(400, 380), "c1", Right);
    d.earth(c1[1]);
    let c2 = d.part(capacitor, "c2", "5n", "ic=2.5", (480, 340), TURNED);
    d.wire(&[at(400, 260), at(480, 260)]);
    d.wire(&[at(480, 260), c2[0]]);
    d.earth(c2[1]);
    let r3 = d.part(resistor, "r3", "50", "", (560, 260), UPRIGHT);
    d.wire(&[at(480, 260), r3[0]]);
    let c3 = d.part(capacitor, "c3", "0.5n", "ic=2.5", (660, 340), TURNED);
    d.wire(&[r3[1], at(660, 260)]);
    d.wire(&[at(660, 260), c3[0]]);
    d.wire(&[at(660, 260), vout]);
    d.earth(c3[1]);
    d.master("loopf_mx")
}

/// The project as an engineer draws it, its transient running to `stop`
/// from the initial conditions.
pub(super) fn project(stop: &str) -> RunTestHost {
    let mut drawing = Drawing::new("pll");
    drawing.model_library("c02-pll/net/pll-models.cir");
    let filter = loop_filter(&mut drawing);
    let vco = drawing.source_view("vco_pwl", ViewType::VerilogA, "c02-pll/net/vco_pwl.va");
    let pfd = drawing.source_view("div20_pfd", ViewType::Verilog, "c02-pll/net/div20_pfd.v");

    // The forward path left to right along the top: the 10 MHz reference
    // and its bridge onto the event side at 0.5 V, the phase-frequency
    // detector, the loop filter and the VCO. The feedback path below: the
    // VCO's bridge and the divide-by-2 back to the detector; beside it the
    // D/As the deck plots the divider and the reference through.
    use NetLabelSide::{Above, Below, Right};
    let d = &mut drawing;
    let reference = "v1=0 v2=3.3 td=10n tr=1n tf=1n pw=50n per=100n";
    let vref = d.part(
        ComponentType::VoltageSourcePulse,
        "vref",
        "",
        reference,
        (40, 240),
        UPRIGHT,
    );
    d.earth(vref[1]);
    let adc = "in_low=0.5 in_high=0.5";
    let bridge = ComponentType::XspiceAdcBridge;
    let abridgeref = d.part(bridge, "abridgeref", "", adc, (200, 200), UPRIGHT);
    d.wire(&[vref[0], at(40, 200)]);
    d.wire(&[at(40, 200), abridgeref[0]]);
    d.name(at(40, 200), "ref", Right);
    let abridgevco = d.part(bridge, "abridgevco", "", adc, (360, 340), UPRIGHT);
    d.tag(abridgevco[0], Out::Left, "vco_out");
    let dac = "out_low=0 out_high=1 out_undef=0.5 input_load=5e-12 t_rise=1e-10 t_fall=1e-10";
    let plot = ComponentType::XspiceDacBridge;
    for (name, input, output, y) in [
        ("abridgew1", "d_divout", "s1", 340),
        ("abridgew2", "d_ref", "s2", 420),
    ] {
        let pins = d.part(plot, name, "", dac, (720, y), UPRIGHT);
        d.tag(pins[0], Out::Left, input);
        d.tag(pins[1], Out::Right, output);
    }

    let oscillator = d.instance(&vco, "xvco", (680, 180), &[("out", "vco_out")]);
    let divide = "div_factor=2 high_cycles=1 i_count=0 rise_delay=1e-10 fall_delay=1e-10";
    let divider = d.shelf_xspice(
        "rspice.xspice.d_fdiv",
        "adiv2",
        divide,
        (480, 340),
        &[("freq_out", "d_div2")],
    );
    d.wire(&[abridgevco[1], at(400, 340)]);
    d.wire(&[at(400, 340), Drawing::pin_of(&divider, "freq_in")]);
    d.name(at(400, 340), "d_vco", Above);
    let detector = d.instance(&pfd, "xpfd", (360, 190), &[("clkin", "d_div2")]);
    // The divided clock leaves downward, clear of the pump's two lines.
    let divided = Drawing::pin_of(&detector, "divo");
    let turn = at(divided.x + 20, divided.y);
    d.wire(&[divided, turn, at(turn.x, turn.y + 30)]);
    d.name(at(turn.x, turn.y + 30), "d_divout", Below);
    d.wire(&[abridgeref[1], at(260, 200)]);
    d.wire(&[at(260, 200), Drawing::pin_of(&detector, "ref")]);
    d.name(at(260, 200), "d_ref", Above);
    let filter = d.instance(&filter, "xlf", (520, 180), &[]);
    for (from, to, net) in [("up_n", "u1n", "up_n"), ("dn", "d1", "dn")] {
        let (from, to) = (
            Drawing::pin_of(&detector, from),
            Drawing::pin_of(&filter, to),
        );
        let middle = at(440, from.y);
        d.wire(&[from, middle]);
        d.wire(&[middle, to]);
        d.name(middle, net, Above);
    }
    let control = Drawing::pin_of(&filter, "vout");
    d.wire(&[control, at(600, control.y)]);
    d.wire(&[at(600, control.y), Drawing::pin_of(&oscillator, "cont")]);
    d.name(at(600, control.y), "cont", Above);

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
