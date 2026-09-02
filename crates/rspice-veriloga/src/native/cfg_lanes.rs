//! Splitting a differentiated [`CfgFunction`]'s packed derivative lanes into
//! ordinary scalar values, one function-wide.
//!
//! [`differentiate`](crate::canonical_ir::differentiate) produces one value per
//! derivative rather than one per lane: a packed value of type
//! [`CfgValueType::Lanes`] holding the partials with respect to every unknown
//! its liveness proved reachable. That representation is what makes the
//! generated-Rust backend's emitted code compact — one `L4` local and one `a +
//! b` line where a scalar form would spell four — and it is the right one for a
//! backend whose target language has aggregates.
//!
//! The native block model's `ValueType` has one variant, `F64`. This pass is
//! how the packed level reaches it: every packed value becomes one scalar value
//! per lane of its shape, and the result is an ordinary
//! [`CfgFunction`] carrying no packed value at all — one the CFG's own
//! [`validate`](CfgFunction::validate) accepts, the reference interpreter
//! evaluates, [`prune_to_outputs`](crate::canonical_ir::prune_cfg_to_outputs)
//! slices, and [`lower_cfg_function`](super::cfg_program::lower_cfg_function)
//! lowers with no knowledge that a lane ever existed.
//!
//! # Why this is the shape the CFG already has
//!
//! Not a new form invented for this backend. `CfgFunction::validate_shapes`
//! already admits every kind here with an empty lane set — its own words for
//! the stateful operators' Jacobian actions are that "the scalar `ddx` shadow
//! form falls out of the same rule with an empty lane set on both sides". A
//! `ddx` readback is exactly one lane of one derivative, spelled scalar, and
//! the whole of this pass is that spelling applied to every lane rather than to
//! the one a model asked for by name. So the output is not a dialect: it is the
//! form the derivative pass would have produced had it been asked for one
//! unknown at a time.
//!
//! # The lane algebra is closed, and that is what makes this total
//!
//! Only ten kinds can carry a shape, and `validate_shapes` rejects a packed
//! operand reaching any other. Five are the lane algebra proper — [`LaneSplat`],
//! [`LaneWiden`], [`LaneBinary`], [`LaneScalar`], [`LaneExtract`] — five are the
//! stateful operators' Jacobian actions, whose `*_derivative` operands are the
//! packed ones and whose remaining operands are primal scalars, and the tenth is
//! a block parameter. Every one of them scalarizes elementwise, because that is
//! what packing meant. Anything else arriving packed is refused by name rather
//! than assumed elementwise.
//!
//! [`LaneSplat`]: CfgValueKind::LaneSplat
//! [`LaneWiden`]: CfgValueKind::LaneWiden
//! [`LaneBinary`]: CfgValueKind::LaneBinary
//! [`LaneScalar`]: CfgValueKind::LaneScalar
//! [`LaneExtract`]: CfgValueKind::LaneExtract
//!
//! # Widening is an alias, not an instruction
//!
//! [`LaneWiden`] re-lays its operand out over a wider shape, zero-filling the
//! lanes it lacks, and [`LaneExtract`] names one lane of one value. Scalarized,
//! both are pure renaming: lane `L` of a widened value *is* lane `L` of its
//! operand, or the zero constant. Emitting a copy instruction for either would
//! put arithmetic into the program that the packed form did not contain, so
//! both resolve through the map instead and produce no value of their own.
//!
//! # Not an optimization
//!
//! Scalarizing multiplies the value count by the average live lane width, and
//! this pass makes no attempt to hold that down: the caller prunes to one
//! output afterwards, which deletes every lane it did not ask for, and doing
//! that work twice would be slower than doing it once.

#![cfg_attr(not(feature = "native"), allow(dead_code))]
// Same standing as the block lowering it feeds: W-F is the lane that gives this
// a shipped caller, and until then the census and the unit tests below are the
// only ones.
#![cfg_attr(not(test), allow(dead_code))]

use std::collections::HashMap;

use crate::canonical_ir::{
    BlockId, CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgValue, CfgValueKind,
    CfgValueType, ValueId,
};
use crate::jit::{JitError, JitResult};

/// A function whose packed derivatives have been split into scalars.
#[derive(Debug, Clone)]
pub(crate) struct ScalarLanes {
    /// The scalarized function. Carries no value of type
    /// [`CfgValueType::Lanes`], and no shape table.
    pub(crate) function: CfgFunction,
    /// New id of each value the source carried as a scalar, a
    /// [`CfgValueKind::LaneExtract`] read-out included: an extract emits no
    /// value of its own, so what is recorded for it is the lane it names.
    plain: Vec<Option<ValueId>>,
}

