//! `.DCMATCH` against closed-form mismatch variance.
//!
//! Two designs whose DC mismatch spread can be written down: a resistor
//! divider, where the variance is an exact function of the two resistances,
//! and a Pelgrom-limited differential pair, where the output-referred spread
//! is the gain times the root-two-scaled threshold spread of one device.
//!
//! These are oracles, not goldens. Nothing here is compared against a
//! recorded number: each assertion is the analytic result computed in the
//! test from the deck's own declared values, so a regression in the
//! sensitivity path, the variance sum, or the statistical plan shows up as
//! disagreement with algebra rather than as a diff against a captured file.
//!
//! Both decks travel the path a user's deck takes: a Spectre `.scs` library
//! carrying the `statistics` block, pulled in by `.include`, lowered by the
//! include expander, with the card authored as `.DCMATCH` in the deck.

use rspice_core::analysis::dcmatch::DcMatchScope;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, DcMatchCard, Netlist};
use std::fs;
use std::path::Path;

/// Write `library` beside a deck and parse the deck with its own path, so
/// `.include` resolves and the Spectre adapter sees a `.scs` source.
fn deck_with_spectre_library(directory: &Path, library: &str, deck: &str) -> Netlist {
    let library_path = directory.join("statistics.scs");
    fs::write(&library_path, library).expect("Spectre statistics library is written");
    Netlist::parse_with_path(deck, &directory.join("design.cir"))
        .expect("the deck and its Spectre statistics library parse")
}

/// The single `.DCMATCH` card the deck authored.
fn dcmatch_card(netlist: &Netlist) -> DcMatchCard {
    let cards = netlist
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            AnalysisCommand::DcMatch(card) => Some((**card).clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(cards.len(), 1, "the deck authors exactly one .DCMATCH card");
    cards.into_iter().next().expect("one card")
}

const DIVIDER_LIBRARY: &str = "\
// Resistor divider mismatch, declared the way a PDK declares it.
parameters r1v=1000 r2v=2000
statistics {
 mismatch {
  vary r1v dist=gauss std=10
  vary r2v dist=gauss std=10
 }
}
";

const DIVIDER_DECK: &str = "\
Resistor divider DC mismatch
.include \"statistics.scs\"
V1 in 0 1
R1 in out {r1v}
R2 out 0 {r2v}
.DCMATCH OUT=V(out) CONTRIBUTORS=0 SIGMA=3
.end
";

#[test]
fn dcmatch_reproduces_the_analytic_resistor_divider_variance() {
    let directory = tempfile::tempdir().expect("scratch directory");
    let netlist = deck_with_spectre_library(directory.path(), DIVIDER_LIBRARY, DIVIDER_DECK);
    assert_eq!(
        netlist.spectre_statistics.variations.len(),
        2,
        "both mismatch variations reach the executable plan"
    );

    let card = dcmatch_card(&netlist);
    assert_eq!(card.sigma_multiplier, 3.0);
    assert_eq!(card.contributor_limit, 0);

    let result = Engine::new(SimulationConfig::default())
        .run_dc_match(&netlist, &card)
        .expect("the divider's mismatch variance solves");

    // V(out) = Vs*R2/(R1+R2); d/dR1 = -Vs*R2/(R1+R2)^2, d/dR2 = Vs*R1/(R1+R2)^2.
    let (source, r1, r2, sigma) = (1.0_f64, 1.0e3_f64, 2.0e3_f64, 10.0_f64);
    let sum = r1 + r2;
    let from_r1 = source * r2 / (sum * sum) * sigma;
    let from_r2 = source * r1 / (sum * sum) * sigma;
    let expected = (from_r1 * from_r1 + from_r2 * from_r2).sqrt();

    assert!(
        (result.nominal_value - source * r2 / sum).abs() < 1.0e-12,
        "nominal V(out) is {}",
        result.nominal_value
    );
    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 1.0e-3,
        "sigma(V(out)) is {} against the analytic {expected} (relative error {error})",
        result.sigma_total
    );
    assert_eq!(
        result.sigma_process, 0.0,
        "the deck declares no process spread"
    );
    assert_eq!(result.quoted_sigma(), 3.0 * result.sigma_total);

    // Six (instance, variable) pairs — three instances times two declared
    // variables — of which only two carry variance, and the shares account
    // for all of it.
    assert_eq!(result.evaluated_contributors, 6);
    let shares: f64 = result.contributors.iter().map(|entry| entry.share).sum();
    assert!((shares - 1.0).abs() < 1.0e-9, "shares sum to {shares}");
    assert!(
        result
            .contributors
            .iter()
            .all(|entry| entry.scope == DcMatchScope::Mismatch)
    );
    let largest = &result.contributors[0];
    assert_eq!(largest.instance, "R1");
    assert_eq!(largest.parameter, "R1V");
}

