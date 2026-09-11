//! Shared CFG charge recovery and qualification of the weighted F/Q equation.

use super::{
    BlockId, CfgBinaryOp, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue,
    CfgValueKind, CfgValueType, ValueId,
};
use std::collections::HashMap;

/// Recover reactive terms through arithmetic and guards, leaving conduction
/// in the original residual. This projection alone does not certify a complete
/// first-order DAE split: a supported term can sit beside an unrecovered one.
/// Split eligibility also uses the completeness result from the same walk.
///
/// # The CFG level's one charge extraction
///
/// Crate-visible rather than backend-private because it is the only
/// charge extraction that operates on the CFG, and the native block route needs
/// the same answer. The tree has two others and both work on flatter forms:
/// `DeviceIR::extract_charge` peels an arena expression, and
/// `jit::plan_builder::canonical_extract_reactive_charge` peels a MIR
/// expression. Neither can see a guarded contribution, because in a CFG the
/// guard is a block parameter rather than a `Conditional` node — which is
/// exactly the case this one exists to cover, and exactly the case a
/// self-heating model is written in. A fourth copy for the block route would be
/// a fourth set of shape rules to keep in step.
pub(crate) fn stored_charges(
    function: &mut CfgFunction,
    residuals: &[ValueId],
) -> Vec<Option<ValueId>> {
    recover_stored_charges(function, residuals).0
}

pub(crate) fn recover_stored_charges(
    function: &mut CfgFunction,
    residuals: &[ValueId],
) -> (Vec<Option<ValueId>>, bool) {
    let reaches = values_reaching_a_ddt(function);
    let mut insertions: Vec<(ValueId, ValueId)> = Vec::new();
    let mut first_order_complete = true;
    let charges: Vec<Option<ValueId>> = residuals
        .iter()
        .map(|residual| {
            resolve_charge(function, &reaches, *residual, 0, &mut first_order_complete)
                .and_then(|charge| materialise_charge(function, &charge, &mut insertions))
        })
        .collect();
    // Once, at the end: an instruction list cannot be rebuilt while it is still
    // being read for the next residual, and two residuals routinely share a
    // subexpression.
    apply_insertions(function, &insertions);
    (charges, first_order_complete)
}

/// Splice each built instruction in directly after the one it mirrors.
fn apply_insertions(function: &mut CfgFunction, insertions: &[(ValueId, ValueId)]) {
    if insertions.is_empty() {
        return;
    }
    let mut after: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
    for (anchor, value) in insertions {
        after.entry(*anchor).or_default().push(*value);
    }
    for block in &mut function.blocks {
        if !block
            .instructions
            .iter()
            .any(|instruction| after.contains_key(&instruction.result))
        {
            continue;
        }
        let mut rebuilt = Vec::with_capacity(block.instructions.len() + insertions.len());
        for instruction in std::mem::take(&mut block.instructions) {
            let anchor = instruction.result;
            rebuilt.push(instruction);
            if let Some(values) = after.get(&anchor) {
                rebuilt.extend(
                    values
                        .iter()
                        .map(|result| CfgInstruction { result: *result }),
                );
            }
        }
        block.instructions = rebuilt;
    }
}

impl Charge {
    fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }
}

/// How deep a chain of merges to follow before giving up.
///
/// A charge behind more than a handful of nested guards is not a shape any
/// released model has, and a bound is what keeps a loop-carried residual — where
/// a block parameter can reach itself — from recursing forever.
const MAX_CHARGE_MERGE_DEPTH: usize = 8;

