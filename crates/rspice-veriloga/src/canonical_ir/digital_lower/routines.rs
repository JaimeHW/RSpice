//! Tasks and functions (IEEE 1364-2005 section 10, design P2 §4.1).
//!
//! Each routine of each module instance lowers to its own function. A static
//! routine keeps its arguments and variables in signals, which every
//! activation shares (section 10.2.1); an automatic one keeps them in the
//! activation's frame slots. A caller copies the inputs in through the call
//! and reads the outputs back after it: a task enable suspends the caller's
//! frame like a timing control, and a function call is an expression node.
use super::super::digital::{
    DigitalRoutine, DigitalRoutineId, DigitalRoutineKind, DigitalRoutineParam,
    DigitalRoutineStorage, DigitalScopeKind,
};
use super::*;
use crate::ast::{DigitalRoutineType, DigitalRoutineValueKind, DigitalTaskEnable, PortDirection};
use crate::semantic::AnalyzedDigitalRoutine;
use std::collections::BTreeMap;

/// The value a routine argument, result or variable holds.
#[derive(Debug, Clone, Copy)]
pub(super) struct RoutineShape {
    pub real: bool,
    pub integer: bool,
    /// Whether the declaration has a range (a `reg` without one is scalar).
    pub packed: bool,
    pub bounds: VectorBounds,
    pub signed: bool,
}

impl RoutineShape {
    pub fn width(self) -> u32 {
        if self.real { 0 } else { self.bounds.width() }
    }

    pub fn value_type(self) -> CfgValueType {
        if self.real {
            CfgValueType::Real
        } else {
            CfgValueType::FourState {
                width: self.width(),
            }
        }
    }
}

/// What a call site needs of a routine.
#[derive(Debug, Clone)]
pub(super) struct RoutineSignature {
    pub id: DigitalRoutineId,
    pub kind: DigitalRoutineKind,
    pub params: Vec<(PortDirection, RoutineShape)>,
    pub result: Option<RoutineShape>,
}

/// The routines a scope's bodies can call, by name.
pub(super) type RoutineTable = BTreeMap<SmolStr, RoutineSignature>;

/// The table of a body that can call no routine.
pub(super) static NO_ROUTINES: RoutineTable = BTreeMap::new();

