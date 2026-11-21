//! `.ASSERT` evaluation over a published result.
//!
//! An assertion is evaluated after each analysis it applies to, over that
//! analysis's accepted result, in the same pass as `.MEASURE`. It reads the
//! result and changes nothing.
//!
//! **Transient.** The timeline is the union of the accepted rows and the
//! instants of every event operand. On each segment of it, an event operand
//! (`D()`, `E()`) is constant and an analog operand (`V()`, `I()`,
//! `@inst[param]`) is linear, so every bound and every comparison is solved on
//! that interpolant, never sampled: a crossing of an affine check is the root
//! of a line. A **violation** is a maximal half-open interval `[start, end)`
//! on which the check is false; one still false at the stop time is open.
//! `DURATION=d` drops a violation shorter than `d` and keeps one exactly `d`
//! long.
//!
//! **Four-state.** X and Z are undefined values. A comparison, an arithmetic
//! operator or a function with an undefined operand is undefined; `&&` and
//! `||` are false and true as soon as one known operand decides them (IEEE
//! 1800-2017 §11.4.7), and `isunknown()` is always known. A check that is
//! undefined fails (§16.3), unless the card says `XZ=IGNORE`: then the stretch
//! neither passes nor fails and is counted.
//!
//! **Operating point and DC sweep.** Each solved point is checked. A sweep
//! reports the ranges of the swept value its consecutive failing points span.
//!
//! **Device forms** expand to every instance the target names, in elaboration
//! order, each with its own report.
//!
//! **Streamed.** A transient kept in a results store is checked without the
//! run in memory ([`TranAssertStream`]): each check walks the accepted rows in
//! order as the store gives them back, a chunk at a time, through the same
//! walk the evaluation in memory feeds the run's rows to, so the two classify
//! the same pieces with the same arithmetic.

use std::borrow::Cow;
use std::collections::HashMap;

use super::error::SimulationError;
use super::measure::events::{Level, Steps, event_steps, event_steps_within};
use super::measure_signals::{DcSweepSeries, retained_transient_signal_map};
use super::transient::TransientResult;
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::circuit::DeviceOpReport;
use crate::netlist::expr::{BinOpKind, Expr, UnaryOpKind};
use crate::netlist::{
    AssertAnalysis, AssertCheck, AssertLevel, AssertSample, AssertStatement, AssertTarget, Element,
    ElementKind, Flattener, Netlist, NetlistSourceLocation, ParamContext, canonical_symbol,
};
use crate::solver::SimulationResult;

/// One stretch on which an assertion's check is false.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertViolation {
    /// Where it starts: a time (tran) or a swept value (dc); `None` at an
    /// operating point.
    pub start: Option<Value>,
    /// Where it ends, exclusive in time and inclusive on a sweep; `None` when
    /// it was still open at the stop time, and at an operating point.
    pub end: Option<Value>,
    /// The checked value farthest outside its band, for `EXPR=` and the
    /// device forms; `None` for `COND=`.
    pub extreme: Option<Value>,
    /// Where that value was reached; `None` at an operating point.
    pub at: Option<Value>,
}

/// Expected outputs a vector file states for after the transient's stop
/// time, which no run that stopped there can check.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplesAfterStop {
    /// How many expected-output samples fall after the stop time.
    pub count: usize,
    /// The earliest of their times, in seconds.
    pub first_time: Value,
}

/// One assertion's outcome after one analysis, for one checked instance.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionReport {
    /// The card's name.
    pub name: String,
    /// The analysis it was evaluated after.
    pub analysis: AssertAnalysis,
    /// What the card checks: its expression, its condition, or
    /// `@inst[param]` for a device form.
    pub target: String,
    /// The checked instance, for the device forms.
    pub instance: Option<String>,
    pub level: AssertLevel,
    pub message: Option<String>,
    /// Where the card was written.
    pub origin: NetlistSourceLocation,
    /// The violations, in order.
    pub violations: Vec<AssertViolation>,
    /// Under `XZ=IGNORE`, the stretches on which the check was undefined.
    pub unknown_stretches: usize,
    /// Why the assertion could not be evaluated; `None` when it was.
    pub error: Option<String>,
    /// For a vector file's warning that some of its expected outputs fall
    /// after the stop time, how many and the first of them; `None` for every
    /// other report. The message says the same in words.
    pub after_stop: Option<SamplesAfterStop>,
}

/// Evaluate every assertion that applies to a transient analysis over its
/// accepted trajectory.
pub fn evaluate_tran_assertions(
    netlist: &Netlist,
    result: &TransientResult,
    abort: &dyn AbortSignal,
) -> Result<Vec<AssertionReport>, SimulationError> {
    let statements = applicable(netlist, AssertAnalysis::Tran);
    if statements.is_empty() && netlist.vector_files.is_empty() {
        return Ok(Vec::new());
    }
    let signals = retained_transient_signal_map(result);
    let source = Source {
        analysis: AssertAnalysis::Tran,
        axis: &result.time,
        signals,
        params: &netlist.params,
        transient: Some(result),
        span: None,
        devices: None,
    };
    let mut reports = Vec::new();
    for statement in statements {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        evaluate_statement(netlist, statement, &source, &mut reports);
    }
    // A `.VEC` file's expected outputs, read from the file the build drove
    // the inputs from.
    let limit = crate::resource::ResourceLimits::default().max_external_data_bytes;
    for reference in &netlist.vector_files {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        match load_vector_file(reference, &netlist.params, limit) {
            Ok(file) => {
                let statements = file.output_checks(reference);
                for statement in &statements {
                    evaluate_statement(netlist, statement, &source, &mut reports);
                }
                reports.extend(samples_after_stop(reference, &statements, &result.time));
            }
            Err(error) => reports.push(AssertionReport {
                name: reference.report_name(),
                analysis: AssertAnalysis::Tran,
                target: reference.path.clone(),
                instance: None,
                level: reference.level,
                message: None,
                origin: reference.origin.clone(),
                violations: Vec::new(),
                unknown_stretches: 0,
                error: Some(error),
                after_stop: None,
            }),
        }
    }
    Ok(reports)
}

/// The one WARNING a vector file gets when the transient stops before some
/// of its expected outputs are due: those samples are not checked, so the
/// finding counts them and names the first row that states one. It is not a
/// violation.
fn samples_after_stop(
    reference: &crate::netlist::VectorFileReference,
    statements: &[AssertStatement],
    time: &[Value],
) -> Option<AssertionReport> {
    let &stop = time.last()?;
    let late: Vec<&AssertSample> = statements
        .iter()
        .filter_map(|statement| match &statement.check {
            AssertCheck::Sampled { samples, .. } => Some(samples),
            _ => None,
        })
        .flatten()
        .filter(|sample| sample.time > stop)
        .collect();
    let first = late.iter().min_by_key(|sample| sample.origin.line)?;
    let count = late.len();
    let first_time = late
        .iter()
        .map(|sample| sample.time)
        .fold(Value::INFINITY, Value::min);
    Some(AssertionReport {
        name: reference.report_name(),
        analysis: AssertAnalysis::Tran,
        target: reference.path.clone(),
        instance: None,
        level: AssertLevel::Warning,
        message: Some(format!(
            "{count} expected-output sample{} after the transient's stop time {stop:e} s {} \
             not checked; the first is on the row at {}",
            if count == 1 { "" } else { "s" },
            if count == 1 { "is" } else { "are" },
            first.origin
        )),
        origin: first.origin.clone(),
        violations: Vec::new(),
        unknown_stretches: 0,
        error: None,
        after_stop: Some(SamplesAfterStop { count, first_time }),
    })
}

