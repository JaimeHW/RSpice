//! Lowering one analog [`CfgFunction`] output onto the native block model.
//!
//! The shipped native route lowers MIR: a flat postfix stream per equation,
//! with every conditional already dissolved into a select. This is the other
//! route — the canonical CFG's blocks, terminators and typed block parameters
//! carried straight onto [`ssa::Program`]'s, so a conditional stays a branch
//! and a loop stays a loop.
//!
//! # What it is not
//!
//! Not a second semantic authority. Every runtime name it produces comes from
//! somewhere that already owned it: parameter and node indices from
//! [`MirModel`], branch-unknown indices from the map the shipped planner
//! builds, and analog-operator state records from
//! [`CfgStateAllocation`], which is the composition of the executed
//! correspondence with [`CanonicalStateLayout`]. Nothing here invents an index.
//!
//! # Total, or refused by name
//!
//! [`CfgValueKind`] spans both halves of Verilog-AMS and every analog operator
//! the language has. This lowering covers the analog kinds the shipped bundle
//! actually contains and refuses every other kind by naming it, rather than
//! folding an unhandled case into a plausible default. A refusal is a
//! [`JitError::UnsupportedCanonicalOp`] carrying the kind, so a census can
//! report exactly which construct stopped which model.
//!
//! # The discrete-domain half is refused, and this is what it needs
//!
//! A digital process function is a [`CfgFunction`] like any other, and every
//! part of it is refused here by name: [`CfgTerminator::Wait`], the four-state
//! and integer value types, and each discrete-domain
//! [`CfgValueKind`]. Landing half of it would be worse than landing none,
//! because a process that lowered its arithmetic and dropped its suspension
//! would run to completion and be wrong at every tick. What it needs is
//! settled here so the next lane implements a decision rather than making one.
//!
//! **A four-state value is a pair of `I64` SSA values, and its width rides on
//! the instruction.** [`ValueType`] gains `I64`; a value of
//! [`CfgValueType::FourState`] becomes two of them, the `aval` and `bval`
//! planes, and a value wider than 64 bits becomes a fixed number of such pairs
//! — fixed because the CFG makes the width static. The alternatives were
//! considered and rejected. Packing both planes into one 128-bit value needs a
//! register class none of the three backends has: WebAssembly's `v128` is
//! behind the SIMD proposal the shipped module does not enable. Carrying the
//! planes as two `F64`s would tunnel integers through the floating-point bank,
//! which is exactly what [`CfgValueType::Integer`]'s own documentation refuses
//! at the level above — "a discrete-domain index, shift count, or delay is an
//! integer whose wrapping and division behaviour is defined, and rounding it
//! out of a float at each use is how those definitions get lost". Two separate
//! SSA values rather than one two-field value is what lets the existing block
//! parameters carry a four-state value across a merge with no new merge kind:
//! it is two parameters. And because every value the analog path produces
//! stays `F64`, adding the variant moves no analog code and no shipped image.
//!
//! **`Wait` is a terminator that returns the resume state as data.** The
//! interpreter's contract (see
//! [`digital_eval`](crate::canonical_ir::digital_eval)) is that a suspension
//! carries the resume block and the values that bind to its parameters, and
//! that resuming starts from an *empty* value table and rebinds only those
//! parameters. A `Suspend { wait, resume: Edge }` terminator reproduces it
//! exactly: it writes the resume block's id and the edge's arguments into the
//! caller's frame and returns. Resuming is a second entry point that switches
//! on the block id and rebinds that block's parameters from the frame before
//! falling into it — a jump table every backend can emit. The sensitivity list
//! is compile-time data hung off the program for the scheduler, not code.
//!
//! Two of this model's invariants have to be restated before that compiles: a
//! program has exactly one `Return`, which a process with several suspensions
//! does not; and a state write must dominate the exit, which is a statement
//! about analog operator history and has no meaning for a process. Both are
//! properties of an *analog* function, and the honest change is a program kind
//! rather than a weakening of the rules the analog path depends on.
//!
//! What remains, named: `ValueType::I64` and the roughly thirty four-state
//! `NativeOp`s the discrete kinds need; the signal-store ABI (`read_signal`,
//! `write_signal`, `defer_update`, `drive_signal`) as helper calls; the
//! `Suspend` terminator and the resume entry on x64, AArch64 and WebAssembly;
//! and an event-for-event agreement harness against
//! [`digital_eval`](crate::canonical_ir::digital_eval) over `tests/verilog`,
//! the sixteen-case oracle corpus, the twelve-circuit scale suite and
//! `tests/verilog/ams`.
//!
//! # Layout
//!
//! [`ssa::Program`] numbers blocks in layout order and requires every forward
//! edge to run forward in it, each natural loop to occupy one contiguous
//! range, and the single `Return` to be last. A CFG carries none of that: its
//! block ids are creation order, in which a conditional's join precedes the
//! blocks of its arms. [`layout_order`] is what reconciles the two, and it
//! refuses rather than guesses when it cannot.

#![cfg_attr(not(feature = "native"), allow(dead_code))]
// The route is internal by design: W-D adds the Jacobians a shipped model
// needs and W-F is what flips the default over, so until then the census and
// the unit tests below are this module's only callers, and a shipped
// constructor would be a second default nobody asked for.
#![cfg_attr(not(test), allow(dead_code))]

use smol_str::SmolStr;

use crate::canonical_ir::{
    BlockId as CfgBlockId, CanonicalStateOperator, CfgBinaryOp, CfgFunction, CfgStateAllocation,
    CfgTerminator, CfgUnaryOp, CfgValueKind, CfgValueType, MirModel, NodeId, ValueId as CfgValueId,
};
use crate::jit::expr::{
    BinaryMathOp, BranchUnknownRuntimeMapping, CompareOp, ExtremumOp, LogicalOp, NativeOp,
    UnaryMathOp, VoltageNode,
};
use crate::jit::ssa::{
    BlockId, BuilderTerminator, BuilderValue, Program, ProgramBuilder, ValueType,
};
use crate::jit::{JitError, JitResult};