fn shape(
    ty: &DigitalRoutineType,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Option<RoutineShape> {
    Some(match ty.kind {
        DigitalRoutineValueKind::Real => RoutineShape {
            real: true,
            integer: false,
            packed: false,
            bounds: VectorBounds::SCALAR,
            signed: true,
        },
        DigitalRoutineValueKind::Integer => RoutineShape {
            real: false,
            integer: true,
            packed: true,
            bounds: INTEGER_BOUNDS,
            signed: true,
        },
        // Section 3.9: `time` is an unsigned 64-bit variable.
        DigitalRoutineValueKind::Time => RoutineShape {
            real: false,
            integer: false,
            packed: true,
            bounds: VectorBounds { msb: 63, lsb: 0 },
            signed: false,
        },
        DigitalRoutineValueKind::Reg => {
            let bounds = match &ty.range {
                None => VectorBounds::SCALAR,
                Some(range) => {
                    let bound =
                        |expression| match constants::scalar(expression, constants, time_scale)? {
                            crate::numeric_literal::NumericLiteralValue::Integer(value) => {
                                Some(value)
                            }
                            crate::numeric_literal::NumericLiteralValue::Real(value) => {
                                crate::semantic::SemanticAnalyzer::exact_const_i64(value)
                            }
                        };
                    VectorBounds {
                        msb: bound(&range.msb)?,
                        lsb: bound(&range.lsb)?,
                    }
                }
            };
            RoutineShape {
                real: false,
                integer: false,
                packed: ty.range.is_some(),
                bounds,
                signed: ty.signedness.is_signed(),
            }
        }
    })
}

/// Number the routines of one scope from `next` and type their arguments,
/// before any body is lowered: a body may call any routine of its scope,
/// itself included.
pub(super) fn signatures(
    routines: &[AnalyzedDigitalRoutine],
    next: &mut u32,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    diagnostics: &mut Vec<DigitalLoweringDiagnostic>,
) -> RoutineTable {
    let mut table = RoutineTable::new();
    for analyzed in routines {
        let routine = &analyzed.routine;
        let mut params = Vec::with_capacity(routine.arguments.len());
        for argument in &routine.arguments {
            match shape(&argument.ty, constants, time_scale) {
                Some(shape) => params.push((argument.direction, shape)),
                None => diagnostics.push(DigitalLoweringDiagnostic::refusal(
                    format!(
                        "the range of argument `{}` must be a constant expression",
                        argument.name
                    ),
                    argument.span.into(),
                )),
            }
        }
        let result = match &routine.kind {
            crate::ast::DigitalRoutineKind::Task => None,
            crate::ast::DigitalRoutineKind::Function(ty) => {
                match shape(ty, constants, time_scale) {
                    Some(shape) => Some(shape),
                    None => {
                        diagnostics.push(DigitalLoweringDiagnostic::refusal(
                            format!(
                                "the range of function `{}` must be a constant expression",
                                routine.name
                            ),
                            routine.span.into(),
                        ));
                        continue;
                    }
                }
            }
        };
        let id = DigitalRoutineId(*next);
        *next += 1;
        table.insert(
            routine.name.clone(),
            RoutineSignature {
                id,
                kind: if routine.is_function() {
                    DigitalRoutineKind::Function
                } else {
                    DigitalRoutineKind::Task
                },
                params,
                result,
            },
        );
    }
    table
}

/// The signal a static routine variable lives in.
fn static_signal(
    signals: &mut Vec<DigitalSignal>,
    name: String,
    shape: RoutineShape,
    span: Span,
) -> DigitalSignalId {
    let mut storage_name = name;
    while signals.iter().any(|signal| signal.name == storage_name) {
        storage_name.insert(0, '$');
    }
    let id = DigitalSignalId::from(signals.len());
    signals.push(DigitalSignal {
        local: None,
        initial_value: None,
        id,
        name: storage_name.into(),
        kind: if shape.real {
            DigitalSignalKind::Real(DigitalRealResolution::Single)
        } else {
            DigitalSignalKind::FourState
        },
        width: shape.width(),
        bounds: (!shape.real && shape.packed).then_some((shape.bounds.msb, shape.bounds.lsb)),
        signed: shape.signed,
        integer: shape.integer,
        procedurally_assignable: true,
        span: span.into(),
    });
    id
}

/// Everything one scope's bodies are lowered against.
pub(super) struct RoutineScope<'a> {
    pub signals: &'a mut Vec<DigitalSignal>,
    pub arrays: &'a HashMap<DigitalSignalId, super::super::digital::DigitalArray>,
    pub index: &'a HashMap<&'a str, DigitalSignalId>,
    pub array_names: &'a HashSet<SmolStr>,
    pub constants: &'a ResolvedConstants,
    pub analog_variables: &'a HashMap<SmolStr, AnalogVariable>,
    pub probes: &'a mut Vec<DigitalAnalogProbe>,
    pub time_scale: crate::time_scale::ModuleTimeScale,
    pub calls: &'a mut PlanCalls,
    pub scope: &'a ScopeFrame,
    pub routines: &'a RoutineTable,
    /// The prefix of the scope's elaborated signal names: empty for the
    /// root, `path.` for an instance.
    pub prefix: &'a str,
}

