//! Oracles for `.DCMATCH`.
//!
//! Each expectation is algebra computed in the test from the deck it just
//! ran, not a recorded number: a resistor divider whose variance is an exact
//! function of its two resistances, a series pair that separates independent
//! mismatch from perfectly correlated process spread, and a differential pair
//! whose output-referred offset is the Pelgrom coefficient times the gain the
//! operating point reports.

use super::*;
use crate::engine::SimulationConfig;
use crate::netlist::{SpectreDistribution, SpectreSpread, SpectreStatisticsPlan, SpectreVariation};

/// One `vary` declaration, as the Spectre adapter would have lowered it.
fn variation(
    line: usize,
    scope: SpectreVariationScope,
    parameter: &str,
    std: &str,
) -> SpectreVariation {
    SpectreVariation {
        line,
        scope,
        parameter: parameter.to_owned(),
        distribution: SpectreDistribution::Gaussian,
        spread: SpectreSpread::StandardDeviation(std.to_owned()),
        percent: false,
    }
}

fn plan(variations: Vec<SpectreVariation>) -> SpectreStatisticsPlan {
    SpectreStatisticsPlan {
        variations,
        correlations: Vec::new(),
    }
}

/// Splice a lowered statistics plan into a deck the way the include
/// expander does, and parse it.
fn deck(plan: &SpectreStatisticsPlan, body: &str) -> Netlist {
    Netlist::parse(&format!(
        "DCMATCH fixture\n.RSPICE_SPECTRE_STAT {}\n{body}\n.end\n",
        plan.encode_internal()
    ))
    .expect("statistical deck parses")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

const DIVIDER_SOURCE: Value = 1.0;
const DIVIDER_R1: Value = 1.0e3;
const DIVIDER_R2: Value = 2.0e3;
const DIVIDER_SIGMA: Value = 10.0;

/// `V1 -- R1 -- out -- R2 -- 0`, both resistances varied independently.
fn divider() -> Netlist {
    deck(
        &plan(vec![
            variation(2, SpectreVariationScope::Mismatch, "r1v", "10"),
            variation(2, SpectreVariationScope::Mismatch, "r2v", "10"),
        ]),
        ".param r1v=1k r2v=2k\nV1 in 0 1\nR1 in out {r1v}\nR2 out 0 {r2v}",
    )
}

fn all_contributors(card: &DcMatchCard) -> DcMatchCard {
    DcMatchCard {
        contributor_limit: 0,
        ..card.clone()
    }
}

#[test]
fn a_dcmatch_sigma_matches_the_analytic_resistor_divider() {
    let total = DIVIDER_R1 + DIVIDER_R2;
    // dV(out)/dR1 = -Vs*R2/(R1+R2)^2 and dV(out)/dR2 = Vs*R1/(R1+R2)^2.
    let from_r1 = DIVIDER_SOURCE * DIVIDER_R2 / (total * total) * DIVIDER_SIGMA;
    let from_r2 = DIVIDER_SOURCE * DIVIDER_R1 / (total * total) * DIVIDER_SIGMA;
    let expected = libm::sqrt(from_r1 * from_r1 + from_r2 * from_r2);

    let result = engine()
        .run_dc_match(
            &divider(),
            &all_contributors(&DcMatchCard::voltage_probe("OUT")),
        )
        .expect("mismatch variance solves");

    assert_eq!(result.output, "V(OUT)");
    assert!(
        (result.nominal_value - DIVIDER_SOURCE * DIVIDER_R2 / total).abs() < 1e-12,
        "nominal {}",
        result.nominal_value
    );
    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 1.0e-3,
        "sigma {} vs analytic {expected} (relative error {error})",
        result.sigma_total
    );
    assert_eq!(result.sigma_mismatch, result.sigma_total);
    assert_eq!(result.sigma_process, 0.0);

    // Each resistance is varied only by its own parameter, so the two
    // cross terms are identically zero rather than small.
    let effective = |instance: &str, parameter: &str| {
        result
            .contributors
            .iter()
            .find(|entry| entry.instance == instance && entry.parameter == parameter)
            .unwrap_or_else(|| panic!("{instance}/{parameter} is a contributor"))
            .clone()
    };
    assert_eq!(effective("R1", "R2V").sensitivity, 0.0);
    assert_eq!(effective("R2", "R1V").sensitivity, 0.0);
    assert!(
        (effective("R1", "R1V").contribution.abs() - from_r1).abs() / from_r1 < 1.0e-3,
        "R1 contribution {}",
        effective("R1", "R1V").contribution
    );
    assert!(
        (effective("R2", "R2V").contribution.abs() - from_r2).abs() / from_r2 < 1.0e-3,
        "R2 contribution {}",
        effective("R2", "R2V").contribution
    );
}

