//! One member of the deck's `.MC` study or `.STEP` sweep, solved as an
//! ordinary transient.
//!
//! The member is materialized by the engine entries its whole family runs
//! through: a Monte Carlo trial by the engine's trial sampler at the trial's
//! own index (a population of one starting there, which draws what that index
//! draws in the whole population), and a `.STEP` point by the deck plan's
//! materializer at the point's coordinate. The materialized circuit then runs
//! the task's transient through the same dispatch every transient takes, so
//! the member keeps its waveforms, events, `.MEAS` results, assertions and
//! finish exactly as a plain transient of that circuit would.

use std::path::Path;
use std::sync::Mutex;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::AnalysisCommand;
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_simulation_contract::mc_draft::McVariationSource;

use super::{EngineBridge, SimulationInput, ensure_not_aborted};
use crate::error::SimulationError;
use crate::execution_options::TransientMember;
use crate::results::SimulationResult;

impl EngineBridge {
    /// Solve `member` of the deck's own study or sweep with `config`, which
    /// must be a transient.
    pub(crate) fn run_transient_member(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
        environment: Option<crate::runner::AnalysisExecutionEnvironment>,
        member: TransientMember,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        if !matches!(config, AnalysisConfig::Transient(_)) {
            return Err(SimulationError::InvalidConfig(format!(
                "{} is solved as a transient; this task's analysis is not one",
                member.label()
            )));
        }
        let netlist = self.materialize_input(
            &SimulationInput {
                config,
                netlist_str,
                source_path,
                supply_corner: None,
                environment,
            },
            abort,
        )?;
        match member {
            TransientMember::MonteCarloTrial {
                index,
                variation_source,
            } => self.run_monte_carlo_trial(config, &netlist, index, variation_source, abort),
            TransientMember::StepPoint { index } => {
                self.run_step_point(config, &netlist, index, abort)
            }
        }
    }

    fn run_monte_carlo_trial(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        index: usize,
        variation_source: McVariationSource,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let card = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::MonteCarlo(card) => Some(card),
                _ => None,
            })
            .ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "trial {index} names the deck's .MC study, and the deck has no .MC card"
                ))
            })?;
        let first = card.first_trial;
        let end = first.saturating_add(card.runs);
        if !(first..end).contains(&index) {
            return Err(SimulationError::InvalidConfig(format!(
                "trial {index} is outside the deck's .MC study, whose trials are {first} to {}",
                end.saturating_sub(1)
            )));
        }
        let study = crate::transient_family::monte_carlo_study(card, variation_source, index, 1);
        let solved: Mutex<Option<Result<SimulationResult, SimulationError>>> = Mutex::new(None);
        let engine = self.engine.resolved_for_netlist(netlist);
        let outcome = engine.run_monte_carlo_measurements_with_abort(
            netlist,
            &study,
            abort,
            |engine, trial, _, abort| {
                let bridge = EngineBridge {
                    engine: engine.resolved_for_netlist(trial),
                    measurement_references: self.measurement_references.clone(),
                };
                let result = bridge.dispatch_analysis(config, trial, abort);
                let failed = result.is_err();
                *solved
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(result);
                if failed {
                    Err(rspice_core::SimulationError::Circuit(format!(
                        "Monte Carlo trial {index} failed"
                    )))
                } else {
                    Ok(vec![0.0])
                }
            },
        );
        ensure_not_aborted(abort)?;
        let solved = solved
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match (solved, outcome) {
            // The trial's own result or its own failure, whatever the study
            // around it made of that.
            (Some(result), _) => result,
            (None, Err(error)) => Err(self.translate_error(error)),
            (None, Ok(_)) => Err(SimulationError::InvalidConfig(format!(
                "Monte Carlo trial {index} was never solved"
            ))),
        }
    }

    fn run_step_point(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        index: usize,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let refused = |stage: &str, error: &dyn std::fmt::Display| {
            if abort.is_aborted() {
                SimulationError::Aborted
            } else {
                SimulationError::InvalidConfig(format!("point {index}: {stage}: {error}"))
            }
        };
        if !netlist
            .analyses
            .iter()
            .any(|analysis| matches!(analysis, AnalysisCommand::Step(_)))
        {
            return Err(SimulationError::InvalidConfig(format!(
                "point {index} names the deck's .STEP sweep, and the deck has no .STEP card"
            )));
        }
        let engine = self.engine.resolved_for_netlist(netlist);
        let plan = rspice_core::execution::DeckPlan::from_netlist_with_abort(
            netlist,
            &engine.config().resource_limits,
            abort,
        )
        .map_err(|error| refused("the .STEP plan", &error))?;
        let materializer = engine
            .prepare_deck_plan_materializer_with_abort(netlist, &plan, abort)
            .map_err(|error| refused("the .STEP plan", &error))?;
        if index >= materializer.len() {
            return Err(SimulationError::InvalidConfig(format!(
                "point {index} is outside the deck's .STEP sweep of {} points",
                materializer.len()
            )));
        }
        let run = materializer
            .materialize_run_with_abort(index, abort)
            .map_err(|error| refused("materializing the point", &error))?;
        let (_, point, _, _) = run.into_parts();
        EngineBridge {
            engine: engine.resolved_for_netlist(&point),
            measurement_references: self.measurement_references.clone(),
        }
        .dispatch_analysis(config, &point, abort)
    }
}
