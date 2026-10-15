//! Constant function calls (IEEE 1364-2005 section 10.4.5).
//!
//! A parameter, a range or an initializer may call a function of its own
//! module. The call runs at elaboration on the interpreter that runs
//! processes. The functions it reaches are lowered against the resolved
//! parameters and nothing else, so a body that reads a signal is refused, and
//! with every variable in a frame slot, so each call starts from fresh
//! variables whether the function is static or automatic. A system task in a
//! body is ignored (rule d).
use super::super::digital::{DigitalRoutine, DigitalScopeId};
use super::routines::{RoutineScope, RoutineTable, lower_routine, signatures};
use super::*;
use crate::ast::{DigitalLValue, DigitalRoutine as SourceRoutine};
use crate::semantic::AnalyzedDigitalRoutine;

/// The functions a constant expression reaches, lowered for elaboration.
#[derive(Default)]
pub(super) struct ConstantRoutines {
    pub table: RoutineTable,
    pub routines: Vec<DigitalRoutine>,
}

/// The functions of `functions` that `roots` call, directly or through other
/// functions, in declaration order.
pub(super) fn reachable<'f>(
    functions: &'f [SourceRoutine],
    roots: &[&Expression],
) -> Vec<&'f SourceRoutine> {
    if functions.is_empty() {
        return Vec::new();
    }
    let mut reached = vec![false; functions.len()];
    let mut pending = Vec::new();
    for root in roots {
        mark_calls(functions, root, &mut reached, &mut pending);
    }
    while let Some(index) = pending.pop() {
        let mut expressions = Vec::new();
        routine_expressions(&functions[index], &mut expressions);
        for expression in expressions {
            mark_calls(functions, expression, &mut reached, &mut pending);
        }
    }
    functions
        .iter()
        .zip(reached)
        .filter_map(|(function, reached)| reached.then_some(function))
        .collect()
}

fn mark_calls(
    functions: &[SourceRoutine],
    expression: &Expression,
    reached: &mut [bool],
    pending: &mut Vec<usize>,
) {
    crate::ast::visit_expression(expression, &mut |node| {
        let Expression::Call(call) = node else {
            return;
        };
        if call.resolved_builtin {
            return;
        }
        if let Some(index) = functions
            .iter()
            .position(|function| function.name == call.name)
            && !reached[index]
        {
            reached[index] = true;
            pending.push(index);
        }
    });
}

/// Every expression a routine's declarations and body hold.
fn routine_expressions<'a>(routine: &'a SourceRoutine, out: &mut Vec<&'a Expression>) {
    declaration_expressions(routine, out);
    statement_expressions(&routine.body, out);
}

/// The expressions of a routine's argument, result and variable declarations.
pub(super) fn declaration_expressions<'a>(
    routine: &'a SourceRoutine,
    out: &mut Vec<&'a Expression>,
) {
    let result = match &routine.kind {
        crate::ast::DigitalRoutineKind::Function(ty) => Some(ty.as_ref()),
        crate::ast::DigitalRoutineKind::Task => None,
    };
    for range in routine
        .arguments
        .iter()
        .map(|argument| &argument.ty)
        .chain(result)
        .filter_map(|ty| ty.range.as_ref())
    {
        out.extend([&range.msb, &range.lsb]);
    }
    variable_expressions(&routine.variables, &routine.digital_variables, out);
}

fn variable_expressions<'a>(
    variables: &'a [crate::ast::VariableDecl],
    digital_variables: &'a [crate::ast::DigitalVariableDecl],
    out: &mut Vec<&'a Expression>,
) {
    for item in variables.iter().flat_map(|declaration| &declaration.items) {
        out.extend(&item.init);
        for dimension in &item.dimensions {
            out.extend([&dimension.start, &dimension.end]);
        }
    }
    for declaration in digital_variables {
        if let Some(range) = &declaration.range {
            out.extend([&range.msb, &range.lsb]);
        }
        for item in &declaration.items {
            out.extend(&item.init);
            for dimension in &item.dimensions {
                out.extend([&dimension.start, &dimension.end]);
            }
        }
    }
}