#[test]
fn a_dcmatch_sigma_matches_a_monte_carlo_population_on_a_resistor_divider() {
    const RUNS: usize = 2000;
    let netlist = divider();
    let linearized = engine()
        .run_dc_match(&netlist, &DcMatchCard::voltage_probe("OUT"))
        .expect("mismatch variance solves");

    // The same statistics block, sampled by the existing Monte Carlo
    // driver: nothing about the deck changes, only how the variation is
    // consumed. The generic distribution argument is inert here, because a
    // deck carrying native Spectre statistics varies exactly what that
    // block declares.
    let mut config = SimulationConfig::default();
    config.resource_limits.max_parallel_workers = 4;
    let population = Engine::new(config)
        .run_monte_carlo(&netlist, RUNS, 20_260_916)
        .expect("Monte Carlo population solves");
    assert_eq!(population.num_failures, 0);
    let sampled = population
        .std_dev("V(OUT)")
        .expect("the population reports V(OUT)");

    let error = (sampled - linearized.sigma_total).abs() / linearized.sigma_total;
    assert!(
        error < 0.05,
        "population sigma {sampled} vs linearized {} (relative error {error})",
        linearized.sigma_total
    );
}

#[test]
fn contributors_sum_to_the_total_variance() {
    let netlist = divider();
    let probe = DcMatchCard::voltage_probe("OUT");
    let complete = engine()
        .run_dc_match(&netlist, &all_contributors(&probe))
        .expect("mismatch variance solves");
    // Every (instance, variable) pair is evaluated: three instances
    // (`V1`, `R1`, `R2`) times two declared mismatch variables. A pair the
    // instance cannot read is reported with a measured zero rather than
    // guessed at from the instance's expressions.
    assert_eq!(complete.evaluated_contributors, 6);
    assert_eq!(complete.contributors.len(), 6);
    let sum: Value = complete.contributors.iter().map(|entry| entry.share).sum();
    assert!((sum - 1.0).abs() < 1.0e-9, "shares sum to {sum}");
    // Ranked, so the first entry is the largest share.
    assert!(complete.contributors[0].share > complete.contributors[1].share);

    let largest = engine()
        .run_dc_match(
            &netlist,
            &DcMatchCard {
                contributor_limit: 1,
                ..probe.clone()
            },
        )
        .expect("mismatch variance solves");
    assert_eq!(largest.contributors.len(), 1);
    assert_eq!(largest.evaluated_contributors, 6);
    assert_eq!(largest.contributors[0], complete.contributors[0]);
    assert_eq!(largest.sigma_total, complete.sigma_total);

    let above_half = engine()
        .run_dc_match(
            &netlist,
            &DcMatchCard {
                contributor_limit: 0,
                threshold: 0.5,
                ..probe
            },
        )
        .expect("mismatch variance solves");
    assert_eq!(above_half.contributors.len(), 1);
    assert!(above_half.contributors[0].share > 0.5);
    assert_eq!(above_half.sigma_total, complete.sigma_total);
}

#[test]
fn process_spreads_are_perfectly_correlated_across_instances() {
    const CURRENT: Value = 1.0e-3;
    const SIGMA: Value = 10.0;
    let body = ".param rv=1k\nI1 0 top DC 1m\nR1 top mid {rv}\nR2 mid 0 {rv}";

    let independent = engine()
        .run_dc_match(
            &deck(
                &plan(vec![variation(
                    2,
                    SpectreVariationScope::Mismatch,
                    "rv",
                    "10",
                )]),
                body,
            ),
            &all_contributors(&DcMatchCard::voltage_probe("TOP")),
        )
        .expect("mismatch variance solves");
    let expected_mismatch = libm::sqrt(2.0) * CURRENT * SIGMA;
    assert!(
        (independent.sigma_total - expected_mismatch).abs() / expected_mismatch < 1.0e-6,
        "mismatch sigma {} vs sqrt(2)*I*sigma {expected_mismatch}",
        independent.sigma_total
    );

    let correlated = engine()
        .run_dc_match(
            &deck(
                &plan(vec![variation(
                    2,
                    SpectreVariationScope::Process,
                    "rv",
                    "10",
                )]),
                body,
            ),
            &DcMatchCard {
                mismatch: false,
                process: true,
                contributor_limit: 0,
                ..DcMatchCard::voltage_probe("TOP")
            },
        )
        .expect("process variance solves");
    let expected_process = 2.0 * CURRENT * SIGMA;
    assert!(
        (correlated.sigma_total - expected_process).abs() / expected_process < 1.0e-6,
        "process sigma {} vs 2*I*sigma {expected_process}",
        correlated.sigma_total
    );
    assert_eq!(correlated.sigma_mismatch, 0.0);
    assert_eq!(correlated.sigma_process, correlated.sigma_total);
    // One design-wide variable, so one contributor owning all of it.
    assert_eq!(correlated.contributors.len(), 1);
    assert_eq!(correlated.contributors[0].instance, PROCESS_OWNER);
    assert_eq!(correlated.contributors[0].scope, DcMatchScope::Process);
}