/// Read and parse the vector file a `.VEC` card names, refusing what RSpice
/// does not realize. The file comes from the data files a prepared run
/// registered under the card's spelling, or from disk; an error names the
/// file and the line.
pub(crate) fn load_vector_file(
    reference: &crate::netlist::VectorFileReference,
    params: &ParamContext,
    limit: usize,
) -> Result<crate::netlist::hspice_vec::VectorFile, String> {
    let text = crate::xspice::read_data_file_to_string_limited(&reference.path, limit).map_err(
        |error| {
            format!(
                "{}: the vector file '{}' cannot be read: {error}",
                reference.origin, reference.path
            )
        },
    )?;
    crate::netlist::hspice_vec::parse(&text, &|name| params.get(name))
        .and_then(|file| file.check_realizable().map(|()| file))
        .map_err(|error| {
            let location = if error.line == 0 {
                reference.path.clone()
            } else {
                format!("{}:{}", reference.path, error.line)
            };
            format!(
                "{location}: {} (vector file of the .VEC card at {})",
                error.message, reference.origin
            )
        })
}

/// Evaluate every assertion that applies to an operating point. `devices` is
/// the operating-point report the device forms and `@inst[param]` read.
pub fn evaluate_op_assertions(
    netlist: &Netlist,
    result: &SimulationResult,
    devices: &DeviceOpReport,
) -> Vec<AssertionReport> {
    let statements = applicable(netlist, AssertAnalysis::Op);
    if statements.is_empty() {
        return Vec::new();
    }
    solved_points(
        netlist,
        AssertAnalysis::Op,
        &[(0.0, result.clone())],
        Some(devices),
        &statements,
    )
}

/// Evaluate every assertion that applies to a DC sweep, point by point.
pub fn evaluate_dc_assertions(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<AssertionReport> {
    let statements = applicable(netlist, AssertAnalysis::Dc);
    if statements.is_empty() {
        return Vec::new();
    }
    solved_points(netlist, AssertAnalysis::Dc, sweep, None, &statements)
}

fn solved_points(
    netlist: &Netlist,
    analysis: AssertAnalysis,
    sweep: &[(Value, SimulationResult)],
    devices: Option<&DeviceOpReport>,
    statements: &[&AssertStatement],
) -> Vec<AssertionReport> {
    let series = match DcSweepSeries::from_sweep(sweep) {
        Ok(Some(series)) => series,
        Ok(None) => {
            return statements
                .iter()
                .map(|statement| {
                    failed(
                        statement,
                        analysis,
                        None,
                        "the analysis solved no point".into(),
                    )
                })
                .collect();
        }
        Err(error) => {
            return statements
                .iter()
                .map(|statement| failed(statement, analysis, None, error.to_string()))
                .collect();
        }
    };
    let source = Source {
        analysis,
        axis: series.axis(),
        signals: series.signal_map(),
        params: &netlist.params,
        transient: None,
        span: None,
        devices,
    };
    let mut reports = Vec::new();
    for statement in statements {
        evaluate_statement(netlist, statement, &source, &mut reports);
    }
    reports
}

//=============================================================================
// Which cards apply
//=============================================================================

/// What an assertion's operands need from a result.
#[derive(Default)]
struct Needs {
    events: bool,
    devices: bool,
}

fn needs(statement: &AssertStatement) -> Needs {
    fn scan(expression: &Expr, needs: &mut Needs) {
        match expression {
            Expr::Param(name) if name.starts_with('@') => needs.devices = true,
            Expr::FnCall { name, args } => {
                if matches!(name.as_str(), "D" | "E" | "ISUNKNOWN") {
                    needs.events = true;
                }
                for argument in args {
                    scan(argument, needs);
                }
            }
            Expr::BinOp { left, right, .. } => {
                scan(left, needs);
                scan(right, needs);
            }
            Expr::UnaryOp { operand, .. } => scan(operand, needs),
            _ => {}
        }
    }
    let mut needs = Needs::default();
    match &statement.check {
        AssertCheck::Expression { expression, .. } => {
            if let Ok(expression) = crate::netlist::expr::parse_expression(expression) {
                scan(&expression, &mut needs);
            }
        }
        AssertCheck::Condition { condition } => {
            if let Ok(expression) = crate::netlist::expr::parse_expression(condition) {
                scan(&expression, &mut needs);
            }
        }
        AssertCheck::Device { .. } => needs.devices = true,
        AssertCheck::Sampled { .. } => {}
    }
    needs
}

/// The cards evaluated after `analysis`: those that name it, and when a card
/// names none, those whose operands the analysis produces. A solved point
/// carries no event values, and a DC sweep publishes no device quantities.
fn applicable(netlist: &Netlist, analysis: AssertAnalysis) -> Vec<&AssertStatement> {
    netlist
        .assertions
        .iter()
        .filter(|statement| {
            if !statement.analyses.is_empty() {
                return statement.analyses.contains(&analysis);
            }
            let needs = needs(statement);
            match analysis {
                AssertAnalysis::Tran => true,
                AssertAnalysis::Op => !needs.events,
                AssertAnalysis::Dc => !needs.events && !needs.devices,
            }
        })
        .collect()
}

//=============================================================================
// The result an assertion reads
//=============================================================================

struct Source<'a> {
    analysis: AssertAnalysis,
    /// Time, or the swept value; one entry per solved point.
    axis: &'a [Value],
    signals: HashMap<String, &'a [Value]>,
    params: &'a ParamContext,
    transient: Option<&'a TransientResult>,
    /// The run's first and last accepted time, when `transient` holds only its
    /// first rows: the span its event operands step over.
    span: Option<(Value, Value)>,
    devices: Option<&'a DeviceOpReport>,
}

impl<'a> Source<'a> {
    fn lookup(&self, name: &str) -> Option<&'a [Value]> {
        if let Some(values) = self
            .signals
            .get(name)
            .or_else(|| self.signals.get(&name.to_ascii_uppercase()))
            .or_else(|| self.signals.get(&name.to_ascii_lowercase()))
        {
            return Some(*values);
        }
        let wanted = super::canonical_measure_signal_name(name);
        self.signals
            .iter()
            .find(|(key, _)| super::canonical_measure_signal_name(key) == wanted)
            .map(|(_, values)| *values)
    }

    /// The value of `@instance[parameter]` across the axis.
    fn device_column(&self, instance: &str, parameter: &str) -> Result<Cow<'a, [Value]>, String> {
        match self.analysis {
            AssertAnalysis::Dc => Err(format!(
                "@{instance}[{parameter}] is an operating-point quantity, and a DC sweep \
                 publishes none"
            )),
            AssertAnalysis::Op => {
                let wanted = canonical_symbol(instance);
                let entry = self
                    .devices
                    .and_then(|report| {
                        report
                            .entries
                            .iter()
                            .find(|entry| canonical_symbol(&entry.name) == wanted)
                    })
                    .ok_or_else(|| {
                        format!("{instance} is not a device this operating point reports")
                    })?;
                entry
                    .params
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
                    .map(|(_, value)| Cow::Owned(vec![*value]))
                    .ok_or_else(|| {
                        format!("device {instance} does not report the quantity {parameter}")
                    })
            }
            AssertAnalysis::Tran => {
                let result = self.transient.expect("a transient source holds its result");
                let wanted = canonical_symbol(instance);
                let mut traces = result
                    .device_op_traces
                    .iter()
                    .filter(|trace| canonical_symbol(&trace.device_name) == wanted)
                    .peekable();
                if traces.peek().is_none() {
                    return Err(format!(
                        "{instance} is not a device this run recorded operating-point \
                         quantities for"
                    ));
                }
                traces
                    .find(|trace| trace.parameter.eq_ignore_ascii_case(parameter))
                    .map(|trace| Cow::Borrowed(trace.values.as_slice()))
                    .ok_or_else(|| {
                        format!("device {instance} does not report the quantity {parameter}")
                    })
            }
        }
    }
}

//=============================================================================
// The compiled check
//=============================================================================

/// An expression over resolved operands.
#[derive(Debug, Clone)]
enum Node {
    Const(Value),
    Operand(usize),
    Unary(UnaryOpKind, Box<Node>),
    Binary(BinOpKind, Box<Node>, Box<Node>),
    IsUnknown(Box<Node>),
    Call(String, Vec<Node>),
}