impl ScalarLanes {
    /// Where a source value ended up.
    ///
    /// Total over the scalars, which is what a caller holding a residual or a
    /// read-out from
    /// [`AdFunction::derivative_row`](crate::canonical_ir::AdFunction::derivative_row)
    /// needs; a packed value has no single answer and reports `None`.
    pub(crate) fn scalar(&self, value: ValueId) -> Option<ValueId> {
        *self.plain.get(usize::from(value))?
    }
}

/// Split every packed value in `function` into one scalar per lane.
pub(crate) fn scalarize_lanes(function: &CfgFunction) -> JitResult<ScalarLanes> {
    Scalarizer::new(function).run()
}

/// How deep a chain of pure renamings is followed before it is called a cycle.
///
/// A widen's operand and an extract's input are always defined before the value
/// itself, so a real chain is bounded by the value count and this can only fire
/// on a malformed function. It exists so that one does not hang the compiler.
const MAX_ALIAS_DEPTH: usize = 1024;

struct Scalarizer<'a> {
    source: &'a CfgFunction,
    values: Vec<CfgValue>,
    /// Pre-allocated ids, filled with their real kind in the second pass.
    plain: Vec<Option<ValueId>>,
    lanes: HashMap<(ValueId, u32), ValueId>,
    /// One value per distinct splat constant, keyed by bit pattern so that
    /// `-0.0` and `NaN` payloads stay distinct from what they compare equal or
    /// unequal to.
    constants: HashMap<u64, ValueId>,
    zero: ValueId,
}

impl<'a> Scalarizer<'a> {
    fn new(source: &'a CfgFunction) -> Self {
        let zero = ValueId::from(0usize);
        let mut constants = HashMap::new();
        constants.insert(0.0_f64.to_bits(), zero);
        Self {
            source,
            values: vec![CfgValue {
                id: zero,
                value_type: CfgValueType::Real,
                kind: CfgValueKind::RealConstant(0.0),
            }],
            plain: vec![None; source.values.len()],
            lanes: HashMap::new(),
            constants,
            zero,
        }
    }

    fn run(mut self) -> JitResult<ScalarLanes> {
        self.allocate()?;
        self.fill()?;
        let blocks = self.rebuild_blocks()?;
        // A read-out emits nothing of its own, so record the lane it names as
        // where it ended up. Done last, because resolving one needs the map the
        // two passes above build.
        let mut read_outs = Vec::new();
        for value in &self.source.values {
            if let CfgValueKind::LaneExtract { input, lane } = &value.kind {
                read_outs.push((value.id, self.lane_of(*input, *lane, 0)?));
            }
        }
        for (extract, scalar) in read_outs {
            self.plain[usize::from(extract)] = Some(scalar);
        }
        let function = CfgFunction {
            entry: self.source.entry,
            blocks,
            values: self.values,
            shapes: Vec::new(),
        };
        function.validate().map_err(|error| JitError::Verifier {
            model: "cfg-lane-scalarizer".into(),
            detail: format!("scalarized function is malformed: {error:?}").into(),
        })?;
        Ok(ScalarLanes {
            function,
            plain: self.plain,
        })
    }

    /// Give every value that survives as a value an id, before any operand is
    /// resolved.
    ///
    /// Separating allocation from filling is what makes the pass independent of
    /// block order. A CFG numbers its blocks in creation order, in which a
    /// conditional's join precedes the arms that reach it, so a single walk in
    /// id order would meet a use before its definition; allocating first means
    /// every operand already has an id by the time any kind is written.
    fn allocate(&mut self) -> JitResult<()> {
        let source = self.source;
        // Every distinct splat constant first, so a widen's zero fill and a
        // seed's one are ordinary interned leaves by the time any lane reads
        // them.
        for value in &source.values {
            if let CfgValueKind::LaneSplat(constant) = value.kind
                && !self.constants.contains_key(&constant.to_bits())
            {
                let id = ValueId::from(self.values.len());
                self.values.push(CfgValue {
                    id,
                    value_type: CfgValueType::Real,
                    kind: CfgValueKind::RealConstant(constant),
                });
                self.constants.insert(constant.to_bits(), id);
            }
        }
        for value in &source.values {
            match source.lanes_of(value.id) {
                None => {
                    // Renamings produce no value of their own.
                    if matches!(value.kind, CfgValueKind::LaneExtract { .. }) {
                        continue;
                    }
                    let id = ValueId::from(self.values.len());
                    self.values.push(CfgValue {
                        id,
                        value_type: value.value_type,
                        kind: CfgValueKind::RealConstant(0.0),
                    });
                    self.plain[usize::from(value.id)] = Some(id);
                }
                Some(shape) => {
                    if matches!(
                        value.kind,
                        CfgValueKind::LaneWiden { .. } | CfgValueKind::LaneSplat(_)
                    ) {
                        // A widen renames and a splat is one constant per lane;
                        // neither needs an id reserved ahead of its operands.
                        continue;
                    }
                    for lane in shape.to_vec() {
                        let id = ValueId::from(self.values.len());
                        self.values.push(CfgValue {
                            id,
                            value_type: CfgValueType::Real,
                            kind: CfgValueKind::RealConstant(0.0),
                        });
                        self.lanes.insert((value.id, lane), id);
                    }
                }
            }
        }
        Ok(())
    }

