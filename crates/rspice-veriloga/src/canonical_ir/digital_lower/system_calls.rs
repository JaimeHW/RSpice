//! System task and function calls (IEEE 1364-2005 section 17) as site-table
//! entries and call nodes.
//!
//! A call lowers to one node that names its site, plus whatever the call
//! needs around it: its evaluated operands before, and for each output
//! argument a [`CfgValueKind::DigitalCallOutput`] and an ordinary write after.
//! The site table entry carries everything that is not a runtime value — the
//! kind, string literals, empty arguments, the scope `%m` prints, the module
//! time unit `%t` scales from — so the host formats a call from the entry and
//! the operands alone.
//!
//! `$strobe` and `$monitor` evaluate their arguments in the monitor region,
//! not at the call (sections 17.1.2 and 17.1.3). Their arguments therefore
//! lower into a separate observer function, and the call itself only arms it.

use super::super::digital::{
    DigitalArgumentUse, DigitalObserver, DigitalScope, DigitalScopeId, DigitalScopeKind,
    DigitalSystemArgument, DigitalSystemCall, DigitalSystemCallId, DigitalSystemCallKind,
    DigitalValueClass,
};
use super::*;
use crate::ast::SystemFunction;
use std::sync::Arc;

/// One scope a call can be written in.
#[derive(Clone)]
pub(super) struct ScopeFrame {
    pub id: DigitalScopeId,
    pub path: SmolStr,
    pub module: SmolStr,
    pub time_scale: crate::time_scale::ModuleTimeScale,
}

/// The plan tables one process's lowering appends to. Identities are global:
/// each table's ids start where the plan's table stood when the process began,
/// so a lowered process's tables append to the plan unchanged.
#[derive(Default)]
pub(super) struct CallTables {
    pub scope_base: u32,
    pub site_base: u32,
    pub observer_base: u32,
    pub scopes: Vec<DigitalScope>,
    pub sites: Vec<DigitalSystemCall>,
    pub observers: Vec<DigitalObserver>,
    /// The scope being lowered, innermost last; the frame's module scope is at
    /// the bottom. Empty for a constant or a continuous assignment, where no
    /// call is allowed.
    pub stack: Vec<ScopeFrame>,
    /// Every module instance scope of the plan, by full path, for a call that
    /// names one (`$printtimescale(top.u1)`).
    pub module_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    /// `disable` lowering: the names some `disable` targets, the disabled
    /// named blocks around the statement being lowered, and the routine
    /// being lowered.
    pub threads: super::disable::ThreadTables,
}

/// The plan-level tables the processes' calls collect into.
#[derive(Default)]
pub(super) struct PlanCalls {
    pub scopes: Vec<DigitalScope>,
    pub sites: Vec<DigitalSystemCall>,
    pub observers: Vec<DigitalObserver>,
    pub module_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    /// Every name a `disable` of the design targets.
    pub disabled: Arc<std::collections::HashSet<SmolStr>>,
    /// The `disable`s that name a block outside their own, resolved once
    /// every scope exists.
    pub pending_disables: Vec<super::disable::PendingDisable>,
}

impl PlanCalls {
    /// A module instance scope, root first.
    pub fn add_module_scope(
        &mut self,
        parent: Option<DigitalScopeId>,
        path: SmolStr,
        module: SmolStr,
        time_scale: crate::time_scale::ModuleTimeScale,
    ) -> ScopeFrame {
        let id = DigitalScopeId(self.scopes.len() as u32);
        self.scopes.push(DigitalScope {
            id,
            parent,
            path: path.clone(),
            module: module.clone(),
            kind: DigitalScopeKind::Module,
            time_scale,
        });
        Arc::make_mut(&mut self.module_scopes).insert(path.clone(), id);
        ScopeFrame {
            id,
            path,
            module,
            time_scale,
        }
    }

    /// Tables for one process lowered in `scope`.
    pub fn tables_for(&self, scope: Option<&ScopeFrame>) -> CallTables {
        CallTables {
            scope_base: self.scopes.len() as u32,
            site_base: self.sites.len() as u32,
            observer_base: self.observers.len() as u32,
            stack: scope.cloned().into_iter().collect(),
            module_scopes: Arc::clone(&self.module_scopes),
            threads: super::disable::ThreadTables::new(Arc::clone(&self.disabled)),
            ..Default::default()
        }
    }

