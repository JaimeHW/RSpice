//! Monte Carlo analysis runner.

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{
    DEFAULT_MONTE_CARLO_SEED, build_engine_config, parse_runner_netlist_with_abort,
    parse_runner_netlist_with_statistical_sampling_and_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use std::path::Path;

/// Monte Carlo variable summary statistics.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableData {
    pub name: String,
    /// Exact finite values retained in engine execution order.
    pub samples: Vec<Value>,
    pub mean: Value,
    pub std_dev: Value,
    pub min: Value,
    pub max: Value,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<Value>,
}

/// Monte Carlo analysis data.
#[derive(Debug, Clone)]
pub struct MonteCarloData {
    /// Effective random seed used for this analysis, including the default
    /// when `.MC` did not specify one.
    pub seed: u64,
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub num_failures: usize,
    pub all_converged: bool,
    pub variables: Vec<MonteCarloVariableData>,
}

/// Run Monte Carlo analysis by executing the first `.MC` command in the
/// netlist.
///
/// Test-only. Shipping dispatch uses the environment-aware entry point below.
#[cfg(test)]
pub fn run_monte_carlo_analysis(netlist_text: &str) -> Result<MonteCarloData, String> {
    run_monte_carlo_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run Monte Carlo analysis with cooperative cancellation. Test-only; see
/// [`run_monte_carlo_analysis`].
#[cfg(test)]
pub fn run_monte_carlo_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    run_monte_carlo_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run Monte Carlo analysis with source-path resolution and cooperative
/// cancellation through parsing, trial execution, and result conversion.
#[cfg(test)]
pub fn run_monte_carlo_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
        netlist_text,
        source_path,
        None,
        None,
        None,
        &[],
        abort,
    )
}

/// Run parameter-tolerance Monte Carlo under one exact Run Set environment.
pub(crate) fn run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    temperature_celsius: Option<Value>,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    if temperature_celsius.is_none()
        && (supply_voltage.is_some() || nominal_supply_voltage.is_some())
    {
        return Err(ServiceRunError::Failure(
            "Run Set supply overrides require a temperature-scoped environment".to_owned(),
        ));
    }
    ensure_not_aborted(abort)?;

    let mc_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(cmd) => Some(cmd),
            _ => None,
        })
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "Monte Carlo analysis requires a .MC command in the netlist".to_string(),
            )
        })?;

    let distribution = match mc_cmd.distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: mc_cmd.relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: mc_cmd.relative_spread,
        },
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
            tolerance: mc_cmd.relative_spread,
        },
    };

    let seed = mc_cmd.seed.unwrap_or(DEFAULT_MONTE_CARLO_SEED);
    let parameter_filter = (!mc_cmd.params.is_empty()).then_some(mc_cmd.params.as_slice());

    let mut engine_config = build_engine_config(&netlist, None);
    if let Some(temperature_celsius) = temperature_celsius {
        engine_config.temperature = rspice_core::constants::celsius_to_kelvin(temperature_celsius);
    }
    let engine = Engine::new(engine_config);
    let environment =
        temperature_celsius.map(
            |temperature_celsius| rspice_core::engine::MonteCarloEnvironment {
                temperature_celsius,
                supply_voltage,
                nominal_supply_voltage,
                supply_source_names: supply_source_names.to_vec(),
            },
        );
    let result = engine
        .run_monte_carlo_with_options_environment_and_abort(
            &netlist,
            mc_cmd.runs,
            seed,
            distribution,
            parameter_filter,
            environment,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Monte Carlo analysis error", error))?;

    let mut variables = Vec::with_capacity(result.variables.len());
    for (index, stats) in result.variables.into_values().enumerate() {
        poll_periodically(abort, index)?;
        variables.push(MonteCarloVariableData {
            name: stats.name,
            samples: stats.samples,
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            histogram: stats.histogram,
            bin_edges: stats.bin_edges,
        });
    }
    ensure_not_aborted(abort)?;
    variables.sort_by(|a, b| a.name.cmp(&b.name));
    ensure_not_aborted(abort)?;

    let data = MonteCarloData {
        seed,
        runs_requested: mc_cmd.runs,
        runs_completed: result.num_runs,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
    };
    validate_monte_carlo_data(&data)?;
    Ok(data)
}

/// Run Monte Carlo from the deck's own statistical expressions.
///
/// The parameter-tolerance driver above perturbs eligible `.param` values
/// numerically and leaves model cards untouched, so a PDK whose variation is
/// authored as `agauss`/`gauss` inside `.model` cards varied nothing at all
/// under it. This driver instead re-materializes the deck once per trial with
/// a fresh statistical seed, so every statistical expression in the deck — in
/// `.param` cards, in element values, and in model cards alike — redraws.
///
/// The `.MC` command's seed is the base seed. Per-trial seeds are expanded
/// from it, so the whole analysis is reproducible from one number and no two
/// trials share a stream.
#[cfg(test)]
pub fn run_statistical_monte_carlo_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
        netlist_text,
        source_path,
        None,
        None,
        None,
        &[],
        abort,
    )
}