/// The runtime numbering a lowered program addresses.
///
/// Held apart from the CFG because the CFG names things canonically and the
/// runtime names them positionally, and the translation belongs to whoever
/// owns both — never to the CFG level, which must stay independent of any one
/// backend's storage layout.
#[derive(Debug, Clone)]
pub(crate) struct CfgRuntimeBindings {
    pub(crate) model: SmolStr,
    pub(crate) terminal_count: usize,
    pub(crate) internal_node_count: usize,
    pub(crate) parameter_count: usize,
    /// Indexed by canonical `BranchUnknownId`.
    pub(crate) branch_unknowns: Vec<BranchUnknownRuntimeMapping>,
    /// Runtime variable slot of each event-controlled procedural state, indexed
    /// by the CFG's dense event-state slot.
    ///
    /// The two numbering spaces are different: the CFG numbers the state
    /// variables of one module in declaration order, while the runtime numbers
    /// every variable the model has. The caller owns both and supplies the
    /// correspondence, for the same reason it supplies the branch-unknown map.
    ///
    /// `None` for a slot the caller could not match, which is refused by name
    /// rather than dropped: dropping one would renumber every slot after it,
    /// and a renumbered event state reads another variable's history.
    pub(crate) event_state_variables: Vec<Option<usize>>,
    /// External-terminal names in canonical port order, for `$port_connected`.
    pub(crate) terminal_names: Vec<SmolStr>,
}

impl CfgRuntimeBindings {
    /// Read every numbering this lowering needs out of MIR.
    ///
    /// `branch_unknowns` is supplied rather than derived: matching a canonical
    /// branch unknown to a runtime branch source needs the compiled model, and
    /// the shipped planner already owns that correlation.
    pub(crate) fn from_mir(
        model: impl Into<SmolStr>,
        mir: &MirModel,
        branch_unknowns: Vec<BranchUnknownRuntimeMapping>,
        event_state_variables: Vec<Option<usize>>,
    ) -> Self {
        let terminal_names: Vec<SmolStr> = mir
            .nodes
            .iter()
            .filter(|node| node.is_external)
            .map(|node| node.name.clone())
            .collect();
        Self {
            model: model.into(),
            terminal_count: terminal_names.len(),
            internal_node_count: mir.nodes.len() - terminal_names.len(),
            parameter_count: mir.parameters.len(),
            branch_unknowns,
            event_state_variables,
            terminal_names,
        }
    }
}

/// Lower `function`, pruned to `output`, onto the block model.
///
/// The caller prunes: [`crate::canonical_ir::prune_cfg_to_outputs`] already
/// exists, knows the CFG's own dead-code rules, and returns the renumbered
/// output, so doing it here would be a second implementation of it.
pub(crate) fn lower_cfg_function(
    function: &CfgFunction,
    output: CfgValueId,
    state: &CfgStateAllocation,
    bindings: &CfgRuntimeBindings,
) -> JitResult<Program> {
    Lowerer {
        function,
        state,
        bindings,
    }
    .run(output)
}

struct Lowerer<'a> {
    function: &'a CfgFunction,
    state: &'a CfgStateAllocation,
    bindings: &'a CfgRuntimeBindings,
}

