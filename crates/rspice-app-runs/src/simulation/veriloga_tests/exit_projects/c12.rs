//! C12, the Verilog-AMS testbench over the C5 LDO
//! (`tests/mixed-corpus/c12-ldo-tb`, variant `full`): the LDO drawn as a
//! schematic cell with its own symbol, its BSIM4 cards imported as a model
//! library and attached to the plan, the trim register as a Verilog view
//! and the testbench as a Verilog-AMS view, on a top sheet with the supply
//! and the load.

use super::*;
use crate::state::PortDirectionType;

/// The LDO of `c05-ldo-trim/net/ldo_body.inc` as its own schematic cell:
/// reference, error amplifier, pass device and the trimmed feedback ladder,
/// behind the ports vin, vout and tr3..tr0.
fn ldo(drawing: &mut Drawing) -> CellViewRef {
    use NetLabelSide::{Above, Below, Left, Right};
    let d = drawing;
    // The supply comes in on the left and runs along the top rail; the
    // output leaves on the right; the trim bits come in at the bottom left.
    let vin = d.port("vin", PortDirectionType::InOutPower, (-40, 40), HALF);
    let vout = d.port("vout", PortDirectionType::OutputAnalog, (900, 160), UPRIGHT);
    // Each trim port names its bit's net, which the ladder's switches take
    // by their labels.
    for (index, bit) in ["tr3", "tr2", "tr1", "tr0"].into_iter().enumerate() {
        let y = 440 + 40 * index as i32;
        d.port(bit, PortDirectionType::InputAnalog, (-40, y), HALF);
    }
    let (resistor, capacitor) = (ComponentType::Resistor, ComponentType::Capacitor);
    let (nmos, pmos) = (ComponentType::Nmos, ComponentType::Pmos);
    // A transistor's bulk tied to its source beside the channel.
    let tie = |d: &mut Drawing, bulk: Point, source: Point, side: i32| {
        d.wire(&[bulk, at(bulk.x + side, bulk.y)]);
        d.wire(&[at(bulk.x + side, bulk.y), at(source.x + side, source.y)]);
        d.wire(&[at(source.x + side, source.y), source]);
    };

    // Reference and bias.
    let vref = d.part(
        ComponentType::VoltageSource,
        "vref",
        "0.5",
        "",
        (40, 220),
        UPRIGHT,
    );
    d.earth(vref[1]);
    d.wire(&[vref[0], at(40, 160)]);
    d.name(at(40, 160), "vr0", Right);
    let rref = d.part(resistor, "rref", "100k", "", (100, 160), UPRIGHT);
    d.wire(&[at(40, 160), rref[0]]);
    let cref = d.part(capacitor, "cref", "10p", "", (160, 220), TURNED);
    d.wire(&[rref[1], at(160, 160)]);
    d.wire(&[at(160, 160), cref[0]]);
    d.name(at(160, 160), "vref", Right);
    d.earth(cref[1]);
    let ibias = d.part(
        ComponentType::CurrentSource,
        "ibias",
        "10u",
        "",
        (280, 120),
        UPRIGHT,
    );
    d.wire(&[at(280, 40), ibias[0]]);
    let mb = d.part(nmos, "mb", "NBSIM4", "w=2u l=1u", (260, 220), UPRIGHT);
    d.wire(&[ibias[1], at(280, 150)]);
    d.wire(&[at(280, 150), mb[0]]);
    d.wire(&[mb[1], at(240, 150)]);
    d.wire(&[at(240, 150), at(280, 150)]);
    d.name(at(280, 150), "nb", Right);
    tie(d, mb[3], mb[2], 20);
    d.earth(mb[2]);
    // Tail, input pair and mirror.
    let mt = d.part(nmos, "mt", "NBSIM4", "w=4u l=1u", (440, 380), UPRIGHT);
    d.wire(&[mb[1], at(240, 380)]);
    d.wire(&[at(240, 380), mt[1]]);
    tie(d, mt[3], mt[2], 20);
    d.earth(mt[2]);
    let m1 = d.part(nmos, "m1", "NBSIM4", "w=8u l=1u", (400, 260), UPRIGHT);
    d.tag(m1[1], Out::Left, "fb");
    let m2 = d.part(nmos, "m2", "NBSIM4", "w=8u l=1u", (540, 260), MIRRORED);
    d.tag(m2[1], Out::Right, "vref");
    d.wire(&[m1[3], at(470, 260)]);
    d.wire(&[at(470, 260), m2[3]]);
    d.earth(at(470, 260));
    d.wire(&[m1[2], at(420, 320)]);
    d.wire(&[at(420, 320), at(460, 320)]);
    d.wire(&[at(460, 320), at(520, 320)]);
    d.wire(&[at(520, 320), m2[2]]);
    d.wire(&[at(460, 320), mt[0]]);
    d.name(at(460, 320), "tail", Right);
    let m3 = d.part(pmos, "m3", "PBSIM4", "w=4u l=1u", (440, 120), HALF);
    let m4 = d.part(pmos, "m4", "PBSIM4", "w=4u l=1u", (500, 120), FLIPPED);
    d.wire(&[m3[1], at(470, 120)]);
    d.wire(&[at(470, 120), m4[1]]);
    d.wire(&[at(470, 120), at(470, 180)]);
    d.wire(&[at(470, 180), at(420, 180)]);
    d.wire(&[m3[0], at(420, 180)]);
    d.wire(&[at(420, 180), m1[0]]);
    d.name(at(420, 180), "d1", Left);
    tie(d, m3[3], m3[2], -20);
    tie(d, m4[3], m4[2], 20);
    d.wire(&[m3[2], at(420, 40)]);
    d.wire(&[m4[2], at(520, 40)]);
    d.wire(&[m4[0], at(520, 180)]);
    d.wire(&[at(520, 180), m2[0]]);
    // The loop probe and the pass device.
    let vprobe = d.part(
        ComponentType::VoltageSource,
        "vprobe",
        "0",
        "",
        (600, 160),
        HALF,
    );
    d.wire(&[at(520, 180), at(560, 180)]);
    d.wire(&[at(560, 180), vprobe[0]]);
    d.name(at(560, 180), "og", Below);
    let pass = "w=10u l=0.2u m=60";
    let mp = d.part(pmos, "mp", "PBSIM4", pass, (660, 120), FLIPPED);
    d.wire(&[vprobe[1], at(600, 120)]);
    d.wire(&[at(600, 120), mp[1]]);
    d.name(at(600, 120), "og2", Above);
    tie(d, mp[3], mp[2], 20);
    d.wire(&[mp[2], at(680, 40)]);
    // The rail.
    d.wire(&[vin, at(280, 40)]);
    d.wire(&[at(280, 40), at(420, 40)]);
    d.wire(&[at(420, 40), at(520, 40)]);
    d.wire(&[at(520, 40), at(680, 40)]);
    // The feedback ladder down the right, from the pass device's drain: a
    // set trim bit shorts its resistor. Each node is named on the wire
    // between its two resistors, clear of their values.
    let x = 680;
    d.wire(&[mp[0], at(780, 160)]);
    d.wire(&[at(780, 160), vout]);
    let rtop = d.part(resistor, "rtop", "100k", "", (x, 180), TURNED);
    let mut lower = rtop[1];
    let rungs = [
        ("rb0", "60k", 250, "fb"),
        ("rt3", "8k", 320, "l1"),
        ("rt2", "4k", 390, "l2"),
        ("rt1", "2k", 460, "l3"),
        ("rt0", "1k", 530, "l4"),
    ];
    for (name, value, y, node) in rungs {
        let rung = d.part(resistor, name, value, "", (x, y), TURNED);
        let named = at(x, lower.y + 20);
        d.wire(&[lower, named]);
        d.wire(&[named, rung[0]]);
        d.name(named, node, Right);
        lower = rung[1];
    }
    d.earth(lower);
    let switch = "vt=0.9 vh=0.1 ron=10 roff=1e12";
    for (bit, y) in [("3", 320), ("2", 390), ("1", 460), ("0", 530)] {
        let control = format!("tr{bit}");
        let pins = d.part(
            ComponentType::VSwitch,
            &format!("s{bit}"),
            "",
            switch,
            (x + 80, y),
            TURNED,
        );
        d.wire(&[at(x, y - 20), pins[0]]);
        if bit == "0" {
            d.earth(pins[1]);
        } else {
            d.wire(&[at(x, y + 20), pins[1]]);
        }
        d.tag(pins[2], Out::Right, &control);
        d.earth_left(pins[3]);
    }
    d.master("ldo")
}