/// An operand resolved against the result.
enum Operand<'p> {
    /// One value per point of the axis, linear between points: the result's own column
    /// where it holds one.
    Column(Cow<'p, [Value]>),
    /// A right-continuous step function (transient event operands).
    Steps(Steps),
}

struct Program<'p> {
    operands: Vec<Operand<'p>>,
    params: &'p ParamContext,
}

fn truth(value: Value) -> bool {
    value != 0.0
}

fn flag(value: bool) -> Value {
    if value { 1.0 } else { 0.0 }
}

impl Program<'_> {
    /// The value of `node` given each operand's value; `None` is X or Z.
    fn eval(&self, node: &Node, values: &[Option<Value>]) -> Result<Option<Value>, String> {
        Ok(match node {
            Node::Const(value) => Some(*value),
            Node::Operand(index) => values[*index],
            Node::IsUnknown(inner) => Some(flag(self.eval(inner, values)?.is_none())),
            Node::Unary(op, inner) => self.eval(inner, values)?.map(|value| match op {
                UnaryOpKind::Neg => -value,
                UnaryOpKind::Not => flag(!truth(value)),
            }),
            Node::Binary(BinOpKind::And, left, right) => {
                match (self.eval(left, values)?, self.eval(right, values)?) {
                    (Some(left), _) if !truth(left) => Some(0.0),
                    (_, Some(right)) if !truth(right) => Some(0.0),
                    (Some(_), Some(_)) => Some(1.0),
                    _ => None,
                }
            }
            Node::Binary(BinOpKind::Or, left, right) => {
                match (self.eval(left, values)?, self.eval(right, values)?) {
                    (Some(left), _) if truth(left) => Some(1.0),
                    (_, Some(right)) if truth(right) => Some(1.0),
                    (Some(_), Some(_)) => Some(0.0),
                    _ => None,
                }
            }
            Node::Binary(op, left, right) => {
                let (Some(left), Some(right)) =
                    (self.eval(left, values)?, self.eval(right, values)?)
                else {
                    return Ok(None);
                };
                Some(match op {
                    BinOpKind::Add => left + right,
                    BinOpKind::Sub => left - right,
                    BinOpKind::Mul => left * right,
                    BinOpKind::Div => left / right,
                    BinOpKind::Mod => left % right,
                    BinOpKind::Pow => left.powf(right),
                    BinOpKind::Gt => flag(left > right),
                    BinOpKind::Lt => flag(left < right),
                    BinOpKind::Ge => flag(left >= right),
                    BinOpKind::Le => flag(left <= right),
                    BinOpKind::Eq => flag(left == right),
                    BinOpKind::Ne => flag(left != right),
                    BinOpKind::And | BinOpKind::Or => unreachable!("handled above"),
                })
            }
            Node::Call(name, args) => {
                let mut numbers = Vec::with_capacity(args.len());
                for argument in args {
                    let Some(value) = self.eval(argument, values)? else {
                        return Ok(None);
                    };
                    numbers.push(Expr::Number(value));
                }
                let call = Expr::FnCall {
                    name: name.clone(),
                    args: numbers,
                };
                Some(
                    crate::netlist::expr::evaluate(&call, self.params)
                        .map_err(|error| format!("{name}() could not be evaluated: {error}"))?,
                )
            }
        })
    }
}

/// Builds a [`Program`] from parsed expressions, resolving each operand once.
struct Compiler<'s, 'a> {
    source: &'s Source<'a>,
    operands: Vec<Operand<'a>>,
    keys: HashMap<String, usize>,
}