impl Lowerer<'_> {
    fn run(&self, output: CfgValueId) -> JitResult<Program> {
        let layout = layout_order(self.function)?;
        let order = &layout.order;
        let mut position = vec![usize::MAX; self.function.blocks.len()];
        for (index, block) in order.iter().enumerate() {
            position[usize::from(*block)] = index;
        }

        let parameters = order
            .iter()
            .map(|block| {
                self.function
                    .block(*block)
                    .params
                    .iter()
                    .map(|param| self.value_type(*param))
                    .collect::<JitResult<Vec<_>>>()
            })
            .collect::<JitResult<Vec<_>>>()?;
        let mut builder = ProgramBuilder::new(&parameters)?;

        // Values the CFG pins to a block: instruction results and block
        // parameters. Everything else it reads is a leaf, which the CFG
        // deliberately leaves unpinned so that any block may read one, and
        // which is materialized here in the entry block — the one block that
        // dominates every use by construction.
        let unpinned = self.unpinned_leaves(order, output)?;
        let hoisted = self.hoisted_state_operators(&layout)?;

        let mut lowered: Vec<Option<BuilderValue>> = vec![None; self.function.values.len()];
        for (index, block) in order.iter().enumerate() {
            let id = BlockId::new(index)?;
            builder.begin_block(id)?;
            for (slot, param) in self.function.block(*block).params.iter().enumerate() {
                lowered[usize::from(*param)] = Some(builder.parameter(id, slot)?);
            }
            if index == 0 {
                for leaf in &unpinned {
                    let emitted = self.emit(&mut builder, &lowered, *leaf)?;
                    lowered[usize::from(*leaf)] = Some(emitted);
                }
            }
            for instruction in &self.function.block(*block).instructions {
                if lowered[usize::from(instruction.result)].is_some() {
                    continue;
                }
                let emitted = self.emit(&mut builder, &lowered, instruction.result)?;
                lowered[usize::from(instruction.result)] = Some(emitted);
            }
            for value in &hoisted[usize::from(*block)] {
                let emitted = self.emit(&mut builder, &lowered, *value)?;
                lowered[usize::from(*value)] = Some(emitted);
            }
            let terminator = self.terminator(*block, &position, &lowered, output)?;
            builder.end_block(terminator)?;
        }

        let exit = order
            .iter()
            .position(|block| {
                matches!(
                    self.function.block(*block).terminator,
                    CfgTerminator::Return
                )
            })
            .ok_or_else(|| self.refuse("CFG function has no Return terminator".to_string()))?;
        builder.finish(BlockId::new(0)?, BlockId::new(exit)?)
    }

    /// Where each analog operator that owns a state record is emitted, indexed
    /// by CFG block: the block it appears in, unless that block runs
    /// conditionally.
    ///
    /// An operator's record must advance once per evaluation, whatever the
    /// control flow does — that is the whole reason the block model's verifier
    /// refuses a state write on a block that does not dominate the exit. The
    /// shipped MIR route meets it accidentally, because its select form
    /// evaluates both arms of every conditional, so a `ddt` written under an
    /// `if` runs whichever way the condition goes. The block form keeps the
    /// conditional, so it has to meet the rule deliberately: the operator moves
    /// to the deepest block that dominates the exit and still follows every one
    /// of its operands. Its *result* does not move — it is still read only
    /// where the source read it, so the residual is unchanged and the reference
    /// interpreter still agrees; what changes is only that the record advances
    /// on the path the source did not take, which is what the shipped route
    /// already does.
    ///
    /// When no such block exists — an operator whose operand is itself
    /// computed inside the conditional — this refuses by name rather than
    /// speculating the operand's whole cone. Verilog-AMS LRM 2.4 section 4.4.1
    /// does not admit an analog operator under a non-constant condition at all,
    /// so hoisting what can be hoisted is already a compatibility extension;
    /// extending it to arbitrary speculation is a decision about the
    /// compatibility contract rather than about this lowering.
    fn hoisted_state_operators(&self, layout: &Layout) -> JitResult<Vec<Vec<CfgValueId>>> {
        let mut definition: Vec<Option<usize>> = vec![None; self.function.values.len()];
        for block in &self.function.blocks {
            let index = usize::from(block.id);
            for param in &block.params {
                definition[usize::from(*param)] = Some(index);
            }
            for instruction in &block.instructions {
                definition[usize::from(instruction.result)] = Some(index);
            }
        }
        let mut hoisted: Vec<Vec<CfgValueId>> = vec![Vec::new(); self.function.blocks.len()];
        for block in &layout.order {
            let index = usize::from(*block);
            if dominates(&layout.idom, index, layout.exit) {
                continue;
            }
            for instruction in &self.function.block(*block).instructions {
                let value = self.function.value(instruction.result);
                if value.kind.state_site().is_none() {
                    continue;
                }
                let deepest_operand = value
                    .kind
                    .operands()
                    .into_iter()
                    .filter_map(|operand| definition[usize::from(operand)])
                    .max_by_key(|candidate| dominator_depth(&layout.idom, *candidate))
                    .unwrap_or(usize::from(self.function.entry));
                let mut target = index;
                loop {
                    if dominates(&layout.idom, target, layout.exit)
                        && dominates(&layout.idom, deepest_operand, target)
                    {
                        break;
                    }
                    match layout.idom[target] {
                        Some(parent) if parent != target => target = parent,
                        _ => {
                            return Err(self.refuse(format!(
                                "canonical analog operator {} runs under a condition and its operand is computed there too, so its record cannot be made to advance once per evaluation",
                                kind_name(&value.kind)
                            )));
                        }
                    }
                }
                hoisted[target].push(instruction.result);
            }
        }
        Ok(hoisted)
    }

    /// Values that are read but that no block defines, in value order.
    ///
    /// Derived from the function rather than from a copy of the CFG level's
    /// leaf predicate: a value nothing defines and something reads *is* a
    /// leaf, and one that turns out to have operands is a malformed function
    /// rather than a kind this lowering forgot.
    fn unpinned_leaves(
        &self,
        order: &[CfgBlockId],
        output: CfgValueId,
    ) -> JitResult<Vec<CfgValueId>> {
        let mut pinned = vec![false; self.function.values.len()];
        let mut read = vec![false; self.function.values.len()];
        let mark = |value: CfgValueId, flags: &mut Vec<bool>| {
            if let Some(flag) = flags.get_mut(usize::from(value)) {
                *flag = true;
            }
        };
        mark(output, &mut read);
        for block in order {
            let block = self.function.block(*block);
            for param in &block.params {
                mark(*param, &mut pinned);
            }
            for instruction in &block.instructions {
                mark(instruction.result, &mut pinned);
                for operand in self.function.value(instruction.result).kind.operands() {
                    mark(operand, &mut read);
                }
            }
            match &block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        mark(*arg, &mut read);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    mark(*condition, &mut read);
                    for arg in then_args.iter().chain(else_args) {
                        mark(*arg, &mut read);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
            }
        }
        let mut leaves = Vec::new();
        for value in &self.function.values {
            let index = usize::from(value.id);
            if pinned[index] || !read[index] {
                continue;
            }
            if !value.kind.operands().is_empty() {
                return Err(self.refuse(format!(
                    "CFG value {index} is read but no block defines it, and it is not a leaf"
                )));
            }
            leaves.push(value.id);
        }
        Ok(leaves)
    }

    fn terminator(
        &self,
        block: CfgBlockId,
        position: &[usize],
        lowered: &[Option<BuilderValue>],
        output: CfgValueId,
    ) -> JitResult<BuilderTerminator> {
        let target = |block: CfgBlockId| BlockId::new(position[usize::from(block)]);
        let arguments = |args: &[CfgValueId]| {
            args.iter()
                .map(|value| self.read(lowered, *value))
                .collect::<JitResult<Vec<_>>>()
        };
        Ok(match &self.function.block(block).terminator {
            CfgTerminator::Return => BuilderTerminator::Return(self.read(lowered, output)?),
            CfgTerminator::Jump { target: to, args } => BuilderTerminator::Jump {
                target: target(*to)?,
                arguments: arguments(args)?,
            },
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => BuilderTerminator::Branch {
                condition: self.read(lowered, *condition)?,
                then_target: target(*then_target)?,
                then_arguments: arguments(then_args)?,
                else_target: target(*else_target)?,
                else_arguments: arguments(else_args)?,
            },
            CfgTerminator::Wait { .. } => {
                return Err(self.refuse(
                    "digital process suspension (Wait) has no analog block-model counterpart"
                        .to_string(),
                ));
            }
            CfgTerminator::Unset => {
                return Err(
                    self.refuse(format!("CFG block {} is unterminated", usize::from(block)))
                );
            }
        })
    }

    fn read(&self, lowered: &[Option<BuilderValue>], value: CfgValueId) -> JitResult<BuilderValue> {
        lowered
            .get(usize::from(value))
            .copied()
            .flatten()
            .ok_or_else(|| {
                self.refuse(format!(
                    "CFG value {} is read before the block that defines it",
                    usize::from(value)
                ))
            })
    }

    fn value_type(&self, value: CfgValueId) -> JitResult<ValueType> {
        match self.function.value(value).value_type {
            CfgValueType::Real | CfgValueType::Boolean => Ok(ValueType::F64),
            other => Err(self.refuse(format!(
                "CFG value {} carries type {other:?}, which the native block model has no register class for",
                usize::from(value)
            ))),
        }
    }

    fn emit(
        &self,
        builder: &mut ProgramBuilder,
        lowered: &[Option<BuilderValue>],
        value: CfgValueId,
    ) -> JitResult<BuilderValue> {
        let entry = self.function.value(value);
        let value_type = self.value_type(value)?;
        let mut push = |op: NativeOp, operands: &[BuilderValue]| -> JitResult<BuilderValue> {
            builder.push(op, operands, value_type)
        };
        let operand = |value: CfgValueId| self.read(lowered, value);
        match &entry.kind {
            CfgValueKind::RealConstant(constant) => push(NativeOp::Const(*constant), &[]),
            CfgValueKind::BooleanConstant(constant) => {
                push(NativeOp::Const(f64::from(u8::from(*constant))), &[])
            }
            CfgValueKind::Parameter(id) => {
                let index = self.parameter_index(usize::from(*id))?;
                push(NativeOp::LoadParam(index), &[])
            }
            CfgValueKind::ParameterGiven(id) => {
                let index = self.parameter_index(usize::from(*id))?;
                push(NativeOp::LoadParamGiven(index), &[])
            }
            CfgValueKind::PortConnected(port) => {
                let index = usize::try_from(*port).unwrap_or(usize::MAX);
                if index >= self.bindings.terminal_count {
                    return Err(self.refuse(format!(
                        "CFG $port_connected names terminal {index} of {}",
                        self.bindings.terminal_count
                    )));
                }
                push(NativeOp::LoadPortConnected(index), &[])
            }
            // The accepted value of one event-controlled procedural variable,
            // which the runtime restores into that variable's slot before an
            // evaluation begins. A separate op would be a second ABI for a
            // value the variable array already holds.
            CfgValueKind::EventState(slot) => {
                let slot = usize::try_from(*slot).unwrap_or(usize::MAX);
                let index = self
                    .bindings
                    .event_state_variables
                    .get(slot)
                    .copied()
                    .flatten()
                    .ok_or_else(|| {
                        self.refuse(format!(
                            "CFG event-state slot {slot} has no runtime variable; the module declares {} of them",
                            self.bindings.event_state_variables.len()
                        ))
                    })?;
                push(NativeOp::LoadVariable(index), &[])
            }
            CfgValueKind::Temperature => push(NativeOp::LoadTemperature, &[]),
            CfgValueKind::ThermalVoltage => push(NativeOp::LoadThermalVoltage, &[]),
            CfgValueKind::Multiplicity => push(NativeOp::LoadMfactor, &[]),
            CfgValueKind::Time => push(NativeOp::LoadTime, &[]),
            CfgValueKind::Analysis(name) => match analysis_code(name) {
                Some(code) => push(NativeOp::Analysis(code), &[]),
                // The same answer the shipped lowering gives an analysis name
                // no runtime reports: it is not active.
                None => push(NativeOp::Const(0.0), &[]),
            },
            // The runtime leaf, answered by its source fallback. The shipped
            // MIR route folds `$simparam` to a table of compile-time values
            // instead, which `NativeProgram`'s own documentation records as a
            // known divergence between the two backends; this route matches
            // the CFG reference interpreter given no simulator override, which
            // is the semantics the CFG carries.
            CfgValueKind::SimParam { fallback, .. } => {
                let fallback = operand(*fallback)?;
                push(NativeOp::AddConst(0.0), &[fallback])
            }
            CfgValueKind::NodePotential(node) => {
                let pos = self.voltage_node(Some(*node))?;
                push(
                    NativeOp::LoadVoltage {
                        pos,
                        neg: VoltageNode::Ground,
                    },
                    &[],
                )
            }
            CfgValueKind::BranchUnknownFlow(unknown) => {
                let mapping = self
                    .bindings
                    .branch_unknowns
                    .get(usize::from(*unknown))
                    .copied()
                    .ok_or_else(|| {
                        self.refuse(format!(
                            "CFG branch unknown {} has no runtime source",
                            usize::from(*unknown)
                        ))
                    })?;
                let loaded = push(NativeOp::LoadBranchUnknown(mapping.runtime_index), &[])?;
                if mapping.inverted {
                    push(NativeOp::Neg, &[loaded])
                } else {
                    Ok(loaded)
                }
            }
            // Identically zero in the large signal; only the derivative pass
            // gives it an amplitude.
            CfgValueKind::NoiseProcess(_) => push(NativeOp::Const(0.0), &[]),
            CfgValueKind::DdtScale => {
                let one = push(NativeOp::Const(1.0), &[])?;
                push(NativeOp::DdtJacobian, &[one])
            }
            CfgValueKind::IdtScale => {
                let one = push(NativeOp::Const(1.0), &[])?;
                push(NativeOp::IdtJacobian, &[one])
            }
            CfgValueKind::Ddt { operator, input } => {
                let slot = self.state_slot(*operator, CanonicalStateOperator::Ddt)?;
                let input = operand(*input)?;
                push(NativeOp::DdtState(slot), &[input])
            }
            CfgValueKind::Idt {
                operator,
                input,
                ic,
            } => {
                let slot = self.state_slot(*operator, CanonicalStateOperator::Idt)?;
                let input = operand(*input)?;
                let ic = operand(*ic)?;
                push(NativeOp::IdtState(slot), &[input, ic])
            }
            CfgValueKind::Unary { op, input } => {
                let input = operand(*input)?;
                let native = unary_op(*op).ok_or_else(|| {
                    self.refuse(format!(
                        "CFG unary operator {op:?} has no native block-model lowering"
                    ))
                })?;
                push(native, &[input])
            }
            CfgValueKind::Binary { op, left, right } => {
                let left = operand(*left)?;
                let right = operand(*right)?;
                push(binary_op(*op), &[left, right])
            }
            other => Err(self.refuse(format!(
                "CFG value kind {} has no native block-model lowering",
                kind_name(other)
            ))),
        }
    }

    fn parameter_index(&self, index: usize) -> JitResult<usize> {
        if index >= self.bindings.parameter_count {
            return Err(self.refuse(format!(
                "CFG parameter {index} is outside the runtime's {} parameters",
                self.bindings.parameter_count
            )));
        }
        Ok(index)
    }

    fn voltage_node(&self, node: Option<NodeId>) -> JitResult<VoltageNode> {
        let Some(node) = node else {
            return Ok(VoltageNode::Ground);
        };
        let index = usize::from(node);
        if index < self.bindings.terminal_count {
            return Ok(VoltageNode::Terminal(index));
        }
        let internal = index - self.bindings.terminal_count;
        if internal < self.bindings.internal_node_count {
            return Ok(VoltageNode::Internal(internal));
        }
        Err(self.refuse(format!(
            "CFG node {index} is outside the runtime's {} terminals and {} internal nodes",
            self.bindings.terminal_count, self.bindings.internal_node_count
        )))
    }

    /// The runtime record a CFG operator owns, or the refusal
    /// [`CfgStateAllocation`] would give for it.
    fn state_slot(
        &self,
        operator: crate::canonical_ir::ExprId,
        expected: CanonicalStateOperator,
    ) -> JitResult<usize> {
        let site = self.state.site(operator).ok_or_else(|| {
            self.refuse(format!(
                "canonical {} operator {operator} owns no state record in this module's layout",
                expected.name()
            ))
        })?;
        if site.kind.family() != expected.family() {
            return Err(self.refuse(format!(
                "canonical {} operator {operator} resolves to a {} record",
                expected.name(),
                site.kind.name()
            )));
        }
        usize::try_from(site.slot).map_err(|_| {
            self.refuse(format!(
                "canonical {} operator {operator} owns slot {} outside the host index space",
                expected.name(),
                site.slot
            ))
        })
    }

    fn refuse(&self, detail: String) -> JitError {
        JitError::UnsupportedCanonicalOp {
            model: self.bindings.model.clone(),
            op: detail.into(),
        }
    }
}