    /// Append what one process's lowering produced; its `disable`s that
    /// await resolution belong to `owner`.
    pub fn absorb(&mut self, tables: CallTables, owner: super::disable::DisableOwner) {
        debug_assert_eq!(tables.scope_base as usize, self.scopes.len());
        debug_assert_eq!(tables.site_base as usize, self.sites.len());
        debug_assert_eq!(tables.observer_base as usize, self.observers.len());
        self.scopes.extend(tables.scopes);
        self.sites.extend(tables.sites);
        self.observers.extend(tables.observers);
        self.pending_disables
            .extend(tables.threads.pending.into_iter().map(|mut pending| {
                pending.owner = Some(owner);
                pending
            }));
    }
}

/// The variable an output argument names, as an assignment target.
fn expression_as_target(argument: &Expression) -> Option<DigitalLValue> {
    Some(match argument {
        Expression::Identifier(identifier) => DigitalLValue::Identifier {
            name: identifier.name.clone(),
            span: identifier.span,
        },
        Expression::ArrayAccess(access) => DigitalLValue::BitSelect {
            name: access.array.clone(),
            index: access.index.clone(),
            span: access.span,
        },
        Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
            DigitalLValue::PartSelect {
                name: select.name.clone(),
                msb: select.msb.clone(),
                lsb: select.lsb.clone(),
                span: select.span,
            }
        }
        Expression::Digital(crate::ast::DigitalExpr::ArraySelect(select)) => {
            DigitalLValue::ArraySelect(select.clone())
        }
        Expression::Digital(crate::ast::DigitalExpr::IndexedPartSelect(select)) => {
            DigitalLValue::IndexedPartSelect(select.clone())
        }
        _ => return None,
    })
}