impl<'a> Compiler<'_, 'a> {
    fn operand(
        &mut self,
        key: String,
        resolve: impl FnOnce(&Source<'a>) -> Result<Operand<'a>, String>,
    ) -> Result<Node, String> {
        if let Some(index) = self.keys.get(&key) {
            return Ok(Node::Operand(*index));
        }
        let operand = resolve(self.source)?;
        self.operands.push(operand);
        self.keys.insert(key, self.operands.len() - 1);
        Ok(Node::Operand(self.operands.len() - 1))
    }

    fn column(&mut self, text: String) -> Result<Node, String> {
        self.operand(text.clone(), |source| {
            source
                .lookup(&text)
                .map(|values| Operand::Column(Cow::Borrowed(values)))
                .ok_or_else(|| missing_signal(source, &text))
        })
    }

    fn compile(&mut self, expression: &Expr) -> Result<Node, String> {
        Ok(match expression {
            Expr::Number(value) => Node::Const(*value),
            Expr::ComplexNumber(_) | Expr::StringLiteral(_) => {
                return Err("an assertion reads real values only".into());
            }
            Expr::Param(name) if name.starts_with('@') => {
                let (instance, parameter) = name[1..]
                    .strip_suffix(']')
                    .and_then(|inner| inner.split_once('['))
                    .ok_or_else(|| format!("{name} is not an @inst[param] reference"))?;
                let (instance, parameter) = (instance.to_string(), parameter.to_string());
                self.operand(name.clone(), |source| {
                    if let Some(values) = source.lookup(name) {
                        return Ok(Operand::Column(Cow::Borrowed(values)));
                    }
                    source
                        .device_column(&instance, &parameter)
                        .map(Operand::Column)
                })?
            }
            Expr::Param(name) if name.eq_ignore_ascii_case("TIME") => self
                .operand("TIME".into(), |source| {
                    Ok(Operand::Column(Cow::Borrowed(source.axis)))
                })?,
            Expr::Param(name) => {
                if let Some(values) = self.source.lookup(name) {
                    return self
                        .operand(name.clone(), |_| Ok(Operand::Column(Cow::Borrowed(values))));
                }
                let value = crate::netlist::expr::evaluate(expression, self.source.params)
                    .map_err(|_| {
                        format!(
                            "'{name}' is neither a signal of this result nor a parameter of the \
                             deck"
                        )
                    })?;
                Node::Const(value)
            }
            Expr::UnaryOp { op, operand } => Node::Unary(*op, Box::new(self.compile(operand)?)),
            Expr::BinOp { op, left, right } => Node::Binary(
                *op,
                Box::new(self.compile(left)?),
                Box::new(self.compile(right)?),
            ),
            Expr::FnCall { name, args } => self.call(name, args)?,
        })
    }

    fn call(&mut self, name: &str, args: &[Expr]) -> Result<Node, String> {
        let names = || {
            args.iter()
                .map(|argument| match argument {
                    Expr::Param(name) => Some(name.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
        };
        match name {
            "D" | "E" => {
                let net = match names().as_deref() {
                    Some([net]) => net.clone(),
                    _ => return Err(format!("{name}() takes one event net or bus")),
                };
                let digital = name == "D";
                let text = format!("{name}({net})");
                self.operand(text.clone(), |source| match source.transient {
                    Some(result) => match source.span {
                        Some((start, end)) => event_steps_within(digital, &net, result, start, end),
                        None => event_steps(digital, &net, result),
                    }
                    .map(Operand::Steps),
                    None => Err(format!(
                        "{text} is an event operand, and {} solves points that carry no event \
                         values",
                        source.analysis
                    )),
                })
            }
            "ISUNKNOWN" => match args {
                [argument] => Ok(Node::IsUnknown(Box::new(self.compile(argument)?))),
                _ => Err("isunknown() takes one operand".into()),
            },
            "V" => match names().as_deref() {
                Some([node]) => self.column(format!("V({node})")),
                Some([positive, negative]) => Ok(Node::Binary(
                    BinOpKind::Sub,
                    Box::new(self.column(format!("V({positive})"))?),
                    Box::new(self.column(format!("V({negative})"))?),
                )),
                _ => Err("V() takes one node or two".into()),
            },
            _ if is_probe_accessor(name) => match names() {
                Some(names) if !names.is_empty() => {
                    self.column(format!("{name}({})", names.join(",")))
                }
                _ => Err(format!("{name}() takes signal names")),
            },
            _ => Ok(Node::Call(
                name.to_string(),
                args.iter()
                    .map(|argument| self.compile(argument))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
        }
    }
}

fn is_probe_accessor(name: &str) -> bool {
    matches!(
        name,
        "I" | "N" | "VM" | "VR" | "VI" | "VP" | "VDB" | "IM" | "IR" | "II" | "IP" | "IDB"
    ) || crate::netlist::is_device_lead_current_accessor(name)
}

fn missing_signal(source: &Source<'_>, text: &str) -> String {
    let net = text
        .strip_prefix("V(")
        .and_then(|inner| inner.strip_suffix(')'));
    if let (Some(net), Some(result)) = (net, source.transient) {
        if result.digital_trace_named(net).is_some() {
            return format!("{text} names '{net}', an event net: read its value as D({net})");
        }
        if result.real_trace_named(net).is_some() {
            return format!("{text} names '{net}', a real event net: read its value as E({net})");
        }
    }
    format!(
        "{text} names no signal this {} result holds",
        source.analysis
    )
}

//=============================================================================
// One card
//=============================================================================

/// The checked function and the band it must stay in, or a condition.
enum Check {
    Band {
        value: Node,
        min: Option<Value>,
        max: Option<Value>,
    },
    Condition(Node),
}

/// The functions whose sign changes are the instants the check can change.
fn atoms(check: &Check) -> Vec<(Node, Value)> {
    fn comparisons(node: &Node, atoms: &mut Vec<(Node, Value)>) {
        match node {
            Node::Binary(op, left, right) => {
                if matches!(
                    op,
                    BinOpKind::Gt
                        | BinOpKind::Lt
                        | BinOpKind::Ge
                        | BinOpKind::Le
                        | BinOpKind::Eq
                        | BinOpKind::Ne
                ) {
                    atoms.push((
                        Node::Binary(BinOpKind::Sub, left.clone(), right.clone()),
                        0.0,
                    ));
                }
                comparisons(left, atoms);
                comparisons(right, atoms);
            }
            Node::Unary(_, inner) | Node::IsUnknown(inner) => comparisons(inner, atoms),
            Node::Call(_, args) => args.iter().for_each(|arg| comparisons(arg, atoms)),
            Node::Const(_) | Node::Operand(_) => {}
        }
    }
    let mut atoms = Vec::new();
    match check {
        Check::Band { value, min, max } => {
            for bound in [min, max].into_iter().flatten() {
                atoms.push((value.clone(), *bound));
            }
            comparisons(value, &mut atoms);
        }
        Check::Condition(condition) => comparisons(condition, &mut atoms),
    }
    atoms
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Pass,
    Fail,
    Unknown,
}

/// The class of the check at one point, and the checked value's distance
/// outside its band with the value itself.
fn classify(
    program: &Program<'_>,
    check: &Check,
    values: &[Option<Value>],
) -> Result<(Class, Option<(Value, Value)>), String> {
    match check {
        Check::Band { value, min, max } => {
            let Some(value) = program.eval(value, values)? else {
                return Ok((Class::Unknown, None));
            };
            let excess = min
                .map_or(Value::NEG_INFINITY, |min| min - value)
                .max(max.map_or(Value::NEG_INFINITY, |max| value - max));
            let inside = min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max);
            Ok(if inside {
                (Class::Pass, None)
            } else {
                (Class::Fail, Some((excess, value)))
            })
        }
        Check::Condition(condition) => Ok(match program.eval(condition, values)? {
            None => (Class::Unknown, None),
            Some(value) if truth(value) => (Class::Pass, None),
            Some(_) => (Class::Fail, None),
        }),
    }
}

fn failed(
    statement: &AssertStatement,
    analysis: AssertAnalysis,
    instance: Option<String>,
    error: String,
) -> AssertionReport {
    let mut report = blank_report(statement, analysis, instance);
    report.error = Some(error);
    report
}

fn blank_report(
    statement: &AssertStatement,
    analysis: AssertAnalysis,
    instance: Option<String>,
) -> AssertionReport {
    let target = match &statement.check {
        AssertCheck::Expression { expression, .. } => expression.clone(),
        AssertCheck::Condition { condition } => condition.clone(),
        AssertCheck::Device { parameter, .. } => {
            format!("@{}[{parameter}]", instance.as_deref().unwrap_or("*"))
        }
        AssertCheck::Sampled { node, .. } => format!("V({node})"),
    };
    AssertionReport {
        name: statement.name.clone(),
        analysis,
        target,
        instance,
        level: statement.level,
        message: statement.message.clone(),
        origin: statement.origin.clone(),
        violations: Vec::new(),
        unknown_stretches: 0,
        error: None,
        after_stop: None,
    }
}

fn evaluate_statement(
    netlist: &Netlist,
    statement: &AssertStatement,
    source: &Source<'_>,
    reports: &mut Vec<AssertionReport>,
) {
    let analysis = source.analysis;
    match &statement.check {
        AssertCheck::Expression {
            expression,
            min,
            max,
        } => {
            let compiled = compile(source, expression).map(|(program, value)| {
                (
                    program,
                    Check::Band {
                        value,
                        min: *min,
                        max: *max,
                    },
                )
            });
            reports.push(run(statement, source, None, compiled));
        }
        AssertCheck::Condition { condition } => {
            let compiled = compile(source, condition)
                .map(|(program, condition)| (program, Check::Condition(condition)));
            reports.push(run(statement, source, None, compiled));
        }
        AssertCheck::Device {
            target,
            parameter,
            min,
            max,
        } => {
            let instances = match instances(netlist, target) {
                Ok(instances) => instances,
                Err(error) => {
                    reports.push(failed(statement, analysis, None, error));
                    return;
                }
            };
            for instance in instances {
                let compiled = source.device_column(&instance, parameter).map(|column| {
                    (
                        Program {
                            operands: vec![Operand::Column(column)],
                            params: source.params,
                        },
                        Check::Band {
                            value: Node::Operand(0),
                            min: *min,
                            max: *max,
                        },
                    )
                });
                reports.push(run(statement, source, Some(instance), compiled));
            }
        }
        AssertCheck::Sampled {
            node,
            column,
            samples,
        } => sampled(statement, source, node, *column, samples, reports),
    }
}

/// A vector file's expected outputs on one bit. Each sample is read off the
/// accepted trajectory at its instant, on the line between the accepted
/// points around it, and each mismatch is a report of its own, located at
/// the row that states the expectation. A bit with no mismatch has one
/// report with no violation. A sample outside the run's time span is not
/// checked; those after the stop time are counted in the file's one warning
/// ([`samples_after_stop`]).
fn sampled(
    statement: &AssertStatement,
    source: &Source<'_>,
    node: &str,
    column: usize,
    samples: &[AssertSample],
    reports: &mut Vec<AssertionReport>,
) {
    let analysis = source.analysis;
    if analysis != AssertAnalysis::Tran {
        reports.push(failed(
            statement,
            analysis,
            None,
            "a sampled check reads a transient".into(),
        ));
        return;
    }
    let signal = format!("V({node})");
    let Some(values) = source.lookup(&signal) else {
        reports.push(failed(
            statement,
            analysis,
            None,
            missing_signal(source, &signal),
        ));
        return;
    };
    let time = source.axis;
    let (Some(&first), Some(&stop)) = (time.first(), time.last()) else {
        reports.push(failed(
            statement,
            analysis,
            None,
            "the transient accepted no time point".into(),
        ));
        return;
    };
    if values.len() != time.len() {
        reports.push(failed(
            statement,
            analysis,
            None,
            format!(
                "{signal} holds {} samples on a time axis of {}",
                values.len(),
                time.len()
            ),
        ));
        return;
    }
    let mut mismatches = 0usize;
    for sample in samples {
        let at = sample.time;
        if !(at >= first && at <= stop) {
            continue;
        }
        let row = time.partition_point(|t| *t <= at) - 1;
        let value = if time[row] == at || row + 1 == time.len() {
            values[row]
        } else {
            let (t0, t1) = (time[row], time[row + 1]);
            values[row] + (values[row + 1] - values[row]) * ((at - t0) / (t1 - t0))
        };
        let holds = if sample.high {
            value >= sample.threshold
        } else {
            value <= sample.threshold
        };
        if holds {
            continue;
        }
        mismatches += 1;
        let mut report = blank_report(statement, analysis, None);
        report.origin = sample.origin.clone();
        report.message = Some(format!(
            "{}: column {column} expects {} on {node}: {signal} {} {} V",
            sample.origin,
            u8::from(sample.high),
            if sample.high { ">=" } else { "<=" },
            sample.threshold
        ));
        report.violations.push(AssertViolation {
            start: Some(at),
            end: Some(at),
            extreme: Some(value),
            at: Some(at),
        });
        reports.push(report);
    }
    if mismatches == 0 {
        reports.push(blank_report(statement, analysis, None));
    }
}

fn compile<'p>(source: &Source<'p>, text: &str) -> Result<(Program<'p>, Node), String> {
    // Parsed and compiled against `source`'s signal table: a stream compiles against its first
    // rows, which hold every name the whole run does.
    let expression = crate::netlist::expr::parse_expression(text)
        .map_err(|error| format!("'{text}' is not an expression: {error}"))?;
    let mut compiler = Compiler {
        source,
        operands: Vec::new(),
        keys: HashMap::new(),
    };
    let node = compiler.compile(&expression)?;
    Ok((
        Program {
            operands: compiler.operands,
            params: source.params,
        },
        node,
    ))
}

fn run(
    statement: &AssertStatement,
    source: &Source<'_>,
    instance: Option<String>,
    compiled: Result<(Program<'_>, Check), String>,
) -> AssertionReport {
    let analysis = source.analysis;
    let (program, check) = match compiled {
        Ok(compiled) => compiled,
        Err(error) => return failed(statement, analysis, instance, error),
    };
    let outcome = match analysis {
        AssertAnalysis::Tran => transient(statement, source, &program, &check),
        AssertAnalysis::Op | AssertAnalysis::Dc => points(statement, source, &program, &check),
    };
    match outcome {
        Ok((violations, unknown_stretches)) => {
            let mut report = blank_report(statement, analysis, instance);
            report.violations = violations;
            report.unknown_stretches = unknown_stretches;
            report
        }
        Err(error) => failed(statement, analysis, instance, error),
    }
}

//=============================================================================
// Solved points
//=============================================================================

fn points(
    statement: &AssertStatement,
    source: &Source<'_>,
    program: &Program<'_>,
    check: &Check,
) -> Result<(Vec<AssertViolation>, usize), String> {
    let operating_point = source.analysis == AssertAnalysis::Op;
    let mut violations = Vec::new();
    let mut unknown = 0;
    // (first, last, excess, extreme, at) of the failing run in progress.
    let mut run: Option<(Value, Value, Value, Option<Value>, Value)> = None;
    let mut values = Vec::with_capacity(program.operands.len());
    let close = |run: (Value, Value, Value, Option<Value>, Value),
                 violations: &mut Vec<AssertViolation>| {
        let (first, last, _, extreme, at) = run;
        if operating_point {
            violations.push(AssertViolation {
                start: None,
                end: None,
                extreme,
                at: None,
            });
        } else if statement
            .duration
            .is_none_or(|duration| reaches(last - first, duration, first, last))
        {
            violations.push(AssertViolation {
                start: Some(first),
                end: Some(last),
                extreme,
                at: extreme.map(|_| at),
            });
        }
    };
    for (row, &coordinate) in source.axis.iter().enumerate() {
        if !operating_point
            && (statement.from.is_some_and(|from| coordinate < from)
                || statement.to.is_some_and(|to| coordinate > to))
        {
            if let Some(open) = run.take() {
                close(open, &mut violations);
            }
            continue;
        }
        values.clear();
        values.extend(program.operands.iter().map(|operand| match operand {
            Operand::Column(column) => Some(column[row]),
            Operand::Steps(steps) => known(steps.level_at(coordinate)),
        }));
        let (class, excess) = classify(program, check, &values)?;
        let class = match class {
            Class::Unknown if statement.ignore_unknown => {
                unknown += 1;
                Class::Pass
            }
            Class::Unknown => Class::Fail,
            class => class,
        };
        if class == Class::Fail {
            let (excess, value) = excess.map_or((Value::NEG_INFINITY, None), |(e, v)| (e, Some(v)));
            match &mut run {
                Some((_, last, worst, extreme, at)) => {
                    *last = coordinate;
                    if excess > *worst {
                        (*worst, *extreme, *at) = (excess, value, coordinate);
                    }
                }
                None => run = Some((coordinate, coordinate, excess, value, coordinate)),
            }
        } else if let Some(open) = run.take() {
            close(open, &mut violations);
        }
    }
    if let Some(open) = run.take() {
        close(open, &mut violations);
    }
    Ok((violations, unknown))
}

fn known(level: Level) -> Option<Value> {
    match level {
        Level::Known(value) => Some(value),
        Level::Undefined(_) => None,
    }
}

/// Whether a stretch `length` long is at least `duration` long, the bound
/// inclusive: a stretch whose ends were solved to the last bit of their
/// magnitude still reaches a duration it equals.
fn reaches(length: Value, duration: Value, start: Value, end: Value) -> bool {
    let scale = start.abs().max(end.abs()).max(duration.abs());
    length >= duration - 8.0 * Value::EPSILON * scale
}

//=============================================================================
// The transient timeline
//=============================================================================

/// The maximal stretches a class holds over, merged as pieces arrive in
/// time order.
struct Timeline<'s> {
    statement: &'s AssertStatement,
    /// The analysis's stop time: a violation still open there stays open.
    stop: Value,
    run: Option<Run>,
    violations: Vec<AssertViolation>,
    unknown: usize,
}

struct Run {
    class: Class,
    start: Value,
    end: Value,
    /// The largest distance outside the band, the value there and its time.
    worst: Option<(Value, Value, Value)>,
}

impl Timeline<'_> {
    fn push(
        &mut self,
        start: Value,
        end: Value,
        class: Class,
        worst: Option<(Value, Value, Value)>,
    ) {
        let class = match class {
            Class::Unknown if !self.statement.ignore_unknown => Class::Fail,
            class => class,
        };
        if let Some(run) = &mut self.run
            && run.class == class
        {
            run.end = end;
            if let Some(candidate) = worst
                && run.worst.is_none_or(|(excess, _, _)| candidate.0 > excess)
            {
                run.worst = Some(candidate);
            }
            return;
        }
        self.close();
        self.run = Some(Run {
            class,
            start,
            end,
            worst,
        });
    }

    fn close(&mut self) {
        let Some(run) = self.run.take() else {
            return;
        };
        match run.class {
            Class::Pass => {}
            Class::Unknown => self.unknown += 1,
            Class::Fail => {
                if self.statement.duration.is_some_and(|duration| {
                    !reaches(run.end - run.start, duration, run.start, run.end)
                }) {
                    return;
                }
                let open = run.end >= self.stop;
                self.violations.push(AssertViolation {
                    start: Some(run.start),
                    end: (!open).then_some(run.end),
                    extreme: run.worst.map(|(_, value, _)| value),
                    at: run.worst.map(|(_, _, at)| at),
                });
            }
        }
    }
}

fn transient(
    statement: &AssertStatement,
    source: &Source<'_>,
    program: &Program<'_>,
    check: &Check,
) -> Result<(Vec<AssertViolation>, usize), String> {
    let time = source.axis;
    let (Some(&first), Some(&stop)) = (time.first(), time.last()) else {
        return Err("the transient accepted no time point".into());
    };
    for operand in &program.operands {
        if let Operand::Column(column) = operand
            && column.len() != time.len()
        {
            return Err(format!(
                "an operand holds {} samples on a time axis of {}",
                column.len(),
                time.len()
            ));
        }
    }
    let lower = statement.from.map_or(first, |from| from.max(first));
    let upper = statement.to.map_or(stop, |to| to.min(stop));
    if lower > upper {
        return Ok((Vec::new(), 0));
    }
    let mut walk = Walk::new(statement, program, check, upper, stop);
    // Each operand's value at one row; an event operand's entry is not read.
    let mut row = vec![Value::NAN; program.operands.len()];
    let fill = |index: usize, row: &mut [Value]| {
        for (value, operand) in row.iter_mut().zip(&program.operands) {
            if let Operand::Column(column) = operand {
                *value = column[index];
            }
        }
    };
    if time.len() == 1 || lower == upper {
        // A single instant: the check at that point.
        let index = time.partition_point(|t| *t <= lower).saturating_sub(1);
        fill(index, &mut row);
        walk.instant = Some((time[index], row));
        walk.check_instant(program, check, lower, upper)?;
        return Ok(walk.finish());
    }
    for (index, &t) in time.iter().enumerate() {
        fill(index, &mut row);
        walk.row(program, check, t, &row)?;
    }
    Ok(walk.finish())
}

/// One check walked along the transient timeline, fed one accepted row at a
/// time: the evaluation in memory feeds it the run's rows, and
/// [`TranAssertStream`] the rows a store gives back a chunk at a time.
struct Walk<'s> {
    from: Option<Value>,
    to: Option<Value>,
    /// The end of the checked window: `TO` clipped to the stop time.
    upper: Value,
    /// The start of the checked window, which the first row sets.
    lower: Option<Value>,
    /// Every instant an event operand steps at inside the window, which the
    /// first row sets.
    events: Vec<Value>,
    next_event: usize,
    atoms: Vec<(Node, Value)>,
    timeline: Timeline<'s>,
    values: Vec<Option<Value>>,
    /// The previous row: its time and each operand's value there.
    previous: Option<(Value, Vec<Value>)>,
    /// The last row at or before the window's start: the point a window of a
    /// single instant is checked at.
    instant: Option<(Value, Vec<Value>)>,
}

impl<'s> Walk<'s> {
    fn new(
        statement: &'s AssertStatement,
        program: &Program<'_>,
        check: &Check,
        upper: Value,
        stop: Value,
    ) -> Self {
        Self {
            from: statement.from,
            to: statement.to,
            upper,
            lower: None,
            events: Vec::new(),
            next_event: 0,
            atoms: atoms(check),
            timeline: Timeline {
                statement,
                stop,
                run: None,
                violations: Vec::new(),
                unknown: 0,
            },
            values: vec![None; program.operands.len()],
            previous: None,
            instant: None,
        }
    }

    /// The next accepted row: its time and each operand's value there.
    fn row(
        &mut self,
        program: &Program<'_>,
        check: &Check,
        t1: Value,
        row: &[Value],
    ) -> Result<(), String> {
        let lower = match self.lower {
            Some(lower) => lower,
            None => {
                let lower = self.from.map_or(t1, |from| from.max(t1));
                let upper = self.upper;
                let mut events = program
                    .operands
                    .iter()
                    .filter_map(|operand| match operand {
                        Operand::Steps(steps) => Some(steps.instants()),
                        Operand::Column(_) => None,
                    })
                    .flatten()
                    .copied()
                    .filter(|instant| *instant > lower && *instant < upper)
                    .collect::<Vec<_>>();
                events.sort_by(Value::total_cmp);
                events.dedup();
                self.events = events;
                self.lower = Some(lower);
                lower
            }
        };
        if t1 <= lower {
            match &mut self.instant {
                Some((at, values)) => {
                    *at = t1;
                    values.clear();
                    values.extend_from_slice(row);
                }
                None => self.instant = Some((t1, row.to_vec())),
            }
        }
        let Some((t0, mut v0)) = self.previous.take() else {
            self.previous = Some((t1, row.to_vec()));
            return Ok(());
        };
        let upper = self.upper;
        if !(t1 <= t0 || t1 <= lower || t0 >= upper) {
            let mut start = t0.max(lower);
            let end = t1.min(upper);
            while start < end {
                while self.next_event < self.events.len() && self.events[self.next_event] <= start {
                    self.next_event += 1;
                }
                let piece_end = self
                    .events
                    .get(self.next_event)
                    .copied()
                    .filter(|instant| *instant < end)
                    .unwrap_or(end);
                let interpolant = Interpolant::new(program, (t0, v0.as_slice()), (t1, row), start);
                segment(
                    check,
                    &self.atoms,
                    &interpolant,
                    (start, piece_end),
                    &mut self.values,
                    &mut self.timeline,
                )?;
                start = piece_end;
            }
        }
        v0.clear();
        v0.extend_from_slice(row);
        self.previous = Some((t1, v0));
        Ok(())
    }

    /// The check of a window of a single instant, at `lower`, with the values
    /// of the last row at or before it.
    fn check_instant(
        &mut self,
        program: &Program<'_>,
        check: &Check,
        lower: Value,
        upper: Value,
    ) -> Result<(), String> {
        let Some((at, row)) = &self.instant else {
            return Ok(());
        };
        let interpolant =
            Interpolant::new(program, (*at, row.as_slice()), (*at, row.as_slice()), lower);
        interpolant.at(lower, &mut self.values);
        let (class, excess) = classify(program, check, &self.values)?;
        self.timeline.push(
            lower,
            upper,
            class,
            excess.map(|(excess, value)| (excess, value, lower)),
        );
        Ok(())
    }

    /// The violations and unknown stretches of a walk whose every row has
    /// been fed: `first` and `stop` the first and last time, `count` the rows.
    fn conclude(
        mut self,
        program: &Program<'_>,
        check: &Check,
        first: Option<Value>,
        stop: Value,
        count: usize,
    ) -> Result<(Vec<AssertViolation>, usize), String> {
        let Some(first) = first else {
            return Err("the transient accepted no time point".into());
        };
        let lower = self.from.map_or(first, |from| from.max(first));
        let upper = self.to.map_or(stop, |to| to.min(stop));
        if lower > upper {
            return Ok((Vec::new(), 0));
        }
        if count == 1 || lower == upper {
            // A window of one instant holds no piece: the check is made at
            // that instant alone.
            self.check_instant(program, check, lower, upper)?;
        }
        Ok(self.finish())
    }

    fn finish(mut self) -> (Vec<AssertViolation>, usize) {
        self.timeline.close();
        (self.timeline.violations, self.timeline.unknown)
    }
}

/// Operand values on one segment: analog columns on the line between two
/// rows, event operands at their level from the segment's start.
struct Interpolant<'p, 'q, 'r> {
    program: &'p Program<'q>,
    t0: Value,
    t1: Value,
    /// Each operand's value at the two rows.
    v0: &'r [Value],
    v1: &'r [Value],
    levels: Vec<Option<Value>>,
}