/// The native op for one CFG unary operator, or `None` where the runtime has
/// no counterpart.
///
/// `LimitedExpDerivative` is the one `None`. It is the derivative of the
/// runtime's clamped exponential, which the derivative pass introduces and
/// which no primal analog body contains, so there is nothing to lower rather
/// than something to approximate.
fn unary_op(op: CfgUnaryOp) -> Option<NativeOp> {
    let math = |op| Some(NativeOp::UnaryMath(op));
    match op {
        CfgUnaryOp::Neg => Some(NativeOp::Neg),
        CfgUnaryOp::Not => Some(NativeOp::Logical(LogicalOp::Not)),
        CfgUnaryOp::Exp => math(UnaryMathOp::Exp),
        CfgUnaryOp::LimExp => math(UnaryMathOp::Limexp),
        CfgUnaryOp::LimitedExp => math(UnaryMathOp::LimitedExp),
        CfgUnaryOp::LimitedExpDerivative => None,
        CfgUnaryOp::Ln => math(UnaryMathOp::Log),
        CfgUnaryOp::Log10 => math(UnaryMathOp::Log10),
        CfgUnaryOp::Sqrt => Some(NativeOp::Sqrt),
        CfgUnaryOp::Abs => Some(NativeOp::Abs),
        CfgUnaryOp::Sin => math(UnaryMathOp::Sin),
        CfgUnaryOp::Cos => math(UnaryMathOp::Cos),
        CfgUnaryOp::Tan => math(UnaryMathOp::Tan),
        CfgUnaryOp::Sinh => math(UnaryMathOp::Sinh),
        CfgUnaryOp::Cosh => math(UnaryMathOp::Cosh),
        CfgUnaryOp::Tanh => math(UnaryMathOp::Tanh),
        CfgUnaryOp::Asin => math(UnaryMathOp::Asin),
        CfgUnaryOp::Acos => math(UnaryMathOp::Acos),
        CfgUnaryOp::Atan => math(UnaryMathOp::Atan),
        CfgUnaryOp::Asinh => math(UnaryMathOp::Asinh),
        CfgUnaryOp::Acosh => math(UnaryMathOp::Acosh),
        CfgUnaryOp::Atanh => math(UnaryMathOp::Atanh),
        CfgUnaryOp::Floor => math(UnaryMathOp::Floor),
        CfgUnaryOp::Ceil => math(UnaryMathOp::Ceil),
    }
}