    /// The scalar standing for one lane of a packed source value.
    ///
    /// Resolves the two renaming kinds rather than looking them up, which is
    /// why it is a function and not an index.
    fn lane_of(&self, value: ValueId, lane: u32, depth: usize) -> JitResult<ValueId> {
        if depth > MAX_ALIAS_DEPTH {
            return Err(self.refuse(format!(
                "packed value {} widens through more than {MAX_ALIAS_DEPTH} renamings",
                usize::from(value)
            )));
        }
        match &self.source.value(value).kind {
            CfgValueKind::LaneSplat(constant) => self
                .constants
                .get(&constant.to_bits())
                .copied()
                .ok_or_else(|| self.refuse(format!("splat constant {constant} was not interned"))),
            CfgValueKind::LaneWiden { input } => {
                if self.source.lane_position(*input, lane).is_some() {
                    self.lane_of(*input, lane, depth + 1)
                } else {
                    // The lane the operand lacks; widening fills it with zero.
                    Ok(self.zero)
                }
            }
            _ => self.lanes.get(&(value, lane)).copied().ok_or_else(|| {
                self.refuse(format!(
                    "packed value {} does not carry lane {lane}",
                    usize::from(value)
                ))
            }),
        }
    }

    /// The scalar standing for a scalar source value.
    fn plain_of(&self, value: ValueId) -> JitResult<ValueId> {
        if let CfgValueKind::LaneExtract { input, lane } = &self.source.value(value).kind {
            return self.lane_of(*input, *lane, 0);
        }
        self.plain[usize::from(value)].ok_or_else(|| {
            self.refuse(format!(
                "scalar value {} was not allocated",
                usize::from(value)
            ))
        })
    }

    /// Any source operand, packed or not, at one lane.
    ///
    /// `lane` is `None` for an operand the kind reads as a primal scalar, which
    /// is every operand of an ordinary instruction and the non-`*_derivative`
    /// operands of a stateful operator's Jacobian action.
    fn operand(&self, value: ValueId, lane: Option<u32>) -> JitResult<ValueId> {
        match lane {
            Some(lane) if self.source.lanes_of(value).is_some() => self.lane_of(value, lane, 0),
            _ => self.plain_of(value),
        }
    }

    /// Write the real kind of every allocated value.
    fn fill(&mut self) -> JitResult<()> {
        let function = self.source;
        for source in &function.values {
            match function.lanes_of(source.id) {
                None => {
                    if matches!(source.kind, CfgValueKind::LaneExtract { .. }) {
                        continue;
                    }
                    let kind = self.scalar_kind(source)?;
                    let id = self.plain_of(source.id)?;
                    self.values[usize::from(id)].kind = kind;
                }
                Some(shape) => {
                    if matches!(
                        source.kind,
                        CfgValueKind::LaneWiden { .. } | CfgValueKind::LaneSplat(_)
                    ) {
                        continue;
                    }
                    for lane in shape.to_vec() {
                        let kind = self.lane_kind(source, lane)?;
                        let id = self.lane_of(source.id, lane, 0)?;
                        self.values[usize::from(id)].kind = kind;
                    }
                }
            }
        }
        Ok(())
    }

    /// An ordinary scalar kind with its operands renumbered.
    fn scalar_kind(&self, source: &CfgValue) -> JitResult<CfgValueKind> {
        let mut kind = source.kind.clone();
        let operands = kind.operands();
        let mut rewritten = Vec::with_capacity(operands.len());
        for operand in operands {
            rewritten.push(self.operand(operand, None)?);
        }
        rewrite_operands(&mut kind, &rewritten)?;
        Ok(kind)
    }