impl<'p, 'q, 'r> Interpolant<'p, 'q, 'r> {
    fn new(
        program: &'p Program<'q>,
        (t0, v0): (Value, &'r [Value]),
        (t1, v1): (Value, &'r [Value]),
        start: Value,
    ) -> Self {
        let levels = program
            .operands
            .iter()
            .map(|operand| match operand {
                Operand::Steps(steps) => known(steps.level_at(start)),
                Operand::Column(_) => None,
            })
            .collect();
        Self {
            program,
            t0,
            t1,
            v0,
            v1,
            levels,
        }
    }

    fn at(&self, time: Value, values: &mut [Option<Value>]) {
        let span = self.t1 - self.t0;
        let fraction = if span > 0.0 {
            (time - self.t0) / span
        } else {
            0.0
        };
        for (index, operand) in self.program.operands.iter().enumerate() {
            values[index] = match operand {
                Operand::Column(_) => {
                    let (v0, v1) = (self.v0[index], self.v1[index]);
                    Some(if fraction == 1.0 {
                        v1
                    } else {
                        v0 + (v1 - v0) * fraction
                    })
                }
                Operand::Steps(_) => self.levels[index],
            };
        }
    }
}

//=============================================================================
// The transient checked from its store
//=============================================================================

/// `.ASSERT` checks of a transient evaluated over its rows without the run in
/// memory, the counterpart of [`super::TranMeasureStream`]: [`Self::rows`]
/// over every accepted row of the columns [`Self::columns`] names, in row
/// order from the first, then [`Self::finish`].
pub struct TranAssertStream<'n> {
    checks: Vec<Streamed<'n>>,
    /// The caller's columns the rows carry, in the order [`Self::rows`] takes
    /// them.
    columns: Vec<usize>,
    /// The run's last accepted time, as the plan was given it.
    stop: Value,
    first: Option<Value>,
    last: Value,
    count: usize,
    nondecreasing: bool,
    row: Vec<Value>,
}

enum Streamed<'n> {
    /// A report the plan decided: a device target the deck's hierarchy does
    /// not expand.
    Decided(Box<AssertionReport>),
    Walking(Box<Walking<'n>>),
}

struct Walking<'n> {
    statement: &'n AssertStatement,
    instance: Option<String>,
    program: Program<'n>,
    check: Check,
    /// Where each operand's value at a row comes from; `None` for an event
    /// operand, which the walk reads from its history.
    carried: Vec<Option<Carried>>,
    walk: Walk<'n>,
    error: Option<String>,
}

