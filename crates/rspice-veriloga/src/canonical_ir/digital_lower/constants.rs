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

pub(super) fn lower_math_call(
    lowerer: &mut ProcessLowerer<'_>,
    block: BlockId,
    call: &crate::ast::CallExpr,
) -> ValueId {
    let kind = match (math_call(&call.name), call.args.as_slice()) {
        (Some(MathCall::Unary(op)), [input]) => CfgValueKind::Unary {
            op,
            input: lowerer.real_expression(block, input),
        },
        (Some(MathCall::Binary(op)), [left, right]) => CfgValueKind::Binary {
            op,
            left: lowerer.real_expression(block, left),
            right: lowerer.real_expression(block, right),
        },
        _ => {
            lowerer.error(
                format!("`{}` has no supported constant math signature", call.name),
                call.span,
            );
            return lowerer.real_constant(0.0);
        }
    };
    lowerer.builder.push(block, CfgValueType::Real, kind)
}

pub(super) fn resolve(
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    processes: &[AnalyzedDigitalProcess],
    assignments: &[crate::semantic::AnalyzedContinuousAssign],
) -> Result<ResolvedConstants, Vec<DigitalLoweringDiagnostic>> {
    let mut required = BTreeSet::new();
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
        pending.extend(dependencies.into_iter().rev().map(|name| (name, false)));
    }
    Ok(resolved)
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
    let mut probes = Vec::new();
    let mut lowerer = ProcessLowerer {
        constant_expression: true,
        time_scale,
        signals: &[],
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
    let integer = declaration.type_is_explicit && declaration.param_type == ParamType::Integer;
    let signed = integer || lowerer.self_signed(expression);
    let value = if integer {
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
            for declaration in &block.variables {
                for item in &declaration.items {
                    if let Some(value) = &item.init {
                        collect_visible(value, &locals, reads);
                    }
                    locals.insert(item.name.to_string());
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
                    locals.insert(item.name.to_string());
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
