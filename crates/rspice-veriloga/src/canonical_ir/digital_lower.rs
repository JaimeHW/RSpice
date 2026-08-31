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
//! A module-level `reg` or `wire` is never an SSA value. It is a signal, read
//! by a [`CfgValueKind::DigitalSignalRead`] node and written by a write node.
//! That is not a simplification of the model — it *is* the model. A process
//! that suspends and resumes must see whatever the signal holds when it wakes,
//! and a value carried across the suspension in a register would see what it
//! held when the process went to sleep.
//!
//! A variable declared *inside* the process is the opposite case, and the two
//! must not be confused. It belongs to the process, nothing else can observe
//! it, and it is an ordinary SSA variable: merged at joins by a block
//! parameter, carried across a suspension by a resume argument. IEEE 1364-2005
//! section 9.8.1 makes such a variable static; the reading frozen here is
//! automatic — it starts at `x` each time control enters the block that
//! declares it — because the difference is observable only by reading one
//! before writing it, and the case that matters is a loop counter.
//!
//! Everything crossing a `Wait` crosses as a resume argument: every in-scope
//! local, plus the right-hand side of an intra-assignment timing control
//! (`q <= #5 d`, section 9.2.2), whose value is read before the suspension and
//! written after it. Nothing else survives — the interpreter starts a
//! resumption with an empty value table, which is what proves the lowering
//! routed the state through the terminator rather than assuming a register
//! kept it.
//!
//! # The subset
//!
//! Everything the front end parses is not everything this lowers. Refusals are
//! by name and with a span, so a model that crosses one is told what is
//! missing rather than compiled into a device that is quietly short of what
//! its author wrote. What still refuses here:
//!
//! - A module-level analog `integer` or `real` read from a process. It is
//!   shared with the continuous-domain body, which runs on a different clock
//!   in a different value domain; a process-local declaration is the lowered
//!   form of the same intent.
//! - A process-local `real` or `string`, for the same reason from the other
//!   side: a process computes in four-state values.
//! - A nonblocking assignment to a process-local, which would need a store to
//!   defer the update into, and a partial (bit- or part-select) write to one,
//!   which is a read-modify-write this wave has no node for outside the signal
//!   store.
//! - A process-local `reg` whose bounds are not literal, and an array of any
//!   kind inside a process.
//! - `**`, a non-constant delay, and a non-constant select bound.
//! - Anything carrying the `signed` marker: a net, variable or port declared
//!   `signed`, and a literal written with the `s` base marker. IEEE 1364-2005
//!   section 5.4.2's signed semantics are not implemented, and this front end
//!   computes on unsigned values throughout — so lowering a `signed`
//!   declaration would produce a device that sign-extends nothing, compares
//!   unsigned, and shifts in zeros, which is a wrong answer rather than a
//!   missing one.
//!
//! Refused before this pass, and still refused: tasks and functions,
//! `fork`/`join`, `wait`, `disable`, and `force`/`release` — the parser stops
//! each on its own keyword, so none of them reaches a lowering decision.
//!
//! Nor does a generate region. IEEE 1364-2005 section 12.4 makes one an
//! elaboration-time construct, and the parser unrolls it into ordinary module
//! items at `endmodule`, so what arrives here is the flat result and there is
//! no generate anything to lower.
//!
//! # Hierarchy
//!
//! A hierarchical instance is no longer among them. It is resolved before this
//! pass by [`digital_elaborate`](crate::semantic::digital_elaborate), which
//! turns the instance tree into a flat list of frames, and this pass turns the
//! frames into one plan: one signal table, one process list, one driver list.
//! Nothing downstream sees a hierarchy — the interpreter is unchanged, and the
//! event kernel that follows it will be too.
//!
//! What survives flattening is identity. Each frame is lowered against its own
//! scope with its own freshly allocated process ids, so two instances of one
//! module are two sets of processes a scheduler can resume individually, and
//! two sets of drivers a resolver can tell apart, rather than one body lowered
//! twice.

