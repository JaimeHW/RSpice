//! Resolve parameter values once, using the process expression rules.

use super::*;
use crate::ast::{ParamType, ParameterDecl};
use crate::canonical_ir::digital_eval::{DigitalScalar, evaluate_constant_expression};
use crate::canonical_ir::{CfgBinaryOp, CfgUnaryOp};
use std::collections::HashSet;

/// Canonical values stay in the lowering layer; semantic declarations keep
/// source expressions and never depend on the executable value representation.
#[derive(Default)]
pub(super) struct ResolvedConstants {
    pub bits: HashMap<SmolStr, (FourStateValue, bool)>,
    pub bounds: HashMap<SmolStr, VectorBounds>,
    integers: HashMap<SmolStr, i64>,
    reals: HashMap<SmolStr, f64>,
    non_finite_reals: HashMap<SmolStr, f64>,
}

impl ResolvedConstants {
    pub fn integer(&self, name: &str) -> Option<i64> {
        self.integers.get(name).copied()
    }
    pub fn real(&self, name: &str) -> Option<f64> {
        self.reals.get(name).copied()
    }
    pub fn non_finite_real(&self, name: &str) -> Option<f64> {
        self.non_finite_reals.get(name).copied()
    }
}

/// Evaluate a closed index expression with the same sizing, signedness and
/// four-state operations as an executable digital expression. Runtime names
/// must be excluded before constructing this isolated constant-only lowerer.
pub(super) fn scalar(
    expression: &Expression,
    resolved: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Option<crate::numeric_literal::NumericLiteralValue> {
    scalar_impl(expression, resolved, time_scale, None)
}

pub(super) fn scalar_prepared(
    expression: &Expression,
    resolved: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    shapes: &expressions::Shapes,
) -> Option<crate::numeric_literal::NumericLiteralValue> {
    scalar_impl(expression, resolved, time_scale, Some(shapes))
}

fn scalar_impl(
    expression: &Expression,
    resolved: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    shapes: Option<&expressions::Shapes>,
) -> Option<crate::numeric_literal::NumericLiteralValue> {
    let mut reads = BTreeSet::new();
    collect_expression_reads(expression, &mut reads);
    if reads.iter().any(|name| {
        name != "inf"
            && !resolved.bits.contains_key(name.as_str())
            && resolved.integer(name).is_none()
            && resolved.real(name).is_none()
            && resolved.non_finite_real(name).is_none()
    }) {
        return None;
    }
    let empty_index = HashMap::new();
    let empty_analog = HashMap::new();
    let empty_arrays = HashMap::new();
    let mut probes = Vec::new();
    let mut signals = Vec::new();
    let mut lowerer = ProcessLowerer {
        process: None,
        local_arrays: Vec::new(),
        constant_expression: true,
        time_scale,
        signals: &mut signals,
        arrays: &empty_arrays,
        index: &empty_index,
        constants: resolved,
        analog_variables: &empty_analog,
        probes: &mut probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };
    let entry = lowerer.builder.create_block();
    lowerer.builder.seal_block(entry);
    let shape = shapes.map_or_else(
        || expressions::shape(&lowerer, expression),
        |shapes| shapes.get(expression),
    );
    let signed = shape.signed;
    let mode = if shape.real {
        expressions::Mode::Real
    } else {
        expressions::Mode::Bits(Context::SELF_DETERMINED)
    };
    let value = if let Some(shapes) = shapes {
        expressions::lower_prepared(&mut lowerer, entry, expression, mode, shapes)
    } else {
        expressions::lower(&mut lowerer, entry, expression, mode)
    };
    lowerer.builder.set_terminator(entry, CfgTerminator::Return);
    if !lowerer.diagnostics.is_empty() {
        return None;
    }
    let (function, outputs) = lowerer.builder.finish_with_outputs(entry, &[value]).ok()?;
    use crate::numeric_literal::NumericLiteralValue::{Integer, Real};
    match evaluate_constant_expression(function, outputs[0], expression.span().into()).ok()? {
        DigitalScalar::FourState(value) => value.bit_index(signed).map(Integer),
        DigitalScalar::Integer(value) => Some(Integer(i64::from(value))),
        DigitalScalar::Real(value) => Some(Real(value)),
        DigitalScalar::Effect => None,
    }
}

pub(super) enum MathCall {
    Unary(CfgUnaryOp),
    Binary(CfgBinaryOp),
}

pub(super) fn math_call(name: &str) -> Option<MathCall> {
    Some(match name {
        "abs" => MathCall::Unary(CfgUnaryOp::Abs),
        "sqrt" => MathCall::Unary(CfgUnaryOp::Sqrt),
        "exp" => MathCall::Unary(CfgUnaryOp::Exp),
        "ln" | "log" => MathCall::Unary(CfgUnaryOp::Ln),
        "log10" => MathCall::Unary(CfgUnaryOp::Log10),
        "floor" => MathCall::Unary(CfgUnaryOp::Floor),
        "ceil" => MathCall::Unary(CfgUnaryOp::Ceil),
        "min" => MathCall::Binary(CfgBinaryOp::Min),
        "max" => MathCall::Binary(CfgBinaryOp::Max),
        "pow" => MathCall::Binary(CfgBinaryOp::Pow),
        _ => return None,
    })
}

pub(super) fn resolve<'a>(
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    processes: &[AnalyzedDigitalProcess],
    assignments: &[crate::semantic::AnalyzedContinuousAssign],
    initializers: impl IntoIterator<Item = &'a Expression>,
) -> Result<ResolvedConstants, Vec<DigitalLoweringDiagnostic>> {
    let mut required = BTreeSet::new();
    for initializer in initializers {
        collect_expression_reads(initializer, &mut required);
    }
    for process in processes {
        collect_parameters(&process.body, &BTreeSet::new(), &mut required);
    }
    for assignment in assignments {
        collect_expression_reads(&assignment.assignment.value, &mut required);
        collect_lvalue_index_reads(&assignment.assignment.target, &mut required);
        if let Some(delay) = &assignment.assignment.delay {
            collect_expression_reads(delay, &mut required);
        }
    }
    let definitions: HashMap<_, _> = source
        .definitions
        .iter()
        .map(|value| (value.name.as_str(), value))
        .collect();
    let mut resolved = ResolvedConstants {
        bits: HashMap::new(),
        bounds: HashMap::new(),
        integers: source.integers.clone(),
        reals: source.reals.clone(),
        non_finite_reals: source.non_finite_reals.clone(),
    };
    let mut finished = HashSet::new();
    let mut active = HashSet::new();
    let mut pending: Vec<_> = required
        .into_iter()
        .rev()
        .map(|name| (name, false))
        .collect();
    while let Some((name, ready)) = pending.pop() {
        let Some(declaration) = definitions.get(name.as_str()).copied() else {
            continue;
        };
        if finished.contains(&name) {
            continue;
        }
        if ready {
            resolve_one(&name, declaration, &mut resolved, time_scale)?;
            active.remove(&name);
            finished.insert(name);
            continue;
        }
        if !active.insert(name.clone()) {
            return Err(vec![DigitalLoweringDiagnostic::refusal(
                format!("parameter `{name}` has a cyclic digital constant dependency"),
                declaration.span.into(),
            )]);
        }
        pending.push((name, true));
        let mut dependencies = BTreeSet::new();
        if let Some(expression) = &declaration.default {
            collect_expression_reads(expression, &mut dependencies);
        }
        if let Some(range) = &declaration.packed_range {
            collect_expression_reads(&range.msb, &mut dependencies);
            collect_expression_reads(&range.lsb, &mut dependencies);
        }
        pending.extend(dependencies.into_iter().rev().map(|name| (name, false)));
    }
    Ok(resolved)
}

