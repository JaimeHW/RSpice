//! Monte Carlo analysis runner.

use super::{build_engine_config, parse_runner_netlist, DEFAULT_MONTE_CARLO_SEED};
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use rspice_core::Value;
use std::path::Path;

/// Monte Carlo variable summary statistics.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableData {
    pub name: String,
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
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub num_failures: usize,
    pub all_converged: bool,
    pub variables: Vec<MonteCarloVariableData>,
}

/// Run Monte Carlo analysis by executing the first `.MC` command in the netlist.
pub fn run_monte_carlo_analysis(netlist_text: &str) -> Result<MonteCarloData, String> {
    run_monte_carlo_analysis_with_source_path(netlist_text, None)
}

/// Run Monte Carlo analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_monte_carlo_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<MonteCarloData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;

    let mc_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(cmd) => Some(cmd),
            _ => None,
        })
        .ok_or_else(|| "Monte Carlo analysis requires a .MC command in the netlist".to_string())?;

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
        .run_monte_carlo_with_options(&netlist, mc_cmd.runs, seed, distribution, parameter_filter)
        .map_err(|e| format!("Monte Carlo analysis error: {}", e))?;

    let mut variables: Vec<MonteCarloVariableData> = result
        .variables
        .into_values()
        .map(|stats| MonteCarloVariableData {
            name: stats.name,
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            histogram: stats.histogram,
            bin_edges: stats.bin_edges,
        })
        .collect();
    variables.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(MonteCarloData {
        runs_requested: mc_cmd.runs,
        runs_completed: result.num_runs,
        num_failures: result.num_failures,
        all_converged: result.all_converged,
        variables,
    })
}
