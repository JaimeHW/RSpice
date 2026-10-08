//! Lower cross-domain event subscriptions to retained analog occurrence counters.
//! Analog operators keep their ordinary state, root detection and rollback. A
//! private digital signal carries occurrence counts, independently of data values.
use std::collections::{BTreeMap, BTreeSet};
use crate::ast::*;
use crate::error::{CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct AnalogEventBinding {
    /// Authored event-assigned variable, absent for an event-function site.
    pub source_variable: Option<SmolStr>,
    pub variable: SmolStr,
    pub signal: SmolStr,
    pub span: Span,
}

pub(super) struct LoweredAnalogEvents {
    pub module: Module,
    pub bindings: Vec<AnalogEventBinding>,
    pub event_assigned_variables: Vec<SmolStr>,
    pub immutable_variables: Vec<SmolStr>,
}

pub(super) fn lower(source: &Module) -> CompileResult<LoweredAnalogEvents> {
    let mut module = source.clone();
    let mut writes = BTreeMap::<SmolStr, (bool, bool)>::new();
    let mut analog_local_names = BTreeSet::new();
    if let Some(block) = &source.analog_block {
        for statement in &block.statements {
            collect_writes(
                statement,
                false,
                &BTreeSet::new(),
                &mut writes,
                &mut analog_local_names,
            );
        }
    }
    // A variable with no numerical-body writer is constant after startup.
    // Declaration and analog-initial assignments still run before it is read.
    // Digital ownership is resolved later and excludes digital-written names.
    let immutable_variables = source
        .variables
        .iter()
        .flat_map(|decl| &decl.items)
        .filter(|item| item.dimensions.is_empty() && !writes.contains_key(&item.name))
        .map(|item| item.name.clone())
        .collect();
    if let Some(block) = &source.analog_initial {
        for statement in &block.statements {
            collect_writes(
                statement,
                false,
                &BTreeSet::new(),
                &mut writes,
                &mut analog_local_names,
            );
        }
    }
    let eligible: BTreeSet<_> = source
        .variables
        .iter()
        .flat_map(|decl| &decl.items)
        .filter(|item| item.dimensions.is_empty() && writes.get(&item.name) == Some(&(true, false)))
        .map(|item| item.name.clone())
        .collect();
    let mut lower = Lower {
        names: source
            .variables
            .iter()
            .flat_map(|d| d.items.iter().map(|i| i.name.clone()))
            .chain(
                source
                    .digital_variables
                    .iter()
                    .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
            )
            .chain(
                source
                    .digital_nets
                    .iter()
                    .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
            )
            .chain(source.ports.iter().map(|p| p.name.clone()))
            .chain(
                source
                    .parameters
                    .iter()
                    .chain(&source.localparams)
                    .map(|p| p.name.clone()),
            )
            .chain(source.nets.iter().flat_map(|net| net.names.iter().cloned()))
            .chain(source.branches.iter().map(|branch| branch.name.clone()))
            .collect(),
        bindings: Vec::new(),
        functions: Vec::new(),
        variable_events: BTreeMap::new(),
        eligible,
    };
    lower.names.extend(analog_local_names);
    for process in &mut module.digital_processes {
        lower.statement(&mut process.body, &BTreeSet::new())?;
    }
    // A declaration initializer on a net is a continuous assignment too.
    for value in module
        .continuous_assigns
        .iter()
        .map(|assign| &assign.value)
        .chain(
            module
                .digital_nets
                .iter()
                .flat_map(|decl| decl.items.iter().filter_map(|item| item.init.as_ref())),
        )
    {
        lower.continuous_reads(value);
    }
    if lower.bindings.is_empty() {
        return Ok(LoweredAnalogEvents {
            module,
            bindings: Vec::new(),
            event_assigned_variables: lower.eligible.into_iter().collect(),
            immutable_variables,
        });
    }
    if let Some(block) = &mut module.analog_block {
        for statement in &mut block.statements {
            instrument(statement, &lower.variable_events, &BTreeSet::new());
        }
    }
    let block = module.analog_block.get_or_insert(AnalogBlock {
        statements: Vec::new(),
        span: module.span,
    });
    for (event, binding) in lower.functions {
        block
            .statements
            .push(AnalogStatement::EventControl(EventControlStmt {
                span: binding.span,
                event,
                statement: Box::new(increment(&binding.variable, binding.span)),
            }));
    }
    for binding in &lower.bindings {
        let span = binding.span;
        module.variables.push(VariableDecl {
            var_type: VarType::Integer,
            span,
            items: vec![VariableItem {
                name: binding.variable.clone(),
                dimensions: Vec::new(),
                init: Some(number(0, span)),
                span,
            }],
        });
        module.digital_variables.push(DigitalVariableDecl {
            kind: DigitalVariableKind::Reg,
            signedness: Signedness::Unsigned,
            range: Some(VectorRange {
                msb: number(31, span),
                lsb: number(0, span),
                span,
            }),
            items: vec![DigitalDeclItem {
                name: binding.signal.clone(),
                dimensions: Vec::new(),
                init: Some(number(0, span)),
                span,
            }],
            span,
        });
    }
    Ok(LoweredAnalogEvents {
        module,
        bindings: lower.bindings,
        event_assigned_variables: lower.eligible.into_iter().collect(),
        immutable_variables,
    })
}

struct Lower {
    names: BTreeSet<SmolStr>,
    bindings: Vec<AnalogEventBinding>,
    functions: Vec<(EventExpr, AnalogEventBinding)>,
    variable_events: BTreeMap<SmolStr, AnalogEventBinding>,
    eligible: BTreeSet<SmolStr>,
}
impl Lower {
    fn binding(&mut self, span: Span, source_variable: Option<SmolStr>) -> AnalogEventBinding {
        let mut index = self.bindings.len();
        loop {
            let variable: SmolStr = format!("$rspice$analog_event${index}$count").into();
            let signal: SmolStr = format!("$rspice$analog_event${index}$signal").into();
            if !self.names.contains(&variable) && !self.names.contains(&signal) {
                self.names.insert(variable.clone());
                self.names.insert(signal.clone());
                let binding = AnalogEventBinding {
                    source_variable,
                    variable,
                    signal,
                    span,
                };
                self.bindings.push(binding.clone());
                return binding;
            }
            index += 1;
        }
    }
    fn variable_binding(&mut self, name: &SmolStr, span: Span) -> AnalogEventBinding {
        if let Some(binding) = self.variable_events.get(name) {
            return binding.clone();
        }
        let binding = self.binding(span, Some(name.clone()));
        self.variable_events.insert(name.clone(), binding.clone());
        binding
    }
    fn continuous_reads(&mut self, expression: &Expression) {
        super::flow_probes::visit_expression(expression, &mut |expression| {
            if let Expression::Identifier(id) = expression {
                if self.eligible.contains(&id.name) {
                    self.variable_binding(&id.name, id.span);
                }
            }
        });
    }
    fn timing(
        &mut self,
        timing: &mut TimingControl,
        locals: &BTreeSet<SmolStr>,
    ) -> CompileResult<()> {
        let TimingControl::Event(control) = timing else {
            return Ok(());
        };
        let Sensitivity::Explicit(terms) = &mut control.sensitivity else {
            return Ok(());
        };
        for term in terms {
            let binding = if let Some(event) = event_function(&term.signal)? {
                if term.edge.is_some() {
                    return invalid(
                        "an analog event function cannot have a digital edge qualifier",
                        term.span,
                    );
                }
                let mut local = None;
                super::flow_probes::visit_expression(&term.signal, &mut |e| {
                    if let Expression::Identifier(id) = e {
                        if locals.contains(&id.name) {
                            local = Some(id.name.clone());
                        }
                    }
                });
                if let Some(name) = local {
                    return unsupported(
                        format!(
                            "analog event operand uses process-local '{name}', which requires an analog subscription storage binding"
                        ),
                        term.span,
                    );
                }
                let binding = self.binding(term.span, None);
                self.functions.push((event, binding.clone()));
                Some(binding)
            } else if let Expression::Identifier(id) = &term.signal {
                if self.eligible.contains(&id.name) && !locals.contains(&id.name) {
                    if term.edge.is_some() {
                        return invalid(
                            "an analog event-assigned variable uses assignment events without posedge/negedge",
                            term.span,
                        );
                    }
                    Some(self.variable_binding(&id.name, term.span))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(binding) = binding {
                term.signal = identifier(&binding.signal, term.span);
            }
        }
        Ok(())
    }
    fn statement(
        &mut self,
        statement: &mut DigitalStatement,
        locals: &BTreeSet<SmolStr>,
    ) -> CompileResult<()> {
        match statement {
            DigitalStatement::Block(block) => {
                let mut locals = locals.clone();
                locals.extend(
                    block
                        .variables
                        .iter()
                        .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
                );
                locals.extend(
                    block
                        .digital_variables
                        .iter()
                        .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
                );
                self.names.extend(locals.iter().cloned());
                for statement in &mut block.statements {
                    self.statement(statement, &locals)?;
                }
            }
            DigitalStatement::Timing(statement) => {
                self.timing(&mut statement.control, locals)?;
                if let Some(body) = &mut statement.statement {
                    self.statement(body, locals)?;
                }
            }
            DigitalStatement::BlockingAssign(assign)
            | DigitalStatement::NonblockingAssign(assign) => {
                if let Some(timing) = &mut assign.timing {
                    self.timing(timing, locals)?;
                }
            }
            DigitalStatement::Conditional(statement) => {
                self.statement(&mut statement.then_branch, locals)?;
                if let Some(body) = &mut statement.else_branch {
                    self.statement(body, locals)?;
                }
            }
            DigitalStatement::Case(statement) => {
                for item in &mut statement.items {
                    self.statement(&mut item.statement, locals)?;
                }
                if let Some(body) = &mut statement.default {
                    self.statement(body, locals)?;
                }
            }
            DigitalStatement::For(statement) => {
                if let Some(timing) = &mut statement.init.timing {
                    self.timing(timing, locals)?;
                }
                if let Some(timing) = &mut statement.update.timing {
                    self.timing(timing, locals)?;
                }
                self.statement(&mut statement.body, locals)?;
            }
            DigitalStatement::While(statement) => self.statement(&mut statement.body, locals)?,
            DigitalStatement::Repeat(statement) => self.statement(&mut statement.body, locals)?,
            DigitalStatement::Forever(statement) => self.statement(&mut statement.body, locals)?,
            DigitalStatement::Null(_) => {}
        }
        Ok(())
    }
}

fn event_function(expression: &Expression) -> CompileResult<Option<EventExpr>> {
    let Expression::Call(call) = expression else {
        return Ok(None);
    };
    let (name, args, span) = (call.name.as_str(), &call.args, call.span);
    if !matches!(name, "cross" | "above" | "timer") {
        return Ok(None);
    }
    let argument = |index: usize| {
        args.get(index)
            .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
            .cloned()
            .map(Box::new)
    };
    let max = if name == "cross" { 5 } else { 4 };
    if args.len() > max || argument(0).is_none() {
        return invalid(
            format!("invalid argument count or missing expression for {name} event"),
            span,
        );
    }
    Ok(Some(match name {
        "cross" => {
            if ((argument(2).is_some() || argument(3).is_some()) && argument(1).is_none())
                || (argument(3).is_some() && argument(2).is_none())
            {
                return invalid(
                    "cross tolerances require direction and expr_tol requires time_tol",
                    span,
                );
            }
            EventExpr::Cross {
                signal: *argument(0).unwrap(),
                direction: argument(1),
                time_tol: argument(2),
                expr_tol: argument(3),
                enable: argument(4),
                span,
            }
        }
        "above" => {
            if argument(2).is_some() && argument(1).is_none() {
                return invalid("above expr_tol requires time_tol", span);
            }
            EventExpr::Above {
                signal: *argument(0).unwrap(),
                time_tol: argument(1),
                expr_tol: argument(2),
                enable: argument(3),
                span,
            }
        }
        _ => EventExpr::Timer {
            start: *argument(0).unwrap(),
            period: argument(1),
            time_tol: argument(2),
            enable: argument(3),
            span,
        },
    }))
}

fn collect_writes(
    statement: &AnalogStatement,
    event: bool,
    shadowed: &BTreeSet<SmolStr>,
    writes: &mut BTreeMap<SmolStr, (bool, bool)>,
    reserved: &mut BTreeSet<SmolStr>,
) {
    match statement {
        AnalogStatement::Assignment(assign) => {
            if shadowed.contains(assign.target_name()) {
                return;
            }
            let entry = writes.entry(assign.target_name().clone()).or_default();
            if event {
                entry.0 = true;
            } else {
                entry.1 = true;
            }
        }
        AnalogStatement::EventControl(control) => {
            collect_writes(&control.statement, true, shadowed, writes, reserved)
        }
        AnalogStatement::Block(block) => {
            let mut shadowed = shadowed.clone();
            shadowed.extend(
                block
                    .variables
                    .iter()
                    .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
            );
            reserved.extend(shadowed.iter().cloned());
            for statement in &block.statements {
                collect_writes(statement, event, &shadowed, writes, reserved);
            }
        }
        AnalogStatement::Conditional(control) => {
            collect_writes(&control.then_branch, event, shadowed, writes, reserved);
            if let Some(statement) = &control.else_branch {
                collect_writes(statement, event, shadowed, writes, reserved);
            }
        }
        AnalogStatement::Case(control) => {
            for item in &control.items {
                collect_writes(&item.statement, event, shadowed, writes, reserved);
            }
            if let Some(statement) = &control.default {
                collect_writes(statement, event, shadowed, writes, reserved);
            }
        }
        AnalogStatement::For(control) => {
            for name in [&control.var, control.update.target_name()] {
                if !shadowed.contains(name) {
                    let entry = writes.entry(name.clone()).or_default();
                    if event {
                        entry.0 = true;
                    } else {
                        entry.1 = true;
                    }
                }
            }
            collect_writes(&control.body, event, shadowed, writes, reserved);
        }
        AnalogStatement::While(control) => {
            collect_writes(&control.body, event, shadowed, writes, reserved)
        }
        AnalogStatement::Repeat(control) => {
            collect_writes(&control.body, event, shadowed, writes, reserved)
        }
        _ => {}
    }
}
fn instrument(
    statement: &mut AnalogStatement,
    bindings: &BTreeMap<SmolStr, AnalogEventBinding>,
    shadowed: &BTreeSet<SmolStr>,
) {
    match statement {
        AnalogStatement::Assignment(assign) => {
            if !shadowed.contains(assign.target_name()) {
                if let Some(binding) = bindings.get(assign.target_name()) {
                    let span = assign.span;
                    *statement = AnalogStatement::Block(BlockStmt {
                        name: None,
                        variables: Vec::new(),
                        statements: vec![statement.clone(), increment(&binding.variable, span)],
                        span,
                    });
                }
            }
        }
        AnalogStatement::EventControl(control) => {
            instrument(&mut control.statement, bindings, shadowed)
        }
        AnalogStatement::Block(block) => {
            let mut shadowed = shadowed.clone();
            shadowed.extend(
                block
                    .variables
                    .iter()
                    .flat_map(|d| d.items.iter().map(|i| i.name.clone())),
            );
            for statement in &mut block.statements {
                instrument(statement, bindings, &shadowed);
            }
        }
        AnalogStatement::Conditional(control) => {
            instrument(&mut control.then_branch, bindings, shadowed);
            if let Some(statement) = &mut control.else_branch {
                instrument(statement, bindings, shadowed);
            }
        }
        AnalogStatement::Case(control) => {
            for item in &mut control.items {
                instrument(&mut item.statement, bindings, shadowed);
            }
            if let Some(statement) = &mut control.default {
                instrument(statement, bindings, shadowed);
            }
        }
        AnalogStatement::For(control) => {
            instrument(&mut control.body, bindings, shadowed);
            // Preserve the for loop and its bounded-loop lowering. The private
            // counters are observed after this analog evaluation completes.
            if let Some(binding) = bindings
                .get(control.update.target_name())
                .filter(|_| !shadowed.contains(control.update.target_name()))
            {
                control.body = Box::new(AnalogStatement::Block(BlockStmt {
                    name: None,
                    variables: Vec::new(),
                    span: control.span,
                    statements: vec![
                        *control.body.clone(),
                        increment(&binding.variable, control.update.span),
                    ],
                }));
            }
            if let Some(binding) = bindings
                .get(&control.var)
                .filter(|_| !shadowed.contains(&control.var))
            {
                let span = control.span;
                *statement = AnalogStatement::Block(BlockStmt {
                    name: None,
                    variables: Vec::new(),
                    span,
                    statements: vec![increment(&binding.variable, span), statement.clone()],
                });
            }
        }
        AnalogStatement::While(control) => instrument(&mut control.body, bindings, shadowed),
        AnalogStatement::Repeat(control) => instrument(&mut control.body, bindings, shadowed),
        _ => {}
    }
}
fn increment(name: &str, span: Span) -> AnalogStatement {
    AnalogStatement::Assignment(AssignmentStmt {
        target: LValue::Variable {
            name: name.into(),
            span,
        },
        span,
        value: Expression::Conditional(ConditionalExpr {
            condition: Box::new(Expression::Binary(BinaryExpr {
                op: BinaryOp::Eq,
                left: Box::new(identifier(name, span)),
                right: Box::new(number(i32::MAX, span)),
                span,
            })),
            then_expr: Box::new(number(0, span)),
            else_expr: Box::new(Expression::Binary(BinaryExpr {
                op: BinaryOp::Add,
                left: Box::new(identifier(name, span)),
                right: Box::new(number(1, span)),
                span,
            })),
            span,
        }),
    })
}
fn number(value: i32, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
    })
}
fn identifier(name: &str, span: Span) -> Expression {
    Expression::Identifier(Identifier {
        name: name.into(),
        span,
    })
}
fn invalid<T>(message: impl Into<String>, span: Span) -> CompileResult<T> {
    Err(SemanticError::new(SemanticErrorKind::InvalidExpression(message.into()), span).into())
}
fn unsupported<T>(message: impl Into<String>, span: Span) -> CompileResult<T> {
    Err(SemanticError::new(SemanticErrorKind::UnsupportedFeature(message.into()), span).into())
}

/// Function output/inout arguments become explicit writes during semantic
/// lowering. They must not leave an apparently unwritten source variable marked
/// immutable merely because its name was absent from an authored assignment LHS.
pub(super) fn filter_immutable_variables(module: &mut super::AnalyzedModule) {
    let mut pending = vec![module.statements.as_slice()];
    let mut written = BTreeSet::new();
    while let Some(statements) = pending.pop() {
        for statement in statements {
            match statement {
                super::AnalyzedStatement::Assignment(assignment) => {
                    written.insert(assignment.target.clone());
                }
                super::AnalyzedStatement::Loop(loop_) => pending.push(&loop_.body),
                super::AnalyzedStatement::Initialization { .. }
                | super::AnalyzedStatement::Task(_) => {}
            }
        }
    }
    module
        .digital
        .immutable_analog_variables
        .retain(|name| !written.contains(name));
}