/// The project as an engineer draws it, its transient running to `stop`.
pub(super) fn project(stop: &str) -> RunTestHost {
    let mut drawing = Drawing::new("ldotb");
    drawing.model_library("c05-ldo-trim/net/models-bsim4.cir");
    let ldo = ldo(&mut drawing);
    let trim = drawing.source_view("spi_trim", ViewType::Verilog, "c05-ldo-trim/net/spi_trim.v");
    let bench = drawing.source_view(
        "tb_ldo",
        ViewType::VerilogAms,
        "c12-ldo-tb/net/tb_ldo_full.vams",
    );

    // The testbench drives the trim register over SPI, the register trims
    // the LDO, and the LDO feeds the static load (C5's, without its load
    // step); the supply comes in from above.
    use NetLabelSide::{Above, Right};
    let d = &mut drawing;
    let vin = d.part(
        ComponentType::VoltageSource,
        "vin",
        "1.8",
        "ac=1",
        (440, 200),
        UPRIGHT,
    );
    d.earth(vin[1]);
    let rload = d.part(
        ComponentType::Resistor,
        "rload",
        "100",
        "",
        (760, 380),
        TURNED,
    );
    d.earth(rload[1]);
    let cout = d.part(
        ComponentType::Capacitor,
        "cout",
        "1u",
        "",
        (680, 380),
        TURNED,
    );
    let resr = d.part(
        ComponentType::Resistor,
        "resr",
        "0.1",
        "",
        (680, 480),
        TURNED,
    );
    // The capacitor's ESR node, named halfway down to its resistor.
    let esr = at(680, 430);
    d.wire(&[cout[1], esr]);
    d.wire(&[esr, resr[0]]);
    d.name(esr, "co", Right);
    d.earth(resr[1]);

    let regulator = d.instance(&ldo, "xldo", (560, 290), &[]);
    let register = d.instance(&trim, "xtrim", (320, 300), &[]);
    let bench = d.instance(
        &bench,
        "xtb",
        (100, 310),
        &[("vout", "vout"), ("ok", "ok"), ("meas", "meas")],
    );
    // The supply, down onto the regulator's input.
    d.wire(&[vin[0], at(440, 150)]);
    d.wire(&[at(440, 150), at(500, 150)]);
    d.wire(&[at(500, 150), at(500, 250)]);
    d.wire(&[at(500, 250), Drawing::pin_of(&regulator, "vin")]);
    d.name(at(440, 150), "vin", Above);
    // SPI from the testbench into the register, and the trim bits on.
    for (net, side) in [("sclk", Above), ("sdi", Above), ("csn", Above)] {
        let (from, to) = (
            Drawing::pin_of(&bench, net),
            Drawing::pin_of(&register, net),
        );
        let middle = at(210, from.y);
        d.wire(&[from, middle]);
        d.wire(&[middle, to]);
        d.name(middle, net, side);
    }
    for (net, side) in [
        ("tr3", Above),
        ("tr2", Above),
        ("tr1", Above),
        ("tr0", Above),
    ] {
        let (from, to) = (
            Drawing::pin_of(&register, net),
            Drawing::pin_of(&regulator, net),
        );
        let middle = at(440, from.y);
        d.wire(&[from, middle]);
        d.wire(&[middle, to]);
        d.name(middle, net, side);
    }
    // The output and its load.
    let out = Drawing::pin_of(&regulator, "vout");
    d.wire(&[out, at(680, out.y)]);
    d.wire(&[at(680, out.y), cout[0]]);
    d.wire(&[at(680, out.y), at(760, out.y)]);
    d.wire(&[at(760, out.y), rload[0]]);
    d.name(at(760, out.y), "vout", Right);

    drawing.variable("vcc", "1.8");
    drawing.transient("0.5u", stop);
    drawing.measure("okt", ".MEAS TRAN okt WHEN D(ok)=0.5 RISE=1");
    drawing.finish()
}

