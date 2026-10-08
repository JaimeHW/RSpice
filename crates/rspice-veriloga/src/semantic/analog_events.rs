//! Lower cross-domain event subscriptions to retained analog occurrence counters.
//! Analog operators keep their ordinary state, root detection and rollback. A
//! private digital signal carries occurrence counts, independently of data values.
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use crate::ast::*;
use crate::error::{CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct AnalogEventBinding {
    /// Authored event-assigned variable, absent for an event-function site.
    pub source_variable: Option<SmolStr>,
    pub array: bool,
    /// Normal analog evaluation slots for expr, delta, time_tol, expr_tol, enable.
    /// An interpolating observer owns the occurrence signal instead of an analog counter.
    pub observation: Option<[SmolStr; 5]>,
    pub variable: SmolStr,
    pub signal: SmolStr,
    pub span: Span,
}

pub(super) struct LoweredAnalogEvents {
    pub module: Module,
    pub bindings: Vec<AnalogEventBinding>,
    pub candidates: BTreeSet<SmolStr>,
    pub local_inputs: Vec<HashMap<Span, SmolStr>>,
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
        .filter(|item| !digital_writes.contains(&item.name))
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
        local_scopes: Vec::new(),
        local_inputs: HashMap::new(),
        local_declarations: Vec::new(),
        bindings: Vec::new(),
        functions: Vec::new(),
        observers: Vec::new(),
        variable_events: BTreeMap::new(),
        arrays: source
            .variables
            .iter()
            .flat_map(|decl| &decl.items)
            .filter(|item| !item.dimensions.is_empty())
            .map(|item| (item.name.clone(), item.dimensions.clone()))
            .collect(),
        candidates,
    };
    lower.names.extend(analog_local_names);
    let mut local_inputs = Vec::new();
    for process in &mut module.digital_processes {
        lower.statement(&mut process.body, &BTreeSet::new())?;
        local_inputs.push(std::mem::take(&mut lower.local_inputs));
    }
    module
        .digital_variables
        .append(&mut lower.local_declarations);
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
            local_inputs,
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
    for (operands, binding) in lower.observers {
        for (value, name) in operands.into_iter().zip(binding.observation.unwrap()) {
            block
                .statements
                .push(AnalogStatement::Assignment(AssignmentStmt {
                    target: LValue::Variable {
                        name: name.clone(),
                        span: binding.span,
                    },
                    value,
                    span: binding.span,
                }));
            module.variables.push(VariableDecl {
                var_type: VarType::Real,
                items: vec![VariableItem {
                    name,
                    dimensions: Vec::new(),
                    init: None,
                    span: binding.span,
                }],
                span: binding.span,
            });
        }
    }
    for binding in &lower.bindings {
        let span = binding.span;
        let dimensions = binding
            .source_variable
            .as_ref()
            .and_then(|name| lower.arrays.get(name))
            .cloned()
            .unwrap_or_default();
        let init = dimensions.is_empty().then(|| number(0, span));
        if binding.observation.is_none() {
            module.variables.push(VariableDecl {
                var_type: VarType::Integer,
                span,
                items: vec![VariableItem {
                    name: binding.variable.clone(),
                    dimensions: dimensions.clone(),
                    init: init.clone(),
                    span,
                }],
            });
        }
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
                dimensions,
                init,
                span,
            }],
            span,
        });
    }
    Ok(LoweredAnalogEvents {
        module,
        bindings: lower.bindings,
        candidates: lower.candidates,
        local_inputs,
    })
}

#[derive(Clone)]
struct LocalDeclaration {
    kind: Option<DigitalVariableKind>,
    signedness: Signedness,
    range: Option<VectorRange>,
    dimensions: Vec<ArrayDimension>,
    span: Span,
}