/// Run deck-statistical Monte Carlo under one exact Run Set environment.
/// Every trial is reparsed to redraw statistical expressions, so the
/// environment is applied to every reparsed deck before its operating point.
pub(crate) fn run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    temperature_celsius: Option<Value>,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    let mut netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    if let Some(temperature_celsius) = temperature_celsius {
        super::apply_run_environment(
            &mut netlist,
            temperature_celsius,
            supply_voltage,
            nominal_supply_voltage,
            supply_source_names,
            abort,
        )?;
    } else if supply_voltage.is_some() || nominal_supply_voltage.is_some() {
        return Err(ServiceRunError::Failure(
            "Run Set supply overrides require a temperature-scoped environment".to_owned(),
        ));
    }
    ensure_not_aborted(abort)?;

    let (runs, base_seed) = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(cmd) => {
                Some((cmd.runs, cmd.seed.unwrap_or(DEFAULT_MONTE_CARLO_SEED)))
            }
            _ => None,
        })
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "Monte Carlo analysis requires a .MC command in the netlist".to_string(),
            )
        })?;
    if runs == 0 {
        return Err(ServiceRunError::Failure(
            "Monte Carlo requires at least one trial".to_string(),
        ));
    }

    let mut trials: Vec<(Vec<Value>, Vec<String>)> = Vec::new();
    for trial in 0..runs {
        poll_periodically(abort, trial)?;
        let deck = with_statistical_seed(netlist_text, trial_seed(base_seed, trial));
        let mut trial_netlist =
            parse_runner_netlist_with_statistical_sampling_and_abort(&deck, source_path, abort)?;
        if let Some(temperature_celsius) = temperature_celsius {
            super::apply_run_environment(
                &mut trial_netlist,
                temperature_celsius,
                supply_voltage,
                nominal_supply_voltage,
                supply_source_names,
                abort,
            )?;
        }
        ensure_not_aborted(abort)?;
        let engine = Engine::new(build_engine_config(&trial_netlist, None));
        // A trial that fails to converge is dropped from the distribution and
        // counted, exactly as the parameter-tolerance driver does; only a
        // cancellation stops the analysis.
        match engine.run_dc_op_with_abort(&trial_netlist, abort) {
            Ok(result) => trials.push((result.node_voltages, result.node_names)),
            Err(rspice_core::SimulationError::Aborted) => {
                return Err(ServiceRunError::Aborted);
            }
            Err(_) => {}
        }
    }
    ensure_not_aborted(abort)?;

    // A deck that states no statistical variation produces identical trials.
    // Reporting a zero-width distribution would read as "this design has no
    // spread" when the truth is that nothing was asked to vary.
    if trials.len() > 1 && trials.windows(2).all(|pair| pair[0].0 == pair[1].0) {
        return Err(ServiceRunError::Failure(
            "every trial resolved to the same operating point: this deck states no statistical \
             variation. Statistical Monte Carlo draws from agauss/gauss/unif expressions in the \
             deck and its model cards; add them, or select the parameter-tolerance variation \
             source."
                .to_string(),
        ));
    }

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .monte_carlo_result_from_trials(trials, runs)
        .map_err(|error| ServiceRunError::from_core("Monte Carlo analysis error", error))?;

    let mut variables = Vec::with_capacity(result.variables.len());
    for (index, stats) in result.variables.into_values().enumerate() {
        poll_periodically(abort, index)?;
        variables.push(MonteCarloVariableData {
            name: stats.name,
            samples: stats.samples,
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            histogram: stats.histogram,
            bin_edges: stats.bin_edges,
        });
    }
    ensure_not_aborted(abort)?;
    variables.sort_by(|a, b| a.name.cmp(&b.name));

    let data = MonteCarloData {
        seed: base_seed,
        runs_requested: runs,
        runs_completed: result.num_runs,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
    };
    validate_monte_carlo_data(&data)?;
    Ok(data)
}

