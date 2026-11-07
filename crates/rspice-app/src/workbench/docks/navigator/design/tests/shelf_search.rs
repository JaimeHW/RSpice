//! The Component shelf's ranked search: what a reader typing a query gets
//! first, and what they never get at all.

use super::super::shelf::search::{MatchTier, ShelfQuery, ShelfTerms};
use super::super::shelf::*;
use super::*;

fn hits(app: &RSpiceApp, raw: &str) -> Vec<ShelfHit> {
    let query = ShelfQuery::new(raw);
    let parts = library_part_rows(app, &query);
    let cells = cell_candidates(&app.state);
    shelf_hits(&app.state, &query, &parts, &cells)
}

/// The queries a reader types for the parts they mean, each with the rows
/// the first answer may be.
const TOP_HITS: &[(&str, &[&str])] = &[
    ("resistor", &["Resistor"]),
    ("res", &["Resistor"]),
    ("r", &["Resistor"]),
    ("cap", &["Capacitor"]),
    ("coil", &["Inductor"]),
    ("op amp", &["Ideal Op-Amp"]),
    ("opamp", &["Ideal Op-Amp"]),
    ("OpAmp", &["Ideal Op-Amp"]),
    ("amplifier", &["Ideal Op-Amp"]),
    ("mosfet", &["NMOS", "PMOS"]),
    ("nfet", &["NMOS"]),
    ("pfet", &["PMOS"]),
    ("transistor", &["NMOS", "PMOS", "NPN BJT", "PNP BJT"]),
    ("bjt", &["NPN BJT", "PNP BJT"]),
    ("pnp", &["PNP BJT"]),
    ("jfet", &["N-JFET", "P-JFET"]),
    ("gnd", &["Ground"]),
    ("0", &["Ground"]),
    ("laplace", &["S-Domain Transfer Function"]),
    ("transfer function", &["S-Domain Transfer Function"]),
    ("schmitt", &["Hysteresis"]),
    ("comparator", &["Hysteresis"]),
    ("bsource", &["Behavioral Source"]),
    ("expression", &["Behavioral Source"]),
    ("pot", &["Potentiometer"]),
    ("vcvs", &["VCVS (E)"]),
    ("switch", &["V-Switch (S)"]),
    ("s/h", &["Sample and Hold"]),
    ("microstrip", &["Microstrip Line"]),
    ("flip flop", &["D Flip-Flop (event-driven)"]),
    ("and gate", &["AND Gate (event-driven)"]),
    ("adc", &["ADC Bridge"]),
    ("sine", &["Sinusoidal Voltage Source"]),
    ("transmission line", &["Transmission Line"]),
    ("pin", &["Port"]),
    ("led", &["LED (red)"]),
    ("light emitting diode", &["LED (red)"]),
    ("zener", &["Zener Diode"]),
    ("schottky", &["Schottky Diode"]),
    ("battery", &["Battery"]),
    ("earth", &["Earth Ground"]),
    ("chassis", &["Chassis Ground"]),
    ("electrolytic", &["Polarized Capacitor"]),
    ("polarized capacitor", &["Polarized Capacitor"]),
    ("rspice_zener", &["Zener Diode"]),
    ("rspice_led_green", &["LED (green)"]),
    ("vdd", &["VDD"]),
    ("vcc", &["VCC"]),
    ("vss", &["VSS"]),
    ("vee", &["VEE"]),
    ("rail", &["VDD", "VCC", "VSS", "VEE"]),
];

#[test]
fn every_query_puts_the_part_it_names_first() {
    assert!(TOP_HITS.len() >= 30);
    let app = RSpiceApp::test_instance();
    let failures = TOP_HITS
        .iter()
        .filter_map(|(query, expected)| {
            let found = hits(&app, query);
            let top = top_hit(&found).map(|hit| hit.label.as_str());
            (!top.is_some_and(|top| expected.contains(&top)))
                .then(|| format!("  {query:?} -> {top:?}, expected one of {expected:?}"))
        })
        .collect::<Vec<_>>();
    assert!(
        failures.is_empty(),
        "queries whose first answer is wrong:\n{}",
        failures.join("\n")
    );
}

#[test]
fn a_word_matches_from_its_start_and_never_from_inside() {
    let app = RSpiceApp::test_instance();
    for hit in hits(&app, "led") {
        let label = hit.label.to_lowercase();
        assert!(
            !label.contains("coupled") && !label.contains("controlled"),
            "`led` reached {:?} from inside a word",
            hit.label
        );
    }
    let led = ShelfQuery::new("led");
    let words = |label| ShelfTerms {
        label,
        keywords: &[],
        part_numbers: &[],
    };
    assert!(led.rank(&words("Voltage-Controlled Switch")).is_none());
    assert!(led.rank(&words("LED")).is_some());
}

#[test]
fn exact_beats_a_word_prefix_beats_a_part_number() {
    let tier = |query: &str, terms: ShelfTerms<'_>| {
        ShelfQuery::new(query).rank(&terms).map(|rank| rank.tier)
    };
    let row = ShelfTerms {
        label: "Zener Diode",
        keywords: &["avalanche"],
        part_numbers: &["RSPICE_ZENER"],
    };
    assert_eq!(tier("zener diode", row), Some(MatchTier::Exact));
    let row = ShelfTerms {
        label: "Zener Diode",
        keywords: &["avalanche"],
        part_numbers: &["RSPICE_ZENER"],
    };
    assert_eq!(tier("diode", row), Some(MatchTier::WordPrefix));
    let part = || ShelfTerms {
        label: "",
        keywords: &[],
        part_numbers: &["D1N4148"],
    };
    assert_eq!(tier("d1n", part()), Some(MatchTier::PartNumberPrefix));
    assert_eq!(tier("4148", part()), Some(MatchTier::PartNumberSubstring));
    // Separators fold away on both sides.
    assert_eq!(tier("d-1n_4148", part()), Some(MatchTier::PartNumberPrefix));
}

#[test]
fn a_query_nothing_answers_lists_nothing() {
    let app = RSpiceApp::test_instance();
    assert!(!hits(&app, "resistor").is_empty());
    assert!(hits(&app, "no-such-component-or-cell").is_empty());
}

/// A library card a variant row places is answered by that row alone.
#[test]
fn a_card_a_variant_row_places_is_listed_once() {
    let app = RSpiceApp::test_instance();
    let labels = hits(&app, "led")
        .into_iter()
        .map(|hit| hit.label)
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        ["LED (red)", "LED (green)", "LED (blue)", "LED (white)"]
    );
}