const PAIR_LIBRARY: &str = "\
// Pelgrom threshold mismatch: A_VT over the square root of the drawn area,
// with the area in the micron-valued units the coefficient is quoted in.
parameters avt=10m wum=10 lum=1 dvth=0 vth0=0.7
statistics {
 mismatch {
  vary dvth dist=gauss std=avt/sqrt(wum*lum)
 }
}
";

const PAIR_DECK: &str = "\
Resistor-loaded NMOS differential pair, Pelgrom threshold mismatch
.include \"statistics.scs\"
.param w={wum*1e-6} l={lum*1e-6}
.model nch nmos level=1 vto={vth0+dvth} kp=100u lambda=0 gamma=0
VDD vdd 0 3
VG g 0 1.5
M1 d1 g s 0 nch W={w} L={l}
M2 d2 g s 0 nch W={w} L={l}
RL1 vdd d1 20k
RL2 vdd d2 20k
RSS s 0 10k
.DCMATCH OUT=V(d1,d2) CONTRIBUTORS=2
.end
";

#[test]
fn dcmatch_reproduces_the_pelgrom_differential_pair_offset() {
    let directory = tempfile::tempdir().expect("scratch directory");
    let netlist = deck_with_spectre_library(directory.path(), PAIR_LIBRARY, PAIR_DECK);
    let engine = Engine::new(SimulationConfig::default());

    // Independent oracle for the gain: the transconductance the converged
    // operating point reports, times the load resistance. One device's
    // threshold shift moves V(d1,d2) by exactly gm*RL whatever the tail
    // impedance is, so two independent devices give sqrt(2)*gm*RL*sigma(Vth).
    let (_, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("the pair biases");
    let transconductance = |device: &str| {
        report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(device))
            .and_then(|entry| {
                entry
                    .params
                    .iter()
                    .find(|(label, _)| *label == "gm")
                    .map(|(_, value)| *value)
            })
            .unwrap_or_else(|| panic!("{device} reports gm at the operating point"))
    };
    let gm = transconductance("M1");
    assert!(
        (gm - transconductance("M2")).abs() / gm < 1.0e-9,
        "the nominal pair is balanced"
    );
    assert!(gm > 0.0, "the pair conducts");

    let sigma_threshold = 10.0e-3 / 10.0_f64.sqrt();
    let expected = 2.0_f64.sqrt() * gm * 20.0e3 * sigma_threshold;

    let result = engine
        .run_dc_match(&netlist, &dcmatch_card(&netlist))
        .expect("the pair's mismatch variance solves");

    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 0.03,
        "output-referred sigma is {} against gain*sqrt(2)*A/sqrt(W*L) = {expected} \
         (relative error {error})",
        result.sigma_total
    );

    // The card asked for two contributors, and the two transistors are the
    // ones that own the variance, in equal parts.
    assert_eq!(result.contributors.len(), 2);
    let mut owners = result
        .contributors
        .iter()
        .map(|entry| entry.instance.as_str())
        .collect::<Vec<_>>();
    owners.sort_unstable();
    assert_eq!(owners, ["M1", "M2"]);
    for entry in &result.contributors {
        assert_eq!(entry.parameter, "DVTH");
        assert!(
            (entry.share - 0.5).abs() < 1.0e-6,
            "{} owns share {}",
            entry.instance,
            entry.share
        );
    }
    assert!(
        result.evaluated_contributors > result.contributors.len(),
        "the passive instances were evaluated and found to own no variance"
    );
}

#[test]
fn a_design_without_a_statistics_block_is_refused_with_the_remedy() {
    let netlist = Netlist::parse(
        "Divider with no declared statistics\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         R2 out 0 2k\n\
         .DCMATCH OUT=V(out)\n\
         .end\n",
    )
    .expect("the deck parses; the card is refused at run time, not at parse time");

    let message = Engine::new(SimulationConfig::default())
        .run_dc_match(&netlist, &dcmatch_card(&netlist))
        .expect_err("there is no declared spread to report")
        .to_string();
    assert!(
        message.contains("statistics { mismatch { vary ... } }")
            && message.contains("none is bound to this design"),
        "{message}"
    );
}