fn validate_monte_carlo_data(data: &MonteCarloData) -> ServiceRunResult<()> {
    if data.runs_requested == 0
        || data.runs_completed == 0
        || data.runs_completed.saturating_add(data.num_failures) != data.runs_requested
        || data.all_converged != (data.num_failures == 0)
        || data.variables.is_empty()
    {
        return Err(ServiceRunError::Failure(
            "Monte Carlo returned an inconsistent run summary".to_owned(),
        ));
    }
    let mut names = std::collections::HashSet::with_capacity(data.variables.len());
    for variable in &data.variables {
        if variable.name.trim().is_empty()
            || !names.insert(variable.name.trim().to_ascii_lowercase())
            || variable.samples.len() != data.runs_completed
            || variable.samples.iter().any(|value| !value.is_finite())
            || !variable.mean.is_finite()
            || !variable.std_dev.is_finite()
            || variable.std_dev < 0.0
            || !variable.min.is_finite()
            || !variable.max.is_finite()
            || variable.min > variable.max
            || variable.histogram.is_empty()
            || variable.bin_edges.len() != variable.histogram.len() + 1
            || variable.histogram.iter().sum::<usize>() != variable.samples.len()
            || variable.bin_edges.iter().any(|edge| !edge.is_finite())
            || variable.bin_edges.windows(2).any(|pair| pair[1] < pair[0])
        {
            return Err(ServiceRunError::Failure(format!(
                "Monte Carlo variable '{}' has an invalid statistical payload",
                variable.name
            )));
        }
    }
    Ok(())
}

/// Give a deck one statistical seed of our choosing.
///
/// The parser pre-scans every `.options`/`.option`/`.opt` card for `seed=` and
/// keeps the last one it reads, so a card spliced in ahead of the terminal
/// `.end` overrides whatever the deck or the plan's own options block asked
/// for. Nothing else in the deck is touched.
fn with_statistical_seed(netlist_text: &str, seed: u64) -> String {
    let card = format!(".options seed={seed}");
    let mut lines: Vec<&str> = netlist_text.lines().collect();
    let terminal_end = lines
        .iter()
        .rposition(|line| line.trim().eq_ignore_ascii_case(".end"));
    match terminal_end {
        Some(index) => lines.insert(index, &card),
        None => lines.push(&card),
    }
    lines.join("\n") + "\n"
}

