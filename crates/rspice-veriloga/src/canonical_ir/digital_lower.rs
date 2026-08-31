//! Lowering the analyzed discrete-domain half of a module into CFG process
//! functions.
//!
//! The analog counterpart is [`cfg_lower`](super::cfg_lower), which turns a
//! HIR body into one function. This does the same job for processes, from the
//! analyzed syntax tree rather than from HIR: a process body is already
//! source-shaped nested control flow, and routing it through a second
//! source-shaped level would copy it without resolving anything.
//!
//! # What state lives where
//!
//! Nothing in a process is an SSA variable. Every `reg` and `wire` is a
//! signal, read by a [`CfgValueKind::DigitalSignalRead`] node and written by a
//! write node, and a value therefore never has to cross a block boundary. That
//! is not a simplification of the model — it *is* the model. A process that
//! suspends and resumes must see whatever the signal holds when it wakes, and
//! a value carried across the suspension in a register would see what it held
//! when the process went to sleep.
//!
//! The consequence is that this wave's process functions have no block
//! parameters and every `Wait` resumes with no arguments. The mechanism for
//! carrying live state across a suspension exists in the terminator anyway,
//! because the first process-local variable will need it.
//!
//! # The wave-1 subset
//!
//! Everything the front end parses is not everything this lowers. Refusals are
//! by name, with a span, and are listed in [`refusals`](self#refusals) below
//! rather than discovered by a reader at the point of failure:
//!
//! - `for` and `repeat` need a loop counter, which is a process-local variable
//!   and therefore a typed SSA merge this wave does not build.
//! - A block-local `integer` or `real` declaration is the same problem.
//! - A continuous assignment (`assign`) drives a net rather than executing in
//!   a process. It has no representation here at all, and inventing one — a
//!   process that writes a wire — would encode something the language forbids
//!   everywhere else.
//!
//! Each of those refuses with a message naming the construct, so a model that
//! uses one is told what is missing rather than compiled into a device that is
//! quietly short of what its author wrote.

use super::cfg::{CfgTerminator, CfgValueKind, CfgValueType, DigitalWait, SsaBuilder};
use super::diagnostic::{CompilerPhase, IrDiagnostic, SourceSpanRef};
use super::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalEdge, DigitalProcessKind,
    DigitalSchedulingRegion, DigitalSensitivityOrigin, DigitalSensitivityTerm, DigitalSignal,
    DigitalStaticSensitivity, DigitalWriteSelect, DigitalWriteTarget,
};
use super::digital_value::{
    ArithmeticOp, BitwiseOp, FourStateValue, LogicalOp, RelationalOp, ShiftOp,
};
use super::ids::{BlockId, DigitalProcessId, DigitalSignalId, ValueId};
use crate::ast::DigitalProcessKind as AstKind;
use crate::ast::{
    ArrayLiteralElement, BinaryOp, DigitalAssign, DigitalCase, DigitalLValue, DigitalStatement,
    EdgeKind, Expression, TimingControl, UnaryOp,
};
use crate::four_state::FourStateBit;
use crate::semantic::{AnalyzedDigital, AnalyzedDigitalProcess, AnalyzedDigitalSignal};
use crate::source::Span;
use std::collections::{BTreeSet, HashMap};