use super::cfg::{CfgTerminator, CfgValueKind, CfgValueType, CfgVariable, DigitalWait, SsaBuilder};
use super::diagnostic::{CompilerPhase, IrDiagnostic, SourceSpanRef};
use super::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalDriver, DigitalDriverId, DigitalEdge,
    DigitalProcessKind, DigitalRealResolution, DigitalSchedulingRegion, DigitalSensitivityOrigin,
    DigitalSensitivityTerm, DigitalSignal, DigitalSignalKind, DigitalStaticSensitivity,
    DigitalWriteSelect, DigitalWriteTarget,
};
use super::digital_value::{
    ArithmeticOp, BitwiseOp, DigitalCaseMatch, FourStateValue, LogicalOp, RelationalOp, ShiftOp,
};
use super::ids::{BlockId, DigitalLocalId, DigitalProcessId, DigitalSignalId, ValueId};
use crate::ast::DigitalProcessKind as AstKind;
use crate::ast::{
    ArrayLiteralElement, BinaryOp, DigitalAssign, DigitalCase, DigitalLValue, DigitalStatement,
    EdgeKind, Expression, ReductionOp, TimingControl, UnaryOp, WrealResolution,
};
use crate::four_state::FourStateBit;
use crate::semantic::{AnalyzedDigital, AnalyzedDigitalProcess, AnalyzedDigitalSignal};
use crate::source::Span;
use smol_str::SmolStr;
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

    // ------------------------------------------------------------------
    // The elaborated signal table.
    //
    // Built whole before anything is lowered, because a process function
    // carries signal ids and a scope that hands out ids has to know all of
    // them. The compiled module's own signals keep their names and their
    // positions, so a design with no hierarchy produces exactly the table it
    // always did. Each instance frame then contributes the signals it declares
    // under the elaborated name the front end gave them — except a net port
    // that collapsed onto the net it connects to, which the front end named
    // after *that* net and which is therefore already in the table. Reusing
    // the entry is what collapsing is; there is no separate merge step.
    // ------------------------------------------------------------------
    let mut signals = lower_signals(&digital.signals);
    let mut elaborated: HashMap<SmolStr, DigitalSignalId> = signals
        .iter()
        .map(|signal| (signal.name.clone(), signal.id))
        .collect();
    let mut frame_signal_ids: Vec<Vec<DigitalSignalId>> =
        Vec::with_capacity(digital.instances.len());
    for instance in &digital.instances {
        let mut ids = Vec::with_capacity(instance.signals.len());
        for signal in &instance.signals {
            let id = match elaborated.get(&signal.name) {
                Some(existing) => *existing,
                None => {
                    let id = DigitalSignalId::from(signals.len());
                    signals.push(lower_signal(&signal.declared, id, signal.name.clone()));
                    elaborated.insert(signal.name.clone(), id);
                    id
                }
            };
            ids.push(id);
        }
        frame_signal_ids.push(ids);
    }

    // One scope per frame, each mapping the names *that frame's* body writes to
    // the elaborated ids they resolve to. The instance's body is lowered
    // unmodified against its own scope, which is what keeps two instances of
    // one module two separately addressable things rather than one body
    // rewritten twice.
    let module_scope: HashMap<&str, DigitalSignalId> = digital
        .signals
        .iter()
        .zip(&signals)
        .map(|(analyzed, signal)| (analyzed.name.as_str(), signal.id))
        .collect();
    let frame_scopes: Vec<HashMap<&str, DigitalSignalId>> = digital
        .instances
        .iter()
        .zip(&frame_signal_ids)
        .map(|(instance, ids)| {
            instance
                .signals
                .iter()
                .zip(ids)
                .map(|(signal, id)| (signal.declared.name.as_str(), *id))
                .collect()
        })
        .collect();
    // The scope an implicit port driver resolves in. It names both sides of a
    // connection, which live in two different instances, so it is the only one
    // keyed by elaborated name.
    let elaborated_scope: HashMap<&str, DigitalSignalId> = signals
        .iter()
        .map(|signal| (signal.name.as_str(), signal.id))
        .collect();

    // ------------------------------------------------------------------
    // Processes and drivers.
    // ------------------------------------------------------------------
    // Continuous assignments become processes too, numbered after the ones the
    // front end named. An `assign` has no source-level process id — the parser
    // assigns those to `always` and `initial` only — and neither has any
    // process of an instance frame, because two instances of one module would
    // otherwise share every id their module was given. So the numbering
    // continues from the compiled module's own highest, in one fixed order:
    // the module's own processes, its own assignments, then each frame in
    // elaboration order with its processes, its assignments, and last its
    // implicit port drivers. Driver indices fall out of the same order, which
    // is what makes them stable across a recompilation.
    let mut next_id = digital
        .processes
        .iter()
        .map(|process| process.id.0)
        .max()
        .map_or(0, |highest| highest + 1);
    let mut allocate = move || {
        let id = DigitalProcessId::from(next_id as usize);
        next_id += 1;
        id
    };

    // An instance frame's body is lowered against *its own* module's constants,
    // never against the module the artifact is being built for. The two tables
    // never meet: folding a child's `WIDTH` with the parent's would be a wrong
    // answer rather than a refused one, and a child whose `{WIDTH{1'b0}}`
    // resolved to the parent's number would compile into a device silently the
    // wrong width.
    //
    // A synthesized port driver is lowered against no constants at all, because
    // it belongs to neither scope: this pass wrote it, in elaborated names, and
    // the only expressions in one are a name and a select whose bounds are
    // already literals.
    let no_constants: HashMap<SmolStr, i64> = HashMap::new();

    let mut processes = Vec::new();
    let mut drivers = Vec::new();
    for process in &digital.processes {
        let id = DigitalProcessId::from(usize::try_from(process.id.0).unwrap_or(usize::MAX));
        match lower_process(process, id, &signals, &module_scope, &digital.constants) {
            Ok(lowered) => processes.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    for assignment in &digital.continuous_assigns {
        match lower_continuous_assign(
            assignment,
            &signals,
            &module_scope,
            &digital.constants,
            allocate(),
            &mut drivers,
        ) {
            Ok(lowered) => processes.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    for (instance, scope) in digital.instances.iter().zip(&frame_scopes) {
        for process in &instance.processes {
            match lower_process(process, allocate(), &signals, scope, &instance.constants) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        for assignment in &instance.continuous_assigns {
            match lower_continuous_assign(
                assignment,
                &signals,
                scope,
                &instance.constants,
                allocate(),
                &mut drivers,
            ) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        for assignment in &instance.port_drivers {
            match lower_continuous_assign(
                assignment,
                &signals,
                &elaborated_scope,
                &no_constants,
                allocate(),
                &mut drivers,
            ) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(CanonicalDigitalPlan {
        signals,
        processes,
        drivers,
    })
}

/// Lower one continuous assignment into a driver process.
///
/// The shape is the design of a driver, so it is worth reading as one. The
/// entry block evaluates the right-hand side and publishes it as this driver's
/// contribution; the entry block *then* suspends on the operands it read. A
/// driver is active from the start of the simulation rather than from the first
/// change of an operand (IEEE 1364-2005 section 6.1), and evaluating before
/// waiting is how that is spelled in a graph.
///
/// The sensitivity is derived from the right-hand side's read set, the same
/// rule section 9.7.5 gives `@*`, and reported as
/// [`DigitalSensitivityOrigin::Implicit`] because that is what it is.
///
/// A driver with no operands — `assign y = 1'b0;` — has no list to wait on and
/// returns instead of looping. Its value cannot change, so a process that woke
/// for it would have nothing to do.
fn lower_continuous_assign(
    assignment: &crate::semantic::AnalyzedContinuousAssign,
    signals: &[DigitalSignal],
    index: &HashMap<&str, DigitalSignalId>,
    constants: &HashMap<SmolStr, i64>,
    id: DigitalProcessId,
    drivers: &mut Vec<DigitalDriver>,
) -> Result<CfgDigitalProcess, Vec<IrDiagnostic>> {
    let mut lowerer = ProcessLowerer {
        signals,
        index,
        constants,
        builder: SsaBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
    };

    if let Some(delay) = &assignment.assignment.delay {
        lowerer.error(
            "a delay on a continuous assignment has no lowered form yet: it is a \
             transport delay on the driver, which needs the kernel's timing wheel \
             rather than a suspension in the process",
            delay.span(),
        );
    }

    let entry = lowerer.builder.create_block();
    // The driven net is the assignment's left-hand side, and section 5.4.1
    // does not distinguish a continuous assignment from a procedural one:
    // `assign p = a * b;` with an eight-bit `p` multiplies at eight bits.
    let context = lowerer.lvalue_width(&assignment.assignment.target);
    let value = lowerer.assigned_value(entry, &assignment.assignment.value, context);
    lowerer.drive(entry, &assignment.assignment.target, value, id, drivers);

    let mut reads = BTreeSet::new();
    collect_expression_reads(&assignment.assignment.value, &mut reads);
    let terms: Vec<DigitalSensitivityTerm> = reads
        .into_iter()
        .filter_map(|name| index.get(name.as_str()).copied())
        .map(|signal| DigitalSensitivityTerm { signal, edge: None })
        .collect();

    let static_sensitivity = if terms.is_empty() {
        lowerer.builder.set_terminator(entry, CfgTerminator::Return);
        None
    } else {
        let resume = lowerer.builder.create_block();
        lowerer.builder.set_terminator(
            entry,
            CfgTerminator::Wait {
                wait: DigitalWait::Event(terms.clone()),
                resume,
                resume_args: Vec::new(),
            },
        );
        lowerer.builder.seal_block(resume);
        lowerer.builder.set_terminator(
            resume,
            CfgTerminator::Jump {
                target: entry,
                args: Vec::new(),
            },
        );
        Some(DigitalStaticSensitivity {
            terms,
            origin: DigitalSensitivityOrigin::Implicit,
        })
    };
    lowerer.builder.seal_all_blocks();

    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }
    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!("lowering a continuous assignment produced an invalid graph: {error}"),
            assignment.span.into(),
        )]
    })?;

    Ok(CfgDigitalProcess {
        id,
        kind: DigitalProcessKind::ContinuousAssign,
        function,
        static_sensitivity,
        span: assignment.span.into(),
    })
}

fn lower_signals(analyzed: &[AnalyzedDigitalSignal]) -> Vec<DigitalSignal> {
    analyzed
        .iter()
        .enumerate()
        .map(|(position, signal)| {
            lower_signal(signal, DigitalSignalId::from(position), signal.name.clone())
        })
        .collect()
}

/// One declaration, under the identity and name the elaborated scope gives it.
///
/// The name is a parameter rather than read off the declaration because an
/// instance's signal is named by its instance path, and a port that collapsed
/// is named by the net it joined.
fn lower_signal(
    signal: &AnalyzedDigitalSignal,
    id: DigitalSignalId,
    name: SmolStr,
) -> DigitalSignal {
    DigitalSignal {
        id,
        name,
        kind: match signal.class.wreal_resolution() {
            None => DigitalSignalKind::FourState,
            Some(resolution) => DigitalSignalKind::Real(match resolution {
                WrealResolution::Single => DigitalRealResolution::Single,
                WrealResolution::Sum => DigitalRealResolution::Sum,
                WrealResolution::Average => DigitalRealResolution::Average,
                WrealResolution::Minimum => DigitalRealResolution::Minimum,
                WrealResolution::Maximum => DigitalRealResolution::Maximum,
            }),
        },
        width: signal.width,
        bounds: signal.range.map(|range| (range.msb, range.lsb)),
        signed: signal.signedness.is_signed(),
        procedurally_assignable: signal.class.is_variable(),
        span: signal.span.into(),
    }
}

/// Lower one process under the identity the plan gives it.
///
/// `id` is the plan's, not the source's. They are the same number for a
/// process of the compiled module, and deliberately not for one of an instance
/// frame: two instances of a module have the same source process and must have
/// two identities, because a scheduler resumes one of them.
fn lower_process(
    process: &AnalyzedDigitalProcess,
    id: DigitalProcessId,
    signals: &[DigitalSignal],
    index: &HashMap<&str, DigitalSignalId>,
    constants: &HashMap<SmolStr, i64>,
) -> Result<CfgDigitalProcess, Vec<IrDiagnostic>> {
    let mut lowerer = ProcessLowerer {
        signals,
        index,
        constants,
        builder: SsaBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
    };

    let kind = match process.kind {
        AstKind::Always => DigitalProcessKind::Always,
        AstKind::Initial => DigitalProcessKind::Initial,
    };

    let entry = lowerer.builder.create_block();
    // An `initial` process's entry has no predecessors and can be sealed at
    // once. An `always` process's entry gains one when the restart edge is
    // added below, so sealing it here would decide a merge before the loop
    // exists.
    if !kind.restarts() {
        lowerer.builder.seal_block(entry);
    }
    let exit = lowerer.statement(entry, &process.body);

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
    lowerer.builder.seal_block(entry);
    // Every construct seals the blocks it creates as soon as their
    // predecessors are known. This is the backstop for the paths that stopped
    // early: a construct that refused left its blocks behind, and an unsealed
    // block reaching `finish` holds parameters whose arguments never arrived.
    lowerer.builder.seal_all_blocks();

    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }

    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!("lowering process {id} produced an invalid graph: {error}"),
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
        id,
        kind,
        function,
        static_sensitivity,
        span: process.span.into(),
    })
}

/// What a loop runs at the end of each pass.
enum LoopUpdate<'a> {
    /// A `for` loop's third clause, written by the author.
    Assign(&'a DigitalAssign),
    /// A `repeat` loop's decrement of the counter the lowering invented.
    Decrement(DigitalLocalId),
}

/// A variable that lives in the process function rather than in the signal
/// store.
struct ProcessLocal {
    /// `None` for a counter the lowering invented, which no source name can
    /// reach and which therefore cannot be shadowed or read by mistake.
    name: Option<SmolStr>,
    width: u32,
    /// Whether reading it yields a signed value, IEEE 1364-2005 table 5-21.
    ///
    /// True for an `integer`, which the table makes signed without any
    /// qualifier, and for a `reg signed`. False for a plain `reg` and for the
    /// invented `repeat` counter, which counts down to zero and is nobody's
    /// operand.
    signed: bool,
    /// Where it was declared, for a diagnostic about it.
    span: Span,
}

/// The width and signedness an enclosing expression imposes on an operand.
///
/// The two halves of IEEE 1364-2005's top-down pass, carried together because
/// section 5.5 determines them together: "the size and the signedness of the
/// expression are determined from the whole context before evaluation". Sizing
/// an operand without also deciding how it is extended answers half the
/// question, and the half it leaves out is the one that decides whether
/// `-1` reaches an eight-bit operator as -1 or as 15.
#[derive(Clone, Copy)]
struct Context {
    /// Section 5.4.1's context size. Zero asks for nothing.
    width: u32,
    /// Whether the *enclosing* expression is signed.
    ///
    /// Not a claim about the operand. Section 5.4.2 rule (j) makes an
    /// expression signed only when every one of its context-determined
    /// operands is, so this flag travels downward as a permission that is
    /// `and`ed with each operand's own classification: one unsigned operand
    /// makes the shared context unsigned, and every other operand in it is
    /// then extended and interpreted as unsigned however it was declared.
    signed: bool,
}

