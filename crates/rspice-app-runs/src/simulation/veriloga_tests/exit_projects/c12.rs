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
    drawing.port("vin", PortDirectionType::InOutPower);
    drawing.port("vout", PortDirectionType::OutputAnalog);
    for bit in ["tr3", "tr2", "tr1", "tr0"] {
        drawing.port(bit, PortDirectionType::InputAnalog);
    }
    let (resistor, capacitor) = (ComponentType::Resistor, ComponentType::Capacitor);
    let (nmos, pmos) = (ComponentType::Nmos, ComponentType::Pmos);

    // Reference and bias.
    drawing.part(
        ComponentType::VoltageSource,
        "vref",
        "0.5",
        "",
        &["vr0", "0"],
    );
    drawing.part(resistor, "rref", "100k", "", &["vr0", "vref"]);
    drawing.part(capacitor, "cref", "10p", "", &["vref", "0"]);
    drawing.part(
        ComponentType::CurrentSource,
        "ibias",
        "10u",
        "",
        &["vin", "nb"],
    );
    drawing.part(nmos, "mb", "NBSIM4", "w=2u l=1u", &["nb", "nb", "0", "0"]);
    // Tail, input pair and mirror.
    drawing.part(nmos, "mt", "NBSIM4", "w=4u l=1u", &["tail", "nb", "0", "0"]);
    drawing.part(
        nmos,
        "m1",
        "NBSIM4",
        "w=8u l=1u",
        &["d1", "fb", "tail", "0"],
    );
    drawing.part(
        nmos,
        "m2",
        "NBSIM4",
        "w=8u l=1u",
        &["og", "vref", "tail", "0"],
    );
    drawing.part(
        pmos,
        "m3",
        "PBSIM4",
        "w=4u l=1u",
        &["d1", "d1", "vin", "vin"],
    );
    drawing.part(
        pmos,
        "m4",
        "PBSIM4",
        "w=4u l=1u",
        &["og", "d1", "vin", "vin"],
    );
    // The loop probe and the pass device.
    drawing.part(
        ComponentType::VoltageSource,
        "vprobe",
        "0",
        "",
        &["og", "og2"],
    );
    let pass = "w=10u l=0.2u m=60";
    drawing.part(pmos, "mp", "PBSIM4", pass, &["vout", "og2", "vin", "vin"]);
    // The feedback ladder: a set trim bit shorts its resistor.
    drawing.part(resistor, "rtop", "100k", "", &["vout", "fb"]);
    drawing.part(resistor, "rb0", "60k", "", &["fb", "l1"]);
    let ladder = [
        ("3", "8k", "l1", "l2"),
        ("2", "4k", "l2", "l3"),
        ("1", "2k", "l3", "l4"),
    ];
    for (bit, value, top, bottom) in ladder {
        drawing.part(resistor, &format!("rt{bit}"), value, "", &[top, bottom]);
    }
    drawing.part(resistor, "rt0", "1k", "", &["l4", "0"]);
    let switch = "vt=0.9 vh=0.1 ron=10 roff=1e12";
    let switches = [
        ("3", "l1", "l2"),
        ("2", "l2", "l3"),
        ("1", "l3", "l4"),
        ("0", "l4", "0"),
    ];
    for (bit, top, bottom) in switches {
        let control = format!("tr{bit}");
        drawing.part(
            ComponentType::VSwitch,
            &format!("s{bit}"),
            "",
            switch,
            &[top, bottom, &control, "0"],
        );
    }
    drawing.ground();
    drawing.master("ldo")
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

    // Supply and the static load (C5's, without its load step).
    drawing.part(
        ComponentType::VoltageSource,
        "vin",
        "1.8",
        "ac=1",
        &["vin", "0"],
    );
    drawing.part(ComponentType::Resistor, "rload", "100", "", &["vout", "0"]);
    drawing.part(ComponentType::Capacitor, "cout", "1u", "", &["vout", "co"]);
    drawing.part(ComponentType::Resistor, "resr", "0.1", "", &["co", "0"]);
    drawing.ground();

    let bits = [
        ("tr3", "tr3"),
        ("tr2", "tr2"),
        ("tr1", "tr1"),
        ("tr0", "tr0"),
    ];
    let mut regulator = vec![("vin", "vin"), ("vout", "vout")];
    regulator.extend(bits);
    drawing.instance(&ldo, "xldo", 0, &regulator);
    let mut register = vec![("sclk", "sclk"), ("sdi", "sdi"), ("csn", "csn")];
    register.extend(bits);
    drawing.instance(&trim, "xtrim", 1, &register);
    drawing.instance(
        &bench,
        "xtb",
        2,
        &[
            ("vout", "vout"),
            ("sclk", "sclk"),
            ("sdi", "sdi"),
            ("csn", "csn"),
            ("ok", "ok"),
            ("meas", "meas"),
        ],
    );

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