/// Typed assignment evidence; numeric defaults still retain executable expressions.
pub(crate) struct ParameterAssignment {
    pub numeric_type: ParamType,
    pub value: Option<Expression>,
    pub bounds: Option<VectorBounds>,
}

pub(super) fn parameter_assignments(
    declarations: &[&ParameterDecl],
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Vec<ParameterAssignment>, Vec<DigitalLoweringDiagnostic>> {
    let mut resolved = ResolvedConstants::default();
    let mut unavailable = HashSet::new();
    let source = DigitalConstants {
        definitions: declarations.iter().map(|value| (*value).clone()).collect(),
        ..Default::default()
    };
    let mut result = Vec::with_capacity(declarations.len());
    for declaration in declarations {
        let mut reads = BTreeSet::new();
        if let Some(expression) = &declaration.default {
            collect_expression_reads(expression, &mut reads);
        }
        if let Some(range) = &declaration.packed_range {
            collect_expression_reads(&range.msb, &mut reads);
            collect_expression_reads(&range.lsb, &mut reads);
        }
        let packed = declaration.packed_range.is_some() || declaration.signedness.is_some();
        let can_resolve = declaration.dimensions.is_empty()
            && declaration.param_type != ParamType::String
            && !reads.iter().any(|name| unavailable.contains(name.as_str()));
        let mut resolved_value = can_resolve
            && resolve_one(&declaration.name, declaration, &mut resolved, time_scale).is_ok();
        if packed && !resolved_value {
            // A packed bound can refer to a later parameter. The shared graph
            // resolver handles that dependency and diagnoses cycles; default
            // initializer ordering is checked separately by semantic analysis.
            let root = Expression::Identifier(crate::ast::Identifier {
                name: declaration.name.clone(),
                span: declaration.span,
            });
            let complete = resolve(&source, time_scale, &[], &[], [&root])?;
            resolved.bits.extend(complete.bits);
            resolved.bounds.extend(complete.bounds);
            resolved.integers.extend(complete.integers);
            resolved.reals.extend(complete.reals);
            resolved.non_finite_reals.extend(complete.non_finite_reals);
            resolved_value = true;
        }
        if !resolved_value {
            unavailable.insert(declaration.name.clone());
        }
        let inferred = if resolved_value
            && resolved
                .bits
                .get(&declaration.name)
                .is_some_and(|(value, signed)| value.width() == 32 && *signed)
        {
            ParamType::Integer
        } else {
            ParamType::Real
        };
        result.push(ParameterAssignment {
            numeric_type: if declaration.type_is_explicit {
                declaration.param_type
            } else {
                inferred
            },
            value: resolved_value
                .then(|| resolved_literal(&declaration.name, &resolved, declaration.span).ok())
                .flatten(),
            bounds: resolved.bounds.get(&declaration.name).copied(),
        });
    }
    Ok(result)
}

fn resolve_one(
    name: &str,
    declaration: &ParameterDecl,
    resolved: &mut ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<(), Vec<DigitalLoweringDiagnostic>> {
    let refuse = |detail| {
        vec![DigitalLoweringDiagnostic::refusal(
            detail,
            declaration.span.into(),
        )]
    };
    let Some(expression) = declaration.default.as_ref() else {
        return Err(refuse(format!(
            "parameter `{name}` has no digital constant default"
        )));
    };
    let empty_index = HashMap::new();
    let empty_analog = HashMap::new();
    let empty_arrays = HashMap::new();
    let mut probes = Vec::new();
    let mut signals = Vec::new();
    let mut lowerer = ProcessLowerer {
        process: None,
        local_arrays: Vec::new(),
        constant_expression: true,
        time_scale,
        signals: &mut signals,
        arrays: &empty_arrays,
        index: &empty_index,
        constants: resolved,
        analog_variables: &empty_analog,
        probes: &mut probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };
    let entry = lowerer.builder.create_block();
    lowerer.builder.seal_block(entry);
    let bounds = declaration
        .packed_range
        .as_ref()
        .map(|range| {
            let integer_bound =
                |expression: &Expression| match scalar(expression, resolved, time_scale) {
                    Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Ok(value),
                    _ => Err(refuse(format!(
                        "parameter `{name}` packed bounds require known integer constants"
                    ))),
                };
            let bounds = VectorBounds {
                msb: integer_bound(&range.msb)?,
                lsb: integer_bound(&range.lsb)?,
            };
            if bounds.width() > crate::semantic::MAX_DIGITAL_VECTOR_WIDTH {
                return Err(refuse(format!(
                    "parameter `{name}` packed width exceeds the supported packed width"
                )));
            }
            Ok(bounds)
        })
        .transpose()?;
    let integer = declaration.type_is_explicit && declaration.param_type == ParamType::Integer;
    let signed = integer
        || declaration.signedness.map_or_else(
            || bounds.is_none() && lowerer.self_signed(expression),
            |signing| signing == crate::ast::Signedness::Signed,
        );
    let value = if let Some(bounds) = bounds {
        let rhs_signed = lowerer.self_signed(expression);
        let value = lowerer.assigned_value(entry, expression, bounds.width());
        lowerer.resize(entry, value, bounds.width(), rhs_signed)
    } else if integer {
        let value = lowerer.assigned_value(entry, expression, 32);
        lowerer.resize(entry, value, 32, signed)
    } else if (declaration.type_is_explicit && declaration.param_type == ParamType::Real)
        || lowerer.is_real_expression(expression)
    {
        lowerer.real_expression(entry, expression)
    } else {
        lowerer.expression(entry, expression)
    };
    lowerer.builder.set_terminator(entry, CfgTerminator::Return);
    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }
    let (function, outputs) = lowerer
        .builder
        .finish_with_outputs(entry, &[value])
        .map_err(|error| {
            refuse(format!(
                "parameter `{name}` expression failed validation: {error:?}"
            ))
        })?;
    let value = evaluate_constant_expression(function, outputs[0], declaration.span.into())
        .map_err(|error| {
            refuse(format!(
                "parameter `{name}` cannot be evaluated as a digital constant: {error}"
            ))
        })?;
    resolved.bounds.remove(name);
    if let Some(bounds) = bounds {
        resolved.bounds.insert(name.into(), bounds);
    }
    resolved.integers.remove(name);
    resolved.reals.remove(name);
    resolved.non_finite_reals.remove(name);
    match value {
        DigitalScalar::FourState(value) => {
            if let Some(integer) = value
                .to_integer(signed)
                .and_then(|value| i64::try_from(value).ok())
            {
                resolved.integers.insert(name.into(), integer);
            }
            resolved.bits.insert(name.into(), (value, signed));
        }
        DigitalScalar::Integer(value) => {
            resolved.integers.insert(name.into(), i64::from(value));
            resolved.bits.insert(
                name.into(),
                (FourStateValue::from_integer(32, i128::from(value)), true),
            );
        }
        DigitalScalar::Real(value) => {
            if value.is_finite() {
                resolved.reals.insert(name.into(), value);
                if let Some(integer) = crate::array_index::checked_rounded_i64(value)
                    .ok()
                    .filter(|_| value.fract() == 0.0)
                {
                    resolved.integers.insert(name.into(), integer);
                }
            } else {
                resolved.non_finite_reals.insert(name.into(), value);
            }
        }
        DigitalScalar::Effect => {
            return Err(refuse(format!(
                "parameter `{name}` produced an effect instead of a value"
            )));
        }
    }
    Ok(())
}

/// Produce one initial value per storage cell, in increasing logical-index order.
pub(super) fn initializers(
    signal: &AnalyzedDigitalSignal,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Vec<Option<super::super::digital::DigitalInitialValue>>, Vec<DigitalLoweringDiagnostic>>
{
    let Some(bounds) = signal.unpacked else {
        return Ok(vec![initializer(signal, constants, time_scale)?]);
    };
    let len = bounds.width() as usize;
    let Some(expression) = &signal.initializer else {
        return Ok(vec![None; len]);
    };
    let refuse = |detail: String| {
        vec![DigitalLoweringDiagnostic::refusal(
            format!("declaration initializer of `{}`: {detail}", signal.name),
            signal.span.into(),
        )]
    };
    let Expression::ArrayLiteral(literal) = expression else {
        return Err(refuse("requires an array literal".into()));
    };
    if literal.first_replication().is_some() {
        return Err(refuse(
            "replicated array initialization requires element-pattern expansion".into(),
        ));
    }
    if literal.elements.len() != len {
        return Err(refuse(format!(
            "requires {len} elements, found {}",
            literal.elements.len()
        )));
    }
    let mut element = signal.clone();
    element.unpacked = None;
    let mut values = Vec::with_capacity(len);
    for value in &literal.elements {
        let ArrayLiteralElement::Value(value) = value else {
            unreachable!("replication rejected");
        };
        element.initializer = Some(value.clone());
        values.push(initializer(&element, constants, time_scale)?);
    }
    if bounds.msb > bounds.lsb {
        values.reverse();
    }
    Ok(values)
}

/// Module variable declarations require constant expressions (VAMS-2023 A.2.2).
/// Use the same typed expression evaluator as parameters, with the declared
/// assignment type, so rounding, overflow and four-state bits are preserved.
pub(super) fn initializer(
    signal: &AnalyzedDigitalSignal,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Option<super::super::digital::DigitalInitialValue>, Vec<DigitalLoweringDiagnostic>> {
    use super::super::digital::DigitalInitialValue;
    let Some(expression) = &signal.initializer else {
        return Ok(None);
    };
    let refuse = |detail: String| {
        vec![DigitalLoweringDiagnostic::refusal(
            format!("declaration initializer of `{}`: {detail}", signal.name),
            signal.span.into(),
        )]
    };
    let empty_index = HashMap::new();
    let empty_analog = HashMap::new();
    let empty_arrays = HashMap::new();
    let mut probes = Vec::new();
    let mut signals = Vec::new();
    let mut lowerer = ProcessLowerer {
        process: None,
        local_arrays: Vec::new(),
        constant_expression: true,
        time_scale,
        signals: &mut signals,
        arrays: &empty_arrays,
        index: &empty_index,
        constants,
        analog_variables: &empty_analog,
        probes: &mut probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };
    let entry = lowerer.builder.create_block();
    lowerer.builder.seal_block(entry);
    let value = if signal.class.is_real() {
        lowerer.real_expression(entry, expression)
    } else {
        let signed = lowerer.self_signed(expression);
        let value = lowerer.assigned_value(entry, expression, signal.width);
        lowerer.resize(entry, value, signal.width, signed)
    };
    lowerer.builder.set_terminator(entry, CfgTerminator::Return);
    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }
    let (function, outputs) = lowerer
        .builder
        .finish_with_outputs(entry, &[value])
        .map_err(|error| refuse(format!("constant expression failed validation: {error:?}")))?;
    let value = evaluate_constant_expression(function, outputs[0], signal.span.into())
        .map_err(|error| refuse(format!("cannot evaluate constant expression: {error}")))?;
    Ok(Some(match value {
        DigitalScalar::FourState(value) if !signal.class.is_real() => {
            DigitalInitialValue::FourState(value)
        }
        DigitalScalar::Integer(value) if !signal.class.is_real() => DigitalInitialValue::FourState(
            FourStateValue::from_integer(signal.width, i128::from(value)),
        ),
        DigitalScalar::Real(value) if signal.class.is_real() && value.is_finite() => {
            DigitalInitialValue::Real(value)
        }
        _ => {
            return Err(refuse(
                "requires a finite real or a correctly typed integer constant".into(),
            ));
        }
    }))
}

// Implicit sensitivity excludes declaration initializers and event expressions.
// Parameter resolution must inspect both, and respect lexical shadowing.
fn collect_parameters(
    statement: &DigitalStatement,
    locals: &BTreeSet<String>,
    reads: &mut BTreeSet<String>,
) {
    let expression = |value: &Expression, reads: &mut BTreeSet<String>| {
        let mut names = BTreeSet::new();
        collect_expression_reads(value, &mut names);
        reads.extend(names.into_iter().filter(|name| !locals.contains(name)));
    };
    let timing = |control: &TimingControl, reads: &mut BTreeSet<String>| match control {
        TimingControl::Delay(delay) => expression(&delay.value, reads),
        TimingControl::Event(event) => {
            if let Some(count) = &event.repeat {
                expression(count, reads);
            }
            if let crate::ast::Sensitivity::Explicit(terms) = &event.sensitivity {
                for term in terms {
                    expression(&term.signal, reads);
                }
            }
        }
    };
    match statement {
        DigitalStatement::Null(_) => {}
        DigitalStatement::Block(block) => {
            let mut locals = locals.clone();
            locals.extend(block.variables.iter().flat_map(|declaration| {
                declaration.items.iter().map(|item| item.name.to_string())
            }));
            locals.extend(block.digital_variables.iter().flat_map(|declaration| {
                declaration.items.iter().map(|item| item.name.to_string())
            }));
            for declaration in &block.variables {
                for item in &declaration.items {
                    if let Some(value) = &item.init {
                        collect_visible(value, &locals, reads);
                    }
                    for dimension in &item.dimensions {
                        collect_visible(&dimension.start, &locals, reads);
                        collect_visible(&dimension.end, &locals, reads);
                    }
                }
            }
            for declaration in &block.digital_variables {
                if let Some(range) = &declaration.range {
                    collect_visible(&range.msb, &locals, reads);
                    collect_visible(&range.lsb, &locals, reads);
                }
                for item in &declaration.items {
                    if let Some(value) = &item.init {
                        collect_visible(value, &locals, reads);
                    }
                    for dimension in &item.dimensions {
                        collect_visible(&dimension.start, &locals, reads);
                        collect_visible(&dimension.end, &locals, reads);
                    }
                }
            }
            for child in &block.statements {
                collect_parameters(child, &locals, reads);
            }
        }
        DigitalStatement::BlockingAssign(assign) | DigitalStatement::NonblockingAssign(assign) => {
            expression(&assign.value, reads);
            let mut names = BTreeSet::new();
            collect_lvalue_index_reads(&assign.target, &mut names);
            reads.extend(names.into_iter().filter(|name| !locals.contains(name)));
            if let Some(control) = &assign.timing {
                timing(control, reads);
            }
        }
        DigitalStatement::Conditional(branch) => {
            expression(&branch.condition, reads);
            collect_parameters(&branch.then_branch, locals, reads);
            if let Some(branch) = &branch.else_branch {
                collect_parameters(branch, locals, reads);
            }
        }
        DigitalStatement::Case(case) => {
            expression(&case.selector, reads);
            for item in &case.items {
                for label in &item.labels {
                    expression(label, reads);
                }
                collect_parameters(&item.statement, locals, reads);
            }
            if let Some(branch) = &case.default {
                collect_parameters(branch, locals, reads);
            }
        }
        DigitalStatement::For(loop_) => {
            collect_parameters(
                &DigitalStatement::BlockingAssign((*loop_.init).clone()),
                locals,
                reads,
            );
            expression(&loop_.condition, reads);
            collect_parameters(
                &DigitalStatement::BlockingAssign((*loop_.update).clone()),
                locals,
                reads,
            );
            collect_parameters(&loop_.body, locals, reads);
        }
        DigitalStatement::While(loop_) => {
            expression(&loop_.condition, reads);
            collect_parameters(&loop_.body, locals, reads);
        }
        DigitalStatement::Repeat(loop_) => {
            expression(&loop_.count, reads);
            collect_parameters(&loop_.body, locals, reads);
        }
        DigitalStatement::Forever(loop_) => collect_parameters(&loop_.body, locals, reads),
        DigitalStatement::Timing(wait) => {
            timing(&wait.control, reads);
            if let Some(body) = &wait.statement {
                collect_parameters(body, locals, reads);
            }
        }
    }
}

fn collect_visible(value: &Expression, locals: &BTreeSet<String>, reads: &mut BTreeSet<String>) {
    let mut names = BTreeSet::new();
    collect_expression_reads(value, &mut names);
    reads.extend(names.into_iter().filter(|name| !locals.contains(name)));
}

/// Close an instance override in its parent's scope using the child's declared
/// assignment type. Only source expressions cross back into elaboration.
pub(super) fn override_literal(
    declaration: &ParameterDecl,
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Expression, String> {
    let expression = declaration
        .default
        .as_ref()
        .ok_or("missing override expression")?;
    if !declaration.dimensions.is_empty() || declaration.param_type == ParamType::String {
        return Err(
            "array and string digital parameter overrides require additional constant lowering"
                .into(),
        );
    }
    let diagnostic = |errors: Vec<DigitalLoweringDiagnostic>| {
        errors
            .into_iter()
            .map(|error| error.diagnostic.message)
            .collect::<Vec<_>>()
            .join("; ")
    };
    let mut required = vec![expression];
    if let Some(range) = &declaration.packed_range {
        required.extend([&range.msb, &range.lsb]);
    }
    let mut resolved = resolve(source, time_scale, &[], &[], required).map_err(diagnostic)?;
    // This name cannot shadow an operand: the expression is evaluated before
    // resolve_one publishes its result in the constant environment.
    let name = "$rspice_override";
    resolve_one(name, declaration, &mut resolved, time_scale).map_err(diagnostic)?;
    resolved_literal(name, &resolved, declaration.span)
}

fn resolved_literal(
    name: &str,
    resolved: &ResolvedConstants,
    span: crate::source::Span,
) -> Result<Expression, String> {
    if let Some((value, signed)) = resolved.bits.get(name) {
        let raw = format!(
            "{}'{}b{}",
            value.width(),
            if *signed { "s" } else { "" },
            value.spelling()
        );
        let literal = crate::four_state::decode(&raw).map_err(|error| error.to_string())?;
        return Ok(Expression::Digital(crate::ast::DigitalExpr::FourState(
            crate::ast::FourStateLit {
                value: literal,
                span,
            },
        )));
    }
    let value = resolved
        .real(name)
        .ok_or("non-finite parameter overrides require non-finite real execution")?;
    Ok(Expression::Number(crate::ast::NumberLit {
        value,
        raw: format!("{value:e}").into(),
        span,
    }))
}