/// Expand one base seed into a well-separated per-trial seed.
///
/// SplitMix64's finalizer, which is the standard way to turn a counter into
/// seeds that do not share low-order structure. Two trials whose indices differ
/// by one must not draw correlated streams.
const fn trial_seed(base: u64, trial: usize) -> u64 {
    let mut z = base.wrapping_add((trial as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    const MONTE_CARLO_DECK: &str = "\
Monte Carlo cancellation
.param rload=1k
V1 in 0 1
R1 in out {rload}
R2 out 0 1k
.mc 64 gauss 0.1 seed 7 params rload
.end
";

    const RETENTION_DECK: &str = "\
Monte Carlo retention
.param rload=1k
V1 in 0 1
R1 in out {rload}
R2 out 0 1k
.mc 6 gauss 0.1 seed 19 params rload
.end
";

    /// The case the parameter-tolerance driver cannot serve: the spread is
    /// authored on a model card, which that driver never touches.
    const MODEL_CARD_STATISTICS_DECK: &str = "\
Model card statistics
.model nm nmos level=1 vto={agauss(0.7, 0.1, 1)} kp=200u
M1 d g 0 0 nm w=10u l=1u
R1 vdd d 10k
V1 vdd 0 3
V2 g 0 1.2
.mc 8 seed 5
.end
";

    const NO_STATISTICS_DECK: &str = "\
No statistics at all
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.mc 6 seed 5
.end
";

    const STATISTICAL_PARAMETER_DECK: &str = "\
Statistical parameter environment
.param rload={agauss(1k, 100, 1)}
V1 in 0 1
R1 in out {rload}
R2 out 0 1k
.mc 8 seed 23
.end
";

    fn samples<'a>(data: &'a MonteCarloData, name: &str) -> &'a [Value] {
        &data
            .variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing Monte Carlo variable {name}"))
            .samples
    }

    fn assert_supply_doubles_samples(reference: &MonteCarloData, doubled: &MonteCarloData) {
        let reference = samples(reference, "V(out)");
        let doubled = samples(doubled, "V(out)");
        assert_eq!(reference.len(), doubled.len());
        for (reference, doubled) in reference.iter().zip(doubled) {
            let tolerance = reference.abs().max(1.0) * 1.0e-10;
            assert!(
                (*doubled - 2.0 * *reference).abs() <= tolerance,
                "bound supply did not double the trial sample: {reference} -> {doubled}"
            );
        }
    }

    fn run_statistical(deck: &str) -> ServiceRunResult<MonteCarloData> {
        run_statistical_monte_carlo_with_source_path_and_abort(deck, None, &NoAbort)
    }

    #[test]
    fn statistical_monte_carlo_varies_a_model_card_parameter() {
        let data = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("statistical trials run");

        assert_eq!(data.runs_requested, 8);
        assert!(data.all_converged, "{} trial(s) failed", data.num_failures);

        let drain = data
            .variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case("V(d)"))
            .expect("the drain node is reported");
        // The whole point of this driver: a threshold voltage written as
        // agauss on a .model card moves the operating point trial to trial.
        assert!(
            drain.std_dev > 0.0 && drain.min < drain.max,
            "model-card statistics produced no spread: {drain:?}"
        );
    }

    #[test]
    fn statistical_monte_carlo_is_reproducible_from_its_seed() {
        let first = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("first analysis");
        let second = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("second analysis");

        let samples = |data: &MonteCarloData| {
            data.variables
                .iter()
                .map(|variable| (variable.name.clone(), variable.samples.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(samples(&first), samples(&second));
    }

    #[test]
    fn parameter_tolerance_monte_carlo_applies_the_run_set_environment() {
        let reference = run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
            RETENTION_DECK,
            None,
            Some(25.0),
            None,
            None,
            &[],
            &NoAbort,
        )
        .expect("reference Monte Carlo executes");
        let doubled = run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
            RETENTION_DECK,
            None,
            Some(125.0),
            Some(2.0),
            Some(1.0),
            &["V1".to_owned()],
            &NoAbort,
        )
        .expect("PVT Monte Carlo executes");

        assert_supply_doubles_samples(&reference, &doubled);
    }

    #[test]
    fn deck_statistical_monte_carlo_applies_the_environment_to_every_trial() {
        let reference = run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
            STATISTICAL_PARAMETER_DECK,
            None,
            Some(25.0),
            None,
            None,
            &[],
            &NoAbort,
        )
        .expect("reference statistical Monte Carlo executes");
        let doubled = run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
            STATISTICAL_PARAMETER_DECK,
            None,
            Some(125.0),
            Some(2.0),
            Some(1.0),
            &["V1".to_owned()],
            &NoAbort,
        )
        .expect("PVT statistical Monte Carlo executes");

        assert_supply_doubles_samples(&reference, &doubled);
    }

    #[test]
    fn statistical_monte_carlo_refuses_a_deck_that_states_no_variation() {
        let error = run_statistical(NO_STATISTICS_DECK).expect_err("a degenerate deck is refused");

        let message = error.to_string();
        assert!(
            message.contains("no statistical variation"),
            "the refusal must name the cause, got: {message}"
        );
    }

    #[test]
    fn the_seed_card_is_spliced_ahead_of_the_terminal_end() {
        let deck = with_statistical_seed(NO_STATISTICS_DECK, 41);
        let lines: Vec<&str> = deck.lines().collect();

        let seed_line = lines
            .iter()
            .position(|line| line.trim() == ".options seed=41")
            .expect("the seed card is present");
        let end_line = lines
            .iter()
            .position(|line| line.trim().eq_ignore_ascii_case(".end"))
            .expect("the terminal end card survives");
        assert!(seed_line < end_line);
        // Every original line is still there, in order.
        assert!(lines.contains(&"R1 in out 1k"));
    }

    #[test]
    fn a_deck_without_a_terminal_end_still_receives_the_seed() {
        let deck = with_statistical_seed("title\nV1 in 0 1\n", 9);
        assert!(deck.lines().any(|line| line.trim() == ".options seed=9"));
    }

    #[test]
    fn trials_do_not_share_a_stream() {
        let seeds: Vec<u64> = (0..64).map(|trial| trial_seed(11, trial)).collect();
        let unique: std::collections::HashSet<u64> = seeds.iter().copied().collect();
        assert_eq!(unique.len(), seeds.len());
        // Adjacent trials must not differ only in their low bits, which is
        // what a bare counter would give the statistical stream.
        assert!(seeds.windows(2).all(|pair| pair[0] ^ pair[1] > 0xFFFF));
    }

    #[test]
    fn monte_carlo_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_monte_carlo_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn monte_carlo_honors_abort_during_trial_execution() {
        let abort = AbortOnPoll::new(8);
        let result = run_monte_carlo_analysis_with_abort(MONTE_CARLO_DECK, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 8);
    }

    #[test]
    fn monte_carlo_retains_effective_seed_and_exact_samples() {
        let result = run_monte_carlo_analysis(RETENTION_DECK).expect("analysis succeeds");

        assert_eq!(result.seed, 19);
        assert_eq!(result.runs_requested, 6);
        assert_eq!(result.runs_completed + result.num_failures, 6);
        assert!(!result.variables.is_empty());
        for variable in &result.variables {
            assert_eq!(variable.samples.len(), result.runs_completed);
            assert!(variable.samples.iter().all(|sample| sample.is_finite()));
            let exact_mean = variable.samples.iter().sum::<f64>() / variable.samples.len() as f64;
            assert_eq!(variable.mean, exact_mean);
        }
    }
}