/// Lower the analyzed discrete-domain content of a module.
///
/// Returns the plan on success, or every refusal at once — the same
/// accumulate-then-report discipline the rest of the front end uses, so an
/// author with three unsupported constructs learns about three.
pub fn lower(digital: &AnalyzedDigital) -> Result<CanonicalDigitalPlan, Vec<IrDiagnostic>> {
    if digital.is_empty() {
        return Ok(CanonicalDigitalPlan::default());
    }

    let mut diagnostics = Vec::new();
    let signals = lower_signals(&digital.signals);
    let index: HashMap<&str, DigitalSignalId> = signals
        .iter()
        .map(|signal| (signal.name.as_str(), signal.id))
        .collect();

    // A continuous assignment has no lowered form. Refuse each by name rather
    // than dropping it: a plan that silently omits a driver describes a
    // different circuit.
    for assignment in &digital.continuous_assigns {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!(
                "continuous assignment to `{}` has no lowered form yet; only \
                 `always` and `initial` processes are lowered in this wave",
                assignment.target
            ),
            assignment.span.into(),
        ));
    }

    let mut processes = Vec::new();
    for process in &digital.processes {
        match lower_process(process, &signals, &index) {
            Ok(lowered) => processes.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(CanonicalDigitalPlan { signals, processes })
}

fn lower_signals(analyzed: &[AnalyzedDigitalSignal]) -> Vec<DigitalSignal> {
    analyzed
        .iter()
        .enumerate()
        .map(|(position, signal)| DigitalSignal {
            id: DigitalSignalId::from(position),
            name: signal.name.clone(),
            width: signal.width,
            bounds: signal.range.map(|range| (range.msb, range.lsb)),
            signed: signal.signedness.is_signed(),
            procedurally_assignable: signal.class.is_variable(),
            span: signal.span.into(),
        })
        .collect()
}

fn lower_process(
    process: &AnalyzedDigitalProcess,
    signals: &[DigitalSignal],
    index: &HashMap<&str, DigitalSignalId>,
) -> Result<CfgDigitalProcess, Vec<IrDiagnostic>> {
    let mut lowerer = ProcessLowerer {
        signals,
        index,
        builder: SsaBuilder::new(),
        diagnostics: Vec::new(),
    };

    let entry = lowerer.builder.create_block();
    let exit = lowerer.statement(entry, &process.body);

    let kind = match process.kind {
        AstKind::Always => DigitalProcessKind::Always,
        AstKind::Initial => DigitalProcessKind::Initial,
    };

    // IEEE 1364-2005 sections 9.9.1 and 9.9.2, as a difference in the graph
    // rather than a flag: `always` loops back to its own entry, `initial`
    // returns. Nothing else has to be told which kind it is looking at.
    let terminator = if kind.restarts() {
        CfgTerminator::Jump {
            target: entry,
            args: Vec::new(),
        }
    } else {
        CfgTerminator::Return
    };
    lowerer.builder.set_terminator(exit, terminator);

    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }

    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!(
                "lowering process {} produced an invalid graph: {error}",
                process.id
            ),
            process.span.into(),
        )]
    })?;

    // Read the static list back off the entry block's `Wait` rather than
    // computing it a second time. The metadata and the terminator then cannot
    // disagree, which is the failure a separately-derived copy invites — and
    // an `@*` list would otherwise be derived twice and reported twice.
    let static_sensitivity = match (&process.body, &function.block(entry).terminator) {
        (
            DigitalStatement::Timing(timing),
            CfgTerminator::Wait {
                wait: DigitalWait::Event(terms),
                ..
            },
        ) => match &timing.control {
            TimingControl::Event(event) => Some(DigitalStaticSensitivity {
                terms: terms.clone(),
                origin: match event.sensitivity {
                    crate::ast::Sensitivity::Implicit => DigitalSensitivityOrigin::Implicit,
                    crate::ast::Sensitivity::Explicit(_) => DigitalSensitivityOrigin::Explicit,
                },
            }),
            TimingControl::Delay(_) => None,
        },
        _ => None,
    };

    Ok(CfgDigitalProcess {
        id: DigitalProcessId::from(usize::try_from(process.id.0).unwrap_or(usize::MAX)),
        kind,
        function,
        static_sensitivity,
        span: process.span.into(),
    })
}

struct ProcessLowerer<'a> {
    signals: &'a [DigitalSignal],
    index: &'a HashMap<&'a str, DigitalSignalId>,
    builder: SsaBuilder,
    diagnostics: Vec<IrDiagnostic>,
}