    /// One lane of a packed kind.
    fn lane_kind(&self, source: &CfgValue, lane: u32) -> JitResult<CfgValueKind> {
        Ok(match &source.kind {
            CfgValueKind::BlockParameter => CfgValueKind::BlockParameter,
            CfgValueKind::LaneBinary { op, left, right } => CfgValueKind::Binary {
                op: *op,
                left: self.lane_of(*left, lane, 0)?,
                right: self.lane_of(*right, lane, 0)?,
            },
            CfgValueKind::LaneScalar { op, input, scalar } => CfgValueKind::Binary {
                op: *op,
                left: self.lane_of(*input, lane, 0)?,
                right: self.plain_of(*scalar)?,
            },
            // The stateful operators' Jacobian actions. Their `*_derivative`
            // operands carry this value's lanes and take the lane; everything
            // else is a primal scalar the runtime reads its local coefficient
            // from, and takes none.
            CfgValueKind::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise,
                fall,
            } => CfgValueKind::TransitionDerivative {
                site: *site,
                input: self.plain_of(*input)?,
                input_derivative: self.lane_of(*input_derivative, lane, 0)?,
                delay: self.plain_of(*delay)?,
                rise: self.plain_of(*rise)?,
                fall: self.plain_of(*fall)?,
            },
            CfgValueKind::AbsDelayDerivative {
                operator,
                input,
                input_derivative,
                delay,
                delay_derivative,
                max_delay,
                order,
            } => CfgValueKind::AbsDelayDerivative {
                operator: *operator,
                input: self.plain_of(*input)?,
                input_derivative: self.lane_of(*input_derivative, lane, 0)?,
                delay: self.plain_of(*delay)?,
                delay_derivative: self.lane_of(*delay_derivative, lane, 0)?,
                max_delay: max_delay.map(|value| self.plain_of(value)).transpose()?,
                order: *order,
            },
            CfgValueKind::SlewDerivative {
                operator,
                input,
                input_derivative,
                max_rise,
                max_rise_derivative,
                max_fall,
                max_fall_derivative,
            } => CfgValueKind::SlewDerivative {
                operator: *operator,
                input: self.plain_of(*input)?,
                input_derivative: self.lane_of(*input_derivative, lane, 0)?,
                max_rise: self.plain_of(*max_rise)?,
                max_rise_derivative: self.lane_of(*max_rise_derivative, lane, 0)?,
                max_fall: self.plain_of(*max_fall)?,
                max_fall_derivative: self.lane_of(*max_fall_derivative, lane, 0)?,
            },
            CfgValueKind::LaplaceDerivative {
                operator,
                input_derivative,
                transfer,
            } => CfgValueKind::LaplaceDerivative {
                operator: *operator,
                input_derivative: self.lane_of(*input_derivative, lane, 0)?,
                transfer: transfer.clone(),
            },
            CfgValueKind::ZiDerivative {
                operator,
                input_derivative,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => CfgValueKind::ZiDerivative {
                operator: *operator,
                input_derivative: self.lane_of(*input_derivative, lane, 0)?,
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: self.plain_of(*period)?,
                transition: self.plain_of(*transition)?,
                first_transition: self.plain_of(*first_transition)?,
                direct_assignment: *direct_assignment,
            },
            other => {
                return Err(self.refuse(format!(
                    "canonical value kind {} carries derivative lanes and this pass has no \
                     elementwise reading of it",
                    super::cfg_program::kind_name(other)
                )));
            }
        })
    }

    /// Rebuild every block over the new values.
    fn rebuild_blocks(&self) -> JitResult<Vec<CfgBlock>> {
        // Where each block's packed parameters landed: one scalar parameter per
        // lane, appended after the scalars in shape order, so an argument list
        // is built by the same walk.
        let mut blocks = Vec::with_capacity(self.source.blocks.len());
        for block in &self.source.blocks {
            let mut params = Vec::new();
            for param in &block.params {
                match self.source.lanes_of(*param) {
                    None => params.push(self.plain_of(*param)?),
                    Some(shape) => {
                        for lane in shape {
                            params.push(self.lane_of(*param, *lane, 0)?);
                        }
                    }
                }
            }
            let mut instructions = Vec::new();
            for instruction in &block.instructions {
                let result = instruction.result;
                match self.source.lanes_of(result) {
                    None => {
                        if matches!(
                            self.source.value(result).kind,
                            CfgValueKind::LaneExtract { .. }
                        ) {
                            continue;
                        }
                        instructions.push(CfgInstruction {
                            result: self.plain_of(result)?,
                        });
                    }
                    Some(shape) => {
                        if matches!(
                            self.source.value(result).kind,
                            CfgValueKind::LaneWiden { .. } | CfgValueKind::LaneSplat(_)
                        ) {
                            continue;
                        }
                        for lane in shape {
                            instructions.push(CfgInstruction {
                                result: self.lane_of(result, *lane, 0)?,
                            });
                        }
                    }
                }
            }
            let terminator = self.rebuild_terminator(block)?;
            blocks.push(CfgBlock {
                id: block.id,
                params,
                instructions,
                terminator,
            });
        }
        Ok(blocks)
    }