fn binary_op(op: CfgBinaryOp) -> NativeOp {
    match op {
        CfgBinaryOp::Add => NativeOp::Add,
        CfgBinaryOp::Sub => NativeOp::Sub,
        CfgBinaryOp::Mul => NativeOp::Mul,
        CfgBinaryOp::Div => NativeOp::Div,
        CfgBinaryOp::Mod => NativeOp::BinaryMath(BinaryMathOp::Mod),
        CfgBinaryOp::Pow => NativeOp::BinaryMath(BinaryMathOp::Pow),
        CfgBinaryOp::Min => NativeOp::Extremum(ExtremumOp::Min),
        CfgBinaryOp::Max => NativeOp::Extremum(ExtremumOp::Max),
        CfgBinaryOp::Hypot => NativeOp::BinaryMath(BinaryMathOp::Hypot),
        CfgBinaryOp::Atan2 => NativeOp::BinaryMath(BinaryMathOp::Atan2),
        CfgBinaryOp::Eq => NativeOp::Compare(CompareOp::Eq),
        CfgBinaryOp::Ne => NativeOp::Compare(CompareOp::Ne),
        CfgBinaryOp::Lt => NativeOp::Compare(CompareOp::Lt),
        CfgBinaryOp::Le => NativeOp::Compare(CompareOp::Le),
        CfgBinaryOp::Gt => NativeOp::Compare(CompareOp::Gt),
        CfgBinaryOp::Ge => NativeOp::Compare(CompareOp::Ge),
        CfgBinaryOp::And => NativeOp::Logical(LogicalOp::And),
        CfgBinaryOp::Or => NativeOp::Logical(LogicalOp::Or),
    }
}

