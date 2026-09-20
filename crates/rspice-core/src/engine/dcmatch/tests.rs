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
use crate::netlist::{
    SpectreCorrelation, SpectreCorrelationMatrix, SpectreDistribution, SpectreSpread,
    SpectreStatisticsPlan, SpectreVariation,
};

/// One `vary` declaration, as the Spectre adapter would have lowered it.
fn variation(
    line: usize,
    scope: SpectreVariationScope,
    parameter: &str,
    std: &str,
) -> SpectreVariation {
    SpectreVariation {
        bounds: None,
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

/// One `correlate param=[...] cc=...` statement, as the Spectre adapter would
/// have lowered it.
fn correlate(
    scope: SpectreVariationScope,
    parameters: &[&str],
    coefficient: &str,
) -> SpectreCorrelation {
    SpectreCorrelation {
        line: 2,
        scope,
        parameters: parameters.iter().map(|name| (*name).to_owned()).collect(),
        coefficient: coefficient.to_owned(),
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
    // An empty scope is a spread of zero, not of negative zero: the two compare
    // equal, but every published artifact carries these bits and "-0" reads as
    // a defect.
    assert!(
        result.sigma_process.is_sign_positive(),
        "an empty scope must report +0.0"
    );

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

//=============================================================================
// Declared correlations
//=============================================================================

/// The signed one-sigma contributions of the divider's two resistances,
/// exactly as the engine forms them.
///
/// These are the *central differences at step sigma*, not the continuum
/// derivatives: for `V(out) = Vs*R2/(R1+R2)` the difference quotient is
/// `-Vs*R2/((R1+R2)^2 - sigma^2)` rather than `-Vs*R2/(R1+R2)^2`, and at the
/// 1% spread these fixtures declare the two differ by one part in 1e5 — a
/// hundred times the tolerance the correlated cases assert. The signs are
/// opposite and fixed by the topology: `V(out)` falls when `R1` rises and
/// rises when `R2` rises.
fn divider_contributions(r1: Value, r2: Value, sigma: Value) -> (Value, Value) {
    let sum = r1 + r2;
    let span = sum * sum - sigma * sigma;
    (
        -DIVIDER_SOURCE * r2 * sigma / span,
        DIVIDER_SOURCE * r1 * sigma / span,
    )
}

/// The divider again, with both resistances varied by design-wide `process`
/// variables that the deck declares correlated.
fn correlated_process_divider(r2: Value, coefficient: &str) -> Netlist {
    deck(
        &SpectreStatisticsPlan {
            variations: vec![
                variation(2, SpectreVariationScope::Process, "r1v", "10"),
                variation(2, SpectreVariationScope::Process, "r2v", "10"),
            ],
            correlations: vec![correlate(
                SpectreVariationScope::Process,
                &["r1v", "r2v"],
                coefficient,
            )],
        },
        &format!(
            ".param r1v={DIVIDER_R1} r2v={r2}\nV1 in 0 {DIVIDER_SOURCE}\nR1 in out {{r1v}}\n\
             R2 out 0 {{r2v}}"
        ),
    )
}

/// A card that reads the design-wide process scope and retains everything.
fn process_card() -> DcMatchCard {
    DcMatchCard {
        mismatch: false,
        process: true,
        contributor_limit: 0,
        ..DcMatchCard::voltage_probe("OUT")
    }
}

#[test]
fn a_declared_process_correlation_enters_the_variance_exactly() {
    let (from_r1, from_r2) = divider_contributions(DIVIDER_R1, DIVIDER_R2, DIVIDER_SIGMA);
    assert!(from_r1 < 0.0 && from_r2 > 0.0, "the two signs are opposite");
    for coefficient in [-1.0, -0.5, 0.0, 0.5, 1.0] {
        let result = engine()
            .run_dc_match(
                &correlated_process_divider(DIVIDER_R2, &format!("{coefficient}")),
                &process_card(),
            )
            .expect("the correlated process variance solves");
        // sigma^2 = c1^2 + c2^2 + 2*rho*c1*c2, the quadratic form written out
        // for two variables.
        let expected = libm::sqrt(
            from_r1 * from_r1 + from_r2 * from_r2 + 2.0 * coefficient * from_r1 * from_r2,
        );
        let error = (result.sigma_total - expected).abs() / expected;
        assert!(
            error < 1.0e-6,
            "cc={coefficient}: sigma {} vs analytic {expected} (relative error {error})",
            result.sigma_total
        );
        assert_eq!(result.sigma_process, result.sigma_total);
        assert_eq!(result.sigma_mismatch, 0.0);
        assert_eq!(result.applied_correlations_process, 1);
        assert_eq!(result.applied_correlations_mismatch, 0);
    }

    // Equal resistances make the two contributions equal and opposite, so a
    // unit coefficient cancels the variance outright: `c1 + c2 = 0`, and the
    // design has no spread at this output at all.
    //
    // Asserted as a bound and a sign rather than as `== 0.0`: each
    // contribution is a difference of two separately solved operating points,
    // so the two agree to the solver's own last bits rather than bit for bit,
    // and their quadratic form lands within rounding of zero from either side.
    // What is exact is that a standard deviation never comes back as "-0".
    let (equal_r1, equal_r2) = divider_contributions(DIVIDER_R1, DIVIDER_R1, DIVIDER_SIGMA);
    assert_eq!(equal_r1, -equal_r2);
    let independent = libm::sqrt(equal_r1 * equal_r1 + equal_r2 * equal_r2);
    let cancelled = engine()
        .run_dc_match(
            &correlated_process_divider(DIVIDER_R1, "1"),
            &process_card(),
        )
        .expect("the perfectly correlated variance solves");
    assert!(
        cancelled.sigma_total < 1.0e-9 * independent,
        "a perfectly correlated cancellation left sigma {} against an independent {independent}",
        cancelled.sigma_total
    );
    assert!(
        cancelled.sigma_total.is_sign_positive() && cancelled.sigma_process.is_sign_positive(),
        "a cancelled variance reports +0.0, never -0"
    );
}

const LADDER_CURRENT: Value = 1.0e-3;
const LADDER_SIGMA_A: Value = 10.0;
const LADDER_SIGMA_B: Value = 20.0;
const LADDER_COEFFICIENT: Value = 0.5;

/// Two series resistances driven by a current source, each reading *both*
/// mismatch variables, so each instance is a two-variable correlated group of
/// its own.
///
/// `V(top) = I*(R1+R2)` is affine in every resistance, so the engine's central
/// difference is the derivative exactly and the algebra the test writes down
/// is not an approximation of anything.
fn correlated_mismatch_ladder(coefficient: &str) -> Netlist {
    deck(
        &SpectreStatisticsPlan {
            variations: vec![
                variation(2, SpectreVariationScope::Mismatch, "da", "10"),
                variation(2, SpectreVariationScope::Mismatch, "db", "20"),
            ],
            correlations: vec![correlate(
                SpectreVariationScope::Mismatch,
                &["da", "db"],
                coefficient,
            )],
        },
        ".param rnom=1k da=0 db=0\nI1 0 top DC 1m\nR1 top mid {rnom+da+db}\n\
         R2 mid 0 {rnom+da+db}",
    )
}

#[test]
fn a_correlation_inside_one_instance_does_not_couple_two_instances() {
    let result = engine()
        .run_dc_match(
            &correlated_mismatch_ladder(&format!("{LADDER_COEFFICIENT}")),
            &all_contributors(&DcMatchCard::voltage_probe("TOP")),
        )
        .expect("the correlated mismatch variance solves");

    // One instance's variable displaced by one sigma moves V(top) by the
    // current times that displacement, whichever variable and whichever
    // resistance it is.
    let from_a = LADDER_CURRENT * LADDER_SIGMA_A;
    let from_b = LADDER_CURRENT * LADDER_SIGMA_B;
    let per_instance =
        from_a * from_a + from_b * from_b + 2.0 * LADDER_COEFFICIENT * from_a * from_b;
    let expected = libm::sqrt(2.0 * per_instance);
    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 1.0e-9,
        "sigma {} vs the sum of two instances' quadratic forms {expected} \
         (relative error {error})",
        result.sigma_total
    );

    // Had the two instances been correlated with each other — a draw this
    // design did not declare — the same four contributions would have given
    // `(2*c_a)^2 + (2*c_b)^2 + 2*rho*(2*c_a)(2*c_b) = 4 * per_instance`:
    // exactly twice the variance, and a sigma larger by sqrt(2), 41.4% high.
    let coupled = libm::sqrt(4.0 * per_instance);
    assert!(
        (coupled / result.sigma_total - libm::sqrt(2.0)).abs() < 1.0e-6,
        "the fully coupled reading {coupled} is sqrt(2) times this one {}",
        result.sigma_total
    );
    assert_eq!(result.applied_correlations_mismatch, 1);
    assert_eq!(result.applied_correlations_process, 0);

    // Three instances times two declared variables, and the correlation left
    // the instance count alone.
    assert_eq!(result.evaluated_contributors, 6);
}

#[test]
fn uncorrelated_statistics_report_the_numbers_they_always_did() {
    let result = engine()
        .run_dc_match(
            &divider(),
            &all_contributors(&DcMatchCard::voltage_probe("OUT")),
        )
        .expect("mismatch variance solves");

    // The arithmetic the analysis performed before correlations were honoured,
    // term for term: a sum of squares folded from +0.0, and each share that
    // square over the total. Asserted by equality rather than by tolerance —
    // a scope with no `correlate` statement must not have moved by one bit.
    let variance = result
        .contributors
        .iter()
        .map(|entry| entry.contribution * entry.contribution)
        .fold(0.0, |variance, term| variance + term);
    assert_eq!(result.sigma_total, libm::sqrt(variance));
    assert_eq!(result.sigma_mismatch, libm::sqrt(variance));
    assert_eq!(result.sigma_process, 0.0);
    assert!(result.sigma_process.is_sign_positive());
    for entry in &result.contributors {
        assert_eq!(
            entry.share,
            entry.contribution * entry.contribution / variance,
            "{}/{} share",
            entry.instance,
            entry.parameter
        );
        assert!(
            entry.share >= 0.0,
            "an uncorrelated share is never negative"
        );
    }
    assert_eq!(result.applied_correlations_mismatch, 0);
    assert_eq!(result.applied_correlations_process, 0);

    // And the closed form for this divider, unchanged.
    let (from_r1, from_r2) = divider_contributions(DIVIDER_R1, DIVIDER_R2, DIVIDER_SIGMA);
    let expected = libm::sqrt(from_r1 * from_r1 + from_r2 * from_r2);
    let error = (result.sigma_total - expected).abs() / expected;
    assert!(
        error < 1.0e-6,
        "sigma {} vs analytic {expected} (relative error {error})",
        result.sigma_total
    );
}

/// Two series resistances driven by a current source, varied by design-wide
/// process variables: `V(top) = I*(R1+R2)` rises with both, so the two
/// contributions carry the *same* sign and a negative coefficient is what
/// makes one of them cancel the other.
fn correlated_process_ladder(coefficient: &str) -> Netlist {
    deck(
        &SpectreStatisticsPlan {
            variations: vec![
                variation(2, SpectreVariationScope::Process, "rav", "10"),
                variation(2, SpectreVariationScope::Process, "rbv", "20"),
            ],
            correlations: vec![correlate(
                SpectreVariationScope::Process,
                &["rav", "rbv"],
                coefficient,
            )],
        },
        ".param rav=1k rbv=1k\nI1 0 out DC 1m\nR1 out mid {rav}\nR2 mid 0 {rbv}",
    )
}

#[test]
fn the_variance_shares_sum_to_one_and_keep_their_sign() {
    const COEFFICIENT: Value = -0.9;
    let result = engine()
        .run_dc_match(&correlated_process_ladder("-0.9"), &process_card())
        .expect("the anticorrelated process variance solves");

    let from_a = LADDER_CURRENT * LADDER_SIGMA_A;
    let from_b = LADDER_CURRENT * LADDER_SIGMA_B;
    let variance = from_a * from_a + from_b * from_b + 2.0 * COEFFICIENT * from_a * from_b;
    let share = |parameter: &str| {
        result
            .contributors
            .iter()
            .find(|entry| entry.parameter == parameter)
            .unwrap_or_else(|| panic!("{parameter} is a contributor"))
            .share
    };

    // Euler allocation: `c_i * (R c)_i / sigma^2`. The smaller contribution's
    // partner more than cancels it, so its share is negative — it removes
    // variance from the total — while the two still account for all of it.
    let expected_a = from_a * (from_a + COEFFICIENT * from_b) / variance;
    let expected_b = from_b * (from_b + COEFFICIENT * from_a) / variance;
    assert!(expected_a < 0.0 && expected_b > 1.0, "the algebra says so");
    for (parameter, expected) in [("RAV", expected_a), ("RBV", expected_b)] {
        let error = (share(parameter) - expected).abs() / expected.abs();
        assert!(
            error < 1.0e-9,
            "{parameter} share {} vs analytic {expected} (relative error {error})",
            share(parameter)
        );
    }
    let sum: Value = result.contributors.iter().map(|entry| entry.share).sum();
    assert!((sum - 1.0).abs() < 1.0e-12, "shares sum to {sum}");
    assert!(
        result.contributors.iter().any(|entry| entry.share < 0.0),
        "a cancelling contributor keeps its negative sign: {:?}",
        result
            .contributors
            .iter()
            .map(|entry| entry.share)
            .collect::<Vec<_>>()
    );
    // Ranked by magnitude, so the cancelling contributor is listed rather than
    // sorted off the end of the report.
    assert_eq!(result.contributors[0].parameter, "RBV");
    assert_eq!(result.contributors[1].parameter, "RAV");
}

#[test]
fn an_invalid_correlation_matrix_is_refused_in_the_samplers_own_words() {
    // Three coefficients no set of variables can satisfy at once: `pa` and
    // `pb` agree strongly, `pb` and `pc` agree strongly, and `pa` and `pc`
    // disagree just as strongly.
    let netlist = deck(
        &SpectreStatisticsPlan {
            variations: vec![
                variation(2, SpectreVariationScope::Process, "pa", "10"),
                variation(2, SpectreVariationScope::Process, "pb", "10"),
                variation(2, SpectreVariationScope::Process, "pc", "10"),
            ],
            correlations: vec![
                correlate(SpectreVariationScope::Process, &["pa", "pb"], "0.9"),
                correlate(SpectreVariationScope::Process, &["pb", "pc"], "0.9"),
                correlate(SpectreVariationScope::Process, &["pa", "pc"], "-0.9"),
            ],
        },
        ".param pa=1k pb=1k pc=1k\nI1 0 out DC 1m\nR1 out mid {pa}\nR2 mid low {pb}\n\
         R3 low 0 {pc}",
    );
    let message = engine()
        .run_dc_match(&netlist, &process_card())
        .expect_err("three mutually inconsistent coefficients are not a correlation matrix")
        .to_string();

    // The sampler's own words, from the same constructor `sample_scope` hands
    // its target matrix to, over the same matrix in the same order (the
    // variables are resolved sorted by canonical name).
    let sampler = SpectreCorrelationMatrix::new(vec![
        vec![1.0, 0.9, -0.9],
        vec![0.9, 1.0, 0.9],
        vec![-0.9, 0.9, 1.0],
    ])
    .expect_err("the same matrix is not positive semidefinite")
    .to_string();
    assert!(
        message.contains(&sampler),
        "`.DCMATCH` said {message}\nthe sampler says {sampler}"
    );
}

#[test]
fn dc_mismatch_and_monte_carlo_agree_on_a_correlated_divider() {
    const RUNS: usize = 2000;
    let netlist = correlated_process_divider(DIVIDER_R2, "0.8");
    let linearized = engine()
        .run_dc_match(&netlist, &process_card())
        .expect("the correlated process variance solves");
    assert_eq!(linearized.applied_correlations_process, 1);

    // The same statistics block, sampled: the Gaussian copula's latent matrix
    // equals the target for two Gaussian variables, so the population carries
    // exactly the declared 0.8.
    let mut config = SimulationConfig::default();
    config.resource_limits.max_parallel_workers = 4;
    let population = Engine::new(config)
        .run_monte_carlo(&netlist, RUNS, 20_260_918)
        .expect("Monte Carlo population solves");
    assert_eq!(population.num_failures, 0);
    let sampled = population
        .std_dev("V(OUT)")
        .expect("the population reports V(OUT)");

    // A standard deviation estimated from N draws has relative standard error
    // 1/sqrt(2*(N-1)); three of those is 4.75% at N = 2000. The divider's own
    // linearization error at a 1% spread is four orders of magnitude below it.
    let bound = 3.0 / libm::sqrt(2.0 * (RUNS as Value - 1.0));
    let error = (sampled - linearized.sigma_total).abs() / linearized.sigma_total;
    assert!(
        error < bound,
        "population sigma {sampled} vs correlated linearization {} \
         (relative error {error}, 3-sigma sampling bound {bound})",
        linearized.sigma_total
    );

    // And the correlation is what makes them agree: summing the same two
    // contributions as independent is 67% high, far outside that bound.
    let (from_r1, from_r2) = divider_contributions(DIVIDER_R1, DIVIDER_R2, DIVIDER_SIGMA);
    let independent = libm::sqrt(from_r1 * from_r1 + from_r2 * from_r2);
    let independent_error = (sampled - independent).abs() / independent;
    assert!(
        independent_error > bound,
        "dropping the declared correlation would have given {independent} against a sampled \
         {sampled} (relative error {independent_error})"
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
