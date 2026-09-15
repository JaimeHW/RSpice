//! Resolve control output against immutable, named analysis datasets.

use super::*;
use crate::ComplexValue;
use crate::execution::SignalUnit;
use crate::netlist::expr::{
    BinOpKind, Expr, ParseExpressionWithAbortError, UnaryOpKind, evaluate_complex,
    parse_control_expression_prefix_with_abort,
};

/// A canonical vector identity. Dataset names remain stable for the session.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlVectorId {
    pub dataset: String,
    pub signal: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCurrentSource {
    pub dataset: String,
    pub owner: crate::CurrentImpulseOwner,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlVector {
    pub expression: String,
    /// Dataset supplying the sample grid. Other operands must have the exact
    /// same physical grid; this resolver never interpolates between runs.
    pub dataset: String,
    pub unit: SignalUnit,
    /// Finite waveform samples, preserving complex AC values.
    pub samples: Vec<ComplexValue>,
    /// Transient current inputs to this expression. Their independent charge
    /// events and coverage remain in the named datasets' `current_impulses`.
    /// A renderer must accompany these samples with that information. These
    /// identities do not assert that a nonlinear expression has a defined
    /// impulse transform, or that unrecorded impulses are absent.
    pub current_sources: Vec<ControlCurrentSource>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlTrace {
    pub x: ControlVector,
    pub y: ControlVector,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ControlPlotOptions {
    pub title: Option<String>,
    pub x_limits: Option<[Value; 2]>,
    pub y_limits: Option<[Value; 2]>,
    pub x_logarithmic: bool,
    pub y_logarithmic: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlPresentationKind {
    Plot {
        traces: Vec<ControlTrace>,
        options: ControlPlotOptions,
    },
    Print(Vec<ControlTrace>),
    /// Metadata has already been updated atomically by the circuit host.
    UnitsChanged {
        vectors: Vec<ControlVectorId>,
        unit: SignalUnit,
    },
}

#[derive(Debug, Clone)]
pub struct ControlPresentation {
    pub command: ControlCommand,
    pub kind: ControlPresentationKind,
}

#[derive(Clone, Copy)]
enum Column {
    Ground,
    Node(usize),
    Branch(usize),
    Scale,
}

struct Selected<'a> {
    dataset: &'a ControlNamedDataset,
    id: ControlVectorId,
    column: Column,
    unit: SignalUnit,
}

impl ControlNamedDataset {
    fn length(&self) -> usize {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(_) => 1,
            ControlAnalysisResult::Ac(points) => points.len(),
            ControlAnalysisResult::Transient(result) => result.time.len(),
        }
    }

    fn scale_unit(&self) -> SignalUnit {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(_) => SignalUnit::Dimensionless,
            ControlAnalysisResult::Ac(_) => SignalUnit::Hertz,
            ControlAnalysisResult::Transient(_) => SignalUnit::Second,
        }
    }

    fn scale_name(&self) -> &'static str {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(_) => "index",
            ControlAnalysisResult::Ac(_) => "frequency",
            ControlAnalysisResult::Transient(_) => "time",
        }
    }

    fn scale_value(&self, row: usize) -> Option<Value> {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(_) => (row == 0).then_some(0.0),
            ControlAnalysisResult::Ac(points) => points.get(row).map(|point| point.frequency),
            ControlAnalysisResult::Transient(result) => result.time.get(row).copied(),
        }
    }

    fn branch_names(&self) -> &[String] {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(result) => &result.branch_names,
            ControlAnalysisResult::Ac(points) => points.first().map_or(&[], |p| &p.branch_names),
            ControlAnalysisResult::Transient(result) => &result.branch_names,
        }
    }
}