    /// Arguments matched to the *target's* parameter shapes.
    ///
    /// A lane the incoming argument does not carry passes zero, which is what
    /// the derivative pass's own widening does. Doing it here as well is not
    /// redundant: `CfgFunction::validate_shapes` exempts block parameters, so
    /// agreement between a packed parameter and its arguments is the one shape
    /// rule the CFG leaves unverified, and reading the target's shape rather
    /// than the argument's is what keeps a disagreement from silently landing
    /// one unknown's partial in another's slot.
    fn arguments(&self, target: BlockId, args: &[ValueId]) -> JitResult<Vec<ValueId>> {
        let params = &self.source.block(target).params;
        if params.len() != args.len() {
            return Err(self.refuse(format!(
                "block {} takes {} parameters but an edge passes {}",
                usize::from(target),
                params.len(),
                args.len()
            )));
        }
        let mut rebuilt = Vec::new();
        for (param, arg) in params.iter().zip(args) {
            match self.source.lanes_of(*param) {
                None => rebuilt.push(self.plain_of(*arg)?),
                Some(shape) => {
                    for lane in shape {
                        if self.source.lane_position(*arg, *lane).is_some() {
                            rebuilt.push(self.lane_of(*arg, *lane, 0)?);
                        } else {
                            rebuilt.push(self.zero);
                        }
                    }
                }
            }
        }
        Ok(rebuilt)
    }

    fn rebuild_terminator(&self, block: &CfgBlock) -> JitResult<CfgTerminator> {
        Ok(match &block.terminator {
            CfgTerminator::Jump { target, args } => CfgTerminator::Jump {
                target: *target,
                args: self.arguments(*target, args)?,
            },
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => CfgTerminator::Branch {
                condition: self.plain_of(*condition)?,
                then_target: *then_target,
                then_args: self.arguments(*then_target, then_args)?,
                else_target: *else_target,
                else_args: self.arguments(*else_target, else_args)?,
            },
            CfgTerminator::Return => CfgTerminator::Return,
            other => {
                return Err(self.refuse(format!(
                    "terminator {other:?} has no scalarized form; this pass is analog-only"
                )));
            }
        })
    }