impl ProcessLowerer<'_> {
    fn error(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            message,
            SourceSpanRef::from(span),
        ));
    }

    fn width_of(&self, signal: DigitalSignalId) -> u32 {
        self.signals
            .get(usize::from(signal))
            .map_or(1, |signal| signal.width)
    }

    // ------------------------------------------------------------------
    // Statements
    // ------------------------------------------------------------------

    /// Lower `statement` starting in `block`; returns the block execution
    /// continues in, which differs from `block` whenever the statement
    /// branched or suspended.
    fn statement(&mut self, block: BlockId, statement: &DigitalStatement) -> BlockId {
        match statement {
            DigitalStatement::Null(_) => block,
            DigitalStatement::Block(inner) => {
                if let Some(declaration) = inner.variables.first() {
                    self.error(
                        "a block-local `integer` or `real` declaration inside a \
                         process has no lowered form yet",
                        declaration.span,
                    );
                }
                let mut current = block;
                for statement in &inner.statements {
                    current = self.statement(current, statement);
                }
                current
            }
            DigitalStatement::BlockingAssign(assign) => self.assign(block, assign, false),
            DigitalStatement::NonblockingAssign(assign) => self.assign(block, assign, true),
            DigitalStatement::Conditional(conditional) => {
                let condition = self.condition(block, &conditional.condition);
                let then_entry = self.builder.create_block();
                let else_entry = self.builder.create_block();
                let join = self.builder.create_block();
                self.builder.set_terminator(
                    block,
                    CfgTerminator::Branch {
                        condition,
                        then_target: then_entry,
                        then_args: Vec::new(),
                        else_target: else_entry,
                        else_args: Vec::new(),
                    },
                );
                let then_exit = self.statement(then_entry, &conditional.then_branch);
                self.builder.set_terminator(
                    then_exit,
                    CfgTerminator::Jump {
                        target: join,
                        args: Vec::new(),
                    },
                );
                let else_exit = match &conditional.else_branch {
                    Some(branch) => self.statement(else_entry, branch),
                    None => else_entry,
                };
                self.builder.set_terminator(
                    else_exit,
                    CfgTerminator::Jump {
                        target: join,
                        args: Vec::new(),
                    },
                );
                join
            }
            DigitalStatement::Case(case) => self.case(block, case),
            DigitalStatement::Timing(timing) => {
                let resume = self.wait(block, &timing.control, timing.statement.as_deref());
                match &timing.statement {
                    Some(statement) => self.statement(resume, statement),
                    None => resume,
                }
            }
            DigitalStatement::Forever(forever) => {
                // A `forever` body is entered once and re-entered from its own
                // exit. Whatever follows it in source is unreachable, so the
                // continuation block is fresh and never jumped to — the graph
                // says the statement does not fall through, which is true.
                let body = self.builder.create_block();
                self.builder.set_terminator(
                    block,
                    CfgTerminator::Jump {
                        target: body,
                        args: Vec::new(),
                    },
                );
                let exit = self.statement(body, &forever.body);
                self.builder.set_terminator(
                    exit,
                    CfgTerminator::Jump {
                        target: body,
                        args: Vec::new(),
                    },
                );
                self.builder.create_block()
            }
            DigitalStatement::While(statement) => {
                let header = self.builder.create_block();
                let body = self.builder.create_block();
                let exit = self.builder.create_block();
                self.builder.set_terminator(
                    block,
                    CfgTerminator::Jump {
                        target: header,
                        args: Vec::new(),
                    },
                );
                let condition = self.condition(header, &statement.condition);
                self.builder.set_terminator(
                    header,
                    CfgTerminator::Branch {
                        condition,
                        then_target: body,
                        then_args: Vec::new(),
                        else_target: exit,
                        else_args: Vec::new(),
                    },
                );
                let body_exit = self.statement(body, &statement.body);
                self.builder.set_terminator(
                    body_exit,
                    CfgTerminator::Jump {
                        target: header,
                        args: Vec::new(),
                    },
                );
                exit
            }
            DigitalStatement::For(statement) => {
                self.error(
                    "a `for` statement inside a process has no lowered form yet: \
                     its loop counter is a process-local variable",
                    statement.span,
                );
                block
            }
            DigitalStatement::Repeat(statement) => {
                self.error(
                    "a `repeat` statement inside a process has no lowered form yet: \
                     its iteration counter is a process-local variable",
                    statement.span,
                );
                block
            }
        }
    }

    /// Lower a `case`, `casez`, or `casex` as a chain of equality tests.
    ///
    /// Wildcard matching is not folded in here. `casez` and `casex` compare
    /// against a mask, which is a different operator from `==`, and this wave
    /// has no node for it — so they refuse rather than silently behaving as
    /// `case`, which would match on `x` where the author asked not to.
    fn case(&mut self, block: BlockId, case: &DigitalCase) -> BlockId {
        if !matches!(case.kind, crate::ast::CaseKind::Exact) {
            self.error(
                format!(
                    "`{}` has no lowered form yet: wildcard matching is a distinct \
                     operator from `==` and lowering it as one would match where \
                     the author asked not to",
                    case.kind.keyword()
                ),
                case.span,
            );
            return block;
        }

        let selector = self.expression(block, &case.selector);
        let join = self.builder.create_block();
        let mut current = block;

        for item in &case.items {
            let mut matched: Option<ValueId> = None;
            for label in &item.labels {
                let label_value = self.expression(current, label);
                let test = self.builder.push(
                    current,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalEquality {
                        left: selector,
                        right: label_value,
                        negate: false,
                    },
                );
                matched = Some(match matched {
                    None => test,
                    Some(previous) => self.builder.push(
                        current,
                        CfgValueType::FourState { width: 1 },
                        CfgValueKind::DigitalLogical {
                            op: LogicalOp::Or,
                            left: previous,
                            right: test,
                        },
                    ),
                });
            }
            let Some(matched) = matched else {
                continue;
            };
            let arm = self.builder.create_block();
            let next = self.builder.create_block();
            self.builder.set_terminator(
                current,
                CfgTerminator::Branch {
                    condition: matched,
                    then_target: arm,
                    then_args: Vec::new(),
                    else_target: next,
                    else_args: Vec::new(),
                },
            );
            let arm_exit = self.statement(arm, &item.statement);
            self.builder.set_terminator(
                arm_exit,
                CfgTerminator::Jump {
                    target: join,
                    args: Vec::new(),
                },
            );
            current = next;
        }

        let default_exit = match &case.default {
            Some(statement) => self.statement(current, statement),
            None => current,
        };
        self.builder.set_terminator(
            default_exit,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        join
    }

    /// Lower one assignment, blocking or nonblocking.
    ///
    /// An intra-assignment timing control (`q <= #5 d`) evaluates the
    /// right-hand side *before* suspending, per IEEE 1364-2005 section 9.2.2 —
    /// which is why the value node is emitted into the current block and only
    /// the write lands after the wait.
    fn assign(&mut self, block: BlockId, assign: &DigitalAssign, nonblocking: bool) -> BlockId {
        let value = self.expression(block, &assign.value);
        let block = match &assign.timing {
            Some(control) => self.wait(block, control, None),
            None => block,
        };
        self.write(block, &assign.target, value, nonblocking);
        block
    }

    /// Emit the write nodes for one target.
    ///
    /// A concatenation target becomes one write per element, over slices of
    /// the right-hand side taken from the most significant end down, which is
    /// what `{carry, sum} = ...` means.
    fn write(&mut self, block: BlockId, target: &DigitalLValue, value: ValueId, nonblocking: bool) {
        match target {
            DigitalLValue::Concat { elements, .. } => {
                let widths: Vec<u32> = elements
                    .iter()
                    .map(|part| self.lvalue_width(part))
                    .collect();
                let mut offset: u32 = widths.iter().sum();
                for (element, width) in elements.iter().zip(widths) {
                    offset -= width;
                    let slice = self.builder.push(
                        block,
                        CfgValueType::FourState { width },
                        CfgValueKind::DigitalPartSelect {
                            input: value,
                            msb: i64::from(offset + width - 1),
                            lsb: i64::from(offset),
                        },
                    );
                    self.write(block, element, slice, nonblocking);
                }
            }
            _ => {
                let Some(resolved) = self.write_target(target) else {
                    return;
                };
                let kind = if nonblocking {
                    CfgValueKind::DigitalNonblockingWrite {
                        target: resolved,
                        value,
                        region: DigitalSchedulingRegion::NonBlockingAssign,
                    }
                } else {
                    CfgValueKind::DigitalBlockingWrite {
                        target: resolved,
                        value,
                    }
                };
                self.builder.push(block, CfgValueType::Effect, kind);
            }
        }
    }

    fn write_target(&mut self, target: &DigitalLValue) -> Option<DigitalWriteTarget> {
        let (name, span, select) = match target {
            DigitalLValue::Identifier { name, span } => (name, *span, DigitalWriteSelect::Whole),
            DigitalLValue::BitSelect { name, index, span } => {
                let index = self.constant_index(index)?;
                (name, *span, DigitalWriteSelect::Bit(index))
            }
            DigitalLValue::PartSelect {
                name,
                msb,
                lsb,
                span,
            } => {
                let msb = self.constant_index(msb)?;
                let lsb = self.constant_index(lsb)?;
                (name, *span, DigitalWriteSelect::Part { msb, lsb })
            }
            DigitalLValue::Concat { .. } => unreachable!("a concatenation is split before here"),
        };
        match self.index.get(name.as_str()) {
            Some(signal) => Some(DigitalWriteTarget {
                signal: *signal,
                select,
            }),
            None => {
                self.error(format!("`{name}` is not a discrete-domain signal"), span);
                None
            }
        }
    }

    fn lvalue_width(&mut self, target: &DigitalLValue) -> u32 {
        match target {
            DigitalLValue::Identifier { name, .. } => self
                .index
                .get(name.as_str())
                .map_or(1, |signal| self.width_of(*signal)),
            DigitalLValue::BitSelect { .. } => 1,
            DigitalLValue::PartSelect { msb, lsb, .. } => {
                match (constant_of(msb), constant_of(lsb)) {
                    (Some(msb), Some(lsb)) => msb.abs_diff(lsb) as u32 + 1,
                    _ => 1,
                }
            }
            DigitalLValue::Concat { elements, .. } => {
                elements.iter().map(|part| self.lvalue_width(part)).sum()
            }
        }
    }

    /// Lower a timing control into a `Wait` and return the resume block.
    fn wait(
        &mut self,
        block: BlockId,
        control: &TimingControl,
        guarded: Option<&DigitalStatement>,
    ) -> BlockId {
        let resume = self.builder.create_block();
        let wait = match control {
            TimingControl::Event(event) => {
                let terms = self.sensitivity_terms(&event.sensitivity, guarded, event.span);
                DigitalWait::Event(terms)
            }
            TimingControl::Delay(delay) => {
                let value = self.delay(&delay.value);
                DigitalWait::Delay(value)
            }
        };
        self.builder.set_terminator(
            block,
            CfgTerminator::Wait {
                wait,
                resume,
                resume_args: Vec::new(),
            },
        );
        resume
    }

    /// Lower a delay operand, which is an integer number of time units.
    ///
    /// A leaf, not a block instruction: a constant delay reads nothing, and
    /// the `Wait` that consumes it is the terminator of a block it would
    /// otherwise have to be placed in.
    fn delay(&mut self, expression: &Expression) -> ValueId {
        let value = match constant_of(expression) {
            Some(value) => i32::try_from(value).unwrap_or(i32::MAX),
            None => {
                self.error(
                    "a delay must be a constant number of time units in this wave",
                    expression.span(),
                );
                0
            }
        };
        self.builder
            .push_leaf(CfgValueType::Integer, CfgValueKind::IntegerConstant(value))
    }

    /// Resolve a sensitivity list to signal terms.
    ///
    /// `@*` is computed here from the guarded statement's read set, per IEEE
    /// 1364-2005 section 9.7.5. The front end deliberately does not
    /// materialize it: doing so needs the statement, and a stale copy stored
    /// beside the source would be worse than none.
    fn sensitivity_terms(
        &mut self,
        sensitivity: &crate::ast::Sensitivity,
        guarded: Option<&DigitalStatement>,
        span: Span,
    ) -> Vec<DigitalSensitivityTerm> {
        match sensitivity {
            crate::ast::Sensitivity::Explicit(terms) => terms
                .iter()
                .filter_map(|term| {
                    let name = signal_name(&term.signal)?;
                    let signal = self.index.get(name)?;
                    Some(DigitalSensitivityTerm {
                        signal: *signal,
                        edge: term.edge.map(|edge| match edge {
                            EdgeKind::Posedge => DigitalEdge::Posedge,
                            EdgeKind::Negedge => DigitalEdge::Negedge,
                        }),
                    })
                })
                .collect(),
            crate::ast::Sensitivity::Implicit => {
                let mut reads = BTreeSet::new();
                if let Some(statement) = guarded {
                    collect_reads(statement, &mut reads);
                }
                if reads.is_empty() {
                    self.error(
                        "`@*` names no signal: the statement it guards reads none, \
                         so the process could never resume",
                        span,
                    );
                }
                reads
                    .into_iter()
                    .filter_map(|name| self.index.get(name.as_str()).copied())
                    .map(|signal| DigitalSensitivityTerm { signal, edge: None })
                    .collect()
            }
        }
    }

    // ------------------------------------------------------------------
    // Expressions
    // ------------------------------------------------------------------

    fn value_width(&self, value: ValueId) -> u32 {
        self.builder
            .value_type_of(value)
            .and_then(CfgValueType::width)
            .unwrap_or(1)
    }

    /// Lower an expression used as a branch condition.
    ///
    /// The CFG's `Branch` reads a truth value, so a wider four-state value is
    /// reduced to one bit here rather than at every branch site.
    fn condition(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        let value = self.expression(block, expression);
        if self.value_width(value) == 1 {
            return value;
        }
        // `!!x` is the standard reduction to a truth value: the inner `!`
        // collapses the width and the outer one restores the sense.
        let negated = self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogicalNot { input: value },
        );
        self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogicalNot { input: negated },
        )
    }

    fn expression(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        match expression {
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                let value = FourStateValue::from_literal(&literal.value);
                let width = value.width();
                self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(value),
                )
            }
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
                let input = self.named_value(block, &select.name, select.span);
                let msb = self.constant_index(&select.msb).unwrap_or(0);
                let lsb = self.constant_index(&select.lsb).unwrap_or(0);
                let width = msb.abs_diff(lsb) as u32 + 1;
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalPartSelect { input, msb, lsb },
                )
            }
            Expression::Number(number) => {
                // IEEE 1364-2005 section 3.5.1 gives an unsized literal at
                // least 32 bits, and this front end has no context width to
                // narrow it with.
                let width = 32;
                let bits = if number.value < 0.0 || number.value.fract() != 0.0 {
                    self.error(
                        "only a non-negative whole number is a discrete-domain \
                         literal in this wave",
                        number.span,
                    );
                    0
                } else {
                    number.value as u64
                };
                self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(FourStateValue::from_u64(width, bits)),
                )
            }
            Expression::Identifier(identifier) => {
                self.named_value(block, &identifier.name, identifier.span)
            }
            Expression::ArrayAccess(access) => {
                let input = self.named_value(block, &access.array, access.span);
                let index = self.constant_index(&access.index).unwrap_or(0);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalPartSelect {
                        input,
                        msb: index,
                        lsb: index,
                    },
                )
            }
            Expression::ArrayLiteral(literal) => {
                let mut parts = Vec::new();
                for element in &literal.elements {
                    self.concat_element(block, element, &mut parts);
                }
                let width = parts.iter().map(|part| self.value_width(*part)).sum();
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalConcat { parts },
                )
            }
            Expression::Conditional(conditional) => {
                let condition = self.condition(block, &conditional.condition);
                let then_value = self.expression(block, &conditional.then_expr);
                let else_value = self.expression(block, &conditional.else_expr);
                let width = self
                    .value_width(then_value)
                    .max(self.value_width(else_value));
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalSelect {
                        condition,
                        then_value,
                        else_value,
                    },
                )
            }
            Expression::Unary(unary) => self.unary(block, unary),
            Expression::Binary(binary) => self.binary(block, binary),
            other => {
                self.error(
                    "this expression form has no discrete-domain lowering",
                    other.span(),
                );
                self.unknown(1)
            }
        }
    }

    fn concat_element(
        &mut self,
        block: BlockId,
        element: &ArrayLiteralElement,
        parts: &mut Vec<ValueId>,
    ) {
        match element {
            ArrayLiteralElement::Value(expression) => {
                parts.push(self.expression(block, expression));
            }
            ArrayLiteralElement::Replication(replication) => {
                // IEEE 1364-2005 section 4.1.14 requires a constant
                // replication count, so the repetition is expanded here and
                // the IR needs no replication node.
                let Some(count) = constant_of(&replication.count).filter(|count| *count >= 0)
                else {
                    self.error(
                        "a replication count must be a non-negative constant",
                        replication.span,
                    );
                    return;
                };
                for _ in 0..count {
                    for element in &replication.elements {
                        self.concat_element(block, element, parts);
                    }
                }
            }
        }
    }

    fn unary(&mut self, block: BlockId, unary: &crate::ast::UnaryExpr) -> ValueId {
        let input = self.expression(block, &unary.operand);
        let width = self.value_width(input);
        let (result_width, kind) = match unary.op {
            UnaryOp::Not => (1, CfgValueKind::DigitalLogicalNot { input }),
            UnaryOp::BitNot => (width, CfgValueKind::DigitalBitwiseNot { input }),
            UnaryOp::Pos => return input,
            UnaryOp::Neg => {
                // `-x` is `0 - x` at the operand width, which is what makes it
                // wrap rather than go negative.
                let zero = self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(FourStateValue::zero(width)),
                );
                (
                    width,
                    CfgValueKind::DigitalArithmetic {
                        op: ArithmeticOp::Sub,
                        left: zero,
                        right: input,
                    },
                )
            }
        };
        self.builder.push(
            block,
            CfgValueType::FourState {
                width: result_width,
            },
            kind,
        )
    }

    fn binary(&mut self, block: BlockId, binary: &crate::ast::BinaryExpr) -> ValueId {
        let left = self.expression(block, &binary.left);
        let right = self.expression(block, &binary.right);
        let widest = self.value_width(left).max(self.value_width(right));
        let (width, kind) = match binary.op {
            BinaryOp::BitAnd => (
                widest,
                CfgValueKind::DigitalBitwise {
                    op: BitwiseOp::And,
                    left,
                    right,
                },
            ),
            BinaryOp::BitOr => (
                widest,
                CfgValueKind::DigitalBitwise {
                    op: BitwiseOp::Or,
                    left,
                    right,
                },
            ),
            BinaryOp::BitXor => (
                widest,
                CfgValueKind::DigitalBitwise {
                    op: BitwiseOp::Xor,
                    left,
                    right,
                },
            ),
            BinaryOp::And => (
                1,
                CfgValueKind::DigitalLogical {
                    op: LogicalOp::And,
                    left,
                    right,
                },
            ),
            BinaryOp::Or => (
                1,
                CfgValueKind::DigitalLogical {
                    op: LogicalOp::Or,
                    left,
                    right,
                },
            ),
            BinaryOp::Eq => (
                1,
                CfgValueKind::DigitalEquality {
                    left,
                    right,
                    negate: false,
                },
            ),
            BinaryOp::Ne => (
                1,
                CfgValueKind::DigitalEquality {
                    left,
                    right,
                    negate: true,
                },
            ),
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                let op = match binary.op {
                    BinaryOp::Lt => RelationalOp::Lt,
                    BinaryOp::Le => RelationalOp::Le,
                    BinaryOp::Gt => RelationalOp::Gt,
                    _ => RelationalOp::Ge,
                };
                (1, CfgValueKind::DigitalRelational { op, left, right })
            }
            BinaryOp::Shl | BinaryOp::Shr => {
                let op = if matches!(binary.op, BinaryOp::Shl) {
                    ShiftOp::Left
                } else {
                    ShiftOp::Right
                };
                (
                    self.value_width(left),
                    CfgValueKind::DigitalShift {
                        op,
                        value: left,
                        count: right,
                    },
                )
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                let op = match binary.op {
                    BinaryOp::Add => ArithmeticOp::Add,
                    BinaryOp::Sub => ArithmeticOp::Sub,
                    BinaryOp::Mul => ArithmeticOp::Mul,
                    BinaryOp::Div => ArithmeticOp::Div,
                    _ => ArithmeticOp::Mod,
                };
                (widest, CfgValueKind::DigitalArithmetic { op, left, right })
            }
            BinaryOp::Pow => {
                self.error(
                    "`**` has no discrete-domain lowering in this wave",
                    binary.span,
                );
                return self.unknown(widest);
            }
        };
        self.builder
            .push(block, CfgValueType::FourState { width }, kind)
    }

    fn named_value(&mut self, block: BlockId, name: &str, span: Span) -> ValueId {
        match self.index.get(name) {
            Some(signal) => {
                let width = self.width_of(*signal);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalSignalRead { signal: *signal },
                )
            }
            None => {
                self.error(
                    format!(
                        "`{name}` is not a discrete-domain signal; reading an analog \
                         variable from a process has no lowered form yet"
                    ),
                    span,
                );
                self.unknown(1)
            }
        }
    }

    /// A placeholder for an expression that failed to lower.
    ///
    /// Lowering continues after an error so that a second one is reported in
    /// the same pass; the value is all-`x` so that anything built on it is
    /// visibly unknown rather than accidentally plausible.
    fn unknown(&mut self, width: u32) -> ValueId {
        self.builder.push_leaf(
            CfgValueType::FourState { width },
            CfgValueKind::FourStateConstant(FourStateValue::splat(width, FourStateBit::Unknown)),
        )
    }

    fn constant_index(&mut self, expression: &Expression) -> Option<i64> {
        match constant_of(expression) {
            Some(index) => Some(index),
            None => {
                self.error(
                    "a bit or part select must have constant bounds in this wave",
                    expression.span(),
                );
                None
            }
        }
    }
}