impl Selected<'_> {
    fn sample(&self, row: usize) -> Option<ComplexValue> {
        if row >= self.dataset.length() {
            return None;
        }
        match (self.column, &self.dataset.result) {
            (Column::Ground, _) => Some(0.0.into()),
            (Column::Scale, _) => self.dataset.scale_value(row).map(Into::into),
            (Column::Node(index), ControlAnalysisResult::OperatingPoint(result)) => {
                result.try_voltage(index).map(Into::into)
            }
            (Column::Branch(index), ControlAnalysisResult::OperatingPoint(result)) => {
                result.branch_current(index).map(Into::into)
            }
            (Column::Node(index), ControlAnalysisResult::Ac(points)) => {
                let point = points.get(row)?;
                (point.node_names == points.first()?.node_names)
                    .then(|| point.voltages.get(index).copied())
                    .flatten()
            }
            (Column::Branch(index), ControlAnalysisResult::Ac(points)) => {
                let point = points.get(row)?;
                (point.branch_names == points.first()?.branch_names)
                    .then(|| point.currents.get(index).copied())
                    .flatten()
            }
            (Column::Node(index), ControlAnalysisResult::Transient(result)) => {
                let waveform = result.voltages.get(index)?;
                (waveform.len() == result.time.len())
                    .then(|| waveform.get(row).copied().map(Into::into))
                    .flatten()
            }
            (Column::Branch(index), ControlAnalysisResult::Transient(result)) => {
                let waveform = result.branch_currents.get(index)?;
                (waveform.len() == result.time.len())
                    .then(|| waveform.get(row).copied().map(Into::into))
                    .flatten()
            }
        }
    }
}

impl ControlCircuit {
    fn dataset(&self, name: &str) -> Option<&ControlNamedDataset> {
        self.datasets
            .iter()
            .find(|dataset| dataset.name.eq_ignore_ascii_case(name))
    }

    fn qualified<'a, 'b>(
        &'a self,
        name: &'b str,
        line: usize,
    ) -> Result<(&'a ControlNamedDataset, &'b str), ControlError> {
        if let Some((prefix, rest)) = name.split_once('.')
            && let Some(dataset) = self.dataset(prefix)
        {
            return Ok((dataset, rest));
        }
        self.datasets
            .last()
            .map(|dataset| (dataset, name))
            .ok_or_else(|| command_error(line, "output requires a completed analysis dataset"))
    }