/// Lower one routine of a scope.
pub(super) fn lower_routine(
    analyzed: &AnalyzedDigitalRoutine,
    scope: RoutineScope<'_>,
) -> Result<DigitalRoutine, Vec<DigitalLoweringDiagnostic>> {
    let routine = &analyzed.routine;
    let Some(signature) = scope.routines.get(&routine.name).cloned() else {
        // Its signature was refused, and that refusal is already reported.
        return Err(Vec::new());
    };
    let RoutineScope {
        signals,
        arrays,
        index,
        array_names,
        constants,
        analog_variables,
        probes,
        time_scale,
        calls,
        scope,
        routines,
        prefix,
    } = scope;
    let mut lowerer = ProcessLowerer {
        calls: calls.tables_for(Some(scope)),
        analog_local_inputs: HashMap::new(),
        process: None,
        routines,
        local_arrays: Vec::new(),
        constant_expression: false,
        time_scale,
        signals,
        arrays,
        index,
        array_names,
        constants,
        analog_variables,
        probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };
    let kind = if routine.is_function() {
        DigitalScopeKind::Function
    } else {
        DigitalScopeKind::Task
    };
    let routine_scope = lowerer.enter_scope(&routine.name, kind);
    let entry = lowerer.builder.create_block();
    lowerer.builder.seal_block(entry);

    // The arguments, the function's result and the routine's variables form
    // the outermost declarative region of the body (section 10.2.1).
    lowerer.scopes.push(Vec::new());
    let mut params = Vec::with_capacity(routine.arguments.len());
    for (argument, (_, shape)) in routine.arguments.iter().zip(&signature.params) {
        let local = lowerer.declare_routine_local(entry, &argument.name, *shape, argument.span);
        params.push((argument.name.clone(), argument.direction, local, *shape));
    }
    let result = signature.result.map(|shape| {
        (
            routine.name.clone(),
            lowerer.declare_routine_local(entry, &routine.name, shape, routine.span),
            shape,
        )
    });
    let variables = crate::ast::DigitalBlock {
        name: None,
        variables: routine.variables.clone(),
        digital_variables: routine.digital_variables.clone(),
        statements: Vec::new(),
        span: routine.span,
    };
    lowerer.declare_block_locals(entry, &variables);
    lowerer.initialize_static_locals(entry, &routine.body);
    lowerer.static_local_count = lowerer.locals.len();

    // Every named variable of the body, at any depth, moves to the routine's
    // storage before any statement is lowered.
    let mut frame_slots = Vec::new();
    let path = format!("{prefix}{}", routine.name);
    for id in 0..lowerer.locals.len() {
        let local = &lowerer.locals[id];
        let Some(name) = local.name.clone() else {
            continue;
        };
        if local.shared.is_some() {
            continue;
        }
        let shape = RoutineShape {
            real: local.real,
            integer: local.integer,
            packed: local.packed,
            bounds: local.bounds,
            signed: local.signed,
        };
        if routine.automatic {
            lowerer.locals[id].slot = Some(frame_slots.len() as u32);
            frame_slots.push(shape.value_type());
        } else {
            let span = local.span;
            let signal = static_signal(lowerer.signals, format!("{path}.{name}"), shape, span);
            lowerer.locals[id].shared = Some(signal);
        }
    }
    let storage = |lowerer: &ProcessLowerer<'_>, local: DigitalLocalId| {
        let local = &lowerer.locals[usize::from(local)];
        match (local.shared, local.slot) {
            (Some(signal), _) => DigitalRoutineStorage::Signal(signal),
            (None, Some(slot)) => DigitalRoutineStorage::Slot(slot),
            (None, None) => unreachable!("every routine variable has storage"),
        }
    };
    let params: Vec<_> = params
        .into_iter()
        .map(|(name, direction, local, shape)| DigitalRoutineParam {
            name,
            direction,
            storage: storage(&lowerer, local),
            value_type: shape.value_type(),
            signed: shape.signed,
        })
        .collect();
    let result = result.map(|(name, local, shape)| DigitalRoutineParam {
        name,
        direction: PortDirection::Output,
        storage: storage(&lowerer, local),
        value_type: shape.value_type(),
        signed: shape.signed,
    });

    let exit = lowerer.statement(entry, &routine.body);
    lowerer.builder.set_terminator(exit, CfgTerminator::Return);
    lowerer.scopes.pop();
    lowerer.leave_named_scope();
    lowerer.builder.seal_all_blocks();
    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }
    let Some(routine_scope) = routine_scope else {
        return Err(vec![DigitalLoweringDiagnostic::invariant(
            "a routine was lowered outside any module scope",
            routine.span.into(),
        )]);
    };
    calls.absorb(lowerer.calls);
    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![DigitalLoweringDiagnostic::invariant(
            format!(
                "lowering `{}` produced an invalid graph: {error}",
                routine.name
            ),
            routine.span.into(),
        )]
    })?;
    Ok(DigitalRoutine {
        id: signature.id,
        name: routine.name.clone(),
        kind: signature.kind,
        automatic: routine.automatic,
        function,
        params,
        result,
        frame_slots,
        scope: routine_scope,
        time_scale,
        span: routine.span.into(),
    })
}

