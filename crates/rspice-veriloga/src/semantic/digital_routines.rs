//! Tasks, functions and the thread statements of the discrete domain
//! (IEEE 1364-2005 sections 9.7.3, 9.7.6, 9.8.2, 10.2-10.4).
//!
//! A routine's body is checked like a process body with its arguments and
//! locals as the innermost declarative region, and its result variable (a
//! function's own name) among them. Lowering reads the routine verbatim.

use super::digital::{AnalyzedDigitalSignal, AnalyzedProcessLocal, ProcessLocalKind};
use super::{SemanticAnalyzer, SemanticErrorKind};
use crate::ast::*;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// What a call site needs to know about a routine.
#[derive(Debug, Clone)]
pub(crate) struct RoutineSignature {
    pub is_function: bool,
    pub directions: Vec<PortDirection>,
    /// Whether the body can suspend, through its own timing controls or
    /// through a task it enables.
    pub suspends: bool,
    pub span: crate::source::Span,
}

/// A task or function of an analyzed module, ready for lowering.
#[derive(Debug, Clone)]
pub struct AnalyzedDigitalRoutine {
    pub routine: DigitalRoutine,
    /// Whether its body can suspend (IEEE 1364-2005 section 10.2.1: a task
    /// may contain timing controls; a function may not).
    pub suspends: bool,
}