/// A primitive whose voltage/flow derivative gives the reactive Jacobian.
///
/// Outside coefficients carry a derivative barrier. The primitive is not a
/// conservative stored charge unless those coefficients are time independent.
///
/// Resolution is separated from construction because a merge has to be created
/// bottom-up: the parameter that carries a guarded charge can only be added once
/// every arm's charge exists to be passed on the edges.
enum Charge {
    /// A value the graph already holds — the operand of a `ddt`, or one side of
    /// an operation that carries no charge of its own.
    Value(ValueId),
    /// A multiplier/divisor outside ddt is evaluated at the bias point but
    /// held constant while differentiating the reactive primitive.
    HeldCoefficient { anchor: ValueId, value: ValueId },
    /// This path stores nothing. At the top that means the contribution is not
    /// reactive at all; inside a merge it is the arm that was not taken, and
    /// inside a sum it is the conduction half.
    Nothing,
    /// One charge per edge into `block`, in the order [`edges_into`] gives them.
    Merge { block: BlockId, arms: Vec<Charge> },
    /// An operation the charge needs that the graph only has in its `ddt` form.
    ///
    /// `I(db) <+ TYPE * ddt(QD)` projects to `held(TYPE) * QD`, and that product
    /// exists nowhere until it is built. It is inserted directly after `anchor` — the
    /// instruction it mirrors — so its operands are in scope exactly where the
    /// original's were, without any dominance question to answer.
    Op {
        anchor: ValueId,
        kind: Box<ChargeOp>,
    },
}

enum ChargeOp {
    Binary {
        op: CfgBinaryOp,
        left: Charge,
        right: Charge,
    },
    Unary {
        op: CfgUnaryOp,
        input: Charge,
    },
}

/// Which values can reach a `ddt` at all.
///
/// A fixed point rather than one pass, because a loop-carried block parameter
/// depends on values computed from itself. It exists so resolution can answer
/// "stores nothing" in O(1) instead of walking a residual's whole expression
/// DAG as a tree — which on a compact model is both exponential and, if it were
/// bounded to stop that, silently wrong for anything deeper than the bound.
pub(crate) fn values_reaching_a_ddt(function: &CfgFunction) -> Vec<bool> {
    let incoming = {
        let mut incoming: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
        for block in &function.blocks {
            for (target, args) in outgoing_edges(block) {
                for (param, argument) in function.block(target).params.iter().zip(args) {
                    incoming.entry(*param).or_default().push(argument);
                }
            }
        }
        incoming
    };
    let mut reaches = vec![false; function.values.len()];
    loop {
        let mut changed = false;
        for value in &function.values {
            let index = usize::from(value.id);
            if reaches[index] {
                continue;
            }
            let reached = match &value.kind {
                CfgValueKind::Ddt { .. } => true,
                CfgValueKind::BlockParameter => incoming
                    .get(&value.id)
                    .is_some_and(|args| args.iter().any(|arg| reaches[usize::from(*arg)])),
                kind => kind
                    .operands()
                    .into_iter()
                    .any(|operand| reaches[usize::from(operand)]),
            };
            if reached {
                reaches[index] = true;
                changed = true;
            }
        }
        if !changed {
            return reaches;
        }
    }
}

fn outgoing_edges(block: &crate::canonical_ir::cfg::CfgBlock) -> Vec<(BlockId, Vec<ValueId>)> {
    match &block.terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args.clone())],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![
            (*then_target, then_args.clone()),
            (*else_target, else_args.clone()),
        ],
        // See `canonical_ir::ad::outgoing`: the edge is reported, even though
        // this backend refuses a process before it can walk one.
        CfgTerminator::Wait {
            resume,
            resume_args,
            ..
        } => vec![(*resume, resume_args.clone())],
        CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
    }
}

/// `merges` counts only nested merges — arithmetic recurses freely, because in
/// SSA it is a DAG and the only way back to a value already on the stack is
/// through a loop-carried block parameter.
fn resolve_charge(
    function: &CfgFunction,
    reaches: &[bool],
    residual: ValueId,
    merges: usize,
    first_order_complete: &mut bool,
) -> Option<Charge> {
    if !reaches.get(usize::from(residual)).copied().unwrap_or(false) {
        return Some(Charge::Nothing);
    }
    if merges > MAX_CHARGE_MERGE_DEPTH {
        *first_order_complete = false;
        return None;
    }

    let charge = resolve_charge_kind(function, reaches, residual, merges, first_order_complete);
    *first_order_complete &= charge.is_some();
    charge
}