#[derive(Clone, Copy)]
enum Carried {
    Axis,
    /// A column, by its place in [`TranAssertStream::columns`].
    Column(usize),
}

impl<'n> TranAssertStream<'n> {
    /// Plan the deck's transient `.ASSERT` cards against `probe`, a result
    /// holding the run's first rows and its whole event histories, whose every
    /// column the rows could be read from is among `columns` (`None` for one
    /// that is not); `stop` is the run's last accepted time.
    ///
    /// `None` when a card cannot be checked from the rows, and the run is
    /// checked in memory: a `.VEC` file, whose expected outputs are read at
    /// their own instants; an operand that is not one of `columns` (an instance
    /// parameter or a stored expression); a card that does not compile against
    /// the probe, whose report the evaluation in memory words.
    pub fn plan(
        netlist: &'n Netlist,
        probe: &TransientResult,
        stop: Value,
        columns: &[Option<&[Value]>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        if !netlist.vector_files.is_empty() {
            return Ok(None);
        }
        let Some(&first) = probe.time.first() else {
            return Ok(None);
        };
        let source = Source {
            analysis: AssertAnalysis::Tran,
            axis: &probe.time,
            signals: retained_transient_signal_map(probe),
            params: &netlist.params,
            transient: Some(probe),
            span: Some((first, stop)),
            devices: None,
        };
        let time = (probe.time.as_ptr() as usize, probe.time.len());
        let mut needed: Vec<usize> = Vec::new();
        let mut checks = Vec::new();
        for statement in applicable(netlist, AssertAnalysis::Tran) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let compiled = match &statement.check {
                AssertCheck::Expression {
                    expression,
                    min,
                    max,
                } => vec![(
                    None,
                    compile(&source, expression).map(|(program, value)| {
                        let check = Check::Band {
                            value,
                            min: *min,
                            max: *max,
                        };
                        (program, check)
                    }),
                )],
                AssertCheck::Condition { condition } => vec![(
                    None,
                    compile(&source, condition)
                        .map(|(program, condition)| (program, Check::Condition(condition))),
                )],
                AssertCheck::Device {
                    target,
                    parameter,
                    min,
                    max,
                } => match instances(netlist, target) {
                    Err(error) => {
                        checks.push(Streamed::Decided(Box::new(failed(
                            statement,
                            AssertAnalysis::Tran,
                            None,
                            error,
                        ))));
                        continue;
                    }
                    Ok(instances) => instances
                        .into_iter()
                        .map(|instance| {
                            let compiled =
                                source.device_column(&instance, parameter).map(|column| {
                                    let program = Program {
                                        operands: vec![Operand::Column(column)],
                                        params: source.params,
                                    };
                                    let check = Check::Band {
                                        value: Node::Operand(0),
                                        min: *min,
                                        max: *max,
                                    };
                                    (program, check)
                                });
                            (Some(instance), compiled)
                        })
                        .collect(),
                },
                AssertCheck::Sampled { .. } => return Ok(None),
            };
            for (instance, compiled) in compiled {
                let Ok((program, check)) = compiled else {
                    return Ok(None);
                };
                let mut carried = Vec::with_capacity(program.operands.len());
                let mut operands = Vec::with_capacity(program.operands.len());
                for operand in program.operands {
                    match operand {
                        Operand::Steps(steps) => {
                            carried.push(None);
                            operands.push(Operand::Steps(steps));
                        }
                        Operand::Column(Cow::Borrowed(values)) => {
                            let address = (values.as_ptr() as usize, values.len());
                            let place = if address == time {
                                Carried::Axis
                            } else {
                                let Some(column) = columns.iter().position(|column| {
                                    column.is_some_and(|column| {
                                        !column.is_empty()
                                            && (column.as_ptr() as usize, column.len()) == address
                                    })
                                }) else {
                                    return Ok(None);
                                };
                                Carried::Column(
                                    match needed.iter().position(|&known| known == column) {
                                        Some(place) => place,
                                        None => {
                                            needed.push(column);
                                            needed.len() - 1
                                        }
                                    },
                                )
                            };
                            carried.push(Some(place));
                            operands.push(Operand::Column(Cow::Owned(Vec::new())));
                        }
                        Operand::Column(Cow::Owned(_)) => return Ok(None),
                    }
                }
                let program = Program {
                    operands,
                    params: &netlist.params,
                };
                let walk = Walk::new(
                    statement,
                    &program,
                    &check,
                    statement.to.map_or(stop, |to| to.min(stop)),
                    stop,
                );
                checks.push(Streamed::Walking(Box::new(Walking {
                    statement,
                    instance,
                    program,
                    check,
                    carried,
                    walk,
                    error: None,
                })));
            }
        }
        Ok(Some(Self {
            checks,
            columns: needed,
            stop,
            first: None,
            last: Value::NAN,
            count: 0,
            nondecreasing: true,
            row: Vec::new(),
        }))
    }

