//! C4, the 8-bit SAR ADC (`tests/mixed-corpus/c04-sar8`, variant `ports`):
//! the successive-approximation register as a Verilog view, the comparator
//! as a Verilog-A view, and the binary-weighted capacitor DAC with its
//! switches, switch drives and code probe drawn on the top sheet, clocked
//! and started by pulse sources and fed a staircase of mid-code inputs.

use super::*;

/// The input staircase's codes, conversion by conversion: each input is
/// `(code + 0.5) / 256` V, half an LSB from both decision levels, so the
/// register settles on exactly this code (the corpus's analytic oracle).
const CODES: [u32; 32] = [
    0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 127, 128, 129, 144, 170, 191, 200, 233, 250, 254, 255,
    64, 63, 32, 31, 16, 15, 7, 6, 4, 192,
];

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

/// The project as an engineer draws it, its transient running to `stop`.
fn project(stop: &str) -> RunTestHost {
    let mut drawing = Drawing::new("sar");
    let sar8 = drawing.source_view("sar8", ViewType::Verilog, "c04-sar8/net/sar8.v");
    let cmp = drawing.source_view("cmp_va", ViewType::VerilogA, "c04-sar8/net/cmp_va.va");

    // Supplies, references and stimulus.
    let source = ComponentType::VoltageSource;
    drawing.part(source, "vdd", "1.8", "", &["vdd", "0"]);
    drawing.part(source, "vref", "1.0", "", &["vref", "0"]);
    drawing.part(source, "vcref", "0", "ac=1", &["cref", "0"]);
    let pulse = ComponentType::VoltageSourcePulse;
    let clock = "v1=0 v2=1.8 td=0 tr=1n tf=1n pw=49n per=100n";
    drawing.part(pulse, "vclk", "", clock, &["clk", "0"]);
    let start = "v1=1.8 v2=0 td=50n tr=1n tf=1n pw=898n per=1u";
    drawing.part(pulse, "vstart", "", start, &["start", "0"]);
    let input = staircase();
    drawing.part(
        ComponentType::VoltageSourcePwl,
        "vin",
        &input,
        "",
        &["vin", "0"],
    );
    drawing.ground();

    // The register and the comparator.
    let bits = ["b7", "b6", "b5", "b4", "b3", "b2", "b1", "b0"];
    let mut register = vec![
        ("clk", "clk"),
        ("start", "start"),
        ("cmp", "cmp"),
        ("samp", "samp"),
        ("eoc", "eoc"),
    ];
    register.extend(bits.iter().map(|bit| (*bit, *bit)));
    drawing.instance(&sar8, "xsar", 0, &register);
    drawing.instance(
        &cmp,
        "xcmp",
        1,
        &[
            ("inp", "top"),
            ("inn", "cref"),
            ("out", "cmp"),
            ("vdd", "vdd"),
            ("gnd", "0"),
        ],
    );

    // The capacitor DAC: Cu = 20 fF, C_k = 2^k Cu, and a dummy Cu.
    let switch = ComponentType::VSwitch;
    let model = "vt=0.9 vh=0.05 ron=50 roff=1e12";
    let capacitor = ComponentType::Capacitor;
    drawing.part(switch, "stop", "", model, &["top", "0", "samp", "0"]);
    drawing.part(
        ComponentType::Resistor,
        "rsd",
        "1k",
        "",
        &["samp", "samp_d"],
    );
    drawing.part(capacitor, "csd", "1p", "", &["samp_d", "0"]);
    let behavioural = ComponentType::BehavioralSource;
    for (k, value) in [
        "20f", "40f", "80f", "160f", "320f", "640f", "1.28p", "2.56p",
    ]
    .into_iter()
    .enumerate()
    .rev()
    {
        let (plate, bit) = (format!("bt{k}"), format!("b{k}"));
        let (reference, ground) = (format!("cr{k}"), format!("cg{k}"));
        drawing.part(capacitor, &format!("ctop{k}"), value, "", &["top", &plate]);
        drawing.part(
            switch,
            &format!("sv{k}"),
            "",
            model,
            &[&plate, "vin", "samp_d", "0"],
        );
        drawing.part(
            switch,
            &format!("sr{k}"),
            "",
            model,
            &[&plate, "vref", &reference, "0"],
        );
        drawing.part(
            switch,
            &format!("sg{k}"),
            "",
            model,
            &[&plate, "0", &ground, "0"],
        );
        drawing.part(
            behavioural,
            &format!("bcr{k}"),
            &format!("V = v({bit})*(1.8-v(samp_d))/1.8"),
            "",
            &[&reference, "0"],
        );
        drawing.part(
            behavioural,
            &format!("bcg{k}"),
            &format!("V = (1.8-v({bit}))*(1.8-v(samp_d))/1.8"),
            "",
            &[&ground, "0"],
        );
    }
    drawing.part(capacitor, "ctopd", "20f", "", &["top", "btd"]);
    drawing.part(switch, "svd", "", model, &["btd", "vin", "samp_d", "0"]);
    drawing.part(switch, "sgd", "", model, &["btd", "0", "cgd", "0"]);
    drawing.part(behavioural, "bcgd", "V = 1.8-v(samp_d)", "", &["cgd", "0"]);

    // The code probe: the bits read at half the rail, as a number.
    let code = bits
        .iter()
        .enumerate()
        .map(|(index, bit)| format!("(v({bit})>0.9 ? {} : 0)", 128 >> index))
        .collect::<Vec<_>>()
        .join(" + ");
    drawing.part(
        behavioural,
        "bcode",
        &format!("V = {code}"),
        "",
        &["code", "0"],
    );

    // The register's D/A converters drive at vcc.
    drawing.variable("vcc", "1.8");
    drawing.transient("1n", stop);
    drawing.measure(
        "tconv",
        ".MEAS TRAN tconv TRIG D(samp) VAL=0.5 FALL=1 TARG D(eoc) VAL=0.5 RISE=1",
    );
    drawing.finish()
}

