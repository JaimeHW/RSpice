//! Lower cross-domain event subscriptions to retained analog occurrence counters.
//! Analog operators keep their ordinary state, root detection and rollback. A
//! private digital signal carries occurrence counts, independently of data values.
use std::collections::{BTreeMap, BTreeSet, HashSet};
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
    pub candidates: BTreeSet<SmolStr>,
}

pub(super) fn lower(source: &Module) -> CompileResult<LoweredAnalogEvents> {
    let mut module = source.clone();
    // Numeric declarations acquire digital ownership from their procedural
    // writers. Analog ownership/event classification is completed after function
    // copy-outs and all other assignments have been semantically lowered.
    let mut digital_writes = HashSet::new();
    for process in &source.digital_processes {
        super::digital_walk::collect_module_writes(&process.body, &mut digital_writes);
    }
    let candidates: BTreeSet<_> = source
        .variables
        .iter()
        .flat_map(|decl| &decl.items)
        .filter(|item| item.dimensions.is_empty() && !digital_writes.contains(&item.name))
        .map(|item| item.name.clone())
        .collect();
    let mut analog_local_names = BTreeSet::new();
    for block in source.analog_block.iter().chain(&source.analog_initial) {
        reserve_locals(&block.statements, &mut analog_local_names);
    }
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
        candidates,
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
            candidates: lower.candidates,
        });
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
        candidates: lower.candidates,
    })
}

struct Lower {
    names: BTreeSet<SmolStr>,
    bindings: Vec<AnalogEventBinding>,
    functions: Vec<(EventExpr, AnalogEventBinding)>,
    variable_events: BTreeMap<SmolStr, AnalogEventBinding>,
    candidates: BTreeSet<SmolStr>,
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
                if self.candidates.contains(&id.name) {
                    self.variable_binding(&id.name, id.span);
                }
            }
        });
    }
    fn implicit_reads(&mut self, statement: &DigitalStatement, locals: &BTreeSet<SmolStr>) {
        super::digital_walk::visit_sensitivity_roots(
            statement,
            locals,
            &mut |expression, locals| {
                super::flow_probes::visit_expression(expression, &mut |expression| {
                    if let Expression::Identifier(id) = expression {
                        if self.candidates.contains(&id.name) && !locals.contains(&id.name) {
                            self.variable_binding(&id.name, id.span);
                        }
                    }
                });
            },
        );
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
                if self.candidates.contains(&id.name) && !locals.contains(&id.name) {
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
            DigitalStatement::Timing(timing) if implicit(&timing.control) => {
                if let Some(body) = &timing.statement {
                    self.implicit_reads(body, locals);
                }
            }
            DigitalStatement::BlockingAssign(assign)
            | DigitalStatement::NonblockingAssign(assign)
                if assign.timing.as_ref().is_some_and(implicit) =>
            {
                self.implicit_reads(statement, locals)
            }
            DigitalStatement::For(loop_) => {
                for assign in [&loop_.init, &loop_.update] {
                    if assign.timing.as_ref().is_some_and(implicit) {
                        self.implicit_reads(
                            &DigitalStatement::BlockingAssign((**assign).clone()),
                            locals,
                        );
                    }
                }
            }
            _ => {}
        }
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

fn implicit(timing: &TimingControl) -> bool {
    matches!(timing, TimingControl::Event(event) if matches!(event.sensitivity, Sensitivity::Implicit))
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

fn reserve_locals(statements: &[AnalogStatement], reserved: &mut BTreeSet<SmolStr>) {
    let mut pending: Vec<_> = statements.iter().collect();
    while let Some(statement) = pending.pop() {
        match statement {
            AnalogStatement::Block(block) => {
                reserved.extend(
                    block
                        .variables
                        .iter()
                        .flat_map(|decl| decl.items.iter().map(|item| item.name.clone())),
                );
                pending.extend(&block.statements);
            }
            AnalogStatement::EventControl(control) => pending.push(&control.statement),
            AnalogStatement::Conditional(control) => {
                pending.push(&control.then_branch);
                if let Some(body) = &control.else_branch {
                    pending.push(body);
                }
            }
            AnalogStatement::Case(control) => {
                pending.extend(control.items.iter().map(|item| item.statement.as_ref()));
                if let Some(body) = &control.default {
                    pending.push(body);
                }
            }
            AnalogStatement::For(control) => pending.push(&control.body),
            AnalogStatement::While(control) => pending.push(&control.body),
            AnalogStatement::Repeat(control) => pending.push(&control.body),
            _ => {}
        }
    }
}

/// Track resolved storage writes, including inlined function output/inout
/// copy-outs. Counter assignments pass through ordinary semantic lowering and
/// inherit the exact branch/event guard of the write that caused them.
#[derive(Default)]
pub(super) struct AssignmentEvents {
    pub event_depth: usize,
    writes: BTreeMap<SmolStr, Writes>,
    counters: BTreeMap<SmolStr, SmolStr>,
}
#[derive(Default)]
struct Writes {
    initial: bool,
    event: bool,
    continuous: bool,
}
impl AssignmentEvents {
    pub fn new(lowered: Option<&LoweredAnalogEvents>) -> Self {
        let Some(lowered) = lowered else {
            return Self::default();
        };
        Self {
            event_depth: 0,
            writes: lowered
                .candidates
                .iter()
                .cloned()
                .map(|name| (name, Writes::default()))
                .collect(),
            counters: lowered
                .bindings
                .iter()
                .filter_map(|binding| {
                    Some((binding.source_variable.clone()?, binding.variable.clone()))
                })
                .collect(),
        }
    }
    pub fn record(&mut self, name: &SmolStr, initial: bool, span: Span) -> Option<AnalogStatement> {
        let writes = self.writes.get_mut(name)?;
        if initial {
            writes.initial = true;
        } else if self.event_depth > 0 {
            writes.event = true;
            return self
                .counters
                .get(name)
                .map(|counter| increment(counter, span));
        } else {
            writes.continuous = true;
        }
        None
    }
    pub fn finish(&self, module: &mut super::AnalyzedModule) -> CompileResult<()> {
        let event_assigned =
            |writes: &Writes| writes.event && !writes.continuous && !writes.initial;
        let immutable = |writes: &Writes| !writes.event && !writes.continuous;
        for binding in &module.digital.analog_events {
            if let Some(name) = &binding.source_variable {
                let writes = &self.writes[name];
                if !event_assigned(writes) && !immutable(writes) {
                    return unsupported(
                        format!(
                            "analog variable `{name}` is not assigned exclusively in analog event statements and cannot provide assignment-event subscriptions"
                        ),
                        binding.span,
                    );
                }
            }
        }
        module.digital.event_assigned_variables = self
            .writes
            .iter()
            .filter(|(_, writes)| event_assigned(writes))
            .map(|(name, _)| name.clone())
            .collect();
        module.digital.immutable_analog_variables = self
            .writes
            .iter()
            .filter(|(_, writes)| immutable(writes))
            .map(|(name, _)| name.clone())
            .collect();
        Ok(())
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
