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
    /// The name the source declares the module by: a library binding's
    /// `library.cell` without its library, otherwise the module's own name.
    pub cell: SmolStr,
    /// The scope's own name: the root module's, an instance's or a block's.
    pub name: SmolStr,
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
    pub override_base: u32,
    pub scopes: Vec<DigitalScope>,
    pub sites: Vec<DigitalSystemCall>,
    pub observers: Vec<DigitalObserver>,
    pub overrides: Vec<super::super::digital::DigitalOverride>,
    /// The scope being lowered, innermost last; the frame's module scope is at
    /// the bottom. Empty for a constant or a continuous assignment, where no
    /// call is allowed.
    pub stack: Vec<ScopeFrame>,
    /// Every module instance scope of the plan, by full path, for a call that
    /// names one (`$printtimescale(top.u1)`).
    pub module_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    /// The module instance each hierarchical scope argument names, by the
    /// referring module scope and the parser's symbol.
    pub scope_references: Arc<HashMap<(DigitalScopeId, SmolStr), DigitalScopeId>>,
    /// The other scopes hierarchical scope arguments name, by full path:
    /// generate blocks, named blocks, tasks and functions.
    pub scope_paths: Arc<HashMap<(DigitalScopeId, SmolStr), SmolStr>>,
    /// The joined ports hierarchical reference arguments name, as indices
    /// into the plan's port aliases.
    pub alias_references: Arc<HashMap<(DigitalScopeId, SmolStr), u32>>,
    /// Every generate block's scope, by full path.
    pub generate_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    /// The source's other top-level modules, which a scope argument may name
    /// by their names alone (`$printtimescale(othertop)`).
    pub top_levels: Arc<HashMap<SmolStr, crate::time_scale::ModuleTimeScale>>,
    /// The other top-level modules' scopes this process named first, by path.
    pub external_scopes: Vec<(SmolStr, DigitalScopeId)>,
    /// Scope arguments that name a scope another process may open.
    pub pending_scopes: Vec<PendingScope>,
    /// The compiled root's path, and whether the design calls `$dumpvars`.
    pub root: SmolStr,
    pub dumps: bool,
    /// `disable` lowering: the names some `disable` targets, the disabled
    /// named blocks around the statement being lowered, and the routine
    /// being lowered.
    pub threads: super::disable::ThreadTables,
    /// The whole-net operands of the call whose arguments are being lowered.
    pub pending_nets: Vec<super::super::digital::DigitalOperandNet>,
}

/// The plan-level tables the processes' calls collect into.
#[derive(Default)]
pub(super) struct PlanCalls {
    pub scopes: Vec<DigitalScope>,
    pub sites: Vec<DigitalSystemCall>,
    pub observers: Vec<DigitalObserver>,
    pub overrides: Vec<super::super::digital::DigitalOverride>,
    pub module_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    pub scope_references: Arc<HashMap<(DigitalScopeId, SmolStr), DigitalScopeId>>,
    pub scope_paths: Arc<HashMap<(DigitalScopeId, SmolStr), SmolStr>>,
    pub alias_references: Arc<HashMap<(DigitalScopeId, SmolStr), u32>>,
    pub generate_scopes: Arc<HashMap<SmolStr, DigitalScopeId>>,
    pub top_levels: Arc<HashMap<SmolStr, crate::time_scale::ModuleTimeScale>>,
    /// Scope arguments resolved once every scope exists.
    pub pending_scopes: Vec<PendingScope>,
    /// Whether the design calls `$dumpvars`, so that a named block's
    /// variables keep storage a dump can follow.
    pub dumps: bool,
    /// The scopes that stand for other top-level modules' instances, which
    /// this plan names but does not hold.
    pub external_scopes: Vec<DigitalScopeId>,
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
        cell: SmolStr,
        name: SmolStr,
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
            cell,
            name,
            time_scale,
        }
    }

    /// A generate block's scope (IEEE 1364-2005 section 12.4) in `parent`.
    pub fn add_generate_scope(
        &mut self,
        parent: DigitalScopeId,
        path: SmolStr,
        module: SmolStr,
        time_scale: crate::time_scale::ModuleTimeScale,
    ) {
        let id = DigitalScopeId(self.scopes.len() as u32);
        self.scopes.push(DigitalScope {
            id,
            parent: Some(parent),
            path: path.clone(),
            module,
            kind: DigitalScopeKind::Generate,
            time_scale,
        });
        Arc::make_mut(&mut self.generate_scopes).insert(path, id);
    }

    /// The scope standing for an instance of another top-level module: the
    /// module itself when `path` is its name. Linking keeps its path.
    pub fn external_scope(
        &mut self,
        path: SmolStr,
        module: SmolStr,
        time_scale: crate::time_scale::ModuleTimeScale,
    ) -> DigitalScopeId {
        if let Some(&id) = self.module_scopes.get(&path) {
            return id;
        }
        let name: SmolStr = path
            .rsplit_once('.')
            .map_or(path.as_str(), |(_, name)| name)
            .into();
        let scope = self.add_module_scope(None, path, module.clone(), module, name, time_scale);
        self.external_scopes.push(scope.id);
        scope.id
    }

    /// Tables for one process lowered in `scope`.
    pub fn tables_for(&self, scope: Option<&ScopeFrame>) -> CallTables {
        CallTables {
            scope_base: self.scopes.len() as u32,
            site_base: self.sites.len() as u32,
            observer_base: self.observers.len() as u32,
            override_base: self.overrides.len() as u32,
            stack: scope.cloned().into_iter().collect(),
            module_scopes: Arc::clone(&self.module_scopes),
            scope_references: Arc::clone(&self.scope_references),
            scope_paths: Arc::clone(&self.scope_paths),
            alias_references: Arc::clone(&self.alias_references),
            generate_scopes: Arc::clone(&self.generate_scopes),
            root: self
                .scopes
                .first()
                .map(|scope| scope.path.clone())
                .unwrap_or_default(),
            dumps: self.dumps,
            top_levels: Arc::clone(&self.top_levels),
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
        debug_assert_eq!(tables.override_base as usize, self.overrides.len());
        self.scopes.extend(tables.scopes);
        for (path, id) in tables.external_scopes {
            Arc::make_mut(&mut self.module_scopes).insert(path, id);
            self.external_scopes.push(id);
        }
        self.sites.extend(tables.sites);
        self.pending_scopes.extend(tables.pending_scopes);
        self.observers.extend(tables.observers);
        self.overrides.extend(tables.overrides);
        self.pending_disables
            .extend(tables.threads.pending.into_iter().map(|mut pending| {
                pending.owner = Some(owner);
                pending
            }));
    }
}

