//! Metamorphic rewrites: a deck rewritten in a way that cannot change its
//! circuit must give the same answer.
//!
//! Each circuit below is solved as written and again after one rewrite:
//!
//! * `Reorder` reverses its element cards and its model and subcircuit
//!   definitions;
//! * `Rename` gives every node a new name, chosen so the new names sort in
//!   the reverse of the old order;
//! * `Wrap` moves every device except the independent sources into a
//!   subcircuit and instantiates it once;
//! * `Split` replaces a device written with `m=k` by `k` parallel copies.
//!
//! None of these changes the circuit, so a difference beyond round-off is a
//! defect in parsing, elaboration, or the order the engine assembles and
//! solves in. The operating point, an AC sweep, a transient and a noise sweep
//! are compared at every observed node, with the observed sources' currents.
//!
//! Two runs agree when `|a - b| <= floor + ROUND_OFF * max(|a|, |b|)` for
//! every sample, magnitudes taken for complex values. `ROUND_OFF` is 1e-9:
//! far below any modelling error, and well above the drift a different
//! summation order leaves. The absolute floors only keep a sample that should
//! be zero from demanding exact zeros. Each comparison prints whether the two
//! runs were bit-identical and, if not, its worst deviation as a fraction of
//! the allowance.
//!
//! The operating point, AC and noise run under [`CONVERGED`] solver
//! tolerances. At the default ones Newton stops anywhere within RELTOL of the
//! solution, and a rewrite that changes the system (`Split` gives each copy
//! its own internal nodes) changes where: a BJT split that way differs by
//! about 2e-9 relative at the defaults and by round-off under these. The
//! transient keeps the default tolerances, because RELTOL also sets its step
//! control and a tolerance this tight makes the accepted grid follow
//! round-off.
//!
//! Every analysis runs with GMIN too small to count. In the ngspice dialect
//! a device's M is folded into its saturation currents and GMIN is added
//! once per junction whatever M is (as ngspice's own `mos1load.c` and
//! `dioload.c` do), so `m=k` and `k` copies legitimately differ by
//! `(k - 1) * GMIN` per junction: a few parts in 1e9 for the MOS1 split at
//! the default GMIN of 1e-12.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use rspice_core::NoAbort;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The relative difference two runs of the same circuit may show.
const ROUND_OFF: f64 = 1e-9;
/// Absolute floors: a volt, an ampere and a noise density (V^2/Hz) this
/// small is zero for the comparison.
const VOLT_FLOOR: f64 = 1e-12;
const AMP_FLOOR: f64 = 1e-15;
const NOISE_FLOOR: f64 = 1e-30;
/// The grid a transient's time points are compared on.
const SECOND_FLOOR: f64 = 1e-18;

const AC_FREQUENCIES: [f64; 6] = [1e1, 1e2, 1e3, 1e4, 1e5, 1e6];
const NOISE_FREQUENCIES: [f64; 5] = [1e1, 1e2, 1e3, 1e4, 1e5];
const TEMPERATURE: f64 = 300.15;

/// Solver tolerances tight enough that Newton's stopping point is round-off,
/// for the operating point, AC and noise; GMIN as [`TRANSIENT`] sets it.
const CONVERGED: &str = ".options reltol=1e-9 vntol=1e-15 abstol=1e-21 gmin=1e-30";
/// The transient's options: the default tolerances, and a GMIN too small to
/// count.
const TRANSIENT: &str = ".options gmin=1e-30";

/// A deck as the rewrites see it: the element cards at the top level, and the
/// model and subcircuit definitions they use, each one entry.
#[derive(Clone, Debug)]
struct Deck {
    elements: Vec<String>,
    library: Vec<String>,
}

impl Deck {
    fn new(elements: &[&str], library: &[&str]) -> Self {
        Self {
            elements: elements.iter().map(|card| card.to_string()).collect(),
            library: library.iter().map(|card| card.to_string()).collect(),
        }
    }

    /// The deck's text, with `options` (a `.options` card) when given.
    fn text(&self, options: Option<&str>) -> String {
        let mut text = String::from("* metamorphic\n");
        let cards = self
            .elements
            .iter()
            .chain(&self.library)
            .map(String::as_str);
        for card in cards.chain(options) {
            text.push_str(card);
            text.push('\n');
        }
        text.push_str(".end\n");
        text
    }
}