    fn refuse(&self, detail: String) -> JitError {
        JitError::UnsupportedCanonicalOp {
            model: "cfg-lane-scalarizer".into(),
            op: detail.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::canonical_ir::{
        AdSeed, CfgBinaryOp, CfgEvalInputs, CfgTerminator, CfgValueType, CfgVariable, NodeId,
        ParamId, SsaBuilder, VariableId, differentiate, evaluate_cfg,
    };

    fn inputs(potentials: Vec<f64>, parameters: Vec<f64>) -> CfgEvalInputs<f64> {
        CfgEvalInputs {
            parameter_given: vec![false; parameters.len()],
            parameters,
            port_connected: Vec::new(),
            event_state: Vec::new(),
            event_controls: HashMap::new(),
            node_potentials: potentials,
            branch_flows: Vec::new(),
            branch_unknown_flows: Vec::new(),
            temperature: 300.15,
            thermal_voltage: 300.15 * 8.617_333_262e-5,
            multiplicity: 1.0,
            time: 0.0,
            analyses: HashSet::new(),
            simparams: HashMap::new(),
            ddt: 0.0,
            ddt_scale: 0.0,
            idt: 0.0,
            idt_scale: 0.0,
            staged: Vec::new(),
        }
    }

    /// `if (V(0) > p0) r = V(0) * V(1) else r = V(0) + V(1)`.
    ///
    /// A diamond, because that is what makes the packed form use a block
    /// parameter — the one carrier of lanes across control flow, and the one
    /// `CfgFunction::validate_shapes` does not check.
    fn diamond() -> (CfgFunction, ValueId) {
        let mut builder = SsaBuilder::new();
        let variable = CfgVariable::Local(VariableId::from(0usize));
        builder.declare_variable(variable, CfgValueType::Real);
        let entry = builder.create_block();
        builder.seal_block(entry);
        let drain = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::NodePotential(NodeId::from(0usize)),
        );
        let gate = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::NodePotential(NodeId::from(1usize)),
        );
        let threshold = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::Parameter(ParamId::from(0usize)),
        );
        let condition = builder.push(
            entry,
            CfgValueType::Boolean,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Gt,
                left: drain,
                right: threshold,
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
        let product = builder.push(
            then_block,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: drain,
                right: gate,
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
        // Only the drain reaches this arm, so the merge widens a one-lane
        // derivative into the two-lane shape the other arm carries.
        let sum = builder.push(
            else_block,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: drain,
                right: threshold,
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
        (function, outputs[0])
    }

    /// The whole contract in one assertion: every lane of the scalarized
    /// function answers what the packed function's own read-out answers.
    ///
    /// Against the reference interpreter on both sides rather than against a
    /// derivative rule, because the claim is that scalarizing changed nothing
    /// — not that the derivative is right, which
    /// `tests/cfg_derivatives.rs` already establishes.
    #[test]
    fn every_scalarized_lane_answers_what_the_packed_lane_answers() {
        let (function, residual) = diamond();
        let seeds = [
            AdSeed::NodePotential(NodeId::from(0usize)),
            AdSeed::NodePotential(NodeId::from(1usize)),
        ];
        let mut differentiated =
            differentiate(&function, &seeds).expect("the diamond differentiates");
        let row = differentiated.derivative_row(residual);
        let scalarized = scalarize_lanes(&differentiated.function).expect("scalarizes");

        // Both arms, so neither the taken nor the untaken merge goes unchecked.
        for potentials in [vec![2.0, 3.0], vec![-1.0, 3.0]] {
            let point = inputs(potentials, vec![0.5]);
            let packed = evaluate_cfg(&differentiated.function, &point).expect("packed evaluates");
            let scalar = evaluate_cfg(&scalarized.function, &point).expect("scalar evaluates");
            for entry in row.iter().flatten() {
                let expected = packed.value(*entry).expect("the packed row is defined");
                let mapped = scalarized
                    .scalar(*entry)
                    .expect("every read-out has a scalar");
                let actual = scalar.value(mapped).expect("the scalar row is defined");
                assert_eq!(
                    expected,
                    actual,
                    "lane read-out {} disagrees after scalarizing",
                    usize::from(*entry)
                );
            }
        }
    }

    /// The scalarized function carries no shape at all — which is what makes it
    /// lowerable by a block model whose only value type is `F64`.
    #[test]
    fn scalarizing_leaves_no_packed_value_behind() {
        let (function, residual) = diamond();
        let seeds = [
            AdSeed::NodePotential(NodeId::from(0usize)),
            AdSeed::NodePotential(NodeId::from(1usize)),
        ];
        let mut differentiated = differentiate(&function, &seeds).expect("differentiates");
        let _ = differentiated.derivative_row(residual);
        assert!(
            differentiated
                .function
                .values
                .iter()
                .any(|value| value.value_type.shape().is_some()),
            "the packed function must contain the thing this pass removes"
        );
        let scalarized = scalarize_lanes(&differentiated.function).expect("scalarizes");
        assert!(scalarized.function.shapes.is_empty());
        assert!(
            scalarized
                .function
                .values
                .iter()
                .all(|value| value.value_type.shape().is_none())
        );
    }
}

/// Replace a kind's operands, in the order [`CfgValueKind::operands`] reports
/// them.
///
/// Written against that one accessor rather than as a second match over every
/// kind, because a second match is what goes stale when a kind is added: the
/// accessor's own documentation is that "having one definition of it is what
/// keeps a newly added kind from being silently skipped by half of them".
fn rewrite_operands(kind: &mut CfgValueKind, rewritten: &[ValueId]) -> JitResult<()> {
    let mut index = 0;
    let mut short = false;
    kind.map_operands(|_| {
        let Some(value) = rewritten.get(index) else {
            short = true;
            return ValueId::from(0usize);
        };
        index += 1;
        *value
    });
    if short || index != rewritten.len() {
        return Err(JitError::Verifier {
            model: "cfg-lane-scalarizer".into(),
            detail: "operand accessor and rewriter disagree about a kind's arity".into(),
        });
    }
    Ok(())
}