fn resolve_charge_kind(
    function: &CfgFunction,
    reaches: &[bool],
    residual: ValueId,
    merges: usize,
    first_order_complete: &mut bool,
) -> Option<Charge> {
    match &function.value(residual).kind {
        CfgValueKind::Ddt { input, .. } => {
            // Preserve the existing reactive projection, but do not certify
            // a higher-order equation for the first-order OneStep split.
            *first_order_complete &= !reaches[usize::from(*input)];
            Some(Charge::Value(*input))
        }
        CfgValueKind::RealConstant(constant) if *constant == 0.0 => Some(Charge::Nothing),
        // Project arithmetic linear in ddt: k * ddt(q) has reactive derivative
        // k * dq/dx, with no q * dk/dx term. Holding the outside coefficient's
        // tangent gives that result while preserving its current bias value.
        // Products of derivatives are nonlinear and cannot use this projection.
        CfgValueKind::Binary { op, left, right } => {
            let (op, left, right) = (*op, *left, *right);
            // An unrecovered operand must invalidate split eligibility even
            // when the existing reactive projection retains the other term.
            let charged_left =
                resolve_charge(function, reaches, left, merges, first_order_complete);
            let charged_right =
                resolve_charge(function, reaches, right, merges, first_order_complete);
            let stores =
                |charge: &Option<Charge>| matches!(charge, Some(charge) if !charge.is_nothing());
            match (op, stores(&charged_left), stores(&charged_right)) {
                // A sum of a conduction term and a charge stores only the
                // charge — the conduction half is already in the residual and
                // the reactive matrix does not want it. That is what the `0 + x`
                // accumulator is, generalised.
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, false) => charged_left,
                (CfgBinaryOp::Add, false, true) => charged_right,
                (CfgBinaryOp::Sub, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary {
                        op: CfgUnaryOp::Neg,
                        input: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Mul, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::HeldCoefficient {
                            anchor: residual,
                            value: right,
                        },
                    }),
                }),
                (CfgBinaryOp::Mul, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: Charge::HeldCoefficient {
                            anchor: residual,
                            value: left,
                        },
                        right: charged_right?,
                    }),
                }),
                // Dividing a charge by something that carries none is still
                // linear in the charge; the other way round is not.
                (CfgBinaryOp::Div, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::HeldCoefficient {
                            anchor: residual,
                            value: right,
                        },
                    }),
                }),
                (_, false, false) => Some(Charge::Nothing),
                _ => None,
            }
        }
        CfgValueKind::Unary {
            op: op @ CfgUnaryOp::Neg,
            input,
        } => {
            let (op, input) = (*op, *input);
            match resolve_charge(function, reaches, input, merges, first_order_complete)? {
                Charge::Nothing => Some(Charge::Nothing),
                charge => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary { op, input: charge }),
                }),
            }
        }
        // A guarded contribution. `I(a, b) <+ ddt(q)` inside an `if` reaches its
        // equation as a merge of the `ddt` from the arm that ran and zero from
        // the arm that did not, so matching the residual against `Ddt` alone
        // finds nothing and the whole reactive contribution disappears — silent
        // in DC, wrong in AC and transient. Self-heating blocks are guarded as a
        // matter of course, so this is the common case rather than an edge one.
        CfgValueKind::BlockParameter => {
            let block = owning_block(function, residual)?;
            let position = function
                .block(block)
                .params
                .iter()
                .position(|param| *param == residual)?;
            let mut arms = Vec::new();
            let mut stores = false;
            for (source, slot) in edges_into(function, block) {
                let argument = *edge_arguments(function, source, slot).get(position)?;
                let arm = resolve_charge(
                    function,
                    reaches,
                    argument,
                    merges + 1,
                    first_order_complete,
                )?;
                stores |= !matches!(arm, Charge::Nothing);
                arms.push(arm);
            }
            // Every arm storing nothing is a merge worth building only if some
            // arm stores something; otherwise the contribution is conduction and
            // adding a parameter for it would be noise in the graph.
            stores.then_some(Charge::Merge { block, arms })
        }
        _ => None,
    }
}

