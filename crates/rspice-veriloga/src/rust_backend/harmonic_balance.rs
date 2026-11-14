//! Whether harmonic balance can sample a generated model's residual.
//!
//! Harmonic balance evaluates a device at the collocation samples of one
//! period, in any order, and carries nothing from one evaluation to the next.
//! It can take a model whose residual is `F(x) + dQ(x)/dt`, sampled through a
//! static stamp and a companion stamp of the same point, and nothing else: an
//! operator that keeps state in time, a variable the next evaluation reads
//! back, or a `ddt` that is not the derivative of a contributed charge each
//! puts the residual outside what the spectrum carries.
//!
//! The generator sees every operator and every variable a model keeps, so it
//! decides here, once per model, and the generated code states the answer.
//!
//! # Variables kept between evaluations
//!
//! A variable the front end marks as state (`is_state`) is one an event body
//! writes, or one some path reads before assigning. Its accepted value enters
//! the lowered body as a single leaf per slot, and SSA construction routes
//! every read through that leaf exactly when some path from the entry reaches
//! the read without an assignment. So the slot is read back only where its
//! leaf reaches the residual, an equation's activation or a branch condition;
//! a variable recomputed before every read leaves the leaf unreached. A slot
//! whose candidate is its own leaf on every path is never assigned by the
//! evaluated body: an initialization phase, which cannot read the circuit,
//! sets it once. Either way the value is the same at every sample. A slot
//! that is both read back and assigned by the body keeps history, and refuses.

use crate::canonical_ir::cfg::{CfgTerminator, CfgValueKind};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::charge::{ChargeSplitObstacle, block_parameter_arguments};
use crate::canonical_ir::{CanonicalIrArtifact, ValueId};
use std::collections::{HashMap, HashSet};

/// Why harmonic balance cannot sample the model's residual, or `None` when it
/// can. `residuals` and `activations` are the body's equation outputs;
/// `split` is the first obstacle to the `F(x) + dQ(x)/dt` form.
pub(super) fn harmonic_balance_refusal(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    residuals: &[ValueId],
    activations: &[Option<ValueId>],
    split: Option<ChargeSplitObstacle>,
) -> Option<String> {
    if let Some(reason) = operator_refusal(cfg) {
        return Some(reason);
    }
    let incoming = block_parameter_arguments(&cfg.function);
    let observed = residual_inputs(cfg, residuals, activations, &incoming);
    if cfg
        .function
        .values
        .iter()
        .any(|value| matches!(value.kind, CfgValueKind::Time) && observed[usize::from(value.id)])
    {
        return Some(
            "it reads `$abstime`, so its equations change with time in a way that cannot be \
             certified periodic over the analysis period"
                .to_owned(),
        );
    }
    let retained = retained_variables(artifact, cfg, &observed, &incoming);
    if let Some((last, rest)) = retained.split_last() {
        return Some(if rest.is_empty() {
            format!(
                "`{last}` is read before it is assigned, so it keeps its value from the last \
                 accepted time point, which no harmonic-balance spectrum carries"
            )
        } else {
            let rest = rest
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{rest} and `{last}` are read before they are assigned, so they keep their values \
                 from the last accepted time point, which no harmonic-balance spectrum carries"
            )
        });
    }
    split.map(|obstacle| split_refusal(obstacle).to_owned())
}

/// The first operator the body uses whose state lies outside the residual.
fn operator_refusal(cfg: &CfgModel) -> Option<String> {
    let used = |wanted: fn(&CfgValueKind) -> bool| {
        cfg.function.values.iter().any(|value| wanted(&value.kind))
    };
    if used(|kind| {
        matches!(
            kind,
            CfgValueKind::IdtMod { .. }
                | CfgValueKind::IdtModInitial { .. }
                | CfgValueKind::IdtModBranchDerivative { .. }
                | CfgValueKind::IntegralDerivative { wrap: Some(_), .. }
        )
    }) {
        return Some(
            "it uses `idtmod`, whose integral wraps at its modulus, which is state outside the \
             waveform's spectrum"
                .to_owned(),
        );
    }
    for (operator, wanted) in [
        (
            "cross",
            (|kind: &CfgValueKind| matches!(kind, CfgValueKind::Cross { .. }))
                as fn(&CfgValueKind) -> bool,
        ),
        ("above", |kind| matches!(kind, CfgValueKind::Above { .. })),
        ("last_crossing", |kind| {
            matches!(kind, CfgValueKind::LastCrossing { .. })
        }),
        ("timer", |kind| matches!(kind, CfgValueKind::Timer { .. })),
    ] {
        if used(wanted) {
            return Some(format!(
                "it uses `{operator}`, which schedules events in time that a harmonic-balance \
                 spectrum does not carry; transient analysis runs it"
            ));
        }
    }
    None
}