    fn select<'a>(
        &'a self,
        dataset: &'a ControlNamedDataset,
        name: &str,
        probe: Option<&str>,
        line: usize,
    ) -> Result<Selected<'a>, ControlError> {
        let lower = name.to_ascii_lowercase();
        let branch = probe == Some("I") || (probe.is_none() && lower.ends_with("#branch"));
        let (column, signal, unit) = if branch {
            let name = if probe.is_none() {
                &lower[..lower.len() - 7]
            } else {
                &lower
            };
            let index = dataset
                .branch_names()
                .iter()
                .position(|item| item.eq_ignore_ascii_case(name))
                .ok_or_else(|| unavailable(line, dataset, name))?;
            (
                Column::Branch(index),
                format!("i({name})"),
                SignalUnit::Ampere,
            )
        } else if probe.is_none() && lower == dataset.scale_name() {
            (Column::Scale, lower, dataset.scale_unit())
        } else if matches!(lower.as_str(), "0" | "gnd" | "gnd!") {
            (Column::Ground, "v(0)".into(), SignalUnit::Volt)
        } else {
            let names = match &dataset.result {
                ControlAnalysisResult::OperatingPoint(result) => &result.node_names,
                ControlAnalysisResult::Ac(points) => {
                    &points
                        .first()
                        .ok_or_else(|| unavailable(line, dataset, name))?
                        .node_names
                }
                ControlAnalysisResult::Transient(result) => &result.node_names,
            };
            let index = names
                .iter()
                .position(|item| item.eq_ignore_ascii_case(name))
                .ok_or_else(|| unavailable(line, dataset, name))?;
            (Column::Node(index), format!("v({lower})"), SignalUnit::Volt)
        };
        let id = ControlVectorId {
            dataset: dataset.name.clone(),
            signal,
        };
        let unit = self.vector_units.get(&id).cloned().unwrap_or(unit);
        Ok(Selected {
            dataset,
            id,
            column,
            unit,
        })
    }

    fn direct<'a>(&'a self, expression: &Expr, line: usize) -> Result<Selected<'a>, ControlError> {
        match expression {
            Expr::Param(name) => {
                let (dataset, name) = self.qualified(name, line)?;
                self.select(dataset, name, None, line)
            }
            Expr::FnCall { name, args } => {
                let (dataset, probe) = self.qualified(name, line)?;
                if !matches!(probe, "V" | "N" | "I") {
                    return Err(command_error(
                        line,
                        "a vector reference must name V, N, I or a retained raw vector",
                    ));
                }
                let [Expr::Param(name)] = args.as_slice() else {
                    return Err(command_error(
                        line,
                        "a direct vector reference requires one raw name",
                    ));
                };
                self.select(dataset, name, Some(probe), line)
            }
            _ => Err(command_error(
                line,
                "settype requires direct vector references",
            )),
        }
    }

    fn group<'a>(
        &'a self,
        expression: &Expr,
        line: usize,
    ) -> Result<Option<Vec<Selected<'a>>>, ControlError> {
        let Expr::Param(name) = expression else {
            return Ok(None);
        };
        let (dataset, name) = self.qualified(name, line)?;
        if !name.eq_ignore_ascii_case("alli") {
            return Ok(None);
        }
        let values = dataset
            .branch_names()
            .iter()
            .map(|name| self.select(dataset, name, Some("I"), line))
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            return Err(command_error(
                line,
                format!("{}.alli contains no retained currents", dataset.name),
            ));
        }
        Ok(Some(values))
    }

    pub(super) fn present(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ControlPresentation, ControlExecutionError> {
        let line = command.line;
        let mut input = command.arguments.trim();
        let kind = if command.name == "settype" {
            let type_name = word(&mut input, line)?;
            let unit = match type_name.to_ascii_lowercase().as_str() {
                "voltage" => SignalUnit::Volt,
                "current" => SignalUnit::Ampere,
                "impedance" | "resistance" => SignalUnit::Ohm,
                "admittance" | "conductance" => SignalUnit::Siemens,
                "frequency" => SignalUnit::Hertz,
                "time" => SignalUnit::Second,
                "notype" => SignalUnit::Unspecified,
                _ => {
                    return Err(command_error(
                        line,
                        format!("unsupported vector type '{type_name}'"),
                    )
                    .into());
                }
            };
            let mut vectors = Vec::new();
            while !input.is_empty() {
                let (expression, _) = expression(&mut input, line, abort)?;
                let selected = match self.group(&expression, line)? {
                    Some(group) => group,
                    None => vec![self.direct(&expression, line)?],
                };
                for selected in selected {
                    if selected.sample(0).is_none() {
                        return Err(unavailable(line, selected.dataset, &selected.id.signal).into());
                    }
                    vectors.push(selected.id);
                }
                engine
                    .ensure_result_values(self.retained_values.saturating_add(vectors.len()))
                    .map_err(|error| simulation_error(line, error))?;
            }
            if vectors.is_empty() {
                return Err(command_error(line, "settype requires at least one vector").into());
            }
            check_abort(abort, line)?;
            for id in &vectors {
                self.vector_units.insert(id.clone(), unit.clone());
            }
            ControlPresentationKind::UnitsChanged { vectors, unit }
        } else {
            let mut resolver = Resolver {
                circuit: self,
                engine,
                variables,
                abort,
                line,
                used_values: self.retained_values,
            };
            let mut traces = Vec::new();
            let mut options = ControlPlotOptions::default();
            while !input.is_empty() {
                check_abort(abort, line)?;
                if command.name == "plot"
                    && parse_option(&mut input, &mut options, variables, line)?
                {
                    continue;
                }
                let (y, label) = expression(&mut input, line, abort)?;
                let x = if first_word(input).eq_ignore_ascii_case("vs") {
                    word(&mut input, line)?;
                    Some(expression(&mut input, line, abort)?)
                } else {
                    None
                };
                let ys = if let Some(group) = self.group(&y, line)? {
                    group
                        .into_iter()
                        .map(|selected| {
                            let label = format!("{}.{}", selected.id.dataset, selected.id.signal);
                            resolver.materialize(
                                Expr::Param("\0V0".into()),
                                vec![selected],
                                label,
                                None,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?
                } else {
                    vec![resolver.evaluate(y, label, None)?]
                };
                for y in ys {
                    let dataset = self.dataset(&y.dataset).expect("resolved dataset exists");
                    let x = if let Some((expression, label)) = &x {
                        resolver.evaluate(expression.clone(), label.clone(), Some(dataset))?
                    } else {
                        resolver.charge(dataset.length().saturating_mul(2))?;
                        let mut samples = Vec::with_capacity(dataset.length());
                        for row in 0..dataset.length() {
                            check_abort(abort, line)?;
                            let value = dataset
                                .scale_value(row)
                                .filter(|v| v.is_finite())
                                .ok_or_else(|| {
                                    command_error(line, "dataset scale is unavailable or nonfinite")
                                })?;
                            samples.push(value.into());
                        }
                        ControlVector {
                            expression: dataset.scale_name().into(),
                            dataset: dataset.name.clone(),
                            unit: dataset.scale_unit(),
                            samples,
                            current_sources: Vec::new(),
                        }
                    };
                    resolver.same_grid(
                        dataset,
                        self.dataset(&x.dataset).expect("resolved dataset exists"),
                    )?;
                    traces.push(ControlTrace { x, y });
                }
            }
            if traces.is_empty() {
                return Err(
                    command_error(line, "output requires at least one vector expression").into(),
                );
            }
            if command.name == "plot" {
                validate_log_limits(&options, line)?;
                ControlPresentationKind::Plot { traces, options }
            } else {
                ControlPresentationKind::Print(traces)
            }
        };
        Ok(ControlPresentation {
            command: command.clone(),
            kind,
        })
    }
}

struct Resolver<'a> {
    circuit: &'a ControlCircuit,
    engine: &'a Engine,
    variables: &'a ParamContext,
    abort: &'a dyn AbortSignal,
    line: usize,
    used_values: usize,
}

impl<'a> Resolver<'a> {
    fn charge(&mut self, count: usize) -> Result<(), ControlExecutionError> {
        self.used_values = self.used_values.saturating_add(count);
        self.engine
            .ensure_result_values(self.used_values)
            .map_err(|error| simulation_error(self.line, error))
    }

    fn same_grid(
        &self,
        a: &ControlNamedDataset,
        b: &ControlNamedDataset,
    ) -> Result<(), ControlExecutionError> {
        if std::ptr::eq(a, b) {
            return Ok(());
        }
        if a.scale_unit() != b.scale_unit() || a.length() != b.length() {
            return Err(command_error(
                self.line,
                format!("{} and {} have different sample grids", a.name, b.name),
            )
            .into());
        }
        for row in 0..a.length() {
            check_abort(self.abort, self.line)?;
            if a.scale_value(row) != b.scale_value(row) {
                return Err(command_error(
                    self.line,
                    format!("{} and {} have different sample grids", a.name, b.name),
                )
                .into());
            }
        }
        Ok(())
    }

    fn evaluate(
        &mut self,
        mut expression: Expr,
        label: String,
        preferred: Option<&'a ControlNamedDataset>,
    ) -> Result<ControlVector, ControlExecutionError> {
        let mut inputs = Vec::new();
        let unit = self.bind(&mut expression, &mut inputs)?;
        expression
            .ensure_stack_safe_depth()
            .map_err(|error| command_error(self.line, error.to_string()))?;
        let mut result = self.materialize(expression, inputs, label, preferred)?;
        result.unit = unit;
        Ok(result)
    }

    fn bind(
        &self,
        expression: &mut Expr,
        inputs: &mut Vec<Selected<'a>>,
    ) -> Result<SignalUnit, ControlExecutionError> {
        check_abort(self.abort, self.line)?;
        let unit = match expression {
            Expr::Number(_) | Expr::ComplexNumber(_) => SignalUnit::Dimensionless,
            Expr::Param(name) => {
                let (dataset, raw) = self.circuit.qualified(name, self.line)?;
                if let Ok(selected) = self.circuit.select(dataset, raw, None, self.line) {
                    return Ok(bind_selected(expression, selected, inputs));
                }
                let value = self.variables.get_complex(name).ok_or_else(|| {
                    command_error(
                        self.line,
                        format!("vector or scalar '{name}' is unavailable"),
                    )
                })?;
                *expression = Expr::ComplexNumber(value);
                SignalUnit::Unspecified
            }
            Expr::FnCall { name, args } => {
                let probe = name.rsplit('.').next().unwrap_or(name);
                if matches!(probe, "V" | "N" | "I") {
                    if matches!(probe, "V" | "N") && args.len() == 2 {
                        let mut first = Expr::FnCall {
                            name: name.clone(),
                            args: vec![args[0].clone()],
                        };
                        let mut second = Expr::FnCall {
                            name: name.clone(),
                            args: vec![args[1].clone()],
                        };
                        let a = self.bind(&mut first, inputs)?;
                        let b = self.bind(&mut second, inputs)?;
                        *expression = Expr::BinOp {
                            op: BinOpKind::Sub,
                            left: Box::new(first),
                            right: Box::new(second),
                        };
                        return Ok(binary_unit(BinOpKind::Sub, a, b));
                    }
                    let selected = self.circuit.direct(expression, self.line)?;
                    return Ok(bind_selected(expression, selected, inputs));
                }
                // Only functions whose control-vector semantics are implemented
                // here are accepted. In particular, scalar PH uses degrees and
                // cannot stand in for the control language's phase policy.
                let canonical = match name.as_str() {
                    "ABS" | "MAG" | "MAGNITUDE" => "ABS",
                    "REAL" | "RE" => "REAL",
                    _ => {
                        return Err(command_error(
                            self.line,
                            format!("control vector function '{name}' is not supported"),
                        )
                        .into());
                    }
                };
                if args.len() != 1 {
                    return Err(command_error(
                        self.line,
                        format!("{name} requires one vector expression"),
                    )
                    .into());
                }
                let unit = self.bind(&mut args[0], inputs)?;
                *name = canonical.into();
                unit
            }
            Expr::BinOp { op, left, right } => {
                binary_unit(*op, self.bind(left, inputs)?, self.bind(right, inputs)?)
            }
            Expr::UnaryOp { op, operand } => {
                let unit = self.bind(operand, inputs)?;
                if *op == UnaryOpKind::Not {
                    SignalUnit::Dimensionless
                } else {
                    unit
                }
            }
            Expr::StringLiteral(_) => {
                return Err(command_error(self.line, "a string is not a numeric vector").into());
            }
        };
        Ok(unit)
    }

    fn materialize(
        &mut self,
        expression: Expr,
        inputs: Vec<Selected<'a>>,
        label: String,
        preferred: Option<&'a ControlNamedDataset>,
    ) -> Result<ControlVector, ControlExecutionError> {
        let dataset = inputs
            .first()
            .map(|input| input.dataset)
            .or(preferred)
            .or_else(|| self.circuit.datasets.last())
            .ok_or_else(|| {
                command_error(self.line, "output requires a completed analysis dataset")
            })?;
        for input in &inputs {
            self.same_grid(dataset, input.dataset)?;
        }
        self.charge(
            dataset
                .length()
                .saturating_mul(2)
                .saturating_add(inputs.len().saturating_mul(2)),
        )?;
        let keys: Vec<_> = (0..inputs.len())
            .map(|index| format!("\0V{index}"))
            .collect();
        let mut context = ParamContext::new();
        let mut samples = Vec::with_capacity(dataset.length());
        for row in 0..dataset.length() {
            check_abort(self.abort, self.line)?;
            for (input, key) in inputs.iter().zip(&keys) {
                let value = input
                    .sample(row)
                    .filter(|v| v.re.is_finite() && v.im.is_finite())
                    .ok_or_else(|| unavailable(self.line, input.dataset, &input.id.signal))?;
                context.set_complex(key, value);
            }
            let value = evaluate_complex(&expression, &context)
                .map_err(|error| command_error(self.line, error.to_string()))?;
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(command_error(
                    self.line,
                    format!("'{label}' is nonfinite at sample {row}"),
                )
                .into());
            }
            samples.push(value);
        }
        let mut current_sources = Vec::new();
        for input in &inputs {
            if let Column::Branch(index) = input.column
                && matches!(input.dataset.result, ControlAnalysisResult::Transient(_))
            {
                let source = ControlCurrentSource {
                    dataset: input.dataset.name.clone(),
                    owner: crate::CurrentImpulseOwner::Branch {
                        branch_name: input.dataset.branch_names()[index].to_ascii_lowercase(),
                    },
                };
                if !current_sources.contains(&source) {
                    current_sources.push(source);
                }
            }
        }
        Ok(ControlVector {
            expression: label,
            dataset: dataset.name.clone(),
            unit: inputs
                .first()
                .map_or(SignalUnit::Dimensionless, |input| input.unit.clone()),
            samples,
            current_sources,
        })
    }
}

fn bind_selected<'a>(
    expression: &mut Expr,
    selected: Selected<'a>,
    inputs: &mut Vec<Selected<'a>>,
) -> SignalUnit {
    // NUL cannot occur in an authored identifier. Scalars are snapshotted into
    // literals, so neither user bindings nor function scopes can capture this.
    *expression = Expr::Param(format!("\0V{}", inputs.len()));
    let unit = selected.unit.clone();
    inputs.push(selected);
    unit
}

