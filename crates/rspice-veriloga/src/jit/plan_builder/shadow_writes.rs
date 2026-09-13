//! Derivative publication belongs to a reaching write, not its variable slot.
//!
//! Portable AD interleaves the demanded shadows immediately before each primal
//! assignment. Later writes to that variable may need fewer orders or none.
//! Keep that schedule when differentiating MIR for native and Wasm replay;
//! generating every allocated shadow at every write both inflates the image
//! and evaluates derivatives that the source never requests.
//!
//! This reads storage/publication metadata only. The computed expressions still
//! come from canonical MIR. In-place ddx staging has its own validated schedule.

use super::{AssignmentStep, CompiledModel, derivative_shadow_axes_from_suffix};

fn target(step: &AssignmentStep) -> Option<usize> {
    match step {
        AssignmentStep::Assign(write) => Some(write.var_index),
        AssignmentStep::AssignIndexed { base, .. } => Some(*base),
        _ => None,
    }
}

pub(super) fn preceding<'a>(
    model: &CompiledModel,
    steps: &'a [AssignmentStep],
    position: usize,
) -> &'a [AssignmentStep] {
    let Some(name) = target(&steps[position]).and_then(|slot| model.variable_names.get(slot))
    else {
        return &[];
    };
    let mut start = position;
    while start > 0 {
        let step = &steps[start - 1];
        let Some(shadow) = target(step).and_then(|slot| model.variable_names.get(slot)) else {
            break;
        };
        let Some(suffix) = shadow
            .strip_prefix(name.as_str())
            .and_then(|tail| tail.strip_prefix('@'))
        else {
            break;
        };
        if derivative_shadow_axes_from_suffix(suffix).is_none() {
            break;
        }
        start -= 1;
    }
    &steps[start..position]
}

pub(super) fn contains_scalar(steps: &[AssignmentStep], slot: usize) -> bool {
    steps
        .iter()
        .any(|step| matches!(step, AssignmentStep::Assign(write) if write.var_index == slot))
}

pub(super) fn contains_indexed(
    steps: &[AssignmentStep],
    base: usize,
    len: usize,
    lower: i64,
) -> bool {
    steps.iter().any(|step| {
        matches!(step, AssignmentStep::AssignIndexed {
        base: written_base, len: written_len, lower: written_lower, ..
    } if (*written_base, *written_len, *written_lower) == (base, len, lower))
    })
}