impl Context {
    /// The context of an expression that nothing outside sizes or signs.
    ///
    /// `signed: true` is the absence of an imposition rather than a claim:
    /// nothing outside is forcing unsignedness, so the expression's own
    /// classification stands. Every self-determined position of table 5-22
    /// uses this — a `case` selector, a branch condition, a `repeat` count, an
    /// event term, a concatenation operand, a shift count, a comparison
    /// operand, a reduction operand.
    const SELF_DETERMINED: Self = Self {
        width: 0,
        signed: true,
    };
}

struct ProcessLowerer<'a> {
    signals: &'a [DigitalSignal],
    index: &'a HashMap<&'a str, DigitalSignalId>,
    /// The elaboration-time constants a name in this body may denote.
    ///
    /// IEEE 1364-2005 section 12.2 fixes a parameter's value at elaboration, so
    /// `reg [WIDTH-1:0] q` and `{WIDTH{1'b0}}` are constant expressions. The
    /// table is the *declaring* module's, which is why it travels with the
    /// scope rather than being read out of one place: an instance frame's body
    /// is lowered against an empty table so a child's `WIDTH` can never be
    /// folded with a parent's.
    constants: &'a HashMap<SmolStr, i64>,
    builder: SsaBuilder,
    diagnostics: Vec<IrDiagnostic>,
    /// Every variable declared in the process, by id.
    locals: Vec<ProcessLocal>,
    /// Declarative regions, innermost last (IEEE 1364-2005 section 9.8.1).
    scopes: Vec<Vec<DigitalLocalId>>,
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

    /// Whether a signal was declared `signed`, IEEE 1364-2005 table 5-21.
    fn signed_signal(&self, signal: DigitalSignalId) -> bool {
        self.signals
            .get(usize::from(signal))
            .is_some_and(|signal| signal.signed)
    }

    // ------------------------------------------------------------------
    // Process-local variables
    // ------------------------------------------------------------------

    /// The local a name resolves to, innermost region first.
    ///
    /// The same order the analyzer resolves in, and for the same reason: a
    /// name that meant the local in one pass and the module signal in the
    /// other would be two compilers disagreeing about one program.
    fn lookup_local(&self, name: &str) -> Option<DigitalLocalId> {
        self.scopes.iter().rev().find_map(|scope| {
            scope
                .iter()
                .rev()
                .copied()
                .find(|id| self.locals[usize::from(*id)].name.as_deref() == Some(name))
        })
    }

    fn local_width(&self, id: DigitalLocalId) -> u32 {
        self.locals[usize::from(id)].width
    }

    fn local_signed(&self, id: DigitalLocalId) -> bool {
        self.locals[usize::from(id)].signed
    }

    /// Declare a local and give it its initial value in `block`.
    ///
    /// The initial value is written at the declaration, which makes every
    /// later read reach a definition and is what keeps the merge machinery
    /// from ever having to answer "what was this before anything set it".
    ///
    /// That models a block variable as *automatic*: it starts at `x` (IEEE
    /// 1364-2005 section 4.2.2) each time control enters the block. Section
    /// 9.8.1 makes a named block's variable static, so an `always` process that
    /// read one before writing it would see the previous pass's value; that is
    /// a read of an uninitialised variable in any case, and the reading frozen
    /// here is the one a loop counter needs.
    fn declare_local(
        &mut self,
        block: BlockId,
        name: Option<SmolStr>,
        width: u32,
        signed: bool,
        span: Span,
        initial: Option<ValueId>,
    ) -> DigitalLocalId {
        let id = DigitalLocalId::from(self.locals.len());
        self.locals.push(ProcessLocal {
            name,
            width,
            signed,
            span,
        });
        let variable = CfgVariable::DigitalLocal(id);
        self.builder
            .declare_variable(variable, CfgValueType::FourState { width });
        let initial = match initial {
            Some(value) => self.resize(block, value, width, false),
            None => self.builder.push_leaf(
                CfgValueType::FourState { width },
                CfgValueKind::FourStateConstant(FourStateValue::splat(
                    width,
                    FourStateBit::Unknown,
                )),
            ),
        };
        self.builder.write_variable(variable, block, initial);
        if let Some(scope) = self.scopes.last_mut() {
            scope.push(id);
        }
        id
    }

    /// Read a local's current value in `block`.
    ///
    /// A local always has a definition — the declaration wrote one — so a
    /// miss is a lowering bug rather than a program error, and it reports as
    /// one instead of producing a value nothing defined.
    fn read_local(&mut self, block: BlockId, id: DigitalLocalId) -> ValueId {
        let variable = CfgVariable::DigitalLocal(id);
        match self.builder.read_variable(variable, block) {
            Some(value) => value,
            None => {
                let local = &self.locals[usize::from(id)];
                let (width, span) = (local.width, local.span);
                let name = local.name.clone();
                self.error(
                    format!(
                        "process-local `{}` is read on a path that never defines it",
                        name.as_deref().unwrap_or("<loop counter>")
                    ),
                    span,
                );
                self.unknown(width)
            }
        }
    }

    /// Write a local, resizing per IEEE 1364-2005 section 5.2.1.
    ///
    /// Zero-fill, and correct because it is never reached with a value that
    /// needed the other kind: an assignment's right-hand side is sign-extended
    /// to the target's width by [`Self::assigned_value`] before it gets here,
    /// so a narrower value arriving at this point is an unsigned one.
    fn write_local(&mut self, block: BlockId, id: DigitalLocalId, value: ValueId) {
        let width = self.local_width(id);
        let value = self.resize(block, value, width, false);
        self.builder
            .write_variable(CfgVariable::DigitalLocal(id), block, value);
    }

    /// `counter = counter - 1` at the counter's own width, which is what makes
    /// a `repeat` of `2'b00` passes stop rather than wrap forever.
    fn decrement(&mut self, block: BlockId, id: DigitalLocalId) {
        let width = self.local_width(id);
        let current = self.read_local(block, id);
        let one = self.builder.push_leaf(
            CfgValueType::FourState { width },
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(width, 1)),
        );
        let next = self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalArithmetic {
                op: ArithmeticOp::Sub,
                left: current,
                right: one,
                // The counter is the lowering's own, counts down to zero, and
                // is nobody's operand; unsigned is what "how many passes are
                // left" means.
                signed: false,
            },
        );
        self.builder
            .write_variable(CfgVariable::DigitalLocal(id), block, next);
    }

    /// Every local currently in scope, outermost region first.
    ///
    /// The order is declaration order, which is what makes the parameter list
    /// of a resume block reproducible across runs.
    fn locals_in_scope(&self) -> Vec<DigitalLocalId> {
        self.scopes.iter().flatten().copied().collect()
    }

    /// Lower the declarations of one `begin`/`end` block.
    fn declare_block_locals(&mut self, block: BlockId, inner: &crate::ast::DigitalBlock) {
        for declaration in &inner.variables {
            let width = match declaration.var_type {
                // IEEE 1364-2005 section 3.9: an `integer` is a 32-bit
                // variable. It is four-state here rather than the IR's own
                // `Integer`, which has no `x` — and section 4.2.2 gives an
                // unwritten `integer` exactly that.
                crate::ast::VarType::Integer => 32,
                crate::ast::VarType::Real | crate::ast::VarType::String => {
                    self.error(
                        format!(
                            "a process-local `{}` has no lowered form yet: a process \
                             computes in four-state values",
                            if matches!(declaration.var_type, crate::ast::VarType::Real) {
                                "real"
                            } else {
                                "string"
                            }
                        ),
                        declaration.span,
                    );
                    continue;
                }
            };
            for item in &declaration.items {
                if !item.dimensions.is_empty() {
                    self.error(
                        format!(
                            "an array `{}` inside a process has no lowered form yet",
                            item.name
                        ),
                        item.span,
                    );
                    continue;
                }
                // The declared variable is the assignment's target, so it seeds
                // the context exactly as an ordinary assignment's does.
                let initial = item
                    .init
                    .as_ref()
                    .map(|init| self.assigned_value(block, init, width));
                // IEEE 1364-2005 table 5-21 makes an `integer` signed, and
                // gives it no qualifier with which to say otherwise. So a loop
                // counter compares signed, and `i < 0` can be true.
                self.declare_local(
                    block,
                    Some(item.name.clone()),
                    width,
                    true,
                    item.span,
                    initial,
                );
            }
        }

        for declaration in &inner.digital_variables {
            let signed = declaration.signedness.is_signed();
            let width = match &declaration.range {
                None => Some(1),
                Some(range) => match (self.constant(&range.msb), self.constant(&range.lsb)) {
                    (Some(msb), Some(lsb)) => Some(msb.abs_diff(lsb) as u32 + 1),
                    _ => {
                        self.error(
                            "the bounds of a process-local `reg` must be literal in this wave",
                            range.span,
                        );
                        None
                    }
                },
            };
            let Some(width) = width else { continue };
            for item in &declaration.items {
                if !item.dimensions.is_empty() {
                    self.error(
                        format!(
                            "an array `{}` inside a process has no lowered form yet",
                            item.name
                        ),
                        item.span,
                    );
                    continue;
                }
                let initial = item
                    .init
                    .as_ref()
                    .map(|init| self.assigned_value(block, init, width));
                self.declare_local(
                    block,
                    Some(item.name.clone()),
                    width,
                    signed,
                    item.span,
                    initial,
                );
            }
        }
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
                self.scopes.push(Vec::new());
                self.declare_block_locals(block, inner);
                let mut current = block;
                for statement in &inner.statements {
                    current = self.statement(current, statement);
                }
                self.scopes.pop();
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
                // Both arms have their one predecessor now, so they are sealed
                // before anything inside them reads a variable.
                self.builder.seal_block(then_entry);
                self.builder.seal_block(else_entry);
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
                self.builder.seal_block(join);
                join
            }
            DigitalStatement::Case(case) => self.case(block, case),
            DigitalStatement::Timing(timing) => {
                let resume =
                    self.wait(block, &timing.control, timing.statement.as_deref(), &mut []);
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
                // Sealed only now: the back edge is the body's second
                // predecessor, and a variable read inside the body has to see
                // both of them or it merges with half the loop missing.
                self.builder.seal_block(body);
                let unreachable = self.builder.create_block();
                self.builder.seal_block(unreachable);
                unreachable
            }
            DigitalStatement::While(statement) => {
                self.loop_statement(block, &statement.body, None, |lowerer, header| {
                    lowerer.condition(header, &statement.condition)
                })
            }
            // IEEE 1364-2005 section 9.6.2. The initialization runs once
            // before the loop and the update at the end of each pass, which is
            // exactly where they are placed here; the counter is an ordinary
            // process-local, so nothing about the loop needs a mechanism of its
            // own once one exists.
            DigitalStatement::For(statement) => {
                let block = self.assign(block, &statement.init, false);
                self.loop_statement(
                    block,
                    &statement.body,
                    Some(LoopUpdate::Assign(&statement.update)),
                    |lowerer, header| lowerer.condition(header, &statement.condition),
                )
            }
            // Section 9.6.2 evaluates the count *once*, before the loop, and
            // runs the body that many times. A count with an `x` or `z` bit
            // has no number of passes, so the loop runs zero of them — which
            // is what the truth-value reduction of the counter already says,
            // without a rule of its own.
            DigitalStatement::Repeat(statement) => {
                let count = self.expression(block, &statement.count);
                let width = self.value_width(count);
                // The counter lives in a region of its own so that it crosses
                // a suspension inside the body like any other local, and so
                // that nothing in the body can name it.
                self.scopes.push(Vec::new());
                let counter =
                    self.declare_local(block, None, width, false, statement.span, Some(count));
                let exit = self.loop_statement(
                    block,
                    &statement.body,
                    Some(LoopUpdate::Decrement(counter)),
                    |lowerer, header| {
                        let value = lowerer.read_local(header, counter);
                        lowerer.truth_value(header, value)
                    },
                );
                self.scopes.pop();
                exit
            }
        }
    }

    /// Lower a loop whose header tests a condition before each pass.
    ///
    /// `while`, `for`, and `repeat` differ only in what the header tests and
    /// what runs at the end of a pass, so they share the graph shape: the
    /// header is a merge point with two predecessors, and it cannot be sealed
    /// until the back edge exists — which is the one place SSA construction
    /// genuinely needs two passes.
    fn loop_statement(
        &mut self,
        block: BlockId,
        body_statement: &DigitalStatement,
        update: Option<LoopUpdate<'_>>,
        condition_of: impl FnOnce(&mut Self, BlockId) -> ValueId,
    ) -> BlockId {
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

        let condition = condition_of(self, header);
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
        self.builder.seal_block(body);
        self.builder.seal_block(exit);

        let mut body_exit = self.statement(body, body_statement);
        match update {
            Some(LoopUpdate::Assign(assign)) => {
                body_exit = self.assign(body_exit, assign, false);
            }
            Some(LoopUpdate::Decrement(counter)) => self.decrement(body_exit, counter),
            None => {}
        }
        self.builder.set_terminator(
            body_exit,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(header);
        exit
    }

    /// Lower a `case`, `casez`, or `casex` as a chain of match tests.
    ///
    /// The test is [`CfgValueKind::DigitalCaseMatch`], not `==`. IEEE
    /// 1364-2005 section 9.5 compares a case item against the selector *bit by
    /// bit including `x` and `z`*, so `case (sel) 2'bx0:` matches a selector of
    /// `x0` — where `==` yields `x` and would send it to the default. Section
    /// 9.5.1 then adds the wildcard forms, which ignore the positions where
    /// either operand holds a don't-care value. All three are the same
    /// operator with a different ignore set, which is why they are one node
    /// and not a lowering trick.
    fn case(&mut self, block: BlockId, case: &DigitalCase) -> BlockId {
        let match_kind = match case.kind {
            crate::ast::CaseKind::Exact => DigitalCaseMatch::Exact,
            crate::ast::CaseKind::WildcardZ => DigitalCaseMatch::WildcardZ,
            crate::ast::CaseKind::WildcardXZ => DigitalCaseMatch::WildcardXZ,
        };
        let selector = self.expression(block, &case.selector);
        let join = self.builder.create_block();
        let mut current = block;

        for item in &case.items {
            let mut matched: Option<ValueId> = None;
            for label in &item.labels {
                // Section 9.5 extends every case expression to the width of the
                // widest, and the extension is section 5.4.2's: signed when the
                // selector and *this* label both are. Decided per label rather
                // than once for the arm, because two labels of one arm may be
                // classified differently and each is its own comparison.
                let signed = self.comparison_is_signed(&case.selector, label);
                let label_value = self.expression(current, label);
                let test = self.builder.push(
                    current,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalCaseMatch {
                        selector,
                        label: label_value,
                        kind: match_kind,
                        signed,
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
            self.builder.seal_block(arm);
            self.builder.seal_block(next);
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
        self.builder.seal_block(join);
        join
    }

    /// Lower one assignment, blocking or nonblocking.
    ///
    /// An intra-assignment timing control (`q <= #5 d`) evaluates the
    /// right-hand side *before* suspending, per IEEE 1364-2005 section 9.2.2 —
    /// which is why the value node is emitted into the current block and only
    /// the write lands after the wait.
    fn assign(&mut self, block: BlockId, assign: &DigitalAssign, nonblocking: bool) -> BlockId {
        let context = self.lvalue_width(&assign.target);
        let mut carried = [self.assigned_value(block, &assign.value, context)];
        let block = match &assign.timing {
            // The value crosses the suspension as a resume argument; without
            // that the write would read a value the interpreter no longer has.
            Some(control) => self.wait(block, control, None, &mut carried),
            None => block,
        };
        self.write(block, &assign.target, carried[0], nonblocking);
        block
    }

    /// Lower an assignment's right-hand side under the context its target
    /// gives it, IEEE 1364-2005 sections 5.4.1, 5.4.2 and 5.2.1.
    ///
    /// The target's *width* seeds the sizing, because section 5.4.1 puts the
    /// left-hand side in the right-hand side's context. Its *signedness* does
    /// not enter: section 5.4.2 rule (a) says an expression's type depends only
    /// on its operands and not on what it is assigned to, so an unsigned target
    /// cannot make a signed right-hand side compute unsigned, and a signed
    /// target cannot rescue an unsigned one.
    ///
    /// The extension afterwards is the one step the write below cannot do. A
    /// value narrower than its target is padded wherever it lands — at the
    /// concatenation split, at [`Self::write_local`], in the interpreter's
    /// signal write — and every one of those zero-fills, which is section
    /// 5.2.1's rule for an unsigned expression and the wrong answer for a
    /// signed one. So only the signed half is emitted here; stating the
    /// unsigned half as well would put a node where the rule already applies.
    fn assigned_value(&mut self, block: BlockId, value: &Expression, width: u32) -> ValueId {
        let signed = self.self_signed(value);
        let lowered = self.sized(block, value, Context { width, signed });
        if signed && self.value_width(lowered) < width {
            return self.resize(block, lowered, width, true);
        }
        lowered
    }

    /// Emit the write nodes for one target.
    ///
    /// A concatenation target becomes one write per element, over slices of
    /// the right-hand side taken from the most significant end down, which is
    /// what `{carry, sum} = ...` means.
    ///
    /// The right-hand side is resized to the concatenation's *total* width
    /// first. IEEE 1364-2005 section 5.2.1 makes the assignment context the
    /// whole left-hand side, so `{carry, sum} = src` with a one-bit `src`
    /// zero-extends and gives `carry` a 0. Slicing an unresized value instead
    /// reads bits that are not there, and section 4.2.1 makes those `x` — so
    /// the defect this fixes did not fail loudly, it wrote `x` into the top of
    /// every concatenation target narrower than the sum of its parts.
    fn write(&mut self, block: BlockId, target: &DigitalLValue, value: ValueId, nonblocking: bool) {
        match target {
            DigitalLValue::Concat { elements, .. } => {
                let widths: Vec<u32> = elements
                    .iter()
                    .map(|part| self.lvalue_width(part))
                    .collect();
                let total: u32 = widths.iter().sum();
                let value = self.resize(block, value, total, false);
                let mut offset = total;
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
            // A process-local is an SSA variable, not a signal: writing one is
            // a definition in the current block rather than a node in the
            // instruction stream.
            DigitalLValue::Identifier { name, .. } if self.lookup_local(name).is_some() => {
                let local = self.lookup_local(name).expect("just resolved");
                if nonblocking {
                    self.error(
                        format!(
                            "a nonblocking assignment to the process-local `{name}` has no \
                             lowered form yet: a deferred update needs a store to defer it \
                             into, and a process-local lives in the function"
                        ),
                        target.span(),
                    );
                    return;
                }
                self.write_local(block, local, value);
            }
            DigitalLValue::BitSelect { name, .. } | DigitalLValue::PartSelect { name, .. }
                if self.lookup_local(name).is_some() =>
            {
                self.error(
                    format!(
                        "a select on the process-local `{name}` cannot be assigned yet: a \
                         partial write is a read-modify-write and this wave has no node \
                         for one outside the signal store"
                    ),
                    target.span(),
                );
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

    /// Emit the driver-write nodes for one continuous assignment's target.
    ///
    /// The same split a procedural concatenation target gets, and for the same
    /// reason — `assign {cout, sum} = a + b;` resizes to the total width and
    /// then distributes — but each element becomes a *separate driver*. That is
    /// what it is: two nets, each driven by one expression, and a resolver
    /// working on `cout` has no business being handed `sum`'s contribution.
    fn drive(
        &mut self,
        block: BlockId,
        target: &DigitalLValue,
        value: ValueId,
        process: DigitalProcessId,
        drivers: &mut Vec<DigitalDriver>,
    ) {
        match target {
            DigitalLValue::Concat { elements, .. } => {
                let widths: Vec<u32> = elements
                    .iter()
                    .map(|part| self.lvalue_width(part))
                    .collect();
                let total: u32 = widths.iter().sum();
                let value = self.resize(block, value, total, false);
                let mut offset = total;
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
                    self.drive(block, element, slice, process, drivers);
                }
            }
            _ => {
                let Some(resolved) = self.write_target(target) else {
                    return;
                };
                // Declaration order among this net's drivers, which is what
                // makes the identity stable across a recompilation.
                let index = drivers
                    .iter()
                    .filter(|driver| driver.id.signal == resolved.signal)
                    .count() as u32;
                let driver = DigitalDriverId {
                    signal: resolved.signal,
                    index,
                };
                drivers.push(DigitalDriver {
                    id: driver,
                    target: resolved.clone(),
                    process,
                    span: target.span().into(),
                });
                self.builder.push(
                    block,
                    CfgValueType::Effect,
                    CfgValueKind::DigitalDriverWrite {
                        driver,
                        target: resolved,
                        value,
                    },
                );
            }
        }
    }

    /// Resize a value to `width`, extending by `signed`.
    ///
    /// Built from the nodes the IR already has rather than from a resize node
    /// of its own: truncation is a part select of the low bits, zero-extension
    /// is a concatenation with a zero constant, and an exact fit is nothing at
    /// all. A dedicated node would need its own semantics in every consumer,
    /// and these already have theirs.
    ///
    /// Sign extension is built from them too, and is the standard's own idiom:
    /// `{{n{value[msb]}}, value}`, a concatenation of `n` copies of the sign
    /// bit above the value. That is section 5.4.1's extension of a signed
    /// operand and, because a part select copies the bit rather than testing
    /// it, section 4.3.2's rule for a sign position holding `x` or `z` — such a
    /// value extends with that bit and is unknown all the way up, which is what
    /// makes the extension honest about not knowing the sign.
    ///
    /// A zero-fill is section 5.2.1's assignment-context resizing, and does
    /// *not* propagate a leading `x` the way section 3.5.1 propagates one in a
    /// literal.
    fn resize(&mut self, block: BlockId, value: ValueId, width: u32, signed: bool) -> ValueId {
        let current = self.value_width(value);
        if current == width {
            return value;
        }
        if current > width {
            return self.builder.push(
                block,
                CfgValueType::FourState { width },
                CfgValueKind::DigitalPartSelect {
                    input: value,
                    msb: i64::from(width) - 1,
                    lsb: 0,
                },
            );
        }
        let mut parts = Vec::with_capacity((width - current + 1) as usize);
        if signed {
            let sign = self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalPartSelect {
                    input: value,
                    msb: i64::from(current) - 1,
                    lsb: i64::from(current) - 1,
                },
            );
            parts.resize(usize::try_from(width - current).unwrap_or(0), sign);
        } else {
            parts.push(self.builder.push_leaf(
                CfgValueType::FourState {
                    width: width - current,
                },
                CfgValueKind::FourStateConstant(FourStateValue::zero(width - current)),
            ));
        }
        parts.push(value);
        self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalConcat { parts },
        )
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
                self.error(
                    format!(
                        "`{name}` is not a discrete-domain signal; assigning a module-level \
                         analog variable from a process has no lowered form yet — declare \
                         the variable inside the process instead"
                    ),
                    span,
                );
                None
            }
        }
    }

    fn lvalue_width(&mut self, target: &DigitalLValue) -> u32 {
        match target {
            DigitalLValue::Identifier { name, .. } => match self.lookup_local(name) {
                Some(local) => self.local_width(local),
                None => self
                    .index
                    .get(name.as_str())
                    .map_or(1, |signal| self.width_of(*signal)),
            },
            DigitalLValue::BitSelect { .. } => 1,
            DigitalLValue::PartSelect { msb, lsb, .. } => {
                match (self.constant(msb), self.constant(lsb)) {
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
    ///
    /// Everything the resumed half of the process needs travels through
    /// `resume_args`, because a suspension does not preserve the value table:
    /// the process stopped, and the kernel that starts it again does so from a
    /// resume state and nothing else. Two things cross here — every
    /// process-local in scope, and whatever `carried` names, which is how
    /// `q <= #5 d` gets the `d` it read *before* the delay (IEEE 1364-2005
    /// section 9.2.2) to the write that lands after it.
    ///
    /// Every in-scope local crosses, not only the ones the resumed half reads.
    /// A liveness analysis would carry fewer; carrying one that is never read
    /// again costs a bound parameter and nothing else, and getting liveness
    /// wrong costs correctness.
    fn wait(
        &mut self,
        block: BlockId,
        control: &TimingControl,
        guarded: Option<&DigitalStatement>,
        carried: &mut [ValueId],
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
        for value in carried.iter_mut() {
            *value = self.builder.carry_value(*value, block, resume);
        }
        for local in self.locals_in_scope() {
            self.builder
                .carry_variable(CfgVariable::DigitalLocal(local), block, resume);
        }
        self.builder.seal_block(resume);
        resume
    }

    /// Lower a delay operand, which is an integer number of time units.
    ///
    /// A leaf, not a block instruction: a constant delay reads nothing, and
    /// the `Wait` that consumes it is the terminator of a block it would
    /// otherwise have to be placed in.
    fn delay(&mut self, expression: &Expression) -> ValueId {
        let value = match self.constant(expression) {
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
        self.truth_value(block, value)
    }

    /// Reduce a value to the one bit a `Branch` reads.
    fn truth_value(&mut self, block: BlockId, value: ValueId) -> ValueId {
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

    /// Lower an expression that no outer expression sizes.
    ///
    /// The self-determined case of [`Self::sized`], and the only one a caller
    /// outside the expression machinery wants: a `case` selector, a branch
    /// condition, a `repeat` count and an event term are all self-determined
    /// per IEEE 1364-2005 table 5-22. An assignment's right-hand side is not —
    /// see [`Self::assigned_value`] — and goes through `sized` with the
    /// target's width.
    fn expression(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        self.sized(block, expression, Context::SELF_DETERMINED)
    }

    /// Lower `expression` under `context`, IEEE 1364-2005 sections 5.4.1 and
    /// 5.4.2.
    ///
    /// # The rule this implements
    ///
    /// Sizing an expression is two passes over one tree, and doing it in one
    /// is the defect this exists to prevent. The first pass is bottom-up:
    /// [`Self::self_width`] gives every expression its *self-determined* size
    /// from its operands alone, and [`Self::self_signed`] its self-determined
    /// signedness the same way. The second is top-down and is this function:
    /// the context size is the larger of the self-determined size and whatever
    /// the enclosing expression asks for, the context signedness is the
    /// enclosing expression's `and`ed with this expression's own, and both are
    /// pushed back down into the operands the standard calls
    /// *context-determined*, which are extended to them **before** the
    /// operation runs.
    ///
    /// Section 5.4.1 puts the assignment's left-hand side in that context. So
    /// `p = a * b` with four-bit operands and an eight-bit `p` multiplies at
    /// eight bits and yields 225, where multiplying at the operand width and
    /// widening the product afterwards yields 1. Both are total answers; only
    /// one is the language's.
    ///
    /// # Why the signedness rides the same context
    ///
    /// Section 5.5 settles both before evaluation and from the whole context,
    /// and section 5.4.2 rule (j) makes an expression signed only when *every*
    /// one of its context-determined operands is. So one unsigned operand makes
    /// the shared context unsigned, and that decision travels back down into
    /// the other operands exactly as the width does: in `p = (a + b) + c` with
    /// `a` and `b` signed and `c` a plain `reg`, the inner `a + b` is computed
    /// unsigned too. Carrying the signedness separately from the width would
    /// mean two walks that can disagree about which subexpression they are
    /// describing; carrying it in the same [`Context`] means the pair is
    /// decided once, at each node, and used together.
    ///
    /// # What is returned
    ///
    /// A value of exactly `max(self_width(expression), context.width)` bits
    /// when the expression is context-determined, and of exactly `self_width`
    /// when it is not. A self-determined expression is *not* padded here:
    /// whether its value needs extending depends on what consumes it, and the
    /// consumer that needs it — a context-determined operator — does it through
    /// [`Self::operand`], which is also the only place that knows whether to
    /// pad with zeros or with the sign bit.
    ///
    /// # The classification (table 5-22 and section 5.4.2)
    ///
    /// Context-determined operands, which receive the context: both sides of
    /// `+ - * / %`, of the bitwise `& | ^ ~^`, the operand of unary `~ + -`,
    /// the *left* operand of `<< >> >>>`, and both arms of `?:`. Each of those
    /// operators takes the context size as its result size, and is signed iff
    /// all of those operands are.
    ///
    /// Self-determined, which receive nothing: the right operand of a shift,
    /// every operand of a concatenation and its replication count, a reduction
    /// operand, the condition of `?:`, and both operands of a logical
    /// `&& || !`. A comparison's two operands size to each other and to nothing
    /// outside; the result of a comparison, a logical operator or a reduction
    /// is one *unsigned* bit whatever surrounds it, which is rules (g) and (h).
    ///
    /// Unsigned whatever their operands, per rules (d), (e) and (f): a
    /// bit-select, a part-select even of a whole vector, and a concatenation or
    /// replication. Those three are why a signed context can be lost inside an
    /// expression that reads nothing but signed declarations.
    ///
    /// A comparison's operands are lowered self-determined and are *not*
    /// resized to each other here, because they need no node to be: section
    /// 4.1.7's equality, section 9.5's identity comparison and section 4.1.6's
    /// relational operators each extend the narrower operand themselves, under
    /// the signedness the comparison node carries. Emitting the resize would
    /// state the rule twice and mean it once.
    fn sized(&mut self, block: BlockId, expression: &Expression, context: Context) -> ValueId {
        let width = self.self_width(expression).max(context.width);
        let inner = Context {
            width,
            signed: context.signed && self.self_signed(expression),
        };
        match expression {
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                // A sized literal keeps the width its author wrote and is
                // extended, if at all, as an ordinary operand. An unsized one
                // takes the context, padded by section 3.5.1's rule rather than
                // with zeros — `'bx` in a wide context is wide `x`.
                let value = match literal.value.declared_width {
                    Some(_) => FourStateValue::from_literal(&literal.value),
                    None => FourStateValue::from_bits_msb_first(&literal.value.bits_at(width)),
                };
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
            Expression::Digital(crate::ast::DigitalExpr::Xnor(xnor)) => {
                let left = self.operand(block, &xnor.left, inner);
                let right = self.operand(block, &xnor.right, inner);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalBitwise {
                        op: BitwiseOp::Xnor,
                        left,
                        right,
                    },
                )
            }
            // Section 4.1.12. The left operand is context-determined and
            // carries the result size, exactly as `>>`'s does; the count is
            // self-determined. What is decided here is the fill: `>>>` shifts
            // in the sign bit only when the shift's own expression is signed,
            // and is `>>` when it is not — so the lowering answers the question
            // once and the IR node means what it says.
            Expression::Digital(crate::ast::DigitalExpr::ArithmeticShiftRight(shift)) => {
                let value = self.operand(block, &shift.left, inner);
                let count = self.expression(block, &shift.right);
                let op = if inner.signed {
                    ShiftOp::ArithmeticRight
                } else {
                    ShiftOp::Right
                };
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalShift { op, value, count },
                )
            }
            Expression::Digital(crate::ast::DigitalExpr::CaseEquality(equality)) => {
                self.case_equality(block, equality)
            }
            Expression::Digital(crate::ast::DigitalExpr::Reduction(reduction)) => {
                self.reduction(block, reduction)
            }
            Expression::Number(number) => {
                // IEEE 1364-2005 section 3.5.1: a *sized* literal is exactly as
                // wide as its author wrote it, and an unsized one is at least
                // 32 bits — and section 5.4.1 gives it the context's width when
                // that is larger, which is what `width` already is.
                //
                // The size is recovered from the literal's own source spelling,
                // because that is the only place it survives: the lexer routes a
                // based literal whose digits are all `0`/`1` to the integer
                // decoder, which keeps the number and drops the width.
                //
                // Reading it back is not cosmetic. Every operator that combines
                // widths depends on it, and a concatenation depends on nothing
                // else: `{a, b, c, 1'b1}` is four bits, while the same
                // concatenation holding a 32-bit `1` is thirty-five, whose low
                // four bits are a different value entirely.
                if let Ok(literal) = crate::four_state::decode(&number.raw) {
                    let value = match literal.declared_width {
                        Some(_) => FourStateValue::from_literal(&literal),
                        None => FourStateValue::from_bits_msb_first(&literal.bits_at(width)),
                    };
                    let width = value.width();
                    return self.builder.push_leaf(
                        CfgValueType::FourState { width },
                        CfgValueKind::FourStateConstant(value),
                    );
                }
                // A plain decimal with no base marker. Section 3.5.1 sizes one
                // as an unsized literal, so it too takes a wider context.
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
            // Section 5.4.1 makes every operand of a concatenation
            // self-determined, and the concatenation's own size the sum of
            // them. The context stops here: in `{a, b + c}` the sum is as wide
            // as `b` and `c`, and wraps, however wide the target is. This is
            // the operator the whole rule is usually got wrong on, because
            // pushing the context through it looks like the same thing.
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
            // Both arms are context-determined; the condition is not. A `?:`
            // is therefore not a place the context is dropped — `p = s ? a*b :
            // a+b` computes both at `p`'s width.
            Expression::Conditional(conditional) => {
                let condition = self.condition(block, &conditional.condition);
                let then_value = self.operand(block, &conditional.then_expr, inner);
                let else_value = self.operand(block, &conditional.else_expr, inner);
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
            Expression::Unary(unary) => self.unary(block, unary, inner),
            Expression::Binary(binary) => self.binary(block, binary, inner),
            other => {
                self.error(
                    "this expression form has no discrete-domain lowering",
                    other.span(),
                );
                self.unknown(1)
            }
        }
    }

    /// Lower a context-determined operand and extend it to the context.
    ///
    /// The extension is section 5.4.1's, which happens *before* the operator
    /// runs — the whole point of the pass — and section 5.4.2 decides what it
    /// fills with. Note which signedness that is: the **enclosing
    /// expression's**, carried in `context`, not the operand's own. Rule (j)
    /// makes the two agree whenever the expression is signed, because an
    /// expression is signed only when all of its context-determined operands
    /// are; the case the distinction covers is the other one, where a single
    /// unsigned operand has made the whole context unsigned and a signed
    /// sibling must therefore be zero-extended into it.
    ///
    /// [`Self::sized`] already returns `context.width` bits for a
    /// context-determined operand, so the resize is a no-op for one and a real
    /// extension only for a self-determined operand — a comparison, a
    /// reduction, a concatenation, a register narrower than the context.
    fn operand(&mut self, block: BlockId, expression: &Expression, context: Context) -> ValueId {
        let value = self.sized(block, expression, context);
        self.resize(block, value, context.width, context.signed)
    }

    /// The self-determined signedness of an expression, IEEE 1364-2005 section
    /// 5.4.2.
    ///
    /// The bottom-up half of the signing, and the exact counterpart of
    /// [`Self::self_width`]: pure, emitting nothing, consulted by
    /// [`Self::sized`] for every expression it lowers so the two halves cannot
    /// disagree about which subexpression they describe.
    ///
    /// # The clause, rule by rule
    ///
    /// * **(a)** The type depends only on the operands, never on the left-hand
    ///   side. That is why this takes no context: an assignment cannot make its
    ///   right-hand side signed, and cannot make a signed one unsigned either.
    /// * **(b)** A decimal number with no base is signed. `-1` is therefore a
    ///   signed 32-bit value, which is the whole reason `a == -1` behaves
    ///   differently from `a == 32'hFFFFFFFF`.
    /// * **(c)** A based number is unsigned *unless* its base carries the `s`
    ///   marker: `4'd9` is unsigned, `4'sd9` is signed, and the two spell the
    ///   same four bits.
    /// * **(d), (e), (f)** A bit-select, a part-select, and a concatenation or
    ///   replication are unsigned regardless of their operands — a part-select
    ///   of a whole `reg signed` included. These three are how a signed
    ///   expression stops being one without any unsigned declaration in sight.
    /// * **(g), (h)** A comparison and a reduction yield an unsigned bit, and
    ///   so does a logical operator, whatever they were given.
    /// * **(j)** For everything with context-determined operands, the result is
    ///   signed iff *every* one of those operands is. One unsigned operand
    ///   makes the whole expression unsigned, and [`Self::sized`] then carries
    ///   that decision back down into the signed siblings.
    ///
    /// A form this cannot classify does not lower either, and answers unsigned
    /// — the classification of the all-`x` placeholder left after the refusal.
    fn self_signed(&self, expression: &Expression) -> bool {
        match expression {
            // Rule (c), from the source spelling: the marker survives decoding
            // into `FourStateLiteral::signed`.
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                literal.value.signed
            }
            // Rules (b) and (c) together. A number that carries a base marker
            // is signed only with `s`; one that carries none is a plain decimal
            // and is signed. Read from the raw spelling because that is where
            // both facts live — `crate::four_state::decode` cannot be asked, as
            // an analog literal's raw text may not decode at all.
            Expression::Number(number) => {
                !number.raw.contains('\'') || crate::four_state::has_signed_marker(&number.raw)
            }
            // Table 5-21: a declaration is signed only when it says so, and an
            // `integer` says so by being one.
            Expression::Identifier(identifier) => match self.lookup_local(&identifier.name) {
                Some(local) => self.local_signed(local),
                None => self
                    .index
                    .get(identifier.name.as_str())
                    .is_some_and(|signal| self.signed_signal(*signal)),
            },
            // Rules (d), (e) and (f).
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(_))
            | Expression::ArrayAccess(_)
            | Expression::ArrayLiteral(_) => false,
            // Rules (g) and (h).
            Expression::Digital(crate::ast::DigitalExpr::CaseEquality(_))
            | Expression::Digital(crate::ast::DigitalExpr::Reduction(_)) => false,
            // Rule (j) over the operands each operator makes
            // context-determined. Both arms of `?:`, both sides of `~^`, and
            // the operand of `~ + -`; the condition of `?:` and the operand of
            // `!` are self-determined and take no part, and `!` yields an
            // unsigned bit in any case.
            Expression::Digital(crate::ast::DigitalExpr::Xnor(xnor)) => {
                self.self_signed(&xnor.left) && self.self_signed(&xnor.right)
            }
            Expression::Digital(crate::ast::DigitalExpr::ArithmeticShiftRight(shift)) => {
                self.self_signed(&shift.left)
            }
            Expression::Conditional(conditional) => {
                self.self_signed(&conditional.then_expr) && self.self_signed(&conditional.else_expr)
            }
            Expression::Unary(unary) => match unary.op {
                UnaryOp::Not => false,
                UnaryOp::BitNot | UnaryOp::Pos | UnaryOp::Neg => self.self_signed(&unary.operand),
            },
            Expression::Binary(binary) => match binary.op {
                BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Mod
                | BinaryOp::Pow => {
                    self.self_signed(&binary.left) && self.self_signed(&binary.right)
                }
                BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => false,
                // A shift takes its type from the value being shifted. The
                // count is self-determined and cannot make a signed shift
                // unsigned, which is what keeps `a >>> 1` arithmetic when the
                // count is a plain `reg`.
                BinaryOp::Shl | BinaryOp::Shr => self.self_signed(&binary.left),
            },
            _ => false,
        }
    }

    /// The self-determined size of an expression, IEEE 1364-2005 table 5-22.
    ///
    /// The bottom-up half of the sizing, and pure: it reads declarations and
    /// literals and emits nothing, so it can be asked before a single node
    /// exists. [`Self::sized`] consults it for every expression it lowers,
    /// which is what keeps the two halves from disagreeing — the width a node
    /// is emitted at is `max(self_width, context)` by construction rather than
    /// by a second derivation that happens to match.
    ///
    /// A form this cannot size is one that does not lower either; each such
    /// arm answers 1, which is the width of the all-`x` placeholder
    /// [`Self::unknown`] leaves behind after the refusal.
    fn self_width(&self, expression: &Expression) -> u32 {
        match expression {
            // Sized literals keep their written width; unsized ones report the
            // 32-bit floor, and grow past it only through the context.
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                literal.value.width()
            }
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
                match (self.constant(&select.msb), self.constant(&select.lsb)) {
                    (Some(msb), Some(lsb)) => msb.abs_diff(lsb) as u32 + 1,
                    // Refused when lowered, and one bit of `x` when it is.
                    _ => 1,
                }
            }
            Expression::Digital(crate::ast::DigitalExpr::Xnor(xnor)) => self
                .self_width(&xnor.left)
                .max(self.self_width(&xnor.right)),
            // Section 4.1.12 and table 5-22, the same row `<<` and `>>` are on:
            // a shift is as wide as the value being shifted, and the count does
            // not enter.
            Expression::Digital(crate::ast::DigitalExpr::ArithmeticShiftRight(shift)) => {
                self.self_width(&shift.left)
            }
            // Section 4.1.8: an identity comparison is one bit, and so is a
            // reduction of section 4.1.10.
            Expression::Digital(crate::ast::DigitalExpr::CaseEquality(_))
            | Expression::Digital(crate::ast::DigitalExpr::Reduction(_)) => 1,
            Expression::Number(number) => match crate::four_state::decode(&number.raw) {
                Ok(literal) => literal.width(),
                Err(_) => crate::four_state::UNSIZED_FOUR_STATE_WIDTH,
            },
            Expression::Identifier(identifier) => match self.lookup_local(&identifier.name) {
                Some(local) => self.local_width(local),
                None => self
                    .index
                    .get(identifier.name.as_str())
                    .map_or(1, |signal| self.width_of(*signal)),
            },
            Expression::ArrayAccess(_) => 1,
            Expression::ArrayLiteral(literal) => literal
                .elements
                .iter()
                .map(|element| self.element_width(element))
                .sum(),
            Expression::Conditional(conditional) => self
                .self_width(&conditional.then_expr)
                .max(self.self_width(&conditional.else_expr)),
            Expression::Unary(unary) => match unary.op {
                // Section 4.1.8: logical negation is one bit.
                UnaryOp::Not => 1,
                UnaryOp::BitNot | UnaryOp::Pos | UnaryOp::Neg => self.self_width(&unary.operand),
            },
            Expression::Binary(binary) => match binary.op {
                BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Mod
                | BinaryOp::Pow => self
                    .self_width(&binary.left)
                    .max(self.self_width(&binary.right)),
                // Sections 4.1.6, 4.1.7 and 4.1.8: one bit, and the operands'
                // widths take no part in it.
                BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => 1,
                // Section 4.1.12 and table 5-22: a shift is as wide as the
                // value being shifted. The count is self-determined and does
                // not enter.
                BinaryOp::Shl | BinaryOp::Shr => self.self_width(&binary.left),
            },
            _ => 1,
        }
    }

    /// The self-determined width one concatenation element contributes.
    ///
    /// A replication contributes its count times the width of what it repeats,
    /// and nothing when the count is not the constant section 4.1.14 requires
    /// — which is what the lowering contributes too, having refused it.
    fn element_width(&self, element: &ArrayLiteralElement) -> u32 {
        match element {
            ArrayLiteralElement::Value(expression) => self.self_width(expression),
            ArrayLiteralElement::Replication(replication) => {
                let Some(count) = self
                    .constant(&replication.count)
                    .filter(|count| *count >= 0)
                    .and_then(|count| u32::try_from(count).ok())
                else {
                    return 0;
                };
                let inner: u32 = replication
                    .elements
                    .iter()
                    .map(|element| self.element_width(element))
                    .sum();
                count.saturating_mul(inner)
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
                let Some(count) = self
                    .constant(&replication.count)
                    .filter(|count| *count >= 0)
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

    /// Lower `a === b` / `a !== b`, IEEE 1364-2005 section 4.1.8.
    ///
    /// Onto the node `case` already uses, because they are the same operator:
    /// section 9.5's case comparison is an identity comparison over all four
    /// states, which is exactly what `===` is, and
    /// [`DigitalCaseMatch::Exact`](super::digital_value::DigitalCaseMatch::Exact)
    /// is that comparison. Giving `===` a node of its own would put a second
    /// transcription of one rule into the interpreter, with the usual two
    /// chances to disagree about `4'b10xz === 4'b10xz`.
    ///
    /// `!==` is the complement, and can be one safely: `===` yields a definite
    /// bit for every pair of operands, so negating it cannot manufacture the
    /// `x` that `!=` would have produced.
    fn case_equality(
        &mut self,
        block: BlockId,
        equality: &crate::ast::CaseEqualityExpr,
    ) -> ValueId {
        let signed = self.comparison_is_signed(&equality.left, &equality.right);
        let selector = self.expression(block, &equality.left);
        let label = self.expression(block, &equality.right);
        let matched = self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalCaseMatch {
                selector,
                label,
                kind: DigitalCaseMatch::Exact,
                signed,
            },
        );
        if !equality.negate {
            return matched;
        }
        self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogicalNot { input: matched },
        )
    }

    /// Lower a reduction operator, IEEE 1364-2005 section 4.1.10.
    ///
    /// # Why this is a desugaring rather than a node
    ///
    /// Section 4.1.10 does not define reduction as a new function. It defines
    /// it as *the section 4.1.9 bitwise operator applied successively across
    /// the bits of one operand*, and the `nand`/`nor`/`xnor` forms as the
    /// `and`/`or`/`xor` fold with the single-bit result inverted. So the
    /// faithful lowering is that iteration written out — one bit select per
    /// bit, one existing binary node per step — and a `CfgValueKind` of its own
    /// would be a second place to state a rule the tables already state.
    ///
    /// That is not merely cheaper. A new kind is four edits that must land
    /// together (`leaf_class`, the `is_digital` anchor, the AD guard, and the
    /// interpreter), and `leaf_class`'s catch-all would cache a reduction at
    /// module scope if the arm were missed — a defect no type error catches.
    ///
    /// # What it does to `x` and `z`
    ///
    /// Exactly what the tables do, which is the whole reason to build it out of
    /// them. `&{1'b0, 1'bx}` is `0`, not `x`, because `0` is AND's controlling
    /// value; `^{1'b0, 1'bx}` is `x`, because XOR has none. A lowering that
    /// poisoned the result whenever any operand bit was unknown would get the
    /// first of those wrong.
    ///
    /// A one-bit operand reduces to itself (with the inversion, for the
    /// complemented forms), which is what a fold with no second element is.
    fn reduction(&mut self, block: BlockId, reduction: &crate::ast::ReductionExpr) -> ValueId {
        let input = self.expression(block, &reduction.operand);
        let width = self.value_width(input);
        let op = match reduction.op {
            ReductionOp::And | ReductionOp::Nand => BitwiseOp::And,
            ReductionOp::Or | ReductionOp::Nor => BitwiseOp::Or,
            ReductionOp::Xor | ReductionOp::Xnor => BitwiseOp::Xor,
        };

        let bit = |lowerer: &mut Self, index: u32| {
            lowerer.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalPartSelect {
                    input,
                    msb: i64::from(index),
                    lsb: i64::from(index),
                },
            )
        };

        // Least significant bit first, so the fold reads the way the value is
        // indexed. The operators are associative and commutative over the
        // section 4.1.9 tables, so the direction is a readability choice.
        let mut folded = bit(self, 0);
        for index in 1..width {
            let next = bit(self, index);
            folded = self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalBitwise {
                    op,
                    left: folded,
                    right: next,
                },
            );
        }

        if !reduction.op.inverts() {
            return folded;
        }
        self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalBitwiseNot { input: folded },
        )
    }

    /// Lower a unary operator under `context`.
    ///
    /// Table 5-22 splits the four: `!` is one bit and its operand is
    /// self-determined, while `~`, `+` and `-` take the context size and pass
    /// it to their operand. The difference is observable — `~(a == b)` in an
    /// eight-bit context inverts a zero-extended one bit and yields
    /// `8'b11111110`, not the `8'b00000000` that inverting first would give.
    fn unary(
        &mut self,
        block: BlockId,
        unary: &crate::ast::UnaryExpr,
        context: Context,
    ) -> ValueId {
        let width = context.width;
        match unary.op {
            UnaryOp::Not => {
                let input = self.expression(block, &unary.operand);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalLogicalNot { input },
                )
            }
            UnaryOp::BitNot => {
                let input = self.operand(block, &unary.operand, context);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalBitwiseNot { input },
                )
            }
            // Unary `+` is the identity on its operand however that operand is
            // signed, so the operand already extended to the context *is* the
            // result. The extension is where `+a` differs from `a`, and
            // [`Self::operand`] has already done it.
            UnaryOp::Pos => self.operand(block, &unary.operand, context),
            UnaryOp::Neg => {
                // `-x` is `0 - x` at the context width. Two's complement makes
                // that the same subtraction for a signed and an unsigned
                // operand; what differs is the extension that produced `x`,
                // which is why the negation of a narrow signed value is right
                // only when the operand reached the context signed.
                let input = self.operand(block, &unary.operand, context);
                let zero = self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(FourStateValue::zero(width)),
                );
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalArithmetic {
                        op: ArithmeticOp::Sub,
                        left: zero,
                        right: input,
                        signed: context.signed,
                    },
                )
            }
        }
    }

    /// Lower a binary operator under `context`.
    ///
    /// Three groups, and which group an operator is in is the whole of table
    /// 5-22 for the binary forms:
    ///
    /// * **arithmetic and bitwise** — both operands context-determined, result
    ///   `context.width` and signed by `context.signed`. This is where the
    ///   context has to reach or the operation runs narrow and the answer is
    ///   wrong rather than merely narrow.
    /// * **logical and comparison** — one bit of result, operands
    ///   self-determined. A comparison's two operands size to each other, which
    ///   the operators themselves do (see [`Self::sized`]); the outer context
    ///   reaches neither, and neither does the outer signedness. What the node
    ///   does carry is the comparison's *own* signedness, from its two operands
    ///   alone, because section 5.4.2 makes the comparison signed only when
    ///   both of them are.
    /// * **shift** — the left operand is context-determined and carries the
    ///   result size; the right is self-determined, being a number of positions
    ///   rather than a value combined with anything.
    fn binary(
        &mut self,
        block: BlockId,
        binary: &crate::ast::BinaryExpr,
        context: Context,
    ) -> ValueId {
        let width = context.width;
        let kind = match binary.op {
            BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                let op = match binary.op {
                    BinaryOp::BitAnd => BitwiseOp::And,
                    BinaryOp::BitOr => BitwiseOp::Or,
                    _ => BitwiseOp::Xor,
                };
                let left = self.operand(block, &binary.left, context);
                let right = self.operand(block, &binary.right, context);
                CfgValueKind::DigitalBitwise { op, left, right }
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                let op = match binary.op {
                    BinaryOp::Add => ArithmeticOp::Add,
                    BinaryOp::Sub => ArithmeticOp::Sub,
                    BinaryOp::Mul => ArithmeticOp::Mul,
                    BinaryOp::Div => ArithmeticOp::Div,
                    _ => ArithmeticOp::Mod,
                };
                let left = self.operand(block, &binary.left, context);
                let right = self.operand(block, &binary.right, context);
                CfgValueKind::DigitalArithmetic {
                    op,
                    left,
                    right,
                    signed: context.signed,
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                let op = if matches!(binary.op, BinaryOp::And) {
                    LogicalOp::And
                } else {
                    LogicalOp::Or
                };
                let left = self.expression(block, &binary.left);
                let right = self.expression(block, &binary.right);
                return self.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalLogical { op, left, right },
                );
            }
            BinaryOp::Eq | BinaryOp::Ne => {
                let negate = matches!(binary.op, BinaryOp::Ne);
                let signed = self.comparison_is_signed(&binary.left, &binary.right);
                let left = self.expression(block, &binary.left);
                let right = self.expression(block, &binary.right);
                return self.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalEquality {
                        left,
                        right,
                        negate,
                        signed,
                    },
                );
            }
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                let op = match binary.op {
                    BinaryOp::Lt => RelationalOp::Lt,
                    BinaryOp::Le => RelationalOp::Le,
                    BinaryOp::Gt => RelationalOp::Gt,
                    _ => RelationalOp::Ge,
                };
                let signed = self.comparison_is_signed(&binary.left, &binary.right);
                let left = self.expression(block, &binary.left);
                let right = self.expression(block, &binary.right);
                return self.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalRelational {
                        op,
                        left,
                        right,
                        signed,
                    },
                );
            }
            BinaryOp::Shl | BinaryOp::Shr => {
                // Section 4.1.12: `<<` and `>>` fill with zero whatever the
                // expression's sign, so neither needs one. `<<<` arrives here
                // as `<<`, which the standard says it is.
                let op = if matches!(binary.op, BinaryOp::Shl) {
                    ShiftOp::Left
                } else {
                    ShiftOp::Right
                };
                let value = self.operand(block, &binary.left, context);
                let count = self.expression(block, &binary.right);
                CfgValueKind::DigitalShift { op, value, count }
            }
            BinaryOp::Pow => {
                self.error(
                    "`**` has no discrete-domain lowering in this wave",
                    binary.span,
                );
                return self.unknown(width);
            }
        };
        self.builder
            .push(block, CfgValueType::FourState { width }, kind)
    }

    /// Whether a comparison is made on signed numbers, IEEE 1364-2005 sections
    /// 4.1.6 and 5.4.2.
    ///
    /// The operands are context-determined *with respect to each other* and to
    /// nothing outside, so the comparison forms its own context — and rule (j)
    /// applies inside it: signed only when both operands are. A single
    /// unsigned operand makes the comparison unsigned, which is why `-1 < 0`
    /// stops holding the moment one side is a plain `reg`.
    ///
    /// The enclosing expression takes no part in either direction. Rule (g)
    /// makes the result an unsigned bit however the comparison was made, so a
    /// signed context outside cannot reach in, and an unsigned one cannot
    /// suppress a signed comparison within.
    fn comparison_is_signed(&self, left: &Expression, right: &Expression) -> bool {
        self.self_signed(left) && self.self_signed(right)
    }

    fn named_value(&mut self, block: BlockId, name: &str, span: Span) -> ValueId {
        // A process-local shadows a module signal of the same name, per IEEE
        // 1364-2005 section 9.8.1, so the innermost region is asked first.
        if let Some(local) = self.lookup_local(name) {
            return self.read_local(block, local);
        }
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
                        "`{name}` is not a discrete-domain signal; reading a module-level \
                         analog variable from a process has no lowered form yet — declare \
                         the variable inside the process instead"
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

    /// The constant value of an expression in *this* body's scope.
    ///
    /// A literal, or a name the declaring module gave an integer parameter or
    /// localparam — IEEE 1364-2005 section 12.2 fixes both at elaboration. A
    /// name that also denotes a signal is never folded: a signal is a runtime
    /// value, and reading one as a constant would replace a whole design's
    /// behaviour with one number.
    fn constant(&self, expression: &Expression) -> Option<i64> {
        if let Expression::Identifier(identifier) = expression {
            if self.index.contains_key(identifier.name.as_str()) {
                return None;
            }
            return self.constants.get(&identifier.name).copied();
        }
        if let Expression::Unary(unary) = expression {
            return match unary.op {
                UnaryOp::Neg => self.constant(&unary.operand).map(|value| -value),
                UnaryOp::Pos => self.constant(&unary.operand),
                _ => None,
            };
        }
        if let Expression::Binary(binary) = expression {
            let left = self.constant(&binary.left)?;
            let right = self.constant(&binary.right)?;
            return match binary.op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Sub => left.checked_sub(right),
                BinaryOp::Mul => left.checked_mul(right),
                BinaryOp::Div => left.checked_div(right),
                _ => None,
            };
        }
        constant_of(expression)
    }

    fn constant_index(&mut self, expression: &Expression) -> Option<i64> {
        match self.constant(expression) {
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
        // Every discrete-domain form at once, through the two accessors on
        // `DigitalExpr`, rather than one arm per variant. The catch-all below
        // is what makes that matter: a form this function forgot would
        // contribute no reads, and a continuous assignment with an empty read
        // set gets no sensitivity list at all — it would evaluate once at time
        // zero and never again, which is a wrong waveform rather than a
        // refusal.
        Expression::Digital(digital) => {
            if let Some(name) = digital.base_name() {
                reads.insert(name.to_string());
            }
            for child in digital.children() {
                collect_expression_reads(child, reads);
            }
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