fn binary_unit(op: BinOpKind, a: SignalUnit, b: SignalUnit) -> SignalUnit {
    use BinOpKind::*;
    use SignalUnit::*;
    match op {
        Gt | Lt | Ge | Le | Eq | Ne | And | Or => Dimensionless,
        Add | Sub if a == b => a,
        Mul if a == Dimensionless => b,
        Mul | Div if b == Dimensionless => a,
        Div if a == b && a != Unspecified => Dimensionless,
        Div if a == Volt && b == Ampere => Ohm,
        Div if a == Ampere && b == Volt => Siemens,
        Mul if (a == Volt && b == Ampere) || (a == Ampere && b == Volt) => Watt,
        _ => Unspecified,
    }
}

fn check_abort(abort: &dyn AbortSignal, line: usize) -> Result<(), ControlExecutionError> {
    if abort.is_aborted() {
        Err(simulation_error(line, SimulationError::Aborted))
    } else {
        Ok(())
    }
}

fn unavailable(line: usize, dataset: &ControlNamedDataset, name: &str) -> ControlError {
    command_error(
        line,
        format!(
            "vector '{}.{name}' is unavailable in the retained result",
            dataset.name
        ),
    )
}

fn expression(
    input: &mut &str,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(Expr, String), ControlExecutionError> {
    let (expression, length) =
        parse_control_expression_prefix_with_abort(input, abort).map_err(|error| match error {
            ParseExpressionWithAbortError::Aborted => {
                simulation_error(line, SimulationError::Aborted)
            }
            ParseExpressionWithAbortError::Parse(error) => {
                ControlError::new(line, ControlErrorKind::Expression, error.to_string()).into()
            }
        })?;
    let label = input[..length].trim().to_string();
    *input = input[length..].trim_start();
    Ok((expression, label))
}

fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or_default()
}