impl SemanticAnalyzer {
    /// Register the module's routines and the names a `disable` may target,
    /// before any body is checked.
    pub(super) fn register_digital_routines(&mut self, module: &Module) {
        self.digital_routines.clear();
        self.digital_scope_names.clear();
        for routine in &module.digital_routines {
            if let Some(existing) = self.digital_routines.get(&routine.name) {
                let first_defined = existing.span;
                self.record_error_at(
                    SemanticErrorKind::DuplicateSymbol {
                        name: routine.name.clone(),
                        first_defined,
                    },
                    routine.span,
                );
                continue;
            }
            self.digital_routines.insert(
                routine.name.clone(),
                RoutineSignature {
                    is_function: routine.is_function(),
                    directions: routine
                        .arguments
                        .iter()
                        .map(|argument| argument.direction)
                        .collect(),
                    suspends: false,
                    span: routine.span,
                },
            );
            if !routine.is_function() {
                self.digital_scope_names.insert(routine.name.clone());
            }
            collect_block_names(&routine.body, &mut self.digital_scope_names);
        }
        for process in &module.digital_processes {
            collect_block_names(&process.body, &mut self.digital_scope_names);
        }
        // Whether a task suspends depends on the tasks it enables; iterate to
        // the fixed point, which a recursive task reaches as well.
        loop {
            let mut changed = false;
            for routine in &module.digital_routines {
                let suspends = statement_suspends(&routine.body, &self.digital_routines);
                if let Some(signature) = self.digital_routines.get_mut(&routine.name)
                    && suspends
                    && !signature.suspends
                {
                    signature.suspends = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    /// Whether a process body can suspend, counting task enables of tasks
    /// that do.
    pub(super) fn digital_statement_suspends(&self, statement: &DigitalStatement) -> bool {
        statement_suspends(statement, &self.digital_routines)
    }

    /// Check every routine body and return them for lowering.
    pub(super) fn analyze_digital_routines(
        &mut self,
        module: &Module,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> Vec<AnalyzedDigitalRoutine> {
        let mut analyzed = Vec::new();
        for routine in &module.digital_routines {
            let scope = self.routine_scope(routine);
            self.digital_scopes.clear();
            self.digital_scopes.push(scope);
            if routine.is_function() {
                self.check_function_restrictions(routine);
            }
            self.check_digital_statement(&routine.body, signals, index);
            self.digital_scopes.clear();
            if routine.automatic {
                let mut locals: HashSet<SmolStr> = routine
                    .arguments
                    .iter()
                    .map(|argument| argument.name.clone())
                    .collect();
                locals.extend(
                    routine
                        .variables
                        .iter()
                        .flat_map(|declaration| &declaration.items)
                        .map(|item| item.name.clone()),
                );
                locals.extend(
                    routine
                        .digital_variables
                        .iter()
                        .flat_map(|declaration| &declaration.items)
                        .map(|item| item.name.clone()),
                );
                if let Some((name, span, construct)) =
                    automatic_nonblocking(&routine.body, &mut locals)
                {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{construct} `{name}`, a variable of the automatic {} `{}` (IEEE \
                             1364-2005 section 10.2.1)",
                            if routine.is_function() {
                                "function"
                            } else {
                                "task"
                            },
                            routine.name
                        )),
                        span,
                    );
                }
            }
            analyzed.push(AnalyzedDigitalRoutine {
                suspends: self
                    .digital_routines
                    .get(&routine.name)
                    .is_some_and(|signature| signature.suspends),
                routine: routine.clone(),
            });
        }
        analyzed
    }

    /// The declarative region of a routine: its arguments, its locals, and a
    /// function's result variable.
    fn routine_scope(&mut self, routine: &DigitalRoutine) -> Vec<AnalyzedProcessLocal> {
        let mut scope: Vec<AnalyzedProcessLocal> = Vec::new();
        if let DigitalRoutineKind::Function(ty) = &routine.kind {
            let local = self.routine_value_local(&routine.name, ty, routine.span);
            scope.push(local);
        }
        for argument in &routine.arguments {
            let local = self.routine_value_local(&argument.name, &argument.ty, argument.span);
            self.push_routine_local(&mut scope, local);
        }
        let block = DigitalBlock {
            name: None,
            variables: routine.variables.clone(),
            digital_variables: routine.digital_variables.clone(),
            statements: Vec::new(),
            span: routine.span,
        };
        for local in self.collect_process_locals(&block) {
            self.push_routine_local(&mut scope, local);
        }
        scope
    }

    fn push_routine_local(
        &mut self,
        scope: &mut Vec<AnalyzedProcessLocal>,
        local: AnalyzedProcessLocal,
    ) {
        if let Some(existing) = scope.iter().find(|entry| entry.name == local.name) {
            let first_defined = existing.span;
            self.record_error_at(
                SemanticErrorKind::DuplicateSymbol {
                    name: local.name.clone(),
                    first_defined,
                },
                local.span,
            );
            return;
        }
        scope.push(local);
    }

    fn routine_value_local(
        &mut self,
        name: &SmolStr,
        ty: &DigitalRoutineType,
        span: crate::source::Span,
    ) -> AnalyzedProcessLocal {
        let (kind, range, width) = match ty.kind {
            DigitalRoutineValueKind::Integer => (ProcessLocalKind::Integer, None, 32),
            DigitalRoutineValueKind::Real => (ProcessLocalKind::Real, None, 0),
            DigitalRoutineValueKind::Time => (
                ProcessLocalKind::Reg,
                Some(super::digital::VectorBounds { msb: 63, lsb: 0 }),
                64,
            ),
            DigitalRoutineValueKind::Reg => {
                let range = self.resolve_vector_range(ty.range.as_ref(), "reg");
                (
                    ProcessLocalKind::Reg,
                    range,
                    range.map_or(1, super::digital::VectorBounds::width),
                )
            }
        };
        AnalyzedProcessLocal {
            unpacked: None,
            dimensions: Vec::new(),
            name: name.clone(),
            kind,
            range,
            width,
            span,
        }
    }

    /// IEEE 1364-2005 section 10.4.4: a function runs in zero time, so it
    /// may contain no timing control, task enable or `fork`, and section
    /// 10.4.1 gives it inputs only.
    fn check_function_restrictions(&mut self, routine: &DigitalRoutine) {
        if let Some(argument) = routine
            .arguments
            .iter()
            .find(|argument| argument.direction != PortDirection::Input)
        {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "function `{}` declares `{}` as an output; IEEE 1364-2005 section \
                     10.4.1 gives a function input arguments only",
                    routine.name, argument.name
                )),
                argument.span,
            );
        }
        let mut found = None;
        find_function_violation(&routine.body, &mut found);
        if let Some((what, span)) = found {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "function `{}` contains {what}; IEEE 1364-2005 section 10.4.4 makes a \
                     function execute in zero time, so it may not contain a timing control, \
                     a `wait`, a task enable or a `fork`",
                    routine.name
                )),
                span,
            );
        }
    }

    /// A user task enable: the task exists, is a task, and gets one argument
    /// per declared argument; an output or inout argument is a variable target.
    pub(super) fn check_task_enable(
        &mut self,
        enable: &DigitalTaskEnable,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        for argument in &enable.args {
            self.check_digital_expression(argument, signals, index);
        }
        if is_reference_symbol(&enable.name) {
            // A hierarchical task name; lowering resolves it against the
            // elaborated design.
            return;
        }
        let Some(signature) = self.digital_routines.get(&enable.name).cloned() else {
            self.record_error_at(
                SemanticErrorKind::UndeclaredSymbol {
                    name: enable.name.clone(),
                },
                enable.span,
            );
            return;
        };
        if signature.is_function {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{}` is a function, and a function is called inside an expression, not \
                     enabled as a statement (IEEE 1364-2005 section 10.4.3)",
                    enable.name
                )),
                enable.span,
            );
            return;
        }
        self.check_routine_arity(&enable.name, &signature, enable.args.len(), enable.span);
        for (argument, direction) in enable.args.iter().zip(&signature.directions) {
            if *direction != PortDirection::Input {
                match expression_as_lvalue(argument) {
                    Some(target) => self.check_digital_lvalue(&target, signals, index, true),
                    None => self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "an output or inout argument of task `{}` must be a variable, a \
                             select of one, or a concatenation of these (IEEE 1364-2005 \
                             section 10.2.2)",
                            enable.name
                        )),
                        argument.span(),
                    ),
                }
            }
        }
    }

    /// A user function call: arity and argument expressions. Returns whether
    /// `call` names a user function.
    pub(super) fn check_function_call(
        &mut self,
        call: &CallExpr,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> bool {
        if is_reference_symbol(&call.name) {
            // A hierarchical function name; lowering resolves it against the
            // elaborated design, as it does a hierarchical task enable.
            for argument in &call.args {
                self.check_digital_expression(argument, signals, index);
            }
            return true;
        }
        let Some(signature) = self.digital_routines.get(&call.name).cloned() else {
            return false;
        };
        if !signature.is_function {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{}` is a task, and a task is enabled as a statement, not called inside \
                     an expression (IEEE 1364-2005 section 10.2.2)",
                    call.name
                )),
                call.span,
            );
        } else {
            self.check_routine_arity(&call.name, &signature, call.args.len(), call.span);
        }
        for argument in &call.args {
            self.check_digital_expression(argument, signals, index);
        }
        true
    }

    fn check_routine_arity(
        &mut self,
        name: &SmolStr,
        signature: &RoutineSignature,
        given: usize,
        span: crate::source::Span,
    ) {
        if signature.directions.len() != given {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{name}` takes {} argument{} and is given {given}",
                    signature.directions.len(),
                    if signature.directions.len() == 1 {
                        ""
                    } else {
                        "s"
                    },
                )),
                span,
            );
        }
    }

    /// `disable name`: a named block or a task of this module.
    pub(super) fn check_disable(&mut self, disable: &DigitalDisable) {
        if is_reference_symbol(&disable.target)
            || self.digital_scope_names.contains(&disable.target)
        {
            return;
        }
        self.record_error_at(
            SemanticErrorKind::UndeclaredSymbol {
                name: disable.target.clone(),
            },
            disable.span,
        );
    }

    /// Whether `name` is a module-level named event.
    pub(super) fn names_digital_event(
        &self,
        name: &SmolStr,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> bool {
        if self
            .digital_scopes
            .iter()
            .any(|scope| scope.iter().any(|local| local.name == *name))
        {
            return false;
        }
        index.get(name).is_some_and(|position| {
            matches!(
                signals[*position].class,
                super::digital::DigitalSignalClass::Variable(DigitalVariableKind::Event)
            )
        })
    }

    /// A named event is declared before it is used: only a net can be
    /// declared implicitly (IEEE 1364-2005 section 4.5).
    pub(super) fn check_event_declared_before(
        &mut self,
        name: &SmolStr,
        used: crate::source::Span,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        // A borrowed declaration stands at the reference; the target's own
        // declaration is in another scope.
        if is_reference_symbol(name) {
            return;
        }
        let Some(declared) = index.get(name).map(|position| signals[*position].span) else {
            return;
        };
        if declared.source == used.source && declared.start > used.start {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "the named event `{name}` is used before its declaration (IEEE 1364-2005 \
                     section 4.5 declares only nets implicitly)"
                )),
                used,
            );
        }
    }

    /// `-> event`: the target is a named event.
    pub(super) fn check_event_trigger(
        &mut self,
        trigger: &DigitalEventTrigger,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        let name = match &trigger.target {
            DigitalLValue::Identifier { name, .. } => name,
            _ => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(
                        "`->` triggers a named event; write `-> name`".to_string(),
                    ),
                    trigger.span,
                );
                return;
            }
        };
        if is_reference_symbol(name) && !index.contains_key(name) {
            return;
        }
        if self.names_digital_event(name, signals, index) {
            self.check_event_declared_before(name, trigger.span, signals, index);
            return;
        }
        self.record_error_at(
            SemanticErrorKind::InvalidExpression(format!(
                "`-> {name}` names something that is not a named event (IEEE 1364-2005 \
                 section 9.7.3)"
            )),
            trigger.span,
        );
    }
}

/// The variable target an output argument expression denotes.
pub(crate) fn expression_as_lvalue(expression: &Expression) -> Option<DigitalLValue> {
    Some(match expression {
        Expression::Identifier(identifier) => DigitalLValue::Identifier {
            name: identifier.name.clone(),
            span: identifier.span,
        },
        Expression::ArrayAccess(access) => DigitalLValue::BitSelect {
            name: access.array.clone(),
            index: access.index.clone(),
            span: access.span,
        },
        Expression::Digital(DigitalExpr::PartSelect(select)) => DigitalLValue::PartSelect {
            name: select.name.clone(),
            msb: select.msb.clone(),
            lsb: select.lsb.clone(),
            span: select.span,
        },
        Expression::Digital(DigitalExpr::ArraySelect(select)) => {
            DigitalLValue::ArraySelect(select.clone())
        }
        Expression::Digital(DigitalExpr::IndexedPartSelect(select)) => {
            DigitalLValue::IndexedPartSelect(select.clone())
        }
        Expression::ArrayLiteral(literal) if !literal.assignment_pattern => DigitalLValue::Concat {
            elements: literal
                .elements
                .iter()
                .map(|element| match element {
                    ArrayLiteralElement::Value(value) => expression_as_lvalue(value),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()?,
            span: literal.span,
        },
        _ => return None,
    })
}

fn statement_suspends(
    statement: &DigitalStatement,
    routines: &HashMap<SmolStr, RoutineSignature>,
) -> bool {
    let nested = |statement: &DigitalStatement| statement_suspends(statement, routines);
    match statement {
        DigitalStatement::Timing(_) | DigitalStatement::Wait(_) => true,
        DigitalStatement::TaskEnable(enable) => routines
            .get(&enable.name)
            .is_some_and(|signature| signature.suspends),
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            block.statements.iter().any(nested)
        }
        DigitalStatement::Conditional(conditional) => {
            nested(&conditional.then_branch)
                || conditional.else_branch.as_deref().is_some_and(nested)
        }
        DigitalStatement::Case(case) => {
            case.items.iter().any(|item| nested(&item.statement))
                || case.default.as_deref().is_some_and(nested)
        }
        DigitalStatement::For(statement) => nested(&statement.body),
        DigitalStatement::While(statement) => nested(&statement.body),
        DigitalStatement::Repeat(statement) => nested(&statement.body),
        DigitalStatement::Forever(statement) => nested(&statement.body),
        // A nonblocking assignment's timing control delays the update, not
        // the process (section 9.2.2).
        DigitalStatement::BlockingAssign(assign) => assign.timing.is_some(),
        DigitalStatement::NonblockingAssign(_)
        | DigitalStatement::SystemTask(_)
        | DigitalStatement::Disable(_)
        | DigitalStatement::EventTrigger(_)
        | DigitalStatement::ProceduralContinuous(_)
        | DigitalStatement::Null(_) => false,
    }
}

/// The variable an expression node reads directly, if it names one.
fn expression_base_name(expression: &Expression) -> Option<(SmolStr, crate::source::Span)> {
    match expression {
        Expression::Identifier(identifier) => Some((identifier.name.clone(), identifier.span)),
        Expression::ArrayAccess(access) => Some((access.array.clone(), access.span)),
        Expression::Digital(digital) => digital
            .base_name()
            .map(|name| (name.clone(), expression.span())),
        _ => None,
    }
}

/// The first nonblocking write of an automatic variable: the update would
/// land after the activation that owns the variable may be gone. `locals`
/// grows by each block's declarations, which are automatic too.
fn automatic_nonblocking(
    statement: &DigitalStatement,
    locals: &mut HashSet<SmolStr>,
) -> Option<(SmolStr, crate::source::Span, &'static str)> {
    if let DigitalStatement::Block(block) | DigitalStatement::Fork(block) = statement {
        locals.extend(
            block
                .variables
                .iter()
                .flat_map(|declaration| &declaration.items)
                .map(|item| item.name.clone()),
        );
        locals.extend(
            block
                .digital_variables
                .iter()
                .flat_map(|declaration| &declaration.items)
                .map(|item| item.name.clone()),
        );
    }
    let mut nested = |statement: &DigitalStatement| automatic_nonblocking(statement, locals);
    match statement {
        DigitalStatement::NonblockingAssign(assign) => assign
            .target
            .written_names()
            .into_iter()
            .find(|(name, _)| locals.contains(*name))
            .map(|(name, span)| (name.clone(), span, "a nonblocking assignment cannot write")),
        // Section 10.2.1 forbids an automatic variable as the target or an
        // operand of a procedural continuous assignment: the override outlives
        // the activation that owns the variable.
        DigitalStatement::ProceduralContinuous(statement) => {
            let construct = match statement.kind {
                ProceduralContinuousKind::Force | ProceduralContinuousKind::Release => {
                    "a `force` or `release` cannot use"
                }
                ProceduralContinuousKind::Assign | ProceduralContinuousKind::Deassign => {
                    "a procedural `assign` or `deassign` cannot use"
                }
            };
            let mut found = statement
                .target
                .written_names()
                .into_iter()
                .find(|(name, _)| locals.contains(*name))
                .map(|(name, span)| (name.clone(), span, construct));
            if let Some(value) = &statement.value
                && found.is_none()
            {
                crate::ast::visit_expression(value, &mut |expression| {
                    if found.is_none()
                        && let Some((name, span)) = expression_base_name(expression)
                        && locals.contains(&name)
                    {
                        found = Some((name, span, construct));
                    }
                });
            }
            found
        }
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            block.statements.iter().find_map(nested)
        }
        DigitalStatement::Conditional(conditional) => nested(&conditional.then_branch)
            .or_else(|| conditional.else_branch.as_deref().and_then(nested)),
        DigitalStatement::Case(case) => case
            .items
            .iter()
            .find_map(|item| nested(&item.statement))
            .or_else(|| case.default.as_deref().and_then(nested)),
        DigitalStatement::For(statement) => nested(&statement.body),
        DigitalStatement::While(statement) => nested(&statement.body),
        DigitalStatement::Repeat(statement) => nested(&statement.body),
        DigitalStatement::Forever(statement) => nested(&statement.body),
        DigitalStatement::Timing(timing) => timing.statement.as_deref().and_then(nested),
        DigitalStatement::Wait(wait) => wait.statement.as_deref().and_then(nested),
        DigitalStatement::BlockingAssign(_)
        | DigitalStatement::SystemTask(_)
        | DigitalStatement::TaskEnable(_)
        | DigitalStatement::Disable(_)
        | DigitalStatement::EventTrigger(_)
        | DigitalStatement::Null(_) => None,
    }
}

/// The first construct section 10.4.4 forbids in a function body.
fn find_function_violation(
    statement: &DigitalStatement,
    found: &mut Option<(&'static str, crate::source::Span)>,
) {
    if found.is_some() {
        return;
    }
    match statement {
        DigitalStatement::Timing(timing) => *found = Some(("a timing control", timing.span)),
        DigitalStatement::Wait(wait) => *found = Some(("a `wait`", wait.span)),
        DigitalStatement::TaskEnable(enable) => *found = Some(("a task enable", enable.span)),
        DigitalStatement::Fork(block) => *found = Some(("a `fork`", block.span)),
        DigitalStatement::ProceduralContinuous(statement) => {
            *found = Some((
                "a procedural continuous assignment, which section 10.4.4 also forbids",
                statement.span,
            ))
        }
        DigitalStatement::BlockingAssign(assign) | DigitalStatement::NonblockingAssign(assign)
            if assign.timing.is_some() =>
        {
            *found = Some(("an intra-assignment timing control", assign.span))
        }
        DigitalStatement::Block(block) => {
            for statement in &block.statements {
                find_function_violation(statement, found);
            }
        }
        DigitalStatement::Conditional(conditional) => {
            find_function_violation(&conditional.then_branch, found);
            if let Some(branch) = &conditional.else_branch {
                find_function_violation(branch, found);
            }
        }
        DigitalStatement::Case(case) => {
            for item in &case.items {
                find_function_violation(&item.statement, found);
            }
            if let Some(default) = &case.default {
                find_function_violation(default, found);
            }
        }
        DigitalStatement::For(statement) => find_function_violation(&statement.body, found),
        DigitalStatement::While(statement) => find_function_violation(&statement.body, found),
        DigitalStatement::Repeat(statement) => find_function_violation(&statement.body, found),
        DigitalStatement::Forever(statement) => find_function_violation(&statement.body, found),
        DigitalStatement::BlockingAssign(_)
        | DigitalStatement::NonblockingAssign(_)
        | DigitalStatement::SystemTask(_)
        | DigitalStatement::Disable(_)
        | DigitalStatement::EventTrigger(_)
        | DigitalStatement::Null(_) => {}
    }
}

/// Every block name in a body, `fork` names included.
fn collect_block_names(statement: &DigitalStatement, names: &mut HashSet<SmolStr>) {
    match statement {
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            if let Some(name) = &block.name {
                names.insert(name.clone());
            }
            for statement in &block.statements {
                collect_block_names(statement, names);
            }
        }
        DigitalStatement::Conditional(conditional) => {
            collect_block_names(&conditional.then_branch, names);
            if let Some(branch) = &conditional.else_branch {
                collect_block_names(branch, names);
            }
        }
        DigitalStatement::Case(case) => {
            for item in &case.items {
                collect_block_names(&item.statement, names);
            }
            if let Some(default) = &case.default {
                collect_block_names(default, names);
            }
        }
        DigitalStatement::For(statement) => collect_block_names(&statement.body, names),
        DigitalStatement::While(statement) => collect_block_names(&statement.body, names),
        DigitalStatement::Repeat(statement) => collect_block_names(&statement.body, names),
        DigitalStatement::Forever(statement) => collect_block_names(&statement.body, names),
        DigitalStatement::Timing(timing) => {
            if let Some(statement) = &timing.statement {
                collect_block_names(statement, names);
            }
        }
        DigitalStatement::Wait(wait) => {
            if let Some(statement) = &wait.statement {
                collect_block_names(statement, names);
            }
        }
        DigitalStatement::BlockingAssign(_)
        | DigitalStatement::NonblockingAssign(_)
        | DigitalStatement::SystemTask(_)
        | DigitalStatement::TaskEnable(_)
        | DigitalStatement::Disable(_)
        | DigitalStatement::EventTrigger(_)
        | DigitalStatement::ProceduralContinuous(_)
        | DigitalStatement::Null(_) => {}
    }
}