/// Every value the residual, an equation's activation or a branch condition
/// reads, directly or through a merge.
fn residual_inputs(
    cfg: &CfgModel,
    residuals: &[ValueId],
    activations: &[Option<ValueId>],
    incoming: &HashMap<ValueId, Vec<ValueId>>,
) -> Vec<bool> {
    let function = &cfg.function;
    let mut observed = vec![false; function.values.len()];
    let mut pending: Vec<ValueId> = residuals
        .iter()
        .copied()
        .chain(activations.iter().flatten().copied())
        .chain(
            function
                .blocks
                .iter()
                .filter_map(|block| match block.terminator {
                    CfgTerminator::Branch { condition, .. } => Some(condition),
                    _ => None,
                }),
        )
        .collect();
    while let Some(id) = pending.pop() {
        let index = usize::from(id);
        if std::mem::replace(&mut observed[index], true) {
            continue;
        }
        match &function.values[index].kind {
            CfgValueKind::BlockParameter => {
                pending.extend(incoming.get(&id).into_iter().flatten().copied());
            }
            kind => pending.extend(kind.operands()),
        }
    }
    observed
}

/// The state slots a read leaf of the value names: one for a scalar read,
/// every slot of the array for an indexed one.
fn read_slots(kind: &CfgValueKind) -> std::ops::Range<usize> {
    match *kind {
        CfgValueKind::EventState(slot) | CfgValueKind::EvaluationInput(slot) => {
            slot as usize..slot as usize + 1
        }
        CfgValueKind::EvaluationInputIndexed { base, len, .. } => {
            base as usize..base as usize + len as usize
        }
        _ => 0..0,
    }
}

/// The variables, by the names their authors declared, that the residual
/// reads back from an earlier evaluation and the body assigns.
fn retained_variables(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    observed: &[bool],
    incoming: &HashMap<ValueId, Vec<ValueId>>,
) -> Vec<String> {
    let function = &cfg.function;
    let mut read_back = vec![false; cfg.event_state_candidates.len()];
    for value in &function.values {
        if observed[usize::from(value.id)] {
            for slot in read_slots(&value.kind) {
                if let Some(read) = read_back.get_mut(slot) {
                    *read = true;
                }
            }
        }
    }
    let state_names: Vec<&str> = artifact
        .hir
        .variables
        .iter()
        .filter(|variable| variable.is_state)
        .map(|variable| variable.name.as_str())
        .collect();
    let mut names = Vec::new();
    for (slot, &candidate) in cfg.event_state_candidates.iter().enumerate() {
        if !read_back[slot] || !assigned_by_the_body(function, slot, candidate, incoming) {
            continue;
        }
        let name = state_names.get(slot).copied().unwrap_or("<state>");
        let name = name.split_once('[').map_or(name, |(base, _)| base);
        if !names.iter().any(|known| known == name) {
            names.push(name.to_owned());
        }
    }
    names
}

/// Whether some path of the body gives slot `slot` a value other than the
/// one it entered with.
fn assigned_by_the_body(
    function: &crate::canonical_ir::cfg::CfgFunction,
    slot: usize,
    candidate: ValueId,
    incoming: &HashMap<ValueId, Vec<ValueId>>,
) -> bool {
    let mut seen = HashSet::new();
    let mut pending = vec![candidate];
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        let kind = &function.values[usize::from(id)].kind;
        match kind {
            CfgValueKind::BlockParameter => {
                pending.extend(incoming.get(&id).into_iter().flatten().copied());
            }
            CfgValueKind::EventState(read) | CfgValueKind::EvaluationInput(read)
                if *read as usize == slot => {}
            _ => return true,
        }
    }
    false
}

/// The refusal for an obstacle to the `F(x) + dQ(x)/dt` form.
pub(super) fn split_refusal(obstacle: ChargeSplitObstacle) -> &'static str {
    match obstacle {
        ChargeSplitObstacle::Unlowered => {
            "its analog body does not lower to the form whose residual the generator can prove \
             F(x) + dQ(x)/dt"
        }
        ChargeSplitObstacle::DerivativeSteersControl => {
            "a branch or a switch-branch form depends on a `ddt` value, so its residual is not \
             F(x) + dQ(x)/dt"
        }
        // The residual proof does not count what only observes a derivative.
        ChargeSplitObstacle::DerivativeObserved | ChargeSplitObstacle::NestedDerivative => {
            "it applies another operator to a `ddt` value (a nested derivative), so its residual \
             is not F(x) + dQ(x)/dt"
        }
        ChargeSplitObstacle::FlowProbe => {
            "it reads a branch current its own `ddt` contributions can reach, so its residual is \
             not F(x) + dQ(x)/dt"
        }
        ChargeSplitObstacle::UnrecoveredCharge => {
            "it uses `ddt` on something other than a charge it contributes, so its residual is \
             not F(x) + dQ(x)/dt"
        }
        ChargeSplitObstacle::BiasScaledDerivative => {
            "it scales a `ddt` by a quantity that varies with bias, so its residual is not \
             F(x) + dQ(x)/dt"
        }
    }
}