/// Resistor-loaded NMOS differential pair with a Pelgrom threshold
/// spread. `A / sqrt(W*L)` is authored in micron-valued parameters, which
/// is the unit the coefficient is quoted in.
fn differential_pair() -> Netlist {
    deck(
        &plan(vec![variation(
            2,
            SpectreVariationScope::Mismatch,
            "dvth",
            "avt/sqrt(wum*lum)",
        )]),
        ".param avt=10m wum=10 lum=1 dvth=0 vth0=0.7\n\
         .param w={wum*1e-6} l={lum*1e-6}\n\
         .model nch nmos level=1 vto={vth0+dvth} kp=100u lambda=0 gamma=0\n\
         VDD vdd 0 3\n\
         VG g 0 1.5\n\
         M1 d1 g s 0 nch W={w} L={l}\n\
         M2 d2 g s 0 nch W={w} L={l}\n\
         RL1 vdd d1 20k\n\
         RL2 vdd d2 20k\n\
         RSS s 0 10k",
    )
}

#[test]
fn a_differential_pair_offset_matches_pelgrom() {
    let netlist = differential_pair();
    // sigma(Vth) = A / sqrt(W*L), the authored spread.
    let sigma_vth = 10.0e-3 / libm::sqrt(10.0);

    // Independent oracle for the gain: the transconductance the converged
    // operating point reports, times the load. A single device's threshold
    // shift moves V(d1,d2) by exactly gm*RL whatever the tail impedance,
    // so two independent devices give sqrt(2)*gm*RL*sigma(Vth).
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("the pair biases");
    let gm = |name: &str| {
        report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .and_then(|entry| {
                entry
                    .params
                    .iter()
                    .find(|(label, _)| *label == "gm")
                    .map(|(_, value)| *value)
            })
            .unwrap_or_else(|| panic!("{name} reports gm"))
    };
    let gm1 = gm("M1");
    assert!(
        (gm1 - gm("M2")).abs() / gm1 < 1.0e-9,
        "the nominal pair is balanced"
    );
    let expected = libm::sqrt(2.0) * gm1 * 20.0e3 * sigma_vth;

    let result = engine()
        .run_dc_match(
            &netlist,
            &DcMatchCard {
                reference_node: Some("D2".to_owned()),
                contributor_limit: 0,
                ..DcMatchCard::voltage_probe("D1")
            },
        )
        .expect("mismatch variance solves");

    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 0.03,
        "sigma {} vs Pelgrom gain*sqrt(2)*A/sqrt(w*l) {expected} (relative error {error})",
        result.sigma_total
    );

    let top_two = &result.contributors[..2];
    let mut owners = top_two
        .iter()
        .map(|entry| entry.instance.as_str())
        .collect::<Vec<_>>();
    owners.sort_unstable();
    assert_eq!(owners, ["M1", "M2"]);
    assert!(
        (top_two[0].share - 0.5).abs() < 1.0e-6 && (top_two[1].share - 0.5).abs() < 1.0e-6,
        "the matched devices own equal shares: {:?}",
        top_two.iter().map(|entry| entry.share).collect::<Vec<_>>()
    );
    for entry in &result.contributors[2..] {
        assert_eq!(entry.share, 0.0, "{} owns no variance", entry.instance);
    }
}

#[test]
fn a_design_without_statistics_is_refused_with_the_remedy() {
    let netlist = Netlist::parse("plain divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.end\n")
        .expect("deck parses");
    let error = engine()
        .run_dc_match(&netlist, &DcMatchCard::voltage_probe("OUT"))
        .expect_err("a design with no statistics has no mismatch to report");
    let message = error.to_string();
    assert!(
        message.contains("statistics { mismatch { vary ... } }")
            && message.contains("none is bound to this design"),
        "{message}"
    );
}

#[test]
fn a_scope_the_design_does_not_declare_is_refused_by_name() {
    let netlist = deck(
        &plan(vec![variation(
            2,
            SpectreVariationScope::Mismatch,
            "rv",
            "10",
        )]),
        ".param rv=1k\nV1 in 0 1\nR1 in out {rv}\nR2 out 0 2k",
    );
    let error = engine()
        .run_dc_match(
            &netlist,
            &DcMatchCard {
                mismatch: false,
                process: true,
                ..DcMatchCard::voltage_probe("OUT")
            },
        )
        .expect_err("the design declares no process variation");
    let message = error.to_string();
    assert!(
        message.contains("nothing to vary") && message.contains("the process scope"),
        "{message}"
    );
}