impl ProcessLowerer<'_> {
    /// Enter a named block's (IEEE 1364-2005 section 9.8.3), task's or
    /// function's scope, which `%m` inside it names.
    pub(super) fn enter_scope(
        &mut self,
        name: &SmolStr,
        kind: DigitalScopeKind,
    ) -> Option<DigitalScopeId> {
        let parent = self.calls.stack.last().cloned()?;
        let id = DigitalScopeId(self.calls.scope_base + self.calls.scopes.len() as u32);
        let path: SmolStr = format!("{}.{name}", parent.path).into();
        self.calls.scopes.push(DigitalScope {
            id,
            parent: Some(parent.id),
            path: path.clone(),
            module: parent.module.clone(),
            kind,
            time_scale: parent.time_scale,
        });
        self.calls.stack.push(ScopeFrame {
            id,
            path,
            module: parent.module,
            time_scale: parent.time_scale,
        });
        Some(id)
    }

    pub(super) fn leave_named_scope(&mut self) {
        self.calls.stack.pop();
    }

    /// Refuse a call where no scope is being lowered: a constant expression or
    /// a continuous assignment.
    fn call_context(&mut self, call: &SystemFunction) -> Option<ScopeFrame> {
        if let Some(scope) = self.calls.stack.last() {
            return Some(scope.clone());
        }
        let message = if self.constant_expression {
            format!(
                "`{}` is not a constant expression: a system call runs during simulation",
                call.name
            )
        } else {
            format!(
                "`{}` cannot be called in a continuous assignment, which re-evaluates only \
                 when an operand changes; call it in an `initial` or `always` process",
                call.name
            )
        };
        self.error(message, call.span);
        None
    }

    fn allocate_site(
        &mut self,
        kind: DigitalSystemCallKind,
        scope: &ScopeFrame,
        span: Span,
    ) -> (DigitalSystemCallId, usize) {
        let local = self.calls.sites.len();
        let id = DigitalSystemCallId(self.calls.site_base + local as u32);
        self.calls.sites.push(DigitalSystemCall {
            id,
            kind,
            arguments: Vec::new(),
            outputs: 0,
            result: None,
            scope: scope.id,
            time_scale: self.time_scale,
            level: None,
            observer: None,
            span: span.into(),
            origin: None,
        });
        (id, local)
    }

    /// A system task enable statement.
    pub(super) fn system_task_statement(
        &mut self,
        block: BlockId,
        task: &SystemFunction,
    ) -> BlockId {
        // IEEE 1364-2005 section 10.4.5 d): a constant function ignores its
        // system tasks.
        if self.constant_expression {
            return block;
        }
        let Some(kind) = DigitalSystemCallKind::from_name(&task.name) else {
            self.invariant(
                format!("system task `{}` reached lowering unrecognized", task.name),
                task.span,
            );
            return block;
        };
        if kind.is_observer() {
            self.observer_call(block, task, kind);
        } else {
            self.lower_call(block, task, kind, true);
        }
        block
    }

    /// A system function call in an expression, as its value.
    pub(super) fn system_function_value(
        &mut self,
        block: BlockId,
        call: &SystemFunction,
    ) -> ValueId {
        match DigitalSystemCallKind::from_name(&call.name) {
            Some(kind) if kind.is_function() => self
                .lower_call(block, call, kind, false)
                .unwrap_or_else(|| self.unknown(32)),
            _ => {
                self.invariant(
                    format!("`{}` reached lowering as a system function", call.name),
                    call.span,
                );
                self.unknown(32)
            }
        }
    }

    /// The constant finish level of `$finish(n)`, `$stop(n)` or `$fatal(n, ...)`
    /// (IEEE 1364-2005 section 17.4.1: 0, 1 or 2).
    fn finish_level(&mut self, argument: &Expression, call: &SystemFunction) -> u8 {
        match self.constant(argument) {
            Some(level @ 0..=2) => level as u8,
            Some(level) => {
                self.error(
                    format!(
                        "`{}`'s finish number is {level}; IEEE 1364-2005 section 17.4.1 \
                         defines 0, 1 and 2",
                        call.name
                    ),
                    argument.span(),
                );
                1
            }
            None => {
                self.error(
                    format!(
                        "`{}`'s finish number must be a constant expression",
                        call.name
                    ),
                    argument.span(),
                );
                1
            }
        }
    }

    /// An input argument: an empty position, a string literal, or a value.
    /// `formats` says the call formats its arguments, where a bare
    /// `$realtime` has a default format of its own.
    fn input_argument(
        &mut self,
        block: BlockId,
        argument: &Expression,
        operands: &mut Vec<ValueId>,
        formats: bool,
    ) -> DigitalSystemArgument {
        match argument {
            Expression::NullArgument(_) => DigitalSystemArgument::Empty,
            Expression::StringLit(literal) => DigitalSystemArgument::String(literal.value.clone()),
            Expression::SystemFunction(function)
                if formats && function.name == "$realtime" && function.args.is_empty() =>
            {
                let value = self.real_expression(block, argument);
                operands.push(value);
                DigitalSystemArgument::RealTime {
                    operand: (operands.len() - 1) as u32,
                }
            }
            _ => {
                let shape = expressions::shape(self, argument);
                let (value, class) = if shape.real {
                    (
                        self.real_expression(block, argument),
                        DigitalValueClass::Real,
                    )
                } else {
                    let value = self.expression(block, argument);
                    (
                        value,
                        DigitalValueClass::FourState {
                            width: self.value_width(value),
                            signed: shape.signed,
                        },
                    )
                };
                operands.push(value);
                DigitalSystemArgument::Value {
                    operand: (operands.len() - 1) as u32,
                    class,
                }
            }
        }
    }

    /// A reference argument: a signal, or a module instance or block scope.
    fn reference_argument(
        &mut self,
        argument: &Expression,
        scope: &ScopeFrame,
        call: &SystemFunction,
    ) -> DigitalSystemArgument {
        let Expression::Identifier(identifier) = argument else {
            if !matches!(argument, Expression::NullArgument(_)) {
                self.error(
                    format!(
                        "`{}` names a module instance or signal here, not an expression",
                        call.name
                    ),
                    argument.span(),
                );
            }
            return DigitalSystemArgument::Empty;
        };
        if let Some(signal) = self.resolved_signal(&identifier.name) {
            return DigitalSystemArgument::Signal(signal);
        }
        let name = identifier.name.as_str();
        // The frame's module scope is the bottom of the stack; a block scope
        // only extends its path.
        let module = self
            .calls
            .stack
            .first()
            .cloned()
            .unwrap_or_else(|| scope.clone());
        let found =
            if name == module.module.as_str() || module.path.rsplit('.').next() == Some(name) {
                // The module's own name (`$printtimescale(top)` inside `top`).
                Some(module.id)
            } else {
                [name.to_string(), format!("{}.{name}", module.path)]
                    .iter()
                    .find_map(|path| self.calls.module_scopes.get(path.as_str()).copied())
            };
        match found {
            Some(id) => DigitalSystemArgument::Scope(id),
            None => {
                self.error(
                    format!(
                        "`{}` names `{name}`, which is neither a signal nor a module instance \
                         visible here",
                        call.name
                    ),
                    identifier.span,
                );
                DigitalSystemArgument::Empty
            }
        }
    }

    /// The value class an output target is written with.
    fn target_class(&mut self, target: &DigitalLValue) -> DigitalValueClass {
        if self.lvalue_is_real(target) {
            DigitalValueClass::Real
        } else {
            let signed = match target {
                DigitalLValue::Identifier { name, .. } => match self.lookup_local(name) {
                    Some(local) => self.local_signed(local),
                    None => self
                        .index
                        .get(name.as_str())
                        .is_some_and(|signal| self.signed_signal(*signal)),
                },
                _ => false,
            };
            DigitalValueClass::FourState {
                width: self.lvalue_width(target),
                signed,
            }
        }
    }

    /// Lower one call; a function's value is returned.
    fn lower_call(
        &mut self,
        block: BlockId,
        call: &SystemFunction,
        kind: DigitalSystemCallKind,
        task: bool,
    ) -> Option<ValueId> {
        let scope = self.call_context(call)?;
        let (site, local) = self.allocate_site(kind, &scope, call.span);
        let mut level = None;
        let mut skip = 0;
        match kind {
            DigitalSystemCallKind::Finish | DigitalSystemCallKind::Stop => {
                level = Some(match call.args.first() {
                    Some(argument) if !matches!(argument, Expression::NullArgument(_)) => {
                        self.finish_level(argument, call)
                    }
                    _ => 1,
                });
                skip = call.args.len();
            }
            DigitalSystemCallKind::Fatal => match call.args.first() {
                None | Some(Expression::StringLit(_)) => level = Some(1),
                Some(Expression::NullArgument(_)) => {
                    level = Some(1);
                    skip = 1;
                }
                Some(argument) => {
                    level = Some(self.finish_level(argument, call));
                    skip = 1;
                }
            },
            _ => {}
        }
        let mut arguments = Vec::with_capacity(call.args.len());
        let mut operands = Vec::new();
        let mut outputs: Vec<(DigitalLValue, DigitalValueClass)> = Vec::new();
        for (position, argument) in call.args.iter().enumerate().skip(skip) {
            match kind.argument_use(position) {
                DigitalArgumentUse::Input => {
                    let argument =
                        self.input_argument(block, argument, &mut operands, kind.formats());
                    arguments.push(argument);
                }
                DigitalArgumentUse::Reference => {
                    let argument = self.reference_argument(argument, &scope, call);
                    arguments.push(argument);
                }
                DigitalArgumentUse::ArrayOutput => {
                    let array = match argument {
                        Expression::Identifier(identifier) => self.digital_array(&identifier.name),
                        _ => None,
                    };
                    match array {
                        Some(array) => {
                            let class = if self.real_signal(array.base) {
                                DigitalValueClass::Real
                            } else {
                                DigitalValueClass::FourState {
                                    width: self.width_of(array.base),
                                    signed: self.signed_signal(array.base),
                                }
                            };
                            arguments.push(DigitalSystemArgument::Array { array, class });
                        }
                        None => {
                            self.error(
                                format!(
                                    "argument {} of `{}` must name an unpacked array",
                                    position + 1,
                                    call.name
                                ),
                                argument.span(),
                            );
                            arguments.push(DigitalSystemArgument::Empty);
                        }
                    }
                }
                DigitalArgumentUse::Output | DigitalArgumentUse::InOut => {
                    let Some(target) = expression_as_target(argument) else {
                        self.error(
                            format!(
                                "argument {} of `{}` is written by the call, so it must name \
                                 a variable",
                                position + 1,
                                call.name
                            ),
                            argument.span(),
                        );
                        arguments.push(DigitalSystemArgument::Empty);
                        continue;
                    };
                    let class = self.target_class(&target);
                    // An output is read as well as written: a call that leaves
                    // its variable alone (`$value$plusargs` with no match, a
                    // short `$fscanf`) writes back what the variable held.
                    let value = if class == DigitalValueClass::Real {
                        self.real_expression(block, argument)
                    } else {
                        self.expression(block, argument)
                    };
                    operands.push(value);
                    let operand = Some((operands.len() - 1) as u32);
                    arguments.push(DigitalSystemArgument::Output {
                        output: outputs.len() as u32,
                        operand,
                        class,
                    });
                    outputs.push((target, class));
                }
            }
        }
        let result = (!task).then_some(DigitalValueClass::FourState {
            width: 32,
            signed: kind.result_signed(),
        });
        {
            let entry = &mut self.calls.sites[local];
            entry.arguments = arguments;
            entry.outputs = outputs.len() as u32;
            entry.result = result;
            entry.level = level;
        }
        let node = if task {
            self.builder.push(
                block,
                CfgValueType::Effect,
                CfgValueKind::DigitalSystemTask {
                    site,
                    args: operands,
                },
            )
        } else {
            self.builder.push(
                block,
                CfgValueType::FourState { width: 32 },
                CfgValueKind::DigitalSystemFunction {
                    site,
                    args: operands,
                },
            )
        };
        for (index, (target, class)) in outputs.into_iter().enumerate() {
            let ty = match class {
                DigitalValueClass::FourState { width, .. } => CfgValueType::FourState { width },
                DigitalValueClass::Real => CfgValueType::Real,
            };
            let value = self.builder.push(
                block,
                ty,
                CfgValueKind::DigitalCallOutput {
                    call: node,
                    index: index as u32,
                },
            );
            self.write(block, &target, value, false);
        }
        (!task).then_some(node)
    }

    /// `$strobe`/`$monitor` (and their file forms): arm an observer whose
    /// function evaluates the arguments in the monitor region.
    fn observer_call(
        &mut self,
        block: BlockId,
        task: &SystemFunction,
        kind: DigitalSystemCallKind,
    ) {
        let Some(scope) = self.call_context(task) else {
            return;
        };
        let Some(process) = self.process else {
            self.error(
                format!(
                    "`{}` inside a task or function has no executable form yet",
                    task.name
                ),
                task.span,
            );
            return;
        };
        let (site, local) = self.allocate_site(kind, &scope, task.span);
        // The arguments lower into their own function. Every process-local
        // they read was promoted to signal storage before lowering began, so
        // the reads are signal reads and need nothing from this activation.
        let process_builder = std::mem::replace(&mut self.builder, ProcessBuilder::new());
        let entry = self.builder.create_block();
        self.builder.seal_block(entry);
        let mut operands = Vec::new();
        let mut arguments = Vec::with_capacity(task.args.len());
        for argument in &task.args {
            let argument = self.input_argument(entry, argument, &mut operands, kind.formats());
            arguments.push(argument);
        }
        self.builder.push(
            entry,
            CfgValueType::Effect,
            CfgValueKind::DigitalSystemTask {
                site,
                args: operands,
            },
        );
        self.builder.set_terminator(entry, CfgTerminator::Return);
        self.builder.seal_all_blocks();
        let observer_builder = std::mem::replace(&mut self.builder, process_builder);
        let function = match observer_builder.finish(entry) {
            Ok(function) => function,
            Err(error) => {
                self.invariant(
                    format!("`{}` argument function is invalid: {error}", task.name),
                    task.span,
                );
                return;
            }
        };
        let mut dependencies: Vec<DigitalSignalId> = Vec::new();
        for value in &function.values {
            match &value.kind {
                CfgValueKind::DigitalSignalRead { signal }
                | CfgValueKind::DigitalRealSignalRead { signal } => dependencies.push(*signal),
                CfgValueKind::DigitalArrayRead { array, .. } => {
                    if let Some(range) = array.cell_range() {
                        dependencies.extend(range.map(DigitalSignalId::new));
                    }
                }
                _ => {}
            }
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        let observer = self.calls.observer_base + self.calls.observers.len() as u32;
        self.calls.observers.push(DigitalObserver {
            site,
            process,
            function,
            dependencies,
        });
        {
            let entry = &mut self.calls.sites[local];
            entry.arguments = arguments;
            entry.observer = Some(observer);
        }
        self.builder.push(
            block,
            CfgValueType::Effect,
            CfgValueKind::DigitalSystemTask {
                site,
                args: Vec::new(),
            },
        );
    }
}