struct Lower {
    local_scopes: Vec<HashMap<SmolStr, LocalDeclaration>>,
    local_inputs: HashMap<Span, SmolStr>,
    local_declarations: Vec<DigitalVariableDecl>,
    names: BTreeSet<SmolStr>,
    bindings: Vec<AnalogEventBinding>,
    functions: Vec<(EventExpr, AnalogEventBinding)>,
    observers: Vec<([Expression; 5], AnalogEventBinding)>,
    variable_events: BTreeMap<SmolStr, AnalogEventBinding>,
    candidates: BTreeSet<SmolStr>,
    arrays: BTreeMap<SmolStr, Vec<ArrayDimension>>,
}
impl Lower {
    fn binding(&mut self, span: Span, source_variable: Option<SmolStr>) -> AnalogEventBinding {
        let mut index = self.bindings.len();
        loop {
            let variable: SmolStr = format!("$rspice$analog_event${index}$count").into();
            let signal: SmolStr = format!("$rspice$analog_event${index}$signal").into();
            if !self.names.contains(&variable)
                && !self.names.contains(&signal)
                && (0..5).all(|index| {
                    !self
                        .names
                        .contains(&SmolStr::from(format!("{variable}$operand{index}")))
                })
            {
                self.names.insert(variable.clone());
                self.names.insert(signal.clone());
                let binding = AnalogEventBinding {
                    observation: None,
                    array: source_variable
                        .as_ref()
                        .is_some_and(|name| self.arrays.contains_key(name)),
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
            let name = match expression {
                Expression::Identifier(id) => &id.name,
                Expression::ArrayAccess(access) => &access.array,
                Expression::Digital(DigitalExpr::ArraySelect(access)) => &access.name,
                _ => return,
            };
            if self.candidates.contains(name) {
                self.variable_binding(name, expression.span());
            }
        });
    }
    fn implicit_reads(&mut self, statement: &DigitalStatement, locals: &BTreeSet<SmolStr>) {
        super::digital_walk::visit_sensitivity_roots(
            statement,
            locals,
            &mut |expression, locals| {
                super::flow_probes::visit_expression(expression, &mut |expression| {
                    let name = match expression {
                        Expression::Identifier(id) => &id.name,
                        Expression::ArrayAccess(access) => &access.array,
                        Expression::Digital(DigitalExpr::ArraySelect(access)) => &access.name,
                        _ => return,
                    };
                    if self.candidates.contains(name) && !locals.contains(name) {
                        self.variable_binding(name, expression.span());
                    }
                });
            },
        );
    }
    /// Bind each lexical declaration to one digital storage bank. Only the
    /// extracted analog expression uses this private name; source locals keep
    /// their scope and ordinary initialization, writes and suspension semantics.
    fn bind_local_operands(&mut self, expression: &mut Expression) -> CompileResult<()> {
        let mut pending = vec![expression];
        while let Some(expression) = pending.pop() {
            if let Expression::BranchAccess(access) = expression {
                let names: Vec<_> = match access {
                    BranchAccess::Nodes { pos, neg, .. } => {
                        std::iter::once(&*pos).chain(neg.iter()).collect()
                    }
                    BranchAccess::Branch { name, .. } => vec![&*name],
                };
                if let Some(name) = names.into_iter().find(|name| {
                    self.local_scopes
                        .iter()
                        .any(|scope| scope.contains_key(*name))
                }) {
                    return invalid(
                        format!(
                            "analog access names process-local storage `{name}`, not a continuous net or branch"
                        ),
                        access.span(),
                    );
                }
            }
            let name = match expression {
                Expression::Identifier(id) => Some(&mut id.name),
                Expression::ArrayAccess(access) => Some(&mut access.array),
                Expression::Digital(DigitalExpr::PartSelect(select)) => Some(&mut select.name),
                Expression::Digital(DigitalExpr::ArraySelect(select)) => Some(&mut select.name),
                _ => None,
            };
            if let Some(name) = name
                && let Some(declaration) = self
                    .local_scopes
                    .iter()
                    .rev()
                    .find_map(|scope| scope.get(name))
                    .cloned()
            {
                if let Some(signal) = self.local_inputs.get(&declaration.span) {
                    *name = signal.clone();
                } else {
                    let Some(kind) = declaration.kind else {
                        return invalid(
                            "analog event operands require numeric local storage",
                            declaration.span,
                        );
                    };
                    let mut index = self.local_declarations.len();
                    let signal: SmolStr = loop {
                        let name: SmolStr = format!("$rspice$analog_local${index}").into();
                        if self.names.insert(name.clone()) {
                            break name;
                        }
                        index += 1;
                    };
                    self.local_declarations.push(DigitalVariableDecl {
                        kind,
                        signedness: declaration.signedness,
                        range: declaration.range,
                        items: vec![DigitalDeclItem {
                            name: signal.clone(),
                            dimensions: declaration.dimensions,
                            init: None,
                            span: declaration.span,
                        }],
                        span: declaration.span,
                    });
                    self.local_inputs.insert(declaration.span, signal.clone());
                    *name = signal;
                }
            }
            super::flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        Ok(())
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
            if matches!(&term.signal, Expression::Call(call)
                if matches!(call.name.as_str(), "absdelta" | "cross" | "above" | "timer"))
            {
                self.bind_local_operands(&mut term.signal)?;
            }
            if let Expression::Call(call) = &term.signal
                && call.name == "absdelta"
            {
                if term.edge.is_some() {
                    return invalid("absdelta cannot have a digital edge qualifier", term.span);
                }
                if !(2..=5).contains(&call.args.len())
                    || call.args[..2]
                        .iter()
                        .any(|arg| matches!(arg, Expression::NullArgument(_)))
                {
                    return invalid(
                        "absdelta requires expr and delta, followed by at most three optional operands",
                        term.span,
                    );
                }
                let operands = std::array::from_fn(|index| {
                    call.args
                        .get(index)
                        .filter(|arg| !matches!(arg, Expression::NullArgument(_)))
                        .cloned()
                        .unwrap_or_else(|| number(i32::from(index == 4), term.span))
                });
                let mut binding = self.binding(term.span, None);
                binding.observation = Some(std::array::from_fn(|index| {
                    format!("{}$operand{index}", binding.variable).into()
                }));
                *self.bindings.last_mut().expect("new observer binding") = binding.clone();
                self.observers.push((operands, binding.clone()));
                term.signal = identifier(&binding.signal, term.span);
                continue;
            }
            if let Expression::Digital(DigitalExpr::ArraySelect(access)) = &mut term.signal
                && self.candidates.contains(&access.name)
                && !locals.contains(&access.name)
            {
                let rank = self.arrays.get(&access.name).map_or(0, Vec::len);
                if term.edge.is_some()
                    || !access
                        .split(rank)
                        .is_some_and(|(_, packed)| packed.is_none())
                {
                    return invalid(
                        "analog array assignment events require a complete unpacked element without a packed selection or edge qualifier",
                        term.span,
                    );
                }
                access.name = self.variable_binding(&access.name, term.span).signal;
                continue;
            }
            if let Expression::ArrayAccess(access) = &mut term.signal {
                if self.candidates.contains(&access.array) && !locals.contains(&access.array) {
                    if term.edge.is_some() || access.packed.is_some() {
                        return invalid(
                            "analog array assignment events require an unpacked element without an edge qualifier",
                            term.span,
                        );
                    }
                    let binding = self.variable_binding(&access.array, term.span);
                    access.array = binding.signal;
                    continue;
                }
            }
            let binding = if let Some(event) = event_function(&term.signal)? {
                if term.edge.is_some() {
                    return invalid(
                        "an analog event function cannot have a digital edge qualifier",
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
                let mut scope = HashMap::new();
                for declaration in &block.variables {
                    for item in &declaration.items {
                        scope.insert(
                            item.name.clone(),
                            LocalDeclaration {
                                kind: match declaration.var_type {
                                    VarType::Real => Some(DigitalVariableKind::Real),
                                    VarType::Integer => Some(DigitalVariableKind::Integer),
                                    VarType::String => None,
                                },
                                signedness: Signedness::Signed,
                                range: matches!(declaration.var_type, VarType::Integer).then(
                                    || VectorRange {
                                        msb: number(31, item.span),
                                        lsb: number(0, item.span),
                                        span: item.span,
                                    },
                                ),
                                dimensions: item.dimensions.clone(),
                                span: item.span,
                            },
                        );
                    }
                }
                for declaration in &block.digital_variables {
                    for item in &declaration.items {
                        scope.insert(
                            item.name.clone(),
                            LocalDeclaration {
                                kind: Some(declaration.kind),
                                signedness: declaration.signedness,
                                range: declaration.range.clone(),
                                dimensions: item.dimensions.clone(),
                                span: item.span,
                            },
                        );
                    }
                }
                self.local_scopes.push(scope);
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
                self.local_scopes.pop();
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
    arrays: BTreeMap<SmolStr, Vec<SmolStr>>,
    array_targets: BTreeMap<SmolStr, (SmolStr, i64)>,
}
#[derive(Default)]
struct Writes {
    initial: bool,
    event: bool,
    continuous: bool,
}
impl Writes {
    fn record(&mut self, initial: bool, event: bool) {
        if initial {
            self.initial = true;
        } else if event {
            self.event = true;
        } else {
            self.continuous = true;
        }
    }
}
impl AssignmentEvents {
    pub fn new(lowered: Option<&LoweredAnalogEvents>) -> Self {
        let Some(lowered) = lowered else {
            return Self::default();
        };
        Self {
            event_depth: 0,
            arrays: BTreeMap::new(),
            array_targets: BTreeMap::new(),
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
    pub fn register_arrays(
        &mut self,
        arrays: &std::collections::HashMap<SmolStr, super::AnalyzedArray>,
    ) {
        for (name, layout) in arrays {
            if self.writes.remove(name).is_none() {
                continue;
            }
            let dimensions = if layout.dimensions.is_empty() {
                vec![(
                    layout.lower,
                    layout
                        .lower
                        .checked_add(layout.len as i64 - 1)
                        .expect("validated array bounds"),
                )]
            } else {
                layout.dimensions.clone()
            };
            let shape = crate::array_index::UnpackedArrayLayout::new(&dimensions, 65_536)
                .expect("validated source shape");
            let mut cells = Vec::with_capacity(layout.len);
            for offset in 0..layout.len {
                let index = layout.lower + offset as i64;
                let element: SmolStr =
                    crate::array_index::element_name(name, &shape, offset).into();
                cells.push(element.clone());
                self.writes.insert(element.clone(), Writes::default());
                if let Some(counter) = self.counters.get(name) {
                    self.array_targets.insert(element, (counter.clone(), index));
                }
            }
            self.arrays.insert(name.clone(), cells);
        }
    }
    pub fn has_counter(&self, name: &SmolStr) -> bool {
        self.counters.contains_key(name)
    }
    pub fn record_indexed(
        &mut self,
        name: &SmolStr,
        index: Expression,
        initial: bool,
        span: Span,
    ) -> Option<AnalogStatement> {
        for element in self.arrays.get(name)? {
            self.writes
                .get_mut(element)
                .expect("registered array element")
                .record(initial, self.event_depth > 0);
        }
        if !initial && self.event_depth > 0 {
            self.counters
                .get(name)
                .map(|counter| increment_at(counter, Some(index), span))
        } else {
            None
        }
    }
    fn record_write(&mut self, name: &SmolStr, initial: bool) {
        let Some(writes) = self.writes.get_mut(name) else {
            return;
        };
        writes.record(initial, self.event_depth > 0);
    }
    pub fn record(&mut self, name: &SmolStr, initial: bool, span: Span) -> Option<AnalogStatement> {
        self.record_write(name, initial);
        if initial || self.event_depth == 0 {
            return None;
        }
        if let Some((counter, index)) = self.array_targets.get(name) {
            Some(increment_at(counter, Some(integer(*index, span)), span))
        } else {
            self.counters
                .get(name)
                .filter(|_| !self.arrays.contains_key(name))
                .map(|counter| increment(counter, span))
        }
    }
    pub fn finish(&self, module: &mut super::AnalyzedModule) -> CompileResult<()> {
        let event_assigned =
            |writes: &Writes| writes.event && !writes.continuous && !writes.initial;
        let immutable = |writes: &Writes| !writes.event && !writes.continuous;
        for binding in &module.digital.analog_events {
            if let Some(name) = &binding.source_variable {
                if self.arrays.contains_key(name) {
                    continue;
                }
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
        for (name, elements) in &self.arrays {
            let cells: Vec<_> = elements
                .iter()
                .map(|element| &self.writes[element])
                .collect();
            if cells.iter().all(|writes| immutable(writes)) {
                module.digital.immutable_analog_variables.push(name.clone());
            } else if cells
                .iter()
                .all(|writes| immutable(writes) || event_assigned(writes))
            {
                module.digital.event_assigned_variables.push(name.clone());
            }
        }
        Ok(())
    }
}

fn increment(name: &str, span: Span) -> AnalogStatement {
    increment_at(name, None, span)
}
fn increment_at(name: &str, index: Option<Expression>, span: Span) -> AnalogStatement {
    let (target, value) = if let Some(index) = index {
        (
            LValue::ArrayAccess {
                normalized: true,
                additional_indices: Vec::new(),
                name: name.into(),
                index: Box::new(index.clone()),
                span,
            },
            Expression::ArrayAccess(ArrayAccessExpr {
                normalized: true,
                array: name.into(),
                index: Box::new(index),
                packed: None,
                discrete_validity: None,
                span,
            }),
        )
    } else {
        (
            LValue::Variable {
                name: name.into(),
                span,
            },
            identifier(name, span),
        )
    };
    AnalogStatement::Assignment(AssignmentStmt {
        target,
        span,
        value: Expression::Conditional(ConditionalExpr {
            condition: Box::new(Expression::Binary(BinaryExpr {
                op: BinaryOp::Eq,
                left: Box::new(value.clone()),
                right: Box::new(number(i32::MAX, span)),
                span,
            })),
            then_expr: Box::new(number(0, span)),
            else_expr: Box::new(Expression::Binary(BinaryExpr {
                op: BinaryOp::Add,
                left: Box::new(value.clone()),
                right: Box::new(number(1, span)),
                span,
            })),
            span,
        }),
    })
}
fn integer(value: i64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
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