/// A scope argument naming a scope that may not exist yet where the call is
/// lowered: argument `position` of call `site`, and the scope's full path.
pub(super) struct PendingScope {
    site: DigitalSystemCallId,
    position: usize,
    path: SmolStr,
    span: Span,
}

/// Point each pending scope argument at its scope, which every process and
/// routine has opened by now.
pub(super) fn resolve_scopes(
    sites: &mut [DigitalSystemCall],
    scopes: &[DigitalScope],
    pending: Vec<PendingScope>,
    diagnostics: &mut Vec<DigitalLoweringDiagnostic>,
) {
    if pending.is_empty() {
        return;
    }
    let mut by_path: HashMap<&str, DigitalScopeId> = HashMap::new();
    for scope in scopes {
        by_path.entry(scope.path.as_str()).or_insert(scope.id);
    }
    for pending in pending {
        let argument = sites
            .get_mut(pending.site.0 as usize)
            .and_then(|site| site.arguments.get_mut(pending.position));
        match (by_path.get(pending.path.as_str()), argument) {
            (Some(&id), Some(argument)) => *argument = DigitalSystemArgument::Scope(id),
            _ => diagnostics.push(DigitalLoweringDiagnostic::invariant(
                format!("scope `{}` has no lowered scope", pending.path),
                pending.span.into(),
            )),
        }
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
            cell: parent.cell,
            name: name.clone(),
            time_scale: parent.time_scale,
        });
        Some(id)
    }

    /// Enter a generate block's scope, made before any process was lowered.
    pub(super) fn enter_generate_scope(&mut self, name: &SmolStr) {
        let Some(parent) = self.calls.stack.last().cloned() else {
            return;
        };
        let path: SmolStr = format!("{}.{name}", parent.path).into();
        match self.calls.generate_scopes.get(&path).copied() {
            Some(id) => self.calls.stack.push(ScopeFrame {
                id,
                path,
                module: parent.module,
                cell: parent.cell,
                name: name.clone(),
                time_scale: parent.time_scale,
            }),
            None => {
                self.enter_scope(name, DigitalScopeKind::Generate);
            }
        }
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
            nets: Vec::new(),
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
                let operand = (operands.len() - 1) as u32;
                // A bare net name: `%v` prints that net's strengths.
                if formats
                    && let Expression::Identifier(identifier) = argument
                    && let Some(signal) = self.resolved_signal(&identifier.name)
                    && self.signals.get(usize::from(signal)).is_some_and(|signal| {
                        !signal.procedurally_assignable && !signal.kind.is_real()
                    })
                {
                    self.calls
                        .pending_nets
                        .push(super::super::digital::DigitalOperandNet { operand, signal });
                }
                DigitalSystemArgument::Value { operand, class }
            }
        }
    }

    /// A reference argument: a signal, or a module instance or block scope.
    /// It is argument `position` of call `site`.
    fn reference_argument(
        &mut self,
        argument: &Expression,
        scope: &ScopeFrame,
        call: &SystemFunction,
        (site, position): (DigitalSystemCallId, usize),
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
        let module = self
            .calls
            .stack
            .first()
            .cloned()
            .unwrap_or_else(|| scope.clone());
        let named = (module.id, identifier.name.clone());
        if let Some(&id) = self.calls.scope_references.get(&named) {
            // `$dumpvars(0, top.u1)`: resolved during elaboration.
            return DigitalSystemArgument::Scope(id);
        }
        let dump =
            DigitalSystemCallKind::from_name(&call.name) == Some(DigitalSystemCallKind::Dumpvars);
        if dump && let Some(path) = self.calls.scope_paths.get(&named) {
            // `$dumpvars(0, top.b)`: a generate block, named block, task or
            // function (IEEE 1364-2005 section 18.1.2), whose scope the
            // process that opens it makes.
            self.calls.pending_scopes.push(PendingScope {
                site,
                position,
                path: path.clone(),
                span: identifier.span,
            });
            return DigitalSystemArgument::Scope(scope.id);
        }
        if dump && crate::ast::is_reference_symbol(&identifier.name) {
            // Section 18.1.2 dumps such a variable under the scopes of its
            // own path; a port joined to another scope's net keeps its own.
            if let Some(&alias) = self.calls.alias_references.get(&named) {
                return DigitalSystemArgument::Alias(alias);
            }
            if let Some(signal) = self.resolved_signal(&identifier.name) {
                return DigitalSystemArgument::Signal(signal);
            }
        }
        if DigitalSystemCallKind::from_name(&call.name)
            == Some(DigitalSystemCallKind::PrintTimeScale)
            && crate::ast::is_reference_symbol(&identifier.name)
        {
            self.invalid(
                "the argument of `$printtimescale` names a module (IEEE 1364-2005 section \
                 17.4.2), and this hierarchical name is not a module instance",
                identifier.span,
            );
            return DigitalSystemArgument::Empty;
        }
        if crate::ast::is_reference_symbol(&identifier.name) {
            self.error(
                format!(
                    "`{}` of a hierarchically named signal is not supported yet",
                    call.name
                ),
                identifier.span,
            );
            return DigitalSystemArgument::Empty;
        }
        if let Some(signal) = self.resolved_signal(&identifier.name) {
            return DigitalSystemArgument::Signal(signal);
        }
        let name = identifier.name.as_str();
        // A library binding's module is known by `library.cell`, and its
        // source names the cell. Only a binding's name drops a prefix: a
        // module declared `\a.u ` is named `a.u`, and its child `u` is not it.
        let found = if name == module.module.as_str()
            || name == module.cell.as_str()
            || name == module.name.as_str()
        {
            // The module's own name (`$printtimescale(top)` inside `top`).
            Some(module.id)
        } else {
            [name.to_string(), format!("{}.{name}", module.path)]
                .iter()
                .find_map(|path| self.calls.module_scopes.get(path.as_str()).copied())
                .or_else(|| self.top_level_scope(name))
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

    /// Another top-level module named alone (`$printtimescale(othertop)`):
    /// a scope that stands for it, made once per plan.
    fn top_level_scope(&mut self, name: &str) -> Option<DigitalScopeId> {
        if let Some((_, id)) = self
            .calls
            .external_scopes
            .iter()
            .find(|(path, _)| path == name)
        {
            return Some(*id);
        }
        let time_scale = *self.calls.top_levels.get(name)?;
        let id = DigitalScopeId(self.calls.scope_base + self.calls.scopes.len() as u32);
        self.calls.scopes.push(DigitalScope {
            id,
            parent: None,
            path: name.into(),
            module: name.into(),
            kind: DigitalScopeKind::Module,
            time_scale,
        });
        self.calls.external_scopes.push((name.into(), id));
        Some(id)
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
        let enclosing_nets = std::mem::take(&mut self.calls.pending_nets);
        let mut outputs: Vec<(DigitalLValue, DigitalValueClass)> = Vec::new();
        for (position, argument) in call.args.iter().enumerate().skip(skip) {
            let usage = match kind.argument_use(position) {
                // `$fread` loads an array when it names one, and otherwise
                // writes a variable.
                DigitalArgumentUse::ArrayOrOutput => match argument {
                    Expression::Identifier(identifier)
                        if self.digital_array(&identifier.name).is_some() =>
                    {
                        DigitalArgumentUse::ArrayOutput
                    }
                    _ => DigitalArgumentUse::Output,
                },
                usage => usage,
            };
            match usage {
                DigitalArgumentUse::Input => {
                    let argument =
                        self.input_argument(block, argument, &mut operands, kind.formats());
                    arguments.push(argument);
                }
                DigitalArgumentUse::Reference => {
                    let argument =
                        self.reference_argument(argument, &scope, call, (site, arguments.len()));
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
                DigitalArgumentUse::Output
                | DigitalArgumentUse::InOut
                | DigitalArgumentUse::ArrayOrOutput => {
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
            let nets = std::mem::replace(&mut self.calls.pending_nets, enclosing_nets);
            let entry = &mut self.calls.sites[local];
            entry.arguments = arguments;
            entry.nets = nets;
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
        let enclosing_nets = std::mem::take(&mut self.calls.pending_nets);
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
            let nets = std::mem::replace(&mut self.calls.pending_nets, enclosing_nets);
            let entry = &mut self.calls.sites[local];
            entry.arguments = arguments;
            entry.nets = nets;
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