    /// The caller's columns, by their place in the `columns` [`Self::plan`]
    /// was given, that [`Self::rows`] takes, in the order it takes them.
    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    /// The next accepted rows: their times `x` and each of [`Self::columns`]
    /// over the same rows.
    pub fn rows(&mut self, x: &[Value], columns: &[&[Value]]) {
        let Self {
            checks,
            first,
            last,
            count,
            nondecreasing,
            row,
            ..
        } = self;
        for (index, &t) in x.iter().enumerate() {
            if first.is_none() {
                *first = Some(t);
            } else if t < *last {
                *nondecreasing = false;
            }
            *last = t;
            *count += 1;
            for check in checks.iter_mut() {
                let Streamed::Walking(walking) = check else {
                    continue;
                };
                if walking.error.is_some() {
                    continue;
                }
                row.clear();
                row.extend(walking.carried.iter().map(|carried| match carried {
                    Some(Carried::Axis) => t,
                    Some(Carried::Column(place)) => columns[*place][index],
                    None => Value::NAN,
                }));
                if let Err(error) = walking.walk.row(&walking.program, &walking.check, t, row) {
                    walking.error = Some(error);
                }
            }
        }
    }

    /// Every card's report, as [`evaluate_tran_assertions`] gives them for the
    /// whole run, once every row has been read. `None` when the time axis ran
    /// backwards, which the evaluation in memory reads differently, or the rows
    /// did not end at the stop the plan was given, so that the run is checked
    /// in memory.
    pub fn finish(self) -> Option<Vec<AssertionReport>> {
        if !self.nondecreasing || self.last.to_bits() != self.stop.to_bits() {
            return None;
        }
        let (first, stop, count) = (self.first, self.last, self.count);
        let reports = self
            .checks
            .into_iter()
            .map(|check| match check {
                Streamed::Decided(report) => *report,
                Streamed::Walking(walking) => {
                    let Walking {
                        statement,
                        instance,
                        program,
                        check,
                        walk,
                        error,
                        ..
                    } = *walking;
                    let outcome = match error {
                        Some(error) => Err(error),
                        None => walk.conclude(&program, &check, first, stop, count),
                    };
                    match outcome {
                        Ok((violations, unknown_stretches)) => {
                            let mut report =
                                blank_report(statement, AssertAnalysis::Tran, instance);
                            report.violations = violations;
                            report.unknown_stretches = unknown_stretches;
                            report
                        }
                        Err(error) => failed(statement, AssertAnalysis::Tran, instance, error),
                    }
                }
            })
            .collect();
        Some(reports)
    }
}

