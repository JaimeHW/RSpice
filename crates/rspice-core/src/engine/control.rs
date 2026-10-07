//! Electrical host for the shared resumable control-command machine.

use super::{
    Engine, FrequencyDataResult, SimulationConfigError, SimulationError, TransientStartupMode,
};
use crate::analysis::AcResult;
use crate::analysis::transient::TransientResult;
use crate::control_protocol::{
    ControlCommand, ControlError, ControlErrorKind, ControlScalarEvaluator,
};
use crate::netlist::expr::{
    ParamContext, eval_expression_complex, evaluate_complex_with_functions, is_real,
    parse_expression,
};
use crate::netlist::{AnalysisCommand, Netlist};
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::{AbortSignal, ComplexValue, Value};
use std::collections::BTreeMap;

mod analysis;
mod options;
mod presentation;
mod transient;
pub use options::ControlSettings;
pub(super) use options::apply_runtime_options;
pub use presentation::{
    ControlCurrentSource, ControlPlotOptions, ControlPresentation, ControlPresentationKind,
    ControlScalar, ControlTrace, ControlVector, ControlVectorId,
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
    AcTable(FrequencyDataResult<AcResult>),
    Noise(Vec<crate::analysis::NoiseResult>),
    NoiseTable(FrequencyDataResult<crate::analysis::NoiseResult>),
    DcSweep(Box<super::DcSweepResult>),
    TransferFunction(Box<crate::analysis::TransferFunctionResult>),
    PoleZero(Box<crate::analysis::PoleZeroResult>),
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

enum PreparedCommand {
    Complete(ControlCommandEffect),
    Analyses(Vec<(AnalysisCommand, Option<usize>)>),
}

enum CommandKind {
    Options,
    Set,
    Alter,
    Analysis,
    Run,
    Presentation,
}

impl CommandKind {
    fn parse(command: &ControlCommand) -> Result<Self, ControlError> {
        Ok(match command.name.as_str() {
            "option" | "options" => Self::Options,
            "set" => Self::Set,
            "alter" => Self::Alter,
            "op" | "dc" | "ac" | "noise" | "tran" | "tf" | "pz" => Self::Analysis,
            "run" => Self::Run,
            "plot" | "print" | "settype" => Self::Presentation,
            _ => {
                return Err(command_error(
                    command.line,
                    format!(
                        "control command '{}' has no electrical or presentation handler",
                        command.name
                    ),
                ));
            }
        })
    }
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
    vector_units: BTreeMap<ControlVectorId, crate::signal_unit::SignalUnit>,
    settings: ControlSettings,
    runtime_options: crate::netlist::SimulationOptions,
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
            settings: ControlSettings::default(),
            runtime_options: crate::netlist::SimulationOptions::default(),
        })
    }

    pub fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    pub fn settings(&self) -> &ControlSettings {
        &self.settings
    }

    pub fn datasets(&self) -> &[ControlNamedDataset] {
        &self.datasets
    }

    /// Consume the completed session for publication without cloning results.
    pub fn into_datasets(self) -> Vec<ControlNamedDataset> {
        self.datasets
    }

    fn prepare_command(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedCommand, ControlExecutionError> {
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
        match CommandKind::parse(command)? {
            CommandKind::Options => {
                self.apply_options(engine, command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Set => {
                self.apply_set(command, variables)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Alter => {
                self.alter(engine, command, variables)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Analysis => {
                let analysis =
                    Self::parse_analysis_command(command, engine.config().resource_limits, abort)?;
                Ok(PreparedCommand::Analyses(vec![(analysis, None)]))
            }
            CommandKind::Run => {
                if !command.arguments.is_empty() {
                    return Err(command_error(line, "run does not accept arguments").into());
                }
                let analyses = self
                    .netlist
                    .analyses
                    .iter()
                    .enumerate()
                    .filter(|(_, command)| {
                        !matches!(
                            command,
                            AnalysisCommand::Four { .. }
                                | AnalysisCommand::Step(_)
                                | AnalysisCommand::Temp { .. }
                        )
                    })
                    .map(|(index, command)| (command.clone(), Some(index)))
                    .collect::<Vec<_>>();
                if analyses.is_empty() {
                    return Err(
                        command_error(line, "run has no declarative analysis to execute").into(),
                    );
                }
                Ok(PreparedCommand::Analyses(analyses))
            }
            CommandKind::Presentation => {
                self.present(engine, command, variables, abort)
                    .map(|request| {
                        PreparedCommand::Complete(ControlCommandEffect::Presentation(request))
                    })
            }
        }
    }

    fn run_analysis<E, F>(
        &mut self,
        engine: &Engine,
        analysis: AnalysisCommand,
        authored_index: Option<usize>,
        line: usize,
        abort: &dyn AbortSignal,
        runner: &mut F,
    ) -> Result<String, E>
    where
        E: From<ControlExecutionError> + From<ControlError>,
        F: FnMut(
            &Engine,
            &Netlist,
            &AnalysisCommand,
            crate::identity::AnalysisInstanceId,
            &dyn AbortSignal,
        ) -> Result<TransientResult, E>,
    {
        engine
            .ensure_batch_runs(self.datasets.len().saturating_add(1))
            .map_err(|error| simulation_error(line, error))?;
        // Resolve authored options before marking this bounded engine resolved.
        // Then apply only settings changed by executed commands: a caller's
        // resolved policy must not be overwritten by unrelated authored options.
        let resolved = engine.resolved_for_netlist(&self.netlist);
        let mut configured = crate::resolve_simulation_config(
            resolved.config(),
            Some(&self.runtime_options),
            &crate::SimulationConfigOverrides::default(),
        );
        if let Some(workers) = self.settings.maximum_parallel_workers {
            configured.resource_limits.max_parallel_workers =
                configured.resource_limits.max_parallel_workers.min(workers);
        }
        configured.resource_limits.max_result_values = configured
            .resource_limits
            .max_result_values
            .saturating_sub(self.retained_values);
        let bounded = if matches!(
            analysis,
            AnalysisCommand::AcData { .. } | AnalysisCommand::NoiseData { .. }
        ) {
            engine.control_frequency_table_engine(&self.runtime_options, configured.resource_limits)
        } else {
            engine.try_resolved_with_config(configured)
        }
        .map_err(|source| ControlExecutionError::Configuration { line, source })?;
        let (kind, identity_kind) = match &analysis {
            AnalysisCommand::Op => ("op", crate::identity::AnalysisKind::Op),
            AnalysisCommand::Dc { .. } => ("dc", crate::identity::AnalysisKind::Dc),
            AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => {
                ("ac", crate::identity::AnalysisKind::Ac)
            }
            AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => {
                ("noise", crate::identity::AnalysisKind::Noise)
            }
            AnalysisCommand::Tran { .. } => ("tran", crate::identity::AnalysisKind::Tran),
            AnalysisCommand::Tf { .. } => ("tf", crate::identity::AnalysisKind::TransferFunction),
            AnalysisCommand::PoleZero { .. } => ("pz", crate::identity::AnalysisKind::PoleZero),
            _ => {
                return Err(command_error(
                    line,
                    "this analysis has no control-host execution handler",
                )
                .into());
            }
        };
        let ordinal = self
            .ordinals
            .get(kind)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| command_error(line, "dataset ordinal overflow"))?;
        let identity_ordinal = u32::try_from(ordinal - 1)
            .map_err(|_| command_error(line, "analysis identity ordinal overflow"))?;
        let analysis_id = crate::identity::AnalysisInstanceId::new(identity_kind, identity_ordinal);
        let name = format!("{kind}{ordinal}");
        let netlist = transient::analysis_netlist(
            &self.netlist,
            &analysis,
            authored_index,
            line,
            &bounded.config().resource_limits,
            abort,
        )?;
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
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode,
                sweep2,
            } => {
                let spec = crate::netlist::DcSweepSpec {
                    start: *start,
                    stop: *stop,
                    step: *step,
                    mode: mode.clone(),
                };
                let result = bounded
                    .run_dc_analysis_with_abort(&netlist, source, &spec, sweep2.as_ref(), abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.value_count();
                (
                    "dc",
                    ControlAnalysisResult::DcSweep(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Ac { .. } | AnalysisCommand::Noise { .. } => {
                let (result, count) =
                    analysis::frequency(&bounded, &netlist, &analysis, line, abort)?;
                (kind, result, count)
            }
            AnalysisCommand::AcData { .. } | AnalysisCommand::NoiseData { .. } => {
                let (result, count) =
                    analysis::frequency_table(&bounded, &netlist, &analysis, line, abort)?;
                (kind, result, count)
            }
            AnalysisCommand::Tran { .. } => {
                let result = runner(&bounded, &netlist, &analysis, analysis_id, abort)?;
                let count = Engine::transient_result_value_count(&result);
                (
                    "tran",
                    ControlAnalysisResult::Transient(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::PoleZero { .. } => {
                let result = bounded
                    .run_pz_from_card_with_abort(&netlist, &analysis, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.retained_value_count();
                (
                    kind,
                    ControlAnalysisResult::PoleZero(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => {
                // The fixed scalar result can be admitted before solving, with
                // cumulative diagnostics against the caller's full allowance.
                engine
                    .ensure_result_values(self.retained_values.saturating_add(3))
                    .map_err(|error| simulation_error(line, error))?;
                let result = bounded
                    .run_transfer_function_with_abort(
                        &netlist,
                        output_node,
                        reference_node.as_deref(),
                        *output_is_current,
                        input_source,
                        abort,
                    )
                    .map_err(|error| simulation_error(line, error))?;
                (
                    kind,
                    ControlAnalysisResult::TransferFunction(Box::new(result)),
                    3,
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
            return Err(simulation_error(line, SimulationError::Aborted).into());
        }
        let retained_values = self.retained_values.saturating_add(count);
        engine
            .ensure_result_values(retained_values)
            .map_err(|error| simulation_error(line, error))?;
        self.datasets.push(ControlNamedDataset {
            name: name.clone(),
            analysis_id,
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
    ) -> Result<ComplexValue, ControlError> {
        parse_expression(expression)
            .and_then(|expression| {
                evaluate_complex_with_functions(
                    &expression,
                    variables,
                    &mut |name| presentation::resolve_scalar(self, name),
                    &mut |name, args| presentation::resolve_root_function(self, name, args),
                )
            })
            .map_err(|error| {
                ControlError::new(line, ControlErrorKind::Expression, error.to_string())
            })
    }
}

fn scalar(expression: &str, variables: &ParamContext, line: usize) -> Result<Value, ControlError> {
    let value = eval_expression_complex(expression, variables).map_err(|error| {
        ControlError::new(line, ControlErrorKind::Expression, error.to_string())
    })?;
    if !value.re.is_finite() || !value.im.is_finite() {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "command value is nonfinite",
        ))
    } else if !is_real(value) {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "command requires a real value; use real(), imag() or mag() explicitly",
        ))
    } else {
        Ok(value.re)
    }
}

fn command_error(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Host, message)
}

fn simulation_error(line: usize, source: SimulationError) -> ControlExecutionError {
    ControlExecutionError::Simulation { line, source }
}