/// When the testbench raises its verdict: after sixteen trim steps of
/// 100 us (the corpus's analytic oracle).
const VERDICT: f64 = 1.6e-3;

/// The testbench's reading of code `k`, held on `meas` through the corpus
/// window: 92.5 us after code `k`'s latch.
fn reading(run: &SimulationRun, k: usize) -> f64 {
    held_at(run, "meas", k as f64 * 100e-6 + 99.5e-6)
}

/// V(vout) at the code-`k` operating point, the trim code held
/// (ngspice-46, `ref/c<k>.op.O5-O1.out`).
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

/// Draw C12, save it, reopen it, run both, and hold the runs to each other
/// and to the corpus deck route over the first `codes` trim codes.
fn exit_run(codes: usize) {
    let stop = if codes == 16 {
        "1.65m".to_owned()
    } else {
        format!("{}u", codes * 100)
    };
    let built = project(&stop);
    let reopened = save_and_reopen(&built, "c12");
    let designed = design_route(built);
    let rerun = design_route(reopened);
    assert_eq!(
        digest(&designed),
        digest(&rerun),
        "the reopened project runs to the same result"
    );
    let deck = deck_route("c12-ldo-tb/deck/full.tran.cir", ("0.5u", &stop));

    // vk.<k> (O5-O1, 1 mV): each reading against the deck route's and the
    // code's operating point.
    for k in 0..codes {
        let (app, reference) = (reading(&designed, k), reading(&deck, k));
        assert!(
            (app - reference).abs() <= 1e-3,
            "code {k}: {app} against {reference}"
        );
        assert!((app - oracle(k)).abs() <= 1e-3, "code {k}: {app}");
    }

    // ok (O6, 1 ns): the verdict on the testbench's port, as the deck's
    // converter endpoint carries it.
    let (app, reference) = (events(&designed, "ok"), events(&deck, "xtb.ok__event_0"));
    assert_eq!(app.len(), reference.len(), "{app:?} against {reference:?}");
    for ((time, code), (deck_time, deck_code)) in app.iter().zip(&reference) {
        assert_eq!(code, deck_code, "{app:?} against {reference:?}");
        assert!(
            (time - deck_time).abs() <= 1e-9,
            "{app:?} against {reference:?}"
        );
    }
    let rose = app
        .iter()
        .find(|(_, code)| *code == 1)
        .map(|(time, _)| *time);
    if codes == 16 {
        assert!(
            rose.is_some_and(|time| (time - VERDICT).abs() <= 1e-9),
            "{app:?}"
        );
        // okt (O6, 1 ps): the plan's measurement of that rise.
        let (app, reference) = (measured(&designed, "okt"), measured(&deck, "okt"));
        assert!(
            (app - reference).abs() <= 1e-12,
            "{app:e} against {reference:e}"
        );
        assert!((app - VERDICT).abs() <= 1e-12, "{app:e}");
    } else {
        assert_eq!(rose, None, "no verdict before the sequence ends: {app:?}");
    }
}

/// C12 drawn in the app reads its trim codes as the corpus deck does: each
/// reading of V(vout) against the deck route's and the code's operating
/// point, and the verdict's events; and saved and reopened, it runs to the
/// same result. Two codes keep the unoptimized test build quick; the whole
/// sequence is [`c12_runs_the_whole_trim_sequence_as_its_deck`].
#[test]
fn c12_drawn_saved_and_reopened_reads_its_codes_as_its_deck() {
    exit_run(2);
}

/// All sixteen trim codes, the verdict and its measurement.
#[test]
#[ignore = "the corpus's whole 1.6 ms trim sequence; run with --ignored for the exit evidence"]
fn c12_runs_the_whole_trim_sequence_as_its_deck() {
    exit_run(16);
}
