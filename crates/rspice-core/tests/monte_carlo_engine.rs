use rspice_core::analysis::Distribution;
use rspice_core::{Engine, Netlist};

const PARAMETRIC_DIVIDER: &str = "\
Monte Carlo divider
.param rval=1k
V1 in 0 1
R1 in out {rval}
R2 out 0 1k
.end
";

#[test]
fn monte_carlo_is_reproducible_and_preserves_run_order() {
    let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("fixture parses");
    let filter = ["RVAL".to_string()];
    let engine = Engine::default();

    let first = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            0x5eed,
            Distribution::Gaussian { sigma: 0.1 },
            Some(&filter),
        )
        .expect("first run completes");
    let second = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            0x5eed,
            Distribution::Gaussian { sigma: 0.1 },
            Some(&filter),
        )
        .expect("second run completes");

    assert_eq!(first.num_runs, 32);
    assert!(first.all_converged);
    assert_eq!(first.num_failures, 0);
    let first_samples = &first.variables["V(OUT)"].samples;
    let second_samples = &second.variables["V(OUT)"].samples;
    assert_eq!(first_samples, second_samples);
    assert!(first_samples.windows(2).any(|pair| pair[0] != pair[1]));
}

#[test]
fn monte_carlo_with_zero_spread_matches_the_nominal_solution() {
    let netlist = Netlist::parse(PARAMETRIC_DIVIDER).expect("fixture parses");
    let filter = ["RVAL".to_string()];
    let result = Engine::default()
        .run_monte_carlo_with_options(
            &netlist,
            8,
            42,
            Distribution::Uniform { tolerance: 0.0 },
            Some(&filter),
        )
        .expect("analysis completes");

    let samples = &result.variables["V(OUT)"].samples;
    assert_eq!(samples.len(), 8);
    assert!(samples.iter().all(|value| (*value - 0.5).abs() < 1.0e-12));
}