/// The constant value of an expression, when it has one.
fn constant_of(expression: &Expression) -> Option<i64> {
    match expression {
        Expression::Number(number) if number.value.fract() == 0.0 => Some(number.value as i64),
        Expression::Unary(unary) if matches!(unary.op, UnaryOp::Neg) => {
            constant_of(&unary.operand).map(|value| -value)
        }
        Expression::Unary(unary) if matches!(unary.op, UnaryOp::Pos) => constant_of(&unary.operand),
        _ => None,
    }
}

/// The signal an event term names, if it names one directly.
fn signal_name(expression: &Expression) -> Option<&str> {
    match expression {
        Expression::Identifier(identifier) => Some(identifier.name.as_str()),
        _ => None,
    }
}

/// Every signal name a statement reads, for `@*`.
///
/// IEEE 1364-2005 section 9.7.5: the implicit list is what the statement
/// reads, and a name that is only written does not appear. That asymmetry is
/// the whole reason the rule exists — an assignment target that triggered its
/// own process would never settle.
fn collect_reads(statement: &DigitalStatement, reads: &mut BTreeSet<String>) {
    match statement {
        DigitalStatement::Null(_) => {}
        DigitalStatement::Block(block) => {
            for statement in &block.statements {
                collect_reads(statement, reads);
            }
        }
        DigitalStatement::BlockingAssign(assign) | DigitalStatement::NonblockingAssign(assign) => {
            collect_expression_reads(&assign.value, reads);
            // A select's *index* is read even though the target is written.
            collect_lvalue_index_reads(&assign.target, reads);
        }
        DigitalStatement::Conditional(conditional) => {
            collect_expression_reads(&conditional.condition, reads);
            collect_reads(&conditional.then_branch, reads);
            if let Some(branch) = &conditional.else_branch {
                collect_reads(branch, reads);
            }
        }
        DigitalStatement::Case(case) => {
            collect_expression_reads(&case.selector, reads);
            for item in &case.items {
                for label in &item.labels {
                    collect_expression_reads(label, reads);
                }
                collect_reads(&item.statement, reads);
            }
            if let Some(default) = &case.default {
                collect_reads(default, reads);
            }
        }
        DigitalStatement::For(statement) => {
            collect_expression_reads(&statement.condition, reads);
            collect_reads(
                &DigitalStatement::BlockingAssign((*statement.init).clone()),
                reads,
            );
            collect_reads(
                &DigitalStatement::BlockingAssign((*statement.update).clone()),
                reads,
            );
            collect_reads(&statement.body, reads);
        }
        DigitalStatement::While(statement) => {
            collect_expression_reads(&statement.condition, reads);
            collect_reads(&statement.body, reads);
        }
        DigitalStatement::Repeat(statement) => {
            collect_expression_reads(&statement.count, reads);
            collect_reads(&statement.body, reads);
        }
        DigitalStatement::Forever(statement) => collect_reads(&statement.body, reads),
        DigitalStatement::Timing(timing) => {
            if let Some(statement) = &timing.statement {
                collect_reads(statement, reads);
            }
        }
    }
}