fn statement_expressions<'a>(statement: &'a DigitalStatement, out: &mut Vec<&'a Expression>) {
    match statement {
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            variable_expressions(&block.variables, &block.digital_variables, out);
            for statement in &block.statements {
                statement_expressions(statement, out);
            }
        }
        DigitalStatement::BlockingAssign(assign) | DigitalStatement::NonblockingAssign(assign) => {
            out.push(&assign.value);
            lvalue_expressions(&assign.target, out);
        }
        DigitalStatement::Conditional(conditional) => {
            out.push(&conditional.condition);
            statement_expressions(&conditional.then_branch, out);
            if let Some(branch) = &conditional.else_branch {
                statement_expressions(branch, out);
            }
        }
        DigitalStatement::Case(case) => {
            out.push(&case.selector);
            for item in &case.items {
                out.extend(&item.labels);
                statement_expressions(&item.statement, out);
            }
            if let Some(default) = &case.default {
                statement_expressions(default, out);
            }
        }
        DigitalStatement::For(statement) => {
            for assign in [&statement.init, &statement.update] {
                out.push(&assign.value);
                lvalue_expressions(&assign.target, out);
            }
            out.push(&statement.condition);
            statement_expressions(&statement.body, out);
        }
        DigitalStatement::While(statement) => {
            out.push(&statement.condition);
            statement_expressions(&statement.body, out);
        }
        DigitalStatement::Repeat(statement) => {
            out.push(&statement.count);
            statement_expressions(&statement.body, out);
        }
        DigitalStatement::Forever(statement) => statement_expressions(&statement.body, out),
        DigitalStatement::Timing(timing) => {
            if let Some(statement) = &timing.statement {
                statement_expressions(statement, out);
            }
        }
        DigitalStatement::Wait(wait) => {
            out.push(&wait.condition);
            if let Some(statement) = &wait.statement {
                statement_expressions(statement, out);
            }
        }
        DigitalStatement::SystemTask(task) => out.extend(&task.args),
        DigitalStatement::TaskEnable(enable) => out.extend(&enable.args),
        DigitalStatement::EventTrigger(trigger) => lvalue_expressions(&trigger.target, out),
        DigitalStatement::ProceduralContinuous(statement) => {
            lvalue_expressions(&statement.target, out);
            out.extend(&statement.value);
        }
        DigitalStatement::Disable(_) | DigitalStatement::Null(_) => {}
    }
}

fn lvalue_expressions<'a>(target: &'a DigitalLValue, out: &mut Vec<&'a Expression>) {
    match target {
        DigitalLValue::Identifier { .. } => {}
        DigitalLValue::BitSelect { index, .. } => out.push(index),
        DigitalLValue::PartSelect { msb, lsb, .. } => out.extend([&**msb, &**lsb]),
        DigitalLValue::ArraySelect(select) => out.extend(select.children()),
        DigitalLValue::IndexedPartSelect(select) => out.extend(select.children()),
        DigitalLValue::Concat { elements, .. } => {
            for element in elements {
                lvalue_expressions(element, out);
            }
        }
    }
}

/// Lower the functions `roots` reach against the resolved parameters.
pub(super) fn lower(
    roots: &[&Expression],
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<ConstantRoutines, Vec<DigitalLoweringDiagnostic>> {
    let reached = reachable(&constants.functions, roots);
    if reached.is_empty() {
        return Ok(ConstantRoutines::default());
    }
    let analyzed: Vec<_> = reached
        .into_iter()
        .map(|routine| AnalyzedDigitalRoutine {
            routine: routine.clone(),
            suspends: false,
        })
        .collect();
    let mut diagnostics = Vec::new();
    let table = signatures(&analyzed, &mut 0, constants, time_scale, &mut diagnostics);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let mut signals = Vec::new();
    let mut probes = Vec::new();
    let arrays = HashMap::new();
    let index = HashMap::new();
    let array_names = HashSet::new();
    let analog_variables = HashMap::new();
    let mut calls = PlanCalls {
        disabled: std::sync::Arc::new(super::disable::disabled_names(
            analyzed.iter().map(|routine| &routine.routine.body),
        )),
        ..PlanCalls::default()
    };
    let scope = ScopeFrame {
        id: DigitalScopeId(0),
        path: SmolStr::default(),
        module: SmolStr::default(),
        time_scale,
    };
    let mut routines = Vec::with_capacity(analyzed.len());
    for routine in &analyzed {
        routines.push(lower_routine(
            routine,
            RoutineScope {
                signals: &mut signals,
                arrays: &arrays,
                index: &index,
                array_names: &array_names,
                constants,
                analog_variables: &analog_variables,
                probes: &mut probes,
                time_scale,
                calls: &mut calls,
                scope: &scope,
                routines: &table,
                prefix: "",
                constant: true,
            },
        )?);
    }
    Ok(ConstantRoutines { table, routines })
}