/// Classify one segment `[start, end)` of the timeline, on which event
/// operands are constant and analog operands linear, cut at every root of
/// every atom, into `timeline`.
fn segment(
    check: &Check,
    atoms: &[(Node, Value)],
    interpolant: &Interpolant<'_, '_, '_>,
    (start, end): (Value, Value),
    values: &mut [Option<Value>],
    timeline: &mut Timeline<'_>,
) -> Result<(), String> {
    let program = interpolant.program;
    let mut atom_at = |node: &Node, offset: Value, time: Value| -> Result<Option<Value>, String> {
        interpolant.at(time, values);
        Ok(program.eval(node, values)?.map(|value| value - offset))
    };
    let mut cuts = vec![start, end];
    for (node, offset) in atoms {
        let (Some(at_start), Some(at_end)) =
            (atom_at(node, *offset, start)?, atom_at(node, *offset, end)?)
        else {
            continue;
        };
        if at_start == 0.0 || at_end == 0.0 || (at_start > 0.0) == (at_end > 0.0) {
            continue;
        }
        if !at_start.is_finite() || !at_end.is_finite() {
            continue;
        }
        let root = root(start, end, at_start, at_end, |time| {
            atom_at(node, *offset, time).map(|value| value.unwrap_or(0.0))
        })?;
        if root > start && root < end {
            cuts.push(root);
        }
    }
    cuts.sort_by(Value::total_cmp);
    cuts.dedup();
    for pair in cuts.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        if to <= from {
            continue;
        }
        interpolant.at(0.5 * (from + to), values);
        let (class, _) = classify(program, check, values)?;
        let worst = if class == Class::Fail {
            let mut worst: Option<(Value, Value, Value)> = None;
            for time in [from, to] {
                interpolant.at(time, values);
                let (_, excess) = classify(program, check, values)?;
                if let Some((excess, value)) = excess
                    && worst.is_none_or(|(best, _, _)| excess > best)
                {
                    worst = Some((excess, value, time));
                }
            }
            worst
        } else {
            None
        };
        timeline.push(from, to, class, worst);
    }
    Ok(())
}

/// The instant in `(lower, upper)` where a function with values of opposite
/// sign at its ends is zero: the root of the line through the ends, which is
/// exact for a check affine in its operands, refined by bracketing for one
/// that is not.
fn root(
    lower: Value,
    upper: Value,
    at_lower: Value,
    at_upper: Value,
    mut function: impl FnMut(Value) -> Result<Value, String>,
) -> Result<Value, String> {
    let (mut lower, mut upper, mut at_lower, mut at_upper) = (lower, upper, at_lower, at_upper);
    let scale = at_lower.abs().max(at_upper.abs());
    let mut stale = 0i8;
    for _ in 0..200 {
        let guess = lower + (upper - lower) * (at_lower / (at_lower - at_upper));
        let guess = if guess > lower && guess < upper {
            guess
        } else {
            0.5 * (lower + upper)
        };
        let value = function(guess)?;
        if value == 0.0
            || value.abs() <= 1e-13 * scale
            || upper - lower <= 4.0 * Value::EPSILON * upper.abs()
        {
            return Ok(guess);
        }
        // Illinois: halve the stale end's value so a curved function cannot
        // pin one end in place.
        if (value > 0.0) == (at_lower > 0.0) {
            lower = guess;
            at_lower = value;
            if stale == 1 {
                at_upper *= 0.5;
            }
            stale = 1;
        } else {
            upper = guess;
            at_upper = value;
            if stale == -1 {
                at_lower *= 0.5;
            }
            stale = -1;
        }
    }
    Ok(0.5 * (lower + upper))
}

//=============================================================================
// Device targets
//=============================================================================

/// The instances a device-form target names, in elaboration order.
fn instances(netlist: &Netlist, target: &AssertTarget) -> Result<Vec<String>, String> {
    let mut flattener = Flattener::new(&netlist.subcircuits);
    let elements = flattener
        .flatten(netlist)
        .map_err(|error| format!("the deck's hierarchy could not be expanded: {error}"))?;
    let selected: Vec<String> = match target {
        AssertTarget::Instance(name) => {
            let wanted = canonical_symbol(name);
            elements
                .iter()
                .filter(|element| canonical_symbol(&element.name) == wanted)
                .map(|element| element.name.clone())
                .collect()
        }
        AssertTarget::Model(model) => elements
            .iter()
            .filter(|element| {
                model_name(&element.kind).is_some_and(|name| name.eq_ignore_ascii_case(model))
            })
            .map(|element| element.name.clone())
            .collect(),
        AssertTarget::Primitive(primitive) => elements
            .iter()
            .filter(|element| {
                primitive_name(&element.kind)
                    .is_some_and(|name| name.eq_ignore_ascii_case(primitive))
            })
            .map(|element| element.name.clone())
            .collect(),
        AssertTarget::Subcircuit(subcircuit) => {
            let prefixes = flattener
                .instance_metadata()
                .iter()
                .filter(|instance| instance.subcircuit_name.eq_ignore_ascii_case(subcircuit))
                .map(|instance| {
                    format!("{}{}", instance.path, instance.path.separator()).to_ascii_lowercase()
                })
                .collect::<Vec<_>>();
            elements
                .iter()
                .filter(|element| primitive_name(&element.kind).is_some())
                .filter(|element| {
                    let name = element.name.to_ascii_lowercase();
                    prefixes.iter().any(|prefix| name.starts_with(prefix))
                })
                .map(|element: &Element| element.name.clone())
                .collect()
        }
    };
    if selected.is_empty() {
        let (key, name) = match target {
            AssertTarget::Instance(name) => ("DEV", name),
            AssertTarget::Model(name) => ("MOD", name),
            AssertTarget::Subcircuit(name) => ("SUB", name),
            AssertTarget::Primitive(name) => ("PRIMITIVE", name),
        };
        return Err(format!("{key}={name} names no device of this circuit"));
    }
    Ok(selected)
}

/// The model card an element names.
fn model_name(kind: &ElementKind) -> Option<&str> {
    match kind {
        ElementKind::Resistor { model, .. }
        | ElementKind::Capacitor { model, .. }
        | ElementKind::Inductor { model, .. } => model.as_deref(),
        ElementKind::Diode { model, .. }
        | ElementKind::Bjt { model, .. }
        | ElementKind::Mosfet { model, .. }
        | ElementKind::Jfet { model, .. }
        | ElementKind::Mesfet { model, .. } => Some(model),
        _ => None,
    }
}

/// The device family `PRIMITIVE=` names an element by.
fn primitive_name(kind: &ElementKind) -> Option<&'static str> {
    Some(match kind {
        ElementKind::Resistor { .. } => "resistor",
        ElementKind::Capacitor { .. } => "capacitor",
        ElementKind::Inductor { .. } => "inductor",
        ElementKind::Diode { .. } => "diode",
        ElementKind::Bjt { .. } => "bjt",
        ElementKind::Mosfet { .. } => "mosfet",
        ElementKind::Jfet { .. } => "jfet",
        ElementKind::Mesfet { .. } => "mesfet",
        _ => return None,
    })
}

#[cfg(test)]
mod tests;
