//! Electrical host for the shared resumable control-command machine.

use super::{Engine, SimulationConfigError, SimulationError, TransientStartupMode};
use crate::analysis::AcResult;
use crate::analysis::transient::TransientResult;
use crate::execution::control::{
    ControlCommand, ControlError, ControlErrorKind, ControlScalarEvaluator,
    ParameterScalarEvaluator,
};
use crate::netlist::expr::{ParamContext, eval_expression};
use crate::netlist::{AnalysisCommand, Netlist};
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::{AbortSignal, Value};
use std::collections::BTreeMap;

mod presentation;
pub use presentation::{
    ControlCurrentSource, ControlPlotOptions, ControlPresentation, ControlPresentationKind,
    ControlTrace, ControlVector, ControlVectorId,
};

#[derive(Debug, thiserror::Error)]
pub enum ControlExecutionError {
    #[error(transparent)]
    Command(#[from] ControlError),
    #[error("control line {line}: {source}")]
    Simulation {
        line: usize,
        #[source]
        source: SimulationError,
    },
    #[error("control line {line}: {source}")]
    Configuration {
        line: usize,
        #[source]
        source: SimulationConfigError,
    },
}

#[derive(Debug)]
pub enum ControlAnalysisResult {
    OperatingPoint(Box<crate::solver::SimulationResult>),
    Ac(Vec<AcResult>),
    Transient(Box<TransientResult>),
}

#[derive(Debug)]
pub struct ControlNamedDataset {
    pub name: String,
    pub analysis_id: crate::identity::AnalysisInstanceId,
    pub device_op_report: Option<Box<crate::circuit::DeviceOpReport>>,
    pub command: AnalysisCommand,
    pub result: ControlAnalysisResult,
}

#[derive(Debug)]
pub enum ControlCommandEffect {
    CircuitChanged,
    /// Names of newly retained, independently owned result datasets.
    Analyses(Vec<String>),
    /// The presentation host must handle this request before advancing the
    /// command session. Returning it is not a claim that rendering occurred.
    Presentation(ControlPresentation),
}

/// Mutable circuit state and immutable completed datasets for one script.
///
/// Construct the netlist from `ControlProgram::declarative_source`, using the
/// caller's normal path/sealed-source parser. Drive this host with that
/// program's `ControlSession`. This preserves one execution path on all targets.
pub struct ControlCircuit {
    netlist: Netlist,
    datasets: Vec<ControlNamedDataset>,
    ordinals: BTreeMap<&'static str, usize>,
    retained_values: usize,
    vector_units: BTreeMap<ControlVectorId, crate::execution::SignalUnit>,
}

impl ControlCircuit {
    pub fn new(netlist: Netlist) -> Result<Self, ControlExecutionError> {
        if !netlist.control_dispositions.is_empty() {
            return Err(command_error(0, "construct the control circuit from the program's declarative source; promoted script commands would duplicate execution").into());
        }
        Ok(Self {
            netlist,
            datasets: Vec::new(),
            ordinals: BTreeMap::new(),
            retained_values: 0,
            vector_units: BTreeMap::new(),
        })
    }

    pub fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    pub fn datasets(&self) -> &[ControlNamedDataset] {
        &self.datasets
    }

    /// Consume the completed session for publication without cloning results.
    pub fn into_datasets(self) -> Vec<ControlNamedDataset> {
        self.datasets
    }