fn materialise_charge(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> Option<ValueId> {
    match charge {
        Charge::Value(value) => Some(*value),
        Charge::HeldCoefficient { anchor, value } => {
            let held = push_value(
                function,
                CfgValueType::Real,
                CfgValueKind::Unary {
                    op: CfgUnaryOp::FreezeDerivative,
                    input: *value,
                },
            );
            insertions.push((*anchor, held));
            Some(held)
        }
        Charge::Nothing => None,
        Charge::Op { anchor, kind } => {
            let result = match &**kind {
                ChargeOp::Binary { op, left, right } => {
                    let left = materialise_operand(function, left, insertions);
                    let right = materialise_operand(function, right, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Binary {
                            op: *op,
                            left,
                            right,
                        },
                    )
                }
                ChargeOp::Unary { op, input } => {
                    let input = materialise_operand(function, input, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Unary { op: *op, input },
                    )
                }
            };
            insertions.push((*anchor, result));
            Some(result)
        }
        Charge::Merge { block, arms } => {
            // Depth first: an arm that is itself a merge has to own a parameter
            // before this one can name it on an edge.
            let mut arguments = Vec::with_capacity(arms.len());
            for arm in arms {
                let value = match materialise_charge(function, arm, insertions) {
                    Some(value) => value,
                    None => zero_constant(function),
                };
                arguments.push(value);
            }
            let parameter = push_value(function, CfgValueType::Real, CfgValueKind::BlockParameter);
            function.blocks[usize::from(*block)].params.push(parameter);
            for ((source, slot), argument) in
                edges_into(function, *block).into_iter().zip(arguments)
            {
                edge_arguments_mut(function, source, slot).push(argument);
            }
            Some(parameter)
        }
    }
}

/// An operand of a built operation, with "stores nothing" spelled as a zero.
fn materialise_operand(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> ValueId {
    match materialise_charge(function, charge, insertions) {
        Some(value) => value,
        None => zero_constant(function),
    }
}

/// Which arm of a terminator an edge leaves by.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeSlot {
    Jump,
    Then,
    Else,
}

/// Every edge into `block`, in a deterministic order: by source block id, then
/// then-arm before else-arm. Parameters and arguments are index-aligned, so this
/// order is the contract between resolution and construction.
fn edges_into(function: &CfgFunction, block: BlockId) -> Vec<(BlockId, EdgeSlot)> {
    let mut edges = Vec::new();
    for source in &function.blocks {
        match &source.terminator {
            CfgTerminator::Jump { target, .. } if *target == block => {
                edges.push((source.id, EdgeSlot::Jump));
            }
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                if *then_target == block {
                    edges.push((source.id, EdgeSlot::Then));
                }
                if *else_target == block {
                    edges.push((source.id, EdgeSlot::Else));
                }
            }
            _ => {}
        }
    }
    edges
}

fn edge_arguments(function: &CfgFunction, source: BlockId, slot: EdgeSlot) -> &[ValueId] {
    match (&function.block(source).terminator, slot) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => &[],
    }
}

fn edge_arguments_mut(
    function: &mut CfgFunction,
    source: BlockId,
    slot: EdgeSlot,
) -> &mut Vec<ValueId> {
    match (&mut function.blocks[usize::from(source)].terminator, slot) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => unreachable!("edges_into only reports slots the terminator has"),
    }
}