fn analysis_code(name: &str) -> Option<u8> {
    match name.to_ascii_lowercase().as_str() {
        "dc" | "op" => Some(0),
        "ac" => Some(1),
        "tran" | "transient" => Some(2),
        "noise" => Some(3),
        "ic" => Some(4),
        "static" => Some(5),
        "smallsig" | "smallsignal" | "small_signal" => Some(6),
        "__rspice_initial_step" => Some(7),
        "__rspice_final_step" => Some(8),
        _ => None,
    }
}

/// A layout order for `function`'s blocks that satisfies the block model.
///
/// Three properties have to hold at once, and a CFG's creation order gives
/// none of them — a conditional's join block is created before the blocks of
/// its arms: every edge that is not a loop back edge must run forward, each
/// branch's two arms must form one contiguous single-entry region that
/// reconverges, and each natural loop must occupy one contiguous range with
/// the single `Return` last.
///
/// Reverse postorder gives only the first of those. What gives all three is
/// walking the *regions* rather than the edges, which the CFG is structured
/// enough to allow: a two-way branch's arms rejoin at its immediate
/// post-dominator, and a loop header's immediate post-dominator is the block
/// the loop exits to. So emitting a branch, then each arm up to that block,
/// then that block, reproduces the source's nesting exactly — and a back edge
/// is simply an arm reaching a block already emitted, which stops the walk.
///
/// The result is checked rather than assumed: every edge must run forward
/// unless its target dominates its source, and an irreducible graph — which
/// Verilog-A cannot express, having no `goto` and no `break` — is refused by
/// name.
struct Layout {
    order: Vec<CfgBlockId>,
    /// Immediate dominators over the CFG, indexed by raw block index.
    idom: Vec<Option<usize>>,
    /// The block that returns.
    exit: usize,
}

fn layout_order(function: &CfgFunction) -> JitResult<Layout> {
    let count = function.blocks.len();
    let refuse = |detail: String| JitError::InvalidCanonicalIr {
        model: "canonical-cfg-layout".into(),
        detail: detail.into(),
    };
    let entry = usize::from(function.entry);
    let exit = function
        .blocks
        .iter()
        .position(|block| matches!(block.terminator, CfgTerminator::Return))
        .ok_or_else(|| refuse("CFG function has no Return terminator".to_string()))?;

    let successors: Vec<Vec<usize>> = function
        .blocks
        .iter()
        .map(|block| {
            block
                .successors()
                .into_iter()
                .map(usize::from)
                .collect::<Vec<_>>()
        })
        .collect();
    let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); count];
    for (source, targets) in successors.iter().enumerate() {
        for target in targets.iter().copied() {
            predecessors[target].push(source);
        }
    }
    let idom = dominator_tree(count, entry, &successors, &predecessors);
    let ipdom = dominator_tree(count, exit, &predecessors, &successors);

    let mut order = Vec::with_capacity(count);
    let mut emitted = vec![false; count];
    emit_region(
        entry,
        None,
        function,
        &ipdom,
        &mut emitted,
        &mut order,
        &refuse,
    )?;

    let mut position = vec![usize::MAX; count];
    for (index, block) in order.iter().enumerate() {
        position[*block] = index;
    }
    for (source, targets) in successors.iter().enumerate() {
        if position[source] == usize::MAX {
            // Unreachable from the entry: the block model only admits blocks
            // that run, and the CFG level's own pruning is what removes these.
            continue;
        }
        for target in targets.iter().copied() {
            if position[target] == usize::MAX {
                return Err(refuse(format!(
                    "CFG block {source} reaches block {target}, which the layout never placed"
                )));
            }
            let backwards = position[target] <= position[source];
            if backwards && !dominates(&idom, target, source) {
                return Err(refuse(format!(
                    "CFG edge {source} -> {target} closes a cycle whose target does not dominate its source; the control-flow graph is irreducible"
                )));
            }
        }
    }
    Ok(Layout {
        order: order.into_iter().map(CfgBlockId::from).collect::<Vec<_>>(),
        idom,
        exit,
    })
}

