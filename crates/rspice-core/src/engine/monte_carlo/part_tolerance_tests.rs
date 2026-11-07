//! A `.TOLERANCE` parameter varies by its own stated spread in a Monte Carlo
//! and reads nominal everywhere else.
use super::*;

/// A first-order low-pass with a 1 kHz corner, both parts at ±5 %.
fn low_pass(distribution: &str) -> Netlist {
    Netlist::parse(&format!(
        "RC low-pass\n.param rr=1k cc=159.15494309189535n\nV1 in 0 DC 0 AC 1\n\
         R1 in out {{rr}}\nC1 out 0 {{cc}}\n.TOLERANCE rr 0.05 {distribution}\n\
         .TOLERANCE cc 0.05 {distribution}\n.end\n"
    ))
    .unwrap()
}

/// The corner frequency from one AC point: |H(f0)|² = 1 / (1 + (f0/fc)²).
fn corner(engine: &Engine, circuit: &Netlist) -> Result<Vec<Value>, SimulationError> {
    let f0 = 1.0e3;
    let point = &engine.run_ac(circuit, &[f0])?[0];
    let out = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let gain = point.voltages[out].norm_sqr();
    Ok(vec![f0 / (1.0 / gain - 1.0).sqrt()])
}

fn corners(circuit: &Netlist, study: &MonteCarloStudyConfig) -> Vec<Value> {
    Engine::default()
        .run_monte_carlo_measurements_with_abort(circuit, study, &NoAbort, |engine, trial, _, _| {
            corner(engine, trial)
        })
        .unwrap()
        .variables["fc"]
        .samples
        .clone()
}

#[test]
fn gaussian_part_tolerances_give_the_corner_frequency_its_expected_spread() {
    let circuit = low_pass("GAUSS");
    let nominal = corner(&Engine::default(), &circuit).unwrap()[0];
    assert!((nominal - 1.0e3).abs() < 1e-6, "nominal corner {nominal}");
    // ±5 % at 3σ on R and C: ln fc = -ln R - ln C + k, so the corner's
    // relative σ is √2 · 0.05/3 = 2.357 %. N = 1000 puts the sample σ's
    // standard error at 1/√(2(N-1)) = 2.24 %; the bound is four of them.
    let expected = std::f64::consts::SQRT_2 * 0.05 / 3.0;
    for source in [
        MonteCarloVariationSource::ParameterTolerance,
        MonteCarloVariationSource::DeckStatistics,
    ] {
        let mut study = MonteCarloStudyConfig::new(1000, 7, vec!["fc".into()]);
        study.variation_source = source;
        let samples = corners(&circuit, &study);
        let n = samples.len() as f64;
        let mean = samples.iter().sum::<f64>() / n;
        let sigma = (samples.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
        let relative = sigma / mean;
        assert!(
            (relative / expected - 1.0).abs() < 4.0 / (2.0 * (n - 1.0)).sqrt(),
            "{source:?}: corner σ {relative} against {expected}"
        );
        assert!(
            (mean - 1.0e3).abs() < 4.0 * expected * 1.0e3 / n.sqrt(),
            "{source:?}: mean corner {mean}"
        );
    }
}

#[test]
fn a_worst_case_study_puts_every_part_at_an_end_of_its_band() {
    let circuit = low_pass("UNIFORM");
    let mut study = MonteCarloStudyConfig::new(64, 3, vec!["fc".into()]);
    study.distribution = Distribution::WorstCase { tolerance: 0.01 };
    let ends = [(1.05, 1.05), (1.05, 0.95), (0.95, 0.95)].map(|(r, c)| 1.0e3 / (r * c));
    let samples = corners(&circuit, &study);
    for fc in &samples {
        assert!(
            ends.iter().any(|end| (fc / end - 1.0).abs() < 1e-9),
            "corner {fc} is not at a band end"
        );
    }
    for end in ends {
        assert!(samples.iter().any(|fc| (fc / end - 1.0).abs() < 1e-9));
    }
}

#[test]
fn uniform_part_tolerances_stay_inside_their_band() {
    let circuit = low_pass("UNIFORM");
    let mut study = MonteCarloStudyConfig::new(200, 11, vec!["fc".into()]);
    study.distribution = Distribution::Gaussian { sigma: 0.5 };
    let samples = corners(&circuit, &study);
    let (low, high) = (1.0e3 / (1.05 * 1.05), 1.0e3 / (0.95 * 0.95));
    assert!(samples.iter().all(|fc| (low..=high).contains(fc)));
    assert!(samples.windows(2).any(|pair| pair[0] != pair[1]));
}

#[test]
fn a_tolerance_on_an_undefined_parameter_is_refused() {
    let circuit =
        Netlist::parse("undefined\nV1 in 0 1\nR1 in 0 1k\n.TOLERANCE nothing 0.05 GAUSS\n.end\n")
            .unwrap();
    let study = MonteCarloStudyConfig::new(2, 1, vec!["x".into()]);
    let error = Engine::default()
        .run_monte_carlo_measurements_with_abort(&circuit, &study, &NoAbort, |_, _, _, _| {
            Ok(vec![0.0])
        })
        .unwrap_err();
    assert!(error.to_string().contains("NOTHING"), "{error}");
    assert!(Netlist::parse("negative\n.param p=1\n.TOLERANCE p -0.05\n.end\n").is_err());
}