fn owning_block(function: &CfgFunction, parameter: ValueId) -> Option<BlockId> {
    function
        .blocks
        .iter()
        .find(|block| block.params.contains(&parameter))
        .map(|block| block.id)
}

fn zero_constant(function: &mut CfgFunction) -> ValueId {
    if let Some(existing) = function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::RealConstant(constant) if constant == 0.0))
    {
        return existing.id;
    }
    push_value(
        function,
        CfgValueType::Real,
        CfgValueKind::RealConstant(0.0),
    )
}

fn push_value(function: &mut CfgFunction, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
    let id = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id,
        value_type,
        kind,
    });
    id
}

/// Prove that replacing each contributed derivative by its weighted charge
/// difference cannot change control, effects, state inputs or public readbacks.
/// Failure selects the ordinary companion formulation; it does not refuse the model.
pub(crate) fn one_step_dae_split_safe(
    artifact: &super::CanonicalIrArtifact,
    per_instance_ports: bool,
) -> bool {
    let Ok(mut cfg) =
        super::CfgModel::from_hir_for_dae_proof(&artifact.hir, &artifact.mir, per_instance_ports)
    else {
        return false;
    };
    let reaches = values_reaching_a_ddt(&cfg.function);
    let reaches_ddt = |id: ValueId| reaches[usize::from(id)];
    if cfg.function.blocks.iter().any(|block| {
        matches!(
            block.terminator, CfgTerminator::Branch { condition, .. } if reaches_ddt(condition)
        )
    }) || cfg
        .activations
        .iter()
        .flatten()
        .chain(&cfg.event_state_candidates)
        .chain(&cfg.observation_roots)
        .chain(cfg.timestep_bound.iter())
        .chain(cfg.discontinuity.iter())
        .copied()
        .any(reaches_ddt)
    {
        return false;
    }
    // Noise magnitudes are retained separately from the deterministic residual.
    // Their inputs must see the physical derivative, never the doubled DDT rule.
    let noise_reads_ddt = cfg.noise.iter().any(|source| {
        [source.active, source.psd]
            .into_iter()
            .chain(source.exponent)
            .chain(source.table.iter().copied())
            .any(reaches_ddt)
    }) || cfg.noise_processes.iter().any(|process| {
        [process.active, process.psd]
            .into_iter()
            .chain(process.exponent)
            .chain(process.table.iter().copied())
            .chain(
                process
                    .site
                    .iter()
                    .flat_map(|site| std::iter::once(site.psd).chain(site.exponent)),
            )
            .any(reaches_ddt)
    });
    if noise_reads_ddt {
        return false;
    }
    for value in &cfg.function.values {
        let kind = &value.kind;
        // A nested derivative has no first-order Q projection. A derivative
        // fed to another state operator or a task would observe the doubled
        // numerical value rather than merely contributing it to an equation.
        if (kind.state_site().is_some() || matches!(kind, CfgValueKind::AnalogTask(_)))
            && kind.operands().into_iter().any(reaches_ddt)
        {
            return false;
        }
        if matches!(
            kind,
            CfgValueKind::BranchFlow(_) | CfgValueKind::ContributedCurrent { .. }
        ) {
            return false;
        }
    }
    let (charges, complete) = recover_stored_charges(&mut cfg.function, &cfg.residuals);
    if !complete
        || cfg
            .residuals
            .iter()
            .zip(charges)
            .any(|(residual, charge)| reaches_ddt(*residual) && charge.is_none())
    {
        return false;
    }
    let scopes: Vec<_> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.scope)
        .collect();
    let schedule = super::schedule::schedule_with_parameter_scopes(&cfg.function, &scopes);
    cfg.function.values.iter().all(|value| {
        !matches!(
            value.kind,
            CfgValueKind::Unary {
                op: CfgUnaryOp::FreezeDerivative,
                ..
            }
        ) || schedule.class(value.id) <= super::schedule::InvalidationClass::Temperature
    })
}