impl ProcessLowerer<'_> {
    /// `wait (condition) statement` (IEEE 1364-2005 section 9.7.6): go on at
    /// once when the condition is true, otherwise wait for its value to change
    /// and test it again. An `x` or `z` condition is not true.
    pub(super) fn wait_statement(
        &mut self,
        block: BlockId,
        wait: &crate::ast::DigitalWaitStatement,
    ) -> BlockId {
        let header = self.builder.create_block();
        self.builder.set_terminator(
            block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        let condition = self.condition(header, &wait.condition);
        let waiting = self.builder.create_block();
        let done = self.builder.create_block();
        self.builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: done,
                then_args: Vec::new(),
                else_target: waiting,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(waiting);
        self.builder.seal_block(done);
        let control = crate::ast::TimingControl::Event(crate::ast::EventControl {
            repeat: None,
            sensitivity: crate::ast::Sensitivity::Explicit(vec![crate::ast::EventTerm {
                edge: None,
                signal: wait.condition.clone(),
                span: wait.condition.span(),
            }]),
            span: wait.span,
        });
        let resume = self.wait(waiting, &control, None, &mut []);
        self.builder.set_terminator(
            resume,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(header);
        match &wait.statement {
            Some(statement) => self.statement(done, statement),
            None => done,
        }
    }

    /// `-> event` (section 9.7.3). A named event is a one-bit variable that
    /// starts at 0, so toggling it is a value change that wakes every process
    /// waiting on `@event`, now, in the active region.
    pub(super) fn event_trigger(
        &mut self,
        block: BlockId,
        trigger: &crate::ast::DigitalEventTrigger,
    ) -> BlockId {
        let source = match &trigger.target {
            DigitalLValue::Identifier { name, span } => {
                Expression::Identifier(crate::ast::Identifier {
                    name: name.clone(),
                    span: *span,
                })
            }
            DigitalLValue::ArraySelect(access) => {
                Expression::Digital(crate::ast::DigitalExpr::ArraySelect(access.clone()))
            }
            _ => {
                self.error(
                    "`->` names an event or an element of an event array",
                    trigger.span,
                );
                return block;
            }
        };
        let current = self.expression(block, &source);
        let width = self.value_width(current);
        let toggled = self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalBitwiseNot { input: current },
        );
        self.write(block, &trigger.target, toggled, false);
        block
    }

    /// Declare an argument or a function's result as a local of `shape`.
    fn declare_routine_local(
        &mut self,
        block: BlockId,
        name: &SmolStr,
        shape: RoutineShape,
        span: Span,
    ) -> DigitalLocalId {
        let local = if shape.real {
            self.declare_real_local(block, Some(name.clone()), span, None)
        } else {
            self.declare_local(
                block,
                Some(name.clone()),
                shape.bounds,
                shape.signed,
                span,
                None,
            )
        };
        let declared = &mut self.locals[usize::from(local)];
        declared.integer = shape.integer;
        declared.packed = shape.packed;
        local
    }

    /// The routine a call names, when it names one of this scope's.
    pub(super) fn user_routine(&self, name: &str) -> Option<&RoutineSignature> {
        self.routines.get(name)
    }

    /// The function a call expression names, unless the front end resolved
    /// it to a built-in.
    pub(super) fn user_function(&self, call: &crate::ast::CallExpr) -> Option<&RoutineSignature> {
        (!call.resolved_builtin)
            .then(|| self.user_routine(&call.name))
            .flatten()
            .filter(|signature| signature.kind == DigitalRoutineKind::Function)
    }

    /// One input argument, converted to the argument's type the way an
    /// assignment converts (section 10.2.2).
    fn routine_argument(
        &mut self,
        block: BlockId,
        shape: RoutineShape,
        argument: &Expression,
    ) -> ValueId {
        if shape.real {
            return self.real_expression(block, argument);
        }
        // The assignment context widens the operands; the argument itself
        // then truncates or zero-fills to its declared width (section 5.2.1).
        let value = self.assigned_value(block, argument, shape.width());
        if self.value_width(value) == shape.width() {
            value
        } else {
            self.resize(block, value, shape.width(), false)
        }
    }

    /// A user function call (section 10.4.3), typed by the function's result.
    pub(super) fn function_call_value(
        &mut self,
        block: BlockId,
        call: &crate::ast::CallExpr,
    ) -> ValueId {
        let Some(signature) = self.user_function(call).cloned() else {
            self.invariant("a function call names no function", call.span);
            return self.unknown(1);
        };
        let result = signature.result.expect("a function has a result");
        if call.args.len() != signature.params.len() {
            self.error(
                format!(
                    "function `{}` takes {} argument(s)",
                    call.name,
                    signature.params.len()
                ),
                call.span,
            );
            return self.unknown(result.width().max(1));
        }
        let mut args = Vec::with_capacity(call.args.len());
        for ((_, shape), argument) in signature.params.iter().zip(&call.args) {
            args.push(self.routine_argument(block, *shape, argument));
        }
        self.builder.push(
            block,
            result.value_type(),
            CfgValueKind::DigitalCall {
                routine: signature.id,
                args,
            },
        )
    }

    /// A task enable (section 10.2.2): copy the inputs in, suspend the frame
    /// until the task ends, and copy the outputs back if it completed.
    pub(super) fn task_enable(&mut self, block: BlockId, enable: &DigitalTaskEnable) -> BlockId {
        let Some(signature) = self
            .user_routine(&enable.name)
            .filter(|signature| signature.kind == DigitalRoutineKind::Task)
            .cloned()
        else {
            self.error(
                format!("`{}` is not a task of this scope", enable.name),
                enable.span,
            );
            return block;
        };
        if enable.args.len() != signature.params.len() {
            self.error(
                format!(
                    "task `{}` takes {} argument(s)",
                    enable.name,
                    signature.params.len()
                ),
                enable.span,
            );
            return block;
        }
        let mut args = Vec::new();
        let mut outputs = Vec::new();
        for ((direction, shape), argument) in signature.params.iter().zip(&enable.args) {
            if matches!(direction, PortDirection::Input | PortDirection::Inout) {
                args.push(self.routine_argument(block, *shape, argument));
            }
            if matches!(direction, PortDirection::Output | PortDirection::Inout) {
                let Some(target) =
                    crate::semantic::digital_routines::expression_as_lvalue(argument)
                else {
                    self.error(
                        "an output or inout argument must be a variable or a select of one",
                        argument.span(),
                    );
                    return block;
                };
                outputs.push((*shape, target));
            }
        }
        let ret = self.suspend(
            block,
            DigitalWait::Call {
                routine: signature.id,
                args,
            },
            &mut [],
        );
        if outputs.is_empty() {
            return ret;
        }
        // Section 10.3 leaves a disabled task's outputs unspecified; RSpice
        // copies nothing back (design P2 §4.1).
        let completed = self.builder.push(
            ret,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalRoutineCompleted,
        );
        let copy = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.set_terminator(
            ret,
            CfgTerminator::Branch {
                condition: completed,
                then_target: copy,
                then_args: Vec::new(),
                else_target: join,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(copy);
        for (index, (shape, target)) in outputs.iter().enumerate() {
            let value = self.builder.push(
                copy,
                shape.value_type(),
                CfgValueKind::DigitalRoutineOutput {
                    index: index as u32,
                },
            );
            let value = if shape.real == self.lvalue_is_real(target) {
                value
            } else if shape.real {
                let width = self.lvalue_width(target);
                self.builder.push(
                    copy,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalRealToInteger {
                        input: value,
                        width,
                    },
                )
            } else {
                self.builder.push(
                    copy,
                    CfgValueType::Real,
                    CfgValueKind::DigitalIntegerToReal {
                        input: value,
                        signed: shape.signed,
                    },
                )
            };
            let value = if shape.real || self.lvalue_is_real(target) {
                value
            } else {
                let width = self.lvalue_width(target);
                self.resize(copy, value, width, shape.signed)
            };
            self.write(copy, target, value, false);
        }
        self.builder.set_terminator(
            copy,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(join);
        join
    }
}