/// A circuit and what is compared on it.
struct Circuit {
    name: &'static str,
    deck: Deck,
    /// Nodes whose voltages are compared; each stays a port under `Wrap`.
    observed: &'static [&'static str],
    /// Voltage sources whose branch currents are compared.
    sources: &'static [&'static str],
    /// The noise output node and input source.
    noise: (&'static str, &'static str),
    /// Transient stop time and maximum step.
    tran: (f64, f64),
    /// The device written with `m=k`, and `k`.
    split: Option<(&'static str, usize)>,
}

#[derive(Clone, Copy, Debug)]
enum Rewrite {
    Reorder,
    Rename,
    Wrap,
    Split,
}

impl Rewrite {
    /// The rewritten deck, and the new name of each node that changed name.
    fn apply(self, circuit: &Circuit) -> (Deck, BTreeMap<String, String>) {
        match self {
            Self::Reorder => (reorder(&circuit.deck), BTreeMap::new()),
            Self::Rename => rename(&circuit.deck),
            Self::Wrap => (wrap(circuit), BTreeMap::new()),
            Self::Split => {
                let (device, copies) = circuit
                    .split
                    .unwrap_or_else(|| panic!("{} has no device to split", circuit.name));
                (split(&circuit.deck, device, copies), BTreeMap::new())
            }
        }
    }
}

/// The token positions of a card's nodes.
fn node_positions(card: &str) -> std::ops::Range<usize> {
    let tokens: Vec<&str> = card.split_whitespace().collect();
    let kind = tokens[0]
        .chars()
        .next()
        .expect("a card has a name")
        .to_ascii_lowercase();
    let count = match kind {
        'r' | 'c' | 'l' | 'v' | 'i' | 'd' => 2,
        'q' => 3,
        'm' | 'e' | 'g' => 4,
        // Every token between the name and the subcircuit's name, which is
        // the last one before any parameter.
        'x' => {
            let parameters = tokens
                .iter()
                .position(|token| token.contains('='))
                .unwrap_or(tokens.len());
            parameters - 2
        }
        other => panic!("no node layout for a `{other}` card: {card}"),
    };
    1..1 + count
}

fn nodes_of(card: &str) -> Vec<String> {
    let tokens: Vec<&str> = card.split_whitespace().collect();
    tokens[node_positions(card)]
        .iter()
        .map(|node| node.to_ascii_lowercase())
        .collect()
}

fn is_source(card: &str) -> bool {
    matches!(card.as_bytes()[0].to_ascii_lowercase(), b'v' | b'i')
}

fn reorder(deck: &Deck) -> Deck {
    Deck {
        elements: deck.elements.iter().rev().cloned().collect(),
        library: deck.library.iter().rev().cloned().collect(),
    }
}