fn collect_lvalue_index_reads(target: &DigitalLValue, reads: &mut BTreeSet<String>) {
    match target {
        DigitalLValue::Identifier { .. } => {}
        DigitalLValue::BitSelect { index, .. } => collect_expression_reads(index, reads),
        DigitalLValue::PartSelect { msb, lsb, .. } => {
            collect_expression_reads(msb, reads);
            collect_expression_reads(lsb, reads);
        }
        DigitalLValue::Concat { elements, .. } => {
            for element in elements {
                collect_lvalue_index_reads(element, reads);
            }
        }
    }
}

fn collect_expression_reads(expression: &Expression, reads: &mut BTreeSet<String>) {
    match expression {
        Expression::Identifier(identifier) => {
            reads.insert(identifier.name.to_string());
        }
        Expression::ArrayAccess(access) => {
            reads.insert(access.array.to_string());
            collect_expression_reads(&access.index, reads);
        }
        Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
            reads.insert(select.name.to_string());
            collect_expression_reads(&select.msb, reads);
            collect_expression_reads(&select.lsb, reads);
        }
        Expression::Binary(binary) => {
            collect_expression_reads(&binary.left, reads);
            collect_expression_reads(&binary.right, reads);
        }
        Expression::Unary(unary) => collect_expression_reads(&unary.operand, reads),
        Expression::Conditional(conditional) => {
            collect_expression_reads(&conditional.condition, reads);
            collect_expression_reads(&conditional.then_expr, reads);
            collect_expression_reads(&conditional.else_expr, reads);
        }
        Expression::ArrayLiteral(literal) => {
            for element in &literal.elements {
                match element {
                    ArrayLiteralElement::Value(expression) => {
                        collect_expression_reads(expression, reads);
                    }
                    ArrayLiteralElement::Replication(replication) => {
                        collect_expression_reads(&replication.count, reads);
                        for element in &replication.elements {
                            match element {
                                ArrayLiteralElement::Value(expression) => {
                                    collect_expression_reads(expression, reads);
                                }
                                ArrayLiteralElement::Replication(_) => {}
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