/// Emit one single-entry region: `start` and everything it reaches before
/// `stop`, in nesting order.
fn emit_region(
    start: usize,
    stop: Option<usize>,
    function: &CfgFunction,
    ipdom: &[Option<usize>],
    emitted: &mut [bool],
    order: &mut Vec<usize>,
    refuse: &impl Fn(String) -> JitError,
) -> JitResult<()> {
    let mut current = start;
    loop {
        if Some(current) == stop || emitted[current] {
            return Ok(());
        }
        emitted[current] = true;
        order.push(current);
        match &function.blocks[current].terminator {
            CfgTerminator::Return => return Ok(()),
            CfgTerminator::Jump { target, .. } => current = usize::from(*target),
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                let join = ipdom[current].filter(|join| *join != current).ok_or_else(|| {
                    refuse(format!(
                        "CFG block {current} branches without reconverging, which the block model has no structure for"
                    ))
                })?;
                for arm in [*then_target, *else_target] {
                    emit_region(
                        usize::from(arm),
                        Some(join),
                        function,
                        ipdom,
                        emitted,
                        order,
                        refuse,
                    )?;
                }
                current = join;
            }
            CfgTerminator::Wait { .. } => {
                return Err(refuse(format!(
                    "CFG block {current} ends in a digital process suspension (Wait), which the analog block model has no counterpart for"
                )));
            }
            CfgTerminator::Unset => {
                return Err(refuse(format!("CFG block {current} is unterminated")));
            }
        }
    }
}