    pub fn execute(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ControlCommandEffect, ControlExecutionError> {
        let line = command.line;
        if abort.is_aborted() {
            return Err(simulation_error(line, SimulationError::Aborted));
        }
        ResourceLimitError::ensure(
            ResourceKind::NetlistBytes,
            command.name.len().saturating_add(command.arguments.len()),
            engine.config().resource_limits.max_netlist_bytes,
        )
        .map_err(|error| simulation_error(line, error.into()))?;
        if command.arguments.contains(['\r', '\n']) {
            return Err(command_error(
                line,
                "a control command cannot contain another physical line",
            )
            .into());
        }
        match command.name.as_str() {
            "alter" => {
                self.alter(engine, command, variables)?;
                Ok(ControlCommandEffect::CircuitChanged)
            }
            "op" | "ac" | "tran" => {
                let source = format!(
                    "control analysis\n.{} {}\n.end\n",
                    command.name, command.arguments
                );
                let parsed = Netlist::parse_with_abort(&source, abort).map_err(|error| {
                    ControlError::new(
                        line,
                        if abort.is_aborted() {
                            ControlErrorKind::Aborted
                        } else {
                            ControlErrorKind::Syntax
                        },
                        error.to_string(),
                    )
                })?;
                let [analysis] = parsed.analyses.as_slice() else {
                    return Err(command_error(
                        line,
                        "analysis command did not produce exactly one analysis",
                    )
                    .into());
                };
                let name = self.run_analysis(engine, analysis.clone(), line, abort)?;
                Ok(ControlCommandEffect::Analyses(vec![name]))
            }
            "run" => {
                if !command.arguments.is_empty() {
                    return Err(command_error(line, "run does not accept arguments").into());
                }
                let analyses = self.netlist.analyses.clone();
                if analyses.is_empty() {
                    return Err(
                        command_error(line, "run has no declarative analysis to execute").into(),
                    );
                }
                let mut names = Vec::new();
                for analysis in analyses {
                    names.push(self.run_analysis(engine, analysis, line, abort)?);
                }
                Ok(ControlCommandEffect::Analyses(names))
            }
            "plot" | "print" | "settype" => self
                .present(engine, command, variables, abort)
                .map(ControlCommandEffect::Presentation),
            _ => Err(command_error(
                line,
                format!(
                    "control command '{}' has no electrical or presentation handler",
                    command.name
                ),
            )
            .into()),
        }
    }

    fn run_analysis(
        &mut self,
        engine: &Engine,
        analysis: AnalysisCommand,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlExecutionError> {
        engine
            .ensure_batch_runs(self.datasets.len().saturating_add(1))
            .map_err(|error| simulation_error(line, error))?;
        let mut configured = engine.config().clone();
        configured.resource_limits.max_result_values = configured
            .resource_limits
            .max_result_values
            .saturating_sub(self.retained_values);
        let bounded = engine
            .try_resolved_with_config(configured)
            .map_err(|source| ControlExecutionError::Configuration { line, source })?;
        let mut netlist = self.netlist.clone();
        netlist.analyses = vec![analysis.clone()];
        let mut device_op_report = None;
        let (kind, result, count) = match &analysis {
            AnalysisCommand::Op => {
                let (result, report) = bounded
                    .run_dc_op_with_report_and_abort(&netlist, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = report.entries.iter().fold(
                    Engine::simulation_result_value_count(&result),
                    |count, entry| count.saturating_add(entry.params.len()),
                );
                device_op_report = Some(Box::new(report));
                (
                    "op",
                    ControlAnalysisResult::OperatingPoint(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => {
                let frequencies = crate::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
                    *variation,
                    *points,
                    *start_freq,
                    *stop_freq,
                    bounded.config().resource_limits.max_analysis_points,
                    abort,
                )
                .map_err(|error| command_error(line, error.to_string()))?;
                let result = bounded
                    .run_ac_with_abort(&netlist, &frequencies, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result
                    .iter()
                    .map(|point| {
                        point
                            .voltages
                            .len()
                            .saturating_add(point.currents.len())
                            .saturating_mul(2)
                            .saturating_add(1)
                    })
                    .fold(0usize, usize::saturating_add);
                ("ac", ControlAnalysisResult::Ac(result), count)
            }
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => {
                let maximum_step = crate::execution::resolve_transient_maximum_step(
                    *step, *stop, *start, *max_step,
                )
                .map_err(|error| command_error(line, error.to_string()))?;
                let result = bounded
                    .run_tran_with_startup_mode_and_abort(
                        &netlist,
                        *stop,
                        maximum_step,
                        TransientStartupMode::from_uic(*uic),
                        abort,
                    )
                    .map_err(|error| simulation_error(line, error))?;
                let count = Engine::transient_result_value_count(&result);
                (
                    "tran",
                    ControlAnalysisResult::Transient(Box::new(result)),
                    count,
                )
            }
            _ => {
                return Err(command_error(
                    line,
                    "this analysis has no control-host execution handler",
                )
                .into());
            }
        };
        if abort.is_aborted() {
            return Err(simulation_error(line, SimulationError::Aborted));
        }
        let retained_values = self.retained_values.saturating_add(count);
        engine
            .ensure_result_values(retained_values)
            .map_err(|error| simulation_error(line, error))?;
        let ordinal = self
            .ordinals
            .get(kind)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| command_error(line, "dataset ordinal overflow"))?;
        let name = format!("{kind}{ordinal}");
        let identity_kind = match &result {
            ControlAnalysisResult::OperatingPoint(_) => crate::identity::AnalysisKind::Op,
            ControlAnalysisResult::Ac(_) => crate::identity::AnalysisKind::Ac,
            ControlAnalysisResult::Transient(_) => crate::identity::AnalysisKind::Tran,
        };
        let identity_ordinal = u32::try_from(ordinal - 1)
            .map_err(|_| command_error(line, "analysis identity ordinal overflow"))?;
        self.datasets.push(ControlNamedDataset {
            name: name.clone(),
            analysis_id: crate::identity::AnalysisInstanceId::new(identity_kind, identity_ordinal),
            device_op_report,
            command: analysis,
            result,
        });
        self.ordinals.insert(kind, ordinal);
        self.retained_values = retained_values;
        Ok(name)
    }

    fn alter(
        &mut self,
        _engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let text = command.arguments.trim();
        let split = text
            .find(|character: char| character.is_whitespace() || character == '=')
            .unwrap_or(text.len());
        let target = text.get(..split).unwrap_or_default();
        let value = text.get(split..).unwrap_or_default().trim();
        let value = value.strip_prefix('=').unwrap_or(value).trim();
        if target.is_empty() || value.is_empty() {
            return Err(command_error(line, "alter requires a target and value").into());
        }
        let (name, parameter) = if let Some(target) = target.strip_prefix('@') {
            let (name, parameter) = target
                .split_once('[')
                .ok_or_else(|| command_error(line, "@alter target requires [parameter]"))?;
            let parameter = parameter
                .strip_suffix(']')
                .ok_or_else(|| command_error(line, "unterminated alter parameter"))?;
            (name, Some(parameter))
        } else {
            (target, None)
        };
        let element = self
            .netlist
            .elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| command_error(line, format!("alter target '{name}' was not found")))?;
        let mut candidate = element.kind.clone();
        let mut changes = Vec::new();
        if parameter.is_some_and(|parameter| parameter.eq_ignore_ascii_case("sin")) {
            let values = value
                .strip_prefix('[')
                .and_then(|body| body.strip_suffix(']'))
                .ok_or_else(|| {
                    command_error(line, "SIN alteration requires a bracketed value vector")
                })?;
            let values = values
                .split_whitespace()
                .map(|value| scalar(value, variables, line))
                .collect::<Result<Vec<_>, _>>()?;
            if !(3..=6).contains(&values.len()) {
                return Err(command_error(line, "SIN alteration requires offset, amplitude, frequency and up to three optional values").into());
            }
            for (index, parameter) in ["VO", "VA", "FREQ", "TD", "THETA", "PHASE"]
                .iter()
                .enumerate()
            {
                let value = values.get(index).copied().unwrap_or(0.0);
                Engine::apply_device_step_value(&mut candidate, Some(parameter), value)
                    .map_err(|error| simulation_error(line, error))?;
                let canonical = Engine::canonical_device_parameter(&candidate, Some(parameter));
                changes.push((canonical, value));
            }
        } else {
            let value = scalar(value, variables, line)?;
            let canonical = Engine::canonical_device_parameter(&candidate, parameter);
            Engine::apply_device_step_value(&mut candidate, Some(&canonical), value)
                .map_err(|error| simulation_error(line, error))?;
            changes.push((canonical, value));
        }
        // Publish only after every field has validated and applied to a private
        // candidate. The overlay keeps changes through later source replay.
        element.kind = candidate;
        for (parameter, value) in changes {
            self.netlist
                .ast_overlay
                .device_parameters
                .insert((element.name.to_ascii_uppercase(), parameter), value);
        }
        Ok(())
    }
}

impl ControlScalarEvaluator for ControlCircuit {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
    ) -> Result<Value, ControlError> {
        ParameterScalarEvaluator.evaluate_scalar(expression, variables, line)
    }
}

fn scalar(expression: &str, variables: &ParamContext, line: usize) -> Result<Value, ControlError> {
    let value = eval_expression(expression, variables).map_err(|error| {
        ControlError::new(line, ControlErrorKind::Expression, error.to_string())
    })?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "alter value is nonfinite",
        ))
    }
}

fn command_error(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Host, message)
}

fn simulation_error(line: usize, source: SimulationError) -> ControlExecutionError {
    ControlExecutionError::Simulation { line, source }
}