fn rename(deck: &Deck) -> (Deck, BTreeMap<String, String>) {
    let nodes: BTreeSet<String> = deck
        .elements
        .iter()
        .flat_map(|card| nodes_of(card))
        .filter(|node| node != "0")
        .collect();
    let count = nodes.len();
    let names: BTreeMap<String, String> = nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| (node, format!("w{:02}", count - index)))
        .collect();
    let elements = deck
        .elements
        .iter()
        .map(|card| {
            let positions = node_positions(card);
            card.split_whitespace()
                .enumerate()
                .map(|(index, token)| {
                    if positions.contains(&index) {
                        names
                            .get(&token.to_ascii_lowercase())
                            .map_or(token, String::as_str)
                    } else {
                        token
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    let deck = Deck {
        elements,
        library: deck.library.clone(),
    };
    (deck, names)
}

fn wrap(circuit: &Circuit) -> Deck {
    let (sources, devices): (Vec<&String>, Vec<&String>) = circuit
        .deck
        .elements
        .iter()
        .partition(|card| is_source(card));
    let ports: BTreeSet<String> = sources
        .iter()
        .flat_map(|card| nodes_of(card))
        .chain(circuit.observed.iter().map(|node| node.to_string()))
        .chain(std::iter::once(circuit.noise.0.to_string()))
        .filter(|node| node != "0")
        .collect();
    let ports = ports.into_iter().collect::<Vec<_>>().join(" ");
    let mut definition = format!(".subckt wrapped {ports}\n");
    for card in devices {
        definition.push_str(card);
        definition.push('\n');
    }
    definition.push_str(".ends wrapped");
    let mut elements: Vec<String> = sources.into_iter().cloned().collect();
    elements.push(format!("xwrapped {ports} wrapped"));
    let mut library = circuit.deck.library.clone();
    library.push(definition);
    Deck { elements, library }
}

fn split(deck: &Deck, device: &str, copies: usize) -> Deck {
    let multiplier = format!("m={copies}");
    let mut found = false;
    let mut elements = Vec::new();
    for card in &deck.elements {
        let tokens: Vec<&str> = card.split_whitespace().collect();
        if !tokens[0].eq_ignore_ascii_case(device) {
            elements.push(card.clone());
            continue;
        }
        let rest: Vec<&str> = tokens[1..]
            .iter()
            .copied()
            .filter(|token| !token.eq_ignore_ascii_case(&multiplier))
            .collect();
        assert_eq!(rest.len() + 2, tokens.len(), "{card} carries {multiplier}");
        found = true;
        for copy in 1..=copies {
            elements.push(format!("{device}_{copy} {}", rest.join(" ")));
        }
    }
    assert!(found, "no {device} card to split");
    Deck {
        elements,
        library: deck.library.clone(),
    }
}

/// One compared value: real values carry a zero imaginary part.
#[derive(Debug)]
struct Sample {
    label: String,
    value: [f64; 2],
    floor: f64,
}

/// Everything one analysis is compared on.
#[derive(Debug)]
struct Samples {
    analysis: &'static str,
    values: Vec<Sample>,
}

impl Samples {
    fn new(analysis: &'static str) -> Self {
        Self {
            analysis,
            values: Vec::new(),
        }
    }

    fn real(&mut self, label: String, value: f64, floor: f64) {
        self.values.push(Sample {
            label,
            value: [value, 0.0],
            floor,
        });
    }

    fn complex(&mut self, label: String, re: f64, im: f64, floor: f64) {
        self.values.push(Sample {
            label,
            value: [re, im],
            floor,
        });
    }
}

fn index_of(names: &[String], name: &str, what: &str) -> usize {
    names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no {what} `{name}` among {names:?}"))
}

/// Solve every analysis on `deck`, naming each sample by the circuit's
/// original node names; `node` gives a node's name in `deck`.
fn observe(circuit: &Circuit, deck: &Deck, node: &dyn Fn(&str) -> String) -> Vec<Samples> {
    let name = circuit.name;
    let parse = |options| {
        let text = deck.text(options);
        let netlist =
            Netlist::parse(&text).unwrap_or_else(|error| panic!("{name}: parse: {error}\n{text}"));
        (netlist, text)
    };
    let (netlist, text) = parse(Some(CONVERGED));
    let (tran_netlist, tran_text) = parse(Some(TRANSIENT));
    let engine = Engine::new(SimulationConfig::default());

    let mut op = Samples::new("op");
    let result = engine
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("{name} op: {error}\n{text}"));
    for observed in circuit.observed {
        let index = index_of(&result.node_names, &node(observed), "node");
        op.real(
            format!("V({observed})"),
            result.node_voltages[index],
            VOLT_FLOOR,
        );
    }
    for source in circuit.sources {
        let index = index_of(&result.branch_names, source, "branch");
        op.real(
            format!("I({source})"),
            result.branch_currents[index],
            AMP_FLOOR,
        );
    }

    let mut ac = Samples::new("ac");
    let sweep = engine
        .run_ac(&netlist, &AC_FREQUENCIES)
        .unwrap_or_else(|error| panic!("{name} ac: {error}\n{text}"));
    for point in &sweep {
        for observed in circuit.observed {
            let index = index_of(&point.node_names, &node(observed), "node");
            let value = point.voltages[index];
            let label = format!("V({observed})@{:e}", point.frequency);
            ac.complex(label, value.re, value.im, VOLT_FLOOR);
        }
        for source in circuit.sources {
            let index = index_of(&point.branch_names, source, "branch");
            let value = point.currents[index];
            let label = format!("I({source})@{:e}", point.frequency);
            ac.complex(label, value.re, value.im, AMP_FLOOR);
        }
    }

    let mut tran = Samples::new("tran");
    let (stop, max_step) = circuit.tran;
    let waveform = engine
        .run_tran(&tran_netlist, stop, max_step)
        .unwrap_or_else(|error| panic!("{name} tran: {error}\n{tran_text}"));
    for (step, time) in waveform.time.iter().enumerate() {
        tran.real(format!("t[{step}]"), *time, SECOND_FLOOR);
    }
    for observed in circuit.observed {
        let index = index_of(&waveform.node_names, &node(observed), "node");
        for (step, value) in waveform.voltages[index].iter().enumerate() {
            tran.real(format!("V({observed})[{step}]"), *value, VOLT_FLOOR);
        }
    }

    let mut noise = Samples::new("noise");
    let (output, input) = circuit.noise;
    let spectrum = engine
        .run_noise_named_with_input_source_and_abort(
            &netlist,
            &node(output),
            None,
            input,
            &NOISE_FREQUENCIES,
            TEMPERATURE,
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("{name} noise: {error}\n{text}"));
    for point in &spectrum {
        let frequency = point.frequency;
        noise.real(
            format!("onoise@{frequency:e}"),
            point.output_noise_density,
            NOISE_FLOOR,
        );
        noise.real(
            format!("inoise@{frequency:e}"),
            point.input_referred_density,
            NOISE_FLOOR,
        );
    }

    vec![op, ac, tran, noise]
}

/// How two runs of one analysis compare.
struct Verdict {
    bit_identical: bool,
    /// The worst deviation as a fraction of its allowance, and its sample.
    worst: Option<(f64, String)>,
    failures: Vec<String>,
}

impl fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bit_identical {
            return formatter.write_str("bit-identical");
        }
        match &self.worst {
            Some((ratio, label)) => write!(formatter, "worst {ratio:.3e} of allowance at {label}"),
            None => formatter.write_str("not compared"),
        }
    }
}

fn magnitude([re, im]: [f64; 2]) -> f64 {
    re.hypot(im)
}

fn compare(base: &Samples, other: &Samples) -> Verdict {
    if base.values.len() != other.values.len() {
        let first_difference = base
            .values
            .iter()
            .zip(&other.values)
            .find(|(a, b)| a.label != b.label)
            .map(|(a, _)| a.label.clone());
        return Verdict {
            bit_identical: false,
            worst: None,
            failures: vec![format!(
                "{}: {} samples against {} (labels part at {first_difference:?})",
                base.analysis,
                base.values.len(),
                other.values.len()
            )],
        };
    }
    let mut bit_identical = true;
    let mut worst: Option<(f64, String)> = None;
    let mut failures = Vec::new();
    for (a, b) in base.values.iter().zip(&other.values) {
        assert_eq!(a.label, b.label, "{} samples line up", base.analysis);
        bit_identical &= a.value[0].to_bits() == b.value[0].to_bits()
            && a.value[1].to_bits() == b.value[1].to_bits();
        let difference = magnitude([a.value[0] - b.value[0], a.value[1] - b.value[1]]);
        let allowance = a.floor + ROUND_OFF * magnitude(a.value).max(magnitude(b.value));
        let ratio = difference / allowance;
        if worst.as_ref().is_none_or(|(worst, _)| ratio > *worst) {
            worst = Some((ratio, a.label.clone()));
        }
        if ratio.is_nan() || ratio > 1.0 {
            failures.push(format!(
                "{} {}: {:?} against {:?}",
                base.analysis, a.label, a.value, b.value
            ));
        }
    }
    Verdict {
        bit_identical,
        worst,
        failures,
    }
}

/// Solve `circuit` as written and after `rewrite`, and require the two to
/// agree to round-off on every analysis.
fn check(circuit: Circuit, rewrite: Rewrite) {
    let base = observe(&circuit, &circuit.deck, &|node| node.to_string());
    let (deck, names) = rewrite.apply(&circuit);
    let renamed = |node: &str| names.get(node).cloned().unwrap_or_else(|| node.to_string());
    let rewritten = observe(&circuit, &deck, &renamed);
    let mut failures = Vec::new();
    for (base, rewritten) in base.iter().zip(&rewritten) {
        let verdict = compare(base, rewritten);
        println!(
            "METAMORPHIC {} {rewrite:?} {}: {verdict}",
            circuit.name, base.analysis
        );
        failures.extend(verdict.failures.into_iter().take(5));
    }
    assert!(
        failures.is_empty(),
        "{} changed under {rewrite:?}:\n{}\nrewritten deck:\n{}",
        circuit.name,
        failures.join("\n"),
        deck.text(None)
    );
}

fn rc() -> Circuit {
    Circuit {
        name: "RC",
        deck: Deck::new(
            &[
                "v1 in 0 dc 1 ac 1 sin(1 0.5 10k)",
                "r1 in out 2k m=2",
                "c1 out 0 10n",
                "r2 out 0 100k",
            ],
            &[],
        ),
        observed: &["in", "out"],
        sources: &["v1"],
        noise: ("out", "v1"),
        tran: (200e-6, 2e-6),
        split: Some(("r1", 2)),
    }
}

fn diode() -> Circuit {
    Circuit {
        name: "diode",
        deck: Deck::new(
            &[
                "v1 in 0 dc 0.8 ac 1 sin(0.8 0.1 1k)",
                "r1 in a 100",
                "d1 a 0 dmod m=3",
                "c1 a 0 100n",
            ],
            &[".model dmod d(is=1e-14 n=1.05 rs=1 cjo=2p kf=1e-16 af=1)"],
        ),
        observed: &["in", "a"],
        sources: &["v1"],
        noise: ("a", "v1"),
        tran: (2e-3, 20e-6),
        split: Some(("d1", 3)),
    }
}

fn bjt() -> Circuit {
    Circuit {
        name: "BJT",
        deck: Deck::new(
            &[
                "vcc cc 0 dc 5",
                "vin in 0 dc 1.2 ac 1 sin(1.2 0.01 1k)",
                "rb in b 100k",
                "rc cc c 2k",
                "q1 c b 0 qmod m=2",
                "cl c 0 10n",
            ],
            &[
                ".model qmod npn(bf=100 is=1e-15 vaf=50 rb=10 re=1 rc=5 cje=1p cjc=0.5p tf=1n kf=1e-15)",
            ],
        ),
        observed: &["cc", "in", "b", "c"],
        sources: &["vcc", "vin"],
        noise: ("c", "vin"),
        tran: (2e-3, 20e-6),
        split: Some(("q1", 2)),
    }
}

fn mos1() -> Circuit {
    Circuit {
        name: "MOS1",
        deck: Deck::new(
            &[
                "vdd dd 0 dc 5",
                "vin g 0 dc 2 ac 1 sin(2 0.1 1k)",
                "rl dd d 10k",
                "m1 d g 0 0 nmod w=10u l=1u m=2",
                "cl d 0 10n",
            ],
            &[
                ".model nmod nmos(level=1 vto=1 kp=100u lambda=0.02 rd=10 rs=10 cgso=1n cgdo=1n cbd=10f cbs=10f kf=1e-24 af=1)",
            ],
        ),
        observed: &["dd", "g", "d"],
        sources: &["vdd", "vin"],
        noise: ("d", "vin"),
        tran: (2e-3, 20e-6),
        split: Some(("m1", 2)),
    }
}

fn bsim4() -> Circuit {
    Circuit {
        name: "BSIM4",
        deck: Deck::new(
            &[
                "vdd dd 0 dc 1",
                "vin g 0 dc 0.8 ac 1 sin(0.8 0.05 1meg)",
                "rl dd d 10k",
                "m1 d g 0 0 n45 w=1u l=45n m=2",
                "cl d 0 1p",
            ],
            &[".model n45 nmos level=54 version=4.8 fnoimod=1 tnoimod=0"],
        ),
        observed: &["dd", "g", "d"],
        sources: &["vdd", "vin"],
        noise: ("d", "vin"),
        tran: (2e-6, 20e-9),
        split: Some(("m1", 2)),
    }
}

/// A deck that is already hierarchical: two RC sections from one subcircuit
/// and a clamp diode, so `Wrap` nests one subcircuit inside another.
fn subcircuit() -> Circuit {
    Circuit {
        name: "subcircuit",
        deck: Deck::new(
            &[
                "v1 in 0 dc 1 ac 1 sin(1 0.5 10k)",
                "x1 in mid section",
                "x2 mid out section",
                "d1 out 0 dmod m=2",
            ],
            &[
                ".subckt section a b\nr1 a b 1k\nc1 b 0 1n\n.ends section",
                ".model dmod d(is=1e-14 n=1.05 rs=1 cjo=2p)",
            ],
        ),
        observed: &["in", "mid", "out"],
        sources: &["v1"],
        noise: ("out", "v1"),
        tran: (200e-6, 2e-6),
        split: Some(("d1", 2)),
    }
}

macro_rules! metamorphic {
    ($($(#[$attribute:meta])* $test:ident: $circuit:ident, $rewrite:ident;)*) => {
        $(
            #[test]
            $(#[$attribute])*
            fn $test() {
                check($circuit(), Rewrite::$rewrite);
            }
        )*
    };
}

metamorphic! {
    rc_reorder: rc, Reorder;
    rc_rename: rc, Rename;
    rc_wrap: rc, Wrap;
    rc_split: rc, Split;
    diode_reorder: diode, Reorder;
    diode_rename: diode, Rename;
    diode_wrap: diode, Wrap;
    diode_split: diode, Split;
    bjt_reorder: bjt, Reorder;
    bjt_rename: bjt, Rename;
    bjt_wrap: bjt, Wrap;
    bjt_split: bjt, Split;
    mos1_reorder: mos1, Reorder;
    mos1_rename: mos1, Rename;
    mos1_wrap: mos1, Wrap;
    mos1_split: mos1, Split;
    bsim4_reorder: bsim4, Reorder;
    bsim4_rename: bsim4, Rename;
    bsim4_wrap: bsim4, Wrap;
    bsim4_split: bsim4, Split;
    subcircuit_reorder: subcircuit, Reorder;
    subcircuit_rename: subcircuit, Rename;
    subcircuit_wrap: subcircuit, Wrap;
    subcircuit_split: subcircuit, Split;
}

/// The rewrites themselves, on text alone.
mod rewrites {
    use super::*;

    #[test]
    fn rename_touches_nodes_only_and_reverses_their_order() {
        let (deck, names) = rename(&Deck::new(
            &[
                "v1 a 0 dc 1 sin(1 0.5 1k)",
                "q1 c b a qmod m=2",
                "x1 a c sub p=1",
            ],
            &[".model qmod npn"],
        ));
        assert_eq!(
            deck.elements,
            [
                "v1 w03 0 dc 1 sin(1 0.5 1k)",
                "q1 w01 w02 w03 qmod m=2",
                "x1 w03 w01 sub p=1"
            ]
        );
        assert_eq!(names["a"], "w03");
        assert_eq!(deck.library, [".model qmod npn"]);
    }

    #[test]
    fn wrap_keeps_sources_and_observed_nodes_outside() {
        let circuit = rc();
        let deck = wrap(&circuit);
        assert_eq!(
            deck.elements,
            [
                "v1 in 0 dc 1 ac 1 sin(1 0.5 10k)",
                "xwrapped in out wrapped"
            ]
        );
        assert_eq!(
            deck.library,
            [
                ".subckt wrapped in out\nr1 in out 2k m=2\nc1 out 0 10n\nr2 out 0 100k\n.ends wrapped"
            ]
        );
    }

    #[test]
    fn split_makes_parallel_copies_without_the_multiplier() {
        let deck = split(&Deck::new(&["d1 a 0 dmod m=3", "r1 a 0 1k"], &[]), "d1", 3);
        assert_eq!(
            deck.elements,
            [
                "d1_1 a 0 dmod",
                "d1_2 a 0 dmod",
                "d1_3 a 0 dmod",
                "r1 a 0 1k"
            ]
        );
    }

    #[test]
    fn a_difference_beyond_round_off_fails_and_an_exact_copy_is_bit_identical() {
        let mut base = Samples::new("op");
        base.real("V(a)".into(), 1.0, VOLT_FLOOR);
        let mut same = Samples::new("op");
        same.real("V(a)".into(), 1.0, VOLT_FLOOR);
        let verdict = compare(&base, &same);
        assert!(verdict.bit_identical && verdict.failures.is_empty());

        let mut close = Samples::new("op");
        close.real("V(a)".into(), 1.0 + 1e-12, VOLT_FLOOR);
        let verdict = compare(&base, &close);
        assert!(!verdict.bit_identical && verdict.failures.is_empty());

        let mut far = Samples::new("op");
        far.real("V(a)".into(), 1.0 + 1e-6, VOLT_FLOOR);
        assert_eq!(compare(&base, &far).failures.len(), 1);
    }
}