/// Immediate dominators of the graph rooted at `root`, by the iterative
/// data-flow algorithm.
///
/// Used twice: once over the CFG for dominance, and once over the reversed
/// CFG rooted at the `Return` block for post-dominance, which is what names a
/// branch's reconvergence point. Creation order is not a topological order —
/// a conditional's join block is created before its arms — so this iterates to
/// a fixed point rather than assuming one pass suffices.
fn dominator_tree(
    count: usize,
    root: usize,
    successors: &[Vec<usize>],
    predecessors: &[Vec<usize>],
) -> Vec<Option<usize>> {
    let mut order = Vec::with_capacity(count);
    let mut seen = vec![false; count];
    let mut stack = vec![(root, 0usize)];
    seen[root] = true;
    while let Some((block, child)) = stack.pop() {
        if child < successors[block].len() {
            stack.push((block, child + 1));
            let next = successors[block][child];
            if !seen[next] {
                seen[next] = true;
                stack.push((next, 0));
            }
        } else {
            order.push(block);
        }
    }
    order.reverse();
    let mut rank = vec![usize::MAX; count];
    for (index, block) in order.iter().enumerate() {
        rank[*block] = index;
    }
    let mut idom: Vec<Option<usize>> = vec![None; count];
    idom[root] = Some(root);
    loop {
        let mut changed = false;
        for block in order.iter().copied() {
            if block == root {
                continue;
            }
            let mut candidate: Option<usize> = None;
            for predecessor in predecessors[block].iter().copied() {
                if idom[predecessor].is_none() {
                    continue;
                }
                candidate = Some(match candidate {
                    None => predecessor,
                    Some(current) => intersect(&idom, &rank, current, predecessor),
                });
            }
            if idom[block] != candidate {
                idom[block] = candidate;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    idom
}

fn intersect(idom: &[Option<usize>], rank: &[usize], left: usize, right: usize) -> usize {
    let mut left = left;
    let mut right = right;
    while left != right {
        while rank[left] > rank[right] {
            match idom[left] {
                Some(parent) if parent != left => left = parent,
                _ => return right,
            }
        }
        while rank[right] > rank[left] {
            match idom[right] {
                Some(parent) if parent != right => right = parent,
                _ => return left,
            }
        }
    }
    left
}

/// How far `block` sits from the root of the dominator tree.
///
/// Two blocks on one dominator chain are ordered by it, which is what picks the
/// deepest of an operator's operand definitions.
fn dominator_depth(idom: &[Option<usize>], block: usize) -> usize {
    let mut cursor = block;
    let mut depth = 0;
    while let Some(parent) = idom[cursor] {
        if parent == cursor || depth > idom.len() {
            break;
        }
        cursor = parent;
        depth += 1;
    }
    depth
}

fn dominates(idom: &[Option<usize>], ancestor: usize, block: usize) -> bool {
    let mut cursor = block;
    loop {
        if cursor == ancestor {
            return true;
        }
        match idom[cursor] {
            Some(parent) if parent != cursor => cursor = parent,
            _ => return false,
        }
    }
}

/// The kind's constructor name, for a refusal that says which construct
/// stopped it.
///
/// Taken from the `Debug` rendering rather than from an exhaustive match: the
/// enum has more than seventy variants, and a hand-written table of their
/// names would be a second list to keep in step with no compiler checking it.
/// The rendering starts with the constructor and the payload follows a
/// delimiter, so the leading identifier is exactly the name and nothing of the
/// payload — which carries value ids that mean nothing outside the function —
/// escapes into the message.
fn kind_name(kind: &CfgValueKind) -> String {
    format!("{kind:?}")
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .next()
        .unwrap_or("value")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{CfgRuntimeBindings, lower_cfg_function};
    use crate::canonical_ir::hir::HirModel;
    use crate::canonical_ir::{
        CfgBinaryOp, CfgStateAllocation, CfgTerminator, CfgValueKind, CfgValueType, CfgVariable,
        DigitalWait, ParamId, SsaBuilder, VariableId,
    };

    /// A module with no analog operator, so its state allocation is empty and
    /// every refusal a test sees is about the construct it is testing.
    fn empty_state() -> CfgStateAllocation {
        let hir = HirModel {
            module_id: crate::canonical_ir::ModuleId::from(0usize),
            module_name: "cfg-program-test".into(),
            schema_version: 0,
            source_package: "".into(),
            source_digest: "".into(),
            source_identity: "".into(),
            compiler_version: "".into(),
            feature_flags: Vec::new(),
            default_transition: 0.0,
            ports: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
            arrays: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            statements: Vec::new(),
            body: Vec::new(),
            executed_correspondence: Default::default(),
            expressions: Vec::new(),
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
        };
        let function = {
            let mut builder = SsaBuilder::new();
            let entry = builder.create_block();
            builder.seal_block(entry);
            let zero = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0));
            let _ = zero;
            builder.set_terminator(entry, CfgTerminator::Return);
            builder.finish(entry).expect("empty function")
        };
        CfgStateAllocation::build(&hir, &function).expect("a module with no operator allocates")
    }

    fn bindings(parameter_count: usize) -> CfgRuntimeBindings {
        CfgRuntimeBindings {
            model: "cfg-program-test".into(),
            terminal_count: 2,
            internal_node_count: 0,
            parameter_count,
            branch_unknowns: Vec::new(),
            event_state_variables: Vec::new(),
            terminal_names: vec!["a".into(), "b".into()],
        }
    }

    /// `if (p0 > 0) r = p0 * 2 else r = p0 + 3`, as the CFG builds it: a
    /// diamond whose join takes the merged result as a block parameter.
    #[test]
    fn a_diamond_lowers_to_a_branch_and_a_block_parameter() {
        let mut builder = SsaBuilder::new();
        let variable = CfgVariable::Local(VariableId::from(0usize));
        builder.declare_variable(variable, CfgValueType::Real);
        let entry = builder.create_block();
        builder.seal_block(entry);
        let parameter = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::Parameter(ParamId::from(0usize)),
        );
        let zero = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0));
        let condition = builder.push(
            entry,
            CfgValueType::Boolean,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Gt,
                left: parameter,
                right: zero,
            },
        );
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let join = builder.create_block();
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(then_block);
        builder.seal_block(else_block);

        let two = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(2.0));
        let product = builder.push(
            then_block,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: parameter,
                right: two,
            },
        );
        builder.write_variable(variable, then_block, product);
        builder.set_terminator(
            then_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        let three = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(3.0));
        let sum = builder.push(
            else_block,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: parameter,
                right: three,
            },
        );
        builder.write_variable(variable, else_block, sum);
        builder.set_terminator(
            else_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        builder.seal_block(join);
        let merged = builder
            .read_variable(variable, join)
            .expect("the join merges both arms");
        builder.set_terminator(join, CfgTerminator::Return);
        let (function, outputs) = builder
            .finish_with_outputs(entry, &[merged])
            .expect("valid CFG");

        let program = lower_cfg_function(&function, outputs[0], &empty_state(), &bindings(1))
            .expect("the diamond lowers");
        assert_eq!(program.blocks().len(), 4, "branch, two arms, join");
        assert_eq!(
            program.block_parameter_count(),
            1,
            "the merged variable arrives as one block parameter"
        );
    }

    /// A `while` loop, as `runtime_loop` builds it: a header the entry edge and
    /// the back edge both reach.
    #[test]
    fn a_loop_lowers_to_a_back_edge_the_block_model_admits() {
        let mut builder = SsaBuilder::new();
        let counter = CfgVariable::Local(VariableId::from(0usize));
        builder.declare_variable(counter, CfgValueType::Real);
        let entry = builder.create_block();
        builder.seal_block(entry);
        let zero = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0));
        let one = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(1.0));
        let limit = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::Parameter(ParamId::from(0usize)),
        );
        builder.write_variable(counter, entry, zero);

        let header = builder.create_block();
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        let current = builder
            .read_variable(counter, header)
            .expect("the header reads the carried counter");
        let test = builder.push(
            header,
            CfgValueType::Boolean,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Lt,
                left: current,
                right: limit,
            },
        );
        let body = builder.create_block();
        let exit = builder.create_block();
        builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition: test,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);
        let advanced = builder.push(
            body,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: current,
                right: one,
            },
        );
        builder.write_variable(counter, body, advanced);
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        builder.seal_block(header);
        builder.seal_block(exit);
        let result = builder
            .read_variable(counter, exit)
            .expect("the exit reads the counter the loop left");
        builder.set_terminator(exit, CfgTerminator::Return);
        let (function, outputs) = builder
            .finish_with_outputs(entry, &[result])
            .expect("valid CFG");

        let program = lower_cfg_function(&function, outputs[0], &empty_state(), &bindings(1))
            .expect("the loop lowers");
        let loops = program.loop_ranges().expect("loop ranges");
        assert_eq!(loops.len(), 1, "one natural loop, from one back edge");
        assert!(
            program.block_parameter_count() >= 1,
            "the counter is carried around the loop as a block parameter"
        );
    }

    /// The pinned refusal for the discrete-domain half.
    ///
    /// A process function's suspension is the one construct that cannot be
    /// dropped quietly: a lowering that ignored it would run the process to
    /// completion and be wrong at every tick. See this module's documentation
    /// for what the terminator needs.
    #[test]
    fn a_process_suspension_is_refused_by_name() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        let resume = builder.create_block();
        builder.set_terminator(
            entry,
            CfgTerminator::Wait {
                wait: DigitalWait::Event(Vec::new()),
                resume,
                resume_args: Vec::new(),
            },
        );
        builder.seal_block(resume);
        let zero = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0));
        builder.set_terminator(resume, CfgTerminator::Return);
        let (function, outputs) = builder
            .finish_with_outputs(entry, &[zero])
            .expect("valid CFG");
        let error = lower_cfg_function(&function, outputs[0], &empty_state(), &bindings(0))
            .expect_err("a suspension has no analog counterpart");
        assert!(
            error.to_string().contains("suspension"),
            "unexpected refusal: {error}"
        );
    }
}
