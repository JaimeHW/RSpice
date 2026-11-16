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

/// The project as an engineer draws it on an A3 sheet, its transient running
/// to `stop`: the stimulus down the left; the capacitor DAC across the top,
/// its top plate running right into the comparator, each bit's bottom plate
/// switched to the input, the reference or ground; the register under the
/// comparator; the switch drives in a bank below the array; and the code
/// probe along the bottom.
fn project(stop: &str) -> RunTestHost {
    use NetLabelSide::{Above, Left, Right};
    let mut drawing = Drawing::new("sar");
    let sar8 = drawing.source_view("sar8", ViewType::Verilog, "c04-sar8/net/sar8.v");
    let cmp = drawing.source_view("cmp_va", ViewType::VerilogA, "c04-sar8/net/cmp_va.va");
    let d = &mut drawing;

    // Supplies, references and stimulus, each on its net's label.
    let stimulus = |d: &mut Drawing,
                    kind: ComponentType,
                    name: &str,
                    value: &str,
                    params: &str,
                    y: i32,
                    net: &str| {
        let pins = d.part(kind, name, value, params, (20, y), UPRIGHT);
        d.tag(pins[0], Out::Up, net);
        d.earth(pins[1]);
    };
    let source = ComponentType::VoltageSource;
    stimulus(d, source, "vdd", "1.8", "", 80, "vdd");
    stimulus(d, source, "vref", "1.0", "", 200, "vref");
    stimulus(d, source, "vcref", "0", "ac=1", 320, "cref");
    let pulse = ComponentType::VoltageSourcePulse;
    let clock = "v1=0 v2=1.8 td=0 tr=1n tf=1n pw=49n per=100n";
    stimulus(d, pulse, "vclk", "", clock, 460, "clk");
    let start = "v1=1.8 v2=0 td=50n tr=1n tf=1n pw=898n per=1u";
    stimulus(d, pulse, "vstart", "", start, 620, "start");
    let input = staircase();
    stimulus(
        d,
        ComponentType::VoltageSourcePwl,
        "vin",
        &input,
        "",
        780,
        "vin",
    );

    // The register under the comparator, and the comparator at the top
    // plate's end.
    let bits = ["b7", "b6", "b5", "b4", "b3", "b2", "b1", "b0"];
    let mut register = vec![
        ("clk", "clk"),
        ("start", "start"),
        ("cmp", "cmp"),
        ("samp", "samp"),
        ("eoc", "eoc"),
    ];
    register.extend(bits.iter().map(|bit| (*bit, *bit)));
    d.instance(&sar8, "xsar", (1400, 330), &register);
    let comparator = d.instance(
        &cmp,
        "xcmp",
        (1400, 100),
        &[("inn", "cref"), ("out", "cmp"), ("vdd", "vdd")],
    );
    // Its ground off to the right, clear of the cell's name below it.
    let ground = Drawing::pin_of(&comparator, "gnd");
    d.wire(&[ground, at(ground.x + 40, ground.y)]);
    d.earth(at(ground.x + 40, ground.y));
    let top_end = Drawing::pin_of(&comparator, "inp");

    // The sampling switch grounds the top plate while samp is high; the
    // bottom plates' input switches follow samp through an RC delay.
    let switch = ComponentType::VSwitch;
    let model = "vt=0.9 vh=0.05 ron=50 roff=1e12";
    let capacitor = ComponentType::Capacitor;
    let sampler = d.part(
        switch,
        "stop",
        "",
        model,
        (200, 100),
        Orient(Rotation::R90, false, true),
    );
    d.tag(sampler[2], Out::Left, "samp");
    d.wire(&[sampler[3], at(220, 120), sampler[1]]);
    d.earth(sampler[1]);
    let rsd = d.part(
        ComponentType::Resistor,
        "rsd",
        "1k",
        "",
        (200, 220),
        UPRIGHT,
    );
    d.tag(rsd[0], Out::Left, "samp");
    let csd = d.part(capacitor, "csd", "1p", "", (240, 260), TURNED);
    d.wire(&[rsd[1], at(240, 220)]);
    d.wire(&[at(240, 220), csd[0]]);
    d.name(at(240, 220), "samp_d", Right);
    d.earth(csd[1]);

    // The capacitor DAC: Cu = 20 fF, C_k = 2^k Cu, and a dummy Cu, one
    // column a bit along the top plate. Each bottom plate runs down past
    // its switches to the input, the reference and ground; each switch's
    // drive comes in on its label.
    let behavioural = ComponentType::BehavioralSource;
    let mut tap = at(200, 80);
    let mut slot = 0;
    // A bit's column: the top plate's tap, its capacitor, and the bottom
    // plate's label. The bottom plate's end.
    let column =
        |d: &mut Drawing, tap: &mut Point, x: i32, name: &str, value: &str, plate: &str| {
            let cap = d.part(capacitor, name, value, "", (x, 120), TURNED);
            d.wire(&[*tap, at(x, 80)]);
            d.wire(&[at(x, 80), cap[0]]);
            *tap = at(x, 80);
            d.name(cap[1], plate, Left);
            cap[1]
        };
    // A switch off the bottom plate at `y`: its far side to `to` (ground
    // when `None`), its drive on `drive`. The plate's new end.
    let throw =
        |d: &mut Drawing, plate: Point, y: i32, name: &str, to: Option<&str>, drive: &str| {
            let pins = d.part(switch, name, "", model, (plate.x + 40, y), UPRIGHT);
            d.wire(&[plate, at(plate.x, y)]);
            d.wire(&[at(plate.x, y), pins[0]]);
            d.tag(pins[2], Out::Right, drive);
            match to {
                Some(net) => d.tag(pins[1], Out::Right, net),
                None => d.wire(&[pins[1], at(plate.x + 60, y + 20), pins[3]]),
            }
            d.earth(pins[3]);
            at(plate.x, y)
        };
    // A switch drive in the bank below the array: lying, its label on the
    // right and ground on the left.
    let mut drive = |d: &mut Drawing, name: &str, expression: &str, net: &str| {
        let (column, row) = (slot % 6, slot / 6);
        slot += 1;
        let origin = (300 + 210 * column, 560 + 90 * row);
        let pins = d.part(behavioural, name, expression, "", origin, TURNED);
        d.tag(pins[0], Out::Right, net);
        d.earth_left(pins[1]);
    };
    for (k, value) in [
        "20f", "40f", "80f", "160f", "320f", "640f", "1.28p", "2.56p",
    ]
    .into_iter()
    .enumerate()
    .rev()
    {
        let x = 300 + 120 * (7 - k as i32);
        let (plate, bit) = (format!("bt{k}"), format!("b{k}"));
        let (reference, ground) = (format!("cr{k}"), format!("cg{k}"));
        let end = column(d, &mut tap, x, &format!("ctop{k}"), value, &plate);
        let end = throw(d, end, 200, &format!("sv{k}"), Some("vin"), "samp_d");
        let end = throw(d, end, 320, &format!("sr{k}"), Some("vref"), &reference);
        throw(d, end, 440, &format!("sg{k}"), None, &ground);
        drive(
            d,
            &format!("bcr{k}"),
            &format!("V = v({bit})*(1.8-v(samp_d))/1.8"),
            &reference,
        );
        drive(
            d,
            &format!("bcg{k}"),
            &format!("V = (1.8-v({bit}))*(1.8-v(samp_d))/1.8"),
            &ground,
        );
    }
    let end = column(d, &mut tap, 1260, "ctopd", "20f", "btd");
    let end = throw(d, end, 200, "svd", Some("vin"), "samp_d");
    throw(d, end, 440, "sgd", None, "cgd");
    drive(d, "bcgd", "V = 1.8-v(samp_d)", "cgd");
    let named = at(top_end.x - 40, top_end.y);
    d.wire(&[tap, named]);
    d.wire(&[named, top_end]);
    d.name(named, "top", Above);

    // The code probe: the bits read at half the rail, as a number.
    let code = bits
        .iter()
        .enumerate()
        .map(|(index, bit)| format!("(v({bit})>0.9 ? {} : 0)", 128 >> index))
        .collect::<Vec<_>>()
        .join(" + ");
    let probe = d.part(
        behavioural,
        "bcode",
        &format!("V = {code}"),
        "",
        (200, 860),
        UPRIGHT,
    );
    d.tag(probe[0], Out::Up, "code");
    d.earth(probe[1]);
    d.a3_sheet();

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