#[test]
fn a_probe_the_design_does_not_have_is_refused() {
    let error = engine()
        .run_dc_match(&divider(), &DcMatchCard::voltage_probe("nowhere"))
        .expect_err("an absent probe cannot be measured");
    assert!(
        error.to_string().contains(".DCMATCH output"),
        "{}",
        error.to_string()
    );
}

#[test]
fn a_dcmatch_result_quotes_the_requested_sigma_multiple() {
    let result = engine()
        .run_dc_match(
            &divider(),
            &DcMatchCard {
                sigma_multiplier: 3.0,
                ..DcMatchCard::voltage_probe("OUT")
            },
        )
        .expect("mismatch variance solves");
    assert_eq!(result.sigma_multiplier, 3.0);
    assert_eq!(result.quoted_sigma(), 3.0 * result.sigma_total);
}

/// One hundred transistor cells, three mismatch variables each: the
/// threshold and gain of the device and the value of its source
/// resistance. Every cell is a subcircuit instance, which is the mismatch
/// scope a PDK device occupies, so the design has exactly one hundred
/// device scopes.
fn hundred_cell_array() -> Netlist {
    let mut body = String::from(
        ".param avt=10m abeta=2 wum=10 lum=1 rs=100k\n\
         .param dvth=0 dbeta=0 dr=0 vth0=0.7 kpnom=100u\n\
         .param w={wum*1e-6} l={lum*1e-6}\n\
         .model nch nmos level=1 vto={vth0+dvth} kp={kpnom*(1+dbeta)} lambda=0 gamma=0\n\
         .subckt cell out vdd g\n\
         M1 vdd g out 0 nch W={w} L={l}\n\
         RS out 0 {rs*(1+dr)}\n\
         .ends\n\
         VDD vdd 0 3\n\
         VG g 0 1.5",
    );
    for index in 1..=100 {
        body.push_str(&format!("\nX{index} out vdd g cell"));
    }
    deck(
        &plan(vec![
            variation(
                2,
                SpectreVariationScope::Mismatch,
                "dvth",
                "avt/sqrt(wum*lum)",
            ),
            variation(
                2,
                SpectreVariationScope::Mismatch,
                "dbeta",
                "abeta/sqrt(wum*lum)/100",
            ),
            variation(2, SpectreVariationScope::Mismatch, "dr", "0.01"),
        ]),
        &body,
    )
}

/// The cost model, pinned by the count rather than by a wall clock: two
/// warm operating points for every `(instance, variable)` pair, over
/// every mismatch scope the design has. A hundred device cells plus the
/// two top-level sources is 102 scopes; three variables each is 306
/// pairs, so 612 displaced solves.
#[test]
fn a_hundred_cell_array_evaluates_one_pair_per_scope_and_variable() {
    let result = engine()
        .run_dc_match(
            &hundred_cell_array(),
            &all_contributors(&DcMatchCard::voltage_probe("OUT")),
        )
        .expect("the array's mismatch variance solves");
    assert_eq!(result.evaluated_contributors, 306);
    assert!(result.sigma_total > 0.0, "the array has a spread");
    let shares: Value = result.contributors.iter().map(|entry| entry.share).sum();
    assert!((shares - 1.0).abs() < 1.0e-9, "shares sum to {shares}");
    // The hundred matched cells own it all; the two sources own none.
    let owning = result
        .contributors
        .iter()
        .filter(|entry| entry.share > 0.0)
        .count();
    assert_eq!(owning, 300);
}

#[test]
fn a_displaced_operating_point_warm_starts_from_the_nominal_solution() {
    let netlist = differential_pair();
    let engine = engine();
    let nominal = engine
        .run_dc_op_state_with_startup_and_abort(
            &netlist,
            DcOpStartup::Automatic { use_hints: true },
            &NoAbort,
        )
        .expect("the pair biases");
    let cold = engine.convergence_quality().total_iterations;

    let mut displaced = netlist.clone();
    displaced.spectre_mismatch_override =
        Some(SpectreMismatchOverride::single("M1", "DVTH", 3.162e-3));
    engine
        .run_dc_op_state_with_startup_and_abort(
            &displaced,
            DcOpStartup::PreviousSolution(&nominal.solution),
            &NoAbort,
        )
        .expect("the displaced pair re-solves");
    let warm = engine.convergence_quality().total_iterations;

    assert!(
        warm <= 3,
        "a one-sigma displacement should resume from the nominal answer, \
         took {warm} Newton assemblies (cold took {cold})"
    );
    assert!(warm < cold, "warm {warm} vs cold {cold}");
}