/// The code on the register's bits at each rise of `eoc`.
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

/// Draw C4, save it, reopen it, run both, and hold the runs to each other
/// and to the corpus deck route over `conversions` conversions.
fn exit_run(conversions: usize) {
    let stop = format!("{conversions}u");
    let built = project(&stop);
    let reopened = save_and_reopen(&built, "c04");
    let designed = design_route(built);
    let rerun = design_route(reopened);
    assert_eq!(
        digest(&designed),
        digest(&rerun),
        "the reopened project runs to the same result"
    );
    let deck = deck_route("c04-sar8/deck/ports.tran.cir", ("1n", &stop));

    // codes (O6, exact): the register's bits at each eoc rise.
    let (app, reference) = (codes(&designed), codes(&deck));
    assert_eq!(app, reference, "the codes the deck route converts");
    assert_eq!(app, CODES[..conversions], "the staircase's codes");

    // tconv (O6, 2 ps): nine clock periods from samp's fall to eoc's rise.
    let (app, reference) = (measured(&designed, "tconv"), measured(&deck, "tconv"));
    assert!(
        (app - reference).abs() <= 2e-12,
        "{app:e} against {reference:e}"
    );
    assert!((app - 900e-9).abs() <= 2e-12, "{app:e}");

    // code (O6, 0.01): the probe 70 ns after each eoc, while the bits hold.
    for (k, expected) in CODES[..conversions].iter().enumerate() {
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
}

/// C4 drawn in the app converts as the corpus deck does: the same codes on
/// the register's bits, the same conversion time and the same code probe,
/// at the corpus tolerances; and saved and reopened, it runs to the same
/// result. Three conversions keep the unoptimized test build quick; the whole
/// staircase is [`c4_converts_the_whole_staircase_as_its_deck`].
#[test]
fn c4_drawn_saved_and_reopened_converts_as_its_deck() {
    exit_run(3);
}

/// All 32 conversions of the corpus staircase.
#[test]
#[ignore = "the corpus's whole 32 us staircase; run with --ignored for the exit evidence"]
fn c4_converts_the_whole_staircase_as_its_deck() {
    exit_run(32);
}