fn word<'a>(input: &mut &'a str, line: usize) -> Result<&'a str, ControlError> {
    let token = first_word(input);
    if token.is_empty() {
        return Err(command_error(line, "missing command argument"));
    }
    *input = input[token.len()..].trim_start();
    Ok(token)
}

fn parse_option(
    input: &mut &str,
    options: &mut ControlPlotOptions,
    variables: &ParamContext,
    line: usize,
) -> Result<bool, ControlError> {
    let name = first_word(input).to_ascii_lowercase();
    match name.as_str() {
        "title" => {
            word(input, line)?;
            let title = if let Some(quote @ ('\'' | '"')) = input.chars().next() {
                let rest = &input[1..];
                let end = rest
                    .find(quote)
                    .ok_or_else(|| command_error(line, "unterminated plot title"))?;
                let title = rest[..end].to_string();
                let remaining = &rest[end + 1..];
                if !remaining.is_empty() && !remaining.starts_with(char::is_whitespace) {
                    return Err(command_error(
                        line,
                        "plot title must end at an argument boundary",
                    ));
                }
                *input = remaining.trim_start();
                title
            } else {
                word(input, line)?.to_string()
            };
            options.title = Some(title);
        }
        "xlimit" | "ylimit" => {
            word(input, line)?;
            let low = scalar(word(input, line)?, variables, line)?;
            let high = scalar(word(input, line)?, variables, line)?;
            if low >= high {
                return Err(command_error(line, "plot limits must be increasing"));
            }
            if name == "xlimit" {
                options.x_limits = Some([low, high])
            } else {
                options.y_limits = Some([low, high])
            }
        }
        "loglog" | "xlog" | "ylog" | "linear" => {
            word(input, line)?;
            options.x_logarithmic = matches!(name.as_str(), "loglog" | "xlog");
            options.y_logarithmic = matches!(name.as_str(), "loglog" | "ylog");
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn validate_log_limits(options: &ControlPlotOptions, line: usize) -> Result<(), ControlError> {
    for (log, limits) in [
        (options.x_logarithmic, options.x_limits),
        (options.y_logarithmic, options.y_limits),
    ] {
        if log && limits.is_some_and(|limits| limits[0] <= 0.0) {
            return Err(command_error(
                line,
                "logarithmic plot limits must be positive",
            ));
        }
    }
    Ok(())
}