#[cfg(test)]
mod tests {
    use super::one_step_dae_split_safe;
    use crate::VerilogACompiler;

    #[cfg(all(feature = "native", feature = "wasm-jit"))]
    #[test]
    fn one_step_proof_is_carried_by_native_and_wasm_images() {
        for (source, safe) in [
            (
                include_str!("../../tests/fixtures/distinct_integration_rules.va"),
                true,
            ),
            (
                "module effect(p,n); inout p,n; electrical p,n; analog begin $bound_step(abs(ddt(V(p,n)))+1); I(p,n)<+V(p,n); end endmodule",
                false,
            ),
        ] {
            let report = VerilogACompiler::default()
                .compile_runtime(source, None)
                .unwrap();
            let wasm =
                crate::wasm_jit::compile_model_value_module(&report.model, &report.canonical_ir)
                    .unwrap();
            let executable =
                crate::wasm_jit::WasmJitExecutable::from_artifact(&report.model, &wasm).unwrap();
            assert_eq!(executable.one_step_dae_split_safe(), safe);
            let device = crate::device::VerilogADevice::try_new_with_canonical_ir(
                "proof",
                report.model,
                &report.canonical_ir,
                &[1, 0],
            )
            .unwrap();
            assert_eq!(device.one_step_dae_split_safe(), safe);
        }
    }

    #[test]
    fn one_step_proof_covers_equations_effects_and_observed_values() {
        for (body, safe) in [
            ("I(p,n)<+2*V(p,n)+ddt(V(p,n)*V(p,n));", true),
            ("I(p,n)<+k*ddt(V(p,n));", true),
            ("if(k>0) I(p,n)<+ddt(V(p,n));", true),
            ("I(p,n)<+V(p,n)*ddt(V(p,n));", false),
            ("I(p,n)<+ddt(V(p,n))*ddt(V(p,n));", false),
            ("I(p,n)<+ddt(ddt(V(p,n)));", false),
            ("I(p,n)<+ddt(V(p,n))+sin(ddt(V(p,n)));", false),
            ("if(ddt(V(p,n))>0) I(p,n)<+V(p,n);", false),
            ("I(p,n)<+idt(V(p,n),2)+ddt(V(p,n));", true),
            ("I(p,n)<+idt(ddt(V(p,n)),2);", false),
            ("I(p,n)<+white_noise(1e-12,\"n\")+ddt(V(p,n));", true),
            ("I(p,n)<+white_noise(abs(ddt(V(p,n))),\"n\");", false),
            (
                "I(p,n)<+flicker_noise(1e-12,abs(ddt(V(p,n))),\"n\");",
                false,
            ),
            ("$bound_step(abs(ddt(V(p,n)))+1); I(p,n)<+V(p,n);", false),
            ("$finish(ddt(V(p,n))); I(p,n)<+V(p,n);", false),
            ("observed=ddt(V(p,n)); I(p,n)<+V(p,n);", false),
            ("observed=ddt(V(p,n)); observed=2; I(p,n)<+V(p,n);", true),
            (
                "sampled=ddt(V(p,n)); @(timer(1)) observed=sampled; sampled=2; I(p,n)<+observed;",
                false,
            ),
            (
                "I(p,n)<+laplace_nd(V(p,n),'{1.0},'{1.0,1.0})+ddt(V(p,n));",
                true,
            ),
        ] {
            let source = format!(
                "module proof(p,n); inout p,n; electrical p,n; parameter real k=2; real observed, sampled; analog begin {body} end endmodule"
            );
            let artifact = VerilogACompiler::default()
                .compile_canonical_ir(&source)
                .unwrap();
            for runtime in [false, true] {
                assert_eq!(
                    one_step_dae_split_safe(&artifact, runtime),
                    safe,
                    "runtime={runtime}, source={body}"
                );
            }
        }
    }
}
