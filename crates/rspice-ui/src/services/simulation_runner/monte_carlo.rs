//! Monte Carlo analysis runner.

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
#[cfg(test)]
use super::parse_runner_netlist_with_statistical_sampling_and_abort;
use super::{DEFAULT_MONTE_CARLO_SEED, build_engine_config, parse_runner_netlist_with_abort};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::Engine;
use rspice_core::engine::monte_carlo_deck_trial_seed as trial_seed;
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use std::path::Path;

mod confidence;
mod trial_evidence;

/// Monte Carlo variable summary statistics.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableData {
    /// Confidence in the mean, with estimator and successful-trial population.
    pub mean_confidence: Option<crate::state::MonteCarloMeanConfidence>,
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
    /// What each retained trial measured, with the trial's own identity.
    ///
    /// Includes failed trials as explicit missing observations. Independent
    /// deck draws and shared parameter streams retain distinct replay identities.
    pub trial_measurements: Vec<crate::state::FamilyMemberMeasurements>,
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
    let mut result = engine
        .run_monte_carlo_voltages_with_abort(
            &netlist,
            &rspice_core::engine::MonteCarloRunConfig {
                first_trial: mc_cmd.first_trial,
                num_runs: mc_cmd.runs,
                seed,
                distribution,
                variation_source:
                    rspice_core::engine::MonteCarloVariationSource::ParameterTolerance,
                parameter_filter,
                environment: environment.as_ref(),
            },
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Monte Carlo analysis error", error))?;

    result
        .compute_mean_confidence(
            mc_cmd.confidence_pct,
            mc_cmd.confidence_method.into(),
            engine.config().resource_limits,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Monte Carlo confidence error", error))?;

    finish_monte_carlo_result(
        result,
        engine.config().resource_limits.max_result_values,
        abort,
    )
}

pub(crate) fn finish_monte_carlo_result(
    result: rspice_core::analysis::monte_carlo::MonteCarloResult,
    result_value_limit: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    let trial_indices = result.successful_trial_indices.as_deref().ok_or_else(|| {
        ServiceRunError::Failure("Monte Carlo engine omitted original trial indices".into())
    })?;
    let sampling = result.sampling.ok_or_else(|| {
        ServiceRunError::Failure("Monte Carlo engine omitted sampling provenance".into())
    })?;
    let trial_measurements = trial_evidence::parameter_population(
        &result,
        trial_indices,
        sampling,
        result_value_limit,
        abort,
    )?;
    let mut variables = Vec::with_capacity(result.variables.len());
    for (index, stats) in result.variables.into_values().enumerate() {
        poll_periodically(abort, index)?;
        variables.push(MonteCarloVariableData {
            mean_confidence: confidence::retain(result.confidence, stats.mean_confidence),
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
        seed: sampling.seed,
        runs_requested: result.num_runs,
        runs_completed: result.num_runs - result.num_failures,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
        trial_measurements,
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
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    if temperature_celsius.is_none()
        && (supply_voltage.is_some() || nominal_supply_voltage.is_some())
    {
        return Err(ServiceRunError::Failure(
            "Run Set supply overrides require a temperature-scoped environment".into(),
        ));
    }
    let command = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(command) => Some(command),
            _ => None,
        })
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "Monte Carlo analysis requires a .MC command in the netlist".into(),
            )
        })?;
    let environment =
        temperature_celsius.map(
            |temperature_celsius| rspice_core::engine::MonteCarloEnvironment {
                temperature_celsius,
                supply_voltage,
                nominal_supply_voltage,
                supply_source_names: supply_source_names.to_vec(),
            },
        );
    let mut config = build_engine_config(&netlist, None);
    if let Some(temperature) = temperature_celsius {
        config.temperature = rspice_core::constants::celsius_to_kelvin(temperature);
    }
    let engine = Engine::new(config);
    let mut result = engine
        .run_monte_carlo_voltages_with_abort(
            &netlist,
            &rspice_core::engine::MonteCarloRunConfig {
                first_trial: command.first_trial,
                num_runs: command.runs,
                seed: command.seed.unwrap_or(DEFAULT_MONTE_CARLO_SEED),
                distribution: Distribution::Uniform { tolerance: 0.0 },
                variation_source: rspice_core::engine::MonteCarloVariationSource::DeckStatistics,
                parameter_filter: None,
                environment: environment.as_ref(),
            },
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Monte Carlo analysis error", error))?;
    result
        .compute_mean_confidence(
            command.confidence_pct,
            command.confidence_method.into(),
            engine.config().resource_limits,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Monte Carlo confidence error", error))?;
    finish_monte_carlo_result(
        result,
        engine.config().resource_limits.max_result_values,
        abort,
    )
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
    crate::state::FamilyMemberMeasurements::validate_monte_carlo_sequence(
        &data.trial_measurements,
        data.seed,
        data.runs_requested,
        data.runs_completed,
        data.num_failures,
        data.variables
            .iter()
            .map(|variable| (variable.name.as_str(), variable.samples.as_slice())),
    )
    .map_err(ServiceRunError::Failure)?;
    let mut names = std::collections::HashSet::with_capacity(data.variables.len());
    for variable in &data.variables {
        if let Some(confidence) = variable.mean_confidence {
            confidence
                .validate(variable.samples.len(), data.num_failures)
                .map_err(ServiceRunError::Failure)?;
        }
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

/// Expand one base seed into a well-separated per-trial seed.
///
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
    fn statistical_monte_carlo_retains_constant_observations() {
        for deck in [NO_STATISTICS_DECK.to_owned(),
            "Constant MC output\n.param rload=agauss(1000,100,1)\nV1 out 0 1\nR1 out 0 {rload}\n.mc 8 seed 7\n.end\n".to_owned()] {
            let data = run_statistical(&deck).unwrap();
            assert_eq!(data.runs_completed, data.runs_requested);
            assert!(data.variables.iter().all(|variable| variable.std_dev == 0.0));
            assert_eq!(data.trial_measurements.len(), data.runs_completed);
        }
    }

    #[test]
    fn both_monte_carlo_drivers_exclude_digital_placeholder_voltages() {
        let deck = "Mixed MC\n.param rval=agauss(1000,100,1)\nV1 in 0 3.3\nR1 in out {rval}\nR2 out 0 9k\na_adc [out] [digital] adc\n.model adc adc_bridge(in_low=1.6 in_high=1.7)\nV2 later 0 2\nR3 later 0 1k\n.mc 4 uniform 0.1 seed 7\n.end\n";
        let parsed = rspice_core::Netlist::parse(deck).unwrap();
        let nominal = Engine::default().run_dc_op(&parsed).unwrap();
        let digital_id = nominal
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("digital"))
            .unwrap();
        let later_id = nominal
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("later"))
            .unwrap();
        for data in [
            run_monte_carlo_analysis(deck).unwrap(),
            run_statistical(deck).unwrap(),
        ] {
            assert_eq!(data.runs_completed, 4);
            assert!(
                !data
                    .variables
                    .iter()
                    .any(|variable| variable.name == "V(DIGITAL)"
                        || variable.name == format!("V({digital_id})"))
            );
            let analog = data
                .variables
                .iter()
                .find(|variable| variable.name == format!("V({later_id})"))
                .unwrap();
            assert_eq!(analog.samples, vec![2.0; 4]);
            for trial in data.trial_measurements {
                assert!(!trial.measurements.iter().any(|m| m.name == "V(DIGITAL)"));
                assert!(
                    trial
                        .measurements
                        .iter()
                        .any(|m| m.name == "V(LATER)" && m.value == Some(2.0))
                );
            }
        }
    }

    #[test]
    fn trial_seed_override_preserves_the_authored_source() {
        let parsed = parse_runner_netlist_with_statistical_sampling_and_abort(
            NO_STATISTICS_DECK,
            None,
            41,
            &NoAbort,
        )
        .unwrap();
        assert_eq!(parsed.source_text.as_deref(), Some(NO_STATISTICS_DECK));
        assert_eq!(parsed.options.seed, Some(41));
    }

    #[test]
    fn trial_seed_override_handles_annotated_end_and_preserves_the_title() {
        for (title, ending) in [
            ("seeded trial", ".end ; done"),
            ("seeded trial", ".END $ done"),
            ("seeded trial", ".end\n.options seed=99\n.end"),
            ("seeded trial", ".end; done"),
            ("seeded trial", ".if 0\n.options seed=99\n.endif\n.end"),
            (".end", ".end"),
        ] {
            let source =
                format!("{title}\n.options seed=7\n.param sample={{aunif(0,1)}}\n{ending}\n");
            let parsed = parse_runner_netlist_with_statistical_sampling_and_abort(
                &source, None, 41, &NoAbort,
            )
            .unwrap();
            assert_eq!(parsed.title, title);
            assert_eq!(parsed.options.seed, Some(41), "{source}");
            assert_eq!(parsed.params.random().seed(), 41, "{source}");
            assert_eq!(parsed.source_text.as_deref(), Some(source.as_str()));
        }
    }

    #[test]
    fn annotated_termination_preserves_each_trials_distribution() {
        let ordinary = run_statistical(STATISTICAL_PARAMETER_DECK).unwrap();
        let annotated =
            run_statistical(&STATISTICAL_PARAMETER_DECK.replace(".end", ".end ; circuit end"))
                .unwrap();
        assert_eq!(samples(&ordinary, "V(out)"), samples(&annotated, "V(out)"));
    }

    #[test]
    fn a_deck_without_a_terminal_end_still_receives_the_seed() {
        let source = "title\nV1 in 0 1\n";
        let parsed =
            parse_runner_netlist_with_statistical_sampling_and_abort(source, None, 9, &NoAbort)
                .unwrap();
        assert_eq!(parsed.options.seed, Some(9));
        assert_eq!(parsed.source_text.as_deref(), Some(source));
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
    fn both_monte_carlo_drivers_retain_failed_trial_gaps() {
        for (statistical, parameter) in [(false, "0.2"), (true, "aunif(0.2,0.2)")] {
            let deck = format!(
                "Partial MC\n.param offset={parameter}\nB1 out 0 V=V(out)^2+{{offset}}\nR1 out 0 1k\n.mc 16 uniform 1 seed 7\n.end\n"
            );
            let data = if statistical {
                run_statistical(&deck)
            } else {
                run_monte_carlo_analysis_with_abort(&deck, &NoAbort)
            }
            .unwrap();
            assert!(data.num_failures > 0);
            assert!(data.runs_completed > 0);
            assert_eq!(data.trial_measurements.len(), 16);
            let samples = &data
                .variables
                .iter()
                .find(|value| value.name == "V(OUT)")
                .unwrap()
                .samples;
            let mut retained = 0;
            for (index, member) in data.trial_measurements.iter().enumerate() {
                assert_eq!(member.member.index(), index);
                let observation = member.evidence_for("V(out)").unwrap();
                if let Some(value) = observation.value {
                    assert_eq!(value, samples[retained]);
                    assert!(observation.passed);
                    retained += 1;
                } else {
                    assert!(!observation.passed);
                    assert!(observation.error.is_some());
                }
            }
            assert_eq!(retained, data.runs_completed);
        }
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

    /// The seed a trial is retained under must be the seed that produced it.
    #[test]
    fn every_retained_trial_records_the_seed_its_index_expands_to() {
        // The deck asks for `.mc 8 seed 5`, so 5 is the base seed the per-trial
        // seeds are expanded from.
        let data = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("statistical trials run");

        assert_eq!(
            data.trial_measurements.len(),
            data.runs_completed,
            "every converged trial contributes evidence"
        );
        for member in &data.trial_measurements {
            let crate::state::FamilyMemberId::MonteCarloTrial { index, seed } = member.member
            else {
                panic!(
                    "a Monte Carlo trial must be identified as one: {:?}",
                    member.member
                );
            };
            assert_eq!(
                seed,
                trial_seed(5, index),
                "trial {index} recorded a seed its index does not expand to"
            );
        }
    }

    /// Trial-level reproducibility: the identity a trial is retained under has
    /// to reproduce that exact trial on its own.
    ///
    /// Analysis-level reproducibility — the same deck run twice giving the same
    /// distribution — was already true, and is not the property a worst-trial
    /// verdict depends on. That verdict sends an operator to re-run *one*
    /// trial, so what must hold is that re-materializing the deck at the
    /// recorded seed reproduces the numbers recorded against it, with no
    /// reference to the trials before it.
    ///
    /// This driver reparses each trial with an explicit seed override. That
    /// makes a single trial re-runnable in isolation:
    /// each trial's draws are a pure function of its own seed. A change that
    /// made trial N depend on the trials before it would leave the distribution
    /// reproducible and break this.
    #[test]
    fn a_retained_trial_reproduces_on_its_own_from_the_seed_it_recorded() {
        let data = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("statistical trials run");
        assert!(
            !data.trial_measurements.is_empty(),
            "the driver must attribute its trials"
        );

        let mut compared = 0_usize;
        for member in &data.trial_measurements {
            let crate::state::FamilyMemberId::MonteCarloTrial { index, seed } = member.member
            else {
                panic!("a Monte Carlo trial must be identified as one");
            };

            // Re-run this one trial exactly as the driver would have, without
            // running any other.
            let netlist = parse_runner_netlist_with_statistical_sampling_and_abort(
                MODEL_CARD_STATISTICS_DECK,
                None,
                seed,
                &NoAbort,
            )
            .expect("the trial deck reparses");
            let engine = Engine::new(build_engine_config(&netlist, None));
            let solved = engine
                .run_dc_op_with_abort(&netlist, &NoAbort)
                .expect("the trial re-solves");

            for (node_id, node_name) in solved.node_names.iter().enumerate().skip(1) {
                let name = format!("V({})", node_name.trim());
                let Some(evidence) = member.evidence_for(&name) else {
                    continue;
                };
                assert_eq!(
                    evidence.value.map(f64::to_bits),
                    Some(solved.node_voltages[node_id].to_bits()),
                    "trial {index} did not reproduce {name} from seed {seed}"
                );
                compared += 1;
            }
        }
        assert!(
            compared > 0,
            "the test compared nothing, so it proves nothing"
        );
    }

    /// A trial's evidence must be the trial's own, not the distribution's.
    #[test]
    fn trials_disagree_with_each_other_when_the_deck_states_variation() {
        let data = run_statistical(MODEL_CARD_STATISTICS_DECK).expect("statistical trials run");

        let drain: Vec<Option<f64>> = data
            .trial_measurements
            .iter()
            .filter_map(|member| member.evidence_for("V(d)").map(|evidence| evidence.value))
            .collect();

        assert!(drain.len() > 1, "the deck asked for several trials");
        assert!(
            drain.windows(2).any(|pair| pair[0] != pair[1]),
            "every trial reported the same drain voltage, so the per-trial \
             evidence is a copy of one trial rather than each trial's own"
        );
    }

    #[test]
    fn both_monte_carlo_drivers_retain_the_mean_interval() {
        let parameter = run_monte_carlo_analysis(RETENTION_DECK).unwrap();
        let deck = run_statistical_monte_carlo_with_source_path_and_abort(
            STATISTICAL_PARAMETER_DECK,
            None,
            &NoAbort,
        )
        .unwrap();
        for (data, critical) in [(parameter, 2.570581835636314), (deck, 2.364624251010299)] {
            let variable = data
                .variables
                .iter()
                .find(|variable| variable.name.eq_ignore_ascii_case("V(out)"))
                .unwrap();
            let confidence = variable
                .mean_confidence
                .expect("computed mean interval retained");
            assert_eq!(confidence.level_pct, 95.0);
            assert_eq!(confidence.successful_samples, data.runs_completed);
            assert!(!confidence.conditional_on_successful_trials);
            assert_eq!(
                confidence.method,
                crate::state::MonteCarloMeanMethod::StudentT
            );
            let crate::state::MonteCarloMeanInterval::Available { lower, upper } =
                confidence.interval
            else {
                panic!("finite interval")
            };
            let half = variable.std_dev / (variable.samples.len() as f64).sqrt() * critical;
            assert!((lower - (variable.mean - half)).abs() < 1e-10);
            assert!((upper - (variable.mean + half)).abs() < 1e-10);
        }
    }

    #[test]
    fn partial_monte_carlo_failures_retain_the_successful_population() {
        // x = x^2 + offset has real operating points only for offset <= 1/4.
        // The uniform samples straddle that boundary, so this checks a real
        // mixture of successful solves and convergence failures.
        let deck = "MC partial failures\n.param offset=0.2\nB1 out 0 V=V(out)^2+{offset}\nR1 out 0 1k\n.mc 16 uniform 1 seed 7 params offset\n.end\n";
        let data = run_monte_carlo_analysis(deck).unwrap();
        assert!(data.num_failures > 0);
        assert!(data.runs_completed > 0);
        assert_eq!(data.runs_completed + data.num_failures, 16);
        assert!(!data.all_converged);
        for variable in &data.variables {
            let confidence = variable
                .mean_confidence
                .expect("conditional mean confidence");
            assert!(confidence.conditional_on_successful_trials);
            assert_eq!(confidence.successful_samples, data.runs_completed);
        }
        assert!(
            data.variables
                .iter()
                .all(|variable| variable.samples.len() == data.runs_completed)
        );
    }

    #[test]
    fn the_parameter_tolerance_driver_retains_authored_trial_identity() {
        let data = run_monte_carlo_analysis(RETENTION_DECK).unwrap();
        assert_eq!(data.trial_measurements.len(), data.runs_requested);
        for (index, trial) in data.trial_measurements.iter().enumerate() {
            assert!(
                matches!(&trial.member, crate::state::FamilyMemberId::MonteCarloSequenceTrial { index: actual, seed, policy }
                if *actual == index && *seed == data.seed && policy == "parameter-xoroshiro128plus-2018-v1")
            );
            let value = trial.evidence_for("V(OUT)").unwrap().value.unwrap();
            let variable = data.variables.iter().find(|v| v.name == "V(OUT)").unwrap();
            assert_eq!(value, variable.samples[index]);
        }
    }

    /// Two dividers, one parameter each, so each node reports exactly one
    /// parameter's spread. `PARAMS ra` must leave the other node identical
    /// across every trial.
    ///
    /// A shared node would prove nothing: a subset that was silently ignored
    /// still moves a node both parameters reach. Separating them is what makes
    /// "only" checkable.
    const SUBSET_DECK: &str = "\
Monte Carlo subset
.param ra=1k
.param rb=1k
V1 in 0 1
R1 in na {ra}
R2 na 0 1k
R3 in nb {rb}
R4 nb 0 1k
.mc 8 gauss 0.2 seed 31{subset}
.end
";

    fn subset_run(subset: &str) -> MonteCarloData {
        run_monte_carlo_analysis(&SUBSET_DECK.replace("{subset}", subset))
            .expect("the subset deck runs")
    }

    fn is_constant(values: &[Value]) -> bool {
        values.windows(2).all(|pair| pair[0] == pair[1])
    }

    #[test]
    fn a_monte_carlo_run_varies_only_the_named_parameters() {
        let both = subset_run("");
        assert!(
            !is_constant(samples(&both, "V(na)")) && !is_constant(samples(&both, "V(nb)")),
            "with no subset both parameters vary, or this deck cannot show a subset at all"
        );

        let only_ra = subset_run(" params ra");
        assert!(
            !is_constant(samples(&only_ra, "V(na)")),
            "the named parameter must still vary"
        );
        let unnamed = samples(&only_ra, "V(nb)");
        assert!(
            is_constant(unnamed),
            "the unnamed parameter's node moved, so the subset did not reach the engine: \
             {unnamed:?}"
        );
    }

    #[test]
    fn a_monte_carlo_subset_naming_nothing_eligible_is_refused() {
        let error = run_monte_carlo_analysis(&SUBSET_DECK.replace("{subset}", " params rz"))
            .expect_err("a parameter the deck does not define cannot be varied");

        assert!(
            error.contains("not defined or not eligible"),
            "the engine's own refusal must reach the caller: {error}"
        );
    }
}
