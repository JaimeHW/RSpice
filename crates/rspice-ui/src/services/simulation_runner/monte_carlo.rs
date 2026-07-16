//! Monte Carlo analysis runner.

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{DEFAULT_MONTE_CARLO_SEED, build_engine_config, parse_runner_netlist_with_abort};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
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

/// Run Monte Carlo analysis by executing the first `.MC` command in the netlist.
pub fn run_monte_carlo_analysis(netlist_text: &str) -> Result<MonteCarloData, String> {
    run_monte_carlo_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run Monte Carlo analysis with cooperative cancellation.
pub fn run_monte_carlo_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    run_monte_carlo_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run Monte Carlo analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_monte_carlo_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<MonteCarloData, String> {
    run_monte_carlo_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Monte Carlo analysis with source-path resolution and cooperative
/// cancellation through parsing, trial execution, and result conversion.
pub fn run_monte_carlo_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MonteCarloData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
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

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .run_monte_carlo_with_options_and_abort(
            &netlist,
            mc_cmd.runs,
            seed,
            distribution,
            parameter_filter,
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

    Ok(MonteCarloData {
        seed,
        runs_requested: mc_cmd.runs,
        runs_completed: result.num_runs,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
    })
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
